extern crate proc_macro;

use pgx_utils::{categorize_return_type, CategorizedType};
use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use std::ops::Deref;
use std::str::FromStr;
use syn::export::{Span, TokenStream2};
use syn::spanned::Spanned;
use syn::{
    FnArg, ForeignItem, ForeignItemFn, ItemFn, ItemForeignMod, Pat, ReturnType, Signature, Type,
    Visibility,
};

pub struct PgGuardRewriter();

impl PgGuardRewriter {
    pub fn new() -> Self {
        PgGuardRewriter()
    }

    pub fn extern_block(&self, block: ItemForeignMod) -> proc_macro2::TokenStream {
        let mut stream = TokenStream2::new();

        for item in block.items.into_iter() {
            stream.extend(self.foreign_item(item));
        }

        stream
    }

    pub fn item_fn(
        &self,
        func: ItemFn,
        rewrite_args: bool,
        is_raw: bool,
        no_guard: bool,
    ) -> proc_macro2::TokenStream {
        if rewrite_args {
            self.item_fn_with_rewrite(func, is_raw, no_guard)
        } else {
            self.item_fn_without_rewrite(func)
        }
    }

    fn item_fn_with_rewrite(
        &self,
        mut func: ItemFn,
        is_raw: bool,
        no_guard: bool,
    ) -> proc_macro2::TokenStream {
        // remember the original visibility and signature classifications as we want
        // to use those for the outer function
        let vis = func.vis.clone();

        // but for the inner function (the one we're wrapping) we don't need any kind of
        // abi classification
        func.sig.abi = None;

        let arg_list = PgGuardRewriter::build_arg_list(&func.sig, true);
        let func_name = &func.sig.ident;
        let func_span = func.span().clone();
        let rewritten_args = self.rewrite_args(func.clone(), is_raw);
        let rewritten_return_type = self.rewrite_return_type(func.clone());
        let func_name_wrapper = Ident::new(
            &format!("{}_wrapper", &func.sig.ident.to_string()),
            func_span,
        );

        let returns_void = rewritten_return_type
            .to_string()
            .contains("pg_return_void()");
        let result_var_name = if returns_void {
            Ident::new("_", Span::call_site())
        } else {
            Ident::new("result", Span::call_site())
        };

        let func_call = if no_guard {
            quote! {
                    let result = {
                        #rewritten_args

                        #func_name(#arg_list)
                    };
            }
        } else {
            quote! {
                    let #result_var_name = pg_sys::guard::guard( || {
                        #rewritten_args

                        #func_name(#arg_list)
                    } );
            }
        };

        let prolog = quote! {
            #[inline]
            #func

            #[no_mangle]
            #[allow(unused_variables)]
        };
        match categorize_return_type(&func) {
            CategorizedType::Default => PgGuardRewriter::impl_standard_udf(
                func_span,
                prolog,
                vis,
                func_name_wrapper,
                func_call,
                rewritten_return_type,
            ),

            CategorizedType::Iterator(types) if types.len() == 1 => {
                PgGuardRewriter::impl_setof_srf(
                    types,
                    func_span,
                    prolog,
                    vis,
                    func_name_wrapper,
                    func_call,
                )
            }
            CategorizedType::Iterator(types) => PgGuardRewriter::impl_table_srf(
                types,
                func_span,
                prolog,
                vis,
                func_name_wrapper,
                func_call,
            ),
        }
    }

    fn impl_standard_udf(
        func_span: Span,
        prolog: proc_macro2::TokenStream,
        vis: Visibility,
        func_name_wrapper: Ident,
        func_call: proc_macro2::TokenStream,
        rewritten_return_type: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        quote_spanned! {func_span=>
            #prolog
            #vis unsafe fn #func_name_wrapper(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {

                #func_call

                #rewritten_return_type
            }
        }
    }

    fn impl_setof_srf(
        types: Vec<String>,
        func_span: Span,
        prolog: proc_macro2::TokenStream,
        vis: Visibility,
        func_name_wrapper: Ident,
        func_call: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let generic_type = proc_macro2::TokenStream::from_str(types.first().unwrap()).unwrap();

        quote_spanned! {func_span=>
            #prolog
            #vis fn #func_name_wrapper(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {

                struct IteratorHolder<T> {
                    iter: *mut dyn Iterator<Item=T>,
                }

                let mut funcctx: pgx::PgBox<pg_sys::FuncCallContext>;
                let mut iterator_holder: pgx::PgBox<IteratorHolder<#generic_type>>;

                if srf_is_first_call(fcinfo) {
                    funcctx = pgx::srf_first_call_init(fcinfo);
                    funcctx.user_fctx = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).palloc_struct::<IteratorHolder<#generic_type>>() as void_mut_ptr;

                    iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#generic_type>);

                    #func_call
                    iterator_holder.iter = Box::leak(Box::new(result));
                }

                funcctx = pgx::srf_per_call_setup(fcinfo);
                iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#generic_type>);

                let mut iter = unsafe { Box::from_raw(iterator_holder.iter) };
                match iter.next() {
                    Some(result) => {
                        // we need to leak the boxed iterator so that it's not freed by rust and we can
                        // continue to use it
                        Box::leak(iter);
                        let datum = result.into_datum();
                        pgx::srf_return_next(fcinfo, &mut funcctx);
                        return datum.unwrap();
                    },
                    None => {
                        // explicitly drop the boxed iterator since we're done with it
                        drop(iter);
                        pgx::srf_return_done(fcinfo, &mut funcctx);
                        pgx::pg_return_null(fcinfo)
                    },
                }
            }
        }
    }

    fn impl_table_srf(
        types: Vec<String>,
        func_span: Span,
        prolog: proc_macro2::TokenStream,
        vis: Visibility,
        func_name_wrapper: Ident,
        func_call: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let numtypes = types.len();
        let i = (0..numtypes).map(syn::Index::from);
        let create_heap_tuple = quote! {
            let mut datums: [usize; #numtypes] = [0; #numtypes];
            let mut nulls: [bool; #numtypes] = [false; #numtypes];

            // TODO:  how to detect that 'result.i' is an Option, and if it's none
            //        set nulls[i] to true?
            #( datums[#i] = result.#i.into_datum().unwrap() as usize; )*

            let heap_tuple = unsafe { pgx::pg_sys::heap_form_tuple(funcctx.tuple_desc, datums.as_mut_ptr(), nulls.as_mut_ptr()) };
        };

        let composite_type = format!("({})", types.join(","));
        let generic_type = proc_macro2::TokenStream::from_str(&composite_type).unwrap();
        quote_spanned! {func_span=>
            #prolog
            #vis fn #func_name_wrapper(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {

                struct IteratorHolder<T> {
                    iter: *mut dyn Iterator<Item=T>,
                }

                let mut funcctx: pgx::PgBox<pg_sys::FuncCallContext>;
                let mut iterator_holder: pgx::PgBox<IteratorHolder<#generic_type>>;

                if srf_is_first_call(fcinfo) {
                    funcctx = pgx::srf_first_call_init(fcinfo);
                    funcctx.user_fctx = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).palloc_struct::<IteratorHolder<#generic_type>>() as void_mut_ptr;
                    funcctx.tuple_desc = pgx::PgMemoryContexts::For(funcctx.multi_call_memory_ctx).switch_to(|| {
                        let mut tupdesc: *mut pgx::pg_sys::TupleDescData = std::ptr::null_mut();

                        unsafe {
                            /* Build a tuple descriptor for our result type */
                            if pgx::pg_sys::get_call_result_type(fcinfo, std::ptr::null_mut(), &mut tupdesc) != pgx::pg_sys::TypeFuncClass_TYPEFUNC_COMPOSITE {
                                pgx::error!("return type must be a row type");
                            }

                            pgx::pg_sys::BlessTupleDesc(tupdesc)
                        }
                    });
                    iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#generic_type>);

                    #func_call
                    iterator_holder.iter = Box::leak(Box::new(result));
                }

                funcctx = pgx::srf_per_call_setup(fcinfo);
                iterator_holder = pgx::PgBox::from_pg(funcctx.user_fctx as *mut IteratorHolder<#generic_type>);

                let mut iter = unsafe { Box::from_raw(iterator_holder.iter) };
                match iter.next() {
                    Some(result) => {
                        // we need to leak the boxed iterator so that it's not freed by rust and we can
                        // continue to use it
                        Box::leak(iter);

                        #create_heap_tuple

                        let datum = pgx::heap_tuple_get_datum(heap_tuple);
                        pgx::srf_return_next(fcinfo, &mut funcctx);
                        return datum as pgx::pg_sys::Datum;
                    },
                    None => {
                        // explicitly drop the boxed iterator since we're done with it
                        drop(iter);
                        pgx::srf_return_done(fcinfo, &mut funcctx);
                        pgx::pg_return_null(fcinfo)
                    },
                }
            }
        }
    }

    fn item_fn_without_rewrite(&self, mut func: ItemFn) -> proc_macro2::TokenStream {
        // remember the original visibility and signature classifications as we want
        // to use those for the outer function
        let sig = func.sig.clone();
        let vis = func.vis.clone();

        // but for the inner function (the one we're wrapping) we don't need any kind of
        // abi classification
        func.sig.abi = None;

        // nor do we need a visibility beyond "private"
        func.vis = Visibility::Inherited;

        let arg_list = PgGuardRewriter::build_arg_list(&sig, false);
        let func_name = PgGuardRewriter::build_func_name(&sig);

        quote_spanned! {func.span()=>
            #[no_mangle]
            #vis #sig {
                #func

                pg_sys::guard::guard( || #func_name(#arg_list) )
            }
        }
    }

    pub fn foreign_item(&self, item: ForeignItem) -> proc_macro2::TokenStream {
        match item {
            ForeignItem::Fn(func) => {
                if func.sig.variadic.is_some() {
                    return quote! { extern "C" { #func } };
                }

                self.foreign_item_fn(func)
            }
            _ => quote! { extern "C" { #item } },
        }
    }

    pub fn foreign_item_fn(&self, func: ForeignItemFn) -> proc_macro2::TokenStream {
        let func_name = PgGuardRewriter::build_func_name(&func.sig);
        let arg_list = PgGuardRewriter::rename_arg_list(&func.sig);
        let arg_list_with_types = PgGuardRewriter::rename_arg_list_with_types(&func.sig);
        let return_type = PgGuardRewriter::get_return_type(&func.sig);

        quote! {
            pub unsafe fn #func_name ( #arg_list_with_types ) #return_type {
                extern "C" {
                    pub fn #func_name( #arg_list_with_types ) #return_type ;
                }

                pg_sys::guard::guard(|| unsafe { #func_name( #arg_list) })
            }
        }
    }

    pub fn build_func_name(sig: &Signature) -> Ident {
        sig.ident.clone()
    }

    pub fn build_arg_list(sig: &Signature, suffix_arg_name: bool) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        if suffix_arg_name {
                            let ident = Ident::new(&format!("{}_", ident.ident), ident.span());
                            arg_list.extend(quote! { #ident, });
                        } else {
                            arg_list.extend(quote! { #ident, });
                        }
                    }
                }
                FnArg::Receiver(_) => panic!(
                    "#[pg_guard] doesn't support external functions with 'self' as the argument"
                ),
            }
        }

        arg_list
    }

    pub fn rename_arg_list(sig: &Signature) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(ident) = ty.pat.deref() {
                        // prefix argument name with "arg_""
                        let name = Ident::new(&format!("arg_{}", ident.ident), ident.ident.span());
                        arg_list.extend(quote! { #name, });
                    }
                }
                FnArg::Receiver(_) => panic!(
                    "#[pg_guard] doesn't support external functions with 'self' as the argument"
                ),
            }
        }

        arg_list
    }

    pub fn rename_arg_list_with_types(sig: &Signature) -> proc_macro2::TokenStream {
        let mut arg_list = proc_macro2::TokenStream::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Typed(ty) => {
                    if let Pat::Ident(_) = ty.pat.deref() {
                        // prefix argument name with a "arg_"
                        let arg =
                            proc_macro2::TokenStream::from_str(&format!("arg_{}", quote! {#ty}))
                                .unwrap();
                        arg_list.extend(quote! { #arg, });
                    }
                }
                FnArg::Receiver(_) => panic!(
                    "#[pg_guard] doesn't support external functions with 'self' as the argument"
                ),
            }
        }

        arg_list
    }

    pub fn get_return_type(sig: &Signature) -> ReturnType {
        sig.output.clone()
    }

    pub fn rewrite_args(&self, func: ItemFn, is_raw: bool) -> proc_macro2::TokenStream {
        let fsr = FunctionSignatureRewriter::new(func);
        let args = fsr.args(is_raw);

        quote! {
            #args
        }
    }

    pub fn rewrite_return_type(&self, func: ItemFn) -> proc_macro2::TokenStream {
        let fsr = FunctionSignatureRewriter::new(func);
        let result = fsr.return_type();

        quote! {
            #result
        }
    }
}

struct FunctionSignatureRewriter {
    func: ItemFn,
}

impl FunctionSignatureRewriter {
    fn new(func: ItemFn) -> Self {
        FunctionSignatureRewriter { func }
    }

    fn return_type(&self) -> proc_macro2::TokenStream {
        let mut stream = proc_macro2::TokenStream::new();
        match &self.func.sig.output {
            ReturnType::Default => {
                stream.extend(quote! {
                    pgx::pg_return_void()
                });
            }
            ReturnType::Type(_, type_) => {
                if type_matches(type_, "Option") {
                    stream.extend(quote! {
                        match result {
                            Some(result) => {
                                result.into_datum().unwrap_or_else(|| panic!("returned Option<T> was NULL"))
                            },
                            None => pgx::pg_return_null(fcinfo)
                        }
                    });
                } else {
                    stream.extend(quote! {
                        result.into_datum().unwrap_or_else(|| panic!("returned Datum was NULL"))
                    });
                }
            }
        }

        stream
    }

    fn args(&self, is_raw: bool) -> proc_macro2::TokenStream {
        if self.func.sig.inputs.len() == 1 && self.return_type_is_datum() {
            match self.func.sig.inputs.first().unwrap() {
                FnArg::Typed(ty) => {
                    if type_matches(&ty.ty, "pg_sys :: FunctionCallInfo") {
                        return proc_macro2::TokenStream::new();
                    }
                }
                _ => {}
            }
        }

        let mut stream = proc_macro2::TokenStream::new();
        let mut i = 0usize;
        let mut have_fcinfo = false;
        for arg in &self.func.sig.inputs {
            match arg {
                FnArg::Receiver(_) => panic!("Functions that take self are not supported"),
                FnArg::Typed(ty) => match ty.pat.deref() {
                    Pat::Ident(ident) => {
                        let name = Ident::new(&format!("{}_", ident.ident), ident.span());
                        let type_ = &ty.ty;
                        let is_option = type_matches(type_, "Option");

                        if have_fcinfo {
                            panic!("When using `pg_sys::FunctionCallInfo` as an argument it must be the last argument")
                        }

                        let ts = if is_option {
                            let option_type = extract_option_type(type_);
                            quote_spanned! {ident.span()=>
                                let #name = pgx::pg_getarg::<#option_type>(fcinfo, #i);
                            }
                        } else if type_matches(type_, "pg_sys :: FunctionCallInfo") {
                            have_fcinfo = true;
                            quote_spanned! {ident.span()=>
                                let #name = fcinfo;
                            }
                        } else if is_raw {
                            quote_spanned! {ident.span()=>
                                let #name = pgx::pg_getarg_datum_raw(fcinfo, #i) as #type_;
                            }
                        } else {
                            quote_spanned! {ident.span()=>
                                let #name = pgx::pg_getarg::<#type_>(fcinfo, #i).unwrap_or_else(|| panic!("{} is null", stringify!{#ident}));
                            }
                        };

                        stream.extend(ts);

                        i += 1;
                    }
                    _ => panic!("Unrecognized function arg type"),
                },
            }
        }

        stream
    }

    fn return_type_is_datum(&self) -> bool {
        match &self.func.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ty) => type_matches(ty, "pg_sys :: Datum"),
        }
    }
}

fn type_matches(ty: &Box<Type>, pattern: &str) -> bool {
    match ty.deref() {
        Type::Path(path) => {
            let path = format!("{}", quote! {#path});
            path.starts_with(pattern)
        }
        _ => false,
    }
}

fn extract_option_type(ty: &Box<Type>) -> proc_macro2::TokenStream {
    match ty.deref() {
        Type::Path(path) => {
            let mut stream = proc_macro2::TokenStream::new();
            for segment in &path.path.segments {
                let arguments = &segment.arguments;

                stream.extend(quote! { #arguments });
            }

            let string = stream.to_string();
            let string = string.trim().trim_start_matches('<');
            let string = string.trim().trim_end_matches('>');

            proc_macro2::TokenStream::from_str(string.trim()).unwrap()
        }
        _ => panic!("No type found inside Option"),
    }
}

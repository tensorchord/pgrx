//
// we allow improper_ctypes just to eliminate these warnings:
//      = note: `#[warn(improper_ctypes)]` on by default
//      = note: 128-bit integers don't currently have a known stable ABI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(clippy::unneeded_field_pattern)]

//
// our actual bindings modules -- these are generated by build.rs
//

mod common;

#[cfg(feature = "pg10")]
pub mod pg10_specific;

#[cfg(feature = "pg11")]
pub mod pg11_specific;

#[cfg(feature = "pg12")]
pub mod pg12_specific;

//
// publicly expose the contents of our version modules
// these are hidden behind feature gates because we can
// only expose one of these behind "pg_sys" at a time
//

pub use all_versions::*;
pub use internal::*;

// version modules
// These exist to allow us to add additional items to the various version namespaces
// that couldn't be auto-generated by bindgen

/// item declarations we want to add to all versions
mod all_versions {
    use memoffset::*;
    use std::str::FromStr;

    /// this comes from `postgres_ext.h`
    pub const InvalidOid: super::Oid = 0;
    pub const InvalidOffsetNumber: super::OffsetNumber = 0;
    pub const FirstOffsetNumber: super::OffsetNumber = 1;
    pub const MaxOffsetNumber: super::OffsetNumber =
        (super::BLCKSZ as usize / std::mem::size_of::<super::ItemIdData>()) as super::OffsetNumber;
    pub const InvalidBlockNumber: u32 = 0xFFFF_FFFF as crate::pg_sys::BlockNumber;
    pub const VARHDRSZ: usize = std::mem::size_of::<super::int32>();
    pub const InvalidTransactionId: super::TransactionId = 0 as super::TransactionId;
    pub const BootstrapTransactionId: super::TransactionId = 1 as super::TransactionId;
    pub const FrozenTransactionId: super::TransactionId = 2 as super::TransactionId;
    pub const FirstNormalTransactionId: super::TransactionId = 3 as super::TransactionId;
    pub const MaxTransactionId: super::TransactionId = 0xFFFF_FFFF as super::TransactionId;

    #[inline]
    pub fn VARHDRSZ_EXTERNAL() -> usize {
        offset_of!(super::varattrib_1b_e, va_data)
    }

    #[inline]
    pub fn VARHDRSZ_SHORT() -> usize {
        offset_of!(super::varattrib_1b, va_data)
    }

    #[inline]
    pub fn get_pg_major_version_string() -> &'static str {
        let mver = std::ffi::CStr::from_bytes_with_nul(super::PG_MAJORVERSION).unwrap();
        mver.to_str().unwrap()
    }

    #[inline]
    pub fn get_pg_major_version_num() -> u16 {
        u16::from_str(super::get_pg_major_version_string()).unwrap()
    }

    #[inline]
    pub fn get_pg_version_string() -> &'static str {
        let ver = std::ffi::CStr::from_bytes_with_nul(super::PG_VERSION_STR).unwrap();
        ver.to_str().unwrap()
    }

    #[inline]
    pub fn TransactionIdIsNormal(xid: super::TransactionId) -> bool {
        xid >= FirstNormalTransactionId
    }

    #[inline]
    pub fn HeapTupleHeaderGetXmin(
        htup_header: super::HeapTupleHeader,
    ) -> Option<super::TransactionId> {
        extern "C" {
            pub fn pgx_HeapTupleHeaderGetXmin(
                htup_header: super::HeapTupleHeader,
            ) -> super::TransactionId;
        }

        if htup_header.is_null() {
            None
        } else {
            Some(unsafe { pgx_HeapTupleHeaderGetXmin(htup_header) })
        }
    }

    #[inline]
    pub fn HeapTupleHeaderGetRawCommandId(
        htup_header: super::HeapTupleHeader,
    ) -> Option<super::CommandId> {
        extern "C" {
            pub fn pgx_HeapTupleHeaderGetRawCommandId(
                htup_header: super::HeapTupleHeader,
            ) -> super::CommandId;
        }

        if htup_header.is_null() {
            None
        } else {
            Some(unsafe { pgx_HeapTupleHeaderGetRawCommandId(htup_header) })
        }
    }
}

mod internal {
    #[cfg(feature = "pg10")]
    pub use pg10::*;

    #[cfg(feature = "pg11")]
    pub use pg11::*;

    #[cfg(feature = "pg12")]
    pub use pg12::*;

    //
    // for specific versions
    //

    #[cfg(feature = "pg10")]
    mod pg10 {
        pub use crate::pg_sys::common::*;
        pub use crate::pg_sys::pg10_specific::*;

        pub use crate::pg_sys::pg10_specific::AllocSetContextCreate as AllocSetContextCreateExtended;

        pub unsafe fn IndexBuildHeapScan<T>(
            heap_relation: crate::pg_sys::Relation,
            index_relation: crate::pg_sys::Relation,
            index_info: *mut IndexInfo,
            build_callback: crate::pg_sys::IndexBuildCallback,
            build_callback_state: *mut T,
        ) {
            crate::pg_sys::pg10_specific::IndexBuildHeapScan(
                heap_relation,
                index_relation,
                index_info,
                true,
                build_callback,
                build_callback_state as *mut std::os::raw::c_void,
            );
        }
    }

    #[cfg(feature = "pg11")]
    mod pg11 {
        pub use crate::pg_sys::common::*;
        pub use crate::pg_sys::pg11_specific::*;

        pub unsafe fn IndexBuildHeapScan<T>(
            heap_relation: crate::pg_sys::Relation,
            index_relation: crate::pg_sys::Relation,
            index_info: *mut IndexInfo,
            build_callback: crate::pg_sys::IndexBuildCallback,
            build_callback_state: *mut T,
        ) {
            crate::pg_sys::pg11_specific::IndexBuildHeapScan(
                heap_relation,
                index_relation,
                index_info,
                true,
                build_callback,
                build_callback_state as *mut std::os::raw::c_void,
                std::ptr::null_mut(),
            );
        }
    }

    #[cfg(feature = "pg12")]
    mod pg12 {
        pub use crate::pg_sys::common::*;
        pub use crate::pg_sys::pg12_specific::*;

        pub use crate::pg_sys::pg12_specific::AllocSetContextCreateInternal as AllocSetContextCreateExtended;

        pub unsafe fn IndexBuildHeapScan<T>(
            heap_relation: crate::pg_sys::Relation,
            index_relation: crate::pg_sys::Relation,
            index_info: *mut IndexInfo,
            build_callback: crate::pg_sys::IndexBuildCallback,
            build_callback_state: *mut T,
        ) {
            let heap_relation_ref = heap_relation.as_ref().unwrap();
            let table_am = heap_relation_ref.rd_tableam.as_ref().unwrap();

            table_am.index_build_range_scan.unwrap()(
                heap_relation,
                index_relation,
                index_info,
                true,
                false,
                true,
                0,
                crate::pg_sys::InvalidBlockNumber,
                build_callback,
                build_callback_state as *mut std::os::raw::c_void,
                std::ptr::null_mut(),
            );
        }
    }
}

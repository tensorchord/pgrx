use crate::{pg_guard, pg_sys, PgBox, PgList};
use std::ops::Deref;

pub struct HookResult<T> {
    inner: T,
}

impl<T> HookResult<T> {
    pub fn new(value: T) -> Self {
        HookResult { inner: value }
    }
}

impl<T> Deref for HookResult<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait PgHooks {
    /// Hook for plugins to get control in ExecutorStart()
    fn executor_start(
        &mut self,
        query_desc: PgBox<pg_sys::QueryDesc>,
        eflags: i32,
        prev_hook: fn(query_desc: PgBox<pg_sys::QueryDesc>, eflags: i32) -> HookResult<()>,
    ) -> HookResult<()> {
        prev_hook(query_desc, eflags)
    }

    /// Hook for plugins to get control in ExecutorRun()
    fn executor_run(
        &mut self,
        query_desc: PgBox<pg_sys::QueryDesc>,
        direction: pg_sys::ScanDirection,
        count: u64,
        execute_once: bool,
        prev_hook: fn(
            query_desc: PgBox<pg_sys::QueryDesc>,
            direction: pg_sys::ScanDirection,
            count: u64,
            execute_once: bool,
        ) -> HookResult<()>,
    ) -> HookResult<()> {
        prev_hook(query_desc, direction, count, execute_once)
    }

    /// Hook for plugins to get control in ExecutorFinish()
    fn executor_finish(
        &mut self,
        query_desc: PgBox<pg_sys::QueryDesc>,
        prev_hook: fn(query_desc: PgBox<pg_sys::QueryDesc>) -> HookResult<()>,
    ) -> HookResult<()> {
        prev_hook(query_desc)
    }

    /// Hook for plugins to get control in ExecutorEnd()
    fn executor_end(
        &mut self,
        query_desc: PgBox<pg_sys::QueryDesc>,
        prev_hook: fn(query_desc: PgBox<pg_sys::QueryDesc>) -> HookResult<()>,
    ) -> HookResult<()> {
        prev_hook(query_desc)
    }

    /// Hook for plugins to get control in ExecCheckRTPerms()
    fn executor_check_perms(
        &mut self,
        range_table: PgList<*mut pg_sys::RangeTblEntry>,
        ereport_on_violation: bool,
        prev_hook: fn(
            range_table: PgList<*mut pg_sys::RangeTblEntry>,
            ereport_on_violation: bool,
        ) -> HookResult<bool>,
    ) -> HookResult<bool> {
        prev_hook(range_table, ereport_on_violation)
    }

    /// Hook for plugins to get control of the planner
    fn planner(
        &mut self,
        parse: PgBox<pg_sys::Query>,
        cursor_options: i32,
        bound_params: PgBox<pg_sys::ParamListInfoData>,
        prev_hook: fn(
            parse: PgBox<pg_sys::Query>,
            cursor_options: i32,
            bound_params: PgBox<pg_sys::ParamListInfoData>,
        ) -> HookResult<*mut pg_sys::PlannedStmt>,
    ) -> HookResult<*mut pg_sys::PlannedStmt> {
        prev_hook(parse, cursor_options, bound_params)
    }
}

struct Hooks {
    current_hook: Box<&'static mut (dyn PgHooks)>,
    prev_executor_start_hook: pg_sys::ExecutorStart_hook_type,
    prev_executor_run_hook: pg_sys::ExecutorRun_hook_type,
    prev_executor_finish_hook: pg_sys::ExecutorFinish_hook_type,
    prev_executor_end_hook: pg_sys::ExecutorEnd_hook_type,
    prev_executor_check_perms_hook: pg_sys::ExecutorCheckPerms_hook_type,
    prev_planner_hook: pg_sys::planner_hook_type,
}

static mut HOOKS: Option<Hooks> = None;

/// Register a `PgHook` instance to respond to the various hook points
pub unsafe fn register_hook(hook: &'static mut (dyn PgHooks)) {
    if HOOKS.is_some() {
        panic!("PgHook instance already registered");
    }
    HOOKS = Some(Hooks {
        current_hook: Box::new(hook),
        prev_executor_start_hook: pg_sys::ExecutorStart_hook
            .replace(pgx_executor_start)
            .or(Some(pgx_standard_executor_start_wrapper)),
        prev_executor_run_hook: pg_sys::ExecutorRun_hook
            .replace(pgx_executor_run)
            .or(Some(pgx_standard_executor_run_wrapper)),
        prev_executor_finish_hook: pg_sys::ExecutorFinish_hook
            .replace(pgx_executor_finish)
            .or(Some(pgx_standard_executor_finish_wrapper)),
        prev_executor_end_hook: pg_sys::ExecutorEnd_hook
            .replace(pgx_executor_end)
            .or(Some(pgx_standard_executor_end_wrapper)),
        prev_executor_check_perms_hook: pg_sys::ExecutorCheckPerms_hook
            .replace(pgx_executor_check_perms)
            .or(Some(pgx_standard_executor_check_perms_wrapper)),
        prev_planner_hook: pg_sys::planner_hook
            .replace(pgx_planner)
            .or(Some(pgx_standard_planner_wrapper)),
    })
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_start(query_desc: *mut pg_sys::QueryDesc, eflags: i32) {
    fn prev(query_desc: PgBox<pg_sys::QueryDesc>, eflags: i32) -> HookResult<()> {
        HookResult::new(unsafe {
            (HOOKS
                .as_mut()
                .unwrap()
                .prev_executor_start_hook
                .as_ref()
                .unwrap())(query_desc.into_pg(), eflags)
        })
    }
    let hook = &mut HOOKS.as_mut().unwrap().current_hook;
    hook.executor_start(PgBox::from_pg(query_desc), eflags, prev);
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_run(
    query_desc: *mut pg_sys::QueryDesc,
    direction: pg_sys::ScanDirection,
    count: u64,
    execute_once: bool,
) {
    fn prev(
        query_desc: PgBox<pg_sys::QueryDesc>,
        direction: pg_sys::ScanDirection,
        count: u64,
        execute_once: bool,
    ) -> HookResult<()> {
        HookResult::new(unsafe {
            (HOOKS
                .as_mut()
                .unwrap()
                .prev_executor_run_hook
                .as_ref()
                .unwrap())(query_desc.into_pg(), direction, count, execute_once)
        })
    }
    let hook = &mut HOOKS.as_mut().unwrap().current_hook;
    hook.executor_run(
        PgBox::from_pg(query_desc),
        direction,
        count,
        execute_once,
        prev,
    );
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_finish(query_desc: *mut pg_sys::QueryDesc) {
    fn prev(query_desc: PgBox<pg_sys::QueryDesc>) -> HookResult<()> {
        HookResult::new(unsafe {
            (HOOKS
                .as_mut()
                .unwrap()
                .prev_executor_finish_hook
                .as_ref()
                .unwrap())(query_desc.into_pg())
        })
    }
    let hook = &mut HOOKS.as_mut().unwrap().current_hook;
    hook.executor_finish(PgBox::from_pg(query_desc), prev);
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_end(query_desc: *mut pg_sys::QueryDesc) {
    fn prev(query_desc: PgBox<pg_sys::QueryDesc>) -> HookResult<()> {
        HookResult::new(unsafe {
            (HOOKS
                .as_mut()
                .unwrap()
                .prev_executor_end_hook
                .as_ref()
                .unwrap())(query_desc.into_pg())
        })
    }
    let hook = &mut HOOKS.as_mut().unwrap().current_hook;
    hook.executor_end(PgBox::from_pg(query_desc), prev);
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_check_perms(
    range_table: *mut pg_sys::List,
    ereport_on_violation: bool,
) -> bool {
    fn prev(
        range_table: PgList<*mut pg_sys::RangeTblEntry>,
        ereport_on_violation: bool,
    ) -> HookResult<bool> {
        HookResult::new(unsafe {
            (HOOKS
                .as_mut()
                .unwrap()
                .prev_executor_check_perms_hook
                .as_ref()
                .unwrap())(range_table.into_pg(), ereport_on_violation)
        })
    }
    let hook = &mut HOOKS.as_mut().unwrap().current_hook;
    hook.executor_check_perms(PgList::from_pg(range_table), ereport_on_violation, prev)
        .inner
}

#[pg_guard]
unsafe extern "C" fn pgx_planner(
    parse: *mut pg_sys::Query,
    cursor_options: i32,
    bound_params: pg_sys::ParamListInfo,
) -> *mut pg_sys::PlannedStmt {
    fn prev(
        parse: PgBox<pg_sys::Query>,
        cursor_options: i32,
        bound_params: PgBox<pg_sys::ParamListInfoData>,
    ) -> HookResult<*mut pg_sys::PlannedStmt> {
        HookResult::new(unsafe {
            (HOOKS.as_mut().unwrap().prev_planner_hook.as_ref().unwrap())(
                parse.into_pg(),
                cursor_options,
                bound_params.into_pg(),
            )
        })
    }
    let hook = &mut HOOKS.as_mut().unwrap().current_hook;
    hook.planner(
        PgBox::from_pg(parse),
        cursor_options,
        PgBox::from_pg(bound_params),
        prev,
    )
    .inner
}

unsafe extern "C" fn pgx_standard_executor_start_wrapper(
    query_desc: *mut pg_sys::QueryDesc,
    eflags: i32,
) {
    pg_sys::standard_ExecutorStart(query_desc, eflags);
}

unsafe extern "C" fn pgx_standard_executor_run_wrapper(
    query_desc: *mut pg_sys::QueryDesc,
    direction: pg_sys::ScanDirection,
    count: u64,
    execute_once: bool,
) {
    pg_sys::standard_ExecutorRun(query_desc, direction, count, execute_once);
}

unsafe extern "C" fn pgx_standard_executor_finish_wrapper(query_desc: *mut pg_sys::QueryDesc) {
    pg_sys::standard_ExecutorFinish(query_desc);
}

unsafe extern "C" fn pgx_standard_executor_end_wrapper(query_desc: *mut pg_sys::QueryDesc) {
    pg_sys::standard_ExecutorEnd(query_desc);
}

unsafe extern "C" fn pgx_standard_executor_check_perms_wrapper(
    _range_table: *mut pg_sys::List,
    _ereport_on_violation: bool,
) -> bool {
    true
}

unsafe extern "C" fn pgx_standard_planner_wrapper(
    parse: *mut pg_sys::Query,
    cursor_options: i32,
    bound_params: pg_sys::ParamListInfo,
) -> *mut pg_sys::PlannedStmt {
    pg_sys::standard_planner(parse, cursor_options, bound_params)
}

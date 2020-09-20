use crate::lwlock::*;
use crate::{pg_sys, PgAtomic};
use uuid::Uuid;

pub unsafe trait PGXSharedMemory {}

#[macro_export]
macro_rules! pg_shmem_init {
    ($thing:expr) => {
        $thing.pg_init();

        unsafe {
            static mut PREV_SHMEM_STARTUP_HOOK: Option<unsafe extern "C" fn()> = None;
            PREV_SHMEM_STARTUP_HOOK = pg_sys::shmem_startup_hook;
            pg_sys::shmem_startup_hook = Some(shmem_hook);

            #[pg_guard]
            extern "C" fn shmem_hook() {
                unsafe {
                    if let Some(i) = PREV_SHMEM_STARTUP_HOOK {
                        i();
                    }
                }
                $thing.shmem_init();
            }
        }
    };
}
/// A trait that types can implement to provide their own Postgres Shared Memory initialization process
pub trait PgSharedMemoryInitialization {
    /// Automatically called when the an extension is loaded.  If using the `pg_shmem_init!()` macro
    /// in `_PG_init()`, this is called automatically
    fn pg_init(&'static self);

    /// Automatically called by the `pg_shmem_init!()` macro, when Postgres is initializing its
    /// shared memory system
    fn shmem_init(&'static self);
}

impl<T> PgSharedMemoryInitialization for PgLwLock<T>
where
    T: Default + PGXSharedMemory + 'static,
{
    fn pg_init(&'static self) {
        PgSharedMem::pg_init_locked(self);
    }

    fn shmem_init(&'static self) {
        PgSharedMem::shmem_init_locked(self);
    }
}

impl<T> PgSharedMemoryInitialization for PgAtomic<T>
where
    T: atomic_traits::Atomic + Default,
{
    fn pg_init(&'static self) {
        PgSharedMem::pg_init_atomic(self);
    }

    fn shmem_init(&'static self) {
        PgSharedMem::shmem_init_atomic(self);
    }
}

/// This struct contains methods to drive creation of types in shared memory
pub struct PgSharedMem {}

impl PgSharedMem {
    /// Must be run from PG_init, use for types which are guarded by a LWLock
    pub fn pg_init_locked<T: Default + PGXSharedMemory>(lock: &PgLwLock<T>) {
        unsafe {
            let lock = std::ffi::CString::new(lock.get_name()).expect("CString::new failed");
            pg_sys::RequestAddinShmemSpace(std::mem::size_of::<T>());
            pg_sys::RequestNamedLWLockTranche(lock.as_ptr(), 1);
        }
    }

    /// Must be run from PG_init for atomics
    pub fn pg_init_atomic<T: atomic_traits::Atomic + Default>(_atomic: &PgAtomic<T>) {
        unsafe {
            pg_sys::RequestAddinShmemSpace(std::mem::size_of::<T>());
        }
    }

    /// Must be run from the shared memory init hook, use for types which are guarded by a LWLock
    pub fn shmem_init_locked<T: Default + PGXSharedMemory>(lock: &PgLwLock<T>) {
        let mut found = false;
        unsafe {
            let shm_name = std::ffi::CString::new(lock.get_name()).expect("CString::new failed");
            let addin_shmem_init_lock: *mut pg_sys::LWLock =
                &mut (*pg_sys::MainLWLockArray.add(21)).lock;
            pg_sys::LWLockAcquire(addin_shmem_init_lock, pg_sys::LWLockMode_LW_EXCLUSIVE);

            let fv_shmem =
                pg_sys::ShmemInitStruct(shm_name.into_raw(), std::mem::size_of::<T>(), &mut found)
                    as *mut T;

            std::ptr::write(fv_shmem, <T>::default());

            lock.attach(fv_shmem);
            pg_sys::LWLockRelease(addin_shmem_init_lock);
        }
    }

    pub fn shmem_init_atomic<T: atomic_traits::Atomic + Default>(atomic: &PgAtomic<T>) {
        unsafe {
            let shm_name =
                std::ffi::CString::new(Uuid::new_v4().to_string()).expect("CString::new() failed");

            let addin_shmem_init_lock: *mut pg_sys::LWLock =
                &mut (*pg_sys::MainLWLockArray.add(21)).lock;

            let mut found = false;
            pg_sys::LWLockAcquire(addin_shmem_init_lock, pg_sys::LWLockMode_LW_EXCLUSIVE);
            let fv_shmem =
                pg_sys::ShmemInitStruct(shm_name.into_raw(), std::mem::size_of::<T>(), &mut found)
                    as *mut T;

            atomic.attach(fv_shmem);
            let atomic = T::default();
            std::ptr::copy(&atomic, fv_shmem, 1);
            pg_sys::LWLockRelease(addin_shmem_init_lock);
        }
    }
}

unsafe impl PGXSharedMemory for bool {}
unsafe impl PGXSharedMemory for char {}
unsafe impl PGXSharedMemory for str {}
unsafe impl PGXSharedMemory for () {}
unsafe impl PGXSharedMemory for i8 {}
unsafe impl PGXSharedMemory for i16 {}
unsafe impl PGXSharedMemory for i32 {}
unsafe impl PGXSharedMemory for i64 {}
unsafe impl PGXSharedMemory for i128 {}
unsafe impl PGXSharedMemory for u8 {}
unsafe impl PGXSharedMemory for u16 {}
unsafe impl PGXSharedMemory for u32 {}
unsafe impl PGXSharedMemory for u64 {}
unsafe impl PGXSharedMemory for u128 {}
unsafe impl PGXSharedMemory for usize {}
unsafe impl PGXSharedMemory for isize {}
unsafe impl PGXSharedMemory for f32 {}
unsafe impl PGXSharedMemory for f64 {}
unsafe impl<T> PGXSharedMemory for [T] where T: PGXSharedMemory + Default {}
unsafe impl<A, B> PGXSharedMemory for (A, B)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
{
}
unsafe impl<A, B, C> PGXSharedMemory for (A, B, C)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
    C: PGXSharedMemory + Default,
{
}
unsafe impl<A, B, C, D> PGXSharedMemory for (A, B, C, D)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
    C: PGXSharedMemory + Default,
    D: PGXSharedMemory + Default,
{
}
unsafe impl<A, B, C, D, E> PGXSharedMemory for (A, B, C, D, E)
where
    A: PGXSharedMemory + Default,
    B: PGXSharedMemory + Default,
    C: PGXSharedMemory + Default,
    D: PGXSharedMemory + Default,
    E: PGXSharedMemory + Default,
{
}
unsafe impl<N: Default + PGXSharedMemory, T: heapless::ArrayLength<N>> PGXSharedMemory
    for heapless::Vec<N, T>
{
}
unsafe impl<
        K: Eq + hash32::Hash,
        V: Default,
        N: heapless::ArrayLength<heapless::Bucket<K, V>>
            + heapless::ArrayLength<Option<heapless::Pos>>,
        S,
    > PGXSharedMemory for heapless::IndexMap<K, V, N, S>
{
}

//! Host->guest callback identifiers for reviewed Core callouts.
//!
//! Synchronous callouts use `SyncCallback`. Retained UI listeners additionally
//! carry a destroy callback ID so the engine can release guest-side state when
//! the native listener/resource is detached.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncCallback {
    pub id: u32,
    pub user_data: u32,
}

impl SyncCallback {
    pub const fn new(id: u32, user_data: u32) -> Self {
        Self { id, user_data }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetainedCallback {
    pub id: u32,
    pub user_data: u32,
    pub destroy_id: u32,
}

impl RetainedCallback {
    pub const fn new(id: u32, user_data: u32, destroy_id: u32) -> Self {
        Self {
            id,
            user_data,
            destroy_id,
        }
    }
}

use core::sync::atomic::{AtomicPtr, Ordering};

pub const SYNC_CLOSURE_CALLBACK_ID: u32 = 0x4342_5359; // 'CBSY'

struct SyncClosureNode {
    invoke: unsafe fn(*mut ()),
    data: *mut (),
    /// Set while `invoke` is executing, so a re-entrant dispatch targeting this
    /// same node declines instead of aliasing the closure (see
    /// `dispatch_sync_closure`).
    running: bool,
}

/// Top of the active `with_sync_closure` stack.
///
/// This is a plain `static`, not a thread-local: the guest is single-threaded
/// wasm32. If the guest ever gains threads this must become thread-local, since
/// two threads would otherwise share one stack of nodes.
static CURRENT_SYNC_NODE: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());

unsafe fn sync_trampoline<F: FnMut()>(data: *mut ()) {
    // SAFETY: data points to an `F` alive on the caller's stack in `with_sync_closure`.
    let f = unsafe { &mut *(data as *mut F) };
    f();
}

/// Run a closure synchronously across an engine callback.
///
/// Registers the closure on the stack for the duration of `run` and supplies
/// a sentinel `SyncCallback` targeting `SYNC_CLOSURE_CALLBACK_ID`.
pub fn with_sync_closure<F: FnMut(), R>(mut f: F, run: impl FnOnce(SyncCallback) -> R) -> R {
    let prev = CURRENT_SYNC_NODE.load(Ordering::Acquire);
    let mut node = SyncClosureNode {
        invoke: sync_trampoline::<F>,
        data: &mut f as *mut F as *mut (),
        running: false,
    };
    CURRENT_SYNC_NODE.store(
        &mut node as *mut SyncClosureNode as *mut (),
        Ordering::Release,
    );

    struct ResetGuard(*mut ());
    impl Drop for ResetGuard {
        fn drop(&mut self) {
            CURRENT_SYNC_NODE.store(self.0, Ordering::Release);
        }
    }
    let _guard = ResetGuard(prev);
    run(SyncCallback::new(SYNC_CLOSURE_CALLBACK_ID, 0))
}

/// Invoked by `spring:callback/dispatch` when `callback_id == SYNC_CLOSURE_CALLBACK_ID`.
///
/// Returns `false` when there is no active closure, or when the top closure is
/// already running and the host has re-entered it: invoking it again would
/// create a second `&mut F` aliasing the first.
#[inline]
pub fn dispatch_sync_closure() -> bool {
    let ptr = CURRENT_SYNC_NODE.load(Ordering::Acquire);
    if ptr.is_null() {
        return false;
    }
    let node = ptr as *mut SyncClosureNode;
    // SAFETY: The node is alive on the stack frame of `with_sync_closure` which
    // invoked the host call that synchronously entered this dispatch. Fields are
    // read through raw pointers so that no reference to the node is held across
    // `invoke`, which may re-enter this function.
    unsafe {
        if core::ptr::addr_of!((*node).running).read() {
            return false;
        }
        core::ptr::addr_of_mut!((*node).running).write(true);
        let invoke = core::ptr::addr_of!((*node).invoke).read();
        let data = core::ptr::addr_of!((*node).data).read();
        invoke(data);
        core::ptr::addr_of_mut!((*node).running).write(false);
    }
    true
}

pub trait SyncHandler<R = ()> {
    fn run_sync(self, execute: impl FnOnce(SyncCallback) -> crate::Result<R>) -> crate::Result<R>;
}

impl<R> SyncHandler<R> for SyncCallback {
    #[inline]
    fn run_sync(self, execute: impl FnOnce(SyncCallback) -> crate::Result<R>) -> crate::Result<R> {
        execute(self)
    }
}

impl<F: FnMut(), R> SyncHandler<R> for F {
    #[inline]
    fn run_sync(self, execute: impl FnOnce(SyncCallback) -> crate::Result<R>) -> crate::Result<R> {
        with_sync_closure(self, execute)
    }
}

/// Export the single Core callback dispatcher expected by reviewed callback
/// imports. The handler receives the callback ID and opaque 32-bit user data.
/// It may call Spring imports recursively; the host callback/re-entry budget
/// decides whether that nested import is allowed.
#[macro_export]
macro_rules! export_callback_dispatch {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:callback/dispatch")]
        pub extern "C" fn __spring_callback_dispatch(callback_id: i32, user_data: i32) {
            let id = callback_id as u32;
            if id == $crate::callback::SYNC_CLOSURE_CALLBACK_ID {
                if $crate::callback::dispatch_sync_closure() {
                    return;
                }
            }
            $handler(id, user_data as u32)
        }
    };
}

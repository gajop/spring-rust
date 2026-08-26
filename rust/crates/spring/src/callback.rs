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
            $handler(callback_id as u32, user_data as u32)
        }
    };
}

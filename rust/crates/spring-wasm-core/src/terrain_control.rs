//! Reviewed synchronous terrain-control callback imports.

use super::{ApiError, ErrorCode, Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:terrain-control")]
    extern "C" {
        #[link_name = "set-height-map-func"]
        pub fn set_height_map_func(callback_id: i32, user_data: i32) -> i64;
        #[link_name = "set-original-height-map-func"]
        pub fn set_original_height_map_func(callback_id: i32, user_data: i32) -> i64;
        #[link_name = "set-smooth-mesh-func"]
        pub fn set_smooth_mesh_func(callback_id: i32, user_data: i32) -> i64;
    }
}

macro_rules! sync_callback_import {
    ($name:ident, $raw:ident) => {
        #[inline]
        pub fn $name(callback: SyncCallback) -> Result<bool> {
            #[cfg(target_arch = "wasm32")]
            {
                return super::unpack_bool(unsafe {
                    raw::$raw(callback.id as i32, callback.user_data as i32)
                });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = callback;
                Err(unreachable!())
            }
        }
    };
}

sync_callback_import!(set_height_map_func, set_height_map_func);
sync_callback_import!(set_original_height_map_func, set_original_height_map_func);
sync_callback_import!(set_smooth_mesh_func, set_smooth_mesh_func);

//! Reviewed synchronous terrain-control callback imports.

#[cfg(feature = "alloc")]
pub use crate::owned::terrain_control::{
    add_grass, add_height_map, add_original_height_map, add_smooth_mesh, adjust_height_map,
    adjust_original_height_map, adjust_smooth_mesh, level_original_height_map, level_smooth_mesh,
    rebuild_smooth_mesh, remove_grass, revert_height_map, revert_original_height_map,
    revert_smooth_mesh, set_map_square_terrain_type, set_original_height_map, set_smooth_mesh,
    set_terrain_type_data, set_tidal, set_wind,
};

use super::{Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:terrain-control")]
    unsafe extern "C" {
        #[link_name = "set-height-map-func"]
        pub safe fn set_height_map_func(callback_id: i32, user_data: i32) -> i64;
        #[link_name = "set-original-height-map-func"]
        pub safe fn set_original_height_map_func(callback_id: i32, user_data: i32) -> i64;
        #[link_name = "set-smooth-mesh-func"]
        pub safe fn set_smooth_mesh_func(callback_id: i32, user_data: i32) -> i64;
    }
}

macro_rules! sync_callback_import {
    ($name:ident, $raw:ident) => {
        #[inline]
        pub fn $name(callback: SyncCallback) -> Result<bool> {
            #[cfg(target_arch = "wasm32")]
            {
                return super::unpack_bool(raw::$raw(
                    callback.id as i32,
                    callback.user_data as i32,
                ));
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

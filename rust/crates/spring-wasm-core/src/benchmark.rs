// Benchmark-critical Core-Wasm guest wrappers. Timer/message functions are
// instrumentation used by the shared benchmark suite; RulesParams, Terrain,
// UnitControl and Gfx are ordinary semantic API calls with specialized lowering.

#[path = "unit_control.rs"]
mod unit_control;
pub use unit_control::*;

use super::{ApiError, ErrorCode, Result};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:profiling")]
    extern "C" {
        #[link_name = "get-timer-micros"]
        pub fn get_timer_micros() -> i64;
    }

    #[link(wasm_import_module = "spring:messages")]
    extern "C" {
        #[link_name = "send-lua-rules-msg"]
        pub fn send_lua_rules_msg(pointer: i32, length: i32) -> i64;
        #[link_name = "send-lua-ui-msg"]
        pub fn send_lua_ui_msg(
            message_pointer: i32,
            message_length: i32,
            mode_pointer: i32,
            mode_length: i32,
        ) -> i64;
    }

    #[link(wasm_import_module = "spring:rules-params")]
    extern "C" {
        #[link_name = "set-unit-rules-param-f32"]
        pub fn set_unit_rules_param_f32(
            unit_id: i32,
            name_pointer: i32,
            name_length: i32,
            value: f32,
            los: i32,
        ) -> i64;
        #[link_name = "get-unit-rules-param-f32"]
        pub fn get_unit_rules_param_f32(
            unit_id: i32,
            name_pointer: i32,
            name_length: i32,
        ) -> i64;
    }

    #[link(wasm_import_module = "spring:terrain")]
    extern "C" {
        #[link_name = "get-ground-orig-height"]
        pub fn get_ground_orig_height(x: f32, z: f32) -> i64;
    }

    #[link(wasm_import_module = "spring:terrain-control")]
    extern "C" {
        #[link_name = "set-height-map"]
        pub fn set_height_map(x: f32, z: f32, height: f32, terraform: f32) -> i64;
        #[link_name = "level-height-map"]
        pub fn level_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> i64;
        #[link_name = "set-height-map-func"]
        pub fn set_height_map_func(callback_id: i32, user_data: i32) -> i64;
    }

    #[link(wasm_import_module = "spring:gfx")]
    extern "C" {
        #[link_name = "vertex"]
        pub fn gfx_vertex(x: f32, y: f32, z: f32, w: f32, count: i32) -> i32;
        #[link_name = "begin-end"]
        pub fn gfx_begin_end(primitive: i32, callback_id: i32, user_data: i32) -> i32;
    }
}

#[inline]
fn bytes_parts(value: &[u8]) -> (i32, i32) {
    if value.is_empty() {
        return (0, 0);
    }
    let pointer = value.as_ptr() as usize;
    debug_assert!(pointer <= u32::MAX as usize);
    debug_assert!(value.len() <= u32::MAX as usize);
    (pointer as u32 as i32, value.len() as u32 as i32)
}

#[inline]
fn unpack_bool_local(packed: i64) -> Result<bool> {
    let packed = packed as u64;
    let status = (packed >> 32) as u32 as i32;
    if status != 0 {
        return Err(ApiError::new(status));
    }
    match packed as u32 {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ApiError::new(ErrorCode::Internal as i32)),
    }
}

#[inline]
fn unpack_f32_local(packed: i64) -> Result<f32> {
    let packed = packed as u64;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        Ok(f32::from_bits(packed as u32))
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn status_result(status: i32) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
pub fn get_timer_micros() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        return Ok(unsafe { raw::get_timer_micros() } as u64);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn send_lua_rules_msg(message: &str) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = bytes_parts(message.as_bytes());
        return unpack_bool_local(unsafe { raw::send_lua_rules_msg(pointer, length) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn send_lua_ui_msg(message: &str, mode: &str) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (message_pointer, message_length) = bytes_parts(message.as_bytes());
        let (mode_pointer, mode_length) = bytes_parts(mode.as_bytes());
        return unpack_bool_local(unsafe {
            raw::send_lua_ui_msg(
                message_pointer,
                message_length,
                mode_pointer,
                mode_length,
            )
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (message, mode);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn set_unit_rules_param_f32(
    unit_id: i32,
    name: &str,
    value: f32,
    los: i32,
) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (name_pointer, name_length) = bytes_parts(name.as_bytes());
        return unpack_bool_local(unsafe {
            raw::set_unit_rules_param_f32(unit_id, name_pointer, name_length, value, los)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, name, value, los);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn get_unit_rules_param_f32(unit_id: i32, name: &str) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        let (name_pointer, name_length) = bytes_parts(name.as_bytes());
        return unpack_f32_local(unsafe {
            raw::get_unit_rules_param_f32(unit_id, name_pointer, name_length)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, name);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn get_ground_orig_height(x: f32, z: f32) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_f32_local(unsafe { raw::get_ground_orig_height(x, z) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn set_height_map(x: f32, z: f32, height: f32, terraform: f32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_bool_local(unsafe { raw::set_height_map(x, z, height, terraform) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z, height, terraform);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn level_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_bool_local(unsafe { raw::level_height_map(x1, z1, x2, z2, height) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x1, z1, x2, z2, height);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn set_height_map_func(callback_id: u32, user_data: u32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_bool_local(unsafe {
            raw::set_height_map_func(callback_id as i32, user_data as i32)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (callback_id, user_data);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn gfx_vertex(x: f32, y: f32, z: f32, w: f32, count: u32) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        return status_result(unsafe { raw::gfx_vertex(x, y, z, w, count as i32) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, y, z, w, count);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn gfx_begin_end(primitive: u32, callback_id: u32, user_data: u32) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        return status_result(unsafe {
            raw::gfx_begin_end(primitive as i32, callback_id as i32, user_data as i32)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (primitive, callback_id, user_data);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

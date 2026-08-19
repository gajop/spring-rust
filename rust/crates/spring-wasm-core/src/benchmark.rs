// Benchmark-critical Core-Wasm guest wrappers.  Timer/message functions are
// instrumentation used by the shared benchmark suite; RulesParams and Terrain
// are ordinary semantic API calls with a specialized fast lowering.

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

/// Monotonic engine timer used by the benchmark suite. Host-side NativeInterface
/// errors trap because there is no meaningful benchmark result after timer
/// failure.
#[inline]
pub fn get_timer_micros() -> Result<u64> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY: generated zero-argument numeric Core signature.
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
        // SAFETY: the string bytes remain live for the synchronous import and
        // the host validates the complete range before constructing its C string.
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
        // SAFETY: both byte slices remain live for the synchronous import.
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
        // SAFETY: name is copied/validated by the host before the mutating
        // NativeInterface call is made.
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
        // SAFETY: name remains live for the synchronous import.
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
        // SAFETY: generated scalar-only Core signature.
        return unpack_f32_local(unsafe { raw::get_ground_orig_height(x, z) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

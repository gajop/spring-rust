// RulesParams portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result, unpack_bool, unpack_f32};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:rules-params")]
    unsafe extern "C" {
        #[link_name = "get-unit-rules-param-f32"]
        pub fn get_unit_rules_param_f32(unit_id: i32, name: i32, name_len: i32) -> i64;
        #[link_name = "set-unit-rules-param-f32"]
        pub fn set_unit_rules_param_f32(
            unit_id: i32,
            name: i32,
            name_len: i32,
            value: f32,
            los: i32,
        ) -> i64;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn string_parts(value: &str) -> Result<(i32, i32)> {
    if value.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    if value.is_empty() {
        return Ok((0, 0));
    }
    let pointer = value.as_ptr() as usize;
    if pointer > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok((pointer as u32 as i32, value.len() as u32 as i32))
}

#[inline]
pub fn get_unit_rules_param_f32(unit_id: i32, name: &str) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = string_parts(name)?;
        unpack_f32(unsafe { raw::get_unit_rules_param_f32(unit_id, pointer, length) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, name);
        Err(unreachable!())
    }
}

#[inline]
pub fn set_unit_rules_param_f32(unit_id: i32, name: &str, value: f32, los: i32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = string_parts(name)?;
        unpack_bool(unsafe { raw::set_unit_rules_param_f32(unit_id, pointer, length, value, los) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, name, value, los);
        Err(unreachable!())
    }
}

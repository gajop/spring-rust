// UnitControl portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:unit-control")]
    unsafe extern "C" {
        #[link_name = "give-order-to-unit"]
        pub fn give_order_to_unit(
            unit_id: i32,
            cmd_id: i32,
            params_pointer: i32,
            param_count: i32,
            options: i32,
            timeout: i32,
        ) -> i64;
    }
}

#[inline]
pub fn give_order_to_unit(
    unit_id: i32,
    cmd_id: i32,
    params: &[f32],
    options: u32,
    timeout: i32,
) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        if params.len() > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let (pointer, count) = if params.is_empty() {
            (0, 0)
        } else {
            let pointer = params.as_ptr() as usize;
            debug_assert!(pointer <= u32::MAX as usize);
            (pointer as u32 as i32, params.len() as u32 as i32)
        };
        let packed = unsafe {
            raw::give_order_to_unit(unit_id, cmd_id, pointer, count, options as i32, timeout)
        } as u64;
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, cmd_id, params, options, timeout);
        Err(unreachable!())
    }
}

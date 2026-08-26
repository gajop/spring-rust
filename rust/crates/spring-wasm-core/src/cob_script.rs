//! Reviewed synced COB-script transport.
//!
//! Return storage is supplied up front so a mutating COB call is never retried
//! merely to discover output capacity.

use super::{ApiError, ErrorCode, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CobCallResult {
    pub ret_code: i32,
    pub ret_count: usize,
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:cob-script")]
    unsafe extern "C" {
        #[link_name = "call-cob-script"]
        pub fn call_cob_script(
            unit_id: i32,
            func_id: i32,
            func_name_ptr: i32,
            func_name_len: i32,
            ret_args: i32,
            args_ptr: i32,
            arg_count: i32,
            ret_values_ptr: i32,
            ret_values_capacity: i32,
            ret_count_ptr: i32,
        ) -> i64;
    }
}

#[inline]
pub fn call_cob_script(
    unit_id: i32,
    func_id: i32,
    func_name: &str,
    ret_args: u32,
    args: &[i32],
    ret_values: &mut [i32],
) -> Result<CobCallResult> {
    #[cfg(target_arch = "wasm32")]
    {
        if ret_values.len() < ret_args as usize
            || func_name.len() > u32::MAX as usize
            || args.len() > u32::MAX as usize
            || ret_values.len() > u32::MAX as usize
        {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }

        let name_ptr = func_name.as_ptr() as usize;
        let args_ptr = args.as_ptr() as usize;
        let ret_values_ptr = ret_values.as_mut_ptr() as usize;
        let mut ret_count = 0u32;
        let ret_count_ptr = (&mut ret_count as *mut u32) as usize;
        if name_ptr > u32::MAX as usize
            || args_ptr > u32::MAX as usize
            || ret_values_ptr > u32::MAX as usize
            || ret_count_ptr > u32::MAX as usize
        {
            return Err(ApiError::new(ErrorCode::OutOfBounds as i32));
        }

        let packed = unsafe {
            raw::call_cob_script(
                unit_id,
                func_id,
                name_ptr as u32 as i32,
                func_name.len() as u32 as i32,
                ret_args as i32,
                args_ptr as u32 as i32,
                args.len() as u32 as i32,
                ret_values_ptr as u32 as i32,
                ret_values.len() as u32 as i32,
                ret_count_ptr as u32 as i32,
            )
        };
        let ret_code = super::unpack_i32(packed)?;
        if ret_count as usize > ret_values.len() {
            return Err(ApiError::new(ErrorCode::Internal as i32));
        }
        Ok(CobCallResult {
            ret_code,
            ret_count: ret_count as usize,
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, func_id, func_name, ret_args, args, ret_values);
        Err(unreachable!())
    }
}

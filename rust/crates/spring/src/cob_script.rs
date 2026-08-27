//! Reviewed synced COB-script transport.
//!
//! Return storage is supplied up front so a mutating COB call is never retried
//! merely to discover output capacity.

#[cfg(feature = "alloc")]
pub use crate::owned::cob_script::get_cob_script_id;

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
        pub safe fn call_cob_script(
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
        if ret_values.len() < ret_args as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }

        let (name_ptr, name_len) = super::wasm_slice_parts(func_name.as_bytes())?;
        let (args_ptr, args_len) = super::wasm_slice_parts(args)?;
        let (ret_values_ptr, ret_capacity) = super::wasm_mut_slice_parts(ret_values)?;
        let mut ret_count = 0u32;
        let ret_count_ptr = super::wasm_output_ptr(&mut ret_count)?;

        let packed = raw::call_cob_script(
            unit_id,
            func_id,
            name_ptr,
            name_len,
            ret_args as i32,
            args_ptr,
            args_len,
            ret_values_ptr,
            ret_capacity,
            ret_count_ptr,
        );
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

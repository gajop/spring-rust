#[cfg(feature = "alloc")]
pub use crate::owned::unit_defs::{
    get_unit_def_by_id, get_unit_def_classify, get_unit_def_costs, get_unit_def_count,
    get_unit_def_custom_param, get_unit_def_custom_param_keys, get_unit_def_health,
    get_unit_def_i_ds, get_unit_def_id_by_name, get_unit_def_param_bool, get_unit_def_param_float,
    get_unit_def_param_int, get_unit_def_param_keys, get_unit_def_param_string,
    get_unit_def_param_type, get_unit_def_speed, valid_unit_def_id,
};

// UnitDefs byte-string portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, DefId, ErrorCode, Result};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteBufferFill {
    Complete(usize),
    Insufficient { required: usize },
}

#[cfg(target_arch = "wasm32")]
mod unit_defs_raw {
    #[link(wasm_import_module = "spring:unit-defs")]
    unsafe extern "C" {
        #[link_name = "get-unit-def-name"]
        pub safe fn get_unit_def_name(unit_def_id: i32, output: i32, capacity: i32) -> i64;
        #[link_name = "get-unit-def-human-name"]
        pub safe fn get_unit_def_human_name(unit_def_id: i32, output: i32, capacity: i32) -> i64;
    }
}

#[inline]
fn decode_fill(packed: i64, capacity: usize) -> Result<ByteBufferFill> {
    let packed = packed as u64;
    let required = packed as u32 as usize;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        if required > capacity {
            Err(ApiError::new(ErrorCode::Internal as i32))
        } else {
            Ok(ByteBufferFill::Complete(required))
        }
    } else if status == ErrorCode::BufferOverflow as i32 {
        Ok(ByteBufferFill::Insufficient { required })
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn output_parts(output: &mut [u8]) -> Result<(i32, i32)> {
    super::wasm_mut_slice_parts(output)
}

#[inline]
pub fn get_unit_def_name_into(
    unit_def_id: impl Into<DefId>,
    output: &mut [u8],
) -> Result<ByteBufferFill> {
    let unit_def_id = unit_def_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output)?;
        decode_fill(
            unit_defs_raw::get_unit_def_name(unit_def_id.0, pointer, capacity),
            output.len(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_def_id, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_def_human_name_into(
    unit_def_id: impl Into<DefId>,
    output: &mut [u8],
) -> Result<ByteBufferFill> {
    let unit_def_id = unit_def_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output)?;
        decode_fill(
            unit_defs_raw::get_unit_def_human_name(unit_def_id.0, pointer, capacity),
            output.len(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_def_id, output);
        Err(unreachable!())
    }
}

#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
fn collect_bytes(mut call: impl FnMut(i32, i32) -> i64) -> Result<Vec<u8>> {
    let first = decode_fill(call(0, 0), 0)?;
    let required = match first {
        ByteBufferFill::Complete(0) => return Ok(Vec::new()),
        ByteBufferFill::Complete(count) | ByteBufferFill::Insufficient { required: count } => count,
    };
    let mut output = vec![0u8; required];
    for _ in 0..3 {
        let (pointer, capacity) = super::wasm_mut_slice_parts(&mut output)?;
        let fill = decode_fill(call(pointer, capacity), output.len())?;
        match fill {
            ByteBufferFill::Complete(count) => {
                output.truncate(count);
                return Ok(output);
            }
            ByteBufferFill::Insufficient { required } => output.resize(required, 0),
        }
    }
    Err(ApiError::new(ErrorCode::BufferOverflow as i32))
}

#[cfg(feature = "alloc")]
pub fn get_unit_def_name_bytes(unit_def_id: impl Into<DefId>) -> Result<Vec<u8>> {
    let unit_def_id = unit_def_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        collect_bytes(|pointer, capacity| {
            unit_defs_raw::get_unit_def_name(unit_def_id.0, pointer, capacity)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_def_id;
        Err(unreachable!())
    }
}

#[cfg(feature = "alloc")]
pub fn get_unit_def_human_name_bytes(unit_def_id: impl Into<DefId>) -> Result<Vec<u8>> {
    let unit_def_id = unit_def_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        collect_bytes(|pointer, capacity| {
            unit_defs_raw::get_unit_def_human_name(unit_def_id.0, pointer, capacity)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_def_id;
        Err(unreachable!())
    }
}

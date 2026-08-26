// UnitDefs byte-string portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result};

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
        pub fn get_unit_def_name(unit_def_id: i32, output: i32, capacity: i32) -> i64;
        #[link_name = "get-unit-def-human-name"]
        pub fn get_unit_def_human_name(unit_def_id: i32, output: i32, capacity: i32) -> i64;
    }
}

#[inline]
fn decode_fill(packed: i64) -> Result<ByteBufferFill> {
    let packed = packed as u64;
    let required = packed as u32 as usize;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        Ok(ByteBufferFill::Complete(required))
    } else if status == ErrorCode::BufferOverflow as i32 {
        Ok(ByteBufferFill::Insufficient { required })
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn output_parts(output: &mut [u8]) -> (i32, i32) {
    if output.is_empty() {
        return (0, 0);
    }
    let pointer = output.as_mut_ptr() as usize;
    debug_assert!(pointer <= u32::MAX as usize);
    debug_assert!(output.len() <= u32::MAX as usize);
    (pointer as u32 as i32, output.len() as u32 as i32)
}

#[inline]
pub fn get_unit_def_name_into(unit_def_id: i32, output: &mut [u8]) -> Result<ByteBufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: the output slice remains live for the synchronous import;
        // host validates pointer/capacity and never writes a partial result.
        decode_fill(unsafe { unit_defs_raw::get_unit_def_name(unit_def_id, pointer, capacity) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_def_id, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_def_human_name_into(unit_def_id: i32, output: &mut [u8]) -> Result<ByteBufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = output_parts(output);
        // SAFETY: same validated caller-owned byte-buffer convention as name.
        decode_fill(unsafe {
            unit_defs_raw::get_unit_def_human_name(unit_def_id, pointer, capacity)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_def_id, output);
        Err(unreachable!())
    }
}

#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
fn collect_bytes(mut call: impl FnMut(i32, i32) -> i64) -> Result<Vec<u8>> {
    let first = decode_fill(call(0, 0))?;
    let required = match first {
        ByteBufferFill::Complete(0) => return Ok(Vec::new()),
        ByteBufferFill::Complete(count) | ByteBufferFill::Insufficient { required: count } => count,
    };
    let mut output = vec![0u8; required];
    for _ in 0..3 {
        let pointer = output.as_mut_ptr() as usize;
        debug_assert!(pointer <= u32::MAX as usize);
        let fill = decode_fill(call(pointer as u32 as i32, output.len() as u32 as i32))?;
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
pub fn get_unit_def_name_bytes(unit_def_id: i32) -> Result<Vec<u8>> {
    #[cfg(target_arch = "wasm32")]
    {
        collect_bytes(|pointer, capacity| unsafe {
            unit_defs_raw::get_unit_def_name(unit_def_id, pointer, capacity)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_def_id;
        Err(unreachable!())
    }
}

#[cfg(feature = "alloc")]
pub fn get_unit_def_human_name_bytes(unit_def_id: i32) -> Result<Vec<u8>> {
    #[cfg(target_arch = "wasm32")]
    {
        collect_bytes(|pointer, capacity| unsafe {
            unit_defs_raw::get_unit_def_human_name(unit_def_id, pointer, capacity)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_def_id;
        Err(unreachable!())
    }
}

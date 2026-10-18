//! Safe typed access to start-script mod options.

extern crate alloc;

use alloc::{string::String, vec, vec::Vec};

use crate::{ApiError, ErrorCode, Result};

/// Read one mod option without exposing the Core variable-I/O descriptors.
/// `None` means the option was not present in the start script.
#[cfg(target_arch = "wasm32")]
pub fn get_mod_option(key: &str) -> Result<Option<String>> {
    let mut input_wire = Vec::with_capacity(4 + key.len());
    input_wire.extend_from_slice(&(key.len() as u32).to_le_bytes());
    input_wire.extend_from_slice(key.as_bytes());
    let (input_pointer, input_length) = crate::wasm_slice_parts(&input_wire)?;
    let mut input = [input_pointer as u32, input_length as u32];
    let input_descriptor = crate::wasm_output_ptr(&mut input)?;
    let mut value = vec![0u8; 64];

    loop {
        let (value_pointer, value_capacity) = crate::wasm_mut_slice_parts(&mut value)?;
        let mut output = [value_pointer as u32, value_capacity as u32, 0, 0];
        let output_descriptor = crate::wasm_output_ptr(&mut output)?;
        let status =
            crate::generated::owned::game::get_mod_option(input_descriptor, output_descriptor);
        let required = output[2] as usize;
        if status == ErrorCode::BufferOverflow as i32 {
            value.resize(required, 0);
            continue;
        }
        if status != 0 {
            return Err(ApiError::new(status));
        }
        if output[3] == 0 {
            return Ok(None);
        }
        value.truncate(required);
        return String::from_utf8(value)
            .map(Some)
            .map_err(|_| ApiError::new(ErrorCode::Internal as i32));
    }
}

/// Read all mod option keys.
#[cfg(target_arch = "wasm32")]
pub fn get_mod_options() -> Result<Vec<String>> {
    crate::generated::owned::game::get_mod_options(0)
}

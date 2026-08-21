// UnitsCommands portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandBufferFill {
    Complete(usize),
    Insufficient { required: usize },
}

#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq)]
pub struct UnitCommand {
    pub cmd_id: i32,
    pub options: u8,
    pub tag: i32,
    pub ai_command_id: i32,
    pub timeout: f32,
    pub params: Vec<f32>,
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:units-commands")]
    extern "C" {
        #[link_name = "get-unit-command-count"]
        pub fn get_unit_command_count(unit_id: i32) -> i64;
        #[link_name = "get-unit-commands"]
        pub fn get_unit_commands(
            unit_id: i32,
            max_commands: i32,
            output: i32,
            capacity_bytes: i32,
        ) -> i64;
        #[link_name = "give-order"]
        pub fn give_order(
            cmd_id: i32,
            params: i32,
            param_count: i32,
            options: i32,
            timeout: i32,
        ) -> i64;
        #[link_name = "give-order-to-unit-map"]
        pub fn give_order_to_unit_map(
            unit_ids: i32,
            unit_count: i32,
            cmd_id: i32,
            params: i32,
            param_count: i32,
            options: i32,
            timeout: i32,
        ) -> i64;
    }
}

#[inline]
fn decode_fill(packed: i64) -> Result<CommandBufferFill> {
    let packed = packed as u64;
    let bytes = packed as u32 as usize;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        Ok(CommandBufferFill::Complete(bytes))
    } else if status == ErrorCode::BufferOverflow as i32 {
        Ok(CommandBufferFill::Insufficient { required: bytes })
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn decode_i32(packed: i64) -> Result<i32> {
    let packed = packed as u64;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        Ok(packed as u32 as i32)
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn slice_parts<T>(values: &[T]) -> (i32, i32) {
    if values.is_empty() {
        return (0, 0);
    }
    let pointer = values.as_ptr() as usize;
    debug_assert!(pointer <= u32::MAX as usize);
    debug_assert!(values.len() <= u32::MAX as usize);
    (pointer as u32 as i32, values.len() as u32 as i32)
}

#[inline]
pub fn get_unit_command_count(unit_id: i32) -> Result<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY: generated scalar-only signature.
        let packed = unsafe { raw::get_unit_command_count(unit_id) } as u64;
        let status = (packed >> 32) as u32 as i32;
        return if status == 0 {
            Ok(packed as u32)
        } else {
            Err(ApiError::new(status))
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn get_unit_commands_into(
    unit_id: i32,
    max_commands: u32,
    output: &mut [u8],
) -> Result<CommandBufferFill> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = if output.is_empty() {
            (0, 0)
        } else {
            let pointer = output.as_mut_ptr() as usize;
            debug_assert!(pointer <= u32::MAX as usize);
            debug_assert!(output.len() <= u32::MAX as usize);
            (pointer as u32 as i32, output.len() as u32 as i32)
        };
        // SAFETY: host validates the complete output byte range once and writes
        // the entire nested command representation synchronously.
        return decode_fill(unsafe {
            raw::get_unit_commands(unit_id, max_commands as i32, pointer, capacity)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, max_commands, output);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

/// Issue one command through the UnitsCommands API. The host borrows `params`
/// directly from fixed synced Core memory for the duration of the call.
#[inline]
pub fn give_order(cmd_id: i32, params: &[f32], options: u32, timeout: i32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (params_ptr, param_count) = slice_parts(params);
        // SAFETY: `params` is a properly aligned Rust f32 slice and the host
        // validates the full range before borrowing it synchronously.
        return decode_i32(unsafe {
            raw::give_order(cmd_id, params_ptr, param_count, options as i32, timeout)
        })
        .map(|value| value != 0);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (cmd_id, params, options, timeout);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

/// Issue one command to an explicit set of unit IDs. Both input slices are
/// borrowed directly by the host; no guest or host allocation is required.
#[inline]
pub fn give_order_to_unit_map(
    unit_ids: &[i32],
    cmd_id: i32,
    params: &[f32],
    options: u32,
    timeout: i32,
) -> Result<i32> {
    #[cfg(target_arch = "wasm32")]
    {
        let (unit_ids_ptr, unit_count) = slice_parts(unit_ids);
        let (params_ptr, param_count) = slice_parts(params);
        // SAFETY: both Rust slices are naturally aligned and live for the full
        // synchronous host call; the host validates both ranges independently.
        return decode_i32(unsafe {
            raw::give_order_to_unit_map(
                unit_ids_ptr,
                unit_count,
                cmd_id,
                params_ptr,
                param_count,
                options as i32,
                timeout,
            )
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_ids, cmd_id, params, options, timeout);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[cfg(feature = "alloc")]
struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

#[cfg(feature = "alloc")]
impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn u32(&mut self) -> Result<u32> {
        if self.cursor > self.bytes.len() || self.bytes.len() - self.cursor < 4 {
            return Err(ApiError::new(ErrorCode::Internal as i32));
        }
        let start = self.cursor;
        self.cursor += 4;
        Ok(u32::from_le_bytes([
            self.bytes[start],
            self.bytes[start + 1],
            self.bytes[start + 2],
            self.bytes[start + 3],
        ]))
    }

    fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }

    fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.u32()?))
    }

    fn finished(&self) -> bool {
        self.cursor == self.bytes.len()
    }
}

#[cfg(feature = "alloc")]
fn parse_commands(bytes: &[u8]) -> Result<Vec<UnitCommand>> {
    let mut reader = Reader::new(bytes);
    let count = reader.u32()? as usize;
    let mut commands = Vec::with_capacity(count);
    for _ in 0..count {
        let cmd_id = reader.i32()?;
        let options = reader.u32()?;
        if options > u8::MAX as u32 {
            return Err(ApiError::new(ErrorCode::Internal as i32));
        }
        let tag = reader.i32()?;
        let ai_command_id = reader.i32()?;
        let timeout = reader.f32()?;
        let param_count = reader.u32()? as usize;
        let mut params = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            params.push(reader.f32()?);
        }
        commands.push(UnitCommand {
            cmd_id,
            options: options as u8,
            tag,
            ai_command_id,
            timeout,
            params,
        });
    }
    if !reader.finished() {
        return Err(ApiError::new(ErrorCode::Internal as i32));
    }
    Ok(commands)
}

#[cfg(feature = "alloc")]
pub fn get_unit_commands(unit_id: i32, max_commands: u32) -> Result<Vec<UnitCommand>> {
    #[cfg(target_arch = "wasm32")]
    {
        let first = get_unit_commands_into(unit_id, max_commands, &mut [])?;
        let required = match first {
            CommandBufferFill::Complete(bytes)
            | CommandBufferFill::Insufficient { required: bytes } => bytes,
        };
        let mut wire = vec![0u8; required];
        for _ in 0..3 {
            match get_unit_commands_into(unit_id, max_commands, &mut wire)? {
                CommandBufferFill::Complete(bytes) => {
                    wire.truncate(bytes);
                    return parse_commands(&wire);
                }
                CommandBufferFill::Insufficient { required } => wire.resize(required, 0),
            }
        }
        Err(ApiError::new(ErrorCode::BufferOverflow as i32))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, max_commands);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

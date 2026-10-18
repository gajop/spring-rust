#[cfg(feature = "alloc")]
pub use crate::owned::units_commands::{
    find_unit_cmd_desc, get_command_params, get_command_queue, get_factory_bugger_off,
    get_factory_command_count, get_factory_commands, get_factory_counts, get_full_build_queue,
    get_real_build_queue, get_unit_cmd_descs, get_unit_current_command,
    give_order_array_to_unit_map,
};

// UnitsCommands portion of the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result, UnitId};

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
    unsafe extern "C" {
        #[link_name = "get-unit-command-count"]
        pub safe fn get_unit_command_count(unit_id: i32) -> i64;
        #[link_name = "get-unit-commands"]
        pub safe fn get_unit_commands(
            unit_id: i32,
            max_commands: i32,
            output: i32,
            capacity_bytes: i32,
        ) -> i64;
        #[link_name = "give-order"]
        pub safe fn give_order(
            cmd_id: i32,
            params: i32,
            param_count: i32,
            options: i32,
            timeout: i32,
        ) -> i64;
        #[link_name = "give-order-to-unit-map"]
        pub safe fn give_order_to_unit_map(
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
fn decode_fill(packed: i64, capacity: usize) -> Result<CommandBufferFill> {
    let packed = packed as u64;
    let bytes = packed as u32 as usize;
    let status = (packed >> 32) as u32 as i32;
    if status == 0 {
        if bytes > capacity {
            Err(ApiError::new(ErrorCode::Internal as i32))
        } else {
            Ok(CommandBufferFill::Complete(bytes))
        }
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
fn slice_parts<T>(values: &[T]) -> Result<(i32, i32)> {
    super::wasm_slice_parts(values)
}

#[inline]
pub fn get_unit_command_count(unit_id: impl Into<UnitId>) -> Result<u32> {
    let unit_id = unit_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let packed = raw::get_unit_command_count(unit_id.0) as u64;
        let status = (packed >> 32) as u32 as i32;
        if status == 0 {
            Ok(packed as u32)
        } else {
            Err(ApiError::new(status))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(unreachable!())
    }
}

#[inline]
pub fn get_unit_commands_into(
    unit_id: impl Into<UnitId>,
    max_commands: u32,
    output: &mut [u8],
) -> Result<CommandBufferFill> {
    let unit_id = unit_id.into();
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, capacity) = super::wasm_mut_slice_parts(output)?;
        decode_fill(
            raw::get_unit_commands(unit_id.0, max_commands as i32, pointer, capacity),
            output.len(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, max_commands, output);
        Err(unreachable!())
    }
}

/// Issue one command through the UnitsCommands API. The host borrows `params`
/// directly from fixed synced Core memory for the duration of the call.
#[inline]
pub fn give_order(cmd_id: i32, params: &[f32], options: u32, timeout: i32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (params_ptr, param_count) = slice_parts(params)?;
        decode_i32(raw::give_order(
            cmd_id,
            params_ptr,
            param_count,
            options as i32,
            timeout,
        ))
        .map(|value| value != 0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (cmd_id, params, options, timeout);
        Err(unreachable!())
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
        let (unit_ids_ptr, unit_count) = slice_parts(unit_ids)?;
        let (params_ptr, param_count) = slice_parts(params)?;
        decode_i32(raw::give_order_to_unit_map(
            unit_ids_ptr,
            unit_count,
            cmd_id,
            params_ptr,
            param_count,
            options as i32,
            timeout,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_ids, cmd_id, params, options, timeout);
        Err(unreachable!())
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
pub fn get_unit_commands(
    unit_id: impl Into<UnitId>,
    max_commands: u32,
) -> Result<Vec<UnitCommand>> {
    let unit_id = unit_id.into();
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
        Err(unreachable!())
    }
}

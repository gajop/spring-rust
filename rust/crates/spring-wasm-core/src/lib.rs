#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

//! Fast core-WebAssembly transport for Spring modules.
//!
//! The public API is safe. `unsafe` is contained in the generated transport
//! layer where wasm32 pointers are passed to host imports as 32-bit linear-
//! memory offsets. The host validates every offset/length before dereferencing.

pub const ABI_VERSION: u32 = 1;
pub const POSITION_MID: u32 = 1 << 0;
pub const POSITION_AIM: u32 = 1 << 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    InvalidArgument = 1,
    OutOfBounds = 2,
    NotFound = 3,
    NotAvailable = 4,
    InvalidState = 5,
    PermissionDenied = 6,
    AlreadyExists = 7,
    OperationFailed = 8,
    BufferOverflow = 9,
    InvalidId = 10,
    Internal = 999,
    UnsupportedHostTarget = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiError {
    pub code: i32,
}

impl ApiError {
    pub const fn new(code: i32) -> Self {
        Self { code }
    }
}

pub type Result<T> = core::result::Result<T, ApiError>;

const fn packed_value(value: u64) -> i32 {
    value as u32 as i32
}

const fn packed_status(value: u64) -> i32 {
    (value >> 32) as u32 as i32
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:units-info")]
    extern "C" {
        #[link_name = "get-unit-def-id"]
        pub fn get_unit_def_id(unit_id: i32) -> i64;

        #[link_name = "get-unit-position"]
        pub fn get_unit_position(unit_id: i32, flags: i32, output: i32) -> i32;
    }
}

/// Return the unit definition id using a single scalar Wasm crossing.
#[inline]
pub fn get_unit_def_id(unit_id: i32) -> Result<i32> {
    #[cfg(target_arch = "wasm32")]
    {
        // SAFETY: the import signature is generated together with the host
        // binding. The host returns one packed i64 and touches no guest memory.
        let packed = unsafe { raw::get_unit_def_id(unit_id) } as u64;
        let status = packed_status(packed);
        if status == 0 {
            Ok(packed_value(packed))
        } else {
            Err(ApiError::new(status))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

/// Read a unit position into guest-owned stack memory.
///
/// `mid_pos` and `aim_pos` map directly to the existing NativeInterface
/// options. The host writes exactly three little-endian f32 values after
/// validating the destination range against this instance's linear memory.
#[inline]
pub fn get_unit_position(unit_id: i32, mid_pos: bool, aim_pos: bool) -> Result<[f32; 3]> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0.0f32; 3];
        let mut flags = 0u32;
        if mid_pos {
            flags |= POSITION_MID;
        }
        if aim_pos {
            flags |= POSITION_AIM;
        }
        let pointer = output.as_mut_ptr() as usize;
        debug_assert!(pointer <= u32::MAX as usize);

        // SAFETY: wasm32 pointers are linear-memory offsets. `output` owns 12
        // writable bytes for the duration of the synchronous import. The host
        // validates the complete range before copying the three f32 values.
        let status = unsafe {
            raw::get_unit_position(unit_id, flags as i32, pointer as u32 as i32)
        };
        if status == 0 {
            Ok(output)
        } else {
            Err(ApiError::new(status))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, mid_pos, aim_pos);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

/// Export the standard `game-frame(i32) -> ()` callin without exposing ABI
/// details to module code.
#[macro_export]
macro_rules! export_game_frame {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[export_name = "spring:callin/game-frame"]
        pub extern "C" fn __spring_wasm_core_game_frame(frame: i32) {
            $handler(frame)
        }
    };
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_scalar_layout_matches_host() {
        let packed = 0x0000_0007_ffff_fffeu64;
        assert_eq!(packed_value(packed), -2);
        assert_eq!(packed_status(packed), 7);
    }
}

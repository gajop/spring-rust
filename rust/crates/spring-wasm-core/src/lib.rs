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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageResult {
    pub new_damage: f32,
    pub impulse_mult: f32,
}

impl DamageResult {
    pub const fn unchanged(damage: f32) -> Self {
        Self {
            new_damage: damage,
            impulse_mult: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllowUnitCreationResult {
    pub allow: bool,
    pub drop_order: bool,
}

impl AllowUnitCreationResult {
    pub const ALLOW: Self = Self {
        allow: true,
        drop_order: false,
    };
}

const fn packed_value(value: u64) -> i32 {
    value as u32 as i32
}

const fn packed_status(value: u64) -> i32 {
    (value >> 32) as u32 as i32
}

#[doc(hidden)]
#[inline]
pub fn __pack_f32_pair(first: f32, second: f32) -> i64 {
    let packed = first.to_bits() as u64 | ((second.to_bits() as u64) << 32);
    packed as i64
}

#[doc(hidden)]
#[inline]
pub const fn __pack_allow_unit_creation(value: AllowUnitCreationResult) -> i32 {
    (value.allow as i32) | ((value.drop_order as i32) << 1)
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

/// Export `GameFrame(i32) -> ()`.
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

/// Export `GameFramePost(i32) -> ()`.
#[macro_export]
macro_rules! export_game_frame_post {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[export_name = "spring:callin/game-frame-post"]
        pub extern "C" fn __spring_wasm_core_game_frame_post(frame: i32) {
            $handler(frame)
        }
    };
}

/// Export the unsynced per-render-frame `Update(f32) -> ()` callin.
#[macro_export]
macro_rules! export_update {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[export_name = "spring:callin/update"]
        pub extern "C" fn __spring_wasm_core_update(delta_seconds: f32) {
            $handler(delta_seconds)
        }
    };
}

/// Export `UnitCreated(i32, i32, i32, i32) -> ()`.
#[macro_export]
macro_rules! export_unit_created {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[export_name = "spring:callin/unit-created"]
        pub extern "C" fn __spring_wasm_core_unit_created(
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            builder_id: i32,
        ) {
            $handler(unit_id, unit_def_id, unit_team, builder_id)
        }
    };
}

/// Export the hot synced damage-control callin without an out-pointer.
///
/// The guest handler returns [`DamageResult`]. The transport packs both f32
/// bit-patterns into one i64 Core-Wasm result; the host unpacks them directly.
#[macro_export]
macro_rules! export_unit_pre_damaged {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[export_name = "spring:callin/unit-pre-damaged"]
        pub extern "C" fn __spring_wasm_core_unit_pre_damaged(
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            damage: f32,
            paralyzer: i32,
            weapon_def_id: i32,
            projectile_id: i32,
            attacker_id: i32,
            attacker_def_id: i32,
            attacker_team: i32,
        ) -> i64 {
            let result: $crate::DamageResult = $handler(
                unit_id,
                unit_def_id,
                unit_team,
                damage,
                paralyzer != 0,
                weapon_def_id,
                projectile_id,
                attacker_id,
                attacker_def_id,
                attacker_team,
            );
            $crate::__pack_f32_pair(result.new_damage, result.impulse_mult)
        }
    };
}

/// Export `AllowUnitCreation` with a one-i32 result bitset.
///
/// Result bit 0 is `allow`, bit 1 is `drop_order`; all other bits are reserved
/// and rejected by the host.
#[macro_export]
macro_rules! export_allow_unit_creation {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[export_name = "spring:callin/allow-unit-creation"]
        pub extern "C" fn __spring_wasm_core_allow_unit_creation(
            unit_def_id: i32,
            builder_id: i32,
            builder_team: i32,
            has_build_info: i32,
            build_x: f32,
            build_y: f32,
            build_z: f32,
            build_facing: i32,
        ) -> i32 {
            let result: $crate::AllowUnitCreationResult = $handler(
                unit_def_id,
                builder_id,
                builder_team,
                has_build_info != 0,
                [build_x, build_y, build_z],
                build_facing,
            );
            $crate::__pack_allow_unit_creation(result)
        }
    };
}

/// Export the zero-argument `DrawWorld() -> ()` callin.
#[macro_export]
macro_rules! export_draw_world {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[export_name = "spring:callin/draw-world"]
        pub extern "C" fn __spring_wasm_core_draw_world() {
            $handler()
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

    #[test]
    fn packed_damage_layout_is_bit_exact() {
        let packed = __pack_f32_pair(12.5, -0.25) as u64;
        assert_eq!(packed as u32, 12.5f32.to_bits());
        assert_eq!((packed >> 32) as u32, (-0.25f32).to_bits());
    }

    #[test]
    fn allow_unit_creation_flags_are_stable() {
        assert_eq!(__pack_allow_unit_creation(AllowUnitCreationResult::ALLOW), 1);
        assert_eq!(
            __pack_allow_unit_creation(AllowUnitCreationResult {
                allow: true,
                drop_order: true,
            }),
            3
        );
    }
}

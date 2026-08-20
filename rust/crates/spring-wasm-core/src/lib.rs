#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

//! Fast core-WebAssembly transport for Spring modules.
//!
//! The public API is safe. `unsafe` is contained in the generated transport
//! layer where wasm32 pointers are passed to host imports as 32-bit linear-
//! memory offsets. The host validates every offset/length before dereferencing.

mod benchmark;
mod config;
mod messages;
mod rules_params;
mod terrain;
mod unit_defs;
mod units_commands;
mod units_pieces;
mod units_query;
mod units_query_borrowed;
pub use benchmark::*;
pub use config::*;
pub use messages::*;
pub use rules_params::*;
pub use terrain::*;
pub use unit_defs::*;
pub use units_commands::*;
pub use units_pieces::*;
pub use units_query::*;
pub use units_query_borrowed::*;

/// Generated production-fast Core imports and direct wrappers. This stays
/// namespaced so specialized hand-written hot APIs remain the normal surface.
#[doc(hidden)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/core_generated.rs"));
}

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
pub struct UnitHealth {
    pub health: f32,
    pub max_health: f32,
    pub paralyze_damage: f32,
    pub capture_progress: f32,
    pub build_progress: f32,
}

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

#[inline]
fn unpack_i32(packed: i64) -> Result<i32> {
    let packed = packed as u64;
    let status = packed_status(packed);
    if status == 0 {
        Ok(packed_value(packed))
    } else {
        Err(ApiError::new(status))
    }
}

#[inline]
fn unpack_bool(packed: i64) -> Result<bool> {
    match unpack_i32(packed)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ApiError::new(ErrorCode::Internal as i32)),
    }
}

#[inline]
fn unpack_f32(packed: i64) -> Result<f32> {
    let packed = packed as u64;
    let status = packed_status(packed);
    if status == 0 {
        Ok(f32::from_bits(packed as u32))
    } else {
        Err(ApiError::new(status))
    }
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
        #[link_name = "get-unit-team"]
        pub fn get_unit_team(unit_id: i32) -> i64;
        #[link_name = "get-unit-is-dead"]
        pub fn get_unit_is_dead(unit_id: i32) -> i64;
        #[link_name = "get-unit-experience"]
        pub fn get_unit_experience(unit_id: i32) -> i64;
        #[link_name = "get-unit-position"]
        pub fn get_unit_position(unit_id: i32, flags: i32, output: i32) -> i32;
        #[link_name = "get-unit-velocity"]
        pub fn get_unit_velocity(unit_id: i32, output: i32) -> i32;
        #[link_name = "get-unit-health"]
        pub fn get_unit_health(unit_id: i32, output: i32) -> i32;
    }
}

#[inline]
pub fn get_unit_def_id(unit_id: i32) -> Result<i32> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_i32(unsafe { raw::get_unit_def_id(unit_id) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn get_unit_team(unit_id: i32) -> Result<i32> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_i32(unsafe { raw::get_unit_team(unit_id) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn get_unit_is_dead(unit_id: i32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_bool(unsafe { raw::get_unit_is_dead(unit_id) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn get_unit_experience(unit_id: i32) -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        return unpack_f32(unsafe { raw::get_unit_experience(unit_id) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

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

#[inline]
pub fn get_unit_velocity(unit_id: i32) -> Result<[f32; 3]> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0.0f32; 3];
        let pointer = output.as_mut_ptr() as usize;
        debug_assert!(pointer <= u32::MAX as usize);
        let status = unsafe { raw::get_unit_velocity(unit_id, pointer as u32 as i32) };
        if status == 0 {
            Ok(output)
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

#[inline]
pub fn get_unit_health(unit_id: i32) -> Result<UnitHealth> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0.0f32; 5];
        let pointer = output.as_mut_ptr() as usize;
        debug_assert!(pointer <= u32::MAX as usize);
        let status = unsafe { raw::get_unit_health(unit_id, pointer as u32 as i32) };
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(UnitHealth {
            health: output[0],
            max_health: output[1],
            paralyze_damage: output[2],
            capture_progress: output[3],
            build_progress: output[4],
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = unit_id;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

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

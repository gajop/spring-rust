#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

//! Fast core-WebAssembly transport for Spring modules.
//!
//! The public API is safe. `unsafe` is contained in the generated transport
//! layer where wasm32 pointers are passed to host imports as 32-bit linear-
//! memory offsets. The host validates every offset/length before dereferencing.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
mod benchmark;
#[cfg(target_arch = "wasm32")]
mod callback;
#[cfg(target_arch = "wasm32")]
mod cob_script;
#[cfg(target_arch = "wasm32")]
mod config;
/// Nondeterministic imports usable from synced code. Namespaced on purpose:
/// importing anything here makes a synced guest desync. See `desync.rs`.
#[cfg(target_arch = "wasm32")]
pub mod desync;
#[cfg(target_arch = "wasm32")]
mod gfx;
#[cfg(target_arch = "wasm32")]
mod math_extra;
#[cfg(target_arch = "wasm32")]
mod messages;
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub mod prelude;
#[cfg(target_arch = "wasm32")]
mod profiling;
#[cfg(target_arch = "wasm32")]
mod rml_ui;
#[cfg(target_arch = "wasm32")]
mod rules_params;
#[cfg(target_arch = "wasm32")]
mod system_control;
#[cfg(target_arch = "wasm32")]
mod terrain;
#[cfg(target_arch = "wasm32")]
mod terrain_control;
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub mod typed;
#[cfg(target_arch = "wasm32")]
mod unit_control;
#[cfg(target_arch = "wasm32")]
mod unit_defs;
#[cfg(target_arch = "wasm32")]
mod units_commands;
#[cfg(target_arch = "wasm32")]
mod units_pieces;
#[cfg(target_arch = "wasm32")]
mod units_query;
#[cfg(target_arch = "wasm32")]
mod units_query_borrowed;
#[cfg(target_arch = "wasm32")]
mod vfs;
#[cfg(target_arch = "wasm32")]
pub use benchmark::*;
#[cfg(target_arch = "wasm32")]
pub use callback::*;
#[cfg(target_arch = "wasm32")]
pub use cob_script::*;
#[cfg(target_arch = "wasm32")]
pub use config::*;
#[cfg(target_arch = "wasm32")]
pub use gfx::*;
#[cfg(target_arch = "wasm32")]
pub use math_extra::*;
#[cfg(target_arch = "wasm32")]
pub use messages::*;
#[cfg(target_arch = "wasm32")]
pub use profiling::*;
#[cfg(target_arch = "wasm32")]
pub use rml_ui::*;
#[cfg(target_arch = "wasm32")]
pub use rules_params::*;
#[cfg(target_arch = "wasm32")]
pub use system_control::*;
#[cfg(target_arch = "wasm32")]
pub use terrain::*;
#[cfg(target_arch = "wasm32")]
pub use terrain_control::*;
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub use typed::*;
#[cfg(target_arch = "wasm32")]
pub use unit_control::*;
#[cfg(target_arch = "wasm32")]
pub use unit_defs::*;
#[cfg(target_arch = "wasm32")]
pub use units_commands::*;
#[cfg(target_arch = "wasm32")]
pub use units_pieces::*;
#[cfg(target_arch = "wasm32")]
pub use units_query::*;
#[cfg(target_arch = "wasm32")]
pub use units_query_borrowed::*;
#[cfg(target_arch = "wasm32")]
pub use vfs::*;

/// Generated production-fast Core imports and direct wrappers. This stays
/// namespaced so specialized hand-written hot APIs remain the normal surface.
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/core_generated.rs"));
}

#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub use generated::{gaia_synced, gaia_unsynced, intro, menu, rules_synced, rules_unsynced, ui};

/// Export the selected guest environment as a small Core-Wasm ABI marker.
/// Call this once from the crate that chooses one generated environment module.
#[macro_export]
macro_rules! export_environment_mask {
    ($mask:expr) => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_ENV_MASK() -> i32 {
            $mask as i32
        }
    };
}

/// Owned semantic façade for Core guests. The raw/generated namespaces remain
/// available for allocation-free callers; parity and application code can use
/// this stable surface when the `alloc` feature is enabled.
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub use generated::owned;

#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub use owned::{
    callins, camera, debug_input, display, effects_control, encoding,
    feature_control, feature_defs, features, game, game_config,
    ground_decals, icons, input, lights, los, markers, memory,
    metal_map, move_ctrl, path_finder, platform, player,
    projectile_control, projectiles, selection, sound, synced_ctrl,
    team_control, teams, tracing, types, unit_rendering, unit_script,
    units_info, units_weapons, unsynced_ctrl, unsynced_read, utils,
    weapon_defs,
};

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

#[cfg(any(target_arch = "wasm32", test))]
const fn packed_value(value: u64) -> i32 {
    value as u32 as i32
}

#[cfg(any(target_arch = "wasm32", test))]
const fn packed_status(value: u64) -> i32 {
    (value >> 32) as u32 as i32
}

#[inline]
#[cfg(target_arch = "wasm32")]
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
#[cfg(target_arch = "wasm32")]
fn unpack_bool(packed: i64) -> Result<bool> {
    match unpack_i32(packed)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ApiError::new(ErrorCode::Internal as i32)),
    }
}

#[inline]
#[cfg(target_arch = "wasm32")]
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
    unsafe extern "C" {
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

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_def_id(unit_id: i32) -> Result<i32> {
    unpack_i32(unsafe { raw::get_unit_def_id(unit_id) })
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_team(unit_id: i32) -> Result<i32> {
    unpack_i32(unsafe { raw::get_unit_team(unit_id) })
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_is_dead(unit_id: i32) -> Result<bool> {
    unpack_bool(unsafe { raw::get_unit_is_dead(unit_id) })
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_experience(unit_id: i32) -> Result<f32> {
    unpack_f32(unsafe { raw::get_unit_experience(unit_id) })
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_position(unit_id: i32, mid_pos: bool, aim_pos: bool) -> Result<[f32; 3]> {
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
    let status = unsafe { raw::get_unit_position(unit_id, flags as i32, pointer as u32 as i32) };
    if status == 0 {
        Ok(output)
    } else {
        Err(ApiError::new(status))
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_velocity(unit_id: i32) -> Result<[f32; 3]> {
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

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_health(unit_id: i32) -> Result<UnitHealth> {
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

#[macro_export]
macro_rules! export_game_frame {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:callin/game-frame")]
        pub extern "C" fn __spring_game_frame(frame: i32) {
            $handler(frame)
        }
    };
}

#[macro_export]
macro_rules! export_game_frame_post {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:callin/game-frame-post")]
        pub extern "C" fn __spring_game_frame_post(frame: i32) {
            $handler(frame)
        }
    };
}

#[macro_export]
macro_rules! export_update {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:callin/update")]
        pub extern "C" fn __spring_update(delta_seconds: f32) {
            $handler(delta_seconds)
        }
    };
}

#[macro_export]
macro_rules! export_unit_created {
    ($handler:path) => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:callin/unit-created")]
        pub extern "C" fn __spring_unit_created(
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
        #[unsafe(export_name = "spring:callin/unit-pre-damaged")]
        pub extern "C" fn __spring_unit_pre_damaged(
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
        #[unsafe(export_name = "spring:callin/allow-unit-creation")]
        pub extern "C" fn __spring_allow_unit_creation(
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
        #[unsafe(export_name = "spring:callin/draw-world")]
        pub extern "C" fn __spring_draw_world() {
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
        assert_eq!(
            __pack_allow_unit_creation(AllowUnitCreationResult::ALLOW),
            1
        );
        assert_eq!(
            __pack_allow_unit_creation(AllowUnitCreationResult {
                allow: true,
                drop_order: true,
            }),
            3
        );
    }
}

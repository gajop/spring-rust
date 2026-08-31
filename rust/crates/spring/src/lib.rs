#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

//! Fast core-WebAssembly transport for Spring modules.
//!
//! The public API is safe. `unsafe` is contained in the generated transport
//! layer where wasm32 pointers are passed to host imports as 32-bit linear-
//! memory offsets. The host validates every offset/length before dereferencing.

#[cfg(feature = "alloc")]
extern crate alloc;

/// Portable Rust implementation of the engine's unit-script model.
///
/// The module is available on both wasm and native targets.  Its engine
/// operations are supplied through [`cus::UnitEngine`], keeping game code
/// independent of the transport used by the host.
#[cfg(feature = "alloc")]
pub mod cus;

// Keep the exported Core-CUS ABI macro type-checked in the same target build
// used by game modules. This catches changes to the guest-side scratch ABI
// without creating a runnable example binary for a no_std crate.
#[cfg(all(feature = "alloc", target_arch = "wasm32", test))]
mod core_cus_export_compile_test {
    use super::cus::core_module::{CoreCusCallResult, CoreCusModule};

    #[derive(Default)]
    struct ExampleModule;

    impl CoreCusModule for ExampleModule {
        fn cus_invoke(
            &mut self,
            _instance_id: u32,
            _call: u32,
            _float_arguments: &[f32],
            _integer_arguments: &[i32],
            _result: &mut CoreCusCallResult<'_>,
        ) -> bool {
            false
        }
    }

    crate::export_core_cus!(ExampleModule);
}

#[cfg(target_arch = "wasm32")]
mod benchmark;
#[cfg(target_arch = "wasm32")]
pub mod callback;
#[cfg(target_arch = "wasm32")]
mod cob_script;
#[cfg(target_arch = "wasm32")]
pub mod config;
/// Nondeterministic imports usable from synced code. Namespaced on purpose:
/// importing anything here makes a synced guest desync. See `desync.rs`.
#[cfg(target_arch = "wasm32")]
pub mod desync;
#[cfg(target_arch = "wasm32")]
pub mod gfx;
#[cfg(target_arch = "wasm32")]
pub mod math_extra;
#[cfg(target_arch = "wasm32")]
pub mod messages;
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub mod prelude;
#[cfg(target_arch = "wasm32")]
pub mod profiling;
#[cfg(target_arch = "wasm32")]
pub mod rml_ui;
#[cfg(target_arch = "wasm32")]
pub mod rules_params;
#[cfg(target_arch = "wasm32")]
pub mod system_control;
#[cfg(target_arch = "wasm32")]
pub mod terrain;
#[cfg(target_arch = "wasm32")]
pub mod terrain_control;
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub mod typed;
#[cfg(target_arch = "wasm32")]
pub mod unit_control;
#[cfg(target_arch = "wasm32")]
pub mod unit_defs;
#[cfg(target_arch = "wasm32")]
pub mod units_commands;
#[cfg(target_arch = "wasm32")]
pub mod units_pieces;
#[cfg(target_arch = "wasm32")]
pub mod units_query;
#[cfg(target_arch = "wasm32")]
pub mod units_query_borrowed;
#[cfg(target_arch = "wasm32")]
pub mod vfs;
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

/// Compatibility export for the pre-flat API. New code should use the
/// module paths re-exported directly from this crate; this hidden-in-docs
/// alias remains public so existing guests do not lose a legitimate API while
/// migrating.
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
#[doc(hidden)]
pub use generated::owned;

#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub use owned::{
    callins, camera, debug_input, display, effects_control, encoding, feature_control,
    feature_defs, features, game, game_config, ground_decals, icons, input, lights, los, markers,
    memory, metal_map, move_ctrl, path_finder, platform, player, projectile_control, projectiles,
    selection, sound, synced_ctrl, team_control, teams, tracing, types, unit_rendering,
    unit_script, units_info, units_weapons, unsynced_ctrl, unsynced_read, utils, weapon_defs,
};

#[inline]
fn sqrt_f32(v: f32) -> f32 {
    libm::sqrtf(v)
}

pub const ABI_VERSION: u32 = 1;
pub const POSITION_MID: u32 = 1 << 0;
pub const POSITION_AIM: u32 = 1 << 1;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn length(self) -> f32 {
        sqrt_f32(self.length_squared())
    }

    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline]
    pub fn as_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

pub type Vec3 = Float3;

impl From<[f32; 3]> for Float3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
}

impl From<Float3> for [f32; 3] {
    #[inline]
    fn from(v: Float3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl core::ops::Add for Float3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl core::ops::AddAssign for Float3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl core::ops::Sub for Float3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl core::ops::SubAssign for Float3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl core::ops::Mul<f32> for Float3 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl core::ops::Mul<Float3> for f32 {
    type Output = Float3;
    #[inline]
    fn mul(self, v: Float3) -> Float3 {
        Float3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl core::ops::MulAssign<f32> for Float3 {
    #[inline]
    fn mul_assign(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl core::ops::Neg for Float3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl core::ops::Index<usize> for Float3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Float3 index out of bounds"),
        }
    }
}

impl core::ops::IndexMut<usize> for Float3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Float3 index out of bounds"),
        }
    }
}

macro_rules! newtype_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $name(pub i32);

        impl From<i32> for $name {
            #[inline]
            fn from(v: i32) -> Self { Self(v) }
        }
        impl From<$name> for i32 {
            #[inline]
            fn from(v: $name) -> Self { v.0 }
        }
    };
}

newtype_id!(UnitId);
newtype_id!(DefId);
newtype_id!(TeamId);
newtype_id!(PlayerId);
newtype_id!(WeaponDefId);
newtype_id!(ProjectileId);
newtype_id!(FeatureId);

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
fn decode_packed_f32(packed: i64) -> Result<f32> {
    let packed = packed as u64;
    let status = packed_status(packed);
    if status == 0 {
        Ok(f32::from_bits(packed as u32))
    } else {
        Err(ApiError::new(status))
    }
}

/// Convert a guest slice into the pointer/count pair used by Core imports.
///
/// This is intentionally kept private to the façade. A Core import receives a
/// 32-bit linear-memory offset, so every manual adapter must validate the
/// conversion instead of relying on a debug-only assertion.
#[cfg(target_arch = "wasm32")]
#[inline]
pub(crate) fn wasm_slice_parts<T>(slice: &[T]) -> Result<(i32, i32)> {
    let byte_len = slice
        .len()
        .checked_mul(core::mem::size_of::<T>())
        .ok_or_else(|| ApiError::new(ErrorCode::InvalidArgument as i32))?;
    if slice.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    if slice.is_empty() {
        return Ok((0, 0));
    }
    let pointer = slice.as_ptr() as usize;
    if !wasm_range_fits(pointer, byte_len) {
        return Err(ApiError::new(ErrorCode::OutOfBounds as i32));
    }
    Ok((pointer as u32 as i32, slice.len() as u32 as i32))
}

/// Mutable counterpart to [`wasm_slice_parts`].
#[cfg(target_arch = "wasm32")]
#[inline]
pub(crate) fn wasm_mut_slice_parts<T>(slice: &mut [T]) -> Result<(i32, i32)> {
    let byte_len = slice
        .len()
        .checked_mul(core::mem::size_of::<T>())
        .ok_or_else(|| ApiError::new(ErrorCode::InvalidArgument as i32))?;
    if slice.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    if slice.is_empty() {
        return Ok((0, 0));
    }
    let pointer = slice.as_mut_ptr() as usize;
    if !wasm_range_fits(pointer, byte_len) {
        return Err(ApiError::new(ErrorCode::OutOfBounds as i32));
    }
    Ok((pointer as u32 as i32, slice.len() as u32 as i32))
}

/// Convert a single guest output value into a checked 32-bit linear-memory
/// offset for a Core import.
#[cfg(target_arch = "wasm32")]
#[inline]
pub(crate) fn wasm_output_ptr<T>(value: &mut T) -> Result<i32> {
    let pointer = value as *mut T as usize;
    if !wasm_range_fits(pointer, core::mem::size_of::<T>()) {
        return Err(ApiError::new(ErrorCode::OutOfBounds as i32));
    }
    Ok(pointer as u32 as i32)
}

/// Convert a single guest input value into a checked 32-bit linear-memory
/// offset for a Core import.
#[cfg(target_arch = "wasm32")]
#[inline]
pub(crate) fn wasm_input_ptr<T>(value: &T) -> Result<i32> {
    let pointer = core::ptr::from_ref(value) as usize;
    if !wasm_range_fits(pointer, core::mem::size_of::<T>()) {
        return Err(ApiError::new(ErrorCode::OutOfBounds as i32));
    }
    Ok(pointer as u32 as i32)
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn wasm_range_fits(pointer: usize, byte_len: usize) -> bool {
    (pointer as u64).saturating_add(byte_len as u64) <= (u32::MAX as u64) + 1
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
        pub safe fn get_unit_def_id(unit_id: i32) -> i64;
        #[link_name = "get-unit-team"]
        pub safe fn get_unit_team(unit_id: i32) -> i64;
        #[link_name = "get-unit-is-dead"]
        pub safe fn get_unit_is_dead(unit_id: i32) -> i64;
        #[link_name = "get-unit-experience"]
        pub safe fn get_unit_experience(unit_id: i32) -> i64;
        #[link_name = "get-unit-position"]
        pub safe fn get_unit_position(unit_id: i32, flags: i32, output: i32) -> i32;
        #[link_name = "get-unit-velocity"]
        pub safe fn get_unit_velocity(unit_id: i32, output: i32) -> i32;
        #[link_name = "get-unit-health"]
        pub safe fn get_unit_health(unit_id: i32, output: i32) -> i32;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_def_id(unit_id: impl Into<UnitId>) -> Result<DefId> {
    let unit_id = unit_id.into();
    unpack_i32(raw::get_unit_def_id(unit_id.0)).map(DefId)
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_team(unit_id: impl Into<UnitId>) -> Result<TeamId> {
    let unit_id = unit_id.into();
    unpack_i32(raw::get_unit_team(unit_id.0)).map(TeamId)
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_is_dead(unit_id: impl Into<UnitId>) -> Result<bool> {
    let unit_id = unit_id.into();
    unpack_bool(raw::get_unit_is_dead(unit_id.0))
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_experience(unit_id: impl Into<UnitId>) -> Result<f32> {
    let unit_id = unit_id.into();
    decode_packed_f32(raw::get_unit_experience(unit_id.0))
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_position(
    unit_id: impl Into<UnitId>,
    mid_pos: bool,
    aim_pos: bool,
) -> Result<Float3> {
    let unit_id = unit_id.into();
    let mut output = [0.0f32; 3];
    let mut flags = 0u32;
    if mid_pos {
        flags |= POSITION_MID;
    }
    if aim_pos {
        flags |= POSITION_AIM;
    }
    let pointer = wasm_output_ptr(&mut output)?;
    let status = raw::get_unit_position(unit_id.0, flags as i32, pointer);
    if status == 0 {
        Ok(Float3::from(output))
    } else {
        Err(ApiError::new(status))
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_velocity(unit_id: impl Into<UnitId>) -> Result<Float3> {
    let unit_id = unit_id.into();
    let mut output = [0.0f32; 3];
    let pointer = wasm_output_ptr(&mut output)?;
    let status = raw::get_unit_velocity(unit_id.0, pointer);
    if status == 0 {
        Ok(Float3::from(output))
    } else {
        Err(ApiError::new(status))
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_unit_health(unit_id: impl Into<UnitId>) -> Result<UnitHealth> {
    let unit_id = unit_id.into();
    let mut output = [0.0f32; 5];
    let pointer = wasm_output_ptr(&mut output)?;
    let status = raw::get_unit_health(unit_id.0, pointer);
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

#[cfg(all(test, feature = "alloc", target_arch = "wasm32"))]
mod public_api_tests {
    use super::*;

    #[test]
    fn canonical_module_paths_keep_typed_signatures() {
        let _: fn(&str, f32, f32, f32, &str) -> Result<()> = crate::gfx::text;
        let _: fn(u32, SyncCallback) -> Result<()> = crate::gfx::begin_end;
        let _: fn(&crate::camera::CameraState, f32, f32, f32) -> Result<bool> =
            crate::camera::set_camera_state;
        let _: fn(i32, f32) -> Result<bool> = crate::move_ctrl::set_ground_move_type_max_speed;
        let _: crate::types::Float3 = Float3::ZERO;
    }
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
            $handler(
                $crate::UnitId(unit_id).into(),
                $crate::DefId(unit_def_id).into(),
                $crate::TeamId(unit_team).into(),
                $crate::UnitId(builder_id).into(),
            )
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
                $crate::UnitId(unit_id).into(),
                $crate::DefId(unit_def_id).into(),
                $crate::TeamId(unit_team).into(),
                damage,
                paralyzer != 0,
                $crate::WeaponDefId(weapon_def_id).into(),
                $crate::ProjectileId(projectile_id).into(),
                $crate::UnitId(attacker_id).into(),
                $crate::DefId(attacker_def_id).into(),
                $crate::TeamId(attacker_team).into(),
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
                $crate::DefId(unit_def_id).into(),
                $crate::UnitId(builder_id).into(),
                $crate::TeamId(builder_team).into(),
                has_build_info != 0,
                $crate::Float3::new(build_x, build_y, build_z).into(),
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

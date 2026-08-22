    pub mod units_weapons {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CommonErrorCode {
            ErrorAlreadyExists,
            ErrorBufferOverflow,
            ErrorInternal,
            ErrorInvalidArgument,
            ErrorInvalidId,
            ErrorInvalidState,
            ErrorNone,
            ErrorNotAvailable,
            ErrorNotFound,
            ErrorOperationFailed,
            ErrorOutOfBounds,
            ErrorPermissionDenied,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AtmosphereParams {
            pub fog_color: Option<Vec<f32>>,
            pub sky_color: Option<Vec<f32>>,
            pub sun_color: Option<Vec<f32>>,
            pub cloud_color: Option<Vec<f32>>,
            pub sky_axis_angle: Option<Vec<f32>>,
            pub fog_start: Option<f32>,
            pub fog_end: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CollisionVolumeData {
            pub scale_x: f32,
            pub scale_y: f32,
            pub scale_z: f32,
            pub offset_x: f32,
            pub offset_y: f32,
            pub offset_z: f32,
            pub volume_type: i32,
            pub test_type: i32,
            pub primary_axis: i32,
            pub disabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2 {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2Result {
            pub value: Float2,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Result {
            pub value: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4Result {
            pub value: Float4,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitMaxRangeQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitMaxRangeResult {
            pub max_range: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponCanFireQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponCanFireResult {
            pub can_fire: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponCountQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponDamagesQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponDamagesResult {
            pub damages: UnitWeaponDamages,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponHaveFreeLineOfFireOptions {
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponHaveFreeLineOfFireQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_id: i32,
            pub source_pos: Float3,
            pub target_pos: Float3,
            pub options: GetUnitWeaponHaveFreeLineOfFireOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponHaveFreeLineOfFireResult {
            pub has_free_line_of_fire: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponStateQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponStateResult {
            pub state: UnitWeaponState,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTargetQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTargetResult {
            pub target: UnitWeaponTarget,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTestRangeQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTestRangeResult {
            pub in_range: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTestTargetOptions {
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTestTargetQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_id: i32,
            pub target_pos: Float3,
            pub options: GetUnitWeaponTestTargetOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTestTargetResult {
            pub can_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTryTargetOptions {
            pub user_target: bool,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTryTargetQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_id: i32,
            pub target_pos: Float3,
            pub options: GetUnitWeaponTryTargetOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponTryTargetResult {
            pub can_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponVectorsQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponVectorsResult {
            pub vectors: UnitWeaponVectors,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int2 {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int3 {
            pub x: i32,
            pub y: i32,
            pub z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Result {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MapRenderingParams {
            pub splat_tex_scales: Option<Vec<f32>>,
            pub splat_tex_mults: Option<Vec<f32>>,
            pub void_water: Option<bool>,
            pub void_ground: Option<bool>,
            pub splat_detail_normal_diffuse_alpha: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeExplosionParams {
            pub damages: f32,
            pub weapon_def_id: i32,
            pub owner_id: i32,
            pub hit_unit_id: i32,
            pub hit_feature_id: i32,
            pub crater_area_of_effect: f32,
            pub damage_area_of_effect: f32,
            pub edge_effectiveness: f32,
            pub explosion_speed: f32,
            pub gfx_mod: f32,
            pub impact_only: bool,
            pub ignore_owner: bool,
            pub damage_ground: bool,
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeProjectileParams {
            pub pos: Float3,
            pub speed: Float3,
            pub spread: Float3,
            pub end: Float3,
            pub owner: i32,
            pub team: i32,
            pub weapon_num: i32,
            pub ttl: f32,
            pub gravity: f32,
            pub tracking: f32,
            pub max_range: f32,
            pub up_time: f32,
            pub start_alpha: f32,
            pub end_alpha: f32,
            pub model: String,
            pub ceg_tag: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NumberOrBool {
            pub number: f32,
            pub boolean: bool,
            pub use_boolean: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourcePack {
            pub metal: f32,
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RgbColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SunLightingParams {
            pub ground_ambient_color: Option<Vec<f32>>,
            pub ground_diffuse_color: Option<Vec<f32>>,
            pub ground_specular_color: Option<Vec<f32>>,
            pub model_ambient_color: Option<Vec<f32>>,
            pub model_diffuse_color: Option<Vec<f32>>,
            pub model_specular_color: Option<Vec<f32>>,
            pub specular_exponent: Option<f32>,
            pub ground_shadow_density: Option<f32>,
            pub model_shadow_density: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Result {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCostOverrides {
            pub build_time: f32,
            pub metal_cost: f32,
            pub energy_cost: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitHealthValue {
            pub health: f32,
            pub capture: f32,
            pub paralyze: f32,
            pub build: f32,
            pub use_amounts: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitWeaponDamages {
            pub damages: Vec<f32>,
            pub paralyze_damage_time: f32,
            pub impulse_factor: f32,
            pub impulse_boost: f32,
            pub crater_mult: f32,
            pub crater_boost: f32,
            pub default_damage: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitWeaponState {
            pub reload_time: f32,
            pub reload_frame: f32,
            pub range: f32,
            pub projectile_speed: f32,
            pub accuracy: f32,
            pub spray_angle: f32,
            pub aim_from_height: f32,
            pub salvo_size: f32,
            pub salvo_delay: f32,
            pub salvo_error: f32,
            pub target_move_error: f32,
            pub turn_rate: f32,
            pub auto_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitWeaponTarget {
            pub target_type: i32,
            pub target_id: i32,
            pub target_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitWeaponVectors {
            pub weapon_muzzle_pos: Float3,
            pub weapon_aim_pos: Float3,
            pub weapon_dir: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WaterParams {
            pub absorb: Option<Vec<f32>>,
            pub base_color: Option<Vec<f32>>,
            pub min_color: Option<Vec<f32>>,
            pub surface_color: Option<Vec<f32>>,
            pub diffuse_color: Option<Vec<f32>>,
            pub specular_color: Option<Vec<f32>>,
            pub plane_color: Option<Vec<f32>>,
            pub repeat_x: Option<f32>,
            pub repeat_y: Option<f32>,
            pub surface_alpha: Option<f32>,
            pub ambient_factor: Option<f32>,
            pub diffuse_factor: Option<f32>,
            pub specular_factor: Option<f32>,
            pub specular_power: Option<f32>,
            pub fresnel_min: Option<f32>,
            pub fresnel_max: Option<f32>,
            pub fresnel_power: Option<f32>,
            pub reflection_distortion: Option<f32>,
            pub blur_base: Option<f32>,
            pub blur_exponent: Option<f32>,
            pub perlin_start_freq: Option<f32>,
            pub perlin_lacunarity: Option<f32>,
            pub perlin_amplitude: Option<f32>,
            pub wind_speed: Option<f32>,
            pub wave_offset_factor: Option<f32>,
            pub wave_length: Option<f32>,
            pub wave_foam_distortion: Option<f32>,
            pub wave_foam_intensity: Option<f32>,
            pub caustics_resolution: Option<f32>,
            pub caustics_strength: Option<f32>,
            pub num_tiles: Option<f32>,
            pub shore_waves: Option<bool>,
            pub force_rendering: Option<bool>,
            pub has_water_plane: Option<bool>,
        }

        #[inline]
        pub fn get_unit_max_range(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_weapons::get_unit_max_range(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_can_fire(unit_id: i32, weapon_num: i32) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_can_fire(unit_id, weapon_num)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_count(unit_id: i32) -> Result<u32> {
            let value = crate::generated::units_weapons::get_unit_weapon_count(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_weapon_damages {
            #[link(wasm_import_module = "spring:units-weapons")]
            extern "C" {
                #[link_name = "get-unit-weapon-damages"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-weapons.get-unit-weapon-damages."]
        #[inline]
        pub unsafe fn get_unit_weapon_damages(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_weapon_damages::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_unit_weapon_have_free_line_of_fire(unit_id: i32, weapon_num: i32, target_id: i32, source_pos: Float3, target_pos: Float3, options: GetUnitWeaponHaveFreeLineOfFireOptions) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_have_free_line_of_fire(unit_id, weapon_num, target_id, crate::generated::units_weapons::Float3 { x: source_pos.x, y: source_pos.y, z: source_pos.z }, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z }, crate::generated::units_weapons::GetUnitWeaponHaveFreeLineOfFireOptions { is_ground_target: options.is_ground_target })?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_weapon_state {
            #[link(wasm_import_module = "spring:units-weapons")]
            extern "C" {
                #[link_name = "get-unit-weapon-state"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-weapons.get-unit-weapon-state."]
        #[inline]
        pub unsafe fn get_unit_weapon_state(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_get_unit_weapon_state::call(p0, p1, p2, p3) }
        }

        #[inline]
        pub fn get_unit_weapon_target(unit_id: i32, weapon_num: i32) -> Result<UnitWeaponTarget> {
            let value = crate::generated::units_weapons::get_unit_weapon_target(unit_id, weapon_num)?;
            Ok(UnitWeaponTarget { target_type: value.target_type, target_id: value.target_id, target_pos: Float3 { x: value.target_pos.x, y: value.target_pos.y, z: value.target_pos.z } })
        }

        #[inline]
        pub fn get_unit_weapon_test_range(unit_id: i32, weapon_num: i32, target_pos: Float3) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_test_range(unit_id, weapon_num, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_test_target(unit_id: i32, weapon_num: i32, target_id: i32, target_pos: Float3, options: GetUnitWeaponTestTargetOptions) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_test_target(unit_id, weapon_num, target_id, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z }, crate::generated::units_weapons::GetUnitWeaponTestTargetOptions { is_ground_target: options.is_ground_target })?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_try_target(unit_id: i32, weapon_num: i32, target_id: i32, target_pos: Float3, options: GetUnitWeaponTryTargetOptions) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_try_target(unit_id, weapon_num, target_id, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z }, crate::generated::units_weapons::GetUnitWeaponTryTargetOptions { user_target: options.user_target, is_ground_target: options.is_ground_target })?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_vectors(unit_id: i32, weapon_num: i32) -> Result<UnitWeaponVectors> {
            let value = crate::generated::units_weapons::get_unit_weapon_vectors(unit_id, weapon_num)?;
            Ok(UnitWeaponVectors { weapon_muzzle_pos: Float3 { x: value.weapon_muzzle_pos.x, y: value.weapon_muzzle_pos.y, z: value.weapon_muzzle_pos.z }, weapon_aim_pos: Float3 { x: value.weapon_aim_pos.x, y: value.weapon_aim_pos.y, z: value.weapon_aim_pos.z }, weapon_dir: Float3 { x: value.weapon_dir.x, y: value.weapon_dir.y, z: value.weapon_dir.z } })
        }

    }


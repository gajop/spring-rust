    pub mod los {
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
        pub struct GetClosestValidPositionQuery {
            pub unit_def_id: i32,
            pub x: f32,
            pub z: f32,
            pub radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetClosestValidPositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPositionLosStateQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPositionLosStateResult {
            pub state: PositionLosState,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRadarErrorParamsQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRadarErrorParamsResult {
            pub params: RadarErrorParams,
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
        pub struct IsPosInAirLosQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPosInAirLosResult {
            pub in_air_los: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPosInLosQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPosInLosResult {
            pub in_los: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPosInRadarQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPosInRadarResult {
            pub in_radar: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInAirLosQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInAirLosResult {
            pub in_air_los: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInJammerQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInJammerResult {
            pub in_jammer: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInLosQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInLosResult {
            pub in_los: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInRadarQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInRadarResult {
            pub in_radar: bool,
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
        pub struct PositionLosState {
            pub in_los_or_radar: bool,
            pub in_los: bool,
            pub in_radar: bool,
            pub in_jammer: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RadarErrorParams {
            pub radar_error_size: f32,
            pub base_radar_error_size: f32,
            pub base_radar_error_mult: f32,
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
        pub fn get_closest_valid_position(unit_def_id: i32, x: f32, z: f32, radius: f32) -> Result<Float3> {
            let value = crate::generated::los::get_closest_valid_position(unit_def_id, x, z, radius)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_position_los_state(pos: Float3, ally_team_id: i32) -> Result<PositionLosState> {
            let value = crate::generated::los::get_position_los_state(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(PositionLosState { in_los_or_radar: value.in_los_or_radar, in_los: value.in_los, in_radar: value.in_radar, in_jammer: value.in_jammer })
        }

        #[inline]
        pub fn get_radar_error_params(ally_team_id: i32) -> Result<RadarErrorParams> {
            let value = crate::generated::los::get_radar_error_params(ally_team_id)?;
            Ok(RadarErrorParams { radar_error_size: value.radar_error_size, base_radar_error_size: value.base_radar_error_size, base_radar_error_mult: value.base_radar_error_mult })
        }

        #[inline]
        pub fn is_pos_in_air_los(pos: Float3, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_pos_in_air_los(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_pos_in_los(pos: Float3, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_pos_in_los(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_pos_in_radar(pos: Float3, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_pos_in_radar(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_air_los(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_air_los(unit_id, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_jammer(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_jammer(unit_id, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_los(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_los(unit_id, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_radar(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_radar(unit_id, ally_team_id)?;
            Ok(value)
        }

    }


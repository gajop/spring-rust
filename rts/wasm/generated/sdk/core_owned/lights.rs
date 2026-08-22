    pub mod lights {
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
        pub struct AddLightTrackingTargetQuery {
            pub light_handle: u32,
            pub object_id: i32,
            pub track_unit: bool,
            pub enable_tracking: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddLightTrackingTargetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddMapLightQuery {
            pub params: LightParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddMapLightResult {
            pub light_handle: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddModelLightQuery {
            pub params: LightParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddModelLightResult {
            pub light_handle: u32,
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
        pub struct LightParams {
            pub position: Vec<f32>,
            pub direction: Vec<f32>,
            pub ambient_color: Vec<f32>,
            pub diffuse_color: Vec<f32>,
            pub specular_color: Vec<f32>,
            pub intensity_weight: Vec<f32>,
            pub attenuation: Vec<f32>,
            pub ambient_decay_rate: Vec<f32>,
            pub diffuse_decay_rate: Vec<f32>,
            pub specular_decay_rate: Vec<f32>,
            pub decay_function_type: Vec<f32>,
            pub radius: f32,
            pub fov: f32,
            pub ttl: u32,
            pub priority: u32,
            pub ignore_los: bool,
            pub local_space: bool,
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
        pub struct SetMapLightTrackingStateQuery {
            pub light_handle: u32,
            pub object_id: i32,
            pub enable_tracking: bool,
            pub track_unit: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapLightTrackingStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetModelLightTrackingStateQuery {
            pub light_handle: u32,
            pub object_id: i32,
            pub enable_tracking: bool,
            pub track_unit: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetModelLightTrackingStateResult {
            pub success: bool,
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
        pub struct UpdateMapLightQuery {
            pub light_handle: u32,
            pub params: LightParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateMapLightResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateModelLightQuery {
            pub light_handle: u32,
            pub params: LightParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateModelLightResult {
            pub success: bool,
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
        pub fn add_light_tracking_target(light_handle: u32, object_id: i32, track_unit: bool, enable_tracking: bool) -> Result<bool> {
            let value = crate::generated::lights::add_light_tracking_target(light_handle, object_id, track_unit, enable_tracking)?;
            Ok(value)
        }

        #[inline]
        pub fn add_map_light(params: &LightParams) -> Result<u32> {
            let value = crate::generated::lights::add_map_light(crate::generated::lights::LightParams { position: params.position.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

        #[inline]
        pub fn add_model_light(params: &LightParams) -> Result<u32> {
            let value = crate::generated::lights::add_model_light(crate::generated::lights::LightParams { position: params.position.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

        #[inline]
        pub fn set_map_light_tracking_state(light_handle: u32, object_id: i32, enable_tracking: bool, track_unit: bool) -> Result<bool> {
            let value = crate::generated::lights::set_map_light_tracking_state(light_handle, object_id, enable_tracking, track_unit)?;
            Ok(value)
        }

        #[inline]
        pub fn set_model_light_tracking_state(light_handle: u32, object_id: i32, enable_tracking: bool, track_unit: bool) -> Result<bool> {
            let value = crate::generated::lights::set_model_light_tracking_state(light_handle, object_id, enable_tracking, track_unit)?;
            Ok(value)
        }

        #[inline]
        pub fn update_map_light(light_handle: u32, params: &LightParams) -> Result<bool> {
            let value = crate::generated::lights::update_map_light(light_handle, crate::generated::lights::LightParams { position: params.position.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

        #[inline]
        pub fn update_model_light(light_handle: u32, params: &LightParams) -> Result<bool> {
            let value = crate::generated::lights::update_model_light(light_handle, crate::generated::lights::LightParams { position: params.position.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

    }


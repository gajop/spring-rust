    pub mod tracing {
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TraceFlags {
            TraceFeatures,
            TraceGround,
            TraceNoEnemyUnits,
            TraceOnlyEnemy,
            TraceSky,
            TraceUnits,
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
        pub struct Ray {
            pub origin: Float3,
            pub direction: Float3,
            pub length: f32,
            pub flags: u32,
            pub ally_team_id: i32,
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
        pub struct TraceRayBetweenPositionsQuery {
            pub start: Float3,
            pub end: Float3,
            pub type_: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayBetweenPositionsResult {
            pub hits: Vec<TraceRayHit>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayFeaturesQuery {
            pub ray: Ray,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayFeaturesResult {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundBetweenPositionsOptions {
            pub test_water: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundBetweenPositionsQuery {
            pub start: Float3,
            pub end: Float3,
            pub options: TraceRayGroundBetweenPositionsOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundBetweenPositionsResult {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundInDirectionOptions {
            pub length: Option<f32>,
            pub test_water: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundInDirectionQuery {
            pub start: Float3,
            pub dir: Float3,
            pub options: TraceRayGroundInDirectionOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundInDirectionResult {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayHit {
            pub hit_length: f32,
            pub object_id: i32,
            pub object_type: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayInDirectionOptions {
            pub max_length: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayInDirectionQuery {
            pub pos: Float3,
            pub dir: Float3,
            pub options: TraceRayInDirectionOptions,
            pub type_: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayInDirectionResult {
            pub hits: Vec<TraceRayHit>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayQuery {
            pub ray: Ray,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayResult {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayUnitsQuery {
            pub ray: Ray,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayUnitsResult {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayValue {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayFeaturesValue {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundBetweenPositionsValue {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundInDirectionValue {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayUnitsValue {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[inline]
        pub fn trace_ray(ray: Ray) -> Result<TraceRayValue> {
            let value = crate::generated::tracing::trace_ray(crate::generated::tracing::Ray { origin: crate::generated::tracing::Float3 { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z }, direction: crate::generated::tracing::Float3 { x: ray.direction.x, y: ray.direction.y, z: ray.direction.z }, length: ray.length, flags: ray.flags, ally_team_id: ray.ally_team_id })?;
            Ok(TraceRayValue {
                hit: value.0,
                hit_type: value.1,
                hit_id: value.2,
                hit_pos: Float3 { x: value.3.x, y: value.3.y, z: value.3.z },
                hit_normal: Float3 { x: value.4.x, y: value.4.y, z: value.4.z }
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_trace_ray_between_positions {
            #[link(wasm_import_module = "spring:tracing")]
            extern "C" {
                #[link_name = "trace-ray-between-positions"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:tracing.trace-ray-between-positions."]
        #[inline]
        pub unsafe fn trace_ray_between_positions(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_trace_ray_between_positions::call(p0, p1) }
        }

        #[inline]
        pub fn trace_ray_features(ray: Ray) -> Result<TraceRayFeaturesValue> {
            let value = crate::generated::tracing::trace_ray_features(crate::generated::tracing::Ray { origin: crate::generated::tracing::Float3 { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z }, direction: crate::generated::tracing::Float3 { x: ray.direction.x, y: ray.direction.y, z: ray.direction.z }, length: ray.length, flags: ray.flags, ally_team_id: ray.ally_team_id })?;
            Ok(TraceRayFeaturesValue {
                hit: value.0,
                hit_type: value.1,
                hit_id: value.2,
                hit_pos: Float3 { x: value.3.x, y: value.3.y, z: value.3.z },
                hit_normal: Float3 { x: value.4.x, y: value.4.y, z: value.4.z }
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_trace_ray_ground_between_positions {
            #[link(wasm_import_module = "spring:tracing")]
            extern "C" {
                #[link_name = "trace-ray-ground-between-positions"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:tracing.trace-ray-ground-between-positions."]
        #[inline]
        pub unsafe fn trace_ray_ground_between_positions(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_trace_ray_ground_between_positions::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_trace_ray_ground_in_direction {
            #[link(wasm_import_module = "spring:tracing")]
            extern "C" {
                #[link_name = "trace-ray-ground-in-direction"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:tracing.trace-ray-ground-in-direction."]
        #[inline]
        pub unsafe fn trace_ray_ground_in_direction(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_trace_ray_ground_in_direction::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_trace_ray_in_direction {
            #[link(wasm_import_module = "spring:tracing")]
            extern "C" {
                #[link_name = "trace-ray-in-direction"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:tracing.trace-ray-in-direction."]
        #[inline]
        pub unsafe fn trace_ray_in_direction(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_trace_ray_in_direction::call(p0, p1) }
        }

        #[inline]
        pub fn trace_ray_units(ray: Ray) -> Result<TraceRayUnitsValue> {
            let value = crate::generated::tracing::trace_ray_units(crate::generated::tracing::Ray { origin: crate::generated::tracing::Float3 { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z }, direction: crate::generated::tracing::Float3 { x: ray.direction.x, y: ray.direction.y, z: ray.direction.z }, length: ray.length, flags: ray.flags, ally_team_id: ray.ally_team_id })?;
            Ok(TraceRayUnitsValue {
                hit: value.0,
                hit_type: value.1,
                hit_id: value.2,
                hit_pos: Float3 { x: value.3.x, y: value.3.y, z: value.3.z },
                hit_normal: Float3 { x: value.4.x, y: value.4.y, z: value.4.z }
            })
        }

    }


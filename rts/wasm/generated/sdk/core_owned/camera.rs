    pub mod camera {
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
        pub struct CameraState {
            pub name: String,
            pub pos: Float3,
            pub dir: Float3,
            pub up: Float3,
            pub right: Float3,
            pub fov: f32,
            pub rx: f32,
            pub ry: f32,
            pub rz: f32,
            pub dist: f32,
            pub height: f32,
            pub angle: f32,
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
        pub struct GetCameraDirectionQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraFOVQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraFOVResult {
            pub fov: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraNamesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraNamesResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraPositionQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraPositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraStateQuery {
            pub use_table: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraStateResult {
            pub state: CameraState,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPixelDirQuery {
            pub screen_x: f32,
            pub screen_y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPixelDirResult {
            pub direction: Float3,
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
        pub struct SetCameraStateQuery {
            pub state: CameraState,
            pub transition_time: f32,
            pub transition_time_factor: f32,
            pub transition_time_exponent: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraTargetOptions {
            pub transition_time: Option<f32>,
            pub dir_x: Option<f32>,
            pub dir_y: Option<f32>,
            pub dir_z: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraTargetQuery {
            pub target: Float3,
            pub options: SetCameraTargetOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraTargetResult {
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
        pub struct TraceScreenRayOptions {
            pub only_coords: bool,
            pub use_minimap: bool,
            pub include_sky: bool,
            pub ignore_water: bool,
            pub height_offset: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceScreenRayQuery {
            pub screen_x: f32,
            pub screen_y: f32,
            pub options: TraceScreenRayOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceScreenRayResult {
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
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
        pub struct WorldToScreenCoordsQuery {
            pub world_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WorldToScreenCoordsResult {
            pub screen_pos: Float3,
            pub valid: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceScreenRayValue {
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WorldToScreenCoordsValue {
            pub screen_pos: Float3,
            pub valid: bool,
        }

        #[inline]
        pub fn get_camera_direction(unused: u8) -> Result<Float3> {
            let value = crate::generated::camera::get_camera_direction(unused)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_camera_fov(unused: u8) -> Result<f32> {
            let value = crate::generated::camera::get_camera_fov(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_camera_names {
            #[link(wasm_import_module = "spring:camera")]
            extern "C" {
                #[link_name = "get-camera-names"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:camera.get-camera-names."]
        #[inline]
        pub unsafe fn get_camera_names(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_camera_names::call(p0, p1) }
        }

        #[inline]
        pub fn get_camera_position(unused: u8) -> Result<Float3> {
            let value = crate::generated::camera::get_camera_position(unused)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_camera_state {
            #[link(wasm_import_module = "spring:camera")]
            extern "C" {
                #[link_name = "get-camera-state"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:camera.get-camera-state."]
        #[inline]
        pub unsafe fn get_camera_state(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_camera_state::call(p0, p1) }
        }

        #[inline]
        pub fn get_pixel_dir(screen_x: f32, screen_y: f32) -> Result<Float3> {
            let value = crate::generated::camera::get_pixel_dir(screen_x, screen_y)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_camera_state {
            #[link(wasm_import_module = "spring:camera")]
            extern "C" {
                #[link_name = "set-camera-state"]
                pub fn call(p0: f32, p1: f32, p2: f32, p3: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:camera.set-camera-state."]
        #[inline]
        pub unsafe fn set_camera_state(p0: f32, p1: f32, p2: f32, p3: i32) -> i64 {
            unsafe { __core_owned_set_camera_state::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_camera_target {
            #[link(wasm_import_module = "spring:camera")]
            extern "C" {
                #[link_name = "set-camera-target"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:camera.set-camera-target."]
        #[inline]
        pub unsafe fn set_camera_target(p0: i32) -> i64 {
            unsafe { __core_owned_set_camera_target::call(p0) }
        }

        #[inline]
        pub fn trace_screen_ray(screen_x: f32, screen_y: f32, options: TraceScreenRayOptions) -> Result<TraceScreenRayValue> {
            let value = crate::generated::camera::trace_screen_ray(screen_x, screen_y, crate::generated::camera::TraceScreenRayOptions { only_coords: options.only_coords, use_minimap: options.use_minimap, include_sky: options.include_sky, ignore_water: options.ignore_water, height_offset: options.height_offset })?;
            Ok(TraceScreenRayValue {
                hit_type: value.0,
                hit_id: value.1,
                hit_pos: Float3 { x: value.2.x, y: value.2.y, z: value.2.z }
            })
        }

        #[inline]
        pub fn world_to_screen_coords(world_pos: Float3) -> Result<WorldToScreenCoordsValue> {
            let value = crate::generated::camera::world_to_screen_coords(crate::generated::camera::Float3 { x: world_pos.x, y: world_pos.y, z: world_pos.z })?;
            Ok(WorldToScreenCoordsValue {
                screen_pos: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                valid: value.1
            })
        }

    }


    pub mod sound {
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
        pub struct GetSoundDevicesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSoundDevicesResult {
            pub devices: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSoundEffectParamsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSoundEffectParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSoundStreamTimeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSoundStreamTimeResult {
            pub time: f32,
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
        pub struct LoadSoundDefQuery {
            pub sound_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadSoundDefResult {
            pub success: bool,
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
        pub struct PauseSoundStreamQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PauseSoundStreamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlaySoundFileQuery {
            pub sound_file: String,
            pub volume: f32,
            pub pos: Float3,
            pub velocity: Float3,
            pub channel: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlaySoundFileResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlaySoundStreamQuery {
            pub ogg_file: String,
            pub volume: f32,
            pub enqueue: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlaySoundStreamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PreloadSoundItemQuery {
            pub sound_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PreloadSoundItemResult {
            pub success: bool,
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
        pub struct SetSoundEffectParamsQuery {
            pub params: SoundEffectParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSoundEffectParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSoundStreamVolumeQuery {
            pub volume: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSoundStreamVolumeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StopSoundStreamQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StopSoundStreamResult {
            pub success: bool,
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_sound_devices {
            #[link(wasm_import_module = "spring:sound")]
            extern "C" {
                #[link_name = "get-sound-devices"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:sound.get-sound-devices."]
        #[inline]
        pub unsafe fn get_sound_devices(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_sound_devices::call(p0, p1) }
        }

        #[inline]
        pub fn get_sound_effect_params(unused: u8) -> Result<bool> {
            let value = crate::generated::sound::get_sound_effect_params(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_sound_stream_time(unused: u8) -> Result<f32> {
            let value = crate::generated::sound::get_sound_stream_time(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn load_sound_def(sound_name: &str) -> Result<bool> {
            let mut sound_name_bytes = sound_name.as_bytes().to_vec();
            if sound_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            sound_name_bytes.push(0);
            let sound_name_cstr = core::ffi::CStr::from_bytes_with_nul(&sound_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::sound::load_sound_def(&sound_name_cstr)
        }

        #[inline]
        pub fn pause_sound_stream(unused: u8) -> Result<bool> {
            let value = crate::generated::sound::pause_sound_stream(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_play_sound_file {
            #[link(wasm_import_module = "spring:sound")]
            extern "C" {
                #[link_name = "play-sound-file"]
                pub fn call(p0: f32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:sound.play-sound-file."]
        #[inline]
        pub unsafe fn play_sound_file(p0: f32, p1: i32, p2: i32) -> i64 {
            unsafe { __core_owned_play_sound_file::call(p0, p1, p2) }
        }

        #[inline]
        pub fn play_sound_stream(ogg_file: &str, volume: f32, enqueue: bool) -> Result<bool> {
            let mut ogg_file_bytes = ogg_file.as_bytes().to_vec();
            if ogg_file_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            ogg_file_bytes.push(0);
            let ogg_file_cstr = core::ffi::CStr::from_bytes_with_nul(&ogg_file_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::sound::play_sound_stream(&ogg_file_cstr, volume, enqueue)
        }

        #[inline]
        pub fn preload_sound_item(sound_name: &str) -> Result<bool> {
            let mut sound_name_bytes = sound_name.as_bytes().to_vec();
            if sound_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            sound_name_bytes.push(0);
            let sound_name_cstr = core::ffi::CStr::from_bytes_with_nul(&sound_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::sound::preload_sound_item(&sound_name_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_sound_effect_params {
            #[link(wasm_import_module = "spring:sound")]
            extern "C" {
                #[link_name = "set-sound-effect-params"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:sound.set-sound-effect-params."]
        #[inline]
        pub unsafe fn set_sound_effect_params(p0: i32) -> i64 {
            unsafe { __core_owned_set_sound_effect_params::call(p0) }
        }

        #[inline]
        pub fn set_sound_stream_volume(volume: f32) -> Result<bool> {
            let value = crate::generated::sound::set_sound_stream_volume(volume)?;
            Ok(value)
        }

        #[inline]
        pub fn stop_sound_stream(unused: u8) -> Result<bool> {
            let value = crate::generated::sound::stop_sound_stream(unused)?;
            Ok(value)
        }

    }


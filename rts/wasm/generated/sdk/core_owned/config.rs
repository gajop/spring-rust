    pub mod config {
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
        pub enum ConfigValueType {
            ConfigTypeBool,
            ConfigTypeFloat,
            ConfigTypeInt,
            ConfigTypeString,
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
        pub struct ConfigParam {
            pub name: String,
            pub type_: ConfigValueType,
            pub description: String,
            pub default_value: String,
            pub minimum_value: String,
            pub maximum_value: String,
            pub read_only: bool,
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
        pub struct GetConfigFloatQuery {
            pub key: String,
            pub default_value: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigFloatResult {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigIntQuery {
            pub key: String,
            pub default_value: Option<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigIntResult {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigParamsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigParamsResult {
            pub params: Vec<ConfigParam>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigStringQuery {
            pub key: String,
            pub default_value: Option<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigStringResult {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLogSectionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLogSectionsResult {
            pub sections: Vec<String>,
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
        pub struct SetConfigFloatQuery {
            pub key: String,
            pub value: f32,
            pub use_overlay: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigFloatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigIntQuery {
            pub key: String,
            pub value: i32,
            pub use_overlay: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigIntResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigStringQuery {
            pub key: String,
            pub value: String,
            pub use_overlay: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigStringResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetLogSectionFilterLevelQuery {
            pub section: String,
            pub level: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetLogSectionFilterLevelResult {
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
        pub struct GetConfigFloatValue {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigIntValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigStringValue {
            pub value: String,
            pub exists: bool,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_config_float {
            #[link(wasm_import_module = "spring:config")]
            extern "C" {
                #[link_name = "get-config-float"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-config-float."]
        #[inline]
        pub unsafe fn get_config_float(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_config_float::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_config_int {
            #[link(wasm_import_module = "spring:config")]
            extern "C" {
                #[link_name = "get-config-int"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-config-int."]
        #[inline]
        pub unsafe fn get_config_int(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_config_int::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_config_params {
            #[link(wasm_import_module = "spring:config")]
            extern "C" {
                #[link_name = "get-config-params"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-config-params."]
        #[inline]
        pub unsafe fn get_config_params(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_config_params::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_config_string {
            #[link(wasm_import_module = "spring:config")]
            extern "C" {
                #[link_name = "get-config-string"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-config-string."]
        #[inline]
        pub unsafe fn get_config_string(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_config_string::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_log_sections {
            #[link(wasm_import_module = "spring:config")]
            extern "C" {
                #[link_name = "get-log-sections"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-log-sections."]
        #[inline]
        pub unsafe fn get_log_sections(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_log_sections::call(p0, p1) }
        }

        #[inline]
        pub fn set_config_float(key: &str, value: f32, use_overlay: bool) -> Result<bool> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::config::set_config_float(&key_cstr, value, use_overlay)
        }

        #[inline]
        pub fn set_config_int(key: &str, value: i32, use_overlay: bool) -> Result<bool> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::config::set_config_int(&key_cstr, value, use_overlay)
        }

        #[inline]
        pub fn set_config_string(key: &str, value: &str, use_overlay: bool) -> Result<bool> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::config::set_config_string(&key_cstr, &value_cstr, use_overlay)
        }

        #[inline]
        pub fn set_log_section_filter_level(section: &str, level: i32) -> Result<bool> {
            let mut section_bytes = section.as_bytes().to_vec();
            if section_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            section_bytes.push(0);
            let section_cstr = core::ffi::CStr::from_bytes_with_nul(&section_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::config::set_log_section_filter_level(&section_cstr, level)
        }

    }


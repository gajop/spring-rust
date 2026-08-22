    pub mod feature_defs {
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
        pub struct FeatureDefInfo {
            pub id: i32,
            pub name: String,
            pub description: String,
            pub tooltip: String,
            pub metal: f32,
            pub energy: f32,
            pub max_health: f32,
            pub reclaim_time: f32,
            pub mass: f32,
            pub destructable: bool,
            pub reclaimable: bool,
            pub blocking: bool,
            pub burnable: bool,
            pub floating: bool,
            pub geo_thermal: bool,
            pub model_name: String,
            pub resurrect_as: String,
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
        pub struct GetFeatureDefByIDQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefByIDResult {
            pub info: FeatureDefInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCustomParamKeysQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCustomParamKeysResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCustomParamQuery {
            pub feature_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCustomParamResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefEnergyQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefEnergyResult {
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDByNameQuery {
            pub feature_def_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDByNameResult {
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDsResult {
            pub ids: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefMetalQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefMetalResult {
            pub metal: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefNameQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefNameResult {
            pub name: String,
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
        pub struct ValidFeatureDefIDQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ValidFeatureDefIDResult {
            pub valid: bool,
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
        mod __core_variable_output_get_feature_def_i_ds {
            #[link(wasm_import_module = "spring:feature-defs")]
            extern "C" {
                #[link_name = "get-feature-def-i-ds"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_feature_def_name {
            #[link(wasm_import_module = "spring:feature-defs")]
            extern "C" {
                #[link_name = "get-feature-def-name"]
                pub fn call(pfeature_def_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefByIDValue {
            pub info: FeatureDefInfo,
            pub exists: bool,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_def_by_id {
            #[link(wasm_import_module = "spring:feature-defs")]
            extern "C" {
                #[link_name = "get-feature-def-by-id"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:feature-defs.get-feature-def-by-id."]
        #[inline]
        pub unsafe fn get_feature_def_by_id(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_feature_def_by_id::call(p0, p1) }
        }

        #[inline]
        pub fn get_feature_def_count(unused: u8) -> Result<u32> {
            let value = crate::generated::feature_defs::get_feature_def_count(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_def_custom_param {
            #[link(wasm_import_module = "spring:feature-defs")]
            extern "C" {
                #[link_name = "get-feature-def-custom-param"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:feature-defs.get-feature-def-custom-param."]
        #[inline]
        pub unsafe fn get_feature_def_custom_param(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_feature_def_custom_param::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_def_custom_param_keys {
            #[link(wasm_import_module = "spring:feature-defs")]
            extern "C" {
                #[link_name = "get-feature-def-custom-param-keys"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:feature-defs.get-feature-def-custom-param-keys."]
        #[inline]
        pub unsafe fn get_feature_def_custom_param_keys(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_feature_def_custom_param_keys::call(p0, p1) }
        }

        #[inline]
        pub fn get_feature_def_energy(feature_def_id: i32) -> Result<f32> {
            let value = crate::generated::feature_defs::get_feature_def_energy(feature_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_def_id_by_name(feature_def_name: &str) -> Result<i32> {
            let mut feature_def_name_bytes = feature_def_name.as_bytes().to_vec();
            if feature_def_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            feature_def_name_bytes.push(0);
            let feature_def_name_cstr = core::ffi::CStr::from_bytes_with_nul(&feature_def_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::feature_defs::get_feature_def_id_by_name(&feature_def_name_cstr)
        }

        #[inline]
        pub fn get_feature_def_i_ds(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_feature_def_i_ds::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_feature_def_metal(feature_def_id: i32) -> Result<f32> {
            let value = crate::generated::feature_defs::get_feature_def_metal(feature_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_def_name(feature_def_id: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_feature_def_name::call(feature_def_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (feature_def_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn valid_feature_def_id(feature_def_id: i32) -> Result<bool> {
            let value = crate::generated::feature_defs::valid_feature_def_id(feature_def_id)?;
            Ok(value)
        }

    }


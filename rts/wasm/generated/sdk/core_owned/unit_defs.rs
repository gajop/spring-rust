    pub mod unit_defs {
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
        pub enum UnitDefParamType {
            UnitDefParamBool,
            UnitDefParamFloat,
            UnitDefParamInt,
            UnitDefParamMissing,
            UnitDefParamString,
            UnitDefParamTable,
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
        pub struct GetUnitDefByIDQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefByIDResult {
            pub exists: bool,
            pub basic: UnitDefBasicInfo,
            pub costs: UnitDefCosts,
            pub physics: UnitDefPhysics,
            pub weapons: UnitDefWeapons,
            pub build_options: UnitDefBuildOptions,
            pub sensors: UnitDefSensors,
            pub health: UnitDefHealth,
            pub classify: UnitDefClassify,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefClassifyQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefClassifyResult {
            pub classify: UnitDefClassify,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCostsQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCostsResult {
            pub costs: UnitDefCosts,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCustomParamKeysQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCustomParamKeysResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCustomParamQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCustomParamResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefHealthQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefHealthResult {
            pub health: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefHumanNameQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefHumanNameResult {
            pub human_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefIDByNameQuery {
            pub unit_def_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefIDByNameResult {
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefIDsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefIDsResult {
            pub ids: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefNameQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamBoolQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamBoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamFloatQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamIntQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamIntResult {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamKeysQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamKeysResult {
            pub keys: Vec<UnitDefParamKey>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamStringQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamStringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamTypeQuery {
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamTypeResult {
            pub type_: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefSpeedQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefSpeedResult {
            pub speed: f32,
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
        pub struct UnitDefBasicInfo {
            pub id: i32,
            pub name: String,
            pub human_name: String,
            pub tooltip: String,
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefBuildOptions {
            pub buildable_unit_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefClassify {
            pub is_transport: bool,
            pub is_immobile: bool,
            pub is_building: bool,
            pub is_builder: bool,
            pub is_mobile_builder: bool,
            pub is_static_builder: bool,
            pub is_factory: bool,
            pub is_extractor: bool,
            pub is_ground_unit: bool,
            pub is_air_unit: bool,
            pub is_strafing_air_unit: bool,
            pub is_hovering_air_unit: bool,
            pub is_fighter_air_unit: bool,
            pub is_bomber_air_unit: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefCosts {
            pub metal_cost: f32,
            pub energy_cost: f32,
            pub build_time: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefHealth {
            pub health: f32,
            pub auto_heal: f32,
            pub idle_auto_heal: f32,
            pub idle_time: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefParamKey {
            pub name: String,
            pub type_: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefPhysics {
            pub mass: f32,
            pub height: f32,
            pub radius: f32,
            pub speed: f32,
            pub turn_rate: f32,
            pub acceleration: f32,
            pub brake_rate: f32,
            pub can_fly: bool,
            pub can_move: bool,
            pub can_hover: bool,
            pub float_on_water: bool,
            pub move_def_id: i32,
            pub can_submerge: bool,
            pub waterline: f32,
            pub min_water_depth: f32,
            pub max_water_depth: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefSensors {
            pub los_radius: f32,
            pub air_los_radius: f32,
            pub radar_radius: f32,
            pub sonar_radius: f32,
            pub seismic_radius: f32,
            pub radar_jammer_radius: f32,
            pub sonar_jammer_radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefWeapons {
            pub weapon_def_i_ds: Vec<i32>,
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
        pub struct ValidUnitDefIDQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ValidUnitDefIDResult {
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
        mod __core_variable_output_get_unit_def_i_ds {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-i-ds"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefByIDValue {
            pub exists: bool,
            pub basic: UnitDefBasicInfo,
            pub costs: UnitDefCosts,
            pub physics: UnitDefPhysics,
            pub weapons: UnitDefWeapons,
            pub build_options: UnitDefBuildOptions,
            pub sensors: UnitDefSensors,
            pub health: UnitDefHealth,
            pub classify: UnitDefClassify,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_by_id {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-by-id"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-by-id."]
        #[inline]
        pub unsafe fn get_unit_def_by_id(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_def_by_id::call(p0, p1) }
        }

        #[inline]
        pub fn get_unit_def_classify(unit_def_id: i32) -> Result<UnitDefClassify> {
            let value = crate::generated::unit_defs::get_unit_def_classify(unit_def_id)?;
            Ok(UnitDefClassify { is_transport: value.is_transport, is_immobile: value.is_immobile, is_building: value.is_building, is_builder: value.is_builder, is_mobile_builder: value.is_mobile_builder, is_static_builder: value.is_static_builder, is_factory: value.is_factory, is_extractor: value.is_extractor, is_ground_unit: value.is_ground_unit, is_air_unit: value.is_air_unit, is_strafing_air_unit: value.is_strafing_air_unit, is_hovering_air_unit: value.is_hovering_air_unit, is_fighter_air_unit: value.is_fighter_air_unit, is_bomber_air_unit: value.is_bomber_air_unit })
        }

        #[inline]
        pub fn get_unit_def_costs(unit_def_id: i32) -> Result<UnitDefCosts> {
            let value = crate::generated::unit_defs::get_unit_def_costs(unit_def_id)?;
            Ok(UnitDefCosts { metal_cost: value.metal_cost, energy_cost: value.energy_cost, build_time: value.build_time })
        }

        #[inline]
        pub fn get_unit_def_count(unused: u8) -> Result<u32> {
            let value = crate::generated::unit_defs::get_unit_def_count(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_custom_param {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-custom-param"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-custom-param."]
        #[inline]
        pub unsafe fn get_unit_def_custom_param(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_def_custom_param::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_custom_param_keys {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-custom-param-keys"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-custom-param-keys."]
        #[inline]
        pub unsafe fn get_unit_def_custom_param_keys(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_def_custom_param_keys::call(p0, p1) }
        }

        #[inline]
        pub fn get_unit_def_health(unit_def_id: i32) -> Result<f32> {
            let value = crate::generated::unit_defs::get_unit_def_health(unit_def_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_human_name {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-human-name"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-human-name."]
        #[inline]
        pub unsafe fn get_unit_def_human_name(p0: i32, p1: i32, p2: i32) -> i64 {
            unsafe { __core_owned_get_unit_def_human_name::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_unit_def_id_by_name(unit_def_name: &str) -> Result<i32> {
            let mut unit_def_name_bytes = unit_def_name.as_bytes().to_vec();
            if unit_def_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            unit_def_name_bytes.push(0);
            let unit_def_name_cstr = core::ffi::CStr::from_bytes_with_nul(&unit_def_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unit_defs::get_unit_def_id_by_name(&unit_def_name_cstr)
        }

        #[inline]
        pub fn get_unit_def_i_ds(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_unit_def_i_ds::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_name {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-name"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-name."]
        #[inline]
        pub unsafe fn get_unit_def_name(p0: i32, p1: i32, p2: i32) -> i64 {
            unsafe { __core_owned_get_unit_def_name::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_unit_def_param_bool(unit_def_id: i32, key: &str) -> Result<bool> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unit_defs::get_unit_def_param_bool(unit_def_id, &key_cstr)
        }

        #[inline]
        pub fn get_unit_def_param_float(unit_def_id: i32, key: &str) -> Result<f32> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unit_defs::get_unit_def_param_float(unit_def_id, &key_cstr)
        }

        #[inline]
        pub fn get_unit_def_param_int(unit_def_id: i32, key: &str) -> Result<i32> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unit_defs::get_unit_def_param_int(unit_def_id, &key_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_param_keys {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-param-keys"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-param-keys."]
        #[inline]
        pub unsafe fn get_unit_def_param_keys(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_def_param_keys::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_param_string {
            #[link(wasm_import_module = "spring:unit-defs")]
            extern "C" {
                #[link_name = "get-unit-def-param-string"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-param-string."]
        #[inline]
        pub unsafe fn get_unit_def_param_string(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_def_param_string::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_unit_def_param_type(key: &str) -> Result<i32> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unit_defs::get_unit_def_param_type(&key_cstr)
        }

        #[inline]
        pub fn get_unit_def_speed(unit_def_id: i32) -> Result<f32> {
            let value = crate::generated::unit_defs::get_unit_def_speed(unit_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn valid_unit_def_id(unit_def_id: i32) -> Result<bool> {
            let value = crate::generated::unit_defs::valid_unit_def_id(unit_def_id)?;
            Ok(value)
        }

    }


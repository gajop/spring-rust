    pub mod path_finder {
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
        pub struct DeletePathQuery {
            pub path_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DeletePathResult {
            pub success: bool,
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
        pub struct FreePathNodeCostsArrayQuery {
            pub overlay_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreePathNodeCostsArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNextWayPointQuery {
            pub path_id: u32,
            pub caller_pos: Float3,
            pub min_dist: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNextWayPointResult {
            pub waypoint: Option<Float3>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathNodeCostQuery {
            pub x: u32,
            pub z: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathNodeCostResult {
            pub cost: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathNodeCostsQuery {
            pub overlay_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathNodeCostsResult {
            pub costs: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathWayPointsQuery {
            pub path_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathWayPointsResult {
            pub points: Vec<Float3>,
            pub starts: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct InitPathNodeCostsArrayQuery {
            pub overlay_index: u32,
            pub size_x: u32,
            pub size_z: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct InitPathNodeCostsArrayResult {
            pub success: bool,
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
        pub struct RequestPathQuery {
            pub move_def_id: u32,
            pub move_def_name: Option<String>,
            pub start_pos: Float3,
            pub end_pos: Float3,
            pub radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RequestPathResult {
            pub path_id: u32,
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
        pub struct SetPathNodeCostQuery {
            pub overlay_index: u32,
            pub cost_index: u32,
            pub cost: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetPathNodeCostResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetPathNodeCostsQuery {
            pub overlay_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetPathNodeCostsResult {
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

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_path_node_costs {
            #[link(wasm_import_module = "spring:path-finder")]
            extern "C" {
                #[link_name = "get-path-node-costs"]
                pub fn call(poverlay_index: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathWayPointsValue {
            pub points: Vec<Float3>,
            pub starts: Vec<i32>,
        }

        #[inline]
        pub fn delete_path(path_id: u32) -> Result<bool> {
            let value = crate::generated::path_finder::delete_path(path_id)?;
            Ok(value)
        }

        #[inline]
        pub fn free_path_node_costs_array(overlay_index: u32) -> Result<bool> {
            let value = crate::generated::path_finder::free_path_node_costs_array(overlay_index)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_next_way_point {
            #[link(wasm_import_module = "spring:path-finder")]
            extern "C" {
                #[link_name = "get-next-way-point"]
                pub fn call(p0: i32, p1: f32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:path-finder.get-next-way-point."]
        #[inline]
        pub unsafe fn get_next_way_point(p0: i32, p1: f32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_get_next_way_point::call(p0, p1, p2, p3) }
        }

        #[inline]
        pub fn get_path_node_cost(x: u32, z: u32) -> Result<f32> {
            let value = crate::generated::path_finder::get_path_node_cost(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_path_node_costs(overlay_index: u32) -> Result<Vec<f32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<f32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_path_node_costs::call(overlay_index as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (overlay_index as i32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_path_way_points {
            #[link(wasm_import_module = "spring:path-finder")]
            extern "C" {
                #[link_name = "get-path-way-points"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:path-finder.get-path-way-points."]
        #[inline]
        pub unsafe fn get_path_way_points(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_path_way_points::call(p0, p1) }
        }

        #[inline]
        pub fn init_path_node_costs_array(overlay_index: u32, size_x: u32, size_z: u32) -> Result<bool> {
            let value = crate::generated::path_finder::init_path_node_costs_array(overlay_index, size_x, size_z)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_request_path {
            #[link(wasm_import_module = "spring:path-finder")]
            extern "C" {
                #[link_name = "request-path"]
                pub fn call(p0: i32, p1: f32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:path-finder.request-path."]
        #[inline]
        pub unsafe fn request_path(p0: i32, p1: f32, p2: i32) -> i64 {
            unsafe { __core_owned_request_path::call(p0, p1, p2) }
        }

        #[inline]
        pub fn set_path_node_cost(overlay_index: u32, cost_index: u32, cost: f32) -> Result<bool> {
            let value = crate::generated::path_finder::set_path_node_cost(overlay_index, cost_index, cost)?;
            Ok(value)
        }

        #[inline]
        pub fn set_path_node_costs(overlay_index: u32) -> Result<bool> {
            let value = crate::generated::path_finder::set_path_node_costs(overlay_index)?;
            Ok(value)
        }

    }


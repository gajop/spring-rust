    pub mod terrain {
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
        pub struct GetGrassQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGrassResult {
            pub grass_level: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundBlockedQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundBlockedResult {
            pub blocked: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundExtremesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundExtremesResult {
            pub init_min_height: f32,
            pub init_max_height: f32,
            pub curr_min_height: f32,
            pub curr_max_height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundHeightQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundInfoQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundInfoResult {
            pub terrain_type_index: i32,
            pub terrain_type_name: String,
            pub metal_extraction: f32,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundNormalQuery {
            pub x: f32,
            pub z: f32,
            pub smoothed: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundNormalResult {
            pub normal: Float3,
            pub slope: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundOrigHeightQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundOrigHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHeightMapSizeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHeightMapSizeResult {
            pub points_x: i32,
            pub points_z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSmoothMeshHeightQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSmoothMeshHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTerrainTypeDataQuery {
            pub terrain_type_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTerrainTypeDataResult {
            pub index: i32,
            pub name: String,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterLevelQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterLevelResult {
            pub level: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterPlaneLevelQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterPlaneLevelResult {
            pub level: f32,
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
        pub struct IsPosInMapQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPosInMapResult {
            pub in_map: bool,
            pub in_play_area: bool,
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
        pub struct GetGroundExtremesValue {
            pub init_min_height: f32,
            pub init_max_height: f32,
            pub curr_min_height: f32,
            pub curr_max_height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundInfoValue {
            pub terrain_type_index: i32,
            pub terrain_type_name: String,
            pub metal_extraction: f32,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundNormalValue {
            pub normal: Float3,
            pub slope: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHeightMapSizeValue {
            pub points_x: i32,
            pub points_z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTerrainTypeDataValue {
            pub index: i32,
            pub name: String,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsPosInMapValue {
            pub in_map: bool,
            pub in_play_area: bool,
        }

        #[inline]
        pub fn get_grass(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_grass(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_ground_blocked(x1: f32, z1: f32, x2: f32, z2: f32) -> Result<bool> {
            let value = crate::generated::terrain::get_ground_blocked(x1, z1, x2, z2)?;
            Ok(value)
        }

        #[inline]
        pub fn get_ground_extremes(_unused: u8) -> Result<GetGroundExtremesValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = [0u8; 16];
                let status = unsafe { __core_terrain_ground_extremes::call(output.as_mut_ptr() as usize as u32 as i32) };
                if status != 0 { return Err(crate::ApiError::new(status)); }
                return Ok(GetGroundExtremesValue {
                    init_min_height: f32::from_le_bytes(output[0..4].try_into().unwrap()),
                    init_max_height: f32::from_le_bytes(output[4..8].try_into().unwrap()),
                    curr_min_height: f32::from_le_bytes(output[8..12].try_into().unwrap()),
                    curr_max_height: f32::from_le_bytes(output[12..16].try_into().unwrap()),
                });
            }
            #[cfg(not(target_arch = "wasm32"))]
            { Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_ground_extremes {
            #[link(wasm_import_module = "spring:terrain")]
            extern "C" {
                #[link_name = "get-ground-extremes"]
                pub fn call(output: i32) -> i32;
            }
        }

        #[inline]
        pub fn get_ground_height(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_ground_height(x, z)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_ground_info {
            #[link(wasm_import_module = "spring:terrain")]
            extern "C" {
                #[link_name = "get-ground-info"]
                pub fn call(p0: f32, p1: f32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:terrain.get-ground-info."]
        #[inline]
        pub unsafe fn get_ground_info(p0: f32, p1: f32, p2: i32) -> i32 {
            unsafe { __core_owned_get_ground_info::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_ground_normal(x: f32, z: f32, smoothed: bool) -> Result<GetGroundNormalValue> {
            let value = crate::generated::terrain::get_ground_normal(x, z, smoothed)?;
            Ok(GetGroundNormalValue {
                normal: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                slope: value.1
            })
        }

        #[inline]
        pub fn get_ground_orig_height(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_ground_orig_height(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_height_map_size(_unused: u8) -> Result<GetHeightMapSizeValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = [0u8; 8];
                let status = unsafe { __core_terrain_height_map_size::call(output.as_mut_ptr() as usize as u32 as i32) };
                if status != 0 { return Err(crate::ApiError::new(status)); }
                return Ok(GetHeightMapSizeValue {
                    points_x: i32::from_le_bytes(output[0..4].try_into().unwrap()),
                    points_z: i32::from_le_bytes(output[4..8].try_into().unwrap()),
                });
            }
            #[cfg(not(target_arch = "wasm32"))]
            { Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_height_map_size {
            #[link(wasm_import_module = "spring:terrain")]
            extern "C" {
                #[link_name = "get-height-map-size"]
                pub fn call(output: i32) -> i32;
            }
        }

        #[inline]
        pub fn get_smooth_mesh_height(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_smooth_mesh_height(x, z)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_terrain_type_data {
            #[link(wasm_import_module = "spring:terrain")]
            extern "C" {
                #[link_name = "get-terrain-type-data"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:terrain.get-terrain-type-data."]
        #[inline]
        pub unsafe fn get_terrain_type_data(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_terrain_type_data::call(p0, p1) }
        }

        #[inline]
        pub fn get_water_level(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_water_level(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_water_plane_level(_unused: u8) -> Result<f32> {
            #[cfg(target_arch = "wasm32")]
            {
                let packed = unsafe { __core_terrain_water_plane_level::call() } as u64;
                let status = (packed >> 32) as u32 as i32;
                if status != 0 { return Err(crate::ApiError::new(status)); }
                return Ok(f32::from_bits(packed as u32));
            }
            #[cfg(not(target_arch = "wasm32"))]
            { Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_water_plane_level {
            #[link(wasm_import_module = "spring:terrain")]
            extern "C" {
                #[link_name = "get-water-plane-level"]
                pub fn call() -> i64;
            }
        }

        #[inline]
        pub fn is_pos_in_map(x: f32, z: f32) -> Result<IsPosInMapValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let packed = unsafe { __core_terrain_is_pos_in_map::call(x, z) } as u64;
                let status = (packed >> 32) as u32 as i32;
                if status != 0 { return Err(crate::ApiError::new(status)); }
                let flags = packed as u32;
                return Ok(IsPosInMapValue { in_map: flags & 1 != 0, in_play_area: flags & 2 != 0 });
            }
            #[cfg(not(target_arch = "wasm32"))]
            { let _ = (x, z); Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_is_pos_in_map {
            #[link(wasm_import_module = "spring:terrain")]
            extern "C" {
                #[link_name = "is-pos-in-map"]
                pub fn call(x: f32, z: f32) -> i64;
            }
        }

    }


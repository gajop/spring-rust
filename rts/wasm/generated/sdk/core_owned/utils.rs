    pub mod utils {
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
        pub struct ClosestBuildPosQuery {
            pub team_id: i32,
            pub unit_def_id: i32,
            pub pos: Float3,
            pub search_radius: f32,
            pub min_dist: i32,
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClosestBuildPosResult {
            pub build_pos: Float3,
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
        pub struct GetCEGIDQuery {
            pub ceg_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCEGIDResult {
            pub ceg_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefDimensionsQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefDimensionsResult {
            pub dimensions: UnitDefDimensions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefDimensionsQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefDimensionsResult {
            pub dimensions: UnitDefDimensions,
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
        pub struct Pos2BuildPosQuery {
            pub unit_def_id: i32,
            pub pos: Float3,
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Pos2BuildPosResult {
            pub build_pos: Float3,
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
        pub struct TestBuildOrderQuery {
            pub unit_def_id: i32,
            pub pos: Float3,
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TestBuildOrderResult {
            pub status: i32,
            pub can_build: bool,
            pub feature: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TestMoveOrderOptions {
            pub test_terrain: bool,
            pub test_objects: bool,
            pub center_only: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TestMoveOrderQuery {
            pub unit_def_id: i32,
            pub pos: Float3,
            pub dir: Float3,
            pub options: TestMoveOrderOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TestMoveOrderResult {
            pub can_move: bool,
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
        pub struct UnitDefDimensions {
            pub height: f32,
            pub radius: f32,
            pub midx: f32,
            pub minx: f32,
            pub maxx: f32,
            pub midy: f32,
            pub miny: f32,
            pub maxy: f32,
            pub midz: f32,
            pub minz: f32,
            pub maxz: f32,
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
        pub struct TestBuildOrderValue {
            pub status: i32,
            pub can_build: bool,
            pub feature: i32,
        }

        #[inline]
        pub fn closest_build_pos(team_id: i32, unit_def_id: i32, pos: Float3, search_radius: f32, min_dist: i32, facing: i32) -> Result<Float3> {
            let value = crate::generated::utils::closest_build_pos(team_id, unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, search_radius, min_dist, facing)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_cegid(ceg_name: &str) -> Result<i32> {
            let mut ceg_name_bytes = ceg_name.as_bytes().to_vec();
            if ceg_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            ceg_name_bytes.push(0);
            let ceg_name_cstr = core::ffi::CStr::from_bytes_with_nul(&ceg_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::utils::get_cegid(&ceg_name_cstr)
        }

        #[inline]
        pub fn get_feature_def_dimensions(feature_def_id: i32) -> Result<UnitDefDimensions> {
            let value = crate::generated::utils::get_feature_def_dimensions(feature_def_id)?;
            Ok(UnitDefDimensions { height: value.height, radius: value.radius, midx: value.midx, minx: value.minx, maxx: value.maxx, midy: value.midy, miny: value.miny, maxy: value.maxy, midz: value.midz, minz: value.minz, maxz: value.maxz })
        }

        #[inline]
        pub fn get_unit_def_dimensions(unit_def_id: i32) -> Result<UnitDefDimensions> {
            let value = crate::generated::utils::get_unit_def_dimensions(unit_def_id)?;
            Ok(UnitDefDimensions { height: value.height, radius: value.radius, midx: value.midx, minx: value.minx, maxx: value.maxx, midy: value.midy, miny: value.miny, maxy: value.maxy, midz: value.midz, minz: value.minz, maxz: value.maxz })
        }

        #[inline]
        pub fn pos2_build_pos(unit_def_id: i32, pos: Float3, facing: i32) -> Result<Float3> {
            let value = crate::generated::utils::pos2_build_pos(unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, facing)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn test_build_order(unit_def_id: i32, pos: Float3, facing: i32) -> Result<TestBuildOrderValue> {
            let value = crate::generated::utils::test_build_order(unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, facing)?;
            Ok(TestBuildOrderValue {
                status: value.0,
                can_build: value.1,
                feature: value.2
            })
        }

        #[inline]
        pub fn test_move_order(unit_def_id: i32, pos: Float3, dir: Float3, options: TestMoveOrderOptions) -> Result<bool> {
            let value = crate::generated::utils::test_move_order(unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, crate::generated::utils::Float3 { x: dir.x, y: dir.y, z: dir.z }, crate::generated::utils::TestMoveOrderOptions { test_terrain: options.test_terrain, test_objects: options.test_objects, center_only: options.center_only })?;
            Ok(value)
        }

    }


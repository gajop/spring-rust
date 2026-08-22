    pub mod units_pieces {
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
        pub struct GetFeaturePieceDirectionQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceInfoQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceInfoResult {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceListQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceListResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceMapQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceMapResult {
            pub entries: Vec<PieceMapEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceMatrixQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceMatrixResult {
            pub matrix: PieceMatrix,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePiecePosDirQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePiecePosDirResult {
            pub pos_dir: PiecePosDir,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePiecePositionQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePiecePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRootPieceQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRootPieceResult {
            pub root_piece: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceListQuery {
            pub model_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceListResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceMapQuery {
            pub model_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceMapResult {
            pub entries: Vec<PieceMapEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelRootPieceQuery {
            pub model_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelRootPieceResult {
            pub root_piece: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceDirectionQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceInfoQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceInfoResult {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceListQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceListResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceMapQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceMapResult {
            pub entries: Vec<PieceMapEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceMatrixQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceMatrixResult {
            pub matrix: PieceMatrix,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPiecePosDirQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPiecePosDirResult {
            pub pos_dir: PiecePosDir,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPiecePositionQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPiecePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitRootPieceQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitRootPieceResult {
            pub root_piece: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitScriptNamesQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitScriptNamesResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitScriptPieceQuery {
            pub unit_id: i32,
            pub script_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitScriptPieceResult {
            pub piece_num: i32,
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
        pub struct PieceInfo {
            pub name: String,
            pub parent: String,
            pub children: Vec<String>,
            pub is_empty: bool,
            pub min: Float3,
            pub max: Float3,
            pub piece_num: i32,
            pub offset: Float3,
            pub emit_dir: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PieceMapEntry {
            pub name: String,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PieceMatrix {
            pub m: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PiecePosDir {
            pub position: Float3,
            pub direction: Float3,
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
        pub struct GetFeaturePieceInfoValue {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceInfoValue {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[inline]
        pub fn get_feature_piece_direction(feature_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_feature_piece_direction(feature_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_piece_info {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-feature-piece-info"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-feature-piece-info."]
        #[inline]
        pub unsafe fn get_feature_piece_info(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_feature_piece_info::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_piece_list {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-feature-piece-list"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-feature-piece-list."]
        #[inline]
        pub unsafe fn get_feature_piece_list(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_feature_piece_list::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_piece_map {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-feature-piece-map"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-feature-piece-map."]
        #[inline]
        pub unsafe fn get_feature_piece_map(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_feature_piece_map::call(p0, p1) }
        }

        #[inline]
        pub fn get_feature_piece_matrix(feature_id: i32, piece_num: i32) -> Result<PieceMatrix> {
            let value = crate::generated::units_pieces::get_feature_piece_matrix(feature_id, piece_num)?;
            Ok(PieceMatrix { m: value.m.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()? })
        }

        #[inline]
        pub fn get_feature_piece_pos_dir(feature_id: i32, piece_num: i32) -> Result<PiecePosDir> {
            let value = crate::generated::units_pieces::get_feature_piece_pos_dir(feature_id, piece_num)?;
            Ok(PiecePosDir { position: Float3 { x: value.position.x, y: value.position.y, z: value.position.z }, direction: Float3 { x: value.direction.x, y: value.direction.y, z: value.direction.z } })
        }

        #[inline]
        pub fn get_feature_piece_position(feature_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_feature_piece_position(feature_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_feature_root_piece(feature_id: i32) -> Result<i32> {
            let value = crate::generated::units_pieces::get_feature_root_piece(feature_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_model_piece_list {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-model-piece-list"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-model-piece-list."]
        #[inline]
        pub unsafe fn get_model_piece_list(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_model_piece_list::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_model_piece_map {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-model-piece-map"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-model-piece-map."]
        #[inline]
        pub unsafe fn get_model_piece_map(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_model_piece_map::call(p0, p1) }
        }

        #[inline]
        pub fn get_model_root_piece(model_name: &str) -> Result<i32> {
            let mut model_name_bytes = model_name.as_bytes().to_vec();
            if model_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            model_name_bytes.push(0);
            let model_name_cstr = core::ffi::CStr::from_bytes_with_nul(&model_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::units_pieces::get_model_root_piece(&model_name_cstr)
        }

        #[inline]
        pub fn get_unit_piece_direction(unit_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_unit_piece_direction(unit_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_piece_info {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-unit-piece-info"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-unit-piece-info."]
        #[inline]
        pub unsafe fn get_unit_piece_info(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_piece_info::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_piece_list {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-unit-piece-list"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-unit-piece-list."]
        #[inline]
        pub unsafe fn get_unit_piece_list(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_piece_list::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_piece_map {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-unit-piece-map"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-unit-piece-map."]
        #[inline]
        pub unsafe fn get_unit_piece_map(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_piece_map::call(p0, p1) }
        }

        #[inline]
        pub fn get_unit_piece_matrix(unit_id: i32, piece_num: i32) -> Result<PieceMatrix> {
            let value = crate::generated::units_pieces::get_unit_piece_matrix(unit_id, piece_num)?;
            Ok(PieceMatrix { m: value.m.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()? })
        }

        #[inline]
        pub fn get_unit_piece_pos_dir(unit_id: i32, piece_num: i32) -> Result<PiecePosDir> {
            let value = crate::generated::units_pieces::get_unit_piece_pos_dir(unit_id, piece_num)?;
            Ok(PiecePosDir { position: Float3 { x: value.position.x, y: value.position.y, z: value.position.z }, direction: Float3 { x: value.direction.x, y: value.direction.y, z: value.direction.z } })
        }

        #[inline]
        pub fn get_unit_piece_position(unit_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_unit_piece_position(unit_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_unit_root_piece(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_pieces::get_unit_root_piece(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_script_names {
            #[link(wasm_import_module = "spring:units-pieces")]
            extern "C" {
                #[link_name = "get-unit-script-names"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-unit-script-names."]
        #[inline]
        pub unsafe fn get_unit_script_names(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_script_names::call(p0, p1) }
        }

        #[inline]
        pub fn get_unit_script_piece(unit_id: i32, script_num: i32) -> Result<i32> {
            let value = crate::generated::units_pieces::get_unit_script_piece(unit_id, script_num)?;
            Ok(value)
        }

    }


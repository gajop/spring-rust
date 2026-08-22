    pub mod features {
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
        pub struct ClearFeaturesPreviousDrawFlagQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClearFeaturesPreviousDrawFlagResult {
            pub success: bool,
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
        pub struct FeatureBlockingState {
            pub is_blocking: bool,
            pub is_solid_object_collidable: bool,
            pub is_projectile_collidable: bool,
            pub is_ray_segment_collidable: bool,
            pub crushable: bool,
            pub block_enemy_pushing: bool,
            pub block_height_changes: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureHealth {
            pub health: f32,
            pub max_health: f32,
            pub reclaim_left: f32,
            pub resurrect_progress: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureLastHitPiece {
            pub name: String,
            pub piece_num: i32,
            pub frame: i32,
            pub was_hit: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeaturePositionExt {
            pub position: Float3,
            pub mid_position: Float3,
            pub aim_position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureResources {
            pub metal: f32,
            pub def_metal: f32,
            pub energy: f32,
            pub def_energy: f32,
            pub reclaim_left: f32,
            pub reclaim_time: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureResurrect {
            pub resurrect_as: String,
            pub resurrect_def_id: i32,
            pub facing_dir: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureRotation {
            pub pitch: f32,
            pub yaw: f32,
            pub roll: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureSelectionVolumeData {
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub primary_axis: i32,
            pub use_cont_hit_test: bool,
            pub ignore_hits: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureTransformMatrix {
            pub values: Vec<f32>,
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
        pub struct GetAllFeaturesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllFeaturesResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureAllyTeamQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureAllyTeamResult {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureAlwaysUpdateMatrixQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureAlwaysUpdateMatrixResult {
            pub update: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureBlockingQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureBlockingResult {
            pub blocking_state: FeatureBlockingState,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureCollisionVolumeDataQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureCollisionVolumeDataResult {
            pub volume: CollisionVolumeData,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDResult {
            pub def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDirectionQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDrawFlagQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDrawFlagResult {
            pub flag: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureEngineDrawMaskQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureEngineDrawMaskResult {
            pub mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureFireTimeQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureFireTimeResult {
            pub fire_time: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureHeadingQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureHeadingResult {
            pub heading: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureHealthQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureHealthResult {
            pub health: FeatureHealth,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureHeightQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureLastAttackedPieceQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureLastAttackedPieceResult {
            pub piece: FeatureLastHitPiece,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureLuaDrawQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureLuaDrawResult {
            pub lua_draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureMassQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureMassResult {
            pub mass: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureNoDrawQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureNoDrawResult {
            pub no_draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureNoSelectQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureNoSelectResult {
            pub no_select: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceCollisionVolumeDataQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceCollisionVolumeDataResult {
            pub volume: CollisionVolumeData,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePositionExtQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePositionExtResult {
            pub position: FeaturePositionExt,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePositionQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRadiusQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRadiusResult {
            pub radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureResourcesQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureResourcesResult {
            pub resources: FeatureResources,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureResurrectQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureResurrectResult {
            pub resurrect: FeatureResurrect,
            pub can_resurrect: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRotationQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRotationResult {
            pub rotation: FeatureRotation,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureSelectionVolumeDataQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureSelectionVolumeDataResult {
            pub data: FeatureSelectionVolumeData,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureSeparationQuery {
            pub feature_id1: i32,
            pub feature_id2: i32,
            pub positional: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureSeparationResult {
            pub separation: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureSmokeTimeQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureSmokeTimeResult {
            pub smoke_time: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureTeamQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureTeamResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureTransformMatrixQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureTransformMatrixResult {
            pub matrix: FeatureTransformMatrix,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureVelocityQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureVelocityResult {
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInCylinderQuery {
            pub x: f32,
            pub z: f32,
            pub radius: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInCylinderResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInRectangleQuery {
            pub min_x: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInRectangleResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInSphereQuery {
            pub center: Float3,
            pub radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInSphereResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderFeaturesDrawFlagChangedQuery {
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderFeaturesDrawFlagChangedResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderFeaturesQuery {
            pub draw_mask: i32,
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderFeaturesResult {
            pub features: Vec<i32>,
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
        pub struct ValidFeatureIDQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ValidFeatureIDResult {
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
        mod __core_variable_output_get_all_features {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-all-features"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_features_in_cylinder {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-features-in-cylinder"]
                pub fn call(px: f32, pz: f32, pradius: f32, pheight: f32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_features_in_rectangle {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-features-in-rectangle"]
                pub fn call(pmin_x: f32, pmin_z: f32, pmax_x: f32, pmax_z: f32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_render_features {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-render-features"]
                pub fn call(pdraw_mask: i32, psend_mask: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_render_features_draw_flag_changed {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-render-features-draw-flag-changed"]
                pub fn call(psend_mask: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureResurrectValue {
            pub resurrect: FeatureResurrect,
            pub can_resurrect: bool,
        }

        #[inline]
        pub fn clear_features_previous_draw_flag(unused: u8) -> Result<bool> {
            let value = crate::generated::features::clear_features_previous_draw_flag(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_all_features(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_all_features::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn get_feature_ally_team(feature_id: i32) -> Result<i32> {
            let value = crate::generated::features::get_feature_ally_team(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_always_update_matrix(feature_id: i32) -> Result<bool> {
            let value = crate::generated::features::get_feature_always_update_matrix(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_blocking(feature_id: i32) -> Result<FeatureBlockingState> {
            let value = crate::generated::features::get_feature_blocking(feature_id)?;
            Ok(FeatureBlockingState { is_blocking: value.is_blocking, is_solid_object_collidable: value.is_solid_object_collidable, is_projectile_collidable: value.is_projectile_collidable, is_ray_segment_collidable: value.is_ray_segment_collidable, crushable: value.crushable, block_enemy_pushing: value.block_enemy_pushing, block_height_changes: value.block_height_changes })
        }

        #[inline]
        pub fn get_feature_collision_volume_data(feature_id: i32) -> Result<CollisionVolumeData> {
            let value = crate::generated::features::get_feature_collision_volume_data(feature_id)?;
            Ok(CollisionVolumeData { scale_x: value.scale_x, scale_y: value.scale_y, scale_z: value.scale_z, offset_x: value.offset_x, offset_y: value.offset_y, offset_z: value.offset_z, volume_type: value.volume_type, test_type: value.test_type, primary_axis: value.primary_axis, disabled: value.disabled })
        }

        #[inline]
        pub fn get_feature_def_id(feature_id: i32) -> Result<i32> {
            let value = crate::generated::features::get_feature_def_id(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_direction(feature_id: i32) -> Result<Float3> {
            let value = crate::generated::features::get_feature_direction(feature_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_feature_draw_flag(feature_id: i32) -> Result<u8> {
            let value = crate::generated::features::get_feature_draw_flag(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_engine_draw_mask(feature_id: i32) -> Result<u32> {
            let value = crate::generated::features::get_feature_engine_draw_mask(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_fire_time(feature_id: i32) -> Result<f32> {
            let value = crate::generated::features::get_feature_fire_time(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_heading(feature_id: i32) -> Result<i32> {
            let value = crate::generated::features::get_feature_heading(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_health(feature_id: i32) -> Result<FeatureHealth> {
            let value = crate::generated::features::get_feature_health(feature_id)?;
            Ok(FeatureHealth { health: value.health, max_health: value.max_health, reclaim_left: value.reclaim_left, resurrect_progress: value.resurrect_progress })
        }

        #[inline]
        pub fn get_feature_height(feature_id: i32) -> Result<f32> {
            let value = crate::generated::features::get_feature_height(feature_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_last_attacked_piece {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-feature-last-attacked-piece"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:features.get-feature-last-attacked-piece."]
        #[inline]
        pub unsafe fn get_feature_last_attacked_piece(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_feature_last_attacked_piece::call(p0, p1) }
        }

        #[inline]
        pub fn get_feature_lua_draw(feature_id: i32) -> Result<bool> {
            let value = crate::generated::features::get_feature_lua_draw(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_mass(feature_id: i32) -> Result<f32> {
            let value = crate::generated::features::get_feature_mass(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_no_draw(feature_id: i32) -> Result<bool> {
            let value = crate::generated::features::get_feature_no_draw(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_no_select(feature_id: i32) -> Result<bool> {
            let value = crate::generated::features::get_feature_no_select(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_piece_collision_volume_data(feature_id: i32, piece_num: i32) -> Result<CollisionVolumeData> {
            let value = crate::generated::features::get_feature_piece_collision_volume_data(feature_id, piece_num)?;
            Ok(CollisionVolumeData { scale_x: value.scale_x, scale_y: value.scale_y, scale_z: value.scale_z, offset_x: value.offset_x, offset_y: value.offset_y, offset_z: value.offset_z, volume_type: value.volume_type, test_type: value.test_type, primary_axis: value.primary_axis, disabled: value.disabled })
        }

        #[inline]
        pub fn get_feature_position(feature_id: i32) -> Result<Float3> {
            let value = crate::generated::features::get_feature_position(feature_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_feature_position_ext(feature_id: i32) -> Result<FeaturePositionExt> {
            let value = crate::generated::features::get_feature_position_ext(feature_id)?;
            Ok(FeaturePositionExt { position: Float3 { x: value.position.x, y: value.position.y, z: value.position.z }, mid_position: Float3 { x: value.mid_position.x, y: value.mid_position.y, z: value.mid_position.z }, aim_position: Float3 { x: value.aim_position.x, y: value.aim_position.y, z: value.aim_position.z } })
        }

        #[inline]
        pub fn get_feature_radius(feature_id: i32) -> Result<f32> {
            let value = crate::generated::features::get_feature_radius(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_resources(feature_id: i32) -> Result<FeatureResources> {
            let value = crate::generated::features::get_feature_resources(feature_id)?;
            Ok(FeatureResources { metal: value.metal, def_metal: value.def_metal, energy: value.energy, def_energy: value.def_energy, reclaim_left: value.reclaim_left, reclaim_time: value.reclaim_time })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_resurrect {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-feature-resurrect"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:features.get-feature-resurrect."]
        #[inline]
        pub unsafe fn get_feature_resurrect(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_feature_resurrect::call(p0, p1) }
        }

        #[inline]
        pub fn get_feature_rotation(feature_id: i32) -> Result<FeatureRotation> {
            let value = crate::generated::features::get_feature_rotation(feature_id)?;
            Ok(FeatureRotation { pitch: value.pitch, yaw: value.yaw, roll: value.roll })
        }

        #[inline]
        pub fn get_feature_selection_volume_data(feature_id: i32) -> Result<FeatureSelectionVolumeData> {
            let value = crate::generated::features::get_feature_selection_volume_data(feature_id)?;
            Ok(FeatureSelectionVolumeData { scales: Float3 { x: value.scales.x, y: value.scales.y, z: value.scales.z }, offsets: Float3 { x: value.offsets.x, y: value.offsets.y, z: value.offsets.z }, volume_type: value.volume_type, primary_axis: value.primary_axis, use_cont_hit_test: value.use_cont_hit_test, ignore_hits: value.ignore_hits })
        }

        #[inline]
        pub fn get_feature_separation(feature_id1: i32, feature_id2: i32, positional: bool) -> Result<f32> {
            let value = crate::generated::features::get_feature_separation(feature_id1, feature_id2, positional)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_smoke_time(feature_id: i32) -> Result<f32> {
            let value = crate::generated::features::get_feature_smoke_time(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_team(feature_id: i32) -> Result<i32> {
            let value = crate::generated::features::get_feature_team(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_transform_matrix(feature_id: i32) -> Result<FeatureTransformMatrix> {
            let value = crate::generated::features::get_feature_transform_matrix(feature_id)?;
            Ok(FeatureTransformMatrix { values: value.values.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()? })
        }

        #[inline]
        pub fn get_feature_velocity(feature_id: i32) -> Result<Float3> {
            let value = crate::generated::features::get_feature_velocity(feature_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_features_in_cylinder(x: f32, z: f32, radius: f32, height: f32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_features_in_cylinder::call(x as f32, z as f32, radius as f32, height as f32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (x as f32, z as f32, radius as f32, height as f32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_features_in_rectangle(min_x: f32, min_z: f32, max_x: f32, max_z: f32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_features_in_rectangle::call(min_x as f32, min_z as f32, max_x as f32, max_z as f32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (min_x as f32, min_z as f32, max_x as f32, max_z as f32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_features_in_sphere {
            #[link(wasm_import_module = "spring:features")]
            extern "C" {
                #[link_name = "get-features-in-sphere"]
                pub fn call(p0: f32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:features.get-features-in-sphere."]
        #[inline]
        pub unsafe fn get_features_in_sphere(p0: f32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_features_in_sphere::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_render_features(draw_mask: i32, send_mask: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_render_features::call(draw_mask as i32, u32::from(send_mask) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (draw_mask as i32, u32::from(send_mask) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_render_features_draw_flag_changed(send_mask: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_render_features_draw_flag_changed::call(u32::from(send_mask) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (u32::from(send_mask) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn valid_feature_id(feature_id: i32) -> Result<bool> {
            let value = crate::generated::features::valid_feature_id(feature_id)?;
            Ok(value)
        }

    }


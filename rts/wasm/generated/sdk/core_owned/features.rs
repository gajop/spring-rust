    pub mod features {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearFeaturesPreviousDrawFlagQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearFeaturesPreviousDrawFlagResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureBlockingState {
            pub is_blocking: bool,
            pub is_solid_object_collidable: bool,
            pub is_projectile_collidable: bool,
            pub is_ray_segment_collidable: bool,
            pub crushable: bool,
            pub block_enemy_pushing: bool,
            pub block_height_changes: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeaturePositionExt {
            pub position: Float3,
            pub mid_position: Float3,
            pub aim_position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureRotation {
            pub pitch: f32,
            pub yaw: f32,
            pub roll: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllFeaturesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllFeaturesResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureAllyTeamQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureAllyTeamResult {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureAlwaysUpdateMatrixQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureAlwaysUpdateMatrixResult {
            pub update: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureBlockingQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureBlockingResult {
            pub blocking_state: FeatureBlockingState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureCollisionVolumeDataQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureCollisionVolumeDataResult {
            pub volume: CollisionVolumeData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefIDQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefIDResult {
            pub def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDirectionQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDrawFlagQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDrawFlagResult {
            pub flag: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureEngineDrawMaskQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureEngineDrawMaskResult {
            pub mask: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureFireTimeQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureFireTimeResult {
            pub fire_time: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureHeadingQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureHeadingResult {
            pub heading: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureHealthQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureHealthResult {
            pub health: FeatureHealth,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureHeightQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureLastAttackedPieceQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureLastAttackedPieceResult {
            pub piece: FeatureLastHitPiece,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureLuaDrawQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureLuaDrawResult {
            pub lua_draw: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureMassQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureMassResult {
            pub mass: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureNoDrawQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureNoDrawResult {
            pub no_draw: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureNoSelectQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureNoSelectResult {
            pub no_select: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceCollisionVolumeDataQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceCollisionVolumeDataResult {
            pub volume: CollisionVolumeData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePositionExtQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePositionExtResult {
            pub position: FeaturePositionExt,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePositionQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureRadiusQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureRadiusResult {
            pub radius: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureResourcesQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureResourcesResult {
            pub resources: FeatureResources,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureResurrectQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureResurrectResult {
            pub resurrect: FeatureResurrect,
            pub can_resurrect: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureRotationQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureRotationResult {
            pub rotation: FeatureRotation,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureSelectionVolumeDataQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureSelectionVolumeDataResult {
            pub data: FeatureSelectionVolumeData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureSeparationQuery {
            pub feature_id1: i32,
            pub feature_id2: i32,
            pub positional: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureSeparationResult {
            pub separation: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureSmokeTimeQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureSmokeTimeResult {
            pub smoke_time: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureTeamQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureTeamResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureTransformMatrixQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureTransformMatrixResult {
            pub matrix: FeatureTransformMatrix,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureVelocityQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureVelocityResult {
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturesInSphereQuery {
            pub center: Float3,
            pub radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInSphereResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRenderFeaturesDrawFlagChangedQuery {
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderFeaturesDrawFlagChangedResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRenderFeaturesQuery {
            pub draw_mask: i32,
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderFeaturesResult {
            pub features: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidFeatureIDQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidFeatureIDResult {
            pub valid: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_all_features {
            #[link(wasm_import_module = "spring:features")]
            unsafe extern "C" {
                #[link_name = "get-all-features"]
                pub safe fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_features_in_cylinder {
            #[link(wasm_import_module = "spring:features")]
            unsafe extern "C" {
                #[link_name = "get-features-in-cylinder"]
                pub safe fn call(px: f32, pz: f32, pradius: f32, pheight: f32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_features_in_rectangle {
            #[link(wasm_import_module = "spring:features")]
            unsafe extern "C" {
                #[link_name = "get-features-in-rectangle"]
                pub safe fn call(pmin_x: f32, pmin_z: f32, pmax_x: f32, pmax_z: f32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_render_features {
            #[link(wasm_import_module = "spring:features")]
            unsafe extern "C" {
                #[link_name = "get-render-features"]
                pub safe fn call(pdraw_mask: i32, psend_mask: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_render_features_draw_flag_changed {
            #[link(wasm_import_module = "spring:features")]
            unsafe extern "C" {
                #[link_name = "get-render-features-draw-flag-changed"]
                pub safe fn call(psend_mask: i32, output: i32) -> i32;
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_all_features::call(unused as i32, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
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

        #[inline]
        pub fn get_feature_last_attacked_piece(feature_id: i32) -> Result<FeatureLastHitPiece> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::features::get_feature_last_attacked_piece(feature_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = FeatureLastHitPiece { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, piece_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, frame: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, was_hit: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
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

        #[inline]
        pub fn get_feature_resurrect(feature_id: i32) -> Result<GetFeatureResurrectValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::features::get_feature_resurrect(feature_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetFeatureResurrectValue {
                            resurrect: FeatureResurrect { resurrect_as: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, resurrect_def_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, facing_dir: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            can_resurrect: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                        };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
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
            Ok(FeatureTransformMatrix { values: value.values.into_iter().collect::<Vec<_>>() })
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_features_in_cylinder::call(x, z, radius, height, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (x, z, radius, height);
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_features_in_rectangle::call(min_x, min_z, max_x, max_z, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (min_x, min_z, max_x, max_z);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_features_in_sphere {
            #[link(wasm_import_module = "spring:features")]
            unsafe extern "C" {
                #[link_name = "get-features-in-sphere"]
                pub safe fn call(p0: f32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:features.get-features-in-sphere."]
        #[doc(hidden)]
        #[inline]
        pub fn get_features_in_sphere(p0: f32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_features_in_sphere::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_render_features(draw_mask: i32, send_mask: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_render_features::call(draw_mask, u32::from(send_mask) as i32, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (draw_mask, u32::from(send_mask) as i32);
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_render_features_draw_flag_changed::call(u32::from(send_mask) as i32, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
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


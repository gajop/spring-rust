    pub mod feature_control {
        use super::{Result, Vec};

        pub use super::types::{AddFeatureDamageQuery, AddFeatureDamageResult, AddGrassQuery, AddGrassResult, AddHeightMapQuery, AddHeightMapResult, AddObjectDecalQuery, AddObjectDecalResult, AddOriginalHeightMapQuery, AddOriginalHeightMapResult, AddSmoothMeshQuery, AddSmoothMeshResult, AddTeamResourceExcessStatsQuery, AddTeamResourceExcessStatsResult, AddTeamResourceQuery, AddTeamResourceResult, AddUnitDamageQuery, AddUnitDamageResult, AddUnitExperienceQuery, AddUnitExperienceResult, AddUnitImpulseQuery, AddUnitImpulseResult, AddUnitResourceQuery, AddUnitResourceResult, AddUnitSeismicPingQuery, AddUnitSeismicPingResult, AdjustHeightMapQuery, AdjustHeightMapResult, AdjustOriginalHeightMapQuery, AdjustOriginalHeightMapResult, AdjustSmoothMeshQuery, AdjustSmoothMeshResult, AssignPlayerToTeamQuery, AssignPlayerToTeamResult, AtmosphereParams, BoolResult, BuggerOffOptions, BuggerOffQuery, BuggerOffResult, COBScriptApi, CallCOBScriptQuery, CallCOBScriptResult, CallUnitScriptQuery, CallUnitScriptResult, ClearUnitGoalQuery, ClearUnitGoalResult, CobFunctionRef, CollisionVolumeData, CommonErrorCode, CreateFeatureQuery, CreateFeatureResult, CreateFeatureWreckQuery, CreateFeatureWreckResult, CreateUnitOptions, CreateUnitQuery, CreateUnitResult, CreateUnitWreckQuery, CreateUnitWreckResult, DefRef, DeleteProjectileQuery, DeleteProjectileResult, DestroyFeatureQuery, DestroyFeatureResult, DestroyUnitOptions, DestroyUnitQuery, DestroyUnitResult, EditUnitCmdDescQuery, EditUnitCmdDescResult, EffectsControlApi, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, ForceUnitCollisionUpdateQuery, ForceUnitCollisionUpdateResult, GameConfigApi, GameOverQuery, GameOverResult, GetCOBScriptIDQuery, GetCOBScriptIDResult, GetUnitFeatureSeparationQuery, GetUnitFeatureSeparationResult, GetUnitLeavesGhostQuery, GetUnitLeavesGhostResult, GetUnitPhysicalStateQuery, GetUnitPhysicalStateResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, InsertUnitCmdDescQuery, InsertUnitCmdDescResult, Int2, Int3, Int32Array, Int32Result, KillTeamQuery, KillTeamResult, LevelHeightMapQuery, LevelHeightMapResult, LevelOriginalHeightMapQuery, LevelOriginalHeightMapResult, LevelSmoothMeshQuery, LevelSmoothMeshResult, MapRenderingParams, NativeCommand, NativeCommandDescription, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileControlApi, ProjectileTargetRef, RebuildSmoothMeshQuery, RebuildSmoothMeshResult, RemoveGrassQuery, RemoveGrassResult, RemoveObjectDecalQuery, RemoveObjectDecalResult, RemoveUnitCmdDescQuery, RemoveUnitCmdDescResult, ResourcePack, RevertHeightMapQuery, RevertHeightMapResult, RevertOriginalHeightMapQuery, RevertOriginalHeightMapResult, RevertSmoothMeshQuery, RevertSmoothMeshResult, RgbColor, SetAllyQuery, SetAllyResult, SetAllyTeamStartBoxQuery, SetAllyTeamStartBoxResult, SetCheatingEnabledQuery, SetCheatingEnabledResult, SetExperienceGradeQuery, SetExperienceGradeResult, SetFactoryBuggerOffOptions, SetFactoryBuggerOffQuery, SetFactoryBuggerOffResult, SetFeatureAlwaysVisibleQuery, SetFeatureAlwaysVisibleResult, SetFeatureBlockingOptions, SetFeatureBlockingQuery, SetFeatureBlockingResult, SetFeatureCollisionVolumeDataQuery, SetFeatureCollisionVolumeDataResult, SetFeatureDirectionQuery, SetFeatureDirectionResult, SetFeatureFireTimeQuery, SetFeatureFireTimeResult, SetFeatureHeadingAndUpDirQuery, SetFeatureHeadingAndUpDirResult, SetFeatureHealthQuery, SetFeatureHealthResult, SetFeatureMassQuery, SetFeatureMassResult, SetFeatureMaxHealthQuery, SetFeatureMaxHealthResult, SetFeatureMidAndAimPosQuery, SetFeatureMidAndAimPosResult, SetFeatureMoveCtrlQuery, SetFeatureMoveCtrlResult, SetFeatureNoSelectQuery, SetFeatureNoSelectResult, SetFeaturePhysicsQuery, SetFeaturePhysicsResult, SetFeaturePieceCollisionVolumeDataQuery, SetFeaturePieceCollisionVolumeDataResult, SetFeaturePieceMatrixQuery, SetFeaturePieceMatrixResult, SetFeaturePieceVisibleQuery, SetFeaturePieceVisibleResult, SetFeaturePositionQuery, SetFeaturePositionResult, SetFeatureRadiusAndHeightQuery, SetFeatureRadiusAndHeightResult, SetFeatureReclaimQuery, SetFeatureReclaimResult, SetFeatureResourcesQuery, SetFeatureResourcesResult, SetFeatureResurrectQuery, SetFeatureResurrectResult, SetFeatureRotationQuery, SetFeatureRotationResult, SetFeatureSelectionVolumeDataQuery, SetFeatureSelectionVolumeDataResult, SetFeatureSmokeTimeQuery, SetFeatureSmokeTimeResult, SetFeatureUseAirLosQuery, SetFeatureUseAirLosResult, SetFeatureVelocityQuery, SetFeatureVelocityResult, SetGlobalLosQuery, SetGlobalLosResult, SetGodModeOptions, SetGodModeQuery, SetGodModeResult, SetHeightMapFuncQuery, SetHeightMapFuncResult, SetHeightMapQuery, SetHeightMapResult, SetMapSquareTerrainTypeQuery, SetMapSquareTerrainTypeResult, SetNoPauseQuery, SetNoPauseResult, SetOriginalHeightMapFuncQuery, SetOriginalHeightMapFuncResult, SetOriginalHeightMapQuery, SetOriginalHeightMapResult, SetPieceProjectileParamsQuery, SetPieceProjectileParamsResult, SetPlayerReadyStateQuery, SetPlayerReadyStateResult, SetProjectileAlwaysVisibleQuery, SetProjectileAlwaysVisibleResult, SetProjectileCEGQuery, SetProjectileCEGResult, SetProjectileCollisionQuery, SetProjectileCollisionResult, SetProjectileDamagesQuery, SetProjectileDamagesResult, SetProjectileGravityQuery, SetProjectileGravityResult, SetProjectileIgnoreTrackingErrorQuery, SetProjectileIgnoreTrackingErrorResult, SetProjectileIsInterceptedQuery, SetProjectileIsInterceptedResult, SetProjectileMoveControlQuery, SetProjectileMoveControlResult, SetProjectilePositionQuery, SetProjectilePositionResult, SetProjectileTargetQuery, SetProjectileTargetResult, SetProjectileTimeToLiveQuery, SetProjectileTimeToLiveResult, SetProjectileUseAirLosQuery, SetProjectileUseAirLosResult, SetProjectileVelocityQuery, SetProjectileVelocityResult, SetRadarErrorParamsQuery, SetRadarErrorParamsResult, SetSmoothMeshFuncQuery, SetSmoothMeshFuncResult, SetSmoothMeshQuery, SetSmoothMeshResult, SetSquareBuildingMaskQuery, SetSquareBuildingMaskResult, SetTeamResourceQuery, SetTeamResourceResult, SetTeamShareLevelQuery, SetTeamShareLevelResult, SetTeamStartPositionQuery, SetTeamStartPositionResult, SetTerrainTypeDataQuery, SetTerrainTypeDataResult, SetTidalQuery, SetTidalResult, SetUnitAlwaysVisibleQuery, SetUnitAlwaysVisibleResult, SetUnitArmoredQuery, SetUnitArmoredResult, SetUnitBlockingOptions, SetUnitBlockingQuery, SetUnitBlockingResult, SetUnitBuildParamsQuery, SetUnitBuildParamsResult, SetUnitBuildSpeedQuery, SetUnitBuildSpeedResult, SetUnitBuildeeRadiusQuery, SetUnitBuildeeRadiusResult, SetUnitCloakQuery, SetUnitCloakResult, SetUnitCollisionVolumeDataQuery, SetUnitCollisionVolumeDataResult, SetUnitCostsQuery, SetUnitCostsResult, SetUnitCrashingQuery, SetUnitCrashingResult, SetUnitDirectionQuery, SetUnitDirectionResult, SetUnitExperienceQuery, SetUnitExperienceResult, SetUnitFlankingQuery, SetUnitFlankingResult, SetUnitHarvestStorageQuery, SetUnitHarvestStorageResult, SetUnitHeadingAndUpDirQuery, SetUnitHeadingAndUpDirResult, SetUnitHeadingQuery, SetUnitHeadingResult, SetUnitHealthQuery, SetUnitHealthResult, SetUnitLandGoalQuery, SetUnitLandGoalResult, SetUnitLeavesGhostOptions, SetUnitLeavesGhostQuery, SetUnitLeavesGhostResult, SetUnitLoadingTransportQuery, SetUnitLoadingTransportResult, SetUnitLosMaskQuery, SetUnitLosMaskResult, SetUnitLosStateQuery, SetUnitLosStateResult, SetUnitMassQuery, SetUnitMassResult, SetUnitMaxHealthQuery, SetUnitMaxHealthResult, SetUnitMaxRangeQuery, SetUnitMaxRangeResult, SetUnitMetalExtractionQuery, SetUnitMetalExtractionResult, SetUnitMidAndAimPosQuery, SetUnitMidAndAimPosResult, SetUnitMoveGoalQuery, SetUnitMoveGoalResult, SetUnitNanoPiecesQuery, SetUnitNanoPiecesResult, SetUnitNeutralQuery, SetUnitNeutralResult, SetUnitPhysicalStateBitQuery, SetUnitPhysicalStateBitResult, SetUnitPhysicsQuery, SetUnitPhysicsResult, SetUnitPieceCollisionVolumeDataQuery, SetUnitPieceCollisionVolumeDataResult, SetUnitPieceMatrixQuery, SetUnitPieceMatrixResult, SetUnitPieceParentQuery, SetUnitPieceParentResult, SetUnitPieceVisibleQuery, SetUnitPieceVisibleResult, SetUnitPosErrorParamsQuery, SetUnitPosErrorParamsResult, SetUnitPositionQuery, SetUnitPositionResult, SetUnitRadiusAndHeightQuery, SetUnitRadiusAndHeightResult, SetUnitResourcingQuery, SetUnitResourcingResult, SetUnitRotationQuery, SetUnitRotationResult, SetUnitSeismicSignatureQuery, SetUnitSeismicSignatureResult, SetUnitSelectionVolumeDataQuery, SetUnitSelectionVolumeDataResult, SetUnitSensorRadiusQuery, SetUnitSensorRadiusResult, SetUnitShieldRechargeDelayQuery, SetUnitShieldRechargeDelayResult, SetUnitShieldStateQuery, SetUnitShieldStateResult, SetUnitSonarStealthQuery, SetUnitSonarStealthResult, SetUnitStealthQuery, SetUnitStealthResult, SetUnitStockpileQuery, SetUnitStockpileResult, SetUnitStorageQuery, SetUnitStorageResult, SetUnitTargetOptions, SetUnitTargetQuery, SetUnitTargetResult, SetUnitTooltipQuery, SetUnitTooltipResult, SetUnitUseAirLosQuery, SetUnitUseAirLosResult, SetUnitUseWeaponsOptions, SetUnitUseWeaponsQuery, SetUnitUseWeaponsResult, SetUnitVelocityQuery, SetUnitVelocityResult, SetUnitWeaponDamagesQuery, SetUnitWeaponDamagesResult, SetUnitWeaponStateQuery, SetUnitWeaponStateResult, SetWindQuery, SetWindResult, ShareTeamResourceQuery, ShareTeamResourceResult, SoundEffectParams, SpawnCEGQuery, SpawnCEGResult, SpawnExplosionQuery, SpawnExplosionResult, SpawnProjectileQuery, SpawnProjectileResult, SpawnSFXQuery, SpawnSFXResult, StringArray, StringResult, SunLightingParams, SyncedCtrlApi, TeamControlApi, TerrainControlApi, TransferFeatureQuery, TransferFeatureResult, TransferTeamMaxUnitsQuery, TransferTeamMaxUnitsResult, TransferUnitQuery, TransferUnitResult, UInt32Array, UInt32Result, UnitAttachQuery, UnitAttachResult, UnitControlApi, UnitCostOverrides, UnitDetachFromAirQuery, UnitDetachFromAirResult, UnitDetachQuery, UnitDetachResult, UnitFinishCommandQuery, UnitFinishCommandResult, UnitHealthValue, UnitScriptApi, UnitTargetRef, UnitWeaponFireQuery, UnitWeaponFireResult, UnitWeaponHoldFireQuery, UnitWeaponHoldFireResult, UseTeamResourceQuery, UseTeamResourceResult, UseUnitResourceQuery, UseUnitResourceResult, WaterParams};

        #[inline]
        pub fn add_feature_damage(feature_id: i32, damage: f32, paralyze_time: f32, weapon_def_id: i32, attacker_id: i32, impulse: Float3) -> Result<bool> {
            let value = crate::generated::feature_control::add_feature_damage(feature_id, damage, paralyze_time, weapon_def_id, attacker_id, crate::generated::feature_control::Float3 { x: impulse.x, y: impulse.y, z: impulse.z })?;
            Ok(value)
        }

        #[inline]
        pub fn create_feature(feature_def: &DefRef, pos: Float3, facing: i32, team_id: i32, feature_id: i32) -> Result<i32> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(feature_def.name.len() as u32).to_le_bytes()); __b.extend_from_slice(feature_def.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&feature_def.id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::feature_control::create_feature(facing, team_id, feature_id, &__blob0, &__blob1)
        }

        #[inline]
        pub fn create_feature_wreck(feature_id: i32, wreck_level: i32, do_smoke: bool) -> Result<i32> {
            let value = crate::generated::feature_control::create_feature_wreck(feature_id, wreck_level, do_smoke)?;
            Ok(value)
        }

        #[inline]
        pub fn create_unit_wreck(unit_id: i32, wreck_level: i32, do_smoke: bool) -> Result<i32> {
            let value = crate::generated::feature_control::create_unit_wreck(unit_id, wreck_level, do_smoke)?;
            Ok(value)
        }

        #[inline]
        pub fn destroy_feature(feature_id: i32) -> Result<bool> {
            let value = crate::generated::feature_control::destroy_feature(feature_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_always_visible(feature_id: i32, always_visible: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_always_visible(feature_id, always_visible)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_blocking(feature_id: i32, options: SetFeatureBlockingOptions) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_blocking(feature_id, crate::generated::feature_control::SetFeatureBlockingOptions { blocking: options.blocking, solid_objects: options.solid_objects, projectiles: options.projectiles, quad_map_rays: options.quad_map_rays, crushable: options.crushable, block_enemy_pushing: options.block_enemy_pushing, block_height_changes: options.block_height_changes })?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_collision_volume_data(feature_id: i32, scales: Float3, offsets: Float3, volume_type: i32, test_type: i32, primary_axis: i32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_collision_volume_data(feature_id, crate::generated::feature_control::Float3 { x: scales.x, y: scales.y, z: scales.z }, crate::generated::feature_control::Float3 { x: offsets.x, y: offsets.y, z: offsets.z }, volume_type, test_type, primary_axis)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_direction(feature_id: i32, front_dir: Float3, right_dir: Float3) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_direction(feature_id, crate::generated::feature_control::Float3 { x: front_dir.x, y: front_dir.y, z: front_dir.z }, crate::generated::feature_control::Float3 { x: right_dir.x, y: right_dir.y, z: right_dir.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_fire_time(feature_id: i32, fire_time: f32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_fire_time(feature_id, fire_time)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_heading_and_up_dir(feature_id: i32, heading: i32, up_dir: Float3) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_heading_and_up_dir(feature_id, heading, crate::generated::feature_control::Float3 { x: up_dir.x, y: up_dir.y, z: up_dir.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_health(feature_id: i32, health: f32, check_destruction: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_health(feature_id, health, check_destruction)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_mass(feature_id: i32, mass: f32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_mass(feature_id, mass)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_max_health(feature_id: i32, max_health: f32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_max_health(feature_id, max_health)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_mid_and_aim_pos(feature_id: i32, mid_pos: Float3, aim_pos: Float3, set_relative: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_mid_and_aim_pos(feature_id, crate::generated::feature_control::Float3 { x: mid_pos.x, y: mid_pos.y, z: mid_pos.z }, crate::generated::feature_control::Float3 { x: aim_pos.x, y: aim_pos.y, z: aim_pos.z }, set_relative)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_move_ctrl(feature_id: i32, enable: bool, velocity_or_mask: Float3, acceleration_or_impulse_mask: Float3, movement_mask: Float3) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_move_ctrl(feature_id, enable, crate::generated::feature_control::Float3 { x: velocity_or_mask.x, y: velocity_or_mask.y, z: velocity_or_mask.z }, crate::generated::feature_control::Float3 { x: acceleration_or_impulse_mask.x, y: acceleration_or_impulse_mask.y, z: acceleration_or_impulse_mask.z }, crate::generated::feature_control::Float3 { x: movement_mask.x, y: movement_mask.y, z: movement_mask.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_no_select(feature_id: i32, no_select: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_no_select(feature_id, no_select)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_physics(feature_id: i32, pos: Float3, velocity: Float3, rotation: Float3, drag: Float3) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_physics(feature_id, crate::generated::feature_control::Float3 { x: pos.x, y: pos.y, z: pos.z }, crate::generated::feature_control::Float3 { x: velocity.x, y: velocity.y, z: velocity.z }, crate::generated::feature_control::Float3 { x: rotation.x, y: rotation.y, z: rotation.z }, crate::generated::feature_control::Float3 { x: drag.x, y: drag.y, z: drag.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_piece_collision_volume_data(feature_id: i32, piece_index: i32, enable: bool, scales: Float3, offsets: Float3, volume_type: i32, primary_axis: i32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_piece_collision_volume_data(feature_id, piece_index, enable, crate::generated::feature_control::Float3 { x: scales.x, y: scales.y, z: scales.z }, crate::generated::feature_control::Float3 { x: offsets.x, y: offsets.y, z: offsets.z }, volume_type, primary_axis)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_piece_matrix(feature_id: i32, piece_index: i32, matrix: &[f32]) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_piece_matrix(feature_id, piece_index, { let __values = &matrix[..]; if __values.len() > 16 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 16]; __array[..__values.len()].copy_from_slice(__values); __array })?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_piece_visible(feature_id: i32, piece_index: i32, visible: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_piece_visible(feature_id, piece_index, visible)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_position(feature_id: i32, pos: Float3, snap_to_ground: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_position(feature_id, crate::generated::feature_control::Float3 { x: pos.x, y: pos.y, z: pos.z }, snap_to_ground)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_radius_and_height(feature_id: i32, radius: f32, height: f32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_radius_and_height(feature_id, radius, height)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_reclaim(feature_id: i32, reclaim_left: f32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_reclaim(feature_id, reclaim_left)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_resources(feature_id: i32, metal: f32, energy: f32, reclaim_time: f32, reclaim_left: f32, feature_def_metal: f32, feature_def_energy: f32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_resources(feature_id, metal, energy, reclaim_time, reclaim_left, feature_def_metal, feature_def_energy)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_resurrect(feature_id: i32, unit_def: &DefRef, facing: i32, progress: f32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(unit_def.name.len() as u32).to_le_bytes()); __b.extend_from_slice(unit_def.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&unit_def.id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::feature_control::set_feature_resurrect(feature_id, facing, progress, &__blob0)
        }

        #[inline]
        pub fn set_feature_rotation(feature_id: i32, rotation: Float3) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_rotation(feature_id, crate::generated::feature_control::Float3 { x: rotation.x, y: rotation.y, z: rotation.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_selection_volume_data(feature_id: i32, scales: Float3, offsets: Float3, volume_type: i32, primary_axis: i32, use_cont_hit_test: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_selection_volume_data(feature_id, crate::generated::feature_control::Float3 { x: scales.x, y: scales.y, z: scales.z }, crate::generated::feature_control::Float3 { x: offsets.x, y: offsets.y, z: offsets.z }, volume_type, primary_axis, use_cont_hit_test)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_smoke_time(feature_id: i32, smoke_time: f32) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_smoke_time(feature_id, smoke_time)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_use_air_los(feature_id: i32, use_air_los: bool) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_use_air_los(feature_id, use_air_los)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_velocity(feature_id: i32, velocity: Float3) -> Result<bool> {
            let value = crate::generated::feature_control::set_feature_velocity(feature_id, crate::generated::feature_control::Float3 { x: velocity.x, y: velocity.y, z: velocity.z })?;
            Ok(value)
        }

        #[inline]
        pub fn transfer_feature(feature_id: i32, new_team_id: i32) -> Result<bool> {
            let value = crate::generated::feature_control::transfer_feature(feature_id, new_team_id)?;
            Ok(value)
        }

    }


    pub mod team_control {
        use super::{Result};

        pub use super::types::{AddFeatureDamageQuery, AddFeatureDamageResult, AddGrassQuery, AddGrassResult, AddHeightMapQuery, AddHeightMapResult, AddObjectDecalQuery, AddObjectDecalResult, AddOriginalHeightMapQuery, AddOriginalHeightMapResult, AddSmoothMeshQuery, AddSmoothMeshResult, AddTeamResourceExcessStatsQuery, AddTeamResourceExcessStatsResult, AddTeamResourceQuery, AddTeamResourceResult, AddUnitDamageQuery, AddUnitDamageResult, AddUnitExperienceQuery, AddUnitExperienceResult, AddUnitImpulseQuery, AddUnitImpulseResult, AddUnitResourceQuery, AddUnitResourceResult, AddUnitSeismicPingQuery, AddUnitSeismicPingResult, AdjustHeightMapQuery, AdjustHeightMapResult, AdjustOriginalHeightMapQuery, AdjustOriginalHeightMapResult, AdjustSmoothMeshQuery, AdjustSmoothMeshResult, AssignPlayerToTeamQuery, AssignPlayerToTeamResult, AtmosphereParams, BoolResult, BuggerOffOptions, BuggerOffQuery, BuggerOffResult, COBScriptApi, CallCOBScriptQuery, CallCOBScriptResult, CallUnitScriptQuery, CallUnitScriptResult, ClearUnitGoalQuery, ClearUnitGoalResult, CobFunctionRef, CollisionVolumeData, CommonErrorCode, CreateFeatureQuery, CreateFeatureResult, CreateFeatureWreckQuery, CreateFeatureWreckResult, CreateUnitOptions, CreateUnitQuery, CreateUnitResult, CreateUnitWreckQuery, CreateUnitWreckResult, DefRef, DeleteProjectileQuery, DeleteProjectileResult, DestroyFeatureQuery, DestroyFeatureResult, DestroyUnitOptions, DestroyUnitQuery, DestroyUnitResult, EditUnitCmdDescQuery, EditUnitCmdDescResult, EffectsControlApi, Error, FeatureControlApi, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, ForceUnitCollisionUpdateQuery, ForceUnitCollisionUpdateResult, GameConfigApi, GameOverQuery, GameOverResult, GetCOBScriptIDQuery, GetCOBScriptIDResult, GetUnitFeatureSeparationQuery, GetUnitFeatureSeparationResult, GetUnitLeavesGhostQuery, GetUnitLeavesGhostResult, GetUnitPhysicalStateQuery, GetUnitPhysicalStateResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, InsertUnitCmdDescQuery, InsertUnitCmdDescResult, Int2, Int3, Int32Array, Int32Result, KillTeamQuery, KillTeamResult, LevelHeightMapQuery, LevelHeightMapResult, LevelOriginalHeightMapQuery, LevelOriginalHeightMapResult, LevelSmoothMeshQuery, LevelSmoothMeshResult, MapRenderingParams, NativeCommand, NativeCommandDescription, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileControlApi, ProjectileTargetRef, RebuildSmoothMeshQuery, RebuildSmoothMeshResult, RemoveGrassQuery, RemoveGrassResult, RemoveObjectDecalQuery, RemoveObjectDecalResult, RemoveUnitCmdDescQuery, RemoveUnitCmdDescResult, ResourcePack, RevertHeightMapQuery, RevertHeightMapResult, RevertOriginalHeightMapQuery, RevertOriginalHeightMapResult, RevertSmoothMeshQuery, RevertSmoothMeshResult, RgbColor, SetAllyQuery, SetAllyResult, SetAllyTeamStartBoxQuery, SetAllyTeamStartBoxResult, SetCheatingEnabledQuery, SetCheatingEnabledResult, SetExperienceGradeQuery, SetExperienceGradeResult, SetFactoryBuggerOffOptions, SetFactoryBuggerOffQuery, SetFactoryBuggerOffResult, SetFeatureAlwaysVisibleQuery, SetFeatureAlwaysVisibleResult, SetFeatureBlockingOptions, SetFeatureBlockingQuery, SetFeatureBlockingResult, SetFeatureCollisionVolumeDataQuery, SetFeatureCollisionVolumeDataResult, SetFeatureDirectionQuery, SetFeatureDirectionResult, SetFeatureFireTimeQuery, SetFeatureFireTimeResult, SetFeatureHeadingAndUpDirQuery, SetFeatureHeadingAndUpDirResult, SetFeatureHealthQuery, SetFeatureHealthResult, SetFeatureMassQuery, SetFeatureMassResult, SetFeatureMaxHealthQuery, SetFeatureMaxHealthResult, SetFeatureMidAndAimPosQuery, SetFeatureMidAndAimPosResult, SetFeatureMoveCtrlQuery, SetFeatureMoveCtrlResult, SetFeatureNoSelectQuery, SetFeatureNoSelectResult, SetFeaturePhysicsQuery, SetFeaturePhysicsResult, SetFeaturePieceCollisionVolumeDataQuery, SetFeaturePieceCollisionVolumeDataResult, SetFeaturePieceMatrixQuery, SetFeaturePieceMatrixResult, SetFeaturePieceVisibleQuery, SetFeaturePieceVisibleResult, SetFeaturePositionQuery, SetFeaturePositionResult, SetFeatureRadiusAndHeightQuery, SetFeatureRadiusAndHeightResult, SetFeatureReclaimQuery, SetFeatureReclaimResult, SetFeatureResourcesQuery, SetFeatureResourcesResult, SetFeatureResurrectQuery, SetFeatureResurrectResult, SetFeatureRotationQuery, SetFeatureRotationResult, SetFeatureSelectionVolumeDataQuery, SetFeatureSelectionVolumeDataResult, SetFeatureSmokeTimeQuery, SetFeatureSmokeTimeResult, SetFeatureUseAirLosQuery, SetFeatureUseAirLosResult, SetFeatureVelocityQuery, SetFeatureVelocityResult, SetGlobalLosQuery, SetGlobalLosResult, SetGodModeOptions, SetGodModeQuery, SetGodModeResult, SetHeightMapFuncQuery, SetHeightMapFuncResult, SetHeightMapQuery, SetHeightMapResult, SetMapSquareTerrainTypeQuery, SetMapSquareTerrainTypeResult, SetNoPauseQuery, SetNoPauseResult, SetOriginalHeightMapFuncQuery, SetOriginalHeightMapFuncResult, SetOriginalHeightMapQuery, SetOriginalHeightMapResult, SetPieceProjectileParamsQuery, SetPieceProjectileParamsResult, SetPlayerReadyStateQuery, SetPlayerReadyStateResult, SetProjectileAlwaysVisibleQuery, SetProjectileAlwaysVisibleResult, SetProjectileCEGQuery, SetProjectileCEGResult, SetProjectileCollisionQuery, SetProjectileCollisionResult, SetProjectileDamagesQuery, SetProjectileDamagesResult, SetProjectileGravityQuery, SetProjectileGravityResult, SetProjectileIgnoreTrackingErrorQuery, SetProjectileIgnoreTrackingErrorResult, SetProjectileIsInterceptedQuery, SetProjectileIsInterceptedResult, SetProjectileMoveControlQuery, SetProjectileMoveControlResult, SetProjectilePositionQuery, SetProjectilePositionResult, SetProjectileTargetQuery, SetProjectileTargetResult, SetProjectileTimeToLiveQuery, SetProjectileTimeToLiveResult, SetProjectileUseAirLosQuery, SetProjectileUseAirLosResult, SetProjectileVelocityQuery, SetProjectileVelocityResult, SetRadarErrorParamsQuery, SetRadarErrorParamsResult, SetSmoothMeshFuncQuery, SetSmoothMeshFuncResult, SetSmoothMeshQuery, SetSmoothMeshResult, SetSquareBuildingMaskQuery, SetSquareBuildingMaskResult, SetTeamResourceQuery, SetTeamResourceResult, SetTeamShareLevelQuery, SetTeamShareLevelResult, SetTeamStartPositionQuery, SetTeamStartPositionResult, SetTerrainTypeDataQuery, SetTerrainTypeDataResult, SetTidalQuery, SetTidalResult, SetUnitAlwaysVisibleQuery, SetUnitAlwaysVisibleResult, SetUnitArmoredQuery, SetUnitArmoredResult, SetUnitBlockingOptions, SetUnitBlockingQuery, SetUnitBlockingResult, SetUnitBuildParamsQuery, SetUnitBuildParamsResult, SetUnitBuildSpeedQuery, SetUnitBuildSpeedResult, SetUnitBuildeeRadiusQuery, SetUnitBuildeeRadiusResult, SetUnitCloakQuery, SetUnitCloakResult, SetUnitCollisionVolumeDataQuery, SetUnitCollisionVolumeDataResult, SetUnitCostsQuery, SetUnitCostsResult, SetUnitCrashingQuery, SetUnitCrashingResult, SetUnitDirectionQuery, SetUnitDirectionResult, SetUnitExperienceQuery, SetUnitExperienceResult, SetUnitFlankingQuery, SetUnitFlankingResult, SetUnitHarvestStorageQuery, SetUnitHarvestStorageResult, SetUnitHeadingAndUpDirQuery, SetUnitHeadingAndUpDirResult, SetUnitHeadingQuery, SetUnitHeadingResult, SetUnitHealthQuery, SetUnitHealthResult, SetUnitLandGoalQuery, SetUnitLandGoalResult, SetUnitLeavesGhostOptions, SetUnitLeavesGhostQuery, SetUnitLeavesGhostResult, SetUnitLoadingTransportQuery, SetUnitLoadingTransportResult, SetUnitLosMaskQuery, SetUnitLosMaskResult, SetUnitLosStateQuery, SetUnitLosStateResult, SetUnitMassQuery, SetUnitMassResult, SetUnitMaxHealthQuery, SetUnitMaxHealthResult, SetUnitMaxRangeQuery, SetUnitMaxRangeResult, SetUnitMetalExtractionQuery, SetUnitMetalExtractionResult, SetUnitMidAndAimPosQuery, SetUnitMidAndAimPosResult, SetUnitMoveGoalQuery, SetUnitMoveGoalResult, SetUnitNanoPiecesQuery, SetUnitNanoPiecesResult, SetUnitNeutralQuery, SetUnitNeutralResult, SetUnitPhysicalStateBitQuery, SetUnitPhysicalStateBitResult, SetUnitPhysicsQuery, SetUnitPhysicsResult, SetUnitPieceCollisionVolumeDataQuery, SetUnitPieceCollisionVolumeDataResult, SetUnitPieceMatrixQuery, SetUnitPieceMatrixResult, SetUnitPieceParentQuery, SetUnitPieceParentResult, SetUnitPieceVisibleQuery, SetUnitPieceVisibleResult, SetUnitPosErrorParamsQuery, SetUnitPosErrorParamsResult, SetUnitPositionQuery, SetUnitPositionResult, SetUnitRadiusAndHeightQuery, SetUnitRadiusAndHeightResult, SetUnitResourcingQuery, SetUnitResourcingResult, SetUnitRotationQuery, SetUnitRotationResult, SetUnitSeismicSignatureQuery, SetUnitSeismicSignatureResult, SetUnitSelectionVolumeDataQuery, SetUnitSelectionVolumeDataResult, SetUnitSensorRadiusQuery, SetUnitSensorRadiusResult, SetUnitShieldRechargeDelayQuery, SetUnitShieldRechargeDelayResult, SetUnitShieldStateQuery, SetUnitShieldStateResult, SetUnitSonarStealthQuery, SetUnitSonarStealthResult, SetUnitStealthQuery, SetUnitStealthResult, SetUnitStockpileQuery, SetUnitStockpileResult, SetUnitStorageQuery, SetUnitStorageResult, SetUnitTargetOptions, SetUnitTargetQuery, SetUnitTargetResult, SetUnitTooltipQuery, SetUnitTooltipResult, SetUnitUseAirLosQuery, SetUnitUseAirLosResult, SetUnitUseWeaponsOptions, SetUnitUseWeaponsQuery, SetUnitUseWeaponsResult, SetUnitVelocityQuery, SetUnitVelocityResult, SetUnitWeaponDamagesQuery, SetUnitWeaponDamagesResult, SetUnitWeaponStateQuery, SetUnitWeaponStateResult, SetWindQuery, SetWindResult, ShareTeamResourceQuery, ShareTeamResourceResult, SoundEffectParams, SpawnCEGQuery, SpawnCEGResult, SpawnExplosionQuery, SpawnExplosionResult, SpawnProjectileQuery, SpawnProjectileResult, SpawnSFXQuery, SpawnSFXResult, StringArray, StringResult, SunLightingParams, SyncedCtrlApi, TerrainControlApi, TransferFeatureQuery, TransferFeatureResult, TransferTeamMaxUnitsQuery, TransferTeamMaxUnitsResult, TransferUnitQuery, TransferUnitResult, UInt32Array, UInt32Result, UnitAttachQuery, UnitAttachResult, UnitControlApi, UnitCostOverrides, UnitDetachFromAirQuery, UnitDetachFromAirResult, UnitDetachQuery, UnitDetachResult, UnitFinishCommandQuery, UnitFinishCommandResult, UnitHealthValue, UnitScriptApi, UnitTargetRef, UnitWeaponFireQuery, UnitWeaponFireResult, UnitWeaponHoldFireQuery, UnitWeaponHoldFireResult, UseTeamResourceQuery, UseTeamResourceResult, UseUnitResourceQuery, UseUnitResourceResult, WaterParams};

        #[inline]
        pub fn add_team_resource(team_id: i32, resource_type: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::team_control::add_team_resource(team_id, __core_string_1_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn add_team_resource_excess_stats(team_id: i32, resource_type: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::team_control::add_team_resource_excess_stats(team_id, __core_string_1_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn assign_player_to_team(player_id: i32, team_id: i32) -> Result<bool> {
            let value = crate::generated::team_control::assign_player_to_team(player_id, team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn game_over(winning_ally_teams: &[i32]) -> Result<bool> {
            crate::generated::borrowed::team_control::game_over(winning_ally_teams)
        }

        #[inline]
        pub fn kill_team(team_id: i32) -> Result<bool> {
            let value = crate::generated::team_control::kill_team(team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ally(first_ally_team_id: i32, second_ally_team_id: i32, allied: bool) -> Result<bool> {
            let value = crate::generated::team_control::set_ally(first_ally_team_id, second_ally_team_id, allied)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ally_team_start_box(ally_team_id: i32, min_x: f32, min_z: f32, max_x: f32, max_z: f32) -> Result<bool> {
            let value = crate::generated::team_control::set_ally_team_start_box(ally_team_id, min_x, min_z, max_x, max_z)?;
            Ok(value)
        }

        #[inline]
        pub fn set_global_los(ally_team_id: i32, enabled: bool) -> Result<bool> {
            let value = crate::generated::team_control::set_global_los(ally_team_id, enabled)?;
            Ok(value)
        }

        #[inline]
        pub fn set_player_ready_state(player_id: i32, ready: bool) -> Result<bool> {
            let value = crate::generated::team_control::set_player_ready_state(player_id, ready)?;
            Ok(value)
        }

        #[inline]
        pub fn set_team_resource(team_id: i32, resource_type: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::team_control::set_team_resource(team_id, __core_string_1_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn set_team_share_level(team_id: i32, resource_type: &str, share_level: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::team_control::set_team_share_level(team_id, __core_string_1_buf.as_cstr(), share_level)
        }

        #[inline]
        pub fn set_team_start_position(team_id: i32, pos: Float3) -> Result<bool> {
            let value = crate::generated::team_control::set_team_start_position(team_id, crate::generated::team_control::Float3 { x: pos.x, y: pos.y, z: pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn share_team_resource(team_id: i32, target_team_id: i32, resource_type: &str, amount: f32) -> Result<bool> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(resource_type, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::team_control::share_team_resource(team_id, target_team_id, __core_string_2_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn transfer_team_max_units(from_team_id: i32, to_team_id: i32, amount: i32) -> Result<bool> {
            let value = crate::generated::team_control::transfer_team_max_units(from_team_id, to_team_id, amount)?;
            Ok(value)
        }

        #[inline]
        pub fn use_team_resource(team_id: i32, resource_type: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::team_control::use_team_resource(team_id, __core_string_1_buf.as_cstr(), amount)
        }

    }


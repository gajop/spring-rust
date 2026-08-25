    pub mod terrain_control {
        use super::{Result, String, Vec};

        pub use super::types::{AddFeatureDamageQuery, AddFeatureDamageResult, AddGrassQuery, AddGrassResult, AddHeightMapQuery, AddHeightMapResult, AddObjectDecalQuery, AddObjectDecalResult, AddOriginalHeightMapQuery, AddOriginalHeightMapResult, AddSmoothMeshQuery, AddSmoothMeshResult, AddTeamResourceExcessStatsQuery, AddTeamResourceExcessStatsResult, AddTeamResourceQuery, AddTeamResourceResult, AddUnitDamageQuery, AddUnitDamageResult, AddUnitExperienceQuery, AddUnitExperienceResult, AddUnitImpulseQuery, AddUnitImpulseResult, AddUnitResourceQuery, AddUnitResourceResult, AddUnitSeismicPingQuery, AddUnitSeismicPingResult, AdjustHeightMapQuery, AdjustHeightMapResult, AdjustOriginalHeightMapQuery, AdjustOriginalHeightMapResult, AdjustSmoothMeshQuery, AdjustSmoothMeshResult, AssignPlayerToTeamQuery, AssignPlayerToTeamResult, AtmosphereParams, BoolResult, BuggerOffOptions, BuggerOffQuery, BuggerOffResult, COBScriptApi, CallCOBScriptQuery, CallCOBScriptResult, CallUnitScriptQuery, CallUnitScriptResult, ClearUnitGoalQuery, ClearUnitGoalResult, CobFunctionRef, CollisionVolumeData, CommonErrorCode, CreateFeatureQuery, CreateFeatureResult, CreateFeatureWreckQuery, CreateFeatureWreckResult, CreateUnitOptions, CreateUnitQuery, CreateUnitResult, CreateUnitWreckQuery, CreateUnitWreckResult, DefRef, DeleteProjectileQuery, DeleteProjectileResult, DestroyFeatureQuery, DestroyFeatureResult, DestroyUnitOptions, DestroyUnitQuery, DestroyUnitResult, EditUnitCmdDescQuery, EditUnitCmdDescResult, EffectsControlApi, Error, FeatureControlApi, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, ForceUnitCollisionUpdateQuery, ForceUnitCollisionUpdateResult, GameConfigApi, GameOverQuery, GameOverResult, GetCOBScriptIDQuery, GetCOBScriptIDResult, GetUnitFeatureSeparationQuery, GetUnitFeatureSeparationResult, GetUnitLeavesGhostQuery, GetUnitLeavesGhostResult, GetUnitPhysicalStateQuery, GetUnitPhysicalStateResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, InsertUnitCmdDescQuery, InsertUnitCmdDescResult, Int2, Int3, Int32Array, Int32Result, KillTeamQuery, KillTeamResult, LevelHeightMapQuery, LevelHeightMapResult, LevelOriginalHeightMapQuery, LevelOriginalHeightMapResult, LevelSmoothMeshQuery, LevelSmoothMeshResult, MapRenderingParams, NativeCommand, NativeCommandDescription, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileControlApi, ProjectileTargetRef, RebuildSmoothMeshQuery, RebuildSmoothMeshResult, RemoveGrassQuery, RemoveGrassResult, RemoveObjectDecalQuery, RemoveObjectDecalResult, RemoveUnitCmdDescQuery, RemoveUnitCmdDescResult, ResourcePack, RevertHeightMapQuery, RevertHeightMapResult, RevertOriginalHeightMapQuery, RevertOriginalHeightMapResult, RevertSmoothMeshQuery, RevertSmoothMeshResult, RgbColor, SetAllyQuery, SetAllyResult, SetAllyTeamStartBoxQuery, SetAllyTeamStartBoxResult, SetCheatingEnabledQuery, SetCheatingEnabledResult, SetExperienceGradeQuery, SetExperienceGradeResult, SetFactoryBuggerOffOptions, SetFactoryBuggerOffQuery, SetFactoryBuggerOffResult, SetFeatureAlwaysVisibleQuery, SetFeatureAlwaysVisibleResult, SetFeatureBlockingOptions, SetFeatureBlockingQuery, SetFeatureBlockingResult, SetFeatureCollisionVolumeDataQuery, SetFeatureCollisionVolumeDataResult, SetFeatureDirectionQuery, SetFeatureDirectionResult, SetFeatureFireTimeQuery, SetFeatureFireTimeResult, SetFeatureHeadingAndUpDirQuery, SetFeatureHeadingAndUpDirResult, SetFeatureHealthQuery, SetFeatureHealthResult, SetFeatureMassQuery, SetFeatureMassResult, SetFeatureMaxHealthQuery, SetFeatureMaxHealthResult, SetFeatureMidAndAimPosQuery, SetFeatureMidAndAimPosResult, SetFeatureMoveCtrlQuery, SetFeatureMoveCtrlResult, SetFeatureNoSelectQuery, SetFeatureNoSelectResult, SetFeaturePhysicsQuery, SetFeaturePhysicsResult, SetFeaturePieceCollisionVolumeDataQuery, SetFeaturePieceCollisionVolumeDataResult, SetFeaturePieceMatrixQuery, SetFeaturePieceMatrixResult, SetFeaturePieceVisibleQuery, SetFeaturePieceVisibleResult, SetFeaturePositionQuery, SetFeaturePositionResult, SetFeatureRadiusAndHeightQuery, SetFeatureRadiusAndHeightResult, SetFeatureReclaimQuery, SetFeatureReclaimResult, SetFeatureResourcesQuery, SetFeatureResourcesResult, SetFeatureResurrectQuery, SetFeatureResurrectResult, SetFeatureRotationQuery, SetFeatureRotationResult, SetFeatureSelectionVolumeDataQuery, SetFeatureSelectionVolumeDataResult, SetFeatureSmokeTimeQuery, SetFeatureSmokeTimeResult, SetFeatureUseAirLosQuery, SetFeatureUseAirLosResult, SetFeatureVelocityQuery, SetFeatureVelocityResult, SetGlobalLosQuery, SetGlobalLosResult, SetGodModeOptions, SetGodModeQuery, SetGodModeResult, SetHeightMapFuncQuery, SetHeightMapFuncResult, SetHeightMapQuery, SetHeightMapResult, SetMapSquareTerrainTypeQuery, SetMapSquareTerrainTypeResult, SetNoPauseQuery, SetNoPauseResult, SetOriginalHeightMapFuncQuery, SetOriginalHeightMapFuncResult, SetOriginalHeightMapQuery, SetOriginalHeightMapResult, SetPieceProjectileParamsQuery, SetPieceProjectileParamsResult, SetPlayerReadyStateQuery, SetPlayerReadyStateResult, SetProjectileAlwaysVisibleQuery, SetProjectileAlwaysVisibleResult, SetProjectileCEGQuery, SetProjectileCEGResult, SetProjectileCollisionQuery, SetProjectileCollisionResult, SetProjectileDamagesQuery, SetProjectileDamagesResult, SetProjectileGravityQuery, SetProjectileGravityResult, SetProjectileIgnoreTrackingErrorQuery, SetProjectileIgnoreTrackingErrorResult, SetProjectileIsInterceptedQuery, SetProjectileIsInterceptedResult, SetProjectileMoveControlQuery, SetProjectileMoveControlResult, SetProjectilePositionQuery, SetProjectilePositionResult, SetProjectileTargetQuery, SetProjectileTargetResult, SetProjectileTimeToLiveQuery, SetProjectileTimeToLiveResult, SetProjectileUseAirLosQuery, SetProjectileUseAirLosResult, SetProjectileVelocityQuery, SetProjectileVelocityResult, SetRadarErrorParamsQuery, SetRadarErrorParamsResult, SetSmoothMeshFuncQuery, SetSmoothMeshFuncResult, SetSmoothMeshQuery, SetSmoothMeshResult, SetSquareBuildingMaskQuery, SetSquareBuildingMaskResult, SetTeamResourceQuery, SetTeamResourceResult, SetTeamShareLevelQuery, SetTeamShareLevelResult, SetTeamStartPositionQuery, SetTeamStartPositionResult, SetTerrainTypeDataQuery, SetTerrainTypeDataResult, SetTidalQuery, SetTidalResult, SetUnitAlwaysVisibleQuery, SetUnitAlwaysVisibleResult, SetUnitArmoredQuery, SetUnitArmoredResult, SetUnitBlockingOptions, SetUnitBlockingQuery, SetUnitBlockingResult, SetUnitBuildParamsQuery, SetUnitBuildParamsResult, SetUnitBuildSpeedQuery, SetUnitBuildSpeedResult, SetUnitBuildeeRadiusQuery, SetUnitBuildeeRadiusResult, SetUnitCloakQuery, SetUnitCloakResult, SetUnitCollisionVolumeDataQuery, SetUnitCollisionVolumeDataResult, SetUnitCostsQuery, SetUnitCostsResult, SetUnitCrashingQuery, SetUnitCrashingResult, SetUnitDirectionQuery, SetUnitDirectionResult, SetUnitExperienceQuery, SetUnitExperienceResult, SetUnitFlankingQuery, SetUnitFlankingResult, SetUnitHarvestStorageQuery, SetUnitHarvestStorageResult, SetUnitHeadingAndUpDirQuery, SetUnitHeadingAndUpDirResult, SetUnitHeadingQuery, SetUnitHeadingResult, SetUnitHealthQuery, SetUnitHealthResult, SetUnitLandGoalQuery, SetUnitLandGoalResult, SetUnitLeavesGhostOptions, SetUnitLeavesGhostQuery, SetUnitLeavesGhostResult, SetUnitLoadingTransportQuery, SetUnitLoadingTransportResult, SetUnitLosMaskQuery, SetUnitLosMaskResult, SetUnitLosStateQuery, SetUnitLosStateResult, SetUnitMassQuery, SetUnitMassResult, SetUnitMaxHealthQuery, SetUnitMaxHealthResult, SetUnitMaxRangeQuery, SetUnitMaxRangeResult, SetUnitMetalExtractionQuery, SetUnitMetalExtractionResult, SetUnitMidAndAimPosQuery, SetUnitMidAndAimPosResult, SetUnitMoveGoalQuery, SetUnitMoveGoalResult, SetUnitNanoPiecesQuery, SetUnitNanoPiecesResult, SetUnitNeutralQuery, SetUnitNeutralResult, SetUnitPhysicalStateBitQuery, SetUnitPhysicalStateBitResult, SetUnitPhysicsQuery, SetUnitPhysicsResult, SetUnitPieceCollisionVolumeDataQuery, SetUnitPieceCollisionVolumeDataResult, SetUnitPieceMatrixQuery, SetUnitPieceMatrixResult, SetUnitPieceParentQuery, SetUnitPieceParentResult, SetUnitPieceVisibleQuery, SetUnitPieceVisibleResult, SetUnitPosErrorParamsQuery, SetUnitPosErrorParamsResult, SetUnitPositionQuery, SetUnitPositionResult, SetUnitRadiusAndHeightQuery, SetUnitRadiusAndHeightResult, SetUnitResourcingQuery, SetUnitResourcingResult, SetUnitRotationQuery, SetUnitRotationResult, SetUnitSeismicSignatureQuery, SetUnitSeismicSignatureResult, SetUnitSelectionVolumeDataQuery, SetUnitSelectionVolumeDataResult, SetUnitSensorRadiusQuery, SetUnitSensorRadiusResult, SetUnitShieldRechargeDelayQuery, SetUnitShieldRechargeDelayResult, SetUnitShieldStateQuery, SetUnitShieldStateResult, SetUnitSonarStealthQuery, SetUnitSonarStealthResult, SetUnitStealthQuery, SetUnitStealthResult, SetUnitStockpileQuery, SetUnitStockpileResult, SetUnitStorageQuery, SetUnitStorageResult, SetUnitTargetOptions, SetUnitTargetQuery, SetUnitTargetResult, SetUnitTooltipQuery, SetUnitTooltipResult, SetUnitUseAirLosQuery, SetUnitUseAirLosResult, SetUnitUseWeaponsOptions, SetUnitUseWeaponsQuery, SetUnitUseWeaponsResult, SetUnitVelocityQuery, SetUnitVelocityResult, SetUnitWeaponDamagesQuery, SetUnitWeaponDamagesResult, SetUnitWeaponStateQuery, SetUnitWeaponStateResult, SetWindQuery, SetWindResult, ShareTeamResourceQuery, ShareTeamResourceResult, SoundEffectParams, SpawnCEGQuery, SpawnCEGResult, SpawnExplosionQuery, SpawnExplosionResult, SpawnProjectileQuery, SpawnProjectileResult, SpawnSFXQuery, SpawnSFXResult, StringArray, StringResult, SunLightingParams, SyncedCtrlApi, TeamControlApi, TransferFeatureQuery, TransferFeatureResult, TransferTeamMaxUnitsQuery, TransferTeamMaxUnitsResult, TransferUnitQuery, TransferUnitResult, UInt32Array, UInt32Result, UnitAttachQuery, UnitAttachResult, UnitControlApi, UnitCostOverrides, UnitDetachFromAirQuery, UnitDetachFromAirResult, UnitDetachQuery, UnitDetachResult, UnitFinishCommandQuery, UnitFinishCommandResult, UnitHealthValue, UnitScriptApi, UnitTargetRef, UnitWeaponFireQuery, UnitWeaponFireResult, UnitWeaponHoldFireQuery, UnitWeaponHoldFireResult, UseTeamResourceQuery, UseTeamResourceResult, UseUnitResourceQuery, UseUnitResourceResult, WaterParams};

        #[inline]
        pub fn add_grass(x: f32, z: f32, grass_value: u8) -> Result<bool> {
            let value = crate::generated::terrain_control::add_grass(x, z, grass_value)?;
            Ok(value)
        }

        #[inline]
        pub fn add_height_map(x: f32, z: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::add_height_map(x, z, height)?;
            Ok(value)
        }

        #[inline]
        pub fn add_original_height_map(x: f32, z: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::add_original_height_map(x, z, height)?;
            Ok(value)
        }

        #[inline]
        pub fn add_smooth_mesh(x: f32, z: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::add_smooth_mesh(x, z, height)?;
            Ok(value)
        }

        #[inline]
        pub fn adjust_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::adjust_height_map(x1, z1, x2, z2, height)?;
            Ok(value)
        }

        #[inline]
        pub fn adjust_original_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::adjust_original_height_map(x1, z1, x2, z2, height)?;
            Ok(value)
        }

        #[inline]
        pub fn adjust_smooth_mesh(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::adjust_smooth_mesh(x1, z1, x2, z2, height)?;
            Ok(value)
        }

        #[inline]
        pub fn level_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::level_height_map(x1, z1, x2, z2, height)?;
            Ok(value)
        }

        #[inline]
        pub fn level_original_height_map(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::level_original_height_map(x1, z1, x2, z2, height)?;
            Ok(value)
        }

        #[inline]
        pub fn level_smooth_mesh(x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::level_smooth_mesh(x1, z1, x2, z2, height)?;
            Ok(value)
        }

        #[inline]
        pub fn rebuild_smooth_mesh(unused: u8) -> Result<bool> {
            let value = crate::generated::terrain_control::rebuild_smooth_mesh(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn remove_grass(x: f32, z: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::remove_grass(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn revert_height_map(x1: f32, z1: f32, x2: f32, z2: f32, orig_factor: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::revert_height_map(x1, z1, x2, z2, orig_factor)?;
            Ok(value)
        }

        #[inline]
        pub fn revert_original_height_map(x1: f32, z1: f32, x2: f32, z2: f32, orig_factor: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::revert_original_height_map(x1, z1, x2, z2, orig_factor)?;
            Ok(value)
        }

        #[inline]
        pub fn revert_smooth_mesh(x1: f32, z1: f32, x2: f32, z2: f32, orig_factor: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::revert_smooth_mesh(x1, z1, x2, z2, orig_factor)?;
            Ok(value)
        }

        #[inline]
        pub fn set_height_map(x: f32, z: f32, height: f32, terraform: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::set_height_map(x, z, height, terraform)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_height_map_func {
            #[link(wasm_import_module = "spring:terrain-control")]
            unsafe extern "C" {
                #[link_name = "set-height-map-func"]
                pub safe fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:terrain-control.set-height-map-func."]
        #[doc(hidden)]
        #[inline]
        pub fn set_height_map_func(p0: i32, p1: i32) -> i64 {
            __core_owned_set_height_map_func::call(p0, p1)
        }

        #[inline]
        pub fn set_map_square_terrain_type(x: i32, z: i32, terrain_type: i32) -> Result<bool> {
            let value = crate::generated::terrain_control::set_map_square_terrain_type(x, z, terrain_type)?;
            Ok(value)
        }

        #[inline]
        pub fn set_original_height_map(x: f32, z: f32, height: f32, factor: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::set_original_height_map(x, z, height, factor)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_original_height_map_func {
            #[link(wasm_import_module = "spring:terrain-control")]
            unsafe extern "C" {
                #[link_name = "set-original-height-map-func"]
                pub safe fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:terrain-control.set-original-height-map-func."]
        #[doc(hidden)]
        #[inline]
        pub fn set_original_height_map_func(p0: i32, p1: i32) -> i64 {
            __core_owned_set_original_height_map_func::call(p0, p1)
        }

        #[inline]
        pub fn set_smooth_mesh(x: f32, z: f32, height: f32, terraform: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::set_smooth_mesh(x, z, height, terraform)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_smooth_mesh_func {
            #[link(wasm_import_module = "spring:terrain-control")]
            unsafe extern "C" {
                #[link_name = "set-smooth-mesh-func"]
                pub safe fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:terrain-control.set-smooth-mesh-func."]
        #[doc(hidden)]
        #[inline]
        pub fn set_smooth_mesh_func(p0: i32, p1: i32) -> i64 {
            __core_owned_set_smooth_mesh_func::call(p0, p1)
        }

        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
        pub fn set_terrain_type_data(type_index: i32, tank_speed: f32, kbot_speed: f32, hover_speed: f32, ship_speed: f32, hardness: f32, receive_tracks: bool, name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::terrain_control::set_terrain_type_data(type_index, tank_speed, kbot_speed, hover_speed, ship_speed, hardness, receive_tracks, name_cstr)
        }

        #[inline]
        pub fn set_tidal(tidal: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::set_tidal(tidal)?;
            Ok(value)
        }

        #[inline]
        pub fn set_wind(min_wind: f32, max_wind: f32) -> Result<bool> {
            let value = crate::generated::terrain_control::set_wind(min_wind, max_wind)?;
            Ok(value)
        }

    }


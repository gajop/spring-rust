    pub mod projectile_control {
        use super::{Result, Vec};

        pub use super::types::{AddFeatureDamageQuery, AddFeatureDamageResult, AddGrassQuery, AddGrassResult, AddHeightMapQuery, AddHeightMapResult, AddObjectDecalQuery, AddObjectDecalResult, AddOriginalHeightMapQuery, AddOriginalHeightMapResult, AddSmoothMeshQuery, AddSmoothMeshResult, AddTeamResourceExcessStatsQuery, AddTeamResourceExcessStatsResult, AddTeamResourceQuery, AddTeamResourceResult, AddUnitDamageQuery, AddUnitDamageResult, AddUnitExperienceQuery, AddUnitExperienceResult, AddUnitImpulseQuery, AddUnitImpulseResult, AddUnitResourceQuery, AddUnitResourceResult, AddUnitSeismicPingQuery, AddUnitSeismicPingResult, AdjustHeightMapQuery, AdjustHeightMapResult, AdjustOriginalHeightMapQuery, AdjustOriginalHeightMapResult, AdjustSmoothMeshQuery, AdjustSmoothMeshResult, AssignPlayerToTeamQuery, AssignPlayerToTeamResult, AtmosphereParams, BoolResult, BuggerOffOptions, BuggerOffQuery, BuggerOffResult, COBScriptApi, CallCOBScriptQuery, CallCOBScriptResult, CallUnitScriptQuery, CallUnitScriptResult, ClearUnitGoalQuery, ClearUnitGoalResult, CobFunctionRef, CollisionVolumeData, CommonErrorCode, CreateFeatureQuery, CreateFeatureResult, CreateFeatureWreckQuery, CreateFeatureWreckResult, CreateUnitOptions, CreateUnitQuery, CreateUnitResult, CreateUnitWreckQuery, CreateUnitWreckResult, DefRef, DeleteProjectileQuery, DeleteProjectileResult, DestroyFeatureQuery, DestroyFeatureResult, DestroyUnitOptions, DestroyUnitQuery, DestroyUnitResult, EditUnitCmdDescQuery, EditUnitCmdDescResult, EffectsControlApi, Error, FeatureControlApi, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, ForceUnitCollisionUpdateQuery, ForceUnitCollisionUpdateResult, GameConfigApi, GameOverQuery, GameOverResult, GetCOBScriptIDQuery, GetCOBScriptIDResult, GetUnitFeatureSeparationQuery, GetUnitFeatureSeparationResult, GetUnitLeavesGhostQuery, GetUnitLeavesGhostResult, GetUnitPhysicalStateQuery, GetUnitPhysicalStateResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, InsertUnitCmdDescQuery, InsertUnitCmdDescResult, Int2, Int3, Int32Array, Int32Result, KillTeamQuery, KillTeamResult, LevelHeightMapQuery, LevelHeightMapResult, LevelOriginalHeightMapQuery, LevelOriginalHeightMapResult, LevelSmoothMeshQuery, LevelSmoothMeshResult, MapRenderingParams, NativeCommand, NativeCommandDescription, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, RebuildSmoothMeshQuery, RebuildSmoothMeshResult, RemoveGrassQuery, RemoveGrassResult, RemoveObjectDecalQuery, RemoveObjectDecalResult, RemoveUnitCmdDescQuery, RemoveUnitCmdDescResult, ResourcePack, RevertHeightMapQuery, RevertHeightMapResult, RevertOriginalHeightMapQuery, RevertOriginalHeightMapResult, RevertSmoothMeshQuery, RevertSmoothMeshResult, RgbColor, SetAllyQuery, SetAllyResult, SetAllyTeamStartBoxQuery, SetAllyTeamStartBoxResult, SetCheatingEnabledQuery, SetCheatingEnabledResult, SetExperienceGradeQuery, SetExperienceGradeResult, SetFactoryBuggerOffOptions, SetFactoryBuggerOffQuery, SetFactoryBuggerOffResult, SetFeatureAlwaysVisibleQuery, SetFeatureAlwaysVisibleResult, SetFeatureBlockingOptions, SetFeatureBlockingQuery, SetFeatureBlockingResult, SetFeatureCollisionVolumeDataQuery, SetFeatureCollisionVolumeDataResult, SetFeatureDirectionQuery, SetFeatureDirectionResult, SetFeatureFireTimeQuery, SetFeatureFireTimeResult, SetFeatureHeadingAndUpDirQuery, SetFeatureHeadingAndUpDirResult, SetFeatureHealthQuery, SetFeatureHealthResult, SetFeatureMassQuery, SetFeatureMassResult, SetFeatureMaxHealthQuery, SetFeatureMaxHealthResult, SetFeatureMidAndAimPosQuery, SetFeatureMidAndAimPosResult, SetFeatureMoveCtrlQuery, SetFeatureMoveCtrlResult, SetFeatureNoSelectQuery, SetFeatureNoSelectResult, SetFeaturePhysicsQuery, SetFeaturePhysicsResult, SetFeaturePieceCollisionVolumeDataQuery, SetFeaturePieceCollisionVolumeDataResult, SetFeaturePieceMatrixQuery, SetFeaturePieceMatrixResult, SetFeaturePieceVisibleQuery, SetFeaturePieceVisibleResult, SetFeaturePositionQuery, SetFeaturePositionResult, SetFeatureRadiusAndHeightQuery, SetFeatureRadiusAndHeightResult, SetFeatureReclaimQuery, SetFeatureReclaimResult, SetFeatureResourcesQuery, SetFeatureResourcesResult, SetFeatureResurrectQuery, SetFeatureResurrectResult, SetFeatureRotationQuery, SetFeatureRotationResult, SetFeatureSelectionVolumeDataQuery, SetFeatureSelectionVolumeDataResult, SetFeatureSmokeTimeQuery, SetFeatureSmokeTimeResult, SetFeatureUseAirLosQuery, SetFeatureUseAirLosResult, SetFeatureVelocityQuery, SetFeatureVelocityResult, SetGlobalLosQuery, SetGlobalLosResult, SetGodModeOptions, SetGodModeQuery, SetGodModeResult, SetHeightMapFuncQuery, SetHeightMapFuncResult, SetHeightMapQuery, SetHeightMapResult, SetMapSquareTerrainTypeQuery, SetMapSquareTerrainTypeResult, SetNoPauseQuery, SetNoPauseResult, SetOriginalHeightMapFuncQuery, SetOriginalHeightMapFuncResult, SetOriginalHeightMapQuery, SetOriginalHeightMapResult, SetPieceProjectileParamsQuery, SetPieceProjectileParamsResult, SetPlayerReadyStateQuery, SetPlayerReadyStateResult, SetProjectileAlwaysVisibleQuery, SetProjectileAlwaysVisibleResult, SetProjectileCEGQuery, SetProjectileCEGResult, SetProjectileCollisionQuery, SetProjectileCollisionResult, SetProjectileDamagesQuery, SetProjectileDamagesResult, SetProjectileGravityQuery, SetProjectileGravityResult, SetProjectileIgnoreTrackingErrorQuery, SetProjectileIgnoreTrackingErrorResult, SetProjectileIsInterceptedQuery, SetProjectileIsInterceptedResult, SetProjectileMoveControlQuery, SetProjectileMoveControlResult, SetProjectilePositionQuery, SetProjectilePositionResult, SetProjectileTargetQuery, SetProjectileTargetResult, SetProjectileTimeToLiveQuery, SetProjectileTimeToLiveResult, SetProjectileUseAirLosQuery, SetProjectileUseAirLosResult, SetProjectileVelocityQuery, SetProjectileVelocityResult, SetRadarErrorParamsQuery, SetRadarErrorParamsResult, SetSmoothMeshFuncQuery, SetSmoothMeshFuncResult, SetSmoothMeshQuery, SetSmoothMeshResult, SetSquareBuildingMaskQuery, SetSquareBuildingMaskResult, SetTeamResourceQuery, SetTeamResourceResult, SetTeamShareLevelQuery, SetTeamShareLevelResult, SetTeamStartPositionQuery, SetTeamStartPositionResult, SetTerrainTypeDataQuery, SetTerrainTypeDataResult, SetTidalQuery, SetTidalResult, SetUnitAlwaysVisibleQuery, SetUnitAlwaysVisibleResult, SetUnitArmoredQuery, SetUnitArmoredResult, SetUnitBlockingOptions, SetUnitBlockingQuery, SetUnitBlockingResult, SetUnitBuildParamsQuery, SetUnitBuildParamsResult, SetUnitBuildSpeedQuery, SetUnitBuildSpeedResult, SetUnitBuildeeRadiusQuery, SetUnitBuildeeRadiusResult, SetUnitCloakQuery, SetUnitCloakResult, SetUnitCollisionVolumeDataQuery, SetUnitCollisionVolumeDataResult, SetUnitCostsQuery, SetUnitCostsResult, SetUnitCrashingQuery, SetUnitCrashingResult, SetUnitDirectionQuery, SetUnitDirectionResult, SetUnitExperienceQuery, SetUnitExperienceResult, SetUnitFlankingQuery, SetUnitFlankingResult, SetUnitHarvestStorageQuery, SetUnitHarvestStorageResult, SetUnitHeadingAndUpDirQuery, SetUnitHeadingAndUpDirResult, SetUnitHeadingQuery, SetUnitHeadingResult, SetUnitHealthQuery, SetUnitHealthResult, SetUnitLandGoalQuery, SetUnitLandGoalResult, SetUnitLeavesGhostOptions, SetUnitLeavesGhostQuery, SetUnitLeavesGhostResult, SetUnitLoadingTransportQuery, SetUnitLoadingTransportResult, SetUnitLosMaskQuery, SetUnitLosMaskResult, SetUnitLosStateQuery, SetUnitLosStateResult, SetUnitMassQuery, SetUnitMassResult, SetUnitMaxHealthQuery, SetUnitMaxHealthResult, SetUnitMaxRangeQuery, SetUnitMaxRangeResult, SetUnitMetalExtractionQuery, SetUnitMetalExtractionResult, SetUnitMidAndAimPosQuery, SetUnitMidAndAimPosResult, SetUnitMoveGoalQuery, SetUnitMoveGoalResult, SetUnitNanoPiecesQuery, SetUnitNanoPiecesResult, SetUnitNeutralQuery, SetUnitNeutralResult, SetUnitPhysicalStateBitQuery, SetUnitPhysicalStateBitResult, SetUnitPhysicsQuery, SetUnitPhysicsResult, SetUnitPieceCollisionVolumeDataQuery, SetUnitPieceCollisionVolumeDataResult, SetUnitPieceMatrixQuery, SetUnitPieceMatrixResult, SetUnitPieceParentQuery, SetUnitPieceParentResult, SetUnitPieceVisibleQuery, SetUnitPieceVisibleResult, SetUnitPosErrorParamsQuery, SetUnitPosErrorParamsResult, SetUnitPositionQuery, SetUnitPositionResult, SetUnitRadiusAndHeightQuery, SetUnitRadiusAndHeightResult, SetUnitResourcingQuery, SetUnitResourcingResult, SetUnitRotationQuery, SetUnitRotationResult, SetUnitSeismicSignatureQuery, SetUnitSeismicSignatureResult, SetUnitSelectionVolumeDataQuery, SetUnitSelectionVolumeDataResult, SetUnitSensorRadiusQuery, SetUnitSensorRadiusResult, SetUnitShieldRechargeDelayQuery, SetUnitShieldRechargeDelayResult, SetUnitShieldStateQuery, SetUnitShieldStateResult, SetUnitSonarStealthQuery, SetUnitSonarStealthResult, SetUnitStealthQuery, SetUnitStealthResult, SetUnitStockpileQuery, SetUnitStockpileResult, SetUnitStorageQuery, SetUnitStorageResult, SetUnitTargetOptions, SetUnitTargetQuery, SetUnitTargetResult, SetUnitTooltipQuery, SetUnitTooltipResult, SetUnitUseAirLosQuery, SetUnitUseAirLosResult, SetUnitUseWeaponsOptions, SetUnitUseWeaponsQuery, SetUnitUseWeaponsResult, SetUnitVelocityQuery, SetUnitVelocityResult, SetUnitWeaponDamagesQuery, SetUnitWeaponDamagesResult, SetUnitWeaponStateQuery, SetUnitWeaponStateResult, SetWindQuery, SetWindResult, ShareTeamResourceQuery, ShareTeamResourceResult, SoundEffectParams, SpawnCEGQuery, SpawnCEGResult, SpawnExplosionQuery, SpawnExplosionResult, SpawnProjectileQuery, SpawnProjectileResult, SpawnSFXQuery, SpawnSFXResult, StringArray, StringResult, SunLightingParams, SyncedCtrlApi, TeamControlApi, TerrainControlApi, TransferFeatureQuery, TransferFeatureResult, TransferTeamMaxUnitsQuery, TransferTeamMaxUnitsResult, TransferUnitQuery, TransferUnitResult, UInt32Array, UInt32Result, UnitAttachQuery, UnitAttachResult, UnitControlApi, UnitCostOverrides, UnitDetachFromAirQuery, UnitDetachFromAirResult, UnitDetachQuery, UnitDetachResult, UnitFinishCommandQuery, UnitFinishCommandResult, UnitHealthValue, UnitScriptApi, UnitTargetRef, UnitWeaponFireQuery, UnitWeaponFireResult, UnitWeaponHoldFireQuery, UnitWeaponHoldFireResult, UseTeamResourceQuery, UseTeamResourceResult, UseUnitResourceQuery, UseUnitResourceResult, WaterParams};

        #[inline]
        pub fn delete_projectile(projectile_id: i32) -> Result<bool> {
            let value = crate::generated::projectile_control::delete_projectile(projectile_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_piece_projectile_params(projectile_id: i32, expl_flags: i32, spin_angle: f32, spin_speed: f32, spin_vec: Float3) -> Result<bool> {
            let value = crate::generated::projectile_control::set_piece_projectile_params(projectile_id, expl_flags, spin_angle, spin_speed, crate::generated::projectile_control::Float3 { x: spin_vec.x, y: spin_vec.y, z: spin_vec.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_always_visible(projectile_id: i32, always_visible: bool) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_always_visible(projectile_id, always_visible)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_ceg(projectile_id: i32, ceg_name: &str) -> Result<i32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(ceg_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(ceg_name)?),
            };
            crate::generated::borrowed::projectile_control::set_projectile_ceg(projectile_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_projectile_collision(projectile_id: i32) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_collision(projectile_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_damages(projectile_id: i32, unused: i32, damage_key: &str, damage_value: f32) -> Result<bool> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(damage_key, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(damage_key)?),
            };
            crate::generated::borrowed::projectile_control::set_projectile_damages(projectile_id, unused, __core_string_2_buf.as_cstr(), damage_value)
        }

        #[inline]
        pub fn set_projectile_gravity(projectile_id: i32, gravity: f32) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_gravity(projectile_id, gravity)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_ignore_tracking_error(projectile_id: i32, ignore: bool) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_ignore_tracking_error(projectile_id, ignore)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_is_intercepted(projectile_id: i32, intercepted: bool) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_is_intercepted(projectile_id, intercepted)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_move_control(projectile_id: i32, enable: bool) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_move_control(projectile_id, enable)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_position(projectile_id: i32, pos: Float3) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_position(projectile_id, crate::generated::projectile_control::Float3 { x: pos.x, y: pos.y, z: pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_target(projectile_id: i32, target: ProjectileTargetRef) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_target(projectile_id, crate::generated::projectile_control::ProjectileTargetRef { target_id: target.target_id, target_type: target.target_type, pos: crate::generated::projectile_control::Float3 { x: target.pos.x, y: target.pos.y, z: target.pos.z }, is_ground_target: target.is_ground_target })?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_time_to_live(projectile_id: i32, time_to_live: i32) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_time_to_live(projectile_id, time_to_live)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_use_air_los(projectile_id: i32, use_air_los: bool) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_use_air_los(projectile_id, use_air_los)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_velocity(projectile_id: i32, velocity: Float3) -> Result<bool> {
            let value = crate::generated::projectile_control::set_projectile_velocity(projectile_id, crate::generated::projectile_control::Float3 { x: velocity.x, y: velocity.y, z: velocity.z })?;
            Ok(value)
        }

        #[inline]
        pub fn spawn_projectile(weapon_def_id: i32, projectile_params: &NativeProjectileParams) -> Result<i32> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.speed.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.speed.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.speed.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.spread.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.spread.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.spread.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.error.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.error.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.error.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.end.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.end.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.end.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.owner.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.team.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.weapon_num.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.ttl.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.gravity.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.tracking.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.max_range.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.up_time.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.start_alpha.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&projectile_params.end_alpha.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(projectile_params.model.len() as u32).to_le_bytes()); __b.extend_from_slice(projectile_params.model.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(projectile_params.ceg_tag.len() as u32).to_le_bytes()); __b.extend_from_slice(projectile_params.ceg_tag.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::projectile_control::spawn_projectile(weapon_def_id, &__blob0)
        }

    }


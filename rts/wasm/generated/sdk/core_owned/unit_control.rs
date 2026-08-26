    pub mod unit_control {
        use super::{Result, Vec};

        pub use super::types::{AddFeatureDamageQuery, AddFeatureDamageResult, AddGrassQuery, AddGrassResult, AddHeightMapQuery, AddHeightMapResult, AddObjectDecalQuery, AddObjectDecalResult, AddOriginalHeightMapQuery, AddOriginalHeightMapResult, AddSmoothMeshQuery, AddSmoothMeshResult, AddTeamResourceExcessStatsQuery, AddTeamResourceExcessStatsResult, AddTeamResourceQuery, AddTeamResourceResult, AddUnitDamageQuery, AddUnitDamageResult, AddUnitExperienceQuery, AddUnitExperienceResult, AddUnitImpulseQuery, AddUnitImpulseResult, AddUnitResourceQuery, AddUnitResourceResult, AddUnitSeismicPingQuery, AddUnitSeismicPingResult, AdjustHeightMapQuery, AdjustHeightMapResult, AdjustOriginalHeightMapQuery, AdjustOriginalHeightMapResult, AdjustSmoothMeshQuery, AdjustSmoothMeshResult, AssignPlayerToTeamQuery, AssignPlayerToTeamResult, AtmosphereParams, BoolResult, BuggerOffOptions, BuggerOffQuery, BuggerOffResult, COBScriptApi, CallCOBScriptQuery, CallCOBScriptResult, CallUnitScriptQuery, CallUnitScriptResult, ClearUnitGoalQuery, ClearUnitGoalResult, CobFunctionRef, CollisionVolumeData, CommonErrorCode, CreateFeatureQuery, CreateFeatureResult, CreateFeatureWreckQuery, CreateFeatureWreckResult, CreateUnitOptions, CreateUnitQuery, CreateUnitResult, CreateUnitWreckQuery, CreateUnitWreckResult, DefRef, DeleteProjectileQuery, DeleteProjectileResult, DestroyFeatureQuery, DestroyFeatureResult, DestroyUnitOptions, DestroyUnitQuery, DestroyUnitResult, EditUnitCmdDescQuery, EditUnitCmdDescResult, EffectsControlApi, Error, FeatureControlApi, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, ForceUnitCollisionUpdateQuery, ForceUnitCollisionUpdateResult, GameConfigApi, GameOverQuery, GameOverResult, GetCOBScriptIDQuery, GetCOBScriptIDResult, GetUnitFeatureSeparationQuery, GetUnitFeatureSeparationResult, GetUnitLeavesGhostQuery, GetUnitLeavesGhostResult, GetUnitPhysicalStateQuery, GetUnitPhysicalStateResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, InsertUnitCmdDescQuery, InsertUnitCmdDescResult, Int2, Int3, Int32Array, Int32Result, KillTeamQuery, KillTeamResult, LevelHeightMapQuery, LevelHeightMapResult, LevelOriginalHeightMapQuery, LevelOriginalHeightMapResult, LevelSmoothMeshQuery, LevelSmoothMeshResult, MapRenderingParams, NativeCommand, NativeCommandDescription, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileControlApi, ProjectileTargetRef, RebuildSmoothMeshQuery, RebuildSmoothMeshResult, RemoveGrassQuery, RemoveGrassResult, RemoveObjectDecalQuery, RemoveObjectDecalResult, RemoveUnitCmdDescQuery, RemoveUnitCmdDescResult, ResourcePack, RevertHeightMapQuery, RevertHeightMapResult, RevertOriginalHeightMapQuery, RevertOriginalHeightMapResult, RevertSmoothMeshQuery, RevertSmoothMeshResult, RgbColor, SetAllyQuery, SetAllyResult, SetAllyTeamStartBoxQuery, SetAllyTeamStartBoxResult, SetCheatingEnabledQuery, SetCheatingEnabledResult, SetExperienceGradeQuery, SetExperienceGradeResult, SetFactoryBuggerOffOptions, SetFactoryBuggerOffQuery, SetFactoryBuggerOffResult, SetFeatureAlwaysVisibleQuery, SetFeatureAlwaysVisibleResult, SetFeatureBlockingOptions, SetFeatureBlockingQuery, SetFeatureBlockingResult, SetFeatureCollisionVolumeDataQuery, SetFeatureCollisionVolumeDataResult, SetFeatureDirectionQuery, SetFeatureDirectionResult, SetFeatureFireTimeQuery, SetFeatureFireTimeResult, SetFeatureHeadingAndUpDirQuery, SetFeatureHeadingAndUpDirResult, SetFeatureHealthQuery, SetFeatureHealthResult, SetFeatureMassQuery, SetFeatureMassResult, SetFeatureMaxHealthQuery, SetFeatureMaxHealthResult, SetFeatureMidAndAimPosQuery, SetFeatureMidAndAimPosResult, SetFeatureMoveCtrlQuery, SetFeatureMoveCtrlResult, SetFeatureNoSelectQuery, SetFeatureNoSelectResult, SetFeaturePhysicsQuery, SetFeaturePhysicsResult, SetFeaturePieceCollisionVolumeDataQuery, SetFeaturePieceCollisionVolumeDataResult, SetFeaturePieceMatrixQuery, SetFeaturePieceMatrixResult, SetFeaturePieceVisibleQuery, SetFeaturePieceVisibleResult, SetFeaturePositionQuery, SetFeaturePositionResult, SetFeatureRadiusAndHeightQuery, SetFeatureRadiusAndHeightResult, SetFeatureReclaimQuery, SetFeatureReclaimResult, SetFeatureResourcesQuery, SetFeatureResourcesResult, SetFeatureResurrectQuery, SetFeatureResurrectResult, SetFeatureRotationQuery, SetFeatureRotationResult, SetFeatureSelectionVolumeDataQuery, SetFeatureSelectionVolumeDataResult, SetFeatureSmokeTimeQuery, SetFeatureSmokeTimeResult, SetFeatureUseAirLosQuery, SetFeatureUseAirLosResult, SetFeatureVelocityQuery, SetFeatureVelocityResult, SetGlobalLosQuery, SetGlobalLosResult, SetGodModeOptions, SetGodModeQuery, SetGodModeResult, SetHeightMapFuncQuery, SetHeightMapFuncResult, SetHeightMapQuery, SetHeightMapResult, SetMapSquareTerrainTypeQuery, SetMapSquareTerrainTypeResult, SetNoPauseQuery, SetNoPauseResult, SetOriginalHeightMapFuncQuery, SetOriginalHeightMapFuncResult, SetOriginalHeightMapQuery, SetOriginalHeightMapResult, SetPieceProjectileParamsQuery, SetPieceProjectileParamsResult, SetPlayerReadyStateQuery, SetPlayerReadyStateResult, SetProjectileAlwaysVisibleQuery, SetProjectileAlwaysVisibleResult, SetProjectileCEGQuery, SetProjectileCEGResult, SetProjectileCollisionQuery, SetProjectileCollisionResult, SetProjectileDamagesQuery, SetProjectileDamagesResult, SetProjectileGravityQuery, SetProjectileGravityResult, SetProjectileIgnoreTrackingErrorQuery, SetProjectileIgnoreTrackingErrorResult, SetProjectileIsInterceptedQuery, SetProjectileIsInterceptedResult, SetProjectileMoveControlQuery, SetProjectileMoveControlResult, SetProjectilePositionQuery, SetProjectilePositionResult, SetProjectileTargetQuery, SetProjectileTargetResult, SetProjectileTimeToLiveQuery, SetProjectileTimeToLiveResult, SetProjectileUseAirLosQuery, SetProjectileUseAirLosResult, SetProjectileVelocityQuery, SetProjectileVelocityResult, SetRadarErrorParamsQuery, SetRadarErrorParamsResult, SetSmoothMeshFuncQuery, SetSmoothMeshFuncResult, SetSmoothMeshQuery, SetSmoothMeshResult, SetSquareBuildingMaskQuery, SetSquareBuildingMaskResult, SetTeamResourceQuery, SetTeamResourceResult, SetTeamShareLevelQuery, SetTeamShareLevelResult, SetTeamStartPositionQuery, SetTeamStartPositionResult, SetTerrainTypeDataQuery, SetTerrainTypeDataResult, SetTidalQuery, SetTidalResult, SetUnitAlwaysVisibleQuery, SetUnitAlwaysVisibleResult, SetUnitArmoredQuery, SetUnitArmoredResult, SetUnitBlockingOptions, SetUnitBlockingQuery, SetUnitBlockingResult, SetUnitBuildParamsQuery, SetUnitBuildParamsResult, SetUnitBuildSpeedQuery, SetUnitBuildSpeedResult, SetUnitBuildeeRadiusQuery, SetUnitBuildeeRadiusResult, SetUnitCloakQuery, SetUnitCloakResult, SetUnitCollisionVolumeDataQuery, SetUnitCollisionVolumeDataResult, SetUnitCostsQuery, SetUnitCostsResult, SetUnitCrashingQuery, SetUnitCrashingResult, SetUnitDirectionQuery, SetUnitDirectionResult, SetUnitExperienceQuery, SetUnitExperienceResult, SetUnitFlankingQuery, SetUnitFlankingResult, SetUnitHarvestStorageQuery, SetUnitHarvestStorageResult, SetUnitHeadingAndUpDirQuery, SetUnitHeadingAndUpDirResult, SetUnitHeadingQuery, SetUnitHeadingResult, SetUnitHealthQuery, SetUnitHealthResult, SetUnitLandGoalQuery, SetUnitLandGoalResult, SetUnitLeavesGhostOptions, SetUnitLeavesGhostQuery, SetUnitLeavesGhostResult, SetUnitLoadingTransportQuery, SetUnitLoadingTransportResult, SetUnitLosMaskQuery, SetUnitLosMaskResult, SetUnitLosStateQuery, SetUnitLosStateResult, SetUnitMassQuery, SetUnitMassResult, SetUnitMaxHealthQuery, SetUnitMaxHealthResult, SetUnitMaxRangeQuery, SetUnitMaxRangeResult, SetUnitMetalExtractionQuery, SetUnitMetalExtractionResult, SetUnitMidAndAimPosQuery, SetUnitMidAndAimPosResult, SetUnitMoveGoalQuery, SetUnitMoveGoalResult, SetUnitNanoPiecesQuery, SetUnitNanoPiecesResult, SetUnitNeutralQuery, SetUnitNeutralResult, SetUnitPhysicalStateBitQuery, SetUnitPhysicalStateBitResult, SetUnitPhysicsQuery, SetUnitPhysicsResult, SetUnitPieceCollisionVolumeDataQuery, SetUnitPieceCollisionVolumeDataResult, SetUnitPieceMatrixQuery, SetUnitPieceMatrixResult, SetUnitPieceParentQuery, SetUnitPieceParentResult, SetUnitPieceVisibleQuery, SetUnitPieceVisibleResult, SetUnitPosErrorParamsQuery, SetUnitPosErrorParamsResult, SetUnitPositionQuery, SetUnitPositionResult, SetUnitRadiusAndHeightQuery, SetUnitRadiusAndHeightResult, SetUnitResourcingQuery, SetUnitResourcingResult, SetUnitRotationQuery, SetUnitRotationResult, SetUnitSeismicSignatureQuery, SetUnitSeismicSignatureResult, SetUnitSelectionVolumeDataQuery, SetUnitSelectionVolumeDataResult, SetUnitSensorRadiusQuery, SetUnitSensorRadiusResult, SetUnitShieldRechargeDelayQuery, SetUnitShieldRechargeDelayResult, SetUnitShieldStateQuery, SetUnitShieldStateResult, SetUnitSonarStealthQuery, SetUnitSonarStealthResult, SetUnitStealthQuery, SetUnitStealthResult, SetUnitStockpileQuery, SetUnitStockpileResult, SetUnitStorageQuery, SetUnitStorageResult, SetUnitTargetOptions, SetUnitTargetQuery, SetUnitTargetResult, SetUnitTooltipQuery, SetUnitTooltipResult, SetUnitUseAirLosQuery, SetUnitUseAirLosResult, SetUnitUseWeaponsOptions, SetUnitUseWeaponsQuery, SetUnitUseWeaponsResult, SetUnitVelocityQuery, SetUnitVelocityResult, SetUnitWeaponDamagesQuery, SetUnitWeaponDamagesResult, SetUnitWeaponStateQuery, SetUnitWeaponStateResult, SetWindQuery, SetWindResult, ShareTeamResourceQuery, ShareTeamResourceResult, SoundEffectParams, SpawnCEGQuery, SpawnCEGResult, SpawnExplosionQuery, SpawnExplosionResult, SpawnProjectileQuery, SpawnProjectileResult, SpawnSFXQuery, SpawnSFXResult, StringArray, StringResult, SunLightingParams, SyncedCtrlApi, TeamControlApi, TerrainControlApi, TransferFeatureQuery, TransferFeatureResult, TransferTeamMaxUnitsQuery, TransferTeamMaxUnitsResult, TransferUnitQuery, TransferUnitResult, UInt32Array, UInt32Result, UnitAttachQuery, UnitAttachResult, UnitCostOverrides, UnitDetachFromAirQuery, UnitDetachFromAirResult, UnitDetachQuery, UnitDetachResult, UnitFinishCommandQuery, UnitFinishCommandResult, UnitHealthValue, UnitScriptApi, UnitTargetRef, UnitWeaponFireQuery, UnitWeaponFireResult, UnitWeaponHoldFireQuery, UnitWeaponHoldFireResult, UseTeamResourceQuery, UseTeamResourceResult, UseUnitResourceQuery, UseUnitResourceResult, WaterParams};

        #[inline]
        pub fn add_object_decal(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_control::add_object_decal(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn add_unit_damage(unit_id: i32, damage: f32, paralyze_time: f32, weapon_def_id: i32, attacker_id: i32, impulse: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::add_unit_damage(unit_id, damage, paralyze_time, weapon_def_id, attacker_id, crate::generated::unit_control::Float3 { x: impulse.x, y: impulse.y, z: impulse.z })?;
            Ok(value)
        }

        #[inline]
        pub fn add_unit_experience(unit_id: i32, experience: f32) -> Result<bool> {
            let value = crate::generated::unit_control::add_unit_experience(unit_id, experience)?;
            Ok(value)
        }

        #[inline]
        pub fn add_unit_impulse(unit_id: i32, impulse: Float3, decay_rate: f32) -> Result<bool> {
            let value = crate::generated::unit_control::add_unit_impulse(unit_id, crate::generated::unit_control::Float3 { x: impulse.x, y: impulse.y, z: impulse.z }, decay_rate)?;
            Ok(value)
        }

        #[inline]
        pub fn add_unit_resource(unit_id: i32, resource_type: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::unit_control::add_unit_resource(unit_id, __core_string_1_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn add_unit_seismic_ping(unit_id: i32, ping_size: f32) -> Result<bool> {
            let value = crate::generated::unit_control::add_unit_seismic_ping(unit_id, ping_size)?;
            Ok(value)
        }

        #[inline]
        pub fn bugger_off(pos: Float3, radius: f32, team_id: i32, options: BuggerOffOptions, exclude_unit_def_i_ds: &[i32]) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(if options.spherical { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if options.forced { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&options.exclude_unit_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob2 = { let mut __b = Vec::new(); __b.extend_from_slice(&(exclude_unit_def_i_ds.len() as u32).to_le_bytes()); for __item in exclude_unit_def_i_ds.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes());} __b };
            crate::generated::dynamic_input::unit_control::bugger_off(radius, team_id, &__blob0, &__blob1, &__blob2)
        }

        #[inline]
        pub fn clear_unit_goal(unit_id: i32, cancel_raw: bool) -> Result<bool> {
            let value = crate::generated::unit_control::clear_unit_goal(unit_id, cancel_raw)?;
            Ok(value)
        }

        #[inline]
        pub fn create_unit(unit_def: &DefRef, pos: Float3, facing: i32, team_id: i32, options: CreateUnitOptions) -> Result<i32> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(unit_def.name.len() as u32).to_le_bytes()); __b.extend_from_slice(unit_def.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&unit_def.id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob2 = { let mut __b = Vec::new(); __b.extend_from_slice(&(if options.build { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if options.flatten_ground { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&options.unit_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&options.builder_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::unit_control::create_unit(facing, team_id, &__blob0, &__blob1, &__blob2)
        }

        #[inline]
        pub fn destroy_unit(unit_id: i32, options: DestroyUnitOptions) -> Result<bool> {
            let value = crate::generated::unit_control::destroy_unit(unit_id, crate::generated::unit_control::DestroyUnitOptions { selfd: options.selfd, reclaimed: options.reclaimed, attacker_id: options.attacker_id, recycle_id: options.recycle_id })?;
            Ok(value)
        }

        #[inline]
        pub fn edit_unit_cmd_desc(unit_id: i32, cmd_desc_index: u32, cmd_desc: &NativeCommandDescription) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&cmd_desc.id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&cmd_desc.type_.to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.queueing { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.hidden { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.disabled { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.show_unique { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.only_texture { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.name.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.action.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.action.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.iconname.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.iconname.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.mouseicon.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.mouseicon.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.tooltip.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.tooltip.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.params.len() as u32).to_le_bytes()); for __item in cmd_desc.params.iter() { __b.extend_from_slice(&(__item.len() as u32).to_le_bytes()); __b.extend_from_slice(__item.as_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::unit_control::edit_unit_cmd_desc(unit_id, cmd_desc_index as i32, &__blob0)
        }

        #[inline]
        pub fn force_unit_collision_update(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_control::force_unit_collision_update(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_feature_separation(unit_id: i32, feature_id: i32, ignore_y: bool) -> Result<f32> {
            let value = crate::generated::unit_control::get_unit_feature_separation(unit_id, feature_id, ignore_y)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_leaves_ghost(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_control::get_unit_leaves_ghost(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_physical_state(unit_id: i32) -> Result<u32> {
            let value = crate::generated::unit_control::get_unit_physical_state(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn give_order_array_to_unit(unit_id: i32, commands: &[NativeCommand]) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(commands.len() as u32).to_le_bytes()); for __item in commands.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.cmd_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.params.len() as u32).to_le_bytes()); for __item in __item.params.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_bits().to_le_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.options.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.timeout.to_le_bytes());} __b };
            crate::generated::dynamic_input::unit_control::give_order_array_to_unit(unit_id, &__blob0)
        }

        #[inline]
        pub fn give_order_array_to_unit_array(unit_i_ds: &[i32], commands: &[NativeCommand], pairwise: bool) -> Result<i32> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(unit_i_ds.len() as u32).to_le_bytes()); for __item in unit_i_ds.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes());} __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(commands.len() as u32).to_le_bytes()); for __item in commands.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.cmd_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.params.len() as u32).to_le_bytes()); for __item in __item.params.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_bits().to_le_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.options.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.timeout.to_le_bytes());} __b };
            crate::generated::dynamic_input::unit_control::give_order_array_to_unit_array(pairwise as i32, &__blob0, &__blob1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_give_order_to_unit {
            #[link(wasm_import_module = "spring:unit-control")]
            unsafe extern "C" {
                #[link_name = "give-order-to-unit"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-control.give-order-to-unit."]
        #[doc(hidden)]
        #[inline]
        pub fn give_order_to_unit(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64 {
            __core_owned_give_order_to_unit::call(p0, p1, p2, p3, p4, p5)
        }

        #[inline]
        pub fn give_order_to_unit_array(unit_i_ds: &[i32], cmd_id: i32, params: &[f32], options: u32, timeout: i32) -> Result<bool> {
            crate::generated::borrowed::unit_control::give_order_to_unit_array(unit_i_ds, cmd_id, params, options, timeout)
        }

        #[inline]
        pub fn insert_unit_cmd_desc(unit_id: i32, cmd_desc_index: i32, cmd_desc: &NativeCommandDescription) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&cmd_desc.id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&cmd_desc.type_.to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.queueing { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.hidden { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.disabled { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.show_unique { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if cmd_desc.only_texture { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.name.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.action.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.action.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.iconname.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.iconname.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.mouseicon.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.mouseicon.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.tooltip.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_desc.tooltip.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_desc.params.len() as u32).to_le_bytes()); for __item in cmd_desc.params.iter() { __b.extend_from_slice(&(__item.len() as u32).to_le_bytes()); __b.extend_from_slice(__item.as_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::unit_control::insert_unit_cmd_desc(unit_id, cmd_desc_index, &__blob0)
        }

        #[inline]
        pub fn remove_object_decal(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_control::remove_object_decal(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn remove_unit_cmd_desc(unit_id: i32, cmd_desc_index: i32) -> Result<bool> {
            let value = crate::generated::unit_control::remove_unit_cmd_desc(unit_id, cmd_desc_index)?;
            Ok(value)
        }

        #[inline]
        pub fn set_factory_bugger_off(unit_id: i32, options: SetFactoryBuggerOffOptions) -> Result<bool> {
            let value = crate::generated::unit_control::set_factory_bugger_off(unit_id, crate::generated::unit_control::SetFactoryBuggerOffOptions { perform: options.perform, offset: options.offset, radius: options.radius, rel_heading: options.rel_heading, spherical: options.spherical, forced: options.forced })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_always_visible(unit_id: i32, always_visible: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_always_visible(unit_id, always_visible)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_armored(unit_id: i32, armored_state: bool, armored_multiple: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_armored(unit_id, armored_state, armored_multiple)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_blocking(unit_id: i32, options: SetUnitBlockingOptions) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_blocking(unit_id, crate::generated::unit_control::SetUnitBlockingOptions { blocking: options.blocking, solid_objects: options.solid_objects, projectiles: options.projectiles, quad_map_rays: options.quad_map_rays, crushable: options.crushable, block_enemy_pushing: options.block_enemy_pushing, block_height_changes: options.block_height_changes })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_build_params(unit_id: i32, param_name: &str, value: NumberOrBool) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + param_name.len()); __b.extend_from_slice(&(param_name.len() as u32).to_le_bytes()); __b.extend_from_slice(param_name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&value.number.to_bits().to_le_bytes()); __b.extend_from_slice(&(if value.boolean { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if value.use_boolean { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::unit_control::set_unit_build_params(unit_id, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_unit_build_speed(unit_id: i32, build_speed: f32, repair_speed: f32, reclaim_speed: f32, resurrect_speed: f32, capture_speed: f32, terraform_speed: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_build_speed(unit_id, build_speed, repair_speed, reclaim_speed, resurrect_speed, capture_speed, terraform_speed)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_buildee_radius(unit_id: i32, radius: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_buildee_radius(unit_id, radius)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_cloak(unit_id: i32, cloak: NumberOrBool, cloak_arg: NumberOrBool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_cloak(unit_id, crate::generated::unit_control::NumberOrBool { number: cloak.number, boolean: cloak.boolean, use_boolean: cloak.use_boolean }, crate::generated::unit_control::NumberOrBool { number: cloak_arg.number, boolean: cloak_arg.boolean, use_boolean: cloak_arg.use_boolean })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_collision_volume_data(unit_id: i32, scales: Float3, offsets: Float3, volume_type: i32, test_type: i32, primary_axis: i32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_collision_volume_data(unit_id, crate::generated::unit_control::Float3 { x: scales.x, y: scales.y, z: scales.z }, crate::generated::unit_control::Float3 { x: offsets.x, y: offsets.y, z: offsets.z }, volume_type, test_type, primary_axis)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_costs(unit_id: i32, costs: UnitCostOverrides) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_costs(unit_id, crate::generated::unit_control::UnitCostOverrides { build_time: costs.build_time, metal_cost: costs.metal_cost, energy_cost: costs.energy_cost })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_crashing(unit_id: i32, want_crash: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_crashing(unit_id, want_crash)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_direction(unit_id: i32, front_dir: Float3, right_dir: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_direction(unit_id, crate::generated::unit_control::Float3 { x: front_dir.x, y: front_dir.y, z: front_dir.z }, crate::generated::unit_control::Float3 { x: right_dir.x, y: right_dir.y, z: right_dir.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_experience(unit_id: i32, experience: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_experience(unit_id, experience)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_flanking(unit_id: i32, type_: &str, args: Float3) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + type_.len()); __b.extend_from_slice(&(type_.len() as u32).to_le_bytes()); __b.extend_from_slice(type_.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&args.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&args.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&args.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::unit_control::set_unit_flanking(unit_id, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_unit_harvest_storage(unit_id: i32, stored_metal: f32, max_stored_metal: f32, stored_energy: f32, max_stored_energy: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_harvest_storage(unit_id, stored_metal, max_stored_metal, stored_energy, max_stored_energy)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_heading(unit_id: i32, heading: i32, use_smoothing: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_heading(unit_id, heading, use_smoothing)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_heading_and_up_dir(unit_id: i32, heading: i32, up_dir: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_heading_and_up_dir(unit_id, heading, crate::generated::unit_control::Float3 { x: up_dir.x, y: up_dir.y, z: up_dir.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_health(unit_id: i32, value: UnitHealthValue) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_health(unit_id, crate::generated::unit_control::UnitHealthValue { health: value.health, capture: value.capture, paralyze: value.paralyze, build: value.build, use_amounts: value.use_amounts })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_land_goal(unit_id: i32, pos: Float3, radius_sq: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_land_goal(unit_id, crate::generated::unit_control::Float3 { x: pos.x, y: pos.y, z: pos.z }, radius_sq)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_leaves_ghost(unit_id: i32, options: SetUnitLeavesGhostOptions) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_leaves_ghost(unit_id, crate::generated::unit_control::SetUnitLeavesGhostOptions { leaves_ghost: options.leaves_ghost, leave_dead_ghost: options.leave_dead_ghost })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_loading_transport(unit_id: i32, transport_id: i32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_loading_transport(unit_id, transport_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_los_mask(unit_id: i32, ally_team_id: i32, los_mask: u8) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_los_mask(unit_id, ally_team_id, los_mask)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_los_state(unit_id: i32, ally_team_id: i32, los_state: u8) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_los_state(unit_id, ally_team_id, los_state)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_mass(unit_id: i32, mass: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_mass(unit_id, mass)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_max_health(unit_id: i32, max_health: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_max_health(unit_id, max_health)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_max_range(unit_id: i32, max_range: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_max_range(unit_id, max_range)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_metal_extraction(unit_id: i32, depth: f32, range: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_metal_extraction(unit_id, depth, range)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_mid_and_aim_pos(unit_id: i32, mid_pos: Float3, aim_pos: Float3, set_relative: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_mid_and_aim_pos(unit_id, crate::generated::unit_control::Float3 { x: mid_pos.x, y: mid_pos.y, z: mid_pos.z }, crate::generated::unit_control::Float3 { x: aim_pos.x, y: aim_pos.y, z: aim_pos.z }, set_relative)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_move_goal(unit_id: i32, pos: Float3, radius: f32, speed: f32, raw: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_move_goal(unit_id, crate::generated::unit_control::Float3 { x: pos.x, y: pos.y, z: pos.z }, radius, speed, raw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_nano_pieces(unit_id: i32, piece_indices: &[i32]) -> Result<bool> {
            crate::generated::borrowed::unit_control::set_unit_nano_pieces(unit_id, piece_indices)
        }

        #[inline]
        pub fn set_unit_neutral(unit_id: i32, neutral: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_neutral(unit_id, neutral)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_physical_state_bit(unit_id: i32, state_bit: i32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_physical_state_bit(unit_id, state_bit)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_physics(unit_id: i32, pos: Float3, velocity: Float3, rotation: Float3, drag: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_physics(unit_id, crate::generated::unit_control::Float3 { x: pos.x, y: pos.y, z: pos.z }, crate::generated::unit_control::Float3 { x: velocity.x, y: velocity.y, z: velocity.z }, crate::generated::unit_control::Float3 { x: rotation.x, y: rotation.y, z: rotation.z }, crate::generated::unit_control::Float3 { x: drag.x, y: drag.y, z: drag.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_piece_collision_volume_data(unit_id: i32, piece_index: i32, enable: bool, scales: Float3, offsets: Float3, volume_type: i32, primary_axis: i32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_piece_collision_volume_data(unit_id, piece_index, enable, crate::generated::unit_control::Float3 { x: scales.x, y: scales.y, z: scales.z }, crate::generated::unit_control::Float3 { x: offsets.x, y: offsets.y, z: offsets.z }, volume_type, primary_axis)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_piece_matrix(unit_id: i32, piece_index: i32, matrix: &[f32]) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_piece_matrix(unit_id, piece_index, matrix.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_piece_parent(unit_id: i32, child_piece_index: i32, parent_piece_index: i32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_piece_parent(unit_id, child_piece_index, parent_piece_index)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_piece_visible(unit_id: i32, piece_index: i32, visible: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_piece_visible(unit_id, piece_index, visible)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_pos_error_params(unit_id: i32, pos_error_vector: Float3, pos_error_delta: Float3, next_pos_error_update: i32, ally_team_id: i32, set_pos_error_bit: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_pos_error_params(unit_id, crate::generated::unit_control::Float3 { x: pos_error_vector.x, y: pos_error_vector.y, z: pos_error_vector.z }, crate::generated::unit_control::Float3 { x: pos_error_delta.x, y: pos_error_delta.y, z: pos_error_delta.z }, next_pos_error_update, ally_team_id, set_pos_error_bit)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_position(unit_id: i32, pos: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_position(unit_id, crate::generated::unit_control::Float3 { x: pos.x, y: pos.y, z: pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_radius_and_height(unit_id: i32, radius: f32, height: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_radius_and_height(unit_id, radius, height)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_resourcing(unit_id: i32, type_: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(type_, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(type_)?),
            };
            crate::generated::borrowed::unit_control::set_unit_resourcing(unit_id, __core_string_1_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn set_unit_rotation(unit_id: i32, rotation: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_rotation(unit_id, crate::generated::unit_control::Float3 { x: rotation.x, y: rotation.y, z: rotation.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_seismic_signature(unit_id: i32, seismic_signature: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_seismic_signature(unit_id, seismic_signature)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_selection_volume_data(unit_id: i32, scales: Float3, offsets: Float3, volume_type: i32, test_type: i32, primary_axis: i32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_selection_volume_data(unit_id, crate::generated::unit_control::Float3 { x: scales.x, y: scales.y, z: scales.z }, crate::generated::unit_control::Float3 { x: offsets.x, y: offsets.y, z: offsets.z }, volume_type, test_type, primary_axis)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_sensor_radius(unit_id: i32, sensor_type: &str, radius: i32) -> Result<i32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(sensor_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(sensor_type)?),
            };
            crate::generated::borrowed::unit_control::set_unit_sensor_radius(unit_id, __core_string_1_buf.as_cstr(), radius)
        }

        #[inline]
        pub fn set_unit_shield_recharge_delay(unit_id: i32, weapon_num: i32, recharge_delay: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_shield_recharge_delay(unit_id, weapon_num, recharge_delay)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_shield_state(unit_id: i32, weapon_num: i32, enabled: bool, power: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_shield_state(unit_id, weapon_num, enabled, power)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_sonar_stealth(unit_id: i32, sonar_stealth: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_sonar_stealth(unit_id, sonar_stealth)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_stealth(unit_id: i32, stealth: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_stealth(unit_id, stealth)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_stockpile(unit_id: i32, stockpile: i32, build_percent: f32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_stockpile(unit_id, stockpile, build_percent)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_storage(unit_id: i32, resource: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource)?),
            };
            crate::generated::borrowed::unit_control::set_unit_storage(unit_id, __core_string_1_buf.as_cstr(), amount)
        }

        #[inline]
        pub fn set_unit_target(unit_id: i32, target: UnitTargetRef, options: SetUnitTargetOptions, weapon_num: i32) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_target(unit_id, crate::generated::unit_control::UnitTargetRef { target_id: target.target_id, pos: crate::generated::unit_control::Float3 { x: target.pos.x, y: target.pos.y, z: target.pos.z }, is_ground_target: target.is_ground_target }, crate::generated::unit_control::SetUnitTargetOptions { manual_fire: options.manual_fire, user_target: options.user_target }, weapon_num)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_tooltip(unit_id: i32, tooltip: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(tooltip, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(tooltip)?),
            };
            crate::generated::borrowed::unit_control::set_unit_tooltip(unit_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_unit_use_air_los(unit_id: i32, use_air_los: bool) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_use_air_los(unit_id, use_air_los)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_use_weapons(unit_id: i32, options: SetUnitUseWeaponsOptions) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_use_weapons(unit_id, crate::generated::unit_control::SetUnitUseWeaponsOptions { force_use_weapons: options.force_use_weapons, allow_use_weapons: options.allow_use_weapons })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_velocity(unit_id: i32, velocity: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::set_unit_velocity(unit_id, crate::generated::unit_control::Float3 { x: velocity.x, y: velocity.y, z: velocity.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_weapon_damages(unit_id: i32, weapon_num: i32, damage_key: &str, damage_value: f32) -> Result<bool> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(damage_key, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(damage_key)?),
            };
            crate::generated::borrowed::unit_control::set_unit_weapon_damages(unit_id, weapon_num, __core_string_2_buf.as_cstr(), damage_value)
        }

        #[inline]
        pub fn set_unit_weapon_state(unit_id: i32, weapon_num: i32, key: &str, value: f32) -> Result<bool> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(key, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            crate::generated::borrowed::unit_control::set_unit_weapon_state(unit_id, weapon_num, __core_string_2_buf.as_cstr(), value)
        }

        #[inline]
        pub fn transfer_unit(unit_id: i32, new_team_id: i32, given: bool, adjust_unit_limit: bool) -> Result<bool> {
            let value = crate::generated::unit_control::transfer_unit(unit_id, new_team_id, given, adjust_unit_limit)?;
            Ok(value)
        }

        #[inline]
        pub fn unit_attach(transporter_id: i32, transportee_id: i32, piece_num: i32) -> Result<bool> {
            let value = crate::generated::unit_control::unit_attach(transporter_id, transportee_id, piece_num)?;
            Ok(value)
        }

        #[inline]
        pub fn unit_detach(transportee_id: i32) -> Result<bool> {
            let value = crate::generated::unit_control::unit_detach(transportee_id)?;
            Ok(value)
        }

        #[inline]
        pub fn unit_detach_from_air(transportee_id: i32, pos: Float3) -> Result<bool> {
            let value = crate::generated::unit_control::unit_detach_from_air(transportee_id, crate::generated::unit_control::Float3 { x: pos.x, y: pos.y, z: pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn unit_finish_command(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_control::unit_finish_command(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn unit_weapon_fire(unit_id: i32, weapon_num: i32) -> Result<bool> {
            let value = crate::generated::unit_control::unit_weapon_fire(unit_id, weapon_num)?;
            Ok(value)
        }

        #[inline]
        pub fn unit_weapon_hold_fire(unit_id: i32, weapon_num: i32) -> Result<bool> {
            let value = crate::generated::unit_control::unit_weapon_hold_fire(unit_id, weapon_num)?;
            Ok(value)
        }

        #[inline]
        pub fn use_unit_resource(unit_id: i32, resource_type: &str, amount: f32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(resource_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(resource_type)?),
            };
            crate::generated::borrowed::unit_control::use_unit_resource(unit_id, __core_string_1_buf.as_cstr(), amount)
        }

    }


    pub mod units_info {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearUnitsPreviousDrawFlagQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearUnitsPreviousDrawFlagResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitAllyTeamQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitAllyTeamResult {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitArmoredQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitArmoredResult {
            pub armored_state: UnitArmoredState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBasePositionQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBasePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBlockingQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBlockingResult {
            pub blocking_state: UnitBlockingState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBuildFacingQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBuildFacingResult {
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitBuildParamsQuery {
            pub unit_id: i32,
            pub param_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitBuildParamsResult {
            pub value: Option<NumberOrBool>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBuildeeRadiusQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitBuildeeRadiusResult {
            pub radius: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCollisionVolumeDataQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCollisionVolumeDataResult {
            pub volume: CollisionVolumeData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCostTableQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCostTableResult {
            pub costs: UnitCosts,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCostsQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCostsResult {
            pub costs: UnitCosts,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCrashingQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCrashingResult {
            pub is_aircraft: bool,
            pub crashing: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCurrentBuildPowerQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCurrentBuildPowerResult {
            pub build_power: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefIDQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefIDResult {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDirectionQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitEffectiveBuildRangeQuery {
            pub unit_id: i32,
            pub buildee_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitEffectiveBuildRangeResult {
            pub range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitExperienceQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitExperienceResult {
            pub experience: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitFlankingQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitFlankingResult {
            pub flanking: UnitFlanking,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHarvestStorageQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHarvestStorageResult {
            pub storage: UnitHarvestStorage,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHeadingQuery {
            pub unit_id: i32,
            pub convert_to_radians: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHeadingResult {
            pub heading: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHealthQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHealthResult {
            pub health: UnitHealth,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHeightQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitInBuildStanceQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitInBuildStanceResult {
            pub in_build_stance: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsActiveQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsActiveResult {
            pub is_active: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsBeingBuiltQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsBeingBuiltResult {
            pub is_being_built: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsBuildingQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsBuildingResult {
            pub buildee_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsCloakedQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsCloakedResult {
            pub is_cloaked: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsDeadQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsDeadResult {
            pub is_dead: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsStunnedQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsStunnedResult {
            pub is_stunned: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIsTransportingQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIsTransportingResult {
            pub unit_i_ds: Vec<i32>,
            pub is_transporting: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLastAttackedPieceQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitLastAttackedPieceResult {
            pub piece: LastHitPiece,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLastAttackerQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitLastAttackerResult {
            pub attacker: Option<UnitLastAttacker>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLosStateQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
            pub raw: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLosStateResult {
            pub los_state: UnitLosState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMassQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMassResult {
            pub mass: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMetalExtractionQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMetalExtractionResult {
            pub metal_extraction: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMoveDefIDQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMoveDefIDResult {
            pub move_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNanoPiecesQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNanoPiecesResult {
            pub pieces: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNeutralQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNeutralResult {
            pub neutral: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceCollisionVolumeDataQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceCollisionVolumeDataResult {
            pub volume: CollisionVolumeData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPosErrorParamsQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPosErrorParamsResult {
            pub params: UnitPosErrorParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPositionOptions {
            pub mid_pos: bool,
            pub aim_pos: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPositionQuery {
            pub unit_id: i32,
            pub options: GetUnitPositionOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitRadiusQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitRadiusResult {
            pub radius: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitResourcesQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitResourcesResult {
            pub resources: UnitResources,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitRotationQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitRotationResult {
            pub rotation: UnitRotation,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSeismicSignatureQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSeismicSignatureResult {
            pub seismic_signature: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSelfDTimeQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSelfDTimeResult {
            pub self_d_time: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitSensorRadiusQuery {
            pub unit_id: i32,
            pub type_: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSensorRadiusResult {
            pub radius: UnitSensorRadius,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitShieldStateQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitShieldStateResult {
            pub shield: Option<UnitShieldState>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitStatesQuery {
            pub unit_id: i32,
            pub options: UnitStatesOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitStatesResult {
            pub states: UnitStates,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitStockpileQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitStockpileResult {
            pub stockpile: Option<UnitStockpile>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitStorageQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitStorageResult {
            pub storage: UnitStorage,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitTeamQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitTeamResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitTooltipQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitTooltipResult {
            pub tooltip: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitTransporterQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitTransporterResult {
            pub transporter_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitVectorsQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitVectorsResult {
            pub vectors: UnitVectors,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitVelocityQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitVelocityResult {
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWorkerTaskQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWorkerTaskResult {
            pub task: UnitWorkerTask,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LastHitPiece {
            pub name: String,
            pub piece_num: i32,
            pub frame: i32,
            pub was_hit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitArmoredState {
            pub armored: bool,
            pub armor_multiple: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitBasicInfo {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub team_id: i32,
            pub ally_team_id: i32,
            pub is_neutral: bool,
            pub tooltip: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitBlockingState {
            pub is_blocking: bool,
            pub is_solid_object_collidable: bool,
            pub is_projectile_collidable: bool,
            pub is_ray_segment_collidable: bool,
            pub crushable: bool,
            pub block_enemy_pushing: bool,
            pub block_height_changes: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitBuildParams {
            pub build_distance: f32,
            pub build_speed: f32,
            pub repair_speed: f32,
            pub reclaim_speed: f32,
            pub resurrect_speed: f32,
            pub capture_speed: f32,
            pub terraform_speed: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCosts {
            pub metal_cost: f32,
            pub energy_cost: f32,
            pub build_time: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFlanking {
            pub flanking_mode: u32,
            pub move_factor: f32,
            pub min_damage: f32,
            pub max_damage: f32,
            pub direction: Float3,
            pub mobility: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitHarvestStorage {
            pub stored_metal: f32,
            pub max_stored_metal: f32,
            pub stored_energy: f32,
            pub max_stored_energy: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitHealth {
            pub health: f32,
            pub max_health: f32,
            pub paralyze_damage: f32,
            pub capture_progress: f32,
            pub build_progress: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitLastAttacker {
            pub attacker_id: i32,
            pub attacker_def_id: i32,
            pub attacker_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitLosState {
            pub raw_mask: u8,
            pub los: bool,
            pub radar: bool,
            pub typed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitPosErrorParams {
            pub pos_error_vector: Float3,
            pub pos_error_delta: Float3,
            pub next_pos_error_update: i32,
            pub pos_error_bit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitResources {
            pub metal_make: f32,
            pub metal_use: f32,
            pub energy_make: f32,
            pub energy_use: f32,
            pub metal_income: f32,
            pub energy_income: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitRotation {
            pub pitch: f32,
            pub yaw: f32,
            pub roll: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitSensorRadius {
            pub los: f32,
            pub air_los: f32,
            pub radar: f32,
            pub sonar: f32,
            pub seismic: f32,
            pub radar_jammer: f32,
            pub sonar_jammer: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitShieldState {
            pub shield_enabled: bool,
            pub shield_power: f32,
            pub shield_alpha: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitStates {
            pub fire_state: i32,
            pub move_state: i32,
            pub auto_repair_level: f32,
            pub repeat: bool,
            pub cloak: bool,
            pub active: bool,
            pub trajectory: bool,
            pub auto_land: bool,
            pub loopback_attack: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitStatesOptions {
            pub ret_table: bool,
            pub bin_state: bool,
            pub amt_state: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitStockpile {
            pub stockpile: u32,
            pub stockpile_queue_size: u32,
            pub build_percent: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitStorage {
            pub metal_storage: f32,
            pub energy_storage: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitVectors {
            pub front_dir: Float3,
            pub up_dir: Float3,
            pub right_dir: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWorkerTask {
            pub cmd_id: i32,
            pub target_id: i32,
            pub has_task: bool,
            pub has_target: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_unit_tooltip {
            #[link(wasm_import_module = "spring:units-info")]
            extern "C" {
                #[link_name = "get-unit-tooltip"]
                pub fn call(punit_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitCrashingValue {
            pub is_aircraft: bool,
            pub crashing: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIsTransportingValue {
            pub unit_i_ds: Vec<i32>,
            pub is_transporting: bool,
        }

        #[inline]
        pub fn clear_units_previous_draw_flag(unused: u8) -> Result<bool> {
            let value = crate::generated::units_info::clear_units_previous_draw_flag(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_ally_team(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_info::get_unit_ally_team(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_armored(unit_id: i32) -> Result<UnitArmoredState> {
            let value = crate::generated::units_info::get_unit_armored(unit_id)?;
            Ok(UnitArmoredState { armored: value.armored, armor_multiple: value.armor_multiple })
        }

        #[inline]
        pub fn get_unit_base_position(unit_id: i32) -> Result<Float3> {
            let value = crate::generated::units_info::get_unit_base_position(unit_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_unit_blocking(unit_id: i32) -> Result<UnitBlockingState> {
            let value = crate::generated::units_info::get_unit_blocking(unit_id)?;
            Ok(UnitBlockingState { is_blocking: value.is_blocking, is_solid_object_collidable: value.is_solid_object_collidable, is_projectile_collidable: value.is_projectile_collidable, is_ray_segment_collidable: value.is_ray_segment_collidable, crushable: value.crushable, block_enemy_pushing: value.block_enemy_pushing, block_height_changes: value.block_height_changes })
        }

        #[inline]
        pub fn get_unit_build_facing(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_info::get_unit_build_facing(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_build_params {
            #[link(wasm_import_module = "spring:units-info")]
            unsafe extern "C" {
                #[link_name = "get-unit-build-params"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-info.get-unit-build-params."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_build_params(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_build_params::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_buildee_radius(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_buildee_radius(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_collision_volume_data(unit_id: i32) -> Result<CollisionVolumeData> {
            let value = crate::generated::units_info::get_unit_collision_volume_data(unit_id)?;
            Ok(CollisionVolumeData { scale_x: value.scale_x, scale_y: value.scale_y, scale_z: value.scale_z, offset_x: value.offset_x, offset_y: value.offset_y, offset_z: value.offset_z, volume_type: value.volume_type, test_type: value.test_type, primary_axis: value.primary_axis, disabled: value.disabled })
        }

        #[inline]
        pub fn get_unit_cost_table(unit_id: i32) -> Result<UnitCosts> {
            let value = crate::generated::units_info::get_unit_cost_table(unit_id)?;
            Ok(UnitCosts { metal_cost: value.metal_cost, energy_cost: value.energy_cost, build_time: value.build_time })
        }

        #[inline]
        pub fn get_unit_costs(unit_id: i32) -> Result<UnitCosts> {
            let value = crate::generated::units_info::get_unit_costs(unit_id)?;
            Ok(UnitCosts { metal_cost: value.metal_cost, energy_cost: value.energy_cost, build_time: value.build_time })
        }

        #[inline]
        pub fn get_unit_crashing(unit_id: i32) -> Result<GetUnitCrashingValue> {
            let value = crate::generated::units_info::get_unit_crashing(unit_id)?;
            Ok(GetUnitCrashingValue {
                is_aircraft: value.0,
                crashing: value.1
            })
        }

        #[inline]
        pub fn get_unit_current_build_power(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_current_build_power(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_def_id(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_info::get_unit_def_id(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_direction(unit_id: i32) -> Result<Float3> {
            let value = crate::generated::units_info::get_unit_direction(unit_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_unit_effective_build_range(unit_id: i32, buildee_def_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_effective_build_range(unit_id, buildee_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_experience(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_experience(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_flanking(unit_id: i32) -> Result<UnitFlanking> {
            let value = crate::generated::units_info::get_unit_flanking(unit_id)?;
            Ok(UnitFlanking { flanking_mode: value.flanking_mode, move_factor: value.move_factor, min_damage: value.min_damage, max_damage: value.max_damage, direction: Float3 { x: value.direction.x, y: value.direction.y, z: value.direction.z }, mobility: value.mobility })
        }

        #[inline]
        pub fn get_unit_harvest_storage(unit_id: i32) -> Result<UnitHarvestStorage> {
            let value = crate::generated::units_info::get_unit_harvest_storage(unit_id)?;
            Ok(UnitHarvestStorage { stored_metal: value.stored_metal, max_stored_metal: value.max_stored_metal, stored_energy: value.stored_energy, max_stored_energy: value.max_stored_energy })
        }

        #[inline]
        pub fn get_unit_heading(unit_id: i32, convert_to_radians: bool) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_heading(unit_id, convert_to_radians)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_health(unit_id: i32) -> Result<UnitHealth> {
            let value = crate::generated::units_info::get_unit_health(unit_id)?;
            Ok(UnitHealth { health: value.health, max_health: value.max_health, paralyze_damage: value.paralyze_damage, capture_progress: value.capture_progress, build_progress: value.build_progress })
        }

        #[inline]
        pub fn get_unit_height(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_height(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_in_build_stance(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_info::get_unit_in_build_stance(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_is_active(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_info::get_unit_is_active(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_is_being_built(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_info::get_unit_is_being_built(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_is_building(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_info::get_unit_is_building(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_is_cloaked(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_info::get_unit_is_cloaked(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_is_dead(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_info::get_unit_is_dead(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_is_stunned(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_info::get_unit_is_stunned(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_is_transporting(unit_id: i32) -> Result<GetUnitIsTransportingValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = Vec::<i32>::new();
                let mut state = [0u8; 4];
                loop {
                    let packed = unsafe { __core_units_info_is_transporting::call(unit_id, output.as_mut_ptr() as usize as u32 as i32, output.len() as i32, state.as_mut_ptr() as usize as u32 as i32) } as u64;
                    let status = (packed >> 32) as u32 as i32;
                    let required = packed as u32 as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(GetUnitIsTransportingValue { unit_i_ds: output, is_transporting: u32::from_le_bytes(state) != 0 });
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 { return Err(crate::ApiError::new(status)); }
                    output.resize(required, 0);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            { let _ = unit_id; Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_units_info_is_transporting {
            #[link(wasm_import_module = "spring:units-info")]
            extern "C" {
                #[link_name = "get-unit-is-transporting"]
                pub fn call(unit_id: i32, output: i32, capacity: i32, state: i32) -> i64;
            }
        }

        #[inline]
        pub fn get_unit_last_attacked_piece(unit_id: i32) -> Result<LastHitPiece> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = Vec::<u8>::new();
                loop {
                    match crate::generated::dynamic_output::units_info::get_unit_last_attacked_piece(unit_id, &mut output) {
                        Ok(required) => {
                            output.truncate(required);
                            let mut cursor = 0usize;
                            let name_length = crate::generated::__core_wire::u32(&output, &mut cursor)
                                .ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize;
                            let name_end = cursor.checked_add(name_length)
                                .ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?;
                            let name = super::decode_core_string(output.get(cursor..name_end)
                                .ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
                                .to_vec());
                            cursor = name_end;
                            let piece_num = crate::generated::__core_wire::i32(&output, &mut cursor)
                                .ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?;
                            let frame = crate::generated::__core_wire::i32(&output, &mut cursor)
                                .ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?;
                            let was_hit = crate::generated::__core_wire::boolean(&output, &mut cursor)
                                .ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?;
                            if !crate::generated::__core_wire::finish(&output, &mut cursor, 8) {
                                return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                            }
                            return Ok(LastHitPiece { name, piece_num, frame, was_hit });
                        }
                        Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                            output.resize(error.required, 0);
                        }
                        Err(error) => return Err(error.error),
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = unit_id;
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_last_attacker {
            #[link(wasm_import_module = "spring:units-info")]
            unsafe extern "C" {
                #[link_name = "get-unit-last-attacker"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-info.get-unit-last-attacker."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_last_attacker(p0: i32, p1: i32) -> i32 {
            __core_owned_get_unit_last_attacker::call(p0, p1)
        }

        #[inline]
        pub fn get_unit_los_state(unit_id: i32, ally_team_id: i32, raw: bool) -> Result<UnitLosState> {
            let value = crate::generated::units_info::get_unit_los_state(unit_id, ally_team_id, raw)?;
            Ok(UnitLosState { raw_mask: value.raw_mask, los: value.los, radar: value.radar, typed: value.typed })
        }

        #[inline]
        pub fn get_unit_mass(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_mass(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_metal_extraction(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_metal_extraction(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_move_def_id(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_info::get_unit_move_def_id(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_nano_pieces(unit_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = Vec::<i32>::new();
                loop {
                    let packed = unsafe { __core_units_info_nano_pieces::call(unit_id, output.as_mut_ptr() as usize as u32 as i32, output.len() as i32) } as u64;
                    let status = (packed >> 32) as u32 as i32;
                    let required = packed as u32 as usize;
                    if status == 0 { output.truncate(required); return Ok(output); }
                    if status != crate::ErrorCode::BufferOverflow as i32 { return Err(crate::ApiError::new(status)); }
                    output.resize(required, 0);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            { let _ = unit_id; Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_units_info_nano_pieces {
            #[link(wasm_import_module = "spring:units-info")]
            extern "C" {
                #[link_name = "get-unit-nano-pieces"]
                pub fn call(unit_id: i32, output: i32, capacity: i32) -> i64;
            }
        }

        #[inline]
        pub fn get_unit_neutral(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_info::get_unit_neutral(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_piece_collision_volume_data(unit_id: i32, piece_num: i32) -> Result<CollisionVolumeData> {
            let value = crate::generated::units_info::get_unit_piece_collision_volume_data(unit_id, piece_num)?;
            Ok(CollisionVolumeData { scale_x: value.scale_x, scale_y: value.scale_y, scale_z: value.scale_z, offset_x: value.offset_x, offset_y: value.offset_y, offset_z: value.offset_z, volume_type: value.volume_type, test_type: value.test_type, primary_axis: value.primary_axis, disabled: value.disabled })
        }

        #[inline]
        pub fn get_unit_pos_error_params(unit_id: i32, ally_team_id: i32) -> Result<UnitPosErrorParams> {
            let value = crate::generated::units_info::get_unit_pos_error_params(unit_id, ally_team_id)?;
            Ok(UnitPosErrorParams { pos_error_vector: Float3 { x: value.pos_error_vector.x, y: value.pos_error_vector.y, z: value.pos_error_vector.z }, pos_error_delta: Float3 { x: value.pos_error_delta.x, y: value.pos_error_delta.y, z: value.pos_error_delta.z }, next_pos_error_update: value.next_pos_error_update, pos_error_bit: value.pos_error_bit })
        }

        #[inline]
        pub fn get_unit_position(unit_id: i32, options: GetUnitPositionOptions) -> Result<Float3> {
            let value = crate::generated::units_info::get_unit_position(unit_id, crate::generated::units_info::GetUnitPositionOptions { mid_pos: options.mid_pos, aim_pos: options.aim_pos })?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_unit_radius(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_radius(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_resources(unit_id: i32) -> Result<UnitResources> {
            let value = crate::generated::units_info::get_unit_resources(unit_id)?;
            Ok(UnitResources { metal_make: value.metal_make, metal_use: value.metal_use, energy_make: value.energy_make, energy_use: value.energy_use, metal_income: value.metal_income, energy_income: value.energy_income })
        }

        #[inline]
        pub fn get_unit_rotation(unit_id: i32) -> Result<UnitRotation> {
            let value = crate::generated::units_info::get_unit_rotation(unit_id)?;
            Ok(UnitRotation { pitch: value.pitch, yaw: value.yaw, roll: value.roll })
        }

        #[inline]
        pub fn get_unit_seismic_signature(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_seismic_signature(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_self_d_time(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_info::get_unit_self_d_time(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_sensor_radius(unit_id: i32, type_: &str) -> Result<UnitSensorRadius> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + type_.len()); __b.extend_from_slice(&(type_.len() as u32).to_le_bytes()); __b.extend_from_slice(type_.as_bytes()); __b };
            let mut __output = [0u8; 28];
            crate::generated::dynamic_input::units_info::get_unit_sensor_radius(unit_id, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(UnitSensorRadius { los: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, air_los: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, radar: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, sonar: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, seismic: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, radar_jammer: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, sonar_jammer: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_shield_state {
            #[link(wasm_import_module = "spring:units-info")]
            unsafe extern "C" {
                #[link_name = "get-unit-shield-state"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-info.get-unit-shield-state."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_shield_state(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_shield_state::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_states(unit_id: i32, options: UnitStatesOptions) -> Result<UnitStates> {
            let value = crate::generated::units_info::get_unit_states(unit_id, crate::generated::units_info::UnitStatesOptions { ret_table: options.ret_table, bin_state: options.bin_state, amt_state: options.amt_state })?;
            Ok(UnitStates { fire_state: value.fire_state, move_state: value.move_state, auto_repair_level: value.auto_repair_level, repeat: value.repeat, cloak: value.cloak, active: value.active, trajectory: value.trajectory, auto_land: value.auto_land, loopback_attack: value.loopback_attack })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_stockpile {
            #[link(wasm_import_module = "spring:units-info")]
            unsafe extern "C" {
                #[link_name = "get-unit-stockpile"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-info.get-unit-stockpile."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_stockpile(p0: i32, p1: i32) -> i32 {
            __core_owned_get_unit_stockpile::call(p0, p1)
        }

        #[inline]
        pub fn get_unit_storage(unit_id: i32) -> Result<UnitStorage> {
            let value = crate::generated::units_info::get_unit_storage(unit_id)?;
            Ok(UnitStorage { metal_storage: value.metal_storage, energy_storage: value.energy_storage })
        }

        #[inline]
        pub fn get_unit_team(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_info::get_unit_team(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_tooltip(unit_id: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_unit_tooltip::call(unit_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (unit_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_unit_transporter(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_info::get_unit_transporter(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_vectors(unit_id: i32) -> Result<UnitVectors> {
            let value = crate::generated::units_info::get_unit_vectors(unit_id)?;
            Ok(UnitVectors { front_dir: Float3 { x: value.front_dir.x, y: value.front_dir.y, z: value.front_dir.z }, up_dir: Float3 { x: value.up_dir.x, y: value.up_dir.y, z: value.up_dir.z }, right_dir: Float3 { x: value.right_dir.x, y: value.right_dir.y, z: value.right_dir.z } })
        }

        #[inline]
        pub fn get_unit_velocity(unit_id: i32) -> Result<Float3> {
            let value = crate::generated::units_info::get_unit_velocity(unit_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_unit_worker_task(unit_id: i32) -> Result<UnitWorkerTask> {
            let value = crate::generated::units_info::get_unit_worker_task(unit_id)?;
            Ok(UnitWorkerTask { cmd_id: value.cmd_id, target_id: value.target_id, has_task: value.has_task, has_target: value.has_target })
        }

    }


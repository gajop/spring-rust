    pub mod types {
        use super::{String, Vec};

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

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum UnitFilter {
            UnitFilterAll,
            UnitFilterAllyteam,
            UnitFilterAllyUnits,
            UnitFilterEnemyUnits,
            UnitFilterMyUnits,
            UnitFilterTeam,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ActiveCommandDescription {
            pub id: i32,
            pub type_: i32,
            pub name: String,
            pub action: String,
            pub tooltip: String,
            pub texture: String,
            pub cursor: String,
            pub queueing: bool,
            pub hidden: bool,
            pub disabled: bool,
            pub show_unique: bool,
            pub only_texture: bool,
            pub params: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddFeatureDamageQuery {
            pub feature_id: i32,
            pub damage: f32,
            pub paralyze_time: f32,
            pub weapon_def_id: i32,
            pub attacker_id: i32,
            pub impulse: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddFeatureDamageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddGrassQuery {
            pub x: f32,
            pub z: f32,
            pub grass_value: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddGrassResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddHeightMapQuery {
            pub x: f32,
            pub z: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddObjectDecalQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddObjectDecalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddOriginalHeightMapQuery {
            pub x: f32,
            pub z: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddOriginalHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddSmoothMeshQuery {
            pub x: f32,
            pub z: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddSmoothMeshResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddTeamResourceExcessStatsQuery {
            pub team_id: i32,
            pub resource_type: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddTeamResourceExcessStatsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddTeamResourceQuery {
            pub team_id: i32,
            pub resource_type: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddTeamResourceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitDamageQuery {
            pub unit_id: i32,
            pub damage: f32,
            pub paralyze_time: f32,
            pub weapon_def_id: i32,
            pub attacker_id: i32,
            pub impulse: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitDamageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitExperienceQuery {
            pub unit_id: i32,
            pub experience: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitExperienceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitImpulseQuery {
            pub unit_id: i32,
            pub impulse: Float3,
            pub decay_rate: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitImpulseResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddUnitResourceQuery {
            pub unit_id: i32,
            pub resource_type: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitResourceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitSeismicPingQuery {
            pub unit_id: i32,
            pub ping_size: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitSeismicPingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AdjustHeightMapQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AdjustHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AdjustOriginalHeightMapQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AdjustOriginalHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AdjustSmoothMeshQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AdjustSmoothMeshResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AssignPlayerToTeamQuery {
            pub player_id: i32,
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AssignPlayerToTeamResult {
            pub success: bool,
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BoxQuery {
            pub min: Float3,
            pub max: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BuggerOffOptions {
            pub spherical: bool,
            pub forced: bool,
            pub exclude_unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BuggerOffQuery {
            pub pos: Float3,
            pub radius: f32,
            pub team_id: i32,
            pub options: BuggerOffOptions,
            pub exclude_unit_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BuggerOffResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct COBScriptApi {
            pub call_cob_script: u32,
            pub get_cob_script_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CallCOBScriptQuery {
            pub unit_id: i32,
            pub func: CobFunctionRef,
            pub ret_args: u32,
            pub args: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CallCOBScriptResult {
            pub ret_code: i32,
            pub ret_values: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CallUnitScriptQuery {
            pub unit_id: i32,
            pub function_name: String,
            pub args: Vec<f32>,
            pub ret_capacity: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CallUnitScriptResult {
            pub function_found: bool,
            pub success: bool,
            pub ret_values: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearUnitGoalQuery {
            pub unit_id: i32,
            pub cancel_raw: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearUnitGoalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CobFunctionRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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
        pub struct CreateFeatureQuery {
            pub feature_def: DefRef,
            pub pos: Float3,
            pub facing: i32,
            pub team_id: i32,
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateFeatureResult {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateFeatureWreckQuery {
            pub feature_id: i32,
            pub wreck_level: i32,
            pub do_smoke: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateFeatureWreckResult {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateUnitOptions {
            pub build: bool,
            pub flatten_ground: bool,
            pub unit_id: i32,
            pub builder_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateUnitQuery {
            pub unit_def: DefRef,
            pub pos: Float3,
            pub facing: i32,
            pub team_id: i32,
            pub options: CreateUnitOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateUnitResult {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateUnitWreckQuery {
            pub unit_id: i32,
            pub wreck_level: i32,
            pub do_smoke: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateUnitWreckResult {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CylinderQuery {
            pub center: Float3,
            pub radius: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DeleteProjectileQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DeleteProjectileResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DestroyFeatureQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DestroyFeatureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DestroyUnitOptions {
            pub selfd: bool,
            pub reclaimed: bool,
            pub attacker_id: i32,
            pub recycle_id: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DestroyUnitQuery {
            pub unit_id: i32,
            pub options: DestroyUnitOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DestroyUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EditUnitCmdDescQuery {
            pub unit_id: i32,
            pub cmd_desc_index: u32,
            pub cmd_desc: NativeCommandDescription,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EditUnitCmdDescResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EffectsControlApi {
            pub spawn_explosion: u32,
            pub spawn_ceg: u32,
            pub spawn_sfx: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureControlApi {
            pub create_feature: u32,
            pub destroy_feature: u32,
            pub transfer_feature: u32,
            pub set_feature_health: u32,
            pub set_feature_position: u32,
            pub set_feature_direction: u32,
            pub set_feature_velocity: u32,
            pub set_feature_resources: u32,
            pub add_feature_damage: u32,
            pub set_feature_blocking: u32,
            pub set_feature_mass: u32,
            pub set_feature_max_health: u32,
            pub set_feature_reclaim: u32,
            pub set_feature_resurrect: u32,
            pub set_feature_physics: u32,
            pub set_feature_move_ctrl: u32,
            pub set_feature_heading_and_up_dir: u32,
            pub set_feature_rotation: u32,
            pub set_feature_always_visible: u32,
            pub set_feature_use_air_los: u32,
            pub set_feature_no_select: u32,
            pub set_feature_mid_and_aim_pos: u32,
            pub set_feature_radius_and_height: u32,
            pub set_feature_collision_volume_data: u32,
            pub set_feature_selection_volume_data: u32,
            pub set_feature_fire_time: u32,
            pub set_feature_smoke_time: u32,
            pub create_unit_wreck: u32,
            pub create_feature_wreck: u32,
            pub set_feature_piece_visible: u32,
            pub set_feature_piece_matrix: u32,
            pub set_feature_piece_collision_volume_data: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Float2 {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Float2Result {
            pub value: Float2,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Float3Result {
            pub value: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Float4 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Float4Result {
            pub value: Float4,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ForceUnitCollisionUpdateQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ForceUnitCollisionUpdateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameConfigApi {
            pub set_no_pause: u32,
            pub set_cheating_enabled: u32,
            pub set_god_mode: u32,
            pub set_experience_grade: u32,
            pub set_radar_error_params: u32,
            pub set_square_building_mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameOverQuery {
            pub winning_ally_teams: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameOverResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetActiveCmdDescQuery {
            pub cmd_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCmdDescResult {
            pub cmd_desc: ActiveCommandDescription,
            pub has_command: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetActiveCmdDescsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCmdDescsResult {
            pub cmd_descs: Vec<ActiveCommandDescription>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllUnitsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetBoxSelectionByEngineQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetBoxSelectionByEngineResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetBuildFacingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetBuildFacingResult {
            pub facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetBuildSpacingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetBuildSpacingResult {
            pub spacing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCOBScriptIDQuery {
            pub unit_id: i32,
            pub func_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCOBScriptIDResult {
            pub func_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraRotationQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraRotationResult {
            pub rot_x: f32,
            pub rot_y: f32,
            pub rot_z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraVectorsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraVectorsResult {
            pub forward: Float3,
            pub up: Float3,
            pub right: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetClipboardQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetClipboardResult {
            pub text: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetClosestEnemyUnitOptions {
            pub use_los: bool,
            pub sphere_dist_test: bool,
            pub check_sight_dist: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetClosestEnemyUnitQuery {
            pub pos: Float3,
            pub range: f32,
            pub ally_team_id: i32,
            pub options: GetClosestEnemyUnitOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetClosestEnemyUnitResult {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCmdDescIndexQuery {
            pub cmd_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCmdDescIndexResult {
            pub index: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCustomPaletteColorQuery {
            pub index: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCustomPaletteColorResult {
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDrawSelectionInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDrawSelectionInfoResult {
            pub draw: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePaletteIndexQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePaletteIndexResult {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturesInScreenRectangleQuery {
            pub left: f32,
            pub top: f32,
            pub right: f32,
            pub bottom: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturesInScreenRectangleResult {
            pub feature_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFrustumPlanesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFrustumPlanesResult {
            pub planes: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSecondsInterpolatedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSecondsInterpolatedResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLastMessagePositionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLastMessagePositionsResult {
            pub positions: Vec<Float3>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetNanoProjectileParamsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetNanoProjectileParamsResult {
            pub r: f32,
            pub v: f32,
            pub a: f32,
            pub rand_r: f32,
            pub rand_v: f32,
            pub rand_a: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPieceProjectileNameQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPrevFrameSyncChecksumQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPrevFrameSyncChecksumResult {
            pub checksum: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRenderUnitsDrawFlagChangedQuery {
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderUnitsDrawFlagChangedResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRenderUnitsQuery {
            pub draw_mask: i32,
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamDamageStatsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamDamageStatsResult {
            pub damage_dealt: f32,
            pub damage_received: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitCountQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitDefCountQuery {
            pub team_id: i32,
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitDefCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsByDefsQuery {
            pub team_id: i32,
            pub unit_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsByDefsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitsCountsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsCountsResult {
            pub counts: Vec<UnitDefCount>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitsSortedQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsSortedResult {
            pub groups: Vec<TeamUnitsByDef>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitAlwaysUpdateMatrixQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitAlwaysUpdateMatrixResult {
            pub always_update_matrix: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitArrayCentroidQuery {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitArrayCentroidResult {
            pub centroid: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDrawFlagQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDrawFlagResult {
            pub draw_flag: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitEngineDrawMaskQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitEngineDrawMaskResult {
            pub engine_draw_mask: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitFeatureSeparationQuery {
            pub unit_id: i32,
            pub feature_id: i32,
            pub ignore_y: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitFeatureSeparationResult {
            pub distance: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIconDataQuery {
            pub unit_id: i32,
            pub full_data: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIconDataResult {
            pub icon_name: String,
            pub atlas_tex_coords: Vec<f32>,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitIconQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIconResult {
            pub icon_name: String,
            pub atlas_tex_coords: Vec<f32>,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLeavesGhostQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLeavesGhostResult {
            pub leaves_ghost: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLuaDrawQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitLuaDrawResult {
            pub lua_draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitMapCentroidQuery {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMapCentroidResult {
            pub centroid: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNearestAllyQuery {
            pub unit_id: i32,
            pub range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNearestAllyResult {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNearestEnemyOptions {
            pub use_los: bool,
            pub sphere_dist_test: bool,
            pub check_sight_dist: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNearestEnemyQuery {
            pub unit_id: i32,
            pub range: f32,
            pub options: GetUnitNearestEnemyOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNearestEnemyResult {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoDrawQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoDrawResult {
            pub no_draw: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoGroupQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoGroupResult {
            pub no_group: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoMinimapQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoMinimapResult {
            pub no_minimap: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoSelectQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitNoSelectResult {
            pub no_select: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPaletteIndexQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPaletteIndexResult {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPhysicalStateQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPhysicalStateResult {
            pub physical_state: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSelectionVolumeDataQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSelectionVolumeDataResult {
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub use_cont_hit_test: bool,
            pub primary_axis: i32,
            pub ignore_hits: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSeparationOptions {
            pub positional: bool,
            pub check_map: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSeparationQuery {
            pub unit_id1: i32,
            pub unit_id2: i32,
            pub options: GetUnitSeparationOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSeparationResult {
            pub separation: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitTransformMatrixQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitTransformMatrixResult {
            pub matrix: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitViewPositionQuery {
            pub unit_id: i32,
            pub use_mid_pos: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitViewPositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitsInBoxQuery {
            pub xmin: f32,
            pub ymin: f32,
            pub zmin: f32,
            pub xmax: f32,
            pub ymax: f32,
            pub zmax: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInBoxResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitsInCylinderQuery {
            pub x: f32,
            pub z: f32,
            pub radius: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInCylinderResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInPlanesQuery {
            pub planes: PlanesQuery,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInPlanesResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitsInRectangleQuery {
            pub xmin: f32,
            pub zmin: f32,
            pub xmax: f32,
            pub zmax: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInRectangleResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitsInScreenRectangleQuery {
            pub left: f32,
            pub top: f32,
            pub right: f32,
            pub bottom: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInScreenRectangleResult {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitsInSphereQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub radius: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInSphereResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVisibleFeaturesOptions {
            pub include_icons: bool,
            pub include_geos: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVisibleFeaturesQuery {
            pub ally_team_id: i32,
            pub radius: f32,
            pub options: GetVisibleFeaturesOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleFeaturesResult {
            pub feature_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVisibleProjectilesOptions {
            pub include_synced_projectiles: bool,
            pub include_weapon_projectiles: bool,
            pub include_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVisibleProjectilesQuery {
            pub ally_team_id: i32,
            pub options: GetVisibleProjectilesOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleProjectilesResult {
            pub projectile_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVisibleUnitsQuery {
            pub team_id: i32,
            pub radius: f32,
            pub include_icons: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleUnitsResult {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderArrayToUnitArrayQuery {
            pub unit_i_ds: Vec<i32>,
            pub commands: Vec<NativeCommand>,
            pub pairwise: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GiveOrderArrayToUnitArrayResult {
            pub units_ordered: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderArrayToUnitQuery {
            pub unit_id: i32,
            pub commands: Vec<NativeCommand>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GiveOrderArrayToUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderToUnitArrayQuery {
            pub unit_i_ds: Vec<i32>,
            pub cmd_id: i32,
            pub params: Vec<f32>,
            pub options: u32,
            pub timeout: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GiveOrderToUnitArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GiveOrderToUnitQuery {
            pub unit_id: i32,
            pub cmd_id: i32,
            pub params: Vec<f32>,
            pub options: u32,
            pub timeout: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GiveOrderToUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct InsertUnitCmdDescQuery {
            pub unit_id: i32,
            pub cmd_desc_index: i32,
            pub cmd_desc: NativeCommandDescription,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct InsertUnitCmdDescResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Int2 {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Int32Result {
            pub value: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitAlliedQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitAlliedResult {
            pub allied: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitIconQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitIconResult {
            pub is_icon: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInViewQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInViewResult {
            pub in_view: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitSelectedQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitSelectedResult {
            pub selected: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitVisibleQuery {
            pub unit_id: i32,
            pub radius: f32,
            pub check_icon: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitVisibleResult {
            pub visible: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct KillTeamQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct KillTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LevelHeightMapQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LevelHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LevelOriginalHeightMapQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LevelOriginalHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LevelSmoothMeshQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LevelSmoothMeshResult {
            pub success: bool,
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
        pub struct NativeCommand {
            pub cmd_id: i32,
            pub params: Vec<f32>,
            pub options: u32,
            pub timeout: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeCommandDescription {
            pub id: i32,
            pub type_: i32,
            pub queueing: bool,
            pub hidden: bool,
            pub disabled: bool,
            pub show_unique: bool,
            pub only_texture: bool,
            pub name: String,
            pub action: String,
            pub iconname: String,
            pub mouseicon: String,
            pub tooltip: String,
            pub params: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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
            pub error: Float3,
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct NumberOrBool {
            pub number: f32,
            pub boolean: bool,
            pub use_boolean: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlanesQuery {
            pub planes: Vec<Float4>,
            pub plane_count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileControlApi {
            pub spawn_projectile: u32,
            pub delete_projectile: u32,
            pub set_projectile_position: u32,
            pub set_projectile_velocity: u32,
            pub set_projectile_gravity: u32,
            pub set_projectile_target: u32,
            pub set_projectile_damages: u32,
            pub set_projectile_time_to_live: u32,
            pub set_projectile_is_intercepted: u32,
            pub set_projectile_collision: u32,
            pub set_projectile_ceg: u32,
            pub set_projectile_always_visible: u32,
            pub set_projectile_use_air_los: u32,
            pub set_projectile_move_control: u32,
            pub set_projectile_ignore_tracking_error: u32,
            pub set_piece_projectile_params: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RebuildSmoothMeshQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RebuildSmoothMeshResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RectangleQuery {
            pub min_x: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RemoveGrassQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RemoveGrassResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RemoveObjectDecalQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RemoveObjectDecalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RemoveUnitCmdDescQuery {
            pub unit_id: i32,
            pub cmd_desc_index: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RemoveUnitCmdDescResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ResourcePack {
            pub metal: f32,
            pub energy: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RevertHeightMapQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub orig_factor: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RevertHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RevertOriginalHeightMapQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub orig_factor: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RevertOriginalHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RevertSmoothMeshQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
            pub orig_factor: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RevertSmoothMeshResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RgbColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetAllyQuery {
            pub first_ally_team_id: i32,
            pub second_ally_team_id: i32,
            pub allied: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetAllyResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetAllyTeamStartBoxQuery {
            pub ally_team_id: i32,
            pub min_x: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetAllyTeamStartBoxResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetCheatingEnabledQuery {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetCheatingEnabledResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetExperienceGradeQuery {
            pub exp_grade: f32,
            pub exp_power_scale: f32,
            pub exp_health_scale: f32,
            pub exp_reload_scale: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetExperienceGradeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFactoryBuggerOffOptions {
            pub perform: bool,
            pub offset: f32,
            pub radius: f32,
            pub rel_heading: i32,
            pub spherical: bool,
            pub forced: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFactoryBuggerOffQuery {
            pub unit_id: i32,
            pub options: SetFactoryBuggerOffOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFactoryBuggerOffResult {
            pub perform: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureAlwaysVisibleQuery {
            pub feature_id: i32,
            pub always_visible: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureAlwaysVisibleResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureBlockingOptions {
            pub blocking: bool,
            pub solid_objects: bool,
            pub projectiles: bool,
            pub quad_map_rays: bool,
            pub crushable: bool,
            pub block_enemy_pushing: bool,
            pub block_height_changes: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureBlockingQuery {
            pub feature_id: i32,
            pub options: SetFeatureBlockingOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureBlockingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureCollisionVolumeDataQuery {
            pub feature_id: i32,
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub test_type: i32,
            pub primary_axis: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureCollisionVolumeDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureDirectionQuery {
            pub feature_id: i32,
            pub front_dir: Float3,
            pub right_dir: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureDirectionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureFireTimeQuery {
            pub feature_id: i32,
            pub fire_time: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureFireTimeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureHeadingAndUpDirQuery {
            pub feature_id: i32,
            pub heading: i32,
            pub up_dir: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureHeadingAndUpDirResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureHealthQuery {
            pub feature_id: i32,
            pub health: f32,
            pub check_destruction: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureHealthResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMassQuery {
            pub feature_id: i32,
            pub mass: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMassResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMaxHealthQuery {
            pub feature_id: i32,
            pub max_health: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMaxHealthResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMidAndAimPosQuery {
            pub feature_id: i32,
            pub mid_pos: Float3,
            pub aim_pos: Float3,
            pub set_relative: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMidAndAimPosResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMoveCtrlQuery {
            pub feature_id: i32,
            pub enable: bool,
            pub velocity_or_mask: Float3,
            pub acceleration_or_impulse_mask: Float3,
            pub movement_mask: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureMoveCtrlResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureNoSelectQuery {
            pub feature_id: i32,
            pub no_select: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureNoSelectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePhysicsQuery {
            pub feature_id: i32,
            pub pos: Float3,
            pub velocity: Float3,
            pub rotation: Float3,
            pub drag: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePhysicsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePieceCollisionVolumeDataQuery {
            pub feature_id: i32,
            pub piece_index: i32,
            pub enable: bool,
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub primary_axis: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePieceCollisionVolumeDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeaturePieceMatrixQuery {
            pub feature_id: i32,
            pub piece_index: i32,
            pub matrix: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePieceMatrixResult {
            pub block_script_anims: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePieceVisibleQuery {
            pub feature_id: i32,
            pub piece_index: i32,
            pub visible: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePieceVisibleResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePositionQuery {
            pub feature_id: i32,
            pub pos: Float3,
            pub snap_to_ground: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeaturePositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureRadiusAndHeightQuery {
            pub feature_id: i32,
            pub radius: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureRadiusAndHeightResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureReclaimQuery {
            pub feature_id: i32,
            pub reclaim_left: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureReclaimResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureResourcesQuery {
            pub feature_id: i32,
            pub metal: f32,
            pub energy: f32,
            pub reclaim_time: f32,
            pub reclaim_left: f32,
            pub feature_def_metal: f32,
            pub feature_def_energy: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureResourcesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureResurrectQuery {
            pub feature_id: i32,
            pub unit_def: DefRef,
            pub facing: i32,
            pub progress: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureResurrectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureRotationQuery {
            pub feature_id: i32,
            pub rotation: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureRotationResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureSelectionVolumeDataQuery {
            pub feature_id: i32,
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub primary_axis: i32,
            pub use_cont_hit_test: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureSelectionVolumeDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureSmokeTimeQuery {
            pub feature_id: i32,
            pub smoke_time: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureSmokeTimeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureUseAirLosQuery {
            pub feature_id: i32,
            pub use_air_los: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureUseAirLosResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureVelocityQuery {
            pub feature_id: i32,
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureVelocityResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetGlobalLosQuery {
            pub ally_team_id: i32,
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetGlobalLosResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetGodModeOptions {
            pub control_allies: bool,
            pub control_enemies: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetGodModeQuery {
            pub options: SetGodModeOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetGodModeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetHeightMapFuncQuery {
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetHeightMapFuncResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetHeightMapQuery {
            pub x: f32,
            pub z: f32,
            pub height: f32,
            pub terraform: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMapSquareTerrainTypeQuery {
            pub x: i32,
            pub z: i32,
            pub terrain_type: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMapSquareTerrainTypeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetNoPauseQuery {
            pub no_pause: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetNoPauseResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetOriginalHeightMapFuncQuery {
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetOriginalHeightMapFuncResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetOriginalHeightMapQuery {
            pub x: f32,
            pub z: f32,
            pub height: f32,
            pub factor: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetOriginalHeightMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPieceProjectileParamsQuery {
            pub projectile_id: i32,
            pub expl_flags: i32,
            pub spin_angle: f32,
            pub spin_speed: f32,
            pub spin_vec: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPieceProjectileParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPlayerReadyStateQuery {
            pub player_id: i32,
            pub ready: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPlayerReadyStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileAlwaysVisibleQuery {
            pub projectile_id: i32,
            pub always_visible: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileAlwaysVisibleResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetProjectileCEGQuery {
            pub projectile_id: i32,
            pub ceg_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileCEGResult {
            pub ceg_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileCollisionQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileCollisionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetProjectileDamagesQuery {
            pub projectile_id: i32,
            pub unused: i32,
            pub damage_key: String,
            pub damage_value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileDamagesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileGravityQuery {
            pub projectile_id: i32,
            pub gravity: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileGravityResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileIgnoreTrackingErrorQuery {
            pub projectile_id: i32,
            pub ignore: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileIgnoreTrackingErrorResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileIsInterceptedQuery {
            pub projectile_id: i32,
            pub intercepted: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileIsInterceptedResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileMoveControlQuery {
            pub projectile_id: i32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileMoveControlResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectilePositionQuery {
            pub projectile_id: i32,
            pub pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectilePositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileTargetQuery {
            pub projectile_id: i32,
            pub target: ProjectileTargetRef,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileTargetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileTimeToLiveQuery {
            pub projectile_id: i32,
            pub time_to_live: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileTimeToLiveResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileUseAirLosQuery {
            pub projectile_id: i32,
            pub use_air_los: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileUseAirLosResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileVelocityQuery {
            pub projectile_id: i32,
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetProjectileVelocityResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetRadarErrorParamsQuery {
            pub ally_team_id: i32,
            pub ally_team_error_size: f32,
            pub base_error_size: f32,
            pub base_error_mult: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetRadarErrorParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSmoothMeshFuncQuery {
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSmoothMeshFuncResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSmoothMeshQuery {
            pub x: f32,
            pub z: f32,
            pub height: f32,
            pub terraform: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSmoothMeshResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSquareBuildingMaskQuery {
            pub x: i32,
            pub z: i32,
            pub mask: u16,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSquareBuildingMaskResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetTeamResourceQuery {
            pub team_id: i32,
            pub resource_type: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTeamResourceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetTeamShareLevelQuery {
            pub team_id: i32,
            pub resource_type: String,
            pub share_level: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTeamShareLevelResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTeamStartPositionQuery {
            pub team_id: i32,
            pub pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTeamStartPositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetTerrainTypeDataQuery {
            pub type_index: i32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub hardness: f32,
            pub receive_tracks: bool,
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTerrainTypeDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTidalQuery {
            pub tidal: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTidalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitAlwaysVisibleQuery {
            pub unit_id: i32,
            pub always_visible: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitAlwaysVisibleResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitArmoredQuery {
            pub unit_id: i32,
            pub armored_state: bool,
            pub armored_multiple: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitArmoredResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBlockingOptions {
            pub blocking: bool,
            pub solid_objects: bool,
            pub projectiles: bool,
            pub quad_map_rays: bool,
            pub crushable: bool,
            pub block_enemy_pushing: bool,
            pub block_height_changes: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBlockingQuery {
            pub unit_id: i32,
            pub options: SetUnitBlockingOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBlockingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitBuildParamsQuery {
            pub unit_id: i32,
            pub param_name: String,
            pub value: NumberOrBool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBuildParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBuildSpeedQuery {
            pub unit_id: i32,
            pub build_speed: f32,
            pub repair_speed: f32,
            pub reclaim_speed: f32,
            pub resurrect_speed: f32,
            pub capture_speed: f32,
            pub terraform_speed: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBuildSpeedResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBuildeeRadiusQuery {
            pub unit_id: i32,
            pub radius: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitBuildeeRadiusResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCloakQuery {
            pub unit_id: i32,
            pub cloak: NumberOrBool,
            pub cloak_arg: NumberOrBool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCloakResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCollisionVolumeDataQuery {
            pub unit_id: i32,
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub test_type: i32,
            pub primary_axis: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCollisionVolumeDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCostsQuery {
            pub unit_id: i32,
            pub costs: UnitCostOverrides,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCostsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCrashingQuery {
            pub unit_id: i32,
            pub want_crash: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitCrashingResult {
            pub state_changed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitDirectionQuery {
            pub unit_id: i32,
            pub front_dir: Float3,
            pub right_dir: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitDirectionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitExperienceQuery {
            pub unit_id: i32,
            pub experience: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitExperienceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitFlankingQuery {
            pub unit_id: i32,
            pub type_: String,
            pub args: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitFlankingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHarvestStorageQuery {
            pub unit_id: i32,
            pub stored_metal: f32,
            pub max_stored_metal: f32,
            pub stored_energy: f32,
            pub max_stored_energy: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHarvestStorageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHeadingAndUpDirQuery {
            pub unit_id: i32,
            pub heading: i32,
            pub up_dir: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHeadingAndUpDirResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHeadingQuery {
            pub unit_id: i32,
            pub heading: i32,
            pub use_smoothing: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHeadingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHealthQuery {
            pub unit_id: i32,
            pub value: UnitHealthValue,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitHealthResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLandGoalQuery {
            pub unit_id: i32,
            pub pos: Float3,
            pub radius_sq: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLandGoalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLeavesGhostOptions {
            pub leaves_ghost: bool,
            pub leave_dead_ghost: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLeavesGhostQuery {
            pub unit_id: i32,
            pub options: SetUnitLeavesGhostOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLeavesGhostResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLoadingTransportQuery {
            pub unit_id: i32,
            pub transport_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLoadingTransportResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLosMaskQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
            pub los_mask: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLosMaskResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLosStateQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
            pub los_state: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitLosStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMassQuery {
            pub unit_id: i32,
            pub mass: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMassResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMaxHealthQuery {
            pub unit_id: i32,
            pub max_health: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMaxHealthResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMaxRangeQuery {
            pub unit_id: i32,
            pub max_range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMaxRangeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMetalExtractionQuery {
            pub unit_id: i32,
            pub depth: f32,
            pub range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMetalExtractionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMidAndAimPosQuery {
            pub unit_id: i32,
            pub mid_pos: Float3,
            pub aim_pos: Float3,
            pub set_relative: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMidAndAimPosResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMoveGoalQuery {
            pub unit_id: i32,
            pub pos: Float3,
            pub radius: f32,
            pub speed: f32,
            pub raw: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitMoveGoalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNanoPiecesQuery {
            pub unit_id: i32,
            pub piece_indices: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitNanoPiecesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitNeutralQuery {
            pub unit_id: i32,
            pub neutral: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitNeutralResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPhysicalStateBitQuery {
            pub unit_id: i32,
            pub state_bit: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPhysicalStateBitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPhysicsQuery {
            pub unit_id: i32,
            pub pos: Float3,
            pub velocity: Float3,
            pub rotation: Float3,
            pub drag: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPhysicsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPieceCollisionVolumeDataQuery {
            pub unit_id: i32,
            pub piece_index: i32,
            pub enable: bool,
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub primary_axis: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPieceCollisionVolumeDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitPieceMatrixQuery {
            pub unit_id: i32,
            pub piece_index: i32,
            pub matrix: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPieceMatrixResult {
            pub block_script_anims: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPieceParentQuery {
            pub unit_id: i32,
            pub child_piece_index: i32,
            pub parent_piece_index: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPieceParentResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPieceVisibleQuery {
            pub unit_id: i32,
            pub piece_index: i32,
            pub visible: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPieceVisibleResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPosErrorParamsQuery {
            pub unit_id: i32,
            pub pos_error_vector: Float3,
            pub pos_error_delta: Float3,
            pub next_pos_error_update: i32,
            pub ally_team_id: i32,
            pub set_pos_error_bit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPosErrorParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPositionQuery {
            pub unit_id: i32,
            pub pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitPositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitRadiusAndHeightQuery {
            pub unit_id: i32,
            pub radius: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitRadiusAndHeightResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitResourcingQuery {
            pub unit_id: i32,
            pub type_: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitResourcingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitRotationQuery {
            pub unit_id: i32,
            pub rotation: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitRotationResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitSeismicSignatureQuery {
            pub unit_id: i32,
            pub seismic_signature: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitSeismicSignatureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitSelectionVolumeDataQuery {
            pub unit_id: i32,
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub test_type: i32,
            pub primary_axis: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitSelectionVolumeDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitSensorRadiusQuery {
            pub unit_id: i32,
            pub sensor_type: String,
            pub radius: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitSensorRadiusResult {
            pub new_radius: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitShieldRechargeDelayQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub recharge_delay: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitShieldRechargeDelayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitShieldStateQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub enabled: bool,
            pub power: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitShieldStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitSonarStealthQuery {
            pub unit_id: i32,
            pub sonar_stealth: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitSonarStealthResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitStealthQuery {
            pub unit_id: i32,
            pub stealth: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitStealthResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitStockpileQuery {
            pub unit_id: i32,
            pub stockpile: i32,
            pub build_percent: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitStockpileResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitStorageQuery {
            pub unit_id: i32,
            pub resource: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitStorageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitTargetOptions {
            pub manual_fire: bool,
            pub user_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitTargetQuery {
            pub unit_id: i32,
            pub target: UnitTargetRef,
            pub options: SetUnitTargetOptions,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitTargetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitTooltipQuery {
            pub unit_id: i32,
            pub tooltip: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitTooltipResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitUseAirLosQuery {
            pub unit_id: i32,
            pub use_air_los: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitUseAirLosResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitUseWeaponsOptions {
            pub force_use_weapons: bool,
            pub allow_use_weapons: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitUseWeaponsQuery {
            pub unit_id: i32,
            pub options: SetUnitUseWeaponsOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitUseWeaponsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitVelocityQuery {
            pub unit_id: i32,
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitVelocityResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitWeaponDamagesQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub damage_key: String,
            pub damage_value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitWeaponDamagesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitWeaponStateQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub key: String,
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitWeaponStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetWindQuery {
            pub min_wind: f32,
            pub max_wind: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetWindResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ShareTeamResourceQuery {
            pub team_id: i32,
            pub target_team_id: i32,
            pub resource_type: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ShareTeamResourceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SolveNURBSCurveQuery {
            pub degree: i32,
            pub points: Vec<Float4>,
            pub knots: Vec<f32>,
            pub segments: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SolveNURBSCurveResult {
            pub points: Vec<Float3>,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SpawnCEGQuery {
            pub ceg: DefRef,
            pub pos: Float3,
            pub dir: Float3,
            pub radius: f32,
            pub damage: f32,
            pub dmg_mod: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SpawnCEGResult {
            pub success: bool,
            pub ceg_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SpawnExplosionQuery {
            pub pos: Float3,
            pub dir: Float3,
            pub explosion_params: NativeExplosionParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SpawnExplosionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SpawnProjectileQuery {
            pub weapon_def_id: i32,
            pub projectile_params: NativeProjectileParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SpawnProjectileResult {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SpawnSFXQuery {
            pub unit_id: i32,
            pub sfx_id: i32,
            pub pos: Float3,
            pub dir: Float3,
            pub radius: f32,
            pub damage: f32,
            pub absolute: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SpawnSFXResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SphereQuery {
            pub center: Float3,
            pub radius: f32,
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
        pub struct SyncedCtrlApi {
            pub team: u32,
            pub unit: u32,
            pub feature: u32,
            pub terrain: u32,
            pub projectile: u32,
            pub effects: u32,
            pub game_config: u32,
            pub cob_script: u32,
            pub unit_script: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamControlApi {
            pub set_ally: u32,
            pub set_ally_team_start_box: u32,
            pub kill_team: u32,
            pub assign_player_to_team: u32,
            pub game_over: u32,
            pub set_global_los: u32,
            pub add_team_resource: u32,
            pub add_team_resource_excess_stats: u32,
            pub use_team_resource: u32,
            pub set_team_resource: u32,
            pub set_team_share_level: u32,
            pub share_team_resource: u32,
            pub set_team_start_position: u32,
            pub set_player_ready_state: u32,
            pub transfer_team_max_units: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamUnitsByDef {
            pub unit_def_id: i32,
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TerrainControlApi {
            pub add_height_map: u32,
            pub set_height_map: u32,
            pub revert_height_map: u32,
            pub add_smooth_mesh: u32,
            pub set_smooth_mesh: u32,
            pub revert_smooth_mesh: u32,
            pub set_map_square_terrain_type: u32,
            pub set_terrain_type_data: u32,
            pub set_tidal: u32,
            pub set_wind: u32,
            pub add_grass: u32,
            pub remove_grass: u32,
            pub adjust_height_map: u32,
            pub level_height_map: u32,
            pub add_original_height_map: u32,
            pub set_original_height_map: u32,
            pub revert_original_height_map: u32,
            pub adjust_original_height_map: u32,
            pub level_original_height_map: u32,
            pub adjust_smooth_mesh: u32,
            pub level_smooth_mesh: u32,
            pub rebuild_smooth_mesh: u32,
            pub set_height_map_func: u32,
            pub set_original_height_map_func: u32,
            pub set_smooth_mesh_func: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TransferFeatureQuery {
            pub feature_id: i32,
            pub new_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TransferFeatureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TransferTeamMaxUnitsQuery {
            pub from_team_id: i32,
            pub to_team_id: i32,
            pub amount: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TransferTeamMaxUnitsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TransferUnitQuery {
            pub unit_id: i32,
            pub new_team_id: i32,
            pub given: bool,
            pub adjust_unit_limit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TransferUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UInt32Result {
            pub value: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitAttachQuery {
            pub transporter_id: i32,
            pub transportee_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitAttachResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitControlApi {
            pub create_unit: u32,
            pub destroy_unit: u32,
            pub transfer_unit: u32,
            pub give_order_to_unit: u32,
            pub give_order_to_unit_array: u32,
            pub give_order_array_to_unit: u32,
            pub give_order_array_to_unit_array: u32,
            pub unit_finish_command: u32,
            pub set_unit_health: u32,
            pub set_unit_max_health: u32,
            pub set_unit_experience: u32,
            pub add_unit_experience: u32,
            pub set_unit_neutral: u32,
            pub set_unit_resourcing: u32,
            pub set_unit_metal_extraction: u32,
            pub set_unit_position: u32,
            pub set_unit_velocity: u32,
            pub set_unit_rotation: u32,
            pub set_unit_physics: u32,
            pub add_unit_damage: u32,
            pub add_unit_impulse: u32,
            pub set_unit_cloak: u32,
            pub set_unit_stealth: u32,
            pub set_unit_sonar_stealth: u32,
            pub set_unit_seismic_signature: u32,
            pub set_unit_armored: u32,
            pub set_unit_blocking: u32,
            pub set_unit_mass: u32,
            pub set_unit_leaves_ghost: u32,
            pub set_unit_always_visible: u32,
            pub set_unit_use_air_los: u32,
            pub get_unit_leaves_ghost: u32,
            pub get_unit_physical_state: u32,
            pub get_unit_feature_separation: u32,
            pub edit_unit_cmd_desc: u32,
            pub insert_unit_cmd_desc: u32,
            pub remove_unit_cmd_desc: u32,
            pub set_unit_costs: u32,
            pub set_unit_build_speed: u32,
            pub set_unit_collision_volume_data: u32,
            pub set_unit_selection_volume_data: u32,
            pub set_unit_piece_collision_volume_data: u32,
            pub set_unit_target: u32,
            pub set_unit_shield_state: u32,
            pub set_unit_shield_recharge_delay: u32,
            pub set_unit_flanking: u32,
            pub set_unit_mid_and_aim_pos: u32,
            pub set_unit_radius_and_height: u32,
            pub set_unit_move_goal: u32,
            pub set_unit_land_goal: u32,
            pub clear_unit_goal: u32,
            pub set_unit_stockpile: u32,
            pub set_unit_direction: u32,
            pub unit_attach: u32,
            pub unit_detach: u32,
            pub unit_detach_from_air: u32,
            pub set_unit_loading_transport: u32,
            pub set_unit_crashing: u32,
            pub set_unit_weapon_state: u32,
            pub unit_weapon_fire: u32,
            pub unit_weapon_hold_fire: u32,
            pub set_unit_use_weapons: u32,
            pub set_unit_max_range: u32,
            pub set_unit_physical_state_bit: u32,
            pub set_unit_pos_error_params: u32,
            pub set_unit_weapon_damages: u32,
            pub force_unit_collision_update: u32,
            pub set_unit_heading: u32,
            pub set_unit_heading_and_up_dir: u32,
            pub add_object_decal: u32,
            pub remove_object_decal: u32,
            pub set_unit_buildee_radius: u32,
            pub set_unit_sensor_radius: u32,
            pub set_unit_harvest_storage: u32,
            pub set_unit_build_params: u32,
            pub set_unit_los_mask: u32,
            pub set_unit_los_state: u32,
            pub set_unit_storage: u32,
            pub set_unit_tooltip: u32,
            pub set_factory_bugger_off: u32,
            pub bugger_off: u32,
            pub add_unit_seismic_ping: u32,
            pub add_unit_resource: u32,
            pub use_unit_resource: u32,
            pub set_unit_piece_visible: u32,
            pub set_unit_piece_parent: u32,
            pub set_unit_piece_matrix: u32,
            pub set_unit_nano_pieces: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCostOverrides {
            pub build_time: f32,
            pub metal_cost: f32,
            pub energy_cost: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDefCount {
            pub unit_def_id: i32,
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDetachFromAirQuery {
            pub transportee_id: i32,
            pub pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDetachFromAirResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDetachQuery {
            pub transportee_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDetachResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct UnitFilterParams {
            pub filter: UnitFilter,
            pub team_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFinishCommandQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFinishCommandResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitHealthValue {
            pub health: f32,
            pub capture: f32,
            pub paralyze: f32,
            pub build: f32,
            pub use_amounts: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitScriptApi {
            pub call_unit_script: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWeaponFireQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWeaponFireResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWeaponHoldFireQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWeaponHoldFireResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UseTeamResourceQuery {
            pub team_id: i32,
            pub resource_type: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UseTeamResourceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UseUnitResourceQuery {
            pub unit_id: i32,
            pub resource_type: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UseUnitResourceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidUnitIDQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidUnitIDResult {
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

    }


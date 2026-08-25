    pub mod callins {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct ActivateMenuQuery {
            pub message: String,
            pub message_length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ActivateMenuResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ActiveCommandChangedQuery {
            pub cmd_id: i32,
            pub cmd_type: i32,
            pub name: String,
            pub action: String,
            pub tooltip: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ActiveCommandChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddConsoleLineQuery {
            pub message: String,
            pub section: String,
            pub level: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowBuilderHoldFireQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub action: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowDirectUnitControlQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowFeatureBuildStepQuery {
            pub builder_id: i32,
            pub builder_team: i32,
            pub feature_id: i32,
            pub feature_def_id: i32,
            pub part: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowFeatureCreationQuery {
            pub feature_def_id: i32,
            pub team_id: i32,
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowResourceLevelQuery {
            pub team_id: i32,
            pub type_: String,
            pub level: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowResourceTransferQuery {
            pub old_team: i32,
            pub new_team: i32,
            pub type_: String,
            pub amount: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowStartPositionQuery {
            pub player_id: i32,
            pub team_id: i32,
            pub ready_state: u8,
            pub clamped_pos: Float3,
            pub raw_pick_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitBuildStepQuery {
            pub builder_id: i32,
            pub builder_team: i32,
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub part: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitCloakQuery {
            pub unit_id: i32,
            pub has_enemy: bool,
            pub enemy_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitCreationQuery {
            pub unit_def_id: i32,
            pub builder_id: i32,
            pub builder_team: i32,
            pub has_build_info: bool,
            pub build_pos: Float3,
            pub build_facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitCreationResult {
            pub allow: bool,
            pub drop_order: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitDecloakQuery {
            pub unit_id: i32,
            pub has_object: bool,
            pub object_id: i32,
            pub has_weapon: bool,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitKamikazeQuery {
            pub unit_id: i32,
            pub target_id: i32,
            pub allowed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitTransferQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub old_team: i32,
            pub new_team: i32,
            pub capture: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitTransportPositionQuery {
            pub units: AllowUnitTransportQuery,
            pub position: Float3,
            pub allowed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowUnitTransportQuery {
            pub transporter_id: i32,
            pub transporter_def_id: i32,
            pub transporter_team: i32,
            pub transportee_id: i32,
            pub transportee_def_id: i32,
            pub transportee_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowWeaponInterceptTargetQuery {
            pub interceptor_unit_id: i32,
            pub interceptor_weapon_id: i32,
            pub interceptor_target_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowWeaponTargetCheckQuery {
            pub attacker_id: i32,
            pub attacker_weapon_num: i32,
            pub attacker_weapon_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowWeaponTargetQuery {
            pub attacker_id: i32,
            pub target_id: i32,
            pub attacker_weapon_num: i32,
            pub attacker_weapon_def_id: i32,
            pub has_target_priority: bool,
            pub target_priority: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AllowWeaponTargetResult {
            pub allowed: bool,
            pub target_priority: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ArchiveCallinQuery {
            pub archive: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ArchiveCallinResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BoolCallinResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CollectGarbageQuery {
            pub forced: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CollectGarbageResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CommandFallbackQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub command: NativeCallinCommand,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CommandNotifyQuery {
            pub command: NativeCallinCommand,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DamageCallinResult {
            pub new_damage: f32,
            pub impulse_mult: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DefaultCommandQuery {
            pub unit_id: i32,
            pub feature_id: i32,
            pub current_command: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DefaultCommandResult {
            pub value: bool,
            pub command: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadFailedQuery {
            pub download_id: i32,
            pub error_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadFailedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadFinishedQuery {
            pub download_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadFinishedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadProgressQuery {
            pub download_id: i32,
            pub downloaded: i64,
            pub total: i64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadProgressResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadQueuedQuery {
            pub download_id: i32,
            pub archive_name: String,
            pub archive_type: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadQueuedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadStartedQuery {
            pub download_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadStartedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawAlphaObjectsLuaQuery {
            pub draw_reflection: bool,
            pub draw_refraction: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawAlphaObjectsLuaResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawBuildSquareQuery {
            pub unit_def_id: i32,
            pub x: i32,
            pub z: i32,
            pub facing: i32,
            pub statuses: Vec<u8>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawBuildSquareResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawFeatureQuery {
            pub feature_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawMaterialQuery {
            pub uuid: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawObjectsLuaQuery {
            pub deferred_pass: bool,
            pub draw_reflection: bool,
            pub draw_refraction: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawObjectsLuaResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawProjectileQuery {
            pub projectile_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawScreenQuery {
            pub view_size_x: i32,
            pub view_size_y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawScreenResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawShieldQuery {
            pub unit_id: i32,
            pub weapon_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawUnitQuery {
            pub unit_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawWorldPreParticlesQuery {
            pub draw_above_water: bool,
            pub draw_below_water: bool,
            pub draw_reflection: bool,
            pub draw_refraction: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DrawWorldPreParticlesResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ExplosionQuery {
            pub weapon_def_id: i32,
            pub pos: Float3,
            pub owner_id: i32,
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureCreatedQuery {
            pub feature_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureCreatedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureDamagedQuery {
            pub feature_id: i32,
            pub feature_def_id: i32,
            pub feature_team: i32,
            pub damage: f32,
            pub weapon_def_id: i32,
            pub projectile_id: i32,
            pub attacker_id: i32,
            pub attacker_def_id: i32,
            pub attacker_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureDamagedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureDestroyedQuery {
            pub feature_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureDestroyedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureMovedQuery {
            pub feature_id: i32,
            pub old_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FeatureMovedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Float3CallinQuery {
            pub value: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Float3CallinResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameFramePostQuery {
            pub game_frame: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameFramePostResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameFrameQuery {
            pub game_frame: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameFrameResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameIDQuery {
            pub game_id: Vec<u8>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameIDResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameOverEventQuery {
            pub winning_ally_teams: Vec<u8>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameOverEventResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GamePausedQuery {
            pub player_id: i32,
            pub paused: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GamePausedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GamePreloadQuery {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GamePreloadResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameProgressQuery {
            pub game_frame: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameProgressResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameSetupPlayerState {
            pub player_id: i32,
            pub state: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameSetupQuery {
            pub state: String,
            pub ready: bool,
            pub player_states: Vec<GameSetupPlayerState>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameSetupResult {
            pub handled: bool,
            pub ready: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameStartQuery {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GameStartResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GroupChangedQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HandleLuaCallQuery {
            pub message: Vec<u8>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HandleLuaCallResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HandleLuaMsgQuery {
            pub player_id: i32,
            pub script: i32,
            pub mode: i32,
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HandleLuaMsgResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct InitializeNativeModuleQuery {
            pub host_version_major: u32,
            pub host_version_minor: u32,
            pub host_version_patch: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct InitializeNativeModuleResult {
            pub module_data: u32,
            pub module_version_major: u32,
            pub module_version_minor: u32,
            pub module_version_patch: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IntCallinResult {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct KeyAction {
            pub command: String,
            pub extra: String,
            pub bound_with: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct KeyPressQuery {
            pub key_code: i32,
            pub alt: bool,
            pub ctrl: bool,
            pub meta: bool,
            pub shift: bool,
            pub is_repeat: bool,
            pub label: String,
            pub utf32_char: i32,
            pub scan_code: i32,
            pub action_list: Vec<KeyAction>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct KeyReleaseQuery {
            pub key_code: i32,
            pub alt: bool,
            pub ctrl: bool,
            pub meta: bool,
            pub shift: bool,
            pub label: String,
            pub utf32_char: i32,
            pub scan_code: i32,
            pub action_list: Vec<KeyAction>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LastMessagePositionQuery {
            pub pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LastMessagePositionResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadProgressQuery {
            pub message: String,
            pub message_length: u32,
            pub replace_last_line: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LoadProgressResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MapDrawCmdQuery {
            pub player_id: i32,
            pub type_: i32,
            pub has_pos0: bool,
            pub pos0: Float3,
            pub has_pos1: bool,
            pub pos1: Float3,
            pub has_label: bool,
            pub label: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MiniMapDrawQuery {
            pub size_x: i32,
            pub size_y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MiniMapGeometryChangedQuery {
            pub new_pos_x: i32,
            pub new_pos_y: i32,
            pub new_dim_x: i32,
            pub new_dim_y: i32,
            pub old_pos_x: i32,
            pub old_pos_y: i32,
            pub old_dim_x: i32,
            pub old_dim_y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MiniMapRotationChangedQuery {
            pub new_rot: f32,
            pub old_rot: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MiniMapStateChangedQuery {
            pub is_minimized: bool,
            pub is_maximized: bool,
            pub is_slaved: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MouseMoveQuery {
            pub x: i32,
            pub y: i32,
            pub dx: i32,
            pub dy: i32,
            pub button: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MousePressQuery {
            pub x: i32,
            pub y: i32,
            pub button: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MouseReleaseQuery {
            pub x: i32,
            pub y: i32,
            pub button: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MouseReleaseResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MouseWheelQuery {
            pub up: bool,
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MoveCtrlNotifyQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub data: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeCallinCommand {
            pub id: i32,
            pub time_out: i32,
            pub page_index: u32,
            pub num_params: u32,
            pub tag: u32,
            pub options: u8,
            pub params: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerAddedQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerAddedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerChangedQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerChangedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerRemovedQuery {
            pub player_id: i32,
            pub reason: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerRemovedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PongQuery {
            pub ping_tag: u8,
            pub packet_send_time_millis: i64,
            pub packet_recv_time_millis: i64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PongResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ProjectileEventQuery {
            pub projectile_id: i32,
            pub owner_id: i32,
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ProjectileEventResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RectChangedQuery {
            pub x1: i32,
            pub z1: i32,
            pub x2: i32,
            pub z2: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RectChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RecvFromSyncedQuery {
            pub message: Vec<u8>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RecvFromSyncedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RenderUnitDestroyedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RenderUnitDestroyedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourceExcessEntry {
            pub team_id: i32,
            pub resources: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourceExcessQuery {
            pub entries: Vec<ResourceExcessEntry>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ScreenPositionQuery {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ShieldPreDamagedQuery {
            pub projectile_id: i32,
            pub projectile_owner_id: i32,
            pub shield_weapon_num: i32,
            pub shield_carrier_id: i32,
            pub bounce_projectile: bool,
            pub beam_emitter_weapon_num: i32,
            pub beam_emitter_unit_id: i32,
            pub start_pos: Float3,
            pub hit_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ShutdownQuery {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ShutdownResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SimpleCallinQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SimpleCallinResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct StockpileChangedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub weapon_num: i32,
            pub old_count: i32,
            pub new_count: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct StockpileChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringCallinResult {
            pub value: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SunChangedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SunChangedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TeamChangedQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TeamChangedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TeamDiedQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TeamDiedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TerraformCompleteQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub build_unit_id: i32,
            pub build_unit_def_id: i32,
            pub build_unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TextEditingQuery {
            pub utf8: String,
            pub start: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TextInputQuery {
            pub utf8: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCloakEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCloakEventResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCmdDoneQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub command: NativeCallinCommand,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCmdDoneResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCommandQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub command: NativeCallinCommand,
            pub player_num: i32,
            pub from_synced: bool,
            pub from_lua: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCommandResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitConstructionDecayedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub time_since_last_build: f32,
            pub iteration_period: f32,
            pub part: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitConstructionDecayedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCreatedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub builder_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitCreatedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDamagedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub damage: f32,
            pub paralyzer: bool,
            pub weapon_def_id: i32,
            pub projectile_id: i32,
            pub attacker_id: i32,
            pub attacker_def_id: i32,
            pub attacker_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDamagedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDestroyedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub attacker_id: i32,
            pub attacker_def_id: i32,
            pub attacker_team: i32,
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDestroyedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitExperienceQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub experience: f32,
            pub old_experience: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitExperienceResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFeatureCollisionQuery {
            pub collider_id: i32,
            pub collidee_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFinishedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFinishedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFromFactoryQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub factory_id: i32,
            pub factory_def_id: i32,
            pub user_orders: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitFromFactoryResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitGivenQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub old_team: i32,
            pub new_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitGivenResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitHarvestStorageFullQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitHarvestStorageFullResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitIdleQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitIdleResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitLoadedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub transport_id: i32,
            pub transport_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitLoadedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitLosEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub ally_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitLosEventResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitMoveEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitMoveEventResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitMovementClassEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitMovementClassEventResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitReverseBuiltQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitReverseBuiltResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitSeismicPingQuery {
            pub pos: Float3,
            pub strength: f32,
            pub ally_team: i32,
            pub unit_id: i32,
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitSeismicPingResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitStunnedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub stunned: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitStunnedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitTakenQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub old_team: i32,
            pub new_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitTakenResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitUnitCollisionQuery {
            pub collider_id: i32,
            pub collidee_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitUnloadedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub transport_id: i32,
            pub transport_team: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitUnloadedResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UpdateQuery {
            pub delta_seconds: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UpdateResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ViewResizeQuery {
            pub screen_size_x: i32,
            pub screen_size_y: i32,
            pub screen_pos_x: i32,
            pub screen_pos_y: i32,
            pub window_size_x: i32,
            pub window_size_y: i32,
            pub window_pos_x: i32,
            pub window_pos_y: i32,
            pub window_border_top: i32,
            pub window_border_left: i32,
            pub window_border_bottom: i32,
            pub window_border_right: i32,
            pub view_size_x: i32,
            pub view_size_y: i32,
            pub view_pos_x: i32,
            pub view_pos_y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ViewResizeResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct WorldTooltipQuery {
            pub kind: i32,
            pub unit_id: i32,
            pub feature_id: i32,
            pub ground_pos: Float3,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

    }


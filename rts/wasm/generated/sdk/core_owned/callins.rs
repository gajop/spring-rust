    pub mod callins {
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
        pub struct ActiveCommandChangedQuery {
            pub cmd_id: i32,
            pub cmd_type: i32,
            pub name: String,
            pub action: String,
            pub tooltip: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ActiveCommandChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddConsoleLineQuery {
            pub message: String,
            pub section: String,
            pub level: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowBuilderHoldFireQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub action: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowDirectUnitControlQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub player_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowFeatureBuildStepQuery {
            pub builder_id: i32,
            pub builder_team: i32,
            pub feature_id: i32,
            pub feature_def_id: i32,
            pub part: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowStartPositionQuery {
            pub player_id: i32,
            pub team_id: i32,
            pub ready_state: u8,
            pub clamped_pos: Float3,
            pub raw_pick_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitBuildStepQuery {
            pub builder_id: i32,
            pub builder_team: i32,
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub part: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitCloakQuery {
            pub unit_id: i32,
            pub has_enemy: bool,
            pub enemy_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitCreationQuery {
            pub unit_def_id: i32,
            pub builder_id: i32,
            pub builder_team: i32,
            pub has_build_info: bool,
            pub build_pos: Float3,
            pub build_facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitCreationResult {
            pub allow: bool,
            pub drop_order: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitDecloakQuery {
            pub unit_id: i32,
            pub has_object: bool,
            pub object_id: i32,
            pub has_weapon: bool,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitKamikazeQuery {
            pub unit_id: i32,
            pub target_id: i32,
            pub allowed: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitTransferQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub old_team: i32,
            pub new_team: i32,
            pub capture: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitTransportPositionQuery {
            pub units: AllowUnitTransportQuery,
            pub position: Float3,
            pub allowed: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowUnitTransportQuery {
            pub transporter_id: i32,
            pub transporter_def_id: i32,
            pub transporter_team: i32,
            pub transportee_id: i32,
            pub transportee_def_id: i32,
            pub transportee_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowWeaponInterceptTargetQuery {
            pub interceptor_unit_id: i32,
            pub interceptor_weapon_id: i32,
            pub interceptor_target_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowWeaponTargetCheckQuery {
            pub attacker_id: i32,
            pub attacker_weapon_num: i32,
            pub attacker_weapon_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowWeaponTargetQuery {
            pub attacker_id: i32,
            pub target_id: i32,
            pub attacker_weapon_num: i32,
            pub attacker_weapon_def_id: i32,
            pub has_target_priority: bool,
            pub target_priority: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllowWeaponTargetResult {
            pub allowed: bool,
            pub target_priority: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ArchiveCallinQuery {
            pub archive: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ArchiveCallinResult {
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
        pub struct BoolCallinResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectGarbageQuery {
            pub forced: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectGarbageResult {
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct DamageCallinResult {
            pub new_damage: f32,
            pub impulse_mult: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefaultCommandQuery {
            pub unit_id: i32,
            pub feature_id: i32,
            pub current_command: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefaultCommandResult {
            pub value: bool,
            pub command: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadFailedQuery {
            pub download_id: i32,
            pub error_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadFailedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadFinishedQuery {
            pub download_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadFinishedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadProgressQuery {
            pub download_id: i32,
            pub downloaded: i64,
            pub total: i64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadProgressResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadQueuedQuery {
            pub download_id: i32,
            pub archive_name: String,
            pub archive_type: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadQueuedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadStartedQuery {
            pub download_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadStartedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawAlphaObjectsLuaQuery {
            pub draw_reflection: bool,
            pub draw_refraction: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawBuildSquareResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawFeatureQuery {
            pub feature_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawMaterialQuery {
            pub uuid: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawObjectsLuaQuery {
            pub deferred_pass: bool,
            pub draw_reflection: bool,
            pub draw_refraction: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawObjectsLuaResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawProjectileQuery {
            pub projectile_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawScreenQuery {
            pub view_size_x: i32,
            pub view_size_y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawScreenResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawShieldQuery {
            pub unit_id: i32,
            pub weapon_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawUnitQuery {
            pub unit_id: i32,
            pub draw_mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawWorldPreParticlesQuery {
            pub draw_above_water: bool,
            pub draw_below_water: bool,
            pub draw_reflection: bool,
            pub draw_refraction: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawWorldPreParticlesResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ExplosionQuery {
            pub weapon_def_id: i32,
            pub pos: Float3,
            pub owner_id: i32,
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureCreatedQuery {
            pub feature_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureCreatedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureDamagedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureDestroyedQuery {
            pub feature_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureDestroyedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureMovedQuery {
            pub feature_id: i32,
            pub old_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureMovedResult {
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
        pub struct Float3CallinQuery {
            pub value: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3CallinResult {
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
        pub struct GameFramePostQuery {
            pub game_frame: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameFramePostResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameFrameQuery {
            pub game_frame: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameFrameResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameIDQuery {
            pub game_id: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameIDResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameOverEventQuery {
            pub winning_ally_teams: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameOverEventResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GamePausedQuery {
            pub player_id: i32,
            pub paused: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GamePausedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GamePreloadQuery {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GamePreloadResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameProgressQuery {
            pub game_frame: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameSetupResult {
            pub handled: bool,
            pub ready: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameStartQuery {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameStartResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GroupChangedQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HandleLuaCallQuery {
            pub message: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HandleLuaCallResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HandleLuaMsgQuery {
            pub player_id: i32,
            pub script: i32,
            pub mode: i32,
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HandleLuaMsgResult {
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct LastMessagePositionQuery {
            pub pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LastMessagePositionResult {
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct MapRenderingParams {
            pub splat_tex_scales: Option<Vec<f32>>,
            pub splat_tex_mults: Option<Vec<f32>>,
            pub void_water: Option<bool>,
            pub void_ground: Option<bool>,
            pub splat_detail_normal_diffuse_alpha: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MiniMapDrawQuery {
            pub size_x: i32,
            pub size_y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct MiniMapRotationChangedQuery {
            pub new_rot: f32,
            pub old_rot: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MiniMapStateChangedQuery {
            pub is_minimized: bool,
            pub is_maximized: bool,
            pub is_slaved: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MouseMoveQuery {
            pub x: i32,
            pub y: i32,
            pub dx: i32,
            pub dy: i32,
            pub button: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MousePressQuery {
            pub x: i32,
            pub y: i32,
            pub button: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MouseReleaseQuery {
            pub x: i32,
            pub y: i32,
            pub button: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MouseReleaseResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MouseWheelQuery {
            pub up: bool,
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
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
        pub struct PlayerAddedQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlayerAddedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlayerChangedQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlayerChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlayerRemovedQuery {
            pub player_id: i32,
            pub reason: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlayerRemovedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PongQuery {
            pub ping_tag: u8,
            pub packet_send_time_millis: i64,
            pub packet_recv_time_millis: i64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PongResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileEventQuery {
            pub projectile_id: i32,
            pub owner_id: i32,
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileEventResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RectChangedQuery {
            pub x1: i32,
            pub z1: i32,
            pub x2: i32,
            pub z2: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RectChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RecvFromSyncedQuery {
            pub message: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RecvFromSyncedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RenderUnitDestroyedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
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
        pub struct ScreenPositionQuery {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct ShutdownQuery {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ShutdownResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SimpleCallinQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SimpleCallinResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StockpileChangedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub weapon_num: i32,
            pub old_count: i32,
            pub new_count: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StockpileChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringCallinResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SunChangedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SunChangedResult {
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
        pub struct TeamChangedQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamChangedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamDiedQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamDiedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
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
        pub struct UnitCloakEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCloakEventResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCmdDoneQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub command: NativeCallinCommand,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCommandResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitConstructionDecayedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub time_since_last_build: f32,
            pub iteration_period: f32,
            pub part: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitConstructionDecayedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCostOverrides {
            pub build_time: f32,
            pub metal_cost: f32,
            pub energy_cost: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCreatedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub builder_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCreatedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDamagedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDestroyedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub attacker_id: i32,
            pub attacker_def_id: i32,
            pub attacker_team: i32,
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDestroyedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitExperienceQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub experience: f32,
            pub old_experience: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitExperienceResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitFeatureCollisionQuery {
            pub collider_id: i32,
            pub collidee_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitFinishedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitFinishedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitFromFactoryQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub factory_id: i32,
            pub factory_def_id: i32,
            pub user_orders: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitFromFactoryResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitGivenQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub old_team: i32,
            pub new_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitGivenResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitHarvestStorageFullQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitHarvestStorageFullResult {
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
        pub struct UnitIdleQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitIdleResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitLoadedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub transport_id: i32,
            pub transport_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitLoadedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitLosEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub ally_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitLosEventResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitMoveEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitMoveEventResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitMovementClassEventQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitMovementClassEventResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitReverseBuiltQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitReverseBuiltResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitSeismicPingQuery {
            pub pos: Float3,
            pub strength: f32,
            pub ally_team: i32,
            pub unit_id: i32,
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitSeismicPingResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitStunnedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub stunned: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitStunnedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTakenQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub old_team: i32,
            pub new_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTakenResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitUnitCollisionQuery {
            pub collider_id: i32,
            pub collidee_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitUnloadedQuery {
            pub unit_id: i32,
            pub unit_def_id: i32,
            pub unit_team: i32,
            pub transport_id: i32,
            pub transport_team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitUnloadedResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateQuery {
            pub delta_seconds: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateResult {
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct ViewResizeResult {
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct WorldTooltipQuery {
            pub kind: i32,
            pub unit_id: i32,
            pub feature_id: i32,
            pub ground_pos: Float3,
        }

    }


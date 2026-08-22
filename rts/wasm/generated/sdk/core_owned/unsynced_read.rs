    pub mod unsynced_read {
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
        pub struct BoolResult {
            pub value: bool,
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
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
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
        pub struct GetActiveCmdDescQuery {
            pub cmd_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCmdDescResult {
            pub cmd_desc: ActiveCommandDescription,
            pub has_command: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCmdDescsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCmdDescsResult {
            pub cmd_descs: Vec<ActiveCommandDescription>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetBoxSelectionByEngineQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetBoxSelectionByEngineResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetBuildFacingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetBuildFacingResult {
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetBuildSpacingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetBuildSpacingResult {
            pub spacing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraRotationQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraRotationResult {
            pub rot_x: f32,
            pub rot_y: f32,
            pub rot_z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraVectorsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraVectorsResult {
            pub forward: Float3,
            pub up: Float3,
            pub right: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetClipboardQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetClipboardResult {
            pub text: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCmdDescIndexQuery {
            pub cmd_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCmdDescIndexResult {
            pub index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCustomPaletteColorQuery {
            pub index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCustomPaletteColorResult {
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDrawSelectionInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDrawSelectionInfoResult {
            pub draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePaletteIndexQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePaletteIndexResult {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFrustumPlanesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFrustumPlanesResult {
            pub planes: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSecondsInterpolatedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSecondsInterpolatedResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLastMessagePositionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLastMessagePositionsResult {
            pub positions: Vec<Float3>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNanoProjectileParamsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNanoProjectileParamsResult {
            pub r: f32,
            pub v: f32,
            pub a: f32,
            pub rand_r: f32,
            pub rand_v: f32,
            pub rand_a: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileNameQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPrevFrameSyncChecksumQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPrevFrameSyncChecksumResult {
            pub checksum: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamDamageStatsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamDamageStatsResult {
            pub damage_dealt: f32,
            pub damage_received: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitAlwaysUpdateMatrixQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitAlwaysUpdateMatrixResult {
            pub always_update_matrix: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDrawFlagQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDrawFlagResult {
            pub draw_flag: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitEngineDrawMaskQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitEngineDrawMaskResult {
            pub engine_draw_mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitLuaDrawQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitLuaDrawResult {
            pub lua_draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoDrawQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoDrawResult {
            pub no_draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoGroupQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoGroupResult {
            pub no_group: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoMinimapQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoMinimapResult {
            pub no_minimap: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoSelectQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNoSelectResult {
            pub no_select: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPaletteIndexQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPaletteIndexResult {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitSelectionVolumeDataQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitSelectionVolumeDataResult {
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub use_cont_hit_test: bool,
            pub primary_axis: i32,
            pub ignore_hits: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitTransformMatrixQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitTransformMatrixResult {
            pub matrix: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitViewPositionQuery {
            pub unit_id: i32,
            pub use_mid_pos: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitViewPositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleFeaturesOptions {
            pub include_icons: bool,
            pub include_geos: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleFeaturesQuery {
            pub ally_team_id: i32,
            pub radius: f32,
            pub options: GetVisibleFeaturesOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleFeaturesResult {
            pub feature_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleProjectilesOptions {
            pub include_synced_projectiles: bool,
            pub include_weapon_projectiles: bool,
            pub include_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleProjectilesQuery {
            pub ally_team_id: i32,
            pub options: GetVisibleProjectilesOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVisibleProjectilesResult {
            pub projectile_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
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
        pub struct IsUnitAlliedQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitAlliedResult {
            pub allied: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitIconQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitIconResult {
            pub is_icon: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInViewQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitInViewResult {
            pub in_view: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitSelectedQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitSelectedResult {
            pub selected: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitVisibleQuery {
            pub unit_id: i32,
            pub radius: f32,
            pub check_icon: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsUnitVisibleResult {
            pub visible: bool,
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
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
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
        pub struct UInt32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Result {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCostOverrides {
            pub build_time: f32,
            pub metal_cost: f32,
            pub energy_cost: f32,
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
        pub struct UnitRenderingApi {
            pub get_unit_no_draw: u32,
            pub get_unit_lua_draw: u32,
            pub get_unit_engine_draw_mask: u32,
            pub get_unit_always_update_matrix: u32,
            pub get_unit_draw_flag: u32,
            pub get_unit_no_select: u32,
            pub get_unit_no_minimap: u32,
            pub get_unit_no_group: u32,
            pub get_unit_view_position: u32,
            pub get_unit_transform_matrix: u32,
            pub get_unit_selection_volume_data: u32,
            pub get_unit_icon_data: u32,
            pub get_unit_icon: u32,
            pub get_camera_rotation: u32,
            pub get_camera_vectors: u32,
            pub get_frustum_planes: u32,
            pub get_visible_units: u32,
            pub get_visible_features: u32,
            pub get_visible_projectiles: u32,
            pub get_units_in_screen_rectangle: u32,
            pub get_features_in_screen_rectangle: u32,
            pub is_unit_visible: u32,
            pub is_unit_in_view: u32,
            pub is_unit_icon: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
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

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_clipboard {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-clipboard"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_piece_projectile_name {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-piece-projectile-name"]
                pub fn call(pprojectile_id: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_prev_frame_sync_checksum {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-prev-frame-sync-checksum"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCmdDescValue {
            pub cmd_desc: ActiveCommandDescription,
            pub has_command: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCustomPaletteColorValue {
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePaletteIndexValue {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNanoProjectileParamsValue {
            pub r: f32,
            pub v: f32,
            pub a: f32,
            pub rand_r: f32,
            pub rand_v: f32,
            pub rand_a: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamDamageStatsValue {
            pub damage_dealt: f32,
            pub damage_received: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPaletteIndexValue {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SolveNURBSCurveValue {
            pub points: Vec<Float3>,
            pub success: bool,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_active_cmd_desc {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-active-cmd-desc"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-read.get-active-cmd-desc."]
        #[inline]
        pub unsafe fn get_active_cmd_desc(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_active_cmd_desc::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_active_cmd_descs {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-active-cmd-descs"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-read.get-active-cmd-descs."]
        #[inline]
        pub unsafe fn get_active_cmd_descs(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_active_cmd_descs::call(p0, p1) }
        }

        #[inline]
        pub fn get_box_selection_by_engine(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_read::get_box_selection_by_engine(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_build_facing(unused: u8) -> Result<i32> {
            let value = crate::generated::unsynced_read::get_build_facing(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_build_spacing(unused: u8) -> Result<i32> {
            let value = crate::generated::unsynced_read::get_build_spacing(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_clipboard(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_clipboard::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_cmd_desc_index(cmd_id: i32) -> Result<i32> {
            let value = crate::generated::unsynced_read::get_cmd_desc_index(cmd_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_custom_palette_color(index: i32) -> Result<GetCustomPaletteColorValue> {
            let value = crate::generated::unsynced_read::get_custom_palette_color(index)?;
            Ok(GetCustomPaletteColorValue {
                r: value.0,
                g: value.1,
                b: value.2,
                success: value.3
            })
        }

        #[inline]
        pub fn get_draw_selection_info(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_read::get_draw_selection_info(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_palette_index(feature_id: i32) -> Result<GetFeaturePaletteIndexValue> {
            let value = crate::generated::unsynced_read::get_feature_palette_index(feature_id)?;
            Ok(GetFeaturePaletteIndexValue {
                custom_index: value.0,
                using_custom_color: value.1
            })
        }

        #[inline]
        pub fn get_game_seconds_interpolated(unused: u8) -> Result<f32> {
            let value = crate::generated::unsynced_read::get_game_seconds_interpolated(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_last_message_positions {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-last-message-positions"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-read.get-last-message-positions."]
        #[inline]
        pub unsafe fn get_last_message_positions(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_last_message_positions::call(p0, p1) }
        }

        #[inline]
        pub fn get_nano_projectile_params(unused: u8) -> Result<GetNanoProjectileParamsValue> {
            let value = crate::generated::unsynced_read::get_nano_projectile_params(unused)?;
            Ok(GetNanoProjectileParamsValue {
                r: value.0,
                v: value.1,
                a: value.2,
                rand_r: value.3,
                rand_v: value.4,
                rand_a: value.5
            })
        }

        #[inline]
        pub fn get_piece_projectile_name(projectile_id: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_piece_projectile_name::call(projectile_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (projectile_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_prev_frame_sync_checksum(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_prev_frame_sync_checksum::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_team_damage_stats(team_id: i32) -> Result<GetTeamDamageStatsValue> {
            let value = crate::generated::unsynced_read::get_team_damage_stats(team_id)?;
            Ok(GetTeamDamageStatsValue {
                damage_dealt: value.0,
                damage_received: value.1,
                success: value.2
            })
        }

        #[inline]
        pub fn get_unit_palette_index(unit_id: i32) -> Result<GetUnitPaletteIndexValue> {
            let value = crate::generated::unsynced_read::get_unit_palette_index(unit_id)?;
            Ok(GetUnitPaletteIndexValue {
                custom_index: value.0,
                using_custom_color: value.1
            })
        }

        #[inline]
        pub fn is_unit_allied(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_read::is_unit_allied(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_selected(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_read::is_unit_selected(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_solve_nurbs_curve {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "solve-nurbs-curve"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-read.solve-nurbs-curve."]
        #[inline]
        pub unsafe fn solve_nurbs_curve(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_solve_nurbs_curve::call(p0, p1, p2, p3) }
        }

    }


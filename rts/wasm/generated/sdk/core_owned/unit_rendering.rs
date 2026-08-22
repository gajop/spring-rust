    pub mod unit_rendering {
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
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnsyncedReadApi {
            pub unit_rendering: u32,
            pub get_clipboard: u32,
            pub get_prev_frame_sync_checksum: u32,
            pub get_active_cmd_desc: u32,
            pub get_active_cmd_descs: u32,
            pub get_cmd_desc_index: u32,
            pub get_box_selection_by_engine: u32,
            pub get_build_facing: u32,
            pub get_build_spacing: u32,
            pub get_draw_selection_info: u32,
            pub get_nano_projectile_params: u32,
            pub get_piece_projectile_name: u32,
            pub get_team_damage_stats: u32,
            pub get_last_message_positions: u32,
            pub solve_nurbs_curve: u32,
            pub is_unit_selected: u32,
            pub is_unit_allied: u32,
            pub get_custom_palette_color: u32,
            pub get_unit_palette_index: u32,
            pub get_feature_palette_index: u32,
            pub get_game_seconds_interpolated: u32,
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
        mod __core_variable_output_get_features_in_screen_rectangle {
            #[link(wasm_import_module = "spring:unit-rendering")]
            extern "C" {
                #[link_name = "get-features-in-screen-rectangle"]
                pub fn call(pleft: f32, ptop: f32, pright: f32, pbottom: f32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_units_in_screen_rectangle {
            #[link(wasm_import_module = "spring:unit-rendering")]
            extern "C" {
                #[link_name = "get-units-in-screen-rectangle"]
                pub fn call(pleft: f32, ptop: f32, pright: f32, pbottom: f32, pallegiance: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_visible_units {
            #[link(wasm_import_module = "spring:unit-rendering")]
            extern "C" {
                #[link_name = "get-visible-units"]
                pub fn call(pteam_id: i32, pradius: f32, pinclude_icons: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraRotationValue {
            pub rot_x: f32,
            pub rot_y: f32,
            pub rot_z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraVectorsValue {
            pub forward: Float3,
            pub up: Float3,
            pub right: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIconValue {
            pub icon_name: String,
            pub atlas_tex_coords: Vec<f32>,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIconDataValue {
            pub icon_name: String,
            pub atlas_tex_coords: Vec<f32>,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitSelectionVolumeDataValue {
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub use_cont_hit_test: bool,
            pub primary_axis: i32,
            pub ignore_hits: bool,
        }

        #[inline]
        pub fn get_camera_rotation(unused: u8) -> Result<GetCameraRotationValue> {
            let value = crate::generated::unit_rendering::get_camera_rotation(unused)?;
            Ok(GetCameraRotationValue {
                rot_x: value.0,
                rot_y: value.1,
                rot_z: value.2
            })
        }

        #[inline]
        pub fn get_camera_vectors(unused: u8) -> Result<GetCameraVectorsValue> {
            let value = crate::generated::unit_rendering::get_camera_vectors(unused)?;
            Ok(GetCameraVectorsValue {
                forward: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                up: Float3 { x: value.1.x, y: value.1.y, z: value.1.z },
                right: Float3 { x: value.2.x, y: value.2.y, z: value.2.z }
            })
        }

        #[inline]
        pub fn get_features_in_screen_rectangle(left: f32, top: f32, right: f32, bottom: f32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_features_in_screen_rectangle::call(left as f32, top as f32, right as f32, bottom as f32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (left as f32, top as f32, right as f32, bottom as f32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_frustum_planes(unused: u8) -> Result<Vec<f32>> {
            let value = crate::generated::unit_rendering::get_frustum_planes(unused)?;
            Ok(value.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?)
        }

        #[inline]
        pub fn get_unit_always_update_matrix(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_always_update_matrix(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_draw_flag(unit_id: i32) -> Result<u8> {
            let value = crate::generated::unit_rendering::get_unit_draw_flag(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_engine_draw_mask(unit_id: i32) -> Result<u32> {
            let value = crate::generated::unit_rendering::get_unit_engine_draw_mask(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_icon {
            #[link(wasm_import_module = "spring:unit-rendering")]
            extern "C" {
                #[link_name = "get-unit-icon"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-unit-icon."]
        #[inline]
        pub unsafe fn get_unit_icon(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_unit_icon::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_icon_data {
            #[link(wasm_import_module = "spring:unit-rendering")]
            extern "C" {
                #[link_name = "get-unit-icon-data"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-unit-icon-data."]
        #[inline]
        pub unsafe fn get_unit_icon_data(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_icon_data::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_unit_lua_draw(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_lua_draw(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_draw(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_draw(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_group(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_group(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_minimap(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_minimap(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_select(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_select(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_selection_volume_data(unit_id: i32) -> Result<GetUnitSelectionVolumeDataValue> {
            let value = crate::generated::unit_rendering::get_unit_selection_volume_data(unit_id)?;
            Ok(GetUnitSelectionVolumeDataValue {
                scales: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                offsets: Float3 { x: value.1.x, y: value.1.y, z: value.1.z },
                volume_type: value.2,
                use_cont_hit_test: value.3,
                primary_axis: value.4,
                ignore_hits: value.5
            })
        }

        #[inline]
        pub fn get_unit_transform_matrix(unit_id: i32) -> Result<Vec<f32>> {
            let value = crate::generated::unit_rendering::get_unit_transform_matrix(unit_id)?;
            Ok(value.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?)
        }

        #[inline]
        pub fn get_unit_view_position(unit_id: i32, use_mid_pos: bool) -> Result<Float3> {
            let value = crate::generated::unit_rendering::get_unit_view_position(unit_id, use_mid_pos)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_units_in_screen_rectangle(left: f32, top: f32, right: f32, bottom: f32, allegiance: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_units_in_screen_rectangle::call(left as f32, top as f32, right as f32, bottom as f32, allegiance as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (left as f32, top as f32, right as f32, bottom as f32, allegiance as i32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_visible_features {
            #[link(wasm_import_module = "spring:unit-rendering")]
            extern "C" {
                #[link_name = "get-visible-features"]
                pub fn call(p0: i32, p1: f32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-visible-features."]
        #[inline]
        pub unsafe fn get_visible_features(p0: i32, p1: f32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_get_visible_features::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_visible_projectiles {
            #[link(wasm_import_module = "spring:unit-rendering")]
            extern "C" {
                #[link_name = "get-visible-projectiles"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-visible-projectiles."]
        #[inline]
        pub unsafe fn get_visible_projectiles(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_visible_projectiles::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_visible_units(team_id: i32, radius: f32, include_icons: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_visible_units::call(team_id as i32, radius as f32, u32::from(include_icons) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (team_id as i32, radius as f32, u32::from(include_icons) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn is_unit_icon(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::is_unit_icon(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_view(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::is_unit_in_view(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_visible(unit_id: i32, radius: f32, check_icon: bool) -> Result<bool> {
            let value = crate::generated::unit_rendering::is_unit_visible(unit_id, radius, check_icon)?;
            Ok(value)
        }

    }


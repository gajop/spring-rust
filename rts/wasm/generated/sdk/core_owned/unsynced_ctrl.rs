    pub mod unsynced_ctrl {
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
        pub struct AssignMouseCursorQuery {
            pub command_name: String,
            pub cursor_file_name: String,
            pub overwrite: bool,
            pub hot_spot_top_left: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AssignMouseCursorResult {
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
        pub struct DeselectUnitMapQuery {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DeselectUnitMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawUnitCommandsQuery {
            pub unit_i_ds: Vec<i32>,
            pub table_or_array: bool,
            pub queue_draw_depth: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DrawUnitCommandsResult {
            pub success: bool,
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
        pub struct ForceLayoutUpdateQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ForceLayoutUpdateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ForceTesselationUpdateQuery {
            pub normal: bool,
            pub shadow: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ForceTesselationUpdateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterTextureQuery {
            pub tex_type: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterTextureResult {
            pub tex_name: String,
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
        pub struct LoadCmdColorsConfigQuery {
            pub filename: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadCmdColorsConfigResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadCtrlPanelConfigQuery {
            pub filename: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadCtrlPanelConfigResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadModelTexturesQuery {
            pub model_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadModelTexturesResult {
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
        pub struct PauseDollyCameraQuery {
            pub percent: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PauseDollyCameraResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PreloadFeatureDefModelQuery {
            pub def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PreloadFeatureDefModelResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PreloadUnitDefModelQuery {
            pub def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PreloadUnitDefModelResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReplaceMouseCursorQuery {
            pub old_cursor_file_name: String,
            pub new_cursor_file_name: String,
            pub hot_spot_top_left: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReplaceMouseCursorResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourcePack {
            pub metal: f32,
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResumeDollyCameraQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResumeDollyCameraResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RgbColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RunDollyCameraQuery {
            pub runtime_ms: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RunDollyCameraResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SDLSetTextInputRectQuery {
            pub x: i32,
            pub y: i32,
            pub w: i32,
            pub h: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SDLSetTextInputRectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SDLStartTextInputQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SDLStartTextInputResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SDLStopTextInputQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SDLStopTextInputResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectUnitMapQuery {
            pub unit_i_ds: Vec<i32>,
            pub append: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectUnitMapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetActiveCommandOptions {
            pub left_click: bool,
            pub right_click: bool,
            pub alt: bool,
            pub ctrl: bool,
            pub meta: bool,
            pub shift: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetActiveCommandQuery {
            pub cmd_index: i32,
            pub button: i32,
            pub options: SetActiveCommandOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetActiveCommandResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetAtmosphereQuery {
            pub params: AtmosphereParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetAtmosphereResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetAutoShowMetalQuery {
            pub enable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetAutoShowMetalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBoxSelectionByEngineQuery {
            pub state: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBoxSelectionByEngineResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBuildFacingQuery {
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBuildFacingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBuildSpacingQuery {
            pub spacing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBuildSpacingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraOffsetQuery {
            pub pos_offset: Float3,
            pub tilt_offset: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraOffsetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetClipboardQuery {
            pub text: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetClipboardResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCustomCommandDrawDataQuery {
            pub cmd_id: i32,
            pub cmd_reference: DefRef,
            pub color: Float4,
            pub show_area: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCustomCommandDrawDataResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCustomPaletteColorQuery {
            pub index: i32,
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCustomPaletteColorResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraCurveQuery {
            pub degree: i32,
            pub control_points: Vec<Float4>,
            pub knots: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraCurveResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraLookCurveQuery {
            pub degree: i32,
            pub control_points: Vec<Float4>,
            pub knots: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraLookCurveResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraLookPositionQuery {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraLookPositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraLookUnitQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraLookUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraModeQuery {
            pub mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraModeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraPositionQuery {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraPositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraRelativeModeQuery {
            pub mode: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDollyCameraRelativeModeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawGroundDeferredQuery {
            pub draw_deferred: bool,
            pub draw_forward: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawGroundDeferredResult {
            pub success: bool,
            pub deferred: bool,
            pub forward: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawGroundQuery {
            pub draw_ground: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawGroundResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawModelsDeferredQuery {
            pub draw_units_deferred: bool,
            pub draw_features_deferred: bool,
            pub draw_units_forward: bool,
            pub draw_features_forward: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawModelsDeferredResult {
            pub success: bool,
            pub units_deferred: bool,
            pub features_deferred: bool,
            pub units_forward: bool,
            pub features_forward: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawSelectionInfoQuery {
            pub draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawSelectionInfoResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawSkyQuery {
            pub draw_sky: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawSkyResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawWaterQuery {
            pub draw_water: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawWaterResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetEngineBuildSquareRenderingQuery {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetEngineBuildSquareRenderingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureAlwaysUpdateMatrixQuery {
            pub feature_id: i32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureAlwaysUpdateMatrixResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureEngineDrawMaskQuery {
            pub feature_id: i32,
            pub mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureEngineDrawMaskResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureFadeQuery {
            pub feature_id: i32,
            pub allow: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureFadeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureNoDrawQuery {
            pub feature_id: i32,
            pub no_draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureNoDrawResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeaturePaletteIndexQuery {
            pub feature_id: i32,
            pub custom_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeaturePaletteIndexResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetLastMessagePositionQuery {
            pub pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetLastMessagePositionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetLosViewColorsQuery {
            pub always: RgbColor,
            pub los: RgbColor,
            pub radar: RgbColor,
            pub jam: RgbColor,
            pub radar2: RgbColor,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetLosViewColorsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapRenderingParamsQuery {
            pub params: MapRenderingParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapRenderingParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapShaderQuery {
            pub standard_shader_id: i32,
            pub deferred_shader_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapShaderResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapShadingTextureQuery {
            pub tex_type: String,
            pub tex_name: String,
            pub num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapShadingTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMiniMapRotationQuery {
            pub radians: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMiniMapRotationResult {
            pub success: bool,
            pub rotation: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMouseCursorQuery {
            pub cursor_name: String,
            pub scale: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMouseCursorResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetNanoProjectileParamsQuery {
            pub r: f32,
            pub v: f32,
            pub a: f32,
            pub rand_r: f32,
            pub rand_v: f32,
            pub rand_a: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetNanoProjectileParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetShockFrontFactorsOptions {
            pub min_area: Option<f32>,
            pub min_power: Option<f32>,
            pub dist_adj: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetShockFrontFactorsQuery {
            pub options: SetShockFrontFactorsOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetShockFrontFactorsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSkyBoxTextureQuery {
            pub tex_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSkyBoxTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSunDirectionQuery {
            pub dir: Float3,
            pub intensity: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSunDirectionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSunLightingQuery {
            pub params: SunLightingParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSunLightingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitAlwaysUpdateMatrixQuery {
            pub unit_id: i32,
            pub always_update_matrix: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitAlwaysUpdateMatrixResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitDefIconQuery {
            pub unit_def_id: i32,
            pub icon_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitDefIconResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitDefImageQuery {
            pub unit_def_id: i32,
            pub image: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitDefImageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitEngineDrawMaskQuery {
            pub unit_id: i32,
            pub draw_mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitEngineDrawMaskResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitIconDrawQuery {
            pub unit_id: i32,
            pub draw_icon: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitIconDrawResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitIconQuery {
            pub unit_id: i32,
            pub icon_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitIconResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitLeaveTracksQuery {
            pub unit_id: i32,
            pub leave_tracks: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitLeaveTracksResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoDrawQuery {
            pub unit_id: i32,
            pub no_draw: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoDrawResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoGroupQuery {
            pub unit_id: i32,
            pub no_group: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoGroupResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoMinimapQuery {
            pub unit_id: i32,
            pub no_minimap: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoMinimapResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoSelectQuery {
            pub unit_id: i32,
            pub no_select: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitNoSelectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitPaletteIndexQuery {
            pub unit_id: i32,
            pub custom_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitPaletteIndexResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetVideoCapturingModeQuery {
            pub allow_capture_mode: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetVideoCapturingModeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetVideoCapturingTimeOffsetQuery {
            pub time_offset: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetVideoCapturingTimeOffsetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWMCaptionQuery {
            pub title: String,
            pub title_short: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWMCaptionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWMIconQuery {
            pub icon_file_name: String,
            pub force_resolution: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWMIconResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWaterParamsQuery {
            pub params: WaterParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWaterParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWaterTextureQuery {
            pub tex_type: String,
            pub tex_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWaterTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWindowGeometryOptions {
            pub full_screen: bool,
            pub borderless: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWindowGeometryQuery {
            pub display_index: i32,
            pub window_pos_x: i32,
            pub window_pos_y: i32,
            pub window_size_x: i32,
            pub window_size_y: i32,
            pub options: SetWindowGeometryOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWindowGeometryResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWindowMaximizedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWindowMaximizedResult {
            pub maximized: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWindowMinimizedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetWindowMinimizedResult {
            pub minimized: bool,
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
        pub struct WarpMouseQuery {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WarpMouseResult {
            pub success: bool,
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
        pub struct SetDrawGroundDeferredValue {
            pub success: bool,
            pub deferred: bool,
            pub forward: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetDrawModelsDeferredValue {
            pub success: bool,
            pub units_deferred: bool,
            pub features_deferred: bool,
            pub units_forward: bool,
            pub features_forward: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMiniMapRotationValue {
            pub success: bool,
            pub rotation: i32,
        }

        #[inline]
        pub fn assign_mouse_cursor(command_name: &str, cursor_file_name: &str, overwrite: bool, hot_spot_top_left: bool) -> Result<bool> {
            let mut command_name_bytes = command_name.as_bytes().to_vec();
            if command_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            command_name_bytes.push(0);
            let command_name_cstr = core::ffi::CStr::from_bytes_with_nul(&command_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut cursor_file_name_bytes = cursor_file_name.as_bytes().to_vec();
            if cursor_file_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            cursor_file_name_bytes.push(0);
            let cursor_file_name_cstr = core::ffi::CStr::from_bytes_with_nul(&cursor_file_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::assign_mouse_cursor(&command_name_cstr, &cursor_file_name_cstr, overwrite, hot_spot_top_left)
        }

        #[inline]
        pub fn deselect_unit_map(unit_i_ds: &Vec<i32>) -> Result<bool> {
            crate::generated::borrowed::unsynced_ctrl::deselect_unit_map(unit_i_ds.as_slice())
        }

        #[inline]
        pub fn draw_unit_commands(unit_i_ds: &Vec<i32>, table_or_array: bool, queue_draw_depth: i32) -> Result<bool> {
            crate::generated::borrowed::unsynced_ctrl::draw_unit_commands(unit_i_ds.as_slice(), table_or_array, queue_draw_depth)
        }

        #[inline]
        pub fn force_layout_update(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::force_layout_update(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn force_tesselation_update(normal: bool, shadow: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::force_tesselation_update(normal, shadow)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_water_texture {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "get-water-texture"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.get-water-texture."]
        #[inline]
        pub unsafe fn get_water_texture(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_water_texture::call(p0, p1) }
        }

        #[inline]
        pub fn load_cmd_colors_config(filename: &str) -> Result<bool> {
            let mut filename_bytes = filename.as_bytes().to_vec();
            if filename_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            filename_bytes.push(0);
            let filename_cstr = core::ffi::CStr::from_bytes_with_nul(&filename_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::load_cmd_colors_config(&filename_cstr)
        }

        #[inline]
        pub fn load_ctrl_panel_config(filename: &str) -> Result<bool> {
            let mut filename_bytes = filename.as_bytes().to_vec();
            if filename_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            filename_bytes.push(0);
            let filename_cstr = core::ffi::CStr::from_bytes_with_nul(&filename_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::load_ctrl_panel_config(&filename_cstr)
        }

        #[inline]
        pub fn load_model_textures(model_name: &str) -> Result<bool> {
            let mut model_name_bytes = model_name.as_bytes().to_vec();
            if model_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            model_name_bytes.push(0);
            let model_name_cstr = core::ffi::CStr::from_bytes_with_nul(&model_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::load_model_textures(&model_name_cstr)
        }

        #[inline]
        pub fn pause_dolly_camera(percent: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::pause_dolly_camera(percent)?;
            Ok(value)
        }

        #[inline]
        pub fn preload_feature_def_model(def_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::preload_feature_def_model(def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn preload_unit_def_model(def_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::preload_unit_def_model(def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn replace_mouse_cursor(old_cursor_file_name: &str, new_cursor_file_name: &str, hot_spot_top_left: bool) -> Result<bool> {
            let mut old_cursor_file_name_bytes = old_cursor_file_name.as_bytes().to_vec();
            if old_cursor_file_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            old_cursor_file_name_bytes.push(0);
            let old_cursor_file_name_cstr = core::ffi::CStr::from_bytes_with_nul(&old_cursor_file_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut new_cursor_file_name_bytes = new_cursor_file_name.as_bytes().to_vec();
            if new_cursor_file_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            new_cursor_file_name_bytes.push(0);
            let new_cursor_file_name_cstr = core::ffi::CStr::from_bytes_with_nul(&new_cursor_file_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::replace_mouse_cursor(&old_cursor_file_name_cstr, &new_cursor_file_name_cstr, hot_spot_top_left)
        }

        #[inline]
        pub fn resume_dolly_camera(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::resume_dolly_camera(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn run_dolly_camera(runtime_ms: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::run_dolly_camera(runtime_ms)?;
            Ok(value)
        }

        #[inline]
        pub fn sdl_set_text_input_rect(x: i32, y: i32, w: i32, h: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::sdl_set_text_input_rect(x, y, w, h)?;
            Ok(value)
        }

        #[inline]
        pub fn sdl_start_text_input(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::sdl_start_text_input(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn sdl_stop_text_input(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::sdl_stop_text_input(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn select_unit_map(unit_i_ds: &Vec<i32>, append: bool) -> Result<bool> {
            crate::generated::borrowed::unsynced_ctrl::select_unit_map(unit_i_ds.as_slice(), append)
        }

        #[inline]
        pub fn set_active_command(cmd_index: i32, button: i32, options: SetActiveCommandOptions) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_active_command(cmd_index, button, crate::generated::unsynced_ctrl::SetActiveCommandOptions { left_click: options.left_click, right_click: options.right_click, alt: options.alt, ctrl: options.ctrl, meta: options.meta, shift: options.shift })?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_atmosphere {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-atmosphere"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-atmosphere."]
        #[inline]
        pub unsafe fn set_atmosphere(p0: i32) -> i64 {
            unsafe { __core_owned_set_atmosphere::call(p0) }
        }

        #[inline]
        pub fn set_auto_show_metal(enable: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_auto_show_metal(enable)?;
            Ok(value)
        }

        #[inline]
        pub fn set_box_selection_by_engine(state: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_box_selection_by_engine(state)?;
            Ok(value)
        }

        #[inline]
        pub fn set_build_facing(facing: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_build_facing(facing)?;
            Ok(value)
        }

        #[inline]
        pub fn set_build_spacing(spacing: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_build_spacing(spacing)?;
            Ok(value)
        }

        #[inline]
        pub fn set_camera_offset(pos_offset: Float3, tilt_offset: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_camera_offset(crate::generated::unsynced_ctrl::Float3 { x: pos_offset.x, y: pos_offset.y, z: pos_offset.z }, crate::generated::unsynced_ctrl::Float3 { x: tilt_offset.x, y: tilt_offset.y, z: tilt_offset.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_clipboard(text: &str) -> Result<bool> {
            let mut text_bytes = text.as_bytes().to_vec();
            if text_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            text_bytes.push(0);
            let text_cstr = core::ffi::CStr::from_bytes_with_nul(&text_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_clipboard(&text_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_custom_command_draw_data {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-custom-command-draw-data"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-custom-command-draw-data."]
        #[inline]
        pub unsafe fn set_custom_command_draw_data(p0: i32, p1: i32, p2: i32) -> i64 {
            unsafe { __core_owned_set_custom_command_draw_data::call(p0, p1, p2) }
        }

        #[inline]
        pub fn set_custom_palette_color(index: i32, r: f32, g: f32, b: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_custom_palette_color(index, r, g, b)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_dolly_camera_curve {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-dolly-camera-curve"]
                pub fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-dolly-camera-curve."]
        #[inline]
        pub unsafe fn set_dolly_camera_curve(p0: i32, p1: i32) -> i64 {
            unsafe { __core_owned_set_dolly_camera_curve::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_dolly_camera_look_curve {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-dolly-camera-look-curve"]
                pub fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-dolly-camera-look-curve."]
        #[inline]
        pub unsafe fn set_dolly_camera_look_curve(p0: i32, p1: i32) -> i64 {
            unsafe { __core_owned_set_dolly_camera_look_curve::call(p0, p1) }
        }

        #[inline]
        pub fn set_dolly_camera_look_position(position: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_look_position(crate::generated::unsynced_ctrl::Float3 { x: position.x, y: position.y, z: position.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_look_unit(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_look_unit(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_mode(mode: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_mode(mode)?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_position(position: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_position(crate::generated::unsynced_ctrl::Float3 { x: position.x, y: position.y, z: position.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_relative_mode(mode: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_relative_mode(mode)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_ground(draw_ground: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_ground(draw_ground)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_ground_deferred(draw_deferred: bool, draw_forward: bool) -> Result<SetDrawGroundDeferredValue> {
            let value = crate::generated::unsynced_ctrl::set_draw_ground_deferred(draw_deferred, draw_forward)?;
            Ok(SetDrawGroundDeferredValue {
                success: value.0,
                deferred: value.1,
                forward: value.2
            })
        }

        #[inline]
        pub fn set_draw_models_deferred(draw_units_deferred: bool, draw_features_deferred: bool, draw_units_forward: bool, draw_features_forward: bool) -> Result<SetDrawModelsDeferredValue> {
            let value = crate::generated::unsynced_ctrl::set_draw_models_deferred(draw_units_deferred, draw_features_deferred, draw_units_forward, draw_features_forward)?;
            Ok(SetDrawModelsDeferredValue {
                success: value.0,
                units_deferred: value.1,
                features_deferred: value.2,
                units_forward: value.3,
                features_forward: value.4
            })
        }

        #[inline]
        pub fn set_draw_selection_info(draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_selection_info(draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_sky(draw_sky: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_sky(draw_sky)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_water(draw_water: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_water(draw_water)?;
            Ok(value)
        }

        #[inline]
        pub fn set_engine_build_square_rendering(enabled: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_engine_build_square_rendering(enabled)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_always_update_matrix(feature_id: i32, enable: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_always_update_matrix(feature_id, enable)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_engine_draw_mask(feature_id: i32, mask: u32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_engine_draw_mask(feature_id, mask)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_fade(feature_id: i32, allow: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_fade(feature_id, allow)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_no_draw(feature_id: i32, no_draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_no_draw(feature_id, no_draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_palette_index(feature_id: i32, custom_index: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_palette_index(feature_id, custom_index)?;
            Ok(value)
        }

        #[inline]
        pub fn set_last_message_position(pos: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_last_message_position(crate::generated::unsynced_ctrl::Float3 { x: pos.x, y: pos.y, z: pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_los_view_colors(always: RgbColor, los: RgbColor, radar: RgbColor, jam: RgbColor, radar2: RgbColor) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_los_view_colors(crate::generated::unsynced_ctrl::RgbColor { r: always.r, g: always.g, b: always.b }, crate::generated::unsynced_ctrl::RgbColor { r: los.r, g: los.g, b: los.b }, crate::generated::unsynced_ctrl::RgbColor { r: radar.r, g: radar.g, b: radar.b }, crate::generated::unsynced_ctrl::RgbColor { r: jam.r, g: jam.g, b: jam.b }, crate::generated::unsynced_ctrl::RgbColor { r: radar2.r, g: radar2.g, b: radar2.b })?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_map_rendering_params {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-map-rendering-params"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-map-rendering-params."]
        #[inline]
        pub unsafe fn set_map_rendering_params(p0: i32) -> i64 {
            unsafe { __core_owned_set_map_rendering_params::call(p0) }
        }

        #[inline]
        pub fn set_map_shader(standard_shader_id: i32, deferred_shader_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_map_shader(standard_shader_id, deferred_shader_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_map_shading_texture(tex_type: &str, tex_name: &str, num: i32) -> Result<bool> {
            let mut tex_type_bytes = tex_type.as_bytes().to_vec();
            if tex_type_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            tex_type_bytes.push(0);
            let tex_type_cstr = core::ffi::CStr::from_bytes_with_nul(&tex_type_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut tex_name_bytes = tex_name.as_bytes().to_vec();
            if tex_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            tex_name_bytes.push(0);
            let tex_name_cstr = core::ffi::CStr::from_bytes_with_nul(&tex_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_map_shading_texture(&tex_type_cstr, &tex_name_cstr, num)
        }

        #[inline]
        pub fn set_mini_map_rotation(radians: f32) -> Result<SetMiniMapRotationValue> {
            let value = crate::generated::unsynced_ctrl::set_mini_map_rotation(radians)?;
            Ok(SetMiniMapRotationValue {
                success: value.0,
                rotation: value.1
            })
        }

        #[inline]
        pub fn set_mouse_cursor(cursor_name: &str, scale: f32) -> Result<bool> {
            let mut cursor_name_bytes = cursor_name.as_bytes().to_vec();
            if cursor_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            cursor_name_bytes.push(0);
            let cursor_name_cstr = core::ffi::CStr::from_bytes_with_nul(&cursor_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_mouse_cursor(&cursor_name_cstr, scale)
        }

        #[inline]
        pub fn set_nano_projectile_params(r: f32, v: f32, a: f32, rand_r: f32, rand_v: f32, rand_a: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_nano_projectile_params(r, v, a, rand_r, rand_v, rand_a)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_shock_front_factors {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-shock-front-factors"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-shock-front-factors."]
        #[inline]
        pub unsafe fn set_shock_front_factors(p0: i32) -> i64 {
            unsafe { __core_owned_set_shock_front_factors::call(p0) }
        }

        #[inline]
        pub fn set_sky_box_texture(tex_name: &str) -> Result<bool> {
            let mut tex_name_bytes = tex_name.as_bytes().to_vec();
            if tex_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            tex_name_bytes.push(0);
            let tex_name_cstr = core::ffi::CStr::from_bytes_with_nul(&tex_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_sky_box_texture(&tex_name_cstr)
        }

        #[inline]
        pub fn set_sun_direction(dir: Float3, intensity: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_sun_direction(crate::generated::unsynced_ctrl::Float3 { x: dir.x, y: dir.y, z: dir.z }, intensity)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_sun_lighting {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-sun-lighting"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-sun-lighting."]
        #[inline]
        pub unsafe fn set_sun_lighting(p0: i32) -> i64 {
            unsafe { __core_owned_set_sun_lighting::call(p0) }
        }

        #[inline]
        pub fn set_unit_always_update_matrix(unit_id: i32, always_update_matrix: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_always_update_matrix(unit_id, always_update_matrix)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_def_icon(unit_def_id: i32, icon_name: &str) -> Result<bool> {
            let mut icon_name_bytes = icon_name.as_bytes().to_vec();
            if icon_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            icon_name_bytes.push(0);
            let icon_name_cstr = core::ffi::CStr::from_bytes_with_nul(&icon_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_unit_def_icon(unit_def_id, &icon_name_cstr)
        }

        #[inline]
        pub fn set_unit_def_image(unit_def_id: i32, image: &str) -> Result<bool> {
            let mut image_bytes = image.as_bytes().to_vec();
            if image_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            image_bytes.push(0);
            let image_cstr = core::ffi::CStr::from_bytes_with_nul(&image_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_unit_def_image(unit_def_id, &image_cstr)
        }

        #[inline]
        pub fn set_unit_engine_draw_mask(unit_id: i32, draw_mask: u32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_engine_draw_mask(unit_id, draw_mask)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_icon(unit_id: i32, icon_name: &str) -> Result<bool> {
            let mut icon_name_bytes = icon_name.as_bytes().to_vec();
            if icon_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            icon_name_bytes.push(0);
            let icon_name_cstr = core::ffi::CStr::from_bytes_with_nul(&icon_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_unit_icon(unit_id, &icon_name_cstr)
        }

        #[inline]
        pub fn set_unit_icon_draw(unit_id: i32, draw_icon: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_icon_draw(unit_id, draw_icon)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_leave_tracks(unit_id: i32, leave_tracks: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_leave_tracks(unit_id, leave_tracks)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_draw(unit_id: i32, no_draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_draw(unit_id, no_draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_group(unit_id: i32, no_group: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_group(unit_id, no_group)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_minimap(unit_id: i32, no_minimap: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_minimap(unit_id, no_minimap)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_select(unit_id: i32, no_select: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_select(unit_id, no_select)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_palette_index(unit_id: i32, custom_index: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_palette_index(unit_id, custom_index)?;
            Ok(value)
        }

        #[inline]
        pub fn set_video_capturing_mode(allow_capture_mode: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_video_capturing_mode(allow_capture_mode)?;
            Ok(value)
        }

        #[inline]
        pub fn set_video_capturing_time_offset(time_offset: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_video_capturing_time_offset(time_offset)?;
            Ok(value)
        }

        #[inline]
        pub fn set_wm_caption(title: &str, title_short: &str) -> Result<bool> {
            let mut title_bytes = title.as_bytes().to_vec();
            if title_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            title_bytes.push(0);
            let title_cstr = core::ffi::CStr::from_bytes_with_nul(&title_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut title_short_bytes = title_short.as_bytes().to_vec();
            if title_short_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            title_short_bytes.push(0);
            let title_short_cstr = core::ffi::CStr::from_bytes_with_nul(&title_short_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_wm_caption(&title_cstr, &title_short_cstr)
        }

        #[inline]
        pub fn set_wm_icon(icon_file_name: &str, force_resolution: bool) -> Result<bool> {
            let mut icon_file_name_bytes = icon_file_name.as_bytes().to_vec();
            if icon_file_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            icon_file_name_bytes.push(0);
            let icon_file_name_cstr = core::ffi::CStr::from_bytes_with_nul(&icon_file_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_wm_icon(&icon_file_name_cstr, force_resolution)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_water_params {
            #[link(wasm_import_module = "spring:unsynced-ctrl")]
            extern "C" {
                #[link_name = "set-water-params"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-ctrl.set-water-params."]
        #[inline]
        pub unsafe fn set_water_params(p0: i32) -> i64 {
            unsafe { __core_owned_set_water_params::call(p0) }
        }

        #[inline]
        pub fn set_water_texture(tex_type: &str, tex_name: &str) -> Result<bool> {
            let mut tex_type_bytes = tex_type.as_bytes().to_vec();
            if tex_type_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            tex_type_bytes.push(0);
            let tex_type_cstr = core::ffi::CStr::from_bytes_with_nul(&tex_type_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut tex_name_bytes = tex_name.as_bytes().to_vec();
            if tex_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            tex_name_bytes.push(0);
            let tex_name_cstr = core::ffi::CStr::from_bytes_with_nul(&tex_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::unsynced_ctrl::set_water_texture(&tex_type_cstr, &tex_name_cstr)
        }

        #[inline]
        pub fn set_window_geometry(display_index: i32, window_pos_x: i32, window_pos_y: i32, window_size_x: i32, window_size_y: i32, options: SetWindowGeometryOptions) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_window_geometry(display_index, window_pos_x, window_pos_y, window_size_x, window_size_y, crate::generated::unsynced_ctrl::SetWindowGeometryOptions { full_screen: options.full_screen, borderless: options.borderless })?;
            Ok(value)
        }

        #[inline]
        pub fn set_window_maximized(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_window_maximized(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn set_window_minimized(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_window_minimized(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn warp_mouse(x: i32, y: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::warp_mouse(x, y)?;
            Ok(value)
        }

    }


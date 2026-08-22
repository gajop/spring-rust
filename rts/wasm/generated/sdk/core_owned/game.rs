    pub mod game {
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
        pub struct AreHelperAIsEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AreHelperAIsEnabledResult {
            pub enabled: bool,
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
        pub struct FixedAlliesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FixedAlliesResult {
            pub fixed: bool,
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
        pub struct GameMapInfo {
            pub map_name: String,
            pub map_description: String,
            pub map_checksum: String,
            pub map_hardness: f32,
            pub extractor_radius: f32,
            pub tidal: f32,
            pub water_damage: f32,
            pub gravity: f32,
            pub map_x: i32,
            pub map_y: i32,
            pub map_size_x: i32,
            pub map_size_z: i32,
            pub map_damage: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameModInfo {
            pub game_name: String,
            pub game_short_name: String,
            pub game_version: String,
            pub game_mutator: String,
            pub game_desc: String,
            pub mod_name: String,
            pub mod_short_name: String,
            pub mod_version: String,
            pub mod_mutator: String,
            pub mod_desc: String,
            pub mod_checksum: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameRulesInfo {
            pub max_units: i32,
            pub construction_decay: bool,
            pub construction_decay_time: i32,
            pub construction_decay_speed: f32,
            pub multi_reclaim: i32,
            pub reclaim_method: i32,
            pub reclaim_unit_method: i32,
            pub reclaim_unit_energy_cost_factor: f32,
            pub reclaim_unit_efficiency: f32,
            pub reclaim_feature_energy_cost_factor: f32,
            pub reclaim_unit_drain_health: bool,
            pub reclaim_allow_enemies: bool,
            pub reclaim_allow_allies: bool,
            pub repair_energy_cost_factor: f32,
            pub resurrect_energy_cost_factor: f32,
            pub capture_energy_cost_factor: f32,
            pub transport_air: i32,
            pub transport_ship: i32,
            pub transport_hover: i32,
            pub transport_ground: i32,
            pub fire_at_killed: i32,
            pub fire_at_crashing: i32,
            pub require_sonar_under_water: i32,
            pub paralyze_on_max_health: bool,
            pub paralyze_decline_rate: f32,
            pub allow_engine_playerlist: bool,
            pub native_excess_sharing: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameRulesResourceInfo {
            pub reclaim_unit_cost_factor: ResourcePack,
            pub reclaim_unit_efficiency: ResourcePack,
            pub reclaim_feature_cost_factor: ResourcePack,
            pub repair_cost_factor: ResourcePack,
            pub resurrect_cost_factor: ResourcePack,
            pub capture_cost_factor: ResourcePack,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GameSetupInfo {
            pub start_pos_type: i32,
            pub ghosted_buildings: bool,
            pub demo_play_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamStartBoxQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamStartBoxResult {
            pub box_: StartBox,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFacingFromHeadingQuery {
            pub heading: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFacingFromHeadingResult {
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGaiaTeamIDQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGaiaTeamIDResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameFrameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameFrameResult {
            pub low16: u32,
            pub high16: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameMapInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameMapInfoResult {
            pub info: GameMapInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameModInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameModInfoResult {
            pub info: GameModInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesInfoResult {
            pub info: GameRulesInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesResourceInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesResourceInfoResult {
            pub info: GameRulesResourceInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSecondsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSecondsResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSetupInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSetupInfoResult {
            pub info: GameSetupInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGlobalLosQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGlobalLosResult {
            pub los: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHeadingFromFacingQuery {
            pub facing: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHeadingFromFacingResult {
            pub heading: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHeadingFromVectorQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetHeadingFromVectorResult {
            pub heading: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapOptionQuery {
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapOptionResult {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapOptionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapOptionsResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapStartPositionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapStartPositionsResult {
            pub positions: Vec<StartPosition>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModOptionQuery {
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModOptionResult {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModOptionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModOptionsResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSideDataByIndexQuery {
            pub side_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSideDataByIndexResult {
            pub data: SideData,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSideDataCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSideDataCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSideDataQuery {
            pub side_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSideDataResult {
            pub data: SideData,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamStartPositionQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamStartPositionResult {
            pub position: Float3,
            pub valid: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTidalQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTidalResult {
            pub strength: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVectorFromHeadingQuery {
            pub heading: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVectorFromHeadingResult {
            pub vector: Float2,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWindQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWindResult {
            pub data: WindData,
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
        pub struct IsCheatingEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsCheatingEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsDevLuaEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsDevLuaEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsEditDefsEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsEditDefsEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsGameOverQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsGameOverResult {
            pub game_over: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsGodModeEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsGodModeEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsNoCostEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsNoCostEnabledResult {
            pub enabled: bool,
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
        pub struct SideData {
            pub side_name: String,
            pub case_name: String,
            pub start_unit: String,
            pub side_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StartBox {
            pub min_x: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StartPosition {
            pub pos: Float3,
            pub team_id: i32,
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
        pub struct WindData {
            pub min: f32,
            pub max: f32,
            pub current: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamStartBoxValue {
            pub box_: StartBox,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameFrameValue {
            pub low16: u32,
            pub high16: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapOptionValue {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModOptionValue {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamStartPositionValue {
            pub position: Float3,
            pub valid: bool,
        }

        #[inline]
        pub fn are_helper_a_is_enabled(unused: u8) -> Result<bool> {
            let value = crate::generated::game::are_helper_a_is_enabled(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn fixed_allies(unused: u8) -> Result<bool> {
            let value = crate::generated::game::fixed_allies(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_ally_team_start_box(ally_team_id: i32) -> Result<GetAllyTeamStartBoxValue> {
            let value = crate::generated::game::get_ally_team_start_box(ally_team_id)?;
            Ok(GetAllyTeamStartBoxValue {
                box_: StartBox { min_x: value.0.min_x, min_z: value.0.min_z, max_x: value.0.max_x, max_z: value.0.max_z },
                exists: value.1
            })
        }

        #[inline]
        pub fn get_facing_from_heading(heading: i32) -> Result<i32> {
            let value = crate::generated::game::get_facing_from_heading(heading)?;
            Ok(value)
        }

        #[inline]
        pub fn get_gaia_team_id(unused: u8) -> Result<i32> {
            let value = crate::generated::game::get_gaia_team_id(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_game_frame(unused: u8) -> Result<GetGameFrameValue> {
            let value = crate::generated::game::get_game_frame(unused)?;
            Ok(GetGameFrameValue {
                low16: value.0,
                high16: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_game_map_info {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-game-map-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-game-map-info."]
        #[inline]
        pub unsafe fn get_game_map_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_game_map_info::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_game_mod_info {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-game-mod-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-game-mod-info."]
        #[inline]
        pub unsafe fn get_game_mod_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_game_mod_info::call(p0, p1) }
        }

        #[inline]
        pub fn get_game_rules_info(unused: u8) -> Result<GameRulesInfo> {
            let value = crate::generated::game::get_game_rules_info(unused)?;
            Ok(GameRulesInfo { max_units: value.max_units, construction_decay: value.construction_decay, construction_decay_time: value.construction_decay_time, construction_decay_speed: value.construction_decay_speed, multi_reclaim: value.multi_reclaim, reclaim_method: value.reclaim_method, reclaim_unit_method: value.reclaim_unit_method, reclaim_unit_energy_cost_factor: value.reclaim_unit_energy_cost_factor, reclaim_unit_efficiency: value.reclaim_unit_efficiency, reclaim_feature_energy_cost_factor: value.reclaim_feature_energy_cost_factor, reclaim_unit_drain_health: value.reclaim_unit_drain_health, reclaim_allow_enemies: value.reclaim_allow_enemies, reclaim_allow_allies: value.reclaim_allow_allies, repair_energy_cost_factor: value.repair_energy_cost_factor, resurrect_energy_cost_factor: value.resurrect_energy_cost_factor, capture_energy_cost_factor: value.capture_energy_cost_factor, transport_air: value.transport_air, transport_ship: value.transport_ship, transport_hover: value.transport_hover, transport_ground: value.transport_ground, fire_at_killed: value.fire_at_killed, fire_at_crashing: value.fire_at_crashing, require_sonar_under_water: value.require_sonar_under_water, paralyze_on_max_health: value.paralyze_on_max_health, paralyze_decline_rate: value.paralyze_decline_rate, allow_engine_playerlist: value.allow_engine_playerlist, native_excess_sharing: value.native_excess_sharing })
        }

        #[inline]
        pub fn get_game_rules_resource_info(unused: u8) -> Result<GameRulesResourceInfo> {
            let value = crate::generated::game::get_game_rules_resource_info(unused)?;
            Ok(GameRulesResourceInfo { reclaim_unit_cost_factor: ResourcePack { metal: value.reclaim_unit_cost_factor.metal, energy: value.reclaim_unit_cost_factor.energy }, reclaim_unit_efficiency: ResourcePack { metal: value.reclaim_unit_efficiency.metal, energy: value.reclaim_unit_efficiency.energy }, reclaim_feature_cost_factor: ResourcePack { metal: value.reclaim_feature_cost_factor.metal, energy: value.reclaim_feature_cost_factor.energy }, repair_cost_factor: ResourcePack { metal: value.repair_cost_factor.metal, energy: value.repair_cost_factor.energy }, resurrect_cost_factor: ResourcePack { metal: value.resurrect_cost_factor.metal, energy: value.resurrect_cost_factor.energy }, capture_cost_factor: ResourcePack { metal: value.capture_cost_factor.metal, energy: value.capture_cost_factor.energy } })
        }

        #[inline]
        pub fn get_game_seconds(unused: u8) -> Result<f32> {
            let value = crate::generated::game::get_game_seconds(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_game_setup_info {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-game-setup-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-game-setup-info."]
        #[inline]
        pub unsafe fn get_game_setup_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_game_setup_info::call(p0, p1) }
        }

        #[inline]
        pub fn get_global_los(ally_team_id: i32) -> Result<i32> {
            let value = crate::generated::game::get_global_los(ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_heading_from_facing(facing: i32) -> Result<i32> {
            let value = crate::generated::game::get_heading_from_facing(facing)?;
            Ok(value)
        }

        #[inline]
        pub fn get_heading_from_vector(x: f32, z: f32) -> Result<i32> {
            let value = crate::generated::game::get_heading_from_vector(x, z)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_map_option {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-map-option"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-map-option."]
        #[inline]
        pub unsafe fn get_map_option(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_map_option::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_map_options {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-map-options"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-map-options."]
        #[inline]
        pub unsafe fn get_map_options(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_map_options::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_map_start_positions {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-map-start-positions"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-map-start-positions."]
        #[inline]
        pub unsafe fn get_map_start_positions(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_map_start_positions::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_mod_option {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-mod-option"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-mod-option."]
        #[inline]
        pub unsafe fn get_mod_option(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_mod_option::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_mod_options {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-mod-options"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-mod-options."]
        #[inline]
        pub unsafe fn get_mod_options(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_mod_options::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_side_data {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-side-data"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-side-data."]
        #[inline]
        pub unsafe fn get_side_data(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_side_data::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_side_data_by_index {
            #[link(wasm_import_module = "spring:game")]
            extern "C" {
                #[link_name = "get-side-data-by-index"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-side-data-by-index."]
        #[inline]
        pub unsafe fn get_side_data_by_index(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_side_data_by_index::call(p0, p1) }
        }

        #[inline]
        pub fn get_side_data_count(unused: u8) -> Result<u32> {
            let value = crate::generated::game::get_side_data_count(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_team_start_position(team_id: i32) -> Result<GetTeamStartPositionValue> {
            let value = crate::generated::game::get_team_start_position(team_id)?;
            Ok(GetTeamStartPositionValue {
                position: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                valid: value.1
            })
        }

        #[inline]
        pub fn get_tidal(unused: u8) -> Result<f32> {
            let value = crate::generated::game::get_tidal(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_vector_from_heading(heading: i32) -> Result<Float2> {
            let value = crate::generated::game::get_vector_from_heading(heading)?;
            Ok(Float2 { x: value.x, y: value.y })
        }

        #[inline]
        pub fn get_wind(unused: u8) -> Result<WindData> {
            let value = crate::generated::game::get_wind(unused)?;
            Ok(WindData { min: value.min, max: value.max, current: value.current })
        }

        #[inline]
        pub fn is_cheating_enabled(unused: u8) -> Result<bool> {
            let value = crate::generated::game::is_cheating_enabled(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn is_dev_lua_enabled(unused: u8) -> Result<bool> {
            let value = crate::generated::game::is_dev_lua_enabled(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn is_edit_defs_enabled(unused: u8) -> Result<bool> {
            let value = crate::generated::game::is_edit_defs_enabled(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn is_game_over(unused: u8) -> Result<bool> {
            let value = crate::generated::game::is_game_over(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn is_god_mode_enabled(unused: u8) -> Result<bool> {
            let value = crate::generated::game::is_god_mode_enabled(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn is_no_cost_enabled(unused: u8) -> Result<bool> {
            let value = crate::generated::game::is_no_cost_enabled(unused)?;
            Ok(value)
        }

    }


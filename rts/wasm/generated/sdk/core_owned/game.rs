    pub mod game {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AreHelperAIsEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AreHelperAIsEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FixedAlliesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FixedAlliesResult {
            pub fixed: bool,
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllyTeamStartBoxQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllyTeamStartBoxResult {
            pub box_: StartBox,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFacingFromHeadingQuery {
            pub heading: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFacingFromHeadingResult {
            pub facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGaiaTeamIDQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGaiaTeamIDResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameFrameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameFrameResult {
            pub low16: u32,
            pub high16: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameMapInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameMapInfoResult {
            pub info: GameMapInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameModInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameModInfoResult {
            pub info: GameModInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameRulesInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameRulesInfoResult {
            pub info: GameRulesInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameRulesResourceInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesResourceInfoResult {
            pub info: GameRulesResourceInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSecondsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSecondsResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSetupInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSetupInfoResult {
            pub info: GameSetupInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGlobalLosQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGlobalLosResult {
            pub los: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetHeadingFromFacingQuery {
            pub facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetHeadingFromFacingResult {
            pub heading: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetHeadingFromVectorQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMapOptionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapOptionsResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetModOptionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModOptionsResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSideDataByIndexQuery {
            pub side_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSideDataByIndexResult {
            pub data: SideData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSideDataCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamStartPositionQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamStartPositionResult {
            pub position: Float3,
            pub valid: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTidalQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTidalResult {
            pub strength: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVectorFromHeadingQuery {
            pub heading: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVectorFromHeadingResult {
            pub vector: Float2,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWindQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWindResult {
            pub data: WindData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsCheatingEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsCheatingEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsDevLuaEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsDevLuaEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsEditDefsEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsEditDefsEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsGameOverQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsGameOverResult {
            pub game_over: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsGodModeEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsGodModeEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsNoCostEnabledQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsNoCostEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SideData {
            pub side_name: String,
            pub case_name: String,
            pub start_unit: String,
            pub side_index: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct StartBox {
            pub min_x: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct StartPosition {
            pub pos: Float3,
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct WindData {
            pub min: f32,
            pub max: f32,
            pub current: f32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllyTeamStartBoxValue {
            pub box_: StartBox,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[inline]
        pub fn get_game_map_info(unused: u8) -> Result<GameMapInfo> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::game::get_game_map_info(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GameMapInfo { map_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_description: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_checksum: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_hardness: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, extractor_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, tidal: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, water_damage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, gravity: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_x: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_y: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_size_x: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_size_z: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, map_damage: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_game_mod_info(unused: u8) -> Result<GameModInfo> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::game::get_game_mod_info(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GameModInfo { game_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, game_short_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, game_version: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, game_mutator: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, game_desc: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mod_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mod_short_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mod_version: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mod_mutator: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mod_desc: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mod_checksum: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
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

        #[inline]
        pub fn get_game_setup_info(unused: u8) -> Result<GameSetupInfo> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::game::get_game_setup_info(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GameSetupInfo { start_pos_type: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ghosted_buildings: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, demo_play_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
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
            unsafe extern "C" {
                #[link_name = "get-map-option"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-map-option."]
        #[doc(hidden)]
        #[inline]
        pub fn get_map_option(p0: i32, p1: i32) -> i32 {
            __core_owned_get_map_option::call(p0, p1)
        }

        #[inline]
        pub fn get_map_options(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::game::get_map_options(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_map_start_positions {
            #[link(wasm_import_module = "spring:game")]
            unsafe extern "C" {
                #[link_name = "get-map-start-positions"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-map-start-positions."]
        #[doc(hidden)]
        #[inline]
        pub fn get_map_start_positions(p0: i32, p1: i32) -> i32 {
            __core_owned_get_map_start_positions::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_mod_option {
            #[link(wasm_import_module = "spring:game")]
            unsafe extern "C" {
                #[link_name = "get-mod-option"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-mod-option."]
        #[doc(hidden)]
        #[inline]
        pub fn get_mod_option(p0: i32, p1: i32) -> i32 {
            __core_owned_get_mod_option::call(p0, p1)
        }

        #[inline]
        pub fn get_mod_options(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::game::get_mod_options(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_side_data {
            #[link(wasm_import_module = "spring:game")]
            unsafe extern "C" {
                #[link_name = "get-side-data"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:game.get-side-data."]
        #[doc(hidden)]
        #[inline]
        pub fn get_side_data(p0: i32, p1: i32) -> i32 {
            __core_owned_get_side_data::call(p0, p1)
        }

        #[inline]
        pub fn get_side_data_by_index(side_index: u32) -> Result<SideData> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::game::get_side_data_by_index(side_index as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = SideData { side_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, case_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, start_unit: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, side_index: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
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


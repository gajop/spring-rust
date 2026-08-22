    pub mod teams {
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
        pub struct AIInfo {
            pub skirmish_aiid: i32,
            pub name: String,
            pub hosting_player_id: i32,
            pub short_name: String,
            pub version: String,
            pub options: Vec<AIOption>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AIOption {
            pub key: String,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AllyTeamInfo {
            pub keys: Vec<String>,
            pub values: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ArePlayersAlliedQuery {
            pub player_id1: i32,
            pub player_id2: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ArePlayersAlliedResult {
            pub allied: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AreTeamsAlliedQuery {
            pub team_id1: i32,
            pub team_id2: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AreTeamsAlliedResult {
            pub allied: bool,
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
        pub struct GetAIInfoQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAIInfoResult {
            pub info: AIInfo,
            pub is_ai: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamInfoQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamInfoResult {
            pub info: AllyTeamInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamListQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamListResult {
            pub ally_teams: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerControlledUnitQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerControlledUnitResult {
            pub unit_id: i32,
            pub has_unit: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerInfoQuery {
            pub player_id: i32,
            pub get_player_opts: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerInfoResult {
            pub info: PlayerInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListInAllyTeamQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListInAllyTeamResult {
            pub players: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListInTeamQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListInTeamResult {
            pub players: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListQuery {
            pub team_id: i32,
            pub active: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListResult {
            pub players: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamAllyTeamIDQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamAllyTeamIDResult {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamInfoQuery {
            pub team_id: i32,
            pub get_team_keys: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamInfoResult {
            pub info: TeamInfo,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamListQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamListResult {
            pub teams: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamLuaAIQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamLuaAIResult {
            pub lua_ai: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamMaxUnitsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamMaxUnitsResult {
            pub max_units: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamResourceStatsQuery {
            pub team_id: i32,
            pub resource: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamResourceStatsResult {
            pub resources: TeamResources,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamResourcesQuery {
            pub team_id: i32,
            pub resource: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamResourcesResult {
            pub resources: TeamResources,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamStatsHistoryQuery {
            pub team_id: i32,
            pub start_index: i32,
            pub end_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamStatsHistoryResult {
            pub history: Vec<TeamStatsHistoryPoint>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitStatsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitStatsResult {
            pub stats: TeamUnitStats,
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
        pub struct PlayerInfo {
            pub player_id: i32,
            pub name: String,
            pub is_active: bool,
            pub is_ai: bool,
            pub is_spec: bool,
            pub team_id: i32,
            pub ally_team_id: i32,
            pub ping_time: f32,
            pub cpu_usage: f32,
            pub country: String,
            pub rank: i32,
            pub has_skirmish_a_is_in_team: bool,
            pub custom_keys: String,
            pub desynced: bool,
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
        pub struct TeamInfo {
            pub team_id: i32,
            pub ally_team_id: i32,
            pub leader_id: i32,
            pub is_dead: bool,
            pub side: String,
            pub color: u32,
            pub custom_keys: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamResources {
            pub metal_current: f32,
            pub metal_storage: f32,
            pub metal_pull: f32,
            pub metal_income: f32,
            pub metal_expense: f32,
            pub metal_shared: f32,
            pub metal_sent: f32,
            pub metal_received: f32,
            pub metal_excess: f32,
            pub energy_current: f32,
            pub energy_storage: f32,
            pub energy_pull: f32,
            pub energy_income: f32,
            pub energy_expense: f32,
            pub energy_shared: f32,
            pub energy_sent: f32,
            pub energy_received: f32,
            pub energy_excess: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamStatsHistoryPoint {
            pub metal_used: f32,
            pub metal_produced: f32,
            pub metal_excess: f32,
            pub metal_received: f32,
            pub metal_sent: f32,
            pub energy_used: f32,
            pub energy_produced: f32,
            pub energy_excess: f32,
            pub energy_received: f32,
            pub energy_sent: f32,
            pub damage_dealt: f32,
            pub damage_received: f32,
            pub units_produced: u32,
            pub units_died: u32,
            pub units_received: u32,
            pub units_sent: u32,
            pub units_captured: u32,
            pub units_out_captured: u32,
            pub units_killed: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TeamUnitStats {
            pub killed: u32,
            pub died: u32,
            pub captured_by: u32,
            pub captured_from: u32,
            pub received: u32,
            pub sent: u32,
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

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_ally_team_list {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-ally-team-list"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_player_list {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-player-list"]
                pub fn call(pteam_id: i32, pactive: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_player_list_in_ally_team {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-player-list-in-ally-team"]
                pub fn call(pally_team_id: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_player_list_in_team {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-player-list-in-team"]
                pub fn call(pteam_id: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_team_list {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-team-list"]
                pub fn call(pally_team_id: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_team_lua_ai {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-team-lua-ai"]
                pub fn call(pteam_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAIInfoValue {
            pub info: AIInfo,
            pub is_ai: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerControlledUnitValue {
            pub unit_id: i32,
            pub has_unit: bool,
        }

        #[inline]
        pub fn are_players_allied(player_id1: i32, player_id2: i32) -> Result<bool> {
            let value = crate::generated::teams::are_players_allied(player_id1, player_id2)?;
            Ok(value)
        }

        #[inline]
        pub fn are_teams_allied(team_id1: i32, team_id2: i32) -> Result<bool> {
            let value = crate::generated::teams::are_teams_allied(team_id1, team_id2)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_ai_info {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-ai-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-ai-info."]
        #[inline]
        pub unsafe fn get_ai_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_ai_info::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_ally_team_info {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-ally-team-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-ally-team-info."]
        #[inline]
        pub unsafe fn get_ally_team_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_ally_team_info::call(p0, p1) }
        }

        #[inline]
        pub fn get_ally_team_list(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_ally_team_list::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_player_controlled_unit(player_id: i32) -> Result<GetPlayerControlledUnitValue> {
            let value = crate::generated::teams::get_player_controlled_unit(player_id)?;
            Ok(GetPlayerControlledUnitValue {
                unit_id: value.0,
                has_unit: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_player_info {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-player-info"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-player-info."]
        #[inline]
        pub unsafe fn get_player_info(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_player_info::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_player_list(team_id: i32, active: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_player_list::call(team_id as i32, u32::from(active) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (team_id as i32, u32::from(active) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_player_list_in_ally_team(ally_team_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_player_list_in_ally_team::call(ally_team_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (ally_team_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_player_list_in_team(team_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_player_list_in_team::call(team_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (team_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_team_ally_team_id(team_id: i32) -> Result<i32> {
            let value = crate::generated::teams::get_team_ally_team_id(team_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_info {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-team-info"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-team-info."]
        #[inline]
        pub unsafe fn get_team_info(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_team_info::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_team_list(ally_team_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_team_list::call(ally_team_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (ally_team_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_team_lua_ai(team_id: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_team_lua_ai::call(team_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (team_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_team_max_units(team_id: i32) -> Result<i32> {
            let value = crate::generated::teams::get_team_max_units(team_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_resource_stats {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-team-resource-stats"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-team-resource-stats."]
        #[inline]
        pub unsafe fn get_team_resource_stats(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_team_resource_stats::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_resources {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-team-resources"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-team-resources."]
        #[inline]
        pub unsafe fn get_team_resources(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_team_resources::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_stats_history {
            #[link(wasm_import_module = "spring:teams")]
            extern "C" {
                #[link_name = "get-team-stats-history"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-team-stats-history."]
        #[inline]
        pub unsafe fn get_team_stats_history(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_get_team_stats_history::call(p0, p1, p2, p3) }
        }

        #[inline]
        pub fn get_team_unit_stats(team_id: i32) -> Result<TeamUnitStats> {
            let value = crate::generated::teams::get_team_unit_stats(team_id)?;
            Ok(TeamUnitStats { killed: value.killed, died: value.died, captured_by: value.captured_by, captured_from: value.captured_from, received: value.received, sent: value.sent })
        }

    }


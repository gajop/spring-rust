    pub mod teams {
        use super::{Result, String, Vec};

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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ArePlayersAlliedQuery {
            pub player_id1: i32,
            pub player_id2: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ArePlayersAlliedResult {
            pub allied: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AreTeamsAlliedQuery {
            pub team_id1: i32,
            pub team_id2: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AreTeamsAlliedResult {
            pub allied: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAIInfoQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAIInfoResult {
            pub info: AIInfo,
            pub is_ai: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllyTeamInfoQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamInfoResult {
            pub info: AllyTeamInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllyTeamListQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllyTeamListResult {
            pub ally_teams: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerControlledUnitQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerControlledUnitResult {
            pub unit_id: i32,
            pub has_unit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerInfoQuery {
            pub player_id: i32,
            pub get_player_opts: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerInfoResult {
            pub info: PlayerInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerListInAllyTeamQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListInAllyTeamResult {
            pub players: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerListInTeamQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListInTeamResult {
            pub players: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerListQuery {
            pub team_id: i32,
            pub active: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerListResult {
            pub players: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamAllyTeamIDQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamAllyTeamIDResult {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamInfoQuery {
            pub team_id: i32,
            pub get_team_keys: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamInfoResult {
            pub info: TeamInfo,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamListQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamListResult {
            pub teams: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamLuaAIQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamLuaAIResult {
            pub lua_ai: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamMaxUnitsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamMaxUnitsResult {
            pub max_units: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamResourceStatsQuery {
            pub team_id: i32,
            pub resource: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamResourceStatsResult {
            pub resources: TeamResources,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamResourcesQuery {
            pub team_id: i32,
            pub resource: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamResourcesResult {
            pub resources: TeamResources,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamStatsHistoryQuery {
            pub team_id: i32,
            pub start_index: i32,
            pub end_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamStatsHistoryResult {
            pub history: Vec<TeamStatsHistoryPoint>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitStatsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamUnitStatsResult {
            pub stats: TeamUnitStats,
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
        pub struct TeamInfo {
            pub team_id: i32,
            pub ally_team_id: i32,
            pub leader_id: i32,
            pub is_dead: bool,
            pub side: String,
            pub color: u32,
            pub custom_keys: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TeamUnitStats {
            pub killed: u32,
            pub died: u32,
            pub captured_by: u32,
            pub captured_from: u32,
            pub received: u32,
            pub sent: u32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[inline]
        pub fn get_ai_info(team_id: i32) -> Result<GetAIInfoValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::teams::get_ai_info(team_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetAIInfoValue {
                            info: AIInfo { skirmish_aiid: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, hosting_player_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, short_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, version: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, options: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(AIOption { key: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, value: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items } },
                            is_ai: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                        };
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
        pub fn get_ally_team_info(ally_team_id: i32) -> Result<AllyTeamInfo> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::teams::get_ally_team_info(ally_team_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = AllyTeamInfo { keys: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, values: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } };
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

        #[inline]
        pub fn get_player_info(player_id: i32, get_player_opts: bool) -> Result<PlayerInfo> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::teams::get_player_info(player_id, get_player_opts as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = PlayerInfo { player_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_active: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_ai: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_spec: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, team_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ally_team_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ping_time: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, cpu_usage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, country: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, rank: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, has_skirmish_a_is_in_team: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, custom_keys: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, desynced: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
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
        pub fn get_player_list(team_id: i32, active: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_player_list::call(team_id, u32::from(active) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (team_id, u32::from(active) as i32);
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
                    let status = unsafe { __core_variable_output_get_player_list_in_ally_team::call(ally_team_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (ally_team_id);
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
                    let status = unsafe { __core_variable_output_get_player_list_in_team::call(team_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (team_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_team_ally_team_id(team_id: i32) -> Result<i32> {
            let value = crate::generated::teams::get_team_ally_team_id(team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_team_info(team_id: i32, get_team_keys: bool) -> Result<TeamInfo> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::teams::get_team_info(team_id, get_team_keys as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = TeamInfo { team_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ally_team_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, leader_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_dead: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, side: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, color: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, custom_keys: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
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
        pub fn get_team_list(ally_team_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_team_list::call(ally_team_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (ally_team_id);
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
                    let status = unsafe { __core_variable_output_get_team_lua_ai::call(team_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (team_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_team_max_units(team_id: i32) -> Result<i32> {
            let value = crate::generated::teams::get_team_max_units(team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_team_resource_stats(team_id: i32, resource: &str) -> Result<TeamResources> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + resource.len()); __b.extend_from_slice(&(resource.len() as u32).to_le_bytes()); __b.extend_from_slice(resource.as_bytes()); __b };
            let mut __output = [0u8; 72];
            crate::generated::dynamic_input::teams::get_team_resource_stats(team_id, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(TeamResources { metal_current: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_storage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_pull: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_income: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_expense: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_shared: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_sent: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_received: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_excess: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_current: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_storage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_pull: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_income: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_expense: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_shared: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_sent: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_received: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_excess: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? })
        }

        #[inline]
        pub fn get_team_resources(team_id: i32, resource: &str) -> Result<TeamResources> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + resource.len()); __b.extend_from_slice(&(resource.len() as u32).to_le_bytes()); __b.extend_from_slice(resource.as_bytes()); __b };
            let mut __output = [0u8; 72];
            crate::generated::dynamic_input::teams::get_team_resources(team_id, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(TeamResources { metal_current: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_storage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_pull: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_income: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_expense: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_shared: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_sent: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_received: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal_excess: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_current: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_storage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_pull: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_income: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_expense: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_shared: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_sent: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_received: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_excess: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_stats_history {
            #[link(wasm_import_module = "spring:teams")]
            unsafe extern "C" {
                #[link_name = "get-team-stats-history"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:teams.get-team-stats-history."]
        #[doc(hidden)]
        #[inline]
        pub fn get_team_stats_history(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            __core_owned_get_team_stats_history::call(p0, p1, p2, p3)
        }

        #[inline]
        pub fn get_team_unit_stats(team_id: i32) -> Result<TeamUnitStats> {
            let value = crate::generated::teams::get_team_unit_stats(team_id)?;
            Ok(TeamUnitStats { killed: value.killed, died: value.died, captured_by: value.captured_by, captured_from: value.captured_from, received: value.received, sent: value.sent })
        }

    }


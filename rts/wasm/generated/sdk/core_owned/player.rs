    pub mod player {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLocalAllyTeamIDQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLocalAllyTeamIDResult {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLocalPlayerIDQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLocalPlayerIDResult {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLocalTeamIDQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLocalTeamIDResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerRosterQuery {
            pub sort_mode: i32,
            pub show_pathing_players: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerRosterResult {
            pub entries: Vec<RosterEntry>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerStatisticsQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerStatisticsResult {
            pub stats: PlayerStats,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerTrafficQuery {
            pub player_id: i32,
            pub packet_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerTrafficResult {
            pub traffic: Vec<PlayerTraffic>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSpectatingStateQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSpectatingStateResult {
            pub spectating: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerStats {
            pub mouse_pixels: i32,
            pub mouse_clicks: i32,
            pub key_presses: i32,
            pub unit_commands: u32,
            pub avg_command_size: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlayerTraffic {
            pub player_id: i32,
            pub packets_sent: u32,
            pub packets_received: u32,
            pub bytes_sent: u32,
            pub bytes_received: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RosterEntry {
            pub name: String,
            pub player_id: i32,
            pub team_id: i32,
            pub ally_team_id: i32,
            pub is_ai: bool,
            pub is_spec: bool,
            pub is_active: bool,
            pub ping_time: f32,
            pub cpu_usage: f32,
            pub country: String,
            pub rank: i32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn get_local_ally_team_id(unused: u8) -> Result<i32> {
            let value = crate::generated::player::get_local_ally_team_id(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_local_player_id(unused: u8) -> Result<i32> {
            let value = crate::generated::player::get_local_player_id(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_local_team_id(unused: u8) -> Result<i32> {
            let value = crate::generated::player::get_local_team_id(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_player_roster(sort_mode: i32, show_pathing_players: bool) -> Result<Vec<RosterEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::player::get_player_roster(sort_mode, show_pathing_players as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(RosterEntry { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, player_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, team_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ally_team_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_ai: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_spec: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_active: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ping_time: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, cpu_usage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, country: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, rank: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        pub fn get_player_statistics(player_id: i32) -> Result<PlayerStats> {
            let value = crate::generated::player::get_player_statistics(player_id)?;
            Ok(PlayerStats { mouse_pixels: value.mouse_pixels, mouse_clicks: value.mouse_clicks, key_presses: value.key_presses, unit_commands: value.unit_commands, avg_command_size: value.avg_command_size })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_player_traffic {
            #[link(wasm_import_module = "spring:player")]
            unsafe extern "C" {
                #[link_name = "get-player-traffic"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:player.get-player-traffic."]
        #[doc(hidden)]
        #[inline]
        pub fn get_player_traffic(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_player_traffic::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_spectating_state(unused: u8) -> Result<bool> {
            let value = crate::generated::player::get_spectating_state(unused)?;
            Ok(value)
        }

    }


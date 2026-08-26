    pub mod messages {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct ConsoleEntry {
            pub text: String,
            pub priority: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EchoQuery {
            pub message: String,
            pub rest: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EchoResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetConsoleBufferQuery {
            pub max_lines: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConsoleBufferResult {
            pub entries: Vec<ConsoleEntry>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCurrentTooltipQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCurrentTooltipResult {
            pub tooltip: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUserWritingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUserWritingResult {
            pub writing: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LogQuery {
            pub section: String,
            pub level: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LogResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendAllyChatQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendAllyChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendCommandsQuery {
            pub command: String,
            pub rest: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendCommandsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaGaiaQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendLuaGaiaResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaMenuMsgQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendLuaMenuMsgResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaRulesQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendLuaRulesResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendLuaUIQuery {
            pub message: String,
            pub mode: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendLuaUIResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendMessageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToAllyTeamQuery {
            pub ally_team_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendMessageToAllyTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToPlayerQuery {
            pub player_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendMessageToPlayerResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToSpectatorsQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendMessageToSpectatorsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendMessageToTeamQuery {
            pub team_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendMessageToTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendPrivateChatQuery {
            pub message: String,
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendPrivateChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendPublicChatQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendPublicChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendSkirmishAIMessageQuery {
            pub ai_id: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendSkirmishAIMessageResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendSpectatorChatQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendSpectatorChatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SendToUnsyncedQuery {
            pub message: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SendToUnsyncedResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_current_tooltip {
            #[link(wasm_import_module = "spring:messages")]
            unsafe extern "C" {
                #[link_name = "get-current-tooltip"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[inline]
        pub fn echo(message: &str, rest: &str) -> Result<bool> {
            crate::messages::echo(message, rest)
        }

        #[inline]
        pub fn get_console_buffer(max_lines: u32) -> Result<Vec<ConsoleEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::messages::get_console_buffer(max_lines as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(ConsoleEntry { text: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, priority: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        pub fn get_current_tooltip(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_current_tooltip::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn is_user_writing(unused: u8) -> Result<bool> {
            let value = crate::generated::messages::is_user_writing(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn log(section: &str, level: i32, message: &str) -> Result<bool> {
            crate::messages::log(section, level, message)
        }

        #[inline]
        pub fn send_ally_chat(message: &str) -> Result<bool> {
            crate::messages::send_ally_chat(message)
        }

        #[inline]
        pub fn send_commands(command: &str, rest: &str) -> Result<bool> {
            crate::messages::send_commands(command, rest)
        }

        #[inline]
        pub fn send_lua_gaia_msg(message: &str) -> Result<bool> {
            crate::messages::send_lua_gaia_msg(message)
        }

        #[inline]
        pub fn send_lua_menu_msg(message: &str) -> Result<bool> {
            crate::messages::send_lua_menu_msg(message)
        }

        #[inline]
        pub fn send_lua_rules_msg(message: &str) -> Result<bool> {
            crate::messages::send_lua_rules_msg(message)
        }

        #[inline]
        pub fn send_lua_ui_msg(message: &str, mode: &str) -> Result<bool> {
            crate::messages::send_lua_ui_msg(message, mode)
        }

        #[inline]
        pub fn send_message(message: &str) -> Result<bool> {
            crate::messages::send_message(message)
        }

        #[inline]
        pub fn send_message_to_ally_team(ally_team_id: i32, message: &str) -> Result<bool> {
            crate::messages::send_message_to_ally_team(ally_team_id, message)
        }

        #[inline]
        pub fn send_message_to_player(player_id: i32, message: &str) -> Result<bool> {
            crate::messages::send_message_to_player(player_id, message)
        }

        #[inline]
        pub fn send_message_to_spectators(message: &str) -> Result<bool> {
            crate::messages::send_message_to_spectators(message)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_send_message_to_team {
            #[link(wasm_import_module = "spring:messages")]
            unsafe extern "C" {
                #[link_name = "send-message-to-team"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:messages.send-message-to-team."]
        #[doc(hidden)]
        #[inline]
        pub fn send_message_to_team(p0: i32, p1: i32, p2: i32) -> i64 {
            __core_owned_send_message_to_team::call(p0, p1, p2)
        }

        #[inline]
        pub fn send_private_chat(message: &str, player_id: i32) -> Result<bool> {
            crate::messages::send_private_chat(message, player_id)
        }

        #[inline]
        pub fn send_public_chat(message: &str) -> Result<bool> {
            crate::messages::send_public_chat(message)
        }

        #[inline]
        pub fn send_skirmish_ai_message(ai_id: i32, message: &str) -> Result<bool> {
            crate::messages::send_skirmish_ai_message(ai_id, message)
        }

        #[inline]
        pub fn send_spectator_chat(message: &str) -> Result<bool> {
            crate::messages::send_spectator_chat(message)
        }

        #[inline]
        pub fn send_to_unsynced(message: &str) -> Result<bool> {
            crate::messages::send_to_unsynced(message)
        }

    }


// Messages portion of the Spring Core-Wasm guest SDK.

use super::{unpack_bool, ApiError, ErrorCode, Result};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:messages")]
    extern "C" {
        pub fn echo(message: i32, message_len: i32, rest: i32, rest_len: i32) -> i64;
        pub fn log(
            section: i32,
            section_len: i32,
            level: i32,
            message: i32,
            message_len: i32,
        ) -> i64;
        #[link_name = "send-message"]
        pub fn send_message(message: i32, message_len: i32) -> i64;
        #[link_name = "send-message-to-player"]
        pub fn send_message_to_player(player_id: i32, message: i32, message_len: i32) -> i64;
        #[link_name = "send-message-to-team"]
        pub fn send_message_to_team(team_id: i32, message: i32, message_len: i32) -> i64;
        #[link_name = "send-message-to-ally-team"]
        pub fn send_message_to_ally_team(ally_team_id: i32, message: i32, message_len: i32) -> i64;
        #[link_name = "send-message-to-spectators"]
        pub fn send_message_to_spectators(message: i32, message_len: i32) -> i64;
        #[link_name = "send-public-chat"]
        pub fn send_public_chat(message: i32, message_len: i32) -> i64;
        #[link_name = "send-ally-chat"]
        pub fn send_ally_chat(message: i32, message_len: i32) -> i64;
        #[link_name = "send-spectator-chat"]
        pub fn send_spectator_chat(message: i32, message_len: i32) -> i64;
        #[link_name = "send-private-chat"]
        pub fn send_private_chat(message: i32, message_len: i32, player_id: i32) -> i64;
        #[link_name = "send-commands"]
        pub fn send_commands(command: i32, command_len: i32, rest: i32, rest_len: i32) -> i64;
        #[link_name = "send-lua-menu-msg"]
        pub fn send_lua_menu_msg(message: i32, message_len: i32) -> i64;
        #[link_name = "send-skirmish-ai-message"]
        pub fn send_skirmish_ai_message(ai_id: i32, message: i32, message_len: i32) -> i64;
        #[link_name = "send-lua-ui-msg"]
        pub fn send_lua_ui_msg(message: i32, message_len: i32, mode: i32, mode_len: i32) -> i64;
        #[link_name = "send-lua-gaia-msg"]
        pub fn send_lua_gaia_msg(message: i32, message_len: i32) -> i64;
        #[link_name = "send-lua-rules-msg"]
        pub fn send_lua_rules_msg(message: i32, message_len: i32) -> i64;
        #[link_name = "send-to-unsynced"]
        pub fn send_to_unsynced(message: i32, message_len: i32) -> i64;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn string_parts(value: &str) -> Result<(i32, i32)> {
    if value.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    if value.is_empty() {
        return Ok((0, 0));
    }
    let pointer = value.as_ptr() as usize;
    if pointer > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok((pointer as u32 as i32, value.len() as u32 as i32))
}

macro_rules! one_string {
    ($name:ident, $raw:ident) => {
        #[inline]
        pub fn $name(message: &str) -> Result<bool> {
            #[cfg(target_arch = "wasm32")]
            {
                let (pointer, length) = string_parts(message)?;
                return unpack_bool(unsafe { raw::$raw(pointer, length) });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = message;
                Err(unreachable!())
            }
        }
    };
}

macro_rules! id_string {
    ($name:ident, $raw:ident, $id:ident) => {
        #[inline]
        pub fn $name($id: i32, message: &str) -> Result<bool> {
            #[cfg(target_arch = "wasm32")]
            {
                let (pointer, length) = string_parts(message)?;
                return unpack_bool(unsafe { raw::$raw($id, pointer, length) });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = ($id, message);
                Err(unreachable!())
            }
        }
    };
}

one_string!(send_message, send_message);
id_string!(send_message_to_player, send_message_to_player, player_id);
id_string!(send_message_to_team, send_message_to_team, team_id);
id_string!(
    send_message_to_ally_team,
    send_message_to_ally_team,
    ally_team_id
);
one_string!(send_message_to_spectators, send_message_to_spectators);
one_string!(send_public_chat, send_public_chat);
one_string!(send_ally_chat, send_ally_chat);
one_string!(send_spectator_chat, send_spectator_chat);
one_string!(send_lua_menu_msg, send_lua_menu_msg);
id_string!(send_skirmish_ai_message, send_skirmish_ai_message, ai_id);
one_string!(send_lua_gaia_msg, send_lua_gaia_msg);
one_string!(send_lua_rules_msg, send_lua_rules_msg);
one_string!(send_to_unsynced, send_to_unsynced);

#[inline]
pub fn echo(message: &str, rest: &str) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (message_pointer, message_length) = string_parts(message)?;
        let (rest_pointer, rest_length) = string_parts(rest)?;
        unpack_bool(unsafe {
            raw::echo(message_pointer, message_length, rest_pointer, rest_length)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (message, rest);
        Err(unreachable!())
    }
}

#[inline]
pub fn log(section: &str, level: i32, message: &str) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (section_pointer, section_length) = string_parts(section)?;
        let (message_pointer, message_length) = string_parts(message)?;
        unpack_bool(unsafe {
            raw::log(
                section_pointer,
                section_length,
                level,
                message_pointer,
                message_length,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (section, level, message);
        Err(unreachable!())
    }
}

#[inline]
pub fn send_private_chat(message: &str, player_id: i32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = string_parts(message)?;
        unpack_bool(unsafe { raw::send_private_chat(pointer, length, player_id) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (message, player_id);
        Err(unreachable!())
    }
}

#[inline]
pub fn send_commands(command: &str, rest: &str) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (command_pointer, command_length) = string_parts(command)?;
        let (rest_pointer, rest_length) = string_parts(rest)?;
        unpack_bool(unsafe {
            raw::send_commands(command_pointer, command_length, rest_pointer, rest_length)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (command, rest);
        Err(unreachable!())
    }
}

#[inline]
pub fn send_lua_ui_msg(message: &str, mode: &str) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let (message_pointer, message_length) = string_parts(message)?;
        let (mode_pointer, mode_length) = string_parts(mode)?;
        unpack_bool(unsafe {
            raw::send_lua_ui_msg(message_pointer, message_length, mode_pointer, mode_length)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (message, mode);
        Err(unreachable!())
    }
}

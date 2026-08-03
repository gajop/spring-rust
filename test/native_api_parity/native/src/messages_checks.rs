use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_messages_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "is_user_writing" => {
                let native = self
                    .interface
                    .messages()
                    .is_user_writing()
                    .map_err(|err| format!("is_user_writing() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "writing", native)
            }
            "get_current_tooltip" => {
                let native = self
                    .interface
                    .messages()
                    .get_current_tooltip()
                    .map_err(|err| format!("get_current_tooltip() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "tooltip",
                    native.as_deref().unwrap_or(""),
                )
            }
            "get_console_buffer_count"
            | "echo_console_buffer_count"
            | "send_message_console_buffer_count"
            | "log_console_buffer_count"
            | "send_message_to_player_console_buffer_count"
            | "send_message_to_team_console_buffer_count"
            | "send_message_to_ally_team_console_buffer_count"
            | "send_message_to_spectators_console_buffer_count" => {
                let max_lines = i32_field(message, "maxLines")?;
                let native = self
                    .interface
                    .messages()
                    .get_console_buffer(max_lines as u32)
                    .map_err(|err| format!("get_console_buffer({max_lines}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "send_public_chat" => {
                let success = self
                    .interface
                    .messages()
                    .send_public_chat(str_field(message, "message")?)
                    .map_err(|err| format!("send_public_chat() failed: {err:?}"))?;
                if !success {
                    return Err("send_public_chat returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_ally_chat" => {
                let success = self
                    .interface
                    .messages()
                    .send_ally_chat(str_field(message, "message")?)
                    .map_err(|err| format!("send_ally_chat() failed: {err:?}"))?;
                if !success {
                    return Err("send_ally_chat returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_spectator_chat" => {
                let success = self
                    .interface
                    .messages()
                    .send_spectator_chat(str_field(message, "message")?)
                    .map_err(|err| format!("send_spectator_chat() failed: {err:?}"))?;
                if !success {
                    return Err("send_spectator_chat returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_private_chat" => {
                let success = self
                    .interface
                    .messages()
                    .send_private_chat(
                        str_field(message, "message")?,
                        i32_field(message, "playerID")?,
                    )
                    .map_err(|err| format!("send_private_chat() failed: {err:?}"))?;
                if !success {
                    return Err("send_private_chat returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_commands" => {
                let success = self
                    .interface
                    .messages()
                    .send_commands(str_field(message, "command")?, str_field(message, "rest")?)
                    .map_err(|err| format!("send_commands() failed: {err:?}"))?;
                if !success {
                    return Err("send_commands returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_lua_menu_msg" => {
                let success = self
                    .interface
                    .messages()
                    .send_lua_menu_msg(str_field(message, "message")?)
                    .map_err(|err| format!("send_lua_menu_msg() failed: {err:?}"))?;
                if !success {
                    return Err("send_lua_menu_msg returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_lua_ui_msg" => {
                let success = self
                    .interface
                    .messages()
                    .send_lua_uimsg(str_field(message, "message")?, str_field(message, "mode")?)
                    .map_err(|err| format!("send_lua_uimsg() failed: {err:?}"))?;
                if !success {
                    return Err("send_lua_uimsg returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_lua_gaia_msg" => {
                let success = self
                    .interface
                    .messages()
                    .send_lua_gaia_msg(str_field(message, "message")?)
                    .map_err(|err| format!("send_lua_gaia_msg() failed: {err:?}"))?;
                if !success {
                    return Err("send_lua_gaia_msg returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_lua_rules_msg" => {
                let success = self
                    .interface
                    .messages()
                    .send_lua_rules_msg(str_field(message, "message")?)
                    .map_err(|err| format!("send_lua_rules_msg() failed: {err:?}"))?;
                if !success {
                    return Err("send_lua_rules_msg returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "send_skirmish_ai_message" => {
                let processed = self
                    .interface
                    .messages()
                    .send_skirmish_aimessage(
                        i32_field(message, "aiTeam")?,
                        str_field(message, "message")?,
                    )
                    .map_err(|err| format!("send_skirmish_aimessage() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "processed", processed)
            }
            _ => Err(format!("unsupported messages check `{label}`")),
        }
    }

    pub(crate) fn set_messages_value(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(test_name_field(message)?) {
            "echo_console_buffer_count" => {
                let success = self
                    .interface
                    .messages()
                    .echo(str_field(message, "message")?, str_field(message, "rest")?)
                    .map_err(|err| format!("echo() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("echo returned false".to_string())
                }
            }
            "send_message_console_buffer_count" => {
                let success = self
                    .interface
                    .messages()
                    .send_message(str_field(message, "message")?)
                    .map_err(|err| format!("send_message() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("send_message returned false".to_string())
                }
            }
            "log_console_buffer_count" => {
                let success = self
                    .interface
                    .messages()
                    .log(
                        str_field(message, "section")?,
                        i32_field(message, "level")?,
                        str_field(message, "message")?,
                    )
                    .map_err(|err| format!("log() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("log returned false".to_string())
                }
            }
            "send_message_to_player_console_buffer_count" => {
                let success = self
                    .interface
                    .messages()
                    .send_message_to_player(
                        i32_field(message, "playerID")?,
                        str_field(message, "message")?,
                    )
                    .map_err(|err| format!("send_message_to_player() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("send_message_to_player returned false".to_string())
                }
            }
            "send_message_to_team_console_buffer_count" => {
                let success = self
                    .interface
                    .messages()
                    .send_message_to_team(
                        i32_field(message, "teamID")?,
                        str_field(message, "message")?,
                    )
                    .map_err(|err| format!("send_message_to_team() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("send_message_to_team returned false".to_string())
                }
            }
            "send_message_to_ally_team_console_buffer_count" => {
                let success = self
                    .interface
                    .messages()
                    .send_message_to_ally_team(
                        i32_field(message, "allyTeamID")?,
                        str_field(message, "message")?,
                    )
                    .map_err(|err| format!("send_message_to_ally_team() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("send_message_to_ally_team returned false".to_string())
                }
            }
            "send_message_to_spectators_console_buffer_count" => {
                let success = self
                    .interface
                    .messages()
                    .send_message_to_spectators(str_field(message, "message")?)
                    .map_err(|err| format!("send_message_to_spectators() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("send_message_to_spectators returned false".to_string())
                }
            }
            name => Err(format!("unsupported messages setter `{name}`")),
        }
    }
}

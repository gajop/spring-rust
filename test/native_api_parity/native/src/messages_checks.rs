use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_messages_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
            "is_user_writing" => {
                let native = self.interface.messages().is_user_writing()
                    .map_err(|err| format!("is_user_writing() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "writing", native)
            }
            "get_current_tooltip" => {
                let native = self.interface.messages().get_current_tooltip()
                    .map_err(|err| format!("get_current_tooltip() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "tooltip", native.as_deref().unwrap_or(""))
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
                let native = self.interface.messages().get_console_buffer(max_lines as u32)
                    .map_err(|err| format!("get_console_buffer({max_lines}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            _ => Err(format!("unsupported messages check `{label}`")),
        }
    }

    pub(crate) fn set_messages_value(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(str_field(message, "name")?) {
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
                    .log(str_field(message, "section")?, i32_field(message, "level")?, str_field(message, "message")?)
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
                    .send_message_to_player(i32_field(message, "playerID")?, str_field(message, "message")?)
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
                    .send_message_to_team(i32_field(message, "teamID")?, str_field(message, "message")?)
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
                    .send_message_to_ally_team(i32_field(message, "allyTeamID")?, str_field(message, "message")?)
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

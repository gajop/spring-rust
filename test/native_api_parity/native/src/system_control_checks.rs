use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_system_control_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "call_as_team" => {
                let team_id = i32_field(message, "teamID")?;
                let marker = str_field(message, "marker")?;
                let mut callback_called = false;
                let mut callback_team = -1;
                let success = self
                    .interface
                    .system_control()
                    .call_as_team(team_id, || {
                        callback_called = true;
                        callback_team = team_id;
                    })
                    .map_err(|err| {
                        format!("call_as_team({team_id}, {marker:?}) failed: {err:?}")
                    })?;
                if !success {
                    return Err("call_as_team returned false".to_string());
                }
                if !callback_called {
                    return Err("call_as_team did not invoke its callback".to_string());
                }
                self.same_bool_if_present(label, message, "callbackCalled", callback_called)?;
                self.same_i32_if_present(label, message, "callbackTeam", callback_team)?;
                self.same_i32_if_present(label, message, "returnCount", 2)?;
                self.same_string_if_present(label, message, "returnMarker", marker)?;
                self.same_bool_if_present(label, message, "returnFlag", true)
            }
            "clear_watch_dog_timer" => self.same_i32_if_present(label, message, "returnCount", 0),
            "ping" => {
                let tag = u32::try_from(i32_field(message, "tag")?)
                    .map_err(|_| "ping tag must be non-negative".to_string())?;
                let success = self
                    .interface
                    .system_control()
                    .ping(tag)
                    .map_err(|err| format!("ping({tag}) failed: {err:?}"))?;
                if !success {
                    return Err("ping returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "request_start_position" => {
                let pos = vec3_from_fields(message, "x", "y", "z")?;
                let ready = bool_field(message, "ready")?;
                let success = self
                    .interface
                    .system_control()
                    .request_start_position(pos, ready)
                    .map_err(|err| format!("request_start_position({ready}) failed: {err:?}"))?;
                if !success {
                    return Err("request_start_position returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "set_share_level" => {
                let resource = str_field(message, "resource")?;
                let level = f32_field(message, "level")?;
                let success = self
                    .interface
                    .system_control()
                    .set_share_level(resource, level)
                    .map_err(|err| {
                        format!("set_share_level({resource:?}, {level}) failed: {err:?}")
                    })?;
                if !success {
                    return Err("set_share_level returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "share_resources" => {
                let team_id = i32_field(message, "teamID")?;
                let resource = str_field(message, "resource")?;
                let amount = f32_field(message, "amount")?;
                let success = self
                    .interface
                    .system_control()
                    .share_resources(team_id, resource, amount)
                    .map_err(|err| {
                        format!(
                            "share_resources({team_id}, {resource:?}, {amount}) failed: {err:?}"
                        )
                    })?;
                if !success {
                    return Err("share_resources returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "is_replay" => {
                let native = self
                    .interface
                    .system_control()
                    .is_replay()
                    .map_err(|err| format!("is_replay() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "isReplay", native)
            }
            "get_game_name" => {
                let native = self
                    .interface
                    .system_control()
                    .get_game_name()
                    .map_err(|err| format!("get_game_name() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "gameName",
                    native.as_deref().unwrap_or(""),
                )
            }
            "get_menu_name" => {
                let native = self
                    .interface
                    .system_control()
                    .get_menu_name()
                    .map_err(|err| format!("get_menu_name() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "menuName",
                    native.as_deref().unwrap_or(""),
                )
            }
            "get_replay_length" => {
                let (_seconds, success) = self
                    .interface
                    .system_control()
                    .get_replay_length()
                    .map_err(|err| format!("get_replay_length() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasReplay", success)
            }
            "get_replay_file_path" => {
                let (_path, success) = self
                    .interface
                    .system_control()
                    .get_replay_file_path()
                    .map_err(|err| format!("get_replay_file_path() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "available", success)
            }
            "get_replay_recording_file_path" => {
                let (_path, success) = self
                    .interface
                    .system_control()
                    .get_replay_recording_file_path()
                    .map_err(|err| format!("get_replay_recording_file_path() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "available", success)
            }
            "get_game_state" => {
                let max_latency = f32_field(message, "maxLatency")?;
                let (done_loading, is_saved_game, is_client_paused, is_sim_lagging) = self
                    .interface
                    .system_control()
                    .get_game_state(max_latency)
                    .map_err(|err| format!("get_game_state({max_latency}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "doneLoading", done_loading)?;
                self.same_bool_if_present(label, message, "isSavedGame", is_saved_game)?;
                self.same_bool_if_present(label, message, "isClientPaused", is_client_paused)?;
                self.same_bool_if_present(label, message, "isSimLagging", is_sim_lagging)
            }
            "get_video_capturing_mode" => {
                let native = self
                    .interface
                    .system_control()
                    .get_video_capturing_mode()
                    .map_err(|err| format!("get_video_capturing_mode() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "allowRecord", native)
            }
            "get_gather_mode" => {
                let native = self
                    .interface
                    .system_control()
                    .get_gather_mode()
                    .map_err(|err| format!("get_gather_mode() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "gatherMode", native)
            }
            "get_window_display_mode" => {
                let (width, height, bpp, refresh, format_name, success) = self
                    .interface
                    .system_control()
                    .get_window_display_mode()
                    .map_err(|err| format!("get_window_display_mode() failed: {err:?}"))?;
                if !success {
                    return Ok(());
                }
                self.same_i32_if_present(label, message, "width", width)?;
                self.same_i32_if_present(label, message, "height", height)?;
                self.same_i32_if_present(label, message, "bpp", bpp)?;
                self.same_i32_if_present(label, message, "refresh", refresh)?;
                self.same_string_if_present(
                    label,
                    message,
                    "pixelFormatName",
                    format_name.as_deref().unwrap_or(""),
                )
            }
            _ => Err(format!("unsupported system control check `{label}`")),
        }
    }

    pub(crate) fn set_system_control_value(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(test_name_field(message)?) {
            "call_as_team" => {
                let team_id = i32_field(message, "teamID")?;
                let success = self
                    .interface
                    .system_control()
                    .call_as_team(team_id, || {})
                    .map_err(|err| format!("call_as_team({team_id}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("call_as_team returned false".to_string())
                }
            }
            "clear_watch_dog_timer" => {
                let success = self
                    .interface
                    .system_control()
                    .clear_watch_dog_timer(
                        str_field(message, "threadName")?,
                        bool_field(message, "keepStopped")?,
                    )
                    .map_err(|err| format!("clear_watch_dog_timer() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("clear_watch_dog_timer returned false".to_string())
                }
            }
            name => Err(format!("unsupported system control setter `{name}`")),
        }
    }
}

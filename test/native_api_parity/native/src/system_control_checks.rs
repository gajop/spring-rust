use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_system_control_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
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
}

use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_input_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
            "get_mod_key_state" => {
                let native = self
                    .interface
                    .input()
                    .get_mod_key_state()
                    .map_err(|err| format!("get_mod_key_state() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "alt", native & (1 << 2) != 0)?;
                self.same_bool_if_present(label, message, "ctrl", native & (1 << 1) != 0)?;
                self.same_bool_if_present(label, message, "meta", native & (1 << 3) != 0)?;
                self.same_bool_if_present(label, message, "shift", native & (1 << 0) != 0)
            }
            "get_invert_queue_key" => {
                let native = self
                    .interface
                    .input()
                    .get_invert_queue_key()
                    .map_err(|err| format!("get_invert_queue_key() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "invert", native)
            }
            "get_mouse_state" => {
                let native = self
                    .interface
                    .input()
                    .get_mouse_state()
                    .map_err(|err| format!("get_mouse_state() failed: {err:?}"))?;
                self.same_if_present(label, message, "x", native.x)?;
                self.same_if_present(label, message, "y", native.y)?;
                self.same_bool_if_present(label, message, "left", native.left)?;
                self.same_bool_if_present(label, message, "middle", native.middle)?;
                self.same_bool_if_present(label, message, "right", native.right)?;
                self.same_bool_if_present(label, message, "offscreen", native.offscreen)
            }
            "get_mouse_cursor" => {
                let native = self
                    .interface
                    .input()
                    .get_mouse_cursor()
                    .map_err(|err| format!("get_mouse_cursor() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "cursor",
                    native.as_deref().unwrap_or(""),
                )
            }
            "get_mouse_buttons_pressed" => {
                let native = self
                    .interface
                    .input()
                    .get_mouse_buttons_pressed(&[1, 2, 3])
                    .map_err(|err| format!("get_mouse_buttons_pressed() failed: {err:?}"))?;
                if native.len() != 3 {
                    return Err(format!(
                        "{label}: expected 3 button states, got {}",
                        native.len()
                    ));
                }
                self.same_bool_if_present(label, message, "left", native[0])?;
                self.same_bool_if_present(label, message, "middle", native[1])?;
                self.same_bool_if_present(label, message, "right", native[2])
            }
            "is_above_mini_map" => {
                let screen_x = f32_field(message, "screenX")?;
                let screen_y = f32_field(message, "screenY")?;
                let native = self
                    .interface
                    .input()
                    .is_above_mini_map(screen_x, screen_y)
                    .map_err(|err| {
                        format!("is_above_mini_map({screen_x}, {screen_y}) failed: {err:?}")
                    })?;
                self.same_bool_if_present(label, message, "above", native)
            }
            "get_key_code" => {
                let key_sym = str_field(message, "keySym")?;
                let native = self
                    .interface
                    .input()
                    .get_key_code(key_sym)
                    .map_err(|err| format!("get_key_code({key_sym}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "keyCode", native)
            }
            "get_key_symbol" => {
                let key_code = i32_field(message, "keyCode")?;
                let (name, default_name) = self
                    .interface
                    .input()
                    .get_key_symbol(key_code)
                    .map_err(|err| format!("get_key_symbol({key_code}) failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "keyCodeName",
                    name.as_deref().unwrap_or(""),
                )?;
                self.same_string_if_present(
                    label,
                    message,
                    "keyCodeDefaultName",
                    default_name.as_deref().unwrap_or(""),
                )
            }
            "get_scan_symbol" => {
                let scan_code = i32_field(message, "scanCode")?;
                let (name, default_name) = self
                    .interface
                    .input()
                    .get_scan_symbol(scan_code)
                    .map_err(|err| format!("get_scan_symbol({scan_code}) failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "scanCodeName",
                    name.as_deref().unwrap_or(""),
                )?;
                self.same_string_if_present(
                    label,
                    message,
                    "scanCodeDefaultName",
                    default_name.as_deref().unwrap_or(""),
                )
            }
            "get_key_from_scan_symbol" => {
                let scan_symbol = str_field(message, "scanSymbol")?;
                let native = self
                    .interface
                    .input()
                    .get_key_from_scan_symbol(scan_symbol)
                    .map_err(|err| {
                        format!("get_key_from_scan_symbol({scan_symbol}) failed: {err:?}")
                    })?;
                self.same_string_if_present(
                    label,
                    message,
                    "keyName",
                    native.as_deref().unwrap_or(""),
                )
            }
            "get_key_state" => {
                let key_code = i32_field(message, "keyCode")?;
                let native = self
                    .interface
                    .input()
                    .get_key_state(key_code)
                    .map_err(|err| format!("get_key_state({key_code}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "pressed", native)
            }
            "get_key_bindings_count" => {
                let key_set1 = str_field(message, "keySet1")?;
                let key_set2 = str_field(message, "keySet2")?;
                let native = self
                    .interface
                    .input()
                    .get_key_bindings(key_set1, key_set2)
                    .map_err(|err| {
                        format!("get_key_bindings({key_set1}, {key_set2}) failed: {err:?}")
                    })?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_pressed_keys" => {
                let native = self
                    .interface
                    .input()
                    .get_pressed_keys()
                    .map_err(|err| format!("get_pressed_keys() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "keyCodes", &native)
            }
            "get_pressed_scans" => {
                let native = self
                    .interface
                    .input()
                    .get_pressed_scans()
                    .map_err(|err| format!("get_pressed_scans() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "scanCodes", &native)
            }
            "get_active_page" => {
                let (active_page, max_page) = self
                    .interface
                    .input()
                    .get_active_page()
                    .map_err(|err| format!("get_active_page() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "activePage", active_page)?;
                self.same_i32_if_present(label, message, "maxPage", max_page)
            }
            "get_active_command" => {
                let (command_index, _command_id, _command_type, _command_name) = self
                    .interface
                    .input()
                    .get_active_command()
                    .map_err(|err| format!("get_active_command() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "commandIndex", command_index)
            }
            "get_default_command" => {
                let (command_index, _command_id, _command_type, _command_name) = self
                    .interface
                    .input()
                    .get_default_command()
                    .map_err(|err| format!("get_default_command() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "commandIndex", command_index)
            }
            "get_mouse_start_position" => {
                let button = i32_field(message, "button")?;
                let native = self
                    .interface
                    .input()
                    .get_mouse_start_position(button)
                    .map_err(|err| format!("get_mouse_start_position({button}) failed: {err:?}"))?;
                self.same_if_present(label, message, "x", native.0.x)?;
                self.same_if_present(label, message, "y", native.0.y)?;
                self.same_if_present(label, message, "camPosX", native.1.x)?;
                self.same_if_present(label, message, "camPosY", native.1.y)?;
                self.same_if_present(label, message, "camPosZ", native.1.z)?;
                self.same_if_present(label, message, "dirX", native.2.x)?;
                self.same_if_present(label, message, "dirY", native.2.y)?;
                self.same_if_present(label, message, "dirZ", native.2.z)
            }
            "get_selection_box" => {
                let native = self
                    .interface
                    .input()
                    .get_selection_box()
                    .map_err(|err| format!("get_selection_box() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "active", native.active)
            }
            "get_action_hot_keys" => {
                let action = str_field(message, "action")?;
                let native = self
                    .interface
                    .input()
                    .get_action_hot_keys(action)
                    .map_err(|err| format!("get_action_hot_keys({action}) failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "hotkeys", &native)
            }
            _ => Err(format!("unsupported input check `{label}`")),
        }
    }
}

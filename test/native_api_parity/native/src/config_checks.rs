use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_config_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        match test_name {
            "get_config_int" | "get_config_int_nil_default" | "config_int" => {
                let key = str_field(message, "key")?;
                let default_value = i32_field(message, "defaultValue")?;
                let has_default = bool_field(message, "hasDefault")?;
                let (native, exists) = self
                    .interface
                    .config()
                    .get_config_int(key, has_default.then_some(default_value))
                    .map_err(|err| format!("get_config_int({key}, {default_value}, {has_default}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "exists", exists)?;
                self.same_i32_if_present(label, message, "value", native)
            }
            "get_config_float" | "get_config_float_nil_default" | "config_float" => {
                let key = str_field(message, "key")?;
                let default_value = f32_field(message, "defaultValue")?;
                let has_default = bool_field(message, "hasDefault")?;
                let (native, exists) = self
                    .interface
                    .config()
                    .get_config_float(key, has_default.then_some(default_value))
                    .map_err(|err| format!("get_config_float({key}, {default_value}, {has_default}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "exists", exists)?;
                self.same_if_present(label, message, "value", native)
            }
            "get_config_string" | "get_config_string_nil_default" | "config_string" => {
                let key = str_field(message, "key")?;
                let default_value = str_field(message, "defaultValue")?;
                let has_default = bool_field(message, "hasDefault")?;
                let (native, exists) = self
                    .interface
                    .config()
                    .get_config_string(key, has_default.then_some(default_value))
                    .map_err(|err| format!("get_config_string({key}, {default_value}, {has_default}) failed: {err:?}"))?;
                let native = native
                    .unwrap_or_default();
                self.same_bool_if_present(label, message, "exists", exists)?;
                self.same_string_if_present(label, message, "value", &native)
            }
            "get_config_params" => {
                let native = self
                    .interface
                    .config()
                    .get_config_params()
                    .map_err(|err| format!("get_config_params() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_log_sections" => {
                let native = self
                    .interface
                    .config()
                    .get_log_sections()
                    .map_err(|err| format!("get_log_sections() failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "keys", &native)
            }
            _ => Err(format!("unsupported config check `{label}`")),
        }
    }

    pub(crate) fn set_config_value(&mut self, message: &Value) -> Result<(), String> {
        let key = str_field(message, "key")?;
        let use_overlay = bool_field(message, "useOverlay")?;
        match base_test_name(str_field(message, "name")?) {
            "config_int" => {
                let value = i32_field(message, "value")?;
                self.interface.config().set_config_int(key, value, use_overlay)
                    .map_err(|err| format!("set_config_int({key}) failed: {err:?}"))?;
            }
            "config_float" => {
                let value = f32_field(message, "value")?;
                self.interface.config().set_config_float(key, value, use_overlay)
                    .map_err(|err| format!("set_config_float({key}) failed: {err:?}"))?;
            }
            "config_string" => {
                let value = str_field(message, "value")?;
                self.interface.config().set_config_string(key, value, use_overlay)
                    .map_err(|err| format!("set_config_string({key}) failed: {err:?}"))?;
            }
            name => return Err(format!("unsupported config setter `{name}`")),
        }
        Ok(())
    }
}

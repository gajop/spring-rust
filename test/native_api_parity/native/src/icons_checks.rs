use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_icons_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
            "get_all_icon_data_array_count" => {
                let full_data = bool_field(message, "fullData")?;
                let native = self
                    .interface
                    .icons()
                    .get_all_icon_data_array(full_data)
                    .map_err(|err| {
                        format!("get_all_icon_data_array({full_data}) failed: {err:?}")
                    })?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_icon_data_default" => {
                let icon_name = str_field(message, "iconName")?;
                let full_data = bool_field(message, "fullData")?;
                let native = self
                    .interface
                    .icons()
                    .get_icon_data(icon_name, full_data)
                    .map_err(|err| {
                        format!("get_icon_data({icon_name}, {full_data}) failed: {err:?}")
                    })?;
                self.same_string_if_present(
                    label,
                    message,
                    "iconName",
                    &cstr_or_empty(native.name)?,
                )
            }
            "unit_icon_get_draw" | "unit_icon_draw" | "unit_icon_draw_deprecated_alias" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .icons()
                    .unit_icon_get_draw(unit_id)
                    .map_err(|err| format!("unit_icon_get_draw({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "drawIcon", native)
            }
            _ => Err(format!("unsupported icons check `{label}`")),
        }
    }

    pub(crate) fn set_icons_value(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(str_field(message, "name")?) {
            "unit_icon_draw" | "unit_icon_draw_deprecated_alias" => {
                let unit_id = i32_field(message, "unitID")?;
                let draw_icon = bool_field(message, "drawIcon")?;
                let success = self
                    .interface
                    .icons()
                    .unit_icon_set_draw(unit_id, draw_icon)
                    .map_err(|err| {
                        format!("unit_icon_set_draw({unit_id}, {draw_icon}) failed: {err:?}")
                    })?;
                if success {
                    Ok(())
                } else {
                    Err(format!(
                        "unit_icon_set_draw({unit_id}, {draw_icon}) returned false"
                    ))
                }
            }
            name => Err(format!("unsupported icons setter `{name}`")),
        }
    }
}

use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_selection_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
            "get_selected_units"
            | "select_unit"
            | "deselect_unit"
            | "select_unit_array"
            | "deselect_unit_array"
            | "select_unit_map"
            | "deselect_unit_map" => {
                let native = self.interface.selection().get_selected_units()
                    .map_err(|err| format!("get_selected_units() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "unitIDs", &native)
            }
            "get_selected_units_count" => {
                let native = self.interface.selection().get_selected_units_count()
                    .map_err(|err| format!("get_selected_units_count() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native as i32)
            }
            "get_selected_units_sorted" => {
                let native = self.interface.selection().get_selected_units_sorted()
                    .map_err(|err| format!("get_selected_units_sorted() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "unitIDs", &native)
            }
            "get_selected_units_counts" => {
                let native = self.interface.selection().get_selected_units_counts()
                    .map_err(|err| format!("get_selected_units_counts() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "defCount", native.uniqueCount as i32)
            }
            "get_selected_group" => {
                let native = self.interface.selection().get_selected_group()
                    .map_err(|err| format!("get_selected_group() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "groupID", native)
            }
            "get_group_list" => {
                let native = self.interface.selection().get_group_list()
                    .map_err(|err| format!("get_group_list() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "unit_group" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self.interface.selection().get_unit_group(unit_id)
                    .map_err(|err| format!("get_unit_group({unit_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "groupID", native)
            }
            "get_unit_group" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self.interface.selection().get_unit_group(unit_id)
                    .map_err(|err| format!("get_unit_group({unit_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "groupID", native)
            }
            "get_group_units" => {
                let group_id = i32_field(message, "groupID")?;
                let native = self.interface.selection().get_group_units(group_id)
                    .map_err(|err| format!("get_group_units({group_id}) failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "unitIDs", &native)
            }
            "get_group_units_count" => {
                let group_id = i32_field(message, "groupID")?;
                let native = self.interface.selection().get_group_units_count(group_id)
                    .map_err(|err| format!("get_group_units_count({group_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native as i32)
            }
            "get_group_units_counts" => {
                let group_id = i32_field(message, "groupID")?;
                let native = self.interface.selection().get_group_units_counts(group_id)
                    .map_err(|err| format!("get_group_units_counts({group_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "defCount", native.uniqueCount as i32)
            }
            "get_group_units_sorted_fixed" => {
                let group_id = i32_field(message, "groupID")?;
                let native = self.interface.selection().get_group_units_sorted(group_id)
                    .map_err(|err| format!("get_group_units_sorted({group_id}) failed: {err:?}"))?;
                self.same_team_units_by_def_if_present(label, message, "groups", &native)
            }
            _ => Err(format!("unsupported selection check `{label}`")),
        }
    }

    pub(crate) fn set_selection_value(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(str_field(message, "name")?) {
            "select_unit" => {
                let unit_id = i32_field(message, "unitID")?;
                let append = bool_field(message, "append")?;
                let success = self.interface.selection().select_unit(unit_id, append)
                    .map_err(|err| format!("select_unit({unit_id}, {append}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err(format!("select_unit({unit_id}, {append}) returned false"))
                }
            }
            "deselect_unit" => {
                let unit_id = i32_field(message, "unitID")?;
                let append = bool_field(message, "append")?;
                let selected = self.interface.selection().select_unit(unit_id, append)
                    .map_err(|err| format!("select_unit({unit_id}, {append}) failed: {err:?}"))?;
                let deselected = self.interface.selection().deselect_unit(unit_id)
                    .map_err(|err| format!("deselect_unit({unit_id}) failed: {err:?}"))?;
                if selected && deselected {
                    Ok(())
                } else {
                    Err(format!("select_unit/deselect_unit({unit_id}) returned selected={selected}, deselected={deselected}"))
                }
            }
            "select_unit_array" => {
                let unit_id = i32_field(message, "unitID")?;
                let append = bool_field(message, "append")?;
                let success = self.interface.selection().select_unit_array(&[unit_id], append)
                    .map_err(|err| format!("select_unit_array([{unit_id}], {append}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err(format!("select_unit_array([{unit_id}], {append}) returned false"))
                }
            }
            "deselect_unit_array" => {
                let unit_id = i32_field(message, "unitID")?;
                let append = bool_field(message, "append")?;
                let selected = self.interface.selection().select_unit_array(&[unit_id], append)
                    .map_err(|err| format!("select_unit_array([{unit_id}], {append}) failed: {err:?}"))?;
                let deselected = self.interface.selection().deselect_unit_array(&[unit_id])
                    .map_err(|err| format!("deselect_unit_array([{unit_id}]) failed: {err:?}"))?;
                if selected && deselected {
                    Ok(())
                } else {
                    Err(format!("select_unit_array/deselect_unit_array([{unit_id}]) returned selected={selected}, deselected={deselected}"))
                }
            }
            "select_unit_map" => {
                let unit_id = i32_field(message, "unitID")?;
                let append = bool_field(message, "append")?;
                let success = self.interface.unsynced_ctrl().select_unit_map(&[unit_id], append)
                    .map_err(|err| format!("select_unit_map([{unit_id}], {append}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err(format!("select_unit_map([{unit_id}], {append}) returned false"))
                }
            }
            "deselect_unit_map" => {
                let unit_id = i32_field(message, "unitID")?;
                let append = bool_field(message, "append")?;
                let selected = self.interface.unsynced_ctrl().select_unit_map(&[unit_id], append)
                    .map_err(|err| format!("select_unit_map([{unit_id}], {append}) failed: {err:?}"))?;
                let deselected = self.interface.unsynced_ctrl().deselect_unit_map(&[unit_id])
                    .map_err(|err| format!("deselect_unit_map([{unit_id}]) failed: {err:?}"))?;
                if selected && deselected {
                    Ok(())
                } else {
                    Err(format!("select_unit_map/deselect_unit_map([{unit_id}]) returned selected={selected}, deselected={deselected}"))
                }
            }
            "unit_group" | "get_group_units" | "get_group_units_count" | "get_group_units_counts" | "get_group_units_sorted_fixed" => {
                let unit_id = i32_field(message, "unitID")?;
                let group_id = i32_field(message, "groupID")?;
                let success = self.interface.selection().set_unit_group(unit_id, group_id)
                    .map_err(|err| format!("set_unit_group({unit_id}, {group_id}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err(format!("set_unit_group({unit_id}, {group_id}) returned false"))
                }
            }
            name => Err(format!("unsupported selection setter `{name}`")),
        }
    }
}

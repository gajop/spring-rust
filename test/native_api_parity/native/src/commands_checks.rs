use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_factory_read(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        // The focused factory getter is invoked synchronously by the native
        // setter transport before its temporary factory is destroyed.  Its
        // emitted result row intentionally carries only the six comparable
        // Lua values, so there is no unitID to query again here.
        if base_test_name(label) == "get_factory_bugger_off"
            && bool_field(message, "nativeGetterChecked").unwrap_or(false)
        {
            return Ok(());
        }

        let unit_id = i32_field(message, "unitID")?;
        match base_test_name(label) {
            "get_factory_command_count" => {
                let native = self
                    .interface
                    .units_commands()
                    .get_factory_command_count(unit_id)
                    .map_err(|err| {
                        format!("get_factory_command_count({unit_id}) failed: {err:?}")
                    })?;
                self.same_i32_if_present(label, message, "count", native as i32)
            }
            "get_factory_commands" => {
                let native = self
                    .interface
                    .units_commands()
                    .get_factory_commands(unit_id, u32_field(message, "maxCommands")?)
                    .map_err(|err| format!("get_factory_commands({unit_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_factory_counts" => {
                let native = self
                    .interface
                    .units_commands()
                    .get_factory_counts(
                        unit_id,
                        i32_field(message, "count")?,
                        bool_field(message, "addCmds")?,
                    )
                    .map_err(|err| format!("get_factory_counts({unit_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "uniqueCount", native.uniqueCount as i32)?;
                self.same_i32_if_present(label, message, "totalCount", native.totalCount as i32)
            }
            "get_factory_bugger_off" => {
                let (perform, offset, radius, rel_heading, spherical, forced) = self
                    .interface
                    .units_commands()
                    .get_factory_bugger_off(unit_id)
                    .map_err(|err| format!("get_factory_bugger_off({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "perform", perform)?;
                self.same_if_present(label, message, "offset", offset)?;
                self.same_if_present(label, message, "radius", radius)?;
                self.same_i32_if_present(label, message, "relHeading", rel_heading)?;
                self.same_bool_if_present(label, message, "spherical", spherical)?;
                self.same_bool_if_present(label, message, "forced", forced)
            }
            "get_full_build_queue" => {
                let native = self
                    .interface
                    .units_commands()
                    .get_full_build_queue(unit_id)
                    .map_err(|err| format!("get_full_build_queue({unit_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_real_build_queue" => {
                let native = self
                    .interface
                    .units_commands()
                    .get_real_build_queue(unit_id)
                    .map_err(|err| format!("get_real_build_queue({unit_id}) failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "unitDefIDs", &native)
            }
            _ => Err(format!("unsupported factory read check `{label}`")),
        }
    }

    pub(crate) fn check_unit_commands_count(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_commands()
            .get_unit_command_count(unit_id)
            .map_err(|err| format!("get_unit_command_count({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "count", native as i32)
    }

    pub(crate) fn check_unit_commands_table_count(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let native = match test_name {
            "get_unit_commands" => {
                let max_commands = u32_field(message, "maxCommands")?;
                self.interface
                    .units_commands()
                    .get_unit_commands(unit_id, max_commands)
                    .map_err(|err| {
                        format!("get_unit_commands({unit_id}, {max_commands}) failed: {err:?}")
                    })?
                    .len()
            }
            "get_command_queue" => {
                let max_commands = u32_field(message, "maxCommands")?;
                self.interface
                    .units_commands()
                    .get_command_queue(unit_id, max_commands)
                    .map_err(|err| {
                        format!("get_command_queue({unit_id}, {max_commands}) failed: {err:?}")
                    })?
                    .len()
            }
            "get_unit_cmd_descs" => self
                .interface
                .units_commands()
                .get_unit_cmd_descs(unit_id)
                .map_err(|err| format!("get_unit_cmd_descs({unit_id}) failed: {err:?}"))?
                .len(),
            _ => {
                return Err(format!(
                    "unsupported unit commands table count check `{label}`"
                ))
            }
        };
        self.same_i32_if_present(label, message, "count", native as i32)
    }

    pub(crate) fn check_unit_current_command_presence(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let cmd_index = i32_field(message, "cmdIndex")?;
        let (_, has_command) = self
            .interface
            .units_commands()
            .get_unit_current_command(unit_id, cmd_index)
            .map_err(|err| {
                format!("get_unit_current_command({unit_id}, {cmd_index}) failed: {err:?}")
            })?;
        self.same_bool_if_present(label, message, "hasCommand", has_command)
    }

    pub(crate) fn check_find_unit_cmd_desc_missing(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let cmd_id = i32_field(message, "cmdID")?;
        let (cmd_index, found) = self
            .interface
            .units_commands()
            .find_unit_cmd_desc(unit_id, cmd_id)
            .map_err(|err| format!("find_unit_cmd_desc({unit_id}, {cmd_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "found", found)?;
        if !found {
            self.same_i32_if_present(label, message, "cmdDescIndex", cmd_index)
        } else {
            Ok(())
        }
    }
}

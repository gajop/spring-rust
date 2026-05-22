use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_unit_commands_count(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_commands()
            .get_unit_command_count(unit_id)
            .map_err(|err| format!("get_unit_command_count({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "count", native as i32)
    }

    pub(crate) fn check_unit_commands_table_count(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let native = match test_name {
            "get_unit_commands" => {
                let max_commands = u32_field(message, "maxCommands")?;
                self.interface.units_commands().get_unit_commands(unit_id, max_commands)
                    .map_err(|err| format!("get_unit_commands({unit_id}, {max_commands}) failed: {err:?}"))?.len()
            }
            "get_command_queue" => {
                let max_commands = u32_field(message, "maxCommands")?;
                self.interface.units_commands().get_command_queue(unit_id, max_commands)
                    .map_err(|err| format!("get_command_queue({unit_id}, {max_commands}) failed: {err:?}"))?.len()
            }
            "get_unit_cmd_descs" => self
                .interface
                .units_commands()
                .get_unit_cmd_descs(unit_id)
                .map_err(|err| format!("get_unit_cmd_descs({unit_id}) failed: {err:?}"))?
                .len(),
            _ => return Err(format!("unsupported unit commands table count check `{label}`")),
        };
        self.same_i32_if_present(label, message, "count", native as i32)
    }

    pub(crate) fn check_unit_current_command_presence(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let cmd_index = i32_field(message, "cmdIndex")?;
        let (_, has_command) = self
            .interface
            .units_commands()
            .get_unit_current_command(unit_id, cmd_index)
            .map_err(|err| format!("get_unit_current_command({unit_id}, {cmd_index}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "hasCommand", has_command)
    }

}

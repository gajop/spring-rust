use super::*;
use crate::support::*;
use std::ffi::CString;

fn command_description(
    id: i32,
    command_type: i32,
    action: Option<&CString>,
) -> spring_native::sys::NativeCommandDescription {
    spring_native::sys::NativeCommandDescription {
        id,
        type_: command_type,
        queueing: false,
        hidden: false,
        disabled: false,
        showUnique: false,
        onlyTexture: false,
        name: std::ptr::null(),
        action: action.map_or(std::ptr::null(), |value| value.as_ptr()),
        iconname: std::ptr::null(),
        mouseicon: std::ptr::null(),
        tooltip: std::ptr::null(),
        params: std::ptr::null_mut(),
        paramCount: 0,
    }
}

impl NativeApiParity {
    pub(crate) fn check_unit_cmd_desc_lifecycle(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let command_id = i32_field(message, "cmdID")?;
        let descriptions = self
            .interface
            .units_commands()
            .get_unit_command_descriptions(unit_id)
            .map_err(|err| format!("get_unit_command_descriptions() failed: {err:?}"))?;
        if descriptions
            .iter()
            .any(|description| description.id == command_id)
        {
            return Err(format!(
                "{label}: command description {command_id} remained after lifecycle"
            ));
        }
        self.same_bool_if_present(label, message, "success", true)
    }

    pub(crate) fn set_unit_cmd_desc_lifecycle(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let command_id = i32_field(message, "cmdID")?;
        let command_type = i32_field(message, "cmdType")?;
        let action = CString::new(str_field(message, "action")?)
            .map_err(|_| "action contains an embedded NUL".to_string())?;
        let edited_action = CString::new(str_field(message, "editedAction")?)
            .map_err(|_| "editedAction contains an embedded NUL".to_string())?;
        let synced_ctrl = self.interface.synced_ctrl();
        let unit = synced_ctrl.unit();
        let commands = self.interface.units_commands();
        let before = commands
            .get_unit_command_descriptions(unit_id)
            .map_err(|err| format!("get_unit_command_descriptions() failed: {err:?}"))?
            .len();
        let insert_description = command_description(command_id, command_type, Some(&action));
        if !unit
            .insert_unit_cmd_desc(unit_id, -1, &insert_description)
            .map_err(|err| format!("insert_unit_cmd_desc() failed: {err:?}"))?
        {
            return Err("insert_unit_cmd_desc() returned false".to_string());
        }

        let descriptions = commands
            .get_unit_command_descriptions(unit_id)
            .map_err(|err| format!("get_unit_command_descriptions() failed: {err:?}"))?;
        let index = descriptions
            .iter()
            .position(|description| description.id == command_id)
            .ok_or_else(|| "inserted command description was not found".to_string())?;

        let edit_description = command_description(command_id, command_type, Some(&edited_action));
        if !unit
            .edit_unit_cmd_desc(unit_id, index as u32, &edit_description)
            .map_err(|err| format!("edit_unit_cmd_desc() failed: {err:?}"))?
        {
            return Err("edit_unit_cmd_desc() returned false".to_string());
        }
        let edited = commands
            .get_unit_command_descriptions(unit_id)
            .map_err(|err| format!("get_unit_command_descriptions() failed: {err:?}"))?
            .iter()
            .any(|description| {
                description.id == command_id
                    && description.action == edited_action.to_string_lossy()
            });
        if !edited {
            return Err("edited command description did not expose the new action".to_string());
        }

        if !unit
            .remove_unit_cmd_desc(unit_id, index as i32)
            .map_err(|err| format!("remove_unit_cmd_desc() failed: {err:?}"))?
        {
            return Err("remove_unit_cmd_desc() returned false".to_string());
        }
        let after_descriptions = commands
            .get_unit_command_descriptions(unit_id)
            .map_err(|err| format!("get_unit_command_descriptions() failed: {err:?}"))?;
        if before != after_descriptions.len()
            || after_descriptions
                .iter()
                .any(|description| description.id == command_id)
        {
            return Err(
                "command description lifecycle did not restore the command list".to_string(),
            );
        }
        Ok(())
    }
}

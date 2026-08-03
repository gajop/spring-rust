use super::*;
use crate::support::*;

fn native_command() -> sys::NativeCommand {
    sys::NativeCommand {
        cmdID: 0,
        params: std::ptr::null_mut(),
        paramCount: 0,
        options: 0,
        timeout: 0,
    }
}

fn command_ffi() -> sys::CommandFFI {
    sys::CommandFFI {
        cmdID: 0,
        options: 0,
        tag: 0,
        aiCommandID: 0,
        timeOut: 0.0,
        params: std::ptr::null_mut(),
        paramCount: 0,
    }
}

impl NativeApiParity {
    fn run_order_variant(&self, name: &str, unit_id: i32) -> Result<bool, String> {
        let params: [f32; 0] = [];
        let unit_ids = [unit_id];
        let native_commands = [native_command()];
        let ffi_commands = [command_ffi()];

        match name {
            "give_order_selected" => self
                .interface
                .units_commands()
                .give_order(0, &params, 0, 0)
                .map_err(|err| format!("give_order() failed: {err:?}")),
            "give_order_to_unit_unsynced"
            | "give_order_to_unit_array_unsynced"
            | "give_order_to_unit_map_unsynced" => self
                .interface
                .units_commands()
                .give_order_to_unit_map(&unit_ids, 0, &params, 0, 0)
                .map(|count| count > 0)
                .map_err(|err| format!("give_order_to_unit_map() failed: {err:?}")),
            "give_order_array_to_unit_unsynced"
            | "give_order_array_to_unit_map_unsynced"
            | "give_order_array_to_unit_array_unsynced" => self
                .interface
                .units_commands()
                .give_order_array_to_unit_map(&unit_ids, &ffi_commands)
                .map(|count| count > 0)
                .map_err(|err| format!("give_order_array_to_unit_map() failed: {err:?}")),
            "give_order_to_unit_synced" => self
                .interface
                .synced_ctrl()
                .unit()
                .give_order_to_unit(unit_id, 0, &params, 0, 0)
                .map_err(|err| format!("give_order_to_unit() failed: {err:?}")),
            "give_order_to_unit_array_synced" => self
                .interface
                .synced_ctrl()
                .unit()
                .give_order_to_unit_array(&unit_ids, 0, &params, 0, 0)
                .map_err(|err| format!("give_order_to_unit_array() failed: {err:?}")),
            "give_order_array_to_unit_synced" => self
                .interface
                .synced_ctrl()
                .unit()
                .give_order_array_to_unit(unit_id, &native_commands)
                .map_err(|err| format!("give_order_array_to_unit() failed: {err:?}")),
            "give_order_to_unit_map_synced"
            | "give_order_array_to_unit_map_synced"
            | "give_order_array_to_unit_array_synced_pairwise"
            | "give_order_array_to_unit_array_synced_broadcast" => {
                let pairwise = name.ends_with("_pairwise");
                self.interface
                    .synced_ctrl()
                    .unit()
                    .give_order_array_to_unit_array(&unit_ids, &native_commands, pairwise)
                    .map(|count| count > 0)
                    .map_err(|err| {
                        format!("give_order_array_to_unit_array() failed: {err:?}")
                    })
            }
            _ => Err(format!("unsupported order variant `{name}`")),
        }
    }

    pub(crate) fn check_order_variant(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let name = base_test_name(label);
        let unit_id = i32_field(message, "unitID").unwrap_or_default();
        let native = self.run_order_variant(name, unit_id)?;
        self.same_bool_if_present(label, message, "success", native)
    }

    pub(crate) fn set_order_variant(&mut self, message: &Value) -> Result<(), String> {
        let test_name = message
            .get("testName")
            .and_then(Value::as_str)
            .ok_or_else(|| "missing string field `testName`".to_string())?;
        let name = test_name
            .strip_prefix("set_native_")
            .unwrap_or(test_name);
        let unit_id = i32_field(message, "unitID").unwrap_or_default();
        if !self.run_order_variant(name, unit_id)? {
            return Err(format!("native order variant `{name}` returned false"));
        }
        Ok(())
    }
}

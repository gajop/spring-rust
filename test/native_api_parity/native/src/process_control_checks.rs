use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_process_control(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "quit" => {
                self.same_bool_if_present(label, message, "called", true)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "reload" | "restart" => {
                self.same_bool_if_present(label, message, "called", true)?;
                self.same_i32_if_present(label, message, "returnCount", 0)?;
                message
                    .get("reloaded")
                    .and_then(Value::as_bool)
                    .map(|_| ())
                    .ok_or_else(|| "missing boolean field `reloaded`".to_string())
            }
            "start" => {
                self.same_bool_if_present(label, message, "called", true)?;
                self.same_i32_if_present(label, message, "returnCount", 1)?;
                self.same_bool_if_present(label, message, "result", false)
            }
            _ => Err(format!("unsupported process-control check `{label}`")),
        }
    }

    pub(crate) fn set_process_control(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(test_name_field(message)?) {
            "quit" => {
                let success = self
                    .interface
                    .system_control()
                    .quit()
                    .map_err(|err| format!("quit failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("quit returned false".to_string())
                }
            }
            "reload" => {
                let script = str_field(message, "startScript")?;
                let success = self
                    .interface
                    .system_control()
                    .reload(script)
                    .map_err(|err| format!("reload failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("reload returned false".to_string())
                }
            }
            "restart" => {
                let args = str_field(message, "cmdArgs")?;
                let script = str_field(message, "startScript")?;
                let success = self
                    .interface
                    .system_control()
                    .restart(args, script)
                    .map_err(|err| format!("restart failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("restart returned false".to_string())
                }
            }
            "start" => {
                let args = str_field(message, "cmdArgs")?;
                let script = str_field(message, "startScript")?;
                match self
                    .interface
                    .system_control()
                    .start(args, script)
                {
                    Ok(success) => Err(format!(
                        "start unexpectedly returned Ok({success}); Lua should return false"
                    )),
                    Err(_) => Ok(()),
                }
            }
            name => Err(format!("unsupported process-control setter `{name}`")),
        }
    }
}

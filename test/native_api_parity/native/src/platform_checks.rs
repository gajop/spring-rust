use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_platform_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "platform_architecture" => {
                let native = self
                    .interface
                    .platform()
                    .get_architecture()
                    .map_err(|err| format!("get_architecture() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "architecture",
                    native.as_deref().unwrap_or(""),
                )
            }
            "platform_is_headless" => {
                let native = self
                    .interface
                    .platform()
                    .is_headless()
                    .map_err(|err| format!("is_headless() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "isHeadless", native)
            }
            _ => Err(format!("unsupported platform check `{label}`")),
        }
    }
}

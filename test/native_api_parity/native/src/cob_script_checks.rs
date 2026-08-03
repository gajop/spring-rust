use super::*;
use crate::support::*;
use std::ffi::CString;

impl NativeApiParity {
    pub(crate) fn check_cob_script(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let func_name = CString::new(str_field(message, "funcName")?)
            .map_err(|_| "funcName contains an embedded NUL".to_string())?;

        match base_test_name(label) {
            "get_cobscript_id_non_cob" => {
                let native = self
                    .interface
                    .synced_ctrl()
                    .cob_script()
                    .get_cobscript_id(unit_id, func_name.to_str().unwrap())
                    .map_err(|err| format!("get_cobscript_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "funcID", native)
            }
            "call_cobscript_non_cob" => {
                let result = self.interface.synced_ctrl().cob_script().call_cobscript(
                    unit_id,
                    sys::CobFunctionRef {
                        name: func_name.as_ptr(),
                        id: -1,
                    },
                    u32_field(message, "retArgs")?,
                    &[],
                );
                if result.is_ok() {
                    return Err("call_cobscript() unexpectedly accepted a non-COB unit".to_string());
                }
                self.same_bool_if_present(label, message, "error", true)
            }
            _ => Err(format!("unsupported COB script check `{label}`")),
        }
    }
}

use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_metal_map(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        match test_name {
            "get_metal_map_size" => {
                let (width, height) = self
                    .interface
                    .metal_map()
                    .get_metal_map_size()
                    .map_err(|err| format!("get_metal_map_size() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "width", width)?;
                self.same_i32_if_present(label, message, "height", height)
            }
            "get_metal_amount" | "metal_amount" => {
                let x = i32_field(message, "x")?;
                let z = i32_field(message, "z")?;
                let amount = self
                    .interface
                    .metal_map()
                    .get_metal_amount(x, z)
                    .map_err(|err| format!("get_metal_amount({x}, {z}) failed: {err:?}"))?;
                self.same_if_present(label, message, "amount", amount)
            }
            "get_metal_extraction" => {
                let x = i32_field(message, "x")?;
                let z = i32_field(message, "z")?;
                let extraction = self
                    .interface
                    .metal_map()
                    .get_metal_extraction(x, z)
                    .map_err(|err| format!("get_metal_extraction({x}, {z}) failed: {err:?}"))?;
                self.same_if_present(label, message, "extraction", extraction)
            }
            _ => Err(format!("unsupported metal map check `{label}`")),
        }
    }
    pub(crate) fn set_metal_amount(&mut self, message: &Value) -> Result<(), String> {
        let x = i32_field(message, "x")?;
        let z = i32_field(message, "z")?;
        self.interface
            .metal_map()
            .set_metal_amount(x, z, f32_field(message, "amount")?)
            .map_err(|err| format!("set_metal_amount({x}, {z}) failed: {err:?}"))?;
        Ok(())
    }
}

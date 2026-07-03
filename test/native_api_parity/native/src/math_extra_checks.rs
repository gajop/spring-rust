use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_math_extra_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "math_hypot" => {
                let native = self
                    .interface
                    .math_extra()
                    .hypot(f32_field(message, "x")?, f32_field(message, "y")?)
                    .map_err(|err| format!("hypot() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_diag" => {
                let values = [
                    f32_field(message, "x")?,
                    f32_field(message, "y")?,
                    f32_field(message, "z")?,
                ];
                let native = self
                    .interface
                    .math_extra()
                    .diag(&values)
                    .map_err(|err| format!("diag() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_clamp" => {
                let native = self
                    .interface
                    .math_extra()
                    .clamp(
                        f32_field(message, "valueIn")?,
                        f32_field(message, "min")?,
                        f32_field(message, "max")?,
                    )
                    .map_err(|err| format!("clamp() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_sgn" => {
                let native = self
                    .interface
                    .math_extra()
                    .sgn(f32_field(message, "valueIn")?)
                    .map_err(|err| format!("sgn() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_mix" => {
                let native = self
                    .interface
                    .math_extra()
                    .mix(
                        f32_field(message, "a")?,
                        f32_field(message, "b")?,
                        f32_field(message, "t")?,
                    )
                    .map_err(|err| format!("mix() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_round" => {
                let native = self
                    .interface
                    .math_extra()
                    .round(f32_field(message, "valueIn")?)
                    .map_err(|err| format!("round() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_erf" => {
                let native = self
                    .interface
                    .math_extra()
                    .erf(f32_field(message, "valueIn")?)
                    .map_err(|err| format!("erf() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_smooth_step" => {
                let native = self
                    .interface
                    .math_extra()
                    .smooth_step(
                        f32_field(message, "edge0")?,
                        f32_field(message, "edge1")?,
                        f32_field(message, "x")?,
                    )
                    .map_err(|err| format!("smooth_step() failed: {err:?}"))?;
                self.same_if_present(label, message, "value", native)
            }
            "math_normalize" => {
                let mut native = sys::Float3 {
                    x: f32_field(message, "x")?,
                    y: f32_field(message, "y")?,
                    z: f32_field(message, "z")?,
                };
                self.interface
                    .math_extra()
                    .normalize(&mut native)
                    .map_err(|err| format!("normalize() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "math_bit_or" => {
                let native = self
                    .interface
                    .math_extra()
                    .bit_or(u32_field(message, "a")?, u32_field(message, "b")?)
                    .map_err(|err| format!("bit_or() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "value", native as i32)
            }
            "math_bit_and" => {
                let native = self
                    .interface
                    .math_extra()
                    .bit_and(u32_field(message, "a")?, u32_field(message, "b")?)
                    .map_err(|err| format!("bit_and() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "value", native as i32)
            }
            "math_bit_xor" => {
                let native = self
                    .interface
                    .math_extra()
                    .bit_xor(u32_field(message, "a")?, u32_field(message, "b")?)
                    .map_err(|err| format!("bit_xor() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "value", native as i32)
            }
            "math_bit_inv" => {
                let native = self
                    .interface
                    .math_extra()
                    .bit_inv(u32_field(message, "a")?)
                    .map_err(|err| format!("bit_inv() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "value", native as i32)
            }
            "math_bit_bits" => {
                let bits = [
                    u32_field(message, "bitA")?,
                    u32_field(message, "bitB")?,
                    u32_field(message, "bitC")?,
                ];
                let native = self
                    .interface
                    .math_extra()
                    .bit_bits(&bits)
                    .map_err(|err| format!("bit_bits() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "value", native as i32)
            }
            _ => Err(format!("unsupported math extra check `{label}`")),
        }
    }
}

fn u32_field(message: &Value, field: &str) -> Result<u32, String> {
    message
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| format!("{field}: expected u32"))
}

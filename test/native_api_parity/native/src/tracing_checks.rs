use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_tracing_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let tracing = self.interface.tracing();
        match base_test_name(label) {
            "trace_ray_ground_in_direction" => {
                let start = sys::Float3 {
                    x: f32_field(message, "startX")?,
                    y: f32_field(message, "startY")?,
                    z: f32_field(message, "startZ")?,
                };
                let dir = sys::Float3 {
                    x: f32_field(message, "dirX")?,
                    y: f32_field(message, "dirY")?,
                    z: f32_field(message, "dirZ")?,
                };
                let (hit, hit_length, hit_pos, _) = tracing
                    .trace_ray_ground_in_direction(
                        start,
                        dir,
                        Some(f32_field(message, "maxLength")?),
                        Some(bool_field(message, "testWater")?),
                    )
                    .map_err(|err| format!("trace_ray_ground_in_direction() failed: {err:?}"))?;
                if !hit {
                    return Err("trace_ray_ground_in_direction() returned no hit".to_string());
                }
                self.same_if_present(label, message, "rayLength", hit_length)?;
                self.same_if_present(label, message, "posX", hit_pos.x)?;
                self.same_if_present(label, message, "posY", hit_pos.y)?;
                self.same_if_present(label, message, "posZ", hit_pos.z)
            }
            "trace_ray_ground_between_positions" => {
                let start = sys::Float3 {
                    x: f32_field(message, "startX")?,
                    y: f32_field(message, "startY")?,
                    z: f32_field(message, "startZ")?,
                };
                let end = sys::Float3 {
                    x: f32_field(message, "endX")?,
                    y: f32_field(message, "endY")?,
                    z: f32_field(message, "endZ")?,
                };
                let (hit, hit_length, hit_pos, _) = tracing
                    .trace_ray_ground_between_positions(
                        start,
                        end,
                        Some(bool_field(message, "testWater")?),
                    )
                    .map_err(|err| format!("trace_ray_ground_between_positions() failed: {err:?}"))?;
                if !hit {
                    return Err("trace_ray_ground_between_positions() returned no hit".to_string());
                }
                self.same_if_present(label, message, "rayLength", hit_length)?;
                self.same_if_present(label, message, "posX", hit_pos.x)?;
                self.same_if_present(label, message, "posY", hit_pos.y)?;
                self.same_if_present(label, message, "posZ", hit_pos.z)
            }
            _ => Err(format!("unsupported tracing check `{label}`")),
        }
    }
}

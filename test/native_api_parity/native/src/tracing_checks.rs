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
                        spring_native::TraceRayGroundInDirectionOptions {
                            length: Some(f32_field(message, "maxLength")?),
                            test_water: Some(bool_field(message, "testWater")?),
                        },
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
                        spring_native::TraceRayGroundBetweenPositionsOptions {
                            test_water: Some(bool_field(message, "testWater")?),
                        },
                    )
                    .map_err(|err| {
                        format!("trace_ray_ground_between_positions() failed: {err:?}")
                    })?;
                if !hit {
                    return Err("trace_ray_ground_between_positions() returned no hit".to_string());
                }
                self.same_if_present(label, message, "rayLength", hit_length)?;
                self.same_if_present(label, message, "posX", hit_pos.x)?;
                self.same_if_present(label, message, "posY", hit_pos.y)?;
                self.same_if_present(label, message, "posZ", hit_pos.z)
            }
            "trace_ray_in_direction" => {
                let pos = sys::Float3 {
                    x: f32_field(message, "posX")?,
                    y: f32_field(message, "posY")?,
                    z: f32_field(message, "posZ")?,
                };
                let dir = sys::Float3 {
                    x: f32_field(message, "dirX")?,
                    y: f32_field(message, "dirY")?,
                    z: f32_field(message, "dirZ")?,
                };
                let native = tracing
                    .trace_ray_in_direction(
                        pos,
                        dir,
                        spring_native::TraceRayInDirectionOptions {
                            max_length: Some(f32_field(message, "maxLength")?),
                        },
                        str_field(message, "objectType")?,
                    )
                    .map_err(|err| format!("trace_ray_in_direction() failed: {err:?}"))?;
                self.same_trace_hits(label, message, &native)
            }
            "trace_ray_between_positions" => {
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
                let native = tracing
                    .trace_ray_between_positions(start, end, str_field(message, "objectType")?)
                    .map_err(|err| format!("trace_ray_between_positions() failed: {err:?}"))?;
                self.same_trace_hits(label, message, &native)
            }
            _ => Err(format!("unsupported tracing check `{label}`")),
        }
    }

    fn same_trace_hits(
        &self,
        label: &str,
        message: &Value,
        native: &[sys::TraceRayHit],
    ) -> Result<(), String> {
        let lua = message
            .get("hits")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("{label}.hits: expected array"))?;
        if lua.len() != native.len() {
            return Err(format!(
                "{label}.hits: native={} lua={}",
                native.len(),
                lua.len()
            ));
        }
        for (index, (lua_hit, native_hit)) in lua.iter().zip(native).enumerate() {
            let prefix = format!("{label}.hits[{index}]");
            let lua_length = lua_hit
                .get("hitLength")
                .and_then(Value::as_f64)
                .ok_or_else(|| format!("{prefix}.hitLength: expected number"))?
                as f32;
            self.same(
                &format!("{prefix}.hitLength"),
                native_hit.hitLength,
                lua_length,
            )?;
            let lua_id = lua_hit
                .get("objectID")
                .and_then(Value::as_i64)
                .and_then(|value| i32::try_from(value).ok())
                .ok_or_else(|| format!("{prefix}.objectID: expected integer"))?;
            if native_hit.objectID != lua_id {
                return Err(format!(
                    "{prefix}.objectID: native={} lua={lua_id}",
                    native_hit.objectID
                ));
            }
            let lua_type = lua_hit
                .get("objectType")
                .and_then(Value::as_i64)
                .and_then(|value| i32::try_from(value).ok())
                .ok_or_else(|| format!("{prefix}.objectType: expected integer"))?;
            if native_hit.objectType != lua_type {
                return Err(format!(
                    "{prefix}.objectType: native={} lua={lua_type}",
                    native_hit.objectType
                ));
            }
        }
        Ok(())
    }
}

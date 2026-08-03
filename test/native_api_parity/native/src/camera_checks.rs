use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_camera_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "get_camera_names" => {
                let native = self
                    .interface
                    .camera()
                    .get_camera_names()
                    .map_err(|err| format!("get_camera_names() failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "names", &native)
            }
            "get_camera_position" => {
                let native = self
                    .interface
                    .camera()
                    .get_camera_position()
                    .map_err(|err| format!("get_camera_position() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "get_camera_state" => {
                let use_table = bool_field(message, "useTable")?;
                let native = self
                    .interface
                    .camera()
                    .get_camera_state(use_table)
                    .map_err(|err| format!("get_camera_state({use_table}) failed: {err:?}"))?;
                let camera_name = if native.name.is_null() {
                    ""
                } else {
                    unsafe { CStr::from_ptr(native.name) }
                        .to_str()
                        .map_err(|err| format!("invalid camera state name: {err}"))?
                };
                self.same_string_if_present(label, message, "cameraName", camera_name)
            }
            "get_camera_direction" => {
                let native = self
                    .interface
                    .camera()
                    .get_camera_direction()
                    .map_err(|err| format!("get_camera_direction() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "get_camera_fov" => {
                let native = self
                    .interface
                    .camera()
                    .get_camera_fov()
                    .map_err(|err| format!("get_camera_fov() failed: {err:?}"))?;
                self.same_if_present(label, message, "vFOV", native)
            }
            "get_pixel_dir" => {
                let screen_x = f32_field(message, "screenX")?;
                let screen_y = f32_field(message, "screenY")?;
                let native = self
                    .interface
                    .camera()
                    .get_pixel_dir(screen_x, screen_y)
                    .map_err(|err| {
                        format!("get_pixel_dir({screen_x}, {screen_y}) failed: {err:?}")
                    })?;
                self.same_vec3(label, native, message)
            }
            "trace_screen_ray_ground" => {
                let (hit_type, _hit_id, hit_pos) = self
                    .interface
                    .camera()
                    .trace_screen_ray(
                        f32_field(message, "screenX")?,
                        f32_field(message, "screenY")?,
                        spring_native::TraceScreenRayOptions {
                            only_coords: bool_field(message, "onlyCoords")?,
                            use_minimap: bool_field(message, "useMinimap")?,
                            include_sky: bool_field(message, "includeSky")?,
                            ignore_water: bool_field(message, "ignoreWater")?,
                            height_offset: f32_field(message, "heightOffset")?,
                        },
                    )
                    .map_err(|err| format!("trace_screen_ray() failed: {err:?}"))?;
                let native_type = match hit_type {
                    1 => "unit",
                    2 => "feature",
                    3 => "ground",
                    4 => "sky",
                    _ => "",
                };
                self.same_string_if_present(label, message, "hitType", native_type)?;
                self.same_if_present(label, message, "x", hit_pos.x)?;
                self.same_if_present(label, message, "y", hit_pos.y)?;
                self.same_if_present(label, message, "z", hit_pos.z)
            }
            "world_to_screen_coords" => {
                let world_pos = sys::Float3 {
                    x: f32_field(message, "x")?,
                    y: f32_field(message, "y")?,
                    z: f32_field(message, "z")?,
                };
                let (native, _valid) = self
                    .interface
                    .camera()
                    .world_to_screen_coords(world_pos)
                    .map_err(|err| format!("world_to_screen_coords() failed: {err:?}"))?;
                self.same_if_present(label, message, "screenX", native.x)?;
                self.same_if_present(label, message, "screenY", native.y)?;
                self.same_if_present(label, message, "screenZ", native.z)
            }
            _ => Err(format!("unsupported camera check `{label}`")),
        }
    }

    pub(crate) fn set_camera_target(&mut self, message: &Value) -> Result<(), String> {
        let target = sys::Float3 {
            x: f32_field(message, "x")?,
            y: f32_field(message, "y")?,
            z: f32_field(message, "z")?,
        };
        let transition_time = f32_field(message, "transitionTime")?;
        let success = self
            .interface
            .camera()
            .set_camera_target(
                target,
                spring_native::SetCameraTargetOptions {
                    transition_time: Some(transition_time),
                    ..Default::default()
                },
            )
            .map_err(|err| format!("set_camera_target({transition_time}) failed: {err:?}"))?;
        if success {
            Ok(())
        } else {
            Err("set_camera_target returned false".to_string())
        }
    }
}

use super::*;
use crate::support::*;

fn require_success(label: &str, result: Result<bool, spring_native::Error>) -> Result<(), String> {
    if result.map_err(|err| format!("{label} failed: {err:?}"))? {
        Ok(())
    } else {
        Err(format!("{label} returned false"))
    }
}

fn float4_array_field(message: &Value, field: &str) -> Result<Vec<sys::Float4>, String> {
    let values = message
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array field `{field}`"))?;
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let values = value
                .as_object()
                .ok_or_else(|| format!("{field}[{index}] is not an object"))?;
            Ok(sys::Float4 {
                x: values
                    .get("x")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| format!("missing {field}[{index}].x"))?
                    as f32,
                y: values
                    .get("y")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| format!("missing {field}[{index}].y"))?
                    as f32,
                z: values
                    .get("z")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| format!("missing {field}[{index}].z"))?
                    as f32,
                w: values
                    .get("w")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| format!("missing {field}[{index}].w"))?
                    as f32,
            })
        })
        .collect()
}

fn f32_array_field(message: &Value, field: &str) -> Result<Vec<f32>, String> {
    message
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array field `{field}`"))?
        .iter()
        .enumerate()
        .map(|(index, value)| {
            value
                .as_f64()
                .map(|value| value as f32)
                .ok_or_else(|| format!("{field}[{index}] is not numeric"))
        })
        .collect()
}

fn float4_value_field(message: &Value, field: &str) -> Result<sys::Float4, String> {
    let values = f32_array_field(message, field)?;
    if values.len() < 4 {
        return Err(format!("{field} has fewer than four values"));
    }
    Ok(sys::Float4 {
        x: values[0],
        y: values[1],
        z: values[2],
        w: values[3],
    })
}

impl NativeApiParity {
    pub(crate) fn check_unsynced_control_call(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "assign_mouse_cursor" => {
                let result = self
                    .interface
                    .unsynced_ctrl()
                    .assign_mouse_cursor(
                        str_field(message, "commandName")?,
                        str_field(message, "cursorFileName")?,
                        bool_field(message, "overwrite")?,
                        bool_field(message, "hotSpotTopLeft")?,
                    )
                    .map_err(|err| format!("assign_mouse_cursor() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)
            }
            "replace_mouse_cursor" => {
                let result = self
                    .interface
                    .unsynced_ctrl()
                    .replace_mouse_cursor(
                        str_field(message, "oldCursorFileName")?,
                        str_field(message, "newCursorFileName")?,
                        bool_field(message, "hotSpotTopLeft")?,
                    )
                    .map_err(|err| format!("replace_mouse_cursor() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)
            }
            "set_mouse_cursor" => self
                .interface
                .unsynced_ctrl()
                .set_mouse_cursor(
                    str_field(message, "cursorName")?,
                    f32_field(message, "scale")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_mouse_cursor() failed: {err:?}")),
            "warp_mouse" => self
                .interface
                .unsynced_ctrl()
                .warp_mouse(i32_field(message, "x")?, i32_field(message, "y")?)
                .map(|_| ())
                .map_err(|err| format!("warp_mouse() failed: {err:?}")),
            "set_active_command" => {
                let result = self
                    .interface
                    .unsynced_ctrl()
                    .set_active_command(
                        i32_field(message, "cmdIndex")?,
                        i32_field(message, "button")?,
                        spring_native::SetActiveCommandOptions {
                            left_click: bool_field(message, "leftClick")?,
                            right_click: bool_field(message, "rightClick")?,
                            alt: bool_field(message, "alt")?,
                            ctrl: bool_field(message, "ctrl")?,
                            meta: bool_field(message, "meta")?,
                            shift: bool_field(message, "shift")?,
                        },
                    )
                    .map_err(|err| format!("set_active_command() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)
            }
            "sdl_start_text_input" => self
                .interface
                .unsynced_ctrl()
                .sdlstart_text_input()
                .map(|_| ())
                .map_err(|err| format!("sdlstart_text_input() failed: {err:?}")),
            "sdl_stop_text_input" => self
                .interface
                .unsynced_ctrl()
                .sdlstop_text_input()
                .map(|_| ())
                .map_err(|err| format!("sdlstop_text_input() failed: {err:?}")),
            "sdl_set_text_input_rect" => self
                .interface
                .unsynced_ctrl()
                .sdlset_text_input_rect(
                    i32_field(message, "x")?,
                    i32_field(message, "y")?,
                    i32_field(message, "w")?,
                    i32_field(message, "h")?,
                )
                .map(|_| ())
                .map_err(|err| format!("sdlset_text_input_rect() failed: {err:?}")),
            "unit_set_leave_tracks_unsynced" => self
                .interface
                .unsynced_ctrl()
                .set_unit_leave_tracks(
                    i32_field(message, "unitID")?,
                    bool_field(message, "leaveTracks")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_unit_leave_tracks() failed: {err:?}")),
            "draw_unit_commands_unsynced" => self
                .interface
                .unsynced_ctrl()
                .draw_unit_commands(
                    &[i32_field(message, "unitID")?],
                    bool_field(message, "tableOrArray")?,
                    i32_field(message, "queueDrawDepth")?,
                )
                .map(|_| ())
                .map_err(|err| format!("draw_unit_commands() failed: {err:?}")),
            "set_video_capturing_mode" => self
                .interface
                .unsynced_ctrl()
                .set_video_capturing_mode(bool_field(message, "allowCaptureMode")?)
                .map(|_| ())
                .map_err(|err| format!("set_video_capturing_mode() failed: {err:?}")),
            "set_video_capturing_time_offset" => self
                .interface
                .unsynced_ctrl()
                .set_video_capturing_time_offset(f32_field(message, "timeOffset")?)
                .map(|_| ())
                .map_err(|err| format!("set_video_capturing_time_offset() failed: {err:?}")),
            "set_log_section_filter_level" => self
                .interface
                .config()
                .set_log_section_filter_level(
                    str_field(message, "section")?,
                    i32_field(message, "level")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_log_section_filter_level() failed: {err:?}")),
            "garbage_collect_ctrl" => self
                .interface
                .system_control()
                .garbage_collect_ctrl(
                    i32_field(message, "itersPerBatch")?,
                    i32_field(message, "numStepsPerIter")?,
                    i32_field(message, "minStepsPerIter")?,
                    i32_field(message, "maxStepsPerIter")?,
                    f32_field(message, "minLoopRunTime")?,
                    f32_field(message, "maxLoopRunTime")?,
                    f32_field(message, "baseRunTimeMult")?,
                    f32_field(message, "baseMemLoadMult")?,
                )
                .map(|_| ())
                .map_err(|err| format!("garbage_collect_ctrl() failed: {err:?}")),
            "set_camera_offset" => require_success(
                "set_camera_offset",
                self.interface.unsynced_ctrl().set_camera_offset(
                    vec3_from_fields(message, "posX", "posY", "posZ")?,
                    vec3_from_fields(message, "tiltX", "tiltY", "tiltZ")?,
                ),
            ),
            "set_sun_direction" => require_success(
                "set_sun_direction",
                self.interface.unsynced_ctrl().set_sun_direction(
                    vec3_from_fields(message, "x", "y", "z")?,
                    f32_field(message, "intensity")?,
                ),
            ),
            "set_draw_ground" => require_success(
                "set_draw_ground",
                self.interface
                    .unsynced_ctrl()
                    .set_draw_ground(bool_field(message, "drawGround")?),
            ),
            "set_draw_sky" => require_success(
                "set_draw_sky",
                self.interface
                    .unsynced_ctrl()
                    .set_draw_sky(bool_field(message, "drawSky")?),
            ),
            "set_draw_water" => require_success(
                "set_draw_water",
                self.interface
                    .unsynced_ctrl()
                    .set_draw_water(bool_field(message, "drawWater")?),
            ),
            "set_draw_ground_deferred" => {
                let (success, deferred, forward) = self
                    .interface
                    .unsynced_ctrl()
                    .set_draw_ground_deferred(
                        bool_field(message, "drawDeferred")?,
                        bool_field(message, "drawForward")?,
                    )
                    .map_err(|err| format!("set_draw_ground_deferred() failed: {err:?}"))?;
                if !success {
                    return Err("set_draw_ground_deferred returned false".to_string());
                }
                self.same_bool_if_present(label, message, "deferred", deferred)?;
                self.same_bool_if_present(label, message, "forward", forward)
            }
            "set_draw_models_deferred" => {
                let (success, units_deferred, features_deferred, units_forward, features_forward) =
                    self.interface
                        .unsynced_ctrl()
                        .set_draw_models_deferred(
                            bool_field(message, "unitsDeferred")?,
                            bool_field(message, "featuresDeferred")?,
                            bool_field(message, "unitsForward")?,
                            bool_field(message, "featuresForward")?,
                        )
                        .map_err(|err| format!("set_draw_models_deferred() failed: {err:?}"))?;
                if !success {
                    return Err("set_draw_models_deferred returned false".to_string());
                }
                self.same_bool_if_present(label, message, "unitsDeferred", units_deferred)?;
                self.same_bool_if_present(label, message, "featuresDeferred", features_deferred)?;
                self.same_bool_if_present(label, message, "unitsForward", units_forward)?;
                self.same_bool_if_present(label, message, "featuresForward", features_forward)
            }
            "set_engine_build_square_rendering" => require_success(
                "set_engine_build_square_rendering",
                self.interface
                    .unsynced_ctrl()
                    .set_engine_build_square_rendering(bool_field(message, "enabled")?),
            ),
            "set_auto_show_metal" => require_success(
                "set_auto_show_metal",
                self.interface
                    .unsynced_ctrl()
                    .set_auto_show_metal(bool_field(message, "enable")?),
            ),
            "force_layout_update" => require_success(
                "force_layout_update",
                self.interface.unsynced_ctrl().force_layout_update(),
            ),
            "force_tesselation_update" => {
                let updated = self
                    .interface
                    .unsynced_ctrl()
                    .force_tesselation_update(
                        bool_field(message, "normal")?,
                        bool_field(message, "shadow")?,
                    )
                    .map_err(|err| format!("force_tesselation_update() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "updated", updated)
            }
            "set_custom_command_draw_data" => {
                let color = float4_value_field(message, "color")?;
                let result = self
                    .interface
                    .unsynced_ctrl()
                    .set_custom_command_draw_data(
                        i32_field(message, "cmdID")?,
                        sys::DefRef {
                            name: std::ptr::null(),
                            id: -1,
                        },
                        color,
                        bool_field(message, "showArea")?,
                    )
                    .map_err(|err| format!("set_custom_command_draw_data() failed: {err:?}"))?;
                if !result {
                    return Err("set_custom_command_draw_data returned false".to_string());
                }
                Ok(())
            }
            "set_unit_def_icon" => require_success(
                "set_unit_def_icon",
                self.interface.unsynced_ctrl().set_unit_def_icon(
                    i32_field(message, "unitDefID")?,
                    str_field(message, "iconName")?,
                ),
            ),
            "set_unit_def_image" => require_success(
                "set_unit_def_image",
                self.interface.unsynced_ctrl().set_unit_def_image(
                    i32_field(message, "unitDefID")?,
                    str_field(message, "image")?,
                ),
            ),
            "preload_unit_def_model" => require_success(
                "preload_unit_def_model",
                self.interface
                    .unsynced_ctrl()
                    .preload_unit_def_model(i32_field(message, "defID")?),
            ),
            "preload_feature_def_model" => require_success(
                "preload_feature_def_model",
                self.interface
                    .unsynced_ctrl()
                    .preload_feature_def_model(i32_field(message, "defID")?),
            ),
            "load_model_textures" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .load_model_textures(str_field(message, "modelName")?)
                    .map_err(|err| format!("load_model_textures() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", success)
            }
            "load_cmd_colors_config" => require_success(
                "load_cmd_colors_config",
                self.interface
                    .unsynced_ctrl()
                    .load_cmd_colors_config(str_field(message, "config")?),
            ),
            "load_ctrl_panel_config" => require_success(
                "load_ctrl_panel_config",
                self.interface
                    .unsynced_ctrl()
                    .load_ctrl_panel_config(str_field(message, "config")?),
            ),
            "set_wm_icon" => require_success(
                "set_wm_icon",
                self.interface.unsynced_ctrl().set_wmicon(
                    str_field(message, "iconFileName")?,
                    bool_field(message, "forceResolution")?,
                ),
            ),
            "set_map_shader" => require_success(
                "set_map_shader",
                self.interface.unsynced_ctrl().set_map_shader(
                    i32_field(message, "standardShaderID")?,
                    i32_field(message, "deferredShaderID")?,
                ),
            ),
            "set_map_shading_texture" => {
                let result = self
                    .interface
                    .unsynced_ctrl()
                    .set_map_shading_texture(
                        str_field(message, "texType")?,
                        str_field(message, "texName")?,
                        i32_field(message, "num")?,
                    )
                    .map_err(|err| format!("set_map_shading_texture() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)
            }
            "set_sky_box_texture" => require_success(
                "set_sky_box_texture",
                self.interface
                    .unsynced_ctrl()
                    .set_sky_box_texture(str_field(message, "texName")?),
            ),
            "set_wm_caption" => require_success(
                "set_wm_caption",
                self.interface.unsynced_ctrl().set_wmcaption(
                    str_field(message, "title")?,
                    str_field(message, "titleShort")?,
                ),
            ),
            "set_shock_front_factors" => require_success(
                "set_shock_front_factors",
                self.interface.unsynced_ctrl().set_shock_front_factors(
                    spring_native::SetShockFrontFactorsOptions {
                        min_area: Some(f32_field(message, "minArea")?),
                        min_power: Some(f32_field(message, "minPower")?),
                        dist_adj: Some(f32_field(message, "distAdj")?),
                    },
                ),
            ),
            "run_dolly_camera" => require_success(
                "run_dolly_camera",
                self.interface
                    .unsynced_ctrl()
                    .run_dolly_camera(f32_field(message, "runtimeMs")?),
            ),
            "pause_dolly_camera" => require_success(
                "pause_dolly_camera",
                self.interface
                    .unsynced_ctrl()
                    .pause_dolly_camera(f32_field(message, "percent")?),
            ),
            "resume_dolly_camera" => require_success(
                "resume_dolly_camera",
                self.interface.unsynced_ctrl().resume_dolly_camera(),
            ),
            "set_dolly_camera_mode" => require_success(
                "set_dolly_camera_mode",
                self.interface
                    .unsynced_ctrl()
                    .set_dolly_camera_mode(i32_field(message, "mode")?),
            ),
            "set_dolly_camera_position" => require_success(
                "set_dolly_camera_position",
                self.interface
                    .unsynced_ctrl()
                    .set_dolly_camera_position(vec3_from_fields(message, "x", "y", "z")?),
            ),
            "set_dolly_camera_curve" => {
                let control_points = float4_array_field(message, "controlPoints")?;
                let knots = f32_array_field(message, "knots")?;
                require_success(
                    "set_dolly_camera_curve",
                    self.interface.unsynced_ctrl().set_dolly_camera_curve(
                        i32_field(message, "degree")?,
                        &control_points,
                        &knots,
                    ),
                )
            }
            "set_dolly_camera_look_curve" => {
                let control_points = float4_array_field(message, "controlPoints")?;
                let knots = f32_array_field(message, "knots")?;
                require_success(
                    "set_dolly_camera_look_curve",
                    self.interface.unsynced_ctrl().set_dolly_camera_look_curve(
                        i32_field(message, "degree")?,
                        &control_points,
                        &knots,
                    ),
                )
            }
            "set_dolly_camera_look_position" => require_success(
                "set_dolly_camera_look_position",
                self.interface
                    .unsynced_ctrl()
                    .set_dolly_camera_look_position(vec3_from_fields(message, "x", "y", "z")?),
            ),
            "set_dolly_camera_look_unit" => require_success(
                "set_dolly_camera_look_unit",
                self.interface
                    .unsynced_ctrl()
                    .set_dolly_camera_look_unit(i32_field(message, "unitID")?),
            ),
            "set_dolly_camera_relative_mode" => require_success(
                "set_dolly_camera_relative_mode",
                self.interface
                    .unsynced_ctrl()
                    .set_dolly_camera_relative_mode(i32_field(message, "mode")?),
            ),
            name => Err(format!("unsupported unsynced control check `{name}`")),
        }
    }
}

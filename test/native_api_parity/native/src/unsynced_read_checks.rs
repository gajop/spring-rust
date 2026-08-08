use super::*;
use crate::support::*;

fn float4_array_field(message: &Value, field: &str) -> Result<Vec<sys::Float4>, String> {
    let values = message
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array field `{field}`"))?;
    if values.len() % 4 != 0 {
        return Err(format!("{field} needs a multiple of four values"));
    }
    values
        .chunks(4)
        .enumerate()
        .map(|(index, values)| {
            Ok(sys::Float4 {
                x: values[0]
                    .as_f64()
                    .ok_or_else(|| format!("missing {field}[{}]", index * 4 + 1))?
                    as f32,
                y: values[1]
                    .as_f64()
                    .ok_or_else(|| format!("missing {field}[{}]", index * 4 + 2))?
                    as f32,
                z: values[2]
                    .as_f64()
                    .ok_or_else(|| format!("missing {field}[{}]", index * 4 + 3))?
                    as f32,
                w: values[3]
                    .as_f64()
                    .ok_or_else(|| format!("missing {field}[{}]", index * 4 + 4))?
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
                .ok_or_else(|| format!("{field}[{index}] is not a number"))
        })
        .collect()
}

impl NativeApiParity {
    pub(crate) fn check_unsynced_render_data(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "get_unit_selection_volume_data" => {
                let unit_id = i32_field(message, "unitID")?;
                let (scales, offsets, volume_type, use_cont_hit_test, primary_axis, ignore_hits) =
                    self.interface
                        .unsynced_read()
                        .unit_rendering()
                        .get_unit_selection_volume_data(unit_id)
                        .map_err(|err| {
                            format!("get_unit_selection_volume_data({unit_id}) failed: {err:?}")
                        })?;
                self.same_if_present(label, message, "scaleX", scales.x)?;
                self.same_if_present(label, message, "scaleY", scales.y)?;
                self.same_if_present(label, message, "scaleZ", scales.z)?;
                self.same_if_present(label, message, "offsetX", offsets.x)?;
                self.same_if_present(label, message, "offsetY", offsets.y)?;
                self.same_if_present(label, message, "offsetZ", offsets.z)?;
                self.same_i32_if_present(label, message, "volumeType", volume_type)?;
                self.same_i32_if_present(
                    label,
                    message,
                    "useContHitTest",
                    use_cont_hit_test as i32,
                )?;
                self.same_i32_if_present(label, message, "primaryAxis", primary_axis)?;
                self.same_bool_if_present(label, message, "ignoreHits", ignore_hits)
            }
            "get_feature_selection_volume_data" => {
                let feature_id = i32_field(message, "featureID")?;
                let native = self
                    .interface
                    .features()
                    .get_feature_selection_volume_data(feature_id)
                    .map_err(|err| {
                        format!("get_feature_selection_volume_data({feature_id}) failed: {err:?}")
                    })?;
                self.same_if_present(label, message, "scaleX", native.scales.x)?;
                self.same_if_present(label, message, "scaleY", native.scales.y)?;
                self.same_if_present(label, message, "scaleZ", native.scales.z)?;
                self.same_if_present(label, message, "offsetX", native.offsets.x)?;
                self.same_if_present(label, message, "offsetY", native.offsets.y)?;
                self.same_if_present(label, message, "offsetZ", native.offsets.z)?;
                self.same_i32_if_present(label, message, "volumeType", native.volumeType)?;
                self.same_i32_if_present(
                    label,
                    message,
                    "useContHitTest",
                    native.useContHitTest as i32,
                )?;
                self.same_i32_if_present(label, message, "primaryAxis", native.primaryAxis)?;
                self.same_bool_if_present(label, message, "ignoreHits", native.ignoreHits)
            }
            "get_unit_transform_matrix" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_unit_transform_matrix(unit_id)
                    .map_err(|err| {
                        format!("get_unit_transform_matrix({unit_id}) failed: {err:?}")
                    })?;
                for (index, value) in native.iter().enumerate() {
                    self.same_if_present(label, message, &format!("m{}", index + 1), *value)?;
                }
                Ok(())
            }
            "get_feature_transform_matrix" => {
                let feature_id = i32_field(message, "featureID")?;
                let native = self
                    .interface
                    .features()
                    .get_feature_transform_matrix(feature_id)
                    .map_err(|err| {
                        format!("get_feature_transform_matrix({feature_id}) failed: {err:?}")
                    })?;
                for (index, value) in native.values.iter().enumerate() {
                    self.same_if_present(label, message, &format!("m{}", index + 1), *value)?;
                }
                Ok(())
            }
            "get_unit_palette_index" | "unit_palette_index" => {
                let unit_id = i32_field(message, "unitID")?;
                let (custom_index, using_custom_color) = self
                    .interface
                    .unsynced_read()
                    .get_unit_palette_index(unit_id)
                    .map_err(|err| format!("get_unit_palette_index({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasCustomColor", using_custom_color)?;
                self.same_i32_if_present(
                    label,
                    message,
                    "customIndex",
                    if using_custom_color { custom_index } else { -1 },
                )
            }
            "get_feature_palette_index" | "feature_palette_index" => {
                let feature_id = i32_field(message, "featureID")?;
                let (custom_index, using_custom_color) = self
                    .interface
                    .unsynced_read()
                    .get_feature_palette_index(feature_id)
                    .map_err(|err| {
                        format!("get_feature_palette_index({feature_id}) failed: {err:?}")
                    })?;
                self.same_bool_if_present(label, message, "hasCustomColor", using_custom_color)?;
                self.same_i32_if_present(
                    label,
                    message,
                    "customIndex",
                    if using_custom_color { custom_index } else { -1 },
                )
            }
            "custom_palette_color" => {
                let (r, g, b, _) = self
                    .interface
                    .unsynced_read()
                    .get_custom_palette_color(i32_field(message, "index")?)
                    .map_err(|err| format!("get_custom_palette_color() failed: {err:?}"))?;
                self.same_if_present(label, message, "r", r)?;
                self.same_if_present(label, message, "g", g)?;
                self.same_if_present(label, message, "b", b)
            }
            "get_unit_icon" | "set_unit_icon" => {
                let unit_id = i32_field(message, "unitID")?;
                let (icon_name, _, _, _, _) = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_unit_icon(unit_id)
                    .map_err(|err| format!("get_unit_icon({unit_id}) failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "iconName",
                    icon_name.as_deref().unwrap_or(""),
                )
            }
            name => Err(format!("unsupported unsynced render data check `{name}`")),
        }
    }

    pub(crate) fn check_unsynced_read_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "clear_features_previous_draw_flag" => {
                let success = self
                    .interface
                    .features()
                    .clear_features_previous_draw_flag()
                    .map_err(|err| {
                        format!("clear_features_previous_draw_flag() failed: {err:?}")
                    })?;
                if !success {
                    return Err("clear_features_previous_draw_flag returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "clear_units_previous_draw_flag" => {
                let success = self
                    .interface
                    .units_info()
                    .clear_units_previous_draw_flag()
                    .map_err(|err| format!("clear_units_previous_draw_flag() failed: {err:?}"))?;
                if !success {
                    return Err("clear_units_previous_draw_flag returned false".to_string());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "get_piece_projectile_name" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_piece_projectile_name(i32_field(message, "projectileID")?)
                    .map_err(|err| format!("get_piece_projectile_name() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "name", native.as_deref().unwrap_or(""))
            }
            "get_prev_frame_sync_checksum" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_prev_frame_sync_checksum()
                    .map_err(|err| format!("get_prev_frame_sync_checksum() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "checksum",
                    native.as_deref().unwrap_or(""),
                )
            }
            "get_game_seconds_interpolated" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_game_seconds_interpolated()
                    .map_err(|err| format!("get_game_seconds_interpolated() failed: {err:?}"))?;
                self.same_if_present(label, message, "seconds", native)
            }
            "solve_nurbscurve" => {
                let control_points = float4_array_field(message, "controlPoints")?;
                let knots = f32_array_field(message, "knots")?;
                let (points, success) = self
                    .interface
                    .unsynced_read()
                    .solve_nurbscurve(
                        i32_field(message, "degree")?,
                        &control_points,
                        &knots,
                        i32_field(message, "segments")?,
                    )
                    .map_err(|err| format!("solve_nurbscurve() failed: {err:?}"))?;
                if !success {
                    return Err("solve_nurbscurve() returned false".to_string());
                }
                let flat_points = points
                    .into_iter()
                    .flat_map(|point| [point.x, point.y, point.z])
                    .collect::<Vec<_>>();
                self.same_f32_list_if_present(label, message, "points", &flat_points)
            }
            "get_box_selection_by_engine" | "box_selection_by_engine" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_box_selection_by_engine()
                    .map_err(|err| format!("get_box_selection_by_engine() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "enabled", native)
            }
            "get_build_facing" | "build_facing" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_build_facing()
                    .map_err(|err| format!("get_build_facing() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "facing", native)
            }
            "get_build_spacing" | "build_spacing" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_build_spacing()
                    .map_err(|err| format!("get_build_spacing() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "spacing", native)
            }
            "get_last_message_positions_count" | "last_message_position_count" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_last_message_positions()
                    .map_err(|err| format!("get_last_message_positions() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_draw_selection_info" | "draw_selection_info" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_draw_selection_info()
                    .map_err(|err| format!("get_draw_selection_info() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "draw", native)
            }
            "get_cmd_desc_index_missing" => {
                let cmd_id = i32_field(message, "cmdID")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .get_cmd_desc_index(cmd_id)
                    .map_err(|err| format!("get_cmd_desc_index({cmd_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "cmdDescIndex", native)
            }
            "is_unit_allied" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .is_unit_allied(unit_id)
                    .map_err(|err| format!("is_unit_allied({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "allied", native)
            }
            "get_active_cmd_descs_fixed_count" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_active_cmd_descs()
                    .map_err(|err| format!("get_active_cmd_descs() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_active_cmd_desc_fixed_fields" => {
                let (cmd_desc, has_command) = self
                    .interface
                    .unsynced_read()
                    .get_active_cmd_desc(i32_field(message, "cmdIndex")?)
                    .map_err(|err| format!("get_active_cmd_desc() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasCommand", has_command)?;
                if has_command {
                    let name = unsafe {
                        if cmd_desc.name.is_null() {
                            ""
                        } else {
                            CStr::from_ptr(cmd_desc.name).to_str().unwrap_or("")
                        }
                    };
                    self.same_i32_if_present(label, message, "id", cmd_desc.id)?;
                    self.same_i32_if_present(label, message, "type", cmd_desc.type_)?;
                    self.same_string_if_present(label, message, "name", name)?;
                    self.same_bool_if_present(label, message, "queueing", cmd_desc.queueing)?;
                    self.same_bool_if_present(label, message, "hidden", cmd_desc.hidden)?;
                    self.same_bool_if_present(label, message, "disabled", cmd_desc.disabled)?;
                    self.same_i32_if_present(
                        label,
                        message,
                        "paramCount",
                        cmd_desc.paramCount as i32,
                    )?;
                }
                Ok(())
            }
            "get_camera_rotation" => {
                let (rot_x, rot_y, rot_z) = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_camera_rotation()
                    .map_err(|err| format!("get_camera_rotation() failed: {err:?}"))?;
                self.same_if_present(label, message, "rotX", rot_x)?;
                self.same_if_present(label, message, "rotY", rot_y)?;
                self.same_if_present(label, message, "rotZ", rot_z)
            }
            "get_camera_vectors" => {
                let (forward, up, right) = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_camera_vectors()
                    .map_err(|err| format!("get_camera_vectors() failed: {err:?}"))?;
                self.same_if_present(label, message, "forwardX", forward.x)?;
                self.same_if_present(label, message, "forwardY", forward.y)?;
                self.same_if_present(label, message, "forwardZ", forward.z)?;
                self.same_if_present(label, message, "upX", up.x)?;
                self.same_if_present(label, message, "upY", up.y)?;
                self.same_if_present(label, message, "upZ", up.z)?;
                self.same_if_present(label, message, "rightX", right.x)?;
                self.same_if_present(label, message, "rightY", right.y)?;
                self.same_if_present(label, message, "rightZ", right.z)
            }
            "get_frustum_planes" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_frustum_planes()
                    .map_err(|err| format!("get_frustum_planes() failed: {err:?}"))?;
                self.same_if_present(label, message, "topX", native[0])?;
                self.same_if_present(label, message, "topY", native[1])?;
                self.same_if_present(label, message, "topZ", native[2])?;
                self.same_if_present(label, message, "bottomX", native[4])?;
                self.same_if_present(label, message, "bottomY", native[5])?;
                self.same_if_present(label, message, "bottomZ", native[6])?;
                self.same_if_present(label, message, "leftX", native[8])?;
                self.same_if_present(label, message, "leftY", native[9])?;
                self.same_if_present(label, message, "leftZ", native[10])?;
                self.same_if_present(label, message, "rightX", native[12])?;
                self.same_if_present(label, message, "rightY", native[13])?;
                self.same_if_present(label, message, "rightZ", native[14])
            }
            "clipboard_text" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_clipboard()
                    .map_err(|err| format!("get_clipboard() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "text", native.as_deref().unwrap_or(""))
            }
            "nano_projectile_params" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .get_nano_projectile_params()
                    .map_err(|err| format!("get_nano_projectile_params() failed: {err:?}"))?;
                self.same_if_present(label, message, "rotVal", native.0)?;
                self.same_if_present(label, message, "rotVel", native.1)?;
                self.same_if_present(label, message, "rotAcc", native.2)?;
                self.same_if_present(label, message, "rotValRng", native.3)?;
                self.same_if_present(label, message, "rotVelRng", native.4)?;
                self.same_if_present(label, message, "rotAccRng", native.5)
            }
            "is_unit_selected" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .is_unit_selected(unit_id)
                    .map_err(|err| format!("is_unit_selected({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "selected", native)
            }
            "is_unit_in_view" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .is_unit_in_view(unit_id)
                    .map_err(|err| format!("is_unit_in_view({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "inView", native)
            }
            "is_unit_icon" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .is_unit_icon(unit_id)
                    .map_err(|err| format!("is_unit_icon({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "isIcon", native)
            }
            "is_unit_visible" => {
                let unit_id = i32_field(message, "unitID")?;
                let radius = f32_field(message, "radius")?;
                let check_icon = bool_field(message, "checkIcon")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .is_unit_visible(unit_id, radius, check_icon)
                    .map_err(|err| {
                        format!(
                            "is_unit_visible({unit_id}, {radius}, {check_icon}) failed: {err:?}"
                        )
                    })?;
                self.same_bool_if_present(label, message, "visible", native)
            }
            "get_visible_units" => {
                let team_id = i32_field(message, "teamID")?;
                let radius = f32_field(message, "radius")?;
                let include_icons = bool_field(message, "includeIcons")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_visible_units(team_id, radius, include_icons)
                    .map_err(|err| format!("get_visible_units({team_id}, {radius}, {include_icons}) failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "unitIDs", &native)
            }
            "get_visible_features" => {
                let ally_team_id = i32_field(message, "allyTeamID")?;
                let radius = f32_field(message, "radius")?;
                let include_icons = bool_field(message, "includeIcons")?;
                let include_geos = bool_field(message, "includeGeos")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_visible_features(
                        ally_team_id,
                        radius,
                        spring_native::GetVisibleFeaturesOptions {
                            include_icons,
                            include_geos,
                        },
                    )
                    .map_err(|err| format!("get_visible_features({ally_team_id}, {radius}, {include_icons}, {include_geos}) failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "featureIDs", &native)
            }
            "get_visible_projectiles" => {
                let ally_team_id = i32_field(message, "allyTeamID")?;
                let include_synced = bool_field(message, "includeSynced")?;
                let include_weapon = bool_field(message, "includeWeapon")?;
                let include_piece = bool_field(message, "includePiece")?;
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_visible_projectiles(
                        ally_team_id,
                        spring_native::GetVisibleProjectilesOptions {
                            include_synced_projectiles: include_synced,
                            include_weapon_projectiles: include_weapon,
                            include_piece_projectiles: include_piece,
                        },
                    )
                    .map_err(|err| format!("get_visible_projectiles({ally_team_id}, {include_synced}, {include_weapon}, {include_piece}) failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "projectileIDs", &native)
            }
            "get_units_in_screen_rectangle" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_units_in_screen_rectangle(
                        f32_field(message, "left")?,
                        f32_field(message, "top")?,
                        f32_field(message, "right")?,
                        f32_field(message, "bottom")?,
                        i32_field(message, "allegiance")?,
                    )
                    .map_err(|err| format!("get_units_in_screen_rectangle() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "unitIDs", &native)
            }
            "get_features_in_screen_rectangle" => {
                let native = self
                    .interface
                    .unsynced_read()
                    .unit_rendering()
                    .get_features_in_screen_rectangle(
                        f32_field(message, "left")?,
                        f32_field(message, "top")?,
                        f32_field(message, "right")?,
                        f32_field(message, "bottom")?,
                    )
                    .map_err(|err| format!("get_features_in_screen_rectangle() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "featureIDs", &native)
            }
            _ => Err(format!("unsupported unsynced read check `{label}`")),
        }
    }

    pub(crate) fn set_unsynced_read_value(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(test_name_field(message)?) {
            "clipboard_text" => {
                let text = str_field(message, "text")?;
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_clipboard(text)
                    .map_err(|err| format!("set_clipboard() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_clipboard returned false".to_string())
                }
            }
            "custom_palette_color" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_custom_palette_color(
                        i32_field(message, "index")?,
                        f32_field(message, "r")?,
                        f32_field(message, "g")?,
                        f32_field(message, "b")?,
                    )
                    .map_err(|err| format!("set_custom_palette_color() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_custom_palette_color returned false".to_string())
                }
            }
            "unit_palette_index" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_unit_palette_index(
                        i32_field(message, "unitID")?,
                        i32_field(message, "customIndex")?,
                    )
                    .map_err(|err| format!("set_unit_palette_index() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_unit_palette_index returned false".to_string())
                }
            }
            "feature_palette_index" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_feature_palette_index(
                        i32_field(message, "featureID")?,
                        i32_field(message, "customIndex")?,
                    )
                    .map_err(|err| format!("set_feature_palette_index() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_feature_palette_index returned false".to_string())
                }
            }
            "box_selection_by_engine" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_box_selection_by_engine(bool_field(message, "enabled")?)
                    .map_err(|err| format!("set_box_selection_by_engine() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_box_selection_by_engine returned false".to_string())
                }
            }
            "build_facing" => {
                let facing = i32_field(message, "facing")?;
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_build_facing(facing)
                    .map_err(|err| format!("set_build_facing({facing}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err(format!("set_build_facing({facing}) returned false"))
                }
            }
            "build_spacing" => {
                let spacing = i32_field(message, "spacing")?;
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_build_spacing(spacing)
                    .map_err(|err| format!("set_build_spacing({spacing}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err(format!("set_build_spacing({spacing}) returned false"))
                }
            }
            "draw_selection_info" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_draw_selection_info(bool_field(message, "draw")?)
                    .map_err(|err| format!("set_draw_selection_info() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_draw_selection_info returned false".to_string())
                }
            }
            "last_message_position_count" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_last_message_position(vec3_from_fields(message, "x", "y", "z")?)
                    .map_err(|err| format!("set_last_message_position() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_last_message_position returned false".to_string())
                }
            }
            "nano_projectile_params" => {
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_nano_projectile_params(
                        f32_field(message, "rotVal")?,
                        f32_field(message, "rotVel")?,
                        f32_field(message, "rotAcc")?,
                        f32_field(message, "rotValRng")?,
                        f32_field(message, "rotVelRng")?,
                        f32_field(message, "rotAccRng")?,
                    )
                    .map_err(|err| format!("set_nano_projectile_params() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_nano_projectile_params returned false".to_string())
                }
            }
            name => Err(format!("unsupported unsynced read setter `{name}`")),
        }
    }
}

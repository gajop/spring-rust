use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_unsynced_read_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
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
                    self.same_i32_if_present(label, message, "paramCount", cmd_desc.paramCount as i32)?;
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
                    .map_err(|err| format!("is_unit_visible({unit_id}, {radius}, {check_icon}) failed: {err:?}"))?;
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
                    .get_visible_features(ally_team_id, radius, include_icons, include_geos)
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
                    .get_visible_projectiles(ally_team_id, include_synced, include_weapon, include_piece)
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
        match base_test_name(str_field(message, "name")?) {
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

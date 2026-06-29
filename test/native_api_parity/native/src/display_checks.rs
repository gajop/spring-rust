use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_display_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
            "get_num_displays" => {
                let native = self
                    .interface
                    .display()
                    .get_num_displays()
                    .map_err(|err| format!("get_num_displays() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native as i32)
            }
            "get_draw_frame" => {
                let (low16, high16) = self
                    .interface
                    .display()
                    .get_draw_frame()
                    .map_err(|err| format!("get_draw_frame() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "low16", low16 as i32)?;
                self.same_i32_if_present(label, message, "high16", high16 as i32)
            }
            "get_frame_time_offset" => {
                let native = self
                    .interface
                    .display()
                    .get_frame_time_offset()
                    .map_err(|err| format!("get_frame_time_offset() failed: {err:?}"))?;
                self.same_if_present(label, message, "offset", native)
            }
            "get_last_update_seconds" => {
                let native = self
                    .interface
                    .display()
                    .get_last_update_seconds()
                    .map_err(|err| format!("get_last_update_seconds() failed: {err:?}"))?;
                self.same_if_present(label, message, "seconds", native)
            }
            "get_fps" => {
                let native = self
                    .interface
                    .display()
                    .get_fps()
                    .map_err(|err| format!("get_fps() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "fps", native as i32)
            }
            "get_game_speed" => {
                let (wanted_speed, speed, paused) = self
                    .interface
                    .display()
                    .get_game_speed()
                    .map_err(|err| format!("get_game_speed() failed: {err:?}"))?;
                self.same_if_present(label, message, "wantedSpeedFactor", wanted_speed)?;
                self.same_if_present(label, message, "speedFactor", speed)?;
                self.same_bool_if_present(label, message, "paused", paused)
            }
            "get_view_geometry" => {
                let native = self
                    .interface
                    .display()
                    .get_view_geometry()
                    .map_err(|err| format!("get_view_geometry() failed: {err:?}"))?;
                self.same_view_geometry(label, message, native)
            }
            "get_dual_view_geometry" => {
                let native = self
                    .interface
                    .display()
                    .get_dual_view_geometry()
                    .map_err(|err| format!("get_dual_view_geometry() failed: {err:?}"))?;
                self.same_view_geometry(label, message, native)
            }
            "get_window_geometry" => {
                let native = self
                    .interface
                    .display()
                    .get_window_geometry()
                    .map_err(|err| format!("get_window_geometry() failed: {err:?}"))?;
                self.same_view_geometry(label, message, native)
            }
            "get_screen_geometry" => {
                let screen_num = i32_field(message, "screenNum")?;
                let query_usable = bool_field(message, "queryUsable")?;
                let native = self
                    .interface
                    .display()
                    .get_screen_geometry(screen_num, query_usable)
                    .map_err(|err| format!("get_screen_geometry({screen_num}, {query_usable}) failed: {err:?}"))?;
                self.same_view_geometry(label, message, native)
            }
            "get_mini_map_geometry" => {
                let native = self
                    .interface
                    .display()
                    .get_mini_map_geometry()
                    .map_err(|err| format!("get_mini_map_geometry() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "posX", native.posX)?;
                self.same_i32_if_present(label, message, "posY", native.posY)?;
                self.same_i32_if_present(label, message, "sizeX", native.sizeX)?;
                self.same_i32_if_present(label, message, "sizeY", native.sizeY)?;
                self.same_bool_if_present(label, message, "minimized", native.minimized)?;
                self.same_bool_if_present(label, message, "maximized", native.maximized)
            }
            "get_mini_map_dual_screen" => {
                let (_position, dual_screen) = self
                    .interface
                    .display()
                    .get_mini_map_dual_screen()
                    .map_err(|err| format!("get_mini_map_dual_screen() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "dualScreen", dual_screen)
            }
            "get_mini_map_rotation" | "mini_map_rotation" => {
                let native = self
                    .interface
                    .display()
                    .get_mini_map_rotation()
                    .map_err(|err| format!("get_mini_map_rotation() failed: {err:?}"))?;
                self.same_if_present(label, message, "rotation", native)
            }
            "get_map_draw_mode" => {
                let native = self
                    .interface
                    .display()
                    .get_map_draw_mode()
                    .map_err(|err| format!("get_map_draw_mode() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "mode", native.as_deref().unwrap_or(""))
            }
            "get_water_mode" => {
                let (mode, name) = self
                    .interface
                    .display()
                    .get_water_mode()
                    .map_err(|err| format!("get_water_mode() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "mode", mode)?;
                self.same_string_if_present(label, message, "waterName", name.as_deref().unwrap_or(""))
            }
            "get_team_color" => {
                let team_id = i32_field(message, "teamID")?;
                let native = self
                    .interface
                    .display()
                    .get_team_color(team_id)
                    .map_err(|err| format!("get_team_color({team_id}) failed: {err:?}"))?;
                self.same_team_color(label, message, native)
            }
            "get_team_orig_color" => {
                let team_id = i32_field(message, "teamID")?;
                let native = self
                    .interface
                    .display()
                    .get_team_orig_color(team_id)
                    .map_err(|err| format!("get_team_orig_color({team_id}) failed: {err:?}"))?;
                self.same_team_color(label, message, native)
            }
            "team_color" => {
                let team_id = i32_field(message, "teamID")?;
                let native = self
                    .interface
                    .display()
                    .get_team_color(team_id)
                    .map_err(|err| format!("get_team_color({team_id}) failed: {err:?}"))?;
                self.same_team_color(label, message, native)
            }
            "get_los_view_colors" | "los_view_colors" => {
                let (always, los, _radar, _jam, _radar2) = self
                    .interface
                    .display()
                    .get_los_view_colors()
                    .map_err(|err| format!("get_los_view_colors() failed: {err:?}"))?;
                self.same_if_present(label, message, "alwaysR", always.x)?;
                self.same_if_present(label, message, "alwaysG", always.y)?;
                self.same_if_present(label, message, "alwaysB", always.z)?;
                self.same_if_present(label, message, "losR", los.x)?;
                self.same_if_present(label, message, "losG", los.y)?;
                self.same_if_present(label, message, "losB", los.z)
            }
            "is_gui_hidden" => {
                let native = self
                    .interface
                    .display()
                    .is_guihidden()
                    .map_err(|err| format!("is_guihidden() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hidden", native)
            }
            "have_shadows" => {
                let native = self
                    .interface
                    .display()
                    .have_shadows()
                    .map_err(|err| format!("have_shadows() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "enabled", native)
            }
            "have_adv_shading" => {
                let native = self
                    .interface
                    .display()
                    .have_adv_shading()
                    .map_err(|err| format!("have_adv_shading() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "enabled", native)
            }
            "is_sphere_in_view" => {
                let center = sys::Float3 { x: f32_field(message, "x")?, y: f32_field(message, "y")?, z: f32_field(message, "z")? };
                let radius = f32_field(message, "radius")?;
                let native = self
                    .interface
                    .display()
                    .is_sphere_in_view(center, radius)
                    .map_err(|err| format!("is_sphere_in_view(_, {radius}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "inView", native)
            }
            "is_aabb_in_view" => {
                let mins = sys::Float3 { x: f32_field(message, "minX")?, y: f32_field(message, "minY")?, z: f32_field(message, "minZ")? };
                let maxs = sys::Float3 { x: f32_field(message, "maxX")?, y: f32_field(message, "maxY")?, z: f32_field(message, "maxZ")? };
                let native = self
                    .interface
                    .display()
                    .is_aabbin_view(mins, maxs)
                    .map_err(|err| format!("is_aabbin_view() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "inView", native)
            }
            _ => Err(format!("unsupported display check `{label}`")),
        }
    }

    fn same_view_geometry(&self, label: &str, message: &Value, native: sys::ViewGeometry) -> Result<(), String> {
        self.same_i32_if_present(label, message, "viewSizeX", native.viewSizeX)?;
        self.same_i32_if_present(label, message, "viewSizeY", native.viewSizeY)?;
        self.same_i32_if_present(label, message, "viewPosX", native.viewPosX)?;
        self.same_i32_if_present(label, message, "viewPosY", native.viewPosY)
    }

    fn same_team_color(&self, label: &str, message: &Value, native: sys::TeamColor) -> Result<(), String> {
        self.same_if_present(label, message, "r", native.r)?;
        self.same_if_present(label, message, "g", native.g)?;
        self.same_if_present(label, message, "b", native.b)?;
        self.same_if_present(label, message, "a", native.a)
    }

    pub(crate) fn set_team_color(&mut self, message: &Value) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let color = sys::TeamColor {
            r: f32_field(message, "r")?,
            g: f32_field(message, "g")?,
            b: f32_field(message, "b")?,
            a: f32_field(message, "a")?,
        };
        let success = self
            .interface
            .display()
            .set_team_color(team_id, color)
            .map_err(|err| format!("set_team_color({team_id}) failed: {err:?}"))?;
        if success {
            Ok(())
        } else {
            Err(format!("set_team_color({team_id}) returned false"))
        }
    }

    pub(crate) fn set_display_value(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(str_field(message, "name")?) {
            "mini_map_rotation" => {
                let radians = f32_field(message, "rotation")?;
                let (success, _) = self
                    .interface
                    .unsynced_ctrl()
                    .set_mini_map_rotation(radians)
                    .map_err(|err| format!("set_mini_map_rotation({radians}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err(format!("set_mini_map_rotation({radians}) returned false"))
                }
            }
            "los_view_colors" => {
                let color = |r, g, b| -> Result<sys::RgbColor, String> {
                    Ok(sys::RgbColor {
                        r: f32_field(message, r)?,
                        g: f32_field(message, g)?,
                        b: f32_field(message, b)?,
                    })
                };
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_los_view_colors(
                        color("alwaysR", "alwaysG", "alwaysB")?,
                        color("losR", "losG", "losB")?,
                        color("radarR", "radarG", "radarB")?,
                        color("jamR", "jamG", "jamB")?,
                        color("radar2R", "radar2G", "radar2B")?,
                    )
                    .map_err(|err| format!("set_los_view_colors() failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_los_view_colors returned false".to_string())
                }
            }
            name => Err(format!("unsupported display setter `{name}`")),
        }
    }
}

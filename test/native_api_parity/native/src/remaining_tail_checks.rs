use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_remaining_tail(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "set_factory_bugger_off" => {
                self.same_bool_if_present(label, message, "perform", true)?;
                self.same_if_present(label, message, "offset", 128.0)?;
                self.same_if_present(label, message, "radius", 256.0)?;
                self.same_i32_if_present(label, message, "relHeading", 0)?;
                self.same_bool_if_present(label, message, "spherical", true)?;
                self.same_bool_if_present(label, message, "forced", true)
            }
            "kill_team" => {
                let team_id = i32_field(message, "teamID")?;
                let info = self
                    .interface
                    .teams()
                    .get_team_info(team_id, false)
                    .map_err(|err| format!("get_team_info({team_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "isDead", info.isDead)
            }
            "game_over" => {
                let game_over = self
                    .interface
                    .game()
                    .is_game_over()
                    .map_err(|err| format!("is_game_over() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "accepted", 1)?;
                self.same_bool_if_present(label, message, "gameOver", game_over)
            }
            "set_window_geometry" => {
                self.same_bool_if_present(label, message, "called", true)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "set_window_minimized" | "set_window_maximized" => {
                self.same_bool_if_present(
                    label,
                    message,
                    "result",
                    bool_field(message, "expected")?,
                )
            }
            "yield" => self.same_bool_if_present(label, message, "result", false),
            _ => Err(format!("unsupported remaining-tail check `{label}`")),
        }
    }

    pub(crate) fn set_remaining_tail(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(test_name_field(message)?) {
            "get_factory_bugger_off" => {
                let unit_id = i32_field(message, "factoryID")?;
                let (perform, offset, radius, rel_heading, spherical, forced) = self
                    .interface
                    .units_commands()
                    .get_factory_bugger_off(unit_id)
                    .map_err(|err| format!("get_factory_bugger_off({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present("get_factory_bugger_off", message, "perform", perform)?;
                self.same_if_present("get_factory_bugger_off", message, "offset", offset)?;
                self.same_if_present("get_factory_bugger_off", message, "radius", radius)?;
                self.same_i32_if_present("get_factory_bugger_off", message, "relHeading", rel_heading)?;
                self.same_bool_if_present("get_factory_bugger_off", message, "spherical", spherical)?;
                self.same_bool_if_present("get_factory_bugger_off", message, "forced", forced)
            }
            "set_factory_bugger_off" => {
                let unit_id = i32_field(message, "factoryID")?;
                let perform = bool_field(message, "perform")?;
                let returned = self
                    .interface
                    .synced_ctrl()
                    .unit()
                    .set_factory_bugger_off(
                        unit_id,
                        spring_native::SetFactoryBuggerOffOptions {
                            perform,
                            offset: f32_field(message, "offset")?,
                            radius: f32_field(message, "radius")?,
                            rel_heading: i32_field(message, "relHeading")?,
                            spherical: bool_field(message, "spherical")?,
                            forced: bool_field(message, "forced")?,
                        },
                    )
                    .map_err(|err| {
                        format!("set_factory_bugger_off({unit_id}) failed: {err:?}")
                    })?;
                if returned != perform {
                    return Err(format!(
                        "set_factory_bugger_off returned {returned}, expected {perform}"
                    ));
                }
                Ok(())
            }
            "kill_team" => {
                let team_id = i32_field(message, "teamID")?;
                let success = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .kill_team(team_id)
                    .map_err(|err| format!("kill_team({team_id}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("kill_team returned false".to_string())
                }
            }
            "game_over" => {
                let winning_ally_team = i32_field(message, "winningAllyTeamID")?;
                let success = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .game_over(&[winning_ally_team])
                    .map_err(|err| format!("game_over({winning_ally_team}) failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("game_over returned false".to_string())
                }
            }
            "set_window_geometry" => {
                let display_index = i32_field(message, "displayIndex")? - 1;
                if display_index < 0 {
                    return Err("set_window_geometry requires a one-based Lua display index".to_string());
                }
                let success = self
                    .interface
                    .unsynced_ctrl()
                    .set_window_geometry(
                        display_index,
                        i32_field(message, "windowPosX")?,
                        i32_field(message, "windowPosY")?,
                        i32_field(message, "windowSizeX")?,
                        i32_field(message, "windowSizeY")?,
                        spring_native::SetWindowGeometryOptions {
                            full_screen: bool_field(message, "fullScreen")?,
                            borderless: bool_field(message, "borderless")?,
                        },
                    )
                    .map_err(|err| format!("set_window_geometry failed: {err:?}"))?;
                if success {
                    Ok(())
                } else {
                    Err("set_window_geometry returned false".to_string())
                }
            }
            "set_window_minimized" => {
                self.interface
                    .unsynced_ctrl()
                    .set_window_minimized()
                    .map_err(|err| format!("set_window_minimized first call failed: {err:?}"))?;
                let result = self
                    .interface
                    .unsynced_ctrl()
                    .set_window_minimized()
                    .map_err(|err| format!("set_window_minimized failed: {err:?}"))?;
                if result == bool_field(message, "expected")? {
                    Ok(())
                } else {
                    Err(format!(
                        "set_window_minimized returned {result}, expected {}",
                        bool_field(message, "expected")?
                    ))
                }
            }
            "set_window_maximized" => {
                self.interface
                    .unsynced_ctrl()
                    .set_window_maximized()
                    .map_err(|err| format!("set_window_maximized first call failed: {err:?}"))?;
                let result = self
                    .interface
                    .unsynced_ctrl()
                    .set_window_maximized()
                    .map_err(|err| format!("set_window_maximized failed: {err:?}"))?;
                if result == bool_field(message, "expected")? {
                    Ok(())
                } else {
                    Err(format!(
                        "set_window_maximized returned {result}, expected {}",
                        bool_field(message, "expected")?
                    ))
                }
            }
            "yield" => {
                let result = self
                    .interface
                    .system_control()
                    .r#yield()
                    .map_err(|err| format!("yield failed: {err:?}"))?;
                if result == bool_field(message, "expected")? {
                    Ok(())
                } else {
                    Err(format!(
                        "yield returned {result}, expected {}",
                        bool_field(message, "expected")?
                    ))
                }
            }
            name => Err(format!("unsupported remaining-tail setter `{name}`")),
        }
    }
}

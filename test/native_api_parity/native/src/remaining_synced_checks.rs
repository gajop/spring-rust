use super::*;
use crate::support::*;

fn require_success(label: &str, success: bool) -> Result<(), String> {
    if success {
        Ok(())
    } else {
        Err(format!("{label}: native call returned false"))
    }
}

impl NativeApiParity {
    pub(crate) fn check_remaining_synced(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "add_height_map" => {
                let native = self
                    .interface
                    .terrain()
                    .get_ground_height(f32_field(message, "x")?, f32_field(message, "z")?)
                    .map_err(|err| format!("get_ground_height() failed: {err:?}"))?;
                self.same_if_present(label, message, "height", native)
            }
            "add_original_height_map" | "set_original_height_map" => {
                let native = self
                    .interface
                    .terrain()
                    .get_ground_orig_height(f32_field(message, "x")?, f32_field(message, "z")?)
                    .map_err(|err| format!("get_ground_orig_height() failed: {err:?}"))?;
                self.same_if_present(label, message, "height", native)
            }
            "add_smooth_mesh" | "set_smooth_mesh" => {
                let native = self
                    .interface
                    .terrain()
                    .get_smooth_mesh_height(f32_field(message, "x")?, f32_field(message, "z")?)
                    .map_err(|err| format!("get_smooth_mesh_height() failed: {err:?}"))?;
                self.same_if_present(label, message, "height", native)
            }
            "add_team_resource_excess_stats" => self.check_team_resource_stats(message, label),
            "share_team_resource" => self.check_team_resources(message, label),
            "set_ally" => self.check_teams_bool(message, label),
            "assign_player_to_team" => self.check_player_info(message, label),
            "set_cheating_enabled" => {
                let native = self
                    .interface
                    .game()
                    .is_cheating_enabled()
                    .map_err(|err| format!("is_cheating_enabled() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "enabled", native)
            }
            "set_god_mode" => {
                let native = self
                    .interface
                    .game()
                    .is_god_mode_enabled()
                    .map_err(|err| format!("is_god_mode_enabled() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "enabled", native)
            }
            "set_experience_grade" | "set_no_pause" | "set_square_building_mask" => {
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "set_player_ready_state" => {
                let native = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .set_player_ready_state(
                        i32_field(message, "playerID")?,
                        bool_field(message, "ready")?,
                    )
                    .map_err(|err| format!("set_player_ready_state() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", native)
            }
            "transfer_team_max_units" => {
                let native = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .transfer_team_max_units(
                        i32_field(message, "fromTeamID")?,
                        i32_field(message, "newTeamID")?,
                        i32_field(message, "transferAmnt")?,
                    )
                    .map_err(|err| format!("transfer_team_max_units() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", native)
            }
            _ => Err(format!("unsupported remaining synced check `{label}`")),
        }
    }

    pub(crate) fn set_remaining_synced(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(test_name_field(message)?) {
            "add_height_map" => {
                let x = f32_field(message, "x")?;
                let z = f32_field(message, "z")?;
                let delta = f32_field(message, "delta")?;
                let synced_ctrl = self.interface.synced_ctrl();
                let terrain = synced_ctrl.terrain();
                let mut add_result = Ok(false);
                let callback_result = terrain
                    .set_height_map_func(|| {
                        add_result = terrain.add_height_map(x, z, delta);
                    })
                    .map_err(|err| format!("set_height_map_func() failed: {err:?}"))?;
                require_success("set_height_map_func()", callback_result)?;
                require_success(
                    "add_height_map()",
                    add_result.map_err(|err| format!("add_height_map() failed: {err:?}"))?,
                )
            }
            "add_original_height_map" => {
                let x = f32_field(message, "x")?;
                let z = f32_field(message, "z")?;
                let delta = f32_field(message, "delta")?;
                let synced_ctrl = self.interface.synced_ctrl();
                let terrain = synced_ctrl.terrain();
                let mut add_result = Ok(false);
                let callback_result = terrain
                    .set_original_height_map_func(|| {
                        add_result = terrain.add_original_height_map(x, z, delta);
                    })
                    .map_err(|err| format!("set_original_height_map_func() failed: {err:?}"))?;
                require_success("set_original_height_map_func()", callback_result)?;
                require_success(
                    "add_original_height_map()",
                    add_result
                        .map_err(|err| format!("add_original_height_map() failed: {err:?}"))?,
                )
            }
            "set_original_height_map" => {
                let x = f32_field(message, "x")?;
                let z = f32_field(message, "z")?;
                let height = f32_field(message, "height")?;
                let factor = f32_field(message, "factor")?;
                let synced_ctrl = self.interface.synced_ctrl();
                let terrain = synced_ctrl.terrain();
                let mut set_result = Ok(false);
                let callback_result = terrain
                    .set_original_height_map_func(|| {
                        set_result = terrain.set_original_height_map(x, z, height, factor);
                    })
                    .map_err(|err| format!("set_original_height_map_func() failed: {err:?}"))?;
                require_success("set_original_height_map_func()", callback_result)?;
                require_success(
                    "set_original_height_map()",
                    set_result
                        .map_err(|err| format!("set_original_height_map() failed: {err:?}"))?,
                )
            }
            "add_smooth_mesh" => {
                let x = f32_field(message, "x")?;
                let z = f32_field(message, "z")?;
                let delta = f32_field(message, "delta")?;
                let synced_ctrl = self.interface.synced_ctrl();
                let terrain = synced_ctrl.terrain();
                let mut add_result = Ok(false);
                let callback_result = terrain
                    .set_smooth_mesh_func(|| {
                        add_result = terrain.add_smooth_mesh(x, z, delta);
                    })
                    .map_err(|err| format!("set_smooth_mesh_func() failed: {err:?}"))?;
                require_success("set_smooth_mesh_func()", callback_result)?;
                require_success(
                    "add_smooth_mesh()",
                    add_result.map_err(|err| format!("add_smooth_mesh() failed: {err:?}"))?,
                )
            }
            "set_smooth_mesh" => {
                let x = f32_field(message, "x")?;
                let z = f32_field(message, "z")?;
                let height = f32_field(message, "height")?;
                let terraform = f32_field(message, "terraform")?;
                let synced_ctrl = self.interface.synced_ctrl();
                let terrain = synced_ctrl.terrain();
                let mut set_result = Ok(false);
                let callback_result = terrain
                    .set_smooth_mesh_func(|| {
                        set_result = terrain.set_smooth_mesh(x, z, height, terraform);
                    })
                    .map_err(|err| format!("set_smooth_mesh_func() failed: {err:?}"))?;
                require_success("set_smooth_mesh_func()", callback_result)?;
                require_success(
                    "set_smooth_mesh()",
                    set_result.map_err(|err| format!("set_smooth_mesh() failed: {err:?}"))?,
                )
            }
            "add_team_resource_excess_stats" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .add_team_resource_excess_stats(
                        i32_field(message, "teamID")?,
                        str_field(message, "resource")?,
                        f32_field(message, "amount")?,
                    )
                    .map_err(|err| format!("add_team_resource_excess_stats() failed: {err:?}"))?;
                require_success("add_team_resource_excess_stats()", success)
            }
            "share_team_resource" => {
                let resource = str_field(message, "resource")?;
                let synced_ctrl = self.interface.synced_ctrl();
                let team = synced_ctrl.team();
                let set_success = team
                    .set_team_resource(
                        i32_field(message, "teamID")?,
                        resource,
                        f32_field(message, "baseline")?,
                    )
                    .map_err(|err| format!("set_team_resource() failed: {err:?}"))?;
                require_success("set_team_resource()", set_success)?;
                let success = team
                    .share_team_resource(
                        i32_field(message, "teamID")?,
                        i32_field(message, "targetTeamID")?,
                        resource,
                        f32_field(message, "amount")?,
                    )
                    .map_err(|err| format!("share_team_resource() failed: {err:?}"))?;
                require_success("share_team_resource()", success)
            }
            "set_ally" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .set_ally(
                        i32_field(message, "firstAllyTeamID")?,
                        i32_field(message, "secondAllyTeamID")?,
                        bool_field(message, "allied")?,
                    )
                    .map_err(|err| format!("set_ally() failed: {err:?}"))?;
                require_success("set_ally()", success)
            }
            "assign_player_to_team" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .assign_player_to_team(
                        i32_field(message, "playerID")?,
                        i32_field(message, "teamID")?,
                    )
                    .map_err(|err| format!("assign_player_to_team() failed: {err:?}"))?;
                require_success("assign_player_to_team()", success)
            }
            "set_cheating_enabled" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .game_config()
                    .set_cheating_enabled(bool_field(message, "enabled")?)
                    .map_err(|err| format!("set_cheating_enabled() failed: {err:?}"))?;
                require_success("set_cheating_enabled()", success)
            }
            "set_god_mode" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .game_config()
                    .set_god_mode(spring_native::SetGodModeOptions {
                        control_allies: bool_field(message, "controlAllies")?,
                        control_enemies: bool_field(message, "controlEnemies")?,
                    })
                    .map_err(|err| format!("set_god_mode() failed: {err:?}"))?;
                require_success("set_god_mode()", success)
            }
            "set_experience_grade" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .game_config()
                    .set_experience_grade(
                        f32_field(message, "expGrade")?,
                        f32_field(message, "expPowerScale")?,
                        f32_field(message, "expHealthScale")?,
                        f32_field(message, "expReloadScale")?,
                    )
                    .map_err(|err| format!("set_experience_grade() failed: {err:?}"))?;
                require_success("set_experience_grade()", success)
            }
            "set_no_pause" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .game_config()
                    .set_no_pause(bool_field(message, "noPause")?)
                    .map_err(|err| format!("set_no_pause() failed: {err:?}"))?;
                require_success("set_no_pause()", success)
            }
            "set_player_ready_state" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .set_player_ready_state(
                        i32_field(message, "playerID")?,
                        bool_field(message, "ready")?,
                    )
                    .map_err(|err| format!("set_player_ready_state() failed: {err:?}"))?;
                require_success("set_player_ready_state()", success)
            }
            "set_square_building_mask" => {
                let mask = u16::try_from(i32_field(message, "mask")?)
                    .map_err(|_| "mask is outside the native u16 range".to_owned())?;
                let success = self
                    .interface
                    .synced_ctrl()
                    .game_config()
                    .set_square_building_mask(
                        i32_field(message, "x")?,
                        i32_field(message, "z")?,
                        mask,
                    )
                    .map_err(|err| format!("set_square_building_mask() failed: {err:?}"))?;
                require_success("set_square_building_mask()", success)
            }
            "transfer_team_max_units" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .team()
                    .transfer_team_max_units(
                        i32_field(message, "fromTeamID")?,
                        i32_field(message, "newTeamID")?,
                        i32_field(message, "transferAmnt")?,
                    )
                    .map_err(|err| format!("transfer_team_max_units() failed: {err:?}"))?;
                require_success("transfer_team_max_units()", success)
            }
            name => Err(format!("unsupported remaining synced setter `{name}`")),
        }
    }
}

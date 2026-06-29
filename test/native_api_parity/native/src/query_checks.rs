use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_units_query_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_query()
            .valid_unit_id(unit_id)
            .map_err(|err| format!("valid_unit_id({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "valid", native)
    }
    pub(crate) fn check_units_query_count(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let team_id = i32_field(message, "teamID")?;
        let native = match test_name {
            "get_team_unit_count" => self.interface.units_query().get_team_unit_count(team_id)
                .map_err(|err| format!("get_team_unit_count({team_id}) failed: {err:?}"))?,
            "get_team_unit_def_count" => {
                let unit_def_id = i32_field(message, "unitDefID")?;
                self.interface.units_query().get_team_unit_def_count(team_id, unit_def_id)
                    .map_err(|err| format!("get_team_unit_def_count({team_id}, {unit_def_id}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported units query count check `{label}`")),
        };
        self.same_i32_if_present(label, message, "count", native as i32)
    }
    pub(crate) fn check_units_query_list(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_all_units" => self.interface.units_query().get_all_units()
                .map_err(|err| format!("get_all_units() failed: {err:?}"))?,
            "get_team_units" => {
                let team_id = i32_field(message, "teamID")?;
                self.interface.units_query().get_team_units(team_id)
                    .map_err(|err| format!("get_team_units({team_id}) failed: {err:?}"))?
            }
            "get_team_units_by_defs" => {
                let team_id = i32_field(message, "teamID")?;
                let unit_def_id = i32_field(message, "unitDefID")?;
                self.interface.units_query().get_team_units_by_defs(team_id, &[unit_def_id])
                    .map_err(|err| format!("get_team_units_by_defs({team_id}, [{unit_def_id}]) failed: {err:?}"))?
            }
            "get_render_units" => {
                let draw_mask = i32_field(message, "drawMask")?;
                let send_mask = bool_field(message, "sendMask")?;
                self.interface.units_query().get_render_units(draw_mask, send_mask)
                    .map_err(|err| format!("get_render_units({draw_mask}, {send_mask}) failed: {err:?}"))?
            }
            "get_render_units_draw_flag_changed" => {
                let send_mask = bool_field(message, "sendMask")?;
                self.interface.units_query().get_render_units_draw_flag_changed(send_mask)
                    .map_err(|err| format!("get_render_units_draw_flag_changed({send_mask}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported units query list check `{label}`")),
        };
        if test_name == "get_team_units_by_defs" {
            return self.same_i32_set_if_present(label, message, "unitIDs", &native);
        }
        self.same_i32_list_if_present(label, message, "unitIDs", &native)
    }
    pub(crate) fn check_units_query_counts(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let native = self.interface.units_query().get_team_units_counts(team_id)
            .map_err(|err| format!("get_team_units_counts({team_id}) failed: {err:?}"))?;
        self.same_unit_def_counts_if_present(label, message, "counts", &native)
    }
    pub(crate) fn check_team_units_sorted(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let native = self.interface.units_query().get_team_units_sorted(team_id)
            .map_err(|err| format!("get_team_units_sorted({team_id}) failed: {err:?}"))?;
        self.same_team_units_by_def_if_present(label, message, "groups", &native)
    }
    pub(crate) fn check_units_query_set(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_units_in_rectangle" => self.interface.units_query().get_units_in_rectangle(
                f32_field(message, "minX")?,
                f32_field(message, "minZ")?,
                f32_field(message, "maxX")?,
                f32_field(message, "maxZ")?,
                i32_field(message, "allegiance")?,
            )
                .map_err(|err| format!("get_units_in_rectangle() failed: {err:?}"))?,
            "get_units_in_box" => self.interface.units_query().get_units_in_box(
                f32_field(message, "minX")?,
                f32_field(message, "minY")?,
                f32_field(message, "minZ")?,
                f32_field(message, "maxX")?,
                f32_field(message, "maxY")?,
                f32_field(message, "maxZ")?,
                i32_field(message, "allegiance")?,
            )
                .map_err(|err| format!("get_units_in_box() failed: {err:?}"))?,
            "get_units_in_sphere" => self.interface.units_query().get_units_in_sphere(
                f32_field(message, "x")?,
                f32_field(message, "y")?,
                f32_field(message, "z")?,
                f32_field(message, "radius")?,
                i32_field(message, "allegiance")?,
            )
                .map_err(|err| format!("get_units_in_sphere() failed: {err:?}"))?,
            "get_units_in_cylinder" => self.interface.units_query().get_units_in_cylinder(
                f32_field(message, "x")?,
                f32_field(message, "z")?,
                f32_field(message, "radius")?,
                i32_field(message, "allegiance")?,
            )
                .map_err(|err| format!("get_units_in_cylinder() failed: {err:?}"))?,
            _ => return Err(format!("unsupported units query set check `{label}`")),
        };
        self.same_i32_set_if_present(label, message, "unitIDs", &native)
    }
    pub(crate) fn check_unit_separation(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id1 = i32_field(message, "unitID1")?;
        let unit_id2 = i32_field(message, "unitID2")?;
        let positional = bool_field(message, "positional")?;
        let check_map = bool_field(message, "checkMap")?;
        let native = self
            .interface
            .units_query()
            .get_unit_separation(unit_id1, unit_id2, positional, check_map)
            .map_err(|err| format!("get_unit_separation({unit_id1}, {unit_id2}, {positional}, {check_map}) failed: {err:?}"))?;
        self.same_if_present(label, message, "separation", native)
    }
    pub(crate) fn check_unit_feature_separation(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let feature_id = i32_field(message, "featureID")?;
        let surface_dist = bool_field(message, "surfaceDist")?;
        let native = self
            .interface
            .synced_ctrl()
            .unit()
            .get_unit_feature_separation(unit_id, feature_id, surface_dist)
            .map_err(|err| format!("get_unit_feature_separation({unit_id}, {feature_id}, {surface_dist}) failed: {err:?}"))?;
        self.same_if_present(label, message, "separation", native)
    }
    pub(crate) fn check_unit_centroid(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_unit_array_centroid" => self.interface.units_query().get_unit_array_centroid(&[unit_id])
                .map_err(|err| format!("get_unit_array_centroid([{unit_id}]) failed: {err:?}"))?,
            "get_unit_map_centroid" => self.interface.units_query().get_unit_map_centroid(&[unit_id])
                .map_err(|err| format!("get_unit_map_centroid([{unit_id}]) failed: {err:?}"))?,
            _ => return Err(format!("unsupported unit centroid check `{label}`")),
        };
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_units_query_i32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let (field, native) = match test_name {
            "get_unit_nearest_ally" => {
                let unit_id = i32_field(message, "unitID")?;
                let range = f32_field(message, "range")?;
                ("unitIDResult", self.interface.units_query().get_unit_nearest_ally(unit_id, range)
                    .map_err(|err| format!("get_unit_nearest_ally({unit_id}, {range}) failed: {err:?}"))?)
            }
            "get_unit_nearest_enemy" => {
                let unit_id = i32_field(message, "unitID")?;
                let range = f32_field(message, "range")?;
                let use_los = bool_field(message, "useLOS")?;
                let sphere_dist_test = bool_field(message, "sphereDistTest")?;
                let check_sight_dist = bool_field(message, "checkSightDist")?;
                ("unitIDResult", self.interface.units_query().get_unit_nearest_enemy(unit_id, range, use_los, sphere_dist_test, check_sight_dist)
                    .map_err(|err| format!("get_unit_nearest_enemy({unit_id}, {range}, {use_los}, {sphere_dist_test}, {check_sight_dist}) failed: {err:?}"))?)
            }
            "get_closest_enemy_unit" => {
                let pos = vec3_from_fields(message, "x", "y", "z")?;
                let range = f32_field(message, "range")?;
                let ally_team_id = i32_field(message, "allyTeamID")?;
                let use_los = bool_field(message, "useLOS")?;
                let sphere_dist_test = bool_field(message, "sphereDistTest")?;
                let check_sight_dist = bool_field(message, "checkSightDist")?;
                ("unitIDResult", self.interface.units_query().get_closest_enemy_unit(pos, range, ally_team_id, use_los, sphere_dist_test, check_sight_dist)
                    .map_err(|err| format!("get_closest_enemy_unit(_, {range}, {ally_team_id}, {use_los}, {sphere_dist_test}, {check_sight_dist}) failed: {err:?}"))?)
            }
            _ => return Err(format!("unsupported units query i32 check `{label}`")),
        };
        self.same_i32_if_present(label, message, field, native)
    }
    pub(crate) fn check_features_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .valid_feature_id(feature_id)
            .map_err(|err| format!("valid_feature_id({feature_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "valid", native)
    }
    pub(crate) fn check_features_list(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let native = self
            .interface
            .features()
            .get_all_features()
            .map_err(|err| format!("get_all_features() failed: {err:?}"))?;
        self.same_i32_list_if_present(label, message, "featureIDs", &native)
    }
    pub(crate) fn check_feature_identity(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let feature_id = i32_field(message, "featureID")?;
        let native = match test_name {
            "feature_def_id" | "get_feature_def_id" => self.interface.features().get_feature_def_id(feature_id)
                .map_err(|err| format!("get_feature_def_id({feature_id}) failed: {err:?}"))?,
            "feature_team" | "get_feature_team" => self.interface.features().get_feature_team(feature_id)
                .map_err(|err| format!("get_feature_team({feature_id}) failed: {err:?}"))?,
            "feature_ally_team" | "get_feature_ally_team" => self.interface.features().get_feature_ally_team(feature_id)
                .map_err(|err| format!("get_feature_ally_team({feature_id}) failed: {err:?}"))?,
            _ => return Err(format!("unsupported feature identity check `{label}`")),
        };
        let field = match test_name {
            "feature_def_id" | "get_feature_def_id" => "defID",
            "feature_team" | "get_feature_team" => "teamID",
            "feature_ally_team" | "get_feature_ally_team" => "allyTeamID",
            _ => unreachable!(),
        };
        self.same_i32_if_present(label, message, field, native)
    }
    pub(crate) fn check_teams_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let native = match test_name {
            "are_teams_allied" => {
                let team_id1 = i32_field(message, "teamID1")?;
                let team_id2 = i32_field(message, "teamID2")?;
                self.interface.teams().are_teams_allied(team_id1, team_id2)
                    .map_err(|err| format!("are_teams_allied({team_id1}, {team_id2}) failed: {err:?}"))?
            }
            "are_players_allied" => {
                let player_id1 = i32_field(message, "playerID1")?;
                let player_id2 = i32_field(message, "playerID2")?;
                self.interface.teams().are_players_allied(player_id1, player_id2)
                    .map_err(|err| format!("are_players_allied({player_id1}, {player_id2}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported teams bool check `{label}`")),
        };
        self.same_bool_if_present(label, message, "allied", native)
    }
    pub(crate) fn check_teams_list(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let (field, native) = match test_name {
            "get_team_list" => {
                let ally_team_id = i32_field(message, "allyTeamID")?;
                ("teamIDs", self.interface.teams().get_team_list(ally_team_id)
                    .map_err(|err| format!("get_team_list({ally_team_id}) failed: {err:?}"))?)
            }
            "get_ally_team_list" => {
                ("allyTeamIDs", self.interface.teams().get_ally_team_list()
                    .map_err(|err| format!("get_ally_team_list() failed: {err:?}"))?)
            }
            "get_player_list" => {
                let team_id = i32_field(message, "teamID")?;
                let active = bool_field(message, "active")?;
                ("playerIDs", self.interface.teams().get_player_list(team_id, active)
                    .map_err(|err| format!("get_player_list({team_id}, {active}) failed: {err:?}"))?)
            }
            "get_player_list_in_team" => {
                let team_id = i32_field(message, "teamID")?;
                ("playerIDs", self.interface.teams().get_player_list_in_team(team_id)
                    .map_err(|err| format!("get_player_list_in_team({team_id}) failed: {err:?}"))?)
            }
            "get_player_list_in_ally_team" => {
                let ally_team_id = i32_field(message, "allyTeamID")?;
                ("playerIDs", self.interface.teams().get_player_list_in_ally_team(ally_team_id)
                    .map_err(|err| format!("get_player_list_in_ally_team({ally_team_id}) failed: {err:?}"))?)
            }
            _ => return Err(format!("unsupported teams list check `{label}`")),
        };
        self.same_i32_list_if_present(label, message, field, &native)
    }
    pub(crate) fn check_teams_i32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let team_id = i32_field(message, "teamID")?;
        let (field, native) = match test_name {
            "get_team_ally_team_id" => ("allyTeamID", self.interface.teams().get_team_ally_team_id(team_id)
                .map_err(|err| format!("get_team_ally_team_id({team_id}) failed: {err:?}"))?),
            "get_team_max_units" => ("maxUnits", self.interface.teams().get_team_max_units(team_id)
                .map_err(|err| format!("get_team_max_units({team_id}) failed: {err:?}"))?),
            _ => return Err(format!("unsupported teams i32 check `{label}`")),
        };
        self.same_i32_if_present(label, message, field, native)
    }
    pub(crate) fn check_team_lua_ai(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let native = self
            .interface
            .teams()
            .get_team_lua_ai(team_id)
            .map_err(|err| format!("get_team_lua_ai({team_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "hasLuaAI", native.is_some())?;
        if let Some(lua_ai) = native {
            self.same_string_if_present(label, message, "luaAI", &lua_ai)?;
        }
        Ok(())
    }
    pub(crate) fn check_ai_info_fixed(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let (info, has_ai) = self
            .interface
            .teams()
            .get_aiinfo(team_id)
            .map_err(|err| format!("get_aiinfo({team_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "hasAI", has_ai)?;
        if has_ai {
            self.same_i32_if_present(label, message, "skirmishAIID", info.skirmishAIID)?;
            self.same_i32_if_present(label, message, "hostingPlayerID", info.hostingPlayerID)?;
            self.same_string_if_present(label, message, "name", &cstr_or_empty(info.name)?)?;
            self.same_string_if_present(label, message, "shortName", &cstr_or_empty(info.shortName)?)?;
            self.same_string_if_present(label, message, "version", &cstr_or_empty(info.version)?)?;
            self.same_i32_if_present(label, message, "optionCount", info.optionCount as i32)?;
        }
        Ok(())
    }
    pub(crate) fn check_team_info(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let native = self
            .interface
            .teams()
            .get_team_info(team_id, false)
            .map_err(|err| format!("get_team_info({team_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "teamID", native.teamID)?;
        self.same_i32_if_present(label, message, "leaderID", native.leaderID)?;
        self.same_bool_if_present(label, message, "isDead", native.isDead)?;
        self.same_i32_if_present(label, message, "allyTeamID", native.allyTeamID)?;
        self.same_string_if_present(label, message, "side", &cstr_or_empty(native.side)?)
    }
    pub(crate) fn check_ally_team_info(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let native = self
            .interface
            .teams()
            .get_ally_team_info(ally_team_id)
            .map_err(|err| format!("get_ally_team_info({ally_team_id}) failed: {err:?}"))?;
        let keys = unsafe {
            std::slice::from_raw_parts(native.keys, native.count as usize)
                .iter()
                .map(|key| CStr::from_ptr(*key).to_string_lossy().into_owned())
                .collect::<Vec<_>>()
        };
        self.same_i32_if_present(label, message, "count", native.count as i32)?;
        self.same_string_set_if_present(label, message, "keys", &keys)
    }
    pub(crate) fn check_player_info(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let player_id = i32_field(message, "playerID")?;
        let native = self
            .interface
            .teams()
            .get_player_info(player_id, false)
            .map_err(|err| format!("get_player_info({player_id}) failed: {err:?}"))?;
        self.same_string_if_present(label, message, "playerName", &cstr_or_empty(native.name)?)?;
        self.same_bool_if_present(label, message, "active", native.isActive)?;
        self.same_bool_if_present(label, message, "spectator", native.isSpec)?;
        self.same_i32_if_present(label, message, "teamID", native.teamID)?;
        self.same_i32_if_present(label, message, "allyTeamID", native.allyTeamID)
    }
    pub(crate) fn check_player_controlled_unit(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let player_id = i32_field(message, "playerID")?;
        let (unit_id, has_unit) = self
            .interface
            .teams()
            .get_player_controlled_unit(player_id)
            .map_err(|err| format!("get_player_controlled_unit({player_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "hasUnit", has_unit)?;
        if has_unit {
            self.same_i32_if_present(label, message, "unitIDResult", unit_id)?;
        }
        Ok(())
    }
    pub(crate) fn check_team_resources(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let resource = str_field(message, "resource")?;
        let native = self
            .interface
            .teams()
            .get_team_resources(team_id, resource)
            .map_err(|err| format!("get_team_resources({team_id}, {resource}) failed: {err:?}"))?;
        if resource.starts_with('m') {
            self.same_if_present(label, message, "currentLevel", native.metalCurrent)?;
            self.same_if_present(label, message, "storage", native.metalStorage)?;
            self.same_if_present(label, message, "share", native.metalShared)
        } else if resource.starts_with('e') {
            self.same_if_present(label, message, "currentLevel", native.energyCurrent)?;
            self.same_if_present(label, message, "storage", native.energyStorage)?;
            self.same_if_present(label, message, "share", native.energyShared)
        } else {
            Err(format!("{label}.resource: unsupported resource `{resource}`"))
        }
    }
    pub(crate) fn check_team_resource_stats(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let resource = str_field(message, "resource")?;
        let native = self
            .interface
            .teams()
            .get_team_resource_stats(team_id, resource)
            .map_err(|err| format!("get_team_resource_stats({team_id}, {resource}) failed: {err:?}"))?;
        if resource.starts_with('m') {
            self.same_if_present(label, message, "used", native.metalCurrent)?;
            self.same_if_present(label, message, "produced", native.metalStorage)?;
            self.same_if_present(label, message, "excessed", native.metalPull)?;
            self.same_if_present(label, message, "received", native.metalIncome)?;
            self.same_if_present(label, message, "sent", native.metalExpense)
        } else if resource.starts_with('e') {
            self.same_if_present(label, message, "used", native.energyCurrent)?;
            self.same_if_present(label, message, "produced", native.energyStorage)?;
            self.same_if_present(label, message, "excessed", native.energyPull)?;
            self.same_if_present(label, message, "received", native.energyIncome)?;
            self.same_if_present(label, message, "sent", native.energyExpense)
        } else {
            Err(format!("{label}.resource: unsupported resource `{resource}`"))
        }
    }
    pub(crate) fn check_team_unit_stats(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let native = self
            .interface
            .teams()
            .get_team_unit_stats(team_id)
            .map_err(|err| format!("get_team_unit_stats({team_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "killed", native.killed as i32)?;
        self.same_i32_if_present(label, message, "died", native.died as i32)?;
        self.same_i32_if_present(label, message, "capturedBy", native.capturedBy as i32)?;
        self.same_i32_if_present(label, message, "capturedFrom", native.capturedFrom as i32)?;
        self.same_i32_if_present(label, message, "received", native.received as i32)?;
        self.same_i32_if_present(label, message, "sent", native.sent as i32)
    }
    pub(crate) fn check_team_stats_history_count(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let start_index = i32_field(message, "startIndex")?;
        let end_index = i32_field(message, "endIndex")?;
        let native = self
            .interface
            .teams()
            .get_team_stats_history(team_id, start_index, end_index)
            .map_err(|err| format!("get_team_stats_history({team_id}, {start_index}, {end_index}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "count", native.len() as i32)
    }
    pub(crate) fn check_team_damage_stats(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let (damage_dealt, damage_received, success) = self
            .interface
            .unsynced_read()
            .get_team_damage_stats(team_id)
            .map_err(|err| format!("get_team_damage_stats({team_id}) failed: {err:?}"))?;
        if !success {
            return Ok(());
        }
        self.same_if_present(label, message, "damageDealt", damage_dealt)?;
        self.same_if_present(label, message, "damageReceived", damage_received)
    }
    pub(crate) fn set_team_resource(&mut self, message: &Value) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let resource = str_field(message, "resource")?;
        self.interface
            .synced_ctrl()
            .team()
            .set_team_resource(team_id, resource, f32_field(message, "amount")?)
            .map_err(|err| format!("set_team_resource({team_id}, {resource}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_team_share_level(&mut self, message: &Value) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let resource = str_field(message, "resource")?;
        let share = f32_field(message, "share")?;
        self.interface
            .synced_ctrl()
            .team()
            .set_team_share_level(team_id, resource, share)
            .map_err(|err| format!("set_team_share_level({team_id}, {resource}, {share}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_ally_team_start_box(&mut self, message: &Value) -> Result<(), String> {
        let ally_team_id = i32_field(message, "allyTeamID")?;
        self.interface
            .synced_ctrl()
            .team()
            .set_ally_team_start_box(
                ally_team_id,
                f32_field(message, "xMin")?,
                f32_field(message, "zMin")?,
                f32_field(message, "xMax")?,
                f32_field(message, "zMax")?,
            )
            .map_err(|err| format!("set_ally_team_start_box({ally_team_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_team_start_position(&mut self, message: &Value) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let pos = vec3_from_fields(message, "x", "y", "z")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let team = synced_ctrl.team();
        team.set_ally_team_start_box(ally_team_id, 0.0, 0.0, 2000.0, 2000.0)
            .map_err(|err| format!("set_ally_team_start_box({ally_team_id}) failed: {err:?}"))?;
        team.set_team_start_position(team_id, pos)
            .map_err(|err| format!("set_team_start_position({team_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn add_team_resource(&mut self, message: &Value) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let resource = str_field(message, "resource")?;
        let baseline = f32_field(message, "baseline")?;
        let amount = f32_field(message, "amount")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let team = synced_ctrl.team();
        team.set_team_resource(team_id, resource, baseline)
            .map_err(|err| format!("set_team_resource({team_id}, {resource}, {baseline}) failed: {err:?}"))?;
        team.add_team_resource(team_id, resource, amount)
            .map_err(|err| format!("add_team_resource({team_id}, {resource}, {amount}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn use_team_resource(&mut self, message: &Value) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let resource = str_field(message, "resource")?;
        let baseline = f32_field(message, "baseline")?;
        let amount = f32_field(message, "amount")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let team = synced_ctrl.team();
        team.set_team_resource(team_id, resource, baseline)
            .map_err(|err| format!("set_team_resource({team_id}, {resource}, {baseline}) failed: {err:?}"))?;
        team.use_team_resource(team_id, resource, amount)
            .map_err(|err| format!("use_team_resource({team_id}, {resource}, {amount}) failed: {err:?}"))?;
        Ok(())
    }
}

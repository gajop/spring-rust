use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_unit_extra_read(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "get_unit_current_build_power" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .units_info()
                    .get_unit_current_build_power(unit_id)
                    .map_err(|err| {
                        format!("get_unit_current_build_power({unit_id}) failed: {err:?}")
                    })?;
                self.same_if_present(label, message, "buildPower", native)
            }
            "get_unit_move_def_id" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .units_info()
                    .get_unit_move_def_id(unit_id)
                    .map_err(|err| format!("get_unit_move_def_id({unit_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "moveDefID", native)
            }
            "get_unit_travel" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .units_info()
                    .get_unit_travel(unit_id)
                    .map_err(|err| format!("get_unit_travel({unit_id}) failed: {err:?}"))?;
                self.same_if_present(label, message, "travelPeriod", native.travelPeriod)?;
                self.same_if_present(label, message, "travelTime", native.travelTime)
            }
            "get_unit_fuel" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .units_info()
                    .get_unit_fuel(unit_id)
                    .map_err(|err| format!("get_unit_fuel({unit_id}) failed: {err:?}"))?;
                self.same_if_present(label, message, "fuel", native.fuel)?;
                self.same_if_present(label, message, "maxFuel", native.maxFuel)
            }
            "get_unit_move_type_data" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .move_ctrl()
                    .get_unit_move_type_data(unit_id)
                    .map_err(|err| format!("get_unit_move_type_data({unit_id}) failed: {err:?}"))?;
                let name = unsafe {
                    if native.name.is_null() {
                        ""
                    } else {
                        CStr::from_ptr(native.name).to_str().unwrap_or("")
                    }
                };
                self.same_string_if_present(label, message, "moveTypeName", name)?;
                self.same_if_present(label, message, "maxSpeed", native.maxSpeed)?;
                self.same_if_present(label, message, "maxWantedSpeed", native.maxWantedSpeed)?;
                self.same_if_present(label, message, "goalx", native.goalX)?;
                self.same_if_present(label, message, "goaly", native.goalY)?;
                self.same_if_present(label, message, "goalz", native.goalZ)?;
                self.same_if_present(label, message, "turnRate", native.turnRate)?;
                self.same_if_present(label, message, "accRate", native.accRate)?;
                self.same_if_present(label, message, "decRate", native.decRate)?;
                self.same_if_present(label, message, "maxReverseSpeed", native.maxReverseSpeed)?;
                self.same_if_present(label, message, "wantedSpeed", native.wantedSpeed)?;
                self.same_if_present(label, message, "currentSpeed", native.currentSpeed)?;
                self.same_if_present(label, message, "deltaSpeed", native.deltaSpeed)
            }
            "get_unit_estimated_path" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .move_ctrl()
                    .get_unit_estimated_path(unit_id)
                    .map_err(|err| format!("get_unit_estimated_path({unit_id}) failed: {err:?}"))?;
                let (waypoints, starts) = native;
                self.same_i32_if_present(label, message, "count", waypoints.len() as i32)?;
                self.same_i32_list_if_present(label, message, "starts", &starts)
            }
            name => Err(format!("unsupported unit extra read check `{name}`")),
        }
    }

    pub(crate) fn check_unit_weapon_extra(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let weapon_num = i32_field(message, "weaponNum")?;
        match base_test_name(label) {
            "get_unit_weapon_vectors" => {
                let native = self
                    .interface
                    .units_weapons()
                    .get_unit_weapon_vectors(unit_id, weapon_num)
                    .map_err(|err| {
                        format!("get_unit_weapon_vectors({unit_id}, {weapon_num}) failed: {err:?}")
                    })?;
                self.same_if_present(label, message, "posX", native.weaponMuzzlePos.x)?;
                self.same_if_present(label, message, "posY", native.weaponMuzzlePos.y)?;
                self.same_if_present(label, message, "posZ", native.weaponMuzzlePos.z)?;
                self.same_if_present(label, message, "dirX", native.weaponDir.x)?;
                self.same_if_present(label, message, "dirY", native.weaponDir.y)?;
                self.same_if_present(label, message, "dirZ", native.weaponDir.z)
            }
            "get_unit_weapon_target" => {
                let native = self
                    .interface
                    .units_weapons()
                    .get_unit_weapon_target(unit_id, weapon_num)
                    .map_err(|err| {
                        format!("get_unit_weapon_target({unit_id}, {weapon_num}) failed: {err:?}")
                    })?;
                self.same_i32_if_present(label, message, "targetType", native.targetType)?;
                self.same_i32_if_present(label, message, "targetID", native.targetID)?;
                self.same_if_present(label, message, "targetX", native.targetPos.x)?;
                self.same_if_present(label, message, "targetY", native.targetPos.y)?;
                self.same_if_present(label, message, "targetZ", native.targetPos.z)
            }
            "get_unit_weapon_try_target"
            | "get_unit_weapon_test_target"
            | "get_unit_weapon_have_free_line_of_fire" => {
                let target_id = i32_field(message, "targetID")?;
                let is_ground_target = bool_field(message, "isGroundTarget")?;
                let test_name = base_test_name(label);
                let native = match test_name {
                    "get_unit_weapon_try_target" => {
                        let target_pos = vec3_from_fields(message, "x", "y", "z")?;
                        self.interface
                            .units_weapons()
                            .get_unit_weapon_try_target(
                                unit_id,
                                weapon_num,
                                target_id,
                                target_pos,
                                spring_native::GetUnitWeaponTryTargetOptions {
                                    user_target: bool_field(message, "userTarget")?,
                                    is_ground_target,
                                },
                            )
                            .map_err(|err| {
                                format!("get_unit_weapon_try_target() failed: {err:?}")
                            })?
                    }
                    "get_unit_weapon_test_target" => {
                        let target_pos = vec3_from_fields(message, "x", "y", "z")?;
                        self.interface
                            .units_weapons()
                            .get_unit_weapon_test_target(
                                unit_id,
                                weapon_num,
                                target_id,
                                target_pos,
                                spring_native::GetUnitWeaponTestTargetOptions { is_ground_target },
                            )
                            .map_err(|err| {
                                format!("get_unit_weapon_test_target() failed: {err:?}")
                            })?
                    }
                    "get_unit_weapon_have_free_line_of_fire" => {
                        let source_pos =
                            vec3_from_fields(message, "sourceX", "sourceY", "sourceZ")?;
                        let target_pos =
                            vec3_from_fields(message, "targetX", "targetY", "targetZ")?;
                        self.interface
                            .units_weapons()
                            .get_unit_weapon_have_free_line_of_fire(
                                unit_id,
                                weapon_num,
                                target_id,
                                source_pos,
                                target_pos,
                                spring_native::GetUnitWeaponHaveFreeLineOfFireOptions {
                                    is_ground_target,
                                },
                            )
                            .map_err(|err| {
                                format!("get_unit_weapon_have_free_line_of_fire() failed: {err:?}")
                            })?
                    }
                    _ => return Err(format!("unsupported unit weapon extra check `{label}`")),
                };
                self.same_bool_if_present(label, message, "value", native)
            }
            name => Err(format!("unsupported unit weapon extra check `{name}`")),
        }
    }

    pub(crate) fn check_unit_health(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_health(unit_id)
            .map_err(|err| format!("get_unit_health({unit_id}) failed: {err:?}"))?;

        self.same_if_present(label, message, "health", native.health)?;
        self.same_if_present(label, message, "maxHealth", native.maxHealth)?;
        self.same_if_present(label, message, "paralyzeDamage", native.paralyzeDamage)?;
        self.same_if_present(label, message, "captureProgress", native.captureProgress)?;
        self.same_if_present(label, message, "buildProgress", native.buildProgress)
    }
    pub(crate) fn check_unit_experience(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_experience(unit_id)
            .map_err(|err| format!("get_unit_experience({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "experience", native)
    }
    pub(crate) fn check_unit_neutral(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_neutral(unit_id)
            .map_err(|err| format!("get_unit_neutral({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "neutral", native)
    }
    pub(crate) fn check_unit_seismic_signature(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_seismic_signature(unit_id)
            .map_err(|err| format!("get_unit_seismic_signature({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "seismicSignature", native)
    }
    pub(crate) fn check_unit_mass(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_mass(unit_id)
            .map_err(|err| format!("get_unit_mass({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "mass", native)
    }
    pub(crate) fn check_unit_armored(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_armored(unit_id)
            .map_err(|err| format!("get_unit_armored({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "armored", native.armored)?;
        self.same_if_present(label, message, "armorMultiple", native.armorMultiple)
    }
    pub(crate) fn check_unit_costs(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_costs(unit_id)
            .map_err(|err| format!("get_unit_costs({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "buildTime", native.buildTime)?;
        self.same_if_present(label, message, "metalCost", native.metalCost)?;
        self.same_if_present(label, message, "energyCost", native.energyCost)
    }
    pub(crate) fn check_unit_storage(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let _ = i32_field(message, "unitID")?;
        match str_field(message, "resource")? {
            "metal" | "energy" => {
                let _ = f32_field(message, "amount")?;
                Ok(())
            }
            resource => Err(format!(
                "{label}.resource: unsupported resource `{resource}`"
            )),
        }
    }
    pub(crate) fn check_unit_harvest_storage(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_harvest_storage(unit_id)
            .map_err(|err| format!("get_unit_harvest_storage({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "storedMetal", native.storedMetal)?;
        self.same_if_present(label, message, "maxStoredMetal", native.maxStoredMetal)?;
        self.same_if_present(label, message, "storedEnergy", native.storedEnergy)?;
        self.same_if_present(label, message, "maxStoredEnergy", native.maxStoredEnergy)
    }
    pub(crate) fn check_unit_los_state(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_los_state(unit_id, ally_team_id, false)
            .map_err(|err| {
                format!("get_unit_los_state({unit_id}, {ally_team_id}, false) failed: {err:?}")
            })?;
        self.same_i32_if_present(label, message, "rawMask", i32::from(native.rawMask))?;
        self.same_bool_if_present(label, message, "los", native.los)?;
        self.same_bool_if_present(label, message, "radar", native.radar)?;
        self.same_bool_if_present(label, message, "typed", native.typed)
    }
    pub(crate) fn set_unit_los_state(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let raw_mask = u8::try_from(i32_field(message, "rawMask")? & 0x0F)
            .map_err(|err| format!("rawMask conversion failed: {err}"))?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_los_mask(unit_id, ally_team_id, 15)
            .map_err(|err| {
                format!("set_unit_los_mask({unit_id}, {ally_team_id}, 15) failed: {err:?}")
            })?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_los_state(unit_id, ally_team_id, raw_mask)
            .map_err(|err| {
                format!("set_unit_los_state({unit_id}, {ally_team_id}, {raw_mask}) failed: {err:?}")
            })?;
        Ok(())
    }
    pub(crate) fn check_unit_build_param(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let param_name = str_field(message, "paramName")?;
        let native = self
            .interface
            .units_info()
            .get_unit_build_params(unit_id, param_name)
            .map_err(|err| {
                format!("get_unit_build_params({unit_id}, {param_name}) failed: {err:?}")
            })?;
        let (value, has_value) = native;
        self.same_bool_if_present(label, message, "hasValue", has_value)?;
        if !has_value {
            return Ok(());
        }
        if value.useBoolean {
            self.same_bool_if_present(label, message, "enabled", value.boolean)
        } else {
            self.same_if_present(label, message, "value", value.number)
        }
    }
    pub(crate) fn set_unit_build_param(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let param_name = str_field(message, "paramName")?;
        let value = if param_name == "buildRange3D" {
            spring_native::sys::NumberOrBool {
                number: 0.0,
                boolean: bool_field(message, "enabled")?,
                useBoolean: true,
            }
        } else {
            spring_native::sys::NumberOrBool {
                number: f32_field(message, "value")?,
                boolean: false,
                useBoolean: false,
            }
        };
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_build_params(unit_id, param_name, value)
            .map_err(|err| {
                format!("set_unit_build_params({unit_id}, {param_name}) failed: {err:?}")
            })?;
        Ok(())
    }
    pub(crate) fn check_unit_worker_task(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_worker_task(unit_id)
            .map_err(|err| format!("get_unit_worker_task({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "hasTask", native.hasTask)?;
        self.same_i32_if_present(label, message, "cmdID", native.cmdID)?;
        self.same_bool_if_present(label, message, "hasTarget", native.hasTarget)?;
        self.same_i32_if_present(label, message, "targetID", native.targetID)
    }
    pub(crate) fn check_unit_blocking(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_blocking(unit_id)
            .map_err(|err| format!("get_unit_blocking({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "isBlocking", native.isBlocking)?;
        self.same_bool_if_present(
            label,
            message,
            "isSolidObjectCollidable",
            native.isSolidObjectCollidable,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "isProjectileCollidable",
            native.isProjectileCollidable,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "isRaySegmentCollidable",
            native.isRaySegmentCollidable,
        )?;
        self.same_bool_if_present(label, message, "crushable", native.crushable)?;
        self.same_bool_if_present(
            label,
            message,
            "blockEnemyPushing",
            native.blockEnemyPushing,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "blockHeightChanges",
            native.blockHeightChanges,
        )
    }
    pub(crate) fn check_unit_render_flag(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let unsynced_read = self.interface.unsynced_read();
        let unit_rendering = unsynced_read.unit_rendering();
        match base_test_name(label) {
            "get_unit_no_draw" | "unit_no_draw" => {
                let native = unit_rendering
                    .get_unit_no_draw(unit_id)
                    .map_err(|err| format!("get_unit_no_draw({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "noDraw", native)
            }
            "get_unit_lua_draw" => {
                let native = unit_rendering
                    .get_unit_lua_draw(unit_id)
                    .map_err(|err| format!("get_unit_lua_draw({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "luaDraw", native)
            }
            "get_unit_no_select" | "unit_no_select" => {
                let native = unit_rendering
                    .get_unit_no_select(unit_id)
                    .map_err(|err| format!("get_unit_no_select({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "noSelect", native)
            }
            "get_unit_no_minimap" | "unit_no_minimap" => {
                let native = unit_rendering
                    .get_unit_no_minimap(unit_id)
                    .map_err(|err| format!("get_unit_no_minimap({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "noMinimap", native)
            }
            "get_unit_no_group" | "unit_no_group" => {
                let native = unit_rendering
                    .get_unit_no_group(unit_id)
                    .map_err(|err| format!("get_unit_no_group({unit_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "noGroup", native)
            }
            "get_unit_engine_draw_mask" | "unit_engine_draw_mask" => {
                let native = unit_rendering
                    .get_unit_engine_draw_mask(unit_id)
                    .map_err(|err| {
                        format!("get_unit_engine_draw_mask({unit_id}) failed: {err:?}")
                    })?;
                self.same_i32_if_present(label, message, "mask", native as i32)
            }
            "get_unit_always_update_matrix" | "unit_always_update_matrix" => {
                let native = unit_rendering
                    .get_unit_always_update_matrix(unit_id)
                    .map_err(|err| {
                        format!("get_unit_always_update_matrix({unit_id}) failed: {err:?}")
                    })?;
                self.same_bool_if_present(label, message, "update", native)
            }
            "get_unit_draw_flag" => {
                let native = unit_rendering
                    .get_unit_draw_flag(unit_id)
                    .map_err(|err| format!("get_unit_draw_flag({unit_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "flag", native as i32)
            }
            _ => Err(format!("unsupported unit render flag check `{label}`")),
        }
    }
    pub(crate) fn set_unit_render_flag(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let unsynced_ctrl = self.interface.unsynced_ctrl();
        let success = match base_test_name(test_name_field(message)?) {
            "unit_no_draw" => unsynced_ctrl
                .set_unit_no_draw(unit_id, bool_field(message, "noDraw")?)
                .map_err(|err| format!("set_unit_no_draw({unit_id}) failed: {err:?}"))?,
            "unit_no_select" => unsynced_ctrl
                .set_unit_no_select(unit_id, bool_field(message, "noSelect")?)
                .map_err(|err| format!("set_unit_no_select({unit_id}) failed: {err:?}"))?,
            "unit_no_minimap" => unsynced_ctrl
                .set_unit_no_minimap(unit_id, bool_field(message, "noMinimap")?)
                .map_err(|err| format!("set_unit_no_minimap({unit_id}) failed: {err:?}"))?,
            "unit_no_group" => unsynced_ctrl
                .set_unit_no_group(unit_id, bool_field(message, "noGroup")?)
                .map_err(|err| format!("set_unit_no_group({unit_id}) failed: {err:?}"))?,
            "unit_engine_draw_mask" => unsynced_ctrl
                .set_unit_engine_draw_mask(unit_id, i32_field(message, "mask")? as u32)
                .map_err(|err| format!("set_unit_engine_draw_mask({unit_id}) failed: {err:?}"))?,
            "unit_always_update_matrix" => unsynced_ctrl
                .set_unit_always_update_matrix(unit_id, bool_field(message, "update")?)
                .map_err(|err| {
                    format!("set_unit_always_update_matrix({unit_id}) failed: {err:?}")
                })?,
            name => return Err(format!("unsupported unit render setter `{name}`")),
        };
        if success {
            Ok(())
        } else {
            Err("unit render setter returned false".to_string())
        }
    }
    pub(crate) fn set_unit_blocking(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_blocking(
                unit_id,
                spring_native::SetUnitBlockingOptions {
                    blocking: bool_field(message, "isBlocking")?,
                    solid_objects: bool_field(message, "isSolidObjectCollidable")?,
                    projectiles: bool_field(message, "isProjectileCollidable")?,
                    quad_map_rays: bool_field(message, "isRaySegmentCollidable")?,
                    crushable: bool_field(message, "crushable")?,
                    block_enemy_pushing: bool_field(message, "blockEnemyPushing")?,
                    block_height_changes: bool_field(message, "blockHeightChanges")?,
                },
            )
            .map_err(|err| format!("set_unit_blocking({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn check_unit_max_range(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_weapons()
            .get_unit_max_range(unit_id)
            .map_err(|err| format!("get_unit_max_range({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "maxRange", native)
    }
    pub(crate) fn check_unit_position(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_position(unit_id, spring_native::GetUnitPositionOptions::default())
            .map_err(|err| format!("get_unit_position({unit_id}) failed: {err:?}"))?;
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_unit_def_id(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_def_id(unit_id)
            .map_err(|err| format!("get_unit_def_id({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "defID", native)
    }
    pub(crate) fn check_unit_team(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_team(unit_id)
            .map_err(|err| format!("get_unit_team({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "teamID", native)
    }
    pub(crate) fn check_unit_ally_team(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_ally_team(unit_id)
            .map_err(|err| format!("get_unit_ally_team({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "allyTeamID", native)
    }
    pub(crate) fn check_unit_is_dead(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_is_dead(unit_id)
            .map_err(|err| format!("get_unit_is_dead({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "isDead", native)
    }
    pub(crate) fn check_unit_is_stunned(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_is_stunned(unit_id)
            .map_err(|err| format!("get_unit_is_stunned({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "stunned", native)
    }
    pub(crate) fn check_unit_height(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_height(unit_id)
            .map_err(|err| format!("get_unit_height({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "height", native)
    }
    pub(crate) fn check_unit_radius(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_radius(unit_id)
            .map_err(|err| format!("get_unit_radius({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "radius", native)
    }
    pub(crate) fn check_unit_base_position(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_base_position(unit_id)
            .map_err(|err| format!("get_unit_base_position({unit_id}) failed: {err:?}"))?;
        self.same(&format!("{label}.x"), native.x, f32_field(message, "x")?)?;
        self.same(&format!("{label}.z"), native.z, f32_field(message, "z")?)?;
        Ok(())
    }
    pub(crate) fn check_unit_direction(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_direction(unit_id)
            .map_err(|err| format!("get_unit_direction({unit_id}) failed: {err:?}"))?;
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_unit_vectors(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_vectors(unit_id)
            .map_err(|err| format!("get_unit_vectors({unit_id}) failed: {err:?}"))?;
        self.same(
            &format!("{label}.frontX"),
            native.frontDir.x,
            f32_field(message, "frontX")?,
        )?;
        self.same(
            &format!("{label}.frontY"),
            native.frontDir.y,
            f32_field(message, "frontY")?,
        )?;
        self.same(
            &format!("{label}.frontZ"),
            native.frontDir.z,
            f32_field(message, "frontZ")?,
        )?;
        self.same(
            &format!("{label}.upX"),
            native.upDir.x,
            f32_field(message, "upX")?,
        )?;
        self.same(
            &format!("{label}.upY"),
            native.upDir.y,
            f32_field(message, "upY")?,
        )?;
        self.same(
            &format!("{label}.upZ"),
            native.upDir.z,
            f32_field(message, "upZ")?,
        )?;
        self.same(
            &format!("{label}.rightX"),
            native.rightDir.x,
            f32_field(message, "rightX")?,
        )?;
        self.same(
            &format!("{label}.rightY"),
            native.rightDir.y,
            f32_field(message, "rightY")?,
        )?;
        self.same(
            &format!("{label}.rightZ"),
            native.rightDir.z,
            f32_field(message, "rightZ")?,
        )
    }
    pub(crate) fn check_unit_rotation(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_rotation(unit_id)
            .map_err(|err| format!("get_unit_rotation({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "pitch", native.pitch)?;
        self.same_if_present(label, message, "yaw", native.yaw)?;
        self.same_if_present(label, message, "roll", native.roll)
    }
    pub(crate) fn check_unit_heading(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_heading(unit_id, false)
            .map_err(|err| format!("get_unit_heading({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "heading", native)
    }
    pub(crate) fn check_unit_velocity(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_velocity(unit_id)
            .map_err(|err| format!("get_unit_velocity({unit_id}) failed: {err:?}"))?;
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_unit_info_bool(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let (field, native) = match test_name {
            "get_unit_is_active" => (
                "isActive",
                self.interface
                    .units_info()
                    .get_unit_is_active(unit_id)
                    .map_err(|err| format!("get_unit_is_active({unit_id}) failed: {err:?}"))?,
            ),
            "get_unit_is_cloaked" => (
                "isCloaked",
                self.interface
                    .units_info()
                    .get_unit_is_cloaked(unit_id)
                    .map_err(|err| format!("get_unit_is_cloaked({unit_id}) failed: {err:?}"))?,
            ),
            "get_unit_is_being_built" => (
                "isBeingBuilt",
                self.interface
                    .units_info()
                    .get_unit_is_being_built(unit_id)
                    .map_err(|err| format!("get_unit_is_being_built({unit_id}) failed: {err:?}"))?,
            ),
            "get_unit_in_build_stance" => (
                "inBuildStance",
                self.interface
                    .units_info()
                    .get_unit_in_build_stance(unit_id)
                    .map_err(|err| {
                        format!("get_unit_in_build_stance({unit_id}) failed: {err:?}")
                    })?,
            ),
            "get_unit_last_attacker" => {
                let (_attacker, has_attacker) = self
                    .interface
                    .units_info()
                    .get_unit_last_attacker(unit_id)
                    .map_err(|err| format!("get_unit_last_attacker({unit_id}) failed: {err:?}"))?;
                ("hasAttacker", has_attacker)
            }
            "get_unit_shield_state" => {
                let weapon_num = i32_field(message, "weaponNum")?;
                let (_shield, has_shield) = self
                    .interface
                    .units_info()
                    .get_unit_shield_state(unit_id, weapon_num)
                    .map_err(|err| {
                        format!("get_unit_shield_state({unit_id}, {weapon_num}) failed: {err:?}")
                    })?;
                ("hasShield", has_shield)
            }
            _ => return Err(format!("unsupported unit info bool check `{label}`")),
        };
        self.same_bool_if_present(label, message, field, native)
    }
    pub(crate) fn check_unit_info_i32(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let (field, native) = match test_name {
            "get_unit_build_facing" => (
                "facing",
                self.interface
                    .units_info()
                    .get_unit_build_facing(unit_id)
                    .map_err(|err| format!("get_unit_build_facing({unit_id}) failed: {err:?}"))?,
            ),
            "get_unit_move_def_id" => (
                "moveDefID",
                self.interface
                    .units_info()
                    .get_unit_move_def_id(unit_id)
                    .map_err(|err| format!("get_unit_move_def_id({unit_id}) failed: {err:?}"))?,
            ),
            "get_unit_is_building" => (
                "buildeeID",
                self.interface
                    .units_info()
                    .get_unit_is_building(unit_id)
                    .map_err(|err| format!("get_unit_is_building({unit_id}) failed: {err:?}"))?,
            ),
            "get_unit_transporter" => (
                "transporterID",
                self.interface
                    .units_info()
                    .get_unit_transporter(unit_id)
                    .map_err(|err| format!("get_unit_transporter({unit_id}) failed: {err:?}"))?,
            ),
            _ => return Err(format!("unsupported unit info i32 check `{label}`")),
        };
        self.same_i32_if_present(label, message, field, native)
    }
    pub(crate) fn check_unit_info_list_count(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let count = match test_name {
            "get_unit_nano_pieces_count" => self
                .interface
                .units_info()
                .get_unit_nano_pieces(unit_id)
                .map_err(|err| format!("get_unit_nano_pieces({unit_id}) failed: {err:?}"))?
                .len(),
            _ => return Err(format!("unsupported unit info list-count check `{label}`")),
        };
        self.same_i32_if_present(label, message, "count", count as i32)
    }
    pub(crate) fn check_unit_info_f32(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let (field, native) = match test_name {
            "get_unit_buildee_radius" | "unit_buildee_radius" => (
                "buildeeRadius",
                self.interface
                    .units_info()
                    .get_unit_buildee_radius(unit_id)
                    .map_err(|err| format!("get_unit_buildee_radius({unit_id}) failed: {err:?}"))?,
            ),
            "get_unit_current_build_power" => (
                "buildPower",
                self.interface
                    .units_info()
                    .get_unit_current_build_power(unit_id)
                    .map_err(|err| {
                        format!("get_unit_current_build_power({unit_id}) failed: {err:?}")
                    })?,
            ),
            _ => return Err(format!("unsupported unit info f32 check `{label}`")),
        };
        self.same_if_present(label, message, field, native)
    }
    pub(crate) fn check_unit_effective_build_range(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let buildee_def_id = i32_field(message, "buildeeDefID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_effective_build_range(unit_id, buildee_def_id)
            .map_err(|err| {
                format!(
                    "get_unit_effective_build_range({unit_id}, {buildee_def_id}) failed: {err:?}"
                )
            })?;
        self.same_if_present(label, message, "range", native)
    }
    pub(crate) fn check_unit_view_position(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let mid_pos = bool_field(message, "midPos")?;
        let native = self
            .interface
            .unsynced_read()
            .unit_rendering()
            .get_unit_view_position(unit_id, mid_pos)
            .map_err(|err| {
                format!("get_unit_view_position({unit_id}, {mid_pos}) failed: {err:?}")
            })?;
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_unit_icon_data(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let full_data = bool_field(message, "fullData")?;
        let native = self
            .interface
            .unsynced_read()
            .unit_rendering()
            .get_unit_icon_data(unit_id, full_data)
            .map_err(|err| format!("get_unit_icon_data({unit_id}, {full_data}) failed: {err:?}"))?;
        self.same_string_if_present(
            label,
            message,
            "iconName",
            native.0.as_deref().unwrap_or(""),
        )
    }
    pub(crate) fn set_unit_buildee_radius(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_buildee_radius(unit_id, f32_field(message, "buildeeRadius")?)
            .map_err(|err| format!("set_unit_buildee_radius({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn check_unit_resources(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_resources(unit_id)
            .map_err(|err| format!("get_unit_resources({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "metalMake", native.metalMake)?;
        self.same_if_present(label, message, "metalUse", native.metalUse)?;
        self.same_if_present(label, message, "energyMake", native.energyMake)?;
        self.same_if_present(label, message, "energyUse", native.energyUse)
    }
    pub(crate) fn check_unit_cost_table(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_cost_table(unit_id)
            .map_err(|err| format!("get_unit_cost_table({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "metalCost", native.metalCost)?;
        self.same_if_present(label, message, "energyCost", native.energyCost)?;
        self.same_if_present(label, message, "buildTime", native.buildTime)
    }
    pub(crate) fn check_unit_metal_extraction(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_metal_extraction(unit_id)
            .map_err(|err| format!("get_unit_metal_extraction({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "metalExtraction", native)
    }
    pub(crate) fn check_unit_states(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_states(unit_id, spring_native::UnitStatesOptions::default())
            .map_err(|err| format!("get_unit_states({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "firestate", native.fireState)?;
        self.same_i32_if_present(label, message, "movestate", native.moveState)?;
        self.same_bool_if_present(label, message, "repeat", native.repeat)?;
        self.same_bool_if_present(label, message, "cloak", native.cloak)?;
        self.same_bool_if_present(label, message, "active", native.active)?;
        self.same_bool_if_present(label, message, "trajectory", native.trajectory)
    }
    pub(crate) fn check_unit_sensor_radius(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let sensor_type = str_field(message, "sensorType")?;
        let native = self
            .interface
            .units_info()
            .get_unit_sensor_radius(unit_id, sensor_type)
            .map_err(|err| {
                format!("get_unit_sensor_radius({unit_id}, {sensor_type}) failed: {err:?}")
            })?;
        let radius = match sensor_type {
            "los" => native.los,
            "airLos" => native.airLos,
            "radar" => native.radar,
            "sonar" => native.sonar,
            "seismic" => native.seismic,
            "radarJammer" => native.radarJammer,
            "sonarJammer" => native.sonarJammer,
            _ => {
                return Err(format!(
                    "{label}.sensorType: unsupported sensor type `{sensor_type}`"
                ))
            }
        };
        self.same_if_present(label, message, "radius", radius)
    }
    pub(crate) fn check_unit_leaves_ghost(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .synced_ctrl()
            .unit()
            .get_unit_leaves_ghost(unit_id)
            .map_err(|err| format!("get_unit_leaves_ghost({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "leavesGhost", native)
    }
    pub(crate) fn check_unit_self_dtime(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_self_dtime(unit_id)
            .map_err(|err| format!("get_unit_self_dtime({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "selfDTime", native)
    }
    pub(crate) fn check_unit_collision_volume_data(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_collision_volume_data(unit_id)
            .map_err(|err| format!("get_unit_collision_volume_data({unit_id}) failed: {err:?}"))?;
        self.same_collision_volume(label, message, native)
    }
    pub(crate) fn check_unit_travel(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_travel(unit_id)
            .map_err(|err| format!("get_unit_travel({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "travelPeriod", native.travelPeriod)?;
        self.same_if_present(label, message, "travelTime", native.travelTime)
    }
    pub(crate) fn check_unit_fuel(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_fuel(unit_id)
            .map_err(|err| format!("get_unit_fuel({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "fuel", native.fuel)?;
        self.same_if_present(label, message, "maxFuel", native.maxFuel)
    }
    pub(crate) fn check_unit_tooltip(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_tooltip(unit_id)
            .map_err(|err| format!("get_unit_tooltip({unit_id}) failed: {err:?}"))?;
        self.same_string_if_present(label, message, "tooltip", native.as_deref().unwrap_or(""))
    }
    pub(crate) fn check_unit_physical_state(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .synced_ctrl()
            .unit()
            .get_unit_physical_state(unit_id)
            .map_err(|err| format!("get_unit_physical_state({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "physicalState", native as i32)
    }
    pub(crate) fn check_unit_weapon_state_f32(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let weapon_num = i32_field(message, "weaponNum")?;
        let key = str_field(message, "key")?;
        let native = self
            .interface
            .units_weapons()
            .get_unit_weapon_state(unit_id, weapon_num, key)
            .map_err(|err| {
                format!("get_unit_weapon_state({unit_id}, {weapon_num}, {key}) failed: {err:?}")
            })?;
        let value = match key {
            "range" => native.range,
            "projectileSpeed" => native.projectileSpeed,
            "reloadFrame" => native.reloadFrame,
            "reloadTime" => native.reloadTime,
            _ => return Err(format!("{label}.key: unsupported weapon state key `{key}`")),
        };
        self.same_if_present(label, message, "value", value)
    }
    pub(crate) fn check_unit_weapon_bool(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let weapon_num = i32_field(message, "weaponNum")?;
        let native = match test_name {
            "get_unit_weapon_can_fire" => self
                .interface
                .units_weapons()
                .get_unit_weapon_can_fire(unit_id, weapon_num)
                .map_err(|err| {
                    format!("get_unit_weapon_can_fire({unit_id}, {weapon_num}) failed: {err:?}")
                })?,
            "get_unit_weapon_test_range" => {
                let target_pos = vec3_from_fields(message, "x", "y", "z")?;
                self.interface
                    .units_weapons()
                    .get_unit_weapon_test_range(unit_id, weapon_num, target_pos)
                    .map_err(|err| {
                        format!(
                            "get_unit_weapon_test_range({unit_id}, {weapon_num}) failed: {err:?}"
                        )
                    })?
            }
            _ => return Err(format!("unsupported unit weapon bool check `{label}`")),
        };
        self.same_bool_if_present(label, message, "value", native)
    }
    pub(crate) fn check_unit_weapon_damages_f32(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let weapon_num = i32_field(message, "weaponNum")?;
        let key = str_field(message, "key")?;
        let native = self
            .interface
            .units_weapons()
            .get_unit_weapon_damages(unit_id, weapon_num)
            .map_err(|err| {
                format!("get_unit_weapon_damages({unit_id}, {weapon_num}) failed: {err:?}")
            })?;
        let value = match key {
            "paralyzeDamageTime" => native.paralyzeDamageTime,
            "impulseFactor" => native.impulseFactor,
            "impulseBoost" => native.impulseBoost,
            "craterMult" => native.craterMult,
            "craterBoost" => native.craterBoost,
            _ => {
                return Err(format!(
                    "{label}.key: unsupported weapon damages key `{key}`"
                ))
            }
        };
        self.same_if_present(label, message, "value", value)
    }
    pub(crate) fn set_unit_collision_volume_data(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let scales = vec3_from_fields(message, "scaleX", "scaleY", "scaleZ")?;
        let offsets = vec3_from_fields(message, "offsetX", "offsetY", "offsetZ")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_collision_volume_data(
                unit_id,
                scales,
                offsets,
                i32_field(message, "volumeType")?,
                i32_field(message, "testType")?,
                i32_field(message, "primaryAxis")?,
            )
            .map_err(|err| format!("set_unit_collision_volume_data({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_physical_state_bit(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let state_bit = i32_field(message, "stateBit")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_physical_state_bit(unit_id, state_bit)
            .map_err(|err| {
                format!("set_unit_physical_state_bit({unit_id}, {state_bit}) failed: {err:?}")
            })?;
        Ok(())
    }
    pub(crate) fn set_unit_radius_and_height(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_radius_and_height(
                unit_id,
                f32_field(message, "radius")?,
                f32_field(message, "height")?,
            )
            .map_err(|err| format!("set_unit_radius_and_height({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_sensor_radius(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let sensor_type = str_field(message, "sensorType")?;
        let radius = i32_field(message, "radius")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_sensor_radius(unit_id, sensor_type, radius)
            .map_err(|err| {
                format!("set_unit_sensor_radius({unit_id}, {sensor_type}) failed: {err:?}")
            })?;
        Ok(())
    }
    pub(crate) fn set_unit_cloak(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let cloak = spring_native::sys::NumberOrBool {
            number: 0.0,
            boolean: bool_field(message, "cloak")?,
            useBoolean: true,
        };
        let cloak_arg = spring_native::sys::NumberOrBool {
            number: 0.0,
            boolean: false,
            useBoolean: true,
        };
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_cloak(unit_id, cloak, cloak_arg)
            .map_err(|err| format!("set_unit_cloak({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_direction(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let front = vec3_from_fields(message, "frontX", "frontY", "frontZ")?;
        let right = vec3_from_fields(message, "rightX", "rightY", "rightZ")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_direction(unit_id, front, right)
            .map_err(|err| format!("set_unit_direction({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_rotation(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let rotation = vec3_from_fields(message, "pitch", "yaw", "roll")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_rotation(unit_id, rotation)
            .map_err(|err| format!("set_unit_rotation({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_health(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let value = spring_native::sys::UnitHealthValue {
            health: f32_field(message, "health")?,
            capture: f32_field(message, "captureProgress")?,
            paralyze: f32_field(message, "paralyzeDamage")?,
            build: f32_field(message, "buildProgress")?,
            useAmounts: true,
        };
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_health(unit_id, value)
            .map_err(|err| format!("set_unit_health({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_noop(&mut self, _message: &Value) -> Result<(), String> {
        Ok(())
    }
    pub(crate) fn set_unit_max_health(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_max_health(unit_id, f32_field(message, "maxHealth")?)
            .map_err(|err| format!("set_unit_max_health({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_experience(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_experience(unit_id, f32_field(message, "experience")?)
            .map_err(|err| format!("set_unit_experience({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn add_unit_experience(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let unit = synced_ctrl.unit();
        unit.set_unit_experience(unit_id, f32_field(message, "baseline")?)
            .map_err(|err| format!("set_unit_experience({unit_id}) failed: {err:?}"))?;
        unit.add_unit_experience(unit_id, f32_field(message, "deltaExperience")?)
            .map_err(|err| format!("add_unit_experience({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn add_unit_damage(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let unit = synced_ctrl.unit();
        let baseline = f32_field(message, "baseline")?;
        let health = spring_native::sys::UnitHealthValue {
            health: baseline,
            capture: 0.0,
            paralyze: 0.0,
            build: 1.0,
            useAmounts: true,
        };
        unit.set_unit_health(unit_id, health)
            .map_err(|err| format!("set_unit_health({unit_id}) failed: {err:?}"))?;
        unit.add_unit_damage(
            unit_id,
            f32_field(message, "damage")?,
            0.0,
            -1,
            -1,
            spring_native::sys::Float3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        )
        .map_err(|err| format!("add_unit_damage({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_neutral(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_neutral(unit_id, bool_field(message, "neutral")?)
            .map_err(|err| format!("set_unit_neutral({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_seismic_signature(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_seismic_signature(unit_id, f32_field(message, "seismicSignature")?)
            .map_err(|err| format!("set_unit_seismic_signature({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_mass(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_mass(unit_id, f32_field(message, "mass")?)
            .map_err(|err| format!("set_unit_mass({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_armored(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_armored(
                unit_id,
                bool_field(message, "armored")?,
                f32_field(message, "armorMultiple")?,
            )
            .map_err(|err| format!("set_unit_armored({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_costs(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let costs = spring_native::sys::UnitCostOverrides {
            buildTime: f32_field(message, "buildTime")?,
            metalCost: f32_field(message, "metalCost")?,
            energyCost: f32_field(message, "energyCost")?,
        };
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_costs(unit_id, costs)
            .map_err(|err| format!("set_unit_costs({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_storage(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let resource = str_field(message, "resource")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_storage(unit_id, resource, f32_field(message, "amount")?)
            .map_err(|err| format!("set_unit_storage({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_harvest_storage(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let stored_metal = f32_field(message, "storedMetal")?;
        let max_stored_metal = f32_field(message, "maxStoredMetal")?;
        let stored_energy = f32_field(message, "storedEnergy")?;
        let max_stored_energy = f32_field(message, "maxStoredEnergy")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_harvest_storage(
                unit_id,
                stored_metal,
                max_stored_metal,
                stored_energy,
                max_stored_energy,
            )
            .map_err(|err| format!("set_unit_harvest_storage({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_max_range(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_max_range(unit_id, f32_field(message, "maxRange")?)
            .map_err(|err| format!("set_unit_max_range({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_position(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_position(unit_id, vec3_from_fields(message, "x", "y", "z")?)
            .map_err(|err| format!("set_unit_position({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_velocity(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_velocity(unit_id, vec3_from_fields(message, "x", "y", "z")?)
            .map_err(|err| format!("set_unit_velocity({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_unit_metal_extraction(&mut self, message: &Value) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        self.interface
            .synced_ctrl()
            .unit()
            .set_unit_metal_extraction(
                unit_id,
                f32_field(message, "depth")?,
                f32_field(message, "range")?,
            )
            .map_err(|err| format!("set_unit_metal_extraction({unit_id}) failed: {err:?}"))?;
        Ok(())
    }
}

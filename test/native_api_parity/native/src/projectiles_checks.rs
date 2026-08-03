use super::*;
use crate::support::*;

use std::ffi::CString;

impl NativeApiParity {
    pub(crate) fn check_projectiles_list(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let exclude_weapon_projectiles = bool_field(message, "synced")?;
        let exclude_piece_projectiles = bool_field(message, "weapon")?;
        let native = match test_name {
            "get_all_projectiles" => self
                .interface
                .projectiles()
                .get_all_projectiles(spring_native::GetAllProjectilesOptions {
                    exclude_weapon_projectiles,
                    exclude_piece_projectiles,
                })
                .map_err(|err| {
                    format!("get_all_projectiles({exclude_weapon_projectiles}, {exclude_piece_projectiles}) failed: {err:?}")
                })?,
            "get_projectiles_in_rectangle" => self
                .interface
                .projectiles()
                .get_projectiles_in_rectangle(
                    f32_field(message, "minX")?,
                    f32_field(message, "minZ")?,
                    f32_field(message, "maxX")?,
                    f32_field(message, "maxZ")?,
                    spring_native::GetProjectilesInRectangleOptions {
                        exclude_weapon_projectiles,
                        exclude_piece_projectiles,
                    },
                )
                .map_err(|err| format!("get_projectiles_in_rectangle() failed: {err:?}"))?,
            "get_projectiles_in_sphere" => self
                .interface
                .projectiles()
                .get_projectiles_in_sphere(
                    vec3_from_fields(message, "x", "y", "z")?,
                    f32_field(message, "radius")?,
                    spring_native::GetProjectilesInSphereOptions {
                        exclude_weapon_projectiles,
                        exclude_piece_projectiles,
                    },
                )
                .map_err(|err| format!("get_projectiles_in_sphere() failed: {err:?}"))?,
            _ => return Err(format!("unsupported projectiles list check `{label}`")),
        };
        self.same_i32_set_if_present(label, message, "projectileIDs", &native)
    }

    pub(crate) fn check_projectile_read(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let projectile_id = i32_field(message, "projectileID")?;
        let name = base_test_name(label);
        match name {
            "projectile_position" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_position(projectile_id)
                    .map_err(|err| format!("get_projectile_position() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "projectile_direction" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_direction(projectile_id)
                    .map_err(|err| format!("get_projectile_direction() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "projectile_velocity" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_velocity(projectile_id)
                    .map_err(|err| format!("get_projectile_velocity() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "projectile_gravity" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_gravity(projectile_id)
                    .map_err(|err| format!("get_projectile_gravity() failed: {err:?}"))?;
                self.same_if_present(label, message, "gravity", native.y)
            }
            "projectile_target" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_target(projectile_id)
                    .map_err(|err| format!("get_projectile_target() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "targetType", native.targetType)?;
                self.same_i32_if_present(label, message, "targetID", native.targetID)?;
                self.same_if_present(label, message, "targetX", native.targetPos.x)?;
                self.same_if_present(label, message, "targetY", native.targetPos.y)?;
                self.same_if_present(label, message, "targetZ", native.targetPos.z)
            }
            "projectile_intercepted" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_is_intercepted(projectile_id)
                    .map_err(|err| format!("get_projectile_is_intercepted() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "intercepted", native)
            }
            "projectile_ttl" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_time_to_live(projectile_id)
                    .map_err(|err| format!("get_projectile_time_to_live() failed: {err:?}"))?;
                self.same_if_present(label, message, "ttl", native)
            }
            "projectile_owner" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_owner_id(projectile_id)
                    .map_err(|err| format!("get_projectile_owner_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "ownerID", native)
            }
            "projectile_team" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_team_id(projectile_id)
                    .map_err(|err| format!("get_projectile_team_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "teamID", native)
            }
            "projectile_ally_team" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_ally_team_id(projectile_id)
                    .map_err(|err| format!("get_projectile_ally_team_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "allyTeamID", native)
            }
            "projectile_type" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_type(projectile_id)
                    .map_err(|err| format!("get_projectile_type() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "weapon", native.0)?;
                self.same_bool_if_present(label, message, "piece", native.1)
            }
            "projectile_def_id" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_def_id(projectile_id)
                    .map_err(|err| format!("get_projectile_def_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "defID", native)
            }
            name if name.starts_with("projectile_damage_") => {
                self.check_projectile_damage(message, label, projectile_id)
            }
            "piece_projectile_params" => self.check_piece_projectile_params(message, label),
            _ => Err(format!("unsupported projectile read check `{name}`")),
        }
    }

    fn check_projectile_damage(
        &mut self,
        message: &Value,
        label: &str,
        projectile_id: i32,
    ) -> Result<(), String> {
        let tag = str_field(message, "tag")?;
        let native = self
            .interface
            .projectiles()
            .get_projectile_damages(projectile_id, tag)
            .map_err(|err| format!("get_projectile_damages({tag:?}) failed: {err:?}"))?;
        match tag {
            "paralyzeDamageTime" => {
                self.same_if_present(label, message, "damageValue", native.paralyzeDamageTime)
            }
            "impulseFactor" => {
                self.same_if_present(label, message, "damageValue", native.impulseFactor)
            }
            "impulseBoost" => {
                self.same_if_present(label, message, "damageValue", native.impulseBoost)
            }
            "craterMult" => self.same_if_present(label, message, "damageValue", native.craterMult),
            "craterBoost" => {
                self.same_if_present(label, message, "damageValue", native.craterBoost)
            }
            "dynDamageExp" => {
                self.same_if_present(label, message, "damageValue", native.dynDamageExp)
            }
            "dynDamageMin" => {
                self.same_if_present(label, message, "damageValue", native.dynDamageMin)
            }
            "dynDamageRange" => {
                self.same_if_present(label, message, "damageValue", native.dynDamageRange)
            }
            "dynDamageInverted" => {
                self.same_bool_if_present(label, message, "damageValue", native.dynDamageInverted)
            }
            "craterAreaOfEffect" => {
                self.same_if_present(label, message, "damageValue", native.craterAreaOfEffect)
            }
            "damageAreaOfEffect" => {
                self.same_if_present(label, message, "damageValue", native.damageAreaOfEffect)
            }
            "edgeEffectiveness" => {
                self.same_if_present(label, message, "damageValue", native.edgeEffectiveness)
            }
            "explosionSpeed" => {
                self.same_if_present(label, message, "damageValue", native.explosionSpeed)
            }
            _ => {
                let armor_type = tag
                    .parse::<usize>()
                    .map_err(|err| format!("invalid projectile damage tag {tag:?}: {err}"))?;
                let values = unsafe {
                    if native.damages.is_null() {
                        &[][..]
                    } else {
                        std::slice::from_raw_parts(native.damages, native.damageCount as usize)
                    }
                };
                let value = values.get(armor_type).copied().ok_or_else(|| {
                    format!("native damage array has no armor type {armor_type}: {values:?}")
                })?;
                self.same_if_present(label, message, "damageValue", value)
            }
        }
    }

    fn check_piece_projectile_params(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let projectile_id = i32_field(message, "projectileID")?;
        let (native, is_piece) = self
            .interface
            .projectiles()
            .get_piece_projectile_params(projectile_id)
            .map_err(|err| format!("get_piece_projectile_params() failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "isPiece", is_piece)?;
        self.same_i32_if_present(label, message, "explFlags", native.explFlags)?;
        self.same_if_present(label, message, "spinAngle", native.spinAngle)?;
        self.same_if_present(label, message, "spinSpeed", native.spinSpeed)?;
        self.same_if_present(label, message, "spinX", native.spinVec.x)?;
        self.same_if_present(label, message, "spinY", native.spinVec.y)?;
        self.same_if_present(label, message, "spinZ", native.spinVec.z)
    }

    pub(crate) fn check_projectile_control_call(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let projectile_id = i32_field(message, "projectileID")?;
        let name = base_test_name(label);
        match name {
            "projectile_position_after_set" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_position(projectile_id)
                    .map_err(|err| format!("get_projectile_position() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "projectile_velocity_after_set" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_velocity(projectile_id)
                    .map_err(|err| format!("get_projectile_velocity() failed: {err:?}"))?;
                self.same_vec3(label, native, message)
            }
            "projectile_gravity_after_set" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_gravity(projectile_id)
                    .map_err(|err| format!("get_projectile_gravity() failed: {err:?}"))?;
                self.same_if_present(label, message, "gravity", native.y)
            }
            "projectile_target_after_set" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_target(projectile_id)
                    .map_err(|err| format!("get_projectile_target() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "targetType", native.targetType)?;
                self.same_if_present(label, message, "targetX", native.targetPos.x)?;
                self.same_if_present(label, message, "targetY", native.targetPos.y)?;
                self.same_if_present(label, message, "targetZ", native.targetPos.z)
            }
            "projectile_intercepted_after_set" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_is_intercepted(projectile_id)
                    .map_err(|err| format!("get_projectile_is_intercepted() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "intercepted", native)
            }
            "projectile_ttl_after_set" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_time_to_live(projectile_id)
                    .map_err(|err| format!("get_projectile_time_to_live() failed: {err:?}"))?;
                self.same_if_present(label, message, "ttl", native)
            }
            name if name == "projectile_damage_after_set" => {
                self.check_projectile_damage(message, label, projectile_id)
            }
            "piece_projectile_params_after_set" => {
                self.check_piece_projectile_params(message, label)
            }
            "projectile_set_always_visible" => {
                let result = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_always_visible(projectile_id, bool_field(message, "enabled")?)
                    .map_err(|err| format!("set_projectile_always_visible() failed: {err:?}"))?;
                if !result {
                    return Err("set_projectile_always_visible() returned false".to_owned());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "projectile_set_use_air_los" => {
                let result = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_use_air_los(projectile_id, bool_field(message, "enabled")?)
                    .map_err(|err| format!("set_projectile_use_air_los() failed: {err:?}"))?;
                if !result {
                    return Err("set_projectile_use_air_los() returned false".to_owned());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "projectile_set_move_control" => {
                let result = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_move_control(projectile_id, bool_field(message, "enable")?)
                    .map_err(|err| format!("set_projectile_move_control() failed: {err:?}"))?;
                if !result {
                    return Err("set_projectile_move_control() returned false".to_owned());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "projectile_set_ignore_tracking_error" => {
                let result = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_ignore_tracking_error(
                        projectile_id,
                        bool_field(message, "ignore")?,
                    )
                    .map_err(|err| {
                        format!("set_projectile_ignore_tracking_error() failed: {err:?}")
                    })?;
                if !result {
                    return Err("set_projectile_ignore_tracking_error() returned false".to_owned());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "projectile_set_spin_angle" => {
                let result = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_spin_angle(projectile_id, f32_field(message, "angle")?)
                    .map_err(|err| format!("set_projectile_spin_angle() failed: {err:?}"))?;
                if !result {
                    return Err("set_projectile_spin_angle() returned false".to_owned());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "projectile_set_spin_speed" => {
                let result = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_spin_speed(projectile_id, f32_field(message, "speed")?)
                    .map_err(|err| format!("set_projectile_spin_speed() failed: {err:?}"))?;
                if !result {
                    return Err("set_projectile_spin_speed() returned false".to_owned());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "projectile_set_spin_vec" => {
                let result = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_spin_vec(
                        projectile_id,
                        vec3_from_fields(message, "spinX", "spinY", "spinZ")?,
                    )
                    .map_err(|err| format!("set_projectile_spin_vec() failed: {err:?}"))?;
                if !result {
                    return Err("set_projectile_spin_vec() returned false".to_owned());
                }
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "projectile_set_ceg" => {
                let native = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .set_projectile_ceg(projectile_id, str_field(message, "cegName")?)
                    .map_err(|err| format!("set_projectile_ceg() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "cegID", native)
            }
            "projectile_spawn" => {
                let model_name = CString::new(str_field(message, "modelName")?)
                    .map_err(|err| format!("invalid projectile model name: {err}"))?;
                let ceg_tag = CString::new(str_field(message, "cegTag")?)
                    .map_err(|err| format!("invalid projectile CEG tag: {err}"))?;
                let native = self
                    .interface
                    .synced_ctrl()
                    .projectile()
                    .spawn_projectile(
                        i32_field(message, "weaponDefID")?,
                        native_projectile_params(message, model_name.as_ptr(), ceg_tag.as_ptr())?,
                    )
                    .map_err(|err| format!("spawn_projectile() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "spawned", native >= 0)?;
                if native >= 0 {
                    self.interface
                        .synced_ctrl()
                        .projectile()
                        .delete_projectile(native)
                        .map_err(|err| format!("cleanup spawned projectile failed: {err:?}"))?;
                }
                Ok(())
            }
            // These tests use the native setter phase to perform the
            // destructive operation before the Lua getter is evaluated.
            // There is deliberately no second native call here: repeating a
            // delete/collision after the first call would test invalid-ID
            // handling rather than the Lua contract.
            "projectile_delete" | "projectile_collision" => {
                let native = self
                    .interface
                    .projectiles()
                    .get_projectile_type(projectile_id)
                    .map_err(|err| {
                        format!("get_projectile_type() after destructive setter failed: {err:?}")
                    })?;
                if !native.0 || native.1 {
                    return Err(format!(
                        "projectile type changed after destructive setter: weapon={}, piece={}",
                        native.0, native.1
                    ));
                }
                self.same_i32_if_present(label, message, "returnCount", 2)
            }
            _ => self.same_i32_if_present(label, message, "returnCount", 0),
        }
    }

    pub(crate) fn set_projectile_control(&mut self, message: &Value) -> Result<(), String> {
        let projectile_id = i32_field(message, "projectileID")?;
        let name = base_test_name(test_name_field(message)?);
        let synced_ctrl = self.interface.synced_ctrl();
        let projectile = synced_ctrl.projectile();
        match name {
            "projectile_position_after_set" => projectile
                .set_projectile_position(projectile_id, vec3_from_fields(message, "x", "y", "z")?)
                .map_err(|err| format!("set_projectile_position() failed: {err:?}"))?,
            "projectile_velocity_after_set" => projectile
                .set_projectile_velocity(projectile_id, vec3_from_fields(message, "x", "y", "z")?)
                .map_err(|err| format!("set_projectile_velocity() failed: {err:?}"))?,
            "projectile_gravity_after_set" => projectile
                .set_projectile_gravity(projectile_id, f32_field(message, "gravity")?)
                .map_err(|err| format!("set_projectile_gravity() failed: {err:?}"))?,
            "projectile_target_after_set" => projectile
                .set_projectile_target(
                    projectile_id,
                    spring_native::sys::ProjectileTargetRef {
                        targetID: -1,
                        targetType: 'g' as i32,
                        pos: vec3_from_fields(message, "targetX", "targetY", "targetZ")?,
                        isGroundTarget: true,
                    },
                )
                .map_err(|err| format!("set_projectile_target() failed: {err:?}"))?,
            "projectile_intercepted_after_set" => projectile
                .set_projectile_is_intercepted(projectile_id, bool_field(message, "intercepted")?)
                .map_err(|err| format!("set_projectile_is_intercepted() failed: {err:?}"))?,
            "projectile_ttl_after_set" => projectile
                .set_projectile_time_to_live(projectile_id, i32_field(message, "ttl")?)
                .map_err(|err| format!("set_projectile_time_to_live() failed: {err:?}"))?,
            "projectile_damage_after_set" => projectile
                .set_projectile_damages(
                    projectile_id,
                    0,
                    str_field(message, "tag")?,
                    f32_field(message, "damageValue")?,
                )
                .map_err(|err| format!("set_projectile_damages() failed: {err:?}"))?,
            "piece_projectile_params_after_set" => projectile
                .set_piece_projectile_params(
                    projectile_id,
                    i32_field(message, "explFlags")?,
                    f32_field(message, "spinAngle")?,
                    f32_field(message, "spinSpeed")?,
                    vec3_from_fields(message, "spinX", "spinY", "spinZ")?,
                )
                .map_err(|err| format!("set_piece_projectile_params() failed: {err:?}"))?,
            "projectile_delete" => {
                let result = projectile
                    .delete_projectile(projectile_id)
                    .map_err(|err| format!("delete_projectile() failed: {err:?}"))?;
                if !result {
                    return Err("delete_projectile() returned false".to_owned());
                }
                result
            }
            "projectile_collision" => {
                let result = projectile
                    .set_projectile_collision(projectile_id)
                    .map_err(|err| format!("set_projectile_collision() failed: {err:?}"))?;
                if !result {
                    return Err("set_projectile_collision() returned false".to_owned());
                }
                result
            }
            _ => return Err(format!("unsupported projectile setter `{name}`")),
        };
        Ok(())
    }
}

fn native_projectile_params(
    message: &Value,
    model: *const std::os::raw::c_char,
    ceg_tag: *const std::os::raw::c_char,
) -> Result<spring_native::sys::NativeProjectileParams, String> {
    Ok(spring_native::sys::NativeProjectileParams {
        pos: vec3_from_fields(message, "posX", "posY", "posZ")?,
        speed: vec3_from_fields(message, "speedX", "speedY", "speedZ")?,
        spread: vec3_from_fields(message, "spreadX", "spreadY", "spreadZ")?,
        error: vec3_from_fields(message, "errorX", "errorY", "errorZ")?,
        end: vec3_from_fields(message, "endX", "endY", "endZ")?,
        owner: i32_field(message, "ownerID")?,
        team: i32_field(message, "teamID")?,
        weaponNum: i32_field(message, "weaponNum")?,
        ttl: f32_field(message, "ttl")?,
        gravity: f32_field(message, "gravity")?,
        tracking: f32_field(message, "tracking")?,
        maxRange: f32_field(message, "maxRange")?,
        upTime: f32_field(message, "upTime")?,
        startAlpha: f32_field(message, "startAlpha")?,
        endAlpha: f32_field(message, "endAlpha")?,
        model,
        cegTag: ceg_tag,
    })
}

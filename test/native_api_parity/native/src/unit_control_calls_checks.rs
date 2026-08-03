use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_unit_control_call(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let unit = synced_ctrl.unit();
        if base_test_name(label) == "unit_set_piece_parent"
            && bool_field(message, "expectError").unwrap_or(false)
        {
            let result = unit.set_unit_piece_parent(
                unit_id,
                i32_field(message, "childPieceNum")? - 1,
                i32_field(message, "parentPieceNum")? - 1,
            );
            if result.is_ok() {
                return Err("set_unit_piece_parent unexpectedly accepted invalid piece".to_string());
            }
            return Ok(());
        }
        if base_test_name(label) == "unit_set_land_goal_not_air" {
            let result = unit.set_unit_land_goal(
                unit_id,
                vec3_from_fields(message, "goalX", "goalY", "goalZ")?,
                f32_field(message, "goalRadius")?.powi(2),
            );
            if result.is_ok() {
                return Err("set_unit_land_goal unexpectedly accepted a non-air unit".to_string());
            }
            return self.same_bool_if_present(label, message, "error", true);
        }
        match base_test_name(label) {
            "unit_add_impulse" => unit
                .add_unit_impulse(
                    unit_id,
                    vec3_from_fields(message, "x", "y", "z")?,
                    f32_field(message, "decayRate")?,
                )
                .map_err(|err| format!("add_unit_impulse() failed: {err:?}"))?,
            "unit_add_seismic_ping" => unit
                .add_unit_seismic_ping(unit_id, f32_field(message, "pingSize")?)
                .map_err(|err| format!("add_unit_seismic_ping() failed: {err:?}"))?,
            "unit_add_resource" => unit
                .add_unit_resource(
                    unit_id,
                    str_field(message, "resource")?,
                    f32_field(message, "amount")?,
                )
                .map_err(|err| format!("add_unit_resource() failed: {err:?}"))?,
            "unit_use_resource" => {
                let result = unit
                    .use_unit_resource(
                        unit_id,
                        str_field(message, "resource")?,
                        f32_field(message, "amount")?,
                    )
                    .map_err(|err| format!("use_unit_resource() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "ok", result)?;
                return Ok(());
            }
            "unit_clear_goal" => unit
                .clear_unit_goal(unit_id, bool_field(message, "cancelRaw")?)
                .map_err(|err| format!("clear_unit_goal() failed: {err:?}"))?,
            "unit_force_collision_update" => unit
                .force_unit_collision_update(unit_id)
                .map_err(|err| format!("force_unit_collision_update() failed: {err:?}"))?,
            "unit_set_build_speed" => unit
                .set_unit_build_speed(
                    unit_id,
                    f32_field(message, "buildSpeed")?,
                    f32_field(message, "repairSpeed")?,
                    f32_field(message, "reclaimSpeed")?,
                    f32_field(message, "resurrectSpeed")?,
                    f32_field(message, "captureSpeed")?,
                    f32_field(message, "terraformSpeed")?,
                )
                .map_err(|err| format!("set_unit_build_speed() failed: {err:?}"))?,
            "unit_set_flanking" => unit
                .set_unit_flanking(
                    unit_id,
                    str_field(message, "type")?,
                    vec3_from_fields(message, "frontX", "frontY", "frontZ")?,
                )
                .map_err(|err| format!("set_unit_flanking() failed: {err:?}"))?,
            "unit_set_mid_and_aim_pos" => {
                let result = unit
                    .set_unit_mid_and_aim_pos(
                        unit_id,
                        vec3_from_fields(message, "midX", "midY", "midZ")?,
                        vec3_from_fields(message, "aimX", "aimY", "aimZ")?,
                        bool_field(message, "relative")?,
                    )
                    .map_err(|err| format!("set_unit_mid_and_aim_pos() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)?;
                return Ok(());
            }
            "unit_set_move_goal" => unit
                .set_unit_move_goal(
                    unit_id,
                    vec3_from_fields(message, "x", "y", "z")?,
                    f32_field(message, "radius")?,
                    f32_field(message, "speed")?,
                    bool_field(message, "raw")?,
                )
                .map_err(|err| format!("set_unit_move_goal() failed: {err:?}"))?,
            "unit_set_physics" => unit
                .set_unit_physics(
                    unit_id,
                    vec3_from_fields(message, "posX", "posY", "posZ")?,
                    vec3_from_fields(message, "velX", "velY", "velZ")?,
                    vec3_from_fields(message, "rotX", "rotY", "rotZ")?,
                    vec3_from_fields(message, "dragX", "dragY", "dragZ")?,
                )
                .map_err(|err| format!("set_unit_physics() failed: {err:?}"))?,
            "unit_set_pos_error_params" => unit
                .set_unit_pos_error_params(
                    unit_id,
                    vec3_from_fields(message, "errorX", "errorY", "errorZ")?,
                    vec3_from_fields(message, "deltaX", "deltaY", "deltaZ")?,
                    i32_field(message, "nextUpdate")?,
                    i32_field(message, "allyTeamID")?,
                    bool_field(message, "setBit")?,
                )
                .map_err(|err| format!("set_unit_pos_error_params() failed: {err:?}"))?,
            "unit_set_resourcing" => unit
                .set_unit_resourcing(
                    unit_id,
                    str_field(message, "type")?,
                    f32_field(message, "amount")?,
                )
                .map_err(|err| format!("set_unit_resourcing() failed: {err:?}"))?,
            "unit_set_stealth" => unit
                .set_unit_stealth(unit_id, bool_field(message, "enabled")?)
                .map_err(|err| format!("set_unit_stealth() failed: {err:?}"))?,
            "unit_set_sonar_stealth" => unit
                .set_unit_sonar_stealth(unit_id, bool_field(message, "enabled")?)
                .map_err(|err| format!("set_unit_sonar_stealth() failed: {err:?}"))?,
            "unit_set_always_visible" => unit
                .set_unit_always_visible(unit_id, bool_field(message, "enabled")?)
                .map_err(|err| format!("set_unit_always_visible() failed: {err:?}"))?,
            "unit_set_use_air_los" => unit
                .set_unit_use_air_los(unit_id, bool_field(message, "enabled")?)
                .map_err(|err| format!("set_unit_use_air_los() failed: {err:?}"))?,
            "unit_set_crashing" => {
                let result = unit
                    .set_unit_crashing(unit_id, bool_field(message, "wantCrash")?)
                    .map_err(|err| format!("set_unit_crashing() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)?;
                return Ok(());
            }
            "unit_set_tooltip" => unit
                .set_unit_tooltip(unit_id, str_field(message, "tooltip")?)
                .map_err(|err| format!("set_unit_tooltip() failed: {err:?}"))?,
            "unit_set_stockpile" => unit
                .set_unit_stockpile(
                    unit_id,
                    i32_field(message, "stockpile")?,
                    f32_field(message, "buildPercent")?,
                )
                .map_err(|err| format!("set_unit_stockpile() failed: {err:?}"))?,
            "unit_set_shield_state" => unit
                .set_unit_shield_state(
                    unit_id,
                    -1,
                    bool_field(message, "enabled")?,
                    f32_field(message, "power")?,
                )
                .map_err(|err| format!("set_unit_shield_state() failed: {err:?}"))?,
            "unit_set_shield_recharge_delay" => unit
                .set_unit_shield_recharge_delay(unit_id, -1, f32_field(message, "delay")?)
                .map_err(|err| format!("set_unit_shield_recharge_delay() failed: {err:?}"))?,
            "unit_set_leaves_ghost" => unit
                .set_unit_leaves_ghost(
                    unit_id,
                    spring_native::SetUnitLeavesGhostOptions {
                        leaves_ghost: bool_field(message, "leavesGhost")?,
                        leave_dead_ghost: bool_field(message, "leaveDeadGhost")?,
                    },
                )
                .map_err(|err| format!("set_unit_leaves_ghost() failed: {err:?}"))?,
            "unit_set_use_weapons" => unit
                .set_unit_use_weapons(
                    unit_id,
                    spring_native::SetUnitUseWeaponsOptions {
                        force_use_weapons: bool_field(message, "forceUseWeapons")?,
                        allow_use_weapons: bool_field(message, "allowUseWeapons")?,
                    },
                )
                .map_err(|err| format!("set_unit_use_weapons() failed: {err:?}"))?,
            "unit_set_target_clear" => {
                let result = unit
                    .set_unit_target(
                        unit_id,
                        sys::UnitTargetRef {
                            targetID: -1,
                            pos: sys::Float3 {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            isGroundTarget: false,
                        },
                        spring_native::SetUnitTargetOptions {
                            manual_fire: bool_field(message, "dgun")?,
                            user_target: bool_field(message, "userTarget")?,
                        },
                        -1,
                    )
                    .map_err(|err| format!("set_unit_target() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)?;
                return Ok(());
            }
            "unit_set_loading_transport_clear" => unit
                .set_unit_loading_transport(unit_id, i32_field(message, "transportID")?)
                .map_err(|err| format!("set_unit_loading_transport() failed: {err:?}"))?,
            "unit_set_weapon_state" => unit
                .set_unit_weapon_state(
                    unit_id,
                    i32_field(message, "weaponNum")? - 1,
                    str_field(message, "key")?,
                    f32_field(message, "value")?,
                )
                .map_err(|err| format!("set_unit_weapon_state() failed: {err:?}"))?,
            "unit_set_weapon_damages" => unit
                .set_unit_weapon_damages(
                    unit_id,
                    i32_field(message, "weaponNum")? - 1,
                    str_field(message, "damageKey")?,
                    f32_field(message, "damageValue")?,
                )
                .map_err(|err| format!("set_unit_weapon_damages() failed: {err:?}"))?,
            "unit_set_heading_and_up_dir" => unit
                .set_unit_heading_and_up_dir(
                    unit_id,
                    i32_field(message, "heading")?,
                    vec3_from_fields(message, "upX", "upY", "upZ")?,
                )
                .map_err(|err| format!("set_unit_heading_and_up_dir() failed: {err:?}"))?,
            "unit_add_object_decal" => unit
                .add_object_decal(unit_id)
                .map_err(|err| format!("add_object_decal() failed: {err:?}"))?,
            "unit_remove_object_decal" => unit
                .remove_object_decal(unit_id)
                .map_err(|err| format!("remove_object_decal() failed: {err:?}"))?,
            "unit_set_selection_volume_data" => unit
                .set_unit_selection_volume_data(
                    unit_id,
                    vec3_from_fields(message, "scaleX", "scaleY", "scaleZ")?,
                    vec3_from_fields(message, "offsetX", "offsetY", "offsetZ")?,
                    i32_field(message, "volumeType")?,
                    i32_field(message, "testType")?,
                    i32_field(message, "primaryAxis")?,
                )
                .map_err(|err| format!("set_unit_selection_volume_data() failed: {err:?}"))?,
            "unit_set_piece_collision_volume_data" => unit
                .set_unit_piece_collision_volume_data(
                    unit_id,
                    i32_field(message, "pieceNum")? - 1,
                    bool_field(message, "enable")?,
                    vec3_from_fields(message, "scaleX", "scaleY", "scaleZ")?,
                    vec3_from_fields(message, "offsetX", "offsetY", "offsetZ")?,
                    i32_field(message, "volumeType")?,
                    i32_field(message, "primaryAxis")?,
                )
                .map_err(|err| format!("set_unit_piece_collision_volume_data() failed: {err:?}"))?,
            "unit_set_piece_visible" => unit
                .set_unit_piece_visible(
                    unit_id,
                    i32_field(message, "pieceNum")? - 1,
                    bool_field(message, "visible")?,
                )
                .map_err(|err| format!("set_unit_piece_visible() failed: {err:?}"))?,
            "unit_set_piece_matrix" => {
                let matrix = message
                    .get("matrix")
                    .and_then(Value::as_array)
                    .ok_or_else(|| format!("{label}.matrix: expected array"))?
                    .iter()
                    .map(|value| {
                        value.as_f64().map(|value| value as f32).ok_or_else(|| {
                            format!("{label}.matrix: expected numeric array element")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let matrix: [f32; 16] = matrix
                    .try_into()
                    .map_err(|_| format!("{label}.matrix: expected 16 elements"))?;
                let result = unit
                    .set_unit_piece_matrix(unit_id, i32_field(message, "pieceNum")? - 1, matrix)
                    .map_err(|err| format!("set_unit_piece_matrix() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)?;
                return Ok(());
            }
            "unit_set_piece_parent" => unit
                .set_unit_piece_parent(
                    unit_id,
                    i32_field(message, "childPieceNum")? - 1,
                    i32_field(message, "parentPieceNum")? - 1,
                )
                .map_err(|err| format!("set_unit_piece_parent() failed: {err:?}"))?,
            "unit_set_nano_pieces" => {
                let pieces = [i32_field(message, "pieceNum")? - 1];
                unit.set_unit_nano_pieces(unit_id, &pieces)
                    .map_err(|err| format!("set_unit_nano_pieces() failed: {err:?}"))?
            }
            "unit_set_travel" => unit
                .set_unit_travel()
                .map_err(|err| format!("set_unit_travel() failed: {err:?}"))?,
            "unit_set_fuel" => unit
                .set_unit_fuel()
                .map_err(|err| format!("set_unit_fuel() failed: {err:?}"))?,
            "unit_weapon_fire" => unit
                // Lua weapon numbers are one-based; UnitControl's C ABI is
                // explicitly zero-based.
                .unit_weapon_fire(unit_id, i32_field(message, "weaponNum")? - 1)
                .map_err(|err| format!("unit_weapon_fire() failed: {err:?}"))?,
            "unit_weapon_hold_fire" => unit
                // Lua weapon numbers are one-based; UnitControl's C ABI is
                // explicitly zero-based.
                .unit_weapon_hold_fire(unit_id, i32_field(message, "weaponNum")? - 1)
                .map_err(|err| format!("unit_weapon_hold_fire() failed: {err:?}"))?,
            "unit_finish_command" => unit
                .unit_finish_command(unit_id)
                .map_err(|err| format!("unit_finish_command() failed: {err:?}"))?,
            name => return Err(format!("unsupported unit control check `{name}`")),
        };

        self.same_i32_if_present(label, message, "returnCount", 0)
    }
}

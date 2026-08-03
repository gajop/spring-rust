use super::*;
use crate::support::*;

fn u32_field_from_i32(message: &Value, field: &str) -> Result<u32, String> {
    let value = i32_field(message, field)?;
    u32::try_from(value).map_err(|_| format!("{field}: expected a non-negative integer"))
}

fn explosion_params(message: &Value) -> Result<spring_native::sys::NativeExplosionParams, String> {
    let mut params = spring_native::sys::NativeExplosionParams::default();
    params.damages = f32_field(message, "damages")?;
    params.weaponDefID = i32_field(message, "weaponDef")?;
    params.ownerID = i32_field(message, "owner")?;
    params.hitUnitID = i32_field(message, "hitUnit")?;
    params.hitFeatureID = i32_field(message, "hitFeature")?;
    params.craterAreaOfEffect = f32_field(message, "craterAreaOfEffect")?;
    params.damageAreaOfEffect = f32_field(message, "damageAreaOfEffect")?;
    params.edgeEffectiveness = f32_field(message, "edgeEffectiveness")?;
    params.explosionSpeed = f32_field(message, "explosionSpeed")?;
    params.gfxMod = f32_field(message, "gfxMod")?;
    params.impactOnly = bool_field(message, "impactOnly")?;
    params.ignoreOwner = bool_field(message, "ignoreOwner")?;
    params.damageGround = bool_field(message, "damageGround")?;
    params.projectileID = i32_field(message, "projectileID")?;
    Ok(params)
}

impl NativeApiParity {
    pub(crate) fn check_effects_path_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "spawn_explosion" => {
                self.interface
                    .synced_ctrl()
                    .effects()
                    .spawn_explosion(
                        vec3_from_fields(message, "posX", "posY", "posZ")?,
                        vec3_from_fields(message, "dirX", "dirY", "dirZ")?,
                        explosion_params(message)?,
                    )
                    .map_err(|err| format!("spawn_explosion() failed: {err:?}"))?;
                Ok(())
            }
            "spawn_ceg" => {
                let name = CString::new(str_field(message, "cegName")?)
                    .map_err(|_| "cegName contains an embedded NUL".to_string())?;
                let (native_success, native_id) = self
                    .interface
                    .synced_ctrl()
                    .effects()
                    .spawn_ceg(
                        spring_native::sys::DefRef {
                            name: name.as_ptr(),
                            id: -1,
                        },
                        vec3_from_fields(message, "posX", "posY", "posZ")?,
                        vec3_from_fields(message, "dirX", "dirY", "dirZ")?,
                        f32_field(message, "radius")?,
                        f32_field(message, "damage")?,
                        f32_field(message, "dmgMod")?,
                    )
                    .map_err(|err| format!("spawn_ceg() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native_success)?;
                self.same_i32_if_present(label, message, "cegID", native_id)
            }
            "spawn_sfx" => {
                let native = self
                    .interface
                    .synced_ctrl()
                    .effects()
                    .spawn_sfx(
                        i32_field(message, "unitID")?,
                        i32_field(message, "sfxID")?,
                        vec3_from_fields(message, "posX", "posY", "posZ")?,
                        vec3_from_fields(message, "dirX", "dirY", "dirZ")?,
                        f32_field(message, "radius")?,
                        f32_field(message, "damage")?,
                        bool_field(message, "absolute")?,
                    )
                    .map_err(|err| format!("spawn_sfx() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native)
            }
            "path_node_costs" => {
                let overlay = u32_field_from_i32(message, "overlayIndex")?;
                let size_x = u32_field_from_i32(message, "sizeX")?;
                let size_z = u32_field_from_i32(message, "sizeZ")?;
                let cost_index = u32_field_from_i32(message, "costIndex")?;
                let node_x = u32_field_from_i32(message, "nodeX")?;
                let node_z = u32_field_from_i32(message, "nodeZ")?;
                let cost = f32_field(message, "cost")?;
                let path = self.interface.path_finder();

                let initialized = path
                    .init_path_node_costs_array(overlay, size_x, size_z)
                    .map_err(|err| format!("init_path_node_costs_array() failed: {err:?}"))?;
                let set_cost = path
                    .set_path_node_cost(overlay, cost_index, cost)
                    .map_err(|err| format!("set_path_node_cost() failed: {err:?}"))?;
                let active = path
                    .set_path_node_costs(overlay)
                    .map_err(|err| format!("set_path_node_costs() failed: {err:?}"))?;
                let costs = path
                    .get_path_node_costs(overlay)
                    .map_err(|err| format!("get_path_node_costs() failed: {err:?}"))?;
                let active_cost = path
                    .get_path_node_cost(node_x, node_z)
                    .map_err(|err| format!("get_path_node_cost() failed: {err:?}"))?;
                let freed = path
                    .free_path_node_costs_array(overlay)
                    .map_err(|err| format!("free_path_node_costs_array() failed: {err:?}"))?;

                self.same_bool_if_present(label, message, "init", initialized)?;
                self.same_bool_if_present(label, message, "setCost", set_cost)?;
                self.same_bool_if_present(label, message, "active", active)?;
                self.same_i32_if_present(label, message, "costCount", costs.len() as i32)?;
                let cost_value = costs.get(cost_index as usize).copied().ok_or_else(|| {
                    format!("get_path_node_costs() did not return index {cost_index}")
                })?;
                self.same_if_present(label, message, "costValue", cost_value)?;
                self.same_if_present(label, message, "activeCost", active_cost)?;
                self.same_bool_if_present(label, message, "free", freed)
            }
            "request_path" => {
                let move_def_name = str_field(message, "moveDefName")?;
                let path_id = self
                    .interface
                    .path_finder()
                    .request_path(
                        0,
                        Some(move_def_name),
                        vec3_from_fields(message, "startX", "startY", "startZ")?,
                        vec3_from_fields(message, "endX", "endY", "endZ")?,
                        f32_field(message, "radius")?,
                    )
                    .map_err(|err| format!("request_path() failed: {err:?}"))?;
                let valid = path_id > 0;
                self.same_bool_if_present(label, message, "valid", valid)?;
                if valid {
                    self.interface
                        .path_finder()
                        .delete_path(path_id)
                        .map_err(|err| format!("delete_path() failed: {err:?}"))?;
                }
                Ok(())
            }
            _ => Err(format!("unsupported effects/path check `{label}`")),
        }
    }
}

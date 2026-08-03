use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn set_terrain_control_call(&mut self, message: &Value) -> Result<(), String> {
        let label = test_name_field(message)?;
        let x1 = f32_field(message, "x1").unwrap_or(0.0);
        let z1 = f32_field(message, "z1").unwrap_or(0.0);
        let x2 = f32_field(message, "x2").unwrap_or(x1 + 32.0);
        let z2 = f32_field(message, "z2").unwrap_or(z1 + 32.0);
        let height = f32_field(message, "height").unwrap_or(0.0);
        let factor = f32_field(message, "factor").unwrap_or(1.0);
        let synced_ctrl = self.interface.synced_ctrl();
        let terrain = synced_ctrl.terrain();
        match base_test_name(label) {
            "terrain_add_grass" => terrain
                .add_grass(
                    f32_field(message, "x")?,
                    f32_field(message, "z")?,
                    u8::try_from(i32_field(message, "grass")?)
                        .map_err(|_| "grass must fit in uint8".to_string())?,
                )
                .map_err(|err| format!("add_grass() failed: {err:?}"))?,
            "terrain_remove_grass" => terrain
                .remove_grass(f32_field(message, "x")?, f32_field(message, "z")?)
                .map_err(|err| format!("remove_grass() failed: {err:?}"))?,
            "terrain_level_height_map" => terrain
                .level_height_map(x1, z1, x2, z2, height)
                .map_err(|err| format!("level_height_map() failed: {err:?}"))?,
            "terrain_adjust_height_map" => terrain
                .adjust_height_map(x1, z1, x2, z2, height)
                .map_err(|err| format!("adjust_height_map() failed: {err:?}"))?,
            "terrain_revert_height_map" => terrain
                .revert_height_map(x1, z1, x2, z2, factor)
                .map_err(|err| format!("revert_height_map() failed: {err:?}"))?,
            "terrain_level_original_height_map" => terrain
                .level_original_height_map(x1, z1, x2, z2, height)
                .map_err(|err| format!("level_original_height_map() failed: {err:?}"))?,
            "terrain_adjust_original_height_map" => terrain
                .adjust_original_height_map(x1, z1, x2, z2, height)
                .map_err(|err| format!("adjust_original_height_map() failed: {err:?}"))?,
            "terrain_revert_original_height_map" => terrain
                .revert_original_height_map(x1, z1, x2, z2, factor)
                .map_err(|err| format!("revert_original_height_map() failed: {err:?}"))?,
            "terrain_level_smooth_mesh" => terrain
                .level_smooth_mesh(x1, z1, x2, z2, height)
                .map_err(|err| format!("level_smooth_mesh() failed: {err:?}"))?,
            "terrain_adjust_smooth_mesh" => terrain
                .adjust_smooth_mesh(x1, z1, x2, z2, height)
                .map_err(|err| format!("adjust_smooth_mesh() failed: {err:?}"))?,
            "terrain_revert_smooth_mesh" => terrain
                .revert_smooth_mesh(x1, z1, x2, z2, factor)
                .map_err(|err| format!("revert_smooth_mesh() failed: {err:?}"))?,
            "terrain_rebuild_smooth_mesh" => terrain
                .rebuild_smooth_mesh()
                .map_err(|err| format!("rebuild_smooth_mesh() failed: {err:?}"))?,
            name => return Err(format!("unsupported terrain control setter `{name}`")),
        };
        Ok(())
    }

    pub(crate) fn check_terrain_control_result(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        self.same_i32_if_present(label, message, "returnCount", 0)?;

        let name = base_test_name(label);
        let terrain = self.interface.terrain();
        match name {
            "terrain_add_grass" | "terrain_remove_grass" => {
                let grass = terrain
                    .get_grass(f32_field(message, "x")?, f32_field(message, "z")?)
                    .map_err(|err| format!("get_grass() failed: {err:?}"))?;
                self.same_if_present(label, message, "postGrassLevel", grass)?;
            }
            "terrain_level_height_map"
            | "terrain_adjust_height_map"
            | "terrain_revert_height_map" => {
                let x = f32_field(message, "x1")?;
                let z = f32_field(message, "z1")?;
                let height = terrain
                    .get_ground_height(x, z)
                    .map_err(|err| format!("get_ground_height() failed: {err:?}"))?;
                self.same_if_present(label, message, "postGroundHeight", height)?;
                let (normal, _) = terrain
                    .get_ground_normal(x, z, false)
                    .map_err(|err| format!("get_ground_normal() failed: {err:?}"))?;
                self.same_if_present(label, message, "postNormalX", normal.x)?;
                self.same_if_present(label, message, "postNormalY", normal.y)?;
                self.same_if_present(label, message, "postNormalZ", normal.z)?;
                let affected_height = terrain
                    .get_ground_height(880.0, 919.315979)
                    .map_err(|err| format!("get_ground_height(affected probe) failed: {err:?}"))?;
                self.same_if_present(label, message, "postAffectedGroundHeight", affected_height)?;
            }
            "terrain_level_original_height_map"
            | "terrain_adjust_original_height_map"
            | "terrain_revert_original_height_map" => {
                let height = terrain
                    .get_ground_orig_height(f32_field(message, "x1")?, f32_field(message, "z1")?)
                    .map_err(|err| format!("get_ground_orig_height() failed: {err:?}"))?;
                self.same_if_present(label, message, "postGroundOrigHeight", height)?;
                let affected_height = terrain
                    .get_ground_orig_height(880.0, 919.315979)
                    .map_err(|err| format!("get_ground_orig_height(affected probe) failed: {err:?}"))?;
                self.same_if_present(
                    label,
                    message,
                    "postAffectedGroundOrigHeight",
                    affected_height,
                )?;
            }
            "terrain_level_smooth_mesh"
            | "terrain_adjust_smooth_mesh"
            | "terrain_revert_smooth_mesh" => {
                let height = terrain
                    .get_smooth_mesh_height(f32_field(message, "x1")?, f32_field(message, "z1")?)
                    .map_err(|err| format!("get_smooth_mesh_height() failed: {err:?}"))?;
                self.same_if_present(label, message, "postSmoothMeshHeight", height)?;
            }
            "terrain_rebuild_smooth_mesh" => {
                let height = terrain
                    .get_smooth_mesh_height(904.0, 904.0)
                    .map_err(|err| format!("get_smooth_mesh_height() failed: {err:?}"))?;
                self.same_if_present(label, message, "postSmoothMeshHeight", height)?;
            }
            name => return Err(format!("unsupported terrain control result `{name}`")),
        }

        Ok(())
    }

    pub(crate) fn set_tidal(&mut self, message: &Value) -> Result<(), String> {
        self.interface
            .synced_ctrl()
            .terrain()
            .set_tidal(f32_field(message, "tidalStrength")?)
            .map_err(|err| format!("set_tidal() failed: {err:?}"))?;
        Ok(())
    }
}

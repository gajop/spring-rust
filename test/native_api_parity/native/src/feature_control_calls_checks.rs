use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_feature_control_call(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let feature = synced_ctrl.feature();

        match base_test_name(label) {
            "feature_set_always_visible" => {
                feature
                    .set_feature_always_visible(feature_id, bool_field(message, "enabled")?)
                    .map_err(|err| format!("set_feature_always_visible() failed: {err:?}"))?;
            }
            "feature_set_use_air_los" => {
                feature
                    .set_feature_use_air_los(feature_id, bool_field(message, "useAirLos")?)
                    .map_err(|err| format!("set_feature_use_air_los() failed: {err:?}"))?;
            }
            "feature_set_move_ctrl" => {
                feature
                    .set_feature_move_ctrl(
                        feature_id,
                        bool_field(message, "enabled")?,
                        vec3_from_fields(message, "velocityX", "velocityY", "velocityZ")?,
                        vec3_from_fields(message, "accelX", "accelY", "accelZ")?,
                        vec3_from_fields(message, "movementX", "movementY", "movementZ")?,
                    )
                    .map_err(|err| format!("set_feature_move_ctrl() failed: {err:?}"))?;
            }
            "feature_set_physics" => {
                feature
                    .set_feature_physics(
                        feature_id,
                        vec3_from_fields(message, "posX", "posY", "posZ")?,
                        vec3_from_fields(message, "velX", "velY", "velZ")?,
                        vec3_from_fields(message, "rotX", "rotY", "rotZ")?,
                        vec3_from_fields(message, "dragX", "dragY", "dragZ")?,
                    )
                    .map_err(|err| format!("set_feature_physics() failed: {err:?}"))?;
            }
            "feature_set_heading_and_up_dir" => {
                feature
                    .set_feature_heading_and_up_dir(
                        feature_id,
                        i32_field(message, "heading")?,
                        vec3_from_fields(message, "upX", "upY", "upZ")?,
                    )
                    .map_err(|err| format!("set_feature_heading_and_up_dir() failed: {err:?}"))?;
            }
            "feature_set_mid_and_aim_pos" => {
                let result = feature
                    .set_feature_mid_and_aim_pos(
                        feature_id,
                        vec3_from_fields(message, "midX", "midY", "midZ")?,
                        vec3_from_fields(message, "aimX", "aimY", "aimZ")?,
                        bool_field(message, "relative")?,
                    )
                    .map_err(|err| format!("set_feature_mid_and_aim_pos() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)?;
                return Ok(());
            }
            "feature_set_piece_visible" => {
                feature
                    .set_feature_piece_visible(
                        feature_id,
                        i32_field(message, "pieceNum")? - 1,
                        bool_field(message, "visible")?,
                    )
                    .map_err(|err| format!("set_feature_piece_visible() failed: {err:?}"))?;
            }
            "feature_set_piece_collision_volume_data" => {
                feature
                    .set_feature_piece_collision_volume_data(
                        feature_id,
                        i32_field(message, "pieceNum")? - 1,
                        bool_field(message, "enable")?,
                        vec3_from_fields(message, "scaleX", "scaleY", "scaleZ")?,
                        vec3_from_fields(message, "offsetX", "offsetY", "offsetZ")?,
                        i32_field(message, "volumeType")?,
                        i32_field(message, "primaryAxis")?,
                    )
                    .map_err(|err| {
                        format!("set_feature_piece_collision_volume_data() failed: {err:?}")
                    })?;
            }
            "feature_set_selection_volume_data" => {
                feature
                    .set_feature_selection_volume_data(
                        feature_id,
                        vec3_from_fields(message, "scaleX", "scaleY", "scaleZ")?,
                        vec3_from_fields(message, "offsetX", "offsetY", "offsetZ")?,
                        i32_field(message, "volumeType")?,
                        i32_field(message, "primaryAxis")?,
                        i32_field(message, "testType")? != 0,
                    )
                    .map_err(|err| {
                        format!("set_feature_selection_volume_data() failed: {err:?}")
                    })?;
            }
            "feature_set_piece_matrix" => {
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
                let result = feature
                    .set_feature_piece_matrix(
                        feature_id,
                        i32_field(message, "pieceNum")? - 1,
                        matrix,
                    )
                    .map_err(|err| format!("set_feature_piece_matrix() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", result)?;
                return Ok(());
            }
            "feature_set_blocking" => {
                feature
                    .set_feature_blocking(
                        feature_id,
                        spring_native::SetFeatureBlockingOptions {
                            blocking: bool_field(message, "blocking")?,
                            solid_objects: bool_field(message, "solidObjects")?,
                            projectiles: bool_field(message, "projectiles")?,
                            quad_map_rays: bool_field(message, "quadMapRays")?,
                            crushable: bool_field(message, "crushable")?,
                            block_enemy_pushing: bool_field(message, "blockEnemyPushing")?,
                            block_height_changes: bool_field(message, "blockHeightChanges")?,
                        },
                    )
                    .map_err(|err| format!("set_feature_blocking() failed: {err:?}"))?;
                let native = self
                    .interface
                    .features()
                    .get_feature_blocking(feature_id)
                    .map_err(|err| format!("get_feature_blocking() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "result", native.isBlocking)?;
                return Ok(());
            }
            name => return Err(format!("unsupported feature control check `{name}`")),
        }

        self.same_i32_if_present(label, message, "returnCount", 0)
    }
}

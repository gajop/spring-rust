use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_pieces_string_set(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "get_unit_script_names" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .units_pieces()
                    .get_unit_script_names(unit_id)
                    .map_err(|err| format!("get_unit_script_names({unit_id}) failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "names", &native)
            }
            "get_unit_piece_list" => {
                let unit_id = i32_field(message, "unitID")?;
                let native = self
                    .interface
                    .units_pieces()
                    .get_unit_piece_list(unit_id)
                    .map_err(|err| format!("get_unit_piece_list({unit_id}) failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "names", &native)
            }
            "get_model_piece_list" => {
                let model_name = str_field(message, "modelName")?;
                let native = self
                    .interface
                    .units_pieces()
                    .get_model_piece_list(model_name)
                    .map_err(|err| format!("get_model_piece_list({model_name}) failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "names", &native)
            }
            "get_feature_piece_list" => {
                let feature_id = i32_field(message, "featureID")?;
                let native = self
                    .interface
                    .units_pieces()
                    .get_feature_piece_list(feature_id)
                    .map_err(|err| {
                        format!("get_feature_piece_list({feature_id}) failed: {err:?}")
                    })?;
                self.same_string_set_if_present(label, message, "names", &native)
            }
            name => Err(format!("unsupported pieces string-set check `{name}`")),
        }
    }

    pub(crate) fn check_pieces_map(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let native_entries = match base_test_name(label) {
            "get_model_piece_map" => {
                let model_name = str_field(message, "modelName")?;
                self.interface
                    .units_pieces()
                    .get_model_piece_map(model_name)
                    .map_err(|err| format!("get_model_piece_map({model_name}) failed: {err:?}"))?
            }
            "get_unit_piece_map" => {
                let unit_id = i32_field(message, "unitID")?;
                self.interface
                    .units_pieces()
                    .get_unit_piece_map(unit_id)
                    .map_err(|err| format!("get_unit_piece_map({unit_id}) failed: {err:?}"))?
            }
            "get_feature_piece_map" => {
                let feature_id = i32_field(message, "featureID")?;
                self.interface
                    .units_pieces()
                    .get_feature_piece_map(feature_id)
                    .map_err(|err| format!("get_feature_piece_map({feature_id}) failed: {err:?}"))?
            }
            name => return Err(format!("unsupported pieces map check `{name}`")),
        };
        let native = native_entries
            .iter()
            .map(|entry| {
                let name = unsafe { CStr::from_ptr(entry.name) }
                    .to_string_lossy()
                    .into_owned();
                (name, entry.pieceNum)
            })
            .collect::<Vec<_>>();
        self.same_string_i32_pairs_if_present(label, message, "pieces", &native)
    }

    pub(crate) fn check_pieces_root(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let root_piece = match base_test_name(label) {
            "get_model_root_piece" => {
                let model_name = str_field(message, "modelName")?;
                self.interface
                    .units_pieces()
                    .get_model_root_piece(model_name)
                    .map_err(|err| format!("get_model_root_piece({model_name}) failed: {err:?}"))?
            }
            "get_unit_root_piece" => {
                let unit_id = i32_field(message, "unitID")?;
                self.interface
                    .units_pieces()
                    .get_unit_root_piece(unit_id)
                    .map_err(|err| format!("get_unit_root_piece({unit_id}) failed: {err:?}"))?
            }
            "get_feature_root_piece" => {
                let feature_id = i32_field(message, "featureID")?;
                self.interface
                    .units_pieces()
                    .get_feature_root_piece(feature_id)
                    .map_err(|err| {
                        format!("get_feature_root_piece({feature_id}) failed: {err:?}")
                    })?
            }
            name => return Err(format!("unsupported pieces root check `{name}`")),
        };
        self.same_i32_if_present(label, message, "rootPiece", root_piece)
    }

    pub(crate) fn check_pieces_vec3(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let vector = match base_test_name(label) {
            "get_unit_piece_position" => {
                let unit_id = i32_field(message, "unitID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_unit_piece_position(unit_id, piece_num)
                    .map_err(|err| {
                        format!("get_unit_piece_position({unit_id}, {piece_num}) failed: {err:?}")
                    })?
            }
            "get_unit_piece_direction" => {
                let unit_id = i32_field(message, "unitID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_unit_piece_direction(unit_id, piece_num)
                    .map_err(|err| {
                        format!("get_unit_piece_direction({unit_id}, {piece_num}) failed: {err:?}")
                    })?
            }
            "get_feature_piece_position" => {
                let feature_id = i32_field(message, "featureID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_feature_piece_position(feature_id, piece_num)
                    .map_err(|err| {
                        format!(
                            "get_feature_piece_position({feature_id}, {piece_num}) failed: {err:?}"
                        )
                    })?
            }
            "get_feature_piece_direction" => {
                let feature_id = i32_field(message, "featureID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_feature_piece_direction(feature_id, piece_num)
                    .map_err(|err| {
                        format!(
                            "get_feature_piece_direction({feature_id}, {piece_num}) failed: {err:?}"
                        )
                    })?
            }
            name => return Err(format!("unsupported pieces vec3 check `{name}`")),
        };
        self.same_vec3(label, vector, message)
    }

    pub(crate) fn check_pieces_pos_dir(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let pos_dir = match base_test_name(label) {
            "get_unit_piece_pos_dir" => {
                let unit_id = i32_field(message, "unitID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_unit_piece_pos_dir(unit_id, piece_num)
                    .map_err(|err| {
                        format!("get_unit_piece_pos_dir({unit_id}, {piece_num}) failed: {err:?}")
                    })?
            }
            "get_feature_piece_pos_dir" => {
                let feature_id = i32_field(message, "featureID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_feature_piece_pos_dir(feature_id, piece_num)
                    .map_err(|err| {
                        format!(
                            "get_feature_piece_pos_dir({feature_id}, {piece_num}) failed: {err:?}"
                        )
                    })?
            }
            name => return Err(format!("unsupported pieces pos-dir check `{name}`")),
        };
        self.same_if_present(label, message, "posX", pos_dir.position.x)?;
        self.same_if_present(label, message, "posY", pos_dir.position.y)?;
        self.same_if_present(label, message, "posZ", pos_dir.position.z)?;
        self.same_if_present(label, message, "dirX", pos_dir.direction.x)?;
        self.same_if_present(label, message, "dirY", pos_dir.direction.y)?;
        self.same_if_present(label, message, "dirZ", pos_dir.direction.z)
    }

    pub(crate) fn check_pieces_matrix(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let matrix = match base_test_name(label) {
            "get_unit_piece_matrix" => {
                let unit_id = i32_field(message, "unitID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_unit_piece_matrix(unit_id, piece_num)
                    .map_err(|err| {
                        format!("get_unit_piece_matrix({unit_id}, {piece_num}) failed: {err:?}")
                    })?
            }
            "get_feature_piece_matrix" => {
                let feature_id = i32_field(message, "featureID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_feature_piece_matrix(feature_id, piece_num)
                    .map_err(|err| {
                        format!(
                            "get_feature_piece_matrix({feature_id}, {piece_num}) failed: {err:?}"
                        )
                    })?
            }
            name => return Err(format!("unsupported pieces matrix check `{name}`")),
        };
        for (index, value) in matrix.m.iter().enumerate() {
            self.same_if_present(label, message, &format!("m{}", index + 1), *value)?;
        }
        Ok(())
    }

    pub(crate) fn check_pieces_info(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let (info, exists) = match base_test_name(label) {
            "get_unit_piece_info" => {
                let unit_id = i32_field(message, "unitID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_unit_piece_info(unit_id, piece_num)
                    .map_err(|err| {
                        format!("get_unit_piece_info({unit_id}, {piece_num}) failed: {err:?}")
                    })?
            }
            "get_feature_piece_info" => {
                let feature_id = i32_field(message, "featureID")?;
                let piece_num = i32_field(message, "pieceNum")?;
                self.interface
                    .units_pieces()
                    .get_feature_piece_info(feature_id, piece_num)
                    .map_err(|err| {
                        format!("get_feature_piece_info({feature_id}, {piece_num}) failed: {err:?}")
                    })?
            }
            name => return Err(format!("unsupported pieces info check `{name}`")),
        };

        self.same_bool_if_present(label, message, "exists", exists)?;
        if !exists {
            return Ok(());
        }

        self.same_string_if_present(label, message, "pieceName", &cstr_or_empty(info.name)?)?;
        self.same_string_if_present(label, message, "parent", &cstr_or_empty(info.parent)?)?;
        self.same_bool_if_present(label, message, "isEmpty", info.isEmpty)?;
        self.same_if_present(label, message, "minX", info.min.x)?;
        self.same_if_present(label, message, "minY", info.min.y)?;
        self.same_if_present(label, message, "minZ", info.min.z)?;
        self.same_if_present(label, message, "maxX", info.max.x)?;
        self.same_if_present(label, message, "maxY", info.max.y)?;
        self.same_if_present(label, message, "maxZ", info.max.z)?;
        self.same_if_present(label, message, "offsetX", info.offset.x)?;
        self.same_if_present(label, message, "offsetY", info.offset.y)?;
        self.same_if_present(label, message, "offsetZ", info.offset.z)?;
        self.same_if_present(label, message, "emitDirX", info.emitDir.x)?;
        self.same_if_present(label, message, "emitDirY", info.emitDir.y)?;
        self.same_if_present(label, message, "emitDirZ", info.emitDir.z)?;

        let native_children = if info.childCount == 0 || info.children.is_null() {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(info.children, info.childCount as usize)
                    .iter()
                    .map(|&child| cstr_or_empty(child))
                    .collect::<Result<Vec<_>, _>>()?
            }
        };
        if let Some(lua_children) = message.get("children") {
            let lua_children = lua_children
                .as_array()
                .ok_or_else(|| format!("{label}.children: expected array"))?
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| format!("{label}.children: expected string"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            if native_children != lua_children {
                return Err(format!(
                    "{label}.children: native={native_children:?}, lua={lua_children:?}"
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn check_pieces_script_piece(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        if base_test_name(label) != "get_unit_script_piece" {
            return Err(format!("unsupported pieces script-piece check `{label}`"));
        }
        let unit_id = i32_field(message, "unitID")?;
        let script_num = i32_field(message, "scriptNum")?;
        let native = self
            .interface
            .units_pieces()
            .get_unit_script_piece(unit_id, script_num)
            .map_err(|err| {
                format!("get_unit_script_piece({unit_id}, {script_num}) failed: {err:?}")
            })?;
        self.same_i32_if_present(label, message, "pieceNum", native)
    }
}

use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_pieces_string_set(&mut self, message: &Value, label: &str) -> Result<(), String> {
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
            "get_feature_piece_list" => {
                let feature_id = i32_field(message, "featureID")?;
                let native = self
                    .interface
                    .units_pieces()
                    .get_feature_piece_list(feature_id)
                    .map_err(|err| format!("get_feature_piece_list({feature_id}) failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "names", &native)
            }
            name => Err(format!("unsupported pieces string-set check `{name}`")),
        }
    }

    pub(crate) fn check_pieces_map(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let native_entries = match base_test_name(label) {
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
                let name = unsafe { CStr::from_ptr(entry.name) }.to_string_lossy().into_owned();
                (name, entry.pieceNum)
            })
            .collect::<Vec<_>>();
        self.same_string_i32_pairs_if_present(label, message, "pieces", &native)
    }
}

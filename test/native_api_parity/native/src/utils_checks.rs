use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_utils_i32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_cegid" => {
                let ceg_name = str_field(message, "cegName")?;
                self.interface.utils().get_cegid(ceg_name)
                    .map_err(|err| format!("get_cegid({ceg_name}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported utils i32 check `{label}`")),
        };
        self.same_i32_if_present(label, message, "value", native)
    }

    pub(crate) fn check_utils_vec3(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_def_id = i32_field(message, "unitDefID")?;
        let pos = if message.get("inputX").is_some() {
            vec3_from_fields(message, "inputX", "inputY", "inputZ")?
        } else {
            vec3_from_fields(message, "x", "y", "z")?
        };
        let facing = i32_field(message, "facing")?;
        let native = match test_name {
            "pos2_build_pos" => self.interface.utils().pos2_build_pos(unit_def_id, pos, facing)
                .map_err(|err| format!("pos2_build_pos({unit_def_id}, _, {facing}) failed: {err:?}"))?,
            "closest_build_pos" => {
                let team_id = i32_field(message, "teamID")?;
                let search_radius = f32_field(message, "searchRadius")?;
                let min_dist = i32_field(message, "minDistance")?;
                self.interface.utils().closest_build_pos(team_id, unit_def_id, pos, search_radius, min_dist, facing)
                    .map_err(|err| format!("closest_build_pos({team_id}, {unit_def_id}, _, {search_radius}, {min_dist}, {facing}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported utils vec3 check `{label}`")),
        };
        if test_name == "pos2_build_pos" {
            self.same_if_present(label, message, "buildX", native.x)?;
            self.same_if_present(label, message, "buildY", native.y)?;
            return self.same_if_present(label, message, "buildZ", native.z);
        }
        self.same_vec3(label, native, message)
    }

    pub(crate) fn check_unit_def_dimensions(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_def_id = i32_field(message, "unitDefID")?;
        let native = self
            .interface
            .utils()
            .get_unit_def_dimensions(unit_def_id)
            .map_err(|err| format!("get_unit_def_dimensions({unit_def_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "height", native.height)?;
        self.same_if_present(label, message, "radius", native.radius)?;
        self.same_if_present(label, message, "midx", native.midx)?;
        self.same_if_present(label, message, "minx", native.minx)?;
        self.same_if_present(label, message, "maxx", native.maxx)?;
        self.same_if_present(label, message, "midy", native.midy)?;
        self.same_if_present(label, message, "miny", native.miny)?;
        self.same_if_present(label, message, "maxy", native.maxy)?;
        self.same_if_present(label, message, "midz", native.midz)?;
        self.same_if_present(label, message, "minz", native.minz)?;
        self.same_if_present(label, message, "maxz", native.maxz)
    }

    pub(crate) fn check_test_move_order(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_def_id = i32_field(message, "unitDefID")?;
        let pos = vec3_from_fields(message, "x", "y", "z")?;
        let dir = vec3_from_fields(message, "dirX", "dirY", "dirZ")?;
        let native = self.interface.utils().test_move_order(
            unit_def_id,
            pos,
            dir,
            bool_field(message, "testTerrain")?,
            bool_field(message, "testObjects")?,
            bool_field(message, "centerOnly")?,
        )
            .map_err(|err| format!("test_move_order({unit_def_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "canMove", native)
    }

    pub(crate) fn check_test_build_order(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_def_id = i32_field(message, "unitDefID")?;
        let pos = vec3_from_fields(message, "x", "y", "z")?;
        let facing = i32_field(message, "facing")?;
        let (status, can_build, feature_id) = self
            .interface
            .utils()
            .test_build_order(unit_def_id, pos, facing)
            .map_err(|err| format!("test_build_order({unit_def_id}, _, {facing}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "status", status)?;
        self.same_bool_if_present(label, message, "canBuild", can_build)?;
        self.same_i32_if_present(label, message, "featureID", feature_id)
    }

}

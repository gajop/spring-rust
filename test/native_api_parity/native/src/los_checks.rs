use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_los_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_id = i32_field(message, "unitID")?;
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let (field, native) = match test_name {
            "is_unit_in_los" => ("inLos", self.interface.los().is_unit_in_los(unit_id, ally_team_id)
                .map_err(|err| format!("is_unit_in_los({unit_id}, {ally_team_id}) failed: {err:?}"))?),
            "is_unit_in_air_los" => ("inAirLos", self.interface.los().is_unit_in_air_los(unit_id, ally_team_id)
                .map_err(|err| format!("is_unit_in_air_los({unit_id}, {ally_team_id}) failed: {err:?}"))?),
            "is_unit_in_radar" => ("inRadar", self.interface.los().is_unit_in_radar(unit_id, ally_team_id)
                .map_err(|err| format!("is_unit_in_radar({unit_id}, {ally_team_id}) failed: {err:?}"))?),
            "is_unit_in_jammer" => ("inJammer", self.interface.los().is_unit_in_jammer(unit_id, ally_team_id)
                .map_err(|err| format!("is_unit_in_jammer({unit_id}, {ally_team_id}) failed: {err:?}"))?),
            _ => return Err(format!("unsupported LOS bool check `{label}`")),
        };
        self.same_bool_if_present(label, message, field, native)
    }
    pub(crate) fn check_pos_los_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let pos = sys::Float3 {
            x: f32_field(message, "x")?,
            y: message.get("y").and_then(Value::as_f64).unwrap_or(0.0) as f32,
            z: f32_field(message, "z")?,
        };
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let (field, native) = match test_name {
            "is_pos_in_los" => ("inLos", self.interface.los().is_pos_in_los(pos, ally_team_id)
                .map_err(|err| format!("is_pos_in_los(_, {ally_team_id}) failed: {err:?}"))?),
            "is_pos_in_air_los" => ("inAirLos", self.interface.los().is_pos_in_air_los(pos, ally_team_id)
                .map_err(|err| format!("is_pos_in_air_los(_, {ally_team_id}) failed: {err:?}"))?),
            "is_pos_in_radar" => ("inRadar", self.interface.los().is_pos_in_radar(pos, ally_team_id)
                .map_err(|err| format!("is_pos_in_radar(_, {ally_team_id}) failed: {err:?}"))?),
            _ => return Err(format!("unsupported position LOS bool check `{label}`")),
        };
        self.same_bool_if_present(label, message, field, native)
    }
    pub(crate) fn check_position_los_state(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let pos = vec3_from_fields(message, "x", "y", "z")?;
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let native = self
            .interface
            .los()
            .get_position_los_state(pos, ally_team_id)
            .map_err(|err| format!("get_position_los_state(_, {ally_team_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "inLosOrRadar", native.inLosOrRadar)?;
        self.same_bool_if_present(label, message, "inLos", native.inLos)?;
        self.same_bool_if_present(label, message, "inRadar", native.inRadar)?;
        self.same_bool_if_present(label, message, "inJammer", native.inJammer)
    }
    pub(crate) fn check_radar_error_params(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let native = self
            .interface
            .los()
            .get_radar_error_params(ally_team_id)
            .map_err(|err| format!("get_radar_error_params({ally_team_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "radarErrorSize", native.radarErrorSize)?;
        self.same_if_present(label, message, "baseRadarErrorSize", native.baseRadarErrorSize)?;
        self.same_if_present(label, message, "baseRadarErrorMult", native.baseRadarErrorMult)
    }
    pub(crate) fn check_closest_valid_position(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let pos = sys::Float3 {
            x: f32_field(message, "x")?,
            y: message.get("y").and_then(Value::as_f64).unwrap_or(0.0) as f32,
            z: f32_field(message, "z")?,
        };
        let radius = f32_field(message, "radius")?;
        let unit_def_id = i32_field(message, "unitDefID")?;
        let team_id = message.get("teamID").and_then(Value::as_i64).unwrap_or(0) as i32;
        let native_has_position = self
            .interface
            .los()
            .get_closest_valid_position(pos, radius, unit_def_id, team_id)
            .is_ok();
        self.same_bool_if_present(label, message, "hasPosition", native_has_position)
    }
    pub(crate) fn set_radar_error_params(&mut self, message: &Value) -> Result<(), String> {
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let radar_error_size = f32_field(message, "radarErrorSize")?;
        let base_radar_error_size = f32_field(message, "baseRadarErrorSize")?;
        let base_radar_error_mult = f32_field(message, "baseRadarErrorMult")?;
        self.interface
            .synced_ctrl()
            .game_config()
            .set_radar_error_params(
                ally_team_id,
                radar_error_size,
                base_radar_error_size,
                base_radar_error_mult,
            )
            .map_err(|err| format!("set_radar_error_params({ally_team_id}, ...) failed: {err:?}"))?;
        Ok(())
    }
}

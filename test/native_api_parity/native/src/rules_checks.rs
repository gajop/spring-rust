use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_rules_param(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let param_name = str_field(message, "paramName")?;
        let scope = str_field(message, "scope")?;
        let rules = self.interface.rules_params();
        let (value, _, exists, names) = match scope {
            "game" => {
                let (value, los, exists) = rules
                    .get_game_rules_param(param_name)
                    .map_err(|err| format!("get_game_rules_param({param_name}) failed: {err:?}"))?;
                let names = rules
                    .get_game_rules_params()
                    .map_err(|err| format!("get_game_rules_params() failed: {err:?}"))?;
                (value, los, exists, names)
            }
            "team" => {
                let team_id = i32_field(message, "teamID")?;
                let (value, los, exists) = rules
                    .get_team_rules_param(team_id, param_name)
                    .map_err(|err| format!("get_team_rules_param({team_id}, {param_name}) failed: {err:?}"))?;
                let names = rules
                    .get_team_rules_params(team_id)
                    .map_err(|err| format!("get_team_rules_params({team_id}) failed: {err:?}"))?;
                (value, los, exists, names)
            }
            "player" => {
                let player_id = i32_field(message, "playerID")?;
                let (value, los, exists) = rules
                    .get_player_rules_param(player_id, param_name)
                    .map_err(|err| format!("get_player_rules_param({player_id}, {param_name}) failed: {err:?}"))?;
                let names = rules
                    .get_player_rules_params(player_id)
                    .map_err(|err| format!("get_player_rules_params({player_id}) failed: {err:?}"))?;
                (value, los, exists, names)
            }
            "unit" => {
                let unit_id = i32_field(message, "unitID")?;
                let (value, los, exists) = rules
                    .get_unit_rules_param(unit_id, param_name)
                    .map_err(|err| format!("get_unit_rules_param({unit_id}, {param_name}) failed: {err:?}"))?;
                let names = rules
                    .get_unit_rules_params(unit_id)
                    .map_err(|err| format!("get_unit_rules_params({unit_id}) failed: {err:?}"))?;
                (value, los, exists, names)
            }
            "feature" => {
                let feature_id = i32_field(message, "featureID")?;
                let (value, los, exists) = rules
                    .get_feature_rules_param(feature_id, param_name)
                    .map_err(|err| format!("get_feature_rules_param({feature_id}, {param_name}) failed: {err:?}"))?;
                let names = rules
                    .get_feature_rules_params(feature_id)
                    .map_err(|err| format!("get_feature_rules_params({feature_id}) failed: {err:?}"))?;
                (value, los, exists, names)
            }
            other => return Err(format!("unsupported rules param scope `{other}`")),
        };
        if !exists {
            return Err(format!("{label}.{param_name}: native rules param does not exist"));
        }
        self.same_if_present(label, message, "value", rules_param_float(value)?)?;
        self.same_bool_if_present(label, message, "listed", names.iter().any(|name| name == param_name))
    }
    pub(crate) fn set_rules_param(&mut self, message: &Value) -> Result<(), String> {
        let param_name = str_field(message, "paramName")?;
        let value = rules_param_float_value(f32_field(message, "value")?);
        let los = 32;
        let rules = self.interface.rules_params();
        match str_field(message, "scope")? {
            "game" => rules
                .set_game_rules_param(param_name, value, los)
                .map_err(|err| format!("set_game_rules_param({param_name}) failed: {err:?}"))?,
            "team" => {
                let team_id = i32_field(message, "teamID")?;
                rules
                    .set_team_rules_param(team_id, param_name, value, los)
                    .map_err(|err| format!("set_team_rules_param({team_id}, {param_name}) failed: {err:?}"))?
            }
            "player" => {
                let player_id = i32_field(message, "playerID")?;
                rules
                    .set_player_rules_param(player_id, param_name, value, los)
                    .map_err(|err| format!("set_player_rules_param({player_id}, {param_name}) failed: {err:?}"))?
            }
            "unit" => {
                let unit_id = i32_field(message, "unitID")?;
                rules
                    .set_unit_rules_param(unit_id, param_name, value, los)
                    .map_err(|err| format!("set_unit_rules_param({unit_id}, {param_name}) failed: {err:?}"))?
            }
            "feature" => {
                let feature_id = i32_field(message, "featureID")?;
                rules
                    .set_feature_rules_param(feature_id, param_name, value, los)
                    .map_err(|err| format!("set_feature_rules_param({feature_id}, {param_name}) failed: {err:?}"))?
            }
            other => return Err(format!("unsupported rules param scope `{other}`")),
        };
        Ok(())
    }
}

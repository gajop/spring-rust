use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_unit_defs_list_count(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        match test_name {
            "get_unit_def_ids" => {
                let native = self.interface.unit_defs().get_unit_def_ids()
                    .map_err(|err| format!("get_unit_def_ids() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "ids", &native)
            }
            "get_unit_def_count" => {
                let native = self.interface.unit_defs().get_unit_def_count()
                    .map_err(|err| format!("get_unit_def_count() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native as i32)
            }
            _ => Err(format!("unsupported unit defs list/count check `{label}`")),
        }
    }

    pub(crate) fn check_unit_defs_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_def_id = i32_field(message, "unitDefID")?;
        let native = self.interface.unit_defs().valid_unit_def_id(unit_def_id)
            .map_err(|err| format!("valid_unit_def_id({unit_def_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "valid", native)
    }

    pub(crate) fn check_unit_defs_i32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let name = str_field(message, "defName")?;
        let native = self.interface.unit_defs().get_unit_def_idby_name(name)
            .map_err(|err| format!("get_unit_def_idby_name({name}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "unitDefID", native)
    }

    pub(crate) fn check_unit_defs_string(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_def_id = i32_field(message, "unitDefID")?;
        let native = match test_name {
            "get_unit_def_name" => self.interface.unit_defs().get_unit_def_name(unit_def_id)
                .map_err(|err| format!("get_unit_def_name({unit_def_id}) failed: {err:?}"))?,
            "get_unit_def_human_name" => self.interface.unit_defs().get_unit_def_human_name(unit_def_id)
                .map_err(|err| format!("get_unit_def_human_name({unit_def_id}) failed: {err:?}"))?,
            "get_unit_def_custom_param" => {
                let key = str_field(message, "key")?;
                self.interface.unit_defs().get_unit_def_custom_param(unit_def_id, key)
                    .map_err(|err| format!("get_unit_def_custom_param({unit_def_id}, {key}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported unit defs string check `{label}`")),
        };
        self.same_string_if_present(label, message, "value", native.as_deref().unwrap_or(""))
    }

    pub(crate) fn check_unit_defs_f32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let unit_def_id = i32_field(message, "unitDefID")?;
        let native = match test_name {
            "get_unit_def_health" => self.interface.unit_defs().get_unit_def_health(unit_def_id)
                .map_err(|err| format!("get_unit_def_health({unit_def_id}) failed: {err:?}"))?,
            "get_unit_def_speed" => self.interface.unit_defs().get_unit_def_speed(unit_def_id)
                .map_err(|err| format!("get_unit_def_speed({unit_def_id}) failed: {err:?}"))?,
            "get_unit_def_metal_cost" | "get_unit_def_energy_cost" | "get_unit_def_build_time" => {
                let costs = self.interface.unit_defs().get_unit_def_costs(unit_def_id)
                    .map_err(|err| format!("get_unit_def_costs({unit_def_id}) failed: {err:?}"))?;
                match test_name {
                    "get_unit_def_metal_cost" => costs.metalCost,
                    "get_unit_def_energy_cost" => costs.energyCost,
                    "get_unit_def_build_time" => costs.buildTime,
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("unsupported unit defs f32 check `{label}`")),
        };
        self.same_if_present(label, message, "value", native)
    }

    pub(crate) fn check_unit_defs_keys(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_def_id = i32_field(message, "unitDefID")?;
        let native = self.interface.unit_defs().get_unit_def_custom_param_keys(unit_def_id)
            .map_err(|err| format!("get_unit_def_custom_param_keys({unit_def_id}) failed: {err:?}"))?;
        self.same_string_set_if_present(label, message, "keys", &native)
    }

    pub(crate) fn check_unit_def_by_id(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let unit_def_id = i32_field(message, "unitDefID")?;
        let (exists, basic, costs, _, _, _, _, health) = self.interface.unit_defs().get_unit_def_by_id(unit_def_id)
            .map_err(|err| format!("get_unit_def_by_id({unit_def_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "exists", exists)?;
        self.same_i32_if_present(label, message, "id", basic.id)?;
        self.same_string_if_present(label, message, "nameValue", cstr_or_empty(basic.name)?.as_str())?;
        self.same_string_if_present(label, message, "humanName", cstr_or_empty(basic.humanName)?.as_str())?;
        self.same_if_present(label, message, "metalCost", costs.metalCost)?;
        self.same_if_present(label, message, "health", health.health)
    }

    pub(crate) fn check_feature_defs_list_count(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        match test_name {
            "get_feature_def_ids" => {
                let native = self.interface.feature_defs().get_feature_def_ids()
                    .map_err(|err| format!("get_feature_def_ids() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "ids", &native)
            }
            "get_feature_def_count" => {
                let native = self.interface.feature_defs().get_feature_def_count()
                    .map_err(|err| format!("get_feature_def_count() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native as i32)
            }
            _ => Err(format!("unsupported feature defs list/count check `{label}`")),
        }
    }

    pub(crate) fn check_feature_defs_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_def_id = i32_field(message, "featureDefID")?;
        let native = self.interface.feature_defs().valid_feature_def_id(feature_def_id)
            .map_err(|err| format!("valid_feature_def_id({feature_def_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "valid", native)
    }

    pub(crate) fn check_feature_defs_i32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let name = str_field(message, "defName")?;
        let native = self.interface.feature_defs().get_feature_def_idby_name(name)
            .map_err(|err| format!("get_feature_def_idby_name({name}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "featureDefID", native)
    }

    pub(crate) fn check_feature_defs_string(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_def_id = i32_field(message, "featureDefID")?;
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_feature_def_name" => self.interface.feature_defs().get_feature_def_name(feature_def_id)
                .map_err(|err| format!("get_feature_def_name({feature_def_id}) failed: {err:?}"))?,
            "get_feature_def_custom_param" => {
                let key = str_field(message, "key")?;
                self.interface.feature_defs().get_feature_def_custom_param(feature_def_id, key)
                    .map_err(|err| format!("get_feature_def_custom_param({feature_def_id}, {key}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported feature defs string check `{label}`")),
        };
        self.same_string_if_present(label, message, "value", native.as_deref().unwrap_or(""))
    }

    pub(crate) fn check_feature_defs_f32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_def_id = i32_field(message, "featureDefID")?;
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_feature_def_metal" => self.interface.feature_defs().get_feature_def_metal(feature_def_id)
                .map_err(|err| format!("get_feature_def_metal({feature_def_id}) failed: {err:?}"))?,
            "get_feature_def_energy" => self.interface.feature_defs().get_feature_def_energy(feature_def_id)
                .map_err(|err| format!("get_feature_def_energy({feature_def_id}) failed: {err:?}"))?,
            _ => return Err(format!("unsupported feature defs f32 check `{label}`")),
        };
        self.same_if_present(label, message, "value", native)
    }

    pub(crate) fn check_feature_defs_keys(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_def_id = i32_field(message, "featureDefID")?;
        let native = self.interface.feature_defs().get_feature_def_custom_param_keys(feature_def_id)
            .map_err(|err| format!("get_feature_def_custom_param_keys({feature_def_id}) failed: {err:?}"))?;
        self.same_string_set_if_present(label, message, "keys", &native)
    }

    pub(crate) fn check_feature_def_by_id(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_def_id = i32_field(message, "featureDefID")?;
        let (info, exists) = self.interface.feature_defs().get_feature_def_by_id(feature_def_id)
            .map_err(|err| format!("get_feature_def_by_id({feature_def_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "exists", exists)?;
        self.same_i32_if_present(label, message, "id", info.id)?;
        self.same_string_if_present(label, message, "nameValue", cstr_or_empty(info.name)?.as_str())?;
        self.same_if_present(label, message, "metal", info.metal)?;
        self.same_if_present(label, message, "energy", info.energy)
    }

    pub(crate) fn check_weapon_defs_list_count(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        match test_name {
            "get_weapon_def_ids" => {
                let native = self.interface.weapon_defs().get_weapon_def_ids()
                    .map_err(|err| format!("get_weapon_def_ids() failed: {err:?}"))?;
                self.same_i32_set_if_present(label, message, "ids", &native)
            }
            "get_weapon_def_count" => {
                let native = self.interface.weapon_defs().get_weapon_def_count()
                    .map_err(|err| format!("get_weapon_def_count() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native as i32)
            }
            _ => Err(format!("unsupported weapon defs list/count check `{label}`")),
        }
    }

    pub(crate) fn check_weapon_defs_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let weapon_def_id = i32_field(message, "weaponDefID")?;
        let native = self.interface.weapon_defs().valid_weapon_def_id(weapon_def_id)
            .map_err(|err| format!("valid_weapon_def_id({weapon_def_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "valid", native)
    }

    pub(crate) fn check_weapon_defs_i32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let name = str_field(message, "defName")?;
        let native = self.interface.weapon_defs().get_weapon_def_id(name)
            .map_err(|err| format!("get_weapon_def_id({name}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "weaponDefID", native)
    }

    pub(crate) fn check_weapon_defs_string(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let weapon_def_id = i32_field(message, "weaponDefID")?;
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_weapon_def_name" => self.interface.weapon_defs().get_weapon_def_name(weapon_def_id)
                .map_err(|err| format!("get_weapon_def_name({weapon_def_id}) failed: {err:?}"))?,
            "get_weapon_def_custom_param" => {
                let key = str_field(message, "key")?;
                self.interface.weapon_defs().get_weapon_def_custom_param(weapon_def_id, key)
                    .map_err(|err| format!("get_weapon_def_custom_param({weapon_def_id}, {key}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported weapon defs string check `{label}`")),
        };
        self.same_string_if_present(label, message, "value", native.as_deref().unwrap_or(""))
    }

    pub(crate) fn check_weapon_defs_f32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let weapon_def_id = i32_field(message, "weaponDefID")?;
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_weapon_def_range" => self.interface.weapon_defs().get_weapon_def_range(weapon_def_id)
                .map_err(|err| format!("get_weapon_def_range({weapon_def_id}) failed: {err:?}"))?,
            "get_weapon_def_damage" => self.interface.weapon_defs().get_weapon_def_damage(weapon_def_id)
                .map_err(|err| format!("get_weapon_def_damage({weapon_def_id}) failed: {err:?}"))?,
            _ => return Err(format!("unsupported weapon defs f32 check `{label}`")),
        };
        self.same_if_present(label, message, "value", native)
    }

    pub(crate) fn check_weapon_defs_keys(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let weapon_def_id = i32_field(message, "weaponDefID")?;
        let native = self.interface.weapon_defs().get_weapon_def_custom_param_keys(weapon_def_id)
            .map_err(|err| format!("get_weapon_def_custom_param_keys({weapon_def_id}) failed: {err:?}"))?;
        self.same_string_set_if_present(label, message, "keys", &native)
    }

    pub(crate) fn check_weapon_def_by_id(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let weapon_def_id = i32_field(message, "weaponDefID")?;
        let (info, exists) = self.interface.weapon_defs().get_weapon_def_by_id(weapon_def_id)
            .map_err(|err| format!("get_weapon_def_by_id({weapon_def_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "exists", exists)?;
        self.same_i32_if_present(label, message, "id", info.id)?;
        self.same_string_if_present(label, message, "nameValue", cstr_or_empty(info.name)?.as_str())?;
        self.same_if_present(label, message, "range", info.range)?;
        self.same_if_present(label, message, "damage", info.damage)
    }
}

use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_game_frame(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let (low16, high16) = self
            .interface
            .game()
            .get_game_frame()
            .map_err(|err| format!("get_game_frame() failed: {err:?}"))?;
        let native = ((high16 as u32) << 16) | low16 as u32;
        self.same_i32_if_present(label, message, "frame", native as i32)
    }
    pub(crate) fn check_game_seconds(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let native = self
            .interface
            .game()
            .get_game_seconds()
            .map_err(|err| format!("get_game_seconds() failed: {err:?}"))?;
        self.same_if_present(label, message, "seconds", native)
    }
    pub(crate) fn check_gaia_team_id(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let native = self
            .interface
            .game()
            .get_gaia_team_id()
            .map_err(|err| format!("get_gaia_team_id() failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "teamID", native)
    }
    pub(crate) fn check_heading_from_facing(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let facing = i32_field(message, "facing")?;
        let native = self
            .interface
            .game()
            .get_heading_from_facing(facing)
            .map_err(|err| format!("get_heading_from_facing({facing}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "heading", native)
    }
    pub(crate) fn check_facing_from_heading(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let heading = i32_field(message, "heading")?;
        let native = self
            .interface
            .game()
            .get_facing_from_heading(heading)
            .map_err(|err| format!("get_facing_from_heading({heading}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "facing", native)
    }
    pub(crate) fn check_heading_from_vector(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let x = f32_field(message, "x")?;
        let z = f32_field(message, "z")?;
        let native = self
            .interface
            .game()
            .get_heading_from_vector(x, z)
            .map_err(|err| format!("get_heading_from_vector({x}, {z}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "heading", native)
    }
    pub(crate) fn check_game_bool(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let native = match test_name {
            "is_cheating_enabled" => self
                .interface
                .game()
                .is_cheating_enabled()
                .map_err(|err| format!("is_cheating_enabled() failed: {err:?}"))?,
            "are_helper_ais_enabled" => self
                .interface
                .game()
                .are_helper_ais_enabled()
                .map_err(|err| format!("are_helper_ais_enabled() failed: {err:?}"))?,
            "fixed_allies" => self
                .interface
                .game()
                .fixed_allies()
                .map_err(|err| format!("fixed_allies() failed: {err:?}"))?,
            "is_game_over" => self
                .interface
                .game()
                .is_game_over()
                .map_err(|err| format!("is_game_over() failed: {err:?}"))?,
            "is_god_mode_enabled" => self
                .interface
                .game()
                .is_god_mode_enabled()
                .map_err(|err| format!("is_god_mode_enabled() failed: {err:?}"))?,
            "is_dev_lua_enabled" => self
                .interface
                .game()
                .is_dev_lua_enabled()
                .map_err(|err| format!("is_dev_lua_enabled() failed: {err:?}"))?,
            "is_edit_defs_enabled" => self
                .interface
                .game()
                .is_edit_defs_enabled()
                .map_err(|err| format!("is_edit_defs_enabled() failed: {err:?}"))?,
            "is_no_cost_enabled" => self
                .interface
                .game()
                .is_no_cost_enabled()
                .map_err(|err| format!("is_no_cost_enabled() failed: {err:?}"))?,
            _ => return Err(format!("unsupported game bool check `{label}`")),
        };
        let field = if test_name == "is_game_over" {
            "isGameOver"
        } else {
            "enabled"
        };
        self.same_bool_if_present(label, message, field, native)
    }
    pub(crate) fn check_game_option(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let key = str_field(message, "key")?;
        let test_name = base_test_name(label);
        let (native, exists) = match test_name {
            "get_mod_option" => self
                .interface
                .game()
                .get_mod_option(key)
                .map_err(|err| format!("get_mod_option({key}) failed: {err:?}"))?,
            "get_map_option" => self
                .interface
                .game()
                .get_map_option(key)
                .map_err(|err| format!("get_map_option({key}) failed: {err:?}"))?,
            _ => return Err(format!("unsupported game option check `{label}`")),
        };
        self.same_bool_if_present(label, message, "exists", exists)?;
        self.same_string_if_present(label, message, "value", native.as_deref().unwrap_or(""))
    }
    pub(crate) fn check_game_options(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_mod_options" => self
                .interface
                .game()
                .get_mod_options()
                .map_err(|err| format!("get_mod_options() failed: {err:?}"))?,
            "get_map_options" => self
                .interface
                .game()
                .get_map_options()
                .map_err(|err| format!("get_map_options() failed: {err:?}"))?,
            _ => return Err(format!("unsupported game options check `{label}`")),
        };
        self.same_string_set_if_present(label, message, "keys", &native)
    }
    pub(crate) fn check_game_tidal(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let native = self
            .interface
            .game()
            .get_tidal()
            .map_err(|err| format!("get_tidal() failed: {err:?}"))?;
        self.same_if_present(label, message, "tidalStrength", native)
    }
    pub(crate) fn check_game_wind(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let native = self
            .interface
            .game()
            .get_wind()
            .map_err(|err| format!("get_wind() failed: {err:?}"))?;
        self.same_if_present(label, message, "windStrength", native.current)
    }
    pub(crate) fn check_game_rules_info(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let rules = self
            .interface
            .game()
            .get_game_rules_info()
            .map_err(|err| format!("get_game_rules_info() failed: {err:?}"))?;
        let resources = self
            .interface
            .game()
            .get_game_rules_resource_info()
            .map_err(|err| format!("get_game_rules_resource_info() failed: {err:?}"))?;

        self.same_i32_if_present(label, message, "maxUnits", rules.maxUnits)?;
        self.same_bool_if_present(label, message, "constructionDecay", rules.constructionDecay)?;
        self.same_i32_if_present(
            label,
            message,
            "constructionDecayTime",
            rules.constructionDecayTime,
        )?;
        self.same_if_present(
            label,
            message,
            "constructionDecaySpeed",
            rules.constructionDecaySpeed,
        )?;
        self.same_i32_if_present(label, message, "multiReclaim", rules.multiReclaim)?;
        self.same_i32_if_present(label, message, "reclaimMethod", rules.reclaimMethod)?;
        self.same_i32_if_present(label, message, "reclaimUnitMethod", rules.reclaimUnitMethod)?;
        self.same_if_present(
            label,
            message,
            "reclaimUnitEnergyCostFactor",
            rules.reclaimUnitEnergyCostFactor,
        )?;
        self.same_if_present(
            label,
            message,
            "reclaimUnitEnergyCostFactor",
            resources.reclaimUnitCostFactor.energy,
        )?;
        self.same_if_present(
            label,
            message,
            "reclaimUnitEfficiency",
            rules.reclaimUnitEfficiency,
        )?;
        self.same_if_present(
            label,
            message,
            "reclaimUnitEfficiency",
            resources.reclaimUnitEfficiency.metal,
        )?;
        self.same_if_present(
            label,
            message,
            "reclaimFeatureEnergyCostFactor",
            rules.reclaimFeatureEnergyCostFactor,
        )?;
        self.same_if_present(
            label,
            message,
            "reclaimFeatureEnergyCostFactor",
            resources.reclaimFeatureCostFactor.energy,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "reclaimUnitDrainHealth",
            rules.reclaimUnitDrainHealth,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "reclaimAllowEnemies",
            rules.reclaimAllowEnemies,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "reclaimAllowAllies",
            rules.reclaimAllowAllies,
        )?;
        self.same_if_present(
            label,
            message,
            "repairEnergyCostFactor",
            rules.repairEnergyCostFactor,
        )?;
        self.same_if_present(
            label,
            message,
            "repairEnergyCostFactor",
            resources.repairCostFactor.energy,
        )?;
        self.same_if_present(
            label,
            message,
            "resurrectEnergyCostFactor",
            rules.resurrectEnergyCostFactor,
        )?;
        self.same_if_present(
            label,
            message,
            "resurrectEnergyCostFactor",
            resources.resurrectCostFactor.energy,
        )?;
        self.same_if_present(
            label,
            message,
            "captureEnergyCostFactor",
            rules.captureEnergyCostFactor,
        )?;
        self.same_if_present(
            label,
            message,
            "captureEnergyCostFactor",
            resources.captureCostFactor.energy,
        )?;
        self.same_i32_if_present(label, message, "transportAir", rules.transportAir)?;
        self.same_i32_if_present(label, message, "transportShip", rules.transportShip)?;
        self.same_i32_if_present(label, message, "transportHover", rules.transportHover)?;
        self.same_i32_if_present(label, message, "transportGround", rules.transportGround)?;
        self.same_i32_if_present(label, message, "fireAtKilled", rules.fireAtKilled)?;
        self.same_i32_if_present(label, message, "fireAtCrashing", rules.fireAtCrashing)?;
        self.same_i32_if_present(
            label,
            message,
            "requireSonarUnderWater",
            rules.requireSonarUnderWater,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "paralyzeOnMaxHealth",
            rules.paralyzeOnMaxHealth,
        )?;
        self.same_if_present(
            label,
            message,
            "paralyzeDeclineRate",
            rules.paralyzeDeclineRate,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "allowEnginePlayerlist",
            rules.allowEnginePlayerlist,
        )?;
        self.same_bool_if_present(
            label,
            message,
            "nativeExcessSharing",
            rules.nativeExcessSharing,
        )
    }
    pub(crate) fn check_side_data(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let side_name = str_field(message, "sideName")?;
        let native = self
            .interface
            .game()
            .get_side_data(side_name)
            .map_err(|err| format!("get_side_data({side_name}) failed: {err:?}"))?;
        self.same_side_data(label, message, native)
    }
    pub(crate) fn check_side_data_by_index(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let side_index = i32_field(message, "sideIndex")?;
        let native_index = side_index
            .checked_sub(1)
            .ok_or_else(|| format!("{label}.sideIndex must be one-based"))?
            as u32;
        let native = self
            .interface
            .game()
            .get_side_data_by_index(native_index)
            .map_err(|err| format!("get_side_data_by_index({native_index}) failed: {err:?}"))?;
        self.same_side_data(label, message, native)
    }
    pub(crate) fn check_side_data_count(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let native = self
            .interface
            .game()
            .get_side_data_count()
            .map_err(|err| format!("get_side_data_count() failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "sideCount", native as i32)
    }
    fn same_side_data(
        &mut self,
        label: &str,
        message: &Value,
        native: sys::SideData,
    ) -> Result<(), String> {
        let case_name = unsafe {
            native
                .caseName
                .as_ref()
                .map(|ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
                .unwrap_or_default()
        };
        let start_unit = unsafe {
            native
                .startUnit
                .as_ref()
                .map(|ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
                .unwrap_or_default()
        };
        let normalized_side_name = unsafe {
            native
                .sideName
                .as_ref()
                .map(|ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
                .unwrap_or_default()
        };
        self.same_string_if_present(label, message, "caseName", &case_name)?;
        self.same_string_if_present(label, message, "startUnit", &start_unit)?;
        self.same_string_if_present(label, message, "normalizedSideName", &normalized_side_name)
    }
    pub(crate) fn check_game_global_los(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let native = self
            .interface
            .game()
            .get_global_los(ally_team_id)
            .map_err(|err| format!("get_global_los({ally_team_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "enabled", native != 0)
    }
    pub(crate) fn check_game_vector_from_heading(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let heading = i32_field(message, "heading")?;
        let native = self
            .interface
            .game()
            .get_vector_from_heading(heading)
            .map_err(|err| format!("get_vector_from_heading({heading}) failed: {err:?}"))?;
        self.same(&format!("{label}.x"), native.x, f32_field(message, "x")?)?;
        self.same(&format!("{label}.z"), native.y, f32_field(message, "z")?)
    }
    pub(crate) fn check_game_ally_team_start_box(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let (native, exists) = self
            .interface
            .game()
            .get_ally_team_start_box(ally_team_id)
            .map_err(|err| format!("get_ally_team_start_box({ally_team_id}) failed: {err:?}"))?;
        if !exists {
            return Ok(());
        }
        self.same_if_present(label, message, "xMin", native.minX)?;
        self.same_if_present(label, message, "zMin", native.minZ)?;
        self.same_if_present(label, message, "xMax", native.maxX)?;
        self.same_if_present(label, message, "zMax", native.maxZ)
    }
    pub(crate) fn check_game_team_start_position(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let team_id = i32_field(message, "teamID")?;
        let native = self
            .interface
            .game()
            .get_team_start_position(team_id)
            .map_err(|err| format!("get_team_start_position({team_id}) failed: {err:?}"))?;
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_map_start_positions(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let native = self
            .interface
            .game()
            .get_map_start_positions()
            .map_err(|err| format!("get_map_start_positions() failed: {err:?}"))?;
        self.same_start_positions_if_present(label, message, "positions", &native)
    }
    pub(crate) fn set_global_los(&mut self, message: &Value) -> Result<(), String> {
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let enabled = bool_field(message, "enabled")?;
        self.interface
            .synced_ctrl()
            .team()
            .set_global_los(ally_team_id, enabled)
            .map_err(|err| format!("set_global_los({ally_team_id}, {enabled}) failed: {err:?}"))?;
        Ok(())
    }
}

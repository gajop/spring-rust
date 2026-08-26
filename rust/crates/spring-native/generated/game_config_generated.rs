#[derive(Debug, Clone, Copy, Default)]
pub struct SetGodModeOptions {
    pub control_allies: bool,
    pub control_enemies: bool,
}

impl From<SetGodModeOptions> for sys::SetGodModeOptions {
    fn from(options: SetGodModeOptions) -> Self {
        sys::SetGodModeOptions {
            controlAllies: options.control_allies,
            controlEnemies: options.control_enemies,
        }
    }
}

impl<'a> GameConfig<'a> {
    pub fn set_no_pause(&self, no_pause: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetNoPauseQuery {
                noPause: no_pause,
            };
            let mut result = MaybeUninit::<sys::SetNoPauseResult>::zeroed();
            let func = self.api.SetNoPause.expect("SetNoPause function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_cheating_enabled(&self, enabled: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetCheatingEnabledQuery {
                enabled,
            };
            let mut result = MaybeUninit::<sys::SetCheatingEnabledResult>::zeroed();
            let func = self.api.SetCheatingEnabled.expect("SetCheatingEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_god_mode(&self, options: SetGodModeOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGodModeQuery {
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetGodModeResult>::zeroed();
            let func = self.api.SetGodMode.expect("SetGodMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_experience_grade(&self, exp_grade: f32, exp_power_scale: f32, exp_health_scale: f32, exp_reload_scale: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetExperienceGradeQuery {
                expGrade: exp_grade,
                expPowerScale: exp_power_scale,
                expHealthScale: exp_health_scale,
                expReloadScale: exp_reload_scale,
            };
            let mut result = MaybeUninit::<sys::SetExperienceGradeResult>::zeroed();
            let func = self.api.SetExperienceGrade.expect("SetExperienceGrade function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_radar_error_params(&self, ally_team_id: i32, ally_team_error_size: f32, base_error_size: f32, base_error_mult: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetRadarErrorParamsQuery {
                allyTeamID: ally_team_id,
                allyTeamErrorSize: ally_team_error_size,
                baseErrorSize: base_error_size,
                baseErrorMult: base_error_mult,
            };
            let mut result = MaybeUninit::<sys::SetRadarErrorParamsResult>::zeroed();
            let func = self.api.SetRadarErrorParams.expect("SetRadarErrorParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_square_building_mask(&self, x: i32, z: i32, mask: u16) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetSquareBuildingMaskQuery {
                x,
                z,
                mask,
            };
            let mut result = MaybeUninit::<sys::SetSquareBuildingMaskResult>::zeroed();
            let func = self.api.SetSquareBuildingMask.expect("SetSquareBuildingMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

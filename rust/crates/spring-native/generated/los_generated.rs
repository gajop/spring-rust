impl<'a> Los<'a> {
    pub fn get_position_los_state(&self, pos: sys::Float3, ally_team_id: i32) -> Result<sys::PositionLosState, Error> {
        unsafe {
            let query = sys::GetPositionLosStateQuery {
                pos,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetPositionLosStateResult>::zeroed();
            let func = self.api.GetPositionLosState.expect("GetPositionLosState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.state
            })
        }
    }

    pub fn is_pos_in_los(&self, pos: sys::Float3, ally_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsPosInLosQuery {
                pos,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::IsPosInLosResult>::zeroed();
            let func = self.api.IsPosInLos.expect("IsPosInLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inLos
            })
        }
    }

    pub fn is_pos_in_radar(&self, pos: sys::Float3, ally_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsPosInRadarQuery {
                pos,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::IsPosInRadarResult>::zeroed();
            let func = self.api.IsPosInRadar.expect("IsPosInRadar function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inRadar
            })
        }
    }

    pub fn is_pos_in_air_los(&self, pos: sys::Float3, ally_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsPosInAirLosQuery {
                pos,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::IsPosInAirLosResult>::zeroed();
            let func = self.api.IsPosInAirLos.expect("IsPosInAirLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inAirLos
            })
        }
    }

    pub fn is_unit_in_los(&self, unit_id: i32, ally_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitInLosQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitInLosResult>::zeroed();
            let func = self.api.IsUnitInLos.expect("IsUnitInLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inLos
            })
        }
    }

    pub fn is_unit_in_air_los(&self, unit_id: i32, ally_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitInAirLosQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitInAirLosResult>::zeroed();
            let func = self.api.IsUnitInAirLos.expect("IsUnitInAirLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inAirLos
            })
        }
    }

    pub fn is_unit_in_radar(&self, unit_id: i32, ally_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitInRadarQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitInRadarResult>::zeroed();
            let func = self.api.IsUnitInRadar.expect("IsUnitInRadar function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inRadar
            })
        }
    }

    pub fn is_unit_in_jammer(&self, unit_id: i32, ally_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitInJammerQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitInJammerResult>::zeroed();
            let func = self.api.IsUnitInJammer.expect("IsUnitInJammer function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inJammer
            })
        }
    }

    pub fn get_radar_error_params(&self, ally_team_id: i32) -> Result<sys::RadarErrorParams, Error> {
        unsafe {
            let query = sys::GetRadarErrorParamsQuery {
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetRadarErrorParamsResult>::zeroed();
            let func = self.api.GetRadarErrorParams.expect("GetRadarErrorParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.params
            })
        }
    }

    pub fn get_closest_valid_position(&self, unit_def_id: i32, x: f32, z: f32, radius: f32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetClosestValidPositionQuery {
                unitDefID: unit_def_id,
                x,
                z,
                radius,
            };
            let mut result = MaybeUninit::<sys::GetClosestValidPositionResult>::zeroed();
            let func = self.api.GetClosestValidPosition.expect("GetClosestValidPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

}

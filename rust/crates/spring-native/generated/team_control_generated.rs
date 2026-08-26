impl<'a> TeamControl<'a> {
    pub fn set_ally(&self, first_ally_team_id: i32, second_ally_team_id: i32, allied: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetAllyQuery {
                firstAllyTeamID: first_ally_team_id,
                secondAllyTeamID: second_ally_team_id,
                allied,
            };
            let mut result = MaybeUninit::<sys::SetAllyResult>::zeroed();
            let func = self.api.SetAlly.expect("SetAlly function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ally_team_start_box(&self, ally_team_id: i32, min_x: f32, min_z: f32, max_x: f32, max_z: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetAllyTeamStartBoxQuery {
                allyTeamID: ally_team_id,
                minX: min_x,
                minZ: min_z,
                maxX: max_x,
                maxZ: max_z,
            };
            let mut result = MaybeUninit::<sys::SetAllyTeamStartBoxResult>::zeroed();
            let func = self.api.SetAllyTeamStartBox.expect("SetAllyTeamStartBox function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn kill_team(&self, team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::KillTeamQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::KillTeamResult>::zeroed();
            let func = self.api.KillTeam.expect("KillTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn assign_player_to_team(&self, player_id: i32, team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AssignPlayerToTeamQuery {
                playerID: player_id,
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::AssignPlayerToTeamResult>::zeroed();
            let func = self.api.AssignPlayerToTeam.expect("AssignPlayerToTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn game_over(&self, winning_ally_teams: &[i32]) -> Result<bool, Error> {
        unsafe {
            let query = sys::GameOverQuery {
                winningAllyTeams: winning_ally_teams.as_ptr(),
                count: winning_ally_teams.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GameOverResult>::zeroed();
            let func = self.api.GameOver.expect("GameOver function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_global_los(&self, ally_team_id: i32, enabled: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGlobalLosQuery {
                allyTeamID: ally_team_id,
                enabled,
            };
            let mut result = MaybeUninit::<sys::SetGlobalLosResult>::zeroed();
            let func = self.api.SetGlobalLos.expect("SetGlobalLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_team_resource(&self, team_id: i32, resource_type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::AddTeamResourceQuery {
                teamID: team_id,
                resourceType: resource_type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::AddTeamResourceResult>::zeroed();
            let func = self.api.AddTeamResource.expect("AddTeamResource function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_team_resource_excess_stats(&self, team_id: i32, resource_type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::AddTeamResourceExcessStatsQuery {
                teamID: team_id,
                resourceType: resource_type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::AddTeamResourceExcessStatsResult>::zeroed();
            let func = self.api.AddTeamResourceExcessStats.expect("AddTeamResourceExcessStats function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn use_team_resource(&self, team_id: i32, resource_type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::UseTeamResourceQuery {
                teamID: team_id,
                resourceType: resource_type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::UseTeamResourceResult>::zeroed();
            let func = self.api.UseTeamResource.expect("UseTeamResource function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_team_resource(&self, team_id: i32, resource_type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::SetTeamResourceQuery {
                teamID: team_id,
                resourceType: resource_type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::SetTeamResourceResult>::zeroed();
            let func = self.api.SetTeamResource.expect("SetTeamResource function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_team_share_level(&self, team_id: i32, resource_type: &str, share_level: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::SetTeamShareLevelQuery {
                teamID: team_id,
                resourceType: resource_type_cstr.as_ptr(),
                shareLevel: share_level,
            };
            let mut result = MaybeUninit::<sys::SetTeamShareLevelResult>::zeroed();
            let func = self.api.SetTeamShareLevel.expect("SetTeamShareLevel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn share_team_resource(&self, team_id: i32, target_team_id: i32, resource_type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::ShareTeamResourceQuery {
                teamID: team_id,
                targetTeamID: target_team_id,
                resourceType: resource_type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::ShareTeamResourceResult>::zeroed();
            let func = self.api.ShareTeamResource.expect("ShareTeamResource function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_team_start_position(&self, team_id: i32, pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetTeamStartPositionQuery {
                teamID: team_id,
                pos,
            };
            let mut result = MaybeUninit::<sys::SetTeamStartPositionResult>::zeroed();
            let func = self.api.SetTeamStartPosition.expect("SetTeamStartPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_player_ready_state(&self, player_id: i32, ready: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetPlayerReadyStateQuery {
                playerID: player_id,
                ready,
            };
            let mut result = MaybeUninit::<sys::SetPlayerReadyStateResult>::zeroed();
            let func = self.api.SetPlayerReadyState.expect("SetPlayerReadyState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn transfer_team_max_units(&self, from_team_id: i32, to_team_id: i32, amount: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::TransferTeamMaxUnitsQuery {
                fromTeamID: from_team_id,
                toTeamID: to_team_id,
                amount,
            };
            let mut result = MaybeUninit::<sys::TransferTeamMaxUnitsResult>::zeroed();
            let func = self.api.TransferTeamMaxUnits.expect("TransferTeamMaxUnits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

impl<'a> Teams<'a> {
    pub fn get_team_list(&self, ally_team_id: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetTeamListQuery {
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamListResult>::zeroed();
            let func = self.api.GetTeamList.expect("GetTeamList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.teams.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.teams as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_ally_team_list(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetAllyTeamListQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetAllyTeamListResult>::zeroed();
            let func = self.api.GetAllyTeamList.expect("GetAllyTeamList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.allyTeams.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.allyTeams as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_team_info(&self, team_id: i32, get_team_keys: bool) -> Result<sys::TeamInfo, Error> {
        unsafe {
            let query = sys::GetTeamInfoQuery {
                teamID: team_id,
                getTeamKeys: get_team_keys,
            };
            let mut result = MaybeUninit::<sys::GetTeamInfoResult>::zeroed();
            let func = self.api.GetTeamInfo.expect("GetTeamInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn get_team_ally_team_id(&self, team_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetTeamAllyTeamIDQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamAllyTeamIDResult>::zeroed();
            let func = self.api.GetTeamAllyTeamID.expect("GetTeamAllyTeamID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allyTeamID
            })
        }
    }

    pub fn get_team_max_units(&self, team_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetTeamMaxUnitsQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamMaxUnitsResult>::zeroed();
            let func = self.api.GetTeamMaxUnits.expect("GetTeamMaxUnits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.maxUnits
            })
        }
    }

    pub fn get_team_lua_ai(&self, team_id: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetTeamLuaAIQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamLuaAIResult>::zeroed();
            let func = self.api.GetTeamLuaAI.expect("GetTeamLuaAI function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.luaAI.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.luaAI).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_team_resources(&self, team_id: i32, resource: &str) -> Result<sys::TeamResources, Error> {
        unsafe {
            let resource_cstr = std::ffi::CString::new(resource).map_err(|_| Error::invalid_argument("resource"))?;
            let query = sys::GetTeamResourcesQuery {
                teamID: team_id,
                resource: resource_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetTeamResourcesResult>::zeroed();
            let func = self.api.GetTeamResources.expect("GetTeamResources function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.resources
            })
        }
    }

    pub fn get_team_unit_stats(&self, team_id: i32) -> Result<sys::TeamUnitStats, Error> {
        unsafe {
            let query = sys::GetTeamUnitStatsQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamUnitStatsResult>::zeroed();
            let func = self.api.GetTeamUnitStats.expect("GetTeamUnitStats function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.stats
            })
        }
    }

    pub fn get_team_resource_stats(&self, team_id: i32, resource: &str) -> Result<sys::TeamResources, Error> {
        unsafe {
            let resource_cstr = std::ffi::CString::new(resource).map_err(|_| Error::invalid_argument("resource"))?;
            let query = sys::GetTeamResourceStatsQuery {
                teamID: team_id,
                resource: resource_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetTeamResourceStatsResult>::zeroed();
            let func = self.api.GetTeamResourceStats.expect("GetTeamResourceStats function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.resources
            })
        }
    }

    pub fn get_team_stats_history(&self, team_id: i32, start_index: i32, end_index: i32) -> Result<Vec<sys::TeamStatsHistoryPoint>, Error> {
        unsafe {
            let query = sys::GetTeamStatsHistoryQuery {
                teamID: team_id,
                startIndex: start_index,
                endIndex: end_index,
            };
            let mut result = MaybeUninit::<sys::GetTeamStatsHistoryResult>::zeroed();
            let func = self.api.GetTeamStatsHistory.expect("GetTeamStatsHistory function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.history.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.history as *const sys::TeamStatsHistoryPoint, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_ally_team_info(&self, ally_team_id: i32) -> Result<sys::AllyTeamInfo, Error> {
        unsafe {
            let query = sys::GetAllyTeamInfoQuery {
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetAllyTeamInfoResult>::zeroed();
            let func = self.api.GetAllyTeamInfo.expect("GetAllyTeamInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn are_teams_allied(&self, team_id1: i32, team_id2: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AreTeamsAlliedQuery {
                teamID1: team_id1,
                teamID2: team_id2,
            };
            let mut result = MaybeUninit::<sys::AreTeamsAlliedResult>::zeroed();
            let func = self.api.AreTeamsAllied.expect("AreTeamsAllied function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allied
            })
        }
    }

    pub fn are_players_allied(&self, player_id1: i32, player_id2: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::ArePlayersAlliedQuery {
                playerID1: player_id1,
                playerID2: player_id2,
            };
            let mut result = MaybeUninit::<sys::ArePlayersAlliedResult>::zeroed();
            let func = self.api.ArePlayersAllied.expect("ArePlayersAllied function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allied
            })
        }
    }

    pub fn get_player_list(&self, team_id: i32, active: bool) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetPlayerListQuery {
                teamID: team_id,
                active,
            };
            let mut result = MaybeUninit::<sys::GetPlayerListResult>::zeroed();
            let func = self.api.GetPlayerList.expect("GetPlayerList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.players.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.players as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_player_list_in_team(&self, team_id: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetPlayerListInTeamQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetPlayerListInTeamResult>::zeroed();
            let func = self.api.GetPlayerListInTeam.expect("GetPlayerListInTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.players.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.players as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_player_list_in_ally_team(&self, ally_team_id: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetPlayerListInAllyTeamQuery {
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetPlayerListInAllyTeamResult>::zeroed();
            let func = self.api.GetPlayerListInAllyTeam.expect("GetPlayerListInAllyTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.players.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.players as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_player_info(&self, player_id: i32, get_player_opts: bool) -> Result<sys::PlayerInfo, Error> {
        unsafe {
            let query = sys::GetPlayerInfoQuery {
                playerID: player_id,
                getPlayerOpts: get_player_opts,
            };
            let mut result = MaybeUninit::<sys::GetPlayerInfoResult>::zeroed();
            let func = self.api.GetPlayerInfo.expect("GetPlayerInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn get_player_controlled_unit(&self, player_id: i32) -> Result<(i32, bool), Error> {
        unsafe {
            let query = sys::GetPlayerControlledUnitQuery {
                playerID: player_id,
            };
            let mut result = MaybeUninit::<sys::GetPlayerControlledUnitResult>::zeroed();
            let func = self.api.GetPlayerControlledUnit.expect("GetPlayerControlledUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.unitID,
                result.hasUnit,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_aiinfo(&self, team_id: i32) -> Result<(sys::AIInfo, bool), Error> {
        unsafe {
            let query = sys::GetAIInfoQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetAIInfoResult>::zeroed();
            let func = self.api.GetAIInfo.expect("GetAIInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.info,
                result.isAI,
            );
            Error::result_or(result.error, value)
        }
    }

}

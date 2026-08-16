impl<'a> Player<'a> {
    pub fn get_local_player_id(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetLocalPlayerIDQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLocalPlayerIDResult>::zeroed();
            let func = self.api.GetLocalPlayerID.expect("GetLocalPlayerID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.playerID
            })
        }
    }

    pub fn get_local_team_id(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetLocalTeamIDQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLocalTeamIDResult>::zeroed();
            let func = self.api.GetLocalTeamID.expect("GetLocalTeamID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.teamID
            })
        }
    }

    pub fn get_local_ally_team_id(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetLocalAllyTeamIDQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLocalAllyTeamIDResult>::zeroed();
            let func = self.api.GetLocalAllyTeamID.expect("GetLocalAllyTeamID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allyTeamID
            })
        }
    }

    pub fn get_spectating_state(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetSpectatingStateQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSpectatingStateResult>::zeroed();
            let func = self.api.GetSpectatingState.expect("GetSpectatingState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.spectating
            })
        }
    }

    pub fn get_player_roster(&self, sort_mode: i32, show_pathing_players: bool) -> Result<Vec<sys::RosterEntry>, Error> {
        unsafe {
            let query = sys::GetPlayerRosterQuery {
                sortMode: sort_mode,
                showPathingPlayers: show_pathing_players,
            };
            let mut result = MaybeUninit::<sys::GetPlayerRosterResult>::zeroed();
            let func = self.api.GetPlayerRoster.expect("GetPlayerRoster function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::RosterEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_player_traffic(&self, player_id: i32, packet_id: i32) -> Result<Vec<sys::PlayerTraffic>, Error> {
        unsafe {
            let query = sys::GetPlayerTrafficQuery {
                playerID: player_id,
                packetID: packet_id,
            };
            let mut result = MaybeUninit::<sys::GetPlayerTrafficResult>::zeroed();
            let func = self.api.GetPlayerTraffic.expect("GetPlayerTraffic function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.traffic.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.traffic as *const sys::PlayerTraffic, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_player_statistics(&self, player_id: i32) -> Result<sys::PlayerStats, Error> {
        unsafe {
            let query = sys::GetPlayerStatisticsQuery {
                playerID: player_id,
            };
            let mut result = MaybeUninit::<sys::GetPlayerStatisticsResult>::zeroed();
            let func = self.api.GetPlayerStatistics.expect("GetPlayerStatistics function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.stats
            })
        }
    }

}

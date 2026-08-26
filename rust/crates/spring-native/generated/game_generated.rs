impl<'a> Game<'a> {
    pub fn is_cheating_enabled(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsCheatingEnabledQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsCheatingEnabledResult>::zeroed();
            let func = self.api.IsCheatingEnabled.expect("IsCheatingEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn is_god_mode_enabled(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsGodModeEnabledQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsGodModeEnabledResult>::zeroed();
            let func = self.api.IsGodModeEnabled.expect("IsGodModeEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn is_dev_lua_enabled(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsDevLuaEnabledQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsDevLuaEnabledResult>::zeroed();
            let func = self.api.IsDevLuaEnabled.expect("IsDevLuaEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn is_edit_defs_enabled(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsEditDefsEnabledQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsEditDefsEnabledResult>::zeroed();
            let func = self.api.IsEditDefsEnabled.expect("IsEditDefsEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn is_no_cost_enabled(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsNoCostEnabledQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsNoCostEnabledResult>::zeroed();
            let func = self.api.IsNoCostEnabled.expect("IsNoCostEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn get_global_los(&self, ally_team_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetGlobalLosQuery {
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetGlobalLosResult>::zeroed();
            let func = self.api.GetGlobalLos.expect("GetGlobalLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.los
            })
        }
    }

    pub fn are_helper_ais_enabled(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::AreHelperAIsEnabledQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::AreHelperAIsEnabledResult>::zeroed();
            let func = self.api.AreHelperAIsEnabled.expect("AreHelperAIsEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn fixed_allies(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::FixedAlliesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::FixedAlliesResult>::zeroed();
            let func = self.api.FixedAllies.expect("FixedAllies function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.fixed
            })
        }
    }

    pub fn is_game_over(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsGameOverQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsGameOverResult>::zeroed();
            let func = self.api.IsGameOver.expect("IsGameOver function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.gameOver
            })
        }
    }

    pub fn get_game_frame(&self) -> Result<(u32, u32), Error> {
        unsafe {
            let query = sys::GetGameFrameQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameFrameResult>::zeroed();
            let func = self.api.GetGameFrame.expect("GetGameFrame function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.low16,
                result.high16,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_game_seconds(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetGameSecondsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameSecondsResult>::zeroed();
            let func = self.api.GetGameSeconds.expect("GetGameSeconds function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.seconds
            })
        }
    }

    pub fn get_gaia_team_id(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetGaiaTeamIDQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGaiaTeamIDResult>::zeroed();
            let func = self.api.GetGaiaTeamID.expect("GetGaiaTeamID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.teamID
            })
        }
    }

    pub fn get_game_setup_info(&self) -> Result<sys::GameSetupInfo, Error> {
        unsafe {
            let query = sys::GetGameSetupInfoQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameSetupInfoResult>::zeroed();
            let func = self.api.GetGameSetupInfo.expect("GetGameSetupInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn get_game_map_info(&self) -> Result<sys::GameMapInfo, Error> {
        unsafe {
            let query = sys::GetGameMapInfoQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameMapInfoResult>::zeroed();
            let func = self.api.GetGameMapInfo.expect("GetGameMapInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn get_game_mod_info(&self) -> Result<sys::GameModInfo, Error> {
        unsafe {
            let query = sys::GetGameModInfoQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameModInfoResult>::zeroed();
            let func = self.api.GetGameModInfo.expect("GetGameModInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn get_game_rules_info(&self) -> Result<sys::GameRulesInfo, Error> {
        unsafe {
            let query = sys::GetGameRulesInfoQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameRulesInfoResult>::zeroed();
            let func = self.api.GetGameRulesInfo.expect("GetGameRulesInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn get_map_option(&self, key: &str) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetMapOptionQuery {
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetMapOptionResult>::zeroed();
            let func = self.api.GetMapOption.expect("GetMapOption function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                },
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_map_options(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetMapOptionsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMapOptionsResult>::zeroed();
            let func = self.api.GetMapOptions.expect("GetMapOptions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.keys.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.keys, result.count as usize);
                        slice.iter().map(|&ptr| {
                            if ptr.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(ptr).to_string_lossy().into_owned()
                            }
                        }).collect()
                    }
                }
            })
        }
    }

    pub fn get_mod_option(&self, key: &str) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetModOptionQuery {
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetModOptionResult>::zeroed();
            let func = self.api.GetModOption.expect("GetModOption function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                },
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_mod_options(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetModOptionsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetModOptionsResult>::zeroed();
            let func = self.api.GetModOptions.expect("GetModOptions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.keys.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.keys, result.count as usize);
                        slice.iter().map(|&ptr| {
                            if ptr.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(ptr).to_string_lossy().into_owned()
                            }
                        }).collect()
                    }
                }
            })
        }
    }

    pub fn get_tidal(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetTidalQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetTidalResult>::zeroed();
            let func = self.api.GetTidal.expect("GetTidal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.strength
            })
        }
    }

    pub fn get_wind(&self) -> Result<sys::WindData, Error> {
        unsafe {
            let query = sys::GetWindQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetWindResult>::zeroed();
            let func = self.api.GetWind.expect("GetWind function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.data
            })
        }
    }

    pub fn get_heading_from_vector(&self, x: f32, z: f32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetHeadingFromVectorQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetHeadingFromVectorResult>::zeroed();
            let func = self.api.GetHeadingFromVector.expect("GetHeadingFromVector function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.heading
            })
        }
    }

    pub fn get_vector_from_heading(&self, heading: i32) -> Result<sys::Float2, Error> {
        unsafe {
            let query = sys::GetVectorFromHeadingQuery {
                heading,
            };
            let mut result = MaybeUninit::<sys::GetVectorFromHeadingResult>::zeroed();
            let func = self.api.GetVectorFromHeading.expect("GetVectorFromHeading function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.vector
            })
        }
    }

    pub fn get_facing_from_heading(&self, heading: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetFacingFromHeadingQuery {
                heading,
            };
            let mut result = MaybeUninit::<sys::GetFacingFromHeadingResult>::zeroed();
            let func = self.api.GetFacingFromHeading.expect("GetFacingFromHeading function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.facing
            })
        }
    }

    pub fn get_heading_from_facing(&self, facing: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetHeadingFromFacingQuery {
                facing,
            };
            let mut result = MaybeUninit::<sys::GetHeadingFromFacingResult>::zeroed();
            let func = self.api.GetHeadingFromFacing.expect("GetHeadingFromFacing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.heading
            })
        }
    }

    pub fn get_side_data(&self, side_name: &str) -> Result<sys::SideData, Error> {
        unsafe {
            let side_name_cstr = std::ffi::CString::new(side_name).map_err(|_| Error::invalid_argument("side_name"))?;
            let query = sys::GetSideDataQuery {
                sideName: side_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetSideDataResult>::zeroed();
            let func = self.api.GetSideData.expect("GetSideData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.data
            })
        }
    }

    pub fn get_side_data_by_index(&self, side_index: u32) -> Result<sys::SideData, Error> {
        unsafe {
            let query = sys::GetSideDataByIndexQuery {
                sideIndex: side_index,
            };
            let mut result = MaybeUninit::<sys::GetSideDataByIndexResult>::zeroed();
            let func = self.api.GetSideDataByIndex.expect("GetSideDataByIndex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.data
            })
        }
    }

    pub fn get_side_data_count(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetSideDataCountQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSideDataCountResult>::zeroed();
            let func = self.api.GetSideDataCount.expect("GetSideDataCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_ally_team_start_box(&self, ally_team_id: i32) -> Result<(sys::StartBox, bool), Error> {
        unsafe {
            let query = sys::GetAllyTeamStartBoxQuery {
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetAllyTeamStartBoxResult>::zeroed();
            let func = self.api.GetAllyTeamStartBox.expect("GetAllyTeamStartBox function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.box_,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_team_start_position(&self, team_id: i32) -> Result<(sys::Float3, bool), Error> {
        unsafe {
            let query = sys::GetTeamStartPositionQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamStartPositionResult>::zeroed();
            let func = self.api.GetTeamStartPosition.expect("GetTeamStartPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.position,
                result.valid,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_map_start_positions(&self) -> Result<Vec<sys::StartPosition>, Error> {
        unsafe {
            let query = sys::GetMapStartPositionsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMapStartPositionsResult>::zeroed();
            let func = self.api.GetMapStartPositions.expect("GetMapStartPositions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.positions.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.positions as *const sys::StartPosition, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_game_rules_resource_info(&self) -> Result<sys::GameRulesResourceInfo, Error> {
        unsafe {
            let query = sys::GetGameRulesResourceInfoQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameRulesResourceInfoResult>::zeroed();
            let func = self.api.GetGameRulesResourceInfo.expect("GetGameRulesResourceInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

}

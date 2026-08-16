impl<'a> RulesParams<'a> {
    pub fn get_game_rules_param(&self, param_name: &str) -> Result<(RulesParamValue, i32, bool), Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let query = sys::GetGameRulesParamQuery {
                paramName: param_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetGameRulesParamResult>::zeroed();
            let func = self.api.GetGameRulesParam.expect("GetGameRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                RulesParamValue::from_sys(result.value),
                result.los,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_game_rules_params(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetGameRulesParamsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameRulesParamsResult>::zeroed();
            let func = self.api.GetGameRulesParams.expect("GetGameRulesParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.names.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.names, result.count as usize);
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

    pub fn get_team_rules_param(&self, team_id: i32, param_name: &str) -> Result<(RulesParamValue, i32, bool), Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let query = sys::GetTeamRulesParamQuery {
                teamID: team_id,
                paramName: param_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetTeamRulesParamResult>::zeroed();
            let func = self.api.GetTeamRulesParam.expect("GetTeamRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                RulesParamValue::from_sys(result.value),
                result.los,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_team_rules_params(&self, team_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetTeamRulesParamsQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamRulesParamsResult>::zeroed();
            let func = self.api.GetTeamRulesParams.expect("GetTeamRulesParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.names.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.names, result.count as usize);
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

    pub fn get_player_rules_param(&self, player_id: i32, param_name: &str) -> Result<(RulesParamValue, i32, bool), Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let query = sys::GetPlayerRulesParamQuery {
                playerID: player_id,
                paramName: param_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetPlayerRulesParamResult>::zeroed();
            let func = self.api.GetPlayerRulesParam.expect("GetPlayerRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                RulesParamValue::from_sys(result.value),
                result.los,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_player_rules_params(&self, player_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetPlayerRulesParamsQuery {
                playerID: player_id,
            };
            let mut result = MaybeUninit::<sys::GetPlayerRulesParamsResult>::zeroed();
            let func = self.api.GetPlayerRulesParams.expect("GetPlayerRulesParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.names.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.names, result.count as usize);
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

    pub fn get_unit_rules_param(&self, unit_id: i32, param_name: &str) -> Result<(RulesParamValue, i32, bool), Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let query = sys::GetUnitRulesParamQuery {
                unitID: unit_id,
                paramName: param_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitRulesParamResult>::zeroed();
            let func = self.api.GetUnitRulesParam.expect("GetUnitRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                RulesParamValue::from_sys(result.value),
                result.los,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_rules_params(&self, unit_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetUnitRulesParamsQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitRulesParamsResult>::zeroed();
            let func = self.api.GetUnitRulesParams.expect("GetUnitRulesParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.names.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.names, result.count as usize);
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

    pub fn get_feature_rules_param(&self, feature_id: i32, param_name: &str) -> Result<(RulesParamValue, i32, bool), Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let query = sys::GetFeatureRulesParamQuery {
                featureID: feature_id,
                paramName: param_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetFeatureRulesParamResult>::zeroed();
            let func = self.api.GetFeatureRulesParam.expect("GetFeatureRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                RulesParamValue::from_sys(result.value),
                result.los,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_feature_rules_params(&self, feature_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetFeatureRulesParamsQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureRulesParamsResult>::zeroed();
            let func = self.api.GetFeatureRulesParams.expect("GetFeatureRulesParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.names.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.names, result.count as usize);
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

    pub fn set_game_rules_param(&self, param_name: &str, value: RulesParamValue, los: i32) -> Result<bool, Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let value_sys = value.to_sys()?;
            let query = sys::SetGameRulesParamQuery {
                paramName: param_name_cstr.as_ptr(),
                value: value_sys.value,
                los: los,
            };
            let mut result = MaybeUninit::<sys::SetGameRulesParamResult>::zeroed();
            let func = self.api.SetGameRulesParam.expect("SetGameRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_team_rules_param(&self, team_id: i32, param_name: &str, value: RulesParamValue, los: i32) -> Result<bool, Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let value_sys = value.to_sys()?;
            let query = sys::SetTeamRulesParamQuery {
                teamID: team_id,
                paramName: param_name_cstr.as_ptr(),
                value: value_sys.value,
                los: los,
            };
            let mut result = MaybeUninit::<sys::SetTeamRulesParamResult>::zeroed();
            let func = self.api.SetTeamRulesParam.expect("SetTeamRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_player_rules_param(&self, player_id: i32, param_name: &str, value: RulesParamValue, los: i32) -> Result<bool, Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let value_sys = value.to_sys()?;
            let query = sys::SetPlayerRulesParamQuery {
                playerID: player_id,
                paramName: param_name_cstr.as_ptr(),
                value: value_sys.value,
                los: los,
            };
            let mut result = MaybeUninit::<sys::SetPlayerRulesParamResult>::zeroed();
            let func = self.api.SetPlayerRulesParam.expect("SetPlayerRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_rules_param(&self, unit_id: i32, param_name: &str, value: RulesParamValue, los: i32) -> Result<bool, Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let value_sys = value.to_sys()?;
            let query = sys::SetUnitRulesParamQuery {
                unitID: unit_id,
                paramName: param_name_cstr.as_ptr(),
                value: value_sys.value,
                los: los,
            };
            let mut result = MaybeUninit::<sys::SetUnitRulesParamResult>::zeroed();
            let func = self.api.SetUnitRulesParam.expect("SetUnitRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_rules_param(&self, feature_id: i32, param_name: &str, value: RulesParamValue, los: i32) -> Result<bool, Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let value_sys = value.to_sys()?;
            let query = sys::SetFeatureRulesParamQuery {
                featureID: feature_id,
                paramName: param_name_cstr.as_ptr(),
                value: value_sys.value,
                los: los,
            };
            let mut result = MaybeUninit::<sys::SetFeatureRulesParamResult>::zeroed();
            let func = self.api.SetFeatureRulesParam.expect("SetFeatureRulesParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

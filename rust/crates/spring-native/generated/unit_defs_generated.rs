impl<'a> UnitDefs<'a> {
    pub fn get_unit_def_ids(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitDefIDsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefIDsResult>::zeroed();
            let func = self.api.GetUnitDefIDs.expect("GetUnitDefIDs function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.ids.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.ids as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_def_count(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetUnitDefCountQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefCountResult>::zeroed();
            let func = self.api.GetUnitDefCount.expect("GetUnitDefCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_unit_def_by_id(&self, unit_def_id: i32) -> Result<(bool, sys::UnitDefBasicInfo, sys::UnitDefCosts, sys::UnitDefPhysics, sys::UnitDefWeapons, sys::UnitDefBuildOptions, sys::UnitDefSensors, sys::UnitDefHealth, sys::UnitDefClassify), Error> {
        unsafe {
            let query = sys::GetUnitDefByIDQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefByIDResult>::zeroed();
            let func = self.api.GetUnitDefByID.expect("GetUnitDefByID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.exists,
                result.basic,
                result.costs,
                result.physics,
                result.weapons,
                result.buildOptions,
                result.sensors,
                result.health,
                result.classify,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_def_idby_name(&self, unit_def_name: &str) -> Result<i32, Error> {
        unsafe {
            let unit_def_name_cstr = std::ffi::CString::new(unit_def_name).map_err(|_| Error::invalid_argument("unit_def_name"))?;
            let query = sys::GetUnitDefIDByNameQuery {
                unitDefName: unit_def_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitDefIDByNameResult>::zeroed();
            let func = self.api.GetUnitDefIDByName.expect("GetUnitDefIDByName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.id
            })
        }
    }

    pub fn valid_unit_def_id(&self, unit_def_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::ValidUnitDefIDQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::ValidUnitDefIDResult>::zeroed();
            let func = self.api.ValidUnitDefID.expect("ValidUnitDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.valid
            })
        }
    }

    pub fn get_unit_def_name(&self, unit_def_id: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetUnitDefNameQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefNameResult>::zeroed();
            let func = self.api.GetUnitDefName.expect("GetUnitDefName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.name.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.name).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_unit_def_human_name(&self, unit_def_id: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetUnitDefHumanNameQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefHumanNameResult>::zeroed();
            let func = self.api.GetUnitDefHumanName.expect("GetUnitDefHumanName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.humanName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.humanName).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_unit_def_costs(&self, unit_def_id: i32) -> Result<sys::UnitDefCosts, Error> {
        unsafe {
            let query = sys::GetUnitDefCostsQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefCostsResult>::zeroed();
            let func = self.api.GetUnitDefCosts.expect("GetUnitDefCosts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.costs
            })
        }
    }

    pub fn get_unit_def_speed(&self, unit_def_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitDefSpeedQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefSpeedResult>::zeroed();
            let func = self.api.GetUnitDefSpeed.expect("GetUnitDefSpeed function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.speed
            })
        }
    }

    pub fn get_unit_def_health(&self, unit_def_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitDefHealthQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefHealthResult>::zeroed();
            let func = self.api.GetUnitDefHealth.expect("GetUnitDefHealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.health
            })
        }
    }

    pub fn get_unit_def_custom_param(&self, unit_def_id: i32, key: &str) -> Result<Option<String>, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetUnitDefCustomParamQuery {
                unitDefID: unit_def_id,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitDefCustomParamResult>::zeroed();
            let func = self.api.GetUnitDefCustomParam.expect("GetUnitDefCustomParam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_unit_def_custom_param_keys(&self, unit_def_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetUnitDefCustomParamKeysQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefCustomParamKeysResult>::zeroed();
            let func = self.api.GetUnitDefCustomParamKeys.expect("GetUnitDefCustomParamKeys function pointer must be initialized");
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

    pub fn get_unit_def_classify(&self, unit_def_id: i32) -> Result<sys::UnitDefClassify, Error> {
        unsafe {
            let query = sys::GetUnitDefClassifyQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefClassifyResult>::zeroed();
            let func = self.api.GetUnitDefClassify.expect("GetUnitDefClassify function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.classify
            })
        }
    }

    pub fn get_unit_def_param_keys(&self) -> Result<Vec<sys::UnitDefParamKey>, Error> {
        unsafe {
            let query = sys::GetUnitDefParamKeysQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefParamKeysResult>::zeroed();
            let func = self.api.GetUnitDefParamKeys.expect("GetUnitDefParamKeys function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.keys.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.keys as *const sys::UnitDefParamKey, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_def_param_type(&self, key: &str) -> Result<i32, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetUnitDefParamTypeQuery {
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitDefParamTypeResult>::zeroed();
            let func = self.api.GetUnitDefParamType.expect("GetUnitDefParamType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.type_
            })
        }
    }

    pub fn get_unit_def_param_bool(&self, unit_def_id: i32, key: &str) -> Result<bool, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetUnitDefParamBoolQuery {
                unitDefID: unit_def_id,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitDefParamBoolResult>::zeroed();
            let func = self.api.GetUnitDefParamBool.expect("GetUnitDefParamBool function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_unit_def_param_int(&self, unit_def_id: i32, key: &str) -> Result<i32, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetUnitDefParamIntQuery {
                unitDefID: unit_def_id,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitDefParamIntResult>::zeroed();
            let func = self.api.GetUnitDefParamInt.expect("GetUnitDefParamInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_unit_def_param_float(&self, unit_def_id: i32, key: &str) -> Result<f32, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetUnitDefParamFloatQuery {
                unitDefID: unit_def_id,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitDefParamFloatResult>::zeroed();
            let func = self.api.GetUnitDefParamFloat.expect("GetUnitDefParamFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_unit_def_param_string(&self, unit_def_id: i32, key: &str) -> Result<Option<String>, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetUnitDefParamStringQuery {
                unitDefID: unit_def_id,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitDefParamStringResult>::zeroed();
            let func = self.api.GetUnitDefParamString.expect("GetUnitDefParamString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

}

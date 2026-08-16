impl<'a> FeatureDefs<'a> {
    pub fn get_feature_def_ids(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetFeatureDefIDsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefIDsResult>::zeroed();
            let func = self.api.GetFeatureDefIDs.expect("GetFeatureDefIDs function pointer must be initialized");
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

    pub fn get_feature_def_count(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetFeatureDefCountQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefCountResult>::zeroed();
            let func = self.api.GetFeatureDefCount.expect("GetFeatureDefCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_feature_def_by_id(&self, feature_def_id: i32) -> Result<(sys::FeatureDefInfo, bool), Error> {
        unsafe {
            let query = sys::GetFeatureDefByIDQuery {
                featureDefID: feature_def_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefByIDResult>::zeroed();
            let func = self.api.GetFeatureDefByID.expect("GetFeatureDefByID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.info,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_feature_def_idby_name(&self, feature_def_name: &str) -> Result<i32, Error> {
        unsafe {
            let feature_def_name_cstr = std::ffi::CString::new(feature_def_name).map_err(|_| Error::invalid_argument("feature_def_name"))?;
            let query = sys::GetFeatureDefIDByNameQuery {
                featureDefName: feature_def_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefIDByNameResult>::zeroed();
            let func = self.api.GetFeatureDefIDByName.expect("GetFeatureDefIDByName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.id
            })
        }
    }

    pub fn valid_feature_def_id(&self, feature_def_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::ValidFeatureDefIDQuery {
                featureDefID: feature_def_id,
            };
            let mut result = MaybeUninit::<sys::ValidFeatureDefIDResult>::zeroed();
            let func = self.api.ValidFeatureDefID.expect("ValidFeatureDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.valid
            })
        }
    }

    pub fn get_feature_def_name(&self, feature_def_id: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetFeatureDefNameQuery {
                featureDefID: feature_def_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefNameResult>::zeroed();
            let func = self.api.GetFeatureDefName.expect("GetFeatureDefName function pointer must be initialized");
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

    pub fn get_feature_def_metal(&self, feature_def_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureDefMetalQuery {
                featureDefID: feature_def_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefMetalResult>::zeroed();
            let func = self.api.GetFeatureDefMetal.expect("GetFeatureDefMetal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.metal
            })
        }
    }

    pub fn get_feature_def_energy(&self, feature_def_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureDefEnergyQuery {
                featureDefID: feature_def_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefEnergyResult>::zeroed();
            let func = self.api.GetFeatureDefEnergy.expect("GetFeatureDefEnergy function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.energy
            })
        }
    }

    pub fn get_feature_def_custom_param(&self, feature_def_id: i32, key: &str) -> Result<Option<String>, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetFeatureDefCustomParamQuery {
                featureDefID: feature_def_id,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefCustomParamResult>::zeroed();
            let func = self.api.GetFeatureDefCustomParam.expect("GetFeatureDefCustomParam function pointer must be initialized");
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

    pub fn get_feature_def_custom_param_keys(&self, feature_def_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetFeatureDefCustomParamKeysQuery {
                featureDefID: feature_def_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefCustomParamKeysResult>::zeroed();
            let func = self.api.GetFeatureDefCustomParamKeys.expect("GetFeatureDefCustomParamKeys function pointer must be initialized");
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

}

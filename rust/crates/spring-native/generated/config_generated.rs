impl<'a> Config<'a> {
    pub fn get_config_int(&self, key: &str, default_value: Option<i32>) -> Result<(i32, bool), Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetConfigIntQuery {
                key: key_cstr.as_ptr(),
                defaultValue: default_value.unwrap_or(0),
                hasDefault: default_value.is_some(),
            };
            let mut result = MaybeUninit::<sys::GetConfigIntResult>::zeroed();
            let func = self.api.GetConfigInt.expect("GetConfigInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_config_float(&self, key: &str, default_value: Option<f32>) -> Result<(f32, bool), Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetConfigFloatQuery {
                key: key_cstr.as_ptr(),
                defaultValue: default_value.unwrap_or(0.0),
                hasDefault: default_value.is_some(),
            };
            let mut result = MaybeUninit::<sys::GetConfigFloatResult>::zeroed();
            let func = self.api.GetConfigFloat.expect("GetConfigFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_config_string(&self, key: &str, default_value: Option<&str>) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let default_value_cstr = default_value.as_ref().map(|value| std::ffi::CString::new(*value)).transpose().map_err(|_| Error::invalid_argument("default_value"))?;
            let query = sys::GetConfigStringQuery {
                key: key_cstr.as_ptr(),
                defaultValue: default_value_cstr.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
                hasDefault: default_value.is_some(),
            };
            let mut result = MaybeUninit::<sys::GetConfigStringResult>::zeroed();
            let func = self.api.GetConfigString.expect("GetConfigString function pointer must be initialized");
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

    pub fn get_config_params(&self) -> Result<Vec<sys::ConfigParam>, Error> {
        unsafe {
            let query = sys::GetConfigParamsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetConfigParamsResult>::zeroed();
            let func = self.api.GetConfigParams.expect("GetConfigParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.params.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.params as *const sys::ConfigParam, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn set_config_int(&self, key: &str, value: i32, use_overlay: bool) -> Result<bool, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::SetConfigIntQuery {
                key: key_cstr.as_ptr(),
                value: value,
                useOverlay: use_overlay,
            };
            let mut result = MaybeUninit::<sys::SetConfigIntResult>::zeroed();
            let func = self.api.SetConfigInt.expect("SetConfigInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_config_float(&self, key: &str, value: f32, use_overlay: bool) -> Result<bool, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::SetConfigFloatQuery {
                key: key_cstr.as_ptr(),
                value: value,
                useOverlay: use_overlay,
            };
            let mut result = MaybeUninit::<sys::SetConfigFloatResult>::zeroed();
            let func = self.api.SetConfigFloat.expect("SetConfigFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_config_string(&self, key: &str, value: &str, use_overlay: bool) -> Result<bool, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::SetConfigStringQuery {
                key: key_cstr.as_ptr(),
                value: value_cstr.as_ptr(),
                useOverlay: use_overlay,
            };
            let mut result = MaybeUninit::<sys::SetConfigStringResult>::zeroed();
            let func = self.api.SetConfigString.expect("SetConfigString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_log_sections(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetLogSectionsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLogSectionsResult>::zeroed();
            let func = self.api.GetLogSections.expect("GetLogSections function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.sections.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.sections, result.count as usize);
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

    pub fn set_log_section_filter_level(&self, section: &str, level: i32) -> Result<bool, Error> {
        unsafe {
            let section_cstr = std::ffi::CString::new(section).map_err(|_| Error::invalid_argument("section"))?;
            let query = sys::SetLogSectionFilterLevelQuery {
                section: section_cstr.as_ptr(),
                level: level,
            };
            let mut result = MaybeUninit::<sys::SetLogSectionFilterLevelResult>::zeroed();
            let func = self.api.SetLogSectionFilterLevel.expect("SetLogSectionFilterLevel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

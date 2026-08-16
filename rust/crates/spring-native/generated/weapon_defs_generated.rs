impl<'a> WeaponDefs<'a> {
    pub fn get_weapon_def_ids(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetWeaponDefIDsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefIDsResult>::zeroed();
            let func = self.api.GetWeaponDefIDs.expect("GetWeaponDefIDs function pointer must be initialized");
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

    pub fn get_weapon_def_count(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetWeaponDefCountQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefCountResult>::zeroed();
            let func = self.api.GetWeaponDefCount.expect("GetWeaponDefCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_weapon_def_by_id(&self, weapon_def_id: i32) -> Result<(sys::WeaponDefInfo, bool), Error> {
        unsafe {
            let query = sys::GetWeaponDefByIDQuery {
                weaponDefID: weapon_def_id,
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefByIDResult>::zeroed();
            let func = self.api.GetWeaponDefByID.expect("GetWeaponDefByID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.info,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_weapon_def_id(&self, weapon_def_name: &str) -> Result<i32, Error> {
        unsafe {
            let weapon_def_name_cstr = std::ffi::CString::new(weapon_def_name).map_err(|_| Error::invalid_argument("weapon_def_name"))?;
            let query = sys::GetWeaponDefIDQuery {
                weaponDefName: weapon_def_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefIDResult>::zeroed();
            let func = self.api.GetWeaponDefID.expect("GetWeaponDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.id
            })
        }
    }

    pub fn valid_weapon_def_id(&self, weapon_def_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::ValidWeaponDefIDQuery {
                weaponDefID: weapon_def_id,
            };
            let mut result = MaybeUninit::<sys::ValidWeaponDefIDResult>::zeroed();
            let func = self.api.ValidWeaponDefID.expect("ValidWeaponDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.valid
            })
        }
    }

    pub fn get_weapon_def_name(&self, weapon_def_id: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetWeaponDefNameQuery {
                weaponDefID: weapon_def_id,
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefNameResult>::zeroed();
            let func = self.api.GetWeaponDefName.expect("GetWeaponDefName function pointer must be initialized");
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

    pub fn get_weapon_def_range(&self, weapon_def_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetWeaponDefRangeQuery {
                weaponDefID: weapon_def_id,
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefRangeResult>::zeroed();
            let func = self.api.GetWeaponDefRange.expect("GetWeaponDefRange function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.range
            })
        }
    }

    pub fn get_weapon_def_damage(&self, weapon_def_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetWeaponDefDamageQuery {
                weaponDefID: weapon_def_id,
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefDamageResult>::zeroed();
            let func = self.api.GetWeaponDefDamage.expect("GetWeaponDefDamage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.damage
            })
        }
    }

    pub fn get_weapon_def_custom_param(&self, weapon_def_id: i32, key: &str) -> Result<Option<String>, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetWeaponDefCustomParamQuery {
                weaponDefID: weapon_def_id,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefCustomParamResult>::zeroed();
            let func = self.api.GetWeaponDefCustomParam.expect("GetWeaponDefCustomParam function pointer must be initialized");
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

    pub fn get_weapon_def_custom_param_keys(&self, weapon_def_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetWeaponDefCustomParamKeysQuery {
                weaponDefID: weapon_def_id,
            };
            let mut result = MaybeUninit::<sys::GetWeaponDefCustomParamKeysResult>::zeroed();
            let func = self.api.GetWeaponDefCustomParamKeys.expect("GetWeaponDefCustomParamKeys function pointer must be initialized");
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

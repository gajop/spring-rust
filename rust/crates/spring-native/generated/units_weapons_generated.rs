#[derive(Debug, Clone, Copy, Default)]
pub struct GetUnitWeaponHaveFreeLineOfFireOptions {
    pub is_ground_target: bool,
}

impl From<GetUnitWeaponHaveFreeLineOfFireOptions> for sys::GetUnitWeaponHaveFreeLineOfFireOptions {
    fn from(options: GetUnitWeaponHaveFreeLineOfFireOptions) -> Self {
        sys::GetUnitWeaponHaveFreeLineOfFireOptions {
            isGroundTarget: options.is_ground_target,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetUnitWeaponTestTargetOptions {
    pub is_ground_target: bool,
}

impl From<GetUnitWeaponTestTargetOptions> for sys::GetUnitWeaponTestTargetOptions {
    fn from(options: GetUnitWeaponTestTargetOptions) -> Self {
        sys::GetUnitWeaponTestTargetOptions {
            isGroundTarget: options.is_ground_target,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetUnitWeaponTryTargetOptions {
    pub user_target: bool,
    pub is_ground_target: bool,
}

impl From<GetUnitWeaponTryTargetOptions> for sys::GetUnitWeaponTryTargetOptions {
    fn from(options: GetUnitWeaponTryTargetOptions) -> Self {
        sys::GetUnitWeaponTryTargetOptions {
            userTarget: options.user_target,
            isGroundTarget: options.is_ground_target,
        }
    }
}

impl<'a> UnitsWeapons<'a> {
    pub fn get_unit_weapon_count(&self, unit_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetUnitWeaponCountQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponCountResult>::zeroed();
            let func = self.api.GetUnitWeaponCount.expect("GetUnitWeaponCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_unit_max_range(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitMaxRangeQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitMaxRangeResult>::zeroed();
            let func = self.api.GetUnitMaxRange.expect("GetUnitMaxRange function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.maxRange
            })
        }
    }

    pub fn get_unit_weapon_state(&self, unit_id: i32, weapon_num: i32, key: &str) -> Result<sys::UnitWeaponState, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::GetUnitWeaponStateQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                key: key_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponStateResult>::zeroed();
            let func = self.api.GetUnitWeaponState.expect("GetUnitWeaponState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.state
            })
        }
    }

    pub fn get_unit_weapon_damages(&self, unit_id: i32, weapon_num: i32) -> Result<sys::UnitWeaponDamages, Error> {
        unsafe {
            let query = sys::GetUnitWeaponDamagesQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponDamagesResult>::zeroed();
            let func = self.api.GetUnitWeaponDamages.expect("GetUnitWeaponDamages function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.damages
            })
        }
    }

    pub fn get_unit_weapon_vectors(&self, unit_id: i32, weapon_num: i32) -> Result<sys::UnitWeaponVectors, Error> {
        unsafe {
            let query = sys::GetUnitWeaponVectorsQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponVectorsResult>::zeroed();
            let func = self.api.GetUnitWeaponVectors.expect("GetUnitWeaponVectors function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.vectors
            })
        }
    }

    pub fn get_unit_weapon_try_target(&self, unit_id: i32, weapon_num: i32, target_id: i32, target_pos: sys::Float3, options: GetUnitWeaponTryTargetOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitWeaponTryTargetQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                targetID: target_id,
                targetPos: target_pos,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponTryTargetResult>::zeroed();
            let func = self.api.GetUnitWeaponTryTarget.expect("GetUnitWeaponTryTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.canTarget
            })
        }
    }

    pub fn get_unit_weapon_test_target(&self, unit_id: i32, weapon_num: i32, target_id: i32, target_pos: sys::Float3, options: GetUnitWeaponTestTargetOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitWeaponTestTargetQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                targetID: target_id,
                targetPos: target_pos,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponTestTargetResult>::zeroed();
            let func = self.api.GetUnitWeaponTestTarget.expect("GetUnitWeaponTestTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.canTarget
            })
        }
    }

    pub fn get_unit_weapon_test_range(&self, unit_id: i32, weapon_num: i32, target_pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitWeaponTestRangeQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                targetPos: target_pos,
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponTestRangeResult>::zeroed();
            let func = self.api.GetUnitWeaponTestRange.expect("GetUnitWeaponTestRange function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inRange
            })
        }
    }

    pub fn get_unit_weapon_have_free_line_of_fire(&self, unit_id: i32, weapon_num: i32, target_id: i32, source_pos: sys::Float3, target_pos: sys::Float3, options: GetUnitWeaponHaveFreeLineOfFireOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitWeaponHaveFreeLineOfFireQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                targetID: target_id,
                sourcePos: source_pos,
                targetPos: target_pos,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponHaveFreeLineOfFireResult>::zeroed();
            let func = self.api.GetUnitWeaponHaveFreeLineOfFire.expect("GetUnitWeaponHaveFreeLineOfFire function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.hasFreeLineOfFire
            })
        }
    }

    pub fn get_unit_weapon_can_fire(&self, unit_id: i32, weapon_num: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitWeaponCanFireQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponCanFireResult>::zeroed();
            let func = self.api.GetUnitWeaponCanFire.expect("GetUnitWeaponCanFire function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.canFire
            })
        }
    }

    pub fn get_unit_weapon_target(&self, unit_id: i32, weapon_num: i32) -> Result<sys::UnitWeaponTarget, Error> {
        unsafe {
            let query = sys::GetUnitWeaponTargetQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitWeaponTargetResult>::zeroed();
            let func = self.api.GetUnitWeaponTarget.expect("GetUnitWeaponTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.target
            })
        }
    }

}

impl<'a> MoveCtrl<'a> {
    pub fn get_unit_move_type_data(&self, unit_id: i32) -> Result<sys::MoveTypeData, Error> {
        unsafe {
            let query = sys::GetUnitMoveTypeDataQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitMoveTypeDataResult>::zeroed();
            let func = self.api.GetUnitMoveTypeData.expect("GetUnitMoveTypeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.data
            })
        }
    }

    pub fn get_unit_estimated_path(&self, unit_id: i32) -> Result<(Vec<sys::PathWaypoint>, Vec<i32>), Error> {
        unsafe {
            let query = sys::GetUnitEstimatedPathQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitEstimatedPathResult>::zeroed();
            let func = self.api.GetUnitEstimatedPath.expect("GetUnitEstimatedPath function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    let slice = if result.count == 0 || result.waypoints.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.waypoints as *const sys::PathWaypoint, result.count as usize)
                    };
                    slice.to_vec()
                },
                {
                    let slice = if result.startCount == 0 || result.starts.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.starts as *const i32, result.startCount as usize)
                    };
                    slice.to_vec()
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn move_ctrl(&self, unit_id: i32, enable: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlQuery {
                unitID: unit_id,
                enable: enable,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.MoveCtrl.expect("MoveCtrl function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn is_move_ctrl_enabled(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsMoveCtrlEnabledQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::IsMoveCtrlEnabledResult>::zeroed();
            let func = self.api.IsMoveCtrlEnabled.expect("IsMoveCtrlEnabled function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn set_move_ctrl_gravity(&self, unit_id: i32, gravity_factor: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMoveCtrlGravityQuery {
                unitID: unit_id,
                gravityFactor: gravity_factor,
            };
            let mut result = MaybeUninit::<sys::SetMoveCtrlGravityResult>::zeroed();
            let func = self.api.SetMoveCtrlGravity.expect("SetMoveCtrlGravity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

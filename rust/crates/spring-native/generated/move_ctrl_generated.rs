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
                enable,
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

    pub fn set_tag(&self, unit_id: i32, tag: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMoveCtrlTagQuery {
                unitID: unit_id,
                tag,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetTag.expect("SetTag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_tag(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetMoveCtrlTagQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetMoveCtrlTagResult>::zeroed();
            let func = self.api.GetTag.expect("GetTag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.tag
            })
        }
    }

    pub fn set_progress_state(&self, unit_id: i32, state: sys::MoveCtrlProgressState) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMoveCtrlProgressStateQuery {
                unitID: unit_id,
                state,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetProgressState.expect("SetProgressState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_move_def(&self, unit_id: i32, move_def_id: i32, move_def_name: Option<&str>) -> Result<bool, Error> {
        unsafe {
            let move_def_name_cstr = move_def_name.as_ref().map(|value| std::ffi::CString::new(*value)).transpose().map_err(|_| Error::invalid_argument("move_def_name"))?;
            let query = sys::MoveCtrlMoveDefQuery {
                unitID: unit_id,
                moveDefID: move_def_id,
                moveDefName: move_def_name_cstr.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
                hasMoveDefName: move_def_name.is_some(),
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetMoveDef.expect("SetMoveDef function pointer must be initialized");
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

    pub fn set_ground_move_type_max_speed(&self, unit_id: i32, max_speed: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundMoveTypeMaxSpeedQuery {
                unitID: unit_id,
                maxSpeed: max_speed,
            };
            let mut result = MaybeUninit::<sys::SetGroundMoveTypeMaxSpeedResult>::zeroed();
            let func = self.api.SetGroundMoveTypeMaxSpeed.expect("SetGroundMoveTypeMaxSpeed function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_move_type_numeric(&self, unit_id: i32, field: sys::MoveTypeNumericField, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMoveTypeNumericQuery {
                unitID: unit_id,
                field,
                value,
            };
            let mut result = MaybeUninit::<sys::SetMoveTypeNumericResult>::zeroed();
            let func = self.api.SetMoveTypeNumeric.expect("SetMoveTypeNumeric function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_move_type_boolean(&self, unit_id: i32, field: sys::MoveTypeBooleanField, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMoveTypeBooleanQuery {
                unitID: unit_id,
                field,
                value,
            };
            let mut result = MaybeUninit::<sys::SetMoveTypeBooleanResult>::zeroed();
            let func = self.api.SetMoveTypeBoolean.expect("SetMoveTypeBoolean function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_no_blocking(&self, unit_id: i32, no_blocking: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetNoBlockingQuery {
                unitID: unit_id,
                noBlocking: no_blocking,
            };
            let mut result = MaybeUninit::<sys::SetNoBlockingResult>::zeroed();
            let func = self.api.SetNoBlocking.expect("SetNoBlocking function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_extrapolate(&self, unit_id: i32, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlBoolQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetExtrapolate.expect("SetExtrapolate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_physics(&self, unit_id: i32, position: sys::Float3, velocity: sys::Float3, rotation: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlPhysicsQuery {
                unitID: unit_id,
                position,
                velocity,
                rotation,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetPhysics.expect("SetPhysics function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_position(&self, unit_id: i32, value: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloat3Query {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetPosition.expect("SetPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_velocity(&self, unit_id: i32, value: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloat3Query {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetVelocity.expect("SetVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_relative_velocity(&self, unit_id: i32, value: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloat3Query {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetRelativeVelocity.expect("SetRelativeVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_rotation(&self, unit_id: i32, value: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloat3Query {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetRotation.expect("SetRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_rotation_velocity(&self, unit_id: i32, value: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloat3Query {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetRotationVelocity.expect("SetRotationVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_heading(&self, unit_id: i32, heading: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlHeadingQuery {
                unitID: unit_id,
                heading,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetHeading.expect("SetHeading function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_track_slope(&self, unit_id: i32, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlBoolQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetTrackSlope.expect("SetTrackSlope function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_track_ground(&self, unit_id: i32, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlBoolQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetTrackGround.expect("SetTrackGround function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_track_limits(&self, unit_id: i32, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlBoolQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetTrackLimits.expect("SetTrackLimits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_offset(&self, unit_id: i32, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloatQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetGroundOffset.expect("SetGroundOffset function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_gravity(&self, unit_id: i32, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloatQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetGravity.expect("SetGravity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_drag(&self, unit_id: i32, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloatQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetDrag.expect("SetDrag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_wind_factor(&self, unit_id: i32, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlFloatQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetWindFactor.expect("SetWindFactor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_limits(&self, unit_id: i32, mins: sys::Float3, maxs: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlLimitsQuery {
                unitID: unit_id,
                mins,
                maxs,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetLimits.expect("SetLimits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_collide_stop(&self, unit_id: i32, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlBoolQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetCollideStop.expect("SetCollideStop function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_limits_stop(&self, unit_id: i32, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::MoveCtrlBoolQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::MoveCtrlResult>::zeroed();
            let func = self.api.SetLimitsStop.expect("SetLimitsStop function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

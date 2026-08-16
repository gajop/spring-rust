#[derive(Debug, Clone, Copy, Default)]
pub struct SetCameraTargetOptions {
    pub transition_time: Option<f32>,
    pub dir_x: Option<f32>,
    pub dir_y: Option<f32>,
    pub dir_z: Option<f32>,
}

impl From<SetCameraTargetOptions> for sys::SetCameraTargetOptions {
    fn from(options: SetCameraTargetOptions) -> Self {
        sys::SetCameraTargetOptions {
            transitionTime: options.transition_time.unwrap_or(0.0),
            hasTransitionTime: options.transition_time.is_some(),
            dirX: options.dir_x.unwrap_or(0.0),
            hasDirX: options.dir_x.is_some(),
            dirY: options.dir_y.unwrap_or(0.0),
            hasDirY: options.dir_y.is_some(),
            dirZ: options.dir_z.unwrap_or(0.0),
            hasDirZ: options.dir_z.is_some(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TraceScreenRayOptions {
    pub only_coords: bool,
    pub use_minimap: bool,
    pub include_sky: bool,
    pub ignore_water: bool,
    pub height_offset: f32,
}

impl From<TraceScreenRayOptions> for sys::TraceScreenRayOptions {
    fn from(options: TraceScreenRayOptions) -> Self {
        sys::TraceScreenRayOptions {
            onlyCoords: options.only_coords,
            useMinimap: options.use_minimap,
            includeSky: options.include_sky,
            ignoreWater: options.ignore_water,
            heightOffset: options.height_offset,
        }
    }
}

impl<'a> Camera<'a> {
    pub fn get_camera_names(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetCameraNamesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetCameraNamesResult>::zeroed();
            let func = self.api.GetCameraNames.expect("GetCameraNames function pointer must be initialized");
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

    pub fn get_camera_state(&self, use_table: bool) -> Result<sys::CameraState, Error> {
        unsafe {
            let query = sys::GetCameraStateQuery {
                useTable: use_table,
            };
            let mut result = MaybeUninit::<sys::GetCameraStateResult>::zeroed();
            let func = self.api.GetCameraState.expect("GetCameraState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.state
            })
        }
    }

    pub fn get_camera_position(&self) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetCameraPositionQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetCameraPositionResult>::zeroed();
            let func = self.api.GetCameraPosition.expect("GetCameraPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_camera_direction(&self) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetCameraDirectionQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetCameraDirectionResult>::zeroed();
            let func = self.api.GetCameraDirection.expect("GetCameraDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.direction
            })
        }
    }

    pub fn get_camera_fov(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetCameraFOVQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetCameraFOVResult>::zeroed();
            let func = self.api.GetCameraFOV.expect("GetCameraFOV function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.fov
            })
        }
    }

    pub fn world_to_screen_coords(&self, world_pos: sys::Float3) -> Result<(sys::Float3, bool), Error> {
        unsafe {
            let query = sys::WorldToScreenCoordsQuery {
                worldPos: world_pos,
            };
            let mut result = MaybeUninit::<sys::WorldToScreenCoordsResult>::zeroed();
            let func = self.api.WorldToScreenCoords.expect("WorldToScreenCoords function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.screenPos,
                result.valid,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn trace_screen_ray(&self, screen_x: f32, screen_y: f32, options: TraceScreenRayOptions) -> Result<(i32, i32, sys::Float3), Error> {
        unsafe {
            let query = sys::TraceScreenRayQuery {
                screenX: screen_x,
                screenY: screen_y,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::TraceScreenRayResult>::zeroed();
            let func = self.api.TraceScreenRay.expect("TraceScreenRay function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.hitType,
                result.hitID,
                result.hitPos,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_pixel_dir(&self, screen_x: f32, screen_y: f32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetPixelDirQuery {
                screenX: screen_x,
                screenY: screen_y,
            };
            let mut result = MaybeUninit::<sys::GetPixelDirResult>::zeroed();
            let func = self.api.GetPixelDir.expect("GetPixelDir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.direction
            })
        }
    }

    pub fn set_camera_state(&self, state: sys::CameraState, transition_time: f32, transition_time_factor: f32, transition_time_exponent: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetCameraStateQuery {
                state: state,
                transitionTime: transition_time,
                transitionTimeFactor: transition_time_factor,
                transitionTimeExponent: transition_time_exponent,
            };
            let mut result = MaybeUninit::<sys::SetCameraStateResult>::zeroed();
            let func = self.api.SetCameraState.expect("SetCameraState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_camera_target(&self, target: sys::Float3, options: SetCameraTargetOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetCameraTargetQuery {
                target: target,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetCameraTargetResult>::zeroed();
            let func = self.api.SetCameraTarget.expect("SetCameraTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

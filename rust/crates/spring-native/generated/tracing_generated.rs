#[derive(Debug, Clone, Copy, Default)]
pub struct TraceRayGroundBetweenPositionsOptions {
    pub test_water: Option<bool>,
}

impl From<TraceRayGroundBetweenPositionsOptions> for sys::TraceRayGroundBetweenPositionsOptions {
    fn from(options: TraceRayGroundBetweenPositionsOptions) -> Self {
        sys::TraceRayGroundBetweenPositionsOptions {
            testWater: options.test_water.unwrap_or(false),
            hasTestWater: options.test_water.is_some(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TraceRayGroundInDirectionOptions {
    pub length: Option<f32>,
    pub test_water: Option<bool>,
}

impl From<TraceRayGroundInDirectionOptions> for sys::TraceRayGroundInDirectionOptions {
    fn from(options: TraceRayGroundInDirectionOptions) -> Self {
        sys::TraceRayGroundInDirectionOptions {
            length: options.length.unwrap_or(0.0),
            hasLength: options.length.is_some(),
            testWater: options.test_water.unwrap_or(false),
            hasTestWater: options.test_water.is_some(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TraceRayInDirectionOptions {
    pub max_length: Option<f32>,
}

impl From<TraceRayInDirectionOptions> for sys::TraceRayInDirectionOptions {
    fn from(options: TraceRayInDirectionOptions) -> Self {
        sys::TraceRayInDirectionOptions {
            maxLength: options.max_length.unwrap_or(0.0),
            hasMaxLength: options.max_length.is_some(),
        }
    }
}

impl<'a> Tracing<'a> {
    pub fn trace_ray(&self, ray: sys::Ray) -> Result<(bool, i32, i32, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::TraceRayQuery {
                ray: ray,
            };
            let mut result = MaybeUninit::<sys::TraceRayResult>::zeroed();
            let func = self.api.TraceRay.expect("TraceRay function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.hit,
                result.hitType,
                result.hitID,
                result.hitPos,
                result.hitNormal,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn trace_ray_units(&self, ray: sys::Ray) -> Result<(bool, i32, i32, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::TraceRayUnitsQuery {
                ray: ray,
            };
            let mut result = MaybeUninit::<sys::TraceRayUnitsResult>::zeroed();
            let func = self.api.TraceRayUnits.expect("TraceRayUnits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.hit,
                result.hitType,
                result.hitID,
                result.hitPos,
                result.hitNormal,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn trace_ray_features(&self, ray: sys::Ray) -> Result<(bool, i32, i32, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::TraceRayFeaturesQuery {
                ray: ray,
            };
            let mut result = MaybeUninit::<sys::TraceRayFeaturesResult>::zeroed();
            let func = self.api.TraceRayFeatures.expect("TraceRayFeatures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.hit,
                result.hitType,
                result.hitID,
                result.hitPos,
                result.hitNormal,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn trace_ray_in_direction(&self, pos: sys::Float3, dir: sys::Float3, options: TraceRayInDirectionOptions, r#type: &str) -> Result<Vec<sys::TraceRayHit>, Error> {
        unsafe {
            let r#type_cstr = std::ffi::CString::new(r#type).map_err(|_| Error::invalid_argument("r#type"))?;
            let query = sys::TraceRayInDirectionQuery {
                pos: pos,
                dir: dir,
                options: options.into(),
                type_: r#type_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::TraceRayInDirectionResult>::zeroed();
            let func = self.api.TraceRayInDirection.expect("TraceRayInDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.hits.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.hits as *const sys::TraceRayHit, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn trace_ray_between_positions(&self, start: sys::Float3, end: sys::Float3, r#type: &str) -> Result<Vec<sys::TraceRayHit>, Error> {
        unsafe {
            let r#type_cstr = std::ffi::CString::new(r#type).map_err(|_| Error::invalid_argument("r#type"))?;
            let query = sys::TraceRayBetweenPositionsQuery {
                start: start,
                end: end,
                type_: r#type_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::TraceRayBetweenPositionsResult>::zeroed();
            let func = self.api.TraceRayBetweenPositions.expect("TraceRayBetweenPositions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.hits.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.hits as *const sys::TraceRayHit, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn trace_ray_ground_between_positions(&self, start: sys::Float3, end: sys::Float3, options: TraceRayGroundBetweenPositionsOptions) -> Result<(bool, f32, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::TraceRayGroundBetweenPositionsQuery {
                start: start,
                end: end,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::TraceRayGroundBetweenPositionsResult>::zeroed();
            let func = self.api.TraceRayGroundBetweenPositions.expect("TraceRayGroundBetweenPositions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.hit,
                result.hitLength,
                result.hitPos,
                result.hitNormal,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn trace_ray_ground_in_direction(&self, start: sys::Float3, dir: sys::Float3, options: TraceRayGroundInDirectionOptions) -> Result<(bool, f32, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::TraceRayGroundInDirectionQuery {
                start: start,
                dir: dir,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::TraceRayGroundInDirectionResult>::zeroed();
            let func = self.api.TraceRayGroundInDirection.expect("TraceRayGroundInDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.hit,
                result.hitLength,
                result.hitPos,
                result.hitNormal,
            );
            Error::result_or(result.error, value)
        }
    }

}

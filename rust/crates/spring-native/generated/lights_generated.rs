impl<'a> Lights<'a> {
    pub fn add_map_light(&self, params: sys::LightParams) -> Result<u32, Error> {
        unsafe {
            let query = sys::AddMapLightQuery {
                params: params,
            };
            let mut result = MaybeUninit::<sys::AddMapLightResult>::zeroed();
            let func = self.api.AddMapLight.expect("AddMapLight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.lightHandle
            })
        }
    }

    pub fn add_model_light(&self, params: sys::LightParams) -> Result<u32, Error> {
        unsafe {
            let query = sys::AddModelLightQuery {
                params: params,
            };
            let mut result = MaybeUninit::<sys::AddModelLightResult>::zeroed();
            let func = self.api.AddModelLight.expect("AddModelLight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.lightHandle
            })
        }
    }

    pub fn update_map_light(&self, light_handle: u32, params: sys::LightParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::UpdateMapLightQuery {
                lightHandle: light_handle,
                params: params,
            };
            let mut result = MaybeUninit::<sys::UpdateMapLightResult>::zeroed();
            let func = self.api.UpdateMapLight.expect("UpdateMapLight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn update_model_light(&self, light_handle: u32, params: sys::LightParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::UpdateModelLightQuery {
                lightHandle: light_handle,
                params: params,
            };
            let mut result = MaybeUninit::<sys::UpdateModelLightResult>::zeroed();
            let func = self.api.UpdateModelLight.expect("UpdateModelLight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_map_light_tracking_state(&self, light_handle: u32, object_id: i32, enable_tracking: bool, track_unit: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMapLightTrackingStateQuery {
                lightHandle: light_handle,
                objectID: object_id,
                enableTracking: enable_tracking,
                trackUnit: track_unit,
            };
            let mut result = MaybeUninit::<sys::SetMapLightTrackingStateResult>::zeroed();
            let func = self.api.SetMapLightTrackingState.expect("SetMapLightTrackingState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_model_light_tracking_state(&self, light_handle: u32, object_id: i32, enable_tracking: bool, track_unit: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetModelLightTrackingStateQuery {
                lightHandle: light_handle,
                objectID: object_id,
                enableTracking: enable_tracking,
                trackUnit: track_unit,
            };
            let mut result = MaybeUninit::<sys::SetModelLightTrackingStateResult>::zeroed();
            let func = self.api.SetModelLightTrackingState.expect("SetModelLightTrackingState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_light_tracking_target(&self, light_handle: u32, object_id: i32, track_unit: bool, enable_tracking: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddLightTrackingTargetQuery {
                lightHandle: light_handle,
                objectID: object_id,
                trackUnit: track_unit,
                enableTracking: enable_tracking,
            };
            let mut result = MaybeUninit::<sys::AddLightTrackingTargetResult>::zeroed();
            let func = self.api.AddLightTrackingTarget.expect("AddLightTrackingTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

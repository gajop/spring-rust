impl<'a> Memory<'a> {
    pub fn free_string_array(&self, data: *mut *const i8, length: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeStringArrayQuery {
                data: data,
                length: length,
            };
            let mut result = MaybeUninit::<sys::FreeStringArrayResult>::zeroed();
            let func = self.api.FreeStringArray.expect("FreeStringArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free_int32_array(&self, data: &[i32]) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeInt32ArrayQuery {
                data: data.as_ptr() as *mut _,
                length: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::FreeInt32ArrayResult>::zeroed();
            let func = self.api.FreeInt32Array.expect("FreeInt32Array function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free_uint32_array(&self, data: &[u32]) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeUInt32ArrayQuery {
                data: data.as_ptr() as *mut _,
                length: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::FreeUInt32ArrayResult>::zeroed();
            let func = self.api.FreeUInt32Array.expect("FreeUInt32Array function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free_float_array(&self, data: &[f32]) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeFloatArrayQuery {
                data: data.as_ptr() as *mut _,
                length: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::FreeFloatArrayResult>::zeroed();
            let func = self.api.FreeFloatArray.expect("FreeFloatArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free_float2_array(&self, data: &[sys::Float2]) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeFloat2ArrayQuery {
                data: data.as_ptr() as *mut _,
                length: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::FreeFloat2ArrayResult>::zeroed();
            let func = self.api.FreeFloat2Array.expect("FreeFloat2Array function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free_float3_array(&self, data: &[sys::Float3]) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeFloat3ArrayQuery {
                data: data.as_ptr() as *mut _,
                length: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::FreeFloat3ArrayResult>::zeroed();
            let func = self.api.FreeFloat3Array.expect("FreeFloat3Array function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free_float4_array(&self, data: &[sys::Float4]) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeFloat4ArrayQuery {
                data: data.as_ptr() as *mut _,
                length: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::FreeFloat4ArrayResult>::zeroed();
            let func = self.api.FreeFloat4Array.expect("FreeFloat4Array function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free_int3_array(&self, data: &[sys::Int3]) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeInt3ArrayQuery {
                data: data.as_ptr() as *mut _,
                length: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::FreeInt3ArrayResult>::zeroed();
            let func = self.api.FreeInt3Array.expect("FreeInt3Array function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn free(&self, ptr: *mut std::ffi::c_void) -> Result<(), Error> {
        unsafe {
            let query = sys::FreeQuery {
                ptr: ptr,
            };
            let mut result = MaybeUninit::<sys::FreeResult>::zeroed();
            let func = self.api.Free.expect("Free function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

}

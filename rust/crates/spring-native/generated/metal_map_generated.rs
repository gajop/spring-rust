impl<'a> MetalMap<'a> {
    pub fn get_metal_map_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let query = sys::GetMetalMapSizeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMetalMapSizeResult>::zeroed();
            let func = self.api.GetMetalMapSize.expect("GetMetalMapSize function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.width,
                result.height,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_metal_amount(&self, x: i32, z: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetMetalAmountQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetMetalAmountResult>::zeroed();
            let func = self.api.GetMetalAmount.expect("GetMetalAmount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.amount
            })
        }
    }

    pub fn get_metal_extraction(&self, x: i32, z: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetMetalExtractionQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetMetalExtractionResult>::zeroed();
            let func = self.api.GetMetalExtraction.expect("GetMetalExtraction function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.extraction
            })
        }
    }

    pub fn set_metal_amount(&self, x: i32, z: i32, amount: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::SetMetalAmountQuery {
                x,
                z,
                amount,
            };
            let mut result = MaybeUninit::<sys::SetMetalAmountResult>::zeroed();
            let func = self.api.SetMetalAmount.expect("SetMetalAmount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

}

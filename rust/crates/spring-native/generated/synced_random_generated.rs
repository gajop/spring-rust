impl<'a> SyncedRandom<'a> {
    pub fn next_float(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::NextFloatQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::NextFloatResult>::zeroed();
            let func = self.api.NextFloat.expect("NextFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn next_int_up_to(&self, upper: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::NextIntUpToQuery {
                upper,
            };
            let mut result = MaybeUninit::<sys::NextIntUpToResult>::zeroed();
            let func = self.api.NextIntUpTo.expect("NextIntUpTo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn next_int(&self, lower: i32, upper: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::NextIntQuery {
                lower,
                upper,
            };
            let mut result = MaybeUninit::<sys::NextIntResult>::zeroed();
            let func = self.api.NextInt.expect("NextInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn set_seed(&self, seed: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetSeedQuery {
                seed,
            };
            let mut result = MaybeUninit::<sys::SetSeedResult>::zeroed();
            let func = self.api.SetSeed.expect("SetSeed function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

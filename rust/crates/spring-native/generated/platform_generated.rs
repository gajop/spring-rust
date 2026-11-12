/// The complete result tuple returned by [`get_local_time`].
pub type GetLocalTimeValue = (i32, i32, i32, i32, i32, i32);

impl<'a> Platform<'a> {
    pub fn get_architecture(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetArchitectureQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetArchitectureResult>::zeroed();
            let func = self.api.GetArchitecture.expect("GetArchitecture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.architecture.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.architecture).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn is_headless(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsHeadlessQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsHeadlessResult>::zeroed();
            let func = self.api.IsHeadless.expect("IsHeadless function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isHeadless
            })
        }
    }

    pub fn get_local_time(&self) -> Result<GetLocalTimeValue, Error> {
        unsafe {
            let query = sys::GetLocalTimeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLocalTimeResult>::zeroed();
            let func = self.api.GetLocalTime.expect("GetLocalTime function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.year,
                result.month,
                result.day,
                result.hour,
                result.minute,
                result.second,
            );
            Error::result_or(result.error, value)
        }
    }

}

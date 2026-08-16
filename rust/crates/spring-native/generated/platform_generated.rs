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

}

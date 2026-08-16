impl<'a> CobScript<'a> {
    pub fn call_cobscript(&self, unit_id: i32, func: sys::CobFunctionRef, ret_args: u32, args: &[i32]) -> Result<(i32, Vec<i32>), Error> {
        unsafe {
            let query = sys::CallCOBScriptQuery {
                unitID: unit_id,
                func: func,
                retArgs: ret_args,
                args: args.as_ptr(),
                argCount: args.len() as u32,
            };
            let mut result = MaybeUninit::<sys::CallCOBScriptResult>::zeroed();
            let func = self.api.CallCOBScript.expect("CallCOBScript function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.retCode,
                {
                    let slice = if result.retCount == 0 || result.retValues.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.retValues as *const i32, result.retCount as usize)
                    };
                    slice.to_vec()
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_cobscript_id(&self, unit_id: i32, func_name: &str) -> Result<i32, Error> {
        unsafe {
            let func_name_cstr = std::ffi::CString::new(func_name).map_err(|_| Error::invalid_argument("func_name"))?;
            let query = sys::GetCOBScriptIDQuery {
                unitID: unit_id,
                funcName: func_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetCOBScriptIDResult>::zeroed();
            let func = self.api.GetCOBScriptID.expect("GetCOBScriptID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.funcID
            })
        }
    }

}

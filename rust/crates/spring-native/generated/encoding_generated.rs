impl<'a> Encoding<'a> {
    pub fn decode_base64(&self, text: &str) -> Result<Vec<u8>, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::DecodeBase64Query {
                text: text_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::DecodeBase64Result>::zeroed();
            let func = self.api.DecodeBase64.expect("DecodeBase64 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.decodedLength == 0 || result.decoded.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.decoded, result.decodedLength as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn encode_base64(&self, text: &[u8], strip_padding: bool) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::EncodeBase64Query {
                text: text.as_ptr(),
                textLength: text.len() as u32,
                stripPadding: strip_padding,
            };
            let mut result = MaybeUninit::<sys::EncodeBase64Result>::zeroed();
            let func = self.api.EncodeBase64.expect("EncodeBase64 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.encoded.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.encoded).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn is_valid_base64(&self, text: &str) -> Result<bool, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::IsValidBase64Query {
                text: text_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::IsValidBase64Result>::zeroed();
            let func = self.api.IsValidBase64.expect("IsValidBase64 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.valid
            })
        }
    }

    pub fn decode_base64_url(&self, text: &str) -> Result<Vec<u8>, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::DecodeBase64UrlQuery {
                text: text_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::DecodeBase64UrlResult>::zeroed();
            let func = self.api.DecodeBase64Url.expect("DecodeBase64Url function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.decodedLength == 0 || result.decoded.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.decoded, result.decodedLength as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn encode_base64_url(&self, text: &[u8]) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::EncodeBase64UrlQuery {
                text: text.as_ptr(),
                textLength: text.len() as u32,
            };
            let mut result = MaybeUninit::<sys::EncodeBase64UrlResult>::zeroed();
            let func = self.api.EncodeBase64Url.expect("EncodeBase64Url function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.encoded.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.encoded).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn is_valid_base64_url(&self, text: &str) -> Result<bool, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::IsValidBase64UrlQuery {
                text: text_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::IsValidBase64UrlResult>::zeroed();
            let func = self.api.IsValidBase64Url.expect("IsValidBase64Url function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.valid
            })
        }
    }

}

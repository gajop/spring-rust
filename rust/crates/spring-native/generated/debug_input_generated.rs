impl<'a> DebugInput<'a> {
    pub fn emulate_key(&self, key_code: i32, pressed: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::EmulateKeyQuery {
                keyCode: key_code,
                pressed,
            };
            let mut result = MaybeUninit::<sys::EmulateKeyResult>::zeroed();
            let func = self.api.EmulateKey.expect("EmulateKey function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn emulate_mouse_button(&self, button: i32, pressed: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::EmulateMouseButtonQuery {
                button,
                pressed,
            };
            let mut result = MaybeUninit::<sys::EmulateMouseButtonResult>::zeroed();
            let func = self.api.EmulateMouseButton.expect("EmulateMouseButton function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn emulate_mouse_move(&self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::EmulateMouseMoveQuery {
                x,
                y,
            };
            let mut result = MaybeUninit::<sys::EmulateMouseMoveResult>::zeroed();
            let func = self.api.EmulateMouseMove.expect("EmulateMouseMove function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn emulate_mouse_wheel(&self, delta: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::EmulateMouseWheelQuery {
                delta,
            };
            let mut result = MaybeUninit::<sys::EmulateMouseWheelResult>::zeroed();
            let func = self.api.EmulateMouseWheel.expect("EmulateMouseWheel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn emulate_text_input(&self, utf8_text: &str) -> Result<bool, Error> {
        unsafe {
            let utf8_text_cstr = std::ffi::CString::new(utf8_text).map_err(|_| Error::invalid_argument("utf8_text"))?;
            let query = sys::EmulateTextInputQuery {
                utf8Text: utf8_text_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::EmulateTextInputResult>::zeroed();
            let func = self.api.EmulateTextInput.expect("EmulateTextInput function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.consumed
            })
        }
    }

    pub fn emulate_text_editing(&self, utf8_text: &str, start: u32, length: u32) -> Result<bool, Error> {
        unsafe {
            let utf8_text_cstr = std::ffi::CString::new(utf8_text).map_err(|_| Error::invalid_argument("utf8_text"))?;
            let query = sys::EmulateTextEditingQuery {
                utf8Text: utf8_text_cstr.as_ptr(),
                start,
                length,
            };
            let mut result = MaybeUninit::<sys::EmulateTextEditingResult>::zeroed();
            let func = self.api.EmulateTextEditing.expect("EmulateTextEditing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.consumed
            })
        }
    }

    pub fn clear_emulated_input(&self, fire_releases: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::ClearEmulatedInputQuery {
                fireReleases: fire_releases,
            };
            let mut result = MaybeUninit::<sys::ClearEmulatedInputResult>::zeroed();
            let func = self.api.ClearEmulatedInput.expect("ClearEmulatedInput function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

}

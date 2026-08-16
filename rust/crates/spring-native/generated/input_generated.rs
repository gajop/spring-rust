impl<'a> Input<'a> {
    pub fn get_mouse_state(&self) -> Result<sys::MouseState, Error> {
        unsafe {
            let query = sys::GetMouseStateQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMouseStateResult>::zeroed();
            let func = self.api.GetMouseState.expect("GetMouseState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.state
            })
        }
    }

    pub fn get_mouse_cursor(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetMouseCursorQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMouseCursorResult>::zeroed();
            let func = self.api.GetMouseCursor.expect("GetMouseCursor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.cursor.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.cursor).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_mouse_buttons_pressed(&self, buttons: &[i32]) -> Result<Vec<bool>, Error> {
        unsafe {
            let query = sys::GetMouseButtonsPressedQuery {
                buttons: buttons.as_ptr(),
                count: buttons.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GetMouseButtonsPressedResult>::zeroed();
            let func = self.api.GetMouseButtonsPressed.expect("GetMouseButtonsPressed function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.pressed.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.pressed as *const bool, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_mouse_start_position(&self, button: i32) -> Result<(sys::Float2, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::GetMouseStartPositionQuery {
                button: button,
            };
            let mut result = MaybeUninit::<sys::GetMouseStartPositionResult>::zeroed();
            let func = self.api.GetMouseStartPosition.expect("GetMouseStartPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.position,
                result.camPos,
                result.dir,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_key_state(&self, key_code: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetKeyStateQuery {
                keyCode: key_code,
            };
            let mut result = MaybeUninit::<sys::GetKeyStateResult>::zeroed();
            let func = self.api.GetKeyState.expect("GetKeyState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.pressed
            })
        }
    }

    pub fn get_pressed_keys(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetPressedKeysQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetPressedKeysResult>::zeroed();
            let func = self.api.GetPressedKeys.expect("GetPressedKeys function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.keys.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.keys as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_pressed_scans(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetPressedScansQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetPressedScansResult>::zeroed();
            let func = self.api.GetPressedScans.expect("GetPressedScans function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.scans.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.scans as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_mod_key_state(&self) -> Result<(bool, bool, bool, bool), Error> {
        unsafe {
            let query = sys::GetModKeyStateQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetModKeyStateResult>::zeroed();
            let func = self.api.GetModKeyState.expect("GetModKeyState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.alt,
                result.ctrl,
                result.meta,
                result.shift,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_selection_box(&self) -> Result<sys::SelectionBox, Error> {
        unsafe {
            let query = sys::GetSelectionBoxQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSelectionBoxResult>::zeroed();
            let func = self.api.GetSelectionBox.expect("GetSelectionBox function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.box_
            })
        }
    }

    pub fn get_invert_queue_key(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetInvertQueueKeyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetInvertQueueKeyResult>::zeroed();
            let func = self.api.GetInvertQueueKey.expect("GetInvertQueueKey function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.invert
            })
        }
    }

    pub fn is_above_mini_map(&self, screen_x: f32, screen_y: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsAboveMiniMapQuery {
                screenX: screen_x,
                screenY: screen_y,
            };
            let mut result = MaybeUninit::<sys::IsAboveMiniMapResult>::zeroed();
            let func = self.api.IsAboveMiniMap.expect("IsAboveMiniMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.above
            })
        }
    }

    pub fn get_active_command(&self) -> Result<(i32, i32, i32, Option<String>), Error> {
        unsafe {
            let query = sys::GetActiveCommandQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetActiveCommandResult>::zeroed();
            let func = self.api.GetActiveCommand.expect("GetActiveCommand function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.commandIndex,
                result.commandID,
                result.commandType,
                {
                    if result.commandName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.commandName).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_action_hot_keys(&self, action: &str) -> Result<Vec<String>, Error> {
        unsafe {
            let action_cstr = std::ffi::CString::new(action).map_err(|_| Error::invalid_argument("action"))?;
            let query = sys::GetActionHotKeysQuery {
                action: action_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetActionHotKeysResult>::zeroed();
            let func = self.api.GetActionHotKeys.expect("GetActionHotKeys function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.hotkeys.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.hotkeys, result.count as usize);
                        slice.iter().map(|&ptr| {
                            if ptr.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(ptr).to_string_lossy().into_owned()
                            }
                        }).collect()
                    }
                }
            })
        }
    }

    pub fn get_key_bindings(&self, key_set1: &str, key_set2: &str) -> Result<Vec<sys::KeyBindingEntry>, Error> {
        unsafe {
            let key_set1_cstr = std::ffi::CString::new(key_set1).map_err(|_| Error::invalid_argument("key_set1"))?;
            let key_set2_cstr = std::ffi::CString::new(key_set2).map_err(|_| Error::invalid_argument("key_set2"))?;
            let query = sys::GetKeyBindingsQuery {
                keySet1: key_set1_cstr.as_ptr(),
                keySet2: key_set2_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetKeyBindingsResult>::zeroed();
            let func = self.api.GetKeyBindings.expect("GetKeyBindings function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.bindings.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.bindings as *const sys::KeyBindingEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_key_code(&self, key_sym: &str) -> Result<i32, Error> {
        unsafe {
            let key_sym_cstr = std::ffi::CString::new(key_sym).map_err(|_| Error::invalid_argument("key_sym"))?;
            let query = sys::GetKeyCodeQuery {
                keySym: key_sym_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetKeyCodeResult>::zeroed();
            let func = self.api.GetKeyCode.expect("GetKeyCode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.keyCode
            })
        }
    }

    pub fn get_key_symbol(&self, key_code: i32) -> Result<(Option<String>, Option<String>), Error> {
        unsafe {
            let query = sys::GetKeySymbolQuery {
                keyCode: key_code,
            };
            let mut result = MaybeUninit::<sys::GetKeySymbolResult>::zeroed();
            let func = self.api.GetKeySymbol.expect("GetKeySymbol function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.keyCodeName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.keyCodeName).to_string_lossy().into_owned())
                    }
                },
                {
                    if result.keyCodeDefaultName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.keyCodeDefaultName).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_scan_symbol(&self, scan_code: i32) -> Result<(Option<String>, Option<String>), Error> {
        unsafe {
            let query = sys::GetScanSymbolQuery {
                scanCode: scan_code,
            };
            let mut result = MaybeUninit::<sys::GetScanSymbolResult>::zeroed();
            let func = self.api.GetScanSymbol.expect("GetScanSymbol function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.scanCodeName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.scanCodeName).to_string_lossy().into_owned())
                    }
                },
                {
                    if result.scanCodeDefaultName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.scanCodeDefaultName).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_key_from_scan_symbol(&self, scan_symbol: &str) -> Result<Option<String>, Error> {
        unsafe {
            let scan_symbol_cstr = std::ffi::CString::new(scan_symbol).map_err(|_| Error::invalid_argument("scan_symbol"))?;
            let query = sys::GetKeyFromScanSymbolQuery {
                scanSymbol: scan_symbol_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetKeyFromScanSymbolResult>::zeroed();
            let func = self.api.GetKeyFromScanSymbol.expect("GetKeyFromScanSymbol function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.keyName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.keyName).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_active_page(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let query = sys::GetActivePageQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetActivePageResult>::zeroed();
            let func = self.api.GetActivePage.expect("GetActivePage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.activePage,
                result.maxPage,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_default_command(&self) -> Result<(i32, i32, i32, Option<String>), Error> {
        unsafe {
            let query = sys::GetDefaultCommandQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetDefaultCommandResult>::zeroed();
            let func = self.api.GetDefaultCommand.expect("GetDefaultCommand function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.commandIndex,
                result.commandID,
                result.commandType,
                {
                    if result.commandName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.commandName).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

}

#[derive(Debug, Clone, Copy, Default)]
pub struct RmlDocumentShowOptions {
    pub modal: Option<i32>,
    pub focus: Option<i32>,
}

impl From<RmlDocumentShowOptions> for sys::RmlDocumentShowOptions {
    fn from(options: RmlDocumentShowOptions) -> Self {
        sys::RmlDocumentShowOptions {
            modal: options.modal.unwrap_or(0),
            hasModal: options.modal.is_some(),
            focus: options.focus.unwrap_or(0),
            hasFocus: options.focus.is_some(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RmlRegisterEventTypeOptions {
    pub interruptible: bool,
    pub bubbles: bool,
    pub default_phase: Option<i32>,
}

impl From<RmlRegisterEventTypeOptions> for sys::RmlRegisterEventTypeOptions {
    fn from(options: RmlRegisterEventTypeOptions) -> Self {
        sys::RmlRegisterEventTypeOptions {
            interruptible: options.interruptible,
            bubbles: options.bubbles,
            defaultPhase: options.default_phase.unwrap_or(0),
            hasDefaultPhase: options.default_phase.is_some(),
        }
    }
}

impl<'a> RmlUi<'a> {
    pub fn create_context(&self, name: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlCreateContextQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlCreateContextResult>::zeroed();
            let func = self.api.CreateContext.expect("CreateContext function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.contextHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_context(&self, name: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlGetContextQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlGetContextResult>::zeroed();
            let func = self.api.GetContext.expect("GetContext function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.contextHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn remove_context(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlRemoveContextQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlRemoveContextResult>::zeroed();
            let func = self.api.RemoveContext.expect("RemoveContext function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn remove_context_by_name(&self, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlRemoveContextByNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlRemoveContextByNameResult>::zeroed();
            let func = self.api.RemoveContextByName.expect("RemoveContextByName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_debug_context(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlSetDebugContextQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlSetDebugContextResult>::zeroed();
            let func = self.api.SetDebugContext.expect("SetDebugContext function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_debug_context_by_name(&self, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlSetDebugContextByNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlSetDebugContextByNameResult>::zeroed();
            let func = self.api.SetDebugContextByName.expect("SetDebugContextByName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn load_font_face(&self, file_path: &str, fallback: bool, weight: Option<i32>) -> Result<bool, Error> {
        unsafe {
            let file_path_cstr = std::ffi::CString::new(file_path).map_err(|_| Error::invalid_argument("file_path"))?;
            let query = sys::RmlLoadFontFaceQuery {
                filePath: file_path_cstr.as_ptr(),
                fallback: fallback,
                weight: weight.unwrap_or(0),
                hasWeight: weight.is_some(),
            };
            let mut result = MaybeUninit::<sys::RmlLoadFontFaceResult>::zeroed();
            let func = self.api.LoadFontFace.expect("LoadFontFace function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn register_event_type(&self, event_type: &str, options: RmlRegisterEventTypeOptions) -> Result<i32, Error> {
        unsafe {
            let event_type_cstr = std::ffi::CString::new(event_type).map_err(|_| Error::invalid_argument("event_type"))?;
            let query = sys::RmlRegisterEventTypeQuery {
                eventType: event_type_cstr.as_ptr(),
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::RmlRegisterEventTypeResult>::zeroed();
            let func = self.api.RegisterEventType.expect("RegisterEventType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.eventID
            })
        }
    }

    pub fn regiser_event_type(&self, event_type: &str, options: RmlRegisterEventTypeOptions) -> Result<i32, Error> {
        unsafe {
            let event_type_cstr = std::ffi::CString::new(event_type).map_err(|_| Error::invalid_argument("event_type"))?;
            let query = sys::RmlRegisterEventTypeQuery {
                eventType: event_type_cstr.as_ptr(),
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::RmlRegisterEventTypeResult>::zeroed();
            let func = self.api.RegiserEventType.expect("RegiserEventType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.eventID
            })
        }
    }

    pub fn add_translation_string(&self, key: &str, translation: &str) -> Result<bool, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let translation_cstr = std::ffi::CString::new(translation).map_err(|_| Error::invalid_argument("translation"))?;
            let query = sys::RmlAddTranslationStringQuery {
                key: key_cstr.as_ptr(),
                translation: translation_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlAddTranslationStringResult>::zeroed();
            let func = self.api.AddTranslationString.expect("AddTranslationString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn clear_translations(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlClearTranslationsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::RmlClearTranslationsResult>::zeroed();
            let func = self.api.ClearTranslations.expect("ClearTranslations function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_mouse_cursor_alias(&self, rml_name: &str, recoil_name: &str) -> Result<bool, Error> {
        unsafe {
            let rml_name_cstr = std::ffi::CString::new(rml_name).map_err(|_| Error::invalid_argument("rml_name"))?;
            let recoil_name_cstr = std::ffi::CString::new(recoil_name).map_err(|_| Error::invalid_argument("recoil_name"))?;
            let query = sys::RmlSetMouseCursorAliasQuery {
                rmlName: rml_name_cstr.as_ptr(),
                recoilName: recoil_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlSetMouseCursorAliasResult>::zeroed();
            let func = self.api.SetMouseCursorAlias.expect("SetMouseCursorAlias function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_version(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlGetVersionQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::RmlGetVersionResult>::zeroed();
            let func = self.api.GetVersion.expect("GetVersion function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.version.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.version).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn is_ready(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlIsReadyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::RmlIsReadyResult>::zeroed();
            let func = self.api.IsReady.expect("IsReady function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.ready
            })
        }
    }

    pub fn context_create_document(&self, context_handle: u64, tag: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let tag_cstr = std::ffi::CString::new(tag).map_err(|_| Error::invalid_argument("tag"))?;
            let query = sys::RmlContextCreateDocumentQuery {
                contextHandle: context_handle,
                tag: tag_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextCreateDocumentResult>::zeroed();
            let func = self.api.ContextCreateDocument.expect("ContextCreateDocument function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.documentHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_load_document(&self, context_handle: u64, document_path: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let document_path_cstr = std::ffi::CString::new(document_path).map_err(|_| Error::invalid_argument("document_path"))?;
            let query = sys::RmlContextLoadDocumentQuery {
                contextHandle: context_handle,
                documentPath: document_path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextLoadDocumentResult>::zeroed();
            let func = self.api.ContextLoadDocument.expect("ContextLoadDocument function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.documentHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_get_document(&self, context_handle: u64, name: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlContextGetDocumentQuery {
                contextHandle: context_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextGetDocumentResult>::zeroed();
            let func = self.api.ContextGetDocument.expect("ContextGetDocument function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.documentHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_add_event_listener<F: FnMut() + 'static>(&self, context_handle: u64, event: &str, in_capture_phase: bool, callback: F) -> Result<(u64, bool), Error> {
        unsafe {
            let event_cstr = std::ffi::CString::new(event).map_err(|_| Error::invalid_argument("event"))?;
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = &mut *(user_data as *mut F);
                f();
            }
            unsafe extern "C" fn destroy_callback<F>(user_data: *mut std::ffi::c_void) {
                drop(Box::from_raw(user_data as *mut F));
            }
            let callback_user_data = Box::into_raw(Box::new(callback));
            let query = sys::RmlContextEventListenerCallbackQuery {
                contextHandle: context_handle,
                event: event_cstr.as_ptr(),
                inCapturePhase: in_capture_phase,
                callback: Some(trampoline::<F>),
                userData: callback_user_data as *mut std::ffi::c_void,
                destroyCallback: Some(destroy_callback::<F>),
            };
            let mut result = MaybeUninit::<sys::RmlEventListenerCallbackResult>::zeroed();
            let func = self.api.ContextAddEventListener.expect("ContextAddEventListener function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            if !result.success || !result.error.is_null() { drop(Box::from_raw(callback_user_data)); }
            let value = (
                result.eventListenerHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_update(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextUpdate.expect("ContextUpdate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_render(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextRender.expect("ContextRender function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_unload_all_documents(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextUnloadAllDocuments.expect("ContextUnloadAllDocuments function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_unload_document(&self, context_handle: u64, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextDocumentQuery {
                contextHandle: context_handle,
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextUnloadDocument.expect("ContextUnloadDocument function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_open_data_model(&self, context_handle: u64, name: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlContextOpenDataModelQuery {
                contextHandle: context_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextOpenDataModelResult>::zeroed();
            let func = self.api.ContextOpenDataModel.expect("ContextOpenDataModel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.dataModelHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_create_data_model(&self, context_handle: u64, name: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlContextCreateDataModelQuery {
                contextHandle: context_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextOpenDataModelResult>::zeroed();
            let func = self.api.ContextCreateDataModel.expect("ContextCreateDataModel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.dataModelHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_remove_data_model(&self, context_handle: u64, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlContextStringQuery {
                contextHandle: context_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextRemoveDataModel.expect("ContextRemoveDataModel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_mouse_move(&self, context_handle: u64, x: f32, y: f32, key_modifier_state: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextMouseMoveQuery {
                contextHandle: context_handle,
                x: x,
                y: y,
                keyModifierState: key_modifier_state,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessMouseMove.expect("ContextProcessMouseMove function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_mouse_button_down(&self, context_handle: u64, button: i32, key_modifier_state: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextMouseButtonQuery {
                contextHandle: context_handle,
                button: button,
                keyModifierState: key_modifier_state,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessMouseButtonDown.expect("ContextProcessMouseButtonDown function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_mouse_button_up(&self, context_handle: u64, button: i32, key_modifier_state: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextMouseButtonQuery {
                contextHandle: context_handle,
                button: button,
                keyModifierState: key_modifier_state,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessMouseButtonUp.expect("ContextProcessMouseButtonUp function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_mouse_wheel(&self, context_handle: u64, x: f32, y: f32, key_modifier_state: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextMouseWheelQuery {
                contextHandle: context_handle,
                x: x,
                y: y,
                keyModifierState: key_modifier_state,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessMouseWheel.expect("ContextProcessMouseWheel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_mouse_leave(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessMouseLeave.expect("ContextProcessMouseLeave function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_is_mouse_interacting(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextIsMouseInteracting.expect("ContextIsMouseInteracting function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_key_down(&self, context_handle: u64, key: i32, key_modifier_state: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextKeyQuery {
                contextHandle: context_handle,
                key: key,
                keyModifierState: key_modifier_state,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessKeyDown.expect("ContextProcessKeyDown function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_key_up(&self, context_handle: u64, key: i32, key_modifier_state: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextKeyQuery {
                contextHandle: context_handle,
                key: key,
                keyModifierState: key_modifier_state,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessKeyUp.expect("ContextProcessKeyUp function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_process_text_input(&self, context_handle: u64, text: &str) -> Result<bool, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::RmlContextTextInputQuery {
                contextHandle: context_handle,
                text: text_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextProcessTextInput.expect("ContextProcessTextInput function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_enable_mouse_cursor(&self, context_handle: u64, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextBoolQuery {
                contextHandle: context_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextEnableMouseCursor.expect("ContextEnableMouseCursor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_activate_theme(&self, context_handle: u64, name: &str, value: bool) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlContextStringBoolQuery {
                contextHandle: context_handle,
                name: name_cstr.as_ptr(),
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextActivateTheme.expect("ContextActivateTheme function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_is_theme_active(&self, context_handle: u64, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlContextStringQuery {
                contextHandle: context_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextIsThemeActive.expect("ContextIsThemeActive function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_get_element_at_point(&self, context_handle: u64, x: f32, y: f32, ignore_element_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlContextGetElementAtPointQuery {
                contextHandle: context_handle,
                x: x,
                y: y,
                ignoreElementHandle: ignore_element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextGetElementAtPointResult>::zeroed();
            let func = self.api.ContextGetElementAtPoint.expect("ContextGetElementAtPoint function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_pull_document_to_front(&self, context_handle: u64, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextDocumentQuery {
                contextHandle: context_handle,
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextPullDocumentToFront.expect("ContextPullDocumentToFront function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_push_document_to_back(&self, context_handle: u64, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextDocumentQuery {
                contextHandle: context_handle,
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextPushDocumentToBack.expect("ContextPushDocumentToBack function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_get_name(&self, context_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextGetNameResult>::zeroed();
            let func = self.api.ContextGetName.expect("ContextGetName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.name.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.name).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn context_get_dimensions(&self, context_handle: u64) -> Result<(i32, i32), Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextGetDimensionsResult>::zeroed();
            let func = self.api.ContextGetDimensions.expect("ContextGetDimensions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.x,
                result.y,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_set_dimensions(&self, context_handle: u64, x: i32, y: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextSetDimensionsQuery {
                contextHandle: context_handle,
                x: x,
                y: y,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextSetDimensions.expect("ContextSetDimensions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_get_density_independent_pixel_ratio(&self, context_handle: u64) -> Result<f32, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextGetFloatResult>::zeroed();
            let func = self.api.ContextGetDensityIndependentPixelRatio.expect("ContextGetDensityIndependentPixelRatio function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn context_set_density_independent_pixel_ratio(&self, context_handle: u64, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextSetFloatQuery {
                contextHandle: context_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextSetDensityIndependentPixelRatio.expect("ContextSetDensityIndependentPixelRatio function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_get_focus_element(&self, context_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextGetElementResult>::zeroed();
            let func = self.api.ContextGetFocusElement.expect("ContextGetFocusElement function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_get_hover_element(&self, context_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextGetElementResult>::zeroed();
            let func = self.api.ContextGetHoverElement.expect("ContextGetHoverElement function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_get_root_element(&self, context_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextGetElementResult>::zeroed();
            let func = self.api.ContextGetRootElement.expect("ContextGetRootElement function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn document_pull_to_front(&self, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentPullToFront.expect("DocumentPullToFront function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_push_to_back(&self, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentPushToBack.expect("DocumentPushToBack function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_show(&self, document_handle: u64, options: RmlDocumentShowOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentShowQuery {
                documentHandle: document_handle,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentShow.expect("DocumentShow function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_hide(&self, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentHide.expect("DocumentHide function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_close(&self, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentClose.expect("DocumentClose function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_create_element(&self, document_handle: u64, tag_name: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let tag_name_cstr = std::ffi::CString::new(tag_name).map_err(|_| Error::invalid_argument("tag_name"))?;
            let query = sys::RmlDocumentCreateElementQuery {
                documentHandle: document_handle,
                tagName: tag_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlDocumentCreateElementResult>::zeroed();
            let func = self.api.DocumentCreateElement.expect("DocumentCreateElement function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementPtrHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn document_create_text_node(&self, document_handle: u64, value: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlDocumentStringQuery {
                documentHandle: document_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlDocumentCreateElementResult>::zeroed();
            let func = self.api.DocumentCreateTextNode.expect("DocumentCreateTextNode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementPtrHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn document_reload_style_sheet(&self, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentReloadStyleSheet.expect("DocumentReloadStyleSheet function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_load_inline_script(&self, document_handle: u64, content: &str, source_path: &str, source_line: i32) -> Result<bool, Error> {
        unsafe {
            let content_cstr = std::ffi::CString::new(content).map_err(|_| Error::invalid_argument("content"))?;
            let source_path_cstr = std::ffi::CString::new(source_path).map_err(|_| Error::invalid_argument("source_path"))?;
            let query = sys::RmlDocumentInlineScriptQuery {
                documentHandle: document_handle,
                content: content_cstr.as_ptr(),
                sourcePath: source_path_cstr.as_ptr(),
                sourceLine: source_line,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentLoadInlineScript.expect("DocumentLoadInlineScript function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_load_external_script(&self, document_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlDocumentStringQuery {
                documentHandle: document_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentLoadExternalScript.expect("DocumentLoadExternalScript function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_update_document(&self, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentUpdateDocument.expect("DocumentUpdateDocument function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_append_to_style_sheet(&self, document_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlDocumentStringQuery {
                documentHandle: document_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentAppendToStyleSheet.expect("DocumentAppendToStyleSheet function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_get_title(&self, document_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentGetStringResult>::zeroed();
            let func = self.api.DocumentGetTitle.expect("DocumentGetTitle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn document_set_title(&self, document_handle: u64, title: &str) -> Result<bool, Error> {
        unsafe {
            let title_cstr = std::ffi::CString::new(title).map_err(|_| Error::invalid_argument("title"))?;
            let query = sys::RmlDocumentSetTitleQuery {
                documentHandle: document_handle,
                title: title_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentSetTitle.expect("DocumentSetTitle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn document_get_context(&self, document_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentGetContextResult>::zeroed();
            let func = self.api.DocumentGetContext.expect("DocumentGetContext function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.contextHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn document_get_url(&self, document_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentGetStringResult>::zeroed();
            let func = self.api.DocumentGetUrl.expect("DocumentGetUrl function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn document_is_modal(&self, document_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDocumentHandleQuery {
                documentHandle: document_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentBoolResult>::zeroed();
            let func = self.api.DocumentIsModal.expect("DocumentIsModal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_append_child(&self, element_handle: u64, element_ptr_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlElementAppendChildQuery {
                elementHandle: element_handle,
                elementPtrHandle: element_ptr_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.ElementAppendChild.expect("ElementAppendChild function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_add_event_listener<F: FnMut() + 'static>(&self, element_handle: u64, event: &str, in_capture_phase: bool, callback: F) -> Result<(u64, bool), Error> {
        unsafe {
            let event_cstr = std::ffi::CString::new(event).map_err(|_| Error::invalid_argument("event"))?;
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = &mut *(user_data as *mut F);
                f();
            }
            unsafe extern "C" fn destroy_callback<F>(user_data: *mut std::ffi::c_void) {
                drop(Box::from_raw(user_data as *mut F));
            }
            let callback_user_data = Box::into_raw(Box::new(callback));
            let query = sys::RmlEventListenerCallbackQuery {
                elementHandle: element_handle,
                event: event_cstr.as_ptr(),
                inCapturePhase: in_capture_phase,
                callback: Some(trampoline::<F>),
                userData: callback_user_data as *mut std::ffi::c_void,
                destroyCallback: Some(destroy_callback::<F>),
            };
            let mut result = MaybeUninit::<sys::RmlEventListenerCallbackResult>::zeroed();
            let func = self.api.ElementAddEventListener.expect("ElementAddEventListener function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            if !result.success || !result.error.is_null() { drop(Box::from_raw(callback_user_data)); }
            let value = (
                result.eventListenerHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_blur(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementBlur.expect("ElementBlur function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_click(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementClick.expect("ElementClick function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_focus(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFocus.expect("ElementFocus function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_attribute(&self, element_handle: u64, name: &str) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlElementGetAttributeQuery {
                elementHandle: element_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementGetAttributeResult>::zeroed();
            let func = self.api.ElementGetAttribute.expect("ElementGetAttribute function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                },
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_get_element_by_id(&self, element_handle: u64, value: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.ElementGetElementById.expect("ElementGetElementById function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_query_selector(&self, element_handle: u64, value: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.ElementQuerySelector.expect("ElementQuerySelector function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_has_attribute(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementHasAttribute.expect("ElementHasAttribute function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_has_child_nodes(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementHasChildNodes.expect("ElementHasChildNodes function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_insert_before(&self, element_handle: u64, element_ptr_handle: u64, adjacent_element_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlElementInsertBeforeQuery {
                elementHandle: element_handle,
                elementPtrHandle: element_ptr_handle,
                adjacentElementHandle: adjacent_element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.ElementInsertBefore.expect("ElementInsertBefore function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_is_class_set(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementIsClassSet.expect("ElementIsClassSet function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_remove_attribute(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementRemoveAttribute.expect("ElementRemoveAttribute function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_remove_child(&self, element_handle: u64, child_element_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlElementChildQuery {
                elementHandle: element_handle,
                childElementHandle: child_element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentCreateElementResult>::zeroed();
            let func = self.api.ElementRemoveChild.expect("ElementRemoveChild function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementPtrHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_replace_child(&self, element_handle: u64, element_ptr_handle: u64, replaced_element_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlElementReplaceChildQuery {
                elementHandle: element_handle,
                elementPtrHandle: element_ptr_handle,
                replacedElementHandle: replaced_element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentCreateElementResult>::zeroed();
            let func = self.api.ElementReplaceChild.expect("ElementReplaceChild function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementPtrHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_scroll_into_view(&self, element_handle: u64, align_with_top: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementScrollIntoViewQuery {
                elementHandle: element_handle,
                alignWithTop: align_with_top,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementScrollIntoView.expect("ElementScrollIntoView function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_set_attribute(&self, element_handle: u64, name: &str, value: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementSetAttributeQuery {
                elementHandle: element_handle,
                name: name_cstr.as_ptr(),
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetAttribute.expect("ElementSetAttribute function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_set_class(&self, element_handle: u64, name: &str, value: bool) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlElementStringBoolQuery {
                elementHandle: element_handle,
                name: name_cstr.as_ptr(),
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetClass.expect("ElementSetClass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_elements_by_class_name_count(&self, element_handle: u64, value: &str) -> Result<i32, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementGetIntResult>::zeroed();
            let func = self.api.ElementGetElementsByClassNameCount.expect("ElementGetElementsByClassNameCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn element_get_elements_by_tag_name_count(&self, element_handle: u64, value: &str) -> Result<i32, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementGetIntResult>::zeroed();
            let func = self.api.ElementGetElementsByTagNameCount.expect("ElementGetElementsByTagNameCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn element_query_selector_all_count(&self, element_handle: u64, value: &str) -> Result<i32, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementGetIntResult>::zeroed();
            let func = self.api.ElementQuerySelectorAllCount.expect("ElementQuerySelectorAllCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn element_get_elements_by_class_name(&self, element_handle: u64, value: &str) -> Result<Vec<u64>, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementHandleListResult>::zeroed();
            let func = self.api.ElementGetElementsByClassName.expect("ElementGetElementsByClassName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.elementHandleCount == 0 || result.elementHandles.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.elementHandles as *const u64, result.elementHandleCount as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn element_get_elements_by_tag_name(&self, element_handle: u64, value: &str) -> Result<Vec<u64>, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementHandleListResult>::zeroed();
            let func = self.api.ElementGetElementsByTagName.expect("ElementGetElementsByTagName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.elementHandleCount == 0 || result.elementHandles.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.elementHandles as *const u64, result.elementHandleCount as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn element_query_selector_all(&self, element_handle: u64, value: &str) -> Result<Vec<u64>, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementHandleListResult>::zeroed();
            let func = self.api.ElementQuerySelectorAll.expect("ElementQuerySelectorAll function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.elementHandleCount == 0 || result.elementHandles.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.elementHandles as *const u64, result.elementHandleCount as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn element_clone(&self, element_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDocumentCreateElementResult>::zeroed();
            let func = self.api.ElementClone.expect("ElementClone function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementPtrHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_closest(&self, element_handle: u64, value: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.ElementClosest.expect("ElementClosest function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_set_pseudo_class(&self, element_handle: u64, name: &str, value: bool) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlElementStringBoolQuery {
                elementHandle: element_handle,
                name: name_cstr.as_ptr(),
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetPseudoClass.expect("ElementSetPseudoClass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_is_pseudo_class_set(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementIsPseudoClassSet.expect("ElementIsPseudoClassSet function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_are_pseudo_classes_set(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementArePseudoClassesSet.expect("ElementArePseudoClassesSet function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_active_pseudo_classes(&self, element_handle: u64) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementStringListResult>::zeroed();
            let func = self.api.ElementGetActivePseudoClasses.expect("ElementGetActivePseudoClasses function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.valueCount == 0 || result.values.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.values, result.valueCount as usize);
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

    pub fn element_is_point_within_element(&self, element_handle: u64, x: f32, y: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementPointQuery {
                elementHandle: element_handle,
                x: x,
                y: y,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementIsPointWithinElement.expect("ElementIsPointWithinElement function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_rect(&self, element_handle: u64) -> Result<(f32, f32, f32, f32), Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetRectResult>::zeroed();
            let func = self.api.ElementGetRect.expect("ElementGetRect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.left,
                result.top,
                result.width,
                result.height,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_matches(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementGetByStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementMatches.expect("ElementMatches function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_dispatch_event(&self, element_handle: u64, event: &str) -> Result<bool, Error> {
        unsafe {
            let event_cstr = std::ffi::CString::new(event).map_err(|_| Error::invalid_argument("event"))?;
            let query = sys::RmlElementDispatchEventQuery {
                elementHandle: element_handle,
                event: event_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementDispatchEvent.expect("ElementDispatchEvent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_process_default_action(&self, element_handle: u64, event_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementProcessDefaultActionQuery {
                elementHandle: element_handle,
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementProcessDefaultAction.expect("ElementProcessDefaultAction function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_value(&self, element_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetStringResult>::zeroed();
            let func = self.api.ElementGetValue.expect("ElementGetValue function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn element_get_child(&self, element_handle: u64, index: i32) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlElementGetChildQuery {
                elementHandle: element_handle,
                index: index,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.ElementGetChild.expect("ElementGetChild function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_get_class_name(&self, element_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetStringResult>::zeroed();
            let func = self.api.ElementGetClassName.expect("ElementGetClassName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn element_set_class_name(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementSetStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetClassName.expect("ElementSetClassName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_id(&self, element_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetStringResult>::zeroed();
            let func = self.api.ElementGetId.expect("ElementGetId function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn element_set_id(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementSetStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetId.expect("ElementSetId function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_inner_rml(&self, element_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetStringResult>::zeroed();
            let func = self.api.ElementGetInnerRml.expect("ElementGetInnerRml function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn element_set_inner_rml(&self, element_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementSetStringQuery {
                elementHandle: element_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetInnerRml.expect("ElementSetInnerRml function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_scroll_left(&self, element_handle: u64) -> Result<i32, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetIntResult>::zeroed();
            let func = self.api.ElementGetScrollLeft.expect("ElementGetScrollLeft function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn element_set_scroll_left(&self, element_handle: u64, value: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementSetIntQuery {
                elementHandle: element_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetScrollLeft.expect("ElementSetScrollLeft function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_scroll_top(&self, element_handle: u64) -> Result<i32, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetIntResult>::zeroed();
            let func = self.api.ElementGetScrollTop.expect("ElementGetScrollTop function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn element_set_scroll_top(&self, element_handle: u64, value: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementSetIntQuery {
                elementHandle: element_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementSetScrollTop.expect("ElementSetScrollTop function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_get_tag_name(&self, element_handle: u64) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetStringResult>::zeroed();
            let func = self.api.ElementGetTagName.expect("ElementGetTagName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn element_is_visible(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementIsVisible.expect("ElementIsVisible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_submit(&self, element_handle: u64, name: &str, value: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlElementFormSubmitQuery {
                elementHandle: element_handle,
                name: name_cstr.as_ptr(),
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormSubmit.expect("ElementFormSubmit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_select_add(&self, element_handle: u64, element_ptr_handle: u64, before: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementFormControlSelectAddQuery {
                elementHandle: element_handle,
                elementPtrHandle: element_ptr_handle,
                before: before,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormControlSelectAdd.expect("ElementFormControlSelectAdd function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_select_remove(&self, element_handle: u64, index: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementFormControlSelectRemoveQuery {
                elementHandle: element_handle,
                index: index,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormControlSelectRemove.expect("ElementFormControlSelectRemove function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_select_remove_all(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormControlSelectRemoveAll.expect("ElementFormControlSelectRemoveAll function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_input_select(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormControlInputSelect.expect("ElementFormControlInputSelect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_input_set_selection(&self, element_handle: u64, start: i32, end: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementFormControlSelectionQuery {
                elementHandle: element_handle,
                start: start,
                end: end,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormControlInputSetSelection.expect("ElementFormControlInputSetSelection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_input_get_selection(&self, element_handle: u64) -> Result<(i32, i32, Option<String>, bool), Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementFormControlSelectionResult>::zeroed();
            let func = self.api.ElementFormControlInputGetSelection.expect("ElementFormControlInputGetSelection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.start,
                result.end,
                {
                    if result.text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.text).to_string_lossy().into_owned())
                    }
                },
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_form_control_text_area_select(&self, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormControlTextAreaSelect.expect("ElementFormControlTextAreaSelect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_text_area_set_selection(&self, element_handle: u64, start: i32, end: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementFormControlSelectionQuery {
                elementHandle: element_handle,
                start: start,
                end: end,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementFormControlTextAreaSetSelection.expect("ElementFormControlTextAreaSetSelection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_form_control_text_area_get_selection(&self, element_handle: u64) -> Result<(i32, i32, Option<String>, bool), Error> {
        unsafe {
            let query = sys::RmlElementHandleQuery {
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementFormControlSelectionResult>::zeroed();
            let func = self.api.ElementFormControlTextAreaGetSelection.expect("ElementFormControlTextAreaGetSelection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.start,
                result.end,
                {
                    if result.text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.text).to_string_lossy().into_owned())
                    }
                },
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn element_tab_set_set_panel(&self, element_handle: u64, index: i32, rml: &str) -> Result<bool, Error> {
        unsafe {
            let rml_cstr = std::ffi::CString::new(rml).map_err(|_| Error::invalid_argument("rml"))?;
            let query = sys::RmlElementTabSetIndexStringQuery {
                elementHandle: element_handle,
                index: index,
                rml: rml_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementTabSetSetPanel.expect("ElementTabSetSetPanel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_tab_set_set_tab(&self, element_handle: u64, index: i32, rml: &str) -> Result<bool, Error> {
        unsafe {
            let rml_cstr = std::ffi::CString::new(rml).map_err(|_| Error::invalid_argument("rml"))?;
            let query = sys::RmlElementTabSetIndexStringQuery {
                elementHandle: element_handle,
                index: index,
                rml: rml_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementTabSetSetTab.expect("ElementTabSetSetTab function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_tab_set_remove_tab(&self, element_handle: u64, index: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlElementTabSetIndexQuery {
                elementHandle: element_handle,
                index: index,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementTabSetRemoveTab.expect("ElementTabSetRemoveTab function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn event_listener_on_attach(&self, event_listener_handle: u64, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlEventListenerElementQuery {
                eventListenerHandle: event_listener_handle,
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.EventListenerOnAttach.expect("EventListenerOnAttach function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn event_listener_on_detach(&self, event_listener_handle: u64, element_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlEventListenerElementQuery {
                eventListenerHandle: event_listener_handle,
                elementHandle: element_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.EventListenerOnDetach.expect("EventListenerOnDetach function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn event_listener_process_event(&self, event_listener_handle: u64, event_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlEventListenerEventQuery {
                eventListenerHandle: event_listener_handle,
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.EventListenerProcessEvent.expect("EventListenerProcessEvent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn event_get_current(&self) -> Result<(u64, u64, u64, bool), Error> {
        unsafe {
            let query = sys::RmlEventCurrentQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::RmlEventCurrentResult>::zeroed();
            let func = self.api.EventGetCurrent.expect("EventGetCurrent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.eventHandle,
                result.elementHandle,
                result.documentHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_stop_propagation(&self, event_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.EventStopPropagation.expect("EventStopPropagation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn event_stop_immediate_propagation(&self, event_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.EventStopImmediatePropagation.expect("EventStopImmediatePropagation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn event_get_current_element(&self, event_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.EventGetCurrentElement.expect("EventGetCurrentElement function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_target_element(&self, event_handle: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlElementGetElementResult>::zeroed();
            let func = self.api.EventGetTargetElement.expect("EventGetTargetElement function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementHandle,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_type(&self, event_handle: u64) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlEventGetStringResult>::zeroed();
            let func = self.api.EventGetType.expect("EventGetType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                },
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_phase(&self, event_handle: u64) -> Result<(i32, bool), Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlEventGetIntResult>::zeroed();
            let func = self.api.EventGetPhase.expect("EventGetPhase function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_is_interruptible(&self, event_handle: u64) -> Result<(bool, bool), Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlEventGetBoolResult>::zeroed();
            let func = self.api.EventIsInterruptible.expect("EventIsInterruptible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_is_propagating(&self, event_handle: u64) -> Result<(bool, bool), Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlEventGetBoolResult>::zeroed();
            let func = self.api.EventIsPropagating.expect("EventIsPropagating function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_is_immediate_propagating(&self, event_handle: u64) -> Result<(bool, bool), Error> {
        unsafe {
            let query = sys::RmlEventHandleQuery {
                eventHandle: event_handle,
            };
            let mut result = MaybeUninit::<sys::RmlEventGetBoolResult>::zeroed();
            let func = self.api.EventIsImmediatePropagating.expect("EventIsImmediatePropagating function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_parameter_type(&self, event_handle: u64, name: &str) -> Result<(i32, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlEventParameterQuery {
                eventHandle: event_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlEventGetIntResult>::zeroed();
            let func = self.api.EventGetParameterType.expect("EventGetParameterType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_parameter_bool(&self, event_handle: u64, name: &str) -> Result<(bool, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlEventParameterQuery {
                eventHandle: event_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlEventGetBoolResult>::zeroed();
            let func = self.api.EventGetParameterBool.expect("EventGetParameterBool function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_parameter_int(&self, event_handle: u64, name: &str) -> Result<(i32, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlEventParameterQuery {
                eventHandle: event_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlEventGetIntResult>::zeroed();
            let func = self.api.EventGetParameterInt.expect("EventGetParameterInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_parameter_float(&self, event_handle: u64, name: &str) -> Result<(f32, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlEventParameterQuery {
                eventHandle: event_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlEventGetFloatResult>::zeroed();
            let func = self.api.EventGetParameterFloat.expect("EventGetParameterFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn event_get_parameter_string(&self, event_handle: u64, name: &str) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlEventParameterQuery {
                eventHandle: event_handle,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlEventGetStringResult>::zeroed();
            let func = self.api.EventGetParameterString.expect("EventGetParameterString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                },
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn sol_lua_data_model_set_dirty(&self, data_model_handle: u64, property: &str) -> Result<bool, Error> {
        unsafe {
            let property_cstr = std::ffi::CString::new(property).map_err(|_| Error::invalid_argument("property"))?;
            let query = sys::RmlSolLuaDataModelSetDirtyQuery {
                dataModelHandle: data_model_handle,
                property: property_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.SolLuaDataModelSetDirty.expect("SolLuaDataModelSetDirty function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_bind_bool(&self, data_model_handle: u64, name: &str, initial_value: bool) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlDataModelBindBoolQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                initialValue: initial_value,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelBindResult>::zeroed();
            let func = self.api.DataModelBindBool.expect("DataModelBindBool function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.variableHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_bind_int(&self, data_model_handle: u64, name: &str, initial_value: i32) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlDataModelBindIntQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                initialValue: initial_value,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelBindResult>::zeroed();
            let func = self.api.DataModelBindInt.expect("DataModelBindInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.variableHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_bind_float(&self, data_model_handle: u64, name: &str, initial_value: f32) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlDataModelBindFloatQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                initialValue: initial_value,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelBindResult>::zeroed();
            let func = self.api.DataModelBindFloat.expect("DataModelBindFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.variableHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_bind_string(&self, data_model_handle: u64, name: &str, initial_value: &str) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let initial_value_cstr = std::ffi::CString::new(initial_value).map_err(|_| Error::invalid_argument("initial_value"))?;
            let query = sys::RmlDataModelBindStringQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                initialValue: initial_value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlDataModelBindResult>::zeroed();
            let func = self.api.DataModelBindString.expect("DataModelBindString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.variableHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_set_bool(&self, variable_handle: u64, value: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDataModelVariableBoolQuery {
                variableHandle: variable_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetBool.expect("DataModelSetBool function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_set_int(&self, variable_handle: u64, value: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDataModelVariableIntQuery {
                variableHandle: variable_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetInt.expect("DataModelSetInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_set_float(&self, variable_handle: u64, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDataModelVariableFloatQuery {
                variableHandle: variable_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetFloat.expect("DataModelSetFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_set_string(&self, variable_handle: u64, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::RmlDataModelVariableStringQuery {
                variableHandle: variable_handle,
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetString.expect("DataModelSetString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_get_bool(&self, variable_handle: u64) -> Result<(bool, bool), Error> {
        unsafe {
            let query = sys::RmlDataModelVariableHandleQuery {
                variableHandle: variable_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelGetBoolResult>::zeroed();
            let func = self.api.DataModelGetBool.expect("DataModelGetBool function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_get_int(&self, variable_handle: u64) -> Result<(i32, bool), Error> {
        unsafe {
            let query = sys::RmlDataModelVariableHandleQuery {
                variableHandle: variable_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelGetIntResult>::zeroed();
            let func = self.api.DataModelGetInt.expect("DataModelGetInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_get_float(&self, variable_handle: u64) -> Result<(f32, bool), Error> {
        unsafe {
            let query = sys::RmlDataModelVariableHandleQuery {
                variableHandle: variable_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelGetFloatResult>::zeroed();
            let func = self.api.DataModelGetFloat.expect("DataModelGetFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_get_string(&self, variable_handle: u64) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let query = sys::RmlDataModelVariableHandleQuery {
                variableHandle: variable_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelGetStringResult>::zeroed();
            let func = self.api.DataModelGetString.expect("DataModelGetString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                },
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn vector2f_new(&self, x: f32, y: f32) -> Result<(f32, f32), Error> {
        unsafe {
            let query = sys::RmlVector2fNewQuery {
                x: x,
                y: y,
            };
            let mut result = MaybeUninit::<sys::RmlVector2fNewResult>::zeroed();
            let func = self.api.Vector2fNew.expect("Vector2fNew function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.x,
                result.y,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn vector2i_new(&self, x: i32, y: i32) -> Result<(i32, i32), Error> {
        unsafe {
            let query = sys::RmlVector2iNewQuery {
                x: x,
                y: y,
            };
            let mut result = MaybeUninit::<sys::RmlVector2iNewResult>::zeroed();
            let func = self.api.Vector2iNew.expect("Vector2iNew function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.x,
                result.y,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_bind_color(&self, data_model_handle: u64, name: &str, red: u8, green: u8, blue: u8, alpha: u8) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlDataModelBindColorQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                red: red,
                green: green,
                blue: blue,
                alpha: alpha,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelBindResult>::zeroed();
            let func = self.api.DataModelBindColor.expect("DataModelBindColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.variableHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_set_color(&self, variable_handle: u64, red: u8, green: u8, blue: u8, alpha: u8) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDataModelVariableColorQuery {
                variableHandle: variable_handle,
                red: red,
                green: green,
                blue: blue,
                alpha: alpha,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetColor.expect("DataModelSetColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_get_color(&self, variable_handle: u64) -> Result<(u8, u8, u8, u8, bool), Error> {
        unsafe {
            let query = sys::RmlDataModelVariableHandleQuery {
                variableHandle: variable_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelGetColorResult>::zeroed();
            let func = self.api.DataModelGetColor.expect("DataModelGetColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.red,
                result.green,
                result.blue,
                result.alpha,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_bind_pixels(&self, data_model_handle: u64, name: &str, initial_value: f32) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlDataModelBindPixelsQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                initialValue: initial_value,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelBindResult>::zeroed();
            let func = self.api.DataModelBindPixels.expect("DataModelBindPixels function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.variableHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_set_pixels(&self, variable_handle: u64, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDataModelVariablePixelsQuery {
                variableHandle: variable_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetPixels.expect("DataModelSetPixels function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_get_pixels(&self, variable_handle: u64) -> Result<(f32, bool), Error> {
        unsafe {
            let query = sys::RmlDataModelVariableHandleQuery {
                variableHandle: variable_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelGetPixelsResult>::zeroed();
            let func = self.api.DataModelGetPixels.expect("DataModelGetPixels function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_bind_percent(&self, data_model_handle: u64, name: &str, initial_value: f32) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlDataModelBindPercentQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                initialValue: initial_value,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelBindResult>::zeroed();
            let func = self.api.DataModelBindPercent.expect("DataModelBindPercent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.variableHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_set_percent(&self, variable_handle: u64, value: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDataModelVariablePercentQuery {
                variableHandle: variable_handle,
                value: value,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetPercent.expect("DataModelSetPercent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_get_percent(&self, variable_handle: u64) -> Result<(f32, bool), Error> {
        unsafe {
            let query = sys::RmlDataModelVariableHandleQuery {
                variableHandle: variable_handle,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelGetPercentResult>::zeroed();
            let func = self.api.DataModelGetPercent.expect("DataModelGetPercent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn context_pull_to_front(&self, context_handle: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextPullToFront.expect("ContextPullToFront function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_set_pointer_capture(&self, context_handle: u64, anchor_x: i32, anchor_y: i32, active: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlContextPointerCaptureQuery {
                contextHandle: context_handle,
                anchorX: anchor_x,
                anchorY: anchor_y,
                active: active,
            };
            let mut result = MaybeUninit::<sys::RmlContextBoolResult>::zeroed();
            let func = self.api.ContextSetPointerCapture.expect("ContextSetPointerCapture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_take_pointer_capture_delta(&self, context_handle: u64) -> Result<(i32, i32, i32), Error> {
        unsafe {
            let query = sys::RmlContextHandleQuery {
                contextHandle: context_handle,
            };
            let mut result = MaybeUninit::<sys::RmlContextPointerDeltaResult>::zeroed();
            let func = self.api.ContextTakePointerCaptureDelta.expect("ContextTakePointerCaptureDelta function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.deltaX,
                result.deltaY,
                result.status,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_document_path_requests(&self, document_path: &str) -> Result<Vec<String>, Error> {
        unsafe {
            let document_path_cstr = std::ffi::CString::new(document_path).map_err(|_| Error::invalid_argument("document_path"))?;
            let query = sys::RmlGetDocumentPathRequestsQuery {
                documentPath: document_path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlGetDocumentPathRequestsResult>::zeroed();
            let func = self.api.GetDocumentPathRequests.expect("GetDocumentPathRequests function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.paths.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.paths, result.count as usize);
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

    pub fn clear_document_path_requests(&self, document_path: &str) -> Result<bool, Error> {
        unsafe {
            let document_path_cstr = std::ffi::CString::new(document_path).map_err(|_| Error::invalid_argument("document_path"))?;
            let query = sys::RmlClearDocumentPathRequestsQuery {
                documentPath: document_path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RmlClearDocumentPathRequestsResult>::zeroed();
            let func = self.api.ClearDocumentPathRequests.expect("ClearDocumentPathRequests function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn data_model_bind_rows(&self, data_model_handle: u64, name: &str, fields: &sys::RmlDataFieldDef, field_count: u64) -> Result<(u64, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::RmlDataModelBindRowsQuery {
                dataModelHandle: data_model_handle,
                name: name_cstr.as_ptr(),
                fields: fields as *const _,
                fieldCount: field_count,
            };
            let mut result = MaybeUninit::<sys::RmlDataModelRowsResult>::zeroed();
            let func = self.api.DataModelBindRows.expect("DataModelBindRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.rowsHandle,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn data_model_set_rows(&self, rows_handle: u64, values: &sys::RmlDataValue, row_count: u64) -> Result<bool, Error> {
        unsafe {
            let query = sys::RmlDataModelSetRowsQuery {
                rowsHandle: rows_handle,
                values: values as *const _,
                rowCount: row_count,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.DataModelSetRows.expect("DataModelSetRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn context_remove_event_listener(&self, context_handle: u64, event_listener_handle: u64, event: &str, in_capture_phase: bool) -> Result<bool, Error> {
        unsafe {
            let event_cstr = std::ffi::CString::new(event).map_err(|_| Error::invalid_argument("event"))?;
            let query = sys::RmlContextEventListenerRemoveQuery {
                contextHandle: context_handle,
                eventListenerHandle: event_listener_handle,
                event: event_cstr.as_ptr(),
                inCapturePhase: in_capture_phase,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ContextRemoveEventListener.expect("ContextRemoveEventListener function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn element_remove_event_listener(&self, element_handle: u64, event_listener_handle: u64, event: &str, in_capture_phase: bool) -> Result<bool, Error> {
        unsafe {
            let event_cstr = std::ffi::CString::new(event).map_err(|_| Error::invalid_argument("event"))?;
            let query = sys::RmlElementEventListenerRemoveQuery {
                elementHandle: element_handle,
                eventListenerHandle: event_listener_handle,
                event: event_cstr.as_ptr(),
                inCapturePhase: in_capture_phase,
            };
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self.api.ElementRemoveEventListener.expect("ElementRemoveEventListener function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}



#[cfg(feature = "alloc")]
pub use crate::owned::rml_ui::{add_translation_string, clear_document_path_requests, clear_translations, context_activate_theme, context_create_data_model, context_create_document, context_enable_mouse_cursor, context_get_density_independent_pixel_ratio, context_get_dimensions, context_get_document, context_get_element_at_point, context_get_focus_element, context_get_hover_element, context_get_name, context_get_root_element, context_is_mouse_interacting, context_is_theme_active, context_load_document, context_open_data_model, context_process_key_down, context_process_key_up, context_process_mouse_button_down, context_process_mouse_button_up, context_process_mouse_leave, context_process_mouse_move, context_process_mouse_wheel, context_process_text_input, context_pull_document_to_front, context_pull_to_front, context_push_document_to_back, context_remove_data_model, context_remove_event_listener, context_render, context_set_density_independent_pixel_ratio, context_set_dimensions, context_set_pointer_capture, context_take_pointer_capture_delta, context_unload_all_documents, context_unload_document, context_update, create_context, data_model_bind_bool, data_model_bind_color, data_model_bind_float, data_model_bind_int, data_model_bind_percent, data_model_bind_pixels, data_model_bind_rows, data_model_bind_string, data_model_get_bool, data_model_get_color, data_model_get_float, data_model_get_int, data_model_get_percent, data_model_get_pixels, data_model_get_string, data_model_set_bool, data_model_set_color, data_model_set_float, data_model_set_int, data_model_set_percent, data_model_set_pixels, data_model_set_rows, data_model_set_string, document_append_to_style_sheet, document_close, document_create_element, document_create_text_node, document_get_context, document_get_title, document_get_url, document_hide, document_is_modal, document_load_external_script, document_load_inline_script, document_pull_to_front, document_push_to_back, document_reload_style_sheet, document_set_title, document_show, document_update_document, element_append_child, element_are_pseudo_classes_set, element_blur, element_click, element_clone, element_closest, element_dispatch_event, element_focus, element_form_control_input_get_selection, element_form_control_input_select, element_form_control_input_set_selection, element_form_control_select_add, element_form_control_select_remove, element_form_control_select_remove_all, element_form_control_text_area_get_selection, element_form_control_text_area_select, element_form_control_text_area_set_selection, element_form_submit, element_get_active_pseudo_classes, element_get_attribute, element_get_child, element_get_class_name, element_get_element_by_id, element_get_elements_by_class_name, element_get_elements_by_class_name_count, element_get_elements_by_tag_name, element_get_elements_by_tag_name_count, element_get_id, element_get_inner_rml, element_get_rect, element_get_scroll_left, element_get_scroll_top, element_get_tag_name, element_get_value, element_has_attribute, element_has_child_nodes, element_insert_before, element_is_class_set, element_is_point_within_element, element_is_pseudo_class_set, element_is_visible, element_matches, element_process_default_action, element_query_selector, element_query_selector_all, element_query_selector_all_count, element_remove_attribute, element_remove_child, element_remove_event_listener, element_replace_child, element_scroll_into_view, element_set_attribute, element_set_class, element_set_class_name, element_set_id, element_set_inner_rml, element_set_pseudo_class, element_set_scroll_left, element_set_scroll_top, element_tab_set_remove_tab, element_tab_set_set_panel, element_tab_set_set_tab, event_get_current, event_get_current_element, event_get_parameter_bool, event_get_parameter_float, event_get_parameter_int, event_get_parameter_string, event_get_parameter_type, event_get_phase, event_get_target_element, event_get_type, event_is_immediate_propagating, event_is_interruptible, event_is_propagating, event_stop_immediate_propagation, event_stop_propagation, get_context, get_document_path_requests, get_version, is_ready, load_font_face, regiser_event_type, register_event_type, remove_context, remove_context_by_name, set_debug_context, set_debug_context_by_name, set_mouse_cursor_alias, sol_lua_data_model_set_dirty, vector2f_new, vector2i_new};

use super::{ApiError, ErrorCode, Result, RetainedCallback};

#[cfg(feature = "alloc")]
use alloc::ffi::CString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RmlContextCreateResult {
    pub context_handle: u64,
    pub success: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RmlDocumentLoadResult {
    pub document_handle: u64,
    pub success: bool,
}

/// Create or retrieve an RmlUi context through the engine's borrowed-string
/// contract. This keeps transport details out of guest code and avoids the
/// variable-input façade, which is not the ABI shape of this entry point.
#[cfg(feature = "alloc")]
pub fn create_rml_context(name: &str) -> Result<RmlContextCreateResult> {
    let name = CString::new(name).map_err(|_| ApiError::new(ErrorCode::InvalidArgument as i32))?;
    let output = crate::generated::borrowed::rml_ui::create_context(name.as_c_str())?;
    let (context_handle, success) = decode_handle_bool(output)?;
    Ok(RmlContextCreateResult {
        context_handle,
        success,
    })
}

/// Load a document using the engine's borrowed-string contract.
#[cfg(feature = "alloc")]
pub fn load_rml_document(context_handle: u64, path: &str) -> Result<RmlDocumentLoadResult> {
    let path = CString::new(path).map_err(|_| ApiError::new(ErrorCode::InvalidArgument as i32))?;
    let output =
        crate::generated::borrowed::rml_ui::context_load_document(context_handle, path.as_c_str())?;
    let (document_handle, success) = decode_handle_bool(output)?;
    Ok(RmlDocumentLoadResult {
        document_handle,
        success,
    })
}

/// Show a document with the default RmlUi modal/focus options.
#[cfg(target_arch = "wasm32")]
pub fn show_rml_document(document_handle: u64) -> Result<bool> {
    let mut options = [0_u8; 16];
    let packed = unsafe {
        crate::generated::rml_ui::raw::core_document_show(
            document_handle as i64,
            options.as_mut_ptr() as usize as i32,
        )
    } as u64;
    let error_code = (packed >> 32) as i32;
    if error_code != 0 {
        return Err(ApiError::new(error_code));
    }
    Ok((packed as u32) != 0)
}

#[cfg(feature = "alloc")]
fn decode_handle_bool(output: [u8; 16]) -> Result<(u64, bool)> {
    let handle = u64::from_le_bytes(
        output[0..8]
            .try_into()
            .map_err(|_| ApiError::new(ErrorCode::Internal as i32))?,
    );
    let success = match u32::from_le_bytes(
        output[8..12]
            .try_into()
            .map_err(|_| ApiError::new(ErrorCode::Internal as i32))?,
    ) {
        0 => false,
        1 => true,
        _ => return Err(ApiError::new(ErrorCode::Internal as i32)),
    };
    Ok((handle, success))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventListenerRegistration {
    pub handle: u64,
    pub success: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataEventRegistration {
    pub handle: u64,
    pub success: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentDataEvent {
    pub event_handle: u64,
    pub target_element_handle: u64,
    pub value_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataEventValue<'a> {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(&'a str),
    Color([u8; 4]),
    Pixels(f32),
    Percent(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataEventValueError {
    Api(ApiError),
    BufferTooSmall { required: u32 },
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:rml-ui")]
    unsafe extern "C" {
        #[link_name = "context-add-event-listener"]
        pub fn context_add_event_listener(
            context_handle: i64,
            event_ptr: i32,
            event_len: i32,
            in_capture_phase: i32,
            callback_id: i32,
            user_data: i32,
            destroy_callback_id: i32,
            output: i32,
        ) -> i32;
        #[link_name = "element-add-event-listener"]
        pub fn element_add_event_listener(
            element_handle: i64,
            event_ptr: i32,
            event_len: i32,
            in_capture_phase: i32,
            callback_id: i32,
            user_data: i32,
            destroy_callback_id: i32,
            output: i32,
        ) -> i32;
        #[link_name = "data-model-bind-event"]
        pub fn data_model_bind_event(
            data_model_handle: i64,
            name_ptr: i32,
            name_len: i32,
            callback_id: i32,
            user_data: i32,
            destroy_callback_id: i32,
            field_types_ptr: i32,
            field_count: i64,
            output: i32,
        ) -> i32;
        #[link_name = "data-model-unbind-event"]
        pub fn data_model_unbind_event(event_handle: i64) -> i64;
        #[link_name = "event-listener-on-attach"]
        pub fn event_listener_on_attach(listener_handle: i64, element_handle: i64) -> i64;
        #[link_name = "event-listener-on-detach"]
        pub fn event_listener_on_detach(listener_handle: i64, element_handle: i64) -> i64;
        #[link_name = "event-listener-process-event"]
        pub fn event_listener_process_event(listener_handle: i64, event_handle: i64) -> i64;
        #[link_name = "data-model-current-event"]
        pub fn data_model_current_event(output: i32) -> i32;
        #[link_name = "data-model-current-value"]
        pub fn data_model_current_value(
            index: i64,
            output: i32,
            string_output: i32,
            string_capacity: i32,
        ) -> i32;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn bytes_parts(bytes: &[u8]) -> Result<(i32, i32)> {
    let pointer = bytes.as_ptr() as usize;
    if pointer > u32::MAX as usize || bytes.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok((pointer as u32 as i32, bytes.len() as u32 as i32))
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn string_parts(value: &str) -> Result<(i32, i32)> {
    bytes_parts(value.as_bytes())
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn output_ptr<T>(value: &mut T) -> Result<i32> {
    let pointer = value as *mut T as usize;
    if pointer > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok(pointer as u32 as i32)
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn listener_result(status: i32, output: [u64; 2]) -> Result<EventListenerRegistration> {
    if status != 0 {
        return Err(ApiError::new(status));
    }
    match output[1] as u32 {
        0 => Ok(EventListenerRegistration {
            handle: output[0],
            success: false,
        }),
        1 => Ok(EventListenerRegistration {
            handle: output[0],
            success: true,
        }),
        _ => Err(ApiError::new(ErrorCode::Internal as i32)),
    }
}

#[inline]
pub fn context_add_event_listener(
    context_handle: u64,
    event: &str,
    in_capture_phase: bool,
    callback: RetainedCallback,
) -> Result<EventListenerRegistration> {
    #[cfg(target_arch = "wasm32")]
    {
        let (event_ptr, event_len) = string_parts(event)?;
        let mut output = [0u64; 2];
        let pointer = output_ptr(&mut output)?;
        let status = unsafe {
            raw::context_add_event_listener(
                context_handle as i64,
                event_ptr,
                event_len,
                in_capture_phase as i32,
                callback.id as i32,
                callback.user_data as i32,
                callback.destroy_id as i32,
                pointer,
            )
        };
        listener_result(status, output)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (context_handle, event, in_capture_phase, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn element_add_event_listener(
    element_handle: u64,
    event: &str,
    in_capture_phase: bool,
    callback: RetainedCallback,
) -> Result<EventListenerRegistration> {
    #[cfg(target_arch = "wasm32")]
    {
        let (event_ptr, event_len) = string_parts(event)?;
        let mut output = [0u64; 2];
        let pointer = output_ptr(&mut output)?;
        let status = unsafe {
            raw::element_add_event_listener(
                element_handle as i64,
                event_ptr,
                event_len,
                in_capture_phase as i32,
                callback.id as i32,
                callback.user_data as i32,
                callback.destroy_id as i32,
                pointer,
            )
        };
        listener_result(status, output)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (element_handle, event, in_capture_phase, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn data_model_bind_event(
    data_model_handle: u64,
    name: &str,
    callback: RetainedCallback,
    field_types: &[u8],
) -> Result<DataEventRegistration> {
    #[cfg(target_arch = "wasm32")]
    {
        let (name_ptr, name_len) = string_parts(name)?;
        let (field_types_ptr, _) = bytes_parts(field_types)?;
        let mut output = [0u64; 2];
        let output_pointer = output_ptr(&mut output)?;
        let status = unsafe {
            raw::data_model_bind_event(
                data_model_handle as i64,
                name_ptr,
                name_len,
                callback.id as i32,
                callback.user_data as i32,
                callback.destroy_id as i32,
                field_types_ptr,
                field_types.len() as i64,
                output_pointer,
            )
        };
        if status != 0 {
            return Err(ApiError::new(status));
        }
        let success = match output[1] as u32 {
            0 => false,
            1 => true,
            _ => return Err(ApiError::new(ErrorCode::Internal as i32)),
        };
        Ok(DataEventRegistration {
            handle: output[0],
            success,
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (data_model_handle, name, callback, field_types);
        Err(unreachable!())
    }
}

#[inline]
pub fn data_model_unbind_event(event_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_bool(unsafe { raw::data_model_unbind_event(event_handle as i64) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = event_handle;
        Err(unreachable!())
    }
}

/// Dispatch a retained listener the guest registered earlier. The native call
/// re-enters this guest through the retained-callback path, so it is subject to
/// the host's callback nesting limit rather than being freely recursive.
#[inline]
pub fn event_listener_on_attach(listener_handle: u64, element_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_bool(unsafe {
            raw::event_listener_on_attach(listener_handle as i64, element_handle as i64)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (listener_handle, element_handle);
        Err(unreachable!())
    }
}

#[inline]
pub fn event_listener_on_detach(listener_handle: u64, element_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_bool(unsafe {
            raw::event_listener_on_detach(listener_handle as i64, element_handle as i64)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (listener_handle, element_handle);
        Err(unreachable!())
    }
}

/// A zero `event_handle` dispatches the host's current event.
#[inline]
pub fn event_listener_process_event(listener_handle: u64, event_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_bool(unsafe {
            raw::event_listener_process_event(listener_handle as i64, event_handle as i64)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (listener_handle, event_handle);
        Err(unreachable!())
    }
}

#[inline]
pub fn data_model_current_event() -> Result<CurrentDataEvent> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0u64; 3];
        let pointer = output_ptr(&mut output)?;
        let status = unsafe { raw::data_model_current_event(pointer) };
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(CurrentDataEvent {
            event_handle: output[0],
            target_element_handle: output[1],
            value_count: output[2],
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn data_model_current_value<'a>(
    index: u64,
    string_buffer: &'a mut [u8],
) -> core::result::Result<DataEventValue<'a>, DataEventValueError> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0u32; 6];
        let output_pointer = output_ptr(&mut output).map_err(DataEventValueError::Api)?;
        let (string_pointer, string_capacity) =
            bytes_parts(string_buffer).map_err(DataEventValueError::Api)?;
        let status = unsafe {
            raw::data_model_current_value(
                index as i64,
                output_pointer,
                string_pointer,
                string_capacity,
            )
        };
        if status == ErrorCode::BufferOverflow as i32 {
            return Err(DataEventValueError::BufferTooSmall {
                required: output[5],
            });
        }
        if status != 0 {
            return Err(DataEventValueError::Api(ApiError::new(status)));
        }
        match output[0] {
            0 => match output[1] {
                0 => Ok(DataEventValue::Bool(false)),
                1 => Ok(DataEventValue::Bool(true)),
                _ => Err(DataEventValueError::Api(ApiError::new(
                    ErrorCode::Internal as i32,
                ))),
            },
            1 => Ok(DataEventValue::Int(output[2] as i32)),
            2 => Ok(DataEventValue::Float(f32::from_bits(output[3]))),
            3 => {
                let length = output[5] as usize;
                let text = core::str::from_utf8(&string_buffer[..length]).map_err(|_| {
                    DataEventValueError::Api(ApiError::new(ErrorCode::Internal as i32))
                })?;
                Ok(DataEventValue::String(text))
            }
            4 => {
                let packed = output[4];
                Ok(DataEventValue::Color([
                    packed as u8,
                    (packed >> 8) as u8,
                    (packed >> 16) as u8,
                    (packed >> 24) as u8,
                ]))
            }
            5 => Ok(DataEventValue::Pixels(f32::from_bits(output[3]))),
            6 => Ok(DataEventValue::Percent(f32::from_bits(output[3]))),
            _ => Err(DataEventValueError::Api(ApiError::new(
                ErrorCode::Internal as i32,
            ))),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (index, string_buffer);
        Err(DataEventValueError::Api(ApiError::new(
            ErrorCode::Internal as i32,
        )))
    }
}

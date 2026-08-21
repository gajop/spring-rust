use super::{ApiError, ErrorCode, Result, RetainedCallback};

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
    extern "C" {
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
        return listener_result(status, output);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (context_handle, event, in_capture_phase, callback);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
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
        return listener_result(status, output);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (element_handle, event, in_capture_phase, callback);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
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
        return Ok(DataEventRegistration {
            handle: output[0],
            success,
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (data_model_handle, name, callback, field_types);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn data_model_unbind_event(event_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return super::unpack_bool(unsafe { raw::data_model_unbind_event(event_handle as i64) });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = event_handle;
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

/// Dispatch a retained listener the guest registered earlier. The native call
/// re-enters this guest through the retained-callback path, so it is subject to
/// the host's callback nesting limit rather than being freely recursive.
#[inline]
pub fn event_listener_on_attach(listener_handle: u64, element_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return super::unpack_bool(unsafe {
            raw::event_listener_on_attach(listener_handle as i64, element_handle as i64)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (listener_handle, element_handle);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn event_listener_on_detach(listener_handle: u64, element_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return super::unpack_bool(unsafe {
            raw::event_listener_on_detach(listener_handle as i64, element_handle as i64)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (listener_handle, element_handle);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

/// A zero `event_handle` dispatches the host's current event.
#[inline]
pub fn event_listener_process_event(listener_handle: u64, event_handle: u64) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return super::unpack_bool(unsafe {
            raw::event_listener_process_event(listener_handle as i64, event_handle as i64)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (listener_handle, event_handle);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
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
        return Ok(CurrentDataEvent {
            event_handle: output[0],
            target_element_handle: output[1],
            value_count: output[2],
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
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
        return match output[0] {
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
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (index, string_buffer);
        Err(DataEventValueError::Api(ApiError::new(
            ErrorCode::UnsupportedHostTarget as i32,
        )))
    }
}

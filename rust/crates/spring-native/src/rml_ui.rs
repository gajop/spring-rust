use std::{
    ffi::{CStr, CString, c_void},
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_char,
    ptr, slice,
};

use crate::{error::Error, sys};

#[derive(Clone, Copy)]
pub struct RmlUi<'a> {
    api: &'a sys::RmlUiApi,
}

/// Relative pointer movement accumulated by an engine-owned RmlUi capture.
///
/// The engine returns the physical cursor to its capture anchor after every
/// input event. `delta_x` and `delta_y` are therefore relative movement since
/// the last call rather than an absolute screen position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RmlPointerCaptureDelta {
    pub delta_x: i32,
    pub delta_y: i32,
    pub status: RmlPointerCaptureStatus,
}

/// Lifecycle state returned with an engine-owned pointer capture sample.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RmlPointerCaptureStatus {
    #[default]
    None,
    Active,
    Released,
    Cancelled,
}

/// An engine-owned RmlUi data model.
///
/// Values live entirely in the engine. Rust receives only opaque handles, so a
/// model never borrows Rust memory across the native-module boundary and no
/// value is encoded into a transport format.
pub struct RmlDataModel<'api> {
    ui: RmlUi<'api>,
    handle: u64,
}

/// Scalar types supported by a runtime-defined RmlUi row schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RmlFieldType {
    Bool,
    Int,
    Float,
    String,
    Color,
    Pixels,
    Percent,
}

impl RmlFieldType {
    fn native_type(self) -> u8 {
        match self {
            Self::Bool => 0,
            Self::Int => 1,
            Self::Float => 2,
            Self::String => 3,
            Self::Color => 4,
            Self::Pixels => 5,
            Self::Percent => 6,
        }
    }
}

/// An owned value produced when reading a RmlUi data-model event argument.
///
/// Row writes use [`RmlValueRef`] so string inputs can be borrowed instead of
/// cloned before crossing the native boundary.
#[derive(Clone, Debug, PartialEq)]
pub enum RmlValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Color(RmlColor),
    Pixels(RmlPixels),
    Percent(RmlPercent),
}

impl RmlValue {
    unsafe fn from_native(value: &sys::RmlDataValue) -> Option<Self> {
        match value.type_ {
            0 => Some(Self::Bool(value.boolValue)),
            1 => Some(Self::Int(value.intValue)),
            2 => Some(Self::Float(value.floatValue)),
            3 if !value.stringValue.is_null() => Some(Self::String(
                unsafe { CStr::from_ptr(value.stringValue) }
                    .to_string_lossy()
                    .into_owned(),
            )),
            4 => Some(Self::Color(RmlColor {
                red: value.red,
                green: value.green,
                blue: value.blue,
                alpha: value.alpha,
            })),
            5 => Some(Self::Pixels(RmlPixels(value.floatValue))),
            6 => Some(Self::Percent(RmlPercent(value.floatValue))),
            _ => None,
        }
    }
}

/// A borrowed scalar value used when writing runtime-defined rows.
///
/// The engine copies every value during [`RmlDataRows::set`], so string slices
/// only need to live until that call returns. This keeps per-frame row updates
/// from cloning strings into temporary Rust-owned values first.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RmlValueRef<'a> {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(&'a str),
    Color(RmlColor),
    Pixels(RmlPixels),
    Percent(RmlPercent),
}

impl RmlValueRef<'_> {
    fn into_native(self, string_value: *const c_char) -> sys::RmlDataValue {
        let mut native = sys::RmlDataValue {
            type_: 0,
            boolValue: false,
            intValue: 0,
            floatValue: 0.0,
            stringValue: ptr::null(),
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        match self {
            Self::Bool(item) => {
                native.type_ = RmlFieldType::Bool.native_type();
                native.boolValue = item;
            }
            Self::Int(item) => {
                native.type_ = RmlFieldType::Int.native_type();
                native.intValue = item;
            }
            Self::Float(item) => {
                native.type_ = RmlFieldType::Float.native_type();
                native.floatValue = item;
            }
            Self::String(_) => {
                native.type_ = RmlFieldType::String.native_type();
                native.stringValue = string_value;
            }
            Self::Color(item) => {
                native.type_ = RmlFieldType::Color.native_type();
                native.red = item.red;
                native.green = item.green;
                native.blue = item.blue;
                native.alpha = item.alpha;
            }
            Self::Pixels(item) => {
                native.type_ = RmlFieldType::Pixels.native_type();
                native.floatValue = item.0;
            }
            Self::Percent(item) => {
                native.type_ = RmlFieldType::Percent.native_type();
                native.floatValue = item.0;
            }
        }
        native
    }
}

/// A data-model event callback invocation borrowing the native payload.
///
/// The native values are valid only while the callback is running. Calling
/// [`values`](Self::values) converts individual slots on demand; dispatch does
/// not allocate a `Vec`, and unknown or null values remain positional `None`
/// entries in the iterator.
pub struct RmlDataEventArgs<'a> {
    pub event_handle: u64,
    pub target_element_handle: u64,
    native_values: &'a [sys::RmlDataValue],
}

impl<'a> RmlDataEventArgs<'a> {
    unsafe fn from_native(args: &'a sys::RmlDataEventArgs) -> Self {
        let native_values = if args.values.is_null() || args.count == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(args.values, args.count as usize) }
        };
        Self {
            event_handle: args.eventHandle,
            target_element_handle: args.targetElementHandle,
            native_values,
        }
    }

    pub fn len(&self) -> usize {
        self.native_values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.native_values.is_empty()
    }

    /// Converts one positional argument, preserving the distinction between
    /// an out-of-range index and an unsupported/null value via the outer and
    /// inner `Option` respectively.
    pub fn get(&self, index: usize) -> Option<Option<RmlValue>> {
        self.native_values
            .get(index)
            .map(|value| unsafe { RmlValue::from_native(value) })
    }

    /// Iterates over converted positional arguments without allocating a
    /// collection. A string is copied only if the callback actually reads it.
    pub fn values(&self) -> impl Iterator<Item = Option<RmlValue>> + '_ {
        self.native_values
            .iter()
            .map(|value| unsafe { RmlValue::from_native(value) })
    }
}

/// A registered data-model event callback. The callback remains active until
/// [`RmlDataEvent::unbind`] or removal of its data model.
pub struct RmlDataEvent<'api> {
    ui: RmlUi<'api>,
    handle: u64,
}

/// An engine-owned collection whose row schema is declared at runtime.
pub struct RmlDataRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    field_count: usize,
}

/// A typed, engine-owned field in an [`RmlDataModel`].
///
/// The type parameter prevents, for example, writing a `String` through a
/// field that was declared as an `f32`.
#[derive(Clone, Copy)]
pub struct RmlDataVariable<'api, T: RmlDataValue> {
    ui: RmlUi<'api>,
    handle: u64,
    _value: PhantomData<T>,
}

/// An RGBA colour carried over the RmlUi bridge as four channels, rather than
/// as a CSS string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RmlColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// A CSS pixel length. Rust supplies only the scalar value; the engine owns
/// conversion into RmlUi's style-unit representation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RmlPixels(pub f32);

/// A CSS percentage. Rust carries the scalar value while the engine supplies
/// the percent unit when RmlUi reads a style binding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RmlPercent(pub f32);

mod data_value_sealed {
    use super::{RmlColor, RmlPercent, RmlPixels};

    pub trait Sealed {}

    impl Sealed for bool {}
    impl Sealed for i32 {}
    impl Sealed for f32 {}
    impl Sealed for String {}
    impl Sealed for RmlColor {}
    impl Sealed for RmlPixels {}
    impl Sealed for RmlPercent {}
}

/// Types supported by engine-owned RmlUi data-model fields.
///
/// This trait is sealed because the C++ side needs an exact native layout and
/// RmlUi binding definition for every supported type.
pub trait RmlDataValue: data_value_sealed::Sealed + Sized {
    #[doc(hidden)]
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error>;

    #[doc(hidden)]
    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error>;

    #[doc(hidden)]
    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error>;
}

fn require_success(success: bool, operation: &str) -> Result<(), Error> {
    if success {
        Ok(())
    } else {
        Err(Error::new(
            1,
            format!("RmlUi data-model {operation} failed"),
        ))
    }
}

impl RmlDataValue for bool {
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error> {
        let (handle, success) = ui.data_model_bind_bool(model_handle, name, *initial)?;
        require_success(success, "bool bind")?;
        Ok(handle)
    }

    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error> {
        require_success(ui.data_model_set_bool(variable_handle, *value)?, "bool set")
    }

    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error> {
        let (value, success) = ui.data_model_get_bool(variable_handle)?;
        require_success(success, "bool get")?;
        Ok(value)
    }
}

impl RmlDataValue for i32 {
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error> {
        let (handle, success) = ui.data_model_bind_int(model_handle, name, *initial)?;
        require_success(success, "int bind")?;
        Ok(handle)
    }

    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error> {
        require_success(ui.data_model_set_int(variable_handle, *value)?, "int set")
    }

    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error> {
        let (value, success) = ui.data_model_get_int(variable_handle)?;
        require_success(success, "int get")?;
        Ok(value)
    }
}

impl RmlDataValue for f32 {
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error> {
        let (handle, success) = ui.data_model_bind_float(model_handle, name, *initial)?;
        require_success(success, "float bind")?;
        Ok(handle)
    }

    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error> {
        require_success(
            ui.data_model_set_float(variable_handle, *value)?,
            "float set",
        )
    }

    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error> {
        let (value, success) = ui.data_model_get_float(variable_handle)?;
        require_success(success, "float get")?;
        Ok(value)
    }
}

impl RmlDataValue for String {
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error> {
        let (handle, success) = ui.data_model_bind_string(model_handle, name, initial)?;
        require_success(success, "string bind")?;
        Ok(handle)
    }

    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error> {
        require_success(
            ui.data_model_set_string(variable_handle, value)?,
            "string set",
        )
    }

    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error> {
        let (value, success) = ui.data_model_get_string(variable_handle)?;
        require_success(success, "string get")?;
        value.ok_or_else(|| Error::new(1, "RmlUi data-model string field was null"))
    }
}

impl RmlDataValue for RmlColor {
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error> {
        let (handle, success) = ui.data_model_bind_color(
            model_handle,
            name,
            initial.red,
            initial.green,
            initial.blue,
            initial.alpha,
        )?;
        require_success(success, "colour bind")?;
        Ok(handle)
    }

    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error> {
        require_success(
            ui.data_model_set_color(
                variable_handle,
                value.red,
                value.green,
                value.blue,
                value.alpha,
            )?,
            "colour set",
        )
    }

    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error> {
        let (red, green, blue, alpha, success) = ui.data_model_get_color(variable_handle)?;
        require_success(success, "colour get")?;
        Ok(RmlColor {
            red,
            green,
            blue,
            alpha,
        })
    }
}

impl RmlDataValue for RmlPixels {
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error> {
        let (handle, success) = ui.data_model_bind_pixels(model_handle, name, initial.0)?;
        require_success(success, "pixel-length bind")?;
        Ok(handle)
    }

    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error> {
        require_success(
            ui.data_model_set_pixels(variable_handle, value.0)?,
            "pixel-length set",
        )
    }

    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error> {
        let (value, success) = ui.data_model_get_pixels(variable_handle)?;
        require_success(success, "pixel-length get")?;
        Ok(RmlPixels(value))
    }
}

impl RmlDataValue for RmlPercent {
    fn bind(ui: &RmlUi<'_>, model_handle: u64, name: &str, initial: &Self) -> Result<u64, Error> {
        let (handle, success) = ui.data_model_bind_percent(model_handle, name, initial.0)?;
        require_success(success, "percentage bind")?;
        Ok(handle)
    }

    fn set(ui: &RmlUi<'_>, variable_handle: u64, value: &Self) -> Result<(), Error> {
        require_success(
            ui.data_model_set_percent(variable_handle, value.0)?,
            "percentage set",
        )
    }

    fn get(ui: &RmlUi<'_>, variable_handle: u64) -> Result<Self, Error> {
        let (value, success) = ui.data_model_get_percent(variable_handle)?;
        require_success(success, "percentage get")?;
        Ok(RmlPercent(value))
    }
}

impl<'a> RmlUi<'a> {
    pub(crate) fn new(api: &'a sys::RmlUiApi) -> Self {
        Self { api }
    }

    #[allow(non_snake_case)]
    pub fn sol_lua_data_model___set_dirty(
        &self,
        data_model_handle: u64,
        property: &str,
    ) -> Result<bool, Error> {
        self.sol_lua_data_model_set_dirty(data_model_handle, property)
    }

    /// Creates an engine-owned, typed RmlUi data model.
    ///
    /// The name is RmlUi metadata, not a serialized value. Values are added
    /// through [`RmlDataModel::bind`].
    pub fn create_data_model(
        &self,
        context_handle: u64,
        name: &str,
    ) -> Result<RmlDataModel<'a>, Error> {
        let (handle, success) = self.context_create_data_model(context_handle, name)?;
        require_success(success, "create")?;
        Ok(RmlDataModel { handle, ui: *self })
    }

    /// Removes a native RmlUi data model and all handles bound through it.
    ///
    /// This lets a dynamic subtree replace a model of the same semantic role
    /// without accumulating a model for every rebuild.
    pub fn remove_data_model(&self, context_handle: u64, name: &str) -> Result<(), Error> {
        require_success(
            self.context_remove_data_model(context_handle, name)?,
            "remove",
        )
    }

    /// Returns and clears the captured relative movement accumulated since the
    /// prior call. No string or serialized event payload crosses this boundary.
    pub fn take_pointer_capture_delta(
        &self,
        context_handle: u64,
    ) -> Result<RmlPointerCaptureDelta, Error> {
        let (delta_x, delta_y, status) = self.context_take_pointer_capture_delta(context_handle)?;
        let status = match status {
            0 => RmlPointerCaptureStatus::None,
            1 => RmlPointerCaptureStatus::Active,
            2 => RmlPointerCaptureStatus::Released,
            3 => RmlPointerCaptureStatus::Cancelled,
            _ => return Err(Error::new(1, "invalid RmlUi pointer capture status")),
        };
        Ok(RmlPointerCaptureDelta {
            delta_x,
            delta_y,
            status,
        })
    }
}

impl<'api> RmlDataModel<'api> {
    /// Adds a named native field and returns its typed handle.
    pub fn bind<T: RmlDataValue>(
        &self,
        name: &str,
        initial: T,
    ) -> Result<RmlDataVariable<'api, T>, Error> {
        let handle = T::bind(&self.ui, self.handle, name, &initial)?;
        Ok(RmlDataVariable {
            ui: self.ui,
            handle,
            _value: PhantomData,
        })
    }

    /// Adds an engine-owned collection with a schema chosen at runtime.
    ///
    /// The schema is fixed for the lifetime of the returned collection. Its
    /// values are supplied row-major through [`RmlDataRows::set`]. RmlUi adds
    /// an internal `visible` field to every row for its data-for lifecycle;
    /// that field is not part of this schema or the values passed by callers.
    pub fn bind_rows(
        &self,
        name: &str,
        fields: &[(&str, RmlFieldType)],
    ) -> Result<RmlDataRows<'api>, Error> {
        if fields.is_empty() {
            return Err(Error::invalid_argument("fields"));
        }

        let name = CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
        let field_names = fields
            .iter()
            .map(|(field_name, _)| {
                CString::new(*field_name).map_err(|_| Error::invalid_argument("field name"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let native_fields = fields
            .iter()
            .zip(&field_names)
            .map(|((_, field_type), field_name)| sys::RmlDataFieldDef {
                name: field_name.as_ptr(),
                type_: field_type.native_type(),
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelBindRowsQuery {
            dataModelHandle: self.handle,
            name: name.as_ptr(),
            fields: native_fields.as_ptr(),
            fieldCount: native_fields.len() as u64,
        };

        unsafe {
            let mut result = MaybeUninit::<sys::RmlDataModelRowsResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelBindRows
                .expect("DataModelBindRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let (handle, success) =
                Error::result_or(result.error, (result.rowsHandle, result.success))?;
            require_success(success, "rows bind")?;
            Ok(RmlDataRows {
                ui: self.ui,
                handle,
                field_count: fields.len(),
            })
        }
    }

    /// Binds a data-event expression callback with a declared positional
    /// schema. RmlUi owns the callback until the returned binding is unbound
    /// or this data model is destroyed. Callback arguments borrow the native
    /// payload for the duration of the invocation.
    ///
    /// RmlUi reports an arity or type mismatch when the event expression is
    /// dispatched. Its registration API does not expose the RML expression,
    /// so those arguments cannot be checked earlier.
    pub fn bind_event<F>(
        &self,
        name: &str,
        fields: &[RmlFieldType],
        callback: F,
    ) -> Result<RmlDataEvent<'api>, Error>
    where
        F: for<'event> FnMut(RmlDataEventArgs<'event>) + 'static,
    {
        unsafe extern "C" fn trampoline<F>(
            user_data: *mut c_void,
            args: *const sys::RmlDataEventArgs,
        ) where
            F: for<'event> FnMut(RmlDataEventArgs<'event>) + 'static,
        {
            if user_data.is_null() || args.is_null() {
                return;
            }
            let callback = unsafe { &mut *(user_data as *mut F) };
            let args = unsafe { RmlDataEventArgs::from_native(&*args) };
            callback(args);
        }

        unsafe extern "C" fn destroy_callback<F>(user_data: *mut c_void)
        where
            F: for<'event> FnMut(RmlDataEventArgs<'event>) + 'static,
        {
            if !user_data.is_null() {
                drop(unsafe { Box::from_raw(user_data as *mut F) });
            }
        }

        let name = CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
        let native_fields = fields
            .iter()
            .map(|field_type| field_type.native_type())
            .collect::<Vec<_>>();
        let callback = Box::into_raw(Box::new(callback));
        let query = sys::RmlDataModelBindEventQuery {
            dataModelHandle: self.handle,
            name: name.as_ptr(),
            callback: Some(trampoline::<F>),
            userData: callback as *mut c_void,
            destroyCallback: Some(destroy_callback::<F>),
            fieldTypes: native_fields.as_ptr(),
            fieldCount: native_fields.len() as u64,
        };

        unsafe {
            let mut result = MaybeUninit::<sys::RmlDataModelBindEventResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelBindEvent
                .expect("DataModelBindEvent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let (handle, success) =
                Error::result_or(result.error, (result.eventHandle, result.success))?;
            require_success(success, "event bind")?;
            Ok(RmlDataEvent {
                ui: self.ui,
                handle,
            })
        }
    }
}

impl<'api> RmlDataEvent<'api> {
    /// Stops dispatching this callback and releases its callback data.
    pub fn unbind(&self) -> Result<(), Error> {
        let query = sys::RmlDataModelEventHandleQuery {
            eventHandle: self.handle,
        };
        unsafe {
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelUnbindEvent
                .expect("DataModelUnbindEvent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "event unbind",
            )
        }
    }
}

impl<'api, T: RmlDataValue> RmlDataVariable<'api, T> {
    /// Writes a typed value and marks only this RmlUi field dirty.
    pub fn set(&self, value: T) -> Result<(), Error> {
        T::set(&self.ui, self.handle, &value)
    }

    /// Reads the current native value, including two-way updates made by RmlUi
    /// form controls.
    pub fn get(&self) -> Result<T, Error> {
        T::get(&self.ui, self.handle)
    }
}

impl<'api> RmlDataRows<'api> {
    /// Replaces the assigned rows. Values are row-major and their length must
    /// be a multiple of the schema's field count. The engine copies all
    /// values, including strings, before this call returns.
    pub fn set(&self, values: &[RmlValueRef<'_>]) -> Result<(), Error> {
        if !values.len().is_multiple_of(self.field_count) {
            return Err(Error::invalid_argument("values"));
        }

        let strings = values
            .iter()
            .map(|value| match value {
                RmlValueRef::String(value) => CString::new(*value)
                    .map(Some)
                    .map_err(|_| Error::invalid_argument("string value")),
                _ => Ok(None),
            })
            .collect::<Result<Vec<Option<CString>>, _>>()?;
        let native_values = values
            .iter()
            .zip(&strings)
            .map(|(value, string)| {
                (*value).into_native(string.as_ref().map_or(ptr::null(), |value| value.as_ptr()))
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetRowsQuery {
            rowsHandle: self.handle,
            values: native_values
                .first()
                .map_or(ptr::null(), |value| value as *const _),
            rowCount: (values.len() / self.field_count) as u64,
        };

        unsafe {
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelSetRows
                .expect("DataModelSetRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(Error::result_or(result.error, result.success)?, "rows set")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_argument_conversion_preserves_unknown_slots() {
        let native_values = [
            sys::RmlDataValue {
                type_: 1,
                intValue: 7,
                ..Default::default()
            },
            sys::RmlDataValue {
                type_: 255,
                ..Default::default()
            },
            sys::RmlDataValue {
                type_: 3,
                stringValue: ptr::null(),
                ..Default::default()
            },
        ];
        let native_args = sys::RmlDataEventArgs {
            eventHandle: 11,
            targetElementHandle: 22,
            values: native_values.as_ptr(),
            count: native_values.len() as u64,
        };

        let args = unsafe { RmlDataEventArgs::from_native(&native_args) };

        assert_eq!(args.len(), 3);
        assert_eq!(args.get(0), Some(Some(RmlValue::Int(7))));
        assert_eq!(args.get(1), Some(None));
        assert_eq!(args.get(2), Some(None));
        assert_eq!(args.get(3), None);

        let values = args.values().collect::<Vec<_>>();
        assert_eq!(values, [Some(RmlValue::Int(7)), None, None]);
    }
}

include!(concat!(env!("OUT_DIR"), "/rml_ui_generated.rs"));

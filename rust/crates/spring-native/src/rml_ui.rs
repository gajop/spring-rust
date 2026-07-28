use std::{
    cell::Cell,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::MaybeUninit,
    ptr, slice,
};

use crate::{error::Error, sys};

#[derive(Clone, Copy)]
pub struct RmlUi<'a> {
    api: &'a sys::RmlUiApi,
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

/// A typed, engine-owned field in an [`RmlDataModel`].
///
/// The type parameter prevents, for example, writing a `String` through a
/// field that was declared as an `f32`.
pub struct RmlDataVariable<'api, T: RmlDataValue> {
    ui: RmlUi<'api>,
    handle: u64,
    _value: PhantomData<T>,
}

/// One typed row in an engine-owned RmlUi text collection.
///
/// Its `String` is copied into engine storage when assigned; it is never
/// serialized or retained by the FFI boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RmlTextRow {
    pub text: String,
    pub muted: bool,
}

/// An engine-owned collection of [`RmlTextRow`] values.
///
/// This is deliberately a distinct type from scalar variables: RmlUi renders
/// it through `data-for`, and replacements are copied atomically by the engine.
pub struct RmlDataTextRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
}

/// One typed toast notification row in an engine-owned RmlUi collection.
///
/// The optional progress bar is expressed structurally, rather than as a
/// sentinel numeric value or generated RML.
#[derive(Clone, Debug, PartialEq)]
pub struct RmlNotificationRow {
    pub title: String,
    pub body: String,
    pub warning: bool,
    pub progress: Option<f32>,
}

/// An engine-owned collection of [`RmlNotificationRow`] values.
pub struct RmlDataNotificationRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
}

/// One typed row for an icon-bearing control.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RmlIconRow {
    pub label: String,
    pub icon: String,
    pub tooltip: String,
}

/// An engine-owned collection of [`RmlIconRow`] values.
pub struct RmlDataIconRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    stable_count: StableRowCount,
}

/// One typed option in an engine-owned RmlUi select collection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RmlOptionRow {
    pub value: String,
    pub label: String,
}

/// An engine-owned collection of [`RmlOptionRow`] values.
pub struct RmlDataOptionRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    stable_count: StableRowCount,
}

/// RmlUi updates dependent row bindings before it removes rows from a
/// `data-for` view. The bridge therefore retains the largest assigned length
/// and marks empty-label padding rows invisible in the native layer, keeping
/// that renderer detail out of UI components and dialogs.
#[derive(Default)]
struct StableRowCount(Cell<usize>);

impl StableRowCount {
    fn update(&self, requested: usize) -> usize {
        let count = self.0.get().max(requested);
        self.0.set(count);
        count
    }
}

mod data_value_sealed {
    pub trait Sealed {}

    impl Sealed for bool {}
    impl Sealed for i32 {}
    impl Sealed for f32 {}
    impl Sealed for String {}
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

    /// Adds a native-owned collection for fixed-shape text rows.
    pub fn bind_text_rows(&self, name: &str) -> Result<RmlDataTextRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_text_rows(self.handle, name)?;
        require_success(success, "text-row bind")?;
        Ok(RmlDataTextRows {
            ui: self.ui,
            handle,
        })
    }

    /// Adds a native-owned collection for fixed-shape toast notifications.
    pub fn bind_notification_rows(
        &self,
        name: &str,
    ) -> Result<RmlDataNotificationRows<'api>, Error> {
        let (handle, success) = self
            .ui
            .data_model_bind_notification_rows(self.handle, name)?;
        require_success(success, "notification-row bind")?;
        Ok(RmlDataNotificationRows {
            ui: self.ui,
            handle,
        })
    }

    /// Adds a native-owned collection for icon-bearing controls.
    pub fn bind_icon_rows(&self, name: &str) -> Result<RmlDataIconRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_icon_rows(self.handle, name)?;
        require_success(success, "icon-row bind")?;
        Ok(RmlDataIconRows {
            ui: self.ui,
            handle,
            stable_count: StableRowCount::default(),
        })
    }

    /// Adds a native-owned collection for select options.
    pub fn bind_option_rows(&self, name: &str) -> Result<RmlDataOptionRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_option_rows(self.handle, name)?;
        require_success(success, "option-row bind")?;
        Ok(RmlDataOptionRows {
            ui: self.ui,
            handle,
            stable_count: StableRowCount::default(),
        })
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

impl<'api> RmlDataTextRows<'api> {
    /// Replaces the complete collection through a borrowed, C-compatible row
    /// slice. The engine copies it before this call returns, including an empty
    /// slice, so no Rust allocation crosses the call boundary.
    pub fn set(&self, rows: &[RmlTextRow]) -> Result<(), Error> {
        let strings = rows
            .iter()
            .map(|row| CString::new(row.text.as_str()).map_err(|_| Error::invalid_argument("text")))
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = rows
            .iter()
            .zip(&strings)
            .map(|(row, text)| sys::RmlDataTextRow {
                text: text.as_ptr(),
                muted: row.muted,
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetTextRowsQuery {
            rowsHandle: self.handle,
            rows: native_rows
                .first()
                .map_or(ptr::null(), |row| row as *const _),
            count: native_rows.len() as u64,
        };
        unsafe {
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelSetTextRows
                .expect("DataModelSetTextRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "text-row set",
            )
        }
    }
}

impl<'api> RmlDataNotificationRows<'api> {
    /// Replaces the complete notification collection. The engine copies both
    /// strings and the scalar fields before this call returns.
    pub fn set(&self, rows: &[RmlNotificationRow]) -> Result<(), Error> {
        let strings = rows
            .iter()
            .map(|row| {
                Ok::<_, Error>((
                    CString::new(row.title.as_str())
                        .map_err(|_| Error::invalid_argument("title"))?,
                    CString::new(row.body.as_str()).map_err(|_| Error::invalid_argument("body"))?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = rows
            .iter()
            .zip(&strings)
            .map(|(row, (title, body))| sys::RmlDataNotificationRow {
                title: title.as_ptr(),
                body: body.as_ptr(),
                warning: row.warning,
                hasProgress: row.progress.is_some(),
                progress: row.progress.unwrap_or_default(),
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetNotificationRowsQuery {
            rowsHandle: self.handle,
            rows: native_rows
                .first()
                .map_or(ptr::null(), |row| row as *const _),
            count: native_rows.len() as u64,
        };
        unsafe {
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelSetNotificationRows
                .expect("DataModelSetNotificationRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "notification-row set",
            )
        }
    }
}

impl<'api> RmlDataIconRows<'api> {
    /// Replaces the complete collection. The engine copies every string before
    /// this call returns; no Rust allocation crosses the data model boundary.
    /// Shorter updates retain invisible native padding rows internally.
    pub fn set(&self, rows: &[RmlIconRow]) -> Result<(), Error> {
        let count = self.stable_count.update(rows.len());
        let strings = rows
            .iter()
            .map(|row| {
                Ok::<_, Error>((
                    CString::new(row.label.as_str())
                        .map_err(|_| Error::invalid_argument("label"))?,
                    CString::new(row.icon.as_str()).map_err(|_| Error::invalid_argument("icon"))?,
                    CString::new(row.tooltip.as_str())
                        .map_err(|_| Error::invalid_argument("tooltip"))?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut native_rows = rows
            .iter()
            .zip(&strings)
            .map(|(_, (label, icon, tooltip))| sys::RmlDataIconRow {
                label: label.as_ptr(),
                icon: icon.as_ptr(),
                tooltip: tooltip.as_ptr(),
            })
            .collect::<Vec<_>>();
        let empty = CStr::from_bytes_with_nul(b"\0").expect("static empty C string");
        native_rows.resize_with(count, || sys::RmlDataIconRow {
            label: empty.as_ptr(),
            icon: empty.as_ptr(),
            tooltip: empty.as_ptr(),
        });
        let query = sys::RmlDataModelSetIconRowsQuery {
            rowsHandle: self.handle,
            rows: native_rows
                .first()
                .map_or(ptr::null(), |row| row as *const _),
            count: native_rows.len() as u64,
        };
        unsafe {
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelSetIconRows
                .expect("DataModelSetIconRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "icon-row set",
            )
        }
    }
}

impl<'api> RmlDataOptionRows<'api> {
    /// Replaces the complete option collection. Values and labels are copied by
    /// the engine before this call returns. Shorter updates retain invisible
    /// native padding rows internally.
    pub fn set(&self, rows: &[RmlOptionRow]) -> Result<(), Error> {
        let count = self.stable_count.update(rows.len());
        let strings = rows
            .iter()
            .map(|row| {
                Ok::<_, Error>((
                    CString::new(row.value.as_str())
                        .map_err(|_| Error::invalid_argument("value"))?,
                    CString::new(row.label.as_str())
                        .map_err(|_| Error::invalid_argument("label"))?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut native_rows = rows
            .iter()
            .zip(&strings)
            .map(|(_, (value, label))| sys::RmlDataOptionRow {
                value: value.as_ptr(),
                label: label.as_ptr(),
            })
            .collect::<Vec<_>>();
        let empty = CStr::from_bytes_with_nul(b"\0").expect("static empty C string");
        native_rows.resize_with(count, || sys::RmlDataOptionRow {
            value: empty.as_ptr(),
            label: empty.as_ptr(),
        });
        let query = sys::RmlDataModelSetOptionRowsQuery {
            rowsHandle: self.handle,
            rows: native_rows
                .first()
                .map_or(ptr::null(), |row| row as *const _),
            count: native_rows.len() as u64,
        };
        unsafe {
            let mut result = MaybeUninit::<sys::RmlElementBoolResult>::zeroed();
            let func = self
                .ui
                .api
                .DataModelSetOptionRows
                .expect("DataModelSetOptionRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "option-row set",
            )
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/rml_ui_generated.rs"));

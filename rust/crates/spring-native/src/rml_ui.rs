use std::{
    cell::Cell,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::MaybeUninit,
    ptr, rc::Rc, slice,
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
#[derive(Clone, Copy)]
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
    pub visible: bool,
}

/// An engine-owned collection of [`RmlTextRow`] values.
///
/// This is deliberately a distinct type from scalar variables: RmlUi renders
/// it through `data-for`, and replacements are copied atomically by the engine.
pub struct RmlDataTextRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    stable_count: StableRowCount,
}

/// Semantic severity of a console-log row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RmlLogSeverity {
    Info,
    Warning,
    Error,
}

/// One typed line in an engine-owned console-log collection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RmlLogRow {
    pub text: String,
    pub severity: RmlLogSeverity,
    pub selected: bool,
}

/// An engine-owned collection of [`RmlLogRow`] values.
pub struct RmlDataLogRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    stable_count: StableRowCount,
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
    pub pressed: bool,
    pub disabled: bool,
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

/// One typed row in a selectable label/detail list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RmlChoiceRow {
    pub label: String,
    pub detail: String,
    pub selected: bool,
    pub highlighted: bool,
}

/// An engine-owned collection of selectable label/detail rows.
pub struct RmlDataChoiceRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    stable_count: StableRowCount,
}

/// One labelled boolean status, rendered as presentation rather than input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RmlStatusRow {
    pub label: String,
    pub positive: bool,
}

/// An engine-owned collection of labelled status rows.
#[derive(Clone)]
pub struct RmlDataStatusRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    stable_count: Rc<StableRowCount>,
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

/// One labelled row with a native colour swatch and optional actions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RmlSwatchRow {
    pub label: String,
    pub color: RmlColor,
    pub actions_enabled: bool,
}

/// An engine-owned collection of [`RmlSwatchRow`] values.
pub struct RmlDataSwatchRows<'api> {
    ui: RmlUi<'api>,
    handle: u64,
    stable_count: StableRowCount,
}

/// One typed image-grid row. Presentation state stays scalar and structured,
/// so callers never construct a cell's markup or style attributes.
#[derive(Clone, Debug, PartialEq)]
pub struct RmlGridRow {
    pub label: String,
    pub image: String,
    pub cell_size: RmlPixels,
    pub has_image: bool,
    pub native_image: bool,
    pub selected: bool,
    pub folder: bool,
    /// A layout-only trailing cell. This absorbs flex slack without asking
    /// callers to manufacture a second kind of RML node.
    pub filler: bool,
}

/// An engine-owned collection of [`RmlGridRow`] values.
pub struct RmlDataGridRows<'api> {
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
            stable_count: StableRowCount::default(),
        })
    }

    /// Adds a native-owned collection for semantic console-log rows.
    pub fn bind_log_rows(&self, name: &str) -> Result<RmlDataLogRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_log_rows(self.handle, name)?;
        require_success(success, "log-row bind")?;
        Ok(RmlDataLogRows {
            ui: self.ui,
            handle,
            stable_count: StableRowCount::default(),
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

    /// Adds a native-owned collection for selectable label/detail rows.
    pub fn bind_choice_rows(&self, name: &str) -> Result<RmlDataChoiceRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_choice_rows(self.handle, name)?;
        require_success(success, "choice-row bind")?;
        Ok(RmlDataChoiceRows {
            ui: self.ui,
            handle,
            stable_count: StableRowCount::default(),
        })
    }

    /// Adds a native-owned collection of labelled boolean status rows.
    pub fn bind_status_rows(&self, name: &str) -> Result<RmlDataStatusRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_status_rows(self.handle, name)?;
        require_success(success, "status-row bind")?;
        Ok(RmlDataStatusRows {
            ui: self.ui,
            handle,
            stable_count: Rc::new(StableRowCount::default()),
        })
    }

    /// Adds a native-owned collection for labelled colour-swatch rows.
    pub fn bind_swatch_rows(&self, name: &str) -> Result<RmlDataSwatchRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_swatch_rows(self.handle, name)?;
        require_success(success, "swatch-row bind")?;
        Ok(RmlDataSwatchRows {
            ui: self.ui,
            handle,
            stable_count: StableRowCount::default(),
        })
    }

    /// Adds a native-owned collection for image-grid cells.
    pub fn bind_grid_rows(&self, name: &str) -> Result<RmlDataGridRows<'api>, Error> {
        let (handle, success) = self.ui.data_model_bind_grid_rows(self.handle, name)?;
        require_success(success, "grid-row bind")?;
        Ok(RmlDataGridRows {
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
        let count = self.stable_count.update(rows.len());
        let mut padded_rows = rows.to_vec();
        padded_rows.resize_with(count, || RmlTextRow {
            text: String::new(),
            muted: false,
            visible: false,
        });
        let strings = padded_rows
            .iter()
            .map(|row| CString::new(row.text.as_str()).map_err(|_| Error::invalid_argument("text")))
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = padded_rows
            .iter()
            .zip(&strings)
            .map(|(row, text)| sys::RmlDataTextRow {
                text: text.as_ptr(),
                muted: row.muted,
                visible: row.visible,
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

impl<'api> RmlDataLogRows<'api> {
    /// Replaces semantic log rows atomically in engine-owned storage. Shorter
    /// later updates retain hidden tail rows for RmlUi's `data-for` lifecycle.
    pub fn set(&self, rows: &[RmlLogRow]) -> Result<(), Error> {
        let count = self.stable_count.update(rows.len());
        let mut padded_rows = rows.to_vec();
        padded_rows.resize_with(count, || RmlLogRow {
            text: String::new(),
            severity: RmlLogSeverity::Info,
            selected: false,
        });
        let strings = padded_rows
            .iter()
            .map(|row| CString::new(row.text.as_str()).map_err(|_| Error::invalid_argument("text")))
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = padded_rows
            .iter()
            .zip(&strings)
            .map(|(row, text)| sys::RmlDataLogRow {
                text: text.as_ptr(),
                severity: match row.severity {
                    RmlLogSeverity::Info => 0,
                    RmlLogSeverity::Warning => 1,
                    RmlLogSeverity::Error => 2,
                },
                selected: row.selected,
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetLogRowsQuery {
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
                .DataModelSetLogRows
                .expect("DataModelSetLogRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "log-row set",
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
            .map(|(row, (label, icon, tooltip))| sys::RmlDataIconRow {
                label: label.as_ptr(),
                icon: icon.as_ptr(),
                tooltip: tooltip.as_ptr(),
                pressed: row.pressed,
                disabled: row.disabled,
            })
            .collect::<Vec<_>>();
        let empty = CStr::from_bytes_with_nul(b"\0").expect("static empty C string");
        native_rows.resize_with(count, || sys::RmlDataIconRow {
            label: empty.as_ptr(),
            icon: empty.as_ptr(),
            tooltip: empty.as_ptr(),
            pressed: false,
            disabled: false,
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

impl<'api> RmlDataChoiceRows<'api> {
    /// Replaces the label/detail choices atomically in engine-owned storage.
    pub fn set(&self, rows: &[RmlChoiceRow]) -> Result<(), Error> {
        let count = self.stable_count.update(rows.len());
        let mut padded_rows = rows.to_vec();
        padded_rows.resize_with(count, || RmlChoiceRow {
            label: String::new(),
            detail: String::new(),
            selected: false,
            highlighted: false,
        });
        let labels = padded_rows
            .iter()
            .map(|row| CString::new(row.label.as_str()).map_err(|_| Error::invalid_argument("label")))
            .collect::<Result<Vec<_>, _>>()?;
        let details = padded_rows
            .iter()
            .map(|row| CString::new(row.detail.as_str()).map_err(|_| Error::invalid_argument("detail")))
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = padded_rows
            .iter()
            .zip(&labels)
            .zip(&details)
            .map(|((row, label), detail)| sys::RmlDataChoiceRow {
                label: label.as_ptr(),
                detail: detail.as_ptr(),
                selected: row.selected,
                highlighted: row.highlighted,
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetChoiceRowsQuery {
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
                .DataModelSetChoiceRows
                .expect("DataModelSetChoiceRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "choice-row set",
            )
        }
    }
}

impl<'api> RmlDataStatusRows<'api> {
    /// Replaces the labelled statuses atomically in engine-owned storage.
    pub fn set(&self, rows: &[RmlStatusRow]) -> Result<(), Error> {
        let count = self.stable_count.update(rows.len());
        let mut padded_rows = rows.to_vec();
        padded_rows.resize_with(count, || RmlStatusRow {
            label: String::new(),
            positive: false,
        });
        let labels = padded_rows
            .iter()
            .map(|row| CString::new(row.label.as_str()).map_err(|_| Error::invalid_argument("label")))
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = padded_rows
            .iter()
            .zip(&labels)
            .map(|(row, label)| sys::RmlDataStatusRow {
                label: label.as_ptr(),
                positive: row.positive,
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetStatusRowsQuery {
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
                .DataModelSetStatusRows
                .expect("DataModelSetStatusRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "status-row set",
            )
        }
    }
}

impl<'api> RmlDataSwatchRows<'api> {
    /// Replaces colour-swatch rows atomically. The engine copies the labels
    /// and RGBA channels before this call returns, and retains invisible tail
    /// rows when a later update is shorter.
    pub fn set(&self, rows: &[RmlSwatchRow]) -> Result<(), Error> {
        let count = self.stable_count.update(rows.len());
        let mut padded_rows = rows.to_vec();
        padded_rows.resize_with(count, || RmlSwatchRow {
            label: String::new(),
            color: RmlColor {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            },
            actions_enabled: false,
        });
        let labels = padded_rows
            .iter()
            .map(|row| CString::new(row.label.as_str()).map_err(|_| Error::invalid_argument("label")))
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = padded_rows
            .iter()
            .zip(&labels)
            .map(|(row, label)| sys::RmlDataSwatchRow {
                label: label.as_ptr(),
                red: row.color.red,
                green: row.color.green,
                blue: row.color.blue,
                alpha: row.color.alpha,
                actionsEnabled: row.actions_enabled,
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetSwatchRowsQuery {
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
                .DataModelSetSwatchRows
                .expect("DataModelSetSwatchRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "swatch-row set",
            )
        }
    }
}

impl<'api> RmlDataGridRows<'api> {
    /// Replaces image-grid cells atomically. The engine copies labels, image
    /// paths, and every display flag before this call returns; a shorter later
    /// collection retains invisible tail rows for RmlUi's `data-for` lifecycle.
    pub fn set(&self, rows: &[RmlGridRow]) -> Result<(), Error> {
        let count = self.stable_count.update(rows.len());
        let mut padded_rows = rows.to_vec();
        padded_rows.resize_with(count, || RmlGridRow {
            label: String::new(),
            image: String::new(),
            cell_size: RmlPixels(0.0),
            has_image: false,
            native_image: false,
            selected: false,
            folder: false,
            filler: false,
        });
        let strings = padded_rows
            .iter()
            .map(|row| {
                Ok::<_, Error>(
                    (
                        CString::new(row.label.as_str())
                            .map_err(|_| Error::invalid_argument("label"))?,
                        CString::new(row.image.as_str())
                            .map_err(|_| Error::invalid_argument("image"))?,
                    ),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let native_rows = padded_rows
            .iter()
            .zip(&strings)
            .map(|(row, (label, image))| sys::RmlDataGridRow {
                label: label.as_ptr(),
                image: image.as_ptr(),
                cellSize: row.cell_size.0,
                hasImage: row.has_image,
                nativeImage: row.native_image,
                selected: row.selected,
                folder: row.folder,
                filler: row.filler,
            })
            .collect::<Vec<_>>();
        let query = sys::RmlDataModelSetGridRowsQuery {
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
                .DataModelSetGridRows
                .expect("DataModelSetGridRows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            require_success(
                Error::result_or(result.error, result.success)?,
                "grid-row set",
            )
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/rml_ui_generated.rs"));

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Typed Rml data bindings shared by native and Wasm. Bindings own values and
//! identity, never a Spring reference. Every operation requires a fresh scope.
//! Closing a model invalidates all its bindings. Drop does not call the engine:
//! use `close` explicitly; the engine also clears all UI owned by an unloaded module.

pub use crate::callbacks::{Event, Value};
use crate::types::rml_ui::{RmlDataFieldDef, RmlDataValue};
use crate::{ApiError, Authority, ErrorCode, Result, Spring, capabilities::Session};
use std::{
    cell::Cell,
    marker::PhantomData,
    rc::{Rc, Weak},
};

fn invalid() -> ApiError {
    ApiError::new(ErrorCode::InvalidState as i32)
}
fn success(ok: bool) -> Result<()> {
    if ok { Ok(()) } else { Err(invalid()) }
}

struct ModelState {
    owner: Weak<Authority>,
    alive: Cell<bool>,
}
impl ModelState {
    fn check<E: Session>(&self, spring: &Spring<'_, E>) -> Result<()> {
        spring.engine.check()?;
        if !self.alive.get() || !self.owner.ptr_eq(&Rc::downgrade(&spring.engine.owner)) {
            return Err(invalid());
        }
        Ok(())
    }
}

pub struct DataModel {
    context: u64,
    name: String,
    handle: u64,
    state: Rc<ModelState>,
}
impl DataModel {
    pub fn create<E: Session>(
        spring: &mut Spring<'_, E>,
        context: u64,
        name: &str,
    ) -> Result<Self> {
        let result = spring.rml_ui().context_create_data_model(context, name)?;
        success(result.success)?;
        Ok(Self {
            context,
            name: name.into(),
            handle: result.data_model_handle,
            state: Rc::new(ModelState {
                owner: Rc::downgrade(&spring.engine.owner),
                alive: Cell::new(true),
            }),
        })
    }

    pub fn bind<T: DataType, E: Session>(
        &self,
        spring: &mut Spring<'_, E>,
        name: &str,
        initial: T,
    ) -> Result<Variable<T>> {
        self.state.check(spring)?;
        let handle = T::bind(spring, self.handle, name, initial)?;
        Ok(Variable {
            handle,
            state: Rc::clone(&self.state),
            value: PhantomData,
        })
    }

    pub fn rows<E: Session>(
        &self,
        spring: &mut Spring<'_, E>,
        name: &str,
        fields: &[(&str, ValueType)],
    ) -> Result<Rows> {
        self.state.check(spring)?;
        if fields.is_empty() {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let definitions = fields
            .iter()
            .map(|(name, ty)| RmlDataFieldDef {
                name: (*name).into(),
                type_: *ty as u8,
            })
            .collect::<Vec<_>>();
        let result = spring
            .rml_ui()
            .data_model_bind_rows(self.handle, name, &definitions)?;
        success(result.success)?;
        Ok(Rows {
            handle: result.rows_handle,
            state: Rc::clone(&self.state),
            fields: fields.iter().map(|(_, ty)| *ty).collect(),
        })
    }

    pub fn event<E: Session, F>(
        &self,
        spring: &mut Spring<'_, E>,
        name: &str,
        fields: &[ValueType],
        callback: F,
    ) -> Result<EventBinding>
    where
        F: for<'call> Fn(&mut Spring<'call, E>, Event) -> Result<()> + 'static,
    {
        self.state.check(spring)?;
        let types = fields.iter().map(|ty| *ty as u8).collect::<Vec<_>>();
        let result = spring
            .rml_ui()
            .data_model_bind_event(self.handle, name, &types, callback)?;
        success(result.success)?;
        Ok(EventBinding {
            handle: result.event_handle,
            state: Rc::clone(&self.state),
        })
    }

    pub fn close<E: Session>(&self, spring: &mut Spring<'_, E>) -> Result<()> {
        self.state.check(spring)?;
        // Invalidate before a host call can reenter a listener for this model.
        self.state.alive.set(false);
        success(
            spring
                .rml_ui()
                .context_remove_data_model(self.context, &self.name)?,
        )
    }
}

/// Typed variable identity; values are copied out of the engine before return.
#[derive(Clone)]
pub struct Variable<T: DataType> {
    handle: u64,
    state: Rc<ModelState>,
    value: PhantomData<T>,
}
impl<T: DataType> Variable<T> {
    pub fn get<E: Session>(&self, spring: &mut Spring<'_, E>) -> Result<T> {
        self.state.check(spring)?;
        T::get(spring, self.handle)
    }
    pub fn set<E: Session>(&self, spring: &mut Spring<'_, E>, value: T) -> Result<()> {
        self.state.check(spring)?;
        T::set(spring, self.handle, value)
    }
}

mod sealed {
    pub trait DataType {}
}
/// The supported Rml scalar types. Sealed so transport semantics stay identical.
pub trait DataType: sealed::DataType + Sized {
    #[doc(hidden)]
    fn bind<E: Session>(
        spring: &mut Spring<'_, E>,
        model: u64,
        name: &str,
        value: Self,
    ) -> Result<u64>;
    #[doc(hidden)]
    fn get<E: Session>(spring: &mut Spring<'_, E>, handle: u64) -> Result<Self>;
    #[doc(hidden)]
    fn set<E: Session>(spring: &mut Spring<'_, E>, handle: u64, value: Self) -> Result<()>;
}

macro_rules! scalar {
    ($ty:ty, $bind:ident, $get:ident, $set:ident) => {
        impl sealed::DataType for $ty {}
        impl DataType for $ty {
            fn bind<E: Session>(
                spring: &mut Spring<'_, E>,
                model: u64,
                name: &str,
                value: Self,
            ) -> Result<u64> {
                let result = spring.rml_ui().$bind(model, name, value)?;
                success(result.success)?;
                Ok(result.variable_handle)
            }
            fn get<E: Session>(spring: &mut Spring<'_, E>, handle: u64) -> Result<Self> {
                let result = spring.rml_ui().$get(handle)?;
                success(result.success)?;
                Ok(result.value)
            }
            fn set<E: Session>(spring: &mut Spring<'_, E>, handle: u64, value: Self) -> Result<()> {
                success(spring.rml_ui().$set(handle, value)?)
            }
        }
    };
}
scalar!(
    bool,
    data_model_bind_bool,
    data_model_get_bool,
    data_model_set_bool
);
scalar!(
    i32,
    data_model_bind_int,
    data_model_get_int,
    data_model_set_int
);
scalar!(
    f32,
    data_model_bind_float,
    data_model_get_float,
    data_model_set_float
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixels(pub f32);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent(pub f32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub [u8; 4]);
macro_rules! dimension {
    ($ty:ident, $bind:ident, $get:ident, $set:ident) => {
        impl sealed::DataType for $ty {}
        impl DataType for $ty {
            fn bind<E: Session>(
                spring: &mut Spring<'_, E>,
                model: u64,
                name: &str,
                value: Self,
            ) -> Result<u64> {
                let result = spring.rml_ui().$bind(model, name, value.0)?;
                success(result.success)?;
                Ok(result.variable_handle)
            }
            fn get<E: Session>(spring: &mut Spring<'_, E>, handle: u64) -> Result<Self> {
                let result = spring.rml_ui().$get(handle)?;
                success(result.success)?;
                Ok(Self(result.value))
            }
            fn set<E: Session>(spring: &mut Spring<'_, E>, handle: u64, value: Self) -> Result<()> {
                success(spring.rml_ui().$set(handle, value.0)?)
            }
        }
    };
}
dimension!(
    Pixels,
    data_model_bind_pixels,
    data_model_get_pixels,
    data_model_set_pixels
);
dimension!(
    Percent,
    data_model_bind_percent,
    data_model_get_percent,
    data_model_set_percent
);
impl sealed::DataType for Color {}
impl DataType for Color {
    fn bind<E: Session>(
        spring: &mut Spring<'_, E>,
        model: u64,
        name: &str,
        value: Self,
    ) -> Result<u64> {
        let [r, g, b, a] = value.0;
        let result = spring
            .rml_ui()
            .data_model_bind_color(model, name, r, g, b, a)?;
        success(result.success)?;
        Ok(result.variable_handle)
    }
    fn get<E: Session>(spring: &mut Spring<'_, E>, handle: u64) -> Result<Self> {
        let result = spring.rml_ui().data_model_get_color(handle)?;
        success(result.success)?;
        Ok(Self([result.red, result.green, result.blue, result.alpha]))
    }
    fn set<E: Session>(spring: &mut Spring<'_, E>, handle: u64, value: Self) -> Result<()> {
        let [r, g, b, a] = value.0;
        success(spring.rml_ui().data_model_set_color(handle, r, g, b, a)?)
    }
}
impl sealed::DataType for String {}
impl DataType for String {
    fn bind<E: Session>(
        spring: &mut Spring<'_, E>,
        model: u64,
        name: &str,
        value: Self,
    ) -> Result<u64> {
        let result = spring
            .rml_ui()
            .data_model_bind_string(model, name, &value)?;
        success(result.success)?;
        Ok(result.variable_handle)
    }
    fn get<E: Session>(spring: &mut Spring<'_, E>, handle: u64) -> Result<Self> {
        let result = spring.rml_ui().data_model_get_string(handle)?;
        success(result.success)?;
        Ok(result.value)
    }
    fn set<E: Session>(spring: &mut Spring<'_, E>, handle: u64, value: Self) -> Result<()> {
        success(spring.rml_ui().data_model_set_string(handle, &value)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueType {
    Bool = 0,
    Int = 1,
    Float = 2,
    String = 3,
    Color = 4,
    Pixels = 5,
    Percent = 6,
}
impl Value {
    fn field_type(&self) -> ValueType {
        match self {
            Self::Bool(_) => ValueType::Bool,
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::Color(_) => ValueType::Color,
            Self::Pixels(_) => ValueType::Pixels,
            Self::Percent(_) => ValueType::Percent,
        }
    }
    fn into_raw(self) -> RmlDataValue {
        let mut raw = RmlDataValue {
            type_: self.field_type() as u8,
            ..Default::default()
        };
        match self {
            Self::Bool(v) => raw.bool_value = v,
            Self::Int(v) => raw.int_value = v,
            Self::Float(v) | Self::Pixels(v) | Self::Percent(v) => raw.float_value = v,
            Self::String(v) => raw.string_value = v,
            Self::Color([r, g, b, a]) => {
                raw.red = r;
                raw.green = g;
                raw.blue = b;
                raw.alpha = a;
            }
        }
        raw
    }
}

pub struct Rows {
    handle: u64,
    state: Rc<ModelState>,
    fields: Vec<ValueType>,
}
impl Rows {
    /// Flattened rows, in schema order. Validate the whole batch before mutation.
    pub fn set<E: Session>(&self, spring: &mut Spring<'_, E>, values: Vec<Value>) -> Result<()> {
        self.state.check(spring)?;
        if !values.len().is_multiple_of(self.fields.len())
            || values
                .iter()
                .zip(self.fields.iter().cycle())
                .any(|(v, t)| v.field_type() != *t)
        {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let values = values.into_iter().map(Value::into_raw).collect::<Vec<_>>();
        success(spring.rml_ui().data_model_set_rows(self.handle, &values)?)
    }
}

pub struct EventBinding {
    handle: u64,
    state: Rc<ModelState>,
}
impl EventBinding {
    pub fn unbind<E: Session>(&self, spring: &mut Spring<'_, E>) -> Result<()> {
        self.state.check(spring)?;
        success(spring.rml_ui().data_model_unbind_event(self.handle)?)
    }
}

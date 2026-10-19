/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Owned RmlUi events and retained, engine-thread callbacks.
use crate::{ApiError, Authority, Backend, Environment, ErrorCode, PhantomData, Result, Spring};
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Color([u8; 4]),
    Pixels(f32),
    Percent(f32),
}
impl Value {
    pub fn type_id(&self) -> u8 {
        match self {
            Self::Bool(_) => 0,
            Self::Int(_) => 1,
            Self::Float(_) => 2,
            Self::String(_) => 3,
            Self::Color(_) => 4,
            Self::Pixels(_) => 5,
            Self::Percent(_) => 6,
        }
    }
}

/// Values are copied before entering user code, so a nested engine call
/// cannot invalidate the event's strings or argument array.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub event_handle: u64,
    pub target_element_handle: u64,
    pub values: Vec<Value>,
}

#[cfg(target_arch = "wasm32")]
type Handler = dyn Fn(Option<Event>) -> Result<()>;
#[cfg(not(target_arch = "wasm32"))]
type Handler = dyn Fn(Option<Event>) -> Result<()> + Send;
struct Node {
    owner: crate::Shared<Authority>,
    handler: Box<Handler>,
}
#[cfg(target_arch = "wasm32")]
thread_local! {static REGISTRY:RefCell<BTreeMap<u32,crate::Shared<Node>>>=const {RefCell::new(BTreeMap::new())};}
#[cfg(not(target_arch = "wasm32"))]
static REGISTRY: std::sync::Mutex<BTreeMap<u32, crate::Shared<Node>>> =
    std::sync::Mutex::new(BTreeMap::new());

// Handlers may move with their module (Send), but are called only under their
// owner's reentrant gate. No registry lock is held while invoking/dropping one.
#[cfg(not(target_arch = "wasm32"))]
unsafe impl Sync for Node {}
fn registry<R>(access: impl FnOnce(&mut BTreeMap<u32, crate::Shared<Node>>) -> R) -> R {
    #[cfg(target_arch = "wasm32")]
    {
        REGISTRY.with(|r| access(&mut r.borrow_mut()))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        access(&mut REGISTRY.lock().unwrap_or_else(|p| p.into_inner()))
    }
}
// IDs are never reused, including across module reloads and engine threads.
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
#[cfg(target_arch = "wasm32")]
const INVOKE: u32 = 0x5344_4b45;
#[cfg(target_arch = "wasm32")]
const DESTROY: u32 = 0x5344_4b44;

pub(crate) fn register<E: Environment, F>(
    engine: &Backend<'_>,
    schema: Option<&[u8]>,
    handler: F,
) -> Result<u32>
where
    F: for<'call> Fn(&mut Spring<'call, E>, Event) -> Result<()> + crate::RetainedCapture + 'static,
{
    engine.check()?;
    if schema.is_some_and(|s| s.iter().any(|&t| t > 6)) {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let mut id = NEXT_ID.load(Ordering::Relaxed);
    loop {
        let next = id
            .checked_add(1)
            .ok_or(ApiError::new(ErrorCode::OutOfBounds as i32))?;
        match NEXT_ID.compare_exchange_weak(id, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(current) => id = current,
        }
    }
    let owner = engine
        .owner
        .identity
        .upgrade()
        .ok_or(ApiError::new(ErrorCode::InvalidState as i32))?;
    let callback_owner = crate::Shared::clone(&owner);
    let schema = schema.map(<[u8]>::to_vec);
    // Keep the Rml reader in the registered closure. The common dispatcher
    // must not pull unsynced Rml imports into a synced Wasm module which has
    // no Rml registrations.
    let callback = Box::new(move |event: Option<Event>| {
        let backend = Backend {
            owner: &callback_owner,
            scope: PhantomData,
        };
        backend.check()?;
        let event = match event {
            Some(event) => event,
            #[cfg(target_arch = "wasm32")]
            None => wasm_event(schema.as_deref())?,
            #[cfg(not(target_arch = "wasm32"))]
            None => return Err(ApiError::new(ErrorCode::InvalidState as i32)),
        };
        validate_event(schema.as_deref(), &event)?;
        let result = backend.with_nested::<E, _>(|spring| handler(spring, event));
        backend.finish(result)
    });
    registry(|r| {
        r.insert(
            id,
            crate::Shared::new(Node {
                owner,
                handler: callback,
            }),
        )
    });
    Ok(id)
}

pub(crate) fn remove(id: u32) {
    let node = registry(|r| r.remove(&id));
    drop(node);
}

pub(crate) fn clear_owner(owner: &crate::Shared<Authority>) {
    let nodes = registry(|r| {
        let ids = r
            .iter()
            .filter(|(_, n)| crate::Shared::ptr_eq(&n.owner, owner))
            .map(|(&id, _)| id)
            .collect::<Vec<_>>();
        ids.into_iter()
            .filter_map(|id| r.remove(&id))
            .collect::<Vec<_>>()
    });
    drop(nodes);
}
fn get(id: u32) -> Result<crate::Shared<Node>> {
    registry(|r| r.get(&id).cloned()).ok_or(ApiError::new(ErrorCode::InvalidState as i32))
}

fn validate_event(schema: Option<&[u8]>, event: &Event) -> Result<()> {
    if let Some(schema) = schema
        && (schema.len() != event.values.len()
            || schema
                .iter()
                .zip(&event.values)
                .any(|(&t, v)| t != v.type_id()))
    {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn invoke(node: &Node, event: Event) -> Result<()> {
    (node.handler)(Some(event))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn wire(id: u32) -> spring_sdk_internal::RetainedCallback {
    spring_sdk_internal::RetainedCallback::new(INVOKE, id, DESTROY)
}

#[cfg(target_arch = "wasm32")]
fn wasm_event(schema: Option<&[u8]>) -> Result<Event> {
    if schema.is_none() {
        let event = spring_sdk_internal::generated::owned::rml_ui::event_get_current(0)?;
        if !event.exists {
            return Err(ApiError::new(ErrorCode::NotFound as i32));
        }
        return Ok(Event {
            event_handle: event.event_handle,
            target_element_handle: event.element_handle,
            values: Vec::new(),
        });
    }
    let event = spring_sdk_internal::data_model_current_event()?;
    let count = usize::try_from(event.value_count)
        .map_err(|_| ApiError::new(ErrorCode::OutOfBounds as i32))?;
    if schema.unwrap().len() != count {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let mut values = Vec::with_capacity(count);
    let mut string_buffer = Vec::new();
    for index in 0..count {
        loop {
            use spring_sdk_internal::DataEventValue as V;
            match spring_sdk_internal::data_model_current_value(index as u64, &mut string_buffer) {
                Ok(value) => {
                    values.push(match value {
                        V::Bool(v) => Value::Bool(v),
                        V::Int(v) => Value::Int(v),
                        V::Float(v) => Value::Float(v),
                        V::String(v) => Value::String(v.into()),
                        V::Color(v) => Value::Color(v),
                        V::Pixels(v) => Value::Pixels(v),
                        V::Percent(v) => Value::Percent(v),
                    });
                    break;
                }
                Err(spring_sdk_internal::DataEventValueError::BufferTooSmall { required }) => {
                    string_buffer.resize(required as usize, 0)
                }
                Err(spring_sdk_internal::DataEventValueError::Api(error)) => return Err(error),
            }
        }
    }
    Ok(Event {
        event_handle: event.event_handle,
        target_element_handle: event.target_element_handle,
        values,
    })
}

#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub fn __dispatch(callback_id: u32, id: u32) {
    if callback_id == spring_sdk_internal::callback::SYNC_CLOSURE_CALLBACK_ID {
        if spring_sdk_internal::callback::dispatch_sync_closure() {
            return;
        }
        panic!("Spring synchronous callback reentered its active closure");
    }
    if callback_id == DESTROY {
        remove(id);
        return;
    }
    let result = (|| {
        if callback_id != INVOKE {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let node = get(id)?;
        (node.handler)(None)
    })();
    if let Err(error) = result {
        panic!("Spring callback failed with code {}", error.code);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn native_boundary(id: u32, event: impl FnOnce(&Backend<'_>) -> Result<Event>) {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let Ok(node) = get(id) else {
        return;
    };
    let Ok(_entry) = node.owner.gate.enter() else {
        node.owner.callback_failed.store(true, Ordering::Relaxed);
        return;
    };
    let engine = Backend {
        owner: &node.owner,
        scope: PhantomData,
    };
    let result = catch_unwind(AssertUnwindSafe(|| {
        engine.check()?;
        invoke(&node, event(&engine)?)
    }));
    match result {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            node.owner.error.set(Some(error));
            node.owner.callback_failed.store(true, Ordering::Relaxed);
        }
        Err(panic) => {
            *node.owner.panic.borrow_mut() = Some(panic);
            node.owner.callback_failed.store(true, Ordering::Relaxed);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) unsafe extern "C" fn native_event(data: *mut core::ffi::c_void) {
    native_boundary(data as usize as u32, |engine| {
        let event = crate::native::rml_ui::event_get_current(engine, unsafe {
            engine.owner.native_tables.rml_ui.as_ref()
        })?;
        if !event.exists {
            return Err(ApiError::new(ErrorCode::NotFound as i32));
        }
        Ok(Event {
            event_handle: event.event_handle,
            target_element_handle: event.element_handle,
            values: Vec::new(),
        })
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) unsafe extern "C" fn native_data(
    data: *mut core::ffi::c_void,
    args: *const spring_native_sys::RmlDataEventArgs,
) {
    native_boundary(data as usize as u32, |_| {
        let args =
            unsafe { args.as_ref() }.ok_or(ApiError::new(ErrorCode::InvalidArgument as i32))?;
        let count = usize::try_from(args.count)
            .map_err(|_| ApiError::new(ErrorCode::OutOfBounds as i32))?;
        let raw = unsafe { crate::native::read_slice(args.values, count)? };
        let mut values = Vec::with_capacity(count);
        for value in raw {
            values.push(match value.type_ {
                0 => Value::Bool(value.boolValue),
                1 => Value::Int(value.intValue),
                2 => Value::Float(value.floatValue),
                3 => {
                    let text = if value.stringValue.is_null() {
                        ""
                    } else {
                        unsafe { std::ffi::CStr::from_ptr(value.stringValue) }
                            .to_str()
                            .map_err(|_| ApiError::new(ErrorCode::Internal as i32))?
                    };
                    Value::String(text.into())
                }
                4 => Value::Color([value.red, value.green, value.blue, value.alpha]),
                5 => Value::Pixels(value.floatValue),
                6 => Value::Percent(value.floatValue),
                _ => return Err(ApiError::new(ErrorCode::InvalidArgument as i32)),
            });
        }
        Ok(Event {
            event_handle: args.eventHandle,
            target_element_handle: args.targetElementHandle,
            values,
        })
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) unsafe extern "C" fn native_destroy(data: *mut core::ffi::c_void) {
    // Dropping a captured object may panic. Keep that panic inside Rust and
    // surface it at the next scope boundary after the host has cleaned up.
    let id = data as usize as u32;
    let Ok(node) = get(id) else {
        return;
    };
    let owner = crate::Shared::clone(&node.owner);
    let Ok(_entry) = owner.gate.enter() else {
        owner.callback_failed.store(true, Ordering::Relaxed);
        return;
    };
    drop(node);
    if let Err(panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| remove(id))) {
        *owner.panic.borrow_mut() = Some(panic);
        owner.callback_failed.store(true, Ordering::Relaxed);
    }
}

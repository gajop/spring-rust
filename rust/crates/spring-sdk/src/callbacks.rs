/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Owned RmlUi events and retained, engine-thread callbacks.
use crate::{ApiError, Authority, Backend, Environment, ErrorCode, PhantomData, Result, Spring};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
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

type Handler = dyn Fn(Option<Event>) -> Result<()>;
struct Node {
    owner: Rc<Authority>,
    handler: Box<Handler>,
}
thread_local! {static REGISTRY:RefCell<BTreeMap<u32,Rc<Node>>>=const {RefCell::new(BTreeMap::new())};}
// Never reuse an ID, including across threads. A stale/foreign native callback
// cannot accidentally select a different thread's listener.
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
#[cfg(not(target_arch = "wasm32"))]
type Origin = (
    std::thread::ThreadId,
    std::sync::Weak<std::sync::atomic::AtomicBool>,
);
#[cfg(not(target_arch = "wasm32"))]
static ORIGINS: std::sync::Mutex<BTreeMap<u32, Origin>> = std::sync::Mutex::new(BTreeMap::new());

#[cfg(not(target_arch = "wasm32"))]
fn record_foreign(id: u32) {
    let origins = ORIGINS.lock().unwrap_or_else(|p| p.into_inner());
    if let Some((thread, flag)) = origins.get(&id)
        && *thread != std::thread::current().id()
        && let Some(flag) = flag.upgrade()
    {
        flag.store(true, Ordering::Relaxed);
    }
}
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
    F: for<'call> Fn(&mut Spring<'call, E>, Event) -> Result<()> + 'static,
{
    engine.check()?;
    if schema.is_some_and(|s| s.iter().any(|&t| t > 6)) {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let id = NEXT_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
        .map_err(|_| ApiError::new(ErrorCode::OutOfBounds as i32))?;
    let owner = Rc::clone(&engine.owner);
    #[cfg(not(target_arch = "wasm32"))]
    ORIGINS.lock().unwrap_or_else(|p| p.into_inner()).insert(
        id,
        (
            owner.thread,
            std::sync::Arc::downgrade(&owner.foreign_callback),
        ),
    );
    let callback_owner = Rc::clone(&owner);
    let schema = schema.map(<[u8]>::to_vec);
    // Keep the Rml reader in the registered closure. The common dispatcher
    // must not pull unsynced Rml imports into a synced Wasm module which has
    // no Rml registrations.
    let callback = Box::new(move |event: Option<Event>| {
        let backend = Backend {
            owner: Rc::clone(&callback_owner),
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
    REGISTRY.with(|r| {
        r.borrow_mut().insert(
            id,
            Rc::new(Node {
                owner,
                handler: callback,
            }),
        )
    });
    Ok(id)
}

pub(crate) fn remove(id: u32) {
    let node = REGISTRY.with(|r| r.borrow_mut().remove(&id));
    #[cfg(not(target_arch = "wasm32"))]
    ORIGINS
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .remove(&id);
    drop(node); // User captures are dropped after releasing the registry borrow.
}

pub(crate) fn clear_owner(owner: &Rc<Authority>) {
    let nodes = REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        let ids = r
            .iter()
            .filter(|(_, n)| Rc::ptr_eq(&n.owner, owner))
            .map(|(&id, _)| id)
            .collect::<Vec<_>>();
        ids.into_iter()
            .filter_map(|id| r.remove(&id).map(|node| (id, node)))
            .collect::<Vec<_>>()
    });
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut origins = ORIGINS.lock().unwrap_or_else(|p| p.into_inner());
        for (id, _) in &nodes {
            origins.remove(id);
        }
    }
    drop(nodes);
}

fn get(id: u32) -> Result<Rc<Node>> {
    REGISTRY
        .with(|r| r.borrow().get(&id).cloned())
        .ok_or(ApiError::new(ErrorCode::InvalidState as i32))
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
pub(crate) fn wire(id: u32) -> spring::RetainedCallback {
    spring::RetainedCallback::new(INVOKE, id, DESTROY)
}

#[cfg(target_arch = "wasm32")]
fn wasm_event(schema: Option<&[u8]>) -> Result<Event> {
    if schema.is_none() {
        let event = spring::generated::owned::rml_ui::event_get_current(0)?;
        if !event.exists {
            return Err(ApiError::new(ErrorCode::NotFound as i32));
        }
        return Ok(Event {
            event_handle: event.event_handle,
            target_element_handle: event.element_handle,
            values: Vec::new(),
        });
    }
    let event = spring::data_model_current_event()?;
    let count = usize::try_from(event.value_count)
        .map_err(|_| ApiError::new(ErrorCode::OutOfBounds as i32))?;
    if schema.unwrap().len() != count {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let mut values = Vec::with_capacity(count);
    let mut string_buffer = Vec::new();
    for index in 0..count {
        loop {
            use spring::DataEventValue as V;
            match spring::data_model_current_value(index as u64, &mut string_buffer) {
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
                Err(spring::DataEventValueError::BufferTooSmall { required }) => {
                    string_buffer.resize(required as usize, 0)
                }
                Err(spring::DataEventValueError::Api(error)) => return Err(error),
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
    if callback_id == spring::callback::SYNC_CLOSURE_CALLBACK_ID {
        if spring::callback::dispatch_sync_closure() {
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
        record_foreign(id);
        return;
    };
    let engine = Backend {
        owner: Rc::clone(&node.owner),
        scope: PhantomData,
    };
    let result = catch_unwind(AssertUnwindSafe(|| {
        engine.check()?;
        invoke(&node, event(&engine)?)
    }));
    match result {
        Ok(Ok(())) => {}
        Ok(Err(error)) => node.owner.error.set(Some(error)),
        Err(panic) => *node.owner.panic.borrow_mut() = Some(panic),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) unsafe extern "C" fn native_event(data: *mut core::ffi::c_void) {
    native_boundary(data as usize as u32, |engine| {
        let event = crate::native::rml_ui::event_get_current(engine)?;
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
        record_foreign(id);
        return;
    };
    let owner = Rc::clone(&node.owner);
    drop(node);
    if let Err(panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| remove(id))) {
        *owner.panic.borrow_mut() = Some(panic);
    }
}

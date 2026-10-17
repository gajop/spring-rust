/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use crate::{ApiError, ErrorCode, Result};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{any::Any, ffi::c_char};
use std::ffi::{CStr, CString};

/// Own every pointer passed to a synchronous host call, including nested
/// lists/strings. Host result data is copied before these inputs are dropped.
#[derive(Default)]
struct Inputs {
    storage: Vec<Box<dyn Any>>,
}
impl Inputs {
    fn value<T: 'static>(&mut self, value: T) -> *const T {
        let value = Box::new(value);
        let ptr = &*value as *const T;
        self.storage.push(value);
        ptr
    }
    fn slice<T: 'static>(&mut self, value: Vec<T>) -> *const T {
        let ptr = value.as_ptr();
        self.storage.push(Box::new(value));
        ptr
    }
    fn string(&mut self, value: &str) -> Result<*const c_char> {
        let value =
            CString::new(value).map_err(|_| ApiError::new(ErrorCode::InvalidArgument as i32))?;
        let ptr = value.as_ptr();
        self.storage.push(Box::new(value));
        Ok(ptr)
    }
}

pub(crate) unsafe fn read_slice<'a, T>(ptr: *const T, count: usize) -> Result<&'a [T]> {
    if count == 0 {
        return Ok(&[]);
    }
    if ptr.is_null() || count > isize::MAX as usize / core::mem::size_of::<T>().max(1) {
        return Err(ApiError::new(ErrorCode::InvalidState as i32));
    }
    Ok(unsafe { core::slice::from_raw_parts(ptr, count) })
}

unsafe fn read_string(ptr: *const c_char, length: Option<usize>) -> Result<String> {
    let bytes = if let Some(length) = length {
        unsafe { read_slice(ptr.cast(), length)? }
    } else if ptr.is_null() {
        &[]
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_bytes()
    };
    // Preserve the semantic facade's existing byte fallback on both targets.
    Ok(match core::str::from_utf8(bytes) {
        Ok(s) => String::from(s),
        Err(_) => bytes.iter().copied().map(char::from).collect(),
    })
}

pub(crate) unsafe fn check_error(error: *const spring_native_sys::Error) -> Result<()> {
    if let Some(error) = unsafe { error.as_ref() } {
        Err(ApiError::new(error.code))
    } else {
        Ok(())
    }
}

/// The closure is absent from its slot while running. A recursive invocation
/// of the same callback fails instead of aliasing `&mut F`. No state reference
/// or lock is held across user code, including nested engine callbacks.
fn invoke_sync<E: crate::Environment, F, R>(
    engine: &crate::Backend<'_>,
    callback: F,
    host: impl FnOnce(spring_native_sys::NativeCallback, *mut core::ffi::c_void) -> Result<R>,
) -> Result<R>
where
    F: for<'call> FnMut(&mut crate::Spring<'call, E>),
{
    use std::{
        panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
        sync::atomic::{AtomicBool, Ordering},
    };
    struct State<'a, 'call, F> {
        engine: &'a crate::Backend<'call>,
        callback: Option<F>,
        panic: Option<Box<dyn Any + Send>>,
        thread: std::thread::ThreadId,
        failed: AtomicBool,
    }
    unsafe extern "C" fn run<E: crate::Environment, F>(data: *mut core::ffi::c_void)
    where
        F: for<'call> FnMut(&mut crate::Spring<'call, E>),
    {
        let state = data.cast::<State<'_, '_, F>>();
        // Read only the immutable affinity field before touching the closure.
        if std::thread::current().id() != unsafe { ptr::addr_of!((*state).thread).read() } {
            unsafe { &*ptr::addr_of!((*state).failed) }.store(true, Ordering::Relaxed);
            return;
        }
        if unsafe { &*ptr::addr_of!((*state).failed) }.load(Ordering::Relaxed) {
            return;
        }
        let callback = unsafe { ptr::addr_of_mut!((*state).callback).replace(None) };
        let Some(mut callback) = callback else {
            unsafe { &*ptr::addr_of!((*state).failed) }.store(true, Ordering::Relaxed);
            return;
        };
        let engine = unsafe { ptr::addr_of!((*state).engine).read() };
        let result = catch_unwind(AssertUnwindSafe(|| {
            engine.with_nested::<E, _>(|spring| callback(spring))
        }));
        unsafe {
            ptr::addr_of_mut!((*state).callback).write(Some(callback));
        }
        if let Err(payload) = result {
            unsafe { &*ptr::addr_of!((*state).failed) }.store(true, Ordering::Relaxed);
            unsafe {
                ptr::addr_of_mut!((*state).panic).write(Some(payload));
            }
        }
    }
    use core::ptr;
    let mut state = State {
        engine,
        callback: Some(callback),
        panic: None,
        thread: std::thread::current().id(),
        failed: AtomicBool::new(false),
    };
    let result = host(Some(run::<E, F>), ptr::from_mut(&mut state).cast());
    // Let the engine finish restoring its scoped state before propagating a
    // Rust panic. Never unwind through a C callback boundary.
    if let Some(payload) = state.panic {
        resume_unwind(payload);
    }
    if state.failed.load(Ordering::Relaxed) {
        return Err(ApiError::new(ErrorCode::InvalidState as i32));
    }
    result
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_native.rs"
));

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_native_callins.rs"
));

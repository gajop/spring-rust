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
        if value.is_empty() {
            return Ok(c"".as_ptr());
        }
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

// Callback inputs are snapshots: a reentrant engine call may replace the
// original command queue or scratch string. Common inputs fit on the stack;
// larger inputs reuse separate buffers, including during nested callbacks.
thread_local! {
    static BYTE_INPUTS: std::cell::RefCell<Vec<Vec<u8>>> = const { std::cell::RefCell::new(Vec::new()) };
    static FLOAT_INPUTS: std::cell::RefCell<Vec<Vec<f32>>> = const { std::cell::RefCell::new(Vec::new()) };
}
trait InputElement: Copy {
    fn take_buffer() -> Vec<Self>;
    fn return_buffer(value: Vec<Self>);
}
macro_rules! input_element {
    ($ty:ty,$pool:ident) => {
        impl InputElement for $ty {
            #[cold]
            #[inline(never)]
            fn take_buffer() -> Vec<Self> {
                $pool.with(|pool| pool.borrow_mut().pop().unwrap_or_default())
            }
            #[cold]
            #[inline(never)]
            fn return_buffer(mut value: Vec<Self>) {
                if value.capacity() > 16 * 1024 / core::mem::size_of::<Self>() {
                    return;
                }
                value.clear();
                $pool.with(|pool| {
                    let mut pool = pool.borrow_mut();
                    if pool.len() < 8 {
                        pool.push(value);
                    }
                });
            }
        }
    };
}
input_element!(u8, BYTE_INPUTS);
input_element!(f32, FLOAT_INPUTS);
struct InputBuffer<T: InputElement, const N: usize> {
    inline: [core::mem::MaybeUninit<T>; N],
    spill: Vec<T>,
    len: usize,
}
impl<T: InputElement, const N: usize> InputBuffer<T, N> {
    #[inline]
    fn copy(value: &[T]) -> Self {
        let mut result = Self {
            inline: [const { core::mem::MaybeUninit::uninit() }; N],
            spill: Vec::new(),
            len: value.len(),
        };
        if value.len() <= N {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    value.as_ptr(),
                    result.inline.as_mut_ptr().cast::<T>(),
                    value.len(),
                )
            };
        } else {
            result.spill = T::take_buffer();
            result.spill.extend_from_slice(value);
        }
        result
    }
    #[inline]
    fn as_slice(&self) -> &[T] {
        if self.len <= N {
            // copy initialized exactly this prefix; unused inline slots stay uninitialized.
            unsafe { core::slice::from_raw_parts(self.inline.as_ptr().cast::<T>(), self.len) }
        } else {
            &self.spill
        }
    }
}
impl<T: InputElement, const N: usize> Drop for InputBuffer<T, N> {
    #[inline]
    fn drop(&mut self) {
        if self.spill.capacity() != 0 {
            T::return_buffer(core::mem::take(&mut self.spill));
        }
    }
}
pub struct InputString(InputBuffer<u8, 64>);
impl AsRef<str> for InputString {
    fn as_ref(&self) -> &str {
        // Constructors copy validated UTF-8, or encode the byte fallback.
        unsafe { core::str::from_utf8_unchecked(self.0.as_slice()) }
    }
}
unsafe fn read_callin_string(ptr: *const c_char, length: Option<usize>) -> Result<InputString> {
    let bytes = if let Some(length) = length {
        unsafe { read_slice(ptr.cast(), length)? }
    } else if ptr.is_null() {
        &[]
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_bytes()
    };
    let result = match core::str::from_utf8(bytes) {
        Ok(_) => InputBuffer::copy(bytes),
        Err(_) => {
            let value: String = bytes.iter().copied().map(char::from).collect();
            InputBuffer::copy(value.as_bytes())
        }
    };
    Ok(InputString(result))
}
pub struct CommandInput {
    raw: spring_native_sys::NativeCallinCommand,
    params: InputBuffer<f32, 8>,
}
impl<'a> From<&'a CommandInput> for crate::CallinCommand<'a> {
    fn from(value: &'a CommandInput) -> Self {
        Self {
            id: value.raw.id,
            time_out: value.raw.timeOut,
            page_index: value.raw.pageIndex,
            num_params: value.raw.numParams,
            tag: value.raw.tag,
            options: value.raw.options,
            params: value.params.as_slice(),
        }
    }
}
unsafe fn read_callin_command(
    value: &spring_native_sys::NativeCallinCommand,
) -> Result<CommandInput> {
    let params = unsafe { read_slice(value.params, value.numParams as usize)? };
    Ok(CommandInput {
        raw: *value,
        params: InputBuffer::copy(params),
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

#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn callback_commands_survive_host_reuse_and_nested_inputs() {
        for count in [0, 4, 64, 1024] {
            for _ in 0..3 {
                let mut params = (0..count).map(|i| i as f32).collect::<Vec<_>>();
                let raw = spring_native_sys::NativeCallinCommand {
                    id: 17,
                    numParams: count as u32,
                    params: params.as_ptr(),
                    ..Default::default()
                };
                let outer = unsafe { read_callin_command(&raw) }.unwrap();
                let expected = params.clone();
                params.fill(-1.0);
                let nested = unsafe { read_callin_command(&raw) }.unwrap();
                let view = crate::CallinCommand::from(&outer);
                assert_eq!(view.params, expected);
                assert_eq!(crate::CallinCommand::from(&nested).params, params);
                let retained = view.to_owned();
                drop(nested);
                drop(outer);
                assert_eq!(retained.params, expected);
            }
        }
    }

    #[test]
    fn callback_strings_preserve_utf8_and_byte_fallback_after_host_reuse() {
        for value in ["text".to_owned(), "船🚀".repeat(100)] {
            let mut bytes = value.as_bytes().to_vec();
            let input =
                unsafe { read_callin_string(bytes.as_ptr().cast(), Some(bytes.len())) }.unwrap();
            bytes.fill(b'x');
            let nested =
                unsafe { read_callin_string(bytes.as_ptr().cast(), Some(bytes.len())) }.unwrap();
            assert_eq!(input.as_ref(), value);
            assert_eq!(nested.as_ref(), core::str::from_utf8(&bytes).unwrap());
        }
        let bytes = [0xffu8, 0, 0xfe];
        let input =
            unsafe { read_callin_string(bytes.as_ptr().cast(), Some(bytes.len())) }.unwrap();
        assert_eq!(input.as_ref(), "ÿ\0þ");
    }
}

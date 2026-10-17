/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::host::{CusCallResult, CusModule};
use crate::{ApiError, ErrorCode, Result, Spring, capabilities::Synced};

fn invalid() -> ApiError {
    ApiError::new(ErrorCode::InvalidArgument as i32)
}

#[cfg(not(target_arch = "wasm32"))]
pub unsafe fn native_invoke<E: Synced>(
    host: &impl CusModule<E>,
    spring: &Spring<'_, E>,
    query: *const crate::__sys::CusInvokeQuery,
    output: &mut crate::__sys::CusInvokeResult,
) -> Result<()> {
    let query = unsafe { query.as_ref() }.ok_or_else(invalid)?;
    let floats =
        unsafe { crate::native::read_slice(query.floatArguments, query.floatCount as usize)? }
            .to_vec();
    let integers =
        unsafe { crate::native::read_slice(query.integerArguments, query.integerCount as usize)? }
            .to_vec();
    let values = unsafe { mutable_slice(output.intValues, output.intCapacity as usize)? };
    let mut result = CusCallResult {
        int_value: -1,
        float_value: 1.0,
        bool_value: false,
        complete: false,
        int_count: 0,
        int_values: values,
    };
    output.handled = host.cus_invoke(
        spring,
        query.instanceID,
        query.call,
        &floats,
        &integers,
        &mut result,
    )? as u8;
    if result.int_count > result.int_values.len() {
        return Err(invalid());
    }
    output.intValue = result.int_value;
    output.floatValue = result.float_value;
    output.boolValue = result.bool_value as u8;
    output.complete = result.complete as u8;
    output.intCount = result.int_count.try_into().map_err(|_| invalid())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub unsafe fn native_named<E: Synced>(
    host: &impl CusModule<E>,
    spring: &Spring<'_, E>,
    query: *const crate::__sys::CusNamedQuery,
    output: &mut crate::__sys::CusNamedResult,
) -> Result<()> {
    let query = unsafe { query.as_ref() }.ok_or_else(invalid)?;
    if query.functionName.is_null() {
        return Err(invalid());
    }
    let name = unsafe { std::ffi::CStr::from_ptr(query.functionName) }
        .to_str()
        .map_err(|_| invalid())?;
    let args = unsafe { crate::native::read_slice(query.arguments, query.argumentCount as usize)? }
        .to_vec();
    let values = unsafe { mutable_slice(query.returnValues, query.returnCapacity as usize)? };
    let mut found = false;
    let returned =
        host.cus_call_named(spring, query.instanceID, name, &args, values, &mut found)?;
    output.handled = returned.is_some() as u8;
    output.functionFound = found as u8;
    let count = returned.unwrap_or(0);
    if count > values.len() {
        return Err(invalid());
    }
    output.returnCount = count.try_into().map_err(|_| invalid())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn mutable_slice<'a, T>(pointer: *mut T, count: usize) -> Result<&'a mut [T]> {
    if count == 0 {
        return Ok(&mut []);
    }
    if pointer.is_null() || count > isize::MAX as usize / core::mem::size_of::<T>() {
        return Err(invalid());
    }
    Ok(unsafe { core::slice::from_raw_parts_mut(pointer, count) })
}

#[cfg(target_arch = "wasm32")]
pub mod wire {
    use super::*;
    pub use spring::cus::core_module::{BUFFER_SIZE, MAX_RESULTS, buffer_range_contains};

    unsafe fn values<T: Copy>(pointer: i32, count: i32, base: usize) -> Result<Vec<T>> {
        if pointer < 0 || count < 0 || count > 256 {
            return Err(invalid());
        }
        let bytes = count as usize * core::mem::size_of::<T>();
        if pointer as usize % core::mem::align_of::<T>() != 0
            || !buffer_range_contains(pointer, bytes, base)
        {
            return Err(invalid());
        }
        Ok(unsafe { core::slice::from_raw_parts(pointer as *const T, count as usize) }.to_vec())
    }
    unsafe fn output(pointer: i32, bytes: usize, base: usize) -> Result<*mut u8> {
        if pointer < 0 || pointer % 4 != 0 || !buffer_range_contains(pointer, bytes, base) {
            return Err(invalid());
        }
        Ok(pointer as *mut u8)
    }
    pub unsafe fn invoke<E: Synced>(
        host: &impl CusModule<E>,
        spring: &Spring<'_, E>,
        base: usize,
        instance: i32,
        call: i32,
        float_pointer: i32,
        float_count: i32,
        int_pointer: i32,
        int_count: i32,
        result_pointer: i32,
    ) -> Result<()> {
        let floats = unsafe { values::<f32>(float_pointer, float_count, base)? };
        let integers = unsafe { values::<i32>(int_pointer, int_count, base)? };
        let output = unsafe { output(result_pointer, 540, base)? };
        let values =
            unsafe { core::slice::from_raw_parts_mut(output.add(20).cast::<i32>(), MAX_RESULTS) };
        let mut result = CusCallResult {
            int_value: -1,
            float_value: 1.0,
            bool_value: false,
            complete: false,
            int_count: 0,
            int_values: values,
        };
        let handled = host.cus_invoke(
            spring,
            instance as u32,
            call as u32,
            &floats,
            &integers,
            &mut result,
        )?;
        if result.int_count > result.int_values.len() {
            return Err(invalid());
        }
        unsafe {
            output.cast::<i32>().write(result.int_value);
            output.add(4).cast::<f32>().write(result.float_value);
            output.add(8).cast::<u32>().write(result.bool_value as u32);
            output.add(12).cast::<u32>().write(result.complete as u32);
            output.add(16).cast::<u32>().write(result.int_count as u32);
            output.add(536).cast::<u32>().write(handled as u32);
        }
        Ok(())
    }
    pub unsafe fn named<E: Synced>(
        host: &impl CusModule<E>,
        spring: &Spring<'_, E>,
        base: usize,
        instance: i32,
        name_pointer: i32,
        name_length: i32,
        arg_pointer: i32,
        arg_count: i32,
        result_pointer: i32,
    ) -> Result<()> {
        if name_pointer < 0
            || name_length < 0
            || !buffer_range_contains(name_pointer, name_length as usize, base)
        {
            return Err(invalid());
        }
        let bytes =
            unsafe { core::slice::from_raw_parts(name_pointer as *const u8, name_length as usize) };
        let name = core::str::from_utf8(bytes)
            .map_err(|_| invalid())?
            .to_owned();
        let arguments = unsafe { values::<f32>(arg_pointer, arg_count, base)? };
        let output = unsafe { output(result_pointer, 268, base)? };
        let values =
            unsafe { core::slice::from_raw_parts_mut(output.add(8).cast::<f32>(), MAX_RESULTS) };
        let mut found = false;
        let count = host.cus_call_named(
            spring,
            instance as u32,
            &name,
            &arguments,
            values,
            &mut found,
        )?;
        if count.is_some_and(|n| n > values.len()) {
            return Err(invalid());
        }
        unsafe {
            output.cast::<u32>().write(count.unwrap_or(0) as u32);
            output.add(4).cast::<u32>().write(found as u32);
            output.add(264).cast::<u32>().write(count.is_some() as u32);
        }
        Ok(())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cus_enter {
    ($module:expr, $spring:expr, $accessor:path, $call:expr) => {
        ($call)($accessor($module), $spring)
    };
    ($module:expr, $spring:expr, $accessor:path, $call:expr, $entry:path) => {
        $entry($module, $spring, $call)
    };
}

/// Export a CUS host owned by an existing module. The accessor returns a
/// shared host reference; no whole-host borrow crosses a script or engine call.
#[macro_export]
macro_rules! export_core_cus {
    ($slot:ident, $host:ty, $accessor:path, $environment:ty $(, $entry:path)?) => {
        #[allow(dead_code)]
        pub fn with_cus_module<R>(
            f: impl FnOnce(&$host, &mut $crate::Spring<'_, $environment>) -> $crate::Result<R>,
        ) -> $crate::Result<R> {
            $slot.with(|slot| slot.invoke(|module, spring| $crate::__cus_enter!(module, spring, $accessor, f $(, $entry)?)))
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn RustCUSInvoke(
            _interface: *const $crate::__sys::NativeInterface,
            data: *mut core::ffi::c_void,
            query: *const $crate::__sys::CusInvokeQuery,
            result: *mut $crate::__sys::CusInvokeResult,
        ) {
            let Some(result) = (unsafe { result.as_mut() }) else {
                return;
            };
            result.handled = 0;
            result.intCount = 0;
            result.error = $crate::module::error_pointer(unsafe {
                $slot.with_host(|slot| {
                    slot.validate_identity(data)?;
                    slot.invoke(|module, spring| $crate::__cus_enter!(module, spring, $accessor, |host, spring| unsafe {
                        $crate::cus::exports::native_invoke(
                            host,
                            spring,
                            query,
                            result,
                        )
                    } $(, $entry)?))
                })
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn RustCUSCallNamed(
            _interface: *const $crate::__sys::NativeInterface,
            data: *mut core::ffi::c_void,
            query: *const $crate::__sys::CusNamedQuery,
            result: *mut $crate::__sys::CusNamedResult,
        ) {
            let Some(result) = (unsafe { result.as_mut() }) else {
                return;
            };
            result.handled = 0;
            result.functionFound = 0;
            result.returnCount = 0;
            result.error = $crate::module::error_pointer(unsafe {
                $slot.with_host(|slot| {
                    slot.validate_identity(data)?;
                    slot.invoke(|module, spring| $crate::__cus_enter!(module, spring, $accessor, |host, spring| unsafe {
                        $crate::cus::exports::native_named(host, spring, query, result)
                    } $(, $entry)?))
                })
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn RustCUSTick(
            _interface: *const $crate::__sys::NativeInterface,
            data: *mut core::ffi::c_void,
            query: *const $crate::__sys::CusTickQuery,
            result: *mut $crate::__sys::CusTickResult,
        ) {
            use $crate::cus::host::CusModule as _;
            let Some(result) = (unsafe { result.as_mut() }) else {
                return;
            };
            result.error = $crate::module::error_pointer(unsafe {
                $slot.with_host(|slot| {
                    slot.validate_identity(data)?;
                    let query = unsafe { query.as_ref() }.ok_or($crate::ApiError::new(
                        $crate::ErrorCode::InvalidArgument as i32,
                    ))?;
                    slot.invoke(|module, spring| $crate::__cus_enter!(module, spring, $accessor, |host: &$host, _| {
                        host.cus_tick(query.frame);
                        Ok(())
                    } $(, $entry)?))
                })
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn RustCUSDetach(
            _interface: *const $crate::__sys::NativeInterface,
            data: *mut core::ffi::c_void,
            query: *const $crate::__sys::CusDetachQuery,
            result: *mut $crate::__sys::CusDetachResult,
        ) {
            use $crate::cus::host::CusModule as _;
            let Some(result) = (unsafe { result.as_mut() }) else {
                return;
            };
            result.error = $crate::module::error_pointer(unsafe {
                $slot.with_host(|slot| {
                    slot.validate_identity(data)?;
                    let query = unsafe { query.as_ref() }.ok_or($crate::ApiError::new(
                        $crate::ErrorCode::InvalidArgument as i32,
                    ))?;
                    slot.invoke(|module, spring| $crate::__cus_enter!(module, spring, $accessor, |host: &$host, _| {
                        host.cus_detach(query.instanceID);
                        Ok(())
                    } $(, $entry)?))
                })
            });
        }

        #[cfg(target_arch = "wasm32")]
        static mut __SPRING_CUS_BUFFER: [u32; $crate::cus::exports::wire::BUFFER_SIZE / 4] =
            [0; $crate::cus::exports::wire::BUFFER_SIZE / 4];
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_BUFFER() -> i32 {
            (&raw mut __SPRING_CUS_BUFFER) as *mut u32 as usize as i32
        }
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_BUFFER_SIZE() -> i32 {
            $crate::cus::exports::wire::BUFFER_SIZE as i32
        }
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_INIT() -> i32 {
            0
        }
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SPRING_CUS_INVOKE(
            instance: i32,
            call: i32,
            floats: i32,
            float_count: i32,
            integers: i32,
            int_count: i32,
            result: i32,
        ) -> i32 {
            $crate::module::wasm_boundary(with_cus_module(|host, spring| unsafe {
                $crate::cus::exports::wire::invoke(
                    host,
                    spring,
                    SPRING_CUS_BUFFER() as usize,
                    instance,
                    call,
                    floats,
                    float_count,
                    integers,
                    int_count,
                    result,
                )
            }));
            0
        }
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SPRING_CUS_CALL_NAMED(
            instance: i32,
            name: i32,
            name_len: i32,
            args: i32,
            args_len: i32,
            result: i32,
        ) -> i32 {
            $crate::module::wasm_boundary(with_cus_module(|host, spring| unsafe {
                $crate::cus::exports::wire::named(
                    host,
                    spring,
                    SPRING_CUS_BUFFER() as usize,
                    instance,
                    name,
                    name_len,
                    args,
                    args_len,
                    result,
                )
            }));
            0
        }
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_TICK(frame: i32) {
            use $crate::cus::host::CusModule as _;
            $crate::module::wasm_boundary(with_cus_module(|host, _| {
                host.cus_tick(frame as u32);
                Ok(())
            }));
        }
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_DETACH(instance: i32) {
            use $crate::cus::host::CusModule as _;
            $crate::module::wasm_boundary(with_cus_module(|host, _| {
                host.cus_detach(instance as u32);
                Ok(())
            }));
        }
    };
}

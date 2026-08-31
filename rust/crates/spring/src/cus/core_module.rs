/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

/// Fixed scratch protocol shared by the Core host and guest exports.
pub const BUFFER_SIZE: usize = 16 * 1024;
pub const FLOAT_ARGUMENTS_OFFSET: usize = 0;
pub const INTEGER_ARGUMENTS_OFFSET: usize = 1024;
pub const NAME_OFFSET: usize = 2048;
pub const RESULT_OFFSET: usize = 4096;
pub const MAX_ARGUMENTS: usize = 256;
pub const MAX_RESULTS: usize = 64;

#[derive(Debug)]
pub struct CoreCusCallResult<'a> {
    pub int_value: i32,
    pub float_value: f32,
    pub bool_value: bool,
    pub complete: bool,
    pub int_count: usize,
    pub int_values: &'a mut [i32],
}

pub trait CoreCusModule: Default {
    fn cus_invoke(
        &mut self,
        _instance_id: u32,
        _call: u32,
        _float_arguments: &[f32],
        _integer_arguments: &[i32],
        _result: &mut CoreCusCallResult<'_>,
    ) -> bool {
        false
    }

    fn cus_call_named(
        &mut self,
        _instance_id: u32,
        _function_name: &str,
        _arguments: &[f32],
        _return_values: &mut [f32],
        _found: &mut bool,
    ) -> Option<usize> {
        None
    }

    fn cus_tick(&mut self, _frame: u32) {}
    fn cus_detach(&mut self, _instance_id: u32) {}
}

#[inline]
pub fn read_f32s(pointer: i32, count: i32, buffer_base: usize) -> Option<&'static [f32]> {
    if pointer < 0 || count < 0 || count as usize > MAX_ARGUMENTS {
        return None;
    }
    let bytes = (count as usize).checked_mul(core::mem::size_of::<f32>())?;
    if !buffer_range_contains(pointer, bytes, buffer_base) {
        return None;
    }
    // SAFETY: Core host pointers are offsets into this guest's linear
    // memory and the fixed scratch bounds above are checked first.
    Some(unsafe { core::slice::from_raw_parts(pointer as *const f32, count as usize) })
}

#[inline]
pub fn read_i32s(pointer: i32, count: i32, buffer_base: usize) -> Option<&'static [i32]> {
    if pointer < 0 || count < 0 || count as usize > MAX_ARGUMENTS {
        return None;
    }
    let bytes = (count as usize).checked_mul(core::mem::size_of::<i32>())?;
    if !buffer_range_contains(pointer, bytes, buffer_base) {
        return None;
    }
    // SAFETY: See `read_f32s`.
    Some(unsafe { core::slice::from_raw_parts(pointer as *const i32, count as usize) })
}

#[inline]
pub fn read_name(pointer: i32, length: i32, buffer_base: usize) -> Option<&'static str> {
    if pointer < 0 || length < 0 {
        return None;
    }
    if !buffer_range_contains(pointer, length as usize, buffer_base) {
        return None;
    }
    // SAFETY: See `read_f32s`; UTF-8 validation is performed below.
    let bytes = unsafe { core::slice::from_raw_parts(pointer as *const u8, length as usize) };
    core::str::from_utf8(bytes).ok()
}

#[inline]
pub fn buffer_range_contains(pointer: i32, bytes: usize, buffer_base: usize) -> bool {
    let start = pointer as usize;
    let Some(end) = start.checked_add(bytes) else {
        return false;
    };
    let Some(buffer_end) = buffer_base.checked_add(BUFFER_SIZE) else {
        return false;
    };
    start >= buffer_base && end <= buffer_end
}

/// Export the module-owned CUS registry and callback dispatcher.
/// `T` is normally a game module containing one or more `CusRegistry`
/// values. The host never inspects that state; it only invokes these
/// stable exports and the engine-operation imports.
#[macro_export]
macro_rules! export_core_cus {
    ($module_type:ty) => {
        static mut __SPRING_CUS_MODULE: Option<$module_type> = None;
        static mut __SPRING_CUS_BUFFER: [u8; $crate::cus::core_module::BUFFER_SIZE] =
            [0; $crate::cus::core_module::BUFFER_SIZE];

        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_BUFFER() -> i32 {
            let pointer = &raw mut __SPRING_CUS_BUFFER;
            pointer as *mut u8 as usize as u32 as i32
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_BUFFER_SIZE() -> i32 {
            $crate::cus::core_module::BUFFER_SIZE as i32
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_CUS_INIT() -> i32 {
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                *module = Some(<$module_type as Default>::default());
            }
            0
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SPRING_CUS_INVOKE(
            instance_id: i32,
            call: i32,
            float_pointer: i32,
            float_count: i32,
            integer_pointer: i32,
            integer_count: i32,
            result_pointer: i32,
        ) -> i32 {
            unsafe {
                let buffer_base = (&raw const __SPRING_CUS_BUFFER) as *const u8 as usize;
                let Some(float_arguments) =
                    $crate::cus::core_module::read_f32s(float_pointer, float_count, buffer_base)
                else {
                    return 1;
                };
                let Some(integer_arguments) = $crate::cus::core_module::read_i32s(
                    integer_pointer,
                    integer_count,
                    buffer_base,
                ) else {
                    return 1;
                };
                if result_pointer < 0
                    || !$crate::cus::core_module::buffer_range_contains(
                        result_pointer,
                        $crate::cus::core_module::MAX_RESULTS * 4 + 20,
                        buffer_base,
                    )
                    || !$crate::cus::core_module::buffer_range_contains(
                        result_pointer,
                        540,
                        buffer_base,
                    )
                {
                    return 1;
                }
                let output = result_pointer as *mut u8;
                let int_values = core::slice::from_raw_parts_mut(
                    output.add(20) as *mut i32,
                    $crate::cus::core_module::MAX_RESULTS,
                );
                let mut call_result = $crate::cus::core_module::CoreCusCallResult {
                    int_value: -1,
                    float_value: 1.0,
                    bool_value: false,
                    complete: false,
                    int_count: 0,
                    int_values,
                };
                let module = &raw mut __SPRING_CUS_MODULE;
                let Some(module) = (*module).as_mut() else {
                    return 1;
                };
                let handled = <$module_type as $crate::cus::core_module::CoreCusModule>::cus_invoke(
                    module,
                    instance_id as u32,
                    call as u32,
                    float_arguments,
                    integer_arguments,
                    &mut call_result,
                );
                if !handled {
                    return 0;
                }
                if call_result.int_count > call_result.int_values.len() {
                    return 1;
                }
                (output as *mut i32).write(call_result.int_value);
                (output.add(4) as *mut f32).write(call_result.float_value);
                (output.add(8) as *mut u32).write(call_result.bool_value as u32);
                (output.add(12) as *mut u32).write(call_result.complete as u32);
                (output.add(16) as *mut u32).write(call_result.int_count as u32);
                (output.add(536) as *mut u32).write(handled as u32);
                0
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SPRING_CUS_CALL_NAMED(
            instance_id: i32,
            name_pointer: i32,
            name_length: i32,
            argument_pointer: i32,
            argument_count: i32,
            result_pointer: i32,
        ) -> i32 {
            unsafe {
                let buffer_base = (&raw const __SPRING_CUS_BUFFER) as *const u8 as usize;
                let Some(name) =
                    $crate::cus::core_module::read_name(name_pointer, name_length, buffer_base)
                else {
                    return 1;
                };
                let Some(arguments) = $crate::cus::core_module::read_f32s(
                    argument_pointer,
                    argument_count,
                    buffer_base,
                ) else {
                    return 1;
                };
                if result_pointer < 0
                    || !$crate::cus::core_module::buffer_range_contains(
                        result_pointer,
                        8 + $crate::cus::core_module::MAX_RESULTS * 4,
                        buffer_base,
                    )
                    || !$crate::cus::core_module::buffer_range_contains(
                        result_pointer,
                        268,
                        buffer_base,
                    )
                {
                    return 1;
                }
                let output = result_pointer as *mut u8;
                let return_values = core::slice::from_raw_parts_mut(
                    output.add(8) as *mut f32,
                    $crate::cus::core_module::MAX_RESULTS,
                );
                let mut found = false;
                let module = &raw mut __SPRING_CUS_MODULE;
                let Some(module) = (*module).as_mut() else {
                    return 1;
                };
                let Some(count) =
                    <$module_type as $crate::cus::core_module::CoreCusModule>::cus_call_named(
                        module,
                        instance_id as u32,
                        name,
                        arguments,
                        return_values,
                        &mut found,
                    )
                else {
                    return 0;
                };
                if count > return_values.len() {
                    return 1;
                }
                (output as *mut u32).write(count as u32);
                (output.add(4) as *mut u32).write(found as u32);
                (output.add(264) as *mut u32).write(1);
                0
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SPRING_CUS_TICK(frame: i32) {
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                if let Some(module) = (*module).as_mut() {
                    <$module_type as $crate::cus::core_module::CoreCusModule>::cus_tick(
                        module,
                        frame as u32,
                    );
                }
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SPRING_CUS_DETACH(instance_id: i32) {
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                if let Some(module) = (*module).as_mut() {
                    <$module_type as $crate::cus::core_module::CoreCusModule>::cus_detach(
                        module,
                        instance_id as u32,
                    );
                }
            }
        }
    };
}

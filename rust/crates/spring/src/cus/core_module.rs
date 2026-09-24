/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

/// Fixed scratch protocol shared by the Core host and guest exports.
pub const BUFFER_SIZE: usize = 16 * 1024;
pub const FLOAT_ARGUMENTS_OFFSET: usize = 0;
pub const INTEGER_ARGUMENTS_OFFSET: usize = 1024;
pub const NAME_OFFSET: usize = 2048;
pub const RESULT_OFFSET: usize = 4096;
pub const MAX_ARGUMENTS: usize = 256;
pub const MAX_RESULTS: usize = 64;
/// `SPRING_CUS_INVOKE`/`SPRING_CUS_CALL_NAMED` status for "the CUS state is in
/// use further up the stack". The engine queues a call whose result it doesn't
/// read (`Create`, `StartMoving`, ...) and delivers it once the module returns.
pub const STATUS_BUSY: i32 = 2;

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

/// Marks the module's CUS state as borrowed. `export_core_cus!` holds one while
/// an engine CUS export or `with_cus_module` runs, so the two can never alias
/// the state: the second one sees it busy and backs off.
pub struct CusBusy(&'static core::sync::atomic::AtomicBool);

impl CusBusy {
    #[inline]
    pub fn enter(flag: &'static core::sync::atomic::AtomicBool) -> Option<Self> {
        if flag.swap(true, core::sync::atomic::Ordering::Acquire) {
            None
        } else {
            Some(Self(flag))
        }
    }
}

impl Drop for CusBusy {
    #[inline]
    fn drop(&mut self) {
        self.0.store(false, core::sync::atomic::Ordering::Release);
    }
}

/// Work for the CUS state that arrived while it was in use; see
/// `with_cus_module_or_defer` in [`export_core_cus!`](crate::export_core_cus).
type DeferredWork<T> = alloc::vec::Vec<alloc::boxed::Box<dyn FnOnce(&mut T)>>;

pub struct DeferredCus<T>(core::cell::UnsafeCell<DeferredWork<T>>);

// SAFETY: Core Wasm guests are single-threaded.
unsafe impl<T> Sync for DeferredCus<T> {}

impl<T> Default for DeferredCus<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DeferredCus<T> {
    pub const fn new() -> Self {
        Self(core::cell::UnsafeCell::new(alloc::vec::Vec::new()))
    }

    pub fn push(&self, work: alloc::boxed::Box<dyn FnOnce(&mut T)>) {
        // SAFETY: single-threaded, and no reference into the queue outlives a
        // call of `push` or `run`.
        unsafe { (*self.0.get()).push(work) }
    }

    /// Run the queued work in order, including work queued meanwhile.
    pub fn run(&self, module: &mut T) {
        loop {
            // SAFETY: see `push`; the queue is taken out before any work runs.
            let work = unsafe { core::mem::take(&mut *self.0.get()) };
            if work.is_empty() {
                return;
            }
            for item in work {
                item(module);
            }
        }
    }
}

#[doc(hidden)]
pub fn boxed<T>(work: impl FnOnce(&mut T) + 'static) -> alloc::boxed::Box<dyn FnOnce(&mut T)> {
    alloc::boxed::Box::new(work)
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
///
/// Also defines `with_cus_module(|module| ...)` and
/// `with_cus_module_or_defer(|module| ...)` at the invocation site, so the
/// module's own gadget/rules code can reach the CUS state directly: attach a
/// script from `UnitCreated`, or call a unit script, without going through the
/// engine (which cannot re-enter a running module). It returns `None` while
/// the state is already in use (inside a CUS export or a nested call).
/// `with_cus_module_or_defer` queues the work instead and runs it, in order,
/// as soon as that use ends: for example an attach from a `UnitCreated` raised
/// by a unit script that creates a unit.
///
/// # `Create` and other engine calls
///
/// The engine calls `Create` exactly once for every script the module attaches,
/// after the module has returned from the call in which it attached (the
/// `attach` import can't re-enter the module). Don't run `Create` yourself.
///
/// An engine call that arrives while the CUS state is in use gets
/// [`STATUS_BUSY`](crate::cus::core_module::STATUS_BUSY) back. The engine then queues it if it doesn't read a result
/// (`Create`, `StartMoving`, `RawCall`, ...) and delivers it, in order, once no
/// code of this module is on the stack. A call that returns a value
/// (`AimWeapon`, `QueryWeapon`, `HitByWeapon`, named calls, ...) can't wait and
/// gets the engine's neutral default instead.
#[macro_export]
macro_rules! export_core_cus {
    ($module_type:ty) => {
        static __SPRING_CUS_BUSY: core::sync::atomic::AtomicBool =
            core::sync::atomic::AtomicBool::new(false);

        static __SPRING_CUS_DEFERRED: $crate::cus::core_module::DeferredCus<$module_type> =
            $crate::cus::core_module::DeferredCus::new();

        /// Run `f` on this module's CUS state; `None` if it is not initialized
        /// yet or already in use further up the stack.
        #[allow(dead_code)]
        pub fn with_cus_module<R>(f: impl FnOnce(&mut $module_type) -> R) -> Option<R> {
            let _busy = $crate::cus::core_module::CusBusy::enter(&__SPRING_CUS_BUSY)?;
            // SAFETY: `_busy` excludes every other access path to the state.
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                let module = (*module).as_mut()?;
                __SPRING_CUS_DEFERRED.run(module);
                let result = f(module);
                __SPRING_CUS_DEFERRED.run(module);
                Some(result)
            }
        }

        /// Run `f` on this module's CUS state now, or, while the state is in
        /// use further up the stack (or not initialized yet), as soon as that
        /// use ends. Deferred work runs in the order it was queued. Returns
        /// whether `f` ran now.
        #[allow(dead_code)]
        pub fn with_cus_module_or_defer(f: impl FnOnce(&mut $module_type) + 'static) -> bool {
            let mut f = Some(f);
            let ran = with_cus_module(|module| (f.take().expect("runs once"))(module)).is_some();
            if let Some(f) = f {
                __SPRING_CUS_DEFERRED.push($crate::cus::core_module::boxed(f));
            }
            ran
        }

        // Runs queued work before an export releases the state.
        #[allow(dead_code)]
        unsafe fn __spring_cus_run_deferred() {
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                if let Some(module) = (*module).as_mut() {
                    __SPRING_CUS_DEFERRED.run(module);
                }
            }
        }

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
            let Some(_busy) = $crate::cus::core_module::CusBusy::enter(&__SPRING_CUS_BUSY) else {
                return 1;
            };
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                *module = Some(<$module_type as Default>::default());
                __spring_cus_run_deferred();
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
            // Busy: the state is in use further up this stack.
            let Some(_busy) = $crate::cus::core_module::CusBusy::enter(&__SPRING_CUS_BUSY) else {
                return $crate::cus::core_module::STATUS_BUSY;
            };
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
                __SPRING_CUS_DEFERRED.run(module);
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
            let Some(_busy) = $crate::cus::core_module::CusBusy::enter(&__SPRING_CUS_BUSY) else {
                return $crate::cus::core_module::STATUS_BUSY;
            };
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
                    __SPRING_CUS_DEFERRED.run(module);
                    return 0;
                };
                __SPRING_CUS_DEFERRED.run(module);
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
            let Some(_busy) = $crate::cus::core_module::CusBusy::enter(&__SPRING_CUS_BUSY) else {
                return;
            };
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                if let Some(module) = (*module).as_mut() {
                    <$module_type as $crate::cus::core_module::CoreCusModule>::cus_tick(
                        module,
                        frame as u32,
                    );
                    __SPRING_CUS_DEFERRED.run(module);
                }
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn SPRING_CUS_DETACH(instance_id: i32) {
            let Some(_busy) = $crate::cus::core_module::CusBusy::enter(&__SPRING_CUS_BUSY) else {
                return;
            };
            unsafe {
                let module = &raw mut __SPRING_CUS_MODULE;
                if let Some(module) = (*module).as_mut() {
                    <$module_type as $crate::cus::core_module::CoreCusModule>::cus_detach(
                        module,
                        instance_id as u32,
                    );
                    __SPRING_CUS_DEFERRED.run(module);
                }
            }
        }
    };
}

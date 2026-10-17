/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

/// Declare module ownership and export initialization, shutdown, environment,
/// and retained callbacks. Native automatic admission currently supports Menu;
/// other native environments need a host dispatch/visibility contract first.
/// Add the lifecycle exports your module uses with the individual macros.
#[macro_export]
macro_rules! export_module {
    ($slot:ident, $environment:ty, $module:ty) => {
        std::thread_local! { static $slot: $crate::module::ModuleSlot<$environment,$module> = const { $crate::module::ModuleSlot::new() }; }
        $crate::export_callbacks!();
        #[cfg(target_arch="wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn SPRING_ENV_MASK() -> i32 { <$environment as $crate::Environment>::MASK as i32 }
        #[cfg(not(target_arch="wasm32"))]
        const _: () = assert!(<$environment as $crate::Environment>::MASK==32,"native module entry currently admits Menu only");
        #[cfg(not(target_arch="wasm32"))]
        #[unsafe(no_mangle)]
        pub static NativeModuleEnvironment: u32=<$environment as $crate::Environment>::MASK;
        #[cfg(not(target_arch="wasm32"))]
        #[unsafe(no_mangle)]
        pub static NativeModuleApiVersion: [u32;3]=$crate::NATIVE_API_VERSION;
        #[cfg(not(target_arch="wasm32"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn InitializeNativeModule(interface:*const $crate::__sys::NativeInterface, query:*const $crate::__sys::InitializeNativeModuleQuery, result:*mut $crate::__sys::InitializeNativeModuleResult) {
            let Some(result)=(unsafe {result.as_mut()}) else {return;};
            *result=Default::default();
            [result.moduleVersionMajor,result.moduleVersionMinor,result.moduleVersionPatch]=$crate::NATIVE_API_VERSION;
            let outcome=$slot.with(|slot| {
                let query=unsafe {query.as_ref()}.ok_or($crate::ApiError::new($crate::ErrorCode::InvalidArgument as i32))?;
                if query.hostVersionMajor!=$crate::NATIVE_API_VERSION[0] || query.hostVersionMinor<$crate::NATIVE_API_VERSION[1] {return Err($crate::ApiError::new($crate::ErrorCode::NotAvailable as i32));}
                result.moduleData=unsafe {slot.initialize_native(interface)?};
                Ok(())
            });
            result.error=$crate::module::error_pointer(outcome);
        }
        #[cfg(target_arch="wasm32")]
        # [unsafe(export_name="spring:module/initialize")]
        pub extern "C" fn __spring_sdk_initialize() {
            $crate::module::wasm_boundary($slot.with(|slot|unsafe {slot.initialize_wasm()}));
        }
        #[cfg(not(target_arch="wasm32"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn Shutdown(_interface:*const $crate::__sys::NativeInterface, data:*mut core::ffi::c_void, _query:*const $crate::__sys::ShutdownQuery, result:*mut $crate::__sys::ShutdownResult) {
            let Some(result)=(unsafe {result.as_mut()}) else {return;};
            result.error=$crate::module::error_pointer($slot.with(|slot|slot.validate_identity(data).and_then(|_|slot.shutdown())));
        }
        #[cfg(target_arch="wasm32")]
        #[unsafe(export_name="spring:callin/shutdown")]
        pub extern "C" fn __spring_sdk_shutdown() { $crate::module::wasm_boundary($slot.with(|slot|slot.shutdown())); }
    };
}

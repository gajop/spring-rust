/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Module ownership and engine entry scopes. No mutable module borrow is held
//! across user code. Put independently mutable state behind Resource/RefCell,
//! and end those borrows before callouts that can reenter it.

use crate::{ApiError, Environment, ErrorCode, ModuleContext, Result, Spring};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub trait Module<E: Environment>: Sized + 'static {
    fn initialize(spring: &mut Spring<'_, E>) -> Result<Self>;
    fn shutdown(&self, _spring: &mut Spring<'_, E>) -> Result<()> {
        Ok(())
    }
}

struct Instance<E: Environment, T> {
    context: ModuleContext<E>,
    module: T,
}
/// One module instance per engine thread, normally declared by `export_module!`.
/// A second initialization is rejected until the first instance is shut down.
pub struct ModuleSlot<E: Environment, T> {
    instance: RefCell<Option<Rc<Instance<E, T>>>>,
    transitioning: Cell<bool>,
    poisoned: Cell<bool>,
}
impl<E: Environment, T: Module<E>> Default for ModuleSlot<E, T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<E: Environment, T: Module<E>> ModuleSlot<E, T> {
    pub const fn new() -> Self {
        Self {
            instance: RefCell::new(None),
            transitioning: Cell::new(false),
            poisoned: Cell::new(false),
        }
    }

    /// # Safety
    /// Same admission requirements as `ModuleContext::from_native`.
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn initialize_native(
        &self,
        interface: *const crate::__sys::NativeInterface,
    ) -> Result<*mut core::ffi::c_void> {
        let context = unsafe { ModuleContext::<E>::from_native(interface)? };
        self.initialize(context)?;
        Ok(self.identity())
    }

    /// # Safety
    /// The host must have admitted the module in environment E.
    #[cfg(target_arch = "wasm32")]
    pub unsafe fn initialize_wasm(&self) -> Result<()> {
        self.initialize(unsafe { ModuleContext::<E>::from_wasm() })
    }

    fn initialize(&self, context: ModuleContext<E>) -> Result<()> {
        if self.transitioning.replace(true) {
            return Err(invalid());
        }
        let result = self.boundary(|| {
            if self.instance.borrow().is_some() {
                return Err(invalid());
            }
            let module = context.with_spring(T::initialize)??;
            *self.instance.borrow_mut() = Some(Rc::new(Instance { context, module }));
            self.poisoned.set(false);
            Ok(())
        });
        self.transitioning.set(false);
        result
    }

    /// Run a callin. Clone the instance before invoking user code so nested
    /// callins can enter immediately and no registry borrow crosses a callout.
    /// A module borrow cannot escape a callin even though the slot uses shared
    /// access and can be shut down through another shared reference.
    /// ```compile_fail
    /// use spring_sdk::{Menu,Spring,Result,module::{Module,ModuleSlot}};
    /// struct App;
    /// impl Module<Menu> for App {
    ///     fn initialize(_: &mut Spring<'_,Menu>)->Result<Self> {Ok(Self)}
    /// }
    /// fn escape(slot:&ModuleSlot<Menu,App>)->&App {
    ///     slot.invoke(|module,_|Ok(module)).unwrap()
    /// }
    /// ```
    pub fn invoke<R>(
        &self,
        handler: impl for<'call> FnOnce(&T, &mut Spring<'call, E>) -> Result<R>,
    ) -> Result<R> {
        if self.transitioning.get() || self.poisoned.get() {
            return Err(invalid());
        }
        self.boundary(|| {
            let instance = self
                .instance
                .borrow()
                .as_ref()
                .cloned()
                .ok_or_else(invalid)?;
            instance
                .context
                .with_spring(|spring| handler(&instance.module, spring))?
        })
    }

    #[doc(hidden)]
    pub fn invoke_for_environment<const MASK: u32, R>(
        &self,
        handler: impl for<'call> FnOnce(&T, &mut Spring<'call, E>) -> Result<R>,
    ) -> Result<R> {
        const {
            assert!(
                MASK & E::MASK != 0,
                "callin is not available in this module environment"
            );
        }
        self.invoke(handler)
    }

    pub fn shutdown(&self) -> Result<()> {
        if self.transitioning.replace(true) {
            return Err(invalid());
        }
        let result = self.boundary(|| {
            let instance = self.instance.borrow_mut().take().ok_or_else(invalid)?;
            if self.poisoned.get() {
                return Ok(());
            }
            instance
                .context
                .with_spring(|spring| instance.module.shutdown(spring))?
        });
        self.transitioning.set(false);
        result
    }

    #[doc(hidden)]
    pub fn identity(&self) -> *mut core::ffi::c_void {
        self.instance
            .borrow()
            .as_ref()
            .map_or(core::ptr::null_mut(), |instance| {
                Rc::as_ptr(instance).cast_mut().cast()
            })
    }
    #[doc(hidden)]
    pub fn validate_identity(&self, identity: *mut core::ffi::c_void) -> Result<()> {
        if identity.is_null() || identity != self.identity() {
            Err(invalid())
        } else {
            Ok(())
        }
    }

    fn fail(&self) {
        self.poisoned.set(true);
        if let Some(instance) = self.instance.borrow().as_ref() {
            instance.context.owner.alive.set(false);
        }
    }

    fn boundary<R>(&self, call: impl FnOnce() -> Result<R>) -> Result<R> {
        #[cfg(not(target_arch = "wasm32"))]
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(call))
            .unwrap_or_else(|_| Err(invalid()));
        #[cfg(target_arch = "wasm32")]
        let result = call();
        if result.is_err() {
            self.fail();
        }
        result
    }
}
fn invalid() -> ApiError {
    ApiError::new(ErrorCode::InvalidState as i32)
}

/// Trap errors at a Wasm entry boundary. The engine records the trap and
/// unloads the module; continuing with partially updated game state is unsafe.
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub fn wasm_boundary<R>(result: Result<R>) -> R {
    result.unwrap_or_else(|_| core::arch::wasm32::unreachable())
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static ERROR:RefCell<crate::__sys::Error>=const {RefCell::new(crate::__sys::Error {code:0,message:core::ptr::null()})};
}
/// Error storage belongs to this thread and remains valid until the next
/// callin error is returned. The host reads it immediately at the boundary.
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub fn error_pointer(result: Result<()>) -> *const crate::__sys::Error {
    match result {
        Ok(()) => core::ptr::null(),
        Err(error) => ERROR.with(|slot| {
            let mut slot = slot.borrow_mut();
            slot.code = error.code;
            slot.message = c"spring-sdk module callin failed".as_ptr();
            &*slot as *const _
        }),
    }
}

/// # Safety
/// Pointer must reference a host query byte array of `length` bytes.
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub unsafe fn copy_message(pointer: *const u8, length: usize) -> Result<Vec<u8>> {
    Ok(unsafe { crate::native::read_slice(pointer, length)? }.to_vec())
}

#[doc(hidden)]
pub fn decode_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(value) => value,
        Err(error) => error.into_bytes().into_iter().map(char::from).collect(),
    }
}
fn word(bytes: &[u8], offset: usize) -> Result<u32> {
    let end = offset.checked_add(4).ok_or_else(invalid)?;
    Ok(u32::from_le_bytes(
        bytes
            .get(offset..end)
            .ok_or_else(invalid)?
            .try_into()
            .unwrap(),
    ))
}
/// Reviewed specialized console codec: offset/length pairs then level.
#[doc(hidden)]
pub fn console_payload(bytes: &[u8]) -> Result<(&[u8], &[u8], i32)> {
    if bytes.len() < 20 {
        return Err(invalid());
    }
    let span = |offset: usize| -> Result<&[u8]> {
        let begin = word(bytes, offset)? as usize;
        let length = word(bytes, offset + 4)? as usize;
        if begin < 20 {
            return Err(invalid());
        }
        bytes
            .get(begin..begin.checked_add(length).ok_or_else(invalid)?)
            .ok_or_else(invalid)
    };
    Ok((span(0)?, span(8)?, word(bytes, 16)? as i32))
}
/// Reviewed specialized command codec: six words followed by f32 parameters.
#[doc(hidden)]
pub fn command_payload(bytes: &[u8]) -> Result<(i32, i32, u32, u32, u8, Vec<f32>)> {
    let count = word(bytes, 12)? as usize;
    let expected = count
        .checked_mul(4)
        .and_then(|n| n.checked_add(24))
        .ok_or_else(invalid)?;
    if bytes.len() != expected {
        return Err(invalid());
    }
    let options = word(bytes, 20)?.try_into().map_err(|_| invalid())?;
    let params = bytes[24..]
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    Ok((
        word(bytes, 0)? as i32,
        word(bytes, 4)? as i32,
        word(bytes, 8)?,
        word(bytes, 16)?,
        options,
        params,
    ))
}

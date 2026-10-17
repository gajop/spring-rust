/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Scoped engine access shared by native and WebAssembly modules.

extern crate alloc;
#[cfg(all(not(target_arch = "wasm32"), panic = "abort"))]
compile_error!(
    "native spring-sdk modules require panic=unwind to contain panics at C ABI boundaries"
);
use core::marker::PhantomData;
pub use spring::{ApiError, ErrorCode, Float3, Result};
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub use spring_native_sys as __sys;
use std::{cell::Cell, rc::Rc};

mod sealed {
    pub trait Environment {}
}

/// An engine execution environment. Implementations are sealed.
pub trait Environment: sealed::Environment {
    const MASK: u32;
}

macro_rules! environments {
    ($($name:ident = $mask:expr),* $(,)?) => {$ (
        pub enum $name {}
        impl sealed::Environment for $name {}
        impl Environment for $name { const MASK:u32 = $mask; }
    )*};
}
environments!(
    RulesSynced = 1,
    RulesUnsynced = 2,
    GaiaSynced = 4,
    GaiaUnsynced = 8,
    Ui = 16,
    Menu = 32,
    Intro = 64
);

pub mod capabilities {
    use super::*;
    macro_rules! capability {
        ($name:ident: $($environment:ident),*) => {
            pub trait $name: Environment {}
            $(impl $name for $environment {})*
        };
    }
    capability!(Game: RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui);
    capability!(Render: RulesUnsynced, GaiaUnsynced, Ui, Menu, Intro);
    capability!(Synced: RulesSynced, GaiaSynced);
    capability!(Local: RulesUnsynced, GaiaUnsynced, Ui);
    capability!(Session: RulesUnsynced, GaiaUnsynced, Ui, Menu);
    capability!(Common: RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui, Menu, Intro);
    capability!(Launch: RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui, Menu);
    capability!(Downloads: Ui, Menu);
    capability!(Replay: RulesUnsynced, GaiaUnsynced, Ui, Intro);
    capability!(World: RulesSynced, RulesUnsynced, GaiaSynced, GaiaUnsynced, Ui, Intro);
}

/// Access to Spring for one engine callin. It cannot be stored beyond that
/// callin or sent/shared with a worker thread. It does not borrow game state.
///
/// Engine references cannot be sent to workers:
/// ```compile_fail
/// use spring_sdk::{Spring,Ui};
/// fn worker(spring: &mut Spring<'_,Ui>) {
///     std::thread::scope(|threads| { threads.spawn(move || { let _=spring.game().get_game_frame(); }); });
/// }
/// ```
/// Synced modules cannot invoke rendering calls:
/// ```compile_fail
/// use spring_sdk::{Spring,RulesSynced};
/// fn render(spring: &mut Spring<'_,RulesSynced>) {
///     let _=spring.gfx().get_matrix_data(0);
/// }
/// ```
/// UI modules cannot mutate synced units:
/// ```compile_fail
/// use spring_sdk::{Spring,Ui,unit_control::DestroyUnitOptions};
/// fn mutate(spring: &mut Spring<'_,Ui>) {
///     let _=spring.unit_control().destroy_unit(1,DestroyUnitOptions::default());
/// }
/// ```
/// Snapshot modules expose values, with no ambient engine functions:
/// ```compile_fail
/// use spring_sdk::types::game::get_game_frame;
/// ```
pub struct Spring<'call, E: Environment> {
    engine: Backend<'call>,
    environment: PhantomData<E>,
}

/// Admission authority owned by a module, constructed at its engine entry
/// point. Shared references allow immediate nested callins without borrowing
/// the module's mutable game state or holding a lock across a host call.
pub struct ModuleContext<E: Environment> {
    owner: Rc<Authority>,
    environment: PhantomData<E>,
    affinity: PhantomData<*mut ()>,
}

impl<E: Environment> ModuleContext<E> {
    /// # Safety
    /// Called only by the module's engine initialization entry point, on the
    /// engine callin thread. The interface and its tables must remain valid
    /// until the context is dropped, and belong to environment `E`.
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn from_native(raw: *const spring_native_sys::NativeInterface) -> Result<Self> {
        if raw.is_null() {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        Ok(Self {
            owner: Rc::new(Authority {
                raw,
                thread: std::thread::current().id(),
                alive: Cell::new(true),
                error: Cell::new(None),
                panic: Default::default(),
                foreign_callback: Default::default(),
            }),
            environment: PhantomData,
            affinity: PhantomData,
        })
    }

    /// # Safety
    /// The module must export `E::MASK` and be loaded in that environment.
    #[cfg(target_arch = "wasm32")]
    pub unsafe fn from_wasm() -> Self {
        Self {
            owner: Rc::new(Authority {
                alive: Cell::new(true),
                error: Cell::new(None),
            }),
            environment: PhantomData,
            affinity: PhantomData,
        }
    }

    /// Enter an engine callin. The higher-ranked lifetime prevents a Spring
    /// handle (or one of its API families) escaping the callin. Nested calls
    /// enter another independent scope on the same engine thread.
    ///
    /// An API family cannot escape its callin:
    /// ```compile_fail
    /// use spring_sdk::{ModuleContext,Ui};
    /// fn escape(context: &ModuleContext<Ui>) {
    ///     let _leaked=context.with_spring(|spring| spring.game());
    /// }
    /// ```
    pub fn with_spring<R>(
        &self,
        callin: impl for<'call> FnOnce(&mut Spring<'call, E>) -> R,
    ) -> Result<R> {
        #[cfg(not(target_arch = "wasm32"))]
        if std::thread::current().id() != self.owner.thread {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        let mut spring = Spring {
            engine: Backend {
                owner: Rc::clone(&self.owner),
                scope: PhantomData,
            },
            environment: PhantomData,
        };
        spring.engine.check()?;
        let result = callin(&mut spring);
        spring.engine.check()?;
        Ok(result)
    }
}

impl<E: Environment> Drop for ModuleContext<E> {
    fn drop(&mut self) {
        self.owner.alive.set(false);
        callbacks::clear_owner(&self.owner);
    }
}

struct Authority {
    #[cfg(not(target_arch = "wasm32"))]
    raw: *const spring_native_sys::NativeInterface,
    #[cfg(not(target_arch = "wasm32"))]
    thread: std::thread::ThreadId,
    alive: Cell<bool>,
    error: Cell<Option<ApiError>>,
    #[cfg(not(target_arch = "wasm32"))]
    panic: std::cell::RefCell<Option<Box<dyn core::any::Any + Send>>>,
    #[cfg(not(target_arch = "wasm32"))]
    foreign_callback: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

struct Backend<'call> {
    owner: Rc<Authority>,
    // Invariant call lifetime, and !Send + !Sync on both targets.
    scope: PhantomData<(&'call mut &'call (), *mut ())>,
}

impl Backend<'_> {
    fn reborrow(&mut self) -> Backend<'_> {
        Backend {
            owner: Rc::clone(&self.owner),
            scope: PhantomData,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn raw(&self) -> *const spring_native_sys::NativeInterface {
        self.owner.raw
    }

    fn check(&self) -> Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        if std::thread::current().id() != self.owner.thread {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        if !self.owner.alive.get() {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self
                .owner
                .foreign_callback
                .swap(false, std::sync::atomic::Ordering::Relaxed)
            {
                return Err(ApiError::new(ErrorCode::InvalidState as i32));
            }
            let panic = self.owner.panic.borrow_mut().take();
            if let Some(panic) = panic {
                std::panic::resume_unwind(panic);
            }
        }
        if let Some(error) = self.owner.error.take() {
            return Err(error);
        }
        Ok(())
    }

    fn finish<R>(&self, result: Result<R>) -> Result<R> {
        self.check()?;
        result
    }

    fn with_nested<E: Environment, R>(
        &self,
        callback: impl for<'call> FnOnce(&mut Spring<'call, E>) -> R,
    ) -> R {
        let mut spring = Spring {
            engine: Backend {
                owner: Rc::clone(&self.owner),
                scope: PhantomData,
            },
            environment: PhantomData,
        };
        callback(&mut spring)
    }
}

pub mod types {
    #[cfg(target_arch = "wasm32")]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rts/wasm/generated/sdk/scoped_types_wasm.rs"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rts/wasm/generated/sdk/scoped_types.rs"
    ));
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_api.rs"
));

pub mod callbacks;
mod exports;
pub mod module;
#[cfg(not(target_arch = "wasm32"))]
mod native;
pub mod rml;
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use spring::export_callin_scratch;
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub use spring::generated::__core_callin_scratch_wire as __callin_wire;

/// Export the Wasm callback dispatcher once per module. Native callbacks use
/// C trampolines and need no exported dispatcher.
#[macro_export]
macro_rules! export_callbacks {
    () => {
        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:callback/dispatch")]
        pub extern "C" fn __spring_sdk_callback_dispatch(id: i32, data: i32) {
            $crate::callbacks::__dispatch(id as u32, data as u32);
        }
    };
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_callins.rs"
));
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub use native::callins as __native_callins;

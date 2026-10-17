/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use core::{future::Future, pin::Pin};

pub use crate::RetainedCapture as Capture;
#[cfg(not(target_arch = "wasm32"))]
pub use std::sync::{Arc as Shared, Weak};
#[cfg(target_arch = "wasm32")]
pub use std::{
    cell::RefCell as SharedCell,
    rc::{Rc as Shared, Weak},
};

#[cfg(not(target_arch = "wasm32"))]
pub struct SharedCell<T: ?Sized>(std::sync::Mutex<T>);

#[cfg(not(target_arch = "wasm32"))]
impl<T: Default> Default for SharedCell<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T> SharedCell<T> {
    pub fn new(value: T) -> Self {
        Self(std::sync::Mutex::new(value))
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl<T: ?Sized> SharedCell<T> {
    pub fn try_borrow_mut(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, T>> {
        self.0.try_lock()
    }
    pub fn borrow(&self) -> std::sync::MutexGuard<'_, T> {
        self.0
            .try_lock()
            .expect("CUS state is already borrowed or poisoned")
    }
    pub fn borrow_mut(&self) -> std::sync::MutexGuard<'_, T> {
        self.borrow()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
#[cfg(target_arch = "wasm32")]
pub type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;
#[cfg(not(target_arch = "wasm32"))]
pub type TaskStart = Box<dyn FnOnce(super::UnitCtx) -> TaskFuture + Send>;
#[cfg(target_arch = "wasm32")]
pub type TaskStart = Box<dyn FnOnce(super::UnitCtx) -> TaskFuture>;

pub fn wake<T>(state: &SharedCell<T>, f: impl FnOnce(&mut T)) {
    #[cfg(not(target_arch = "wasm32"))]
    f(&mut state.0.lock().expect("CUS scheduler is poisoned"));
    #[cfg(target_arch = "wasm32")]
    f(&mut state.borrow_mut());
}

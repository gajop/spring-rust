/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use alloc::boxed::Box;
pub use alloc::rc::{Rc as Shared, Weak};
pub use core::cell::RefCell as SharedCell;
use core::{future::Future, pin::Pin};

pub trait Capture {}
impl<T: ?Sized> Capture for T {}

pub type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;
pub type TaskStart = Box<dyn FnOnce(super::UnitCtx) -> TaskFuture>;
#[derive(Clone, Default)]
pub struct EngineScope;

pub fn wake<T>(state: &SharedCell<T>, f: impl FnOnce(&mut T)) {
    f(&mut state.borrow_mut());
}

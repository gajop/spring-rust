/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Explicit per-module command queues. Delayed addon actions run before these
//! queues are flushed. Immediate Spring calls are never queued automatically.

use crate::{Environment, RetainedCapture, Shared, Spring};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    sync::Mutex,
};

#[cfg(not(target_arch = "wasm32"))]
type Action<E> = Box<dyn for<'call> FnOnce(&mut Spring<'call, E>) + Send>;
#[cfg(target_arch = "wasm32")]
type Action<E> = Box<dyn for<'call> FnOnce(&mut Spring<'call, E>)>;
#[cfg(not(target_arch = "wasm32"))]
type Continuation<E, T> = Box<dyn for<'call> FnOnce(&mut Spring<'call, E>, T) + Send>;
#[cfg(target_arch = "wasm32")]
type Continuation<E, T> = Box<dyn for<'call> FnOnce(&mut Spring<'call, E>, T)>;
type ResultSlot<E, T> = Shared<Mutex<Option<Continuation<E, T>>>>;

/// Owned work for one module, using the Spring scope supplied when flushed.
/// A command cannot capture a borrowed Spring reference:
/// ```compile_fail
/// use spring_sdk::{All, Spring, commands::Queue};
/// fn capture(queue: &Queue<All>, spring: &mut Spring<'_, All>) {
///     queue.run(move |_| { let _ = spring.game().get_game_frame(); });
/// }
/// ```
pub struct Queue<E: Environment> {
    actions: RefCell<VecDeque<Action<E>>>,
    depth: Cell<usize>,
    flushing: Cell<bool>,
}
impl<E: Environment> Default for Queue<E> {
    fn default() -> Self {
        Self::new()
    }
}
impl<E: Environment> Queue<E> {
    pub const fn new() -> Self {
        Self {
            actions: RefCell::new(VecDeque::new()),
            depth: Cell::new(0),
            flushing: Cell::new(false),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.actions.borrow().is_empty()
    }
    pub fn is_flushing(&self) -> bool {
        self.flushing.get()
    }
    pub fn run(
        &self,
        action: impl for<'call> FnOnce(&mut Spring<'call, E>) + RetainedCapture + 'static,
    ) {
        self.actions.borrow_mut().push_back(Box::new(action));
    }
    pub fn queue<T: 'static>(
        &self,
        action: impl for<'call> FnOnce(&mut Spring<'call, E>) -> T + RetainedCapture + 'static,
    ) -> Pending<E, T> {
        let slot: ResultSlot<E, T> = Shared::new(Mutex::new(None));
        let then = slot.clone();
        self.run(move |spring| {
            let value = action(spring);
            let continuation = then.lock().unwrap_or_else(|p| p.into_inner()).take();
            if let Some(continuation) = continuation {
                continuation(spring, value);
            }
        });
        Pending { slot }
    }
    /// Drain FIFO, including work appended by nested callins and continuations.
    /// A recursive flush returns immediately; the outer pass drains that work.
    pub fn flush(&self, spring: &mut Spring<'_, E>) {
        if self.flushing.replace(true) {
            return;
        }
        struct Reset<'a>(&'a Cell<bool>);
        impl Drop for Reset<'_> {
            fn drop(&mut self) {
                self.0.set(false);
            }
        }
        let _reset = Reset(&self.flushing);
        let mut applied = 0;
        loop {
            let action = self.actions.borrow_mut().pop_front();
            let Some(action) = action else {
                break;
            };
            applied += 1;
            assert!(
                applied <= 1_000_000,
                "Spring command queue keeps refilling itself"
            );
            action(spring);
        }
    }
    /// Keep this guard around the whole addon/CUS callin. The outermost guard
    /// flushes after application borrows and delayed actions have finished.
    pub fn scope<'a, 'call>(&'a self, spring: &'a mut Spring<'call, E>) -> Scope<'a, 'call, E> {
        self.depth.set(self.depth.get() + 1);
        Scope {
            queue: self,
            spring,
        }
    }
}

#[must_use = "use .then for the result, or drop the pending result explicitly"]
pub struct Pending<E: Environment, T> {
    slot: ResultSlot<E, T>,
}
impl<E: Environment, T: 'static> Pending<E, T> {
    pub fn then(
        self,
        continuation: impl for<'call> FnOnce(&mut Spring<'call, E>, T) + RetainedCapture + 'static,
    ) {
        *self.slot.lock().unwrap_or_else(|p| p.into_inner()) = Some(Box::new(continuation));
    }
}
pub struct Scope<'a, 'call, E: Environment> {
    queue: &'a Queue<E>,
    spring: &'a mut Spring<'call, E>,
}
impl<'call, E: Environment> Scope<'_, 'call, E> {
    pub fn spring(&mut self) -> &mut Spring<'call, E> {
        self.spring
    }
}
impl<E: Environment> Drop for Scope<'_, '_, E> {
    fn drop(&mut self) {
        let depth = self.queue.depth.get() - 1;
        self.queue.depth.set(depth);
        if depth == 0 {
            self.queue.flush(self.spring);
        }
    }
}

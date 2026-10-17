/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Serialize native module entry while allowing immediate same-thread reentry.
//! Only unsafe host entry may change the accepted engine thread. Safe SDK calls
//! and retained callbacks can enter only on the thread most recently admitted.

use crate::{ApiError, ErrorCode, Result};
use std::sync::{Condvar, Mutex};
use std::thread::{self, ThreadId};

pub(crate) struct Gate {
    state: Mutex<State>,
    released: Condvar,
}
struct State {
    engine: ThreadId,
    active: Option<ThreadId>,
    depth: usize,
}
pub(crate) struct Guard<'a>(&'a Gate);
impl Gate {
    pub(crate) fn new() -> Self {
        Self {
            state: Mutex::new(State {
                engine: thread::current().id(),
                active: None,
                depth: 0,
            }),
            released: Condvar::new(),
        }
    }
    pub(crate) fn enter(&self) -> Result<Guard<'_>> {
        let current = thread::current().id();
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        if state.engine != current || state.active.is_some_and(|active| active != current) {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        state.active = Some(current);
        state.depth += 1;
        Ok(Guard(self))
    }
    pub(crate) fn enter_existing(&self) -> Result<Guard<'_>> {
        let current = thread::current().id();
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        if state.engine != current || state.active != Some(current) {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        state.depth += 1;
        Ok(Guard(self))
    }
    /// The caller is an engine ABI entry, not a worker requesting engine access.
    pub(crate) unsafe fn enter_host(&self) -> Guard<'_> {
        let current = thread::current().id();
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        while state.active.is_some_and(|active| active != current) {
            state = self.released.wait(state).unwrap_or_else(|p| p.into_inner());
        }
        state.engine = current;
        state.active = Some(current);
        state.depth += 1;
        Guard(self)
    }
    pub(crate) fn is_current(&self) -> bool {
        let state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        state.engine == thread::current().id()
    }
}
impl Drop for Guard<'_> {
    fn drop(&mut self) {
        let mut state = self.0.state.lock().unwrap_or_else(|p| p.into_inner());
        debug_assert_eq!(state.active, Some(thread::current().id()));
        state.depth -= 1;
        if state.depth == 0 {
            state.active = None;
            self.0.released.notify_one();
        }
    }
}

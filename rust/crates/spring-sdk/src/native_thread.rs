/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Serialize native module entry while allowing immediate same-thread reentry.
//! Only unsafe host entry may change the accepted engine thread. Safe SDK calls
//! and retained callbacks can enter only on the thread most recently admitted.

use crate::{ApiError, ErrorCode, Result};
use std::sync::{
    Condvar, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use std::thread::{self, ThreadId};

// Unique for the process lifetime, including after an OS thread exits. The
// common engine-call check reads TLS and one atomic instead of taking a lock.
static NEXT_THREAD: AtomicUsize = AtomicUsize::new(1);
thread_local! {
    static THREAD: usize = NEXT_THREAD.fetch_update(Ordering::Relaxed, Ordering::Relaxed,
        |value| value.checked_add(1)).expect("native SDK thread identity exhausted");
}
fn current_token() -> usize {
    THREAD.with(|value| *value)
}

pub(crate) struct Gate {
    state: Mutex<State>,
    released: Condvar,
    engine_token: AtomicUsize,
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
            engine_token: AtomicUsize::new(current_token()),
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
    /// The caller is an engine ABI entry, not a worker requesting engine access.
    pub(crate) unsafe fn enter_host(&self) -> Guard<'_> {
        let current = thread::current().id();
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        while state.active.is_some_and(|active| active != current) {
            state = self.released.wait(state).unwrap_or_else(|p| p.into_inner());
        }
        state.engine = current;
        self.engine_token.store(current_token(), Ordering::Relaxed);
        state.active = Some(current);
        state.depth += 1;
        Guard(self)
    }
    pub(crate) fn is_current(&self) -> bool {
        // This only checks identity. Entry guards still exclude concurrent
        // callbacks and synchronize the module's non-atomic state. A Spring
        // token borrows its owner inside such a guard and cannot cross threads.
        self.engine_token.load(Ordering::Relaxed) == current_token()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn thread_check_follows_host_handoff_and_rejects_workers() {
        let gate = Arc::new(Gate::new());
        assert!(gate.is_current());
        let other = gate.clone();
        thread::spawn(move || {
            assert!(!other.is_current());
            assert!(other.enter().is_err());
            let _host = unsafe { other.enter_host() };
            assert!(other.is_current());
            let _nested = other.enter().unwrap();
            assert!(other.is_current());
        })
        .join()
        .unwrap();
        assert!(!gate.is_current());
        assert!(gate.enter().is_err());
        let _host = unsafe { gate.enter_host() };
        assert!(gate.is_current());
    }
}

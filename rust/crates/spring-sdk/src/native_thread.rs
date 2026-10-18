/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Serialize native module entry while allowing immediate same-thread reentry.
//! Only unsafe host entry may change the accepted engine thread. Safe SDK calls
//! and retained callbacks can enter only on the thread most recently admitted.

use crate::{ApiError, ErrorCode, Result};
use std::{
    marker::PhantomData,
    sync::{
        Condvar, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

// Low bit of active is reserved for waiting host threads. Tokens never repeat.
static NEXT_THREAD: AtomicUsize = AtomicUsize::new(1);
thread_local! {
    static THREAD: std::cell::Cell<usize> = const {std::cell::Cell::new(0)};
}
#[cold]
#[inline(never)]
fn initialize_token(slot: &std::cell::Cell<usize>) -> usize {
    let token = NEXT_THREAD
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            (value < usize::MAX / 2).then_some(value + 1)
        })
        .expect("native SDK thread identity exhausted");
    slot.set(token);
    token
}
#[inline]
fn current_token() -> usize {
    THREAD.with(|slot| match slot.get() {
        0 => initialize_token(slot),
        token => token,
    })
}

pub(crate) struct Gate {
    active: AtomicUsize,
    engine_token: AtomicUsize,
    parked: Mutex<()>,
    released: Condvar,
}
pub(crate) struct Guard<'a> {
    gate: &'a Gate,
    outer: bool,
    affinity: PhantomData<*mut ()>,
}
impl Gate {
    pub(crate) fn new() -> Self {
        Self {
            active: AtomicUsize::new(0),
            engine_token: AtomicUsize::new(current_token()),
            parked: Mutex::new(()),
            released: Condvar::new(),
        }
    }
    #[inline]
    fn guard(&self, outer: bool) -> Guard<'_> {
        Guard {
            gate: self,
            outer,
            affinity: PhantomData,
        }
    }
    #[inline]
    pub(crate) fn enter(&self) -> Result<Guard<'_>> {
        let token = current_token();
        if self.engine_token.load(Ordering::Relaxed) != token {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        if self.active.load(Ordering::Relaxed) >> 1 == token {
            return Ok(self.guard(false));
        }
        if self
            .active
            .compare_exchange(0, token << 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        let guard = self.guard(true);
        // A host could have handed ownership off after the first token read.
        if self.engine_token.load(Ordering::Relaxed) != token {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        Ok(guard)
    }
    /// The caller is an engine ABI entry, not a worker requesting engine access.
    #[inline]
    pub(crate) unsafe fn enter_host(&self) -> Guard<'_> {
        let token = current_token();
        if self.active.load(Ordering::Relaxed) >> 1 == token {
            return self.guard(false);
        }
        if self
            .active
            .compare_exchange(0, token << 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            self.wait_for_host(token);
        }
        self.engine_token.store(token, Ordering::Relaxed);
        self.guard(true)
    }
    #[cold]
    fn wait_for_host(&self, token: usize) {
        let mut lock = self.parked.lock().unwrap_or_else(|p| p.into_inner());
        loop {
            let active = self.active.load(Ordering::Relaxed);
            if active == 0 {
                if self
                    .active
                    .compare_exchange(0, token << 1, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    return;
                }
                continue;
            }
            // Atomically marking this exact owner prevents a missed wakeup:
            // its release either observes the bit or wins this CAS, which
            // makes us retry without sleeping. The mutex covers wait/notify.
            if self
                .active
                .compare_exchange(active, active | 1, Ordering::Relaxed, Ordering::Relaxed)
                .is_err()
            {
                continue;
            }
            lock = self.released.wait(lock).unwrap_or_else(|p| p.into_inner());
        }
    }
    #[cfg(test)]
    #[inline]
    pub(crate) fn is_current(&self) -> bool {
        self.engine_token.load(Ordering::Relaxed) == current_token()
    }
    #[cold]
    #[inline(never)]
    fn wake_parked(&self) {
        let _lock = self.parked.lock().unwrap_or_else(|p| p.into_inner());
        self.released.notify_all();
    }
}
impl Drop for Guard<'_> {
    #[inline]
    fn drop(&mut self) {
        if !self.outer {
            return;
        }
        let previous = self.gate.active.swap(0, Ordering::Release);
        debug_assert_eq!(previous >> 1, current_token());
        if previous & 1 != 0 {
            self.gate.wake_parked();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn contending_host_entries_serialize_and_allow_nested_calls() {
        use std::sync::{Barrier, atomic::AtomicUsize};
        let gate = Arc::new(Gate::new());
        let barrier = Arc::new(Barrier::new(4));
        let active = Arc::new(AtomicUsize::new(0));
        let threads = (0..4)
            .map(|_| {
                let (gate, barrier, active) = (gate.clone(), barrier.clone(), active.clone());
                thread::spawn(move || {
                    barrier.wait();
                    for _ in 0..2000 {
                        let _outer = unsafe { gate.enter_host() };
                        assert_eq!(active.fetch_add(1, Ordering::Relaxed), 0);
                        {
                            let _nested = gate.enter().unwrap();
                            let _host = unsafe { gate.enter_host() };
                        }
                        thread::yield_now();
                        assert_eq!(active.fetch_sub(1, Ordering::Relaxed), 1);
                    }
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(active.load(Ordering::Relaxed), 0);
    }

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

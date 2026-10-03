use crate::{AiId, Context, Error, Event, Result, SkirmishAi, sys};
use std::{
    collections::BTreeMap,
    ffi::c_void,
    io::Write,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::Mutex,
};

struct Instance<A> {
    callbacks: usize,
    ai: A,
    quarantined: bool,
}
/// Owns each AI independently. A process can load the same library for several teams.
/// Callback addresses are used only during an engine dispatch, under the registry lock.
/// No dependency on initialization and update running on the same OS thread.
pub struct Runtime<A> {
    instances: Mutex<BTreeMap<AiId, Instance<A>>>,
}
impl<A: SkirmishAi> Default for Runtime<A> {
    fn default() -> Self {
        Self::new()
    }
}
impl<A: SkirmishAi> Runtime<A> {
    pub const fn new() -> Self {
        Self {
            instances: Mutex::new(BTreeMap::new()),
        }
    }
    fn boundary(f: impl FnOnce() -> Result<()>) -> i32 {
        match catch_unwind(AssertUnwindSafe(f)) {
            Ok(Ok(())) => 0,
            Ok(Err(e)) => {
                let _ = writeln!(std::io::stderr().lock(), "Skirmish AI: {e}");
                -1
            }
            Err(_) => {
                let _ = writeln!(std::io::stderr().lock(), "Skirmish AI: panic contained");
                -1
            }
        }
    }
    /// # Safety
    /// The callback table must match the SDK ABI and remain live until `release`.
    pub unsafe fn init(&self, id: AiId, callbacks: *const sys::SSkirmishAICallback) -> i32 {
        Self::boundary(|| {
            if id.0 < 0
                || callbacks.is_null()
                || (callbacks as usize) % std::mem::align_of::<sys::SSkirmishAICallback>() != 0
            {
                return Err(Error::InvalidInput("invalid AI initialization"));
            }
            let mut registry = self.instances.try_lock().map_err(|_| Error::Busy)?;
            if registry.contains_key(&id) {
                return Err(Error::DuplicateAi(id));
            }
            let context = Context::new(id, unsafe { &*callbacks });
            let ai = catch_unwind(AssertUnwindSafe(|| A::new(&context)))
                .map_err(|_| Error::Panicked)??;
            registry.insert(
                id,
                Instance {
                    callbacks: callbacks as usize,
                    ai,
                    quarantined: false,
                },
            );
            Ok(())
        })
    }
    /// # Safety
    /// Event pointers must match `topic` and remain live for the dispatch.
    pub unsafe fn handle(&self, id: AiId, topic: i32, data: *const c_void) -> i32 {
        Self::boundary(|| {
            let mut registry = self.instances.try_lock().map_err(|_| Error::Busy)?;
            let instance = registry.get_mut(&id).ok_or(Error::UnknownAi(id))?;
            if instance.quarantined {
                return Err(Error::Quarantined(id));
            }
            let context = Context::new(id, unsafe {
                &*(instance.callbacks as *const sys::SSkirmishAICallback)
            });
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                instance
                    .ai
                    .handle(&context, unsafe { Event::decode(topic, data)? })
            }));
            match outcome {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => {
                    instance.quarantined = true;
                    Err(e)
                }
                Err(_) => {
                    instance.quarantined = true;
                    Err(Error::Panicked)
                }
            }
        })
    }
    pub fn release(&self, id: AiId) -> i32 {
        Self::boundary(|| {
            let instance = self
                .instances
                .try_lock()
                .map_err(|_| Error::Busy)?
                .remove(&id);
            drop(instance);
            Ok(())
        })
    }
}

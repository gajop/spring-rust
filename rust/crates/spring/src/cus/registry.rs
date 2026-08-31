/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::instance::CusInstance;
use super::scheduler::remove_waiter;
use crate::UnitId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// A typed registry for one script type. Generations prevent a stale handle
/// from accessing a reused slot.
pub struct CusRegistry<S> {
    entries: Vec<Option<RegistryEntry<S>>>,
    generations: Vec<u32>,
    frame: u64,
    scheduled: BTreeMap<u64, Vec<UnitId>>,
    due_units: Vec<UnitId>,
}

struct RegistryEntry<S> {
    generation: u32,
    instance: CusInstance<S>,
    scheduled_frame: Option<u64>,
    queued_due: bool,
}

/// Generation-safe handle returned by [`CusRegistry::attach`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CusHandle {
    unit: UnitId,
    generation: u32,
}

impl<S> Default for CusRegistry<S> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            generations: Vec::new(),
            frame: 0,
            scheduled: BTreeMap::new(),
            due_units: Vec::new(),
        }
    }
}

impl<S> CusRegistry<S> {
    fn unschedule(&mut self, unit: UnitId) {
        let Some(entry) = self
            .entries
            .get_mut(unit.0 as usize)
            .and_then(Option::as_mut)
        else {
            return;
        };
        let Some(frame) = entry.scheduled_frame.take() else {
            return;
        };
        remove_waiter(&mut self.scheduled, frame, unit);
    }

    fn refresh(&mut self, unit: UnitId) {
        if unit.0 < 0 {
            return;
        }
        let index = unit.0 as usize;
        let Some(entry) = self.entries.get_mut(index).and_then(Option::as_mut) else {
            return;
        };
        let next = entry.instance.next_wake_frame();
        if entry.scheduled_frame == next {
            return;
        }
        if let Some(old_frame) = entry.scheduled_frame.take() {
            remove_waiter(&mut self.scheduled, old_frame, unit);
        }
        if let Some(next_frame) = next {
            entry.scheduled_frame = Some(next_frame);
            self.scheduled.entry(next_frame).or_default().push(unit);
        }
    }

    pub fn attach(&mut self, instance: CusInstance<S>) -> CusHandle {
        let unit = instance.unit();
        assert!(unit.0 >= 0, "CUS unit IDs must be non-negative");
        let index = unit.0 as usize;
        if self.entries.len() <= index {
            self.entries.resize_with(index + 1, || None);
            self.generations.resize(index + 1, 0);
        }
        if self.entries[index].is_some() {
            self.unschedule(unit);
            self.due_units.retain(|candidate| *candidate != unit);
        }
        let generation = self.generations[index].wrapping_add(1).max(1);
        self.generations[index] = generation;
        self.entries[index] = Some(RegistryEntry {
            generation,
            instance,
            scheduled_frame: None,
            queued_due: false,
        });
        CusHandle { unit, generation }
    }

    pub fn with<R>(
        &mut self,
        handle: CusHandle,
        f: impl FnOnce(&mut CusInstance<S>) -> R,
    ) -> Option<R> {
        if handle.unit.0 < 0 {
            return None;
        }
        let result = {
            let entry = self.entries.get_mut(handle.unit.0 as usize)?.as_mut()?;
            if entry.generation != handle.generation {
                return None;
            }
            f(&mut entry.instance)
        };
        self.refresh(handle.unit);
        Some(result)
    }

    pub fn detach(&mut self, handle: CusHandle) -> Option<CusInstance<S>> {
        if handle.unit.0 < 0 {
            return None;
        }
        let index = handle.unit.0 as usize;
        let entry = self.entries.get_mut(index)?.take()?;
        if entry.generation != handle.generation {
            self.entries[index] = Some(entry);
            return None;
        }
        if let Some(frame) = entry.scheduled_frame {
            remove_waiter(&mut self.scheduled, frame, handle.unit);
        }
        self.due_units.retain(|unit| *unit != handle.unit);
        Some(entry.instance)
    }

    /// Drain instances whose module-level wake deadline has arrived, in
    /// ascending unit-ID order. Idle instances do not cross the scheduler
    /// boundary on this frame.
    pub fn tick(&mut self, frame: u64) {
        if frame < self.frame {
            return;
        }
        self.frame = frame;
        while let Some((&wake_frame, _)) = self.scheduled.first_key_value() {
            if wake_frame > frame {
                break;
            }
            let Some(units) = self.scheduled.remove(&wake_frame) else {
                continue;
            };
            for unit in units {
                let Some(entry) = self
                    .entries
                    .get_mut(unit.0 as usize)
                    .and_then(Option::as_mut)
                else {
                    continue;
                };
                if entry.scheduled_frame != Some(wake_frame) {
                    continue;
                }
                entry.scheduled_frame = None;
                if !entry.queued_due {
                    entry.queued_due = true;
                    self.due_units.push(unit);
                }
            }
        }

        self.due_units.sort_unstable_by_key(|unit| unit.0);
        let initial_due = self.due_units.len();
        for index in 0..initial_due {
            let unit = self.due_units[index];
            let Some(entry) = self
                .entries
                .get_mut(unit.0 as usize)
                .and_then(Option::as_mut)
            else {
                continue;
            };
            entry.queued_due = false;
            if entry.instance.is_due(frame) {
                entry.instance.tick(frame);
            }
            self.refresh(unit);
        }
        self.due_units.drain(..initial_due);
    }

    #[inline]
    pub fn handle_for(&self, unit: UnitId) -> Option<CusHandle> {
        if unit.0 < 0 {
            return None;
        }
        let generation = self.entries.get(unit.0 as usize)?.as_ref()?.generation;
        Some(CusHandle { unit, generation })
    }
}

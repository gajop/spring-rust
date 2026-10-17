/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::shared::{Shared, SharedCell};
use super::*;
use crate::{Result, RulesSynced, Spring, UnitId, capabilities::Synced};
use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
};

pub trait UnitDefScript<Shared = (), E: Synced = RulesSynced>:
    UnitScript + crate::RetainedCapture
{
    fn capabilities(spring: &Spring<'_, E>, def: i32) -> Option<ScriptCapabilities>;
    fn for_def(
        spring: &Spring<'_, E>,
        def: i32,
        ctx: &mut InitCtx<'_>,
        shared: &mut Shared,
    ) -> Self;
}

pub struct UnitPieces {
    map: Vec<crate::units_pieces::PieceMapEntry>,
    root: Piece,
}
impl UnitPieces {
    pub fn for_unit<E: Synced>(spring: &Spring<'_, E>, unit: UnitId) -> Result<Self> {
        let map = spring.units_pieces().get_unit_piece_map(unit.0)?;
        let root = spring.units_pieces().get_unit_root_piece(unit.0)?;
        Ok(Self {
            map,
            root: root.checked_sub(1).map_or(Piece::INVALID, Piece),
        })
    }
}
impl PieceResolver for UnitPieces {
    fn piece(&self, name: &str) -> Piece {
        self.map
            .iter()
            .find(|entry| entry.name == name)
            .map_or(Piece::INVALID, |entry| Piece(entry.piece_num - 1))
    }
    fn root(&self) -> Piece {
        self.root
    }
    fn pieces(&self) -> Vec<(&str, Piece)> {
        self.map
            .iter()
            .map(|entry| (entry.name.as_str(), Piece(entry.piece_num - 1)))
            .collect()
    }
}

pub struct CusHost<S, Shared = (), E: Synced = RulesSynced> {
    registry: RefCell<CusRegistry<S>>,
    handles: RefCell<BTreeMap<u32, CusHandle>>,
    units: RefCell<BTreeMap<i32, u32>>,
    next_instance: Cell<u32>,
    pending_create: RefCell<BTreeSet<u32>>,
    pub shared: RefCell<Shared>,
    environment: PhantomData<fn() -> E>,
}
impl<S, Shared: Default, E: Synced> Default for CusHost<S, Shared, E> {
    fn default() -> Self {
        Self {
            registry: RefCell::new(CusRegistry::default()),
            handles: RefCell::new(BTreeMap::new()),
            units: RefCell::new(BTreeMap::new()),
            next_instance: Cell::new(0x4355_0000),
            pending_create: RefCell::new(BTreeSet::new()),
            shared: RefCell::new(Shared::default()),
            environment: PhantomData,
        }
    }
}
fn current_frame<E: Synced>(spring: &Spring<'_, E>) -> Result<u64> {
    let frame = spring.game().get_game_frame()?;
    let frame = (frame.high16 << 16) | frame.low16;
    Ok((frame as i32).max(0) as u64)
}

impl<S: UnitScript, State, E: Synced> CusHost<S, State, E> {
    pub fn attach(
        &self,
        spring: &Spring<'_, E>,
        unit: UnitId,
        capabilities: ScriptCapabilities,
        make: impl FnOnce(&mut InitCtx<'_>) -> S,
    ) -> Result<()> {
        self.attach_shared(spring, unit, capabilities, |ctx, _| make(ctx))
    }
    pub fn attach_shared(
        &self,
        spring: &Spring<'_, E>,
        unit: UnitId,
        capabilities: ScriptCapabilities,
        make: impl FnOnce(&mut InitCtx<'_>, &mut State) -> S,
    ) -> Result<()> {
        if self.units.borrow().contains_key(&unit.0) {
            return Ok(());
        }
        let instance = self.next_instance.get();
        self.next_instance
            .set(instance.checked_add(1).ok_or(crate::ApiError::new(
                crate::ErrorCode::OperationFailed as i32,
            ))?);
        let pieces = UnitPieces::for_unit(spring, unit)?;
        let state = make(
            &mut InitCtx::new(unit, &pieces),
            &mut self.shared.borrow_mut(),
        );
        let engine = Shared::new(SharedCell::new(spring.cus().engine(unit, instance)));
        let handle = self
            .registry
            .borrow_mut()
            .attach(CusInstance::attach(unit, state, engine));
        self.handles.borrow_mut().insert(instance, handle);
        self.units.borrow_mut().insert(unit.0, instance);
        if capabilities.contains(ScriptCapabilities::CREATE) {
            self.pending_create.borrow_mut().insert(instance);
        }
        if let Err(error) = spring.cus().attach(unit, instance, capabilities) {
            self.registry.borrow_mut().detach(handle);
            self.handles.borrow_mut().remove(&instance);
            self.units.borrow_mut().remove(&unit.0);
            self.pending_create.borrow_mut().remove(&instance);
            return Err(error);
        }
        Ok(())
    }
    pub fn is_attached(&self, unit: UnitId) -> bool {
        self.units.borrow().contains_key(&unit.0)
    }
    pub fn with_script<R>(
        &self,
        spring: &Spring<'_, E>,
        unit: UnitId,
        f: impl FnOnce(&UnitCtx, &mut S) -> R,
    ) -> Result<Option<R>> {
        let instance_id = self.units.borrow().get(&unit.0).copied();
        let Some(instance_id) = instance_id else {
            return Ok(None);
        };
        let Some((_, instance)) = self.prepare(spring, instance_id)? else {
            return Ok(None);
        };
        let ctx = instance.context();
        let result = instance.with_state(|script| f(&ctx, script));
        self.registry.borrow_mut().refresh(unit);
        Ok(Some(result))
    }
    pub fn with_state<R>(&self, unit: UnitId, f: impl FnOnce(&mut S) -> R) -> Option<R> {
        let instance_id = self.units.borrow().get(&unit.0).copied()?;
        let handle = self.handles.borrow().get(&instance_id).copied()?;
        let instance = self.registry.borrow().instance(handle)?;
        Some(instance.with_state(f))
    }
    pub fn call(
        &self,
        spring: &Spring<'_, E>,
        unit: UnitId,
        name: &str,
        arguments: &[f32],
        returns: &mut [f32],
    ) -> Result<Option<usize>> {
        self.with_script(spring, unit, |ctx, script| {
            script.call_named(ctx, name, arguments, returns)
        })
        .map(Option::flatten)
    }
    pub fn game_frame(&self, frame: i32) {
        self.cus_tick(frame.max(0) as u32);
    }
    pub fn registry(&self) -> RefMut<'_, CusRegistry<S>> {
        self.registry.borrow_mut()
    }
    fn prepare(
        &self,
        spring: &Spring<'_, E>,
        instance_id: u32,
    ) -> Result<Option<(CusHandle, Shared<CusInstance<S>>)>> {
        let Some(handle) = self.handles.borrow().get(&instance_id).copied() else {
            return Ok(None);
        };
        let Some(instance) = self.registry.borrow().instance(handle) else {
            return Ok(None);
        };
        instance.tick(current_frame(spring)?);
        let create = self.pending_create.borrow_mut().remove(&instance_id);
        if create {
            instance.invoke(
                UnitScriptCall::Create,
                &[],
                &[],
                &mut UnitScriptCallResult::default(),
            );
        }
        self.registry.borrow_mut().refresh(instance.unit());
        Ok(Some((handle, instance)))
    }
}

impl<S: UnitDefScript<State, E>, State, E: Synced> CusHost<S, State, E> {
    pub fn unit_created(&self, spring: &Spring<'_, E>, unit: UnitId, def: i32) -> Result<()> {
        let Some(capabilities) = S::capabilities(spring, def) else {
            return Ok(());
        };
        match spring.units_info().get_unit_def_id(unit.0) {
            Ok(current) if current == def => {}
            Ok(_) => return Ok(()),
            Err(error) if error.code == crate::ErrorCode::InvalidArgument as i32 => return Ok(()),
            Err(error) => return Err(error),
        }
        self.attach_shared(spring, unit, capabilities, |ctx, shared| {
            S::for_def(spring, def, ctx, shared)
        })
    }
    pub fn attach_existing(&self, spring: &Spring<'_, E>) -> Result<()> {
        for unit in spring.units_query().get_all_units()? {
            let def = spring.units_info().get_unit_def_id(unit)?;
            self.unit_created(spring, UnitId(unit), def)?;
        }
        Ok(())
    }
}

pub struct CusCallResult<'a> {
    pub int_value: i32,
    pub float_value: f32,
    pub bool_value: bool,
    pub complete: bool,
    pub int_count: usize,
    pub int_values: &'a mut [i32],
}
pub trait CusModule<E: Synced> {
    fn cus_invoke(
        &self,
        spring: &Spring<'_, E>,
        instance: u32,
        call: u32,
        floats: &[f32],
        integers: &[i32],
        result: &mut CusCallResult<'_>,
    ) -> Result<bool>;
    fn cus_call_named(
        &self,
        spring: &Spring<'_, E>,
        instance: u32,
        name: &str,
        arguments: &[f32],
        returns: &mut [f32],
        found: &mut bool,
    ) -> Result<Option<usize>>;
    fn cus_tick(&self, frame: u32);
    fn cus_detach(&self, instance: u32);
}
impl<S: UnitScript, State, E: Synced> CusModule<E> for CusHost<S, State, E> {
    fn cus_invoke(
        &self,
        spring: &Spring<'_, E>,
        instance: u32,
        call: u32,
        floats: &[f32],
        integers: &[i32],
        result: &mut CusCallResult<'_>,
    ) -> Result<bool> {
        let Some(call) = UnitScriptCall::from_u32(call) else {
            return Ok(false);
        };
        if call == UnitScriptCall::Create && !self.pending_create.borrow().contains(&instance) {
            return Ok(self.handles.borrow().contains_key(&instance));
        }
        let Some((_, script)) = self.prepare(spring, instance)? else {
            return Ok(false);
        };
        if call == UnitScriptCall::Create {
            return Ok(true);
        }
        let mut returned = UnitScriptCallResult::default();
        let handled = script
            .try_invoke(call, floats, integers, &mut returned)
            .unwrap_or(false);
        self.registry.borrow_mut().refresh(script.unit());
        if !handled {
            return Ok(false);
        }
        if returned.int_values.len() > result.int_values.len() {
            return Err(crate::ApiError::new(
                crate::ErrorCode::BufferOverflow as i32,
            ));
        }
        result.int_value = returned.int_value;
        result.float_value = returned.float_value;
        result.bool_value = returned.bool_value;
        result.complete = returned.complete;
        result.int_count = returned.int_values.len();
        result.int_values[..result.int_count].copy_from_slice(&returned.int_values);
        Ok(true)
    }
    fn cus_call_named(
        &self,
        spring: &Spring<'_, E>,
        instance: u32,
        name: &str,
        arguments: &[f32],
        returns: &mut [f32],
        found: &mut bool,
    ) -> Result<Option<usize>> {
        let Some((_, script)) = self.prepare(spring, instance)? else {
            return Ok(None);
        };
        let ctx = script.context();
        let state = script.state();
        let Ok(mut state) = state.try_borrow_mut() else {
            return Ok(None);
        };
        let count = state.call_named(&ctx, name, arguments, returns);
        drop(state);
        self.registry.borrow_mut().refresh(script.unit());
        *found = count.is_some();
        Ok(Some(count.unwrap_or(0)))
    }
    fn cus_tick(&self, frame: u32) {
        let due = self.registry.borrow_mut().due(u64::from(frame));
        for (_, instance) in due {
            instance.tick(u64::from(frame));
            self.registry.borrow_mut().refresh(instance.unit());
        }
    }
    fn cus_detach(&self, instance: u32) {
        self.pending_create.borrow_mut().remove(&instance);
        if let Some(handle) = self.handles.borrow_mut().remove(&instance) {
            if let Some(script) = self.registry.borrow().instance(handle) {
                self.units.borrow_mut().remove(&script.unit().0);
            }
            self.registry.borrow_mut().detach(handle);
        }
    }
}

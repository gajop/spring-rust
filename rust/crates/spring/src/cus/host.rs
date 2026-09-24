/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! A ready-made Core-CUS module: attach, piece lookup, frame sync, detach and
//! named calls, so a game only writes its scripts.
//!
//! ```rust,ignore
//! use spring::cus::host::{CusHost, UnitDefScript};
//!
//! impl UnitDefScript for MyScript {
//!     fn capabilities(def: i32) -> Option<ScriptCapabilities> { /* by def */ }
//!     fn for_def(def: i32, ctx: &mut InitCtx<'_>) -> Self { /* build it */ }
//! }
//!
//! spring::export_core_cus!(CusHost<MyScript>);
//!
//! fn unit_created(unit: UnitId, def: DefId, _: TeamId, _: UnitId) {
//!     with_cus_module_or_defer(move |host| host.unit_created(unit, def.0));
//! }
//! ```
//!
//! The engine calls each script's `create` once, after the module has returned
//! from the call that attached it; see [`export_core_cus!`](crate::export_core_cus).

use super::core_module::{CoreCusCallResult, CoreCusModule};
use super::wasm::WasmCus;
use super::{
    CusHandle, CusInstance, CusRegistry, InitCtx, Piece, PieceResolver, ScriptCapabilities,
    UnitCtx, UnitScript, UnitScriptCall, UnitScriptCallResult,
};
use crate::{ApiError, UnitId};
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

/// A script type that knows which unit defs get it.
pub trait UnitDefScript: UnitScript {
    /// The capabilities of `def`'s script, or `None` when units of `def` have
    /// no script.
    fn capabilities(def: i32) -> Option<ScriptCapabilities>;

    /// Build the script for a unit of `def`.
    fn for_def(def: i32, ctx: &mut InitCtx<'_>) -> Self;
}

/// Model pieces of one unit, from the engine's piece map. Script piece numbers
/// are the engine's minus one.
pub struct UnitPieces {
    map: Vec<crate::owned::units_pieces::PieceMapEntry>,
    root: Piece,
}

impl UnitPieces {
    pub fn for_unit(unit: UnitId) -> crate::Result<Self> {
        let map = crate::owned::units_pieces::get_unit_piece_map(unit.0)?;
        let root = crate::owned::units_pieces::get_unit_root_piece(unit.0)
            .ok()
            .and_then(|piece| piece.checked_sub(1))
            .map_or(Piece::INVALID, Piece);
        Ok(Self { map, root })
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
}

/// Owns the scripts of one module: see the [module docs](self).
pub struct CusHost<S> {
    registry: CusRegistry<S>,
    handles: BTreeMap<u32, CusHandle>,
}

impl<S> Default for CusHost<S> {
    fn default() -> Self {
        Self {
            registry: CusRegistry::default(),
            handles: BTreeMap::new(),
        }
    }
}

// Instance ids only need to be unique within this module; the engine pairs
// them with the unit id.
const INSTANCE_TAG: u32 = 0x4355_0000;

fn instance_id(unit: UnitId) -> u32 {
    INSTANCE_TAG.wrapping_add(unit.0 as u32)
}

fn current_frame() -> u64 {
    u64::from(crate::game::frame().unwrap_or(0))
}

impl<S: UnitScript> CusHost<S> {
    /// Attach the script built by `make` to `unit`. Does nothing, successfully,
    /// if the unit already has one. The engine then calls its `create`.
    pub fn attach(
        &mut self,
        unit: UnitId,
        capabilities: ScriptCapabilities,
        make: impl FnOnce(&mut InitCtx<'_>) -> S,
    ) -> crate::Result<()> {
        let instance = instance_id(unit);
        if self.handles.contains_key(&instance) {
            return Ok(());
        }
        let pieces = UnitPieces::for_unit(unit)?;
        let state = make(&mut InitCtx::new(unit, &pieces));
        let engine = Rc::new(RefCell::new(WasmCus::new(unit, instance).engine()));
        let handle = self
            .registry
            .attach(CusInstance::attach(unit, state, engine));
        if let Err(error) = WasmCus::attach(unit, instance, capabilities) {
            let _ = self.registry.detach(handle);
            return Err(error);
        }
        self.handles.insert(instance, handle);
        Ok(())
    }

    /// Whether `unit` has a script from this module.
    pub fn is_attached(&self, unit: UnitId) -> bool {
        self.handles.contains_key(&instance_id(unit))
    }

    /// Run `f` with the unit's script, at the current frame. `None` when the
    /// unit has no script.
    pub fn with_script<R>(
        &mut self,
        unit: UnitId,
        f: impl FnOnce(&UnitCtx, &mut S) -> R,
    ) -> Option<R> {
        let handle = self.sync(instance_id(unit))?;
        self.registry.with(handle, |instance| {
            let ctx = instance.context();
            instance.with_state(|script| f(&ctx, script))
        })
    }

    /// Call the named function of the unit's script, like Lua's
    /// `env.Function(...)` ([`UnitScript::call_named`]). Returns the number of
    /// values written to `returns`, or `None` when the unit has no script or
    /// its script has no such function.
    pub fn call(
        &mut self,
        unit: UnitId,
        name: &str,
        arguments: &[f32],
        returns: &mut [f32],
    ) -> Option<usize> {
        self.with_script(unit, |ctx, script| {
            script.call_named(ctx, name, arguments, returns)
        })
        .flatten()
    }

    /// The registry, for per-instance access this type doesn't cover.
    pub fn registry(&mut self) -> &mut CusRegistry<S> {
        &mut self.registry
    }

    // Tasks start synchronously inside a call, so bring the instance's clock to
    // the current frame first; an idle instance would otherwise start a sleep
    // against an old frame.
    fn sync(&mut self, instance: u32) -> Option<CusHandle> {
        let handle = self.handles.get(&instance).copied()?;
        let frame = current_frame();
        let _ = self.registry.with(handle, |instance| instance.tick(frame));
        Some(handle)
    }
}

impl<S: UnitDefScript> CusHost<S> {
    /// Attach `def`'s script, if it has one, to a new unit. Also safe for work
    /// deferred with `with_cus_module_or_defer`: a unit that has died, or whose
    /// id now belongs to a unit of another def, is skipped.
    pub fn unit_created(&mut self, unit: UnitId, def: i32) {
        let Some(capabilities) = S::capabilities(def) else {
            return;
        };
        if crate::owned::units_info::get_unit_def_id(unit.0).ok() != Some(def) {
            return;
        }
        if let Err(error) = self.attach(unit, capabilities, |ctx| S::for_def(def, ctx)) {
            log_attach_error(unit, error);
        }
    }

    /// Attach scripts to units that existed before this module loaded (a
    /// `/luarules reload`, or units placed by the map or start script).
    pub fn attach_existing(&mut self) {
        let Ok(units) = crate::owned::units_query::get_all_units(0) else {
            return;
        };
        for unit in units {
            if let Ok(def) = crate::owned::units_info::get_unit_def_id(unit) {
                self.unit_created(UnitId(unit), def);
            }
        }
    }
}

fn log_attach_error(unit: UnitId, error: ApiError) {
    let _ = crate::log(
        "cus",
        crate::log_level::WARNING,
        &alloc::format!("no unit script for unit {}: {error:?}", unit.0),
    );
}

impl<S: UnitScript> CoreCusModule for CusHost<S> {
    fn cus_invoke(
        &mut self,
        instance_id: u32,
        call: u32,
        float_arguments: &[f32],
        integer_arguments: &[i32],
        result: &mut CoreCusCallResult<'_>,
    ) -> bool {
        let Some(call) = UnitScriptCall::from_u32(call) else {
            return false;
        };
        let Some(handle) = self.sync(instance_id) else {
            return false;
        };
        let mut call_result = UnitScriptCallResult::default();
        let handled = self
            .registry
            .with(handle, |instance| {
                instance.invoke(call, float_arguments, integer_arguments, &mut call_result)
            })
            .unwrap_or(false);
        if !handled {
            return false;
        }
        result.int_value = call_result.int_value;
        result.float_value = call_result.float_value;
        result.bool_value = call_result.bool_value;
        result.complete = call_result.complete;
        result.int_count = call_result.int_values.len().min(result.int_values.len());
        result.int_values[..result.int_count]
            .copy_from_slice(&call_result.int_values[..result.int_count]);
        true
    }

    fn cus_call_named(
        &mut self,
        instance_id: u32,
        function_name: &str,
        arguments: &[f32],
        return_values: &mut [f32],
        found: &mut bool,
    ) -> Option<usize> {
        let handle = self.sync(instance_id)?;
        let count = self.registry.with(handle, |instance| {
            let ctx = instance.context();
            instance.with_state(|script| {
                script.call_named(&ctx, function_name, arguments, return_values)
            })
        })?;
        *found = count.is_some();
        Some(count.unwrap_or(0))
    }

    fn cus_tick(&mut self, frame: u32) {
        self.registry.tick(u64::from(frame));
    }

    fn cus_detach(&mut self, instance_id: u32) {
        if let Some(handle) = self.handles.remove(&instance_id) {
            let _ = self.registry.detach(handle);
        }
    }
}

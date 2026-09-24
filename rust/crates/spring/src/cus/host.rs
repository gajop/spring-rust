/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! A ready-made Core-CUS module: attach, piece lookup, frame sync, detach and
//! named calls, so a game only writes its scripts.
//!
//! ```rust,ignore
//! use spring::cus::host::{CusHost, UnitDefScript};
//!
//! impl UnitDefScript for MyScript {
//!     fn capabilities(def: i32) -> Option<ScriptCapabilities> { /* by def */ }
//!     fn for_def(def: i32, ctx: &mut InitCtx<'_>, _: &mut ()) -> Self { /* build it */ }
//! }
//!
//! spring::export_core_cus!(CusHost<MyScript>);
//!
//! fn unit_created(unit: UnitId, def: DefId, _: TeamId, _: UnitId) {
//!     with_cus_module_or_defer(move |host| host.unit_created(unit, def.0));
//! }
//!
//! fn game_frame(frame: i32) {
//!     with_cus_module_or_defer(move |host| host.game_frame(frame));
//! }
//! ```
//!
//! The script type must be `pub` (or at least as visible as the module's
//! exports): `with_cus_module_or_defer` closures receive `CusHost<S>`.
//!
//! # Timing
//!
//! - **`create`** runs once per attach. The engine sends it after the module
//!   has returned from the call that attached the script (see
//!   [`export_core_cus!`](crate::export_core_cus)). If anything else reaches the
//!   script first (a named call, an engine callin such as `AimWeapon`), the host
//!   runs `create` right before it, so a script never sees a call before its
//!   `create`, as in Lua where `Create` runs inside `CreateUnit`. `create` only
//!   runs for scripts attached with [`ScriptCapabilities::CREATE`].
//! - **Sleeping tasks** wake like Lua's: Lua unit scripts resume in
//!   `GameFrame`, before the units update (and aim) on that frame. Call
//!   [`CusHost::game_frame`] from the module's `GameFrame` to wake every script
//!   there. Without it, a script still catches up before each call it gets,
//!   and the rest wake in the engine's CUS tick after the unit update.

use super::core_module::{CoreCusCallResult, CoreCusModule};
use super::wasm::WasmCus;
use super::{
    CusHandle, CusInstance, CusRegistry, InitCtx, Piece, PieceResolver, ScriptCapabilities,
    UnitCtx, UnitScript, UnitScriptCall, UnitScriptCallResult,
};
use crate::{ApiError, UnitId};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

/// A script type that knows which unit defs get it.
///
/// `Shared` is state all scripts of the module may need while they are built,
/// kept in [`CusHost::shared`]; hand scripts an `Rc` clone of whatever they need
/// later. Most games use `()`.
pub trait UnitDefScript<Shared = ()>: UnitScript {
    /// The capabilities of `def`'s script, or `None` when units of `def` have
    /// no script.
    fn capabilities(def: i32) -> Option<ScriptCapabilities>;

    /// Build the script for a unit of `def`.
    fn for_def(def: i32, ctx: &mut InitCtx<'_>, shared: &mut Shared) -> Self;
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

    fn pieces(&self) -> Vec<(&str, Piece)> {
        self.map
            .iter()
            .map(|entry| (entry.name.as_str(), Piece(entry.piece_num - 1)))
            .collect()
    }
}

/// Owns the scripts of one module: see the [module docs](self).
pub struct CusHost<S, Shared = ()> {
    registry: CusRegistry<S>,
    handles: BTreeMap<u32, CusHandle>,
    // Attached with CREATE, but `create` hasn't run yet.
    pending_create: BTreeSet<u32>,
    /// State shared by the module's scripts; see [`UnitDefScript`].
    pub shared: Shared,
}

impl<S, Shared: Default> Default for CusHost<S, Shared> {
    fn default() -> Self {
        Self {
            registry: CusRegistry::default(),
            handles: BTreeMap::new(),
            pending_create: BTreeSet::new(),
            shared: Shared::default(),
        }
    }
}

// Instance ids only need to be unique within this module; the engine pairs
// them with the unit id.
const INSTANCE_TAG: u32 = 0x4355_0000;

fn instance_id(unit: UnitId) -> u32 {
    INSTANCE_TAG.wrapping_add(unit.0 as u32)
}

// Units created before the first game frame (frame -1) start at frame 0; a
// frame of -1 read as u32 would put their script clock at u32::MAX.
fn current_frame() -> u64 {
    crate::game::current_frame().max(0) as u64
}

impl<S: UnitScript, Shared> CusHost<S, Shared> {
    /// Attach the script built by `make` to `unit`. Does nothing, successfully,
    /// if the unit already has one. The engine then calls its `create` (with
    /// [`ScriptCapabilities::CREATE`]).
    pub fn attach(
        &mut self,
        unit: UnitId,
        capabilities: ScriptCapabilities,
        make: impl FnOnce(&mut InitCtx<'_>) -> S,
    ) -> crate::Result<()> {
        self.attach_shared(unit, capabilities, |ctx, _| make(ctx))
    }

    /// [`attach`](Self::attach), with [`CusHost::shared`] passed to `make`.
    pub fn attach_shared(
        &mut self,
        unit: UnitId,
        capabilities: ScriptCapabilities,
        make: impl FnOnce(&mut InitCtx<'_>, &mut Shared) -> S,
    ) -> crate::Result<()> {
        let instance = instance_id(unit);
        if self.handles.contains_key(&instance) {
            return Ok(());
        }
        let pieces = UnitPieces::for_unit(unit)?;
        let state = make(&mut InitCtx::new(unit, &pieces), &mut self.shared);
        let engine = Rc::new(RefCell::new(WasmCus::new(unit, instance).engine()));
        let handle = self
            .registry
            .attach(CusInstance::attach(unit, state, engine));
        if let Err(error) = WasmCus::attach(unit, instance, capabilities) {
            let _ = self.registry.detach(handle);
            return Err(error);
        }
        self.handles.insert(instance, handle);
        if capabilities.contains(ScriptCapabilities::CREATE) {
            self.pending_create.insert(instance);
        }
        Ok(())
    }

    /// Whether `unit` has a script from this module.
    pub fn is_attached(&self, unit: UnitId) -> bool {
        self.handles.contains_key(&instance_id(unit))
    }

    /// Run `f` with the unit's script, at the current frame: the script's
    /// clock catches up (waking due tasks) and a pending `create` runs first.
    /// `None` when the unit has no script.
    pub fn with_script<R>(
        &mut self,
        unit: UnitId,
        f: impl FnOnce(&UnitCtx, &mut S) -> R,
    ) -> Option<R> {
        let handle = self.prepare(instance_id(unit))?;
        self.registry.with(handle, |instance| {
            let ctx = instance.context();
            instance.with_state(|script| f(&ctx, script))
        })
    }

    /// Read or change the unit's script state without running anything:
    /// no clock update, no pending `create`. `None` when the unit has no
    /// script.
    pub fn with_state<R>(&mut self, unit: UnitId, f: impl FnOnce(&mut S) -> R) -> Option<R> {
        let handle = self.handles.get(&instance_id(unit)).copied()?;
        self.registry
            .with(handle, |instance| instance.with_state(f))
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

    /// Wake every script's tasks due by `frame`, where Lua wakes its unit
    /// scripts. Call it from the module's `GameFrame`.
    pub fn game_frame(&mut self, frame: i32) {
        self.registry.tick(frame.max(0) as u64);
    }

    /// The registry, for per-instance access this type doesn't cover.
    pub fn registry(&mut self) -> &mut CusRegistry<S> {
        &mut self.registry
    }

    // Tasks start synchronously inside a call, so bring the instance's clock to
    // the current frame first; an idle instance would otherwise start a sleep
    // against an old frame. Then run a pending `create`, so no call reaches a
    // script before it.
    fn prepare(&mut self, instance: u32) -> Option<CusHandle> {
        let handle = self.handles.get(&instance).copied()?;
        let frame = current_frame();
        let _ = self.registry.with(handle, |instance| instance.tick(frame));
        if self.pending_create.remove(&instance) {
            let mut result = UnitScriptCallResult::default();
            let _ = self.registry.with(handle, |instance| {
                instance.invoke(UnitScriptCall::Create, &[], &[], &mut result)
            });
        }
        Some(handle)
    }
}

impl<S: UnitDefScript<Shared>, Shared> CusHost<S, Shared> {
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
        if let Err(error) = self.attach_shared(unit, capabilities, |ctx, shared| {
            S::for_def(def, ctx, shared)
        }) {
            log_attach_error(unit, error);
        }
    }

    /// Attach scripts to units that existed before this module loaded (a
    /// rules restart, or units placed by the map or start script).
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

impl<S: UnitScript, Shared> CoreCusModule for CusHost<S, Shared>
where
    Self: Default,
{
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
        // The engine's own Create: `prepare` has already run it if something
        // else reached the script first.
        if call == UnitScriptCall::Create && !self.pending_create.contains(&instance_id) {
            return self.handles.contains_key(&instance_id);
        }
        let Some(handle) = self.prepare(instance_id) else {
            return false;
        };
        if call == UnitScriptCall::Create {
            return true;
        }
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
        let handle = self.prepare(instance_id)?;
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
        self.pending_create.remove(&instance_id);
        if let Some(handle) = self.handles.remove(&instance_id) {
            let _ = self.registry.detach(handle);
        }
    }
}

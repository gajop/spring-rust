# Wasm re-entrancy: handoff (2026-09-24)

Discussion notes, not decisions. Nothing below is implemented except
"Current state".

## The problem

Many engine calls a module makes (callouts) synchronously call back into the
same module before they return (e.g. `create_unit` → `UnitCreated`, damage →
`UnitPreDamaged`). Lua doesn't care, since all its state is shared mutable
tables. In Rust the outer code usually still holds `&mut` to some state when the
nested callin wants the same state.

## Current state (engine + SDK, committed on `rust-wip`)

- **No aliasing, so no UB.** CUS state is guarded by a busy flag
  (`CusBusy` in `rust/crates/spring/src/cus/core_module.rs`).
  spring-addons `Resources` do runtime borrow checks
  (`rust/crates/spring-addons/REENTRANCY.md`).
- **Still traps at runtime:**
  1. A `Resources` borrow conflict panics and the module is unloaded. It
     only fails when that exact nesting happens.
  2. A busy CUS state gets "not available". Commit `cabe7edb44` turned this into
     deferral for calls whose result isn't read:
     - the guest returns `STATUS_BUSY` (2);
     - the engine queues the call and flushes it once none of the module's code
       is on the stack (`WasmCoreHost::Post`/`FlushPendingCusCalls`);
     - `with_cus_module_or_defer` does the same for the game's own calls.

     Calls that need an answer at once (`AimWeapon`, `QueryWeapon`,
     `HitByWeapon`, named calls with results) still get the engine's neutral
     default: a silent behaviour change compared with Lua.
  3. Deferred work runs after the outer code, not inline as in Lua, so ordering
     differs.

## Options discussed

- **A: defer the callouts** (a command buffer: world-changing callouts are
  queued and run after the callin returns).
  - Simple, and removes the problem entirely.
  - But results aren't available at once: "create, then configure the unit" and
    "damage, then read health" break.
  - Bevy does this (`Commands`, applied at sync points), but its `spawn`
    reserves entity ids up front. We'd need engine-side unit-id reservation, a
    synced API change.
- **B: context token** (recommended).
  - Callouts that can call back take `&mut Ctx`, and module state is reachable
    only through `ctx.state(|s| ...)`.
  - Holding a state borrow across such a call becomes a compile error, not a
    runtime panic.
  - Nested callins get their own `&mut Ctx`, so they always have the state.
    Immediate-result callins then work as in Lua (no neutral defaults).
  - Pure callouts stay free functions.
  - Costs: "copy out, call, write back" style; the classification (below) must
    be right.
- **Hybrid:** B, plus `ctx.defer(...)` / a typed command buffer where later is
  fine (today's `ctx.delay()`).
- **Other engines:**
  - **Bevy:** declared system access, deferred `Commands`, events as data, and
    observers/hooks get a restricted `DeferredWorld`.
  - **Fyrox:** a script is taken out of its node while running (the same idea
    as our busy flag), plus deferred script messages.

  Neither gives nested user code full access *and* immediate answers, which
  Spring's Lua semantics require. B is our equivalent of Bevy's compile-time
  access checking, while keeping Lua timing.

CUS under B: `UnitCtx` becomes the token, and `with_cus_module`, the busy flag
and deferral go away.

## Survey: callouts that can call back (4 read-only agents, 2026-09-24)

About 70–80 of ~1,400 functions; ~35 of them are RmlUi. Every getter and query
is pure. Orders from unsynced code (`UnitsCommands.GiveOrder*`) do NOT call
back: they go over the network, and AllowCommand/UnitCommand fire in a later sim
frame.

| Group | Callouts | Re-enters with |
|---|---|---|
| Unit lifecycle | CreateUnit, DestroyUnit, TransferUnit, SetUnitHealth (build/stun amounts), CreateUnitWreck/FeatureWreck, CreateFeature | UnitCreated/Finished/Destroyed/Taken/Given, FeatureCreated; unit scripts Create/Activate/Killed; AllowUnitTransfer, AllowFeatureCreation |
| Damage | AddUnitDamage, AddFeatureDamage, SpawnExplosion, SetProjectileCollision, UnitWeaponFire, SpawnSFX (fire/detonate), CUS EMIT_SFX | Unit/Feature/ShieldPreDamaged, script HitByWeapon, UnitDamaged/Stunned/Experience, death chain, recursive explosions (units far from the blast are damaged later) |
| Synced orders | GiveOrderToUnit (+ Array variants in SyncedCtrl), UnitFinishCommand, UnitDetach/FromAir, CUS DROP/ATTACH_UNIT, CUS SET_UNIT_VALUE (activation, standing orders) | AllowCommand, UnitCommand, CommandFallback, UnitCmdDone, UnitIdle; scripts Activate/Deactivate/StopMoving; synchronous command-AI SlowUpdate (depth not fully traced) |
| Transport | UnitAttach, UnitDetach*, CUS ATTACH/DROP_UNIT | AllowUnitTransport, UnitLoaded/Unloaded; scripts SetSFXOccupy, Start/StopMoving |
| Movement and state | SetUnitPosition/Physics, SetFeaturePosition/Physics, SetUnitLosState/Mask, Set/AddUnitExperience, SetUnitMetalExtraction, AddUnitSeismicPing; SetUnitMoveGoal/LandGoal, BuggerOff (aircraft only) | UnitMoved, FeatureMoved, LOS/radar enter/leave, UnitExperience, UnitSeismicPing; scripts ExtractionRateChanged, Activate |
| Script calls | CallUnitScript, CallCOBScript, CUS TURN/SPIN/STOP_SPIN (AnimFinished), Core `attach` (Create; detach of the previous script) | the module's own CUS code; COB incl. `lua_*` → LuaRules |
| Messages and actions | SendLuaRulesMsg, SendToUnsynced, SendLuaUIMsg/MenuMsg, SendSkirmishAIMessage, SendCommands, SetActiveCommand (custom), LoadCtrlPanelConfig, DebugInput.*, local markers, GetDefaultCommand, GetCurrentTooltip, sun/minimap/last-message setters | HandleLuaMsg/RecvLuaMsg/RecvFromSynced; through actions, effectively any unsynced callin; IsAbove, DefaultCommand, GetTooltip, Key/Mouse, MapDrawCmd, SunChanged, ... |
| RmlUi | anything that dispatches DOM events, loads/unloads/shows/hides documents, changes the DOM (SetInnerRml, remove/replace child, select/tab ops), sets `checked`/`value`, scrolls, submits, resizes; ContextUpdate; removing listeners, data models or contexts | the module's own listeners (one focus change = nested blur+focus), destroy callbacks; on shared contexts also other modules' and LuaUI's listeners |
| Callbacks by design | Gfx ActiveShader/ActiveFBO/RenderToTexture/CreateList/BeginEnd/PushPopMatrix/UnsafeState/RunQuery/DrawFuncAtUnit, Gfx Unit/Feature (luaDraw), terrain Set*HeightMapFunc, VFS UseArchive, SystemControl CallAsTeam, VFS DownloadArchive (DownloadQueued) | the module's own closure; DrawUnit/DrawFeature |

**Pure** (confirmed or nothing found):
- all getters and queries;
- MoveCtrl;
- rules params (including setters);
- config (observers run later);
- camera (CameraPositionChanged fires in `CCameraHandler::Update`);
- selection (there is no SelectionChanged callin);
- most unit/feature property setters;
- DestroyFeature and DeleteProjectile (deletion is deferred).

**Callbacks whose answer the engine uses immediately** (can't be deferred or
defaulted):
- UnitPreDamaged, FeaturePreDamaged, ShieldPreDamaged, script HitByWeapon,
  Explosion (draw or not);
- AllowCommand, AllowUnitTransfer, AllowUnitTransport, AllowFeatureCreation;
- IsAbove, DefaultCommand, GetTooltip/WorldTooltip, MapDrawCmd,
  Key/Mouse/TextInput (consumed?);
- script QueryWeapon/AimFromWeapon (weapon-piece refresh).

**Implication for B:** the synced set is small and falls into clear groups, so
marking callouts that call back is practical. UI modules are harder: RmlUi DOM
calls call back all over, and any deletion can run a destroy callback. The same
model works, but most UI code would need `&mut Ctx`. A few callouts call back
only under conditions (aircraft move goals, SetUnitHealth amounts); mark them
anyway.

## Bugs found by the survey

1. **(FIXED in 453bec06b2) AnimFinished lost while busy.** It uses `Invoke`/`CallCus`, not `Post`, so
   it isn't queued when the guest is busy. A TURN/SPIN/STOP_SPIN that ends an
   animation with waiters while the module holds its CUS state loses the
   wake-up. Fix: queue it like the other calls (it is result-less).
   (UnitScript.cpp:362/430/492 → NativeUnitScript.cpp:644)
2. **(FIXED in 453bec06b2) Possible use-after-free.** `EventListenerOnDetach` (RmlUi.cpp:3419) runs
   the destroy callback and `delete this` without removing the listener from
   the element.
3. **(FIXED) Synced handlers triggered directly.** `SendLuaRulesMsg`
   delivered synchronously, so unsynced code ran synced LuaRules `RecvLuaMsg`
   and Wasm `HandleLuaMsg` on the sending client only. `SendLuaUIMsg` reached
   only the local LuaUI and `SendLuaGaiaMsg` did nothing. All three now go over
   the network like Lua. A synced Wasm module's `SendLuaRulesMsg` (test fixtures
   report this way) runs on every client, so it stays local, from player -1.
   `SendToUnsynced` was fine: it is synced-only.
4. **(FIXED) Detach without a guard.** `WasmCoreHost::Detach` called the guest's
   `cus-detach` without the EnterCallback budget/re-entry guard; reached from
   `attach` on a unit that already has a CUS script, the guest found its state
   busy and dropped the detach, leaking the instance. It is now guarded and,
   while the module's code is on the stack, queued like the other calls.
5. **Not a bug: stopped collector.** `GetSyncedGCInfo(collect=1)` stops the
   LuaRules collector, but every Lua state keeps it stopped outside
   `CollectGarbage` anyway (LuaHandle.cpp), and Lua's version does the same.
6. **(FIXED) Dropped console events.** `GetConsoleBuffer` (Lua's too) reset
   InfoConsole `newLines`, which dropped pending `AddConsoleLine` events.

Checked and false: "weapon pieces not refreshed after Core attach". `StartCreate`
always queues, and the flush refreshes them.

## Suggested next steps

1. If pursuing B:
   - put the "can call back" classification into codegen as an annotation,
     next to environment masks, seeded from the table above;
   - prototype `Ctx`/`ctx.state` on one synced game (Hunted or TTLD) to judge
     ergonomics before touching the other six.

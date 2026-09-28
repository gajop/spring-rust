# Wasm re-entrancy: design options

Discussion notes, not decisions. Nothing below is implemented except "Today".
Last updated 2026-09-29.

## The problem

Many engine calls a module makes (callouts) call back into the same module
before they return. `create_unit` runs `UnitCreated`, damage runs
`UnitPreDamaged`, and a unit script's `explode` can damage other units. Lua
doesn't care: all its state is shared mutable tables. In Rust, the outer code
usually still holds `&mut` to some state when the nested callin wants the same
state.

The examples below use Hunted (`Hunted-wasm.sdd/wasm-src/rules-synced`),
whose code already works around this. The option sketches (A, B and the
hybrid) use made-up API names; only the "today" code is real.

## Today

- **No aliasing, so no undefined behaviour.** Game state is split into
  `Resource`s that are borrow-checked at runtime
  (`rust/crates/spring-addons/REENTRANCY.md`). CUS state is guarded by a busy
  flag (`CusBusy` in `rust/crates/spring/src/cus/core_module.rs`).
- **A conflict is only found when that exact nesting happens:**
  - A `Resource` conflict panics and the module is unloaded.
  - A busy CUS state makes calls that don't return anything wait until the
    module's code has returned (`with_cus_module_or_defer`, and the engine's
    `WasmCoreHost::Post` queue). Calls that need an answer at once
    (`AimWeapon`, `QueryWeapon`, `HitByWeapon`, named calls with results) get
    the engine's neutral default instead: a silent change from Lua.
  - Deferred work runs after the outer code, not inline as in Lua, so the
    order differs.

### Example: spawning a wave (Hunted `spawning.rs`)

```rust
pub fn update_wave(global: &Global, frame: i32) {
    let mut spawned_this_frame = 0;
    for burrow in 0..BURROWS.len() {
        let Some(batch) = global
            .waves
            .access_mut()
            .next_burrow_batch(burrow, frame, random::float)
        else {
            continue;
        };
        // No borrow is held here: create_unit re-enters UnitCreated.
        for _ in 0..batch {
            if spawn_rabbit(burrow) {
                spawned_this_frame += 1;
            }
        }
    }
    global.waves.access_mut().wave_spawned += spawned_this_frame;
}
```

The comment is doing the compiler's job. This version also compiles:

```rust
let mut waves = global.waves.access_mut();
for burrow in 0..BURROWS.len() {
    if let Some(batch) = waves.next_burrow_batch(burrow, frame, random::float) {
        for _ in 0..batch {
            spawn_rabbit(burrow); // UnitCreated runs in here
        }
    }
}
```

It works until some `UnitCreated` handler reads `waves`. From then on it
panics in the middle of a game and the module is unloaded.

### Example: effects that have to wait (Hunted `gadgets/weapon_handlers.rs`)

Nested callins that touch state someone may be holding use `ctx.delay`:

```rust
fn unit_created(&self, ctx: &AddonContext<'_, Global>, unit: i32, def: i32, ..) {
    if defs::kind_of(def) != Some(UnitKind::Shotgun) {
        return;
    }
    ctx.delay(move |global| {
        units::set_gunship_movement_attributes(unit);
        units::order(unit, spring::cmd::IDLEMODE, &[0.0]);
        global.weapons.access_mut().gun_id = unit;
    });
}
```

Until the outer callin returns, `gun_id` is still -1, although the gun exists.

### Example: a silent default (Hunted `cus/mod.rs`)

```rust
/// Where a lighthouse's script points its beam; `None` before its script
/// runs, or while the CUS state is busy.
pub fn lighthouse_beam(unit: i32) -> Option<(f32, f32)> {
    crate::with_cus_module(|host| {
        host.with_state(UnitId(unit), |script| match script {
            HuntedUnitScript::Lighthouse(state) => state.borrow().beam,
            _ => None,
        })
    })
    .flatten()
    .flatten()
}
```

"No beam" and "couldn't look" come back as the same answer. The engine's own
calls into scripts (`AimWeapon`, `HitByWeapon`, ...) behave the same way.

## Option A: queue the callouts

Callouts that can call back don't run at once. They go into a command buffer
that runs after the callin returns. Pure callouts (getters, queries, most
setters) still run at once. Nothing nests, so every callin gets plain
`&mut State`.

```rust
fn update_wave(ctx: &mut Ctx<State>, frame: i32) {
    let (state, commands) = ctx.split(); // &mut State, &mut Commands
    for burrow in 0..BURROWS.len() {
        let Some(batch) = state.waves.next_burrow_batch(burrow, frame, random::float) else {
            continue;
        };
        for _ in 0..batch {
            let (position, facing) = rabbit_spawn_point(burrow); // pure: runs now
            commands.create_unit(UnitKind::Rabbit.def(), position, TEAM, facing);
        }
        // Counts requests: whether a rabbit could be created is only known
        // after this callin has returned.
        state.waves.wave_spawned += batch;
    }
}
```

Callins become simple, and a callin that must answer at once always has the
state:

```rust
fn unit_pre_damaged(ctx: &mut Ctx<State>, event: &UnitPreDamagedEvent) -> Option<DamageResult> {
    let weapons = &ctx.state().weapons;
    // Made-up rule, to show an immediate answer that depends on state.
    (event.unit_id == weapons.gun_id && weapons.reloading)
        .then(|| DamageResult::unchanged(0.0))
}
```

What breaks is "do something, then use the result":

```rust
// Today (spawning.rs): create, then turn the new unit.
if let Some(unit) = units::create(kind, x, z, 0) {
    units::set_yaw(unit, random::float() * TAU);
}

// A: the unit doesn't exist yet. Either chain a follow-up...
commands
    .create_unit(kind.def(), position, TEAM, 0)
    .then(|unit, commands| commands.set_yaw(unit, yaw));

// ...or have the engine reserve the id up front, as Bevy's `spawn` does.
// That needs a new synced engine API.
let unit = commands.create_unit(kind.def(), position, TEAM, 0); // reserved UnitId
commands.set_yaw(unit, yaw);
```

It also changes timing compared with Lua, and ports would have to account
for it:

- `UnitCreated` runs after the whole callin. Code later in the same callin
  that counts rabbits in the roster doesn't see the new ones yet.
- "Damage, then read health" reads the health from before the damage.
- Callouts that take a closure and run it during the call (`Gfx RunQuery`,
  `CallAsTeam`, height-map functions) can't be queued. They would need to be
  given the state explicitly.

## Option B: context token (recommended)

Callouts that can call back become methods on `&mut Ctx`. Module state is
only reachable through `ctx.state()`, which also borrows `ctx`. The borrow
checker then refuses to keep state borrowed across a call that can call back.
The runtime conflict of today becomes a compile error. Pure callouts stay free
functions and can be used anywhere.

```rust
fn update_wave(ctx: &mut Ctx<State>, frame: i32) {
    for burrow in 0..BURROWS.len() {
        let Some(batch) = ctx.state().waves.next_burrow_batch(burrow, frame, random::float) else {
            continue;
        };
        let mut spawned = 0;
        for _ in 0..batch {
            if spawn_rabbit(ctx, burrow).is_some() {
                spawned += 1;
            }
        }
        ctx.state().waves.wave_spawned += spawned;
    }
}

fn spawn_rabbit(ctx: &mut Ctx<State>, burrow: usize) -> Option<UnitId> {
    let (position, facing) = rabbit_spawn_point(burrow); // pure: free functions
    // UnitCreated runs inside this call, with its own &mut Ctx and the state.
    let unit = ctx.create_unit(UnitKind::Rabbit.def(), position, TEAM, facing).ok()?;
    set_yaw(unit, random::float() * TAU); // pure setter, runs now
    Some(unit)
}
```

The mistake from "Today" no longer compiles:

```rust
let waves = &mut ctx.state().waves;
for _ in 0..batch {
    spawn_rabbit(ctx, burrow);
    // error[E0499]: cannot borrow `*ctx` as mutable more than once at a time
}
waves.wave_spawned += batch;
```

Callins take `&mut Ctx`. The nested `UnitCreated` from `spawn_rabbit` is
sound, because the outer code can't be using the state while it runs:

```rust
fn unit_created(&self, ctx: &mut Ctx<State>, unit: UnitId, def: UnitDefId, ..) {
    let Some(kind) = defs::kind_of(def) else { return };
    let position = get_unit_position(unit).unwrap_or_default(); // pure
    ctx.state().roster.track(unit, kind, position);
    if kind == UnitKind::Shotgun {
        // No delay needed: the gun is configured before create_unit returns.
        set_gunship_movement_attributes(unit);
        ctx.state().weapons.gun_id = unit;
    }
    ctx.attach_script(unit, def); // runs the script's Create, so it's a Ctx method
}
```

Answers needed at once behave as in Lua. `unit_pre_damaged` looks like it
does under A, but it also works when nested, for example inside the
`destroy_unit` that sets off a mine:

```rust
pub fn update_mines(ctx: &mut Ctx<State>) {
    // Update the fuses; copy out the mines that go off and the gun id.
    let (detonations, gun_id) = {
        let weapons = &mut ctx.state().weapons;
        let detonations = weapons.tick_fuses(); // Vec<Mine>
        (detonations, weapons.gun_id)
    };
    for mine in detonations {
        // UnitDestroyed and UnitPreDamaged for units hit by the blast run in
        // here, each with the state available.
        ctx.destroy_unit(mine.unit_id, true, gun_id);
        ctx.state().scares.push(Source::mine(mine.x, mine.z));
    }
}
```

### Unit scripts under B

A script's own state is part of the module state, so script methods can't
keep `&mut self` while calling something that can call back. They take a
`UnitCtx` with the same rules, where `ctx.script()` is the script's state:

```rust
// Today (cus/rabbit.rs): state behind Rc<RefCell<..>>.
fn killed(ctx: &UnitCtx, state: &Rc<RefCell<RabbitState>>) -> WreckLevel {
    let p = state.borrow().pieces;
    for piece in [p.body, p.bleg1, p.bleg2, p.fleg1, p.fleg2] {
        ctx.explode(piece, SfxFlags::EXPLODE); // debris can damage units: calls back
    }
    WreckLevel::One
}

// B: copy out, then call.
fn killed(ctx: &mut UnitCtx<Rabbit>, _recent_damage: f32, _max_health: f32) -> WreckLevel {
    let p = ctx.script().pieces;
    for piece in [p.body, p.bleg1, p.bleg2, p.fleg1, p.fleg2] {
        ctx.explode(piece, SfxFlags::EXPLODE);
    }
    WreckLevel::One
}
```

For tasks, `sleep` takes `&mut self`, so a state borrow can't be held across
an `.await` either:

```rust
async fn stop_moving_task(mut ctx: UnitCtx<Rabbit>) {
    ctx.set_signal_mask(SIG_MOVE);
    ctx.sleep(Duration::from_millis(1500)).await;
    ctx.script().moving = Some(false);
}
```

The game reads script state directly, and always gets an answer:

```rust
pub fn lighthouse_beam(ctx: &mut Ctx<State>, unit: UnitId) -> Option<(f32, f32)> {
    ctx.scripts().get::<Lighthouse>(unit)?.beam
}
```

The busy flag, `with_cus_module_or_defer` and the engine-side deferral are no
longer needed.

### UI under B

RmlUi calls that change the page run the module's own listeners at once
(for example one focus change runs a blur and then a focus). They would be
`Ctx` methods, and listeners would get `&mut Ctx`:

```rust
fn on_click(ctx: &mut Ctx<Hud>, event: &RmlEvent) {
    let label = format!("Ammo: {}", ctx.state().ammo); // copy out
    ctx.rml().set_inner_rml(ctx.state().ammo_element, &label);
    ctx.rml().focus(ctx.state().fire_button); // runs blur/focus listeners here
}
```

Almost all UI code would take `&mut Ctx`.

### Costs

- Code is written as "copy out, call, write back", as in `update_mines`.
- The list of callouts that call back must be right. If one is marked pure but
  calls back, the nested callin finds the state in use. A runtime check stays
  in place and panics with that callout's name.
- `Gadget`, `Widget` and `UnitScript` signatures change. All eight game ports
  need updating.

## Hybrid: B plus `ctx.defer`

B, plus an explicit queue for work that is meant to happen later. It replaces
today's `ctx.delay`, but games only use it when they want "later", not to
avoid a borrow conflict:

```rust
fn game_frame(&self, ctx: &mut Ctx<State>, frame: i32) {
    if ctx.state().session.restart_in_progress {
        // Remove the old round's units once this frame's callins are done.
        ctx.defer(|ctx| restart::destroy_round_units(ctx));
    }
}
```

## Comparison

| | Today | A: queue callouts | B: context token | Hybrid |
|---|---|---|---|---|
| Nested state conflict | Runtime panic, module unloaded | Can't happen | Compile error | Compile error |
| Answer needed at once (`UnitPreDamaged`, `AimWeapon`, ...) | Neutral default when busy | Always correct | Always correct | Always correct |
| Order compared with Lua | Differs where deferred | Differs for every queued callout | Same | Same, except explicit `defer` |
| Use a result at once ("create, then configure") | Works | Needs reserved ids or `.then` | Works | Works |
| Style | Borrow, release, call | Plain `&mut State`, results later | Copy out, call, write back | As B |
| Engine changes | None | Unit-id reservation (synced API) | None | None |
| Game changes | None | Every world-changing call | Every callin signature, scripts lose `&mut self` | As B |

## Survey: callouts that can call back (2026-09-24)

Both A and B need this list: A queues these callouts, B makes them `Ctx`
methods.

About 70–80 of ~1,400 functions call back, and ~35 of those are RmlUi. Every
getter and query is pure. Orders from unsynced code (`UnitsCommands.GiveOrder*`)
don't call back: they go over the network, and `AllowCommand`/`UnitCommand`
fire in a later sim frame. The same goes for `SendLuaRulesMsg`, `SendLuaUIMsg`
and `SendLuaGaiaMsg` from unsynced code.

| Group | Callouts | Calls back with |
|---|---|---|
| Unit lifecycle | CreateUnit, DestroyUnit, TransferUnit, SetUnitHealth (build/stun amounts), CreateUnitWreck/FeatureWreck, CreateFeature | UnitCreated/Finished/Destroyed/Taken/Given, FeatureCreated; unit scripts Create/Activate/Killed; AllowUnitTransfer, AllowFeatureCreation |
| Damage | AddUnitDamage, AddFeatureDamage, SpawnExplosion, SetProjectileCollision, UnitWeaponFire, SpawnSFX (fire/detonate), CUS EMIT_SFX | Unit/Feature/ShieldPreDamaged, script HitByWeapon, UnitDamaged/Stunned/Experience, death chain, recursive explosions (units far from the blast are damaged later) |
| Synced orders | GiveOrderToUnit (+ Array variants in SyncedCtrl), UnitFinishCommand, UnitDetach/FromAir, CUS DROP/ATTACH_UNIT, CUS SET_UNIT_VALUE (activation, standing orders) | AllowCommand, UnitCommand, CommandFallback, UnitCmdDone, UnitIdle; scripts Activate/Deactivate/StopMoving; synchronous command-AI SlowUpdate (depth not fully traced) |
| Transport | UnitAttach, UnitDetach*, CUS ATTACH/DROP_UNIT | AllowUnitTransport, UnitLoaded/Unloaded; scripts SetSFXOccupy, Start/StopMoving |
| Movement and state | SetUnitPosition/Physics, SetFeaturePosition/Physics, SetUnitLosState/Mask, Set/AddUnitExperience, SetUnitMetalExtraction, AddUnitSeismicPing; SetUnitMoveGoal/LandGoal, BuggerOff (aircraft only) | UnitMoved, FeatureMoved, LOS/radar enter/leave, UnitExperience, UnitSeismicPing; scripts ExtractionRateChanged, Activate |
| Script calls | CallUnitScript, CallCOBScript, CUS TURN/SPIN/STOP_SPIN (AnimFinished), Core `attach` (Create; detach of the previous script) | the module's own CUS code; COB incl. `lua_*` → LuaRules |
| Messages and actions | SendLuaRulesMsg from synced modules, SendToUnsynced, SendLuaMenuMsg, SendSkirmishAIMessage, SendCommands, SetActiveCommand (custom), LoadCtrlPanelConfig, DebugInput.*, local markers, GetDefaultCommand, GetCurrentTooltip, sun/minimap/last-message setters | HandleLuaMsg/RecvLuaMsg/RecvFromSynced; through actions, effectively any unsynced callin; IsAbove, DefaultCommand, GetTooltip, Key/Mouse, MapDrawCmd, SunChanged, ... |
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

**Callbacks whose answer the engine uses at once** (can't be deferred or
defaulted without changing behaviour):
- UnitPreDamaged, FeaturePreDamaged, ShieldPreDamaged, script HitByWeapon,
  Explosion (draw or not);
- AllowCommand, AllowUnitTransfer, AllowUnitTransport, AllowFeatureCreation;
- IsAbove, DefaultCommand, GetTooltip/WorldTooltip, MapDrawCmd,
  Key/Mouse/TextInput (consumed?);
- script QueryWeapon/AimFromWeapon (weapon-piece refresh).

A few callouts call back only under conditions (aircraft move goals,
SetUnitHealth amounts). Mark them anyway.

The engine bugs this survey found are fixed (453bec06b2, e2262a19fe).

## Other engines

- **Bevy:** systems declare what they access, world changes go through
  deferred `Commands` (like A, with ids reserved up front), and
  observers/hooks get a restricted `DeferredWorld`.
- **Fyrox:** a script is taken out of its node while it runs (like today's
  busy flag), plus deferred script messages.

Neither lets nested user code have full access *and* give answers at once,
which Spring's Lua semantics need. B is our version of Bevy's compile-time
access checks, while keeping Lua's timing.

## Open decisions (for B)

1. A callout marked pure that turns out to call back: panic with its name
   (suggested), or fall back to today's runtime checks?
2. One state per module (suggested to start), or several `ctx.state::<T>()`,
   which brings runtime borrow checks back?
3. UI modules: move to `&mut Ctx` now, or keep today's model for now?
4. Which game to try it on first: TTLD (suggested: CUS and ordinary callins)
   or Hunted (the examples above).

Before either: put the "can call back" list into codegen as an annotation,
next to the environment masks, seeded from the table above.

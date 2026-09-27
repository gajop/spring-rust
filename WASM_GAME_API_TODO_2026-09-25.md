# WASM game API gaps found in round 6 (2026-09-25)

Found while the seven SpringCabal WASM ports dropped their workarounds for the
fixes in `6994406a6d`..`e2262a19fe`. Every fix from the previous TODO checked
out in the games. Delete items as they are done.

## 1. Rules modules have no pre-game Initialize

A Lua gadget's `Initialize` runs when LuaRules loads, before the first frame.
A spring-addons rules module (and a bare Wasm rules module) runs nothing until
its first callin, so a gadget's `init` only runs then. On a map without
features that is the frame-0 `GameFrame`, after the Lua gadgets' frame-0
`GameFrame`.

It used to go unnoticed because a UI module's startup `SendLuaRulesMsg` was
delivered locally before the first frame, which started the rules module
early. Since `e2262a19fe` that message arrives over the network on frame 0,
and Flove's test scenario (a Lua gadget) read a rules param before the Rust
rules had set it (fixed in the game by starting the scenario at frame 1).

Wanted: an `Initialize` callin delivered to Wasm modules when they load, like
Lua's, and spring-addons running every addon's `init` from it.

## 2. No named constants for the query allegiance

`get_units_in_cylinder` / `_in_rectangle` / `_in_sphere` take Lua's
allegiance argument: a team id, or -1 all units, -2 my allies, -3 my enemies,
-4 ... (see `LuaSyncedRead::GetUnitsIn*`). Games write `-1` or define their own
`ALL_UNITS`. Wanted: `spring::allegiance::{ALL, ALLIED, ENEMY, ...}` (or an
enum in the typed wrappers).

## 3. Log noise: "Could not finalize groundFX texture atlas"

Every headless verification run of every game logs
`[f=-000001] Error: Could not finalize groundFX texture atlas. Use fewer/smaller textures.`
It is the only engine error most suites see, so the suites have to allow one
error. If headless can't build that atlas, it shouldn't report an error.

## 4. `WeaponDefInfo` has no `projectiles` or `sprayAngle`

Hunted's shotgun fires `projectiles` pellets spread by `sprayAngle`, as Lua's `shotgun.lua` reads
them from `WeaponDefs`. The SDK's `WeaponDefInfo` has neither, so the port hard-codes 10 pellets and
`2000 * PI / 0xafff`. Wanted: both fields, with `sprayAngle` as the engine stores it (the sine,
`AccuracyToSin`) or as the def's raw value, documented either way.

## 5. A unit-script attach gadget in spring-addons

Five SpringCabal ports (Area17, Flove, MyCube, Parts Needed, TTLD) each carry the
same rules gadget around their `CusHost`, and Gravitas and Hunted do the same work
inside other gadgets:

```rust
fn init(&self, _ctx)            { with_cus_module_or_defer(|host| host.attach_existing()); }
fn game_frame(&self, _ctx, f)   { with_cus_module_or_defer(move |host| host.game_frame(f)); }
fn unit_created(&self, _ctx, unit, def, _team, _builder) {
    with_cus_module_or_defer(move |host| host.unit_created(UnitId(unit), def));
}
```

`with_cus_module_or_defer` comes from `export_core_cus!`, so the gadget can't live
in a game-side crate without knowing the module's type. Wanted: spring-addons (or
`export_core_cus!`) provides it, e.g. `CusHostGadget::new(with_cus_module_or_defer)`
or a macro argument that registers it, with:

- `unit_created` attaching the new unit's script (deferred while the module is busy);
- `game_frame` calling `CusHost::game_frame`, so sleeping tasks wake where Lua wakes
  them (before the unit update), with a note that it should run after the gadgets
  that create or order units that frame, as Lua's `unit_script.lua` runs late;
- `init` calling `attach_existing()`, optional: on a normal start no game has units
  at the rules' first callin except Parts Needed (the map's featureplacer units),
  and after a rules reload every unit needs it;
- `unit_destroyed` left to the host, as now.

Flove's copy also sets a `createdFrame` rules param in `unit_created`; that part
stays in the game.

## 6. `ResultExt` has no lazily built context

`log_err(&format!("... {unit}"))` formats the context on every call, even when the
call succeeds. Flove and Gravitas keep their own `check(result, || format!(...))`
for calls made every frame or for every unit (publishing rules params, creating
units). Wanted: `log_err_with(|| context)` / `log_err_once_with`.

## 7. `Wait*` completions resume one frame later than Lua

In Lua, `WaitForMove`/`WaitForTurn` threads resume inside the engine's `AnimFinished` callin, in
the frame's animation tick, so a following `Sleep(n)` counts from that frame.
`CusScheduler::wake_animation` (`rust/crates/spring/src/cus/scheduler.rs`) only queues the task,
and `tick` drains each frame once ("wakes that arrive after this pass are deferred until the next
frame"), so the task resumes on the next frame. Found in Hunted's rabbit hop (round 7c):
`Sleep(300)` + `WaitForMove(center)` over a 10-frame move, then `Sleep(100)`; Lua hops at frames
633/666/699, the port at 633/667/701, 1 frame lost per blocking wait. Evidence:
`~/worktrees/golden-diffs-2026-09-24/hunted/r7c-rabbit-controller/`. Poll animation-woken tasks
in the `AnimFinished` dispatch (or run a second drain after the animation tick) to match. This
moves the anim goldens of any game whose scripts block on a `Wait*`.

## 8. No typed getter for a game rules param

The SDK has `rules_params::get_game_rules_param(p0: i32, p1: i32) -> i32` (the raw Core ABI
entry, undocumented) and `get_game_rules_params`, but nothing like Lua's
`Spring.GetGameRulesParam(name)` returning a number or string. Unit rules params have typed getters,
so games publish game-wide state on a carrier unit instead (Flove's spire, MyCube's coal burner for
its test handshake). Wanted: `get_game_rules_param(name) -> Result<Option<RulesParamValue>>`, like
the unit version.

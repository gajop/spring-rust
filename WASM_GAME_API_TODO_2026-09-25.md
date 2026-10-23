# WASM game API gaps found in round 6 (2026-09-25)

Found while the seven SpringCabal WASM ports dropped their workarounds for the
fixes in `6994406a6d`..`e2262a19fe`. Every fix from the previous TODO checked
out in the games. Delete items as they are done. Done since: allegiance constants
(`spring::allegiance`; the Wasm queries now honour my/ally/enemy, which they had treated as all
units), the headless groundFX atlas error, `WeaponDefInfo` projectiles and spray (projectile
params), `log_err_with` / `log_err_once_with`.

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

## 8. No string getter for a game rules param

`rules_params::get_game_rules_param_f32(name)` reads a number; nothing reads a string value like
Lua's `Spring.GetGameRulesParam(name)`. Unit rules params have typed getters,
so games publish game-wide state on a carrier unit instead (Flove's spire, MyCube's coal burner for
its test handshake). Wanted: `get_game_rules_param(name) -> Result<Option<RulesParamValue>>`, like
the unit version.

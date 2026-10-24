# WASM game API gaps found in round 6 (2026-09-25)

Found while the seven SpringCabal WASM ports dropped their workarounds for the
fixes in `6994406a6d`..`e2262a19fe`. Every fix from the previous TODO checked
out in the games. Delete items as they are done. Done since: allegiance constants
(`spring::allegiance`; the Wasm queries now honour my/ally/enemy, which they had treated as all
units), the headless groundFX atlas error, `WeaponDefInfo` projectiles and spray (projectile
params), `log_err_with` / `log_err_once_with`; a pre-game `Initialize` (item 1: `Module::start`
runs when the module loads, and spring-addons runs every addon's `init` from it); a unit-script
attach gadget (item 5: `spring_addons::CusHostGadget::new(with_cus_host)` for a `cus: CusHost<S>`;
`with_cus_host` comes from `export_rules_gadgets!`; ports on the legacy `spring` crate's
`export_core_cus!` move to `spring-sdk`'s `CusHost` to use it).

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

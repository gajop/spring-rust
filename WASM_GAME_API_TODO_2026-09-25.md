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
`export_core_cus!` move to `spring-sdk`'s `CusHost` to use it); `Wait*` completions resuming a
frame late (item 7, already fixed by `ca1985b144`, in releases from `2026.10.15`: animation waiters
resume in the animation-finished dispatch, so a following sleep counts from that frame).

## 8. No string getter for a game rules param

`rules_params::get_game_rules_param_f32(name)` reads a number; nothing reads a string value like
Lua's `Spring.GetGameRulesParam(name)`. Unit rules params have typed getters,
so games publish game-wide state on a carrier unit instead (Flove's spire, MyCube's coal burner for
its test handshake). Wanted: `get_game_rules_param(name) -> Result<Option<RulesParamValue>>`, like
the unit version.

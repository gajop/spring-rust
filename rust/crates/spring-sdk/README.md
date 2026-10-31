# spring-sdk

One scoped Rust API for native and Core-Wasm engine modules. This opt-in crate
keeps existing SDK consumers unchanged. See the
[design and compatibility notes](../../../rts/wasm/docs/recoil_rust_sdk_unification.md).

Games normally use `spring-addons` on top of this crate. Its module registration
macros generate the exports; individual widgets and gadgets only implement their
callback methods. The example below shows direct module registration, as used by
menus and SpringBoard.

```rust
use spring_sdk::{Menu, Spring, Result, module::Module};

struct App;
impl Module<Menu> for App {
    fn initialize(spring: &mut Spring<'_, Menu>) -> Result<Self> {
        let _ = spring.config().get_config_string("PlayerName", Some("Player"))?;
        Ok(Self)
    }
}
fn update(_: &App, _: &mut Spring<'_, Menu>, _delta_seconds: f32) -> Result<()> {
    Ok(())
}
spring_sdk::export_module!(APP, Menu, App);
spring_sdk::export_update!(APP, update);
```

Use a `cdylib` crate. Build normally for native, or with
`--target wasm32-unknown-unknown` for Wasm. Native builds require unwinding;
native loading supports Menu and in-game All modules. Variable-size callins also
require one `export_callin_scratch!(BYTES)` declaration on Wasm.

Handlers receive shared module state and a fresh `&mut Spring<'_, E>`.
Callin handlers take ordinary arguments: scalars/vectors by value, strings as
`&str`, lists as slices and structured commands by shared reference. Returns
use `()`, a single value such as `bool`/`String`, or a tuple for multiple values.
ABI query/result envelopes, dummy parameters and redundant lengths stay in
adapters.

`Result` reports whether the callback completed. A normal veto is `Ok(false)`.
`Err` rejects initialization or stops further use of a failed module.
`spring.units().get_all_units()` is the short form for unit queries.

Release state borrows before reentrant callouts. Send owned snapshots to
workers; Spring references cannot cross threads. Native module state and
retained captures must be Send for loading-thread handoff. Rml binding
identifiers can move with that state, but every operation needs Spring.
The SDK does not queue immediate calls automatically.

Long-lived models can store `ModuleRef<E>` and obtain Spring during their
module's active callbacks. They cannot use it to call the engine from workers.
`All` gives native tools both simulation and UI access; SpringBoard needs no
separate tools API. Environment restrictions and Spring lifetimes are checked
at compile time. Loaded DLL versions and incoming C callbacks still need
runtime checks, because the host is outside Rust's type system.

`rml::DataModel` and its typed bindings work on both targets. Every binding
operation takes a fresh Spring reference. Call `close` / `unbind` explicitly;
engine cleanup handles resources still owned by an unloaded module.

Run the [same-source live menu proof](../../../test/wasm_api/data/scoped_sdk_menu/README.md)
for a working integration, including the optional native/Wasm All game test.

Pure workers use `export_jobs!(APP, { DOUBLE => ("double", double) })` and
`spring.jobs().run_next_tick(DOUBLE, &input)`. Implement `Module::job_result` to receive
ordered results with Spring. Export Update for local modules or GameFrame for
synced Wasm. Native uses a bounded thread pool; Wasm uses fresh private worker
instances. Workers take owned input snapshots and must not access mutable
globals or engine functions. See the design notes for limits and delivery timing.
Nested native Update calls run immediately; only the outer call advances jobs.

`spring.columns()` reads or writes one property of many units in one engine call:
`get_units_f32(UnitColumn::Velocity, &ids, ColumnArg::None, &mut out, Some(&mut found))`
fills every unit's x, then every y, then every z. Each entry reads as the per-unit call reads
it; refused entries are 0 and marked in `found`. `set_units_f32` (synced) writes MoveCtrl
velocity or direction; `get_units_rules_param_f32` reads a float rules parameter. See
[the property list and ABI](../../../rts/wasm/docs/columns_2026-10-31.md).

The SDK depends on private `spring-sdk-internal` transport code and native FFI
bindings. It no longer depends on the legacy public `spring` or `spring-native`
APIs. The old `spring` crate temporarily re-exports the internal implementation
so existing consumers keep building. Renaming the unified public crate to
`spring` can follow once those consumers have migrated.

Command callins receive `&CallinCommand<'_>` with `params: &[f32]`.
Use `command.to_owned()` when retaining a command after its callback. Addons'
command events already use slices and need no caller changes.

Native engine calls do not take a mutex or clone a module `Arc` per API accessor.
Callback entry still serializes module access. `ModuleRef` resolution checks the
current thread's active callbacks and reuses their existing entry guard.

Matched native benchmarks now put scalar queries and numeric mutations at
legacy speed. Vectors still add about 2 ns. Empty callback entry fell from
roughly 170–210 ns to 24–31 ns; legacy remains 2–6 ns. Cold entry and callbacks
that copy strings/commands retain overhead. Small command-list outputs own
nested parameter copies; legacy outputs retain engine pointers.
See [results, raw samples and limits](../../../test/native_api_parity/CORE_WASM_BENCHMARKS.md#native-follow-up-2026-10-18).

## Port status

All consumer edits are in dedicated worktrees. The SDK remains separate from
`rust-wip`; it has not been merged or published.

| Consumer | Change |
| --- | --- |
| SpringBoard | Native `All`; stored engine access becomes `ModuleRef<All>`. Terrain, units and UI remain available together. |
| Seven Cabal games | Wasm modules use scoped Spring calls; their synced/unsynced split remains. Shared helpers, addons and CUS are ported. |
| Cabal menu | One Rust implementation builds as native or Wasm. |
| Other downstream game/menu | Three Wasm game modules, rendering, custom CUS and HUD ported; menu builds as native or Wasm. |

SpringBoard passes 110 Rust tests and its full 85-case integration suite plus
eight smoke checks on Linux; the Windows DLL passes the integration and smoke
suites under Wine. All seven Cabal gameplay suites pass on Linux and Wine.
The other game's full Linux HUD checks, both menus' launch/return checks,
and two 901-frame Linux/Wine matches with real player orders and shared HUD
pause/speed controls pass.

SDK tests, addon tests and the same-source native/Wasm CUS/job fixture also
pass on Linux and Wine. Visual suites still have failures reproduced before
the port; no golden images or tolerances have been changed. Native Windows
hardware and discovery/rooms/reconnect UI remain separate work.

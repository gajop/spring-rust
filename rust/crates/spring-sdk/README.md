# spring-sdk

One scoped Rust API for native and Core-Wasm engine modules. This opt-in crate
keeps existing SDK consumers unchanged. See the
[design and compatibility notes](../../../rts/wasm/docs/recoil_rust_sdk_unification.md).

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

Release state borrows before reentrant callouts. Send owned snapshots to
workers; Spring references cannot cross threads. Native module state and
retained captures must be Send for loading-thread handoff. Rml binding
identifiers can move with that state, but every operation needs Spring.
The SDK does not queue immediate calls automatically.

`rml::DataModel` and its typed bindings work on both targets. Every binding
operation takes a fresh Spring reference. Call `close` / `unbind` explicitly;
engine cleanup handles resources still owned by an unloaded module.

Run the [same-source live menu proof](../../../test/wasm_api/data/scoped_sdk_menu/README.md)
for a working integration, including the optional native/Wasm All game test.

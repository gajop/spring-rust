# spring-sdk

One scoped Rust API for native and Core-Wasm engine modules. This opt-in crate
keeps existing SDK consumers unchanged. See the
[design and compatibility notes](../../../rts/wasm/docs/recoil_rust_sdk_unification.md).

```rust
use spring_sdk::{Menu, Spring, Result, module::Module, types::callins::*};

struct App;
impl Module<Menu> for App {
    fn initialize(spring: &mut Spring<'_, Menu>) -> Result<Self> {
        let _ = spring.config().get_config_string("PlayerName", Some("Player"))?;
        Ok(Self)
    }
}
fn update(_: &App, _: &mut Spring<'_, Menu>, _: &UpdateQuery) -> Result<UpdateResult> {
    Ok(UpdateResult::default())
}
spring_sdk::export_module!(APP, Menu, App);
spring_sdk::export_update!(APP, update);
```

Use a `cdylib` crate. Build normally for native, or with
`--target wasm32-unknown-unknown` for Wasm. Native builds require unwinding;
the safe native factory currently supports Menu. Variable-size callins also
require one `export_callin_scratch!(BYTES)` declaration on Wasm.

Handlers receive shared module state and a fresh `&mut Spring<'_, E>`.
Release state borrows before reentrant callouts. Send owned snapshots to
workers; Spring references and Rml binding identities cannot cross threads.
The SDK does not queue immediate calls automatically.

`rml::DataModel` and its typed bindings work on both targets. Every binding
operation takes a fresh Spring reference. Call `close` / `unbind` explicitly;
engine cleanup handles resources still owned by an unloaded module.

Run the [same-source live menu proof](../../../test/wasm_api/data/scoped_sdk_menu/README.md)
for a working integration rather than converting an existing menu first.

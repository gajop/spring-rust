# spring-skirmish-ai

Native Rust Skirmish AIs for Spring/Recoil's existing **C AI interface**. No engine-side Rust runtime, game dependency, C++ shim or libclang is required by consumers.

```rust
use spring_skirmish_ai::{Context, Event, Result, SkirmishAi};

struct MyAi;
impl SkirmishAi for MyAi {
    fn new(context: &Context<'_>) -> Result<Self> {
        context.log("My AI loaded")?;
        Ok(Self)
    }
    fn handle(&mut self, context: &Context<'_>, event: Event<'_>) -> Result<()> {
        if let Event::Update { frame } = event {
            if frame % 30 == 0 {
                let own = context.owned_units()?;
                let enemies = context.visible_enemies()?;
                // Snapshot permitted state here; plan without engine access.
                context.log(&format!("own={} visible={}", own.len(), enemies.len()))?;
            }
        }
        Ok(())
    }
}
spring_skirmish_ai::export_ai!(MyAi);
```

Your package needs `[lib] crate-type = ["cdylib"]`. Use **panic = "unwind"**, including release builds. `panic = "abort"` cannot protect the host engine from an AI panic. Build the complete example:

```
cargo build --manifest-path examples/observer/Cargo.toml --release
```

Install its library as `AI/Skirmish/RustObserver/0.1/libSkirmishAI.so`, alongside the example's `AIInfo.lua`. The engine needs its normal `AI/Interfaces/C/0.1` installation. Add an `[AI0]` start-script section with `ShortName=RustObserver`, `Version=0.1`, `Team=0`, `Host=0`. Different version directories can hold independent AI binaries. Do not overwrite a baseline binary while comparing versions.

## Contract

- One owned state value per AI ID, including several teams loading the same library. Initialization and updates may run on different OS threads.
- `Context` is borrowed for one dispatch and is not Send/Sync. Copy snapshots for planner threads; never call engine callbacks from them.
- All 28 current event topics are decoded. Nested event strings and slices are borrowed; copy anything retained. Future event topics become `Event::Unknown`.
- Constructors, handlers and destructors cannot unwind through the exported boundary. A handler error or panic quarantines that instance; reentrant/concurrent dispatch returns an error rather than deadlocking. The engine decides how to react to a nonzero return.
- Safe observations use normal callbacks; no safe cheat-enabling API. Friendly enumeration filters allies out of owned units. Exact enemy snapshots should only use the direct-LOS enumeration. Enemy disappearance is not proof of death.
- Safe orders enforce own-unit ownership and finite parameter values. `move_to`, `fight`, `attack`, `stop`, and `custom_order` submit ordinary engine unit commands. These enter the network/replay command stream; submission success does **not** prove that the game accepted the action. Implement game acknowledgements when needed.
- Replay playback does not need the native AI. The game must give custom orders deterministic synced semantics; calling LuaRules directly is not a substitute for recorded commands. Engine/game compatibility still matters.
- AI save/load persistence is the implementer's responsibility (`Event::Save`, `Event::Load`, appropriate `loadSupported` metadata). The SDK does not serialize arbitrary planner state.
- `raw()` exposes the checked-in unsafe ABI for APIs not wrapped yet. You retain responsibility for ownership, fog of war, buffer sizes and engine-thread use when invoking it.

## Compatibility and maintenance

Prototype tested on Linux x86-64 with the vendored callback ABI. **C interface version 0.1 alone does not prove callback-table compatibility with every Spring/Recoil release.** Pin an engine/header revision and validate before distributing a binary to other engines. The unsafe initialization contract requires a complete callback table matching `spring-skirmish-ai-sys`; table length cannot be discovered from a raw pointer. Windows `extern "system"` calling conventions are represented but Windows and 32-bit builds are not validated.

The sys package contains vendored engine headers and generated bindings; this façade depends only on that package. Publish sys first, then the façade. Both are currently GPL-2.0-or-later, matching the vendored engine interface; no publication occurs automatically.

Maintainers regenerate bindings using `rust/tools/skirmish-ai-bindings` in the engine checkout. Consumer builds do not run bindgen. Verify tests, Clippy, the C ABI probe, the standalone example, a live load, and replay without the AI installed when updating the ABI.

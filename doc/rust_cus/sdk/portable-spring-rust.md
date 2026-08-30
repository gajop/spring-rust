# Portable Spring Rust SDK

CUS depends on a broader Rust API goal that is valuable independently:

> Game-facing Spring Rust crates should expose the same API whether the final
> game module is Core Wasm or native.

A game should be able to write:

```rust
use spring::units::UnitId;
use spring::weapons::WeaponId;
use spring::cus::{UnitCtx, UnitScript};
```

without scattering backend conditionals throughout gameplay code.

## Desired layering

```text
                         game crates
                             |
                             v
                     game-facing `spring`
                             |
                 +-----------+-----------+
                 |                       |
            Wasm backend             native backend
                 |                       |
        Core-Wasm imports/ABI     NativeInterface ABI
                 |                       |
                 +-----------+-----------+
                             |
                           engine
```

The public types and behavior should match where the underlying engine
capability is the same.

## Relationship to current crates

`rust-wip` already contains native-oriented generation and wrapper crates such
as:

- `spring-native-sys`;
- `spring-native-codegen`;
- `spring-native`;
- the higher-level `spring` crate.

CUS should push the architecture toward treating native and Wasm bindings as
backend implementations below a common game-facing layer, rather than making
`spring-native` the API games must program against directly.

The exact final crate split is not fixed by this proposal.

## Why this matters for games

A game can build reusable crates normally:

```text
my-game-core
my-targeting
my-status-effects
my-animation-data
my-unit-scripts
```

Those crates can be shared between NativeCUS and WasmCUS builds and can be used
by non-CUS synced game logic as well.

## Environment capability remains explicit

"Same API" does not mean every environment can see every engine function.
RulesSynced, UI, menu, editor-like use cases, and other environments may have
different capabilities.

The environment/capability model should remain a compile-time concern where
possible. Backend selection (Wasm versus native) should not itself force games
to use a different API when the capability is otherwise identical.

## Native is first-class

NativeCUS is not a fallback test mode. It is a supported backend for the same
Rust source and is especially useful for:

- editors and standalone tools;
- single-player games that do not need a restricted Wasm environment;
- debugging/profiling;
- reusable Rust crates that should not care how Spring is embedded.

# Recoil Wasm Execution Environments

## Purpose

Define the Spring role of each Wasm instance.

This is separate from sandboxing. A Wasm module can have powerful Spring simulation APIs while still having no filesystem/network/process access.

## In-game environments

The initial model follows the existing Lua roles.

| Wasm environment | Synced | Baseline |
| --- | --- | --- |
| Rules synced | Yes | LuaRules synced half |
| Rules unsynced | No | LuaRules unsynced half |
| Gaia synced | Yes | LuaGaia synced half |
| Gaia unsynced | No | LuaGaia unsynced half |
| UI | No | LuaUI, with local-player visibility semantics |

The first implementation stage covered the four Rules/Gaia environments.
Phase 8 then enables the UI environment with its role-specific restricted-read
and LOS semantics before the Phase 9 parity gate.

Out of scope:

- LuaMenu;
- LuaIntro;
- LuaParser/definition parsing.

## Why synced and unsynced are separate instances

Native modules currently combine synced and unsynced behavior in one native binary. Wasm should not copy that state model.

For Wasm:

- synced and unsynced environments use separate stores/memories;
- mutable guest state is not shared between them;
- unsynced local state cannot influence a future synced decision;
- imports/exports are statically shaped for the environment.

Communication between synced and unsynced sides, when needed, should happen through explicit engine-mediated channels with defined deterministic semantics.

This is both simpler and safer for lockstep execution than dynamically checking every call against a mixed synced/unsynced module.

## Spring API availability

Environment determines which Spring interfaces are present.

Examples conceptually:

- synced Rules/Gaia: synced simulation read/control and synced callins;
- unsynced Rules/Gaia: unsynced/local/draw APIs and relevant read APIs;
- UI: widget/UI APIs with LuaUI-equivalent visibility semantics.

The exact matrix should be derived from the existing Lua loaders rather than invented from scratch.

For map-shipped game logic, LuaGaia is the precedent: map content already has a powerful synced gadget role and an unsynced draw role. Wasm should preserve that Spring-level functionality while removing generic host-OS authority.

## LuaUI

LuaUI is the one in-game role whose read semantics differ materially from Rules/Gaia.

Do not implement the UI environment by reusing full-read gadget adapters.

Before enabling it:

1. reuse the engine/Lua visibility helpers for ally/visible/typed distinctions;
2. preserve degraded/fuzzed values where LuaUI/Lua read semantics require them;
3. extend the parity fixture to multiple ally teams and LOS/radar states;
4. pass Lua ↔ Wasm parity for those cases.

The UI world is now runtime-enabled through LuaUI/wasm/manifest.txt.
Its host dispatch uses an owned, visibility-filtered payload and a LuaUI
visibility context; the generated UI parity component exercises the widget
surface. The final scripted runtime result remains a Phase 9 verification
gate, rather than an original-design deferral.

## Module discovery

The engine needs a general Wasm module discovery mechanism rather than the current one-native-module shape.

It must determine:

- which game/map/archive declares a Wasm module;
- module bytes and content hash;
- target execution environment;
- module order;
- lifecycle/reload behavior.

For the first implementation, reuse existing game/map loading conventions where possible:

- Rules-style module from the game side;
- Gaia-style module from the map side.

Later, multiple modules may be declared by several participating archives.

## Multiple modules

Each Wasm module instance is independent.

For synced environments, peers must agree on:

- the exact Wasm module bytes/hash;
- the execution environment;
- deterministic instance order;
- runtime/config version.

Callin fan-out uses that order.

## Callin applicability

The canonical callin inventory declares the environments that receive each callin.

This replaces a large amount of ad hoc runtime checking.

Boolean/result callins also need explicit aggregation rules when multiple modules receive the same event. There is no safe universal rule for all callins.

## Native versus Wasm source experience

The Rust application API should remain as similar as practical.

Differences belong mainly in build/configuration:

- native target versus Wasm target;
- selected execution environment;
- module packaging.

The same gameplay code should not need to understand NativeInterface query/result structs or WIT transport mechanics.

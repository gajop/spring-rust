# Overview

## Goal

Implement Spring's unit-script abstraction in Rust without reducing Rust to a
COB-like scripting language.

Rust CUS should support the responsibilities currently handled by COB and LUS:

- unit-script lifecycle and standard engine entry points;
- piece animation commands and completion waits;
- concurrent script tasks, sleeps, and cancellation/signals;
- weapon query/aim/fire hooks;
- custom game-defined unit-script entry points;
- per-unit script state.

At the same time, Rust CUS should be ordinary game code. A Zero-K CUS script,
for example, must be able to call Zero-K Rust APIs directly when those APIs are
part of the same synced module.

## Architecture

```text
                    synced game module
        +---------------------------------------+
        |                                       |
        |   game logic <---- Rust ----> CUS     |
        |       |                     scripts   |
        |       +-------- Spring API --------+  |
        |                                       |
        +------------------+--------------------+
                           |
                common Rust backend API
                           |
                 +---------+---------+
                 |                   |
              Core Wasm            native
                 |                   |
                 +---------+---------+
                           |
                         engine
```

The unit-script scheduler is a per-instance domain inside the synced game
module. It is not another module-wide event fan-out mechanism.

## V1 boundaries

V1 should prove:

- engine attachment and routing for Rust CUS instances;
- standard CUS entry points;
- engine-side animation integration;
- stable Rust async scheduling for sleeps and animation waits;
- game-to-CUS and CUS-to-game communication inside one synced module;
- both Core-Wasm and native execution through the same public Rust API;
- deterministic failure and lifecycle behavior compatible with synced play.

V1 does **not** require exact serialization of suspended Rust futures. See
[Save/load](save-load.md) for the forward path.

## Non-goals

- Replacing the existing `CUnitScript` abstraction with a different engine
  concept.
- One Wasm module per unit or per script.
- Restricting scripts to a fixed COB-sized API.
- Making COB or LUS disappear.
- Defining every game's gameplay API in Spring.
- Requiring nightly Rust.

## Terminology

- **CUS** — Spring's engine-side custom unit-script abstraction.
- **Rust CUS** — Rust implementation/authoring model for CUS.
- **WasmCUS** — Rust CUS running inside a synced Core-Wasm game module.
- **NativeCUS** — the same source/API running in a native game module.
- **COB** — engine bytecode/interpreter unit scripts.
- **LUS** — Lua Unit Scripts.

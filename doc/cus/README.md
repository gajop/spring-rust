# Rust CUS

Rust CUS is a proposed Rust implementation of Spring's existing `CUnitScript`
abstraction. `CUS` is shorthand introduced by these documents for this
`CUnitScript`-based unit-script model; it is not an existing engine acronym.

The proposal has two equally important parts:

1. **Rust CUS** — unit-script instances, scheduling, animation waits, signals,
   standard unit-script entry points, and game-defined entry points.
2. **Portable Spring Rust crates** — the same game-facing Rust API when a game
   is compiled as Core Wasm or as a native module.

CUS is not Wasm-only. These documents live beside the current Rust/Wasm work
because that is where the Rust interface is currently being developed.

The portable V1 runtime, backend-neutral C++ adapter, and concrete NativeCUS
and Core-Wasm transport paths are implemented in this repository. The
attachment, dispatch, and module ABI are recorded in [the implementation
note](implementation.md).

## Core decisions

- A game's CUS scripts live in the **same synced Rust module** as the rest of
  that game's synced Rust logic.
- Game logic and CUS scripts can call each other as ordinary Rust. CUS is not a
  closed capability VM like COB.
- One synced game module can contain many CUS script types and many unit-script
  instances. There is not one Wasm module per unit script.
- The same Rust source should compile for **WasmCUS and NativeCUS**.
- Game-facing Spring crates should expose the same API on both backends.
- V1 uses ordinary stable Rust `async`/`Future`s for suspendable script work.
- Spawned CUS threads/tasks are **named CUS tasks** known to the script/macro;
  the public API does not accept an arbitrary opaque future as a spawned task.
  Their internal storage may still change from boxed futures to generated/inline
  task state without changing game source.
- Ordinary Rust futures are not otherwise forbidden. CUS-provided waits such as
  sleep and animation waits are the suspension operations with stable engine
  semantics and the path a future durable backend is expected to understand.
- Save/load of suspended Rust execution is not a V1 requirement. The authoring
  style should remain usable if durable proc-macro lowering is added later,
  although code using non-durable state across suspension may need local cleanup.
- COB and LUS remain valid backends. Rust CUS is designed for coexistence and
  migration, not mandatory replacement.

A representative authoring shape is:

```rust
#[cus]
impl UnitScript for AmphRaid {
    async fn aim_weapon(
        &mut self,
        ctx: &mut UnitCtx,
        weapon: WeaponId,
        heading: Angle,
        pitch: Angle,
    ) -> bool {
        ctx.turn(self.head, Axis::Y, heading, 380.deg_per_sec());
        ctx.turn(self.turret, Axis::X, -pitch, 160.deg_per_sec());

        ctx.wait_for_turn(self.head, Axis::Y).await;
        ctx.wait_for_turn(self.turret, Axis::X).await;

        true
    }
}
```

The exact trait names and attributes above are provisional. `#[cus]` is used in
these documents rather than `#[callin]`; "callin" already carries Lua/event
meaning in the Spring ecosystem.

The concrete V1 slice uses the stable `spring::cus` traits and
`TaskDefinition` API directly; the `#[cus]` spelling above remains a planned
authoring/code-generation layer and is not required by the runtime.

## Reading order

Start with:

- [Overview](design/overview.md)
- [Authoring UX](design/authoring.md)
- [Module model](design/module-model.md)
- [Game API integration](design/game-api.md)
- [Portable Spring Rust SDK](sdk/portable-spring-rust.md)
- [Concrete V1 implementation](implementation.md)

Then:

- [Scheduling and async](design/scheduling.md)
- [Engine integration](design/engine-integration.md)
- [Save/load path](design/save-load.md)
- [Performance](design/performance.md)

Compatibility and migration evidence:

- [COB, LUS, and Rust CUS](compatibility/cob-lus-cus.md)
- [Game evidence](compatibility/games.md)

Concrete examples:

- [Minimal script](examples/minimal.md)
- [Zero-K `amphraid`](examples/zero-k-amphraid.md)
- [MCL `Infantry`](examples/mcl-infantry.md)
- [Generated animation](examples/generated-animation.md)

Rationale and unresolved details:

- [Why async](rationale/why-async.md)
- [Rust-to-COB](rationale/rust-to-cob.md)
- [Open questions](rationale/open-questions.md)

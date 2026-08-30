# Open Questions

These are implementation/API questions, not unresolved core architecture.

## Authoring syntax

Possible forms include:

```rust
#[cus]
impl UnitScript for Atlas { ... }
```

or a macro over an inherent impl. Trait-based standard methods provide strong
signature validation; exact spelling can be chosen during implementation.

`#[callin]` should be avoided because "callin" strongly suggests the existing
Lua/event terminology.

## Which standard CUS methods may suspend?

Some engine requests require an immediate result, while animation-oriented
methods naturally suspend. The implementer should classify the methods in
`CUnitScript` explicitly rather than make every method async by default.

## Task spawning

CUS needs a constrained equivalent of `StartThread`. Exact syntax and whether
it returns typed handles are open.

## Signals/cancellation

Need to define:

- signal-mask inheritance;
- how tasks are grouped;
- cancellation timing;
- whether dropping a future and running Rust destructors is part of the public
  semantic contract;
- compatibility with LUS/COB edge cases.

## Custom entry-point registry

Need to choose the typed mechanism for:

- game Rust -> CUS calls;
- engine/Lua/other-language -> CUS calls;
- discovering custom exports where dynamic dispatch is needed.

Same-module Rust calls should remain direct whenever possible.

## Unit backend selection

A unit definition needs an explicit deterministic way to select COB, LUS,
WasmCUS, or NativeCUS. Exact syntax is open.

## Save/load requirements

V1 does not persist arbitrary suspended futures. Before building durable macro
lowering, determine what real games require:

- exact continuation of all sleeping tasks;
- restartable/non-persistent tasks;
- persistence of only normal script/game state;
- compatibility expectations relative to current LUS.

## Native/Wasm backend surface

The game-facing API should match. The internal crate layering and which current
`spring-native*` crates remain public are implementation questions.

## Benchmark thresholds

The workload suite is clear, but acceptable overhead targets should be set from
measurement against real BAR-, ZK-, and MCL-like cases rather than guessed here.

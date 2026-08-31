# Rust-to-COB as an Adjacent Approach

Rust-to-COB and Rust CUS solve different problems.

## Rust-to-COB

A Rust-shaped DSL or generator could emit BOS/COB while the engine continues to
execute the existing COB VM.

Advantages:

- almost no new unit-script runtime;
- inherits COB scheduling, waits, signals, animation behavior, and existing
  save/load semantics;
- attractive for generated/mechanical animation scripts;
- compatible with existing COB-heavy content pipelines.

Limitations:

- the source cannot be arbitrary Rust;
- available behavior is limited to what can be lowered to COB;
- arbitrary crates, rich types, heap-backed structures, closures, and direct
  game-specific APIs do not naturally become COB;
- adding a new game capability still needs a bridge/opcode/engine extension.

This is best understood as a modern **authoring frontend for COB**, not a Rust
runtime.

## Rust CUS

Rust CUS executes Rust (native or Wasm) and can share code/data/APIs directly
with the rest of the synced Rust game module.

That is the path for LUS-like rich unit scripts and for games that want unit
logic and game logic to be one typed Rust program.

## Coexistence

A hybrid is reasonable:

- Rust-to-COB for legacy/generated animation pipelines;
- Rust CUS for rich unit scripts and new Rust-native game systems;
- existing hand-written COB/LUS where migration has no value.

The CUS proposal should therefore not require deleting COB tooling.

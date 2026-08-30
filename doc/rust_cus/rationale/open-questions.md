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

## Named task/spawn API

The public decision is settled: `spawn` refers to a known/named CUS task, not an
arbitrary opaque `Future` value. Still open:

- exact syntax for naming/constructing the task;
- whether spawn returns a typed handle;
- whether V1 boxes the generated future or uses generated/inline storage.

The last item is an internal representation choice and should be benchmarked,
not exposed in game source.

## Signals/cancellation

Need to define:

- signal-mask inheritance;
- how tasks are grouped;
- cancellation timing;
- exact guest-internal destructor behavior;
- compatibility with LUS/COB edge cases.

Normal future cancellation may run Rust `Drop` for guest-internal cleanup, but
engine-visible settlement must have an engine-side guarantee and cannot depend
on guest unwinding.

## Custom entry-point registry

Need to choose the typed mechanism for:

- game Rust -> CUS calls;
- engine/Lua/other-language -> CUS calls;
- discovering custom exports where dynamic dispatch is needed.

Same-module Rust calls should remain direct whenever possible.

## Attachment API

CUS can follow the existing structural LUS model: the owning synced game module
explicitly attaches/replaces the unit's `CUnitScript` implementation. The exact
registration/callout spelling and lifetime bookkeeping remain implementation
questions; a new unit-def backend format is not required for V1.

## Durable save/load requirements

V1 does not persist arbitrary suspended futures. Current behavior gives a clear
baseline: LUS does not persist coroutine execution state, while COB does.

Before implementing durable proc-macro lowering, decide whether the intended
CUS target is:

- LUS-level persistence;
- full COB-like continuation of named tasks;
- or an opt-in persistent-task subset.

The async authoring style and named-task model are designed so this can be added
later. Individual V1 bodies that keep non-persistable state across suspension
may still need local changes when opting into durable tasks.

## Native/Wasm backend surface

The game-facing API should match. The internal crate layering and which current
`spring-native*` crates remain public are implementation questions.

## Benchmark thresholds

The workload suite is clear and COB should be the primary performance baseline
for unit-script-heavy workloads. Acceptable overhead targets should be set from
measurement against real BAR-, ZK-, and MCL-like cases rather than guessed here.

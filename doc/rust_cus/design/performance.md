# Performance

CUS should be designed for large unit counts and frequent engine-to-script
interaction, but architecture should be measured before introducing complex
scheduler data structures.

COB is the primary performance baseline for animation-heavy/unit-script-heavy
workloads; LUS remains an important compatibility and richer-script comparison.

## Avoid unnecessary crossings

The same-module model is important for performance as well as UX:

```text
CUS -> game-specific Rust function
```

is an ordinary Rust call. It should not become:

```text
CUS -> engine -> Wasm -> game
```

merely because the caller is a unit script.

The create/attach contract should also provide a generated mask of the standard
entry points implemented by a script. The engine can then skip unimplemented
entry points without entering the guest.

## No per-unit scheduler tick

Sleeping or waiting CUS tasks must not cause the engine to enter the guest once
per unit per frame. Batchable scheduler work is drained for the owning synced
module as a whole, using the existing frame/budget machinery and deterministic
wakeup phases.

Immediate synchronous unit-script requests remain direct per-instance calls;
the invariant is specifically that idle/sleeping task scheduling does not scale
as one ABI crossing per unit.

## Engine-side animations

Turns, moves, spins, and scales continue to interpolate in the engine. A waiting
Rust task should sleep until completion rather than be polled across the ABI
every simulation tick.

The engine's multithreaded interpolation work should never execute Rust script
code. Wakeups occur on the deterministic script/completion side of that boundary.

## Existing synced budget

WasmCUS execution belongs to the existing RulesSynced Wasm execution budget.
The CUS scheduler should be driven from the owning synced module/frame rather
than creating an independent unbounded execution path.

NativeCUS should follow equivalent logical scheduling even though it does not
need Wasmtime fuel/budget enforcement for process isolation.

## Task representation

The public spawn API uses known/named CUS tasks. That deliberately keeps the
storage strategy private.

V1 may box rustc-generated futures for simplicity. If allocation/spawn cost is
material, codegen can later represent the same named task set as inline/generated
state without changing game source or the spawn API.

This choice should be benchmarked early because task allocation is one of the
main ways a Rust async implementation could drift toward the cost profile of
LUS rather than COB.

## Dispatch

Unit IDs are bounded and script routing is hot. Direct indexed storage may be
appropriate, provided stale unit-ID reuse/generation is handled correctly.

Exact choices such as timer-wheel size, intrusive lists, slab layout, and wake
queues should be left to profiling.

## Measurements to require

At minimum compare:

- COB-heavy animation workloads as the primary performance baseline;
- LUS-heavy workloads;
- WasmCUS and NativeCUS with identical Rust source;
- idle scripts;
- many sleeping tasks;
- high task-spawn/cancellation churn;
- many animation completions in one frame;
- frequent weapon query/aim/fire entry points;
- game-specific same-module calls;
- boxed versus generated/inline task storage if both are implemented.

BAR-style large generated animation sets and Zero-K/MCL-style richer scripts
should both be represented in benchmarks.

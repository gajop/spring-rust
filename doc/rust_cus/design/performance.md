# Performance

CUS should be designed for large unit counts and frequent engine-to-script
interaction, but architecture should be measured before introducing complex
scheduler data structures.

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

## Engine-side animations

Turns, moves, spins, and scales continue to interpolate in the engine. A waiting
Rust task should sleep until completion rather than be polled across the ABI
every simulation tick.

## Existing synced budget

WasmCUS execution belongs to the existing RulesSynced Wasm execution budget.
The CUS scheduler should be driven from the owning synced module/frame rather
than creating an independent unbounded execution path.

NativeCUS should follow equivalent logical scheduling even though it does not
need Wasmtime fuel/budget enforcement for process isolation.

## Dispatch

Unit IDs are bounded and script routing is hot. Direct indexed storage may be
appropriate, provided stale unit-ID reuse/generation is handled correctly.

Exact choices such as timer-wheel size, intrusive lists, slab layout, and wake
queues should be left to profiling.

## Measurements to require

At minimum compare:

- COB-heavy animation workloads;
- LUS-heavy workloads;
- WasmCUS and NativeCUS with identical Rust source;
- idle scripts;
- many sleeping tasks;
- many animation completions in one frame;
- frequent weapon query/aim/fire entry points;
- game-specific same-module calls;
- worst-case cancellation/signal churn.

BAR-style large generated animation sets and Zero-K/MCL-style richer scripts
should both be represented in benchmarks.

# Core Wasm benchmark comparison

`run_benchmarks_core.py` extends the existing native API parity benchmark with a fifth raw backend while retaining the same fixtures, scales and comparable test rows:

- Lua
- native C API
- dynamic Component Model host
- typed Rust Component Model host
- unchecked Core Wasm host

Run the bounded suite with the same engine binaries used by the existing runner:

```bash
python3 test/native_api_parity/run_benchmarks_core.py \
  --suite --bounded-suite \
  --spring-headless ./spring-headless \
  --spring ./spring
```

Individual profiles use the existing flags, for example `--callouts`, `--callins`, `--heightmap`, `--workloads`, `--memory` and `--draw`.

The Core runner builds a raw wasm32 guest for each Component benchmark variant and sets `SPRING_WASM_CORE_HOST=1` only for the Core process. Synced Core guests use fixed memory because synced validation requires memory `min == max`.

Core guest-produced rows use the historical `benchmark_wasm.jsonl` sink while engine-side Core callin timing uses `benchmark_wasm_core.jsonl`. The runner merges both streams and normalizes their backend label to `wasm_core` before applying comparable-row validation.

## Comparison ratios

Keep Typed Component Model as a raw reference column, but decision ratios are only:

- Lua vs native
- Lua vs Core
- Core vs native
- dynamic Component Model vs Core

Do not add Typed-vs-Core ratios.

## Variable callins

Variable-shape engine-to-guest callins have a dedicated runner:

```bash
python3 test/native_api_parity/run_variable_callins_core.py \
  --spring-headless ./spring-headless
```

It measures `AddConsoleLine` and `CommandNotify` in two ways:

- `callin_string_event` / `callin_command_event`: identical outer `CEventHandler` boundaries for Lua and Core. These rows are the decision comparison and may publish Lua/Core ratios.
- `callin_string` / `callin_command`: inner transport diagnostics. Lua starts after argument pushing while Core includes scratch lowering, so these rows must not publish Lua/Core ratios.

The Core variable-callin path uses one guest-owned scratch buffer negotiated at bind time. Steady-state lowering performs bounded writes followed by one unchecked host-to-guest call and does not allocate host heap storage.

## Core transport-ceiling rows

The callout profile also emits `core_ceiling_*` rows. These are Core-only absolute measurements and intentionally do not participate in cross-backend validation or ratios.

They remove avoidable per-call allocation/probing so the report can distinguish Core transport cost from the convenience cost of owned Rust APIs:

- fixed multi-field result
- borrowed string input
- borrowed `f32[]` input
- reused string output buffer
- reused flat-list output buffer
- reused nested `UnitCommand[]` wire buffer
- reused spatial-list output buffer

The borrowed input probes still use the normal unchecked Wasmtime import path, host budget check and guest-memory range validation. They remove host allocation/copy only; they are not an artificial unchecked-memory benchmark.

Add another ceiling row only for a materially different ABI shape. The goal is a representative transport matrix, not a large microbenchmark collection.

Do not populate or commit measured values from a different machine merely to fill the Core column. Regenerate the table from one complete comparison run.

# Core Wasm benchmark comparison

`run_benchmarks_sdk.py` runs the existing scenarios through the current public
Rust SDK. Native and Wasm compile the same workload source; Lua uses the existing
Lua implementation and fixture. The three backends are:

- Lua
- native `spring-sdk`
- Core Wasm `spring-sdk`

Run the bounded suite with the same engine binaries used by the existing runner:

```bash
python3 test/native_api_parity/run_benchmarks_sdk.py \
  --suite --bounded-suite \
  --spring-headless ./spring-headless \
  --spring ./spring
```

Individual profiles use the existing flags, for example `--callouts`, `--callins`, `--heightmap`, `--workloads`, `--memory` and `--draw`.

`run_benchmarks_core.py` remains available for comparison with the legacy Rust
APIs. SDK reports use a separate frozen-results directory so a missing SDK
measurement cannot silently inherit a legacy timing. Synced Wasm guests use
fixed memory because synced validation requires memory `min == max`.

`--quick --module-ref` measures a stored module identifier resolved for every
scalar query. Without `--module-ref`, queries use the callback's Spring directly.
`RECOIL_SDK_BENCHMARK_NATIVE_ARTIFACT` selects a previously built native SDK
benchmark library for a before/after comparison; its SHA-256 is recorded in the
summary. The selected artifact must have matching case/count/repeat settings.

Core guest-produced rows use the historical `benchmark_wasm.jsonl` sink while engine-side Core callin timing uses `benchmark_wasm_core.jsonl`. The runner merges both streams and normalizes their backend label to `wasm_core` before applying comparable-row validation.

## Measurements, 2026-10-18

The old native wrapper did not lock on each query. That cost was introduced by
the unified SDK and removed here: callback entry still serializes module state,
while engine calls check thread identity without locking. API-family accessors
borrow their owner instead of cloning an Arc. Stored ModuleRef access reuses the
current callback's entry guard, but still upgrades a weak reference and checks
its scope.

Median nanoseconds per scalar query, 50 samples of one million calls:

| API | Before cleanup | After cleanup | Legacy API |
| --- | ---: | ---: | ---: |
| Native, Spring supplied by callback | 57.20 | 10.28 | 6.08 |
| Native, resolve ModuleRef for every query | 155.06 | 20.18 | — |
| Wasm, Spring supplied by callback | — | 17.68 | 16.86 |
| Lua | — | 32.22 | 32.16 in the legacy comparison run |

The new native direct path still costs about 4.2 ns more than the old wrapper
on this minimal query. Wasm adds about 0.8 ns in this run. ModuleRef lookup is
optional; resolve it once around a loop when appropriate. These are API timings,
not a whole-game FPS measurement.

Selected nominal-count callouts, median ns per call (10 samples):

| Operation | Legacy native | SDK native | Legacy Wasm | SDK Wasm | Lua |
| --- | ---: | ---: | ---: | ---: | ---: |
| Position vector | 6.67 | 12.96 | 37.88 | 37.51 | 62.37 |
| Small command list | 28.08 | 33.90 | 125.85 | 114.95 | 604.01 |
| 1,000 unit IDs | 558.62 | 542.00 | 1109.00 | 1036.00 | 8532.76 |
| Numeric parameter mutation | 39.53 | 32.44 | 36.06 | 36.68 | 204.42 |

This is not a uniform improvement: minimal native vector queries remain roughly
twice as expensive as the legacy wrapper, while some allocation-heavy paths
improve. Full rows and samples are retained, including the Lua variation between
runs.

Engine-to-module callback dispatch also retains overhead. These are **cold-cache**
engine boundary measurements, including the SDK entry path and user handler;
they should not be read as the steady-state cost of a mutex or as an FPS loss.

| Callback | Legacy native | SDK native | Legacy Wasm | SDK Wasm | Lua (SDK comparison run) |
| --- | ---: | ---: | ---: | ---: | ---: |
| GameFrame | 1.65 µs | 2.51 µs | 3.89 µs | 4.79 µs | 5.08 µs |
| UnitCreated | 1.23 µs | 3.67 µs | 3.92 µs | 4.67 µs | 4.96 µs |
| Update | 1.21 µs | 1.92 µs | 2.33 µs | 3.19 µs | 3.54 µs |

All **37 comparable cases pass for all three backends**, covering callouts,
heightmap callbacks, gameplay workloads, engine/Lua memory accounting, drawing
and callback dispatch. Native/Wasm use the same Rust source. Callouts, heightmap,
workloads and memory use nominal counts; dispatch uses scale 0.1, draw 0.01.
Guest timing loops use 10 samples, except the separate 50-sample scalar comparison
above. Engine callback sampling records its actual per-dispatch samples in JSON.
Memory rows report engine/Lua counters and are not a measurement of Rust
allocator calls.

Host: Intel i7-12700, Linux, release Rust guests and the existing release engine
`2026.10.16-44-g4c5a86c`; draw uses Xvfb/llvmpipe. Backend runs are sequential;
this is a workstation, not an isolated performance lab. Samples, ranges, counts,
artifact hashes and the full profile rows are in
[the dated JSON results](../../rts/wasm/docs/data/sdk_perf_2026-10-18/).
The historical SDK artifacts were built from commit `1972d78d36`, using the
same benchmark source and counts. No frozen baseline, golden image or tolerance
was changed.

## Comparison ratios

- Lua vs native
- Lua vs Core
- Core vs native

## Variable callins

Variable-shape engine-to-guest callins still have a separate legacy transport
runner; they have not yet been moved to the SDK benchmark guest:

```bash
python3 test/native_api_parity/run_variable_callins_core.py \
  --spring-headless ./spring-headless
```

It measures `AddConsoleLine` and `CommandNotify` in two ways:

- `callin_string_event` / `callin_command_event`: identical outer `CEventHandler` boundaries for Lua and Core. These rows are the decision comparison and may publish Lua/Core ratios.
- `callin_string` / `callin_command`: inner transport diagnostics. Lua starts after argument pushing while Core includes scratch lowering, so these rows must not publish Lua/Core ratios.

The Core variable-callin path uses one guest-owned scratch buffer negotiated at bind time. Steady-state lowering performs bounded writes followed by one unchecked host-to-guest call and does not allocate host heap storage.

## Core transport-ceiling rows

The legacy runner's callout profile also emits `core_ceiling_*` rows. The SDK
runner does not include those direct transport probes. These are Core-only absolute measurements and intentionally do not participate in cross-backend validation or ratios.

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

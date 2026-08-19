# Core-Wasm benchmark matrix

The purpose of these benchmarks is not to compare generic Wasm throughput. They
measure the costs that matter to Spring: frequent host/guest crossings, native
API marshalling, and cache-cold re-entry after renderer work.

Existing measurements on the project provide the initial floor:

- raw Wasmtime Core guest -> host unchecked: about 4 ns/call;
- raw Wasmtime Core host -> guest unchecked: about 11 ns/call;
- Rust typed Core measurements are in the same performance class;
- typed Component Model crossings were several times more expensive.

Do not compare new engine measurements directly to the naked 4/11 ns numbers:
the engine paths deliberately include Spring host-work/re-entry policy and the
actual NativeInterface function.

## Build variants

Benchmark the exact same guest/module under:

1. `Core/raw`: existing naked Wasmtime Core microbenchmark.
2. `Core/Spring scalar`: `get-unit-def-id` or `get-unit-team`.
3. `Core/Spring scalar-f32`: `get-unit-experience`.
4. `Core/Spring fixed POD`: `get-unit-position`, `get-unit-health`.
5. `Core/Spring list`: `get-team-units` with pre-sized output.
6. `Core/Spring list probe+fill`: zero-capacity size probe followed by exact fill.
7. `Core/Spring bytes`: unit-def name with pre-sized output.
8. Component C dynamic, if retained for reference.
9. Component Rust typed, if retained for reference.
10. Native C++ direct call.
11. Lua equivalent where meaningful.

For Wasmtime Core, measure both fuel disabled and fuel enabled with the intended
production fuel configuration. Fuel instrumentation affects guest execution,
not merely crossings, and should never be inferred from a boundary-only test.

## Guest -> host callouts

Use dependency chains so the compiler/CPU cannot overlap independent calls.
Report median, p95 and ns/call for at least:

- 1 call;
- 10 calls;
- 100 calls;
- 10,000 calls;
- 1,000,000 calls in a synthetic benchmark.

Measure:

```
get-unit-def-id(unit)                         scalar i32 -> packed i64
get-unit-experience(unit)                     scalar i32 -> packed f32/status
get-unit-position(unit,false,false)            12-byte output
get-unit-health(unit)                          20-byte output
get-team-unit-count(team)                      scalar count
get-team-units(team, pre-sized buffer)         list, one crossing
get-team-units(team, probe + exact fill)        list, two crossings
get-unit-def-name(pre-sized byte buffer)        variable bytes
```

For list tests record result count separately. Copy cost should be reported per
element in addition to total call latency.

## Host -> guest callins

Measure cached exports only. No export lookup/type reflection may occur inside
the timed loop.

```
GameFrame(i32)
GameFramePost(i32)
Update(f32)
UnitCreated(4 x i32)
UnitPreDamaged(10 args -> packed f32 pair)
AllowUnitCreation(8 args -> i32 flags)
DrawWorld()
```

Run an empty handler first, then a handler containing realistic Rust logic.
This separates runtime boundary cost from guest work.

## Cache-cold entry

The earlier Component measurements showed a multi-microsecond first call after
large renderer/cache activity. Core ABI savings cannot be assumed to fix this;
treat it as a separate working-set experiment.

For each runtime/call path, record:

1. immediate repeated call (hot);
2. call after touching 64 MiB sequentially;
3. call after touching 256 MiB sequentially;
4. call after the actual renderer world-draw workload;
5. immediate second call after each cold case.

Primary callins:

- `DrawWorld`;
- `GameFrame` as a non-render control;
- an empty Core export as the absolute runtime-entry control.

Record at least 10,000 samples for synthetic eviction and enough real frames to
show the distribution. Report p50/p95/p99 and maximum, not only the mean.

If Core remains multi-microsecond only after renderer activity, profile:

- instruction-cache misses;
- dTLB/iTLB misses;
- LLC misses;
- branch misses;
- Wasmtime code/VMContext working-set pages;
- engine callin dispatch working set.

Linux `perf stat`/`perf record` around a deterministic replay is more useful
here than another generic Wasm benchmark suite.

## Memory access variants

For fixed-output/list calls compare:

1. synced fixed memory: cached base + cached size;
2. unsynced growable memory: Wasmtime base/size query each call;
3. no-copy scalar result when semantically possible.

This directly measures the value of the fixed-memory synced invariant.

## Policy-cost variants

Temporarily benchmark, but do not necessarily ship, these variations:

- import re-entry/host-work guard enabled vs compiled out;
- debug invariant checks vs release invariant checks;
- fuel off vs on;
- NaN canonicalization off vs on for a floating-point-heavy guest;
- standard SIMD off vs on;
- relaxed SIMD remains off for the synced production profile.

The goal is to know the price of each safety/determinism property instead of
removing one based on assumption.

## Runtime comparison later

Once Spring Core ABI is stable, port this exact matrix to WAMR rather than using
CoreMark as the decision benchmark. Minimum useful comparison:

- Wasmtime current Core backend;
- WAMR AOT quick/native entries;
- WAMR LLVM JIT if appropriate.

The guest `.wasm`, Spring ABI signatures, semantic test inputs and expected
outputs should be identical. Runtime-specific AOT artifacts are disposable
cache/build products and must not become the content ABI.

## Acceptance guidance

Treat runtime-boundary differences in absolute frame cost. Examples:

- 10 ns saved on 100 calls/frame = 1 microsecond/frame;
- 10 ns saved on 10,000 calls/frame = 100 microseconds/frame;
- one 5 microsecond cache-cold entry per frame can dominate hundreds of hot
  scalar crossings.

For that reason, report both calls/frame and total microseconds/frame for at
least one real game workload before changing runtime solely because of a
microbenchmark ratio.

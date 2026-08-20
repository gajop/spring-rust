# Core-Wasm benchmark matrix

These benchmarks measure the costs that matter to Spring: frequent host/guest crossings, NativeInterface marshalling, variable payloads, callbacks/re-entry, and cache-cold entry after renderer work.

Existing project measurements provide the initial naked floor:

- raw Wasmtime Core guest -> host unchecked: about 4 ns/call;
- raw Wasmtime Core host -> guest unchecked: about 11 ns/call;
- Rust typed Core measurements are in the same performance class;
- typed Component Model crossings were several times more expensive.

Do not compare engine measurements directly to naked 4/11 ns calls: engine paths intentionally include Spring dispatch/safety policy and actual NativeInterface work.

## Decision comparisons

Raw columns may include Lua, native, dynamic Component Model, typed Component Model, and Core Wasm.

Decision ratios are only:

- Lua vs native;
- Lua vs Core;
- Core vs native;
- dynamic Component Model vs Core.

Typed Component Model remains a historical/reference column, not a decision ratio.

## Representative ABI-shape matrix

Do not add a benchmark for every API. Add one when it exercises a materially different transport shape.

### Guest -> host callouts

1. **Direct scalar:** `get-unit-def-id` / `get-unit-team` (`i32 -> packed i64`).
2. **Direct float result:** `get-unit-experience` (`i32 -> packed f32/status`).
3. **Fixed POD result:** `get-unit-position` (12 bytes), `get-unit-health` (20 bytes).
4. **Flat small list:** pre-sized team-unit output.
5. **Flat large list:** same ABI with enough elements for copy cost to dominate.
6. **Probe + fill list:** zero-capacity size query then exact fill.
7. **Raw byte/string output:** unit-def name into caller-owned storage.
8. **Nested list/records:** `GetUnitCommands` including command records and parameter arrays.
9. **Spatial list:** real area/radius query into reused output storage.
10. **Variable input + mutation:** RulesParam/string or command-order path.
11. **Callback/re-entry:** heightmap and Gfx callback paths.
12. **Mixed variable input/output:** add only when a production executable binding exists.

### Host -> guest callins

1. **Empty:** empty export / `DrawWorld()` control.
2. **Scalar:** `GameFrame(i32)`, `Update(f32)`.
3. **Fixed multi-field:** `UnitCreated(4 x i32)`.
4. **Many args + fixed result:** `UnitPreDamaged(...)->packed f32 pair`.
5. **Fixed control result:** `AllowUnitCreation(...)->flags`.
6. **Variable strings:** `AddConsoleLine` (two byte strings + level).
7. **Fixed record + variable flat list:** `CommandNotify` (command header + `f32[]`).
8. **Missing export:** verifies optional-dispatch floor.
9. **Multi-module fan-out:** four-module callin.
10. **Cache-cold:** `DrawWorld`, `GameFrame`, and empty export after renderer/cache eviction.

The two variable callins use a guest-owned scratch region negotiated once at module binding. Their steady-state Core path should contain one bounded serialization and one unchecked host -> guest call, with no host heap allocation.

## `list<string>` rule

Do **not** benchmark or ship a generic `list<string>` lowering that constructs a host `vector<string>` or performs one allocation per element. A slow generic implementation would only measure an ABI we already know we do not want.

When a real API requires `list<string>`, first implement a reviewed flat representation:

- one packed byte blob;
- one offset/length descriptor array;
- caller-owned/reused storage where practical;
- if NativeInterface requires `const char**`, construct only the pointer table, with no per-element host string ownership.

Then add one representative `list<string>` row. Measure both a small list and a payload large enough for byte-copy cost to dominate, but do not add many near-duplicates.

## Callout iteration counts

Use dependency chains so the compiler/CPU cannot overlap independent calls. Report median/p95/p99 and ns/call for at least:

- 1 call;
- 10 calls;
- 100 calls;
- 10,000 calls;
- 1,000,000 calls for synthetic floor tests.

For list/string tests record payload bytes/elements separately. Report copy cost per byte/element in addition to total latency.

## Variable callin timing

The focused runner currently has inner `callin_string` and `callin_command` rows.

These are useful diagnostics but not yet strict cross-backend ratios:

- Core timing includes scratch serialization + unchecked guest call;
- Lua's central timer begins after its arguments have already been pushed.

The recorder reserves `callin_string_event` and `callin_command_event` for an identical outer event boundary. Use those outer rows for the actual Lua-vs-Core decision ratio once wired. Keep inner rows as transport diagnostics.

## Core transport-ceiling rows

`core_ceiling_*` rows intentionally measure a different question: the practical Core floor when caller-owned storage/probes can be amortized. Do not mix them into high-level API ratios.

Current representative ceiling shapes:

- fixed struct;
- borrowed string input;
- borrowed `f32[]` input;
- reusable string output;
- reusable flat list output;
- reusable nested `UnitCommand[]` wire output;
- reusable spatial list output.

A future flat `list<string>` implementation should get one ceiling row as well.

## Cache-cold entry

The previous Component measurements showed a multi-microsecond first call after renderer/cache activity. Core boundary savings cannot be assumed to eliminate this; treat it separately.

For each path record:

1. immediate repeated call (hot);
2. call after touching 64 MiB sequentially;
3. call after touching 256 MiB sequentially;
4. call after actual renderer world-draw work;
5. immediate second call after each cold case.

Primary callins:

- `DrawWorld`;
- `GameFrame` as non-render control;
- empty Core export as runtime-entry control.

Report p50/p95/p99/max. If renderer-cold Core remains multi-microsecond, profile i-cache, dTLB/iTLB, LLC misses, branches, Wasmtime code/VMContext pages and engine dispatch working set.

## Memory-access variants

For fixed/list calls compare:

1. synced fixed memory: cached base + cached size;
2. unsynced growable memory: current Wasmtime base/size query;
3. scalar no-memory result where semantics permit it.

This measures the value of the synced fixed-memory invariant directly.

## Performance-cost axes

Measure these separately; do not conflate their categories.

### Sync cost

- NaN canonicalization off/on for FP-heavy guest;
- standard SIMD on/off if useful as a control;
- relaxed SIMD remains off for synced production unless deterministic semantics are established;
- deterministic fuel only if it is intended for synced execution.

### Safety cost

- import re-entry/host-work guard enabled vs compiled-out control;
- pointer/range/result validation where a comparable safe control can be written;
- scratch re-entry protection;
- debug-only invariant checks vs release.

Safety controls are measurements, not a recommendation to ship unsafe paths.

### Visibility cost

- rules/gaia specialized scalar path;
- UI generated path with visibility context/filtering.

Do not label this a security benchmark. It measures game-information filtering.

### Security cost

Benchmark only defenses that actually concern sandbox/OS escape and can add recurring runtime cost. Load-time import allow-listing, absence of WASI and similar one-time policy checks do not belong in nanosecond hot-path tables.

## Fuel

Measure fuel-disabled and fuel-enabled configurations only if fuel is a realistic production option. Fuel instrumentation affects guest execution, not merely crossing overhead, so include realistic compute workloads in addition to boundary-only tests.

## Runtime comparison later

Once the Spring Core ABI is stable, port this exact matrix to WAMR rather than choosing from CoreMark alone. Minimum useful comparison:

- Wasmtime Core;
- WAMR AOT quick/native entries;
- WAMR LLVM JIT if appropriate.

Guest `.wasm`, Spring ABI signatures, semantic inputs and expected outputs must remain identical. Runtime-specific AOT artifacts are disposable build/cache products.

## Acceptance guidance

Always translate a nanosecond difference into frame cost:

- 10 ns saved on 100 calls/frame = 1 microsecond/frame;
- 10 ns saved on 10,000 calls/frame = 100 microseconds/frame;
- one 5 microsecond cache-cold entry per frame can dominate hundreds of hot scalar crossings.

Report both calls/frame and total microseconds/frame for at least one real game workload before changing runtime or adding ABI complexity based on a microbenchmark ratio.

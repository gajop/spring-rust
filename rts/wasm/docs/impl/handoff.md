# Handoff: Core Wasm host and API migration

Date: 2026-08-20

Core Wasm is now the intended transport direction. The Component Model paths remain useful as a measurement and compatibility reference, but new implementation work should target Core first.

The generated benchmark results remain in [benchmarking_results.md](benchmarking_results.md). Core benchmark design details are in [core_benchmark_matrix.md](core_benchmark_matrix.md), and the ABI contract is in [core_abi_contract.md](core_abi_contract.md).

## Current decision

Use this architecture:

- Engine and host integration: C++.
- Runtime: Wasmtime Core Wasm through the C/C++ API.
- Guest SDK: Rust first, compiled to `wasm32-unknown-unknown`.
- Synced guests: no WASI and no ambient clock, filesystem, network or random access.
- ABI: generated typed Core imports/exports, not a generic `Value` transport.
- Scalars and IDs cross directly as Wasm numeric values.
- Fixed records use explicit guest-memory wire layouts.
- Strings, lists and other variable data use guest memory with pointer/length or caller-owned output buffers.
- Host callbacks re-enter one exported Core callback dispatcher using numeric callback IDs.
- The engine should not require a Rust toolchain at runtime or for end users. Rust is acceptable for code generation and guest SDKs.

The Component Model implementation should not drive new API design. It remains valuable for historical comparison and as a correctness reference while Core reaches full API coverage.

## Why Core is the target

The API is unusually suitable for a Core ABI because the native surface already uses fixed query/result records, IDs, pointers plus counts, and explicit callback handles. The public Rust guest API can remain ergonomic while generated bindings perform the raw lowering.

The working assumption is that Core will be the preferred performance path. Do not spend time optimizing the typed Component path unless a correctness issue requires it. Benchmarks still retain its raw measurements as a reference, but decision ratios should be Core-centric.

## Benchmark comparison

Use `test/native_api_parity/run_benchmarks_core.py`. It layers Core onto the existing benchmark suite instead of maintaining a second experiment.

The report keeps the raw backend columns:

- Lua
- native
- Wasm dynamic Component Model
- Wasm typed Rust Component Model
- Wasm Core

The comparison ratios should now focus on:

- Lua vs native
- Lua vs Core
- Core vs native
- dynamic Component Model vs Core

Do not add new ratios against typed Component Model. Its raw column is enough. The assumption under test is whether Core is good enough to become the primary Wasm transport, not whether more work can improve the typed Component implementation.

Core benchmark coverage now includes all existing profile families:

- Callouts: scalar, vec3, string, small list, large list, spatial query and mutation.
- Callins: empty, GameFrame, Update, UnitCreated, UnitPreDamaged, AllowUnitCreation, unimplemented export and four-module fan-out.
- Heightmap: empty callback, small/medium/large brushes and region operation.
- Workloads: unit scan, area effect, RulesParams, unit commands and compute.
- Memory: small-call allocation, list allocation, GC pause, frame spikes and memory growth.
- Draw: DrawWorld, per-vertex Gfx callout and UI draw workload.

The Core guest uses the same profile variables, operation counts and fixture behavior as the Component guest. Guest-produced rows and engine-side callin rows are merged into one `wasm_core` result before the existing row validator runs.

No benchmark numbers have been produced in this branch yet. Do not edit `benchmarking_results.md` manually.

## Core runtime implementation

Main runtime files:

- `rts/WasmInterface/WasmCoreHost.{h,cpp}`: Core module loading, instantiation and dispatch.
- `rts/WasmInterface/WasmCoreAbi.{h,cpp}`: guest memory helpers and ABI primitives.
- `rts/WasmInterface/WasmCoreValidation.{h,cpp}`: module/profile validation before instantiation.
- `rts/WasmInterface/WasmCoreRegistry.h`: executable Core import registry and environment masks.
- `rts/WasmInterface/WasmCoreGeneratedSupport.h`: shared helpers for generated and specialized bindings.
- `rts/WasmInterface/WasmCoreWire.h`: deterministic fixed-layout wire writer.

The Core host is reached through the existing typed-host seam. `WasmTypedHost::Enabled()` also recognizes Core, and callin dispatch tries `WasmCoreHost` first. This means Core callins pass through the same engine event path and benchmark timing boundary as the existing Wasm host.

Specialized bindings currently exist for benchmark-critical APIs such as unit queries, unit definitions, unit commands, orders, terrain edits, Gfx callbacks, profiling, RulesParams and benchmark messaging.

## Callback and re-entry model

Callbacks use numeric guest callback IDs plus an opaque `u32` user value. The guest exports:

`spring:callback/dispatch(i32 callback_id, i32 user_data)`

The host resolves and validates this export on first use and caches the `wasmtime_func_t`. Repeated callbacks use the cached unchecked call path. Callback traps propagate through the originating import.

The existing execution-budget re-entry model permits callback -> guest -> host nesting. Terrain `SetHeightMapFunc` and Gfx `BeginEnd` use this path.

Do not retain raw pointers into guest memory across a guest call, callback re-entry or anything that may grow memory. Validate/copy input before mutating engine state.

## Determinism and sandbox profile

Synced Core modules should remain deliberately restricted:

- no WASI
- no threads/shared memory
- fixed non-growable memory
- no relaxed SIMD unless deterministic semantics are explicitly enabled
- deterministic NaN handling when cross-platform floating-point behavior requires it
- deterministic host imports only
- no process clocks/random/filesystem/network in ordinary synced code

The current validator requires synced Core memory to declare `max == min`. Benchmark synced guests use a fixed 16 MiB memory because list-heavy benchmark cases need more than the minimal default.

If execution interruption becomes simulation-visible, use deterministic fuel rather than timing-dependent epoch interruption. Benchmark its cost before enabling it globally.

## Core API generation

Core support belongs in `spring-native-codegen`, not in hundreds of handwritten C++ files.

Relevant generators:

- `render_core_wasm.rs`: builds a transport-neutral Core ABI plan from the shared semantic API model.
- `render_core_wasm_host.rs`: emits Wasmtime host callbacks for mechanically safe shapes.
- `spring-api-codegen`: writes the Core ABI plan, inventory and generated host bindings together with the existing generated API artifacts.

The generator should be expanded aggressively for mechanical cases. Prefer generated code whenever the semantic model contains enough ownership/layout information to prove the lowering.

Target lowering rules:

- scalar, enum, ID and handle input: direct Wasm numeric argument
- scalar/enum 32-bit result: packed value + status in `i64`
- 64-bit or multi-field fixed result: caller-provided output pointer + integer status
- fixed record/fixed array input: one validated wire-record pointer
- string/bytes/list input: descriptor or pointer/count pair into guest memory, copied/validated before mutation
- variable output: caller-provided buffer with required/written length convention
- nested fixed records: deterministic explicit wire layout, never native `sizeof`/padding
- callback: numeric callback ID and user data
- resources: integer handles with explicit create/drop lifetime operations

Keep handwritten bindings only for semantic cases the model cannot yet describe, especially callback ownership, resource lifetime, unusual pointer semantics or APIs that need custom batching.

## API conversion priority

The benchmark subset is no longer the desired stopping point. Continue converting the complete NativeInterface surface to Core.

The efficient order is by lowering shape rather than API module:

1. Generate every automatic scalar and fixed-output function.
2. Generate fixed-record and fixed-array inputs.
3. Generate ordinary strings/bytes and simple lists.
4. Generate variable outputs with caller-owned buffers.
5. Add reusable callback/resource lowering primitives.
6. Leave only genuinely semantic/manual annotations as handwritten implementations.

Do not create one handwritten Core file per remaining callout. Improve the renderer once and let it cover all APIs with that shape.

The generated executable registry must contain only imports whose host callback is actually compiled. The diagnostic ABI inventory may list planned/manual/unsupported functions, but validation must never advertise an import that the linker does not implement.

Specialized benchmark bindings may temporarily shadow generated imports where they use a more efficient ABI. Avoid duplicate linker definitions. Eventually benchmark code should exercise the same generated production binding whenever the generic generated lowering is equally efficient.

## Guest SDK direction

`rust/crates/spring-wasm-core/` is the first guest SDK. Guest authors should not see raw pointer arithmetic or unchecked imports.

The public API should continue returning normal Rust values such as `String`, `Vec<T>`, records and typed IDs. The generated/internal layer handles guest-memory allocation, retry/required-length calls and wire decoding.

For hot APIs, later add explicit borrowed/scratch-backed or batched variants only when benchmarks show owned `Vec`/`String` reconstruction is materially expensive. Do not make the ordinary API unsafe for speculative performance gains.

A C header SDK also exists under `rts/wasm/sdk/core/`; keep the ABI language-neutral even though Rust is the first-class guest SDK.

## Build and benchmark

Engine:

`./docker-build-v2/build.sh --compile linux`

Bounded complete comparison:

`python3 test/native_api_parity/run_benchmarks_core.py --suite --bounded-suite --spring-headless ./spring-headless --spring ./spring`

Nominal suite:

`python3 test/native_api_parity/run_benchmarks_core.py --suite --spring-headless ./spring-headless --spring ./spring`

One profile:

`python3 test/native_api_parity/run_benchmarks_core.py --callouts --scale 1 --no-report --summary-json <path>`

Benchmarks should use a release build without ASAN and an otherwise idle machine. The draw profile opens a window.

## Current verification state

This Core expansion was intentionally prepared without building or running it in the implementation environment. Treat the next real compile as an integration pass, not as evidence that any benchmark result is already known.

Static checks already performed while implementing the benchmark path include:

- matching new NativeInterface query/result type names against headers
- matching capability masks against the generated callout registry
- confirming Core is reached by the same event-dispatch seam used by benchmark callin timing
- confirming the Core validator accepts the numeric signature forms used by the new imports
- matching memory benchmark row fields to the existing report formatter

Known carried-over verification issues from the previous Component work remain relevant until retested:

- ASAN plus CTest have not been run against these changes.
- `test_WasmAllocator` previously had one SIGILL failure inside JIT guest code.
- `verify_codegen.py` previously reported gaia_synced probe drift that predates the Core work.

## Working rules

- Performance matters enough to measure actual engine boundary patterns, not generic Wasm compute benchmarks.
- Keep hot Core calls typed and direct. Do not route them through the generic `WasmValue` layer.
- Batch APIs where it reduces boundary crossings without harming semantics.
- Prefer C++ generated host glue over adding a Rust host FFI layer.
- Keep guest memory validation centralized and small.
- Preserve environment/capability restrictions from the semantic registry.
- Do not claim performance results until the benchmark suite has run on a suitable machine.
- Plain language, concise comments, no speculative micro-optimizations without measurement.

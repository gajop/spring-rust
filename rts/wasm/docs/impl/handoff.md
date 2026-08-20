# Handoff: Core Wasm host and API migration

Date: 2026-08-20

Core Wasm is the intended production transport. Component Model implementations remain as correctness and benchmark references, but new API work targets Core first.

## Priority order

Use this order for every design decision:

1. **Highest practical Wasmtime + Core Wasm performance.** Take mechanically clear wins: typed/unchecked calls, fewer boundary crossings, no unnecessary allocation, no unnecessary copying, caller-owned/reused buffers, compact layouts and batching where semantics naturally support it. Do not add complicated speculative optimizations that require profiling to justify.
2. **Synced determinism.** Synced modules must compute identically across supported platforms. Performance work may not weaken simulation correctness.
3. **Security/sandboxing.** Keep security checks that are load-time, boundary-only or otherwise effectively free. Expensive optional defenses may remain configurable rather than taxing the normal hot path.

Do not call a generic generated path "fast" merely because it is semantically correct.

## Architecture

- Engine/host: C++.
- Runtime: Wasmtime Core WebAssembly through the C/C++ API.
- Guest SDK: Rust first, targeting `wasm32-unknown-unknown`.
- ABI: typed Core imports/exports generated from the shared semantic API model.
- Scalars/IDs/enums: direct Wasm numeric values where possible.
- Fixed records/arrays: explicit deterministic wire layouts, never native struct padding.
- Variable results: caller-owned guest buffers with written/required lengths.
- Callbacks: integer callback IDs and one cached guest callback dispatcher.
- Rust may be used for generators/guest SDKs; end users and the built engine must not require a Rust toolchain.

Hot Core calls must not route through the generic `WasmValue` transport.

## Performance rules

Wasmtime host functions use unchecked typed Core callbacks for reviewed signatures. Keep signatures small and direct. Prefer one import over generic dispatch-by-name.

Allocation/copy rules:

- Reuse caller-owned guest output buffers on steady-state hot paths.
- Public owned `Vec`/`String` APIs may allocate for ergonomics, but keep `_into`/borrowed forms available for performance-sensitive code.
- Do not allocate merely to translate between equivalent scalar/list representations.
- C-string inputs need NUL termination. `WasmCoreGuestInput.h::GuestCString` keeps short strings in inline storage and uses one heap fallback only for long strings.
- Do not retain guest-memory pointers across guest re-entry or memory growth.
- Do not remove a defensive copy unless the NativeInterface lifetime/re-entry contract proves the pointer is call-scoped.

### `list<string>`

Generic `list<string>` automatic lowering is forbidden for now.

The previous generic shape required a `vector<string>`, a `vector<const char*>`, and one string copy/allocation per element. The Core planner now marks any function containing `list<string>` as manual, so it cannot enter the generated executable registry accidentally.

A future reviewed representation should use flat guest data: packed bytes plus offset/length descriptors. If the NativeInterface still requires `const char**`, construct only the minimum pointer table needed by that call; do not allocate one host string per element.

## Generated API policy

Core generation is in `spring-native-codegen`.

Important files:

- `render_core_wasm.rs`: runtime-neutral Core ABI plan.
- `render_core_wasm_host.rs`: direct/fixed host bindings.
- `render_core_wasm_option_host.rs`: fixed `option<T>` bindings.
- `render_core_wasm_variable_host.rs`: variable-input scaffolding.
- `render_core_wasm_variable_output_host.rs`: caller-owned variable-output bindings.
- `render_core_wasm_variable_io_host.rs`: combined variable-I/O scaffolding.
- `render_core_wasm_registry.rs`: executable fast-path registry and coverage report.

Generated variable-input and variable-I/O translation units still exist as implementation scaffolding, but their current forms allocate/copy host vectors. They are therefore **not advertised by the generated executable registry**.

Generated production-executable classes are currently:

- direct/fixed
- fixed option
- caller-owned variable output

Reviewed specialized bindings may expose variable-input APIs with a faster ABI and shadow generated bindings.

`spring-api-codegen` writes `core-executable-coverage.json`. Treat that as the executable fast-path coverage report. `core-abi.json` is a broader planning inventory and must not be interpreted as production parity.

## Variable input direction

Benchmark-only imports now measure the achievable Core floor for borrowed string and `f32[]` inputs: one unchecked Wasmtime import, normal budget/range validation, no host allocation and no copy.

Production zero-copy input should only be enabled where the NativeInterface contract proves that the pointer is consumed synchronously and cannot be retained or used across re-entry. Until that is proven, conservative copies are correct even if benchmark ceiling numbers show a lower possible cost.

## Variable callins

Current Core callins cover empty/scalar/many-argument/fixed-result cases but not representative variable payloads yet.

For engine -> guest string/list callins, do **not** call a guest allocator/reserve export for every event. That would add a second host->guest crossing.

Required design:

- Guest optionally exports `spring:callin/scratch-info() -> i64`.
- Low 32 bits: guest-memory offset; high 32 bits: capacity.
- Host resolves/calls it once during module binding and validates the entire range once.
- Variable callin hot path: host serializes into that cached guest-owned scratch region, then performs exactly one callin invocation.
- Scratch size is guest/module-selected, not an arbitrary mandatory engine constant.
- Oversize handling is a cold path and may use a separate fallback protocol later.

Land the scratch negotiation atomically with the first real variable callin; do not leave unused binding state in the runtime.

Representative real callins suitable for subsequent coverage include `AddConsoleLine` for strings and `AllowCommand`/`CommandNotify` for a nested command containing a variable `float[]`. Avoid starting with `KeyPress`, whose nested strings mix several ABI problems into one measurement.

## Benchmark policy

Use `test/native_api_parity/run_benchmarks_core.py`.

Raw comparable backends remain:

- Lua
- native
- dynamic Component Model
- typed Rust Component Model
- Core Wasm

Decision ratios are only:

- Lua vs native
- Lua vs Core
- Core vs native
- dynamic Component Model vs Core

Do not add Typed-vs-Core decision ratios. The typed CM raw measurement remains for reference.

### Comparable API rows

The existing callout suite covers:

- scalar
- fixed vector/record
- string result
- small nested list
- large list
- spatial list query
- mutating string/scalar call

Existing callins cover:

- empty
- GameFrame
- Update
- UnitCreated
- UnitPreDamaged
- AllowUnitCreation
- missing/unimplemented export
- four-module fan-out

Other profiles cover heightmap callbacks/regions, realistic workloads, memory behavior and drawing.

### Core transport-ceiling rows

The Core guest also emits separate `core_ceiling_*` rows. These are intentionally excluded from cross-backend validation/ratios because they measure the optimized transport floor rather than identical high-level APIs.

Current ceiling samples:

- `core_ceiling_fixed_struct`: fixed multi-field result.
- `core_ceiling_string_in_borrowed`: borrowed string input, range validation only.
- `core_ceiling_f32_list_in_borrowed`: borrowed `f32[]` input, range validation only.
- `core_ceiling_string_out_reuse`: caller-owned reusable string buffer.
- `core_ceiling_list_out_reuse`: caller-owned reusable flat list buffer.
- `core_ceiling_nested_list_out_reuse`: reusable nested `UnitCommand[]` wire buffer.
- `core_ceiling_spatial_list_reuse`: reusable list output around a real spatial query.

These rows answer a different question from the ordinary owned API rows: how close the implementation is to the practical Core boundary floor when allocation/probing can be amortized.

Add new benchmark cases only when they represent a materially different ABI shape. A representative matrix is preferred over dozens of near-duplicates.

## Synced determinism

Synced Core modules remain restricted:

- no WASI
- no ambient filesystem/network/random/clock APIs
- no threads/shared memory
- fixed non-growable memory (`max == min`)
- no relaxed SIMD unless deterministic semantics are explicitly established
- deterministic host imports only
- deterministic floating-point configuration where cross-platform behavior requires it

Do not enable fuel globally for security alone; fuel is currently optional. If execution interruption becomes simulation-visible, deterministic fuel is preferable to timing-dependent epoch interruption, but its performance cost must be measured before normal use.

Do not split runtime engines or add expensive deterministic machinery for speculative gains/losses without evidence. Correctness is mandatory; optimizations around it still require a clear mechanism.

## Security profile

Keep module validation, import allow-lists, memory/table limits, range checks and environment/capability checks. These are load-time or boundary-local protections and are not worth removing speculatively.

Guest pointers/counts must be checked for overflow and range before use. Mutating imports must validate/copy any data whose lifetime is not proven before changing engine state.

If a security feature adds a meaningful recurring hot-path cost, make that tradeoff explicit and configurable rather than silently sacrificing the performance target.

## Runtime/build facts

Main runtime files:

- `rts/WasmInterface/WasmCoreHost.{h,cpp}`
- `rts/WasmInterface/WasmCoreAbi.{h,cpp}`
- `rts/WasmInterface/WasmCoreValidation.{h,cpp}`
- `rts/WasmInterface/WasmCoreRegistry.h`
- `rts/WasmInterface/WasmCoreGeneratedSupport.h`
- `rts/WasmInterface/WasmCoreGuestInput.h`
- `rts/WasmInterface/WasmCoreWire.h`

Specialized bindings currently cover benchmark-critical unit queries/info, unit definitions, commands/orders, terrain, Gfx, profiling, RulesParams and messaging.

Synced memory is fixed, so its cached Wasmtime base/size is stable. Unsynced memory helpers refresh the current base/size when necessary.

## Verification state

This work was intentionally prepared without compiling, running codegen, launching the engine or executing benchmarks. The next execution session is an integration/measurement pass; no performance number is claimed yet.

Do not manually edit generated benchmark results. Run the release benchmark suite on an otherwise idle machine before drawing performance conclusions.

Known carried-over verification items still need retesting in a real build:

- ASAN/CTest have not been run against this Core expansion.
- `test_WasmAllocator` previously had a SIGILL inside JIT guest code.
- `verify_codegen.py` previously reported gaia-synced probe drift predating this work.

# Handoff: Core Wasm host and API migration

Date: 2026-08-20

Core Wasm is the intended production transport. Component Model implementations remain as correctness and benchmark references, but new API work targets Core first.

## Current branch state

The production Core callin path now remains pre-`WasmValue` while using the `WasmInterfaceSystem` module inventory for environment selection, stable module order and result aggregation. The old `WasmCoreHost::DispatchCallin` all-host fan-out remains only as a legacy benchmark seam; the engine's early typed-host path delegates active Core callins to `WasmInterfaceSystem::DispatchActiveCoreCallin`.

Representative variable engine -> guest callins are implemented:

- `AddConsoleLine`: two strings + scalar level.
- `CommandNotify`: fixed command header + variable `f32[]` params.

They use a guest-owned scratch region negotiated once at bind time and perform one bounded serialization plus one unchecked host -> guest call. The hot path has no host allocation. Nested use of the shared scratch is rejected deterministically rather than corrupting the outer payload.

Core host imports now carry the module's real `WasmEnvironment`. UI modules stay on generated visibility-checked imports; rules/gaia keep the hand-specialized scalar path so the existing absolute Core hostcall floor does not pay a visibility-context swap. Unsynced Core faults are removed and the system's Core descriptor inventory is reconciled immediately; synced faults remain sticky.

Known narrow gap: `NativeInterfaceEventClient::DispatchWasmBoolCallin` does not pass the existing `nativeResult` argument into the early typed/Core path. `AddConsoleLine` and `CommandNotify` therefore execute in Core, but a Core `true` return is not yet propagated to the engine. Fix this at the bool-helper/event-client seam; do not introduce global pending-result state to work around it.

Variable-callin benchmark timing is also not yet a perfect cross-backend boundary: Lua's central timer starts after Lua arguments are pushed, while Core's variable timer includes scratch lowering. Treat the inner rows as diagnostics/conservative-to-Core until an outer identical event boundary is recorded. No new variable-callin performance number is claimed yet.

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
- Variable callins: one guest-owned scratch region per guest, negotiated at bind time.
- Callbacks: integer callback IDs and one cached guest callback dispatcher.
- Rust may be used for generators/guest SDKs; end users and the built engine must not require a Rust toolchain.

Hot Core calls must not route through the generic `WasmValue` transport.

## Production callin dispatch

The fast Core callin path is reached from the existing early `WasmTypedHost` seam before `NativeInterfaceWasmAdapter::SerializeCallinQuery` constructs owned values.

`WasmInterfaceSystem::DispatchActiveCoreCallin` builds at most three environment invocations (rules, gaia, UI) in a fixed stack array and passes a `std::span` to `DispatchCoreCallin`; do not reintroduce a per-call `std::vector` allocation here.

`DispatchCoreCallin` uses generated callin metadata to preserve:

- environment masks;
- stable module order;
- `ignore` aggregation;
- boolean `or-true` / `and-false` aggregation;
- `first` aggregation for `DamageCallinResult` and `AllowUnitCreationResult`.

For `first`, every module sees the same incoming engine default; the first contributing result is selected without feeding it into later modules as a new input. `UnitPreDamaged` must preserve an incoming `DamageCallinResult` because an earlier engine event client may already have modified damage/impulse values.

Current mixed Core + Component global ordering is not a solved contract. The early Core path still behaves as the selected fast transport for currently supported typed callins. Do not claim cross-transport module ordering until the two module inventories are unified or an explicit ordering design is implemented.

## Performance rules

Wasmtime host functions use unchecked typed Core callbacks for reviewed signatures. Keep signatures small and direct. Prefer one import over generic dispatch-by-name.

Allocation/copy rules:

- Reuse caller-owned guest output buffers on steady-state hot paths.
- Public owned `Vec`/`String` APIs may allocate for ergonomics, but keep `_into`/borrowed forms available for performance-sensitive code.
- Do not allocate merely to translate between equivalent scalar/list representations.
- C-string inputs need NUL termination. `WasmCoreGuestInput.h::GuestCString` keeps short strings in inline storage and uses one heap fallback only for long strings.
- Do not retain guest-memory pointers across guest re-entry or memory growth.
- Do not remove a defensive copy unless the NativeInterface lifetime/re-entry contract proves the pointer is call-scoped.
- The ordered engine -> Core dispatcher is allocation-free on its invocation-list hot path.

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

Benchmark-only imports measure the achievable Core floor for borrowed string and `f32[]` inputs: one unchecked Wasmtime import, normal budget/range validation, no host allocation and no copy.

Production zero-copy input should only be enabled where the NativeInterface contract proves that the pointer is consumed synchronously and cannot be retained or used across re-entry. Until that is proven, conservative copies are correct even if benchmark ceiling numbers show a lower possible cost.

## Variable callins

Engine -> guest variable callins use one guest-owned scratch region. Do **not** call a guest allocator/reserve export for every event; that would add a second host -> guest crossing.

Implemented protocol:

- Guest optionally exports `spring:callin/scratch-info() -> i64` when it exports a variable callin.
- Low 32 bits: guest-memory offset; high 32 bits: capacity.
- Host resolves/calls it once during module binding and validates the entire range.
- Hot path: host serializes into the cached guest-owned scratch region, then performs exactly one unchecked callin invocation.
- Scratch size is guest/module-selected.
- Oversize payloads currently fail rather than allocating a hidden slow path.
- Shared scratch has an explicit re-entry guard.

Current representative wire shapes:

- `AddConsoleLine`: 20-byte header containing two offset/length pairs and `level`, followed by message and section bytes.
- `CommandNotify`: 24-byte fixed command header followed by packed little-endian `f32` params. Little-endian hosts use one bulk `memcpy`; big-endian hosts encode each float explicitly.

The benchmark guest compiles its 4 KiB scratch only for dedicated variable-callin variants so unrelated Core callin/DrawWorld working-set measurements are not contaminated by permanent scratch memory.

## UI visibility and capability boundary

Component callouts establish `WasmUiVisibility::ScopedContext` before reaching NativeInterface. Core must preserve the same capability boundary.

`HostState` carries the module environment. Generated Core imports use `WasmUiVisibility::ConditionalScopedContext`, which installs a UI perspective only for UI modules and is a literal no-op otherwise. `ScopedContext(false)` is not equivalent to a no-op because it can reset an outer restricted perspective to full-read.

The hand-specialized scalar imports remain registered only for non-UI modules. UI retains the generated visibility-checked definitions rather than shadowing them with legacy scalar fast callbacks. This intentionally keeps the rules/gaia absolute hostcall floor separate from the security-checked UI cost.

`UnitCreated` UI delivery is sanitized before Core callin dispatch: invisible teams suppress the UI event; invisible builders are redacted to `-1`. Add equivalent native-query sanitizers as new Core UI callins are enabled; do not rely only on guest -> host import filtering.

## Benchmark policy

Use `test/native_api_parity/run_benchmarks_core.py` for the established matrix. `test/native_api_parity/run_variable_callins_core.py` is the focused variable-callin runner.

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

Existing fixed callins cover:

- empty
- GameFrame
- Update
- UnitCreated
- UnitPreDamaged
- AllowUnitCreation
- missing/unimplemented export
- four-module fan-out

Other profiles cover heightmap callbacks/regions, realistic workloads, memory behavior and drawing.

The focused variable runner currently records inner `callin_string` / `callin_command` rows. The recorder also reserves `callin_string_event` / `callin_command_event` names for a future identical outer event boundary, but those outer rows are not wired yet. Do not present the inner ratio as strictly apples-to-apples: Lua excludes argument pushing while Core includes scratch lowering.

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
- `rts/WasmInterface/WasmCoreVariableCallins.{h,cpp}`
- `rts/WasmInterface/WasmInterfaceSystemCore.cpp`

Specialized bindings currently cover benchmark-critical unit queries/info, unit definitions, commands/orders, terrain, Gfx, profiling, RulesParams and messaging.

Synced memory is fixed, so its cached Wasmtime base/size is stable. Unsynced memory helpers refresh the current base/size when necessary.

## Verification state

This branch has been edited and statically audited through the GitHub connector, but this session does not have a usable local engine checkout and no branch CI/workflow run has been observed. The current state is therefore **not build-verified**.

Do not manually edit generated benchmark results. Run the release benchmark suite on an otherwise idle machine before drawing performance conclusions.

Immediate verification order:

1. compile after the ordered `std::span` dispatcher conversion;
2. run codegen verification;
3. run the fixed Core callin suite and raw hostcall floor to catch dispatch overhead regressions;
4. run UI visibility/capability tests;
5. run the focused variable-callin benchmark, treating inner rows as diagnostic until outer event rows are wired;
6. rerun cold/warm/64 MiB-trash DrawWorld measurements.

Known carried-over verification items still need retesting in a real build:

- ASAN/CTest have not been run against this Core expansion.
- `test_WasmAllocator` previously had a SIGILL inside JIT guest code.
- `verify_codegen.py` previously reported gaia-synced probe drift predating this work.

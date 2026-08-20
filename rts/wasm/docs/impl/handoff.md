# Handoff: Core Wasm host and API migration

Date: 2026-08-20

Core Wasm is the intended production transport. Component Model implementations remain as correctness and benchmark references, but new API work targets Core first.

## Current branch state

The production Core callin path remains pre-`WasmValue` while using the `WasmInterfaceSystem` module inventory for environment selection, stable module order and result aggregation. The old `WasmCoreHost::DispatchCallin` all-host fan-out remains only as a legacy/benchmark seam; the engine's early typed-host path delegates active Core callins to `WasmInterfaceSystem::DispatchActiveCoreCallin`.

Representative variable engine -> guest callins are implemented:

- `AddConsoleLine`: two strings + scalar level.
- `CommandNotify`: fixed command header + variable `f32[]` params.

They use a guest-owned scratch region negotiated once at bind time and perform one bounded serialization plus one unchecked host -> guest call. The hot path has no host heap allocation. Nested use of the shared scratch is rejected deterministically rather than corrupting the outer payload.

Core host imports carry the module's real `WasmEnvironment`. UI modules stay on generated visibility-checked imports; rules/gaia keep the hand-specialized scalar path so the absolute Core hostcall floor does not pay a visibility-context swap. Unsynced Core faults are removed and the system's Core descriptor inventory is reconciled immediately; synced faults remain sticky.

Known narrow gap: `NativeInterfaceEventClient::DispatchWasmBoolCallin` does not pass the existing `nativeResult` argument into the early typed/Core path. `AddConsoleLine` and `CommandNotify` therefore execute in Core, but a Core `true` return is not yet propagated to the engine. Fix this at the bool-helper/event-client seam; do not introduce global pending-result state to work around it.

Variable-callin benchmark timing is not yet a perfect cross-backend boundary: Lua's central timer starts after Lua arguments are pushed, while Core's variable timer includes scratch lowering. Treat the inner rows as diagnostics/conservative-to-Core until an identical outer event boundary is recorded. No variable-callin performance number is claimed yet.

## Priority and concern taxonomy

Keep these concerns separate. Do not use one category name as shorthand for another.

1. **Performance** — highest practical Wasmtime + Core Wasm performance. Take mechanically clear wins: unchecked typed calls, fewer boundary crossings, no unnecessary allocation, no unnecessary copying, caller-owned/reused buffers, compact layouts and batching where semantics naturally support it. Do not add complicated speculative optimizations that require profiling to justify.
2. **Sync** — synced modules must compute identically across supported platforms. This is deterministic simulation correctness and desync prevention.
3. **Safety** — guest input or host ABI handling must not crash/corrupt the engine. This covers pointer/count validation, integer-overflow checks, guest-memory bounds, trap handling, invalid result validation, re-entry correctness and similar process-integrity concerns. It applies to synced and unsynced modules alike.
4. **Security** — guest code must not escape the Wasm sandbox or gain ambient OS authority such as arbitrary filesystem access, process execution, host networking, environment access or similar capabilities. Expensive optional hardening may be configurable if it measurably hurts the hot path; sandbox escape prevention itself is not optional.
5. **Visibility** — unsynced/UI guests must not see game information hidden from their player/ally-team perspective. This is game-information policy, not sandbox security.

Performance is the first design target, but sync correctness and process safety are correctness constraints rather than optional optimizations. Visibility is enforced where the game API requires it. Security refers only to sandbox/OS authority.

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

For `first`, every module sees the same incoming engine default; the first contributing result is selected without feeding it into later modules as a new input. `UnitPreDamaged` must preserve an incoming `DamageCallinResult` because an earlier engine event client may already have modified damage/impulse values. The ordered dispatcher preserves that default, but the per-module host invocation still needs to be checked so it does not reset it before calling the guest.

Current mixed Core + Component global ordering is not a solved contract. The early Core path behaves as the selected fast transport for currently supported typed callins. Do not claim cross-transport module ordering until the two module inventories are unified or an explicit ordering design is implemented.

## Performance rules

Wasmtime host functions use unchecked typed Core callbacks for reviewed signatures. Keep signatures small and direct. Prefer one typed import/export over generic dispatch-by-name.

Allocation/copy rules:

- Reuse caller-owned guest output buffers on steady-state hot paths.
- Public owned `Vec`/`String` APIs may allocate for ergonomics, but keep `_into`/borrowed forms available for performance-sensitive code.
- Do not allocate merely to translate between equivalent scalar/list representations.
- C-string inputs need NUL termination. `WasmCoreGuestInput.h::GuestCString` keeps short strings in inline storage and uses one heap fallback only for long strings.
- Do not retain guest-memory pointers across guest re-entry or memory growth.
- Do not remove a defensive copy unless the NativeInterface lifetime/re-entry contract proves the pointer is call-scoped.
- The ordered engine -> Core dispatcher is allocation-free on its invocation-list hot path.
- Keep UI visibility work out of rules/gaia hot imports unless the same filter is actually required there.

### `list<string>`

Generic `list<string>` automatic lowering is forbidden for now.

The previous generic shape required a `vector<string>`, a `vector<const char*>`, and one string copy/allocation per element. The Core planner marks any function containing `list<string>` as manual so it cannot enter the executable registry accidentally.

A reviewed representation should use flat guest data: packed bytes plus offset/length descriptors. If the NativeInterface requires `const char**`, construct only the minimum pointer table required by that call; do not allocate one host `string` per element.

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

- direct/fixed;
- fixed option;
- caller-owned variable output.

Reviewed specialized bindings may expose variable-input APIs with a faster ABI and shadow generated bindings.

`spring-api-codegen` writes `core-executable-coverage.json`. Treat that as the executable fast-path coverage report. `core-abi.json` is a broader planning inventory and must not be interpreted as production parity.

## Variable input direction

Benchmark-only imports measure the achievable Core floor for borrowed string and `f32[]` inputs: one unchecked Wasmtime import, normal range/budget validation, no host allocation and no copy.

Production zero-copy input should only be enabled where the NativeInterface contract proves that the pointer is consumed synchronously and cannot be retained or used across re-entry. Until that is proven, conservative copies are a safety requirement even if benchmark ceiling numbers show a lower possible cost.

## Variable callins

Engine -> guest variable callins use one guest-owned scratch region. Do **not** call a guest allocator/reserve export for every event; that would add a second host -> guest crossing.

Implemented protocol:

- Guest optionally exports `spring:callin/scratch-info() -> i64` when it exports a variable callin.
- Low 32 bits: guest-memory offset; high 32 bits: capacity.
- Host resolves/calls it once during module binding and validates the entire range.
- Hot path: host serializes into the cached guest-owned scratch region, then performs exactly one unchecked callin invocation.
- Scratch size is guest/module-selected.
- Oversize payloads currently fail instead of hiding an allocation-heavy slow path.
- Shared scratch has an explicit re-entry guard.

Current representative wire shapes:

- `AddConsoleLine`: 20-byte header containing two offset/length pairs and `level`, followed by message and section bytes.
- `CommandNotify`: 24-byte fixed command header followed by packed little-endian `f32` params. Little-endian hosts use one bulk `memcpy`; big-endian hosts encode each float explicitly.

The benchmark guest compiles its 4 KiB scratch only for dedicated variable-callin variants so unrelated Core callin/DrawWorld working-set measurements are not contaminated by permanent scratch memory.

## Visibility policy

Visibility is independent of sandbox security.

Component callouts establish `WasmUiVisibility::ScopedContext` before reaching NativeInterface. Core must preserve the same game-information boundary.

`HostState` carries the module environment. Generated Core imports use `WasmUiVisibility::ConditionalScopedContext`, which installs a UI perspective only for UI modules and is a literal no-op otherwise. `ScopedContext(false)` is not equivalent to a no-op because it can reset an outer restricted perspective to full-read.

The hand-specialized scalar imports remain registered only for non-UI modules. UI retains the generated visibility-checked definitions rather than shadowing them with legacy scalar fast callbacks. This intentionally keeps the rules/gaia absolute hostcall floor separate from the visibility-filtered UI cost.

`UnitCreated` UI delivery is sanitized before Core callin dispatch: invisible teams suppress the UI event; invisible builders are redacted to `-1`. Add equivalent native-query sanitizers as new Core UI callins are enabled; do not rely only on guest -> host import filtering.

## Sync policy

Synced Core modules remain restricted:

- deterministic host imports only;
- no ambient clock/random/network/filesystem/process APIs;
- no threads/shared memory;
- fixed non-growable memory (`max == min`);
- no relaxed SIMD unless deterministic semantics are explicitly established;
- deterministic floating-point configuration where cross-platform behavior requires it.

If execution interruption becomes simulation-visible, deterministic fuel is preferable to timing-dependent epoch interruption. Do not enable fuel globally merely as sandbox hardening without measuring its execution cost.

## Safety policy

Safety means preventing guest/ABI misuse from crashing or corrupting the engine.

Keep:

- guest pointer/count overflow checks;
- linear-memory bounds checks;
- exact import/export signature validation before unchecked calls;
- result/status validation after unchecked calls;
- re-entry guards for shared scratch/callback state;
- validation/copy-before-mutation when an input lifetime is not proven;
- traps converted into module faults rather than unchecked host failure.

These are not "security" or "visibility" features. They are process correctness requirements.

## Security policy

Security means preventing guest escape into arbitrary OS authority.

Default Core guests should have no WASI and no imports granting arbitrary:

- filesystem access;
- process creation/command execution;
- raw host networking;
- environment-variable access;
- host clocks/randomness except deliberately designed APIs;
- native-library loading or arbitrary FFI.

Use a strict import allow-list and rely on Wasmtime's Wasm sandbox for memory/control-flow isolation. If an additional defense adds meaningful recurring hot-path cost, benchmark it and make the hardening profile explicit/configurable where appropriate rather than conflating it with safety or visibility.

## Benchmark policy

Use `test/native_api_parity/run_benchmarks_core.py` for the established matrix. `test/native_api_parity/run_variable_callins_core.py` is the focused variable-callin runner.

Raw comparable backends remain:

- Lua;
- native;
- dynamic Component Model;
- typed Rust Component Model;
- Core Wasm.

Decision ratios are only:

- Lua vs native;
- Lua vs Core;
- Core vs native;
- dynamic Component Model vs Core.

Do not add Typed-vs-Core decision ratios. The typed CM raw measurement remains for reference.

### Representative comparable API rows

Callouts should cover distinct ABI shapes rather than many near-duplicate APIs:

- direct scalar;
- fixed vector/record;
- string result;
- flat small list;
- large/reused list;
- nested list/record payload;
- spatial list query;
- mutating string/scalar call;
- callback/re-entry path.

Callins should cover:

- empty;
- scalar (`GameFrame`/`Update`);
- fixed multi-field record (`UnitCreated`);
- fixed result (`UnitPreDamaged`, `AllowUnitCreation`);
- variable string payload (`AddConsoleLine`);
- fixed record + variable `f32[]` (`CommandNotify`);
- missing export;
- multi-module fan-out;
- cold/warm `DrawWorld`.

Other profiles cover heightmap callbacks/regions, realistic workloads, memory behavior and drawing.

The focused variable runner currently records inner `callin_string` / `callin_command` rows. The recorder reserves `callin_string_event` / `callin_command_event` names for a future identical outer event boundary, but those outer rows are not wired yet. Do not present the inner ratio as strictly apples-to-apples: Lua excludes argument pushing while Core includes scratch lowering.

### Core transport-ceiling rows

`core_ceiling_*` rows are intentionally excluded from cross-backend validation/ratios because they measure optimized transport floors rather than identical high-level APIs.

Current ceiling samples include fixed structs, borrowed string/f32-list inputs, reusable string/list/nested-list outputs and reusable spatial-list outputs.

These rows answer how close the implementation can get to the practical Core boundary floor when allocation/probing is amortized. Add new rows only for materially different ABI shapes.

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

Specialized bindings cover benchmark-critical unit queries/info, unit definitions, commands/orders, terrain, Gfx, profiling, RulesParams and messaging.

Synced memory is fixed, so its cached Wasmtime base/size is stable. Unsynced memory helpers refresh the current base/size when necessary.

## Verification state

This branch has been edited and statically audited through the GitHub connector, but this session does not have a usable local engine checkout and no branch CI/workflow run has been observed. The current state is therefore **not build-verified**.

Do not manually edit generated benchmark results. Run the release benchmark suite on an otherwise idle machine before drawing performance conclusions.

Immediate verification order:

1. compile after the ordered `std::span` dispatcher conversion;
2. run codegen verification;
3. run the fixed Core callin suite and raw hostcall floor to catch dispatch overhead regressions;
4. run UI visibility tests separately from sandbox/security tests;
5. run process-safety/invalid-pointer/trap tests;
6. run synced cross-platform determinism/hash tests;
7. run the focused variable-callin benchmark, treating inner rows as diagnostic until outer event rows are wired;
8. rerun cold/warm/64 MiB-trash DrawWorld measurements.

Known carried-over verification items still need retesting in a real build:

- ASAN/CTest have not been run against this Core expansion.
- `test_WasmAllocator` previously had a SIGILL inside JIT guest code.
- `verify_codegen.py` previously reported gaia-synced probe drift predating this work.

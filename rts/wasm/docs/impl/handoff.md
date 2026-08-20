# Handoff: Core Wasm production transport

Date: 2026-08-20

This checkout contains a large Core WebAssembly implementation on top of `rust-wip`. Treat `rust-wip` as the baseline when you need to understand what is new here. The merge base at handoff time was `47c940718231af9659ac23a6d7b960dcd8acf545`; this checkout was 375 commits ahead of that baseline.

The intended production direction is Wasmtime + Core WebAssembly. The existing Component Model path remains useful as a correctness/reference implementation and benchmark comparison, but new performance-sensitive work should target Core first.

The next agent is expected to work locally: compile, regenerate code, run tests, inspect generated diffs, benchmark, and fix anything this web-only development session could not verify.

## 1. Design priorities

Keep these concerns separate; do not use one as shorthand for another.

1. **Performance** — highest practical Wasmtime/Core performance with mechanically defensible optimizations: typed unchecked calls after validation, minimal crossings, no unnecessary allocation/copying, caller-owned buffers, compact deterministic layouts, and reuse where semantics allow it.
2. **Sync** — synced simulation must remain deterministic across supported platforms. Nondeterministic host data must not be visible to synced guests.
3. **Safety** — malformed/trapping guest code must not crash or corrupt the engine. Pointer/count validation, bounds checks, overflow checks, signature validation, result validation, re-entry protection, and fault handling are correctness requirements.
4. **Security** — guests must not escape the Wasm sandbox or gain ambient OS authority. No arbitrary filesystem, process execution, networking, environment, native FFI, or equivalent authority by default.
5. **Visibility** — unsynced/UI guests must not observe hidden game state outside the correct player/ally-team perspective. This is a game-information policy, separate from sandbox security.

Performance is the first optimization target, but sync and safety are hard correctness constraints. Visibility must be preserved where the NativeInterface API requires it.

## 2. What this checkout adds on top of `rust-wip`

The delta is not a small experiment. It adds a complete Core-Wasm runtime direction, code generation, SDK work, specialized production bindings, callin dispatch, validation, benchmarking, and documentation.

### 2.1 Core runtime and ABI infrastructure

New runtime pieces under `rts/WasmInterface/` include:

- `WasmCoreAbi.{h,cpp}` — Wasmtime raw ABI helpers, memory binding, signature checking, unchecked export invocation, packed result helpers.
- `WasmCoreBindings.{h,cpp}` — instance import registration and fixed callin export binding.
- `WasmCoreHost.{h,cpp}` — Core module host/runtime ownership.
- `WasmCoreHostFastDispatch.cpp` — fast host-dispatch support.
- `WasmCoreValidation.{h,cpp}` — Core import/export validation against the executable registry and environment policy.
- `WasmCoreRegistry.h` — handwritten executable Core import registry plus optional generated registry fallback.
- `WasmCoreGeneratedSupport.h` — common import guards, execution-budget handling, UI visibility context, memory resolution, callback support.
- `WasmCoreGuestInput.h` — validated guest string/input helpers; short C-string inputs use inline storage with heap fallback only for long strings.
- `WasmCoreWire.h` — deterministic little-endian record reader/writer; native C++ struct padding is never part of the Wasm ABI.
- `WasmCoreVariableCallins.{h,cpp}` — guest-owned scratch protocol for variable engine->guest callins.
- `WasmInterfaceSystemCore.cpp` — ordered Core module dispatch and callin aggregation integrated with the existing module system.

`WasmInterfaceSystem`, `WasmTypedHost`, runtime/resource plumbing, event dispatch, and CMake were modified so Core is a real engine transport rather than an isolated benchmark host.

### 2.2 Production dispatch before `WasmValue`

Supported Core callins are dispatched before the historical Component/`WasmValue` serialization path.

`NativeInterfaceEventClient::DispatchWasmCallin` now:

- attempts `WasmInterfaceSystem::DispatchActiveCoreCallin` first;
- propagates direct native result structs from Core/typed transports;
- only falls through to Component serialization when Core/typed did not handle the call;
- tests `HasComponentModules(...)` before paying Component query serialization costs, rather than generic `HasModules(...)` which also includes Core modules.

This removed accidental Core-only serialization/copy overhead.

The bool-result seam is fixed. `DispatchWasmBoolCallin` supplies a `BoolCallinResult`, recognizes the direct-result `WasmValue::Unit()` marker, propagates Core `true` results, and retains the old Component record path as fallback. `AddConsoleLine` and `CommandNotify` therefore contribute their boolean results correctly.

`AllowUnitCreation` and `UnitPreDamaged` also use explicit direct-result handling instead of inferring transport type from whether the old typed host is enabled.

Legacy native callback-null checks that accidentally prevented Wasm-only implementations from running were removed for:

- `AllowUnitTransportLoad`
- `AllowUnitTransportUnload`
- `AllowUnitDecloak`

### 2.3 Core callin set

The currently implemented representative callins are:

Synced side:

- `GameFrame`
- `GameFramePost`
- `UnitCreated`
- `UnitPreDamaged`
- `AllowUnitCreation`

Unsynced/UI side:

- `Update`
- `AddConsoleLine`
- `CommandNotify`
- `DrawWorld`

The dispatcher preserves environment masks, stable module order, fault policy, and generated aggregation semantics. Boolean `or-true` / `and-false`, `ignore`, and first-contributor result aggregation are implemented. For first-contributor callins, every module sees the same incoming engine default; later modules do not receive an earlier module's result as a new input.

Core faults are handled differently by environment intentionally: unsynced faulted modules are removed/reconciled from active Core inventory; synced faults remain sticky rather than silently changing deterministic module participation.

Mixed Core + Component module ordering is **not** a globally specified inter-transport ordering contract. For supported callins, Core currently acts as the selected fast transport and returns before Component dispatch when handled. Do not claim arbitrary mixed-transport stable ordering without designing it explicitly.

### 2.4 Variable callins: one crossing, guest-owned scratch

`AddConsoleLine` and `CommandNotify` are the representative variable engine->guest callins.

Protocol:

- A guest that exports a variable callin optionally exports `spring:callin/scratch-info() -> i64`.
- Low 32 bits are guest memory offset; high 32 bits are capacity.
- Host resolves/calls this once during bind and validates the full range.
- Hot path serializes into cached guest-owned scratch and performs exactly one unchecked host->guest call.
- No guest allocator/reserve call is made per event.
- Oversize payloads fail rather than hiding an allocation-heavy slow path.
- Shared scratch has an explicit nested/re-entry guard.

Representative payloads:

- `AddConsoleLine`: fixed header containing two offset/length pairs plus level, followed by message/section bytes.
- `CommandNotify`: fixed command header followed by packed little-endian `f32` parameters.

On little-endian hosts the float list can use a bulk copy; big-endian hosts must encode explicitly.

### 2.5 Identical outer variable-callin benchmark boundary

The old handoff warning about incomparable variable-callin timings is obsolete.

`CEventHandler::AddConsoleLine` and `CEventHandler::CommandNotify` now record outer event timings:

- `callin_string_event`
- `callin_command_event`

`test/native_api_parity/run_variable_callins_core.py` runs both Lua and Core with four rows:

- inner `callin_string`
- inner `callin_command`
- outer `callin_string_event`
- outer `callin_command_event`

The outer rows start immediately before the common `CEventHandler` event-client dispatch and end immediately after it. Those rows are the valid Lua/Core decision comparison and include backend-specific lowering on both paths.

The inner rows remain diagnostic only because Lua begins after pushing arguments while Core includes scratch lowering.

### 2.6 Memory and wire conventions

Use these conventions consistently:

- Scalars, IDs, small enums/flags: direct Wasm numeric arguments/results where possible.
- Packed scalar result + error: low 32 bits value/bit pattern, high 32 bits status/error where the existing helper convention applies.
- Fixed records/arrays: explicit little-endian wire layout into caller-owned guest memory; never `memcpy` native structs across the boundary.
- `list<i32>` / `list<f32>` results: guest-owned output + capacity; return full required count on overflow; never partially fill a logical result.
- Variable string results: caller-owned buffers with written/required lengths.
- `list<string>`: reviewed flat descriptor table + packed byte blob, not `vector<string>` per element.
- Borrowed variable inputs: only where NativeInterface consumes the data synchronously and cannot retain the pointer across re-entry.

Synced Core memory is fixed/non-growable, so a cached Wasmtime base/size is stable. Unsynced memory helpers refresh base/size when needed.

Do not retain guest-memory pointers across guest re-entry or growth.

### 2.7 Specialized production callouts

Hand-reviewed bindings exist for important APIs where generic lowering is missing, too expensive, or insufficiently proven.

#### Units information / queries

Existing scalar/fixed fast coverage includes representative `UnitsInfo` and `UnitsQuery` reads such as unit def/team/dead/experience, position/velocity/health, validity/counts, nearest-unit queries, separation, and spatial list queries.

Caller-owned list output coverage includes:

- `GetAllUnits`
- `GetTeamUnits`
- rectangle/box/sphere/cylinder spatial queries

Borrowed zero-copy list-input coverage was added for:

- `GetTeamUnitsByDefs`
- `GetUnitArrayCentroid`
- `GetUnitMapCentroid`

These borrow aligned guest `i32` memory directly on supported little-endian hosts and avoid materializing a host vector. Big-endian hosts return `NotAvailable` rather than silently interpreting Wasm little-endian memory incorrectly.

#### Unit definitions

Reviewed string-result bindings exist for:

- `GetUnitDefName`
- `GetUnitDefHumanName`

They use caller-owned buffers rather than per-call owned host transport values.

#### Unit commands / control

Core bindings include command count/queue access and synced order mutation paths. Reviewed order imports borrow fixed guest arrays where lifetime semantics permit it.

Important order paths include:

- `GiveOrder`
- `GiveOrderToUnitMap`
- `GiveOrderToUnit`

Synced mutating imports remain synced-only.

#### Unit pieces

`GetUnitScriptNames` uses the reviewed flat `list<string>` ABI: descriptor table + packed bytes + required-count/required-byte reporting.

The Core implementation was optimized to read stable model-owned piece names directly (`unit->script->pieces[index]->original->name`) rather than calling the NativeInterface path that first copied names through its 1 KiB thread-local scratch buffer. This removes a redundant copy and second string-length pass for Core while leaving the NativeInterface implementation unchanged for other transports.

#### Terrain control

Reviewed synced-only terrain mutation bindings include:

- `SetHeightMap`
- `LevelHeightMap`
- `SetHeightMapFunc` callback path

#### Terrain reads

A checked-in handwritten Core Terrain read group now covers 11 fixed/read APIs directly rather than relying only on generated snapshots:

- `IsPosInMap`
- `GetGroundHeight`
- `GetGroundOrigHeight`
- `GetSmoothMeshHeight`
- `GetWaterPlaneLevel`
- `GetWaterLevel`
- `GetGroundNormal`
- `GetGroundExtremes`
- `GetHeightMapSize`
- `GetGroundBlocked`
- `GetGrass`

Scalar values use packed results; multi-value outputs use fixed caller-owned buffers.

#### Gfx

Reviewed Core Gfx coverage includes benchmark-critical immediate paths such as `Vertex` and `BeginEnd`. These are unsynced/UI APIs.

#### Profiling

The handwritten Core profiling group now covers fixed forms of:

- `GetTimer`
- `GetTimerMicros`
- `DiffTimers`
- `GetFrameTimer`
- `GetDrawSeconds`
- `GetLuaMemUsage`
- `GetVidMemUsage`
- `GetSyncedGCInfo`

A sync bug was corrected in the Core executable registry: profiling/timer imports are unsynced/UI-only. `GetTimerMicros` calls `spring_now()` and must not be visible to synced guests even though the semantic generated inventory historically marked it all-environment.

#### Messages

A complete reviewed handwritten Messages group contains 18 imports, including:

- echo/logging
- general/player/team/ally-team/spectator messages
- public/ally/spectator/private chat
- engine commands
- Lua menu/UI/Gaia/Rules messaging
- skirmish-AI messaging
- `SendToUnsynced`

String inputs are validated guest `(ptr,len)` values and converted to call-scoped NUL-terminated storage for the NativeInterface C-string contract. Short strings stay inline; long strings may use one fallback allocation. `SendToUnsynced` retains its synced-only semantic policy.

The Rust SDK has matching `no_std` borrowed-`&str` wrappers for this group.

#### Rules parameters

Reviewed specialized f32-only RulesParams bindings exist for:

- `SetUnitRulesParam` float value form
- `GetUnitRulesParam` float value form

The purpose is to expose the common scalar case without dragging the full tagged/string RulesParam union through the hot Core ABI.

Setter is synced-only. Getter is available according to normal read policy. Missing parameter returns `NotFound`; non-float value returns `InvalidArgument`; NativeInterface errors are preserved.

#### Config / benchmark imports

Config includes the reviewed flat `GetLogSections` string-list ABI. Benchmark-only imports and transport-ceiling paths exist to measure raw Core shapes independently of semantic parity rows.

### 2.8 Current half-finished UnitsInfo variable batch

This is the immediate incomplete implementation at handoff time.

Host callbacks are implemented in:

- `rts/WasmInterface/WasmCoreUnitsInfoVariableBindings.{h,cpp}`

They are compiled and registered through:

- `rts/WasmInterface/CMakeLists.txt`
- `rts/WasmInterface/WasmCoreBindings.h`

Implemented callbacks:

1. `get-unit-nano-pieces(unit_id, output, capacity) -> i64`
   - caller-owned `i32` list output;
   - standard packed `(count,status)` return.

2. `get-unit-is-transporting(unit_id, output, capacity, state_output) -> i64`
   - caller-owned transported-unit `i32` list;
   - separate 4-byte little-endian boolean output preserves NativeInterface `isTransporting` exactly instead of deriving it from `count != 0`;
   - standard packed `(count,status)` return.

**Still missing:**

- add both signatures to the handwritten executable import registry (`WasmCoreRegistry.h`) with the correct environment mask;
- add safe Rust guest wrappers, preferably in a small `units_info_variable.rs` module and re-export it from `lib.rs`;
- decide the wrapper result shape for `GetUnitIsTransporting` so the explicit boolean and `BufferFill`/count status are both preserved;
- compile/test this batch locally.

Do this before starting another API family.

## 3. Rust Core guest SDK

A new workspace crate exists at:

`rust/crates/spring-wasm-core/`

It is `no_std` first and exposes safe wrappers over the raw Core ABI. Unsafe pointer conversion is contained at the transport boundary.

Current handwritten SDK modules include:

- `benchmark.rs`
- `config.rs`
- `messages.rs`
- `profiling.rs`
- `rules_params.rs`
- `terrain.rs`
- `unit_control.rs`
- `unit_defs.rs`
- `units_commands.rs`
- `units_pieces.rs`
- `units_query.rs`
- `units_query_borrowed.rs`

`lib.rs` also contains the basic UnitsInfo scalar/fixed wrappers and callin export macros.

Public ergonomic allocating APIs may exist behind `alloc`, but performance-sensitive APIs should retain `_into`/borrowed forms so callers can reuse memory.

There is also a C Core SDK header at:

`rts/wasm/sdk/core/spring_wasm_core.h`

Rust is the first-class guest SDK direction, but the built engine and end users must not require Rust merely to run Core modules.

## 4. Core code generation

`spring-native-codegen` gained a Core ABI generation pipeline.

Important files:

- `render_core_wasm.rs` — Core planning/model inventory.
- `render_core_wasm_callins.rs` — Core callin planning.
- `render_core_wasm_host.rs` — direct/fixed host bindings.
- `render_core_wasm_option_host.rs` — fixed `option<T>` bindings.
- `render_core_wasm_variable_host.rs` — variable-input scaffolding.
- `render_core_wasm_variable_output_host.rs` — caller-owned variable-output generation.
- `render_core_wasm_variable_io_host.rs` — combined variable-I/O scaffolding.
- `render_core_wasm_registry.rs` — executable Core registry and coverage report.
- `render_core_wasm_guest.rs` — generated Rust guest bindings.

`spring-api-codegen` now emits, among other files:

- `core-abi.json`
- `core-callin-plan.json`
- `core-executable-coverage.json`
- `WasmCoreAbiInventory.h`
- `WasmCoreGeneratedRegistry.h`
- generated Core host binding translation units/headers
- generated Rust `core_generated.rs`

Production policy is deliberately narrower than the broad ABI plan:

- direct/fixed lowering may be executable;
- fixed option lowering may be executable;
- caller-owned variable output may be executable;
- allocation-heavy generated variable-input/combined-I/O paths remain implementation scaffolding and must not be advertised as the production fast path merely because code can be generated.

Reviewed handwritten bindings may shadow generated definitions when they implement a materially faster/safer ABI.

### Important generated-artifact state

At this handoff, the repository's checked-in `rts/wasm/generated/` directory does **not** contain the new `WasmCoreGenerated*` artifacts or `core-abi.json`/Core coverage files even though the generator emits them and `verify_codegen.py` compares the complete generated output set.

This web session could not run the generator locally. A desktop agent should resolve this immediately rather than assuming generated Core coverage exists in the build.

Recommended local sequence:

```sh
cargo run --manifest-path rust/Cargo.toml \
  -p spring-native-codegen \
  --bin spring-api-codegen -- \
  --root . \
  --output rts/wasm/generated \
  --strict

git diff -- rts/wasm/generated
python3 rts/wasm/verify_codegen.py
```

Inspect the generated diff before committing it. The generated executable registry must agree with what is actually compiled and safe to expose.

CMake conditionally compiles generated direct/fixed, option, and variable-output Core translation units only when those generated files exist. Without checked-in/regenerated files, handwritten coverage is what the local build actually gets.

## 5. Visibility, sync, safety and security behavior

### Visibility

`HostState` carries the real `WasmEnvironment`.

`generated::ImportGuard` uses `WasmUiVisibility::ConditionalScopedContext`:

- UI modules install the correct restricted perspective;
- rules/gaia pay no unnecessary visibility-context save/restore cost.

The oldest specialized scalar `RegisterFastImports` path is intentionally skipped for UI, so it cannot shadow visibility-checked UI definitions. Other reviewed specialized bindings use `ImportGuard`, so UI visibility policy is applied there.

`UnitCreated` UI delivery is sanitized before guest dispatch: hidden teams suppress the event and hidden builder IDs are redacted to `-1`.

When adding new UI-visible Core APIs, compare them against the Component/NativeInterface visibility behavior explicitly. Do not assume import-level filtering alone is enough for engine->guest callins.

### Sync

Synced Core policy includes:

- deterministic imports only;
- fixed non-growable memory (`max == min`);
- no ambient clocks/random/network/filesystem/process authority;
- no threads/shared memory;
- no relaxed SIMD unless deterministic behavior is deliberately established;
- deterministic floating-point configuration where cross-platform behavior requires it.

The profiling timer correction described above is an example of enforcing this policy even when a broader generated semantic mask is wrong.

If execution interruption becomes simulation-visible, prefer deterministic fuel over timing-dependent epoch interruption. Measure fuel cost before enabling it globally.

### Safety

Maintain:

- exact import/export signature checks before unchecked Wasmtime calls;
- pointer/count multiplication overflow checks;
- guest linear-memory bounds checks;
- deterministic little-endian decoding;
- result/status validation;
- re-entry protection for shared scratch/callback state;
- copy-before-mutation where NativeInterface lifetime is not proven;
- traps converted to module faults rather than host process failure.

### Security

Core guests should have no WASI or ambient OS capability by default. Imports are an allow-list of engine capabilities. Do not add general filesystem/process/network/environment/native-FFI access.

## 6. Benchmark and test infrastructure added

Major additions include:

- `test/native_api_parity/run_benchmarks_core.py`
- `test/native_api_parity/run_variable_callins_core.py`
- `test/native_api_parity/CORE_WASM_BENCHMARKS.md`
- Core benchmark guest crates under `test/wasm_api/`
- benchmark support in `rts/System/BenchmarkCallins.h`
- common outer event timing in `rts/System/EventHandler.cpp`

The benchmark matrix keeps raw measurements for:

- Lua
- native
- dynamic Component Model
- typed Rust Component Model
- Core Wasm

Decision ratios are intended for Lua/native/Core/dynamic-CM comparisons. Typed-CM remains a raw reference; do not manufacture a Typed-vs-Core product decision ratio simply because both values exist.

`core_ceiling_*` rows measure the optimized transport floor for distinct ABI shapes and are intentionally not semantic cross-backend parity rows.

Representative shapes already covered include scalar/fixed calls, string/list outputs, reused large lists, nested outputs, spatial queries, borrowed inputs, variable callins, callbacks, draw/cold-warm behavior, and memory-pressure variants.

Do not update generated benchmark result documents manually. Run the runner on an otherwise idle release build.

## 7. Local verification state

The most important fact for takeover: **the latest Core expansion has not been compiled or run in this web session.** GitHub reported no status checks for the current head. Treat source review as useful but not as build verification.

Run local verification before broadening the API further.

### First-pass Rust/codegen checks

```sh
cargo fmt --manifest-path rust/Cargo.toml --all --check
cargo test --manifest-path rust/Cargo.toml --workspace

cargo run --manifest-path rust/Cargo.toml \
  -p spring-native-codegen \
  --bin spring-api-codegen -- \
  --root . \
  --output rts/wasm/generated \
  --strict

python3 rts/wasm/verify_codegen.py
```

### Engine/ASAN checks

The repository's Wasm CI uses:

```sh
./docker-build-v2/build.sh linux -DUSE_ASAN=ON
./docker-build-v2/build.sh --compile linux -t check
```

Use the equivalent local build if a faster native build workflow is already configured, but make sure the new C++ files are actually compiled with Wasmtime enabled.

### Guest/parity checks

Follow `.github/workflows/wasm.yml` as the canonical current CI recipe. It includes:

- Rust workspace tests/formatting;
- generated artifact verification;
- ASAN engine build/tests;
- guest fixture tests;
- native parity harness;
- Wasm parity harness;
- release performance configuration and `wasm-performance` target.

For the focused variable callins, run:

```sh
python3 test/native_api_parity/run_variable_callins_core.py \
  --spring-headless /path/to/spring-headless
```

Use the outer event rows for Lua/Core decisions. Keep the inner rows diagnostic.

### Historical failures to re-check, not blindly inherit

Earlier local/CI work before the latest web edits had mentioned:

- a `test_WasmAllocator` SIGILL inside JIT guest code;
- gaia-synced probe/codegen drift.

These may predate or be unrelated to the current changes. Reproduce them locally before treating them as current bugs.

## 8. Immediate takeover order

Do this in order unless a compiler failure forces a smaller detour.

1. **Finish the current UnitsInfo variable batch**: registry signatures + Rust wrappers for nano pieces and transporting units.
2. **Regenerate Core artifacts** into `rts/wasm/generated`, inspect the full diff, and make `verify_codegen.py` pass.
3. **Compile the entire Rust workspace and engine with Wasmtime enabled.** Fix compiler/linker errors before adding API coverage.
4. **Run the existing Wasm/ASAN/parity tests.** Separate sync, safety, security and UI-visibility failures when diagnosing them.
5. **Run the focused variable-callin benchmark** and confirm the outer event rows behave as expected.
6. **Run representative Core callout benchmarks**, especially raw fixed/scalar floor, reused list outputs, borrowed inputs, and DrawWorld cold/warm variants.
7. Only after the above is green, continue expanding reviewed Core API coverage.

## 9. How to choose the next API work

Prefer API families in this order:

- fixed/scalar APIs that generic Core codegen can safely own;
- caller-owned simple list outputs;
- borrowed simple list inputs with proven synchronous NativeInterface lifetime;
- flat string/list-string representations with explicit reviewed layouts;
- callback/re-entry APIs only with explicit lifetime and nesting behavior;
- complex nested variable I/O last.

Do not port APIs merely to increase a coverage percentage. The production Core registry should mean "reviewed executable fast path", not "the generator emitted something".

When a generated implementation allocates/copies unnecessarily, either improve the generator for the whole ABI class or use a narrow handwritten binding that makes the performance/lifetime contract explicit.

## 10. Key source map

Core runtime / dispatch:

- `rts/WasmInterface/WasmCoreAbi.{h,cpp}`
- `rts/WasmInterface/WasmCoreBindings.{h,cpp}`
- `rts/WasmInterface/WasmCoreHost.{h,cpp}`
- `rts/WasmInterface/WasmCoreHostFastDispatch.cpp`
- `rts/WasmInterface/WasmCoreValidation.{h,cpp}`
- `rts/WasmInterface/WasmCoreRegistry.h`
- `rts/WasmInterface/WasmInterfaceSystemCore.cpp`
- `rts/NativeInterface/NativeInterfaceEventClient.cpp`

ABI/support:

- `rts/WasmInterface/WasmCoreGeneratedSupport.h`
- `rts/WasmInterface/WasmCoreGuestInput.h`
- `rts/WasmInterface/WasmCoreWire.h`
- `rts/WasmInterface/WasmCoreVariableCallins.{h,cpp}`

Specialized bindings:

- `WasmCoreUnitsQueryBindings.cpp`
- `WasmCoreUnitsQueryBorrowedBindings.cpp`
- `WasmCoreUnitsInfoVariableBindings.cpp`
- `WasmCoreUnitDefsBindings.cpp`
- `WasmCoreUnitsCommandsBindings.cpp`
- `WasmCoreUnitsPiecesBindings.cpp`
- `WasmCoreUnitControlBindings.cpp`
- `WasmCoreTerrainControlBindings.cpp`
- `WasmCoreTerrainReadBindings.cpp`
- `WasmCoreGfxBindings.cpp`
- `WasmCoreProfilingBindings.cpp`
- `WasmCoreMessagesBindings.cpp`
- `WasmCoreRulesParamsBindings.cpp`
- `WasmCoreConfigBindings.cpp`
- `WasmCoreBenchmarkBindings.cpp`

Codegen:

- `rust/crates/spring-native-codegen/src/render_core_wasm*.rs`
- `rust/crates/spring-native-codegen/src/bin/spring-api-codegen.rs`

Guest SDK:

- `rust/crates/spring-wasm-core/`
- `rts/wasm/sdk/core/spring_wasm_core.h`

Benchmarks/tests:

- `test/native_api_parity/run_benchmarks_core.py`
- `test/native_api_parity/run_variable_callins_core.py`
- `test/wasm_api/core_benchmark_guest/`
- `test/wasm_api/core_benchmark_suite_guest/`
- `.github/workflows/wasm.yml`

Design/reference docs:

- `rts/wasm/docs/impl/core_abi_contract.md`
- `rts/wasm/docs/impl/core_fast_abi.md`
- `rts/wasm/docs/impl/core_benchmark_matrix.md`

## 11. Invariants not to regress

- Core supported callins run before `WasmValue` serialization.
- Do not reintroduce per-call heap allocation into ordered Core callin fan-out.
- Do not perform an extra guest allocator call for variable callins.
- Do not use native C++ struct layout as a wire ABI.
- Do not expose nondeterministic profiling/timers to synced guests.
- Do not bypass UI visibility filtering for speed.
- Do not treat visibility as sandbox security or safety as security.
- Do not advertise allocation-heavy generated variable-input scaffolding as production-fast coverage.
- Do not partially fill list results when the caller buffer is too small; report required size and retry.
- Do not replace the explicit `GetUnitIsTransporting` boolean with `count != 0`.
- Do not restore the redundant NativeInterface scratch copy for Core `GetUnitScriptNames`.
- Do not publish variable-callin inner-timing ratios as apples-to-apples; use the common outer event rows.
- Do not claim the current checkout is verified until it has been regenerated, compiled and tested locally.

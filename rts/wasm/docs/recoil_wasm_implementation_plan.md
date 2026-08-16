# Recoil Wasm Implementation Plan

This plan references the split design documents rather than repeating their detailed rationale.

- [Architecture](recoil_wasm_architecture.md)
- [API & Code Generation](recoil_wasm_api_codegen.md)
- [Execution Environments](recoil_wasm_execution_environments.md)
- [Runtime & ABI](recoil_wasm_runtime_abi.md)
- [Sandbox & Determinism](recoil_wasm_sandbox_determinism.md)
- [Parity & Testing](recoil_wasm_parity_testing.md)

## Proposed source layout

Names are illustrative but reflect the intended ownership.

```text
rts/
  NativeInterface/
    api/
      ... existing headers ...
      Callins.def                 # canonical callin inventory
    codegen/
      ... existing Lua/native generation ...

  WasmInterface/
    WasmRuntime.{h,cpp}
    WasmModule.{h,cpp}
    WasmInterfaceSystem.{h,cpp}
    WasmEnvironment.{h,cpp}
    WasmDispatch.{h,cpp}
    WasmResources.{h,cpp}
    generated/
      ... generated host adapters/registration ...

rust/
  crates/
    spring-api-codegen/
      src/
        model.rs
        clang.rs
        semantics.rs
        annotations.rs
        callins.rs
        render_native.rs
        render_wit.rs
        render_wasm_sdk.rs
        render_signatures.rs

    spring-native/
      generated/
      ...

    spring-wasm/
      ...

    spring-module/                # optional shared façade/re-exports

test/
  wasm_api/
    ...
  native_api_parity/
    ... existing harness ...
```

## Phase 0 — Stabilize the existing generator/tests

Before refactoring the generator:

1. commit a deterministic snapshot of current generated native Rust;
2. make normal `spring-native` builds consume the snapshot rather than requiring libclang;
3. add an explicit regeneration command/job;
4. add synthetic-header generator tests for:
   - scalar;
   - string;
   - list/pointer-count;
   - optional;
   - nested record;
   - enum;
   - fixed array;
   - callback;
   - handle;
5. change silent renderer omission into a hard error with explicit exclusions;
6. regenerate/diff in CI;
7. extend the existing parity fixture to two ally teams/LOS/radar states.

Deliverable: generator refactoring has a trustworthy baseline.

## Phase 1 — Shared semantic API model

Rename/generalize `spring-native-codegen` into `spring-api-codegen`.

Split structural C parsing from semantic interpretation.

Implement model types for:

- primitives/enums/records;
- strings/bytes/lists;
- options/results/errors;
- handles/resources;
- callbacks;
- mutation marker;
- execution-environment applicability;
- visibility-sensitive marker for UI-role parity;
- interface/version identity;
- manual/unsupported shapes.

Add adjacent C annotations for relationships that are unsafe to infer from field order.

Create the canonical callin inventory (`Callins.def` or equivalent).

Model the initial environments:

- Rules synced;
- Rules unsynced;
- Gaia synced;
- Gaia unsynced;
- UI (described but initially disabled).

Generate a report of automatic/annotated/manual/unsupported functions.

Deliverable: existing native Rust generation runs through the new semantic model.

## Phase 2 — Wasmtime/Component Model proof

Build a small C++ runtime fixture before generating the full API.

Reuse the existing `native_api_parity` launch approach: game/map/script + `spring-headless`. Keep the Wasm fixture under `test/wasm_api/`.

### Runtime spike

Pin one Wasmtime release and prove:

- component imports from C++;
- strings/lists/records/options/results/enums;
- host resources/drop;
- environment-specific worlds;
- missing-interface diagnostics;
- version compatibility behavior;
- fuel/memory/resource limiting;
- canonical allocator behavior;
- callback/re-entry.

### End-to-end API slice

Hand-wire enough functions to cover the semantic patterns:

- scalar read;
- string result;
- list result;
- nested record;
- error;
- handle/resource;
- one synced mutation;
- one callin;
- one synchronous callback.

Include adversarial cases:

- allocator attempts a nested Spring call;
- allocator traps/burns fuel;
- oversized result;
- invalid handle;
- callback re-entry;
- `CallCOBScript` result bound failure before mutation.

Instantiate separate synced and unsynced modules and prove mutable state is not shared.

### Performance gate

Benchmark:

- scalar callout;
- list/string result;
- callin;
- callback;
- representative `Gfx`.

Choose numeric thresholds before continuing.

Deliverable: explicit go/no-go on Component Model as the default ABI. If it fails, implement the core-Wasm transport from the same semantic model before Phase 3.

## Phase 3 — Generate Wasm callouts and worlds

Implement WIT renderer.

Generate worlds for:

- Rules synced;
- Rules unsynced;
- Gaia synced;
- Gaia unsynced.

Generate the UI world schema if useful, but keep UI runtime loading disabled.

Implement generated C++ host adapters:

1. validate semantic input;
2. build query;
3. call existing NativeInterface API;
4. convert result;
5. return semantic error/value;
6. account host work.

Never generate `Memory.Free*`/native-pointer-management APIs into Wasm.

Enforce the generator invariant:

- mutating + variable-size result is forbidden unless explicitly reviewed;
- current exception: `CallCOBScript`, already handled by Phase 2.

Generate native ↔ Wasm normalized signature artifacts and compare them for each interface as it is enabled.

Deliverable: broad callout coverage for the four gadget environments.

## Phase 4 — Shared Rust module API

Create `spring-wasm` and, if useful, a small shared `spring-module` façade.

Goals:

- same semantic Rust value types;
- same callout naming/shape;
- same module/callin trait concepts;
- target/environment configuration contains transport differences.

Create a representative mid-size module exercising every semantic type family.

It must build/run as:

- native Rust;
- Wasm Rules/Gaia environment.

Use the retained Phase-2 runtime fixture until the engine runtime is integrated.

Deliverable: same module source works on both backends.

## Phase 5 — Engine Wasm system and multiple instances

Implement:

### `WasmRuntime`

- pinned Wasmtime configuration;
- engine/runtime creation;
- feature policy;
- compilation/cache hooks;
- global deterministic configuration.

### `WasmModule`

Per-instance:

- store/memory;
- environment;
- generated imports;
- resource/handle tables;
- callbacks;
- fuel/work accounting;
- lifecycle/fault state.

### `WasmInterfaceSystem`

- discover/load multiple Wasm modules;
- instance registry;
- deterministic ordering;
- unload/reload;
- environment assignment;
- module hash/config integration.

### Module discovery

Use existing game/map conventions first:

- game Rules module;
- map Gaia module.

Add a general declaration format that can later support more than one module/archive.

For synced modules, include module hash + environment + order in match configuration.

### Shared callins

Refactor `NativeInterfaceEventClient` so conversion remains common and backend dispatch varies.

Generate native/Wasm dispatch from the canonical callin inventory.

Deliverable: multiple Wasm modules receive callins and call Spring APIs in-engine.

## Phase 6 — Callbacks, RmlUi, Gfx and remaining API shapes

### RmlUi

Apply the Phase-2 synchronous callback mechanism:

- callback registry IDs;
- host subscription resources;
- destruction/unload;
- controlled re-entry;
- nested callback arguments.

RmlUi remains important for the final API, but it does not block the first gadget-only vertical slice.

### Gfx

Enable full Gfx for the environments whose Lua equivalents have draw access.

Benchmark again with real generated coverage.

If Component Model overhead misses the Phase-2 budget, generate a core-Wasm Gfx transport while keeping the same Rust public API.

### Other special cases

Finish:

- callbacks;
- opaque handles;
- nested pointer records;
- any manual lowering exceptions;
- scoped VFS operations.

Deliverable: all supported NativeInterface type patterns have a defined Wasm lowering.

## Phase 7 — Sandbox and synced deterministic profile

Implement the rules in `recoil_wasm_sandbox_determinism.md`:

- no WASI/general OS imports;
- pre-instantiation validation;
- memory/resource limits;
- deterministic feature configuration;
- host-work accounting;
- re-entry guard;
- handle ownership checks;
- FP/RNG policy;
- fault lifecycle;
- AOT cache safety.

Integrate synced Wasm into the cross-platform sync-test matrix.

Deliverable: Rules/Gaia synced Wasm passes repeated cross-platform lockstep tests.

## Phase 8 — Full Lua/native/Wasm parity gates

Implement normalized Lua signature export.

Require:

- native ↔ Wasm signatures;
- Lua ↔ native ↔ Wasm signatures;
- runtime parity for applicable APIs;
- explicit exclusions only.

Add `--mode wasm` or equivalent to the existing parity harness.

Reuse the Phase-0 multi-ally fixture.

Deliverable: parity is a CI gate, not a generated report.

## Phase 9 — LuaUI environment

Implement the UI-specific role only after gadget environments are stable.

Reuse the engine visibility semantics used by LuaUI/Lua synced read code.

Cover:

- local player/team/ally-team read context;
- ally/visible/typed distinctions;
- LOS/radar behavior;
- degraded/fuzzed values;
- UI-specific callins;
- RmlUi integration.

Enable the UI world only after multi-ally Lua ↔ Wasm parity passes.

Deliverable: LuaUI-equivalent Wasm environment.

## Phase 10 — CI/build/release integration

CI:

- generator tests;
- generated snapshot diff;
- native Rust checks;
- Wasm Rust checks;
- normalized signature gates;
- runtime parity;
- adversarial ABI/security tests;
- cross-platform synced test.

Engine build:

- pin Wasmtime;
- add CMake wiring for `rts/WasmInterface`;
- choose runtime artifact/source-build strategy per supported platform;
- optionally add safe host-generated AOT caching.

Packaging:

- define how game/map archives declare Wasm modules;
- define module/environment metadata;
- define compatibility/version errors.

Out of scope/future:

- native-module trust policy;
- LuaMenu/native lobby module work;
- LuaIntro;
- LuaParser/definition-parsing;
- RmlUi availability outside the current in-game UI path.

## Implementation order summary

```mermaid
flowchart LR
    P0["0 Baseline"] --> P1["1 API model"]
    P1 --> P2["2 ABI spike"]
    P2 --> P3["3 Generate"]
    P3 --> P4["4 Rust SDK"]
    P4 --> P5["5 Engine runtime"]
    P5 --> P6["6 Full API"]
    P6 --> P7["7 Sync/sandbox"]
    P7 --> P8["8 Parity"]
    P8 --> P9["9 LuaUI"]
    P9 --> P10["10 Release"]
```

The largest unknown is Phase 2. Once the ABI/runtime/type-lowering choices are proven, the remaining 1,000+ functions should mostly be a generation and parity-coverage problem rather than handwritten integration.

# Core Wasm fast ABI implementation

Status: implementation foundation on `agent/wasm-core-fast-abi`.

## Implemented

- `WasmCoreAbi.{h,cpp}`
  - ABI version 1 constants.
  - packed fallible scalar result (`i64 = status:i32 | value:i32`).
  - validated wasm32 linear-memory reads/writes with overflow-safe bounds checks.
  - lazy memory discovery for start-function imports and explicit post-instantiation caching.
  - one-time guest export signature validation.
  - cached `wasmtime_func_t` plus `wasmtime_func_call_unchecked` callin dispatch with a stack raw slot.
  - generic Wasm C-API functype construction used only while linking.

- `WasmCoreBindings.{h,cpp}`
  - direct generated-shaped `NativeInterface` callbacks, no `WasmValue` tree.
  - `GetUnitDefID(i32) -> i64` as a pure scalar hot path.
  - `GetUnitPosition(i32, flags:i32, out:i32) -> status:i32` using caller-owned guest memory.
  - imports registered with `wasmtime_linker_define_func_unchecked`.
  - `InstanceBindings` caches guest memory and optional `spring:callin/game-frame`.

- `rust/crates/spring-wasm-core`
  - safe Rust wrappers for both callouts.
  - wasm32 FFI is private and contains the only unsafe calls.
  - `export_game_frame!` emits the stable Core ABI callin export.
  - host builds return an explicit unsupported-target error rather than trying to link Wasm imports.

- CMake and Cargo workspace integration for the new compiled units/crate.

## ABI slice

Imports:

```text
module "spring:units-info"
  get-unit-def-id : (i32 unit-id) -> i64 packed
  get-unit-position : (i32 unit-id, i32 flags, i32 output-ptr) -> i32 status
```

Export:

```text
spring:callin/game-frame : (i32 frame) -> ()
```

`get-unit-def-id` packing:

```text
bits  0..31  signed i32 value
bits 32..63  signed i32 NativeInterface error code
```

`get-unit-position` writes 12 bytes (`x`, `y`, `z` as three `f32`s) into caller-owned guest memory after validating the complete destination range. The host never calls a guest allocator.

## Remaining WasmModule wiring

The existing non-component branch in `WasmModule::Initialize` is still the old `spring.add-i32` vertical slice. Replace only that section; the new binding implementation is already separate so this should remain a small integration change.

`BackendState` needs:

```cpp
std::unique_ptr<recoil::wasm::core::InstanceBindings> coreBindings;
```

Before core instantiation:

```cpp
backendState->coreBindings = std::make_unique<recoil::wasm::core::InstanceBindings>(
    static_cast<NativeInterface*>(hostAdapter->NativeInterfaceHandle()));
if (!backendState->coreBindings->RegisterImports(backendState->coreLinker, error))
    return false;
```

Immediately after `wasmtime_linker_instantiate` succeeds:

```cpp
if (!backendState->coreBindings->Bind(
        wasmtime_store_context(backendState->store), backendState->coreInstance, error))
    return false;
```

`GameFrame` dispatch becomes:

```cpp
if (backendState->coreBindings != nullptr)
    return backendState->coreBindings->GameFrame(
        wasmtime_store_context(backendState->store), frame, error);
```

Do not route Core callouts through `WasmModule::InvokeCallout`; that would recreate the `WasmValue` allocation/conversion cost this transport exists to remove.

## Required validation before enabling

The current execution environment available to this agent has neither the engine checkout nor Cargo, so this branch has not been compiled here. Before switching the benchmark backend:

1. build the normal engine target;
2. build/test the Rust workspace;
3. compile a wasm32 guest using `spring-wasm-core`;
4. run `GetUnitDefID`, `GetUnitPosition`, and `GameFrame` parity tests;
5. benchmark the exact end-to-end rows already used by `benchmarking_results.md`;
6. only then expand codegen to the remaining API shapes.

The Component Model path should remain intact until the Core slice has measured end-to-end numbers and parity coverage.

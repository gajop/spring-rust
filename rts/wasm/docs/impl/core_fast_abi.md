# Core Wasm fast ABI implementation

Status: executable vertical slice on `agent/wasm-core-fast-abi`, pending compilation on the normal engine toolchain.

## Implemented

### Core ABI/runtime primitives

- `WasmCoreAbi.{h,cpp}`
  - ABI version 1 constants.
  - packed fallible scalar result (`i64 = status:i32 | value:i32`).
  - validated wasm32 linear-memory reads/writes with overflow-safe bounds checks.
  - lazy memory discovery for imports and post-instantiation memory caching.
  - one-time guest export signature validation.
  - cached `wasmtime_func_t` plus `wasmtime_func_call_unchecked` callin dispatch with a stack raw slot.
  - generic Wasmtime C-API functype construction used only while linking.

### Direct NativeInterface bindings

- `WasmCoreBindings.{h,cpp}`
  - direct generated-shaped `NativeInterface` callbacks; no `WasmValue` tree.
  - `GetUnitDefID(i32) -> i64` as a pure scalar hot path.
  - `GetUnitPosition(i32, flags:i32, out:i32) -> status:i32` using caller-owned guest memory.
  - imports registered with `wasmtime_linker_define_func_unchecked`.
  - deterministic host-work/re-entry accounting remains active on the direct callbacks.
  - `GetUnitPosition` writes explicit little-endian IEEE-754 bytes rather than depending on host endianness.
  - `InstanceBindings` caches guest memory and optional `spring:callin/game-frame`.

### Native C++ Core host

- `WasmCoreHost.{h,cpp}`
  - selected by `SPRING_WASM_CORE_HOST=1`.
  - uses the same `WasmRuntime` Wasmtime configuration profile as the engine.
  - validates the module before compilation.
  - creates an isolated store/linker/instance per module.
  - applies store memory/table/instance limits and configured fuel.
  - registers the unchecked direct Core imports.
  - resolves `GameFrame` once and calls the cached unchecked function thereafter.
  - faults the instance after a callin trap/failure rather than repeatedly entering a broken guest.
  - currently scoped to the synced rules benchmark world.

`WasmTypedHost` now multiplexes the two alternate benchmark transports:

```text
SPRING_WASM_TYPED_HOST=1  -> Rust typed Component Model host
SPRING_WASM_CORE_HOST=1   -> native C++ Core-Wasm host
```

The existing `NativeInterfaceEventClient` hook is reused unchanged.

For the first no-import Core guest, the existing `WasmModule` Core instance is still instantiated as inert bookkeeping. `GameFrame` is intercepted by `WasmCoreHost` before semantic Component dispatch. This avoids a risky rewrite of the 100+ KB `WasmModule.cpp` before the vertical slice can be compiled and measured.

### Rust guest SDK

- `rust/crates/spring-wasm-core`
  - safe Rust wrappers for both implemented callouts.
  - wasm32 FFI is private and contains the only unsafe calls.
  - `export_game_frame!` emits the stable Core ABI callin export.
  - host builds return an explicit unsupported-target error rather than trying to link Wasm imports.

### Focused engine benchmark

- `test/wasm_api/core_benchmark_guest/`
  - minimal no-import wasm32 guest.
  - exports `spring:callin/game-frame(i32)`.
  - performs one volatile memory write so a real linear memory remains present.
  - `run_engine_gameframe.py` reuses the existing benchmark fixture and engine-side timer but reports only the implemented Core `callin_gameframe` row.

Run after building the engine:

```bash
python3 test/wasm_api/core_benchmark_guest/run_engine_gameframe.py
```

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

`get-unit-position` writes 12 little-endian bytes (`x`, `y`, `z` as three `f32`s) into caller-owned guest memory after validating the complete destination range. The host never calls a guest allocator.

## Current blocker for real callout benchmarks

`WasmRuntime::ValidateModule` still deliberately accepts only the historical synthetic Core import `spring.add-i32`. The alternate Core host independently registers the real imports above, but a module that imports them cannot yet pass the engine's normal module bookkeeping validator.

Do not bypass this validation. The next integration change is to replace the synthetic Core allow-list with generated Core import metadata and then enforce the synced Core memory/table policy there.

For synced Core modules the intended validation rule is stricter than the current generic validator:

- wasm32 only;
- no shared memory/threads;
- no WASI/OS imports;
- no relaxed SIMD;
- exact generated Spring import allow-list;
- at most one linear memory;
- memory/table maximum must be present and equal to minimum, so growth success cannot depend on local host resource availability;
- declared sizes must remain under the configured engine limits.

## Wasmtime security pin

The branch still pins Wasmtime `42.0.1`. This is not suitable for shipping: Wasmtime `42.0.2` fixes an AArch64 Cranelift sandbox escape affecting `42.0.1`, among other security issues.

Do not update only `wasmtime.version`: `wasmtime.sha256` must be updated with the exact official C-API archive hashes for every supported platform in the same commit. Those hashes were not exposed by the available release metadata during this implementation pass, so the pin is intentionally left unchanged rather than guessed.

## What remains

1. Compile the engine and new guest on the normal development/build machine.
2. Run `run_engine_gameframe.py` and record the first real end-to-end Core callin number.
3. Replace the synthetic Core import validator with the generated Core allow-list and fixed synced memory/table policy.
4. Remove the inert second Core instance by integrating `InstanceBindings` directly into `WasmModule` or making CoreHost a first-class module backend in `WasmInterfaceSystem`.
5. Run real `GetUnitDefID` and `GetUnitPosition` engine benchmarks.
6. Expand codegen to scalar/record/list/string/handle/callback shapes.
7. Add the complete Core backend as a normal column in `run_benchmarks.py` only once all rows represent real implementations.
8. Update Wasmtime to at least `42.0.2` with verified artifact hashes.

The Component Model paths remain intact for A/B comparison until the Core transport reaches parity and has measured end-to-end results.

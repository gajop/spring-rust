# Spring Core-Wasm ABI contract (implementation state)

This document describes the ABI implemented by `rts/WasmInterface/WasmCore*` on the
`agent/wasm-core-fast-abi` branch. It is an engine ABI, not a Wasmtime ABI. The
runtime may be replaced later as long as these guest-visible contracts remain
unchanged.

## Goals

1. Keep steady-state host/guest crossings allocation-free.
2. Resolve names and validate function signatures at module load, never per call.
3. Use Wasmtime unchecked calls only after exact load-time validation.
4. Never expose engine pointers to a guest.
5. Never call a guest allocator from an import.
6. Preflight guest output ranges before host mutation.
7. Preserve native/Lua byte semantics for strings rather than forcing UTF-8.
8. Keep synced execution independent of host resource availability.
9. Keep the ABI runtime-neutral so Wasmtime can later be benchmarked/replaced.

## Core profile

The current ABI is wasm32 Core Wasm. The module must:

- be a Core Wasm v1 binary, not a component;
- define and export exactly one linear memory as `memory`;
- use only registered Spring function imports;
- import functions with the exact registered Core signatures;
- use only numeric Core parameter/result types currently supported by the ABI;
- stay within configured module/import/export/table/memory limits;
- not use memory64;
- not use shared memory unless the environment explicitly permits threads.

For synced environments the validator additionally requires:

- linear memory to declare a maximum;
- `memory.min == memory.max`;
- tables to declare a maximum and `table.min == table.max`;
- therefore no successful `memory.grow`/`table.grow` based on local machine
  capacity can occur.

The fixed-memory invariant allows the host to cache Wasmtime's memory base and
byte length after instantiation. Unsynced/growable memories do not use that
optimization.

## Numeric error envelope

A successful 32-bit scalar plus a native error code is transported as one `i64`:

```
bits  0..31 : payload bit pattern
bits 32..63 : signed i32 status/error bit pattern
```

The low bits may represent `i32`, `u32`, `bool`, or an IEEE-754 `f32` bit pattern.
The high 32 bits are zero on success.

Two `f32` results with no independent error envelope may be packed as:

```
bits  0..31 : first f32 bit pattern
bits 32..63 : second f32 bit pattern
```

This is currently used by `UnitPreDamaged`.

## Fixed aggregate output

Fixed aggregates use caller-owned guest memory:

```
(args..., output_ptr: i32) -> status: i32
```

The host:

1. validates flags/scalars;
2. validates the complete output range;
3. invokes the native API;
4. encodes the successful result explicitly in little-endian wire form;
5. returns the native error/status as `i32`.

Native C++ struct layout is never copied directly because padding, `bool`, and
host endianness are not part of the Wasm ABI.

Implemented examples:

- `GetUnitPosition`: three `f32`, 12 bytes;
- `GetUnitVelocity`: three `f32`, 12 bytes;
- `GetUnitHealth`: five `f32`, 20 bytes.

## Variable list output

Integer-ID lists use:

```
(args..., output_ptr: i32, capacity_elements: i32) -> i64
```

Returned `i64`:

```
low  32 bits: complete required/written element count
high 32 bits: status
```

Rules:

- `status == 0`: the complete list was written and low bits are the element count;
- `status == BufferOverflow`: no partial write occurred and low bits are the
  complete required element count;
- native error: low bits are zero and high bits hold the native error code;
- capacity is in elements, not bytes;
- element wire representation is little-endian `i32`;
- `(output_ptr=0, capacity=0)` is a valid size probe;
- guests may retry if the required count changes between probe and fill.

On little-endian native targets the host uses one bounds check plus one bulk
`memcpy`. A portable byte-order fallback exists for other hosts.

Implemented `spring:units-query` examples include:

- `get-all-units`
- `get-team-units`
- `get-units-in-rectangle`
- `get-units-in-box`
- `get-units-in-sphere`
- `get-units-in-cylinder`

The Rust SDK exposes both caller-buffer `*_into(&mut [i32])` calls and optional
`Vec<i32>` convenience calls behind the `alloc` feature.

## Raw byte-string output

Engine strings are byte strings in the Core ABI:

```
(args..., output_ptr: i32, capacity_bytes: i32) -> i64
```

Returned `i64` has the same required-length/status layout as variable lists.

Rules:

- capacity and required length are bytes;
- there is no trailing NUL in guest memory;
- no UTF-8 validation or transcoding occurs;
- `BufferOverflow` performs no partial write;
- `(0, 0)` is a valid size probe.

This deliberately differs from Component Model `string`, which requires UTF-8.
It matches the engine/Lua byte-preserving contract and avoids transcoding cost.

Implemented examples:

- `spring:unit-defs/get-unit-def-name`
- `spring:unit-defs/get-unit-def-human-name`

The Rust helper exposes byte buffers, not `String`; a guest may interpret them as
UTF-8 only when its application contract permits that.

## Reviewed variable-I/O calls

Some Lua-parity operations accept a variable-size input and return a
variable-size result. Their transport uses two descriptors:

```
(direct_args..., input_descriptor: i32, output_descriptor: i32) -> status: i32
```

The descriptor layout is generated from the NativeInterface schema. The host
validates every pointer/count pair before calling native code, reports the
complete required output count, and performs no partial output write on
`BufferOverflow`. The ordinary Rust façade hides both descriptors and exposes
Rust slices and owned values.

`spring:unit-script/call-unit-script` is the first reviewed instance. It is a
typed numeric subset of Lua `Spring.UnitScript.CallAsUnit`: the function name
is a byte string, arguments are `&[f32]`, and results are `Vec<f32>`. Lua's
arbitrary-value `CallAsUnit` remains available to Lua and is not silently
changed into a numeric-only contract. The import is available only to synced
rules/Gaia environments and caps the requested result capacity at 256 values.

## Currently executable imports

### `spring:units-info`

```
get-unit-def-id       (i32)                   -> i64
get-unit-team         (i32)                   -> i64
get-unit-is-dead      (i32)                   -> i64
get-unit-experience   (i32)                   -> i64
get-unit-position     (i32, i32, i32)         -> i32
get-unit-velocity     (i32, i32)              -> i32
get-unit-health       (i32, i32)              -> i32
```

### `spring:units-query`

```
valid-unit-id             (i32)                                        -> i64
get-all-units             (i32, i32)                                   -> i64
get-team-units            (i32, i32, i32)                              -> i64
get-team-unit-def-count   (i32, i32)                                   -> i64
get-team-unit-count       (i32)                                        -> i64
get-units-in-rectangle    (f32, f32, f32, f32, i32, i32, i32)          -> i64
get-units-in-box          (f32, f32, f32, f32, f32, f32, i32, i32, i32)-> i64
get-units-in-sphere       (f32, f32, f32, f32, i32, i32, i32)          -> i64
get-units-in-cylinder     (f32, f32, f32, i32, i32, i32)               -> i64
get-unit-nearest-ally     (i32, f32)                                   -> i64
get-unit-nearest-enemy    (i32, f32, i32)                              -> i64
get-unit-separation       (i32, i32, i32)                              -> i64
```

### `spring:unit-defs`

```
get-unit-def-name         (i32, i32, i32) -> i64
get-unit-def-human-name   (i32, i32, i32) -> i64
```

`WasmCoreValidation.cpp` validates the exact module/name/signature pair against
`WasmCoreRegistry.h` before compilation/instantiation proceeds.

## Cached callin exports

The host currently resolves and type-checks these optional exports once at
instantiation and calls their cached `wasmtime_func_t` through
`wasmtime_func_call_unchecked`:

```
spring:callin/game-frame          (i32) -> ()
spring:callin/game-frame-post     (i32) -> ()
spring:callin/update              (f32) -> ()
spring:callin/unit-created        (i32, i32, i32, i32) -> ()
spring:callin/unit-pre-damaged    (i32, i32, i32, f32, i32,
                                   i32, i32, i32, i32, i32) -> i64
spring:callin/allow-unit-creation (i32, i32, i32, i32,
                                   f32, f32, f32, i32) -> i32
spring:callin/draw-world          () -> ()
```

Callins are optional. A missing export is a no-op; an export with the wrong
signature is a load/bind error.

## Host-work and fuel accounting

Every implemented import enters the existing Spring import/re-entry guard and
charges deterministic host-work units. The Core host also uses Wasmtime fuel
when `instructionFuel != 0`.

`WasmCoreHost::ResetBudget(module, error)` explicitly restores both:

- Spring host-work/result budgets;
- Wasmtime fuel via `wasmtime_context_set_fuel`.

`WasmCoreHost::FuelRemaining` exposes `wasmtime_context_get_fuel` for diagnostics.

No implicit accounting period is chosen by the Core host. The engine must decide
whether reset occurs per simulation frame, tick, event batch, or another fixed
deterministic boundary. Synced peers must make the same choice.

## Runtime safety model

The hot path uses Wasmtime unchecked functions deliberately. Safety comes from
moving checks out of steady state:

- the binary validator checks every allowed import and exact type index;
- the linker defines that exact function type;
- exports are looked up and type-checked once before caching;
- raw slot count is retained with cached exports;
- guest pointers are wasm32 offsets and every dereference is range checked;
- variable counts are overflow checked before byte conversion;
- fixed wire encoding is explicit;
- host callbacks receive stable per-instance `HostState`, never guest-controlled
  native pointers.

A trap or call failure marks a `WasmCoreHost` faulted. Fault state is sticky.
Module-scoped dispatch is available so `WasmInterfaceSystem` can preserve engine
module order/environment semantics rather than using global benchmark fan-out.

## Code generation state

`spring-api-codegen` now contains a Core ABI planner and conservative host
callback renderer:

- `render_core_wasm.rs`
- `render_core_wasm_host.rs`

It emits:

- `core-abi.json`
- `WasmCoreAbiInventory.h`
- `WasmCoreGeneratedBindings.h`
- `WasmCoreGeneratedBindings.cpp`

The generated executable callback translation unit is intentionally not yet in
CMake/runtime registration. The current specialized imports must first be
excluded from generated registration to avoid duplicate linker definitions.
The planner is diagnostic/runtime-neutral and can already show how much of the
semantic API is direct/fixed/variable/manual.

## Runtime replacement boundary

A future WAMR/WasmEdge/etc. backend should preserve everything above the runtime
adapter:

```
semantic NativeInterface model
        |
Spring Core ABI + generated Rust SDK
        |
runtime backend
   |- Wasmtime (current)
   |- WAMR
   `- other
```

Runtime-specific code owns module compilation/instantiation, import registration,
export handles, traps, memory access, fuel/interruption, and precompiled caches.
The import names, signatures, wire layouts, error envelopes, guest SDK and
module bytes remain portable Core Wasm.

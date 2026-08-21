# Core Wasm security review

Status: reviewed against the Core host and generated bindings on 2026-08-22.

This document separates five concerns that are easy to conflate: performance,
deterministic synced correctness, process safety, guest-to-OS security, and
unsynced/UI visibility. A fast or deterministic call is not automatically safe,
and a UI visibility filter is not a sandbox boundary.

## Memory limits

`WasmRuntimeConfig` caps a module at 1024 Wasm pages (64 MiB), one linear
memory, `1 << 20` table elements, 2048 imports, 256 exports, 128 sections, and
16 MiB of result bytes. The store limiter applies the memory and table caps
again after validation. Synced Core modules must declare fixed memory (`max ==
min`), so a synced module cannot grow its address space after peers have
started executing it. The benchmark guest uses a smaller fixed 16 MiB memory.

The host never trusts a guest pointer merely because it is an `i32`. Generated
bindings bind the caller memory and validate offset/length pairs before reading
or writing. Variable strings, byte lists, numeric lists, record lists, and
callback descriptors use the same checked memory helpers. Host-side result
materialization is bounded by `resultBytesLimit` and `maxValueNodes`.

Remaining low-cost hardening: make the production memory/table values explicit
in the game configuration documentation and add negative tests for every
descriptor family (null pointer with non-zero length, integer overflow in
offset-plus-length, and capacity smaller than required).

## Fuel and interruption

Wasmtime fuel consumption is enabled when `instructionFuel` is non-zero. Core
initializes the store fuel budget on load and resets it at the budget window;
the execution budget also tracks host work and result bytes. Generated imports
charge a bounded base cost on entry and charge guest-controlled lengths before
copying, adapting, or iterating. Exhaustion becomes an error/trap and the
module can be faulted and removed from unsynced execution.

There is currently no Wasmtime epoch deadline/epoch-interruption path. The
default `instructionFuel` is zero, so a module with an infinite internal loop
is not bounded by fuel unless the embedding selects a positive budget. This is
an intentional default tradeoff: game throughput and avoiding false-positive
budget crashes take priority over a generic instruction cap. A non-zero fuel
or epoch policy is appropriate only for an explicitly selected diagnostic,
untrusted-module, or server-sandbox profile; it must not silently tax ordinary
game execution. Any such profile must be part of synced runtime identity
before it is used for synced modules.

## Callback reentrancy

`ImportGuard` charges every host import and rejects ordinary re-entry. A
callback may re-enter only when the execution budget says callback re-entry is
allowed. `CallbackGuard` increments the callback depth and rejects nesting
above the configured budget. Both direct callback dispatch and retained
callback dispatch use the guard; retained dispatch also reinstalls the UI
visibility context for the nested invocation. RAII leaves both counters on all
normal and error paths.

The remaining risk is semantic rather than an unchecked pointer: a permitted
callback can call more host APIs and amplify work. Keep callback permissions
small, charge nested host work, and add a regression test that recursively
dispatches until the nesting limit is rejected.

## Variable-input descriptors

Every generated variable-input binding follows the same order: validate/bind
guest memory, enter `ImportGuard`, charge length-dependent work, then copy or
borrow the data. Strings are checked for their declared range and NUL policy;
lists validate element capacity and byte/descriptor ranges; nested records are
decoded with bounded readers. Host vectors and temporary strings are created
only after these checks. The returned status is checked before any decoded
result is exposed to the guest.

The audit requirement is per descriptor, not per API family: each generated
variable input and variable output must have a fixture case for zero length,
valid maximum-sized input, truncated descriptor, out-of-bounds data, and
overflowing offset/length arithmetic. The code generator should eventually
emit that inventory so a new API cannot silently bypass the negative tests.

## Trap handling

Validation rejects malformed sections, unsupported imports, signature mismatches,
unbounded synced memory, and imports outside the Spring/Recoil namespaces.
Instantiation and every Wasmtime call inspect both `wasmtime_error_t*` and
`wasm_trap_t*`; errors are converted to diagnostic text, and a faulted module
is isolated/removed according to its environment. Host bindings return packed
native error codes for ordinary API errors and use traps for violated ABI or
host-state invariants.

Trap text is diagnostic output, not an authorization mechanism. Do not expose
native pointers, filesystem paths, or arbitrary guest strings in new trap
messages without bounding them.

## Guest to OS

WASI is disabled (`allowWasi = false`) and validation rejects WASI imports.
Unknown non-Spring/non-Recoil imports are rejected as well. The Core linker
registers the generated Spring API and the small reviewed callback surface;
there is no filesystem, socket, process, dynamic-library, clock-source, or
environment-variable import exposed to the guest. `spring:desync` is a
diagnostic clock namespace only: the host binding still requires the explicit
`SPRING_ENABLE_SYNCED_TIMERS=1` opt-in for synced reads.

This is process containment, not data confidentiality. Unsynced/UI modules
must still be reviewed for what game state their ordinary Spring imports expose.

## Determinism and visibility are separate

Synced modules use fixed memory and the environment policy controls which
imports are available. Wall-clock timers are intentionally a diagnostic sync
hazard, not a normal deterministic API. UI modules enter
`WasmUiVisibility::ScopedContext` for the whole invocation and nested imports,
so visibility filtering is inherited by callbacks. These rules do not replace
memory validation or trap handling.

## Findings and priority

1. **Documented tradeoff:** fuel defaults to disabled and no epoch deadline is
   configured. Keep that fast/available default for games; provide an explicit
   opt-in bounded profile for hostile or diagnostic workloads, with synced
   identity coverage when applicable.
2. **P2 test coverage gap:** generate negative descriptor tests for every
   variable-input/output shape, including nested records and list-of-strings.
3. **P2 observability gap:** record whether a module was rejected during
   validation, trapped during instantiation, or faulted during a call, without
   leaking unbounded guest data.

The Core path currently has no identified WASI/OS escape, unchecked descriptor
dereference, or callback-guard bypass in the reviewed files. Performance work,
sync-policy work, and UI visibility work must not weaken these boundaries.

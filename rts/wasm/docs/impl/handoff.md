# Handoff: Wasm (Rust, typed, CM) host

Date: 2026-08-18

The measurements and what they mean are in
[../considerations/measured_costs.md](../considerations/measured_costs.md),
"End to end through Rust static bindings". The current numbers are in
[benchmarking_results.md](benchmarking_results.md), which the suite generates.
This file is only the working state.

## What exists

- `rust/crates/spring-wasm-typed-host/` — `bindgen!` over a scoped WIT
  (`wit/host.wit`) declaring two worlds, the 10 host-trait impls, and a C ABI.
  Built as a `cdylib`.
- `rts/WasmInterface/WasmTypedHostShims.cpp` — one C++ shim per callout, POD in,
  POD out, straight to the `NativeInterface` function, plus the shim table.
- `rts/WasmInterface/WasmTypedHost.{h,cpp}` — dlopen loader and typed callin
  dispatch.
- Seam: `WasmInterfaceSystem::LoadModule` starts the host,
  `NativeInterfaceEventClient::DispatchWasmCallin` reroutes to it. Both behind
  `SPRING_WASM_TYPED_HOST`, inert otherwise; `test_WasmInterface` is 2552
  assertions green with the switch off.
- Harness: `wasm_rust_typed` is a fourth backend with its own report column.
  The name spells out all three axes (Rust, typed bindings, Component Model)
  because native modules are usually Rust too, so "rust" alone would name the
  wrong difference.

Covered: the rules-synced and rules-unsynced worlds, 17 callouts, 6 callins,
and the heightmap guest callback. That spans scalars, records in and out,
strings, small and large lists, mutations, a variant, and a host-to-guest
callback, so it is a real feasibility test rather than a toy.

## Facts that cost time

**The engine build image has no cargo.** Static linking is not available. The
host is built on the machine by `run_benchmarks.py` and dlopen'd, like the
native benchmark module. That is why the crate takes a shim table of function
pointers instead of importing engine symbols by name.

**`define_unknown_imports_as_traps` works at instance granularity.** An
interface this host implements only part of still counts as unknown, and
defining it wholesale collides with the real definitions. Call it *first* with
`allow_shadowing(true)` and let the implemented callouts shadow their traps.
The other order fails with ``map entry `...` defined twice``, and only for
guests that actually import the partial interface, so the callin profile passes
and the callout profile does not.

**`Linker::instance()` replaces an interface, it does not append.** A
hand-written import therefore has to carry every function the guest imports
from that interface, not just the one being special-cased, and the world-level
`add_to_linker` has to be split per interface so it does not overwrite it.

**The scoped WIT must match what the guest imports.** Declaring more functions
than the component imports is harmless; declaring fewer fails instantiation
with "function implementation is missing".

**Re-entering the guest needs the store, which the Host trait does not give
you.** `bindgen`'s trait hands out only `&mut HostState`. Define that import by
hand with `func_wrap`, which receives `StoreContextMut`. Component Model
reentrance is permitted here; the heightmap callback works.

**Make the callin export optional.** The `unimplemented` variant guest exports
no callin interface on purpose, so use `Linker::instantiate` plus
`Bindings::new(..).ok()` rather than the generated `instantiate`.

**The guest labels its own rows `"wasm"`** because it is the same component on
either transport. A `wasm_rust_typed` run splits output between
`benchmark_wasm_rust_typed.jsonl` (engine-side callin rows) and `benchmark_wasm.jsonl`
(guest-sent rows); the harness ingests both and relabels.

**Pin the crate features.** Defaults add `component-model-async` and
`stack-switching`, which the C API build lacks, and a component callin measures
348 ns instead of 67.

## Still open

1. `callin_4modules` is marked unavailable for `wasm_rust_typed`: the host holds one
   component instance per process, so a four-module run would dispatch once and
   report it as four. Fixing it means a host per module rather than a singleton.
2. The `draw` profile needs the UI world, which is not implemented. Its three
   rows are marked unavailable for `wasm_rust_typed`.
3. The remaining 1,337 callouts are unimplemented and trap. Only what the
   benchmark table exercises is covered, by design.
4. Nothing here is wired into a non-benchmark path, and the switch is off by
   default. This is a prototype for measurement, not a shipping transport.

## Carried over from the previous cycle

1. ASAN plus CTest have never been run against the callout and callin changes.
2. `test_WasmAllocator` fails one case with SIGILL inside JIT guest code.
3. `verify_codegen.py` fails on gaia_synced probe drift, predating this work.

## Build and run

- Engine: `./docker-build-v2/build.sh --compile linux`.
- Typed host: built automatically by `run_benchmarks.py`; standalone with
  `cargo build --release --manifest-path rust/crates/spring-wasm-typed-host/Cargo.toml`.
- Full suite: `python3 test/native_api_parity/run_benchmarks.py --suite`.
- One profile: `--callins --scale 1 --no-report --summary-json <path>`.
- Benchmarks need a release build with no ASAN and the machine to itself. The
  draw profile opens a window.

## Working preferences

Plain language. No em dashes. Concise. Few comments in code. Measure rather
than project; projections in this area have already been wrong.

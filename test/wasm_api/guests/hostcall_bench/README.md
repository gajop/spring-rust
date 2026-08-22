# Host-call transport floors

Measures what a Wasm host call costs before any engine code runs: a guest that
does nothing but call, a host callback that does nothing but return. Results
and their interpretation live in `rts/wasm/docs/considerations/measured_costs.md`.

```bash
./run.sh                 # 500000 iterations, 21 samples, pinned to CPU 2
./run.sh 200000 9 4      # iterations, samples, cpu
```

`run.sh` rebuilds `core.wasm` from `core.wat`, rebuilds and componentizes the
guest, builds both harnesses, and runs each pinned to one core.

- `wit/bench.wit`, `guest/` are the Component Model guest, built with
  wit-bindgen and encoded by `guest/src/bin/componentize.rs`.
- `core.wat` is the core-Wasm guest with the same call loops.
- `host_c/` is the Wasmtime C API harness, the transport the engine links
  today. It needs the pinned SDK; set `WASMTIME_ROOT` if it is not under
  `build-amd64-linux/_deps`.
- `host_rust/` is the Wasmtime Rust API harness, typed `func_wrap` and
  `TypedFunc`. Its feature set is pinned to match the C API release; the crate
  defaults add per-call work that the C API build does not have.

Nothing here is wired into the engine build or CTest.

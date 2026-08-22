/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

# Core Wasm fixtures

The active Wasm API is the Core transport. Reusable guests are under
`test/wasm_api/guests/`; generator and verification tools are under
`test/wasm_api/tools/`; checked-in manifests and host-only fixtures are under
`test/wasm_api/data/`.

The policy/runtime fixture is pinned to the Wasmtime release in
[`rts/wasm/wasmtime.version`](../../rts/wasm/wasmtime.version). The checked-in
`data/runtime_spike.wast` is a host-independent smoke test:

The engine CMake dependency downloads or consumes the checksum-pinned Wasmtime
42.0.1 C API SDK. The exact ASAN build is:

```text
./docker-build-v2/build.sh linux -DUSE_ASAN=ON
```

`data/host_scalar.wat` is the Core-Wasm import fixture. Unknown imports and
invalid module policy are rejected before instantiation; the runtime never
falls back to a command-line Wasm executable.

`runtime_spike.wast` remains a host-independent CLI smoke fixture:

```text
wasmtime --version                 # must report 42.0.1
wasmtime wast test/wasm_api/data/runtime_spike.wast
```

In-engine archive discovery uses optional `LuaRules/wasm/manifest.txt` from the
game VFS and `LuaGaia/wasm/manifest.txt` from the map VFS. Each manifest uses
the tested `module(name, path, environment, order[, interface-version])`
format; packages should write the explicit `1.0.0` interface version. Omitting
the fifth field is retained only for compatibility with early manifests and
means the current host interface.

The Core parity guest is generated from the canonical API test specification:

```bash
python3 test/wasm_api/guests/parity_guest/generate_probe.py
cargo build --manifest-path test/wasm_api/guests/parity_guest/Cargo.toml \
  --target wasm32-unknown-unknown --release --features core,core_rules_synced
python3 rts/wasm/verify_codegen.py
```

`core_benchmark_suite_guest` contains the transport and workload benchmark
guests used by `test/native_api_parity/run_benchmarks_core.py`. Fuel and epoch
interruption are diagnostic opt-in controls; throughput-first gameplay
measurements leave them disabled.

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

# Wasm runtime and Component Model fixtures

The policy/runtime fixture is pinned to the Wasmtime release in
[`rts/wasm/wasmtime.version`](../../rts/wasm/wasmtime.version). The checked-in
`runtime_spike.wast` is a host-independent smoke test:

The engine CMake dependency downloads or consumes the checksum-pinned Wasmtime
42.0.1 C API SDK. The exact ASAN build is:

```text
./docker-build-v2/build.sh linux -DUSE_ASAN=ON
```

`host_scalar.wat` is the core-Wasm import fixture. The C++
`test_WasmInterface` target also embeds hand-authored and Rust `wit-bindgen`
Component Model fixtures covering scalar imports, strings, byte lists, records,
options, enums, flags, and results. Unknown imports and invalid module policy
are rejected before instantiation; the runtime never falls back to a command-
line Wasm executable.

`runtime_spike.wast` remains a host-independent CLI smoke fixture:

```text
wasmtime --version                 # must report 42.0.1
wasmtime wast test/wasm_api/runtime_spike.wast
```

In-engine archive discovery uses optional `LuaRules/wasm/manifest.txt` from the
game VFS and `LuaGaia/wasm/manifest.txt` from the map VFS. Each manifest uses
the tested `module(name, path, environment, order[, interface-version])`
format; packages should write the explicit `1.0.0` interface version. Omitting
the fifth field is retained only for compatibility with early manifests and
means the current host interface. The runtime test also embeds `value_guest`,
a real `wit-bindgen` Component fixture that exercises
variant payloads, unit variant cases, and owned resource creation/transfer/drop.
Its checked-in C++ bytes are regenerated with:

```bash
cargo build --manifest-path test/wasm_api/value_guest/Cargo.toml \
  --target wasm32-unknown-unknown --release
cargo run --manifest-path test/wasm_api/value_guest/Cargo.toml --bin componentize -- \
  test/wasm_api/value_guest/target/wasm32-unknown-unknown/release/recoil_wasm_value_guest.wasm \
  /tmp/recoil_wasm_value_guest.component.wasm
python3 test/wasm_api/embed_component.py \
  /tmp/recoil_wasm_value_guest.component.wasm \
  test/engine/WasmInterface/ComponentValueFixture.h \
  kComponentValueFixture 'the Rust wit-bindgen variant/resource fixture'
```

The `allocator_guest` fixture is a deliberately hostile canonical-lowering
regression test. Its Rust allocator attempts to call the imported
`units-query` function while Wasmtime is allocating the result of an earlier
import. The engine must reject that nested import, fault the module, and leave
the runtime usable. Rebuild its checked-in bytes with:

```bash
cargo fmt --manifest-path test/wasm_api/allocator_guest/Cargo.toml --all --check
cargo test --manifest-path test/wasm_api/allocator_guest/Cargo.toml
cargo build --manifest-path test/wasm_api/allocator_guest/Cargo.toml \
  --target wasm32-unknown-unknown --release
cargo run --manifest-path test/wasm_api/allocator_guest/Cargo.toml --bin componentize -- \
  test/wasm_api/allocator_guest/target/wasm32-unknown-unknown/release/recoil_wasm_allocator_guest.wasm \
  /tmp/recoil_wasm_allocator_guest.component.wasm
python3 test/wasm_api/embed_component.py \
  /tmp/recoil_wasm_allocator_guest.component.wasm \
  test/engine/WasmInterface/ComponentAllocatorFixture.h \
  kComponentAllocatorFixture 'the hostile canonical allocator fixture'
```

The end-to-end allocator cases are registered as
`testWasmAllocatorReentry`, `testWasmAllocatorTrap`, and
`testWasmAllocatorFuel`. Each is a separate CTest process. Wasmtime's native
signal-trap boundary is process global; isolating these intentionally trapping
or fuel-exhausting cases prevents an unrelated earlier Wasm store in the
aggregate test process from affecting its trap boundary. This is test-process
isolation, not an omitted runtime feature.

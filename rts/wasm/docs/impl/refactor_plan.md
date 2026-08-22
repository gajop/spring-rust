# Wasm/NativeInterface refactoring plan

Scope: only code this project produced — `rts/WasmInterface/`, `rts/wasm/`,
`rust/crates/spring-native-codegen/`, `rust/crates/spring-wasm-core/`,
`test/wasm_api/`, `test/engine/WasmInterface/`. The pre-existing engine
(`rts/NativeInterface/api/*.cpp`, `rts/NativeInterface/codegen/`) is explicitly
out of scope even where it is large.

Two problems to fix: files in the thousands of lines, and flat directories with
~90 files in them.

## Current execution status

This plan is being executed after the Core parity phase. The Core transport
implementation and its runtime-enabled parity gates are complete. The
Component transport purge is now landed in the working tree; the remaining
work is Core-output sharding, package layout, and verification records.

- [x] Core parity handoff and enabled-context verification.
- [x] Legacy codegen wrappers now share a `ModuleSpec` table and one generic
  generation path in `spring-native-codegen/src/lib.rs`.
- [x] Extract the shared Core wire/input lowering module.
- [x] Delete the Component Model transport and its dead fixtures.
- [x] Put the surviving Core parity generator behind a `probe/` package entry
      while preserving the existing CLI path and generated output.
- [x] Shard the surviving generated Core outputs by owned API module.
- [ ] Finish the semantic Python module split and any move-only handwritten
      directory layout changes.

The wrapper-table change is intentionally output-neutral: it preserves all 54
public `generate_*` entry points and is covered by the existing codegen test
and strict regeneration gates. The remaining items stay ordered by the
dependency rule in §0. Component-only files are no longer being reorganized;
they are deleted as one purge.

The shared-wire slice is also output-neutral. The former
`render_core_wasm_variable_io_host.rs` is now 537 lines (from 1,017), and
`render/core/shared/wire.rs` owns the canonical fixed-layout, descriptor,
variable-input, and wire reader/writer helpers. Both variable-I/O and
dynamic-output renderers use that module; generated artifacts remain
byte-identical under `verify_codegen.py`.

Validation completed for this slice:

- `cargo fmt --manifest-path rust/Cargo.toml --all --check`
- `cargo test --manifest-path rust/Cargo.toml --workspace`
- `python3 rts/wasm/verify_codegen.py`
- `git diff --check`

The workspace test passes with the repository's existing generated-code and
renderer warnings; no new test failure or generated-artifact drift remains.

## 0. Ordering rule: do not refactor code that Phase 4 deletes

Phase 4 of the Core-Wasm plan removes the Component Model transport. Roughly
**16k LOC of the worst offenders die with it**, so refactoring them is wasted
work. Skip these entirely:

| Path | LOC | Fate |
| --- | --- | --- |
| `test/engine/WasmInterface/Component*Fixture.h` (8 files) | deleted | purge |
| `rts/WasmInterface/{WasmHost,WasmModule,WasmDispatch,WasmTypedHost}.*` | deleted | purge |
| `rts/wasm/generated/wit/` and `sdk/{generated,callins}.rs` | deleted | purge |
| `rts/wasm/generated/WasmHostAdapter*.{cpp,h}` (58 adapter TUs) | deleted | purge |
| `test/wasm_api/{value,allocator,aggregation,guest,benchmark}_guest` | deleted | purge |
| `test/wasm_api/parity_guest` | retained and ported | Core-only oracle |

Everything below therefore assumes: **Phase 3 (Core parity) → Phase 4 (delete
Component Model) → this refactor.** The deletion includes the 58 generated
`WasmHostAdapter*.cpp/.h` translation units; they are not a keep-list for the
surviving Core tree. `native_api_path` remains the Core native-registry path
and must not be removed as a misleading Component reference.

## 1. The codegen crate — do this first

`rust/crates/spring-native-codegen/src/` is 41 flat `.rs` files, 21 of them
named `render_core_wasm_*`.

**Directory tree** (module-per-directory, `mod.rs` only re-exports):

```
src/
  model/          model.rs, annotations.rs, manifest.rs, lua_loader.rs, callin_semantics.rs
  render/
    core/
      host/       fixed, option, variable, variable_output, variable_io, borrowed,
                  dynamic_input, dynamic_output
      guest/      guest, owned_guest, variable_guest, dynamic_input_guest, dynamic_output_guest
      callins/    the 8 render_core_wasm_callin_* files
      registry.rs, registry_policy.rs, coverage.rs
    shared/       wire layout + input-descriptor lowering currently living in
                  render_core_wasm_variable_io_host.rs
```

The former `render/component/` contents are deleted with the purge. The
surviving renderer is Core-only; `render_core_native.rs` owns the native
registry path.

**`lib.rs` (2,449 → target ~200).** It is ~60 near-identical `generate_*`
functions (`generate_units_query`, `generate_teams`, …), each ~15 lines of the
same shape. Replace with a `const MODULES: &[ModuleSpec]` table plus one
generic `generate(spec)`; keep thin named wrappers only if callers need them.
The `mod semantic_codegen_tests` block at the end moves to `tests/`.

**`render_core_wasm_variable_io_host.rs` (1,017)** is doing double duty: it is
both a renderer and the shared input-lowering library that six other renderers
import (`input_field_supported`, `input_descriptor_layout`, `render_wire_read`,
`fixed_wire_layout`, …). Split those out into `render/core/shared/wire.rs`
first — every other core-host split gets easier once that dependency is named.

The former Component renderers (`render_host.rs`, `render_wit.rs`,
`render_wasm_sdk.rs`, `render_signatures.rs`, and `render_callins.rs`) are
deleted. `render_core_native.rs` is the surviving registry renderer.

**Target:** no renderer over ~600 lines. The natural seam inside each is
`plan()` (classify + compute layout) vs `emit()` (produce text); most of these
files are already implicitly structured that way.

## 2. Generated output — fix the emitter, not the files

`rts/wasm/generated/` contains machine-written output. The Component Model
files and the 58 `WasmHostAdapter*` translation units are deleted. The
remaining Core output is split by transport class and API module where the
large files affect compile or review cost.

| File | LOC |
| --- | --- |
| `sdk/core_owned/*.rs` | one owned façade shard per API module |
| `WasmCoreGeneratedBindings.cpp` | 34,263 |
| `sdk/core_generated.rs` | 26,235 |
| `WasmCoreGeneratedBorrowedBindings.cpp` | 15,093 |
| `WasmCoreGeneratedVariableBindings.cpp` | 13,994 |

The old `WasmHostAdapter_*.cpp` pattern belonged to the deleted Component
transport. It is not a keep-list and must not be recreated. The surviving
Core Rust façade now uses `sdk/core_owned.rs` as a generated prelude,
`sdk/core_owned/*.rs` as module shards, and `sdk/core_owned_footer.rs` as the
closing fragment. `spring-wasm-core/build.rs` concatenates those fragments in
sorted order. `verify_codegen.py` checks the assembled surface and every
shard.

**Directory tree** — the flat 92-file dir becomes:

```
rts/wasm/generated/
  meta/       model.json, signatures.json, callins.json, core-abi.json, coverage/plan JSONs
  host/
    adapter/  (deleted with the Component transport)
    core/
      fixed/ option/ variable/ variable_output/ variable_io/ borrowed/
      dynamic_input/ dynamic_output/ callins/     ← each: one .cpp per API module
    registry/ WasmCoreGeneratedRegistry.h, WasmCalloutRegistry.h, WasmCallinRegistry.h,
              WasmCoreAbiInventory.h
  sdk/
    core_owned.rs             generated prelude
    core_owned/               one .rs per API module
    core_owned_footer.rs      generated closing fragment
    core_generated.rs         generated raw Core imports
    core_borrowed.rs          generated borrowed Core imports
    core_variable.rs          generated variable-input Core imports
  wit/                        (deleted with the Component transport)
```

The Rust build script reads the generated shard directory directly. The
surface verifier assembles the prelude, shards, and footer before checking
every callout and environment projection. C++ binding TU sharding remains a
separate task; it must update CMake and the generated-file drift guard in the
same change.

Do the sharding now that the purge is complete: only surviving Core generated
outputs are in scope. The per-module split of
`WasmCoreGeneratedBindings.cpp` remains the first generated-output task.

Sizing result: `core_owned.rs` is now a small prelude. Owned module shards are
kept near the existing per-module scale; no owned shard is a monolith.

## 3. Hand-written C++ — `rts/WasmInterface/` (Core-only remainder)

The Component/typed system and adapter sources are deleted. The remaining
runtime, Core host, manifest, and system files are intentionally kept in their
existing flat layout until a move-only directory change is worthwhile.

```
rts/WasmInterface/
  runtime/     WasmRuntime, WasmEnvironment, WasmResources,
               WasmModuleManifest
  core/
    host/      WasmCoreHost, WasmCoreAbi, WasmCoreValidation, WasmCoreWire.h,
               WasmCoreGuestInput.h, WasmCoreRegistry.h, WasmCoreRegistryPolicy.h,
               WasmCoreUiCallinFilter.h
    bindings/  the 22 WasmCore*Bindings.cpp/.h pairs (5,736 LOC total — sizes
               are fine, they just need a home)
  system/      WasmInterfaceSystem, WasmInterfaceSystemCore
```

`WasmModule.cpp`, the typed host, and the generated adapter layer are no longer
part of this tree. `WasmRuntime` and `WasmResources` remain shared runtime
infrastructure because their validation and lifecycle code is used by Core.

`WasmRuntime.cpp` (807), `WasmCoreHost.cpp` (776), `WasmInterfaceSystem.cpp`
(659), `WasmTypedHostShims.cpp` (616), `WasmInterfaceSystemCore.cpp` (536),
`WasmCoreBindings.cpp` (526): borderline. Leave them unless a natural seam
shows up while moving them — moving and splitting in one commit makes the diff
unreadable.

## 4. Python tooling

The active CLI remains `test/wasm_api/parity_guest/generate_probe.py`, a
compatibility entry point for `probe/core.py`. Its active CLI is Core-only and
the package entry preserves generated probe output byte-for-byte. The semantic
module split remains independent of the transport purge.

Make it a package before Phase 3 touches it:

```
test/wasm_api/parity_guest/probe/
  __init__.py     CLI entry
  types.py        snake/pascal/kebab/wit_identifier/wit_type/type_kind
  model.py        load_tests, load_model, native_function
  select.py       select_tests, select_tests_with_coverage, supported_output
  expressions.py  candidate_expression, rust_*_expression, build_input_expression,
                  find_candidate, default_expression   (~700 lines; the real core)
  projection.py   output_projection, projection_record_path, sequence_output_projection
  render/
    core.py       Core probe body and manifest emission
    guest.py      probe body emission
    callbacks.py  callback_entries + argument expressions
```

`test/wasm_api/` now retains only the Core parity guest and Core benchmark
guest; the deleted Component guest crates are not to be recreated. Grouping
the surviving fixtures into `guests/`, `tools/`, and `data/` remains optional
until the package split is landed.

`rts/wasm/verify_codegen.py` and `generate_core_abi_surface.py` (513) are fine
as single files.

## 5. `rust/crates/spring-wasm-core/` — no action

24 files, largest under 500 lines, clean module-per-API-area layout. This is
the shape the rest of the project should look like.

## 6. Suggested sequencing

1. **Completed:** land Core parity, the shared-wire seams, and the Core-only
   native registry path.
2. **Completed:** delete the Component transport, its WIT/SDK output, the 58
   generated adapter TUs, and the obsolete guest fixtures.
3. Shard surviving generated Core bindings and SDK output (§2).
4. Package `generate_probe.py`, then make any move-only `rts/WasmInterface/`
   and `test/wasm_api/` directory changes (§3–4).

## 7. Ground rules

- **Never hand-edit `rts/wasm/generated/`.** Every change there is a change to
  an emitter in `rust/crates/spring-native-codegen/`, followed by a regen.
- Move-only commits stay move-only; splits are separate commits. `git log
  --follow` and review both depend on it.
- After each step: `cargo fmt --check`, `cargo test --workspace`, all 10 guest
  crates for `wasm32-unknown-unknown`, `./docker-build-v2/build.sh linux`,
  `--compile linux -t check`, `verify_codegen.py`, and both parity gates.
- Regenerating must produce a byte-identical tree before and after any
  emitter-restructuring commit that is not meant to change output. Diff the
  generated dir as the proof.
- Targets: hand-written files ≤ ~600 lines, generated ≤ ~2,000, directories
  ≤ ~25 entries.

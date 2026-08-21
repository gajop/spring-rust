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
implementation and its runtime-enabled parity gates are complete; the
Component transport is still retained as the oracle and has not yet reached
the deletion phase described below.

- [x] Core parity handoff and enabled-context verification.
- [x] Legacy codegen wrappers now share a `ModuleSpec` table and one generic
  generation path in `spring-native-codegen/src/lib.rs`.
- [x] Extract the shared Core wire/input lowering module.
- [ ] Delete the Component Model transport and its dead fixtures.
- [ ] Shard the surviving generated Core outputs.
- [ ] Finish the handwritten and Python package layout refactors.

The wrapper-table change is intentionally output-neutral: it preserves all 54
public `generate_*` entry points and is covered by the existing codegen test
and strict regeneration gates. The remaining items stay ordered by the
dependency rule in §0; no Component-only code is being reorganized before its
deletion decision is implemented.

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
| `test/engine/WasmInterface/Component*Fixture.h` (7 files) | 13,374 | delete |
| `rts/WasmInterface/WasmModule.cpp` component half | ~1,800 of 2,854 | delete |
| `rts/wasm/generated/wit/` (85 files) | ~30,000 | delete |
| `rts/wasm/generated/sdk/generated.rs` | 86,631 | delete |
| `test/wasm_api/{value,allocator,aggregation,parity,benchmark}_guest` | ~15,000 | delete or port |

Everything below therefore assumes: **Phase 3 (Core parity) → Phase 4 (delete
Component Model) → this refactor.** The one exception is §1, which is worth
doing before Phase 3 because Phase 3 adds more code to exactly those files.

## 1. The codegen crate — do this first

`rust/crates/spring-native-codegen/src/` is 41 flat `.rs` files, 21 of them
named `render_core_wasm_*`.

**Directory tree** (module-per-directory, `mod.rs` only re-exports):

```
src/
  model/          model.rs, annotations.rs, manifest.rs, lua_loader.rs, callin_semantics.rs
  render/
    component/    render_host.rs, render_wit.rs, render_wasm_sdk.rs, render_signatures.rs, render_callins.rs
    core/
      host/       fixed, option, variable, variable_output, variable_io, borrowed,
                  dynamic_input, dynamic_output
      guest/      guest, owned_guest, variable_guest, dynamic_input_guest, dynamic_output_guest
      callins/    the 8 render_core_wasm_callin_* files
      registry.rs, registry_policy.rs, coverage.rs
    shared/       wire layout + input-descriptor lowering currently living in
                  render_core_wasm_variable_io_host.rs
```

Note `render/component/` becomes a delete-in-one-step directory after Phase 4 —
that is a feature of this layout, not a coincidence.

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

**`render_host.rs` (1,688)** is Component Model; leave it, it dies in Phase 4.

**Target:** no renderer over ~600 lines. The natural seam inside each is
`plan()` (classify + compute layout) vs `emit()` (produce text); most of these
files are already implicitly structured that way.

## 2. Generated output — fix the emitter, not the files

`rts/wasm/generated/` is **377,591 LOC across 92 flat files** plus 85 more flat
`.wit` files. These are machine-written, so nobody edits them — but they still
hurt: git diffs, compile times (one 34k-line TU), IDE responsiveness, and
review. Worst:

| File | LOC |
| --- | --- |
| `sdk/generated.rs` | 86,631 (Component; dies Phase 4) |
| `sdk/core_owned.rs` | 64,049 |
| `WasmCoreGeneratedBindings.cpp` | 34,263 |
| `sdk/core_generated.rs` | 26,235 |
| `WasmCoreGeneratedBorrowedBindings.cpp` | 15,093 |
| `WasmCoreGeneratedVariableBindings.cpp` | 13,994 |

**The pattern already exists in this repo.** `WasmHostAdapter_*.cpp` is sharded
per API module (55 files, largest 9.5k). The Core binding emitters just never
adopted it. Change each `WasmCoreGenerated*Bindings.cpp` emitter in
`bin/spring-api-codegen.rs` to write one TU per API module, and the Rust SDK
emitters to write one `.rs` per module with a generated `mod.rs`.

**Directory tree** — the flat 92-file dir becomes:

```
rts/wasm/generated/
  meta/       model.json, signatures.json, callins.json, core-abi.json, coverage/plan JSONs
  host/
    adapter/  WasmHostAdapter_*.cpp  (55 files, already sharded)
    core/
      fixed/ option/ variable/ variable_output/ variable_io/ borrowed/
      dynamic_input/ dynamic_output/ callins/     ← each: one .cpp per API module
    registry/ WasmCoreGeneratedRegistry.h, WasmCalloutRegistry.h, WasmCallinRegistry.h,
              WasmCoreAbiInventory.h
  sdk/
    core/     one .rs per API module per transport class, + mod.rs
  wit/        (unchanged; deleted in Phase 4)
```

Consequences to handle in the same change: the CMake `foreach` list becomes a
`file(GLOB)` or an emitted `.cmake` manifest; `verify_codegen.py` and
`WasmCoreRegistry.h`'s `LookupImport` path both need the new locations; the
generated-registry drift guard's regexes need re-anchoring.

**Do the sharding after Phase 4**, so you shard 30 fewer files and skip the
Component ones entirely. But **land the per-module split of
`WasmCoreGeneratedBindings.cpp` (34k) before Phase 3** — Phase 3 grows it.

Sizing target: no generated file over ~2,000 lines. Per-module sharding gets
`core_owned.rs` from 64k to ~1.2k average.

## 3. Hand-written C++ — `rts/WasmInterface/` (80 flat files)

The file *sizes* here are mostly fine (only `WasmModule.cpp` is over 1k, and
two-thirds of it is Component). The *directory* is the problem: 80 files, 45 of
them `WasmCore*`.

```
rts/WasmInterface/
  runtime/     WasmRuntime, WasmEnvironment, WasmHost, WasmResources,
               WasmModuleManifest, WasmDispatch
  component/   the Component half of WasmModule.cpp, extracted first  ← delete Phase 4
  core/
    host/      WasmCoreHost, WasmCoreAbi, WasmCoreValidation, WasmCoreWire.h,
               WasmCoreGuestInput.h, WasmCoreRegistry.h, WasmCoreRegistryPolicy.h,
               WasmCoreUiCallinFilter.h
    bindings/  the 22 WasmCore*Bindings.cpp/.h pairs (5,736 LOC total — sizes
               are fine, they just need a home)
  system/      WasmInterfaceSystem, WasmInterfaceSystemCore, WasmTypedHost,
               WasmTypedHostShims
```

**`WasmModule.cpp` (2,854).** Split along the transport line — that split is
already latent in the file (`EncodeComponentString`, `LiftComponentValue`,
`CollectComponentExports`, `CheckComponentValueBudget`, … are all Component;
`ParseNativeApiError`, `WasmtimeErrorMessage`, `WasmTrapMessage` are shared).
Move Component into `component/WasmComponentModule.cpp`, shared helpers into
`runtime/WasmtimeError.h`. What remains of the core path is ~600 lines and
needs no further work. Doing the split *before* Phase 4 turns the Phase 4
deletion into `rm -r component/`.

`WasmRuntime.cpp` (807), `WasmCoreHost.cpp` (776), `WasmInterfaceSystem.cpp`
(659), `WasmTypedHostShims.cpp` (616), `WasmInterfaceSystemCore.cpp` (536),
`WasmCoreBindings.cpp` (526): borderline. Leave them unless a natural seam
shows up while moving them — moving and splitting in one commit makes the diff
unreadable.

## 4. Python tooling

`test/wasm_api/parity_guest/generate_probe.py` is **3,850 lines and ~80
top-level functions** — by far the worst hand-written file in the project, and
Phase 3 makes it worse (it needs a `--transport core` mode, and its
`extract_probe_dependencies` bug currently strands 288 tests).

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
    wit.py        render_wit + render_type_record/enum
    guest.py      probe body emission
    callbacks.py  callback_entries + argument expressions
```

`test/wasm_api/` itself is a flat mix of 8 guest crates, 5 scripts, 2 JSON and
some stray `.wat`/`.wast`. Group into `guests/`, `tools/`, `data/`.

`rts/wasm/verify_codegen.py` and `generate_core_abi_surface.py` (513) are fine
as single files.

## 5. `rust/crates/spring-wasm-core/` — no action

24 files, largest under 500 lines, clean module-per-API-area layout. This is
the shape the rest of the project should look like.

## 6. Suggested sequencing

1. **Current safe slice:** finish the codegen crate module tree around the
   completed `ModuleSpec` and shared-wire seams (§1), without reorganizing
   Component-only code.
2. **After the Component deletion decision:** shard surviving generated Core
   bindings and split `WasmModule.cpp` along the Component seam (§2–3).
3. **Phase 3 is complete.** Make and verify the **Phase 4** decision (delete
   the Component Model) before touching the Component-only offenders; that
   deletion removes roughly 16k LOC for free.
4. **After Phase 4:** package `generate_probe.py`, then complete the generated
   output tree, `rts/WasmInterface/` directory tree, and `test/wasm_api/`
   grouping (§2–4).

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

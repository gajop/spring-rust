# Handoff: Core parity coverage (Phase 3)

Date: 2026-08-21
Repo: `gajop/spring-rust`
Branch: `rust-wip`
Head at handoff: `f327a902a6`

Read this together with `rts/wasm/docs/impl/web_agent_handoff.md`. That document
still describes the overall sequencing and the invariants. Where the two
disagree about *numbers*, this one is current: callout coverage is finished and
several of its "still open" items are closed.

## 0. State at handoff

Everything below was actually run on this machine, not inferred.

| check | result |
| --- | --- |
| `spring-api-codegen --strict` | exit 0, **1354/1354 callouts executable, 0 pending** |
| callin coverage | 126/126 planned classes emit, 0 pending, **0 verified** |
| `cargo fmt --check` / `build` / `test --workspace` | clean / green |
| all 10 guest crates under `test/wasm_api/*` for `wasm32-unknown-unknown` | build |
| `./docker-build-v2/build.sh linux` | green |
| `./docker-build-v2/build.sh --compile linux -t check` | **38/38** |
| `python3 rts/wasm/verify_codegen.py` | fails **only** on the pre-existing `gaia_synced` probe drift |
| `test/wasm_api/check_core_command_parity.py` | pass |
| `test/wasm_api/generate_core_parity_plan.py` | runs; `--strict` fails, see §4 |

Working tree clean. Nothing pushed.

**Core parity coverage is still zero.** That is the entire job below.

## 1. The one thing that will bite you if you skip it

`rts/WasmInterface/WasmCoreRegistry.h` resolves reviewed **handwritten** imports
before generated ones (`LookupImport`, then `generated_registry::Find`). When
both registries claim a name, the handwritten signature is the only one a guest
can import: `WasmCoreValidation.cpp:228` compares the module's declared type
against the resolved descriptor and rejects a mismatch outright.

36 names used to be generated with a *different* shape than the handwritten
binding declares — all of `messages`, the GiveOrder family, `GetAllUnits`,
`GetTeamUnits`, the `GetUnitsIn*` queries, `GetUnitDefName`/`HumanName`,
`GetUnitCommands`, the centroids, four `terrain` getters. Those generated
bindings were unreachable, and the coverage report counted them as generated
transports anyway.

That is fixed at `f327a902a6`: they are classified `handwritten-reviewed`
(class count 25 -> 61), the generated registry no longer advertises any name the
handwritten registry owns, and `verify_codegen.py` fails if the two ever
disagree again (`verify_core_import_registries`).

**Consequence for you:** the ABI a Core guest must be built against is the one
that *wins resolution*, not the one in `core-abi.json` alone. For any function
whose coverage class is `handwritten-reviewed`, read the signature out of
`kImports` in `WasmCoreRegistry.h` and the calling convention out of the
handwritten `rts/WasmInterface/WasmCore*Bindings.cpp`. `core-abi.json` describes
the *generic plan*, which for those names is not what is registered.

24 names still appear in both registries with **identical** signatures. That is
harmless — either binding satisfies the guest — but note the handwritten one is
what actually runs, so a parity failure on one of those is a bug in the
handwritten binding, not the generated one.

## 2. The job

Give Core a correctness oracle, then and only then delete Component Model.

The parity harness is healthy and transport-independent in its hard parts:
`test/native_api_parity/api_tests/` holds 76 spec files; the synced context
selects 500 of 507 source tests; there are five contexts
(`synced_gadget`, `unsynced_gadget`, `gaia_synced`, `gaia_unsynced`, `ui`).
Every case currently runs through **Component Model only** —
`test/wasm_api/parity_guest` is a `wit-bindgen` + `wit-component` guest.
`WASM_PARITY_CORE` in `run_harness.py` is a misleading name: it is the raw
module fed *into* componentization, not a Core-ABI guest.

## 3. Design, already traced and validated

### 3.1 The seam is one string

`test/wasm_api/parity_guest/generate_probe.py` is 3738 lines, but every call it
emits goes through exactly one path prefix:

```
crate::bindings::recoil::spring_api::{module}::{function}
```

23 occurrences, all of the same shape (grep it). Record construction uses
`crate::bindings::recoil::spring_api::{module}::{Pascal}` with snake_case
fields.

The error type matches structurally with no adaptation:
`record spring-error { code: s32 }` vs `pub struct ApiError { pub code: i32 }`,
and probes only ever read `.code`.

So the probe generator needs a **transport mode that swaps that prefix**, not a
rewrite. All the hard parts — semantic type walking, record paths, output
projection, coverage selection, fixture inputs — stay untouched.

### 3.2 What has to exist on the other side of the seam

Of the **515 distinct callouts** the probes exercise:

| class | count | what exists today |
| --- | ---: | --- |
| `fixed` + `fixed-option` | 338 | **usable as-is**: `spring_wasm_core::generated::{module}::{fn}` already returns owned typed values with matching shapes |
| `variable-output-caller-owned` | 40 | raw import only |
| `dynamic-output-caller-owned` | 44 | raw import + `&mut [u8]`, no decoder |
| `variable-input-borrowed` (+ mixed-fixed) | 43 | typed, but takes `&CStr` / `&[u8; N]` blobs |
| `variable-io-...` | 31 | raw import only |
| `variable-input-nested-adapted` | 13 | takes `&[u8]` blobs |
| `handwritten-reviewed` | 6 | hand-map these |

So ~180 functions need an **owned-value façade**, plus a dynamic-stream decoder,
plus the record types the Core SDK does not define yet (SDK has 112 structs,
the WIT has 211 records).

Recommended shape: a new generated SDK fragment `rts/wasm/generated/sdk/core_owned.rs`
emitted by a new `rust/crates/spring-native-codegen/src/render_core_wasm_owned_guest.rs`,
included by `spring-wasm-core/build.rs` (add it to the `append_generated` list),
gated behind the crate's existing `alloc` feature — that feature already exists
and is documented as "owned Vec-returning convenience APIs", so this is the
intended home. Namespace it `spring_wasm_core::owned::{module}::{fn}` and the
probe generator's swap becomes a one-line prefix change.

Prefer this over inlining Core call sequences per test: one wrapper per function
is less total code, is reusable outside parity, and can be reviewed against the
host binding it mirrors.

### 3.3 Wire protocols, confirmed by reading the host bindings

**Caller-owned variable output** (`WasmCoreGeneratedVariableOutputBindings.cpp`,
and the output half of `WasmCoreGeneratedVariableIoBindings.cpp`):

- The output descriptor is `N * 12` bytes — one `{ptr, capacity, required}`
  little-endian u32 triple per variable output field, in field order — followed
  by a fixed area for the non-variable outputs (`OutputLayout` in
  `render_core_wasm_variable_io_host.rs`: `descriptor_bytes`, `fixed_offset`,
  `fixed_bytes`, `fixed_alignment`).
- `capacity` is a **count** for lists and **bytes** for strings/bytes.
- On overflow the host writes `required` for **every** field *before* returning
  `BufferOverflow`. So one retry always suffices: call with capacity 0 to learn
  the sizes, allocate, call again. Do not loop more than that.
- A string payload is raw bytes, no NUL and no length prefix; the length is the
  `required` field. List elements are written contiguously with their fixed wire
  layout.

**Dynamic (recursively variable) output** (`WasmCoreGeneratedDynamicOutputBindings.cpp`):

- One 12-byte `{ptr, capacity, required}` descriptor, capacity in **bytes**.
- The payload is a schema-driven canonical stream: little-endian scalars,
  u32-length-prefixed strings/bytes/lists, records in semantic field order,
  fixed arrays in element order. Count fields that back a semantic list are not
  emitted separately.
- `option<T>` is a `u32` presence flag, and the payload follows **only when
  present** — the dynamic stream is length-driven, unlike the descriptor
  encoding below. This was added on 2026-08-21; see `render_field` in
  `render_core_wasm_dynamic_output_host.rs`.
- The decoder is the mirror of `render_type` / `render_field` in that file.
  Generate it from the same walk so the two cannot drift.

**Input descriptors** (`render_core_wasm_variable_io_host.rs` is the shared
lowering; `render_core_wasm_borrowed_host.rs` mirrors it for the zero-copy path):

- Variable input fields occupy an 8-byte `{ptr, len}` pair; fixed fields are
  written at their own wire layout; the whole descriptor is padded to its
  alignment and the host calls `reader.Finish(alignment)`.
- Borrowed strings are **NUL-terminated and validated**: the host views
  `len + 1` bytes and rejects a non-zero terminator. Guest wrappers must pass a
  `&CStr`, not a `&str`.
- `option<T>` on a descriptor is fixed-stride: a `u32` presence flag followed by
  an **always-reserved** payload. An absent option still occupies its slot; the
  host writes a zeroed value of the same shape.
- `option<string>` is presence flag + the same `{ptr, len}` pair, always read,
  with a null native pointer when absent.
- Records containing an option carry an explicit trailing `Align`. Records
  without options do **not** — that asymmetry is deliberate, to keep the
  existing encodings byte-identical. Do not "normalize" it without regenerating
  and re-running everything.

### 3.4 Harness integration

`test/native_api_parity/run_harness.py`:

- `WASM_CONTEXTS`, `WASM_CONTEXT_MODULES`, `WASM_CONTEXT_MANIFESTS`,
  `WASM_CONTEXT_MAP_MANIFESTS`, `WASM_ENVIRONMENT_NAMES` near the top.
- `ensure_wasm_built()` runs the probe generator per context, builds the guest
  for `wasm32-unknown-unknown --release`, then runs the `componentize` bin.
  **For Core, skip the componentize step** — the `.wasm` is the deliverable.
- `--mode` is `lua|native|wasm|both|compare`; `--wasm-context` selects the
  environment. Add the Core transport as a new context or a new mode; a new
  context is less invasive but a Core/CM *comparison* mode is what actually
  proves parity.
- Useful during bring-up: `--tests <ids>`, `--test-prefix N`,
  `--skip-callin-compare`, `--skip-wasm-build`, `--keep-workdir`.

Target: all cases green on Core in all five environments.

## 4. Where the parity plan stands

`python3 test/wasm_api/generate_core_parity_plan.py` (add `--strict` to gate):

```
tests_total                          1239
tests_core_executable                 951
tests_blocked                         288
tests_unmapped                        288
core_executable_callouts             1354
core_generated_callouts              1293
core_reviewed_handwritten_callouts     61
core_executable_callouts_with_oracle  515
missing_callouts_blocking_oracle_tests  0
```

`missing_callouts_blocking_oracle_tests` is **0** — no oracle test is blocked by
a missing transport any more. `tests_blocked == tests_unmapped == 288` means
every remaining block is the *probe scanner* failing to recover a semantic
callout from `probe_generated.rs`, not absent Core coverage. Fixing
`extract_probe_dependencies` (a regex walk over generated probe bodies) is a
cheap early win that will sharpen the plan before you write the façade.

`check_core_command_parity.py` passes: 9/9 oracle tests and 6/6 required
endpoints Core-executable.

## 5. Ground rules

These are inherited and still apply. The first two are the ones an agent
working alone gets wrong.

1. **Generator first, always.** A renderer change fixes a whole class and is
   reviewable as a template plus sample output. If you write the same binding
   shape a third time by hand, stop and change the renderer.
2. **Do not claim verification you did not perform.** Say "not compiled" and
   mean it.
3. **Never hand-edit `rts/wasm/generated/`.** `verify_codegen.py` compares the
   full output set; edits there are silently destroyed.
4. Handwritten bindings need a stated reason. "The generator did not handle it"
   is a reason to improve the generator.
5. Keep an explicit assumptions list — every guessed signature, field name,
   lifetime or environment mask.
6. Many small commits, not one large one.
7. Guest crates under `test/wasm_api/` are **outside the workspace**.
   `cargo build --workspace` does not compile them. If you touch the guest SDK,
   build every one of them for `wasm32-unknown-unknown`.

Commands the local agent runs:

```sh
cargo fmt --manifest-path rust/Cargo.toml --all --check
cargo build --manifest-path rust/Cargo.toml --workspace
cargo test  --manifest-path rust/Cargo.toml --workspace

cargo run --manifest-path rust/Cargo.toml -p spring-native-codegen \
  --bin spring-api-codegen -- --root . --output rts/wasm/generated --strict
python3 rts/wasm/verify_codegen.py

./docker-build-v2/build.sh linux
./docker-build-v2/build.sh --compile linux -t check
```

## 6. Suggested order

1. Fix `extract_probe_dependencies` so the 288 unmapped tests map. Cheap, and it
   tells you what the façade must cover before you build it.
2. `render_core_wasm_owned_guest.rs` emitting `core_owned.rs`, class by class,
   compiling the parity guest after each: re-export `fixed`/`fixed-option`
   first (338 functions, near-zero code), then `variable-output-caller-owned`
   (70 of 92 are single-output `Vec<T>`/`String`), then borrowed/variable-io
   inputs, then the dynamic decoder, then hand-map the 6 handwritten ones.
3. `--transport core` in `generate_probe.py`: swap the path prefix, skip WIT
   emission, emit Core callin exports instead of the `wit-bindgen` `Guest` impl.
   `render_bindings` and `render_context` are the two functions that are
   genuinely CM-specific.
4. A `wasm_core` context in `run_harness.py` that skips componentization.
5. Run it. Expect real bugs — this is the first time most of these bindings will
   have been called at all. That is the point.
6. Only then Phase 4 (delete Component Model), per §6 of the older handoff.

## 7. Invariants not to regress

Unchanged from `web_agent_handoff.md` §8. The ones this work touches most:

- Never use native C++ struct layout as a wire ABI — explicit little-endian.
- Do not partially fill list results; report the required size and let the
  caller retry.
- No nondeterministic data to synced guests except through `spring:desync`.
- Visibility is not sandbox security; safety is not security. Keep distinct.
- A reviewed handwritten transport owns its import outright. No generated
  renderer may emit a second binding for the same name
  (`handwritten_reviewed` in `render_core_wasm_registry.rs`).

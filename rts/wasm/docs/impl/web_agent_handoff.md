# Web-agent handoff: complete Core Wasm coverage, then delete Component Model

Date: 2026-08-21
Repo: `gajop/spring-rust`
Branch: `rust-wip`
Head: `c748c3013e`

Read this instead of `rts/wasm/docs/impl/handoff.md`. That document was written
before the branch was ever compiled and several of its "still open" items are
now closed. Where the two disagree, this one is current.

## 0. What changed since the previous handoff

`agent/wasm-core-fast-abi` has been squashed onto `rust-wip` and **made to
build, link, pass tests, and run benchmarks on real hardware**. Do not redo any
of the following; it is already committed.

- The Rust workspace compiles and its tests pass. `spring-wasm-core` had 60
  compile errors (three modules never imported `ApiError`/`ErrorCode`/`Result`)
  and a broken module structure (`benchmark.rs` and `unit_control.rs` pulled
  siblings in with `#[path]`, so `profiling.rs` was compiled twice and every
  re-exported name was ambiguous). Both fixed.
- `rts/wasm/generated/` now contains the 16 Core artifacts that were missing,
  and CMake actually compiles the generated Core translation units.
- Two generator bugs fixed: option bindings emitted `bool corePresent` once per
  option field in the same scope, and the Core renderers derived the
  `NativeInterface` member by camel-casing the module name, which is wrong for
  the eight groups that live under a sub-struct (`unit_control` is
  `syncedCtrl->unit`, not `unitControl`). All five Core renderers now share
  `render_host::native_api_path`.
- Engine and headless build; **38/38 engine tests pass**.
- `spring:desync` exists — see §4.
- Benchmarks run end to end across all six profiles and five backends, with
  frozen baselines so future runs can measure only the backend under work.

**Still true from the old handoff:** the `gaia_synced` parity probe drift is
real but *pre-existing* — it reproduces at merge-base `47c9407182`, so it is
not caused by this work. `verify_codegen.py` still fails on it.

## 1. The job

Three things, in this order. The order is not negotiable; §3 explains why.

1. **Callins: 9 → 126.**
2. **Callouts: reviewed executable coverage across all 1354 functions.**
3. **Parity coverage for Core**, then **delete Component Model**.

## 2. Where coverage actually stands

### Callouts

1354 NativeInterface functions total.

| state | count | share |
| --- | ---: | ---: |
| Core-executable | 930 | 68.7% |
| pending | 424 | 31.3% |

Executable splits as fixed 817, variable-output-caller-owned 100, fixed-option 13.

**Do not trust the 68.7% as "done".** "Executable" means the generator emitted
a binding in an executable class and the registry lists it. It does not mean a
human reviewed it, and it does not mean anything ever called it. Only **80**
entries exist in the handwritten reviewed registry (`WasmCoreRegistry.h`). So
roughly 850 bindings have never been exercised against a known-good result.
Closing that gap is what item 3 is for.

Pending, by reason:

| reason | count |
| --- | ---: |
| variable input awaits allocation-free/reviewed lowering | 316 |
| semantic/manual lowering required | 53 |
| unsupported Core shape | 38 |
| no executable generated fast-path renderer | 17 |

Pending is concentrated, which is good news — seven modules hold two thirds of it:

| module | executable | pending |
| --- | ---: | ---: |
| `rml_ui` | 102 | 78 |
| `gfx` | 165 | 72 |
| `vfs` | 3 | 49 |
| `unsynced_ctrl` | 60 | 22 |
| `messages` | 2 | 19 |
| `unit_control` | 69 | 19 |
| `rules_params` | 0 | 15 |

**The single highest-leverage change in this entire document is a
borrowed, allocation-free variable-input lowering in the generator.** One
renderer change moves 316 of the 424 pending functions. Do that before
hand-writing anything.

### Callins

126 planned by the generator, **9 implemented**: `GameFrame`, `GameFramePost`,
`Update`, `UnitCreated`, `UnitPreDamaged`, `AllowUnitCreation`,
`AddConsoleLine`, `CommandNotify`, `DrawWorld`.

118 remain. They are mostly easy — 82 of 118 take a direct query:

| query strategy | result strategy | count |
| --- | --- | ---: |
| direct | empty | 53 |
| direct | direct | 26 |
| variable-scratch | empty | 10 |
| variable-scratch | direct | 10 |
| fixed-wire | direct | 5 |
| fixed-wire | empty | 4 |
| direct | fixed-wire | 3 |
| empty | empty | 3 |
| direct | manual | 2 |
| variable-scratch | fixed-wire | 1 |
| fixed-wire | manual | 1 |

Aggregation modes across the 118: `ignore` 72, `or-true` 24, `and-false` 16,
`first` 4, `first-non-empty` 2. All five are already implemented in the
dispatcher, so no new aggregation semantics are needed.

The 23 variable-scratch callins (2 of which are done) are:

`ActiveCommandChanged`, `AddConsoleLine`✔, `AllowCommand`, `AllowResourceLevel`,
`AllowResourceTransfer`, `CommandFallback`, `CommandNotify`✔, `DownloadQueued`,
`DrawBuildSquare`, `GameID`, `GameOver`, `GameSetup`, `HandleLuaCall`,
`HandleLuaMsg`, `KeyPress`, `KeyRelease`, `MapDrawCmd`, `RecvFromSynced`,
`ResourceExcess`, `TextEditing`, `TextInput`, `UnitCmdDone`, `UnitCommand`

Start with the 53 `direct`/`empty` ones. They are the largest uniform block and
carry the least risk.

Plan data lives in `rts/wasm/generated/core-callin-plan.json`, one entry per
callin with query/result strategy, aggregation and environment mask.

### Test parity

**Zero Core coverage.** This is the most important fact in this document.

The parity harness is substantial and healthy — 866 cases across 76 spec files
in `test/native_api_parity/api_tests/`, exercising ~860 distinct NativeInterface
calls, split synced 507 / unsynced 283 / widget 76, plus 43 surface tests. It
compares each call against the Lua implementation.

But `test/wasm_api/parity_guest` is built with `wit-bindgen` + `wit-component`.
**It is a Component Model guest.** Every one of those 866 cases exercises CM,
not Core. `WASM_PARITY_CORE` in `run_harness.py` is a misleading name — it is
the raw module that gets fed *into* componentization, not a Core-ABI guest.

So Core today is well-benchmarked and largely unverified for correctness.

## 3. Why CM must not be deleted first

You have been told CM is inferior and should go. The benchmarks agree — on a
12th Gen i7-12700, dynamic CM costs 5-51× Core, and Core beats typed CM on most
rows. Nothing here defends keeping Component Model in the long run.

But **CM is currently the only correctness oracle Core has.** It is the only
transport the 866 parity cases run through. Delete it before Core has parity
coverage and you remove the only mechanism that would catch a Core binding
returning subtly wrong data — with ~850 unreviewed bindings in the tree, that
is not a hypothetical risk.

Sequence: **Core parity harness → Core green on all 866 cases → then delete CM.**

Two rows also still favour typed CM (`callout_biglist` 630 ns vs Core 1005 ns;
`callout_spatial` 352 ns vs Core 453 ns), so it earns its keep as a comparison
point until those close. Both look like implementation slack rather than a
transport limit — Core's own measured floor for those shapes is 492 ns and
224 ns respectively, so each sits about 2× above its own ceiling.

## 4. `spring:desync` — read before touching timers

Synced guests cannot see nondeterministic host data. That rule stands. But
debugging and benchmarking synced code needs a clock, so there is now an
explicit opt-in group:

- `rts/WasmInterface/WasmCoreDesyncBindings.{h,cpp}`, module `spring:desync`,
  `AllEnvironmentMask`, carrying `get-timer`, `get-timer-micros`, `diff-timers`.
- Rust SDK: `spring_wasm_core::desync`, deliberately **not** re-exported from
  the crate root, so reaching it means writing `desync::` at the call site.

The module name is the warning label. Importing one of these makes a synced
guest diverge from other clients, and a guest that wants to trade determinism
for a clock is entitled to. This is a **sync hazard only** — no sandbox escape,
no OS authority, no hidden game state. Do not put anything in this group that
would make it a safety, security or visibility exception, and do not "fix" the
sync policy by widening `spring:profiling` instead.

## 5. Rules for an agent that cannot compile

This is the part that most affects how you should work.

1. **Generator first, always.** A change to
   `rust/crates/spring-native-codegen/src/render_core_wasm*.rs` that fixes one
   template fixes every function in that class, and a human can review the
   template plus a sample of output. Ten thousand lines of hand-written C++
   cannot be reviewed and cannot be trusted from an agent that never ran the
   compiler. If you find yourself writing the same binding shape a third time,
   stop and change the renderer.
2. **Never hand-edit `rts/wasm/generated/`.** It is regenerated by
   `spring-api-codegen` and `verify_codegen.py` compares the full output set.
   Edits there are silently destroyed and will look like a mystery regression.
3. **Handwritten bindings need a stated reason.** They are for cases where
   generic lowering is materially slower or the lifetime contract needs to be
   explicit — like `GetUnitScriptNames`, which reads model-owned piece names
   directly to skip the NativeInterface scratch copy. "The generator did not
   handle it" is a reason to improve the generator.
4. **Keep an explicit assumptions list.** Every place you guessed at a
   NativeInterface signature, a field name, a lifetime, or an environment mask,
   write it down in the PR body. The local agent verifies those first. An
   unflagged guess that compiles is worse than one that does not.
5. **Do not claim verification you did not perform.** Say "not compiled" and
   mean it. The previous handoff's single most expensive omission was
   presenting unbuilt code as finished work.
6. **Prefer many small commits over one large one.** The last handoff arrived
   as 375 commits that had to be squashed and repaired as a unit; smaller
   reviewable steps would have surfaced the 60 compile errors far earlier.

Commands the local agent will run — write code that survives them:

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

**Note the guest crates under `test/wasm_api/` are outside the workspace.**
`cargo build --workspace` does not compile them, which is exactly why the
benchmark guest's breakage went unnoticed for so long. If you touch the guest
SDK, check every crate in `test/wasm_api/*/Cargo.toml` that depends on it.

## 6. Work plan

### Phase 1 — callins 9 → 126

`rust/crates/spring-native-codegen/src/render_core_wasm_callins.rs` already
plans all 126. Dispatch lives in `WasmCoreHost.cpp` (`Invoke` + the
`InvokeX` methods) and `WasmInterfaceSystemCore.cpp`.

1. The 53 `direct`/`empty` callins. Uniform shape, no result marshalling.
2. The 26 `direct`/`direct`. Result goes back as a packed scalar.
3. The 12 `fixed-wire` variants — deterministic little-endian records.
4. The 21 remaining `variable-scratch`. Follow the existing `AddConsoleLine` /
   `CommandNotify` protocol exactly: one cached guest-owned scratch region
   negotiated at bind time via `spring:callin/scratch-info`, one unchecked
   host→guest call per event, no per-event allocator call, explicit
   nested/re-entry guard.
5. The 3 `manual` results last.

Every callin needs its export name in `WasmCoreRegistry.h` with the correct
environment mask, and a guest export macro in the Rust SDK.

### Phase 2 — callouts to full reviewed coverage

1. **Borrowed variable-input lowering in the generator.** 316 functions.
   Borrow aligned guest memory directly on little-endian hosts; return
   `NotAvailable` on big-endian rather than silently misinterpreting bytes —
   `GetTeamUnitsByDefs` and `GetUnitArrayCentroid` already do this and are the
   pattern to follow. Only borrow where NativeInterface consumes the data
   synchronously and cannot retain the pointer across re-entry.
2. The 17 "no executable generated fast-path renderer" — find the missing
   renderer class and add it.
3. The 53 "semantic/manual lowering required" — these need judgement; handle
   them per module, smallest first.
4. The 38 "unsupported Core shape" — document why each is unsupported. Some
   should stay unsupported. Do not force them.

Prefer module order by pending count, but do `rules_params` (15 pending, 0
executable) and `messages` (19 pending, 2 executable) early: they are small,
heavily used, and currently almost entirely uncovered.

### Phase 3 — Core parity harness

This is the phase that unblocks CM deletion, and it is mostly a code-generation
problem, which suits a web agent well.

`test/wasm_api/parity_guest/generate_probe.py` is 3738 lines and emits:

- `wit/parity.wit`
- `src/probe_generated.rs`
- `src/probe_bindings.rs`
- `src/probe_context.rs`

All the hard parts — semantic type walking, record paths, output projection,
coverage selection — are transport-independent and already written. What is
CM-specific is the binding layer: WIT emission and `wit-bindgen` calls.

Add a Core mode that reuses the semantic machinery and emits probe bodies
against `spring_wasm_core` instead. The output is a plain core module, so the
componentization step in `run_harness.py` is simply skipped — the `.wasm` is
the deliverable. Then teach `run_harness.py` a `wasm_core` context alongside
the existing five.

Target: all 866 cases green on Core, in all five environments
(`synced_gadget`, `unsynced_gadget`, `gaia_synced`, `gaia_unsynced`, `ui`).

Expect real bugs here. That is the point — this is the first time most of those
930 bindings will have been called at all.

### Phase 4 — delete Component Model

Only after Phase 3 is green.

Engine:
- `wasmtime_component_*` uses live in `WasmModule.cpp`, `WasmRuntime.cpp`,
  `WasmTypedHostShims.cpp`.
- `WasmTypedHost.{h,cpp}`, `WasmTypedHostShims.cpp`.
- `WasmValue` and the serialization path — defined in `WasmHost.h` and
  `NativeInterface/NativeInterfaceEventClient.h`. The Core-first dispatch in
  `NativeInterfaceEventClient::DispatchWasmCallin` falls through to this today;
  once CM is gone the fallback goes with it.
- 58 generated `WasmHostAdapter*` translation units.

Rust: `rust/crates/spring-wasm`, `rust/crates/spring-wasm-typed-host`.

Tests/guests: `parity_guest`, `benchmark_guest`, `aggregation_guest`,
`value_guest`, `guest`, `allocator_guest`, `embed_component.py`.

Codegen: the WIT/adapter renderers (`render_wit.rs`, `render_wasm_sdk.rs`,
`render_host.rs`'s adapter output). **Careful:** `render_host.rs` also owns
`native_api_path`, which the Core renderers now depend on. Keep that table.

Benchmarks: drop the `wasm` and `wasm_rust_typed` columns from
`run_benchmarks_core.py`, and delete their frozen baselines under
`test/native_api_parity/frozen_benchmarks/`.

Be careful separating "Component Model" from "shared Wasm infrastructure".
`WasmRuntime`, `WasmModuleManifest`, `WasmResources`, `WasmEnvironment` and
`WasmInterfaceSystem` are shared and must survive; they merely mention
`WasmValue` or components today.

## 7. Benchmarks

Baselines are frozen at `test/native_api_parity/frozen_benchmarks/`, one CSV
per profile and backend, with `metadata.json` recording engine revision and
CPU. Re-measure only what you change:

```sh
RECOIL_BENCHMARK_BACKENDS=wasm_core \
  python3 test/native_api_parity/run_benchmarks_core.py --suite

RECOIL_BENCHMARK_FREEZE=1 ...   # record the measured backends as the new baseline
```

The report keeps all columns and prints a note naming which came from the
frozen store, so a partial run cannot pass stale numbers off as fresh.

Two cautions when reading `rts/wasm/docs/impl/benchmarking_results.md`:

- **Ratios are time**, `A vs B` = time_A / time_B. Higher means A is slower.
- **The callin rows are noisy** — roughly ±15% run to run, with a spread wider
  than the median. Do not read fine distinctions into them. The callout and
  heightmap rows are tight (`callout_scalar` is ±0.55 ns on 11 ns) and support
  real conclusions.

Callin benchmarks measure cold cache: a 64 MB walk runs before each dispatch
(`BenchmarkCallins.h:EvictCache`). Without it the tight loop keeps the cache
warm and reports numbers that never occur in real usage. Do not remove it, and
budget for the runtime cost — the suite needs a timeout far above the 30 s
default.

## 8. Invariants not to regress

- Core supported callins dispatch before `WasmValue` serialization.
- No per-call heap allocation in ordered Core callin fan-out.
- No extra guest allocator call for variable callins; one crossing per event.
- Never use native C++ struct layout as a wire ABI — explicit little-endian.
- No nondeterministic data to synced guests except through `spring:desync`.
- Never bypass UI visibility filtering for speed.
- Visibility is not sandbox security; safety is not security. Keep distinct.
- Do not partially fill list results; report required size and let the caller retry.
- `GetUnitIsTransporting` keeps its explicit boolean; never derive it from `count != 0`.
- Do not restore the redundant NativeInterface scratch copy in Core `GetUnitScriptNames`.
- Use outer event rows for Lua/Core variable-callin comparisons; inner rows are diagnostic.
- `SyncedConfiguration` orders by declared module order, not by name. Sorting
  the composed strings puts modules in name order and breaks the ordering
  contract across transports.

## 9. Definition of done

- `core-executable-coverage.json` reports 0 pending, or an explicit documented
  reason for each remaining entry.
- All 126 callins dispatch through Core with correct aggregation and
  environment masks.
- All 866 parity cases pass on Core in all five environments.
- `verify_codegen.py` passes, apart from the pre-existing `gaia_synced` probe
  drift — fix that too if you can, but do not let it block.
- No `wasmtime_component_*` symbol remains in `rts/`.
- Engine and headless build; `-t check` green.
- Benchmarks re-measured for Core and the report regenerated.

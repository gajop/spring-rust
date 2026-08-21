# Handoff: retire Component Model, finish Core, prove it was worth it

Date: 2026-08-22
Branch: `rust-wip`
Read with: `web_agent_handoff.md` (original phases), `core_parity_handoff.md`
(Phase 3 results — **contains one error, see §1**), `refactor_plan.md`
(structure work — **contains one error, see §7**).

This is the whole remaining program, ordered. Work top to bottom. Each section
says what "done" means. Do not skip ahead — later sections get smaller when
earlier ones land.

## 0. Before anything: commit and push

The purge/parity work is now in the working tree and must be committed and
pushed in coherent chunks before any further refactor work. Keep the generated
Core artifacts reproducible with `verify_codegen.py` after each commit.

## 1. Correction: unsynced and UI are NOT disabled

`core_parity_handoff.md` §0 and §4 claim `rules-unsynced`, `gaia-unsynced` and
`ui` are "runtime-disabled in `WasmEnvironment.cpp`" and need a policy
decision. **This is false.** The policy struct is:

```
{environment, name, synced, runtimeEnabled, permitsSimulationMutation}
```

For those three rows the `false` is the **`synced`** field. `runtimeEnabled` is
`true` for all five environments, and `TestWasmInterface.cpp:82` asserts
`IsRuntimeEnabled(UI)`. The previous agent misread column 3 as column 4.

**Do:** fix those two passages in `core_parity_handoff.md` so nobody else is
misdirected. There is no policy gate. Unsynced/UI parity is simply not done —
it is work, not a decision.

## 2. Purge the Component Model

Nothing depends on it. The parity oracle is **Lua** (`run_harness.py:921`,
`expected = semantic(lua_rows, "lua")`), with the native `.so` as a second
reference — CM was never an oracle. Core covers 1354/1354 callouts, 0 pending.

**Scale: ~235k LOC, of which ~229k is generated.** Only ~6.5k is hand-written.

Delete outright:

| Thing | LOC |
| --- | --- |
| `rts/wasm/generated/sdk/generated.rs` + `callins.rs` | 86,828 |
| 58 `WasmHostAdapter*` TUs + `WasmHostAdapter{,Support}.h` | 89,692 |
| `rts/wasm/generated/wit/` (85 files) | 24,215 |
| `test/engine/WasmInterface/Component*Fixture.h` (8) | 13,374 |
| `parity_guest` CM half + `benchmark_guest` | ~17,700 |
| codegen: `render_wit.rs`, `render_wasm_sdk.rs`, `render_host.rs`, `render_callins.rs` | 2,732 |
| crates `spring-wasm`, `spring-wasm-typed-host` | 1,976 |
| `WasmTypedHost.{h,cpp}`, `WasmTypedHostShims.cpp` | 1,027 |
| guests `guest`, `value_guest`, `allocator_guest`, `aggregation_guest` | 713 |

Needs actual surgery — these files survive:

- `WasmModule.cpp` — 357 CM hits / 2,854 lines. Biggest job, but one clean
  contiguous concern (`LiftComponentValue`, `CollectComponentExports`,
  `EncodeComponentString`, the value-budget code).
- `NativeInterfaceEventClient.cpp` — 180 hits / 3,070 lines. **Careful: this is
  pre-existing engine code, not ours.** It is where `WasmValue` and the
  Core-first dispatch fallback meet. Minimal, surgical edits only.
- `WasmRuntime.cpp` (57), `WasmHost.h` (35 in 130 lines — the `WasmValue`
  definition itself), `WasmInterfaceSystem.cpp` (29),
  `NativeInterfaceWasmAdapter.{h,cpp}`, `WasmModule.h`,
  `WasmInterfaceSystem.h`, `WasmCoreHost.h`, `WasmCoreUiCallinFilter.h`.

`WasmValue` is the spine. Killing that type is what makes the deletion
propagate; the rest is mechanical.

**Keep** (shared runtime infrastructure):
`WasmRuntime`, `WasmModuleManifest`, `WasmResources`, `WasmEnvironment`,
`WasmInterfaceSystem`. The former `native_api_path` is now owned by
`render_core_native.rs`; do not reintroduce `render_host.rs`.

Also drop the `wasm` and `wasm_rust_typed` columns from
`run_benchmarks_core.py` and their frozen baselines.

**Done:** the engine builds, Component files are purged, and both headless Core
parity contexts pass at 330 selected cases each with 0 mismatches and 0
vacuous rows. Graphics-backed contexts are bounded and recorded as display
environment blockers, not as green results.

## 3. Close the owned-façade gap

`rts/wasm/generated/sdk/core_owned.rs` currently emits **574
`UnsupportedHostTarget`** entries — callouts the owned guest physically cannot
call, so they can never be parity-tested regardless of transport coverage.
This, not the transport layer, is the real coverage ceiling.

Per `core_parity_handoff.md` §4 the groups are: dynamic/recursive output
decoders, variable-input descriptors, command and piece record lists, rules and
configuration string/list APIs, and mutating control callouts.

Extend `render_core_wasm_owned_guest.rs` from the shared Core field walk,
group by group, then reduce the manifest exclusions and rerun. Smallest group
first. This is generator work — never hand-edit `core_owned.rs`.

The generator’s coverage parser was corrected so a valid `wasm32` branch is
not mistaken for an unsupported wrapper merely because the non-Wasm fallback
contains `UnsupportedHostTarget`. This raised synced/Gaia-synced depth from
307 to 330 without manufacturing results.

**Current status:** the 574 literal fallback entries remain an explicit façade
worklist; unconditional entries are excluded from parity, while valid Wasm
branches are now eligible. Further façade reduction remains measurable work.

## 4. Finish parity coverage

Two separate gaps:

**Depth.** Synced currently selects **330 of 507** source tests. Find out what
the other 200 are — genuinely inapplicable, or excluded by manifest because of
§3 holes. Drive the number up as §3 lands.

**Breadth.** Three of five environments have never had a Core parity run:
`unsynced_gadget`, `gaia_unsynced`, `ui`. Per §1 nothing blocks this. Guests
already build. Run them:

```sh
python3 test/native_api_parity/run_harness.py \
  --spring-headless build-amd64-linux/install/spring-headless \
  --mode wasm --wasm-context <ctx> --wasm-transport core \
  --skip-native-build --skip-wasm-build --skip-callin-compare
```

Expect real bugs — unsynced and UI exercise paths synced never touches, and
`permitsSimulationMutation = false` means mutating callouts must be rejected
rather than silently no-op'd. Verify rejection is actually observable.

**Do not manufacture a green run by treating missing observations as success.**
The harness reports "vacuous results" for a reason; 0 vacuous is part of pass.

**Current status:** synced and Gaia-synced pass 330/330 with 0 vacuous. The
unsynced gadget, Gaia-unsynced, and UI runs reach the graphics-backed process
but time out in this no-display session before producing a report; their
outputs are preserved as blocked diagnostics. No empty result is counted green.

## 5. Prove the performance claim

This was priority #1 of the whole project and there is still no headline
number. `run_benchmarks_core.py` and frozen baselines exist
(`frozen_benchmarks/metadata.json`, bootstrapped 2026-08-20 on an i7-12700).

Run the bounded suite, produce a plain comparison table, and write it up:

```sh
python3 test/native_api_parity/run_benchmarks_core.py \
  --suite --bounded-suite \
  --spring-headless ./spring-headless --spring ./spring
```

Ratios that matter (per `CORE_WASM_BENCHMARKS.md`): Lua vs native, Lua vs Core,
Core vs native. After §2 the CM columns are gone, so re-freeze baselines.

**Done:** [core_benchmark_results.md](core_benchmark_results.md) states, in numbers, how Core compares to Lua
and to the native C API across callouts, callins, heightmap, workloads, memory
and draw. If Core is not winning, say so plainly — that is the finding, and it
matters more than any amount of green tests.

## 6. Security and process safety review

Priorities #3 and #4, never systematically addressed. Known pieces exist —
`ImportGuard` budget charging, the `CallbackGuard` nesting limit in
`DispatchRetainedCallback`, the 2048 import bound, synced modules requiring
`min == max` memory — but there is no document saying what a hostile guest
can and cannot reach.

**Done:** [core_security_review.md](core_security_review.md) enumerates memory limits, fuel/epoch interruption, callback
reentrancy, pointer/length validation on every variable-input descriptor, what
happens on trap, and what the guest can reach toward the OS (it should be
nothing — confirm no WASI). Note gaps; fix the cheap ones.

Keep the five policy concepts separate, as originally specified: performance /
deterministic synced correctness / process safety / guest→OS security /
unsynced-UI visibility.

## 7. Finish the refactor

`refactor_plan.md` §1 step 1 is partly done (ModuleSpec table,
`render/core/shared/wire.rs`, variable-I/O 1017→537). Remaining, now unblocked
by §2:

- generated-output per-module sharding + directory tree (§2 of that plan)
- `rts/WasmInterface/` directory tree (§3), if a move-only change is still
  worthwhile after the generated split
- semantic `generate_probe.py` module split (§4); its Core `probe/` package
  entry and compatibility CLI now exist
- `test/wasm_api/` grouping

**Correction to that plan:** its §2/§3 keep the 58 `WasmHostAdapter*` TUs alive
in a `host/adapter/` tree. They are CM and die in §2 above. Fix before
following it, or you will build structure for deleted files.

## 8. Ground rules

- Never hand-edit `rts/wasm/generated/`. Change an emitter, regenerate.
- Emitter restructuring that is not meant to change output must produce a
  byte-identical generated tree. Diff it as proof.
- Move-only commits stay move-only; splits are separate commits.
- After each step: `cargo fmt --manifest-path rust/Cargo.toml --all --check`,
  `cargo test --workspace`, all guest crates for `wasm32-unknown-unknown`,
  `./docker-build-v2/build.sh linux`, `--compile linux -t check`,
  `python3 rts/wasm/verify_codegen.py`, both parity gates.
- Native cmake configure is broken; use `./docker-build-v2/build.sh linux`.
- `verify_codegen.py` has one known pre-existing failure (`gaia_synced` probe
  drift). Anything else is yours.
- Commit as you go. Do not leave a 40-file working tree.

## 9. If you get stuck

Report and move on to the next section rather than blocking. Sections 3–6 are
independent of each other; only §2 gates §7. Leave a short note in this file
under the section you stopped in, saying what you tried and what you saw.

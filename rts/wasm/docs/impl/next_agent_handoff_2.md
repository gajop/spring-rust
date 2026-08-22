# Handoff round 2

Date: 2026-08-22
Branch: `rust-wip` (head updated through `6abf6e1e9b`; local follow-up in progress)
Supersedes `next_agent_handoff.md`, whose §0–§2, §5 and §6 are done.

Read with: `core_benchmark_results.md`, `core_security_review.md`,
`rust_sdk_design_notes.md`, `refactor_plan.md`.

Sections are independent unless stated. Work them in order, but if one blocks,
note what you saw and move to the next rather than stalling. Commit as you go;
do not leave a large working tree.

## 0. State

Done last round: Component Model purged (net −51,653 lines), env-policy
correction landed, security review written, benchmarks reframed, callin
dispatch short-circuited, environment modules and marker export added.
`verify_codegen.py` is green. The synced Core probe now selects 331 cases;
missing owned lowerings are omitted at generation time rather than reported as
runtime successes.

Not done: the remaining owned lowerings, unsynced/UI parity, and the refactor
remainder. The unsynced/UI harness currently produces no Wasm rows in the
graphics run and is recorded as a harness/content-discovery block below.

## 1. Housekeeping (do first, it is minutes)

- **Done.** The branch is pushed through `6abf6e1e9b`; the two orphaned SDK
  crates and their dangling references were removed in `0b378bcbad`.

## 2. Fix the benchmark write-up's framing

`core_benchmark_results.md` leads with *"it loses on most callout/callin
paths"*, measured against **native**. Native is an in-process C call with no
sandbox boundary; losing to it is expected and is not the interesting result.

**Lua is what Core replaces, so Lua vs Core is the headline** — and Core wins
nearly everywhere:

| profile | Core vs Lua |
| --- | --- |
| workloads | 4.0–24.6× faster |
| callouts | 1.1–6.8× faster |
| callins | 1.5–2.4× faster |
| draw callouts | ~5.3× faster |

The doc now leads with that, keeps the native column as a floor reference, and
reports the losses plainly (below). Also note that callin rows carry
±2–4 µs variance on 2–5 µs measurements and are near noise; the callout and
workload rows are tight and are the ones to quote.

## 3. Two real performance regressions

Both are visible in `/tmp/core-bench-bounded.md` (regenerate if gone; the
reproduction command is in `core_benchmark_results.md`).

**`callin_drawworld`: Core 5311 ns vs Lua 2731 ns** — Core is ~2× slower than
Lua and 4.2× slower than native. The only place Core loses to Lua by a real
margin. Profile it; find out whether the cost is per-callin dispatch, the UI
visibility context (`WasmUiVisibility::ScopedContext`), or argument marshalling
on the draw path.

**`callin_unimplemented`: Core 1483 ns vs Lua 1015 ns** — this is the path
where the guest does *not* implement the callin. It should be close to free.
Suspect dispatch work being done before the not-implemented check. Look at the
Core callin dispatch entry and make the negative case short-circuit as early as
possible — ideally a precomputed per-module bitmask of implemented callins,
checked before any argument work.

The dispatch now checks a precomputed implemented-callin bitmask before UI
visibility context setup or argument work. Drawworld still needs profiling;
the likely costs are `ScopedContext` and draw-path marshalling.

## 4. Close the owned-façade gap

`rts/wasm/generated/sdk/core_owned.rs` no longer emits the old 574 runtime
fallback entries. The real ceiling remains the set of owned callouts absent by
design: the generator omits a shape without a reviewed lowering, and the probe
generator excludes that symbol from selection with a manifest reason. This is
the semantic façade ceiling, not the transport ceiling.

Groups (per `core_parity_handoff.md` §4): dynamic/recursive output decoders,
variable-input descriptors, command and piece record lists, rules and
configuration string/list APIs, mutating control callouts.

Continue `render_core_wasm_owned_guest.rs` from the shared Core field walk, one
group at a time, smallest first. Reduce the manifest exclusions and rerun both
synced contexts after each group. Generator work only — never hand-edit
`core_owned.rs`. Current generation reports 1,144 automatic, 193 manual, and
17 unsupported transport plans; each omitted owned row needs a recorded shape
or adapter before it can enter parity.

**Done for this pass:** no runtime missing-wrapper sentinel remains and the
selected synced count is 331. **Final façade goal:** every callout is callable
or explicitly absent with a generated reason.

## 5. Unsynced and UI parity

The context runs now separate harness wiring from API mismatches:

- `synced_gadget`: 331 selected; 329 value passes; 2 expected error rows.
- `gaia_synced`: 331 selected; 329 value passes; 2 expected error rows.
- `unsynced_gadget`: 60 selected; 60 passes; zero vacuous rows.
- `ui`: 39 selected; 36 pass; 3 real mismatches.

The graphics run now loads the module and marker. UI mismatches are
`get_current_tooltip` (Core error 999) and two renderer-position values whose
Lua/Core coordinates differ. These are parity issues, not environment-disabled
results.

The earlier missing-marker failure was a build-artifact issue: the marker
function was dead-stripped when emitted only with `export_name`. The macro now
uses a retained `no_mangle` export. Keep copying the context-specific artifact
after each feature build; `--skip-wasm-build` does not do that copy.

The thin counts remain 60 / 60 / 39 versus 331 for synced because the canonical
context test sets are smaller, not because the runs are vacuous.

`permitsSimulationMutation == false` for all three. Mutating callouts must be
**observably rejected**, not silently no-op'd. Verify the rejection is visible
to the guest.

Do not manufacture green by treating missing observations as success. The
current result is a documented block, not a parity pass.

## 6. Environment selection in the guest SDK

Implemented for the Wasm side (see `rust_sdk_design_notes.md` §3); nothing
here depends on any open native question.

Today a guest crate has no environment awareness — env lives only in the
manifest, and a wrong pairing fails at runtime on some later frame.

The generator now emits per-environment modules in the generated SDK, each containing only the
callouts whose `environmentMask` includes that environment:

```rust
use spring_wasm_core::rules_synced as api;   // sim.wasm
use spring_wasm_core::ui as api;             // ui.wasm
```

An illegal callout is then simply absent → ordinary compile error. The mask is
already in `WasmCalloutRegistry.h`, so this is generator work.

**Do not use Cargo features for this.** Cargo unifies features across a build
graph, so a workspace holding both a synced and a UI crate resolves the SDK
once with the union and the guarantee disappears. Resolver v2 does not fix it
for normal deps across workspace members.

The code/manifest gap is closed: each env module emits a marker export
(e.g. `SPRING_ENV_MASK: u32`), and have the engine compare it against the
declared environment at load, failing closed with a message naming both sides.
Runtime mask checking stays regardless — the host never trusts the guest.

Three layers are present: compile (symbol absent) → load (marker vs manifest)
→ runtime (host mask check). Cargo features remain deliberately out of the
environment-selection path because feature unification would erase the split.

## 7. Generated documentation

Split by who maintains it.

**Generated, from `model.json`** — completed:

- full callout reference, per module
- per-callout environment table (from `environmentMask`)
- sync-safety annotation (synced-visible or not)
- which transport class each callout uses
- callable owned façade and transport metadata; absent lowerings are recorded
  by the coverage report rather than emitted as runtime stubs

Precedent in repo: `lua_functions.md`, `rust_functions.md`,
`api_surface_audit.md`.

**Hand-written** — placeholder skeleton exists: install,
quickstart, environment model, sync rules, debugging. A human rewrites the
prose later.

Style for anything you write here: tables and bullet fragments only; no
sentence longer than one line; no adjectives; no intro or summary paragraphs;
facts only. This constraint is deliberate — do not "improve" it into prose.

## 8. Sync verification, first two rungs

Approach is a regression gate, not a proof (`rust_sdk_design_notes.md` §6).
Two structural advantages already hold: wasm floats are IEEE-754 exact and
deterministic by spec, and the environment mask means a synced module cannot
import nondeterministic callouts.

The two cheap rungs are wired:

1. **Same binary, same replay, N runs, hash sim state per frame.** Catches
   intra-binary nondeterminism. Wire it as a test.
2. **Audit the mask.** Generate a table of every synced-visible callout and
   whether it is actually deterministic. Flag anything doubtful. This is a
   generated artifact plus a human review pass — produce the artifact and mark
   your own suspicions; do not silently decide.

No replay stream is checked in yet, so rung 1 is a reusable gate rather than a
claim of a completed engine replay. Cross-platform CI remains rung 3.

## 9. Distribution check

Goal is that users build games without building the engine.

Wasm already satisfies this. The native investigation **verified the loader
shape**: a guest
`.so` needs only ABI bindings and not the engine binary — symbols resolving at
load time from the host. ABI stability and native environment policy remain
open. No crate restructure or publication was made.

## 10. Finish the refactor

`refactor_plan.md` remainder, now unblocked by the CM purge:

- generated-output per-module sharding + directory tree
- `rts/WasmInterface/` directory tree
- `generate_probe.py` → package
- `test/wasm_api/` grouping

Its §2/§3 still describe a `host/adapter/` tree for the 58 `WasmHostAdapter*`
TUs. **Those no longer exist** — they were deleted with the Component purge.
Update the plan before following it.

## 11. Ground rules

- Never hand-edit `rts/wasm/generated/`. Change an emitter, regenerate.
- Emitter restructuring not intended to change output must produce a
  byte-identical generated tree. Diff it as proof.
- Move-only commits stay move-only; splits are separate commits.
- After each step: `cargo fmt --manifest-path rust/Cargo.toml --all --check`,
  `cargo test --workspace`, guest crates for `wasm32-unknown-unknown`,
  `./docker-build-v2/build.sh linux`, `--compile linux -t check`,
  `python3 rts/wasm/verify_codegen.py`, both parity gates.
- Native cmake configure is broken; use `./docker-build-v2/build.sh linux`.
- `verify_codegen.py` is currently fully green. Any failure is yours.
- Report losses and blocks plainly. A regression reported honestly is worth
  more than a green table.

## 12. Out of scope

Do not decide or implement these; they are open questions for the human:

- whether the environment model should apply to native at all
- whether native and wasm should present identical Rust APIs, and what
  "identical" means given native has threads and the crate ecosystem
- ABI stability policy and publishing to crates.io
- whether native is in scope for any sync guarantee

If work in §6–§9 runs into one of these, stop at the boundary and write down
the choice that would need making.

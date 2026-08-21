# Handoff round 2

Date: 2026-08-22
Branch: `rust-wip` (head `57a7e09c75`)
Supersedes `next_agent_handoff.md`, whose §0–§2, §5 and §6 are done.

Read with: `core_benchmark_results.md`, `core_security_review.md`,
`rust_sdk_design_notes.md`, `refactor_plan.md`.

Sections are independent unless stated. Work them in order, but if one blocks,
note what you saw and move to the next rather than stalling. Commit as you go;
do not leave a large working tree.

## 0. State

Done last round: Component Model purged (net −51,653 lines), env-policy
correction landed, security review written, benchmarks run. `cargo test
--workspace` green, `verify_codegen.py` fully green, both synced parity
contexts pass at 330 selected / 0 mismatches / 0 vacuous.

Not done: the owned-façade gap, unsynced/UI parity, the refactor remainder.

## 1. Housekeeping (do first, it is minutes)

- **Push the branch.** Three commits of real work exist only in this working
  copy. Nothing is pushed and the old remote branches were deleted.
- **Delete `rust/crates/spring-wasm/`** — orphaned: not in workspace members,
  nothing depends on it, 672 lines.
- **Delete `rust/crates/spring-wasm-typed-host/`** — now an empty directory.
- Confirm no `Cargo.lock` / CMake references dangle afterwards.

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

Rewrite the doc to lead with that, keep the native column as a floor reference,
and keep reporting the losses plainly (below). Also note that callin rows carry
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

The second is likely the cheaper fix and helps every callin profile.

## 4. Close the owned-façade gap

`rts/wasm/generated/sdk/core_owned.rs` still emits **574
`UnsupportedHostTarget`** entries — unchanged from last round. These are
callouts the owned guest cannot call at all, so they can never be parity-tested
regardless of transport coverage. This is the real coverage ceiling.

Groups (per `core_parity_handoff.md` §4): dynamic/recursive output decoders,
variable-input descriptors, command and piece record lists, rules and
configuration string/list APIs, mutating control callouts.

Extend `render_core_wasm_owned_guest.rs` from the shared Core field walk, one
group at a time, smallest first. Reduce the manifest exclusions and rerun both
synced contexts after each group. Generator work only — never hand-edit
`core_owned.rs`.

**Done when:** the count is materially down and selected-case counts have
risen above 330.

## 5. Unsynced and UI parity

Three contexts have never produced a green Core run: `unsynced_gadget`,
`gaia_unsynced`, `ui`. Nothing gates this — all five environments have
`runtimeEnabled == true`.

Last round reported them "environment-blocked in this headless session".
Establish what that actually means: whether `ui`/draw genuinely need a
graphics context, or whether the harness simply was not wired for them. If a
display is required, note exactly which rows need it and run everything else.

Note the probe counts are thin: 60 / 60 / 39 tests versus 330 for synced. Find
out why — likely §4 holes plus mask-gating — and raise them.

`permitsSimulationMutation == false` for all three. Mutating callouts must be
**observably rejected**, not silently no-op'd. Verify the rejection is visible
to the guest.

Do not manufacture green by treating missing observations as success; 0 vacuous
results is part of pass.

## 6. Environment selection in the guest SDK

Decided design (see `rust_sdk_design_notes.md` §3). Wasm-side only; nothing
here depends on any open native question.

Today a guest crate has no environment awareness — env lives only in the
manifest, and a wrong pairing fails at runtime on some later frame.

**Emit per-environment modules in the generated SDK**, each containing only the
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

Then close the code/manifest gap: have each env module emit a marker export
(e.g. `SPRING_ENV_MASK: u32`), and have the engine compare it against the
declared environment at load, failing closed with a message naming both sides.
Runtime mask checking stays regardless — the host never trusts the guest.

Three layers: compile (symbol absent) → load (marker vs manifest) → runtime
(host mask check).

## 7. Generated documentation

Split by who maintains it.

**Generated, from `model.json`** — do this part:

- full callout reference, per module
- per-callout environment table (from `environmentMask`)
- sync-safety annotation (synced-visible or not)
- which transport class each callout uses
- current `UnsupportedHostTarget` list, so users can see what is not yet callable

Precedent in repo: `lua_functions.md`, `rust_functions.md`,
`api_surface_audit.md`.

**Hand-written** — write only a labelled placeholder skeleton: install,
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

Do the two cheap rungs now:

1. **Same binary, same replay, N runs, hash sim state per frame.** Catches
   intra-binary nondeterminism. Wire it as a test.
2. **Audit the mask.** Generate a table of every synced-visible callout and
   whether it is actually deterministic. Flag anything doubtful. This is a
   generated artifact plus a human review pass — produce the artifact and mark
   your own suspicions; do not silently decide.

Cross-platform CI (Linux + Windows replay checksums) is rung 3; set it up only
if the first two are clean and you have time.

## 9. Distribution check

Goal is that users build games without building the engine.

Wasm already satisfies this. For native, **verify** the claim that a guest
`.so` needs only ABI bindings and not the engine binary — symbols resolving at
load time from the host. Report what you find. Do not restructure crates or
publish anything; this is an investigation.

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

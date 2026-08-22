# Core-Wasm completion record

Date: 2026-08-22
Branch: `rust-wip`; implementation work is pushed, with external execution
and human review boundaries called out below

Previous handoffs are deleted; they are done. Standing reference docs:
`core_abi_contract.md`, `core_parity_handoff.md`, `core_security_review.md`,
`rust_sdk_design_notes.md`, `refactor_plan.md`.

Sections are independent. If one blocks, write down what you saw and move on.
Commit as you go.

## State

Core is the only transport. 1354/1354 callouts reachable from the environment
SDK, enforced by `verify_owned_environment_surface()` in `verify_codegen.py` —
regeneration cannot reduce the surface. No runtime missing-wrapper sentinel
exists. Synced and Gaia-synced parity pass at 335 probes each. UI parity
passes at 39 probes with zero vacuous results. Unsynced and Gaia-unsynced
parity pass at 68 probes each after the reviewed read-only expansion.

## 0. CI is green

The failing codegen clippy step in `.github/workflows/spring-native.yml:60`
was fixed and the exact gate now passes locally:

```sh
cargo clippy --manifest-path rust/Cargo.toml --package spring-native-codegen \
  --all-targets -- --deny warnings
```

The fixes remove five unused imports, one unused model import, three invalid
doc-comment blank lines, one unnecessary `map_or`, and one recursive-only
parameter. `cargo clippy --package spring-native-codegen --all-targets
-- --deny warnings` passes.

`--deny warnings` means any new lint fails the build, so a clean local
`cargo test` proves nothing. CI also runs on `windows-2022`, where a lint can
fire that did not fire locally.

## 1. UI parity: complete

`ui` runs 39 probes and passes with zero vacuous results.

Root causes: `get_current_tooltip` discarded non-UTF-8 Lua bytes, and the
renderer-position wrapper passed options through the wrong ABI shape. Core was
wrong in both cases; the probe was not bent to accept either result.

Fixes: byte-preserving Lua string decoding and generated fixed-record option
marshalling. The final run compares 39/39 with zero vacuous results.

## 2. Parity depth

Selected cases against available cases:

| context | selected | source | |
| --- | ---: | ---: | ---: |
| `synced_gadget` | 335 | 507 | 66% |
| `gaia_synced` | 335 | 507 | 66% |
| `unsynced_gadget` | 68 | 283 | 24% |
| `gaia_unsynced` | 68 | 283 | 24% |
| `ui` | 39 | 76 | 51% |

About half the available cases run. `unsynced` remains the outlier at 24%,
but its selected runtime stream now passes.

The unselected cases are accounted for by generator limitation, manifest
policy, or genuine inapplicability; they are not silently moved out of the
denominator.

The increase covers `platform` read-only metadata and reviewed
`system_control` metadata. For unsynced, the 215 unselected cases break down
as: 113 rendering-disabled, 49 no-native-function, 23 owned-surface gaps, 19
policy exclusions, 6 no-Lua-runtime, 4 unsupported-output, and 1 mutating or
unsupported getter. Profiling remains excluded because its legacy handwritten
ABI disagrees with the generated signature.

0 vacuous results remains part of pass. Never treat a missing observation as
success.

## 3. `callin_drawworld` remains a measured regression

The one remaining performance regression. The bounded rerun after the
dispatch/visibility cleanup measured Core 5150 ns ± 3114 versus Lua 2520 ns ±
2686. Core still loses this row. Every other headline profile has Core ahead of
Lua.

`callin_unimplemented` was fixed by short-circuiting missing callins; this one
was not. It was profiled and the result is recorded in
`core_benchmark_results.md`. Candidates remain per-callin dispatch cost, the
UI visibility context (`WasmUiVisibility::ScopedContext`), or argument
marshalling on the draw path.

Note callin rows carry ±2–4 µs variance on 2–5 µs measurements. Confirm the
regression is real before optimising, and confirm any fix against the same
noise.

Reproduction is in `core_benchmark_results.md`.

## 4. Refactor remainder

`refactor_plan.md`, now much cheaper — the SDK shrank from 311k to 117k lines
and the 58 `WasmHostAdapter*` TUs no longer exist:

- generated-output per-module sharding and directory tree — complete for the
  Rust owned façade and fixed C++ Core bindings
- `rts/WasmInterface/` directory tree — complete: runtime, core/host,
  core/bindings, and system
- `generate_probe.py` → package compatibility entry — complete
- `test/wasm_api/` grouping

`core_owned.rs` is now a generated prelude with 55 per-module shards. The
fixed C++ dispatcher is small, with 55 per-module translation units discovered
by CMake. The probe package now owns the shared type and metadata semantics in
`probe/types.py` and `probe/model.py`; its compatibility entry and generated
output remain unchanged.

## 5. Sync verification rung 3

Rungs 1 and 2 exist: `test/wasm_api/check_sync_replay.py` (same binary, same
replay, three runs, exact per-frame equality) and
`generated_synced_callout_audit.md` (heuristic inventory, human review still
required).

Rung 3 is cross-platform: headless Linux, arm64 Linux, and Windows, same
fixture, compare canonical per-frame checksums derived from exported sync
observations. One GitHub Actions workflow is wired. The workflow has not been
run here because the replay asset and three external runners are required.
Treat a desync as a failing test naming the frame, not as a proof obligation.

The audit in `generated_synced_callout_audit.md` remains explicitly heuristic
and needs a human pass. `generated_synced_callout_review_list.md` surfaces the
568 candidate and 175 review-required rows as a short reviewable list rather
than 753 lines. No deterministic safety claim is made from the heuristic.

## 6. Documentation

`rts/wasm/docs/generated/core_api_reference.md` exists at 1,676 lines.
`rts/wasm/docs/core_sdk_user_guide_placeholder.md` is filled with install,
quickstart, environment, sync, and debugging fragments.

Generated reference — covers, per callout: environments, transport class,
signature, and mutating flag. It has no not-implemented section.

Hand-written guide — tables and bullet fragments: install, quickstart,
environment model (one module per environment, `use
spring_wasm_core::rules_synced as api`, the `SPRING_ENV_MASK` marker), sync
rules, debugging. A human rewrites the prose later.

Style, deliberately: tables and bullet fragments only; no sentence longer than
one line; no adjectives; no intro or summary paragraphs; facts only. Do not
improve this into prose.

## 7. Out of scope

Open questions for the human. Do not decide or implement:

- whether the environment model should apply to native
- whether native and wasm should present identical Rust APIs, given native has
  threads and the crate ecosystem
- ABI stability policy and publishing to crates.io
- whether native is in scope for any sync guarantee

Stop at the boundary and write down the choice that would need making.

## Ground rules

- Never hand-edit `rts/wasm/generated/`. Change an emitter, regenerate.
- Never emit a stub that fails at runtime. If something cannot be generated,
  it is a bug to fix, not a case to skip.
- Regeneration must never reduce the reachable API surface.
- Restructuring not meant to change output must produce a byte-identical
  generated tree. Diff it as proof.
- After each step, and always before pushing:
  `cargo clippy --manifest-path rust/Cargo.toml --package spring-native-codegen
  --all-targets -- --deny warnings`,
  `cargo fmt --manifest-path rust/Cargo.toml --all --check`,
  `cargo test --workspace`, guest crates for `wasm32-unknown-unknown`,
  `./docker-build-v2/build.sh linux`, `--compile linux -t check`,
  `python3 rts/wasm/verify_codegen.py`, both parity gates.
- Native cmake configure is broken; use `./docker-build-v2/build.sh linux`.
- `verify_codegen.py` is green. Any failure is yours.
- Never push with CI red. If a push turns CI red, fixing it is the next task,
  ahead of whatever you were doing.
- Report losses and blocks plainly.

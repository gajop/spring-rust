# Open tasks

Date: 2026-08-22
Branch: `rust-wip`, head `d96b50192c` at the start of this task; verification
and the follow-up push are tracked below.

Reference docs: `core_abi_contract.md`, `core_security_review.md`,
`core_benchmark_results.md`,
`rust_sdk_design_notes.md`, `refactor_plan.md`.

## Done — do not redo

Core is the only transport; the Component Model is gone. 1354/1354 callouts
are reachable from the environment SDK, enforced by
`verify_owned_environment_surface()` in `verify_codegen.py`. No runtime
missing-wrapper sentinel. Environment modules with `SPRING_ENV_MASK` validated
at load. All five parity contexts pass with zero vacuous results. CI green on
Ubuntu and Windows. `rts/WasmInterface/` and the generated C++/Rust output are
sharded. Sync rungs 1 and 2 exist. Generated API reference and a placeholder
user guide exist.

---

## 1. PERFORMANCE — this phase

Implementation status: the comparison/loss/diagnostic/unpaired artifacts and
the repeat-count validation are now generated under `docs/generated/`. Wide
arguments and payload-scaling rows are present. Full inventory callin
coverage remains open; the runner continues to report unmeasured rows as
unpaired rather than fabricating green results.

### 1a. Make the report readable — historical defect

The generated report was 19 columns wide and unreadable. This is a
correction to the previous ask, which wanted `n`, `p50`, `p99` and `spread`
per backend **in the table**. That was wrong. Fix it as follows.

**Split it into three things.**

**1. The comparison table** — `docs/generated/benchmarking_results.md`.
One row per test, eight columns, nothing else:

| Profile | Test | Lua | Native | Core | Core vs Lua | Core vs Native | Lua vs Native |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |

- one number per backend, the median, with its unit
- `Core vs Lua`, `Core vs Native`, and `Lua vs Native` as numeric ratios
- **only rows where Lua, Native and Core were all measured.** A row with an
  empty column does not belong in a comparison table.

Nothing else goes in this file.

**2. The losses** — a short generated section or file listing only the rows
where Core does not beat Lua. That list is the point of the whole exercise and
should be readable in five seconds without scanning a wide table.

**3. Everything else moves out of the comparison.**

- `core_ceiling_*` rows are Core-only transport diagnostics with no Lua or
  Native peer. They are not comparisons and must not sit in the comparison
  table. Separate file.
- `callin_drawworld_*` stage rows are instrumentation of one callin, not
  independent tests. Separate file. They also come from a separate engine run:
  a stage timer nests inside the timed region of the callin row it decomposes,
  so a run cannot report both an honest headline number and a breakdown.
- `n`, `p99` and `spread` go in the CSVs under
  `test/native_api_parity/frozen_benchmarks/`, which is where per-sample detail
  belongs. Keep them there; keep them out of the markdown.
- rows with missing backends (`heightmap`, `memory`, several `draw` rows) go
  wherever is honest, but not into a table whose purpose is A-vs-B.

**Test for whether this is done:** a person who has not read any of these
documents can open the comparison table and answer "where does Core lose to
Lua" in under ten seconds.

### 1b. Fix the sample counts

Every callout row has **n=5**. A p99 or spread from five samples is
meaningless, and `callout_payload_256` — currently the one row showing Core
losing on payload size — rests entirely on it.

Counts are also uneven across backends in the same row: Lua n=533 against
Native n=5033; `callout_draw` has Native n=100000 against Core n=5. Comparing
medians taken at different sample counts is not sound.

Set a single sample count per row, applied to every backend in that row, large
enough that the spread is small relative to the difference being claimed.
Re-run before anything is concluded from the payload curve.

### 1c. Wide arguments and large structs — the priority

This is where a hand-rolled wire protocol should lose to Lua tables, and every
existing row avoids it. Mean input count is 2.1; the tail is the point.

Widest inputs: `gfx::BlitFBO` and `gfx::UploadTexture` (12),
`ground_decals::SetGroundDecalQuadPosAndHeight` (10), `gfx::CopyToTexture`,
`gfx::DrawGroundQuad`, `icons::AddUnitIcon` (9), `gfx::CreateShader`,
`gfx::TexRect`, `system_control::GarbageCollectCtrl`,
`terrain_control::SetTerrainTypeData` (8).

Widest outputs: `gfx::GetFontInfo` (10), `unit_defs::GetUnitDefByID` and
`terrain::GetGroundInfo` (9), `terrain::GetTerrainTypeData` and
`profiling::GetLuaMemUsage` (8).

Largest records: `WaterParams` (34 fields), `GameRulesInfo` (27),
`TeamStatsHistoryPoint` (19), `TeamResources` (18), `MoveTypeData` (18),
`LightParams` (17).

Add **payload-scaling rows** — the same call at several list/struct sizes — so
the marshalling curve is in the data, not a single point.

### 1d. Finish the drawworld attribution

The latest paired UI run measured nested stages:

| stage | ns |
| --- | ---: |
| complete boundary | 9609 |
| native dispatch boundary | 8820 |
| core selection | 461 |
| core aggregation | 5999 |
| module dispatch | 4762 |
| Wasmtime entry / empty guest body | 2368 |
| visibility context | 116 |
| argument marshalling | 0 — DrawWorld has no arguments |

These measurements are nested and must not be added. The old 3822 ns figure
was a subtraction of incomparable boundaries. Current differences bound the
remaining work to dispatch/binding intervals: complete minus native dispatch
is about 789 ns, aggregation minus module dispatch about 1237 ns, and module
dispatch minus Wasmtime entry about 2394 ns.

Lua `empty` is a synced GameFrame fixture; Lua `drawworld` is a UI
DrawWorld fixture. They are not equivalent workloads. Use the paired UI rows
for the comparison and do not attribute their difference to special-casing.

### 1e. Deliverable

`docs/generated/benchmarking_results.md`, regenerated, containing every callin, every
transport class, and the wide-argument and payload-scaling rows, each with a
Lua column. Plus an explicit machine-generated list of every row where Core
does not beat Lua.

No hand-written numbers anywhere. If a number is in a document, a script put
it there.

## 2. Parity depth

Selected against available (the two synced contexts have identical selection
and the two Gaia contexts have identical selection):

| context | selected | source | |
| --- | ---: | ---: | ---: |
| `synced_gadget` | 335 | 507 | 66% |
| `gaia_synced` | 335 | 507 | 66% |
| `unsynced_gadget` | 68 | 283 | 24% |
| `gaia_unsynced` | 68 | 283 | 24% |
| `ui` | 39 | 76 | 51% |

The unselected cases are categorised, not hidden. Exact breakdowns from the
current manifests:

| context family | core-owned | policy | deferred | no Lua | no native | rendering | mutating | output | unclassified | total unselected |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| synced / Gaia synced | 152 | 13 | 2 | 1 | 0 | 2 | 0 | 2 | 0 | 172 |
| unsynced / Gaia unsynced | 23 | 19 | 0 | 6 | 49 | 113 | 1 | 4 | 0 | 215 |
| UI | 26 | 3 | 0 | 0 | 0 | 1 | 7 | 0 | 0 | 37 |

The unsynced rendering-disabled, no-native-function, policy, no-Lua-runtime,
and mutating rows are inapplicable by the current environment rules. The
actionable remainder is the 23 owned-surface gaps plus four unsupported-output
cases. The exact owned gaps are recorded in the generated coverage JSON; the
largest groups are config accessors, player/replay metadata, profiler record
helpers, visible projectiles/icons, and `solve_nurbscurve`.

**Actionable remainder:**

- 23 owned-surface gaps
- 4 unsupported-output cases
The profiling disagreement was real and is now fixed: generated Core expects
the buffer/status signatures in `core-abi.json`, and the handwritten host plus
specialized SDK now use those same signatures. Production import authorization
for the generic profiling namespace remains a separate capability-policy
decision; it is not being mislabeled as an ABI blocker.

The synced and UI splits are included in the table above; they are not being
treated as unexplained parity failures. Zero vacuous results stays part of
pass.

## 3. Refactor leftovers

- **`generate_probe.py` split:** selection and projection are real modules;
  `probe/core.py` is now 2894 lines and all five manifests regenerate
  reproducibly. Expressions/rendering remain coupled and are not claimed
  complete.
- **`test/wasm_api/` is grouped** into `guests/`, `tools/`, and `data/`, with
  stale workflow/docs paths updated.
- **Dead Component-Model leftovers** were removed from the tracked tree:
  `embed_component.py` and the empty `aggregation_guest/`, `allocator_guest/`,
  `guest/`, and `value_guest/` shells.

## 4. Sync rungs 1–3 — local run complete, hosted run pending artifact

Rung 1 is `test/wasm_api/tools/check_sync_replay.py`: same binary, same
replay, three runs, exact per-frame checksum equality; empty or missing streams
fail. Rung 2 is the generated environment/transport audit with the short
human-review list retained in `generated_synced_callout_review_list.md`.
The audit is heuristic and makes no determinism claim until a person reviews
the doubtful rows.

The cross-platform workflow (headless Linux, arm64 Linux, Windows; same
fixture; canonical per-frame checksums) is wired. The local rung-3 headless
run against the checked-in fixture passed with matching guest/reference
streams for all five contexts and zero reported probe failures. The hosted
three-platform job still requires a successful engine-build artifact for the
tested pushed SHA; it has not been falsely marked green locally. A synthetic
checksum stream is not an acceptable substitute. Desync = failing test naming
the frame.

## 5. Concurrent-session incident audit

The concurrent-session incident was audited before this task. Three questions
were checked:

- `render_core_wasm_owned_guest.rs` still contains `decode_core_string` and
  the shard refactor after fmt; no damage was found.
- The nine reverted files were byte-identical to the baseline at the time of
  the audit, and no in-flight work from this branch was present in them; the
  clippy fixes were already committed in `1f96da955f`.
- `probe_generated.rs` is 5007 lines both at the baseline path and now; the
  reported -4657 was a stale path/baseline comparison. Regeneration is
  reproducible and verifier-clean.

These checks are answered above. The incident audit is not part of the active
implementation document set.

## 6. Needs a human, not an agent

- **`generated_synced_callout_review_list.md`** — 568 candidate and 175
  review-required rows from a heuristic scan. No determinism claim can rest on
  it until a person reviews it. This gates any real sync guarantee.
- **`core_sdk_user_guide_placeholder.md`** — fragments are in place; the prose
  rewrite is yours.
- The four open questions in §7 below.

## 7. Out of scope — decisions for the human

Do not decide or implement:

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
- Do not report a task complete when a shim or partial extraction stands in for
  it. Partial is fine; say partial.
- After each step, and always before pushing:
  `cargo clippy --manifest-path rust/Cargo.toml --package spring-native-codegen
  --all-targets -- --deny warnings`,
  `cargo fmt --manifest-path rust/Cargo.toml --all --check`,
  `cargo test --workspace`, guest crates for `wasm32-unknown-unknown`,
  `./docker-build-v2/build.sh linux`, `--compile linux -t check`,
  `python3 rts/wasm/verify_codegen.py`, all five parity contexts.
- Native cmake configure is broken; use `./docker-build-v2/build.sh linux`.
- Never push with CI red. CI also runs `windows-2022`; a lint or path
  assumption can fire there and not locally.
- Report losses and blocks plainly.

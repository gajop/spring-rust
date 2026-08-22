# Open tasks

Date: 2026-08-22
Branch: `rust-wip`, head `d96b50192c` at the start of this task; verification
and the follow-up push are tracked below.

Reference docs: `core_abi_contract.md`, `core_parity_handoff.md`,
`core_security_review.md`, `core_benchmark_results.md`,
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

Performance was priority #1 of the whole project and remains the first
decision gate. The expanded run below is still a representative sample, not
permission to call the whole surface green:

| | exist | benchmarked | |
| --- | ---: | ---: | ---: |
| callins | 179 | 8 | **4%** |
| callouts | 1354 | 15 | **1.1%** |

`callin_drawworld` is not "the one bad row". It is the one bad row **in a 5%
sample**. Assume there are others and go find them.

### 1a. Fix or explain `callin_drawworld`

The current five-repeat run is Core 6166 ns (spread 3749, p99 10395) versus
Lua 2322 ns (spread 2834, p99 5200). It remains the only loss in the eight
measured callin rows, but it is a noisy 2–7 µs operation.

The stage run measured Core Wasmtime entry at 2191 ns and the diagnostic
visibility context at 153 ns. DrawWorld has no arguments and no return value,
so its ABI marshalling contribution is 0 bytes / no argument marshalling; the
remaining time is host dispatch plus the empty guest body. The context stage
is not the dominant cost. No optimization is claimed within the observed
spread; repeat the run before changing the hot path.

This time, produce an **explanation with evidence**, not another attempt.
Instrument the path and attribute the time: how much is Wasmtime entry, how
much is the visibility context, how much is marshalling, how much is the
actual draw work. A number per stage. Then either fix it or state which stage
is irreducible and why.

±2–4 µs spread on a 2–5 µs row: any claimed fix must beat that noise across
repeated runs, not one sample. Do not report a within-noise change as a win.

### 1b. Benchmark every callin

All 179, not 9. The current set includes synthetic rows (`empty`,
`unimplemented`, `4modules`) that measure dispatch, not real work.

The authoritative event inventory is 169 `SETUP_EVENT` entries / 163 loaded
native symbols; this phase measured the eight runnable benchmark rows plus the
draw row separately. The variable-callin fixture was not counted: it does not
stimulate `AddConsoleLine` in the current headless run, so treating its missing
row as zero would violate the no-vacuous-results rule. Group the remaining
inventory by argument shape before adding more fixtures. The current measured
loss is `callin_unimplemented` (Lua 1438 ns, Core 1454 ns); the other seven
rows are Core-faster, subject to the same microsecond-scale noise.

### 1c. Benchmark callouts by transport class and by argument weight

The expanded callout run has the seven cross-backend rows plus eight Core-only
transport-ceiling rows. It still cannot characterise 1354 callouts across 10
transport classes, so the class inventory and unpaired measurements are
reported explicitly:

| class | count |
| --- | ---: |
| `fixed` | 806 |
| `variable-input-borrowed` | 191 |
| `variable-output-caller-owned` | 90 |
| `dynamic-output-caller-owned` | 89 |
| `handwritten-reviewed` | 71 |
| `variable-io-borrowed-input-caller-owned-output` | 52 |
| `variable-input-nested-adapted` | 22 |
| `variable-input-borrowed-mixed-fixed` | 16 |
| `fixed-option` | 13 |
| `variable-input-adapted` | 4 |

`dynamic-output-caller-owned` (89) and `variable-input-nested-adapted` (22)
have never been measured and are the most expensive-looking shapes.

**Priority: wide-argument and large-struct callouts.** This is where Core's
hand-rolled wire protocol should hurt most against Lua's native tables, and it
is exactly what the current 8 rows avoid. Mean input count is 2.1; the tail is
what matters.

Widest inputs:

| callout | inputs |
| --- | ---: |
| `gfx::BlitFBO`, `gfx::UploadTexture` | 12 |
| `ground_decals::SetGroundDecalQuadPosAndHeight` | 10 |
| `gfx::CopyToTexture`, `gfx::DrawGroundQuad`, `icons::AddUnitIcon` | 9 |
| `gfx::CreateShader`, `gfx::TexRect`, `system_control::GarbageCollectCtrl`, `terrain_control::SetTerrainTypeData` | 8 |

Widest outputs:

| callout | outputs |
| --- | ---: |
| `gfx::GetFontInfo` | 10 |
| `unit_defs::GetUnitDefByID`, `terrain::GetGroundInfo` | 9 |
| `terrain::GetTerrainTypeData`, `profiling::GetLuaMemUsage` | 8 |

Largest record payloads, for struct-argument rows: `WaterParams` (34 fields),
`GameRulesInfo` (27), `TeamStatsHistoryPoint` (19), `TeamResources` (18),
`MoveTypeData` (18), `LightParams` (17).

The measured Core-only representatives now cover fixed struct, borrowed string
input, borrowed `f32` list input, reusable string/list/nested-list/spatial
outputs, and adapted `list<string>` output. The listed widest real callouts
remain an unmeasured backlog; no synthetic ceiling row is presented as a
production callout result.

### 1d. Deliverable

Update `core_benchmark_results.md` with:

- Lua vs Core for every measured callin and every measured transport-class
  representative, with the unmeasured inventory visible as such
- an explicit list of every row where Core loses to Lua
- the drawworld stage-by-stage attribution
- what is noise-limited and therefore not a usable signal

If Core loses somewhere it should not, that is the finding. Report it plainly;
a regression found is worth more than a green table.

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

## 4. Sync rung 3 — local run complete, hosted run pending artifact

The cross-platform workflow (headless Linux, arm64 Linux, Windows; same
fixture; canonical per-frame checksums) is wired. The local rung-3 headless
run against the checked-in fixture passed with matching guest/reference
streams for all five contexts and zero reported probe failures. The hosted
three-platform job still requires a successful engine-build artifact for the
tested pushed SHA; it has not been falsely marked green locally. A synthetic
checksum stream is not an acceptable substitute. Desync = failing test naming
the frame.

## 5. Unresolved from the concurrent-session incident

`damage_check.md` records edits made by a second session while an agent was
working. Three questions were never answered:

- `render_core_wasm_owned_guest.rs` still contains `decode_core_string` and
  the shard refactor after fmt; no damage was found.
- The nine reverted files were byte-identical to the baseline at the time of
  the audit, and no in-flight work from this branch was present in them; the
  clippy fixes were already committed in `1f96da955f`.
- `probe_generated.rs` is 5007 lines both at the baseline path and now; the
  reported -4657 was a stale path/baseline comparison. Regeneration is
  reproducible and verifier-clean.

These checks are answered above; delete `damage_check.md` after this edit is
committed with the audit text.

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

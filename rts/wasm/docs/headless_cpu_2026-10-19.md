# Headless client CPU preparation · 2026-10-19

Historical experiment record. The selected source changes are now recorded in local
commits; see [the session integration report](ship_game_session_2026-10-20.md) for
the final scope, exclusions and installation status. Statements below describe the
original experiment at capture time.

Codex-authored (GPT-6.1 Sol), unreviewed engineering experiment. All engine changes
remain uncommitted and unpushed in `codex/opt-headless-client`, rooted at
`/home/gajop/.codex/worktrees/opt-headless-client/spring-bar`. This is not an upstream
submission or an accepted clean-machine benchmark.

The candidate removes the `ModelUniformsStorage::GetObjOffset` leaf that accounts
for 15.2% of sampled baseline user CPU. In the matched sampled large battle,
the all-thread user-CPU estimate per sim frame falls 16.0%, while throughput is
2.0% lower and client Update cadence is 18.2% higher. Correctness, replay export,
local callbacks and interactive smoke checks pass. The result supports narrower
headless preparation work, with no end-to-end or interactive speedup claim.

## Source and runtime boundary

The worktree base is `6cedf0cde7f9e6f542bfb80ad7c6935fe5ff77b9`. Both baseline and
candidate include exactly two explicit prerequisites copied from the original
engine's uncommitted changes: the absent-Wasm-handler `CommandFallback` completion
fix in `rts/NativeInterface/NativeInterfaceEventClient.cpp`, and the non-Tracy
profile-window marker sink in `rts/System/Log/TracySink.cpp`. Their exact diff is
`build-headless-perf/evidence/prerequisite.patch`; the baseline source patch is
saved separately. No other original-checkout WIP was copied. The original engine,
main game checkout, and other agent worktrees are not edited by this lane.

The game payload comes from the frozen compiled runtime at
`/home/gajop/.local/share/ship-game-verify/2026-10-19-sim-profile/game.sdd`,
including its earlier read-only population observer and game INFO log filter.
It is not the clean game branch baseline used by the two pilot optimization
lanes. Both engine sides use the same game WASM/configuration and fixed
synced/unsynced RNG seeds of 7. Earlier diagnostic captures used an engine with
additional unrelated WIP and are not this experiment's baseline.

That runtime retained asset symlinks into a user asset generator's output. A
before/after metadata guard caught updates to 166 unrelated model files after
the first correctness/replay tests and before the first performance controls.
Those initial tests are retained under `evidence/*-assetset-a`; no performance
claim compares the two asset sets. Phalanx/Lancer/Hypaspist and missile models
were unchanged. Final validation and performance work use one materialized,
read-only private asset set B at
`build-headless-perf/frozen-game-assetset-b.sdd`: 4,535 files, 4,814,552,008 bytes.
Its full path/SHA256 map is `evidence/asset-hashes.json` (SHA256
`70aff9331c828599726bf6a3934391297a3d22b54c66f1a08129a92bdac55304`);
`evidence/assetset-b.json` identifies the snapshot boundary. All resolved assets
are regular files with mode 0444; private Lua/WASM/config overlays remain separate.
The two binaries are unchanged across this runtime repair.

Builds use the pinned repository Docker image, GCC 13, explicit
`-O3 -g -DNDEBUG`, Tracy disabled, and a private build directory. Vendored
submodules and the existing ccache are read-only mounts; no original engine
build/install directory is reused. Maximum compiler parallelism is two jobs.
After the initial authorized bootstrap, compiles hold a shared host profiling
lock, and native runs hold the exclusive lock through the game's
`tools/profile_lock.py`. Other user processes remain running.

## Contract audit and candidate

A blanket return from `CGame::Draw`, `UpdateUnsynced`, or `CWorldDrawer::Update`
would suppress necessary work. Headless already suppresses Core Wasm `Draw*`
callins, but Lua/native callbacks still exist. Unit/feature interpolated draw
positions, icon status, and full current/previous draw flags feed public Lua and
native visibility/read APIs. They are retained, as are camera updates, command
handling, RmlUI updates, local unsynced callbacks, profiling and recording.

The candidate limits work in two places:

- `rts/Rendering/Common/ModelDrawerData.h`: headless builds retain draw-flag
  updates but omit the per-object GPU transform/uniform upload arrays.
  `GetUnitTransformMatrix`, `GetFeatureTransformMatrix` and piece matrix reads
  obtain data from the objects directly; renderer storage allocations and public
  buffer size/offset metadata remain intact. The skipped transform uploader also
  forces lazy piece-matrix refresh and resets renderer `wasUpdated`/
  `noInterpolation` caches. Whole-`rts` caller searches find `GetWasUpdated` and
  `GetEffectivePrevModelSpaceTransform` consumed only by that uploader. Public
  piece reads refresh dirty matrices on demand; the script animation traversal
  refreshes dirty transforms before using its raw parent-transform accessor.
  Buffer-uniform setters only update `userDefined` storage, which remains allocated.
- `rts/Rendering/Env/Particles/ProjectileDrawer.cpp`: retain projectile draw
  positions and flag resets, but omit pass culling/sort preparation for headless
  projectiles without `luaDraw`. Opted-in model projectile `DrawProjectile`
  callbacks keep the original selection path. Public `GetVisibleProjectiles`
  uses synced position, camera and LOS directly, independently of those flags.

Interactive builds preprocess both guards away and keep the previous renderer
paths. No synced simulation or pilot code is changed. This deliberately retains
part of the roughly 29% draw-data leaf attribution in the earlier capture:
necessary read state and callbacks take priority over eliminating that number.

## Validation and measurements

The final asset-set-B 200-ship contract check passed on both binaries (frames 0–360, exit 0).
All 361 per-frame diagnostic records agree on unit/projectile/feature counts,
quantised stable-ID unit state digests, create/destroy/fire/damage events, damage
amounts and command counts, covering 800 commands. Both runs deliver Lua `Update`
and `DrawGenesis`, expose changing view positions and icon status, and support
visibility/transform reads without Lua errors. Their complete finalized 14,939,992-byte live world
recordings are byte-identical (SHA256
`76a8f5eb81f1177974f9bd4b45fbfa66c18dd8428c7afe8329800fdb8fe7792f`).

The earlier asset-set-A 2,000-ship gate matches all 361 records through frame 360, covering
8,000 commands, 17,727 weapon fires and 17,609 damage events. The smaller gate
covers 10,480 fires and 10,282 damage events. No units die within these prefixes;
these checks cover active battle openings, not full matches.

With the final asset set B, the candidate additionally replayed the baseline's 200-ship demo to frame 360
(exit 0, no desync): all 361 simulation/event/command trace rows and all 31,340
exported world sample/event rows agree with the live baseline. An initial asset-set-A export
controller attempt stalled on a recorded demo-server pause and is retained as an
invalid setup capture under `evidence/replay-export-incomplete-pause`. The private
controller was corrected to send idempotent autohost unpause commands. No engine
change was made for that setup failure.

The compiled interactive executable's `.text` section is byte-identical between
baseline and candidate (SHA256
`585478e396393ebb62aa7c3530dab9e00fa9d516ce6c20762cd8813abc7c46fc`).
Full binary hashes include debug/build metadata and differ.

The final asset-set-B positive-visibility/model-projectile pair also passed.
Both observe up to 191 visible units and 188 visible synced projectiles and opt
10,498 projectiles into Lua drawing. A private deterministic fixture spawns one
model missile at frame 150; baseline/candidate deliver 3,625/3,229
`DrawProjectile` callbacks respectively. Those counts depend on client cadence;
the requirement is an exercised callback, not equal draw counts. All 361
simulation/event/command rows still agree. No model fixture is used for timings.
Its first baseline attempt used the unavailable `WeaponDefNames` convenience
global in this minimal Lua handle and is retained under
`evidence/baseline/projectile-contract-invalid-no-weapondefnames`. The private
fixture now resolves IDs through `WeaponDefs`; this required no engine change.
A separate Shield scene was also rejected before candidate testing because the
frozen guest panics at frame 2; its invalid baseline data remains with asset set A.

Interactive smoke also passed on candidate `spring` through frame 360 (exit 0,
complete profiling window, no Lua/Wasm runtime errors). The frame-180 screenshot
was visually inspected: ship meshes/textures, exhaust, weapons, health bars,
minimap and RmlUI controls render normally. The retained screenshot is
`evidence/candidate/contract/fleet-combined-far-render-64x64-n200-r1/interactive-smoke.png`.
This is a Linux interactive smoke check, not a renderer performance benchmark.

| Binary | Baseline SHA256 | Candidate SHA256 |
| --- | --- | --- |
| `spring-headless` | `8ed336026321f57cc38d7ce1937f567c0423fb841b6c558abc8527bb28e5252e` | `57b9dd6ffefc8b293adb009f90eabfc7e8be0d7c5891eaa280eaed298c005f6f` |
| `spring` | `2d68dcbb2791abe6cbc924a495effb00bfb4904df5608c039a11c844372d7eef` | `85cf961610440dbf8a14a0d5c7eef10e9dd9225cc9de76a020359e650b03fe19` |

Correctness runs enable the existing `SPRING_PERF_LOG` diagnostic to collect state
and event/command counts. Its import/call timing and the recording observer add
CPU work, so those runs are excluded from performance controls. Matched timing
controls/samples disable that diagnostic on both sides, while retaining the same
existing game-stage/nav-lap clocks and profiler counters.

Native client cadence is an explicit confounder: the headless application still
runs `CGame::Draw`/unsynced updates between queued sim frames, and removing work
can change how often that loop runs. Every control reports the actual unsynced
`Update` call count and calls per sim frame, plus timer cost per update. The Core
Wasm `DrawGenesis` counter is zero in headless because those callins are already
suppressed; zero is not the C++ Draw invocation count. In these non-skipping live
runs, `CGame::Draw` calls `UpdateUnsynced` once and then enters the Draw timer,
so the native Draw call count is derived from unsynced Update callins. Timer and
call snapshots can differ by one boundary loop; no extra native counter was added.
The sampled target leaves
are also normalized by Update calls. Whole-window CPU/throughput is diagnostic
when cadence differs, not an isolated measure of candidate computation. World
preparation also mixes per-loop work with once-per-sim work (e.g. transform
refresh and sky/water updates), so per-update normalization changes how those
costs are amortized. It is a useful comparison, not a perfect fixed-work metric.
Existing `MinDrawFPS`, `MinSimDrawBalance` and `ServerSleepTime` affect sim bursts/server
pacing, not a fixed headless client-loop ceiling; no configuration was tuned for
this comparison.

The first 2,000-ship control pair completed the same 780-frame window
(120–900). All 31 read-only population records agree through frame 900, including
68 unit losses and final counts of 385 Hypaspists, 599 Lancers and 948 Phalanxes.
Neither run reports a guest panic, Wasm trap or desync. These are observed
inclusive wall timers under existing user load:

| Measurement | Baseline | Candidate | Change |
| --- | ---: | ---: | ---: |
| Unsynced Update / derived native Draw calls | 62,126 | 73,420 | +18.2% |
| Calls per sim frame | 79.65 | 94.13 | +18.2% |
| `Update::WorldDrawer`, ms/update | 0.14379 | 0.08853 | −38.4% |
| `CUnitDrawerBase::Update`, ms/update | 0.10666 | 0.05150 | −51.7% |
| Whole `Update`, ms/update | 0.27456 | 0.21987 | −19.9% |
| Native `Draw`, ms/update | 0.22195 | 0.22508 | +1.4% |
| `Update::WorldDrawer`, ms/sim | 11.453 | 8.333 | −27.2% |
| Native `Draw`, ms/sim | 17.678 | 21.186 | +19.8% |
| `Sim`, ms/sim | 72.698 | 80.238 | +10.4% |
| Simulation frames/s | 8.742 | 8.015 | −8.3% |

This supports a preliminary reduction in per-update preparation time, **not an
end-to-end speedup claim**. More client loops offset part of the saved work, and
synced simulation timing also moved despite unchanged simulation code. Four
additional wall-only large controls were omitted because they would
not resolve this cadence/load confounder.

The 200-ship control pair also passed with matching 31 population records and
170 survivors (40 Hypaspists, 40 Lancers, 90 Phalanxes). It used 806 versus 799
Update/native Draw loops over 780 sim frames (1.033 versus 1.024 per sim), with
38.12 versus 85.51 sim frames/s. However, the unchanged `Sim` scope fell from
24.89 to 11.26 ms/sim, so the large wall-throughput difference cannot be assigned
to these renderer guards. WorldDrawer was 0.45728 versus 0.04610 ms/update,
and UnitDrawerBase 0.03516 versus 0.01522 ms/update. These remain diagnostic
observations under load/order effects, not a measured production speedup.

## Native sampled comparison

The final 2,000-ship sampled pair also completed frames 120–900 with the same
31 population records and survivors as the controls. `perf cpu-clock:u` sampled
all engine threads at 199 Hz with frame-pointer unwinding. Recorded sample
periods were summed inside the explicit gameplay markers to estimate user CPU
work; this excludes kernel CPU, startup, warmup and shutdown. These are
sample-based estimates, not exact instruction counts or wall durations.

| Sampled measurement | Baseline | Candidate |
| --- | ---: | ---: |
| Samples / lost samples | 25,252 / 0 | 21,215 / 0 |
| Unknown leaves | 2.99% | 3.58% |
| Shallow stacks (quality diagnostic) | 11.31% | 16.81% |
| Unsynced Update / derived native Draw calls | 61,448 | 72,602 |
| Calls per sim frame | 78.78 | 93.08 |
| Estimated user CPU, ms/sim | 162.685 | 136.677 |
| Simulation frames/s | 9.478 | 9.288 |
| WorldDrawer wall timer, ms/update | 0.13913 | 0.07429 |
| UnitDrawerBase wall timer, ms/update | 0.10412 | 0.03885 |

The observed user-CPU estimate is 16.0% lower despite 18.2% more client loops.
The sampled wall-throughput result is 2.0% lower; together with the slower
unsampled large control, this rules out claiming an end-to-end speedup from
these runs. The reduction in the specific uniform-offset leaf is the strongest
direct attribution evidence:

| Disjoint native leaf group | Baseline samples (% CPU samples) | Candidate samples (% CPU samples) | Baseline CPU ms/update | Candidate CPU ms/update |
| --- | ---: | ---: | ---: | ---: |
| `ModelUniformsStorage::GetObjOffset` | 3,838 (15.20%) | 0 (0%) | 0.31387 | 0 |
| `CCamera::CalcViewPortCoordinates` | 1,168 (4.63%) | 1,543 (7.27%) | 0.09552 | 0.10680 |
| `CUnitDrawerData` task leaves | 423 (1.68%) | 553 (2.61%) | 0.03459 | 0.03828 |
| `CProjectileDrawer::UpdateDrawFlags` task leaves | 610 (2.42%) | 812 (3.83%) | 0.04988 | 0.05620 |
| `CFeatureDrawerData` task leaves | 6 (0.02%) | 7 (0.03%) | 0.00049 | 0.00048 |
| Named groups combined | 6,045 (23.94%) | 2,915 (13.74%) | 0.49435 | 0.20176 |

Combined named-target CPU cost is observed 59.2% lower per Update and 51.8%
lower per sim frame, dominated by uniform-offset removal. Normalization still
has the cadence/amortization limits above. Camera and other retained task leaves
do not show an independent per-update reduction; this experiment does **not**
establish a separate gain for the projectile guard. Keeping the necessary camera
and public visibility state is intentional. No further optimization trial was
run to chase the remaining draw-data CPU share.

Native inclusive ancestry is limited by optimized builds and JIT/host transitions;
these groups use leaf symbols rather than inferred call trees. Zero observed
uniform-offset samples does not prove the function cannot be reached by another
game/API path. `evidence/analysis.json` contains the exact derived comparisons,
all population checks, earlier large trace equality and replay result. Each
sampled case retains `perf-stacks.txt`, folded stacks, flame graph, clipped
sample-period list and `sample-period-summary.json`.

## Review and reproduction

Local artifacts and helper scripts live under the ignored private
`build-headless-perf/` directory. `evidence/binary-identities.json`,
`baseline/source.patch`, `candidate/source.patch`, `evidence/CMakeCache.txt`,
each group's `identity-*.json`, and each case's `command.json`, `start.txt`, `spring.conf`,
stdout and summary record the tested source/runtime/configuration boundary.
The candidate-only patch is `evidence/candidate.patch`; it is separate from the
common `evidence/prerequisite.patch`. All tracked engine source diffs still equal
the candidate build's saved full source patch.

For manual verification, inspect the private probes and run `measure.py` with a
new repeat ID rather than overwriting a recorded case, e.g.
`python3 build-headless-perf/measure.py baseline 200 2 --contract`, then the
same command with `candidate`. The script acquires the exclusive shared profiling
lock, fixes both RNG seeds, guards the frozen assets, and preserves its runtime.
`--projectile-contract` exercises the controlled model callback;
`--contract --render` runs the interactive smoke fixture. Ordinary controls omit
those flags;
`--perf` adds the clipped native sample capture. Captures always use the same
preserved binary and snapshot pair. New builds must hold the common shared lock
and use at most two compiler jobs.

This is Linux-only agent validation, awaiting human review/use. It does not
establish a clean-machine production speedup, exact cross-platform sync proof,
or coverage of every Lua/native extension, feature animation or save/load path.
The per-frame digest is quantised and is an equivalence diagnostic, complemented
by byte-identical world recordings and cross-engine replay. No new core unit test
was added for these renderer integration guards; the private runtime fixtures
exercise the actual engine paths.

`git diff --check` passes. Final asset metadata still matches snapshot B, all
final case summaries are valid, and tracked source still matches the candidate
build patch. No engine commits, pushes, merges or original-checkout edits were
performed. The uncommitted paths are the two candidate renderer files, the two
explicit prerequisite files, and this report.

Raw profiles are private and should not be published without review.

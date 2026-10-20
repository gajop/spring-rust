# Common unit-icon preparation, 2026-10-19

Historical experiment record. The selected source changes are now recorded in local
commits; see [the session integration report](ship_game_session_2026-10-20.md) for
the final scope, exclusions and installation status. Statements below describe the
original experiment at capture time.

Codex (GPT-6.1 Sol) authored this implementation, test, and report. This is local, uncommitted work for human review; it is unreviewed for upstream and has not been verified by a human player. Nothing was committed, pushed, merged, or submitted externally.

## Scope and result

The shared interactive/headless unit-icon path previously called `CCamera::CalcViewPortCoordinates` twice per unit, although it used only the absolute difference between the two projected x coordinates. The candidate snapshots the camera's x/w matrix rows, right vector, and viewport width once per unit-update batch, then calculates both x/w projections together in four SIMD lanes. It preserves the full path's multiply/add order, reciprocal followed by multiplication, and screen-coordinate translation/scale before subtraction. It does not approximate the radius or cancel algebraic terms that affect rounding. Picking radius and icon thresholds retain their previous expressions.

The candidate also evaluates camera-independent `noDraw`, icon, void, and LOS filters once before the draw-pass camera loop. It retains the original draw-bound check and flag reset/preservation, camera-dependent frustum tests, reflection/refraction rules, callbacks, and metadata.

This targets common CPU preparation used by the interactive engine. It does not introduce another headless-only skip. No end-to-end playing-speed improvement is claimed by this report; the coordinator owns the serial rendered 2,000-ship engine comparison. The earlier headless-only work and its cadence/load limitations remain separately documented in `headless_cpu_2026-10-19.md`.

## Source boundary

Worktree: `/home/gajop/.codex/worktrees/opt-headless-client/spring-bar`, branch `codex/opt-headless-client`, unchanged HEAD `6cedf0cde7f9e6f542bfb80ad7c6935fe5ff77b9`.

New common-path source/test diff:

- `rts/Rendering/Units/UnitIconProjection.h` (new): immutable projection helper.
- `rts/Rendering/Units/UnitDrawerData.cpp`: use helper in screen-icon preparation and hoist pass-independent flag filters.
- `rts/Rendering/Units/UnitDrawerData.h`: forward declaration and private method argument.
- `test/engine/Rendering/testUnitIconProjection.cpp` (new): reference and edge tests.
- `test/CMakeLists.txt`: register focused Catch target.

Existing uncommitted prerequisites and headless-only changes remain in `NativeInterfaceEventClient.cpp`, `TracySink.cpp`, `ModelDrawerData.h`, and `ProjectileDrawer.cpp`. Those earlier changes are identical between the preserved engine control and this candidate. The original engine checkout, shared game checkout, and other agent worktrees were not modified.

Private evidence root: `build-headless-perf/interactive-common`. `common-only.patch` isolates the five paths above; SHA256 `1a80a36e6592ba1abd4289cdd53b8b2bfca3f1efc38ba4f97d757861296ca9b6`. `source.patch` contains the full engine diff including prerequisites; SHA256 `194fc2a49d81d518e60d28b319d2de7c14b9e312c8a029c15708b1a323c404ee`. Build/source identities and the exact CMake cache are saved alongside it.

## Preserved controls and build

The earlier tested binary snapshots under `build-headless-perf/baseline` and `build-headless-perf/candidate` were preserved. Only the private scratch build was reused, with the coordinator's permission, for an incremental one-job compile holding the shared profiling lock.

| Image | Path under `build-headless-perf` | SHA256 |
|---|---|---|
| Interactive control | `candidate/spring` | `85cf961610440dbf8a14a0d5c7eef10e9dd9225cc9de76a020359e650b03fe19` |
| Interactive common candidate | `interactive-common/candidate/spring` | `d082bb2345686048bb9fe25cf0ea178145a6b736fbde7a406d5cefd9ecc93613` |
| Headless control | `candidate/spring-headless` | `57b9dd6ffefc8b293adb009f90eabfc7e8be0d7c5891eaa280eaed298c005f6f` |
| Headless common candidate | `interactive-common/candidate/spring-headless` | `976db20c79eae0b01bc2358663809ea7a8bbd300d4cbe53d8dc57c2ff6038678` |

Build image: `ghcr.io/beyond-all-reason/recoil-build-amd64-linux@sha256:31b3667ba0d37620b4ad9b45d1840c8d3e95317f2b233044d2e17697ce8fe61d`. The private build used the existing RelWithDebInfo/O3 configuration, with SSE math and FMA disabled. `interactive-common/build.log` records the successful 91-action incremental build of `test_UnitIconProjection`, `engine-legacy`, and `engine-headless`. Independently copied candidate binaries, patch, and `candidate-CMakeCache.txt` retain this result without depending on later scratch-build changes. Large original populated submodules were mounted read-only; the original engine build/install were untouched.

## Equivalence audit

The original path uses `unit->pos` and the camera right vector, rather than interpolated or LOS-error position. The helper uses those same values. It computes the old projected x for both the center and radius endpoint, including perspective division and viewport mapping, before taking their absolute difference. It does not reuse draw-icon screen positions because those have different position semantics.

Camera state is immutable during the existing joined parallel unit-update batch. The helper owns copies of its inputs, so no reference points at a mutable camera matrix or right vector after construction, and no state is cached across frames. Pan, zoom, rotation, FOV, projection, and viewport changes therefore feed a new snapshot on each update. The lambda's stack reference remains alive until the existing `for_mt_chunk` call joins. No unit data layout, serialized field, public read API, callin contract, or synced simulation/command path changed.

The draw-flag hoist only moves pure unit-state filters ahead of camera queries. A false predicate previously skipped every camera pass; it now returns after the same flag reset and world-bound check. Water reflection capability queries are pure constant returns in the available implementations, and camera lookup has no side effects. Per-camera visibility and water/shadow/reflection tests retain their original order for surviving units.

## Checks

- A brief standalone test built with GCC13/SSE and matching float operation constraints passed **100,252 exact-reference cases**, including IEEE edge cases. `quick-reference.cpp`, its binary, and output are retained in the private evidence root.
- The focused Catch target compares against the real `CMatrix44f::operator*(float4)` plus the original viewport formula. It exercises perspective, asymmetric and orthographic matrices; camera right vectors; zero/small/large viewport widths; map-scale/offscreen positions; zero/negative w; infinities, NaNs, subnormal values; input mutation after snapshot; and 4,096 arbitrary projective cases. Finite and infinite values compare by float bits; NaNs compare classification and their derived threshold behavior. Picking-radius expressions and strict threshold comparisons are checked too.
- Focused Catch execution passed: **46,702 assertions in 3 test cases**, exit 0. The exact command/output are retained in `focused-test.log`.
- `git diff --check` passed. Both interactive and headless engines linked successfully.
- The initial fast candidate fixture completed API checks and exited 0, but only its first screenshot was usable; three post-resize PNGs were black. Its identical old-control attempt completed the engine workload but only delivered 18 draw callbacks, so the original four-draw-per-phase observer saved no screenshots. The failed observer and its logs are retained; it was not treated as an engine failure or a visual pass.
- The corrected 1× control fixture waits for four draw callbacks before advancing and briefly pauses/resumes simulation to retain the observation window. It passed all four API phases, exited 0, and delivered 66 update/draw callbacks. It reproduced the same black PNGs after each resize; RGB nonzero bounds are null in phases 2–4, while phase 1 renders ships and UI. This is evidence that the common patch did not introduce the resize/capture behavior. The source of that behavior (capture state versus the rendered window) remains unresolved and outside this patch.
- The common candidate then passed the same four camera/FOV/icon phases with window size fixed at 1280×720: exit 0, 889 update and 889 draw callbacks, no Lua/Wasm errors, and pinned asset metadata stable. All four screenshots have nonzero image bounds and were inspected: near models, far screen icons, rotated wide-FOV models, and legacy icons with battle effects/UI. The legacy-icon screenshot contains black rectangular markers whose texture/atlas correctness was not assessed; this unchanged legacy icon path is outside the projection helper. This is a focused scene check, not a full visual regression or human playtest.

| Fixed-size candidate phase | Frame | Units | Icons | Models | Visible units |
|---|---:|---:|---:|---:|---:|
| Near, UI icons enabled | 30 | 200 | 0 | 200 | 23 |
| Far, UI icons enabled | 123 | 200 | 200 | 0 | 156 |
| Pan/rotation, FOV 70 | 210 | 200 | 0 | 200 | 14 |
| FOV 30, legacy icons | 280 | 200 | 48 | 152 | 48 |

Fixture game code is a private copy of the frozen compiled baseline, with asset links pinned to immutable assetset B and RNG seeds 7/7. Only presentation and pause/resume commands are added; entity state and gameplay logic are unchanged. The fixture reads unit draw flags, icon/model populations, unit view positions, visible units, and viewport geometry and observes ongoing local update/draw callins. It is not a timed comparison; sim/draw cadence and pause duration deliberately differ between runs. No new broad captures or performance trials were launched by this lane.

Evidence paths under `interactive-common`:

- `interactive-check/`: initial fast candidate, including black resize PNGs.
- `interactive-control/`: initial engine-success/observer-failure control; no screenshots.
- `interactive-control-paced/`: settled old control, all four resize PNGs and API observations.
- `interactive-noresize-paced/`: settled common candidate, four inspected fixed-size PNGs and API observations.
- Each successful case retains `summary.json`, `stdout.txt`, `command.json`, exact start script/config, fixture identity, and screenshots. `screenshot-stats.json` records pixel bounds for the settled control and fixed-size candidate. Private runtime paths remain in `command.json`.
- `interactive-probe.v1.lua` and `interactive-probe-noresize.v1.lua` preserve the original observer; current probe files retain the corrected settling/pausing implementation. No engine source changed during fixture repair.

## Limits

Only the accessible Linux x86-64 environment is built/tested here. No Windows/ARM, human playtest, or complete renderer regression suite is claimed. The tests establish arithmetic equivalence for the radius helper and the code audit covers flag-filter placement; a focused interactive fixture cannot cover every water, shadow, LOS, or game-specific rendering configuration. The coordinator's matched rendered comparisons are required to determine whether this common preparation change materially improves the 2,000-ship playing workload. Cadence, host load, and inclusive timer amortization remain relevant when interpreting per-frame/per-update results.

## Applying to the original checkout

Only read-only checks were performed against `/home/gajop/projects/spring-projects/spring-bar`. `git apply --check common-only.patch` succeeds. The full `source.patch` rejects only `NativeInterfaceEventClient.cpp` and `TracySink.cpp`, because their approved prerequisite changes already exist in the original checkout; `git apply --check --reverse evidence/prerequisite.patch` succeeds there and confirms that boundary. `integration-on-original.patch` excludes those duplicate prerequisites and includes the two earlier headless files plus the five new common source/test paths; its read-only apply check succeeds. SHA256 `c7d106fa1efe9b8bee3a6cb9089336000bf1c32f6331475370d7e495d0a85f4b`. `original-apply-checks.json` and `original-prerequisites-reverse-check.json` retain results. No patch was applied to the original checkout. The coordinator owns any later adoption after review.

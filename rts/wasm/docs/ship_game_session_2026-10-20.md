# Ship Game session handoff — 2026-10-20

Codex authored the work described here, with delegated GPT-6.1 Sol agents. This is a local engineering handoff, not an upstream PR or a claim of human, Windows or cross-platform acceptance. The user authorized engine commits on 2026-10-20 after previously asking that engine work remain uncommitted. Nothing was pushed. No engine binary was installed during this commit transaction.

## Delivered state

The selected engine work is committed on the primary `spring-bar` checkout's `rust-wip` branch. Eleven implementation commits run from `1b036dd893` through `3b0d656fd02f`, touching **60 distinct paths**. This report and its [complete commit/file inventory](data/ship_game_session_2026-10-20/changes.json) form one additional documentation commit. Per-commit file counts below overlap where a later fix touches the same file.

| Commit | Change group | Files |
|---|---|---:|
| `aa052ea6a954` | wasm-reload | 6 |
| `37267cf583f5` | lua-layout | 1 |
| `554ac8bb9541` | command-fallback | 1 |
| `8df3d71eb16b` | muzzle-bounds | 2 |
| `cb275dc375f5` | headless-preparation | 5 |
| `03816baff1a6` | icon-projection | 6 |
| `3579f904bfac` | model-transforms | 9 |
| `319718ab9500` | target-queries | 4 |
| `ecb5d1071d5d` | profiling-timing | 5 |
| `e793464cf4b5` | wasm-teardown | 3 |
| `3b0d656fd02f` | same-frame-jobs | 19 |

The game work is already committed on `ship-game.sdd/main`, through `0e79730a`. The inventory identifies 37 selected session integration commits and 373 historically touched paths, including generated ship assets, reference tests, reports, and a subsequently reverted experiment. That is a touched-path union, not 373 pending changes or an exclusive net diff across interleaved work from other sessions. The latest idle-only pass is precisely `5e4b567d..0e79730a`: 12 commits, 33 net changed files (12 runtime, 17 tests/benchmarks/reference implementations, four docs).

The Triarii authoring work is already committed in `ship-game-assets`: `4a2bc94` and `388cb6a`, 62 distinct paths. Its integration into the game is the Triarii branch through `8d099f00`, merged by `4c305921`. Other asset work and intervening SDK, lobby, multiplayer and frontier changes are outside this session's commit set.

## Triarii: references, model and gameplay

The user selected reference A after several text-to-image rounds: separate scutum-like side defenses, an exposed front/rear, and one compact plasma gun whose shape suggests short range. The editable Python/Blender construction is in the asset repository's `shipassets/triarii/`; the reference, matched views and quality records are under `references/triarii/` and `triarii/`. Named hull, shield, cannon and propulsion components preserve authoring editability. Multiple source/engine views informed refinement of the shoulders, curved shield faces, short open bore, recoil machinery, bridge and fitted engines/thrusters.

Installed content includes full and distance meshes, 4 K surface textures, collision/effects hulls, animated muzzle/recoil, wrecks, salvage fragments, tactical glyph and HUD/build portraits. Eight Blender checks and all eleven final GLB checks passed. The 6,000-triangle collision hull's two retained ray tests had no missed hits, with conservative phantom hits recorded. Singular-tangent repair changed two tangents; other geometry attributes stayed identical to the duel runtime, and two final smoke duels reproduced the recorded outcomes.

Final tuning is 600 value, 105,600 HP, 1,400 reach and 9,000 damage per 1.5 seconds. The controlled live probe verifies **1.500 x damage per value and 2 x HP per value versus Phalanx**, with unupgraded/experience-normalized weapons. Directional multipliers are front 1, side 0.25 and rear 1.3: no front armor, 75% side reduction, vulnerable rear. The shared heavy pilot closes to short standoff and points the hull to fire. No special reserve/cleanup doctrine was added.

Thirty equal-value final duels used seeds 7/19, both deployment sides and pincer/rear variants. Triarii lost 4/4 frontal Phalanx and 4/4 Peltast trials; won 4/4 each against Hypaspist, Hastatus, Lancer and Shield; lost the Phalanx rear/pincer variants and won both Lancer pincer variants. Every trial ended before the 180-game-second cap. These results establish functioning combat and counters, not completed human mixed-fleet balance acceptance.

Game review: `docs/design/triarii.md` and `docs/reports/duels/triarii-2026-10-19-polish/report.md`. Asset review: `triarii/README.md`, selected reference and matched-view gallery. Game commits: `93f93b52`, `80f8705e`, `8d099f00`; asset commits: `4a2bc94`, `388cb6a`.

## Escape playtest corrections

Game commit `080a2f2b` and its documentation follow-up `da4c41a9` cover:

- Triarii's missing Roman production entry: extend the RmlUi catalogue and affordability/queue bindings, including cancellation, repeat and queue ordering.
- Rectangle selection recovery: clear stale right/middle gestures on fresh left press and reconcile movement with physical buttons. The original intermittent trigger was not reproduced; the tested recovery cases no longer require Esc.
- Depot flashing/abandoned allies: declare engine Strategos controllers for scripted friends/rivals/wardens in both launchers while retaining their run-owned behavior.
- Resource-collector lines across physical gaps: end the visible route at the next local wormhole mouth, label a known destination, and suppress remote direct lines without a local route.
- Missile trails and snare mesh: correct GPU capacity/list-length/instance-count ordering with named draw counts; retry missing trail allocations and expire routes after positions disappear. The snare route check validated 11,186 points with maximum error below 0.0001 elmos. Cross-system missile trails were exercised in system 1.
- Crossfire: position the encounter beside onward mouths and send fleets/reinforcements into the opposing station fight. Initial rival/warden damage occurred at frames 69/95 without player commands.
- Neutral ship visibility: distinguish ship palettes and mint neutral strategic icons from amber resource asteroids.
- Wormhole window/rim alignment: make ring/streamer geometry face the camera with the destination window; inspect low/steep camera angles and zoom/yaw changes.

Retained checks include actual HUD/queue interactions, three rectangle-selection cases, snare/trail captures, Crossfire damage, collector routing and portal views. The phase's engine-free suite recorded 941 Rust checks (11 ignored) and 10 launcher checks. These were functional checks against a shared dirty checkout, not a clean performance baseline. Source identities, images and compressed logs live in the game's `docs/reports/escape-fixes-2026-10-19/`; the parent report explicitly states each limitation.

## Engine changes: purpose and boundaries

1. **Local WASM reload.** `/reloadwasmmodules [ui|rules-unsynced|gaia-unsynced|all]` queues replacement until callbacks return, rereads original archive bytes, validates replacements before teardown and attempts last-good restoration on initialization failure. It resets local guest state; it does not migrate state or reload synced guests, unit definitions, manifests, menu or intro. Retained native checks cover invalid bytes preserving a usable HUD, valid changed bytes, three repeated UI reloads and rejecting `rules-synced`. See [hot reload](hot_reload.md).
2. **Nested LuaUI layout.** Read returned layout values relative to the saved Lua stack top, preserving arguments of a nested `LoadCtrlPanelConfig` caller. Manual regression: request that reload inside LuaUI with existing arguments and check command layout and warnings. Earlier HUD checks passed; a new isolated nested-Lua native case was not run during committing.
3. **CommandFallback identity.** An absent Native/WASM handler returns the neutral true value for the engine's AND-combined event clients. Explicit handler results still apply. Manual regression: a Lua-handled custom command must complete without a Native/WASM subscriber; explicit false still holds it. This was a prerequisite of earlier command/event fixtures.
4. **Muzzle-flame culling bounds.** Compute lifetime-dependent quad bounds before culling, including absolute dimensions, diagonals and front offset. Rendering vertex formulas and projectile simulation remain unchanged. Manual regression: pan/zoom across long, growing, negative-dimension and offset effects at view/system boundaries. No fresh exhaustive visual suite was run during committing.
5. **Headless preparation.** Skip GPU-only model upload arrays, non-luaDraw projectile visual sorting, and screen/minimap icon render-buffer construction. Retain public draw/read state, visibility/picking inputs and opted-in projectile callbacks. Earlier 200-ship contract checks match 361 gameplay/event/command records and a byte-identical 14,939,992-byte world recording. Replay and positive projectile-callback checks passed; interactive executable-section comparisons verify the HEADLESS guards' limited scope. See the dated [headless preparation](headless_cpu_2026-10-19.md) and [icon buffers](headless_icon_draw_2026-10-19.md) reports. No interactive speedup is inferred from headless-only skips.
6. **Common icon preparation.** Snapshot camera projection inputs once per joined batch, project center/radius endpoints with the original arithmetic order, and move camera-independent filters before the pass loop. The retained suite passes 46,702 assertions in three cases, with camera/picking/render smoke evidence. This is unsynced presentation SIMD, not enabling SIMD in synced WASM. The persistent projection-cache prototype is excluded. See [common icon preparation](common_icon_cpu_2026-10-19.md).
7. **Local-model transforms.** Separate local/world dirtiness, avoid copying settled previous transforms, bound dirty intervals and invalidate script-animation traversal correctly. Ships retain normal movement, bobbing and animation. Fresh original-primary comparison passes 4,394 snapshots and 5,385 semantic trace lines, original layouts and the animation kernel. The portable committed runner preserves both raw traces;1,541 optional candidate copy-diagnostic lines are validated separately, never used to hide semantic differences. Tests live in `test/validation/local_model_transforms/`.
8. **Empty-hostile-region queries.** Maintain counts/conservative bounds of actual quad-team entries. Reject only certified-empty eligible populations/regions; unknown/nonempty cases execute the original query, filters and ordering. Fresh checks pass 73,728 literal query comparisons,20,000 rectangle cases,20,000 mutation/query steps and four Catch cases/70 assertions. Bounds may overgrow conservatively. Full CREG save/load and live Lua lifecycle integration were not rerun; undefined original huge-radius conversion has no exceptional-domain equivalence claim.
9. **Profiling clocks.** Place task timers after successful dequeues and preserve scheduling. The timing oracle passes 32 cases with identical task order/statistics; empty profiler clocks fall 48 to 0 while actual task scopes remain. Opt-in monotonic window markers also work in ordinary engines. Thread-pool inclusive timers still overlap and cannot be summed as exclusive CPU cost. Runner: `test/validation/threadpool_task_timing/`.
10. **WASM teardown/Tracy.** Detach CUS before SDK shutdown. Disable Tracy's non-chaining crash handler in the dedicated variant so Wasmtime keeps fault-signal trap handling. Earlier captures completed gameplay then failed teardown; they remain failed records. Corrected ordinary/Tracy checks complete 180 steps, exit 0 and match 4,003 sampled state rows.
11. **Same-tick jobs.** Add joined ordered worker batches beside next-tick jobs, with complete pre-admission validation, shared quotas, owned ordered output and worker isolation. Public Rust names are `run_this_tick` and `run_next_tick`; `run_batch`/`submit` remain deprecated aliases. The game enables no parallel navigation backend. Fresh isolated SDK checks pass 59 public+28 internal cases, nine doctests, addon export/context checks and strict wasm Clippy/export checks. Retained C++ feature tests pass 15 cases/6,212 assertions; that C++ binary predates the primary's newer FP/hidden-state tests. Integration preserves those newer source guards/tests by three-way merge, but the newly combined C++ tree was not rebuilt in this commit transaction. See [jobs contract/history](same_frame_jobs_2026-10-19.md).

No global synced FP, NaN-canonicalization, relaxed-SIMD or WASM SIMD setting was relaxed. Windows was unavailable; Linux sampled equality, reference oracles and earlier replay checks are not full cross-platform/history desync proof.

## Game optimization work

The session first profiled combined fleets, reconciled existing plans, and delegated Heavy/Lancer/engine preparation. Equivalent shared observations, lazy contact geometry, ordered spatial queries and deterministic planning budgets reduce repeated work. Expensive route/forecast requests are budgeted and aged; pilots keep updating motion while waiting. A prepared contact/index/SAT batch was rejected after worse aggregate results, rather than retained on instruction-count claims. Lancer launch-path/context reuse and the scalar weapon-range accessor were later integrated after targeted correctness/combat checks.

After the user changed the priority to ordinary idle performance, the work retained all drift/bob, animation, health, reactions, collisions and default stances. Integrated changes avoid repeated Mantis pose/closed-gate maintenance, reuse pure steering only for identical observations, avoid maintenance of changing caches, finalize the snapshot grid once, use bounded prefix cell spans, borrow ordered contact rows, avoid unused support/spacing work, and defer occupancy query indices until an actual consumer needs them. Occupancy body/field/history/budget preparation remains eager where behavior depends on it.

The latest game-side suite passed 739 core cases (five ignored),13 actual-source rules harness cases, strict Clippy/build and import checks. All sampled unit/piece states matched the previous module on the installed engine. The 200-Mantis combat state gate ends at frame 900 with 63 survivors and 38,791 shots on both sides. The ordinary-health 2,000-Phalanx holding/shooting opening reaches frame 285 with all 2,000 alive and 4,268 shots on both sides. Five existing native execution gates pass: line contact, screen contact, cavalry flank, resource delivery and blocked Lancer. These are bounded correctness gates, not capacity claims.

The relevant game reports are `2026-10-19-simulation-profile`, `2026-10-19-optimization-review`, `2026-10-19-crowd-scaling`, `2026-10-20-navigation-preparation`, `2026-10-20-idle-simulation` and `2026-10-20-idle-attribution`, all under `ship-game.sdd/docs/reports/`. The implementation/plan map is `docs/ai/guides/source-map.md` and `docs/ai/proposals/simulation-performance.md`. User-authored `docs/ai/design/principles.md` was preserved.

## Measurements and what they establish

Fixed-frame elapsed tests time completed gameplay steps, excluding loading/warmup, with the same runtime, seed and workload for a pair. Most large windows are frames 120..900 (780 steps). Native runs were serialized under the shared profiling lock; bounded compiler concurrency does not mean exclusive CPU ownership. Background jobs, global GPU activity, swap I/O and client update cadence varied. ASAN, Tracy and sampled CPU captures are identified separately; full timers include their own instrumentation and parallel waits.

| Workload/change | Retained elapsed result | Interpretation |
|---|---|---|
|2,000 mixed ships, bounded planning |75.036->67.325 s;75.718->68.842 s;74.231->69.253 s | Median paired reduction 9.08%; below normal simulation rate |
|2,000 mixed ships, shared reads |75.898->63.576 s | One pair,12.27 steps/s; no clean repeated capacity acceptance |
|2,000 mixed ships, later Lancer reuse |65.365->62.485 s;67.834->62.713 s | Observed median reduction 5.98%; host differences retained |
|2,000 ordinary idle Phalanx, latest short rules pass | controls 2.150/2.123 s; combined 1.971 s |180 steps; approximately 7.8% below bracketing mean |
|2,000 ordinary idle Mantis, same batch | controls 1.126/1.054 s; combined 1.015 s |180 steps; approximately 6.9% below bracketing mean |
|2,000 asteroids, same short controls |0.079..0.082 s | Idle candidate remains about 25 x asteroid elapsed for Phalanx and 13 x for Mantis |

Do not compound those percentages: the comparisons have different baselines, engines, rules versions and host conditions. Earlier Hold Fire/Hold Position and motionless fixtures isolate cost but are not acceptance results for ordinary idle game behavior.

Current installed headless engine plus rebuilt game main,2,000 ordinary idle ships, frames 120..300, full engine timers/game clocks, no CPU sampler or Tracy:

| Nonoverlapping phase / total Sim elapsed | Phalanx | Mantis |
|---|---:|---:|
|Game rules/controllers including engine calls|59.7%|51.0%|
|Engine slow unit maintenance/target checks|29.5%|21.0%|
|Engine weapon vectors/updates|1.9%|14.2%|
|Script tick/animation phase|2.1%|0.9%|
|Engine movement|1.9%|2.2%|
|Remaining Sim|5.0%|10.6%|

Mean Sim is 16.508 ms/step for Phalanx and 11.951 ms for Mantis in those diagnostic runs. Controllers account for 39.25%/42.53% of total Sim, observation/Traffic 6.92%/5.06%, and Phalanx game-side weapon handling another 9.40%. Strategist is disabled in this fixture. Occupancy route-search counters were zero in the separate navigation-clock capture; controller cost still includes local contacts/steering and motion writes. This does not assign all controller time to pathfinding or all slow-maintenance time to targeting.

Mantis has four guns:2,000 ships cause 8,000 native gun-vector refreshes per sim step even without targets. Their phase is 1.442 ms/step (12.07% total Sim, including worker synchronization), with remaining native weapon updates 0.255 ms (2.14%). Geometry is refreshed before target-gated aiming/firing. Phalanx also performs game-owned target/readiness work; its entire game weapon phase is 1.552 ms, without a measured internal split. `SetUnitUseWeapons` and `AllowWeaponTargetCheck` provide game controls, but suspending native weapons also suspends state/targeting and requires correct waking. This remains an optimization opportunity, not a completed sleeping system.

Dedicated Tracy builds and ordinary-engine CPU stacks were both used. Initial wrong-binary handshake, older pre-optimization captures, post-window teardown failures, invalid fixture assumptions and contended runs remain explicitly failed/diagnostic evidence. CPU samples, overlapping inclusive zones and elapsed phase shares are different quantities. No instruction-count reduction substitutes for completed-step timings.

## Human playtest and installed binaries

The user tried the actual `perf-combined` window:2,000 total ships,1,000 per side, each 500 Phalanxes+300 Lancers+200 Hypaspists,96 x 96 map, requested speed 1. The engine confirmed 2,000 spawned. **The user reported it completely unplayable.** A 200-total version was then launched,100 per side, with spawn confirmation; no subsequent performance verdict was supplied. Neither launch used the private performance engine candidate.

Reproduce from `ship-game.sdd`:

```sh
just play fleet-combined --map 96 --speed 1 --setting ShowFPS=1 --setting ShowSpeed=1
just play perf-combined --map 96 --speed 1 --setting ShowFPS=1 --setting ShowSpeed=1
```

`justplay` rebuilds current game sources before launch. The direct launcher used for the handed-over matches reused the already-built game. The installed graphical engine SHA is `80a48871a491c5a075ce9c550c3753a1f286fdb4c3f7f34a5a85a538a2cd31a4`; installed headless is `7c29948e4d11168106096736a21b39738633de87b3fd03b039118834ca3951ae`. Earlier local reload/render fixes were built into the graphical install. The newly committed performance/jobs/teardown source is **not installed** by this transaction. The latest rebuilt-main rules artifact recorded in the idle report is `6a4ea149bba5d3f9e55c70f62475498cc98465b41e28019f7e1bdf0ba5e82ec0`.

The ordinary-idle asteroid target, smooth 2,000-ship battles, full game balance and cross-platform/history sync acceptance all remain open. New source commits are not evidence that those outcomes were achieved.

## Excluded experiments and preserved work

- Optional bulk live unit observations: engine/SDK and four game paths remain experimental. Fresh wire/SDK/differential checks pass, but the long cfgON times sit inside disabled-control variation; no useful gain was established. It is excluded from the jobs commit and main game.
- Persistent icon-projection cache: excluded; the committed helper only shares immutable inputs within a batch.
- Shared weapon aim-origin reuse: rejected after the uninstrumented oracle changed NaN payload/sign bits in 6 of 2,117 snapshots. No finite mismatch occurred, but the change was rejected. Its prototype-specific validation fixture requires the removed prototype and is retained outside the committed suite.
- Game contact-membership history cache: reverted in `734617a5`; this differs from conservative live engine quad metadata. Prepared-contact/index/SAT and optional parallel-navigation candidates remain nonselected experiments.
- Motion import combination was audited only; it was not implemented. No general sleeping/waking scheme or new behavior-changing idle benchmark was adopted.

Engine integration preserved exactly seven primary-checkout dirty paths byte-for-byte: the SMF shader/render-state pair, `Rendering/CommandDrawer.cpp`, and four older SDK/architecture/request documentation files. They are outside this commit set. Game and asset checkouts retain other sessions' dirty/untracked work. No blanket stash/reset, unrelated cleanup or running-game termination was used.

Merged session game worktrees were removed earlier. The temporary clean engine integration worktree and its branch were removed after verifying its commits are present on primary, its status is clean, and no process uses it. Dirty engine experiment and optional game-prototype worktrees remain recoverable; they were not force-deleted or presented as clean. Runtime/write/cache copies from completed verification runs were cleaned while retaining receipts/logs/selected images; the shared frozen fixture remains while the unfinished investigation needs it.

## Review evidence and remaining validation

The committed runners and reference tests accompany the selected source. Detailed per-commit messages explain concrete triggers, boundaries, prior checks and manual reproduction. Full file lists and repository paths are in the adjacent JSON. Local audit/provenance is under `~/.local/share/ship-game-verify/2026-10-20-engine-commit/`; the main historical runtime evidence is `2026-10-20-idle-parity/`, `2026-10-20-hostile-membership/` and the dated game reports.

Fresh commit-transaction checks were source/apply preservation, whitespace, model/timer/query differential checks, isolated merged SDK tests and rerunning the retained C++ job binary. A full rebuild and native smoke/replay of the final combined C++ commit tree were **not** performed during this commit transaction; older component/combined candidate results retain their original source/binary scope. The newer FP/hidden-state checks were preserved, not freshly executed as part of the old C++ job binary. Windows was not available. This report deliberately does not turn those gaps into passed checks.

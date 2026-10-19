# Recoil Wasm vs Lua Performance Study Plan

Status: in progress (2026-10-15). Shared frame instrumentation, engine input/stop controls and a paired-run harness are implemented. Seven game pairs were run diagnostically; accepted performance comparisons remain incomplete.

The 40-minute intensive-work window ended at approximately 12:00 JST on 2026-10-15. Intensive jobs have stopped. The implementation is committed; accepted performance comparisons remain pending.

## Current implementation and evidence

`GamePerformanceLog.h` provides opt-in CSV timing and frame-thread counters. Lua and Wasm callins share the recorder, generated import guards are counted, and Wasm calls have context profiler timers and ordinary Tracy zones with module/callin names. Engine input files and a frame limit avoid per-language quit/driver gadgets. Normal games do not enable the recorder.

`springcabal-games/verification/perf/run.py` stages private content, normalizes seeds/map/settings, records hashes and host state, runs alternating pairs, and writes distributions, state divergence, alive-count comparison and diagnostic ratios. It supports headless and realtime autonomous/scripted runs with bounded process/campaign timeouts. See its README for scope and limits.

Evidence is preserved in `springcabal-games/perf-runs/2026-10-15-autonomous-diagnostic`: five MyCube pairs and a pilot covering all seven game pairs. No performance claim is accepted. Several Lua games error in headless UI/render paths; Gravitas Lua did not reach frame 0; Area 17's shared LuaGaia code errored; state/count comparisons also diverge. The raw result files identify each failing side and keep the logs. These are prerequisites to fix or isolate before a language-speed comparison, not reasons to discard inconvenient runs.

Still needed: per-definition and unit-script work equivalence, per-gadget/per-import detail, load/memory instrumentation, lockstep and attribution modes, controlled scale/scenario patches, instrumentation-overhead measurements, settled repeated runs and accepted comparisons. Existing counters are frame-thread observations and do not claim coverage of private worker jobs. This plan has not met its final acceptance conditions.


Goal: establish whether the SpringCabal Wasm ports are faster than their original Lua versions in real games on this engine, by how much of a whole frame, where the difference comes from, and what engine or Wasm runtime problems show up on the way. The microbenchmarks ([benchmarking.md](benchmarking.md), [generated results](generated/benchmarking_results.md), [interpretation](impl/core_benchmark_results.md)) measure single calls and synthetic workloads. They do not show what share of a real game's frame is script time, or whether that share changes.

## Questions

| Question | Answered by |
|---|---|
| Does a Wasm port spend less time in script code than the Lua original doing the same work? | Script time per sim and draw frame, with equal work established first |
| Does that change whole sim and draw frame times, including tails? | Frame-time distributions per mode, compared with script share |
| Where does Wasm time go: dispatch, guest compute, host-call transport, engine API work? | Attribution runs |
| Where are the losses? The microbenchmark loss list says `callin_drawworld`, `callin_unimplemented` and large payloads; do real games hit them? | Attribution plus host-call counts against the generated loss list |
| Do gains hold or grow beyond jam-game scale? | Scaling and synthetic stress variants |
| What does Wasm cost at load time and in memory? | Load and memory runs |
| Is a port faster because of Wasm or because it was rewritten differently? | Hot-pair code review and the language-only microbenchmark `wl_compute` |

## What "gain" means

Every metric is reported as a distribution: p50, p95, p99, p99.9, max and frames over budget. Means appear only next to these. A comparison is a ratio of Wasm to Lua with a confidence interval over runs.

| Metric | Definition | Source |
|---|---|---|
| Sim frame time | Wall time of `Sim::GameFrame` per sim frame; frames over 33.3 ms | `rts/Game/Game.cpp` timer; frame timing log (engine work 2) |
| Sim throughput | Wall seconds for a fixed frame window at unlimited speed, as the highest sustainable speed factor | Headless throughput mode |
| Sim script time | Synced and unsynced script callin time per sim frame: `Lua::Callins::*` (`rts/Lua/LuaCallInCheck.h`) vs Wasm callins (engine work 1) | Frame timing log |
| Script share | Script time / frame time; the whole-frame gain bound (Amdahl) | Derived |
| Draw frame time | Wall time per draw frame, with the `Update`, `Draw::World`, `Draw::Screen` split; frames over 16.7 ms | Rendered modes |
| UI script time | `DrawWorld*`, `DrawScreen*`, `Update` callin time per draw frame | Frame timing log |
| GC and allocator pauses | Lua: `Lua::CollectGarbage::*` timers (`rts/Lua/LuaHandle.cpp`) plus GC steps inside callins (sampled `luaC_*`). Wasm: `memory.grow` events, guest allocator time (sampled) | Timing log, perf |
| Hitches | Frames over twice the rolling median, per minute, with the subsystem that owns the excess | Timing log, Tracy |
| Memory | Lua heap per handle (`GetLuaMemUsage`); Wasm linear memory pages per instance (peak, growth count); compiled code size; process RSS/PSS (`/proc/<pid>/smaps_rollup`), peak and steady state; slope over a soak | Timing log, sampler |
| Load time | Launch to first sim frame, split into archive scan, Lua parse and `Initialize`, Wasm compile, instantiate and `Initialize` per module | Load zones (engine work 6) |

A gain is accepted when the ratio's 95% bootstrap interval (runs paired by repeat index) excludes 1 and the effect exceeds both 3% and twice the run-to-run spread of the slower backend. Anything smaller is reported as "no measurable difference".

## Equal work

A faster port that does less is not a gain. Each comparison runs both versions with the same inputs, then checks that they did the same work before any timing is read.

### Same inputs

| Item | Rule |
|---|---|
| Engine | One binary for both backends per comparison; commit, build directory and flags recorded |
| Game content | `spring-linux/games/<Name>.sdd` and `<Name>-wasm.sdd` at recorded commits; shipped release modules, never `headless-integration` or `e2e` builds (`just verify installed` must pass) |
| Start script | `launch/games/<game>/lua.txt` and `wasm.txt`, normalised to the same teams, AIs and modoptions; `GAME\FixedRNGSeed` set through `vf_start_set` (`verification/lib/common.sh`). Script hashes recorded |
| Settings | `launch/springsettings.cfg` plus `launch/games/<game>/graphics.cfg`, as verification uses; window size, vsync off, `MaxFPS` uncapped for rendered timing runs |
| Synced RNG | Ports draw from the engine synced RNG in Lua's order (for example `ToTheLastDrop-wasm.sdd/wasm-src/rules-synced/src/random.rs`), so seeded runs can stay identical for a stretch |
| Player input | Autonomous scenarios by default (below). Scripted input only through the engine input script (engine work 5), never through a per-language driver in timing runs |
| End | Engine-side end frame (engine work 5) instead of the Lua quit gadget (`verification/fixtures/quit.lua`), which would add Lua callins to the Wasm runs |

### Known non-identical work

Reported for every game, measured separately where possible, never silently included.

| Difference | Where | Handling |
|---|---|---|
| Residual Lua in ports | `Flove-wasm.sdd` still runs Lua `api_custom_unit_shaders`, `unit_centeroffset`, `wisp_animation`; ports load the base gadget handler (`LuaRules/main.lua`) | Per-gadget zones; reported as "Lua in Wasm port" |
| UI framework | Lua UIs: Chili and LUPS. Ports: RmlUi and Rust Gfx | Draw and UI results are "port vs original", not a language comparison; engine-side RmlUi time reported separately from guest time |
| Unit scripts | Lua LUS vs Rust scripts (all ports "Gold+" in `springcabal-games/TODO.md`) | Pose parity (`ttld pose-parity`, `flove animation-trace`, `unit-scripts`) proves equal animation work |
| Module count | TTLD and Flove have three Wasm modules (rules, rules-unsynced, UI); others two | Per-module zones; fan-out cost compared with `callin_4modules` |
| Algorithmic rewrites | A port may use different data structures than the jam Lua | Hot-pair review (Attribution, step 5) before a large ratio is credited to Wasm |
| Headless draw callins | Headless skips Wasm `Draw*` callins (`rts/NativeInterface/NativeInterfaceEventClient.cpp`) | Check that Lua UI draw callins are skipped equally in headless; otherwise exclude them from headless script time |
| Unsynced RNG | Effects and UI randomness differ | Draw work compared by counters (particles, units drawn), not by pixels |

### Equivalence checks

| Level | Check | Pass rule |
|---|---|---|
| E0 config | Hashes of engine, start script, settings, map, game commits, module files | Identical except the game archive |
| E1 work counters | Per 30-frame interval, backend-neutral and logged by the engine: units alive per def, created, destroyed, weapon fires, projectiles alive, damage events and total damage, commands issued, unit script calls, rules-param writes, synced messages, CEG spawns, particles alive, units drawn | Totals within 1%, interval series within 5%, outside the divergence tail |
| E2 state digest | Per frame: quantised hash of (unitID, def, team, position, health) sorted by unitID, plus the engine sync checksum when the sync log of [Sync Verification](recoil_wasm_sync_verification.md) exists | First divergence frame recorded and explained |
| E3 callin traffic | Callin counts per type delivered to each backend | Reported, not required equal; explains dispatch-cost differences |

Timing windows lie before the E2 divergence frame where possible. After it, results are work-normalised (ms per unit-frame, per damage event, per drawn unit) and flagged. A cell failing E1 is not reported as a gain or a loss.

## Games and scenarios

Every Lua and Wasm version listed in `content.tsv` exists locally under `springcabal-games/spring-linux/games/`.

| Game | Hot script paths (Lua gadgets) | Scenario | Scale knob | Existing parity infrastructure | Role |
|---|---|---|---|---|---|
| To The Last Drop | `unit_ork_ai`, `unit_town_handler`, `unit_status_effects`, `game_storyteller`, unit scripts | `ttld-siege`: storyteller ork waves against militia, player invulnerable and idle | Ork batch multiplier; `/give` orks | `lua-visual`, `pose-parity` | Primary sim |
| Hunted | `rabbit_ai`, `wave_spawner`, `field_spawner`, `rabbit_carrying`, `unit_mine` (Rust `hunted-ai` crate) | `hunted-waves`: waves against fixed mines, no shooting | Rabbit count per wave | `hunted-ai` host tests | Primary sim (AI) |
| Gravitas | `unit_fire`, `unit_jumpjets`, `weapon_impulse`, `game_fall_damage`, `unit_robots`, LUPS fire | `gravitas-beam`: scripted beam on a fixed object set via the input script | Robots and barrels in the field | `gravitas e2e`, `gravitas-fire-compare` probe | Primary sim and effects; largest port |
| Area 17 | `unit_civilian_spawner`, `unit_ufo_abduction`, `unit_drones`, `weapon_area_damage`, LUPS | `area17-harvest`: civilians spawning, UFO abducting on a scripted path | Civilian spawn rate | `area17 headless` (Rust and Lua assertions) | Primary draw and UI |
| My Cube | `unit_eskimos`, `game_storyteller`, `game_resources`, `unit_drill`, `unit_drown_eskimos` | `mycube-survive`: storyteller run, no input | Eskimo count | `mycube lua-headless` | Pipeline bring-up |
| Flove | `grove_spawner`, `mushroom_spawner`, `unit_tree`, `unit_send_love`, `unit_poisoner` | `flove-grove`: mushroom waves against the grove | Spawner rate | `flove animation-trace` | Mixed port case |
| Parts Needed | `unit_wisp_controller`, `unit_collisions`, `unit_powerup_controller` | `pn-idle`: start area, no input | none | `unit-scripts partsneeded` | Fixed-cost floor |

Ship Game has no Lua counterpart and is out of the comparison. Its profiling tooling (`ship-game.sdd/docs/guides/profiling.md`, `tools/profile_*.py`) is reused for method and host-state capture.

### Synthetic stress variants

Applied to both versions identically. Engine-side spawning (`/give` through the input script) is preferred because it needs no game change. Game-side multipliers are patches applied to staged copies only (`verification/fixtures/perf/<game>/`), never to checkouts.

| ID | Load | Games | Equal-work proof |
|---|---|---|---|
| S1 unit scripts | N animated units of one type (walk, idle, attack) | TTLD, Flove, Parts Needed | Pose parity on sampled units |
| S2 spawner scaling | Game's own waves multiplied | TTLD, Hunted, My Cube, Area 17 | E1 counters |
| S3 event storm | Area damage on clustered units: `UnitPreDamaged`, `UnitDamaged`, `UnitDestroyed` rates | Area 17, TTLD | E1 damage events |
| S4 UI load | N units on screen with health bars and labels | TTLD (`ui_health`), Area 17 | Units drawn, UI elements counted |
| S5 soak | Long headless run at unlimited speed | All primary games | E1; memory slope |
| S6 idle floor | Start state, no units added | All games | Fixed per-frame cost: dispatch, unimplemented-callin checks, idle UI |

Scale ladder: 1× (jam), 4×, 16×, 64×, then doubling until one backend misses the 33.3 ms sim budget at 1× speed, plus one step past it. Reported as cost curves: ms per frame against units or events, with the slope and the budget crossing per backend.

## Measurement method

### Run modes

| Mode | Binary | Pacing | Measures |
|---|---|---|---|
| H throughput | `build-amd64-linux/install/spring-headless` | Unlimited speed (`MaxSpeed` raised through `[modoptions]`), fixed frame window | Sim frame distribution, throughput, sim script time, GC, memory |
| R lockstep | `build-amd64-linux/install/spring`, `VF_RENDERER=gpu`, 1024x736 | `Spring.SetVideoCapturingMode(true)`: one sim frame per draw frame | Draw frames paired with sim frames; draw script time without pacing noise |
| R realtime | Same | 1× speed, vsync off, uncapped FPS | Player-realistic draw distribution, sim and draw interference, tails |
| Load | Both | Launch to first sim frame; cold (page cache dropped when permitted, otherwise first run after content change) and warm | Load split per phase |
| Attribution | `build-amd64-linux-tracy/install` (`TRACY_ON_DEMAND`); perf with `SPRING_WASM_JITDUMP=1` | As the timed mode | Zones, sampled stacks; never used for headline timings |
| Counting | Release build with import counters and Lua callout wrappers on | As H throughput | Host-call and callout counts; never used for timings |

Fixed camera and parked cursor through the input script. Rendered timing uses the GPU backend; `software` (llvmpipe) is never used for timing.

### Repeats and order

- At least 5 timed runs per cell (game, scenario, scale, backend, mode), more when the p99 interval is wider than the effect.
- Order ABAB…, each run a fresh process, so slow host drift lands on both backends.
- Warmup is not discarded silently: the first 300 frames (load, Wasm compile tail, Lua table growth) are a reported window of their own; steady-state metrics use a fixed later window, the same for both backends.
- One timed engine at a time. Attribution and counting runs may overlap other non-timed work only when memory allows. The user's interactive game is never stopped; its presence is recorded and the run is marked contended.

### Host conditions

| Item | Rule |
|---|---|
| CPU | i7-12700, hybrid. Pin the engine to P-cores (`taskset -c 0-15`, `/sys/devices/cpu_core/cpus`); record whether E-cores were excluded |
| Governor | `performance` during timed runs (host default is `powersave`); recorded before and after, restored after |
| Turbo, thermals | Recorded per run (`/sys/devices/system/cpu/intel_pstate/no_turbo`, package temperature) |
| Contention | Load average, `/proc/pressure/{cpu,memory,io}`, top foreign processes by CPU, free memory and swap activity, sampled through the run |
| GPU | Renderer, driver and GL strings (`vf_renderer_info`) for rendered modes |
| Gate | A run with CPU pressure `some avg10` above 5% or a foreign process above 10% of one core during the window is kept and marked `contended`; it is excluded from accepted results |

### Instrumentation overhead

- Timed runs use the release build with only the frame timing log on. Its overhead is measured once per engine change: S6 and one primary scenario with the log on and off.
- Tracy builds: compare not-attached against the release build, then attached against not-attached. Zone attribution is used only when the attached run's frame times stay within the overhead recorded for it.
- perf with jitdump changes Wasmtime compilation work; its sampled share is used, never its timings.
- Valid captures prove the tooling worked, not that timings are clean (Ship Game profiling convention). Every result set starts `diagnostic`; `accepted` requires the review in Reporting.

## Engine work

Instrumentation gaps found while writing this plan. Each item applies to both backends so attribution stays symmetric.

| # | Item | Where | Gap today |
|---|---|---|---|
| 1 | Wasm callin timers: `SCOPED_SPECIAL_TIMER` per context (`Wasm::Callins::Synced`, `::Unsynced`, `::UI`) matching `Lua::Callins::*`, and a Tracy zone per callin naming module and callin under plain `TRACY_ENABLE` | `NativeInterfaceEventClient::DispatchWasmCallin`, `rts/NativeInterface/NativeInterfaceEventClient.cpp` | Wasm callins have no TimeProfiler timer; the `Wasm::<callin>` zone exists only with `RECOIL_DETAILED_TRACY_ZONING`, which is off in `build-amd64-linux-tracy` |
| 2 | Frame timing and work counter log: start-script or environment option writing one CSV row per sim and per draw frame (frame times, script times per context and module, Lua GC time, Lua heap, Wasm pages, E1 counters) | Uses the implemented per-frame log described in [Sync Verification](recoil_wasm_sync_verification.md#what-a-matching-hash-establishes) | None; TimeProfiler keeps aggregates, not per-frame rows |
| 3 | Per-gadget and per-widget Lua zones, behind an engine flag, Tracy builds only | Base handlers in `cont/base/springcontent/LuaGadgets/`; games with their own `LuaUI/main.lua` handler get the same wrapper in the staged copy | Lua time is attributable only per context, not per gadget; TTLD's `dbg_gadget_profiler_new.lua` and Gravitas' `dbg_profiler.lua` are game-specific and add their own overhead |
| 4 | Host import counters: count per import per interval, optional per-import timing in diagnostic builds | `ImportGuard`, `rts/WasmInterface/core/host/WasmCoreGeneratedSupport.h` | No per-import counts |
| 5 | Engine input script and end frame: start-script key naming a file of `frame: action` lines (console actions, `/give`, unit orders, LuaRules messages, camera), and `QuitAtFrame` | `rts/Game/` | Inputs come from per-language test drivers; runs end through a Lua gadget |
| 6 | Load zones: Wasm compile, instantiate, `Initialize` per module; Lua parse and `Initialize` per handle | `rts/WasmInterface/runtime/WasmRuntime.cpp`, `rts/WasmInterface/system/`, `rts/Lua/LuaHandle.cpp`, `rts/Game/LoadScreen.cpp` | Load cost not split by backend |
| 7 | Wasm memory stats per instance: current and peak pages, `memory.grow` count | `rts/WasmInterface/runtime/WasmResources.*` | Not exposed for logging |

Lua callout counts come from a diagnostic wrapper over `Spring.*`, `gl.*` and `Script.*` tables in the staged handler, used in counting runs only.

## Attribution: transport vs guest compute

```text
Wasm callin = dispatch + guest compute + Σ imports (marshalling + engine API work)
Lua callin  = dispatch + handler loop + interpreter + Σ C API calls (push/check + engine API work) + GC steps
```

| Step | Method | Output |
|---|---|---|
| 1 | Zone totals per module and callin (Wasm) and per gadget and callin (Lua) | Where script time is, per game |
| 2 | perf sampling with jitdump, stacks bucketed by symbol: JIT frames = guest compute; `recoil::wasm::core` bindings, `ImportGuard`, wire = marshalling; `rts/NativeInterface/api` and engine code = API work. Lua: `luaV_execute`, `luaD_*`, `luaH_*` = interpreter; `lua_push*`, `luaL_check*`, `LuaUtils` = marshalling; `LuaSynced*`, `LuaUnsynced*` = API work; `luaC_*`, `luaM_*` = GC and allocation | Stacked shares per game and backend |
| 3 | Counts × microbenchmark cost: per-import counts times the matching `core_ceiling_*` and callout rows in [generated results](generated/benchmarking_results.md) and [diagnostics](generated/benchmarking_diagnostics.md) | Predicted transport cost, compared with step 2 |
| 4 | Guest codegen: shared crates with host tests (`hunted-ai`, `ttld-shared`, …) benchmarked natively and as Wasm with the same inputs | Wasmtime codegen penalty, separate from transport |
| 5 | Hot-pair review: for the top three callins by ratio, read the Lua gadget and its Rust counterpart; record algorithmic differences | Gain classed as runtime, rewrite, or both |

A prediction off by more than 2× from the measured transport share means the microbenchmarks miss a shape; the shape becomes a new row in `test/native_api_parity/run_benchmarks_core.py`.

Runtime factors checked in attribution runs: NaN canonicalisation (`wasmtime_config_cranelift_nan_canonicalization_set`, `WasmRuntime.cpp`) on unsynced and UI modules; guest build profile (ports use Cargo release defaults: no LTO, 16 codegen units), A/B with LTO and one codegen unit; per-launch compilation (`allowAotDeserialization` is off) in load time; cold callin entry on render callins.

## Harness

A `perf` suite in `springcabal-games/verification/` reusing staging, start scripts, renderer selection and the verify lock:

```bash
just verify perf <game> <scenario> [--scale N] [--mode headless|lockstep|realtime|load|attribution|counting] [--repeats N]
```

It stages both versions, writes normalised start scripts with `FixedRNGSeed`, the input script and the end frame, runs ABAB, samples host state, checks E0 to E2, and writes the run directory. Raw evidence goes to `springcabal-games/perf-runs/<YYYY-MM-DD>-<game>-<scenario>/`, not `verification/out/`, which `just clean-verification-out` prunes. Contents: frame CSVs, infologs, host-state samples, provenance (engine, game and module hashes, start scripts, settings), equivalence results, Tracy captures and perf data for attribution runs only, and `baseline-status.json` (`diagnostic`, `accepted`, `rejected`). Tracy captures are large; one per cell, not per repeat.

## Reporting

Dated reports in `rts/wasm/docs/reports/lua-wasm-game-perf-<YYYY-MM-DD>.md`, first line the date. Each result section states:

| Field | Content |
|---|---|
| Effect | What is compared (backend, or engine change A/B) |
| Scene | Game, scenario, scale, frame window, mode, camera |
| Setup | Engine commit and build, game commits, module hashes, start script hash, settings |
| Conditions | Host state summary, governor, pinning, contention verdict, repeats, baseline status |

Per cell:

| Metric | Lua p50 [p95, p99, max] | Wasm p50 [p95, p99, max] | Wasm/Lua p50 (95% CI) | Wasm/Lua p99 (95% CI) | Verdict |
|---|---:|---:|---:|---:|---|

Plus script share and whole-frame ratio side by side, the attribution stack per backend, E1 and E2 results, and scaling curves. Issues found get their own section: symptom, reproduction command, attribution, suspected owner (engine, runtime, port).

Acceptance review before a result set is marked `accepted`: fixed binaries and inputs across repeats, no contended runs in the set, instrumentation overhead recorded, repeat spread within the effect, E1 pass, attribution coverage (unresolved stack share) stated.

## Decisions this feeds

| Result | Engine priority change |
|---|---|
| Wasm sim script time and whole sim p50 and p99 at or below Lua in every game at jam scale | Gain confirmed; report whole-frame size of it; continue with scale results |
| Script share small in jam games, whole-frame difference below the threshold | Wasm's value for these games is scale and safety, not frame time; engine-side sim costs take priority |
| Wasm slower in script time or tails in any game at jam scale | Blocking: attribute and fix before other Wasm work |
| Transport over a third of Wasm script time in any game | Batched or borrowed APIs for the top imports by cost |
| Render callins (`DrawWorld*`, `DrawScreen*`) material in rendered runs | Render-callin dispatch fix, guided by the `callin_drawworld_*` diagnostic rows |
| Idle floor shows unimplemented-callin or fan-out cost | Callin subscription masks; per-module dispatch cost |
| Wasm compile dominates the load-time difference | Compiled-module cache (serialised modules, `allowAotDeserialization` with validation) |
| Linear memory grows without bound in soak, or peak well above Lua heap | Guest allocator and leak work |
| Large guest codegen penalty (step 4) | Guest build profile defaults in `springcabal` templates; Cranelift and NaN-canonicalisation settings per world |
| Microbenchmark predictions off by more than 2× | New microbenchmark rows for the missing shapes |
| Gains appear only past a scale | State the crossing per game; scaling becomes the advertised case |

## Order of work

1. **Inventory.** Residual Lua per port, modules per game, UI frameworks, which Lua versions run headless to a fixed frame (Gravitas' Lua archive waits at the pre-game boundary under the headless engine), autonomous-scenario switches (invulnerable player, no-input start). Output: inventory table in the first report.
2. **Engine instrumentation.** Items 1, 2, 4, 6, 7, then 5 and 3; overhead measured for each.
3. **Harness.** `perf` suite, host-state capture, ABAB runner, evidence layout.
4. **Equivalence.** E0 to E2 on My Cube and TTLD; explain first divergence frames; fix tolerances.
5. **Pilot.** My Cube and TTLD, H throughput at jam scale; measure run-to-run spread and set repeats.
6. **All games, jam scale.** H throughput, R lockstep, R realtime, load.
7. **Attribution.** Steps 1 to 5 for every game; microbenchmark cross-check; issues filed.
8. **Scaling and stress.** S1 to S6 ladders.
9. **Clean run.** On user request: settled host, performance governor, accepted result set and report.
10. **Follow-up.** Engine fixes A/B on the same scenarios (old vs new engine, same backend); new microbenchmark rows; re-run affected cells.

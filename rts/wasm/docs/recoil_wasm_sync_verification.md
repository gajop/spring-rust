# Recoil Wasm Sync Verification Plan

Status: local Linux memory/checksum verification and deterministic jobs implemented
(2026-10-12); see the [local report](recoil_wasm_synced_jobs_report_2026-10-12.md)
and [API](recoil_wasm_synced_jobs.md). The cross-platform matrix and broader
perturbations below remain planned.

Integration checked on 2026-10-18: `rust-wip` contains the jobs integration
(`c24a0727aa`), opt-in memory-hashing correction (`4dc4e25e0d`), and engine
follow-up fixes (`e36d269449`). The separate `codex/wasm-synced-jobs` development
history was consolidated into the integration commit; it is not pending a merge.
The broader matrix below is future engine verification work, beyond the completed
local task. No GitHub Actions were run or added for that task. Ship Game adoption
and gameplay work belong in Ship Game's own plans.

Goal: prove automatically, without playtesting, that every synced Wasm game stays in sync across the supported platforms (amd64-linux, arm64-linux, amd64-windows) and across machine conditions that must not matter. One binary per platform is shipped, so compiler variety is out of scope; operating system and CPU architecture are in scope.

This plan implements the "Synced determinism tests" of [Parity & Testing](recoil_wasm_parity_testing.md) and the "Synced acceptance tests" of [Sandbox & Determinism](recoil_wasm_sandbox_determinism.md), and extends them from engine fixtures to real games.

## Why games, not only fixtures

| Game | Role |
|---|---|
| Ship Game | Needs sync for its own multiplayer. Large synced Rust/Wasm rules, AI, flight, formations; long matches |
| SpringCabal Wasm ports (Gravitas, Hunted, Area 17, Flove, Parts Needed, MyCube, To The Last Drop) | Single-player today; their value is coverage of engine and Wasm runtime behaviour that fixtures miss |
| Engine fixtures (`test/native_api_parity`, BAR synctest) | Narrow, fast, run on every engine change |

A divergence in any of them is treated the same way: as an engine, runtime or host-API bug until shown to be game code.

## What exists

| Facility | Where | Covers | Gap |
|---|---|---|---|
| Per-frame sync checksum | `rts/System/Sync/SyncChecker.h` (`SYNCCHECK`, on by default) | Running hash of synced engine assignments; the server compares clients' checksums live | Wasm linear memory is not hashed: a guest divergence is invisible until it changes engine state |
| Demo desync detection | Server echoes sync responses every `syncResponseEchoInterval` frames (`rts/Net/GameServer.cpp`); replay compares and logs `[DESYNC WARNING]` (`rts/Net/NetCommands.cpp`) | Any recorded game, replayed anywhere | Nothing runs replays automatically; coarse interval |
| Sync history | `SYNC_HISTORY` in `SyncChecker`, `SYNCDEBUG` builds | Per-assignment history for localisation | Manual, engine state only |
| BAR synctest | `.github/workflows/synctest.yml`, `test/synctest` | One seeded BAR scenario on three platforms, digest compared | Lua game only; manual trigger |
| Wasm Core parity | `.github/workflows/wasm-cross-platform.yml`, `test/native_api_parity` | Synced Wasm fixture: per-frame observations, FP edge cases, RNG, three platforms | Fixture only; manual trigger |
| Float test | `.github/workflows/streflop-float-test.yml` | Engine C++ floating point | — |
| Runtime determinism config | `rts/WasmInterface/runtime/WasmRuntime.cpp` | NaN canonicalisation, deterministic relaxed SIMD, no threads or shared memory in synced instances | — |

## Engine work

### 1. Hash synced Wasm state into the sync checksum

After each simulation frame, synced module metadata and cached job digests are
fed to `CSyncChecker::Sync`. Full linear-memory hashing uses XXH3-64 only when
explicitly enabled, under the policy below.

- **Implemented policy**: full-memory hashing is off in normal games (`system.WasmStateHashInterval = 0` by default). `WasmSyncLog` hashes every frame for local verification without changing the network checksum; a positive game-owned interval explicitly opts into network memory scans. Normal checks cover only module metadata and cached job digests. See [the cost report and fix](recoil_wasm_state_hash_cost_2026-10-12.md). Dirty-page tracking may support cheaper full-state coverage later.
- **Unsynced instances are excluded**: they legitimately differ per client.
- **Coverage**: with diagnostic logging enabled, a linear-memory divergence is visible on its first logged frame, without per-game code. Normal metadata checks do not detect arbitrary guest-memory changes. Opt-in network memory checks detect persistent differences on the next scan; transient differences between scans may escape them.
- **Precondition**: no host call writes client-dependent data into synced guest memory. Any such write is itself a sync bug, and this hash will expose it.

### 2. Sync log

A start-script or command-line option writes one line per frame (or per interval): frame, engine checksum, and each synced instance's memory hash and size. Tests compare logs instead of relying on networked comparison or demo echoes.

### 3. Divergence localisation

On a mismatch between two logs:

1. **First frame**: the earliest differing line.
2. **Which part**: engine checksum or which Wasm instance.
3. **Where in guest memory**: both platforms re-run deterministically to that frame and dump the instance's linear memory; a diff lists differing pages and offsets, mapped to symbols through the module's name section and DWARF where statics are involved.
4. **Which game system**: optional per-game digests (below) narrow heap divergences to a subsystem.
5. **Engine state**: `SYNC_HISTORY` / `SYNCDEBUG` builds for divergences in engine checksums.

The tooling (log compare, dump, page diff, symbol map) lives in `test/synctest/` and runs locally and in CI.

### 4. Perturbation runs on one machine

Some sync bugs desync different machines but show on a single one when conditions change. Each scenario also runs twice locally with one condition varied, logs compared:

| Varied | Catches |
|---|---|
| Fresh process (address layout) | Pointer-ordered iteration, uninitialised memory |
| Frame pacing and simulation speed | Wall-clock or render-frame state leaking into synced code |
| Headless vs rendered client | Unsynced code changing synced state |
| Engine thread count (`PathingThreadCount`, future job workers) | Order-dependent parallel work |
| Two processes over localhost (host plus client) | The live networked path; the server compares checksums as online |

### 5. Demo replay checks

A runner replays recorded demos headless with the sync log on, and fails on any `[DESYNC WARNING]` or log difference. Inputs: each game's recorded demos (Ship Game keeps a replay library; SpringCabal suites can record theirs). Runs on all three platforms against demos recorded on any one of them.

### 6. Static checks for synced guest code

Shared Clippy configuration for synced crates: disallowed types and methods (hash maps and sets with random state, `std::time`, OS entropy), and a check that synced crates link only synced API surfaces. The host API audit list in [Sandbox & Determinism](recoil_wasm_sandbox_determinism.md#host-api-determinism) becomes a checked inventory of synced callouts with their determinism status.

### 7. CI

| Workflow | Runs | Trigger |
|---|---|---|
| `wasm-cross-platform.yml`, `synctest.yml` (existing) | Fixtures, BAR | After every engine build, not only manual |
| `wasm-games-synctest.yml` (new) | Platform × game matrix: each game's scenarios with the sync log, then a cross-platform log compare and, on failure, localisation artifacts | Every engine build on the default branch; nightly with longer scenarios |
| Demo replay job (new) | Each platform replays the demo set | Nightly |
| Perturbation job (new) | Linux, all games, every perturbation | Nightly |

Pinned game versions per engine branch, as BAR's synctest pins its game: an engine change must not be blamed on a game change.

A local loop for developers: `just sync-check <game>` runs the scenarios natively and under Wine with the Windows build, and compares logs. arm64 stays in CI.

## Per-game work

Kept small: the engine side (memory hash, sync log, replay runner, localisation) is generic.

| Item | Ship Game | SpringCabal games |
|---|---|---|
| Seeded scenario ending at a fixed frame | AI-versus-AI skirmish with several AIs and doctrines; formation, flight and economy puzzles; long match nightly | Existing `verification` headless suites (`headless-integration` feature, scripted `e2e` scenarios); each needs a fixed seed and a fixed end frame |
| Inputs only through synced paths | Scenarios drive teams by AI and synced commands; no unsynced widgets issuing orders by wall clock | Same; scripted UI input is replaced by demos or synced test commands |
| Demos for replay checks | Replay library plus CI matches | One recorded playthrough per game |
| Subsystem digests (optional) | Yes: economy, formations, AI knowledge, flight state, logged per interval | Not needed |
| Pin file | Game version and map for each engine branch | Same |

BAR's experience applies: unsynced code that issues orders makes runs irreproducible (its synctest disables LuaUI for that reason). Every scenario must pass the perturbation runs on one machine before its cross-platform result means anything.

## Acceptance

- Identical sync logs (engine checksum and every synced instance hash, every frame) across the three platforms for every game scenario.
- Identical logs across every perturbation on one machine.
- No `[DESYNC WARNING]` replaying any demo in the set on any platform.
- A deliberately injected divergence (a test-only build reading a platform value in synced code) is reported at the right frame, instance and memory offset.

## Order of work

1. Sync log and Wasm memory hashing in the engine; localisation tooling for two logs.
2. Ship Game scenarios on Linux: perturbation runs until reproducible.
3. Ship Game cross-platform: Wine locally, then the CI matrix.
4. SpringCabal headless suites with fixed seeds and end frames, added to the matrix.
5. Demo replay runner and nightly jobs.
6. Static checks and the synced host API inventory.
7. Thread-count perturbation becomes required once parallel synced work (pathfinding threads, job workers) exists.

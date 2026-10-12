# Wasm state hashing costs a whole frame, 2026-10-12

**Regression.** Since `c24a0727aa` ("Add deterministic Wasm jobs…") the engine hashes every
synced Wasm module's complete linear memory after every simulation frame and mixes the hash into
the sync checksum. For Ship Game that is 512 MiB a frame: about 22 ms of simulation-thread time,
which cuts headless simulation throughput about sevenfold and leaves a normal-speed game almost no
frame budget. Whole-memory hashing is a desync-diagnosis aid; it should not be on, and certainly
not every frame, in ordinary games.

## What runs

`CGame::SimFrame` → `WasmSyncVerification::EndFrame(frame)` (`rts/WasmInterface/system/WasmSyncVerification.cpp`)
→ `WasmCoreHost::SyncedStates()` (`rts/WasmInterface/core/host/WasmCoreHost.cpp`) when
`frame % modInfo.wasmStateHashInterval == 0`. `SyncedStates` takes the registry lock and runs
`XXH3_64bits` over each synced module's whole linear memory (`memory.Size()`, not what the module
uses). The states are computed whether or not the build defines `SYNCCHECK`; only the mixing into
`CSyncChecker` is conditional.

`wasmStateHashInterval` defaults to 1 (`rts/Sim/Misc/ModInfo.cpp`): every frame. A game can only
lengthen the interval through `gamedata/modrules.lua` (`system.WasmStateHashInterval`); there is no
way to turn it off.

Ship Game's synced rules module is linked with a fixed 512 MiB memory
(`--initial-memory=536870912 --no-growable-memory`, `wasm-src/build.rs`), so all 512 MiB are hashed
however little of it is in use. `WasmSyncLog` confirms it: one synced module, `shipgame-rules`,
536,870,912 bytes, hashed each frame.

## Measurements

Engine `rust-wip` at `c24a0727aa`, headless release build installed 2026-10-12 09:13. Game
`shipgame` at `6e5ef8f0`, the same immutable snapshot for every run except the one modrules line.
Scenario: `tools/launch.py check skirmish --allies 1 --enemies 2 --spectate`, three AI teams on
the default 64×64 map, Intel Core i7-12700, one engine at a time.

| `WasmStateHashInterval` | Speed 20, frames reached in 30 s | Wall time per frame |
|---|---:|---:|
| 1 (the default) | 1,182 | 25.4 ms |
| 30 | 7,950 | 3.8 ms average, with a 22 ms frame every 30 |
| 300 | 9,030 | 3.3 ms |
| 1,000,000,000 (effectively off) | 8,100 | 3.7 ms |

An earlier pair on the same engine (default against off) gave 1,050 and 8,415 frames. The
difference, about 22 ms a frame, is one XXH3 pass over 512 MiB at memory bandwidth.

At speed 1 for 40 s the default still reaches real time headless (1,050 frames against 1,092 with
hashing off), but only just: 22 of the 33 ms frame budget go to the hash before the game's own
rules (about 5 ms in this skirmish) and before any rendering on the same thread. A larger battle,
a slower machine or a rendered client falls behind real time. A longer interval only turns the
cost into a 22 ms hitch every N frames.

## Why this is a debug feature

The engine's ordinary sync check already covers the simulation state (units, features,
projectiles, commands). A Wasm module that diverges between clients diverges in what it does to
that state, and the ordinary check catches it within frames. Hashing the module's memory finds
the divergence earlier and names the module, which is what one wants while hunting a desync,
not what every player should pay for in every frame of every game.

## Recommendation

1. **Off by default.** `WasmStateHashInterval = 0` (or an equivalent switch) meaning no Wasm
   memory hashing, and make 0 the default. The jobs submission hash (IDs, frames, inputs) is
   cheap and can stay.
2. **Opt in for diagnosis.** Turn it on for desync hunts: the game's modrules for a test build, or
   the existing `WasmSyncLog`/`WasmSyncDumpFrame` diagnostics, which already log every frame
   without touching what enters the network checksum. Replay-based verification
   (`test/synctest/wasm_sync_replay.py`) keeps working with it on.
3. **If it must run in games, make it cheap.** Hash a rotating slice of memory each frame (for
   example 1/256th: about 0.1 ms here, full coverage every ~8.5 s), or only dirty pages (the
   docs already list dirty-page tracking as future work). Never hash a whole fixed-size memory
   synchronously in the frame.

Until the default changes, Ship Game can only lengthen the interval, which trades the per-frame
cost for periodic hitches; it has not done so.

## Reproduction

```sh
cd ship-game.sdd
# A snapshot per setting, with only gamedata/modrules.lua changed:
#   system.WasmStateHashInterval = 30 | 300 | 1000000000
uv run --quiet python tools/launch.py check skirmish --game <snapshot>/game.sdd \
  --seconds 30 --speed 20 --allies 1 --enemies 2 --spectate --run-root <out>
grep -o 'f=[0-9]*' <out>/check/write/stdout.txt | tail -1
# What is hashed:
uv run --quiet python tools/launch.py check skirmish --seconds 20 --speed 20 --allies 1 \
  --enemies 2 --spectate --run-root <out> --setting WasmSyncLog=<out>/sync.jsonl
```

Evidence (outside this repository): `~/.local/share/ship-game-verify/2026-10-12-jobs/`
(`game-*` snapshots, `run-*` speed-20 runs, `speed1-*` real-time runs, `hashlog/sync.jsonl`).

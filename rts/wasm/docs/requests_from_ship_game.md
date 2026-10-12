# Engine requests from Ship Game

Issues and requests found while working on Ship Game, newest first. Each names how to see it.
Reports with measurements live beside this file (for example the
[state hashing cost report](recoil_wasm_state_hash_cost_2026-10-12.md), resolved in `4dc4e25e0d`).

## 2026-10-12 · Creating feature 32001 crashes the engine; the cap limits large maps

**Resolved overflow:** the loader checks available memory pages before allocation,
and automatic-ID admission checks the ID pool. Full-pool creation now returns
no ID (`-1` through the Core/Rust API) without crashing. Pool reuse remains
subject to the existing 32-frame ID-recycling tick. Wasm stress verification
fills the pool, rejects further calls, destroys a feature, then successfully
allocates after recycling; recording and 1/4-thread replays agree for all 37
frames. See [local fix report](ship_game_engine_fixes_2026-10-12.md).

**Cap reviewed:** 32,000 units plus 32,000 features occupy a shared 16-bit ID
range, enforced in `GlobalConstants.h`. Supporting 48,000 features while keeping
the unit cap requires a wider combined ID representation and an audit of the
network, save, and object-reference paths. A modrule or a larger constant alone
cannot safely bypass this constraint. This patch retains both caps; a wider-ID
change is separate work, not part of the overflow fix.

Creating features past `MAX_FEATURES` (32000, `rts/Sim/Misc/GlobalConstants.h`) from Wasm
segfaults the engine instead of failing the call:

```
[CrashHandler] Error: Segmentation fault ...
  StaticMemPool<32000ul, 1536ul, 64ul>::allocMem(unsigned long)   MemPoolTypes.h:309
  CFeatureHandler::LoadFeature(FeatureLoadParams const&)           FeatureHandler.cpp:113
  NativeCreateFeature(...)                                         SyncedCtrl.cpp:3834
  CoreDynamicInput_feature_control_create_feature(...)
```

`CFeatureHandler::CanAddFeature(id)` returns `true` for any `id < 0` (a pool-assigned ID) without
asking whether the pool has room, so `featureMemPool.alloc` runs on a full pool.

Seen with Ship Game's stress option on a 1024×1024 blank map:
`tools/launch.py check skirmish --map 1024 --modoption sb_asteroid_fields=800` (48,000 asteroids,
each a feature). 200 fields (12,000 asteroids) run fine.

**Requests:**

1. `CanAddFeature` (or `LoadFeature`) checks pool capacity, and `create-feature` returns no ID when
   the pool is full. A game can then degrade (fewer asteroids, no wreck) instead of crashing.
2. Consider the cap itself for large maps. Ship Game targets 1024×1024 maps and keeps its own
   navigation memory in proportion to objects, not area. Asteroids and wrecks are features, so a
   dense large map can want more than 32,000. A modrule (like `WasmMemoryLimit`) or a higher
   constant would do. The units cap (`MAX_UNITS`) may deserve the same look.

## 2026-10-12 · Disabled synced timers trap and unload the rules module

**Resolved:** new checked Core timer imports return `NotAvailable` when disabled;
the Rust SDK's existing `Result<u64>` API now uses them. Legacy scalar imports
return deterministic zero for already-built guests instead of trapping. Actual
clock reads still require `SPRING_ENABLE_SYNCED_TIMERS=1` in synced worlds.
Disabled and enabled probes pass; disabled profiling leaves the rules alive.

`spring:desync` timers (`get-timer`, `get-timer-micros`) trap when synced timers are disabled.
The trap faults the calling callin, and the engine deregisters the whole synced rules module:
the game carries on with no rules at all.

```
Warning: Core Wasm module shipgame-rules was deregistered due to fault: GameFrame: core Wasm
export call failed ... Caused by: synced Wasm timers are disabled; set
SPRING_ENABLE_SYNCED_TIMERS=1 for diagnostic benchmarking
```

Seen with Ship Game's profiling option (`--modoption sb_profile_game=1`) without
`SPRING_ENABLE_SYNCED_TIMERS=1`. The Rust SDK's `spring::desync::get_timer_micros()` returns a
`Result`, so a guest already expects to be told no; it never gets the chance.

**Request:** return a status (`NotAvailable`) instead of trapping, so a diagnostic path degrades
(timings read as zero) rather than taking the game's rules down. Keep the trap, if wanted, behind a
strict-determinism setting.

## 2026-10-12 · groundFX atlas error on every load

**Resolved headless message:** groundFX reporting now matches the existing
projectile-atlas path: headless allocation failure does not emit a graphics
error. Rendered failures remain errors. Local headless and Xvfb rendered checks
have no groundFX error, and the rendered recording/replays agree for all seven
frames. This does not increase texture limits or hide rendered failures.

Every Ship Game load, headless included, logs:

```
Error: Could not finalize groundFX texture atlas. Use fewer/smaller textures.
```

Same on the default 64×64 map and on a 1024×1024 one. Nothing visible breaks in headless runs; in
a rendered game it may mean ground decals are missing. Either the atlas limit or the message
deserves a look.

## 2026-10-12 · Note: 1024×1024 maps load and run

For the record, not a request: a six-AI skirmish on a 1024×1024 blank map
(`tools/launch.py check skirmish --map 1024`) loads in about 6 s and runs headless at about 2 GB
resident, with Ship Game's rules frame at 1.3 ms median. With 12,000 asteroids
(`--modoption sb_asteroid_fields=200`) it holds about 3.8 ms median after Ship Game's own scaling
fixes. Ship Game's navigation now keeps memory
per object rather than per map area, so large maps are a target.

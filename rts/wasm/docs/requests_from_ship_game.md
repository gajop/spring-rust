# Engine requests from Ship Game

Issues and requests found while working on Ship Game, newest first. Each names how to see it.
Reports with measurements live beside this file (for example the
[state hashing cost report](recoil_wasm_state_hash_cost_2026-10-12.md), resolved in `4dc4e25e0d`).

## 2026-10-13 · Camera bounds and a minimap view rectangle, for maps whose regions are seen one at a time

**Implemented:** unsynced commands `camerabounds x0 z0 x1 z1` and
`minimap view x0 z0 x1 z1` opt into the two independent rectangles, in world elmos.
`camerabounds clear` and `minimap view clear` restore the whole map. Rectangles must be finite,
ordered and inside the map; invalid input leaves the current rectangle unchanged. Neither
setting persists into another game.

Camera focus clamps apply to all controllers and transitions, with rectangle-sized Spring
and Overhead zoom limits, centred zoom resets and a centred overview. Minimap backgrounds,
world drawing, icons, notifications and camera frames use the rectangle; rotations and
clicks use the same mapping. Icons centred outside it are omitted. Uniform minimap matrices
include the region offset/scale; Lua/SDK `DrawInMiniMap` retains its pixel-space drawing
contract and clips to the minimap. The uncached minimap path also supports the rectangle.

Verification: `xvfb-run -a python3 test/view_rectangles/run.py` runs a graphical fixture with
all seven camera modes, out-of-bounds targets/state, zoom limits/resets, clear/region switches,
four minimap rotations and ray coordinates, cropped two-colour backgrounds, icon filtering,
and oversized chart clipping. Repeat with `--direct` for `MiniMapRenderToTexture=0`.
The normal and headless engines are rebuilt; the commands below are also a manual check in
Ship Game after choosing a region.

Ship Game's wormhole mode (branch `wormholes`, `docs/design/wormholes.md` there) puts two regions
on one null map, a home arena and a frontier, with an empty gap between them that only a wormhole
crosses. The player looks at one region at a time and switches with a game button. Today the
engine lets the camera and the minimap cover the whole map, so arrow keys, edge scrolling,
dragging and zooming out carry the view across the gap into the other region, and the minimap
shows both regions squeezed together.

To see it: in Ship Game, `just play wormhole`, zoom out and scroll down.

**What the game needs:**

1. **Camera bounds:** a world rectangle (x0, z0, x1, z1) that every camera controller keeps its
   focus inside, in place of the map's edges. This covers keys, edge scroll, mouse drag,
   minimap clicks and set state/target. The zoom-out limit (Spring `maxDist`, Overhead
   `maxHeight`, both from the map's longest side today) should fit the rectangle, and the zoom
   reset and the overview camera should centre on it. Clearing it restores the whole map.
2. **Minimap view rectangle:** a world rectangle the minimap shows instead of the whole map. The
   background, unit icons, the camera frustum box, `DrawWorldStuff` (projectiles, commands,
   ranges) and the `DrawInMiniMap` transforms map through it. Clicks map into it
   (`GetMapPosition`, which `trace_screen_ray(use_minimap)` uses), and nothing outside it is
   drawn. Clearing it restores the whole map.

Both are unsynced view state, off by default, with no effect on games that don't set them.
Either form works for Ship Game: text commands it can send with `send_commands` (e.g.
`camerabounds x0 z0 x1 z1`, `minimap view x0 z0 x1 z1`), or SDK calls. Camera clamp sites today:

- `OverheadController.cpp` `Update`, its zoom reset and `maxHeight`
- `SpringController.cpp` `Update`, its zoom reset and `maxDist` (const)
- `RotOverheadController.cpp` `Update`
- `FPSController.cpp`
- `OverviewController.cpp` `SwitchTo`

Minimap sites: `MiniMap.cpp` `UpdateGeometry` (`viewMats`), `GetMapPosition`,
`DrawCameraFrustumAndMouseSelection`, `DrawUnitIcons`, `DrawWorldStuff`, `DrawBackground`,
`DrawCircle`, the aspect-ratio helpers.

Ship Game will send the current region's rectangle on every switch and keep its own boundary
grid and minimap chart in step.

## 2026-10-13 · Single rules-param getters have no typed Rust wrapper (game, team, player, feature)

**Resolved:** the generated owned SDK now lowers string input descriptors together with
recursive dynamic results. All five single-value getters return `Result<RulesParamValue>`;
missing or visibility-hidden values return `ErrorCode::NotFound`. The façade also re-exports
`RulesParamValue` and `RulesParamType`. `get_game_rules_param_f32(name)` uses a new compact,
allocation-free host import and returns `InvalidArgument` for a non-float value.

Verification: `python3 test/wasm_api/check_rules_params.py` builds a guest and runs the headless
engine against all five scopes, round-tripping bools, floats, empty strings and long UTF-8
strings, including buffer growth, cached sizes, missing/hidden values and float type errors.
`python3 rts/wasm/verify_codegen.py` checks generated SDK and parity artifact reproducibility.

Found adding wormholes to Ship Game: the synced rules publish match-wide state with
`set_game_rules_param` (`los::PUBLIC`), and the unsynced and LuaUI Wasm modules could not read it
back.

In the generated SDK (`core_generated.rs`, module `owned::rules_params`), every **list** getter is
typed (`get_game_rules_params(unused) -> Result<Vec<String>>`), but every **single-value** getter
is only the exact Core ABI forwarder, `#[doc(hidden)]`:

```rust
pub fn get_game_rules_param(p0: i32, p1: i32) -> i32              // game
pub fn get_team_rules_param(p0: i32, p1: i32, p2: i32) -> i32     // likewise team, player,
pub fn get_feature_rules_param(p0: i32, p1: i32, p2: i32) -> i32  // feature, unit
```

The typed form a guest would expect is the mirror of the setter,
`get_game_rules_param(name: &str) -> Result<RulesParamValue>`. The generator seems to skip it
because the output is a record with a string inside. Units are only usable because
`spring/src/rules_params.rs` hand-writes `get_unit_rules_param_f32` over its own import.
`spring::rules_params` re-exports the raw forwarders under the typed names, so they look like the
API.

Ship Game works around it in `shipcore::game_params::float` by building the input descriptor
(name wire) and decoding the output record (type, bool, float, string length and bytes, los,
exists) by hand. That depends on the wire layout and breaks silently if it changes.

**Request:** generate the typed single-value getters (`get_{game,team,player,feature,unit}_rules_param`
returning `Result<RulesParamValue>`, with a not-found result), and for symmetry with units a
`get_game_rules_param_f32(name) -> Result<f32>`. Once these exist, Ship Game drops its decoder.

## 2026-10-12 · Rules float code runs 2–3× slower with NaN canonicalization; nearest-enemy search costs its circle's area

Found profiling a 1,000-a-side battle (`tools/launch.py check perf-combined` in Ship Game; report
`docs/ai/reports/battle-scale-2026-10-12.md` there).

**NaN canonicalization.** `WasmRuntime.cpp` sets
`wasmtime_config_cranelift_nan_canonicalization_set(true)` for every module. Ship Game's
`neighbour_cost` example (`cargo build --release -p shipcore --example neighbour_cost --target
wasm32-wasip1`, run with the wasmtime 42.0.1 CLI, the engine's version):

| | Oriented box test, per call | Following check (about 115 neighbours), per call |
|---|---:|---:|
| Native | 12.5 ns | 1.4 µs |
| Wasmtime | 21 ns | 1.8 µs |
| Wasmtime, `-W nan-canonicalization=y` | 67 ns | 3.4 µs |

In the engine the same box test costs 50–60 ns a call (perf samples over call counts), so the
rules module's float-heavy code pays this. Request: weigh what canonicalization buys against
this cost. NaN bits differ between hosts only for NaNs, and matter only where bits are observed
(reinterpretation, state hashing, host calls); if synced determinism needs it, a cheaper form
(canonicalizing where values are stored or leave Wasm rather than after every operation) would
keep most of the speed.

**Nearest-enemy search.** `CGameHelper::GetClosestEnemyUnit` (`QueryUnits`) gathers every
128-elmo quad its circle touches and walks each enemy ally team's units in them, nearest or not,
so its cost follows the circle's area rather than what is near. Ship Game's Lancers search within
8,400 elmos, about 17,000 quads a search; 600 of them searching every frame took a third of the
rules frame. Ship Game now searches once a second per ship and when the enemy is lost; what
remains is still 7% of the rules time. Request: for the closest-unit queries, visit quads in
rings outward from the centre and stop once a ring's nearest edge is farther than the best unit
found. Cost then follows the distance to the nearest enemy.

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

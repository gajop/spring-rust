# Open engine requests from Ship Game

The remaining requests concern weapon target search cost, command lines outside the draw rectangle, float determinism and nearest-enemy search cost. Resolved issue write-ups have been removed; regression fixtures remain with their engine changes.

## 2026-10-16 · Weapon target search is 6–8% of a big battle's simulation

Found profiling Ship Game's simulation (`main`, report `docs/reports/2026-10-16-performance.md`
there): `profile_period.py --period settled --after 60 --seconds 30 --scenario skirmish-ai
--mode headless --modoption sb_start_army=8000 --speed 10`, two AI teams with a few hundred ships
between them. `CGameHelper::GenerateWeaponTargets` took 6.3% of the process's CPU samples in its
own body and 1.5% more in `CQuadField::GetQuads`, nearly all from
`CUnit::SlowUpdateWeapons -> CWeapon::AutoTarget`. Ship Game's weapons reach far (up to 5 550
elmos), so each search covers many quads and candidates, every slow update, for every weapon of
every ship. After its sensor grid was coarsened this is the largest engine cost in the
simulation.

**Request (performance):** cheaper automatic target generation with many units and long
ranges, for example candidates gathered once per ally team and region per slow update and shared
by the weapons that search it, or weapons of one unit sharing one search. Results unchanged.

## 2026-10-16 · Command queue lines respect the draw rectangle

`drawbounds x0 z0 x1 z1` (`330a0a6969`) stops units, features, projectiles and effects outside
the rectangle from being drawn, but the world pass still draws selected units' command queues
(`CommandDrawer::DrawCommands`, `lineDrawer`) for units outside it, and lines from units inside
it out across its edge. On Ship Game's wormhole maps the player sees one system at a time; a
tilted camera at the system's edge shows the order lines of selected ships in another system,
past the empty gap, and a move from one system to another draws a line across the gap.

To see it: Ship Game `main`, `just play wormhole-arenas --modoption sb_wormhole_charted=1`.
Select ships in Home, switch to Frontier (its tab), right-click there: the ships are sent by the
wormhole. Tilt the camera toward Home at Frontier's near edge: the queue lines of the selected
ships in Home show, and the line from them reaches across the gap.

**Request:** while a draw rectangle is set, draw no command queue for a unit outside it, and clip
every queue line segment (and skip its icons) to the rectangle, in the world pass. The minimap
keeps its own rectangle (`minimap view`) as now. Ship Game already keeps its own selection
markers, routes and rally lines to the shown system.

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

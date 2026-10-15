# Open engine requests from Ship Game

The remaining requests concern float determinism and nearest-enemy search cost. Resolved issue write-ups have been removed; regression fixtures remain with their engine changes.

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

# Null-map visibility queries

`blank_map_terrain=null` opts into bounded visibility-query planning and automatic
object-grid storage. Ordinary terrain maps retain their original grid traversal,
ordered row clipping, per-object filters and dense storage.

Two independent runtime settings control null maps:

| Setting | Values | When applied |
| --- | --- | --- |
| `SparseMapVisibilityQuery` | `0` automatic, `1` force grid, `2` force active-object scan | Each query |
| `SparseMapQuadField` | `0` automatic, `1` force dense, `2` force sparse | Game load |

Automatic storage uses dense cells only when Quad headers and alliance-vector
metadata fit the budget in `rts/Sim/Misc/QuadFieldStorage.h`. Otherwise it uses
sparse cells. Object-vector payloads grow with objects in both representations.
Forcing dense storage bypasses that budget, so it is intended for comparisons on
small maps. Storage choice does not select the query algorithm.

Automatic queries scan small populations directly. For larger populations they
visit the camera's grid rows until the cell or candidate-reference budget is
exhausted, then discard partial lists and scan active objects. Large objects can
appear in several cells, so the reference budget allows duplicates. Both paths
retain camera, LOS, allegiance, icon and noDraw filtering at the original call
sites. The Lua and Native APIs share the selector for visible units/features and
screen rectangles. Empty map-marker drawing is skipped only on null maps.

## Focused checks

```sh
docker-build-v2/build.sh --compile -j 1 linux -t test_VisibleGridQuery test_QuadField engine-legacy
build-amd64-linux/test/test_VisibleGridQuery
SDL_VIDEODRIVER=offscreen python3 test/visible_object_queries/run.py --output /tmp/visible-null-16
SDL_VIDEODRIVER=offscreen python3 test/visible_object_queries/run.py --map-size 64 --output /tmp/visible-null-64
SDL_VIDEODRIVER=offscreen python3 test/visible_object_queries/run.py --map-size 1024 --output /tmp/visible-null-1024
SDL_VIDEODRIVER=offscreen python3 test/visible_object_queries/run.py --storage sparse --output /tmp/visible-forced-sparse
SDL_VIDEODRIVER=offscreen python3 test/visible_object_queries/run.py --terrain blank --output /tmp/visible-ordinary
```

The rendered fixture creates a small fixed population, pauses simulation,
suppresses unit meshes and compares sorted IDs across all query modes in the
same frame. It checks unit/feature visibility and screen rectangles in close and
wide views, requires nonempty results, and checks that an empty groundFX atlas
does not emit an allocation error. Sensor grids are coarse for the large-map
case. This is a correctness check, not a fleet/FPS benchmark. An interrupted or
timed-out run terminates its own engine. On systems without SDL offscreen GL,
use `xvfb-run -a` instead.

The engine-free regression checks row order, angled clipping, padding, empty
frusta, duplicate references, moderate views, huge-map work limits, and storage
budget/override behavior. A separate, explicitly selected container diagnostic
compares actual dense and sparse QuadStore reads without a game or renderer:

```sh
build-amd64-linux/test/test_QuadField '[.storage-diagnostic]'
```

It prints structural storage and read timings; allocator overhead and per-object
vector capacity are excluded from the structural figures. No timing is a test
gate or a gameplay FPS prediction. Cell occupancy, alliance count and camera
coverage matter more than a single unit-count threshold.

## Manual Ship Game check

Use the rebuilt rendered engine. Start a null-map match, pan/zoom between a
fleet close-up and the map view, and compare selectable ships and remembered
asteroids with the minimap. Changing `SparseMapVisibilityQuery` through the
unsynced config API should change query work without changing visible IDs. An
ordinary terrain-map match should keep the same visibility and picking behavior
for all settings. Check the Visible/Out of sight/Unexplored legend and move a
sensor away from previously explored space to verify retained discoveries.

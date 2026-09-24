# POSTPONED: Open engine work: LOS sensor maintenance and a typed RmlUi layer

What is left of the 2026-09-22/23 review handoffs; everything else from them is done.

## 1. LOS sensor maintenance cost

From a 500 v 500 Mantis duel capture (Ship Game, 64x64 map, far view): LOS took
~165 ms of a ~219 ms sim step. CPU samples: `PrepareRaycast` 40%, `AddCircle` 25%,
`AddRaycast` 9% (overlapping all-thread shares).

Code:
- `rts/Sim/Misc/LosHandler.cpp`: unchanged-instance test (quantized position,
  bucketed height, radius, ally team) and instance cache (~line 167); remove
  coverage, parallel `PrepareRaycast`, add coverage (~499); per-sensor-type
  dispatch through the thread pool (~822).
- `rts/Sim/Misc/LosMap.cpp`: `AddCircle` writes every cell of the disk on both
  removal and addition (~412); `AddRaycast` (~430); `PrepareRaycast` (~471).

Suggested order:
1. Low-volume per-sensor-type counters/zones: reused/new/recalculated instances,
   touched cells, remove/recalc/add time, queue size.
2. Exact coverage deltas for translated equal-radius disks (terrain-free sensor
   types), keeping reference counts and overlap exact.
3. Raycast reuse/batching for terrain-sensitive LOS with correct invalidation.
   Circular deltas cannot replace terrain-sensitive LOS.

Constraints: synced and deterministic; parallel coverage writes need a race-free,
deterministic design. Do not disable LOS, shrink ranges, change mip levels or
delay updates as an "optimization". A terrain-free sensor capability for space
games is possible but needs declared semantics. Validate LOS-map equality
(edges, ally changes, deaths, terrain changes) against the current code, and
measure on a quiet host.

## 2. Typed RmlUi layer in the Rust SDK

Five SpringCabal Wasm ports carry 100-150 lines of RmlUi setup each; the
friction:
- polling `spring::is_ready(0)` every Update until RmlUi is up;
- three error channels per call (`Result`, a `success` flag, `handle == 0`);
- raw `u64` handles for contexts, documents, elements and data models, with no
  ownership/`Drop`, re-validated periodically because they go stale on reload;
- manual `context_set_dimensions` every render;
- element lookup by id every frame;
- content pushed as `set_inner_rml(format!(..))` instead of data bindings.

Proposed: `Context`, `Document`, `DataModel` and `Element` types (in `spring` or
`spring-addons`) with `Drop`, one `Result`, a context that follows the view size
by default, `Document::load_when_ready(path)` or an addon-level "RmlUi ready"
callin, and data-model helpers that make bindings the easy path.

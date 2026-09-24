# Ship Game unit model bounds match a missile instead of the ship

Observed on 2026-09-24 with the normal Linux engine install (`Spring Engine Version: 2026.07.01-279-g765edd6 rust-wip` in the game log). This is an engine investigation, not a proposed fix yet. No engine code was changed for this report.

## Reproduction and measurements

In Ship Game, start the `fleet-testudo` studio scene and query a `gen_shield_ship` definition and live unit after placement:

| Source | Value |
| --- | --- |
| `Spring.GetUnitRadius(unit)` / Core Wasm `units_info::get_unit_radius` | `11.195` |
| `Spring.GetUnitDefDimensions(def)` / Core Wasm `utils::get_unit_def_dimensions` | height `6.3`, radius `11.195`, X `[-3.15, 3.15]`, Y `[-3.15, 3.15]`, Z `[-8, 12.54]` |
| Core Wasm `unit_defs::get_unit_def_by_id(def).physics` | collision height `215`, bounding radius `168.34117` |
| Ship Game `units/gen_shield_ship.lua` | `objectname = "gen_shield_ship.glb"`; collision box from the ship's measured hull |
| `objects3d/gen_shield_ship.glb` POSITION accessor bounds | dimensions approximately `202.85 × 161.05 × 214.73` in glTF axes |
| `objects3d/gen_shield_ship_missile.glb` POSITION accessor bounds | dimensions `6.3 × 20.54 × 6.3` in glTF axes |

The reported `GetUnitDefDimensions` bounds equal the **missile** model's bounds after glTF-to-Spring axis rotation: its glTF Y length `20.54` becomes Spring Z length `20.54`, and its X/Z lengths become Spring X/Y lengths `6.3`. The resulting half-diagonal is `11.195`. The ship GLB itself has the expected large geometry and no node scale that could explain a 15× shrink. Multiple launches returned the same model radius.

This is not merely two APIs using different legitimate notions of radius. The collision radius is deliberately based on the ship's large box, while the model-bound API reports the dimensions of a *different file*. The rendered hull is visibly large in-game. Ship Game's strategic icon overlay had used the live-unit radius as projected hull size, so it placed icons directly on top of readable ships. That overlay now caches the definition's collision radius as its size estimate; it is a local visual correction, not a fix for this model metadata discrepancy.

## Engine path to inspect

- `SolidObjectDef::LoadModel()` loads `modelName` through `CModelLoader::LoadModel()`; `CUnit::Init()` then calls `SetRadiusAndHeight(model)`, so a wrong `S3DModel::radius` propagates to `CUnit::radius`.
- `GetUnitDefDimensions` loads the same definition model and returns `S3DModel` bounds. `GetUnitRadius` returns `CUnit::radius`. The Core Wasm bindings forward these values; the same paths back the Lua calls.
- `GetUnitDefByID().physics.radius` instead returns `ud->collisionVolume.GetBoundingRadius()`, which explains why it remains near the intended ship size.
- `CModelLoader::GetCachedModel()` caches both full filenames and extensionless names. The shield and missile names share a prefix, but the current comparator appears to require exact equality; a cache collision is only a hypothesis.

Instrument one `gen_shield_ship` load and spawn to log the definition's `modelName`, the returned `S3DModel::name`, model ID, `mins/maxs`, and the unit's `model->name` and `radius`. Compare those with a direct `modelLoader.LoadModel("gen_shield_ship.glb")` and the missile model. Check whether loading order or concurrent preloading changes the result. A regression test should load similarly prefixed GLB names in both orders and assert that each definition and unit retains its own bounds. Do not change the radius API semantics until the wrong-model/bounds source is identified.

Potential effects beyond the reported icons include draw culling, icon LOD, and any engine code that uses `CUnit::radius`; these have **not** been independently verified. The explicit collision box is a separate value and appears correct in the observed definition.

# Piece collision volumes: current support and Core Wasm defect

Investigation for Ship Game on 2026-09-24. This is a capability audit and engine handover; no collision behavior was changed.

## What the engine supports today

- Unit and feature definitions accept `usePieceCollisionVolumes = true`. `SolidObjectDef::ParseCollisionVolume` then disables the single main volume for hit tests and routes rays through the model's local pieces. `usePieceSelectionVolumes = true` independently routes mouse picking through those same piece volumes.
- Each model piece has one `CollisionVolume`, initialized from that piece's geometry bounds. Supported shapes are box, ellipsoid, cylinder, and sphere; the piece transform rotates/translates its volume. There is **no triangle-accurate rendered-mesh collision path** in `CCollisionHandler::IntersectPieceTree`/`IntersectPiecesHelper`; it intersects the volumes, not mesh triangles.
- The piece path is used by projectile collision (`CProjectileHandler::CheckUnitCollisions`), weapon/ray traces, and (when opted in) mouse selection. A hit can report the impacted piece. It also applies to features, subject to their definition and instance settings.
- Moving-unit separation, especially hover/strafe-air and ground movement, uses `unit->radius` and/or footprint logic. Enabling piece collision volumes does **not** make ship-to-ship pushing follow those volumes.
- `.dae` has **not** been removed from this local engine: `IModelParser.cpp` still whitelists it, and the installed normal engine logged `*.dae` among its enabled Assimp formats. GLB/GLTF uses the native GLTF parser. The piece-volume mechanism is in the collision handler rather than tied to DAE.

For Ship Game, merely setting `usePieceCollisionVolumes` on the current shield ship would be poor: its GLB contains the visible hull plus several LOD hull meshes and doors as separate pieces, so default boxes would overlap or duplicate the whole hull. A useful multi-volume setup needs a deliberate small set of collider pieces/proxies, with unwanted piece volumes disabled. Empty GLTF nodes are represented as pieces, but their custom volume must stay inside a valid broadphase bound. Performance and hit quality would need testing.

## Core Wasm setter defect

The generated Core Wasm API exposes `unit_control::set_unit_piece_collision_volume_data(unit_id, piece_index, enable, scales, offsets, volume_type, primary_axis)`. Its native implementation in `rts/NativeInterface/api/SyncedCtrl.cpp` only enters the body when `enable` is true. It initializes a volume, toggles `scriptSetVisible` twice (a net no-op), and **never calls `SetIgnoreHits(!enable)`**. Therefore `enable = false` cannot disable a piece, and `enable = true` does not re-enable a previously ignored piece. This differs from `LuaSyncedCtrl::SetSolidObjectPieceCollisionVolumeData` and the native feature-piece setter, both of which set `IgnoreHits` from `enable`.

Fix the native unit-piece setter to mirror the Lua/feature semantics, without changing visual piece visibility. Add a test that initializes a volume, disables it, verifies an otherwise intersecting ray misses, re-enables it, and verifies the ray hits. The public Core Wasm return value should report the actual success. No game-side workaround should be needed.

## Prerequisite for Ship Game

The shield ship currently reports `CUnit::radius = 11.195` and model dimensions equal to its *missile* GLB, while its collision-box bounding radius is about `168.34`. See `TODO_SHIP_GAME_MODEL_RADIUS_MISMATCH.md`. `CQuadField::MovedUnit` indexes a unit using `unit->radius`, so the mismatch can prevent a ray/projectile near the hull edge from even reaching the detailed hit test. Resolve and verify the model/radius issue before evaluating piece collision for this ship.

Suggested follow-up is a small, isolated game scenario with one large ship and shots aimed at its visible hull, gaps, and empty space. Compare single-box and curated piece-volume hits and profile the broadphase/narrowphase costs. Keep selection and physical ship separation as separately evaluated concerns.

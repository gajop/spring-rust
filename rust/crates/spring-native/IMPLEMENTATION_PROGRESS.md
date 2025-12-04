# Native Interface API Implementation Progress

## Batch 1: Simple Unit Setters (15 functions)

**Status:** Completed (10/10 synced functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement simple unit setter functions in SyncedCtrl

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetUnitCloak | ✅ Done | LuaSyncedCtrl.cpp:2944 | Sets unit->wantCloak (bool) and unit->decloakDistance (float, optional) |
| 2 | SetUnitStealth | ✅ Done | LuaSyncedCtrl.cpp:2980 | Sets unit->stealth (bool) |
| 3 | SetUnitSonarStealth | ✅ Done | LuaSyncedCtrl.cpp:2998 | Sets unit->sonarStealth (bool) |
| 4 | SetUnitSeismicSignature | ✅ Done | LuaSyncedCtrl.cpp:3015 | Sets unit->seismicSignature (float) |
| 5 | SetUnitArmored | ✅ Done | LuaSyncedCtrl.cpp:2765 | Sets unit->armoredState, armoredMultiple, curArmorMultiple |
| 6 | SetUnitBlocking | ✅ Done | LuaSyncedCtrl.cpp:3277 | Sets blocking state and collidable bits (7 bool params) |
| 7 | SetUnitMass | ✅ Done | LuaSyncedCtrl.cpp:4040 | Sets unit mass via SetMass() |
| 8 | SetUnitNoDraw | ⏭️ Skipped | LuaUnsyncedCtrl.cpp:2169 | **UNSYNCED** - requires UnsyncedCtrl API |
| 9 | SetUnitNoSelect | ⏭️ Skipped | LuaUnsyncedCtrl.cpp:2296 | **UNSYNCED** - requires UnsyncedCtrl API |
| 10 | SetUnitNoMinimap | ⏭️ Skipped | LuaUnsyncedCtrl.cpp:2226 | **UNSYNCED** - requires UnsyncedCtrl API |
| 11 | SetUnitNoGroup | ⏭️ Skipped | LuaUnsyncedCtrl.cpp:2273 | **UNSYNCED** - requires UnsyncedCtrl API |
| 12 | SetUnitLeavesGhost | ✅ Done | LuaSyncedCtrl.cpp:3033 | Calls unit->SetLeavesGhost(leavesGhost, leaveDeadGhost) |
| 13 | SetUnitLeaveTracks | ⏭️ Skipped | LuaUnsyncedCtrl.cpp:2324 | **UNSYNCED** - requires UnsyncedCtrl API |
| 14 | SetUnitAlwaysVisible | ✅ Done | LuaSyncedCtrl.cpp:3053 | Sets unit->alwaysVisible (bool) |
| 15 | SetUnitUseAirLos | ✅ Done | LuaSyncedCtrl.cpp:3066 | Sets unit->useAirLos (bool) |

**Note:** 5 functions (SetUnitNoDraw, SetUnitNoSelect, SetUnitNoMinimap, SetUnitNoGroup, SetUnitLeaveTracks) are client-side unsynced operations found in LuaUnsyncedCtrl.cpp. These will be implemented in Batch 11 (UI & Input) when we create the UnsyncedCtrl API.

---

## Batch 2: Simple Unit Getters (15 functions)

**Status:** Completed (15/15 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement simple unit getter functions

### Functions:

| # | Function | Status | Lua Location | API Module | Notes |
|---|----------|--------|--------------|------------|-------|
| 1 | GetUnitNoDraw | ✅ Done | LuaUnsyncedRead.cpp:1243 | UnsyncedRead | Returns unit->noDraw (bool) |
| 2 | GetUnitNoSelect | ✅ Done | LuaUnsyncedRead.cpp:1334 | UnsyncedRead | Returns unit->noSelect (bool) |
| 3 | GetUnitNoMinimap | ✅ Done | LuaUnsyncedRead.cpp:1299 | UnsyncedRead | Returns unit->noMinimap (bool) |
| 4 | GetUnitNoGroup | ✅ Done | LuaUnsyncedRead.cpp:1317 | UnsyncedRead | Returns unit->noGroup (bool) |
| 5 | GetUnitLeavesGhost | ✅ Done | LuaSyncedRead.cpp:3928 | SyncedCtrl | Returns unit->leavesGhost (bool) - **SYNCED** |
| 6 | GetUnitDrawFlag | ✅ Done | LuaUnsyncedRead.cpp:1282 | UnsyncedRead | Returns unit->drawFlag (uint8) |
| 7 | GetUnitLuaDraw | ✅ Done | LuaUnsyncedRead.cpp:1232 | UnsyncedRead | Returns unit->luaDraw (bool) |
| 8 | GetUnitEngineDrawMask | ✅ Done | LuaUnsyncedRead.cpp:1254 | UnsyncedRead | Returns unit->engineDrawMask (uint32) |
| 9 | GetUnitAlwaysUpdateMatrix | ✅ Done | LuaUnsyncedRead.cpp:1265 | UnsyncedRead | Returns unit->alwaysUpdateMat (bool) |
| 10 | GetUnitTransformMatrix | ✅ Done | LuaUnsyncedRead.cpp:1672 | UnsyncedRead | Returns 4x4 transform matrix (16 floats) |
| 11 | GetUnitSelectionVolumeData | ✅ Done | LuaUnsyncedRead.cpp:1523 | UnsyncedRead | Returns collision volume data (scales, offsets, type, flags) |
| 12 | GetUnitViewPosition | ✅ Done | LuaUnsyncedRead.cpp:1842 | UnsyncedRead | Returns unit draw position (Float3) |
| 13 | GetUnitPhysicalState | ✅ Done | LuaSyncedCtrl.cpp:3464 | SyncedCtrl | Returns unit->physicalState (uint8) - **SYNCED** |
| 14 | GetUnitIconData | ✅ Done | LuaUnsyncedRead.cpp:1445 | UnsyncedRead | Returns icon data (name, coords, size, distance, etc.) |
| 15 | GetUnitFeatureSeparation | ✅ Done | LuaSyncedRead.cpp:5893 | SyncedCtrl | Returns distance between unit and feature - **SYNCED** |

**New Files Created:**
- `UnsyncedRead.h` - Header for client-side read API (12 functions)
- `UnsyncedRead.cpp` - Implementation of unsynced getters

**Modules Modified:**
- `SyncedCtrl.h/.cpp` - Added 3 synced read functions to UnitControlApi
- `NativeInterface.h` - Added UnsyncedReadApi pointer
- `NativeInterfaceSystem.cpp` - Registered UNSYNCED_READ_API

### Legend:
- 🔍 Researching - Finding Lua implementation
- 📝 Designing - Creating API structures
- 💻 Implementing - Writing C++ code
- ✅ Done - Function complete
- ⏳ Pending - Not started

---

## Research Notes

### SetUnitCloak (LuaSyncedCtrl.cpp:2944-2967)

**Parameters:**
- unitID (int)
- wantCloak (bool or number) - whether unit wants to cloak
- decloakDistance (optional float or bool) - decloak distance OR bool to use default

**Implementation:**
```cpp
unit->wantCloak = <value>;
if (number provided for param 3)
    unit->decloakDistance = <number>;
if (bool true provided for param 3)
    unit->decloakDistance = unit->unitDef->decloakDistance;
```

**Fields accessed:**
- `unit->wantCloak` (bool)
- `unit->decloakDistance` (float)
- `unit->unitDef->decloakDistance` (float)

---

## Batch 3: Terrain Modification (12 functions)

**Status:** Completed (12/12 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement terrain modification functions in SyncedCtrl

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | AddGrass | ✅ Done | LuaSyncedCtrl.cpp:4591 | Adds grass at x,z position with optional density |
| 2 | RemoveGrass | ✅ Done | LuaSyncedCtrl.cpp:4606 | Removes grass at x,z position |
| 3 | AdjustHeightMap | ✅ Done | LuaSyncedCtrl.cpp:6279 | Adds height to rectangle area |
| 4 | LevelHeightMap | ✅ Done | LuaSyncedCtrl.cpp:6239 | Sets absolute height for rectangle area |
| 5 | AddOriginalHeightMap | ✅ Done | LuaSyncedCtrl.cpp:6572 | Adds to original height cache |
| 6 | SetOriginalHeightMap | ✅ Done | LuaSyncedCtrl.cpp:6555 | Sets original height cache |
| 7 | RevertOriginalHeightMap | ✅ Done | LuaSyncedCtrl.cpp:6607 | Reverts to original height with factor |
| 8 | AdjustOriginalHeightMap | ✅ Done | LuaSyncedCtrl.cpp:6572 | Adds height to original cache |
| 9 | LevelOriginalHeightMap | ✅ Done | LuaSyncedCtrl.cpp:6539 | Sets original height cache level |
| 10 | AdjustSmoothMesh | ✅ Done | LuaSyncedCtrl.cpp:6656 | Adds to smooth mesh height |
| 11 | LevelSmoothMesh | ✅ Done | LuaSyncedCtrl.cpp:6639 | Sets smooth mesh height |
| 12 | RebuildSmoothMesh | ✅ Done | LuaSyncedCtrl.cpp:6671 | Rebuilds smooth mesh from height map |

**API Changes:**
- Added 12 new Query/Result structs to `SyncedCtrl.h`
- Added 12 new function implementations to `SyncedCtrl.cpp`
- Updated `TerrainControlApi` struct with new function pointers

---

## Batch 4: Projectile Advanced Control (12 functions)

**Status:** Completed (12/12 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement advanced projectile control functions in SyncedCtrl

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetProjectileDamages | ✅ Done | LuaSyncedCtrl.cpp:7486 | Sets damage array values by key |
| 2 | SetProjectileTimeToLive | ✅ Done | LuaSyncedCtrl.cpp:7599 | Sets projectile TTL |
| 3 | SetProjectileIsIntercepted | ✅ Done | LuaSyncedCtrl.cpp:7541 | Marks projectile as intercepted |
| 4 | SetProjectileCollision | ✅ Done | LuaSyncedCtrl.cpp:7556 | Enables/disables collision checking |
| 5 | SetProjectileCEG | ✅ Done | LuaSyncedCtrl.cpp:7614 | Sets custom explosion generator tag |
| 6 | SetProjectileAlwaysVisible | ✅ Done | LuaSyncedCtrl.cpp:7634 | Sets always visible flag |
| 7 | SetProjectileUseAirLos | ✅ Done | LuaSyncedCtrl.cpp:7647 | Sets air LOS usage |
| 8 | SetProjectileMoveControl | ✅ Done | LuaSyncedCtrl.cpp:7676 | Enables/disables move control |
| 9 | SetProjectileIgnoreTrackingError | ✅ Done | LuaSyncedCtrl.cpp:7662 | Sets ignore tracking error |
| 10 | SetProjectileSpinAngle | ✅ Done | LuaSyncedCtrl.cpp:7703 | Sets spin angle |
| 11 | SetProjectileSpinSpeed | ✅ Done | LuaSyncedCtrl.cpp:7716 | Sets spin speed |
| 12 | SetProjectileSpinVec | ✅ Done | LuaSyncedCtrl.cpp:7729 | Sets spin vector |

**API Changes:**
- Added 12 new Query/Result structs to `SyncedCtrl.h`
- Added 12 new function implementations to `SyncedCtrl.cpp`
- Updated `ProjectileControlApi` struct with new function pointers

---

## Batch 5: Command & Order System + Unit Advanced Control (8 functions)

**Status:** Completed (8/8 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement command description and advanced unit control functions in SyncedCtrl

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | EditUnitCmdDesc | ✅ Done | LuaSyncedCtrl.cpp:7892 | Updates command description at index |
| 2 | InsertUnitCmdDesc | ✅ Done | LuaSyncedCtrl.cpp:7933 | Inserts command description at index (-1 for append) |
| 3 | RemoveUnitCmdDesc | ✅ Done | LuaSyncedCtrl.cpp:7972 | Removes command description at index (-1 for last) |
| 4 | SetUnitCosts | ✅ Done | LuaSyncedCtrl.cpp:2019 | Sets buildTime, metalCost, energyCost |
| 5 | SetUnitBuildSpeed | ✅ Done | LuaSyncedCtrl.cpp:3164 | Sets build/repair/reclaim/resurrect/capture/terraform speeds |
| 6 | SetUnitCollisionVolumeData | ✅ Done | LuaSyncedCtrl.cpp:3095 | Sets collision volume shape (scales, offsets, type, axis) |
| 7 | SetUnitSelectionVolumeData | ✅ Done | LuaSyncedCtrl.cpp:3135 | Sets selection volume shape |
| 8 | SetUnitPieceCollisionVolumeData | ✅ Done | LuaSyncedCtrl.cpp:3113 | Sets piece collision volume |

**New Structures Added:**
- `NativeCommandDescription` - Command description data (id, type, flags, strings, params)
- `SetUnitCostsQuery/Result` - Unit cost modification
- `SetUnitBuildSpeedQuery/Result` - Builder/factory speeds
- `SetUnitCollisionVolumeDataQuery/Result` - Collision volume
- `SetUnitSelectionVolumeDataQuery/Result` - Selection volume
- `SetUnitPieceCollisionVolumeDataQuery/Result` - Piece collision volume

**API Changes:**
- Added 8 new Query/Result structs to `SyncedCtrl.h`
- Added helper `ApplyNativeCommandDescription()` function
- Added 8 new function implementations to `SyncedCtrl.cpp`
- Added new includes: `CommandDescription.h`, `Factory.h`, `Builder.h`, `3DModel.h`
- Updated `UnitControlApi` struct with new function pointers

---

## Batch 6: Feature Advanced Control (6 functions)

**Status:** Completed (6/6 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement additional feature control functions in SyncedCtrl

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetFeatureUseAirLos | ✅ Done | LuaSyncedCtrl.cpp:4780 | Sets feature->useAirLos (bool) |
| 2 | SetFeatureNoSelect | ✅ Done | LuaSyncedCtrl.cpp:5154 | Sets feature->noSelect (bool) |
| 3 | SetFeatureMidAndAimPos | ✅ Done | LuaSyncedCtrl.cpp:5181 | Sets mid and aim positions |
| 4 | SetFeatureRadiusAndHeight | ✅ Done | LuaSyncedCtrl.cpp:5224 | Sets feature radius and height |
| 5 | SetFeatureCollisionVolumeData | ✅ Done | LuaSyncedCtrl.cpp:5269 | Sets collision volume shape |
| 6 | SetFeatureSelectionVolumeData | ✅ Done | (similar to unit) | Sets selection volume shape |

**API Changes:**
- Added 6 new Query/Result structs to `SyncedCtrl.h`
- Added 6 new function implementations to `SyncedCtrl.cpp`
- Updated `FeatureControlApi` struct with new function pointers

---

*Last updated: 2025-01-17*

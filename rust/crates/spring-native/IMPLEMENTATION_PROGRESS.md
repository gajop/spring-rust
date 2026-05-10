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

## Batch 7: Unit Combat Control (6 functions)

**Status:** Completed (6/6 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement unit targeting, shield, and combat-related functions in SyncedCtrl

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetUnitTarget | ✅ Done | LuaSyncedCtrl.cpp:3524 | Sets unit attack target (unit or ground position) |
| 2 | SetUnitShieldState | ✅ Done | LuaSyncedCtrl.cpp:3326 | Sets shield enabled state and power |
| 3 | SetUnitShieldRechargeDelay | ✅ Done | LuaSyncedCtrl.cpp:3363 | Sets shield recharge delay in seconds |
| 4 | SetUnitFlanking | ✅ Done | LuaSyncedCtrl.cpp:3402 | Sets flanking bonus mode, direction, damage multipliers |
| 5 | SetUnitMidAndAimPos | ✅ Done | LuaSyncedCtrl.cpp:3594 | Sets unit mid and aim positions |
| 6 | SetUnitRadiusAndHeight | ✅ Done | LuaSyncedCtrl.cpp:3655 | Sets unit radius and height |

**API Changes:**
- Added 6 new Query/Result structs to `SyncedCtrl.h`
- Added 6 new function implementations to `SyncedCtrl.cpp`
- Added new includes: `Weapon.h`, `PlasmaRepulser.h`
- Updated `UnitControlApi` struct with new function pointers

---

## Batch 8: Effects Control API (3 functions)

**Status:** Completed (3/3 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement effects spawning functions - new EffectsControlApi

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SpawnExplosion | ✅ Done | LuaSyncedCtrl.cpp:7560 | Creates explosion with damage, crater, effects |
| 2 | SpawnCEG | ✅ Done | LuaSyncedCtrl.cpp:7635 | Spawns Custom Explosion Generator by name or ID |
| 3 | SpawnSFX | ✅ Done | LuaSyncedCtrl.cpp:7678 | Spawns special effects on units |

**New API Module:**
- Created `EffectsControlApi` struct with 3 function pointers
- Added `SpawnExplosionQuery/Result` - full explosion parameters
- Added `SpawnCEGQuery/Result` - CEG by name or ID
- Added `SpawnSFXQuery/Result` - unit-based SFX spawning
- Added `ExplosionGenerator.h` include
- Updated `SyncedCtrlApi` to include effects API

---

## Batch 9: Unit Movement Control (5 functions)

**Status:** Completed (5/5 functions)
**Started:** 2025-01-17
**Completed:** 2025-01-17
**Target:** Implement unit movement and stockpile control functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetUnitMoveGoal | ✅ Done | LuaSyncedCtrl.cpp:3940 | Sets unit movement goal with optional raw mode |
| 2 | SetUnitLandGoal | ✅ Done | LuaSyncedCtrl.cpp:3975 | Sets landing goal for air units |
| 3 | ClearUnitGoal | ✅ Done | LuaSyncedCtrl.cpp:4000 | Stops unit movement |
| 4 | SetUnitStockpile | ✅ Done | LuaSyncedCtrl.cpp:2350 | Sets stockpile count and build percent |
| 5 | SetUnitDirection | ✅ Done | LuaSyncedCtrl.cpp:4146 | Sets unit facing direction |

**API Changes:**
- Added 5 new Query/Result structs to `SyncedCtrl.h`
- Added 5 new function implementations to `SyncedCtrl.cpp`
- Added new includes: `MoveType.h`, `AAirMoveType.h`
- Updated `UnitControlApi` struct with new function pointers

---

## Batch 10: Unit Attachment & Transport (5 functions)

**Status:** Completed (5/5 functions)
**Started:** 2025-12-05
**Completed:** 2025-12-05
**Target:** Implement unit transport and attachment functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | UnitAttach | ✅ Done | LuaSyncedCtrl.cpp:7242 | Attaches transportee to transporter at piece |
| 2 | UnitDetach | ✅ Done | LuaSyncedCtrl.cpp:7278 | Detaches unit from transporter |
| 3 | UnitDetachFromAir | ✅ Done | LuaSyncedCtrl.cpp:7300 | Detaches unit and drops at position |
| 4 | SetUnitLoadingTransport | ✅ Done | LuaSyncedCtrl.cpp:7334 | Disables collision during transport approach |
| 5 | SetUnitCrashing | ✅ Done | LuaSyncedCtrl.cpp:3289 | Sets aircraft crashing/flying state |

**API Changes:**
- Added 5 new Query/Result structs to `SyncedCtrl.h`
- Added 5 new function implementations to `SyncedCtrl.cpp`
- Updated `UnitControlApi` struct with new function pointers

---

## Batch 11: Unit Weapon Control (4 functions)

**Status:** Completed (4/4 functions)
**Started:** 2025-12-05
**Completed:** 2025-12-05
**Target:** Implement weapon state and firing control functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetUnitWeaponState | ✅ Done | LuaSyncedCtrl.cpp:2520 | Sets weapon state by key (reload, range, burst, etc.) |
| 2 | UnitWeaponFire | ✅ Done | LuaSyncedCtrl.cpp:7169 | Forces weapon to fire |
| 3 | UnitWeaponHoldFire | ✅ Done | LuaSyncedCtrl.cpp:7192 | Drops current target |
| 4 | SetUnitUseWeapons | ✅ Done | LuaSyncedCtrl.cpp:2492 | Sets forceUseWeapons and allowUseWeapons |

**Weapon State Keys Supported:**
- reloadState/reloadFrame - reload status
- reloadTime - reload time in seconds
- reaimTime - reacquire target time
- accuracy - accuracy error
- sprayAngle - spray angle
- range - weapon range
- projectileSpeed - projectile speed
- autoTargetRangeBoost - auto-target range boost
- burst - salvo size
- burstRate - salvo delay in seconds
- windup - salvo windup time
- projectiles - projectiles per shot
- salvoLeft - remaining salvos
- nextSalvo - next salvo frame
- aimReady - aim ready flag
- forceAim - force aim reacquisition
- avoidFlags - avoid flags
- collisionFlags - collision flags
- ttl - time to live in seconds

**API Changes:**
- Added 4 new Query/Result structs to `SyncedCtrl.h`
- Added helper `SetSingleWeaponState()` function for weapon state keys
- Added 4 new function implementations to `SyncedCtrl.cpp`
- Added new includes: `GlobalConstants.h`, `StringHash.h`
- Updated `UnitControlApi` struct with new function pointers

---

## Batch 12: Unit Advanced Control (14 functions)

**Status:** Completed (14/14 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Implement advanced unit control functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetUnitMaxRange | ✅ Done | LuaSyncedCtrl.cpp:2706 | Sets unit max attack range |
| 2 | SetUnitPhysicalStateBit | ✅ Done | LuaSyncedCtrl.cpp:3447 | Sets physical state bitmask |
| 3 | SetUnitPosErrorParams | ✅ Done | LuaSyncedCtrl.cpp:3894 | Sets radar wobble parameters |
| 4 | SetUnitWeaponDamages | ✅ Done | LuaSyncedCtrl.cpp:2655 | Sets weapon damage by key |
| 5 | ForceUnitCollisionUpdate | ✅ Done | (MoveType) | Forces collision map update |
| 6 | SetUnitHeading | ✅ Done | LuaSyncedCtrl.cpp:4090 | Sets unit heading |
| 7 | SetUnitBuildeeRadius | ✅ Done | LuaSyncedCtrl.cpp:3219 | Sets buildee radius |
| 8 | SetUnitSensorRadius | ✅ Done | LuaSyncedCtrl.cpp:3845 | Sets sensor radii (los, radar, etc.) |
| 9 | SetUnitHarvestStorage | ✅ Done | LuaSyncedCtrl.cpp:3106 | Sets harvested resources |
| 10 | SetUnitBuildParams | ✅ Done | LuaSyncedCtrl.cpp:3128 | Sets build range and 3D mode |
| 11 | SetUnitLosMask | ✅ Done | LuaSyncedCtrl.cpp:2857 | Sets LOS mask (disables engine updates) |
| 12 | SetUnitLosState | ✅ Done | LuaSyncedCtrl.cpp:2904 | Sets LOS state for ally team |
| 13 | SetUnitStorage | ✅ Done | LuaSyncedCtrl.cpp:2200 | Sets metal/energy storage |
| 14 | SetUnitTooltip | ✅ Done | LuaSyncedCtrl.cpp:2233 | Sets unit tooltip |

**Helper Functions Added:**
- `SetSingleDynDamagesKey()` - Sets damage array properties by key name

**API Changes:**
- Added 14 new Query/Result structs to `SyncedCtrl.h`
- Added 14 new function implementations to `SyncedCtrl.cpp`
- Added new includes: `UnitToolTipMap.hpp`
- Updated `UnitControlApi` struct with new function pointers

---

## Batch 13: Team Control Additions (2 functions)

**Status:** Completed (2/2 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add team/player control functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetTeamStartPosition | ✅ Done | LuaSyncedCtrl.cpp:965 | Sets team start position |
| 2 | SetPlayerReadyState | ✅ Done | LuaSyncedCtrl.cpp:996 | Sets player ready state |

**API Changes:**
- Added 2 new Query/Result structs to `SyncedCtrl.h`
- Added 2 new function implementations to `SyncedCtrl.cpp`
- Updated `TeamControlApi` struct with new function pointers

---

## Batch 14: Factory & Resource Functions (5 functions)

**Status:** Completed (5/5 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add factory bugger-off and unit resource functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetFactoryBuggerOff | ✅ Done | LuaSyncedCtrl.cpp:4216 | Sets factory unit-moving-out behavior |
| 2 | BuggerOff | ✅ Done | LuaSyncedCtrl.cpp:4252 | Tells units to move away from position |
| 3 | AddUnitSeismicPing | ✅ Done | LuaSyncedCtrl.cpp:4422 | Creates seismic ping from unit |
| 4 | AddUnitResource | ✅ Done | LuaSyncedCtrl.cpp:4443 | Adds metal/energy to unit |
| 5 | UseUnitResource | ✅ Done | LuaSyncedCtrl.cpp:4480 | Consumes metal/energy from unit |

**API Changes:**
- Added 5 new Query/Result structs to `SyncedCtrl.h`
- Added 5 new function implementations to `SyncedCtrl.cpp`
- Updated `UnitControlApi` struct with new function pointers

---

## Batch 15: Additional Projectile Function (1 function)

**Status:** Completed (1/1 function)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add piece projectile control function

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetPieceProjectileParams | ✅ Done | LuaSyncedCtrl.cpp:5799 | Sets piece projectile spin and explosion flags |

**API Changes:**
- Added 1 new Query/Result struct to `SyncedCtrl.h`
- Added 1 new function implementation to `SyncedCtrl.cpp`
- Added include for `PieceProjectile.h`
- Updated `ProjectileControlApi` struct with new function pointer

---

## Batch 16: Game Config Control (4 functions)

**Status:** Completed (4/4 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add game configuration and settings functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetNoPause | ✅ Done | LuaSyncedCtrl.cpp:7707 | Disables/enables game pausing (server only) |
| 2 | SetExperienceGrade | ✅ Done | LuaSyncedCtrl.cpp:7729 | Controls UnitExperience callin frequency |
| 3 | SetRadarErrorParams | ✅ Done | LuaSyncedCtrl.cpp:7760 | Sets radar error sizes for ally teams |
| 4 | SetSquareBuildingMask | ✅ Done | LuaSyncedCtrl.cpp:7140 | Sets building placement mask for map tiles |

**New API Module:**
- Created `GameConfigApi` struct with 4 function pointers
- Added to `SyncedCtrlApi` as `gameConfig` member

**API Changes:**
- Added 4 new Query/Result structs to `SyncedCtrl.h`
- Added 4 new function implementations to `SyncedCtrl.cpp`
- Added new includes: `BuildingMaskMap.h`, `GameServer.h`
- Updated `SyncedCtrlApi` struct with new gameConfig member

---

## Batch 17: Feature Fire/Smoke & Wreck Creation (4 functions)

**Status:** Completed (4/4 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add feature fire/smoke time setters and wreck creation functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetFeatureFireTime | ✅ Done | LuaSyncedCtrl.cpp:5320 | Sets feature fire duration (seconds) |
| 2 | SetFeatureSmokeTime | ✅ Done | LuaSyncedCtrl.cpp:5355 | Sets feature smoke duration (seconds) |
| 3 | CreateUnitWreck | ✅ Done | LuaSyncedCtrl.cpp:5392 | Creates wreck from unit at wreck level |
| 4 | CreateFeatureWreck | ✅ Done | LuaSyncedCtrl.cpp:5424 | Creates wreck from feature at wreck level |

**API Changes:**
- Added 4 new Query/Result structs to `SyncedCtrl.h`
- Added 4 new function implementations to `SyncedCtrl.cpp`
- Updated `FeatureControlApi` struct with new function pointers

---

## Batch 18: Piece Control Functions (5 functions)

**Status:** Completed (5/5 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add unit and feature piece control functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetUnitPieceVisible | ✅ Done | LuaSyncedCtrl.cpp:3832 | Sets piece script visibility |
| 2 | SetUnitPieceParent | ✅ Done | LuaSyncedCtrl.cpp:3694 | Changes piece hierarchy |
| 3 | SetUnitPieceMatrix | ✅ Done | LuaSyncedCtrl.cpp:3738 | Sets piece local transform matrix |
| 4 | SetFeaturePieceVisible | ✅ Done | (declared in header) | Sets feature piece visibility |
| 5 | SetFeaturePieceCollisionVolumeData | ✅ Done | (declared in header) | Sets feature piece collision volume |

**API Changes:**
- Added 5 new Query/Result structs to `SyncedCtrl.h`
- Added 5 new function implementations to `SyncedCtrl.cpp`
- Updated `UnitControlApi` struct with 3 new function pointers
- Updated `FeatureControlApi` struct with 2 new function pointers

---

## Batch 19: Rules Params (5 functions)

**Status:** Completed (5/5 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add rules param setting functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetGameRulesParam | ✅ Done | LuaSyncedCtrl.cpp:1527 | Sets global game rules param |
| 2 | SetTeamRulesParam | ✅ Done | LuaSyncedCtrl.cpp:1542 | Sets team-specific rules param |
| 3 | SetPlayerRulesParam | ✅ Done | LuaSyncedCtrl.cpp:1560 | Sets player-specific rules param |
| 4 | SetUnitRulesParam | ✅ Done | LuaSyncedCtrl.cpp:1584 | Sets unit-specific rules param |
| 5 | SetFeatureRulesParam | ✅ Done | LuaSyncedCtrl.cpp:1604 | Sets feature-specific rules param |

**API Changes:**
- Added `RulesParamApi` struct with 5 functions
- Added enums for value types and LOS access flags
- Updated `SyncedCtrlApi` to include `rulesParam` member

---

## Batch 20: COB & Misc Functions (4 functions)

**Status:** Completed (4/4 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add COB script and misc unit functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | CallCOBScript | ✅ Done | LuaSyncedCtrl.cpp:1668 | Calls COB script function with args |
| 2 | GetCOBScriptID | ✅ Done | LuaSyncedCtrl.cpp:1727 | Gets function ID from name |
| 3 | SetUnitNanoPieces | ✅ Done | LuaSyncedCtrl.cpp:3216 | Sets nano emitter pieces for builders |
| 4 | TransferTeamMaxUnits | ✅ Done | LuaSyncedCtrl.cpp:1980 | Transfers max unit capacity between teams |

**API Changes:**
- Added `COBScriptApi` struct with 2 functions
- Added `SetUnitNanoPiecesQuery/Result` structs
- Added `TransferTeamMaxUnitsQuery/Result` structs
- Updated `TeamControlApi` with TransferTeamMaxUnits
- Updated `UnitControlApi` with SetUnitNanoPieces
- Updated `SyncedCtrlApi` to include `cobScript` member

---

## Batch 21: Advanced Order Functions (2 functions)

**Status:** Completed (2/2 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add advanced order array functions for bulk commands

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | GiveOrderArrayToUnit | ✅ Done | LuaSyncedCtrl.cpp:6014 | Gives multiple orders to a single unit |
| 2 | GiveOrderArrayToUnitArray | ✅ Done | LuaSyncedCtrl.cpp:6103 | Gives multiple orders to multiple units (with pairwise mode) |

**API Changes:**
- Added `NativeCommand` struct for representing commands in arrays
- Added `GiveOrderArrayToUnitQuery/Result` structs
- Added `GiveOrderArrayToUnitArrayQuery/Result` structs (with pairwise mode flag)
- Updated `UnitControlApi` with 2 new function pointers

**Notes:**
- `GiveOrderToUnitMap` and `GiveOrderArrayToUnitMap` are not implemented as "map" (table with arbitrary keys) doesn't translate well to C API. Use array versions instead.
- `GiveOrderArrayToUnitArray` supports pairwise mode where unit[i] gets command[i] (like Lua version)

---

## Batch 22: Unit Heading & Decal Functions (3 functions)

**Status:** Completed (3/3 functions)
**Started:** 2025-12-06
**Completed:** 2025-12-06
**Target:** Add remaining unit heading and object decal functions

### Functions:

| # | Function | Status | Lua Location | Notes |
|---|----------|--------|--------------|-------|
| 1 | SetUnitHeadingAndUpDir | ✅ Done | LuaSyncedCtrl.cpp:4184 | Sets unit heading and up direction |
| 2 | AddObjectDecal | ✅ Done | LuaSyncedCtrl.cpp:4550 | Adds ground decal for unit |
| 3 | RemoveObjectDecal | ✅ Done | LuaSyncedCtrl.cpp:4567 | Removes ground decal from unit |

**API Changes:**
- Added `SetUnitHeadingAndUpDirQuery/Result` structs
- Added `AddObjectDecalQuery/Result` and `RemoveObjectDecalQuery/Result` structs
- Added include for `IGroundDecalDrawer.h`
- Updated `UnitControlApi` with 3 new function pointers

**Notes:**
- `SetUnitTravel` and `SetUnitFuel` are stub functions (do nothing) - not implemented
- `SetHeightMapFunc`, `SetOriginalHeightMapFunc`, `SetSmoothMeshFunc` require Lua callbacks - not suitable for C API

---

*Last updated: 2025-12-06*

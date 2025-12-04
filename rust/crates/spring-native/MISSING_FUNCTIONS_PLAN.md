# Missing Lua Functions Implementation Plan

## Overview

**Total Missing Functions:** 368 Spring.* callouts
**Current Implementation:** 390/761 functions (51.2%)
**Goal:** Implement all 368 missing functions in Native Interface API

## Implementation Strategy

Work in batches of 10-20 functions, prioritizing by:
1. **Simplicity** (easy wins first)
2. **Frequency of use** (commonly needed functions)
3. **System completeness** (finish logical subsystems)

For each function:
1. Find Lua implementation in `/rts/Lua/Lua*.cpp`
2. Add Query/Result structs to appropriate `/rts/NativeInterface/api/*.h`
3. Implement in corresponding `.cpp` file
4. Rebuild to regenerate Rust bindings
5. Test with SBC or test program

---

## Batch Schedule

### ✅ Batch 1: Simple Unit Setters (15 functions) - CURRENT
**Priority:** HIGH | **Complexity:** LOW | **Estimated:** 2-3 days

Functions:
1. SetUnitCloak
2. SetUnitStealth
3. SetUnitSonarStealth
4. SetUnitSeismicSignature
5. SetUnitArmored
6. SetUnitBlocking
7. SetUnitMass
8. SetUnitNoDraw
9. SetUnitNoSelect
10. SetUnitNoMinimap
11. SetUnitNoGroup
12. SetUnitLeavesGhost
13. SetUnitLeaveTracks
14. SetUnitAlwaysVisible
15. SetUnitUseAirLos

**Module:** SyncedCtrl.h (UnitControlApi extension)
**Rationale:** Direct field assignments, minimal logic, high frequency

---

### Batch 2: Simple Unit Getters (15 functions)
**Priority:** HIGH | **Complexity:** LOW | **Estimated:** 2-3 days

Functions:
1. GetUnitNoDraw
2. GetUnitNoSelect
3. GetUnitNoMinimap
4. GetUnitNoGroup
5. GetUnitLeavesGhost
6. GetUnitDrawFlag
7. GetUnitLuaDraw
8. GetUnitEngineDrawMask
9. GetUnitAlwaysUpdateMatrix
10. GetUnitTransformMatrix
11. GetUnitSelectionVolumeData
12. GetUnitViewPosition
13. GetUnitPhysicalState
14. GetUnitIconData
15. GetUnitFeatureSeparation

**Module:** NEW UnsyncedRead.h
**Rationale:** Complements Batch 1, direct field reads

---

### Batch 3: Feature Control (20 functions)
**Priority:** MEDIUM | **Complexity:** LOW-MEDIUM | **Estimated:** 3-4 days

Functions:
1. AddFeatureDamage
2. SetFeatureBlocking
3. SetFeatureMass
4. SetFeatureMaxHealth
5. SetFeatureReclaim
6. SetFeatureResurrect
7. SetFeaturePhysics
8. SetFeatureMoveCtrl
9. SetFeatureHeadingAndUpDir
10. SetFeatureRotation
11. SetFeatureFade
12. SetFeatureAlwaysVisible
13. SetFeatureNoDraw
14. SetFeatureNoSelect
15. GetFeatureNoDraw
16. GetFeatureLuaDraw
17. GetFeatureDrawFlag
18. GetFeatureEngineDrawMask
19. GetFeatureTransformMatrix
20. GetFeatureSelectionVolumeData

**Modules:** SyncedCtrl.h (FeatureControlApi), UnsyncedRead.h
**Rationale:** Features need similar control to units

---

### Batch 4: Terrain Modification (15 functions)
**Priority:** MEDIUM | **Complexity:** MEDIUM | **Estimated:** 4-5 days

Functions:
1. AddGrass
2. RemoveGrass
3. AddOriginalHeightMap
4. SetOriginalHeightMap
5. RevertOriginalHeightMap
6. AdjustHeightMap
7. AdjustOriginalHeightMap
8. AdjustSmoothMesh
9. LevelHeightMap
10. LevelOriginalHeightMap
11. LevelSmoothMesh
12. RebuildSmoothMesh
13. SetHeightMapFunc
14. SetOriginalHeightMapFunc
15. SetSmoothMeshFunc

**Module:** SyncedCtrl.h (TerrainControlApi extension)
**Rationale:** Complete terrain editing, needed by map editors

---

### Batch 5: Command & Order System (12 functions)
**Priority:** HIGH | **Complexity:** MEDIUM | **Estimated:** 5-6 days

Functions:
1. EditUnitCmdDesc
2. InsertUnitCmdDesc
3. RemoveUnitCmdDesc
4. GetActiveCmdDesc
5. GetActiveCmdDescs
6. GetCmdDescIndex
7. GiveOrder
8. GiveOrderToUnitMap
9. GiveOrderArrayToUnit
10. GiveOrderArrayToUnitArray
11. GiveOrderArrayToUnitMap
12. SetActiveCommand

**Modules:** UnitsCommands.h extension, NEW UnsyncedCtrl.h
**Rationale:** Critical for custom UI and command system

---

### Batch 6: Ground Decals System (30 functions)
**Priority:** LOW | **Complexity:** MEDIUM | **Estimated:** 6-7 days

All GetGroundDecal* and SetGroundDecal* functions:
- CreateGroundDecal, DestroyGroundDecal
- SetGroundDecalPosAndDims, SetGroundDecalQuadPosAndHeight
- SetGroundDecalRotation, SetGroundDecalTexture
- SetGroundDecalTextureParams, SetGroundDecalAlpha
- SetGroundDecalTint, SetGroundDecalNormal
- SetGroundDecalGlowParams, SetGroundDecalMisc
- SetGroundDecalCreationFrame, SetGroundDecalUserData
- GetAllGroundDecals, GetGroundDecalType
- GetGroundDecalOwner, GetGroundDecalTextures
- ... (18 more getter functions)

**Module:** NEW GroundDecals.h (both read and control)
**Rationale:** Complete new subsystem for visual effects

---

### Batch 7: Projectile Advanced Control (11 functions)
**Priority:** MEDIUM | **Complexity:** MEDIUM | **Estimated:** 4-5 days

Functions:
1. SetProjectileDamages
2. SetProjectileTimeToLive
3. SetProjectileIsIntercepted
4. SetProjectileCollision
5. SetProjectileCEG
6. SetProjectileAlwaysVisible
7. SetProjectileUseAirLos
8. SetProjectileMoveControl
9. SetProjectileIgnoreTrackingError
10. SetNanoProjectileParams
11. SetPieceProjectileParams

**Module:** SyncedCtrl.h (ProjectileControlApi extension)
**Rationale:** Advanced projectile control for custom weapons

---

### Batch 8: Camera Control (13 functions)
**Priority:** MEDIUM | **Complexity:** MEDIUM | **Estimated:** 4-5 days

Functions:
- SetDollyCameraMode, SetDollyCameraPosition
- SetDollyCameraCurve, SetDollyCameraLookPosition
- SetDollyCameraLookUnit, SetDollyCameraLookCurve
- SetDollyCameraRelativeMode
- RunDollyCamera, PauseDollyCamera, ResumeDollyCamera
- SetCameraOffset
- GetCameraVectors, GetCameraRotation

**Module:** NEW UnsyncedCtrl.h (CameraControlApi)
**Rationale:** Useful for cutscenes and replays

---

### Batch 9: Lighting System (7 functions)
**Priority:** LOW | **Complexity:** MEDIUM | **Estimated:** 3-4 days

Functions:
1. AddMapLight
2. AddModelLight
3. UpdateMapLight
4. UpdateModelLight
5. AddLightTrackingTarget
6. SetMapLightTrackingState
7. SetModelLightTrackingState

**Module:** NEW Lights.h
**Rationale:** Advanced graphics feature for dynamic lighting

---

### Batch 10: Rendering Control (20 functions)
**Priority:** LOW | **Complexity:** LOW-MEDIUM | **Estimated:** 4-5 days

Functions:
- SetDrawSky, SetDrawWater, SetDrawGround
- SetDrawGroundDeferred, SetDrawModelsDeferred
- SetAtmosphere, SetSunDirection, SetSunLighting
- SetWaterParams, SetSkyBoxTexture
- SetMapRenderingParams, SetMapShader
- SetMapShadingTexture, SetLosViewColors
- SetDrawSelectionInfo, SetVideoCapturingMode
- SetVideoCapturingTimeOffset, ForceLayoutUpdate
- ForceTesselationUpdate, LoadModelTextures

**Module:** NEW UnsyncedCtrl.h (RenderingControlApi)
**Rationale:** Visual customization, lower gameplay impact

---

### Batch 11: UI & Input (20 functions)
**Priority:** MEDIUM | **Complexity:** LOW-MEDIUM | **Estimated:** 5-6 days

Functions:
- SetMouseCursor, AssignMouseCursor, ReplaceMouseCursor
- WarpMouse, SetClipboard, GetClipboard
- SetWMIcon, SetWMCaption
- SetWindowGeometry, SetWindowMaximized, SetWindowMinimized
- SetBoxSelectionByEngine, SetActiveCommand
- SetCustomCommandDrawData
- SDLStartTextInput, SDLStopTextInput, SDLSetTextInputRect
- SetBuildFacing, SetBuildSpacing
- SetAutoShowMetal, SetMiniMapRotation

**Modules:** NEW UnsyncedCtrl.h, Input.h extension
**Rationale:** Important for custom UI

---

### Batch 12: Unit Advanced Control (20 functions)
**Priority:** MEDIUM | **Complexity:** MEDIUM-HIGH | **Estimated:** 6-8 days

Functions:
- SetUnitFlanking, SetUnitMaxRange
- SetUnitWeaponState, SetUnitWeaponDamages
- SetUnitShieldState, SetUnitShieldRechargeDelay
- SetUnitTarget, SetUnitMoveGoal, SetUnitLandGoal
- SetUnitCrashing, SetUnitLoadingTransport
- SetUnitUseWeapons, SetUnitPosErrorParams
- SetUnitPhysicalStateBit
- CallCOBScript, UnitWeaponFire, UnitWeaponHoldFire
- ClearUnitGoal, UnitAttach, UnitDetach

**Module:** SyncedCtrl.h (UnitControlApi)
**Rationale:** Advanced features, complex implementations

---

### Batch 13: Icon & Marker System (15 functions)
**Priority:** LOW | **Complexity:** MEDIUM | **Estimated:** 4-5 days

Functions:
- AddUnitIcon, FreeUnitIcon, SetUnitIconDraw
- AddWorldIcon, AddWorldText, AddWorldUnit
- MarkerAddPoint, MarkerAddLine, MarkerErasePosition
- GetIconData, GetAllIconDataArray
- IsUnitIcon, UnitIconGetDraw
- RemoveObjectDecal

**Modules:** NEW Icons.h, NEW Markers.h
**Rationale:** Visual markers for UI customization

---

### Batch 14: System & Debug (15 functions)
**Priority:** LOW | **Complexity:** LOW-MEDIUM | **Estimated:** 3-4 days

Functions:
- GetGameName, GetMenuName, GetGameState
- GetReplayLength, GetTimer, GetTimerMicros, DiffTimers
- GetLuaMemUsage, GetVidMemUsage, GetSyncedGCInfo
- GetProfilerTimeRecord, GetProfilerRecordNames
- GarbageCollectCtrl, ClearWatchDogTimer
- Start, Restart, Reload, Quit, Yield

**Modules:** NEW SystemControl.h, NEW Profiling.h
**Rationale:** System queries and control

---

### Batch 15: Remaining Misc (~40 functions)
**Priority:** LOW | **Complexity:** VARIES | **Estimated:** 8-10 days

Remaining functions across various categories:
- Chat functions (SendCommands, SendLuaMenuMsg, etc.)
- Sound functions (PreloadSoundItem, etc.)
- VFS functions (CreateDir, ExtractModArchiveFile)
- Selection extensions (SelectUnitMap, DeselectUnitMap)
- Effects (SpawnCEG, SpawnExplosion, SpawnSFX, AddObjectDecal)
- And more...

**Modules:** Various
**Rationale:** Catch-all for remaining functionality

---

## New API Files To Create

1. **UnsyncedCtrl.h** - Client-side control (rendering, UI, camera)
2. **UnsyncedRead.h** - Client-side queries (visible units, rendering state)
3. **GroundDecals.h** - Ground decal management
4. **Lights.h** - Dynamic lighting system
5. **Icons.h** - Icon and world marker management
6. **Markers.h** - Map marker system
7. **SystemControl.h** - Game lifecycle control
8. **Profiling.h** - Performance profiling

---

## Total Timeline Estimate

- **Simple batches (2):** ~6 days
- **Medium batches (7):** ~30 days
- **Complex batches (4):** ~28 days
- **Very complex (1):** ~10 days
- **Total:** ~74 days (~15 weeks) for one developer

---

## Implementation Phases

**Phase 1 (Weeks 1-3): Core Gameplay**
- Batch 1: Simple Unit Setters ← **CURRENT**
- Batch 2: Simple Unit Getters
- Batch 5: Command & Order System

**Phase 2 (Weeks 4-6): Extended Control**
- Batch 3: Feature Control
- Batch 4: Terrain Modification
- Batch 12: Unit Advanced Control

**Phase 3 (Weeks 7-9): Visual Systems**
- Batch 7: Projectile Advanced Control
- Batch 10: Rendering Control
- Batch 8: Camera Control

**Phase 4 (Weeks 10-12): Subsystems**
- Batch 6: Ground Decals System
- Batch 9: Lighting System
- Batch 11: UI & Input

**Phase 5 (Weeks 13-15): Polish & Completion**
- Batch 13: Icon & Marker System
- Batch 14: System & Debug
- Batch 15: Remaining Misc

---

*Plan created: 2025-01-17*
*Last updated: 2025-01-17*

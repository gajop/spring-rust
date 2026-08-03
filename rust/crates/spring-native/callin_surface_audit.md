# Engine Callin Surface Audit

Generated from the Lua documentation/source comments, the C++ native event client,
the native C query structs, and the Rust `NativeModule` trait.
See `api_surface_contract.md` for intentional-difference policy.

## Inventory

| Surface | Count |
| --- | ---: |
| Lua `Callins` | 155 |
| Lua `SyncedCallins` | 28 |
| Lua `UnsyncedCallins` | 6 |
| Lua documented entries | 189 |
| Native C++ callback symbols | 125 |
| Shared callback names | 114 |
| Documented Lua names without native callback | 41 |
| Native callback names without documented Lua callin | 11 |
| Native callbacks without Rust trait method | 2 |

## Lua names without native callback

These are unresolved until individually classified or ported. They are not treated as intentional.

- `AllowBuilderHoldFire`
- `AllowCommand`
- `AllowDirectUnitControl`
- `AllowFeatureBuildStep`
- `AllowFeatureCreation`
- `AllowResourceLevel`
- `AllowResourceTransfer`
- `AllowStartPosition`
- `AllowUnitBuildStep`
- `AllowUnitCaptureStep`
- `AllowUnitCloak`
- `AllowUnitCreation`
- `AllowUnitKamikaze`
- `AllowUnitTransfer`
- `AllowUnitTransport`
- `AllowUnitTransportLoad`
- `AllowUnitTransportUnload`
- `AllowWeaponInterceptTarget`
- `AllowWeaponTarget`
- `AllowWeaponTargetCheck`
- `CommandFallback`
- `DrawFeature`
- `DrawMaterial`
- `DrawProjectile`
- `DrawShield`
- `DrawUnit`
- `FeaturePreDamaged`
- `GotChatMsg`
- `Initialize`
- `LoadCode`
- `MiniMapGeometryChanged`
- `MiniMapRotationChanged`
- `MiniMapStateChanged`
- `MoveCtrlNotify`
- `RecvFromSynced`
- `RecvLuaMsg`
- `RecvSkirmishAIMessage`
- `ResourceExcess`
- `ShieldPreDamaged`
- `TerraformComplete`
- `UnitPreDamaged`

## Native callback names without documented Lua callin

- `CollectGarbage`
- `DrawAlphaFeaturesLua`
- `DrawAlphaUnitsLua`
- `DrawOpaqueFeaturesLua`
- `DrawOpaqueUnitsLua`
- `FeatureMoved`
- `HandleLuaCall`
- `HandleLuaMsg`
- `LastMessagePosition`
- `Pong`
- `UnitMoved`

## Native callbacks without a Rust trait method

- `DrawInMiniMap`
- `DrawInMiniMapBackground`

## Raw signature field-count audit

A differing count is a diagnostic signal, not automatically a bug:
native queries may currently use compact IDs or pointer/count pairs where Lua
receives expanded definition/team/object fields. Every difference still requires
a source-level decision and, where applicable, a behavior test.

| Native callback | Lua params | Native query fields | Query struct | Status |
| --- | ---: | ---: | --- | --- |
| `ActiveCommandChanged` | 2 | 5 | `ActiveCommandChanged` | `field_count_differs` |
| `AddConsoleLine` | 2 | 3 | `AddConsoleLine` | `field_count_differs` |
| `CameraPositionChanged` | 3 | 1 | `Float3Callin` | `field_count_differs` |
| `CameraRotationChanged` | 3 | 1 | `Float3Callin` | `field_count_differs` |
| `CommandNotify` | 3 | 1 | `CommandNotify` | `field_count_differs` |
| `DefaultCommand` | 3 | 3 | `DefaultCommand` | `same_raw_field_count` |
| `DownloadFailed` | 2 | 2 | `DownloadFailed` | `same_raw_field_count` |
| `DownloadFinished` | 1 | 1 | `DownloadFinished` | `same_raw_field_count` |
| `DownloadProgress` | 3 | 3 | `DownloadProgress` | `same_raw_field_count` |
| `DownloadQueued` | 3 | 3 | `DownloadQueued` | `same_raw_field_count` |
| `DownloadStarted` | 1 | 1 | `DownloadStarted` | `same_raw_field_count` |
| `DrawBuildSquare` | 5 | 6 | `DrawBuildSquare` | `field_count_differs` |
| `DrawFeaturesPostDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGenesis` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPostDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPostForward` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPreDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPreForward` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawInMiniMap` | 2 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawInMiniMapBackground` | 2 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawPreDecals` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawScreen` | 2 | 0 | `DrawScreen` | `field_count_differs` |
| `DrawScreenEffects` | 2 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawScreenPost` | 2 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawShadowFeaturesLua` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawShadowPassTransparent` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawShadowUnitsLua` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawUnitsPostDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWaterPost` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorld` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldPreParticles` | 4 | 4 | `DrawWorldPreParticles` | `same_raw_field_count` |
| `DrawWorldPreUnit` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldReflection` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldRefraction` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldShadow` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `Explosion` | 6 | 4 | `Explosion` | `field_count_differs` |
| `FeatureCreated` | 2 | 1 | `FeatureCreated` | `field_count_differs` |
| `FeatureDamaged` | 9 | 9 | `FeatureDamaged` | `same_raw_field_count` |
| `FeatureDestroyed` | 2 | 1 | `FeatureDestroyed` | `field_count_differs` |
| `FontsChanged` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `GameFrame` | 1 | 1 | `GameFrame` | `same_raw_field_count` |
| `GameFramePost` | 1 | 1 | `GameFramePost` | `same_raw_field_count` |
| `GameID` | 1 | 2 | `GameID` | `field_count_differs` |
| `GameOver` | 1 | 2 | `GameOverEvent` | `field_count_differs` |
| `GamePaused` | 2 | 2 | `GamePaused` | `same_raw_field_count` |
| `GamePreload` | 0 | 0 | `GamePreload` | `same_raw_field_count` |
| `GameProgress` | 1 | 1 | `GameProgress` | `same_raw_field_count` |
| `GameSetup` | 3 | 2 | `GameSetup` | `field_count_differs` |
| `GameStart` | 0 | 0 | `GameStart` | `same_raw_field_count` |
| `GetTooltip` | 2 | 2 | `ScreenPosition` | `same_raw_field_count` |
| `GroupChanged` | 1 | 1 | `GroupChanged` | `same_raw_field_count` |
| `IsAbove` | 2 | 2 | `ScreenPosition` | `same_raw_field_count` |
| `KeyMapChanged` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `KeyPress` | 7 | 3 | `KeyPress` | `field_count_differs` |
| `KeyRelease` | 6 | 2 | `KeyRelease` | `field_count_differs` |
| `Load` | 1 | 1 | `ArchiveCallin` | `same_raw_field_count` |
| `MapDrawCmd` | 8 | 8 | `MapDrawCmd` | `same_raw_field_count` |
| `MouseMove` | 5 | 5 | `MouseMove` | `same_raw_field_count` |
| `MousePress` | 3 | 3 | `MousePress` | `same_raw_field_count` |
| `MouseRelease` | 3 | 3 | `MouseRelease` | `same_raw_field_count` |
| `MouseWheel` | 2 | 2 | `MouseWheel` | `same_raw_field_count` |
| `PlayerAdded` | 1 | 1 | `PlayerAdded` | `same_raw_field_count` |
| `PlayerChanged` | 1 | 1 | `PlayerChanged` | `same_raw_field_count` |
| `PlayerRemoved` | 2 | 2 | `PlayerRemoved` | `same_raw_field_count` |
| `ProjectileCreated` | 3 | 1 | `ProjectileEvent` | `field_count_differs` |
| `ProjectileDestroyed` | 3 | 1 | `ProjectileEvent` | `field_count_differs` |
| `RenderUnitDestroyed` | 3 | 1 | `RenderUnitDestroyed` | `field_count_differs` |
| `Save` | 1 | 1 | `ArchiveCallin` | `same_raw_field_count` |
| `Shutdown` | 0 | 0 | `Shutdown` | `same_raw_field_count` |
| `StockpileChanged` | 6 | 6 | `StockpileChanged` | `same_raw_field_count` |
| `SunChanged` | 0 | 1 | `SunChanged` | `field_count_differs` |
| `TeamChanged` | 1 | 1 | `TeamChanged` | `same_raw_field_count` |
| `TeamDied` | 1 | 1 | `TeamDied` | `same_raw_field_count` |
| `TextEditing` | 3 | 3 | `TextEditing` | `same_raw_field_count` |
| `TextInput` | 1 | 1 | `TextInput` | `same_raw_field_count` |
| `UnitArrivedAtGoal` | 3 | 1 | `UnitMoveEvent` | `field_count_differs` |
| `UnitCloaked` | 3 | 1 | `UnitCloakEvent` | `field_count_differs` |
| `UnitCmdDone` | 7 | 4 | `UnitCmdDone` | `field_count_differs` |
| `UnitCommand` | 7 | 7 | `UnitCommand` | `same_raw_field_count` |
| `UnitConstructionDecayed` | 6 | 4 | `UnitConstructionDecayed` | `field_count_differs` |
| `UnitCreated` | 4 | 2 | `UnitCreated` | `field_count_differs` |
| `UnitDamaged` | 10 | 10 | `UnitDamaged` | `same_raw_field_count` |
| `UnitDecloaked` | 3 | 1 | `UnitCloakEvent` | `field_count_differs` |
| `UnitDestroyed` | 7 | 2 | `UnitDestroyed` | `field_count_differs` |
| `UnitEnteredAir` | 3 | 1 | `UnitMovementClassEvent` | `field_count_differs` |
| `UnitEnteredLos` | 4 | 2 | `UnitLosEvent` | `field_count_differs` |
| `UnitEnteredRadar` | 4 | 2 | `UnitLosEvent` | `field_count_differs` |
| `UnitEnteredUnderwater` | 3 | 1 | `UnitMovementClassEvent` | `field_count_differs` |
| `UnitEnteredWater` | 3 | 1 | `UnitMovementClassEvent` | `field_count_differs` |
| `UnitExperience` | 5 | 2 | `UnitExperience` | `field_count_differs` |
| `UnitFeatureCollision` | 2 | 2 | `UnitFeatureCollision` | `same_raw_field_count` |
| `UnitFinished` | 3 | 1 | `UnitFinished` | `field_count_differs` |
| `UnitFromFactory` | 6 | 3 | `UnitFromFactory` | `field_count_differs` |
| `UnitGiven` | 4 | 3 | `UnitGiven` | `field_count_differs` |
| `UnitHarvestStorageFull` | 3 | 1 | `UnitHarvestStorageFull` | `field_count_differs` |
| `UnitIdle` | 3 | 1 | `UnitIdle` | `field_count_differs` |
| `UnitLeftAir` | 3 | 1 | `UnitMovementClassEvent` | `field_count_differs` |
| `UnitLeftLos` | 4 | 2 | `UnitLosEvent` | `field_count_differs` |
| `UnitLeftRadar` | 4 | 2 | `UnitLosEvent` | `field_count_differs` |
| `UnitLeftUnderwater` | 3 | 1 | `UnitMovementClassEvent` | `field_count_differs` |
| `UnitLeftWater` | 3 | 1 | `UnitMovementClassEvent` | `field_count_differs` |
| `UnitLoaded` | 5 | 2 | `UnitLoaded` | `field_count_differs` |
| `UnitMoveFailed` | 3 | 1 | `UnitMoveEvent` | `field_count_differs` |
| `UnitReverseBuilt` | 3 | 1 | `UnitReverseBuilt` | `field_count_differs` |
| `UnitSeismicPing` | 7 | 5 | `UnitSeismicPing` | `field_count_differs` |
| `UnitStunned` | 4 | 2 | `UnitStunned` | `field_count_differs` |
| `UnitTaken` | 4 | 3 | `UnitTaken` | `field_count_differs` |
| `UnitUnitCollision` | 2 | 2 | `UnitUnitCollision` | `same_raw_field_count` |
| `UnitUnloaded` | 5 | 2 | `UnitUnloaded` | `field_count_differs` |
| `UnsyncedHeightMapUpdate` | 0 | 4 | `RectChanged` | `field_count_differs` |
| `Update` | 1 | 0 | `Update` | `field_count_differs` |
| `ViewResize` | 2 | 1 | `ViewResize` | `field_count_differs` |
| `WorldTooltip` | 4 | 4 | `WorldTooltip` | `same_raw_field_count` |

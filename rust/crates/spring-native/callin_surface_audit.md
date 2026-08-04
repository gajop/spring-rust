# Engine Callin Surface Audit

Generated from the Lua documentation/source comments, the C++ native event client,
the native C query structs, and the Rust `NativeModule` trait.
See `api_surface_contract.md` for intentional-difference policy.

## Inventory

| Surface | Count |
| --- | ---: |
| Lua `Callins` | 155 |
| Lua `SyncedCallins` | 29 |
| Lua `UnsyncedCallins` | 6 |
| Lua documented entries | 190 |
| Native C++ callback symbols | 161 |
| Shared callback names | 150 |
| Documented Lua names without native callback | 6 |
| Native callback names without documented Lua callin | 11 |
| Lua-only callins classified by design | 6 |
| Native-only callbacks classified by design | 11 |
| Unclassified Lua-only names | 0 |
| Unclassified native-only names | 0 |
| Native callbacks without Rust trait method | 0 |

## Lua names without native callback

Every entry is classified below. Classification is a design decision, not evidence that its runtime behavior has already been tested.

| Name | Classification | Reason |
| --- | --- | --- |
| `GotChatMsg` | `lua_only_by_design` | Lua-handle chat routing; native modules receive a separate integration stream. |
| `Initialize` | `lua_only_by_design` | Lua-handle lifecycle callback; native modules use InitializeNativeModule. |
| `LoadCode` | `lua_only_by_design` | Lua-handle code-loading lifecycle callback; native modules are loaded through the native ABI. |
| `RecvFromSynced` | `lua_only_by_design` | IPC between the engine's synced and unsynced Lua handles; native modules are not Lua handles. |
| `RecvLuaMsg` | `lua_only_by_design` | Lua-handle message routing; native modules receive the separate HandleLuaMsg hook. |
| `RecvSkirmishAIMessage` | `lua_only_by_design` | Lua-handle skirmish-AI message routing; no native event-client counterpart exists. |

## Native callback names without documented Lua callin

| Name | Classification | Reason |
| --- | --- | --- |
| `CollectGarbage` | `native_only_by_design` | Native event-client garbage-collection scheduling hook; not a script call-in. |
| `DrawAlphaFeaturesLua` | `native_only_by_design` | Native renderer phase hook; it is separate from Lua's DrawFeature call-in. |
| `DrawAlphaUnitsLua` | `native_only_by_design` | Native renderer phase hook; it is separate from Lua's DrawUnit call-in. |
| `DrawOpaqueFeaturesLua` | `native_only_by_design` | Native renderer phase hook; it is separate from Lua's DrawFeature call-in. |
| `DrawOpaqueUnitsLua` | `native_only_by_design` | Native renderer phase hook; it is separate from Lua's DrawUnit call-in. |
| `FeatureMoved` | `native_only_by_design` | Native engine/rendering movement notification; no script call-in is registered. |
| `HandleLuaCall` | `native_only_by_design` | Native-module ingress for Lua-to-native messages; not a Lua call-in. |
| `HandleLuaMsg` | `native_only_by_design` | Native-module ingress for network Lua messages; not a Lua call-in. |
| `LastMessagePosition` | `native_only_by_design` | Native event for message-position consumers; scripts expose Get/Set callouts instead. |
| `Pong` | `native_only_by_design` | Native network timing callback; no script call-in is registered. |
| `UnitMoved` | `native_only_by_design` | Native engine/rendering movement notification; no script call-in is registered. |

## Native callbacks without a Rust trait method

- None

## Raw signature field-count audit

A differing count is a diagnostic signal, not automatically a bug:
native queries may currently use compact IDs or pointer/count pairs where Lua
receives expanded definition/team/object fields. Every difference still requires
a source-level decision and, where applicable, a behavior test.

| Native callback | Lua params | Native query fields | Query struct | Status |
| --- | ---: | ---: | --- | --- |
| `ActiveCommandChanged` | 2 | 5 | `ActiveCommandChanged` | `field_count_differs` |
| `AddConsoleLine` | 2 | 3 | `AddConsoleLine` | `field_count_differs` |
| `AllowBuilderHoldFire` | 3 | 3 | `AllowBuilderHoldFire` | `same_raw_field_count` |
| `AllowCommand` | 9 | 7 | `UnitCommand` | `field_count_differs` |
| `AllowDirectUnitControl` | 4 | 4 | `AllowDirectUnitControl` | `same_raw_field_count` |
| `AllowFeatureBuildStep` | 5 | 5 | `AllowFeatureBuildStep` | `same_raw_field_count` |
| `AllowFeatureCreation` | 5 | 3 | `AllowFeatureCreation` | `field_count_differs` |
| `AllowResourceLevel` | 3 | 3 | `AllowResourceLevel` | `same_raw_field_count` |
| `AllowResourceTransfer` | 4 | 4 | `AllowResourceTransfer` | `same_raw_field_count` |
| `AllowStartPosition` | 9 | 5 | `AllowStartPosition` | `field_count_differs` |
| `AllowUnitBuildStep` | 5 | 5 | `AllowUnitBuildStep` | `same_raw_field_count` |
| `AllowUnitCaptureStep` | 5 | 5 | `AllowUnitBuildStep` | `same_raw_field_count` |
| `AllowUnitCloak` | 2 | 3 | `AllowUnitCloak` | `field_count_differs` |
| `AllowUnitCreation` | 7 | 6 | `AllowUnitCreation` | `field_count_differs` |
| `AllowUnitDecloak` | 3 | 5 | `AllowUnitDecloak` | `field_count_differs` |
| `AllowUnitKamikaze` | 2 | 3 | `AllowUnitKamikaze` | `field_count_differs` |
| `AllowUnitTransfer` | 5 | 5 | `AllowUnitTransfer` | `same_raw_field_count` |
| `AllowUnitTransport` | 6 | 6 | `AllowUnitTransport` | `same_raw_field_count` |
| `AllowUnitTransportLoad` | 9 | 3 | `AllowUnitTransportPosition` | `field_count_differs` |
| `AllowUnitTransportUnload` | 9 | 3 | `AllowUnitTransportPosition` | `field_count_differs` |
| `AllowWeaponInterceptTarget` | 3 | 3 | `AllowWeaponInterceptTarget` | `same_raw_field_count` |
| `AllowWeaponTarget` | 5 | 6 | `AllowWeaponTarget` | `field_count_differs` |
| `AllowWeaponTargetCheck` | 3 | 3 | `AllowWeaponTargetCheck` | `same_raw_field_count` |
| `CameraPositionChanged` | 3 | 1 | `Float3Callin` | `field_count_differs` |
| `CameraRotationChanged` | 3 | 1 | `Float3Callin` | `field_count_differs` |
| `CommandFallback` | 7 | 4 | `CommandFallback` | `field_count_differs` |
| `CommandNotify` | 3 | 1 | `CommandNotify` | `field_count_differs` |
| `DefaultCommand` | 3 | 3 | `DefaultCommand` | `same_raw_field_count` |
| `DownloadFailed` | 2 | 2 | `DownloadFailed` | `same_raw_field_count` |
| `DownloadFinished` | 1 | 1 | `DownloadFinished` | `same_raw_field_count` |
| `DownloadProgress` | 3 | 3 | `DownloadProgress` | `same_raw_field_count` |
| `DownloadQueued` | 3 | 3 | `DownloadQueued` | `same_raw_field_count` |
| `DownloadStarted` | 1 | 1 | `DownloadStarted` | `same_raw_field_count` |
| `DrawBuildSquare` | 5 | 6 | `DrawBuildSquare` | `field_count_differs` |
| `DrawFeature` | 2 | 2 | `DrawFeature` | `same_raw_field_count` |
| `DrawFeaturesPostDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGenesis` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPostDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPostForward` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPreDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawGroundPreForward` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawInMiniMap` | 2 | 2 | `MiniMapDraw` | `same_raw_field_count` |
| `DrawInMiniMapBackground` | 2 | 2 | `MiniMapDraw` | `same_raw_field_count` |
| `DrawMaterial` | 2 | 2 | `DrawMaterial` | `same_raw_field_count` |
| `DrawPreDecals` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawProjectile` | 2 | 2 | `DrawProjectile` | `same_raw_field_count` |
| `DrawScreen` | 2 | 2 | `DrawScreen` | `same_raw_field_count` |
| `DrawScreenEffects` | 2 | 2 | `DrawScreen` | `same_raw_field_count` |
| `DrawScreenPost` | 2 | 2 | `DrawScreen` | `same_raw_field_count` |
| `DrawShadowFeaturesLua` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawShadowPassTransparent` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawShadowUnitsLua` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawShield` | 3 | 3 | `DrawShield` | `same_raw_field_count` |
| `DrawUnit` | 2 | 2 | `DrawUnit` | `same_raw_field_count` |
| `DrawUnitsPostDeferred` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWaterPost` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorld` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldPreParticles` | 4 | 4 | `DrawWorldPreParticles` | `same_raw_field_count` |
| `DrawWorldPreUnit` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldReflection` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldRefraction` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `DrawWorldShadow` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `Explosion` | 6 | 4 | `Explosion` | `field_count_differs` |
| `FeatureCreated` | 2 | 2 | `FeatureCreated` | `same_raw_field_count` |
| `FeatureDamaged` | 9 | 9 | `FeatureDamaged` | `same_raw_field_count` |
| `FeatureDestroyed` | 2 | 2 | `FeatureDestroyed` | `same_raw_field_count` |
| `FeaturePreDamaged` | 9 | 9 | `FeatureDamaged` | `same_raw_field_count` |
| `FontsChanged` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `GameFrame` | 1 | 1 | `GameFrame` | `same_raw_field_count` |
| `GameFramePost` | 1 | 1 | `GameFramePost` | `same_raw_field_count` |
| `GameID` | 1 | 2 | `GameID` | `field_count_differs` |
| `GameOver` | 1 | 2 | `GameOverEvent` | `field_count_differs` |
| `GamePaused` | 2 | 2 | `GamePaused` | `same_raw_field_count` |
| `GamePreload` | 0 | 0 | `GamePreload` | `same_raw_field_count` |
| `GameProgress` | 1 | 1 | `GameProgress` | `same_raw_field_count` |
| `GameSetup` | 3 | 3 | `GameSetup` | `same_raw_field_count` |
| `GameStart` | 0 | 0 | `GameStart` | `same_raw_field_count` |
| `GetTooltip` | 2 | 2 | `ScreenPosition` | `same_raw_field_count` |
| `GroupChanged` | 1 | 1 | `GroupChanged` | `same_raw_field_count` |
| `IsAbove` | 2 | 2 | `ScreenPosition` | `same_raw_field_count` |
| `KeyMapChanged` | 0 | 1 | `SimpleCallin` | `field_count_differs` |
| `KeyPress` | 7 | 10 | `KeyPress` | `field_count_differs` |
| `KeyRelease` | 6 | 9 | `KeyRelease` | `field_count_differs` |
| `Load` | 1 | 1 | `ArchiveCallin` | `same_raw_field_count` |
| `MapDrawCmd` | 8 | 8 | `MapDrawCmd` | `same_raw_field_count` |
| `MiniMapGeometryChanged` | 8 | 8 | `MiniMapGeometryChanged` | `same_raw_field_count` |
| `MiniMapRotationChanged` | 2 | 2 | `MiniMapRotationChanged` | `same_raw_field_count` |
| `MiniMapStateChanged` | 2 | 3 | `MiniMapStateChanged` | `field_count_differs` |
| `MouseMove` | 5 | 5 | `MouseMove` | `same_raw_field_count` |
| `MousePress` | 3 | 3 | `MousePress` | `same_raw_field_count` |
| `MouseRelease` | 3 | 3 | `MouseRelease` | `same_raw_field_count` |
| `MouseWheel` | 2 | 2 | `MouseWheel` | `same_raw_field_count` |
| `MoveCtrlNotify` | 4 | 4 | `MoveCtrlNotify` | `same_raw_field_count` |
| `PlayerAdded` | 1 | 1 | `PlayerAdded` | `same_raw_field_count` |
| `PlayerChanged` | 1 | 1 | `PlayerChanged` | `same_raw_field_count` |
| `PlayerRemoved` | 2 | 2 | `PlayerRemoved` | `same_raw_field_count` |
| `ProjectileCreated` | 3 | 3 | `ProjectileEvent` | `same_raw_field_count` |
| `ProjectileDestroyed` | 3 | 3 | `ProjectileEvent` | `same_raw_field_count` |
| `RenderUnitDestroyed` | 3 | 3 | `RenderUnitDestroyed` | `same_raw_field_count` |
| `ResourceExcess` | 1 | 2 | `ResourceExcess` | `field_count_differs` |
| `Save` | 1 | 1 | `ArchiveCallin` | `same_raw_field_count` |
| `ShieldPreDamaged` | 13 | 9 | `ShieldPreDamaged` | `field_count_differs` |
| `Shutdown` | 0 | 0 | `Shutdown` | `same_raw_field_count` |
| `StockpileChanged` | 6 | 6 | `StockpileChanged` | `same_raw_field_count` |
| `SunChanged` | 0 | 1 | `SunChanged` | `field_count_differs` |
| `TeamChanged` | 1 | 1 | `TeamChanged` | `same_raw_field_count` |
| `TeamDied` | 1 | 1 | `TeamDied` | `same_raw_field_count` |
| `TerraformComplete` | 6 | 6 | `TerraformComplete` | `same_raw_field_count` |
| `TextEditing` | 3 | 3 | `TextEditing` | `same_raw_field_count` |
| `TextInput` | 1 | 1 | `TextInput` | `same_raw_field_count` |
| `UnitArrivedAtGoal` | 3 | 3 | `UnitMoveEvent` | `same_raw_field_count` |
| `UnitCloaked` | 3 | 3 | `UnitCloakEvent` | `same_raw_field_count` |
| `UnitCmdDone` | 7 | 4 | `UnitCmdDone` | `field_count_differs` |
| `UnitCommand` | 7 | 7 | `UnitCommand` | `same_raw_field_count` |
| `UnitConstructionDecayed` | 6 | 6 | `UnitConstructionDecayed` | `same_raw_field_count` |
| `UnitCreated` | 4 | 4 | `UnitCreated` | `same_raw_field_count` |
| `UnitDamaged` | 10 | 10 | `UnitDamaged` | `same_raw_field_count` |
| `UnitDecloaked` | 3 | 3 | `UnitCloakEvent` | `same_raw_field_count` |
| `UnitDestroyed` | 7 | 7 | `UnitDestroyed` | `same_raw_field_count` |
| `UnitEnteredAir` | 3 | 3 | `UnitMovementClassEvent` | `same_raw_field_count` |
| `UnitEnteredLos` | 4 | 4 | `UnitLosEvent` | `same_raw_field_count` |
| `UnitEnteredRadar` | 4 | 4 | `UnitLosEvent` | `same_raw_field_count` |
| `UnitEnteredUnderwater` | 3 | 3 | `UnitMovementClassEvent` | `same_raw_field_count` |
| `UnitEnteredWater` | 3 | 3 | `UnitMovementClassEvent` | `same_raw_field_count` |
| `UnitExperience` | 5 | 5 | `UnitExperience` | `same_raw_field_count` |
| `UnitFeatureCollision` | 2 | 2 | `UnitFeatureCollision` | `same_raw_field_count` |
| `UnitFinished` | 3 | 3 | `UnitFinished` | `same_raw_field_count` |
| `UnitFromFactory` | 6 | 6 | `UnitFromFactory` | `same_raw_field_count` |
| `UnitGiven` | 4 | 4 | `UnitGiven` | `same_raw_field_count` |
| `UnitHarvestStorageFull` | 3 | 3 | `UnitHarvestStorageFull` | `same_raw_field_count` |
| `UnitIdle` | 3 | 3 | `UnitIdle` | `same_raw_field_count` |
| `UnitLeftAir` | 3 | 3 | `UnitMovementClassEvent` | `same_raw_field_count` |
| `UnitLeftLos` | 4 | 4 | `UnitLosEvent` | `same_raw_field_count` |
| `UnitLeftRadar` | 4 | 4 | `UnitLosEvent` | `same_raw_field_count` |
| `UnitLeftUnderwater` | 3 | 3 | `UnitMovementClassEvent` | `same_raw_field_count` |
| `UnitLeftWater` | 3 | 3 | `UnitMovementClassEvent` | `same_raw_field_count` |
| `UnitLoaded` | 5 | 5 | `UnitLoaded` | `same_raw_field_count` |
| `UnitMoveFailed` | 3 | 3 | `UnitMoveEvent` | `same_raw_field_count` |
| `UnitPreDamaged` | 10 | 10 | `UnitDamaged` | `same_raw_field_count` |
| `UnitReverseBuilt` | 3 | 3 | `UnitReverseBuilt` | `same_raw_field_count` |
| `UnitSeismicPing` | 7 | 5 | `UnitSeismicPing` | `field_count_differs` |
| `UnitStunned` | 4 | 4 | `UnitStunned` | `same_raw_field_count` |
| `UnitTaken` | 4 | 4 | `UnitTaken` | `same_raw_field_count` |
| `UnitUnitCollision` | 2 | 2 | `UnitUnitCollision` | `same_raw_field_count` |
| `UnitUnloaded` | 5 | 5 | `UnitUnloaded` | `same_raw_field_count` |
| `UnsyncedHeightMapUpdate` | 0 | 4 | `RectChanged` | `field_count_differs` |
| `Update` | 1 | 1 | `Update` | `same_raw_field_count` |
| `ViewResize` | 2 | 16 | `ViewResize` | `field_count_differs` |
| `WorldTooltip` | 4 | 4 | `WorldTooltip` | `same_raw_field_count` |

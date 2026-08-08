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
| Lua documented entries (namespace rows) | 190 |
| Lua unique documented callin names | 156 |
| Native C++ callback symbols | 161 |
| Shared callback names | 149 |
| Lifecycle-only labels | 1 |
| Documented Lua names without native callback | 6 |
| Native callback names without documented Lua callin | 11 |
| Lua-only callins classified by design | 6 |
| Native-only callbacks classified by design | 11 |
| Unclassified Lua-only names | 0 |
| Unclassified native-only names | 0 |
| Native callbacks without Rust trait method | 0 |

Every entry below is classified explicitly. Classification is a design decision, not evidence that its runtime behavior has already been tested.

## Lifecycle labels with separate Lua/native meanings

| Name | Classification | Reason |
| --- | --- | --- |
| `Shutdown` | `lifecycle_only_by_design` | Lua-handle and native-module lifecycle hooks share a label but have different owners and no event payload. |

## Lua names without native callback

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

## Semantic signature audit

The native query column is an ABI storage shape. `semantically_mapped` means the
representation difference has an explicit source-level explanation; it does not
replace the value-level runtime comparison. `same_arity_pending_runtime_check`
still needs an executable callback test. Any `unresolved_representation_gap` is
an implementation/documentation queue item, not an intentional omission.

| Native callback | Lua params | Native query fields | Query struct | Status | Notes |
| --- | ---: | ---: | --- | --- | --- |
| `ActiveCommandChanged` | 2 | 5 | `ActiveCommandChanged` | `semantically_mapped` | Lua receives cmdID/cmdType; native also carries name/action/tooltip for native consumers. |
| `AddConsoleLine` | 2 | 3 | `AddConsoleLine` | `semantically_mapped` | Lua receives message/level; native section is an engine-side routing field. |
| `AllowBuilderHoldFire` | 3 | 3 | `AllowBuilderHoldFire` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowCommand` | 9 | 7 | `UnitCommand` | `semantically_mapped` | NativeCallinCommand expands to Lua command ID, params, options, tag and timeout; native also carries ABI flags. |
| `AllowDirectUnitControl` | 4 | 4 | `AllowDirectUnitControl` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowFeatureBuildStep` | 5 | 5 | `AllowFeatureBuildStep` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowFeatureCreation` | 5 | 3 | `AllowFeatureCreation` | `semantically_mapped` | Native Float3 position expands to Lua x,y,z. |
| `AllowResourceLevel` | 3 | 3 | `AllowResourceLevel` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowResourceTransfer` | 4 | 4 | `AllowResourceTransfer` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowStartPosition` | 9 | 5 | `AllowStartPosition` | `semantically_mapped` | Native clamped/raw Float3 values expand to Lua coordinate arguments; player/ready fields retain their Lua meaning. |
| `AllowUnitBuildStep` | 5 | 5 | `AllowUnitBuildStep` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowUnitCaptureStep` | 5 | 5 | `AllowUnitBuildStep` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowUnitCloak` | 2 | 3 | `AllowUnitCloak` | `semantically_mapped` | Native hasEnemy/enemyID presence storage; Lua receives enemyID or nil. |
| `AllowUnitCreation` | 7 | 6 | `AllowUnitCreation` | `semantically_mapped` | Native buildPos Float3 and hasBuildInfo expand to Lua x,y,z and optional build information. |
| `AllowUnitDecloak` | 3 | 5 | `AllowUnitDecloak` | `semantically_mapped` | Native hasObject/hasWeapon presence storage expands to Lua optional object/weapon values. |
| `AllowUnitKamikaze` | 2 | 3 | `AllowUnitKamikaze` | `semantically_mapped` | Native allowed is an engine fallback/result input; Lua receives unitID and targetID. |
| `AllowUnitTransfer` | 5 | 5 | `AllowUnitTransfer` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowUnitTransport` | 6 | 6 | `AllowUnitTransport` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowUnitTransportLoad` | 9 | 3 | `AllowUnitTransportPosition` | `semantically_mapped` | Native position Float3 expands to Lua x,y,z while the nested unit record expands to Lua unit fields. |
| `AllowUnitTransportUnload` | 9 | 3 | `AllowUnitTransportPosition` | `semantically_mapped` | Native position Float3 expands to Lua x,y,z while the nested unit record expands to Lua unit fields. |
| `AllowWeaponInterceptTarget` | 3 | 3 | `AllowWeaponInterceptTarget` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `AllowWeaponTarget` | 5 | 6 | `AllowWeaponTarget` | `semantically_mapped` | Native hasTargetPriority/targetPriority is optional-input storage; Lua receives targetPriority or nil semantics. |
| `AllowWeaponTargetCheck` | 3 | 3 | `AllowWeaponTargetCheck` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `CameraPositionChanged` | 3 | 1 | `Float3Callin` | `semantically_mapped` | Native Float3 expands to Lua x,y,z. |
| `CameraRotationChanged` | 3 | 1 | `Float3Callin` | `semantically_mapped` | Native Float3 expands to Lua x,y,z. |
| `CommandFallback` | 7 | 4 | `CommandFallback` | `semantically_mapped` | NativeCallinCommand expands to Lua command params/options; the native query omits Lua-only callback routing fields. |
| `CommandNotify` | 3 | 1 | `CommandNotify` | `semantically_mapped` | NativeCallinCommand expands to Lua command ID, params, options and tag. |
| `DefaultCommand` | 3 | 3 | `DefaultCommand` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DownloadFailed` | 2 | 2 | `DownloadFailed` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DownloadFinished` | 1 | 1 | `DownloadFinished` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DownloadProgress` | 3 | 3 | `DownloadProgress` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DownloadQueued` | 3 | 3 | `DownloadQueued` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DownloadStarted` | 1 | 1 | `DownloadStarted` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawBuildSquare` | 5 | 6 | `DrawBuildSquare` | `semantically_mapped` | Native status pointer/count expands to Lua's status table. |
| `DrawFeature` | 2 | 2 | `DrawFeature` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawFeaturesPostDeferred` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawGenesis` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawGroundDeferred` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawGroundPostDeferred` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawGroundPostForward` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawGroundPreDeferred` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawGroundPreForward` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawInMiniMap` | 2 | 2 | `MiniMapDraw` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawInMiniMapBackground` | 2 | 2 | `MiniMapDraw` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawMaterial` | 2 | 2 | `DrawMaterial` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawPreDecals` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawProjectile` | 2 | 2 | `DrawProjectile` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawScreen` | 2 | 2 | `DrawScreen` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawScreenEffects` | 2 | 2 | `DrawScreen` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawScreenPost` | 2 | 2 | `DrawScreen` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawShadowFeaturesLua` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawShadowPassTransparent` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawShadowUnitsLua` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawShield` | 3 | 3 | `DrawShield` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawUnit` | 2 | 2 | `DrawUnit` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawUnitsPostDeferred` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawWaterPost` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawWorld` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawWorldPreParticles` | 4 | 4 | `DrawWorldPreParticles` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `DrawWorldPreUnit` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawWorldReflection` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawWorldRefraction` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `DrawWorldShadow` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `Explosion` | 6 | 4 | `Explosion` | `semantically_mapped` | Native position Float3 expands to Lua x,y,z; optional owner is represented by a presence sentinel in the C query. |
| `FeatureCreated` | 2 | 2 | `FeatureCreated` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `FeatureDamaged` | 9 | 9 | `FeatureDamaged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `FeatureDestroyed` | 2 | 2 | `FeatureDestroyed` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `FeaturePreDamaged` | 9 | 9 | `FeatureDamaged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `FontsChanged` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `GameFrame` | 1 | 1 | `GameFrame` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GameFramePost` | 1 | 1 | `GameFramePost` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GameID` | 1 | 2 | `GameID` | `semantically_mapped` | Native byte pointer/count expands to Lua's game ID string. |
| `GameOver` | 1 | 2 | `GameOverEvent` | `semantically_mapped` | Native ally-team pointer/count expands to Lua's winning ally-team table. |
| `GamePaused` | 2 | 2 | `GamePaused` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GamePreload` | 0 | 0 | `GamePreload` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GameProgress` | 1 | 1 | `GameProgress` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GameSetup` | 3 | 3 | `GameSetup` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GameStart` | 0 | 0 | `GameStart` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GetTooltip` | 2 | 2 | `ScreenPosition` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `GroupChanged` | 1 | 1 | `GroupChanged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `IsAbove` | 2 | 2 | `ScreenPosition` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `KeyMapChanged` | 0 | 1 | `SimpleCallin` | `semantically_mapped` | Native SimpleCallinQuery contains only an ABI placeholder; Lua receives no arguments. |
| `KeyPress` | 7 | 10 | `KeyPress` | `semantically_mapped` | Native modifier/action arrays and the key label are expanded into Lua's modifiers and actionList tables. |
| `KeyRelease` | 6 | 9 | `KeyRelease` | `semantically_mapped` | Native modifier/action arrays and the key label are expanded into Lua's modifiers and actionList tables. |
| `Load` | 1 | 1 | `ArchiveCallin` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MapDrawCmd` | 8 | 8 | `MapDrawCmd` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MiniMapGeometryChanged` | 8 | 8 | `MiniMapGeometryChanged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MiniMapRotationChanged` | 2 | 2 | `MiniMapRotationChanged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MiniMapStateChanged` | 3 | 3 | `MiniMapStateChanged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MouseMove` | 5 | 5 | `MouseMove` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MousePress` | 3 | 3 | `MousePress` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MouseRelease` | 3 | 3 | `MouseRelease` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MouseWheel` | 2 | 2 | `MouseWheel` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `MoveCtrlNotify` | 4 | 4 | `MoveCtrlNotify` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `PlayerAdded` | 1 | 1 | `PlayerAdded` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `PlayerChanged` | 1 | 1 | `PlayerChanged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `PlayerRemoved` | 2 | 2 | `PlayerRemoved` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `ProjectileCreated` | 3 | 3 | `ProjectileEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `ProjectileDestroyed` | 3 | 3 | `ProjectileEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `RenderUnitDestroyed` | 3 | 3 | `RenderUnitDestroyed` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `ResourceExcess` | 1 | 2 | `ResourceExcess` | `semantically_mapped` | Native pointer/count entries expand to Lua's resource-excess table. |
| `Save` | 1 | 1 | `ArchiveCallin` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `ShieldPreDamaged` | 13 | 9 | `ShieldPreDamaged` | `semantically_mapped` | Native startPos/hitPos Float3 values expand to Lua coordinate arguments. |
| `Shutdown` | 0 | 0 | `Shutdown` | `lifecycle_only_by_design` | Lua-handle and native-module lifecycle hooks share a label but have different owners and no event payload. |
| `StockpileChanged` | 6 | 6 | `StockpileChanged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `SunChanged` | 0 | 1 | `SunChanged` | `semantically_mapped` | Native query retains the new sun state for native consumers; Lua receives no arguments. |
| `TeamChanged` | 1 | 1 | `TeamChanged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `TeamDied` | 1 | 1 | `TeamDied` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `TerraformComplete` | 6 | 6 | `TerraformComplete` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `TextEditing` | 3 | 3 | `TextEditing` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `TextInput` | 1 | 1 | `TextInput` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitArrivedAtGoal` | 3 | 3 | `UnitMoveEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitCloaked` | 3 | 3 | `UnitCloakEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitCmdDone` | 7 | 4 | `UnitCmdDone` | `semantically_mapped` | NativeCallinCommand expands to Lua command params/options and tag. |
| `UnitCommand` | 7 | 7 | `UnitCommand` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitConstructionDecayed` | 6 | 6 | `UnitConstructionDecayed` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitCreated` | 4 | 4 | `UnitCreated` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitDamaged` | 10 | 10 | `UnitDamaged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitDecloaked` | 3 | 3 | `UnitCloakEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitDestroyed` | 7 | 7 | `UnitDestroyed` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitEnteredAir` | 3 | 3 | `UnitMovementClassEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitEnteredLos` | 4 | 4 | `UnitLosEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitEnteredRadar` | 4 | 4 | `UnitLosEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitEnteredUnderwater` | 3 | 3 | `UnitMovementClassEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitEnteredWater` | 3 | 3 | `UnitMovementClassEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitExperience` | 5 | 5 | `UnitExperience` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitFeatureCollision` | 2 | 2 | `UnitFeatureCollision` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitFinished` | 3 | 3 | `UnitFinished` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitFromFactory` | 6 | 6 | `UnitFromFactory` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitGiven` | 4 | 4 | `UnitGiven` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitHarvestStorageFull` | 3 | 3 | `UnitHarvestStorageFull` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitIdle` | 3 | 3 | `UnitIdle` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitLeftAir` | 3 | 3 | `UnitMovementClassEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitLeftLos` | 4 | 4 | `UnitLosEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitLeftRadar` | 4 | 4 | `UnitLosEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitLeftUnderwater` | 3 | 3 | `UnitMovementClassEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitLeftWater` | 3 | 3 | `UnitMovementClassEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitLoaded` | 5 | 5 | `UnitLoaded` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitMoveFailed` | 3 | 3 | `UnitMoveEvent` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitPreDamaged` | 10 | 10 | `UnitDamaged` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitReverseBuilt` | 3 | 3 | `UnitReverseBuilt` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitSeismicPing` | 7 | 5 | `UnitSeismicPing` | `semantically_mapped` | Native position Float3 expands to Lua x,y,z. |
| `UnitStunned` | 4 | 4 | `UnitStunned` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitTaken` | 4 | 4 | `UnitTaken` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitUnitCollision` | 2 | 2 | `UnitUnitCollision` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnitUnloaded` | 5 | 5 | `UnitUnloaded` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `UnsyncedHeightMapUpdate` | 0 | 4 | `RectChanged` | `semantically_mapped` | Native rectangle is an engine notification payload; Lua's callin is invoked without arguments. |
| `Update` | 1 | 1 | `Update` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |
| `ViewResize` | 2 | 16 | `ViewResize` | `semantically_mapped` | Native geometry fields expand to Lua's single geometry table with named fields. |
| `WorldTooltip` | 4 | 4 | `WorldTooltip` | `same_arity_pending_runtime_check` | Raw arity agrees; value-level parity still requires the executable harness. |

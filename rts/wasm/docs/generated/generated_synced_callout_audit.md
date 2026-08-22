# Synced callout audit

- source: `rts/wasm/generated/model.json`
- coverage: `rts/wasm/generated/core-executable-coverage.json`
- status: heuristic inventory; human review required
- candidate: no name or module heuristic matched
- review-required: module or name may depend on unsynced state

| module | callout | mutating | transport | review |
| --- | --- | ---: | --- | --- |
| `camera` | `SetCameraState` |  | `variable-input-nested-adapted` | `review-required` |
| `camera` | `SetCameraTarget` |  | `fixed-option` | `review-required` |
| `cob_script` | `CallCOBScript` | x | `handwritten-reviewed` | `candidate` |
| `cob_script` | `GetCOBScriptID` |  | `variable-input-borrowed` | `candidate` |
| `config` | `SetConfigFloat` |  | `variable-input-borrowed` | `review-required` |
| `config` | `SetConfigInt` |  | `variable-input-borrowed` | `review-required` |
| `config` | `SetConfigString` |  | `variable-input-borrowed` | `review-required` |
| `config` | `SetLogSectionFilterLevel` |  | `variable-input-borrowed` | `review-required` |
| `debug_input` | `ClearEmulatedInput` |  | `fixed` | `review-required` |
| `debug_input` | `EmulateKey` |  | `fixed` | `review-required` |
| `debug_input` | `EmulateMouseButton` |  | `fixed` | `review-required` |
| `debug_input` | `EmulateMouseMove` |  | `fixed` | `review-required` |
| `debug_input` | `EmulateMouseWheel` |  | `fixed` | `review-required` |
| `debug_input` | `EmulateTextEditing` |  | `variable-input-borrowed` | `review-required` |
| `debug_input` | `EmulateTextInput` |  | `variable-input-borrowed` | `review-required` |
| `display` | `SetTeamColor` |  | `fixed` | `review-required` |
| `effects_control` | `SpawnCEG` | x | `variable-input-nested-adapted` | `candidate` |
| `effects_control` | `SpawnExplosion` | x | `fixed` | `candidate` |
| `effects_control` | `SpawnSFX` | x | `fixed` | `candidate` |
| `encoding` | `DecodeBase64` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `encoding` | `DecodeBase64Url` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `encoding` | `EncodeBase64` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `encoding` | `EncodeBase64Url` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `encoding` | `IsValidBase64` |  | `variable-input-borrowed` | `candidate` |
| `encoding` | `IsValidBase64Url` |  | `variable-input-borrowed` | `candidate` |
| `feature_control` | `AddFeatureDamage` | x | `fixed` | `candidate` |
| `feature_control` | `CreateFeature` | x | `variable-input-nested-adapted` | `candidate` |
| `feature_control` | `CreateFeatureWreck` | x | `fixed` | `candidate` |
| `feature_control` | `CreateUnitWreck` | x | `fixed` | `candidate` |
| `feature_control` | `DestroyFeature` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureAlwaysVisible` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureBlocking` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureCollisionVolumeData` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureDirection` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureFireTime` | x | `fixed` | `review-required` |
| `feature_control` | `SetFeatureHeadingAndUpDir` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureHealth` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureMass` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureMaxHealth` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureMidAndAimPos` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureMoveCtrl` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureNoSelect` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeaturePhysics` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeaturePieceCollisionVolumeData` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeaturePieceMatrix` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeaturePieceVisible` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeaturePosition` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureRadiusAndHeight` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureReclaim` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureResources` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureResurrect` | x | `variable-input-nested-adapted` | `candidate` |
| `feature_control` | `SetFeatureRotation` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureSelectionVolumeData` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureSmokeTime` | x | `fixed` | `review-required` |
| `feature_control` | `SetFeatureUseAirLos` | x | `fixed` | `candidate` |
| `feature_control` | `SetFeatureVelocity` | x | `fixed` | `candidate` |
| `feature_control` | `TransferFeature` | x | `fixed` | `candidate` |
| `feature_defs` | `GetFeatureDefByID` |  | `dynamic-output-caller-owned` | `candidate` |
| `feature_defs` | `GetFeatureDefCount` |  | `fixed` | `candidate` |
| `feature_defs` | `GetFeatureDefCustomParam` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `feature_defs` | `GetFeatureDefCustomParamKeys` |  | `dynamic-output-caller-owned` | `candidate` |
| `feature_defs` | `GetFeatureDefEnergy` |  | `fixed` | `candidate` |
| `feature_defs` | `GetFeatureDefIDByName` |  | `variable-input-borrowed` | `candidate` |
| `feature_defs` | `GetFeatureDefIDs` |  | `variable-output-caller-owned` | `candidate` |
| `feature_defs` | `GetFeatureDefMetal` |  | `fixed` | `candidate` |
| `feature_defs` | `GetFeatureDefName` |  | `variable-output-caller-owned` | `candidate` |
| `feature_defs` | `ValidFeatureDefID` |  | `fixed` | `candidate` |
| `features` | `GetAllFeatures` |  | `variable-output-caller-owned` | `candidate` |
| `features` | `GetFeatureAllyTeam` |  | `fixed` | `candidate` |
| `features` | `GetFeatureBlocking` |  | `fixed` | `candidate` |
| `features` | `GetFeatureCollisionVolumeData` |  | `fixed` | `candidate` |
| `features` | `GetFeatureDefID` |  | `fixed` | `candidate` |
| `features` | `GetFeatureDirection` |  | `fixed` | `candidate` |
| `features` | `GetFeatureFireTime` |  | `fixed` | `review-required` |
| `features` | `GetFeatureHeading` |  | `fixed` | `candidate` |
| `features` | `GetFeatureHealth` |  | `fixed` | `candidate` |
| `features` | `GetFeatureHeight` |  | `fixed` | `candidate` |
| `features` | `GetFeatureLastAttackedPiece` |  | `dynamic-output-caller-owned` | `candidate` |
| `features` | `GetFeatureMass` |  | `fixed` | `candidate` |
| `features` | `GetFeatureNoSelect` |  | `fixed` | `candidate` |
| `features` | `GetFeaturePieceCollisionVolumeData` |  | `fixed` | `candidate` |
| `features` | `GetFeaturePosition` |  | `fixed` | `candidate` |
| `features` | `GetFeaturePositionExt` |  | `fixed` | `candidate` |
| `features` | `GetFeatureRadius` |  | `fixed` | `candidate` |
| `features` | `GetFeatureResources` |  | `fixed` | `candidate` |
| `features` | `GetFeatureResurrect` |  | `dynamic-output-caller-owned` | `candidate` |
| `features` | `GetFeatureRotation` |  | `fixed` | `candidate` |
| `features` | `GetFeatureSeparation` |  | `fixed` | `candidate` |
| `features` | `GetFeatureSmokeTime` |  | `fixed` | `review-required` |
| `features` | `GetFeatureTeam` |  | `fixed` | `candidate` |
| `features` | `GetFeatureVelocity` |  | `fixed` | `candidate` |
| `features` | `GetFeaturesInCylinder` |  | `variable-output-caller-owned` | `candidate` |
| `features` | `GetFeaturesInRectangle` |  | `variable-output-caller-owned` | `candidate` |
| `features` | `GetFeaturesInSphere` |  | `variable-output-caller-owned` | `candidate` |
| `features` | `ValidFeatureID` |  | `fixed` | `candidate` |
| `game` | `AreHelperAIsEnabled` |  | `fixed` | `candidate` |
| `game` | `FixedAllies` |  | `fixed` | `candidate` |
| `game` | `GetAllyTeamStartBox` |  | `fixed` | `candidate` |
| `game` | `GetFacingFromHeading` |  | `fixed` | `candidate` |
| `game` | `GetGaiaTeamID` |  | `fixed` | `candidate` |
| `game` | `GetGameFrame` |  | `fixed` | `candidate` |
| `game` | `GetGameMapInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `game` | `GetGameModInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `game` | `GetGameRulesInfo` |  | `fixed` | `candidate` |
| `game` | `GetGameRulesResourceInfo` |  | `fixed` | `candidate` |
| `game` | `GetGameSeconds` |  | `fixed` | `candidate` |
| `game` | `GetGameSetupInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `game` | `GetGlobalLos` |  | `fixed` | `candidate` |
| `game` | `GetHeadingFromFacing` |  | `fixed` | `candidate` |
| `game` | `GetHeadingFromVector` |  | `fixed` | `candidate` |
| `game` | `GetMapOption` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `game` | `GetMapOptions` |  | `dynamic-output-caller-owned` | `candidate` |
| `game` | `GetMapStartPositions` |  | `variable-output-caller-owned` | `candidate` |
| `game` | `GetModOption` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `game` | `GetModOptions` |  | `dynamic-output-caller-owned` | `candidate` |
| `game` | `GetSideData` |  | `dynamic-output-caller-owned` | `candidate` |
| `game` | `GetSideDataByIndex` |  | `dynamic-output-caller-owned` | `candidate` |
| `game` | `GetSideDataCount` |  | `fixed` | `candidate` |
| `game` | `GetTeamStartPosition` |  | `fixed` | `candidate` |
| `game` | `GetTidal` |  | `fixed` | `candidate` |
| `game` | `GetVectorFromHeading` |  | `fixed` | `candidate` |
| `game` | `GetWind` |  | `fixed` | `candidate` |
| `game` | `IsCheatingEnabled` |  | `fixed` | `candidate` |
| `game` | `IsDevLuaEnabled` |  | `fixed` | `candidate` |
| `game` | `IsEditDefsEnabled` |  | `fixed` | `candidate` |
| `game` | `IsGameOver` |  | `fixed` | `candidate` |
| `game` | `IsGodModeEnabled` |  | `fixed` | `candidate` |
| `game` | `IsNoCostEnabled` |  | `fixed` | `candidate` |
| `game_config` | `SetCheatingEnabled` | x | `fixed` | `candidate` |
| `game_config` | `SetExperienceGrade` | x | `fixed` | `candidate` |
| `game_config` | `SetGodMode` | x | `fixed` | `candidate` |
| `game_config` | `SetNoPause` | x | `fixed` | `candidate` |
| `game_config` | `SetRadarErrorParams` | x | `fixed` | `candidate` |
| `game_config` | `SetSquareBuildingMask` | x | `fixed` | `candidate` |
| `ground_decals` | `CreateGroundDecal` |  | `fixed` | `candidate` |
| `ground_decals` | `DestroyGroundDecal` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalAlpha` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalCreationFrame` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalGlowParams` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalMisc` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalNormal` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalPosAndDims` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalQuadPosAndHeight` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalRotation` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalTexture` |  | `variable-input-borrowed` | `candidate` |
| `ground_decals` | `SetGroundDecalTextureParams` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalTint` |  | `fixed` | `candidate` |
| `ground_decals` | `SetGroundDecalUserData` |  | `fixed` | `candidate` |
| `icons` | `AddUnitIcon` |  | `variable-input-borrowed` | `candidate` |
| `icons` | `FreeUnitIcon` |  | `variable-input-borrowed` | `candidate` |
| `icons` | `UnitIconSetDraw` |  | `fixed` | `candidate` |
| `lights` | `AddLightTrackingTarget` |  | `fixed` | `candidate` |
| `lights` | `AddMapLight` |  | `fixed` | `candidate` |
| `lights` | `AddModelLight` |  | `fixed` | `candidate` |
| `lights` | `SetMapLightTrackingState` |  | `fixed` | `candidate` |
| `lights` | `SetModelLightTrackingState` |  | `fixed` | `candidate` |
| `lights` | `UpdateMapLight` |  | `fixed` | `candidate` |
| `lights` | `UpdateModelLight` |  | `fixed` | `candidate` |
| `los` | `GetClosestValidPosition` |  | `fixed` | `candidate` |
| `los` | `GetPositionLosState` |  | `fixed` | `candidate` |
| `los` | `GetRadarErrorParams` |  | `fixed` | `candidate` |
| `los` | `IsPosInAirLos` |  | `fixed` | `candidate` |
| `los` | `IsPosInLos` |  | `fixed` | `candidate` |
| `los` | `IsPosInRadar` |  | `fixed` | `candidate` |
| `los` | `IsUnitInAirLos` |  | `fixed` | `candidate` |
| `los` | `IsUnitInJammer` |  | `fixed` | `candidate` |
| `los` | `IsUnitInLos` |  | `fixed` | `candidate` |
| `los` | `IsUnitInRadar` |  | `fixed` | `candidate` |
| `markers` | `AddWorldIcon` |  | `fixed` | `candidate` |
| `markers` | `AddWorldText` |  | `variable-input-borrowed-mixed-fixed` | `candidate` |
| `markers` | `AddWorldUnit` |  | `fixed` | `candidate` |
| `markers` | `MarkerAddLine` |  | `fixed` | `candidate` |
| `markers` | `MarkerAddPoint` |  | `variable-input-borrowed-mixed-fixed` | `candidate` |
| `markers` | `MarkerErasePosition` |  | `fixed` | `candidate` |
| `math_extra` | `BitAnd` |  | `fixed` | `candidate` |
| `math_extra` | `BitBits` |  | `variable-input-borrowed` | `candidate` |
| `math_extra` | `BitInv` |  | `fixed` | `candidate` |
| `math_extra` | `BitOr` |  | `fixed` | `candidate` |
| `math_extra` | `BitXor` |  | `fixed` | `candidate` |
| `math_extra` | `Clamp` |  | `fixed` | `candidate` |
| `math_extra` | `Diag` |  | `variable-input-borrowed` | `candidate` |
| `math_extra` | `Erf` |  | `fixed` | `candidate` |
| `math_extra` | `Hypot` |  | `fixed` | `candidate` |
| `math_extra` | `Mix` |  | `fixed` | `candidate` |
| `math_extra` | `Normalize` |  | `handwritten-reviewed` | `candidate` |
| `math_extra` | `Round` |  | `fixed` | `candidate` |
| `math_extra` | `Sgn` |  | `fixed` | `candidate` |
| `math_extra` | `SmoothStep` |  | `fixed` | `candidate` |
| `messages` | `Echo` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `Log` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendAllyChat` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendCommands` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendLuaGaiaMsg` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendLuaMenuMsg` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendLuaRulesMsg` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendLuaUIMsg` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendMessage` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendMessageToAllyTeam` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendMessageToPlayer` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendMessageToSpectators` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendMessageToTeam` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendPrivateChat` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendPublicChat` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendSkirmishAIMessage` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendSpectatorChat` |  | `handwritten-reviewed` | `candidate` |
| `messages` | `SendToUnsynced` |  | `handwritten-reviewed` | `candidate` |
| `metal_map` | `GetMetalAmount` |  | `fixed` | `candidate` |
| `metal_map` | `GetMetalExtraction` |  | `fixed` | `candidate` |
| `metal_map` | `GetMetalMapSize` |  | `fixed` | `candidate` |
| `metal_map` | `SetMetalAmount` | x | `fixed` | `candidate` |
| `move_ctrl` | `GetUnitEstimatedPath` |  | `variable-output-caller-owned` | `review-required` |
| `move_ctrl` | `GetUnitMoveTypeData` |  | `dynamic-output-caller-owned` | `candidate` |
| `move_ctrl` | `IsMoveCtrlEnabled` |  | `fixed` | `candidate` |
| `move_ctrl` | `MoveCtrl` | x | `fixed` | `candidate` |
| `move_ctrl` | `SetMoveCtrlGravity` | x | `fixed` | `candidate` |
| `path_finder` | `DeletePath` | x | `fixed` | `review-required` |
| `path_finder` | `FreePathNodeCostsArray` |  | `fixed` | `review-required` |
| `path_finder` | `GetNextWayPoint` |  | `fixed-option` | `candidate` |
| `path_finder` | `GetPathNodeCost` |  | `fixed` | `review-required` |
| `path_finder` | `GetPathNodeCosts` |  | `variable-output-caller-owned` | `review-required` |
| `path_finder` | `GetPathWayPoints` |  | `variable-output-caller-owned` | `review-required` |
| `path_finder` | `InitPathNodeCostsArray` |  | `fixed` | `review-required` |
| `path_finder` | `RequestPath` |  | `variable-input-borrowed-mixed-fixed` | `review-required` |
| `path_finder` | `SetPathNodeCost` | x | `fixed` | `review-required` |
| `path_finder` | `SetPathNodeCosts` | x | `fixed` | `review-required` |
| `platform` | `GetArchitecture` |  | `variable-output-caller-owned` | `candidate` |
| `platform` | `IsHeadless` |  | `fixed` | `candidate` |
| `profiling` | `GetTimerMicros` |  | `handwritten-reviewed` | `review-required` |
| `projectile_control` | `DeleteProjectile` | x | `fixed` | `candidate` |
| `projectile_control` | `SetPieceProjectileParams` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileAlwaysVisible` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileCEG` | x | `variable-input-borrowed` | `candidate` |
| `projectile_control` | `SetProjectileCollision` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileDamages` | x | `variable-input-borrowed` | `candidate` |
| `projectile_control` | `SetProjectileGravity` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileIgnoreTrackingError` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileIsIntercepted` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileMoveControl` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectilePosition` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileTarget` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileTimeToLive` | x | `fixed` | `review-required` |
| `projectile_control` | `SetProjectileUseAirLos` | x | `fixed` | `candidate` |
| `projectile_control` | `SetProjectileVelocity` | x | `fixed` | `candidate` |
| `projectile_control` | `SpawnProjectile` | x | `variable-input-nested-adapted` | `candidate` |
| `projectiles` | `GetAllProjectiles` |  | `variable-output-caller-owned` | `candidate` |
| `projectiles` | `GetPieceProjectileParams` |  | `dynamic-output-caller-owned` | `candidate` |
| `projectiles` | `GetProjectileAllyTeamID` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileDamages` |  | `dynamic-output-caller-owned` | `candidate` |
| `projectiles` | `GetProjectileDefID` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileDirection` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileGravity` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileIsIntercepted` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileOwnerID` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectilePosition` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileTarget` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileTeamID` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileTimeToLive` |  | `fixed` | `review-required` |
| `projectiles` | `GetProjectileType` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectileVelocity` |  | `fixed` | `candidate` |
| `projectiles` | `GetProjectilesInRectangle` |  | `variable-output-caller-owned` | `candidate` |
| `projectiles` | `GetProjectilesInSphere` |  | `variable-output-caller-owned` | `candidate` |
| `rules_params` | `GetFeatureRulesParam` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetFeatureRulesParams` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetGameRulesParam` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetGameRulesParams` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetPlayerRulesParam` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetPlayerRulesParams` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetTeamRulesParam` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetTeamRulesParams` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetUnitRulesParam` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `GetUnitRulesParams` |  | `dynamic-output-caller-owned` | `candidate` |
| `rules_params` | `SetFeatureRulesParam` | x | `variable-input-nested-adapted` | `candidate` |
| `rules_params` | `SetGameRulesParam` | x | `variable-input-nested-adapted` | `candidate` |
| `rules_params` | `SetPlayerRulesParam` | x | `variable-input-nested-adapted` | `candidate` |
| `rules_params` | `SetTeamRulesParam` | x | `variable-input-nested-adapted` | `candidate` |
| `rules_params` | `SetUnitRulesParam` | x | `variable-input-nested-adapted` | `candidate` |
| `selection` | `DeselectUnit` |  | `fixed` | `candidate` |
| `selection` | `DeselectUnitArray` |  | `variable-input-borrowed` | `candidate` |
| `selection` | `SelectUnit` |  | `fixed` | `candidate` |
| `selection` | `SelectUnitArray` |  | `variable-input-borrowed` | `candidate` |
| `selection` | `SetUnitGroup` |  | `fixed` | `candidate` |
| `sound` | `LoadSoundDef` |  | `variable-input-borrowed` | `review-required` |
| `sound` | `PauseSoundStream` |  | `fixed` | `review-required` |
| `sound` | `PlaySoundFile` |  | `variable-input-borrowed-mixed-fixed` | `review-required` |
| `sound` | `PlaySoundStream` |  | `variable-input-borrowed` | `review-required` |
| `sound` | `PreloadSoundItem` |  | `variable-input-borrowed` | `review-required` |
| `sound` | `SetSoundEffectParams` |  | `variable-input-nested-adapted` | `review-required` |
| `sound` | `SetSoundStreamVolume` |  | `fixed` | `review-required` |
| `sound` | `StopSoundStream` |  | `fixed` | `review-required` |
| `system_control` | `CallAsTeam` |  | `handwritten-reviewed` | `review-required` |
| `system_control` | `ClearWatchDogTimer` |  | `variable-input-borrowed` | `review-required` |
| `system_control` | `GarbageCollectCtrl` |  | `fixed` | `review-required` |
| `system_control` | `Ping` |  | `fixed` | `review-required` |
| `system_control` | `Quit` |  | `fixed` | `review-required` |
| `system_control` | `Reload` |  | `variable-input-borrowed` | `review-required` |
| `system_control` | `RequestStartPosition` |  | `fixed` | `review-required` |
| `system_control` | `Restart` |  | `variable-input-borrowed` | `review-required` |
| `system_control` | `SetShareLevel` |  | `variable-input-borrowed` | `review-required` |
| `system_control` | `ShareResources` |  | `variable-input-borrowed` | `review-required` |
| `system_control` | `Start` |  | `variable-input-borrowed` | `review-required` |
| `system_control` | `Yield` |  | `fixed` | `review-required` |
| `team_control` | `AddTeamResource` | x | `variable-input-borrowed` | `candidate` |
| `team_control` | `AddTeamResourceExcessStats` | x | `variable-input-borrowed` | `candidate` |
| `team_control` | `AssignPlayerToTeam` | x | `fixed` | `candidate` |
| `team_control` | `GameOver` | x | `variable-input-borrowed` | `candidate` |
| `team_control` | `KillTeam` | x | `fixed` | `candidate` |
| `team_control` | `SetAlly` | x | `fixed` | `candidate` |
| `team_control` | `SetAllyTeamStartBox` | x | `fixed` | `candidate` |
| `team_control` | `SetGlobalLos` | x | `fixed` | `candidate` |
| `team_control` | `SetPlayerReadyState` | x | `fixed` | `candidate` |
| `team_control` | `SetTeamResource` | x | `variable-input-borrowed` | `candidate` |
| `team_control` | `SetTeamShareLevel` | x | `variable-input-borrowed` | `candidate` |
| `team_control` | `SetTeamStartPosition` | x | `fixed` | `candidate` |
| `team_control` | `ShareTeamResource` | x | `variable-input-borrowed` | `candidate` |
| `team_control` | `TransferTeamMaxUnits` | x | `fixed` | `candidate` |
| `team_control` | `UseTeamResource` | x | `variable-input-borrowed` | `candidate` |
| `teams` | `ArePlayersAllied` |  | `fixed` | `candidate` |
| `teams` | `AreTeamsAllied` |  | `fixed` | `candidate` |
| `teams` | `GetAIInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `teams` | `GetAllyTeamInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `teams` | `GetAllyTeamList` |  | `variable-output-caller-owned` | `candidate` |
| `teams` | `GetPlayerControlledUnit` |  | `fixed` | `candidate` |
| `teams` | `GetPlayerInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `teams` | `GetPlayerList` |  | `variable-output-caller-owned` | `candidate` |
| `teams` | `GetPlayerListInAllyTeam` |  | `variable-output-caller-owned` | `candidate` |
| `teams` | `GetPlayerListInTeam` |  | `variable-output-caller-owned` | `candidate` |
| `teams` | `GetTeamAllyTeamID` |  | `fixed` | `candidate` |
| `teams` | `GetTeamInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `teams` | `GetTeamList` |  | `variable-output-caller-owned` | `candidate` |
| `teams` | `GetTeamLuaAI` |  | `variable-output-caller-owned` | `candidate` |
| `teams` | `GetTeamMaxUnits` |  | `fixed` | `candidate` |
| `teams` | `GetTeamResourceStats` |  | `variable-input-borrowed` | `candidate` |
| `teams` | `GetTeamResources` |  | `variable-input-borrowed` | `candidate` |
| `teams` | `GetTeamStatsHistory` |  | `variable-output-caller-owned` | `candidate` |
| `teams` | `GetTeamUnitStats` |  | `fixed` | `candidate` |
| `terrain` | `GetGrass` |  | `fixed` | `candidate` |
| `terrain` | `GetGroundBlocked` |  | `fixed` | `candidate` |
| `terrain` | `GetGroundExtremes` |  | `handwritten-reviewed` | `candidate` |
| `terrain` | `GetGroundHeight` |  | `fixed` | `candidate` |
| `terrain` | `GetGroundInfo` |  | `variable-output-caller-owned` | `candidate` |
| `terrain` | `GetGroundNormal` |  | `fixed` | `candidate` |
| `terrain` | `GetGroundOrigHeight` |  | `fixed` | `candidate` |
| `terrain` | `GetHeightMapSize` |  | `handwritten-reviewed` | `candidate` |
| `terrain` | `GetSmoothMeshHeight` |  | `fixed` | `candidate` |
| `terrain` | `GetTerrainTypeData` |  | `variable-output-caller-owned` | `candidate` |
| `terrain` | `GetWaterLevel` |  | `fixed` | `candidate` |
| `terrain` | `GetWaterPlaneLevel` |  | `handwritten-reviewed` | `candidate` |
| `terrain` | `IsPosInMap` |  | `handwritten-reviewed` | `candidate` |
| `terrain_control` | `AddGrass` | x | `fixed` | `candidate` |
| `terrain_control` | `AddHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `AddOriginalHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `AddSmoothMesh` | x | `fixed` | `candidate` |
| `terrain_control` | `AdjustHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `AdjustOriginalHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `AdjustSmoothMesh` | x | `fixed` | `candidate` |
| `terrain_control` | `LevelHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `LevelOriginalHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `LevelSmoothMesh` | x | `fixed` | `candidate` |
| `terrain_control` | `RebuildSmoothMesh` | x | `fixed` | `candidate` |
| `terrain_control` | `RemoveGrass` | x | `fixed` | `candidate` |
| `terrain_control` | `RevertHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `RevertOriginalHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `RevertSmoothMesh` | x | `fixed` | `candidate` |
| `terrain_control` | `SetHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `SetHeightMapFunc` | x | `handwritten-reviewed` | `candidate` |
| `terrain_control` | `SetMapSquareTerrainType` | x | `fixed` | `candidate` |
| `terrain_control` | `SetOriginalHeightMap` | x | `fixed` | `candidate` |
| `terrain_control` | `SetOriginalHeightMapFunc` | x | `handwritten-reviewed` | `candidate` |
| `terrain_control` | `SetSmoothMesh` | x | `fixed` | `candidate` |
| `terrain_control` | `SetSmoothMeshFunc` | x | `handwritten-reviewed` | `candidate` |
| `terrain_control` | `SetTerrainTypeData` | x | `variable-input-borrowed` | `candidate` |
| `terrain_control` | `SetTidal` | x | `fixed` | `candidate` |
| `terrain_control` | `SetWind` | x | `fixed` | `candidate` |
| `tracing` | `TraceRay` |  | `fixed` | `review-required` |
| `tracing` | `TraceRayBetweenPositions` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `tracing` | `TraceRayFeatures` |  | `fixed` | `review-required` |
| `tracing` | `TraceRayGroundBetweenPositions` |  | `fixed-option` | `review-required` |
| `tracing` | `TraceRayGroundInDirection` |  | `fixed-option` | `review-required` |
| `tracing` | `TraceRayInDirection` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `tracing` | `TraceRayUnits` |  | `fixed` | `review-required` |
| `unit_control` | `AddObjectDecal` | x | `fixed` | `candidate` |
| `unit_control` | `AddUnitDamage` | x | `fixed` | `candidate` |
| `unit_control` | `AddUnitExperience` | x | `fixed` | `candidate` |
| `unit_control` | `AddUnitImpulse` | x | `fixed` | `candidate` |
| `unit_control` | `AddUnitResource` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `AddUnitSeismicPing` | x | `fixed` | `candidate` |
| `unit_control` | `BuggerOff` | x | `variable-input-borrowed-mixed-fixed` | `candidate` |
| `unit_control` | `ClearUnitGoal` | x | `fixed` | `candidate` |
| `unit_control` | `CreateUnit` | x | `variable-input-nested-adapted` | `candidate` |
| `unit_control` | `DestroyUnit` | x | `fixed` | `candidate` |
| `unit_control` | `EditUnitCmdDesc` | x | `variable-input-nested-adapted` | `candidate` |
| `unit_control` | `ForceUnitCollisionUpdate` | x | `fixed` | `candidate` |
| `unit_control` | `GetUnitFeatureSeparation` |  | `fixed` | `candidate` |
| `unit_control` | `GetUnitLeavesGhost` |  | `fixed` | `candidate` |
| `unit_control` | `GetUnitPhysicalState` |  | `fixed` | `candidate` |
| `unit_control` | `GiveOrderArrayToUnit` | x | `variable-input-nested-adapted` | `candidate` |
| `unit_control` | `GiveOrderArrayToUnitArray` | x | `variable-input-nested-adapted` | `candidate` |
| `unit_control` | `GiveOrderToUnit` | x | `handwritten-reviewed` | `candidate` |
| `unit_control` | `GiveOrderToUnitArray` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `InsertUnitCmdDesc` | x | `variable-input-nested-adapted` | `candidate` |
| `unit_control` | `RemoveObjectDecal` | x | `fixed` | `candidate` |
| `unit_control` | `RemoveUnitCmdDesc` | x | `fixed` | `candidate` |
| `unit_control` | `SetFactoryBuggerOff` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitAlwaysVisible` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitArmored` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitBlocking` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitBuildParams` | x | `variable-input-borrowed-mixed-fixed` | `candidate` |
| `unit_control` | `SetUnitBuildSpeed` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitBuildeeRadius` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitCloak` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitCollisionVolumeData` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitCosts` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitCrashing` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitDirection` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitExperience` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitFlanking` | x | `variable-input-borrowed-mixed-fixed` | `candidate` |
| `unit_control` | `SetUnitHarvestStorage` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitHeading` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitHeadingAndUpDir` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitHealth` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitLandGoal` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitLeavesGhost` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitLoadingTransport` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitLosMask` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitLosState` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitMass` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitMaxHealth` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitMaxRange` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitMetalExtraction` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitMidAndAimPos` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitMoveGoal` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitNanoPieces` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `SetUnitNeutral` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPhysicalStateBit` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPhysics` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPieceCollisionVolumeData` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPieceMatrix` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPieceParent` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPieceVisible` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPosErrorParams` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitPosition` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitRadiusAndHeight` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitResourcing` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `SetUnitRotation` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitSeismicSignature` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitSelectionVolumeData` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitSensorRadius` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `SetUnitShieldRechargeDelay` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitShieldState` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitSonarStealth` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitStealth` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitStockpile` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitStorage` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `SetUnitTarget` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitTooltip` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `SetUnitUseAirLos` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitUseWeapons` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitVelocity` | x | `fixed` | `candidate` |
| `unit_control` | `SetUnitWeaponDamages` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `SetUnitWeaponState` | x | `variable-input-borrowed` | `candidate` |
| `unit_control` | `TransferUnit` | x | `fixed` | `candidate` |
| `unit_control` | `UnitAttach` | x | `fixed` | `candidate` |
| `unit_control` | `UnitDetach` | x | `fixed` | `candidate` |
| `unit_control` | `UnitDetachFromAir` | x | `fixed` | `candidate` |
| `unit_control` | `UnitFinishCommand` | x | `fixed` | `candidate` |
| `unit_control` | `UnitWeaponFire` | x | `fixed` | `candidate` |
| `unit_control` | `UnitWeaponHoldFire` | x | `fixed` | `candidate` |
| `unit_control` | `UseUnitResource` | x | `variable-input-borrowed` | `candidate` |
| `unit_defs` | `GetUnitDefByID` |  | `dynamic-output-caller-owned` | `candidate` |
| `unit_defs` | `GetUnitDefClassify` |  | `fixed` | `candidate` |
| `unit_defs` | `GetUnitDefCosts` |  | `fixed` | `candidate` |
| `unit_defs` | `GetUnitDefCount` |  | `fixed` | `candidate` |
| `unit_defs` | `GetUnitDefCustomParam` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `unit_defs` | `GetUnitDefCustomParamKeys` |  | `dynamic-output-caller-owned` | `candidate` |
| `unit_defs` | `GetUnitDefHealth` |  | `fixed` | `candidate` |
| `unit_defs` | `GetUnitDefHumanName` |  | `handwritten-reviewed` | `candidate` |
| `unit_defs` | `GetUnitDefIDByName` |  | `variable-input-borrowed` | `candidate` |
| `unit_defs` | `GetUnitDefIDs` |  | `variable-output-caller-owned` | `candidate` |
| `unit_defs` | `GetUnitDefName` |  | `handwritten-reviewed` | `candidate` |
| `unit_defs` | `GetUnitDefParamBool` |  | `variable-input-borrowed` | `candidate` |
| `unit_defs` | `GetUnitDefParamFloat` |  | `variable-input-borrowed` | `candidate` |
| `unit_defs` | `GetUnitDefParamInt` |  | `variable-input-borrowed` | `candidate` |
| `unit_defs` | `GetUnitDefParamKeys` |  | `dynamic-output-caller-owned` | `candidate` |
| `unit_defs` | `GetUnitDefParamString` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `unit_defs` | `GetUnitDefParamType` |  | `variable-input-borrowed` | `candidate` |
| `unit_defs` | `GetUnitDefSpeed` |  | `fixed` | `candidate` |
| `unit_defs` | `ValidUnitDefID` |  | `fixed` | `candidate` |
| `units_commands` | `FindUnitCmdDesc` |  | `fixed` | `candidate` |
| `units_commands` | `GetCommandParams` |  | `variable-input-nested-adapted` | `candidate` |
| `units_commands` | `GetCommandQueue` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_commands` | `GetFactoryBuggerOff` |  | `fixed` | `candidate` |
| `units_commands` | `GetFactoryCommandCount` |  | `fixed` | `candidate` |
| `units_commands` | `GetFactoryCommands` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_commands` | `GetFactoryCounts` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_commands` | `GetFullBuildQueue` |  | `variable-output-caller-owned` | `candidate` |
| `units_commands` | `GetRealBuildQueue` |  | `variable-output-caller-owned` | `candidate` |
| `units_commands` | `GetUnitCmdDescs` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_commands` | `GetUnitCommandCount` |  | `fixed` | `candidate` |
| `units_commands` | `GetUnitCommands` |  | `handwritten-reviewed` | `candidate` |
| `units_commands` | `GetUnitCurrentCommand` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_commands` | `GiveOrder` | x | `handwritten-reviewed` | `candidate` |
| `units_commands` | `GiveOrderArrayToUnitMap` | x | `variable-input-nested-adapted` | `candidate` |
| `units_commands` | `GiveOrderToUnitMap` | x | `handwritten-reviewed` | `candidate` |
| `units_info` | `GetUnitAllyTeam` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitArmored` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitBasePosition` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitBlocking` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitBuildFacing` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitBuildParams` |  | `variable-input-borrowed` | `candidate` |
| `units_info` | `GetUnitBuildeeRadius` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitCollisionVolumeData` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitCostTable` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitCosts` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitCrashing` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitCurrentBuildPower` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitDefID` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitDirection` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitEffectiveBuildRange` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitExperience` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitFlanking` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitHarvestStorage` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitHeading` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitHealth` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitHeight` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitInBuildStance` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitIsActive` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitIsBeingBuilt` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitIsBuilding` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitIsCloaked` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitIsDead` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitIsStunned` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitIsTransporting` |  | `handwritten-reviewed` | `candidate` |
| `units_info` | `GetUnitLastAttackedPiece` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_info` | `GetUnitLastAttacker` |  | `fixed-option` | `candidate` |
| `units_info` | `GetUnitLosState` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitMass` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitMetalExtraction` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitMoveDefID` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitNanoPieces` |  | `handwritten-reviewed` | `candidate` |
| `units_info` | `GetUnitNeutral` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitPieceCollisionVolumeData` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitPosErrorParams` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitPosition` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitRadius` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitResources` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitRotation` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitSeismicSignature` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitSelfDTime` |  | `fixed` | `review-required` |
| `units_info` | `GetUnitSensorRadius` |  | `variable-input-borrowed` | `candidate` |
| `units_info` | `GetUnitShieldState` |  | `fixed-option` | `candidate` |
| `units_info` | `GetUnitStates` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitStockpile` |  | `fixed-option` | `candidate` |
| `units_info` | `GetUnitStorage` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitTeam` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitTooltip` |  | `variable-output-caller-owned` | `candidate` |
| `units_info` | `GetUnitTransporter` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitVectors` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitVelocity` |  | `fixed` | `candidate` |
| `units_info` | `GetUnitWorkerTask` |  | `fixed` | `candidate` |
| `units_pieces` | `GetFeaturePieceDirection` |  | `fixed` | `candidate` |
| `units_pieces` | `GetFeaturePieceInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetFeaturePieceList` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetFeaturePieceMap` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetFeaturePieceMatrix` |  | `fixed` | `candidate` |
| `units_pieces` | `GetFeaturePiecePosDir` |  | `fixed` | `candidate` |
| `units_pieces` | `GetFeaturePiecePosition` |  | `fixed` | `candidate` |
| `units_pieces` | `GetFeatureRootPiece` |  | `fixed` | `candidate` |
| `units_pieces` | `GetModelPieceList` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetModelPieceMap` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetModelRootPiece` |  | `variable-input-borrowed` | `candidate` |
| `units_pieces` | `GetUnitPieceDirection` |  | `fixed` | `candidate` |
| `units_pieces` | `GetUnitPieceInfo` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetUnitPieceList` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetUnitPieceMap` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetUnitPieceMatrix` |  | `fixed` | `candidate` |
| `units_pieces` | `GetUnitPiecePosDir` |  | `fixed` | `candidate` |
| `units_pieces` | `GetUnitPiecePosition` |  | `fixed` | `candidate` |
| `units_pieces` | `GetUnitRootPiece` |  | `fixed` | `candidate` |
| `units_pieces` | `GetUnitScriptNames` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_pieces` | `GetUnitScriptPiece` |  | `fixed` | `candidate` |
| `units_query` | `GetAllUnits` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetClosestEnemyUnit` |  | `fixed` | `candidate` |
| `units_query` | `GetTeamUnitCount` |  | `fixed` | `candidate` |
| `units_query` | `GetTeamUnitDefCount` |  | `fixed` | `candidate` |
| `units_query` | `GetTeamUnits` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetTeamUnitsByDefs` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetTeamUnitsCounts` |  | `variable-output-caller-owned` | `candidate` |
| `units_query` | `GetTeamUnitsSorted` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_query` | `GetUnitArrayCentroid` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetUnitMapCentroid` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetUnitNearestAlly` |  | `fixed` | `candidate` |
| `units_query` | `GetUnitNearestEnemy` |  | `fixed` | `candidate` |
| `units_query` | `GetUnitSeparation` |  | `fixed` | `candidate` |
| `units_query` | `GetUnitsInBox` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetUnitsInCylinder` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetUnitsInPlanes` |  | `variable-output-caller-owned` | `candidate` |
| `units_query` | `GetUnitsInRectangle` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `GetUnitsInSphere` |  | `handwritten-reviewed` | `candidate` |
| `units_query` | `ValidUnitID` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitMaxRange` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponCanFire` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponCount` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponDamages` |  | `dynamic-output-caller-owned` | `candidate` |
| `units_weapons` | `GetUnitWeaponHaveFreeLineOfFire` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponState` |  | `variable-input-borrowed` | `candidate` |
| `units_weapons` | `GetUnitWeaponTarget` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponTestRange` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponTestTarget` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponTryTarget` |  | `fixed` | `candidate` |
| `units_weapons` | `GetUnitWeaponVectors` |  | `fixed` | `candidate` |
| `unsynced_ctrl` | `AssignMouseCursor` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `DeselectUnitMap` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `DrawUnitCommands` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `ForceLayoutUpdate` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `ForceTesselationUpdate` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `GetWaterTexture` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `unsynced_ctrl` | `LoadCmdColorsConfig` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `LoadCtrlPanelConfig` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `LoadModelTextures` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `PauseDollyCamera` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `PreloadFeatureDefModel` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `PreloadUnitDefModel` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `ReplaceMouseCursor` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `ResumeDollyCamera` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `RunDollyCamera` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SDLSetTextInputRect` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SDLStartTextInput` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SDLStopTextInput` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SelectUnitMap` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetActiveCommand` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetAtmosphere` |  | `fixed-option` | `review-required` |
| `unsynced_ctrl` | `SetAutoShowMetal` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetBoxSelectionByEngine` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetBuildFacing` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetBuildSpacing` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetCameraOffset` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetClipboard` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetCustomCommandDrawData` |  | `variable-input-nested-adapted` | `review-required` |
| `unsynced_ctrl` | `SetCustomPaletteColor` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDollyCameraCurve` |  | `variable-input-adapted` | `review-required` |
| `unsynced_ctrl` | `SetDollyCameraLookCurve` |  | `variable-input-adapted` | `review-required` |
| `unsynced_ctrl` | `SetDollyCameraLookPosition` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDollyCameraLookUnit` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDollyCameraMode` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDollyCameraPosition` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDollyCameraRelativeMode` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDrawGround` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDrawGroundDeferred` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDrawModelsDeferred` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDrawSelectionInfo` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDrawSky` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetDrawWater` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetEngineBuildSquareRendering` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetFeatureAlwaysUpdateMatrix` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetFeatureEngineDrawMask` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetFeatureFade` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetFeatureNoDraw` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetFeaturePaletteIndex` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetLastMessagePosition` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetLosViewColors` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetMapRenderingParams` |  | `fixed-option` | `review-required` |
| `unsynced_ctrl` | `SetMapShader` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetMapShadingTexture` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetMiniMapRotation` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetMouseCursor` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetNanoProjectileParams` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetShockFrontFactors` |  | `fixed-option` | `review-required` |
| `unsynced_ctrl` | `SetSkyBoxTexture` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetSunDirection` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetSunLighting` |  | `fixed-option` | `review-required` |
| `unsynced_ctrl` | `SetUnitAlwaysUpdateMatrix` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitDefIcon` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetUnitDefImage` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetUnitEngineDrawMask` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitIcon` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetUnitIconDraw` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitLeaveTracks` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitNoDraw` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitNoGroup` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitNoMinimap` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitNoSelect` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetUnitPaletteIndex` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetVideoCapturingMode` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetVideoCapturingTimeOffset` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetWMCaption` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetWMIcon` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetWaterParams` |  | `fixed-option` | `review-required` |
| `unsynced_ctrl` | `SetWaterTexture` |  | `variable-input-borrowed` | `review-required` |
| `unsynced_ctrl` | `SetWindowGeometry` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetWindowMaximized` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `SetWindowMinimized` |  | `fixed` | `review-required` |
| `unsynced_ctrl` | `WarpMouse` |  | `fixed` | `review-required` |
| `unsynced_read` | `GetPieceProjectileName` |  | `variable-output-caller-owned` | `review-required` |
| `unsynced_read` | `GetTeamDamageStats` |  | `fixed` | `review-required` |
| `utils` | `ClosestBuildPos` |  | `fixed` | `candidate` |
| `utils` | `GetCEGID` |  | `variable-input-borrowed` | `candidate` |
| `utils` | `GetFeatureDefDimensions` |  | `fixed` | `candidate` |
| `utils` | `GetUnitDefDimensions` |  | `fixed` | `candidate` |
| `utils` | `Pos2BuildPos` |  | `fixed` | `candidate` |
| `utils` | `TestBuildOrder` |  | `fixed` | `candidate` |
| `utils` | `TestMoveOrder` |  | `fixed` | `candidate` |
| `vfs` | `CalculateHash` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `CreateDir` |  | `variable-input-borrowed` | `review-required` |
| `vfs` | `DirList` |  | `dynamic-output-caller-owned` | `review-required` |
| `vfs` | `ExtractModArchiveFile` |  | `variable-input-borrowed` | `review-required` |
| `vfs` | `FileExists` |  | `variable-input-borrowed` | `review-required` |
| `vfs` | `GetArchives` |  | `dynamic-output-caller-owned` | `review-required` |
| `vfs` | `GetFileInfo` |  | `dynamic-output-caller-owned` | `review-required` |
| `vfs` | `GetFileSize` |  | `variable-input-borrowed` | `review-required` |
| `vfs` | `GetMapSquareTextureInfo` |  | `fixed` | `review-required` |
| `vfs` | `IsDirectory` |  | `variable-input-borrowed` | `review-required` |
| `vfs` | `ListDir` |  | `dynamic-output-caller-owned` | `review-required` |
| `vfs` | `LoadFile` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `PackF32` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `PackS16` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `PackS32` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `PackS8` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `PackU16` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `PackU32` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `PackU8` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `ReadFile` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `ReadFileAsString` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `SetMapSquareTexture` |  | `variable-input-borrowed` | `review-required` |
| `vfs` | `SubDirs` |  | `dynamic-output-caller-owned` | `review-required` |
| `vfs` | `UnpackF32` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `UnpackS16` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `UnpackS32` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `UnpackS8` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `UnpackU16` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `UnpackU32` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `UnpackU8` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `ZlibCompress` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `vfs` | `ZlibDecompress` |  | `variable-io-borrowed-input-caller-owned-output` | `review-required` |
| `weapon_defs` | `GetWeaponDefByID` |  | `dynamic-output-caller-owned` | `candidate` |
| `weapon_defs` | `GetWeaponDefCount` |  | `fixed` | `candidate` |
| `weapon_defs` | `GetWeaponDefCustomParam` |  | `variable-io-borrowed-input-caller-owned-output` | `candidate` |
| `weapon_defs` | `GetWeaponDefCustomParamKeys` |  | `dynamic-output-caller-owned` | `candidate` |
| `weapon_defs` | `GetWeaponDefDamage` |  | `fixed` | `candidate` |
| `weapon_defs` | `GetWeaponDefID` |  | `variable-input-borrowed` | `candidate` |
| `weapon_defs` | `GetWeaponDefIDs` |  | `variable-output-caller-owned` | `candidate` |
| `weapon_defs` | `GetWeaponDefName` |  | `variable-output-caller-owned` | `candidate` |
| `weapon_defs` | `GetWeaponDefRange` |  | `fixed` | `candidate` |
| `weapon_defs` | `ValidWeaponDefID` |  | `fixed` | `candidate` |

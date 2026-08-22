# Core API reference

- source: `rts/wasm/generated/model.json`
- coverage: `rts/wasm/generated/core-executable-coverage.json`
- callouts: 1354

## Modules

| module | callouts |
| --- | ---: |
| `units_query` | 21 |
| `units_info` | 57 |
| `units_weapons` | 11 |
| `units_commands` | 16 |
| `units_pieces` | 21 |
| `teams` | 19 |
| `features` | 38 |
| `projectiles` | 17 |
| `los` | 10 |
| `unit_defs` | 19 |
| `feature_defs` | 10 |
| `weapon_defs` | 10 |
| `game` | 33 |
| `terrain` | 13 |
| `player` | 7 |
| `math_extra` | 14 |
| `encoding` | 6 |
| `metal_map` | 4 |
| `path_finder` | 10 |
| `platform` | 2 |
| `rules_params` | 15 |
| `move_ctrl` | 5 |
| `camera` | 10 |
| `input` | 20 |
| `debug_input` | 7 |
| `display` | 24 |
| `selection` | 16 |
| `sound` | 11 |
| `messages` | 21 |
| `config` | 9 |
| `tracing` | 7 |
| `utils` | 7 |
| `unsynced_ctrl` | 82 |
| `gfx` | 237 |
| `lights` | 7 |
| `icons` | 6 |
| `markers` | 6 |
| `ground_decals` | 31 |
| `system_control` | 22 |
| `profiling` | 10 |
| `rml_ui` | 180 |
| `vfs` | 52 |
| `unsynced_read` | 20 |
| `team_control` | 15 |
| `unit_control` | 88 |
| `feature_control` | 32 |
| `terrain_control` | 25 |
| `projectile_control` | 16 |
| `effects_control` | 3 |
| `game_config` | 6 |
| `cob_script` | 2 |
| `unit_rendering` | 24 |

## `units_query`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetAllUnits` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetClosestEnemyUnit` | x | x | x | x | x | synced-visible | fixed |  |
| `GetRenderUnits` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetRenderUnitsDrawFlagChanged` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetTeamUnitCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetTeamUnitDefCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetTeamUnits` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetTeamUnitsByDefs` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetTeamUnitsCounts` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetTeamUnitsSorted` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitArrayCentroid` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitMapCentroid` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitNearestAlly` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitNearestEnemy` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitSeparation` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitsInBox` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitsInCylinder` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitsInPlanes` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetUnitsInRectangle` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitsInSphere` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `ValidUnitID` | x | x | x | x | x | synced-visible | fixed |  |

## `units_info`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClearUnitsPreviousDrawFlag` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitAllyTeam` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitArmored` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitBasePosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitBlocking` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitBuildFacing` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitBuildParams` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitBuildeeRadius` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitCollisionVolumeData` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitCostTable` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitCosts` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitCrashing` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitCurrentBuildPower` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitDefID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitDirection` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitEffectiveBuildRange` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitExperience` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitFlanking` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitHarvestStorage` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitHeading` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitHealth` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitHeight` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitInBuildStance` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitIsActive` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitIsBeingBuilt` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitIsBuilding` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitIsCloaked` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitIsDead` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitIsStunned` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitIsTransporting` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitLastAttackedPiece` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitLastAttacker` | x | x | x | x | x | synced-visible | fixed-option |  |
| `GetUnitLosState` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitMass` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitMetalExtraction` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitMoveDefID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitNanoPieces` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitNeutral` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPieceCollisionVolumeData` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPosErrorParams` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitRadius` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitResources` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitRotation` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitSeismicSignature` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitSelfDTime` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitSensorRadius` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitShieldState` | x | x | x | x | x | synced-visible | fixed-option |  |
| `GetUnitStates` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitStockpile` | x | x | x | x | x | synced-visible | fixed-option |  |
| `GetUnitStorage` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitTeam` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitTooltip` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetUnitTransporter` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitVectors` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitVelocity` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWorkerTask` | x | x | x | x | x | synced-visible | fixed |  |

## `units_weapons`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetUnitMaxRange` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponCanFire` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponDamages` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitWeaponHaveFreeLineOfFire` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponState` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitWeaponTarget` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponTestRange` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponTestTarget` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponTryTarget` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitWeaponVectors` | x | x | x | x | x | synced-visible | fixed |  |

## `units_commands`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `FindUnitCmdDesc` | x | x | x | x | x | synced-visible | fixed |  |
| `GetCommandParams` | x | x | x | x | x | synced-visible | variable-input-nested-adapted |  |
| `GetCommandQueue` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFactoryBuggerOff` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFactoryCommandCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFactoryCommands` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFactoryCounts` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFullBuildQueue` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetRealBuildQueue` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetUnitCmdDescs` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitCommandCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitCommands` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitCurrentCommand` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GiveOrder` | x |  | x |  |  | synced-visible | handwritten-reviewed | x |
| `GiveOrderArrayToUnitMap` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `GiveOrderToUnitMap` | x |  | x |  |  | synced-visible | handwritten-reviewed | x |

## `units_pieces`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetFeaturePieceDirection` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeaturePieceInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeaturePieceList` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeaturePieceMap` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeaturePieceMatrix` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeaturePiecePosDir` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeaturePiecePosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureRootPiece` | x | x | x | x | x | synced-visible | fixed |  |
| `GetModelPieceList` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetModelPieceMap` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetModelRootPiece` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitPieceDirection` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPieceInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitPieceList` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitPieceMap` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitPieceMatrix` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPiecePosDir` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPiecePosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitRootPiece` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitScriptNames` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitScriptPiece` | x | x | x | x | x | synced-visible | fixed |  |

## `teams`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ArePlayersAllied` | x | x | x | x | x | synced-visible | fixed |  |
| `AreTeamsAllied` | x | x | x | x | x | synced-visible | fixed |  |
| `GetAIInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetAllyTeamInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetAllyTeamList` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetPlayerControlledUnit` | x | x | x | x | x | synced-visible | fixed |  |
| `GetPlayerInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetPlayerList` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetPlayerListInAllyTeam` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetPlayerListInTeam` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetTeamAllyTeamID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetTeamInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetTeamList` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetTeamLuaAI` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetTeamMaxUnits` | x | x | x | x | x | synced-visible | fixed |  |
| `GetTeamResourceStats` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetTeamResources` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetTeamStatsHistory` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetTeamUnitStats` | x | x | x | x | x | synced-visible | fixed |  |

## `features`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClearFeaturesPreviousDrawFlag` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetAllFeatures` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetFeatureAllyTeam` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureAlwaysUpdateMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeatureBlocking` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureCollisionVolumeData` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureDefID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureDirection` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureDrawFlag` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeatureEngineDrawMask` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeatureFireTime` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureHeading` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureHealth` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureHeight` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureLastAttackedPiece` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeatureLuaDraw` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeatureMass` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureNoDraw` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeatureNoSelect` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeaturePieceCollisionVolumeData` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeaturePosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeaturePositionExt` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureRadius` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureResources` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureResurrect` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeatureRotation` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureSelectionVolumeData` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeatureSeparation` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureSmokeTime` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureTeam` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureTransformMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeatureVelocity` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeaturesInCylinder` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetFeaturesInRectangle` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetFeaturesInSphere` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetRenderFeatures` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetRenderFeaturesDrawFlagChanged` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ValidFeatureID` | x | x | x | x | x | synced-visible | fixed |  |

## `projectiles`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetAllProjectiles` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetPieceProjectileParams` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetProjectileAllyTeamID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileDamages` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetProjectileDefID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileDirection` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileGravity` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileIsIntercepted` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileOwnerID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectilePosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileTarget` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileTeamID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileTimeToLive` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileType` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectileVelocity` | x | x | x | x | x | synced-visible | fixed |  |
| `GetProjectilesInRectangle` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetProjectilesInSphere` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |

## `los`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetClosestValidPosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetPositionLosState` | x | x | x | x | x | synced-visible | fixed |  |
| `GetRadarErrorParams` | x | x | x | x | x | synced-visible | fixed |  |
| `IsPosInAirLos` | x | x | x | x | x | synced-visible | fixed |  |
| `IsPosInLos` | x | x | x | x | x | synced-visible | fixed |  |
| `IsPosInRadar` | x | x | x | x | x | synced-visible | fixed |  |
| `IsUnitInAirLos` | x | x | x | x | x | synced-visible | fixed |  |
| `IsUnitInJammer` | x | x | x | x | x | synced-visible | fixed |  |
| `IsUnitInLos` | x | x | x | x | x | synced-visible | fixed |  |
| `IsUnitInRadar` | x | x | x | x | x | synced-visible | fixed |  |

## `unit_defs`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetUnitDefByID` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitDefClassify` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitDefCosts` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitDefCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitDefCustomParam` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `GetUnitDefCustomParamKeys` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitDefHealth` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitDefHumanName` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitDefIDByName` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitDefIDs` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetUnitDefName` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetUnitDefParamBool` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitDefParamFloat` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitDefParamInt` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitDefParamKeys` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitDefParamString` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `GetUnitDefParamType` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetUnitDefSpeed` | x | x | x | x | x | synced-visible | fixed |  |
| `ValidUnitDefID` | x | x | x | x | x | synced-visible | fixed |  |

## `feature_defs`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetFeatureDefByID` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeatureDefCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureDefCustomParam` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `GetFeatureDefCustomParamKeys` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeatureDefEnergy` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureDefIDByName` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetFeatureDefIDs` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetFeatureDefMetal` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFeatureDefName` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `ValidFeatureDefID` | x | x | x | x | x | synced-visible | fixed |  |

## `weapon_defs`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetWeaponDefByID` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetWeaponDefCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetWeaponDefCustomParam` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `GetWeaponDefCustomParamKeys` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetWeaponDefDamage` | x | x | x | x | x | synced-visible | fixed |  |
| `GetWeaponDefID` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetWeaponDefIDs` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetWeaponDefName` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetWeaponDefRange` | x | x | x | x | x | synced-visible | fixed |  |
| `ValidWeaponDefID` | x | x | x | x | x | synced-visible | fixed |  |

## `game`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AreHelperAIsEnabled` | x | x | x | x | x | synced-visible | fixed |  |
| `FixedAllies` | x | x | x | x | x | synced-visible | fixed |  |
| `GetAllyTeamStartBox` | x | x | x | x | x | synced-visible | fixed |  |
| `GetFacingFromHeading` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGaiaTeamID` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGameFrame` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGameMapInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetGameModInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetGameRulesInfo` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGameRulesResourceInfo` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGameSeconds` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGameSetupInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetGlobalLos` | x | x | x | x | x | synced-visible | fixed |  |
| `GetHeadingFromFacing` | x | x | x | x | x | synced-visible | fixed |  |
| `GetHeadingFromVector` | x | x | x | x | x | synced-visible | fixed |  |
| `GetMapOption` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `GetMapOptions` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetMapStartPositions` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetModOption` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `GetModOptions` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetSideData` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetSideDataByIndex` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetSideDataCount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetTeamStartPosition` | x | x | x | x | x | synced-visible | fixed |  |
| `GetTidal` | x | x | x | x | x | synced-visible | fixed |  |
| `GetVectorFromHeading` | x | x | x | x | x | synced-visible | fixed |  |
| `GetWind` | x | x | x | x | x | synced-visible | fixed |  |
| `IsCheatingEnabled` | x | x | x | x | x | synced-visible | fixed |  |
| `IsDevLuaEnabled` | x | x | x | x | x | synced-visible | fixed |  |
| `IsEditDefsEnabled` | x | x | x | x | x | synced-visible | fixed |  |
| `IsGameOver` | x | x | x | x | x | synced-visible | fixed |  |
| `IsGodModeEnabled` | x | x | x | x | x | synced-visible | fixed |  |
| `IsNoCostEnabled` | x | x | x | x | x | synced-visible | fixed |  |

## `terrain`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetGrass` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGroundBlocked` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGroundExtremes` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetGroundHeight` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGroundInfo` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetGroundNormal` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGroundOrigHeight` | x | x | x | x | x | synced-visible | fixed |  |
| `GetHeightMapSize` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetSmoothMeshHeight` | x | x | x | x | x | synced-visible | fixed |  |
| `GetTerrainTypeData` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetWaterLevel` | x | x | x | x | x | synced-visible | fixed |  |
| `GetWaterPlaneLevel` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `IsPosInMap` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |

## `player`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetLocalAllyTeamID` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetLocalPlayerID` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetLocalTeamID` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetPlayerRoster` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetPlayerStatistics` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetPlayerTraffic` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetSpectatingState` |  | x |  | x | x | unsynced-only | fixed |  |

## `math_extra`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `BitAnd` | x | x | x | x | x | synced-visible | fixed |  |
| `BitBits` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `BitInv` | x | x | x | x | x | synced-visible | fixed |  |
| `BitOr` | x | x | x | x | x | synced-visible | fixed |  |
| `BitXor` | x | x | x | x | x | synced-visible | fixed |  |
| `Clamp` | x | x | x | x | x | synced-visible | fixed |  |
| `Diag` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `Erf` | x | x | x | x | x | synced-visible | fixed |  |
| `Hypot` | x | x | x | x | x | synced-visible | fixed |  |
| `Mix` | x | x | x | x | x | synced-visible | fixed |  |
| `Normalize` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `Round` | x | x | x | x | x | synced-visible | fixed |  |
| `Sgn` | x | x | x | x | x | synced-visible | fixed |  |
| `SmoothStep` | x | x | x | x | x | synced-visible | fixed |  |

## `encoding`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DecodeBase64` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `DecodeBase64Url` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `EncodeBase64` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `EncodeBase64Url` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `IsValidBase64` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `IsValidBase64Url` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |

## `metal_map`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetMetalAmount` | x | x | x | x | x | synced-visible | fixed |  |
| `GetMetalExtraction` | x | x | x | x | x | synced-visible | fixed |  |
| `GetMetalMapSize` | x | x | x | x | x | synced-visible | fixed |  |
| `SetMetalAmount` | x |  | x |  |  | synced-visible | fixed | x |

## `path_finder`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DeletePath` | x |  | x |  |  | synced-visible | fixed | x |
| `FreePathNodeCostsArray` | x | x | x | x | x | synced-visible | fixed |  |
| `GetNextWayPoint` | x | x | x | x | x | synced-visible | fixed-option |  |
| `GetPathNodeCost` | x | x | x | x | x | synced-visible | fixed |  |
| `GetPathNodeCosts` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetPathWayPoints` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `InitPathNodeCostsArray` | x | x | x | x | x | synced-visible | fixed |  |
| `RequestPath` | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |  |
| `SetPathNodeCost` | x |  | x |  |  | synced-visible | fixed | x |
| `SetPathNodeCosts` | x |  | x |  |  | synced-visible | fixed | x |

## `platform`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetArchitecture` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `IsHeadless` | x | x | x | x | x | synced-visible | fixed |  |

## `rules_params`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetFeatureRulesParam` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFeatureRulesParams` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetGameRulesParam` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetGameRulesParams` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetPlayerRulesParam` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetPlayerRulesParams` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetTeamRulesParam` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetTeamRulesParams` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitRulesParam` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetUnitRulesParams` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `SetFeatureRulesParam` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `SetGameRulesParam` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `SetPlayerRulesParam` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `SetTeamRulesParam` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `SetUnitRulesParam` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |

## `move_ctrl`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetUnitEstimatedPath` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetUnitMoveTypeData` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `IsMoveCtrlEnabled` | x |  | x |  |  | synced-visible | fixed |  |
| `MoveCtrl` | x |  | x |  |  | synced-visible | fixed | x |
| `SetMoveCtrlGravity` | x |  | x |  |  | synced-visible | fixed | x |

## `camera`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetCameraDirection` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetCameraFOV` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetCameraNames` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetCameraPosition` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetCameraState` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetPixelDir` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetCameraState` | x | x | x | x | x | synced-visible | variable-input-nested-adapted |  |
| `SetCameraTarget` | x | x | x | x | x | synced-visible | fixed-option |  |
| `TraceScreenRay` |  | x |  | x | x | unsynced-only | fixed |  |
| `WorldToScreenCoords` |  | x |  | x | x | unsynced-only | fixed |  |

## `input`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetActionHotKeys` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetActiveCommand` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetActivePage` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetDefaultCommand` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetInvertQueueKey` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetKeyBindings` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetKeyCode` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetKeyFromScanSymbol` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetKeyState` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetKeySymbol` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetModKeyState` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetMouseButtonsPressed` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetMouseCursor` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetMouseStartPosition` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetMouseState` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetPressedKeys` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetPressedScans` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetScanSymbol` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetSelectionBox` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsAboveMiniMap` |  | x |  | x | x | unsynced-only | fixed |  |

## `debug_input`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClearEmulatedInput` | x | x | x | x | x | synced-visible | fixed |  |
| `EmulateKey` | x | x | x | x | x | synced-visible | fixed |  |
| `EmulateMouseButton` | x | x | x | x | x | synced-visible | fixed |  |
| `EmulateMouseMove` | x | x | x | x | x | synced-visible | fixed |  |
| `EmulateMouseWheel` | x | x | x | x | x | synced-visible | fixed |  |
| `EmulateTextEditing` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `EmulateTextInput` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |

## `display`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetDrawFrame` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetDualViewGeometry` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFPS` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFrameTimeOffset` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGameSpeed` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetLastUpdateSeconds` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetLosViewColors` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetMapDrawMode` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetMiniMapDualScreen` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetMiniMapGeometry` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetMiniMapRotation` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetNumDisplays` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetScreenGeometry` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetTeamColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetTeamOrigColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetViewGeometry` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetWaterMode` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetWindowGeometry` |  | x |  | x | x | unsynced-only | fixed |  |
| `HaveAdvShading` |  | x |  | x | x | unsynced-only | fixed |  |
| `HaveShadows` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsAABBInView` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsGUIHidden` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsSphereInView` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetTeamColor` | x | x | x | x | x | synced-visible | fixed |  |

## `selection`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DeselectUnit` | x | x | x | x | x | synced-visible | fixed |  |
| `DeselectUnitArray` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetGroupList` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetGroupUnits` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetGroupUnitsCount` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroupUnitsCounts` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetGroupUnitsSorted` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetSelectedGroup` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetSelectedUnits` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetSelectedUnitsCount` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetSelectedUnitsCounts` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetSelectedUnitsSorted` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetUnitGroup` |  | x |  | x | x | unsynced-only | fixed |  |
| `SelectUnit` | x | x | x | x | x | synced-visible | fixed |  |
| `SelectUnitArray` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetUnitGroup` | x | x | x | x | x | synced-visible | fixed |  |

## `sound`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetSoundDevices` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetSoundEffectParams` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetSoundStreamTime` |  | x |  | x | x | unsynced-only | fixed |  |
| `LoadSoundDef` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `PauseSoundStream` | x | x | x | x | x | synced-visible | fixed |  |
| `PlaySoundFile` | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |  |
| `PlaySoundStream` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `PreloadSoundItem` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetSoundEffectParams` | x | x | x | x | x | synced-visible | variable-input-nested-adapted |  |
| `SetSoundStreamVolume` | x | x | x | x | x | synced-visible | fixed |  |
| `StopSoundStream` | x | x | x | x | x | synced-visible | fixed |  |

## `messages`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `Echo` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `GetConsoleBuffer` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetCurrentTooltip` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `IsUserWriting` |  | x |  | x | x | unsynced-only | fixed |  |
| `Log` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendAllyChat` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendCommands` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SendLuaGaiaMsg` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendLuaMenuMsg` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendLuaRulesMsg` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendLuaUIMsg` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendMessage` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendMessageToAllyTeam` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendMessageToPlayer` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendMessageToSpectators` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendMessageToTeam` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendPrivateChat` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendPublicChat` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendSkirmishAIMessage` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendSpectatorChat` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `SendToUnsynced` | x |  | x |  |  | synced-visible | handwritten-reviewed |  |

## `config`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetConfigFloat` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `GetConfigInt` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `GetConfigParams` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetConfigString` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetLogSections` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `SetConfigFloat` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetConfigInt` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetConfigString` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetLogSectionFilterLevel` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |

## `tracing`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `TraceRay` | x | x | x | x | x | synced-visible | fixed |  |
| `TraceRayBetweenPositions` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `TraceRayFeatures` | x | x | x | x | x | synced-visible | fixed |  |
| `TraceRayGroundBetweenPositions` | x | x | x | x | x | synced-visible | fixed-option |  |
| `TraceRayGroundInDirection` | x | x | x | x | x | synced-visible | fixed-option |  |
| `TraceRayInDirection` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `TraceRayUnits` | x | x | x | x | x | synced-visible | fixed |  |

## `utils`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClosestBuildPos` | x | x | x | x | x | synced-visible | fixed |  |
| `GetCEGID` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetFeatureDefDimensions` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitDefDimensions` | x | x | x | x | x | synced-visible | fixed |  |
| `Pos2BuildPos` | x | x | x | x | x | synced-visible | fixed |  |
| `TestBuildOrder` | x | x | x | x | x | synced-visible | fixed |  |
| `TestMoveOrder` | x | x | x | x | x | synced-visible | fixed |  |

## `unsynced_ctrl`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AssignMouseCursor` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `DeselectUnitMap` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `DrawUnitCommands` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `ForceLayoutUpdate` | x | x | x | x | x | synced-visible | fixed |  |
| `ForceTesselationUpdate` | x | x | x | x | x | synced-visible | fixed |  |
| `GetWaterTexture` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `LoadCmdColorsConfig` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `LoadCtrlPanelConfig` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `LoadModelTextures` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `PauseDollyCamera` | x | x | x | x | x | synced-visible | fixed |  |
| `PreloadFeatureDefModel` | x | x | x | x | x | synced-visible | fixed |  |
| `PreloadUnitDefModel` | x | x | x | x | x | synced-visible | fixed |  |
| `ReplaceMouseCursor` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `ResumeDollyCamera` | x | x | x | x | x | synced-visible | fixed |  |
| `RunDollyCamera` | x | x | x | x | x | synced-visible | fixed |  |
| `SDLSetTextInputRect` | x | x | x | x | x | synced-visible | fixed |  |
| `SDLStartTextInput` | x | x | x | x | x | synced-visible | fixed |  |
| `SDLStopTextInput` | x | x | x | x | x | synced-visible | fixed |  |
| `SelectUnitMap` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetActiveCommand` | x | x | x | x | x | synced-visible | fixed |  |
| `SetAtmosphere` | x | x | x | x | x | synced-visible | fixed-option |  |
| `SetAutoShowMetal` | x | x | x | x | x | synced-visible | fixed |  |
| `SetBoxSelectionByEngine` | x | x | x | x | x | synced-visible | fixed |  |
| `SetBuildFacing` | x | x | x | x | x | synced-visible | fixed |  |
| `SetBuildSpacing` | x | x | x | x | x | synced-visible | fixed |  |
| `SetCameraOffset` | x | x | x | x | x | synced-visible | fixed |  |
| `SetClipboard` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetCustomCommandDrawData` | x | x | x | x | x | synced-visible | variable-input-nested-adapted |  |
| `SetCustomPaletteColor` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDollyCameraCurve` | x | x | x | x | x | synced-visible | variable-input-adapted |  |
| `SetDollyCameraLookCurve` | x | x | x | x | x | synced-visible | variable-input-adapted |  |
| `SetDollyCameraLookPosition` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDollyCameraLookUnit` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDollyCameraMode` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDollyCameraPosition` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDollyCameraRelativeMode` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDrawGround` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDrawGroundDeferred` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDrawModelsDeferred` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDrawSelectionInfo` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDrawSky` | x | x | x | x | x | synced-visible | fixed |  |
| `SetDrawWater` | x | x | x | x | x | synced-visible | fixed |  |
| `SetEngineBuildSquareRendering` | x | x | x | x | x | synced-visible | fixed |  |
| `SetFeatureAlwaysUpdateMatrix` | x | x | x | x | x | synced-visible | fixed |  |
| `SetFeatureEngineDrawMask` | x | x | x | x | x | synced-visible | fixed |  |
| `SetFeatureFade` | x | x | x | x | x | synced-visible | fixed |  |
| `SetFeatureNoDraw` | x | x | x | x | x | synced-visible | fixed |  |
| `SetFeaturePaletteIndex` | x | x | x | x | x | synced-visible | fixed |  |
| `SetLastMessagePosition` | x | x | x | x | x | synced-visible | fixed |  |
| `SetLosViewColors` | x | x | x | x | x | synced-visible | fixed |  |
| `SetMapRenderingParams` | x | x | x | x | x | synced-visible | fixed-option |  |
| `SetMapShader` | x | x | x | x | x | synced-visible | fixed |  |
| `SetMapShadingTexture` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetMiniMapRotation` | x | x | x | x | x | synced-visible | fixed |  |
| `SetMouseCursor` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetNanoProjectileParams` | x | x | x | x | x | synced-visible | fixed |  |
| `SetShockFrontFactors` | x | x | x | x | x | synced-visible | fixed-option |  |
| `SetSkyBoxTexture` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetSunDirection` | x | x | x | x | x | synced-visible | fixed |  |
| `SetSunLighting` | x | x | x | x | x | synced-visible | fixed-option |  |
| `SetUnitAlwaysUpdateMatrix` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitDefIcon` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetUnitDefImage` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetUnitEngineDrawMask` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitIcon` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetUnitIconDraw` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitLeaveTracks` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitNoDraw` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitNoGroup` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitNoMinimap` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitNoSelect` | x | x | x | x | x | synced-visible | fixed |  |
| `SetUnitPaletteIndex` | x | x | x | x | x | synced-visible | fixed |  |
| `SetVideoCapturingMode` | x | x | x | x | x | synced-visible | fixed |  |
| `SetVideoCapturingTimeOffset` | x | x | x | x | x | synced-visible | fixed |  |
| `SetWMCaption` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetWMIcon` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetWaterParams` | x | x | x | x | x | synced-visible | fixed-option |  |
| `SetWaterTexture` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetWindowGeometry` | x | x | x | x | x | synced-visible | fixed |  |
| `SetWindowMaximized` | x | x | x | x | x | synced-visible | fixed |  |
| `SetWindowMinimized` | x | x | x | x | x | synced-visible | fixed |  |
| `WarpMouse` | x | x | x | x | x | synced-visible | fixed |  |

## `gfx`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ActiveFBO` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `ActiveShader` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `ActiveTexture` |  | x |  | x | x | unsynced-only | fixed |  |
| `AddAtlasTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `AddFallbackFont` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `AddFeatureDefsToSubmissionVAO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `AddFeaturesToSubmissionVAO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `AddUnitDefsToSubmissionVAO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `AddUnitsToSubmissionVAO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `AlphaTest` |  | x |  | x | x | unsynced-only | fixed |  |
| `AlphaToCoverage` |  | x |  | x | x | unsynced-only | fixed |  |
| `AttachIndexBufferVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `AttachInstanceBufferVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `AttachVertexBufferVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `BeginEnd` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `BeginText` |  | x |  | x | x | unsynced-only | fixed |  |
| `Billboard` |  | x |  | x | x | unsynced-only | fixed |  |
| `BindBufferRangeVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `BindImageTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `BindTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `BlendEquation` |  | x |  | x | x | unsynced-only | fixed |  |
| `BlendEquationSeparate` |  | x |  | x | x | unsynced-only | fixed |  |
| `BlendFunc` |  | x |  | x | x | unsynced-only | fixed |  |
| `BlendFuncSeparate` |  | x |  | x | x | unsynced-only | fixed |  |
| `Blending` |  | x |  | x | x | unsynced-only | fixed |  |
| `BlitFBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `CallList` |  | x |  | x | x | unsynced-only | fixed |  |
| `ChangeTextureParams` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `Clear` |  | x |  | x | x | unsynced-only | fixed |  |
| `ClearAttachmentFBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `ClearFallbackFonts` |  | x |  | x | x | unsynced-only | fixed |  |
| `ClearSubmissionVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `ClearVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `ClipDistance` |  | x |  | x | x | unsynced-only | fixed |  |
| `ClipPlane` |  | x |  | x | x | unsynced-only | fixed |  |
| `Color` |  | x |  | x | x | unsynced-only | fixed |  |
| `ColorMask` |  | x |  | x | x | unsynced-only | fixed |  |
| `ConfigMiniMap` |  | x |  | x | x | unsynced-only | fixed |  |
| `CopyToTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `CopyToVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `CreateFBO` |  | x |  | x | x | unsynced-only | variable-input-nested-adapted |  |
| `CreateList` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `CreateQuery` |  | x |  | x | x | unsynced-only | fixed |  |
| `CreateRBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `CreateShader` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `CreateTexture` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `CreateTextureAtlas` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `Culling` |  | x |  | x | x | unsynced-only | fixed |  |
| `DefineVBO` |  | x |  | x | x | unsynced-only | variable-input-adapted |  |
| `DeleteFBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `DeleteFont` |  | x |  | x | x | unsynced-only | fixed |  |
| `DeleteList` |  | x |  | x | x | unsynced-only | fixed |  |
| `DeleteQuery` |  | x |  | x | x | unsynced-only | fixed |  |
| `DeleteRBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `DeleteShader` |  | x |  | x | x | unsynced-only | fixed |  |
| `DeleteTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DeleteTextureAtlas` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DeleteTextureFBO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DeleteVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `DeleteVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `DepthClamp` |  | x |  | x | x | unsynced-only | fixed |  |
| `DepthMask` |  | x |  | x | x | unsynced-only | fixed |  |
| `DepthTest` |  | x |  | x | x | unsynced-only | fixed |  |
| `DispatchCompute` |  | x |  | x | x | unsynced-only | fixed |  |
| `DownloadVBO` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `DrawArraysVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `DrawElementsVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `DrawFuncAtUnit` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `DrawGroundCircle` |  | x |  | x | x | unsynced-only | fixed |  |
| `DrawGroundQuad` |  | x |  | x | x | unsynced-only | fixed |  |
| `DrawListAtUnit` |  | x |  | x | x | unsynced-only | fixed |  |
| `DrawMiniMap` |  | x |  | x | x | unsynced-only | fixed |  |
| `DumpDefinitionVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `EdgeFlag` |  | x |  | x | x | unsynced-only | fixed |  |
| `EndText` |  | x |  | x | x | unsynced-only | fixed |  |
| `Feature` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeatureMultMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeaturePiece` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeaturePieceMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeaturePieceMultMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeatureRaw` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeatureShape` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeatureShapeTextures` |  | x |  | x | x | unsynced-only | fixed |  |
| `FeatureTextures` |  | x |  | x | x | unsynced-only | fixed |  |
| `FinalizeTextureAtlas` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `Finish` |  | x |  | x | x | unsynced-only | fixed |  |
| `Flush` |  | x |  | x | x | unsynced-only | fixed |  |
| `Fog` |  | x |  | x | x | unsynced-only | fixed |  |
| `FogCoord` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontBegin` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontBindTexture` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontEnd` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontGetTextHeight` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `FontGetTextWidth` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `FontPrint` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `FontPrintWorld` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `FontSetAutoOutlineColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontSetOutlineColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontSetTextColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontSubmitBuffered` |  | x |  | x | x | unsynced-only | fixed |  |
| `FontWrapText` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `Frustum` |  | x |  | x | x | unsynced-only | fixed |  |
| `GenerateMipmap` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetActiveUniforms` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetAtlasTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetAtmosphere` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetConsoleCommands` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetEngineAtlasTextures` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetEngineModelUniformDataDef` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetEngineModelUniformDataSize` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetEngineTextureNames` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetEngineUniformBufferDef` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetFixedState` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetFontInfo` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetGlobalTexCoords` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetGlobalTexNames` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetIDVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetMapRendering` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetMatrixData` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetNumber` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetQuery` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetRBOInfo` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetScreenViewTrans` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetShaderLog` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetShadowMapParams` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetString` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetSubroutineIndex` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetSun` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetTextHeight` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetTextWidth` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetUniformLocation` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetVBOInfo` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetViewRange` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetViewSizes` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetWaterRendering` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `HasExtension` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `InstanceDataFromFeatureDefsVBO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `InstanceDataFromFeaturesVBO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `InstanceDataFromUnitDefsVBO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `InstanceDataFromUnitsVBO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `IsValidFBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `Light` |  | x |  | x | x | unsynced-only | fixed |  |
| `Lighting` |  | x |  | x | x | unsynced-only | fixed |  |
| `LineStipple` |  | x |  | x | x | unsynced-only | fixed |  |
| `LineWidth` |  | x |  | x | x | unsynced-only | fixed |  |
| `LoadFont` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `LoadIdentity` |  | x |  | x | x | unsynced-only | fixed |  |
| `LoadMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `LogicOp` |  | x |  | x | x | unsynced-only | fixed |  |
| `Material` |  | x |  | x | x | unsynced-only | fixed |  |
| `MatrixDataFromProjectilesVBO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `MatrixMode` |  | x |  | x | x | unsynced-only | fixed |  |
| `MemoryBarrier` |  | x |  | x | x | unsynced-only | fixed |  |
| `ModelsVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `MultMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `MultiTexCoord` |  | x |  | x | x | unsynced-only | fixed |  |
| `MultiTexEnv` |  | x |  | x | x | unsynced-only | fixed |  |
| `MultiTexGen` |  | x |  | x | x | unsynced-only | fixed |  |
| `Normal` |  | x |  | x | x | unsynced-only | fixed |  |
| `ObjectLabel` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `Ortho` |  | x |  | x | x | unsynced-only | fixed |  |
| `PointParameter` |  | x |  | x | x | unsynced-only | fixed |  |
| `PointSize` |  | x |  | x | x | unsynced-only | fixed |  |
| `PointSprite` |  | x |  | x | x | unsynced-only | fixed |  |
| `PolygonMode` |  | x |  | x | x | unsynced-only | fixed |  |
| `PolygonOffset` |  | x |  | x | x | unsynced-only | fixed |  |
| `PopAttrib` |  | x |  | x | x | unsynced-only | fixed |  |
| `PopDebugGroup` |  | x |  | x | x | unsynced-only | fixed |  |
| `PopMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `PushAttrib` |  | x |  | x | x | unsynced-only | fixed |  |
| `PushDebugGroup` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `PushMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `PushPopMatrix` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `RawBindFBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `ReadPixels` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `Rect` |  | x |  | x | x | unsynced-only | fixed |  |
| `RemoveFromSubmissionVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `RenderToTexture` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `ResetMatrices` |  | x |  | x | x | unsynced-only | fixed |  |
| `ResetState` |  | x |  | x | x | unsynced-only | fixed |  |
| `Rotate` |  | x |  | x | x | unsynced-only | fixed |  |
| `RunQuery` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `SaveImage` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `Scale` |  | x |  | x | x | unsynced-only | fixed |  |
| `Scissor` |  | x |  | x | x | unsynced-only | fixed |  |
| `SecondaryColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetFBOAttachment` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `SetFBODrawBuffers` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `SetFBOReadBuffer` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetFeatureBufferUniforms` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `SetGeometryShaderParameter` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetTesselationShaderParameter` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetUnitBufferUniforms` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ShadeModel` |  | x |  | x | x | unsynced-only | fixed |  |
| `Shape` |  | x |  | x | x | unsynced-only | variable-input-adapted |  |
| `SlaveMiniMap` |  | x |  | x | x | unsynced-only | fixed |  |
| `StencilFunc` |  | x |  | x | x | unsynced-only | fixed |  |
| `StencilFuncSeparate` |  | x |  | x | x | unsynced-only | fixed |  |
| `StencilMask` |  | x |  | x | x | unsynced-only | fixed |  |
| `StencilMaskSeparate` |  | x |  | x | x | unsynced-only | fixed |  |
| `StencilOp` |  | x |  | x | x | unsynced-only | fixed |  |
| `StencilOpSeparate` |  | x |  | x | x | unsynced-only | fixed |  |
| `StencilTest` |  | x |  | x | x | unsynced-only | fixed |  |
| `SubmitVAO` |  | x |  | x | x | unsynced-only | fixed |  |
| `SwapBuffers` |  | x |  | x | x | unsynced-only | fixed |  |
| `TexCoord` |  | x |  | x | x | unsynced-only | fixed |  |
| `TexEnv` |  | x |  | x | x | unsynced-only | fixed |  |
| `TexGen` |  | x |  | x | x | unsynced-only | fixed |  |
| `TexRect` |  | x |  | x | x | unsynced-only | fixed |  |
| `Text` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `TextEnv` |  | x |  | x | x | unsynced-only | fixed |  |
| `TextureInfo` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `Translate` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnbindBufferRangeVBO` |  | x |  | x | x | unsynced-only | fixed |  |
| `Uniform` |  | x |  | x | x | unsynced-only | fixed |  |
| `UniformArrayFloat` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `UniformArrayInt` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `UniformInt` |  | x |  | x | x | unsynced-only | fixed |  |
| `UniformMatrix` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `UniformSubroutine` |  | x |  | x | x | unsynced-only | fixed |  |
| `Unit` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitMultMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitPiece` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitPieceMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitPieceMultMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitRaw` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitShape` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitShapeTextures` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitTextures` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnsafeState` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `UploadTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `UploadVBO` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `UseShader` |  | x |  | x | x | unsynced-only | fixed |  |
| `Vertex` |  | x |  | x | x | unsynced-only | fixed |  |
| `Viewport` |  | x |  | x | x | unsynced-only | fixed |  |

## `lights`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddLightTrackingTarget` | x | x | x | x | x | synced-visible | fixed |  |
| `AddMapLight` | x | x | x | x | x | synced-visible | fixed |  |
| `AddModelLight` | x | x | x | x | x | synced-visible | fixed |  |
| `SetMapLightTrackingState` | x | x | x | x | x | synced-visible | fixed |  |
| `SetModelLightTrackingState` | x | x | x | x | x | synced-visible | fixed |  |
| `UpdateMapLight` | x | x | x | x | x | synced-visible | fixed |  |
| `UpdateModelLight` | x | x | x | x | x | synced-visible | fixed |  |

## `icons`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddUnitIcon` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `FreeUnitIcon` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetAllIconDataArray` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetIconData` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `UnitIconGetDraw` |  | x |  | x | x | unsynced-only | fixed |  |
| `UnitIconSetDraw` | x | x | x | x | x | synced-visible | fixed |  |

## `markers`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddWorldIcon` | x | x | x | x | x | synced-visible | fixed |  |
| `AddWorldText` | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |  |
| `AddWorldUnit` | x | x | x | x | x | synced-visible | fixed |  |
| `MarkerAddLine` | x | x | x | x | x | synced-visible | fixed |  |
| `MarkerAddPoint` | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |  |
| `MarkerErasePosition` | x | x | x | x | x | synced-visible | fixed |  |

## `ground_decals`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `CreateGroundDecal` | x | x | x | x | x | synced-visible | fixed |  |
| `DestroyGroundDecal` | x | x | x | x | x | synced-visible | fixed |  |
| `GetAllGroundDecals` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetGroundDecalAlpha` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalCreationFrame` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalGlowParams` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalMiddlePos` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalMisc` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalNormal` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalOwner` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalQuadPos` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalRotation` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalSizeAndHeight` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalTexture` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetGroundDecalTextureParams` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalTextures` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetGroundDecalTint` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGroundDecalType` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetGroundDecalUserData` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetGroundDecalAlpha` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalCreationFrame` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalGlowParams` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalMisc` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalNormal` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalPosAndDims` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalQuadPosAndHeight` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalRotation` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalTexture` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetGroundDecalTextureParams` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalTint` | x | x | x | x | x | synced-visible | fixed |  |
| `SetGroundDecalUserData` | x | x | x | x | x | synced-visible | fixed |  |

## `system_control`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `CallAsTeam` | x | x | x | x | x | synced-visible | handwritten-reviewed |  |
| `ClearWatchDogTimer` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GarbageCollectCtrl` | x | x | x | x | x | synced-visible | fixed |  |
| `GetGameName` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetGameState` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGatherMode` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetMenuName` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetReplayFilePath` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetReplayLength` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetReplayRecordingFilePath` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetVideoCapturingMode` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetWindowDisplayMode` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `IsReplay` |  | x |  | x | x | unsynced-only | fixed |  |
| `Ping` | x | x | x | x | x | synced-visible | fixed |  |
| `Quit` | x | x | x | x | x | synced-visible | fixed |  |
| `Reload` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `RequestStartPosition` | x | x | x | x | x | synced-visible | fixed |  |
| `Restart` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SetShareLevel` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `ShareResources` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `Start` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `Yield` | x | x | x | x | x | synced-visible | fixed |  |

## `profiling`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DiffTimers` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetDrawSeconds` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFrameTimer` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetLuaMemUsage` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetProfilerRecordNames` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetProfilerTimeRecord` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetSyncedGCInfo` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetTimer` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetTimerMicros` | x | x | x | x | x | synced-visible | fixed |  |
| `GetVidMemUsage` |  | x |  | x | x | unsynced-only | fixed |  |

## `rml_ui`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddTranslationString` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ClearDocumentPathRequests` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ClearTranslations` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextActivateTheme` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextAddEventListener` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `ContextCreateDataModel` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextCreateDocument` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextEnableMouseCursor` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextGetDensityIndependentPixelRatio` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextGetDimensions` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextGetDocument` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextGetElementAtPoint` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextGetFocusElement` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextGetHoverElement` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextGetName` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ContextGetRootElement` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextIsMouseInteracting` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextIsThemeActive` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextLoadDocument` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextOpenDataModel` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextProcessKeyDown` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextProcessKeyUp` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextProcessMouseButtonDown` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextProcessMouseButtonUp` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextProcessMouseLeave` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextProcessMouseMove` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextProcessMouseWheel` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextProcessTextInput` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextPullDocumentToFront` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextPullToFront` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextPushDocumentToBack` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextRemoveDataModel` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextRemoveEventListener` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ContextRender` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextSetDensityIndependentPixelRatio` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextSetDimensions` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextSetPointerCapture` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextTakePointerCaptureDelta` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextUnloadAllDocuments` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextUnloadDocument` |  | x |  | x | x | unsynced-only | fixed |  |
| `ContextUpdate` |  | x |  | x | x | unsynced-only | fixed |  |
| `CreateContext` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelBindBool` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelBindColor` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelBindEvent` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `DataModelBindFloat` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelBindInt` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelBindPercent` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelBindPixels` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelBindRows` |  | x |  | x | x | unsynced-only | variable-input-nested-adapted |  |
| `DataModelBindString` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelGetBool` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelGetColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelGetFloat` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelGetInt` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelGetPercent` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelGetPixels` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelGetString` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `DataModelSetBool` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelSetColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelSetFloat` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelSetInt` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelSetPercent` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelSetPixels` |  | x |  | x | x | unsynced-only | fixed |  |
| `DataModelSetRows` |  | x |  | x | x | unsynced-only | variable-input-nested-adapted |  |
| `DataModelSetString` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DataModelUnbindEvent` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `DocumentAppendToStyleSheet` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DocumentClose` |  | x |  | x | x | unsynced-only | fixed |  |
| `DocumentCreateElement` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DocumentCreateTextNode` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DocumentGetContext` |  | x |  | x | x | unsynced-only | fixed |  |
| `DocumentGetTitle` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `DocumentGetUrl` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `DocumentHide` |  | x |  | x | x | unsynced-only | fixed |  |
| `DocumentIsModal` |  | x |  | x | x | unsynced-only | fixed |  |
| `DocumentLoadExternalScript` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DocumentLoadInlineScript` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DocumentPullToFront` |  | x |  | x | x | unsynced-only | fixed |  |
| `DocumentPushToBack` |  | x |  | x | x | unsynced-only | fixed |  |
| `DocumentReloadStyleSheet` |  | x |  | x | x | unsynced-only | fixed |  |
| `DocumentSetTitle` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `DocumentShow` |  | x |  | x | x | unsynced-only | fixed-option |  |
| `DocumentUpdateDocument` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementAddEventListener` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `ElementAppendChild` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementArePseudoClassesSet` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementBlur` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementClick` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementClone` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementClosest` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementDispatchEvent` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementFocus` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormControlInputGetSelection` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ElementFormControlInputSelect` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormControlInputSetSelection` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormControlSelectAdd` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormControlSelectRemove` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormControlSelectRemoveAll` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormControlTextAreaGetSelection` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ElementFormControlTextAreaSelect` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormControlTextAreaSetSelection` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementFormSubmit` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementGetActivePseudoClasses` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `ElementGetAttribute` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `ElementGetChild` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementGetClassName` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ElementGetElementById` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementGetElementsByClassName` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `ElementGetElementsByClassNameCount` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementGetElementsByTagName` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `ElementGetElementsByTagNameCount` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementGetId` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ElementGetInnerRml` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ElementGetRect` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementGetScrollLeft` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementGetScrollTop` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementGetTagName` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ElementGetValue` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `ElementHasAttribute` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementHasChildNodes` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementInsertBefore` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementIsClassSet` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementIsPointWithinElement` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementIsPseudoClassSet` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementIsVisible` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementMatches` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementProcessDefaultAction` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementQuerySelector` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementQuerySelectorAll` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `ElementQuerySelectorAllCount` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementRemoveAttribute` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementRemoveChild` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementRemoveEventListener` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementReplaceChild` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementScrollIntoView` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementSetAttribute` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementSetClass` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementSetClassName` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementSetId` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementSetInnerRml` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementSetPseudoClass` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementSetScrollLeft` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementSetScrollTop` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementTabSetRemoveTab` |  | x |  | x | x | unsynced-only | fixed |  |
| `ElementTabSetSetPanel` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `ElementTabSetSetTab` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `EventGetCurrent` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventGetCurrentElement` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventGetParameterBool` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `EventGetParameterFloat` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `EventGetParameterInt` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `EventGetParameterString` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `EventGetParameterType` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `EventGetPhase` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventGetTargetElement` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventGetType` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `EventIsImmediatePropagating` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventIsInterruptible` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventIsPropagating` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventListenerOnAttach` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `EventListenerOnDetach` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `EventListenerProcessEvent` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `EventStopImmediatePropagation` |  | x |  | x | x | unsynced-only | fixed |  |
| `EventStopPropagation` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetContext` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetDocumentPathRequests` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetVersion` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `IsReady` |  | x |  | x | x | unsynced-only | fixed |  |
| `LoadFontFace` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `RegiserEventType` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `RegisterEventType` |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |  |
| `RemoveContext` |  | x |  | x | x | unsynced-only | fixed |  |
| `RemoveContextByName` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `SetDebugContext` |  | x |  | x | x | unsynced-only | fixed |  |
| `SetDebugContextByName` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `SetMouseCursorAlias` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `SolLuaDataModelSetDirty` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `Vector2fNew` |  | x |  | x | x | unsynced-only | fixed |  |
| `Vector2iNew` |  | x |  | x | x | unsynced-only | fixed |  |

## `vfs`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AbortDownload` |  |  |  |  | x | unsynced-only | fixed |  |
| `CalculateHash` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `CompressFolder` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `CreateDir` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `DirList` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `DownloadArchive` |  |  |  |  | x | unsynced-only | variable-input-borrowed |  |
| `ExtractModArchiveFile` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `FileExists` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetAllArchives` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetArchiveChecksum` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetArchiveContainingFile` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetArchiveDependencies` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetArchiveInfo` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetArchivePath` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetArchiveReplaces` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetArchives` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetAvailableAIs` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetFileAbsolutePath` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `GetFileInfo` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `GetFileSize` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `GetGames` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetLoadedArchives` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetMapSquareTexture` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `GetMapSquareTextureInfo` | x | x | x | x | x | synced-visible | fixed |  |
| `GetMaps` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetNameFromRapidTag` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |
| `HasArchive` |  | x |  | x | x | unsynced-only | variable-input-borrowed |  |
| `IsDirectory` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `ListDir` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `LoadFile` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `PackF32` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `PackS16` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `PackS32` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `PackS8` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `PackU16` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `PackU32` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `PackU8` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `ReadFile` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `ReadFileAsString` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `ScanAllDirs` |  |  |  |  | x | unsynced-only | fixed |  |
| `SetMapSquareTexture` | x | x | x | x | x | synced-visible | variable-input-borrowed |  |
| `SubDirs` | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |  |
| `UnpackF32` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `UnpackS16` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `UnpackS32` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `UnpackS8` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `UnpackU16` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `UnpackU32` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `UnpackU8` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `UseArchive` |  | x |  | x | x | unsynced-only | handwritten-reviewed |  |
| `ZlibCompress` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |
| `ZlibDecompress` | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |  |

## `unsynced_read`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetActiveCmdDesc` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetActiveCmdDescs` |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |  |
| `GetBoxSelectionByEngine` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetBuildFacing` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetBuildSpacing` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetClipboard` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetCmdDescIndex` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetCustomPaletteColor` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetDrawSelectionInfo` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeaturePaletteIndex` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetGameSecondsInterpolated` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetLastMessagePositions` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetNanoProjectileParams` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetPieceProjectileName` | x | x | x | x | x | synced-visible | variable-output-caller-owned |  |
| `GetPrevFrameSyncChecksum` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetTeamDamageStats` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPaletteIndex` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsUnitAllied` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsUnitSelected` |  | x |  | x | x | unsynced-only | fixed |  |
| `SolveNURBSCurve` |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |  |

## `team_control`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddTeamResource` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `AddTeamResourceExcessStats` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `AssignPlayerToTeam` | x |  | x |  |  | synced-visible | fixed | x |
| `GameOver` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `KillTeam` | x |  | x |  |  | synced-visible | fixed | x |
| `SetAlly` | x |  | x |  |  | synced-visible | fixed | x |
| `SetAllyTeamStartBox` | x |  | x |  |  | synced-visible | fixed | x |
| `SetGlobalLos` | x |  | x |  |  | synced-visible | fixed | x |
| `SetPlayerReadyState` | x |  | x |  |  | synced-visible | fixed | x |
| `SetTeamResource` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetTeamShareLevel` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetTeamStartPosition` | x |  | x |  |  | synced-visible | fixed | x |
| `ShareTeamResource` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `TransferTeamMaxUnits` | x |  | x |  |  | synced-visible | fixed | x |
| `UseTeamResource` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |

## `unit_control`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddObjectDecal` | x |  | x |  |  | synced-visible | fixed | x |
| `AddUnitDamage` | x |  | x |  |  | synced-visible | fixed | x |
| `AddUnitExperience` | x |  | x |  |  | synced-visible | fixed | x |
| `AddUnitImpulse` | x |  | x |  |  | synced-visible | fixed | x |
| `AddUnitResource` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `AddUnitSeismicPing` | x |  | x |  |  | synced-visible | fixed | x |
| `BuggerOff` | x |  | x |  |  | synced-visible | variable-input-borrowed-mixed-fixed | x |
| `ClearUnitGoal` | x |  | x |  |  | synced-visible | fixed | x |
| `CreateUnit` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `DestroyUnit` | x |  | x |  |  | synced-visible | fixed | x |
| `EditUnitCmdDesc` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `ForceUnitCollisionUpdate` | x |  | x |  |  | synced-visible | fixed | x |
| `GetUnitFeatureSeparation` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitLeavesGhost` | x | x | x | x | x | synced-visible | fixed |  |
| `GetUnitPhysicalState` | x |  | x |  |  | synced-visible | fixed |  |
| `GiveOrderArrayToUnit` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `GiveOrderArrayToUnitArray` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `GiveOrderToUnit` | x |  | x |  |  | synced-visible | handwritten-reviewed | x |
| `GiveOrderToUnitArray` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `InsertUnitCmdDesc` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `RemoveObjectDecal` | x |  | x |  |  | synced-visible | fixed | x |
| `RemoveUnitCmdDesc` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFactoryBuggerOff` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitAlwaysVisible` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitArmored` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitBlocking` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitBuildParams` | x |  | x |  |  | synced-visible | variable-input-borrowed-mixed-fixed | x |
| `SetUnitBuildSpeed` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitBuildeeRadius` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitCloak` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitCollisionVolumeData` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitCosts` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitCrashing` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitDirection` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitExperience` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitFlanking` | x |  | x |  |  | synced-visible | variable-input-borrowed-mixed-fixed | x |
| `SetUnitHarvestStorage` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitHeading` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitHeadingAndUpDir` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitHealth` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitLandGoal` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitLeavesGhost` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitLoadingTransport` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitLosMask` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitLosState` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitMass` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitMaxHealth` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitMaxRange` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitMetalExtraction` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitMidAndAimPos` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitMoveGoal` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitNanoPieces` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetUnitNeutral` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPhysicalStateBit` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPhysics` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPieceCollisionVolumeData` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPieceMatrix` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPieceParent` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPieceVisible` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPosErrorParams` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitPosition` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitRadiusAndHeight` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitResourcing` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetUnitRotation` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitSeismicSignature` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitSelectionVolumeData` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitSensorRadius` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetUnitShieldRechargeDelay` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitShieldState` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitSonarStealth` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitStealth` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitStockpile` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitStorage` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetUnitTarget` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitTooltip` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetUnitUseAirLos` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitUseWeapons` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitVelocity` | x |  | x |  |  | synced-visible | fixed | x |
| `SetUnitWeaponDamages` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetUnitWeaponState` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `TransferUnit` | x |  | x |  |  | synced-visible | fixed | x |
| `UnitAttach` | x |  | x |  |  | synced-visible | fixed | x |
| `UnitDetach` | x |  | x |  |  | synced-visible | fixed | x |
| `UnitDetachFromAir` | x |  | x |  |  | synced-visible | fixed | x |
| `UnitFinishCommand` | x |  | x |  |  | synced-visible | fixed | x |
| `UnitWeaponFire` | x |  | x |  |  | synced-visible | fixed | x |
| `UnitWeaponHoldFire` | x |  | x |  |  | synced-visible | fixed | x |
| `UseUnitResource` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |

## `feature_control`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddFeatureDamage` | x |  | x |  |  | synced-visible | fixed | x |
| `CreateFeature` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `CreateFeatureWreck` | x |  | x |  |  | synced-visible | fixed | x |
| `CreateUnitWreck` | x |  | x |  |  | synced-visible | fixed | x |
| `DestroyFeature` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureAlwaysVisible` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureBlocking` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureCollisionVolumeData` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureDirection` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureFireTime` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureHeadingAndUpDir` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureHealth` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureMass` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureMaxHealth` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureMidAndAimPos` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureMoveCtrl` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureNoSelect` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeaturePhysics` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeaturePieceCollisionVolumeData` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeaturePieceMatrix` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeaturePieceVisible` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeaturePosition` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureRadiusAndHeight` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureReclaim` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureResources` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureResurrect` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `SetFeatureRotation` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureSelectionVolumeData` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureSmokeTime` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureUseAirLos` | x |  | x |  |  | synced-visible | fixed | x |
| `SetFeatureVelocity` | x |  | x |  |  | synced-visible | fixed | x |
| `TransferFeature` | x |  | x |  |  | synced-visible | fixed | x |

## `terrain_control`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddGrass` | x |  | x |  |  | synced-visible | fixed | x |
| `AddHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `AddOriginalHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `AddSmoothMesh` | x |  | x |  |  | synced-visible | fixed | x |
| `AdjustHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `AdjustOriginalHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `AdjustSmoothMesh` | x |  | x |  |  | synced-visible | fixed | x |
| `LevelHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `LevelOriginalHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `LevelSmoothMesh` | x |  | x |  |  | synced-visible | fixed | x |
| `RebuildSmoothMesh` | x |  | x |  |  | synced-visible | fixed | x |
| `RemoveGrass` | x |  | x |  |  | synced-visible | fixed | x |
| `RevertHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `RevertOriginalHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `RevertSmoothMesh` | x |  | x |  |  | synced-visible | fixed | x |
| `SetHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `SetHeightMapFunc` | x |  | x |  |  | synced-visible | handwritten-reviewed | x |
| `SetMapSquareTerrainType` | x |  | x |  |  | synced-visible | fixed | x |
| `SetOriginalHeightMap` | x |  | x |  |  | synced-visible | fixed | x |
| `SetOriginalHeightMapFunc` | x |  | x |  |  | synced-visible | handwritten-reviewed | x |
| `SetSmoothMesh` | x |  | x |  |  | synced-visible | fixed | x |
| `SetSmoothMeshFunc` | x |  | x |  |  | synced-visible | handwritten-reviewed | x |
| `SetTerrainTypeData` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetTidal` | x |  | x |  |  | synced-visible | fixed | x |
| `SetWind` | x |  | x |  |  | synced-visible | fixed | x |

## `projectile_control`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DeleteProjectile` | x |  | x |  |  | synced-visible | fixed | x |
| `SetPieceProjectileParams` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileAlwaysVisible` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileCEG` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetProjectileCollision` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileDamages` | x |  | x |  |  | synced-visible | variable-input-borrowed | x |
| `SetProjectileGravity` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileIgnoreTrackingError` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileIsIntercepted` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileMoveControl` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectilePosition` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileTarget` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileTimeToLive` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileUseAirLos` | x |  | x |  |  | synced-visible | fixed | x |
| `SetProjectileVelocity` | x |  | x |  |  | synced-visible | fixed | x |
| `SpawnProjectile` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |

## `effects_control`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `SpawnCEG` | x |  | x |  |  | synced-visible | variable-input-nested-adapted | x |
| `SpawnExplosion` | x |  | x |  |  | synced-visible | fixed | x |
| `SpawnSFX` | x |  | x |  |  | synced-visible | fixed | x |

## `game_config`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `SetCheatingEnabled` | x |  | x |  |  | synced-visible | fixed | x |
| `SetExperienceGrade` | x |  | x |  |  | synced-visible | fixed | x |
| `SetGodMode` | x |  | x |  |  | synced-visible | fixed | x |
| `SetNoPause` | x |  | x |  |  | synced-visible | fixed | x |
| `SetRadarErrorParams` | x |  | x |  |  | synced-visible | fixed | x |
| `SetSquareBuildingMask` | x |  | x |  |  | synced-visible | fixed | x |

## `cob_script`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `CallCOBScript` | x |  | x |  |  | synced-visible | handwritten-reviewed | x |
| `GetCOBScriptID` | x |  | x |  |  | synced-visible | variable-input-borrowed |  |

## `unit_rendering`

| callout | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetCameraRotation` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetCameraVectors` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetFeaturesInScreenRectangle` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetFrustumPlanes` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitAlwaysUpdateMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitDrawFlag` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitEngineDrawMask` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitIcon` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetUnitIconData` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetUnitLuaDraw` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitNoDraw` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitNoGroup` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitNoMinimap` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitNoSelect` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitSelectionVolumeData` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitTransformMatrix` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitViewPosition` |  | x |  | x | x | unsynced-only | fixed |  |
| `GetUnitsInScreenRectangle` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetVisibleFeatures` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetVisibleProjectiles` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `GetVisibleUnits` |  | x |  | x | x | unsynced-only | variable-output-caller-owned |  |
| `IsUnitIcon` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsUnitInView` |  | x |  | x | x | unsynced-only | fixed |  |
| `IsUnitVisible` |  | x |  | x | x | unsynced-only | fixed |  |

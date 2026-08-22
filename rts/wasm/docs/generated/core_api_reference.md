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

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetAllUnits` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetClosestEnemyUnit` | f32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetRenderUnits` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetRenderUnitsDrawFlagChanged` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetTeamUnitCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetTeamUnitDefCount` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetTeamUnits` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetTeamUnitsByDefs` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetTeamUnitsCounts` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetTeamUnitsSorted` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitArrayCentroid` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitMapCentroid` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitNearestAlly` | i32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitNearestEnemy` | i32,f32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitSeparation` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitsInBox` | f32,f32,f32,f32,f32,f32,i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitsInCylinder` | f32,f32,f32,i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitsInPlanes` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetUnitsInRectangle` | f32,f32,f32,f32,i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitsInSphere` | f32,f32,f32,f32,i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `ValidUnitID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `units_info`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClearUnitsPreviousDrawFlag` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitAllyTeam` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitArmored` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitBasePosition` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitBlocking` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitBuildFacing` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitBuildParams` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitBuildeeRadius` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitCollisionVolumeData` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitCostTable` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitCosts` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitCrashing` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitCurrentBuildPower` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitDefID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitDirection` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitEffectiveBuildRange` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitExperience` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitFlanking` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitHarvestStorage` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitHeading` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitHealth` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitHeight` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitInBuildStance` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitIsActive` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitIsBeingBuilt` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitIsBuilding` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitIsCloaked` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitIsDead` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitIsStunned` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitIsTransporting` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitLastAttackedPiece` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitLastAttacker` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed-option |
| `GetUnitLosState` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitMass` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitMetalExtraction` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitMoveDefID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitNanoPieces` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitNeutral` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPieceCollisionVolumeData` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPosErrorParams` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPosition` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitRadius` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitResources` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitRotation` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitSeismicSignature` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitSelfDTime` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitSensorRadius` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitShieldState` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed-option |
| `GetUnitStates` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitStockpile` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed-option |
| `GetUnitStorage` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitTeam` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitTooltip` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetUnitTransporter` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitVectors` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitVelocity` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWorkerTask` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |

## `units_weapons`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetUnitMaxRange` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponCanFire` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponDamages` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitWeaponHaveFreeLineOfFire` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponState` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitWeaponTarget` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponTestRange` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponTestTarget` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponTryTarget` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitWeaponVectors` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |

## `units_commands`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `FindUnitCmdDesc` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetCommandParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-input-nested-adapted |
| `GetCommandQueue` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFactoryBuggerOff` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFactoryCommandCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFactoryCommands` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFactoryCounts` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFullBuildQueue` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetRealBuildQueue` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetUnitCmdDescs` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitCommandCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitCommands` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitCurrentCommand` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GiveOrder` | i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | handwritten-reviewed |
| `GiveOrderArrayToUnitMap` | i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `GiveOrderToUnitMap` | i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | handwritten-reviewed |

## `units_pieces`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetFeaturePieceDirection` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeaturePieceInfo` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeaturePieceList` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeaturePieceMap` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeaturePieceMatrix` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeaturePiecePosDir` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeaturePiecePosition` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureRootPiece` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetModelPieceList` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetModelPieceMap` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetModelRootPiece` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitPieceDirection` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPieceInfo` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitPieceList` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitPieceMap` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitPieceMatrix` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPiecePosDir` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPiecePosition` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitRootPiece` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitScriptNames` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitScriptPiece` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `teams`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ArePlayersAllied` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `AreTeamsAllied` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetAIInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetAllyTeamInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetAllyTeamList` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetPlayerControlledUnit` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetPlayerInfo` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetPlayerList` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetPlayerListInAllyTeam` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetPlayerListInTeam` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetTeamAllyTeamID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetTeamInfo` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetTeamList` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetTeamLuaAI` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetTeamMaxUnits` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetTeamResourceStats` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetTeamResources` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetTeamStatsHistory` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetTeamUnitStats` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |

## `features`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClearFeaturesPreviousDrawFlag` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetAllFeatures` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetFeatureAllyTeam` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureAlwaysUpdateMatrix` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeatureBlocking` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureCollisionVolumeData` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureDefID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureDirection` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureDrawFlag` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeatureEngineDrawMask` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeatureFireTime` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureHeading` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureHealth` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureHeight` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureLastAttackedPiece` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeatureLuaDraw` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeatureMass` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureNoDraw` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeatureNoSelect` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeaturePieceCollisionVolumeData` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeaturePosition` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeaturePositionExt` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureRadius` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureResources` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureResurrect` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeatureRotation` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureSelectionVolumeData` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeatureSeparation` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureSmokeTime` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureTeam` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureTransformMatrix` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeatureVelocity` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFeaturesInCylinder` | f32,f32,f32,f32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetFeaturesInRectangle` | f32,f32,f32,f32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetFeaturesInSphere` | f32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetRenderFeatures` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetRenderFeaturesDrawFlagChanged` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ValidFeatureID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `projectiles`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetAllProjectiles` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetPieceProjectileParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetProjectileAllyTeamID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileDamages` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetProjectileDefID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileDirection` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileGravity` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileIsIntercepted` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileOwnerID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectilePosition` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileTarget` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileTeamID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileTimeToLive` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileType` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectileVelocity` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetProjectilesInRectangle` | f32,f32,f32,f32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetProjectilesInSphere` | f32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |

## `los`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetClosestValidPosition` | i32,f32,f32,f32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetPositionLosState` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetRadarErrorParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `IsPosInAirLos` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsPosInLos` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsPosInRadar` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsUnitInAirLos` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsUnitInJammer` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsUnitInLos` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsUnitInRadar` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `unit_defs`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetUnitDefByID` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitDefClassify` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitDefCosts` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitDefCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitDefCustomParam` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `GetUnitDefCustomParamKeys` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitDefHealth` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitDefHumanName` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitDefIDByName` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitDefIDs` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetUnitDefName` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetUnitDefParamBool` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitDefParamFloat` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitDefParamInt` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitDefParamKeys` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitDefParamString` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `GetUnitDefParamType` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetUnitDefSpeed` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `ValidUnitDefID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `feature_defs`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetFeatureDefByID` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeatureDefCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureDefCustomParam` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `GetFeatureDefCustomParamKeys` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeatureDefEnergy` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureDefIDByName` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetFeatureDefIDs` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetFeatureDefMetal` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetFeatureDefName` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `ValidFeatureDefID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `weapon_defs`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetWeaponDefByID` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetWeaponDefCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetWeaponDefCustomParam` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `GetWeaponDefCustomParamKeys` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetWeaponDefDamage` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetWeaponDefID` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetWeaponDefIDs` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetWeaponDefName` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetWeaponDefRange` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `ValidWeaponDefID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `game`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AreHelperAIsEnabled` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `FixedAllies` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetAllyTeamStartBox` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetFacingFromHeading` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetGaiaTeamID` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetGameFrame` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetGameMapInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetGameModInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetGameRulesInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetGameRulesResourceInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetGameSeconds` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetGameSetupInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetGlobalLos` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetHeadingFromFacing` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetHeadingFromVector` | f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetMapOption` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `GetMapOptions` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetMapStartPositions` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetModOption` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `GetModOptions` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetSideData` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetSideDataByIndex` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetSideDataCount` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetTeamStartPosition` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetTidal` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetVectorFromHeading` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetWind` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `IsCheatingEnabled` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsDevLuaEnabled` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsEditDefsEnabled` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsGameOver` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsGodModeEnabled` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `IsNoCostEnabled` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `terrain`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetGrass` | f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetGroundBlocked` | f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetGroundExtremes` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetGroundHeight` | f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetGroundInfo` | f32,f32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetGroundNormal` | f32,f32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetGroundOrigHeight` | f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetHeightMapSize` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetSmoothMeshHeight` | f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetTerrainTypeData` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetWaterLevel` | f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetWaterPlaneLevel` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `IsPosInMap` | f32,f32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |

## `player`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetLocalAllyTeamID` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetLocalPlayerID` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetLocalTeamID` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetPlayerRoster` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetPlayerStatistics` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetPlayerTraffic` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetSpectatingState` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |

## `math_extra`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `BitAnd` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `BitBits` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `BitInv` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `BitOr` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `BitXor` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Clamp` | f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Diag` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `Erf` | f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Hypot` | f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Mix` | f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Normalize` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `Round` | f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Sgn` | f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SmoothStep` | f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |

## `encoding`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DecodeBase64` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `DecodeBase64Url` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `EncodeBase64` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `EncodeBase64Url` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `IsValidBase64` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `IsValidBase64Url` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |

## `metal_map`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetMetalAmount` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetMetalExtraction` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetMetalMapSize` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `SetMetalAmount` | i32,i32,f32->i32 | x |  | x |  |  | synced-visible | fixed |

## `path_finder`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DeletePath` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `FreePathNodeCostsArray` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetNextWayPoint` | i32,f32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed-option |
| `GetPathNodeCost` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetPathNodeCosts` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetPathWayPoints` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `InitPathNodeCostsArray` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `RequestPath` | i32,f32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |
| `SetPathNodeCost` | i32,i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetPathNodeCosts` | i32->i64 | x |  | x |  |  | synced-visible | fixed |

## `platform`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetArchitecture` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `IsHeadless` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `rules_params`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetFeatureRulesParam` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFeatureRulesParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetGameRulesParam` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetGameRulesParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetPlayerRulesParam` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetPlayerRulesParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetTeamRulesParam` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetTeamRulesParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitRulesParam` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetUnitRulesParams` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `SetFeatureRulesParam` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `SetGameRulesParam` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `SetPlayerRulesParam` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `SetTeamRulesParam` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `SetUnitRulesParam` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |

## `move_ctrl`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetUnitEstimatedPath` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetUnitMoveTypeData` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `IsMoveCtrlEnabled` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `MoveCtrl` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetMoveCtrlGravity` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |

## `camera`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetCameraDirection` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetCameraFOV` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetCameraNames` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetCameraPosition` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetCameraState` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetPixelDir` | f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SetCameraState` | f32,f32,f32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-nested-adapted |
| `SetCameraTarget` | i32->i64 | x | x | x | x | x | synced-visible | fixed-option |
| `TraceScreenRay` | f32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `WorldToScreenCoords` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |

## `input`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetActionHotKeys` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetActiveCommand` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetActivePage` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetDefaultCommand` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetInvertQueueKey` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetKeyBindings` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetKeyCode` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetKeyFromScanSymbol` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetKeyState` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetKeySymbol` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetModKeyState` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetMouseButtonsPressed` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetMouseCursor` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetMouseStartPosition` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetMouseState` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetPressedKeys` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetPressedScans` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetScanSymbol` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetSelectionBox` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `IsAboveMiniMap` | f32,f32->i64 |  | x |  | x | x | unsynced-only | fixed |

## `debug_input`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClearEmulatedInput` | i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `EmulateKey` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `EmulateMouseButton` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `EmulateMouseMove` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `EmulateMouseWheel` | f32->i32 | x | x | x | x | x | synced-visible | fixed |
| `EmulateTextEditing` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `EmulateTextInput` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |

## `display`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetDrawFrame` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetDualViewGeometry` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetFPS` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetFrameTimeOffset` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetGameSpeed` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetLastUpdateSeconds` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetLosViewColors` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetMapDrawMode` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetMiniMapDualScreen` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetMiniMapGeometry` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetMiniMapRotation` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetNumDisplays` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetScreenGeometry` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetTeamColor` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetTeamOrigColor` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetViewGeometry` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetWaterMode` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetWindowGeometry` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `HaveAdvShading` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `HaveShadows` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `IsAABBInView` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `IsGUIHidden` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `IsSphereInView` | f32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `SetTeamColor` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `selection`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DeselectUnit` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `DeselectUnitArray` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetGroupList` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetGroupUnits` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetGroupUnitsCount` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroupUnitsCounts` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetGroupUnitsSorted` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetSelectedGroup` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetSelectedUnits` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetSelectedUnitsCount` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetSelectedUnitsCounts` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetSelectedUnitsSorted` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetUnitGroup` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `SelectUnit` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SelectUnitArray` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetUnitGroup` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `sound`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetSoundDevices` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetSoundEffectParams` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetSoundStreamTime` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `LoadSoundDef` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `PauseSoundStream` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `PlaySoundFile` | f32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |
| `PlaySoundStream` | f32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `PreloadSoundItem` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetSoundEffectParams` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-nested-adapted |
| `SetSoundStreamVolume` | f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `StopSoundStream` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `messages`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `Echo` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetConsoleBuffer` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetCurrentTooltip` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `IsUserWriting` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `Log` | i32,i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendAllyChat` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendCommands` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendLuaGaiaMsg` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendLuaMenuMsg` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendLuaRulesMsg` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendLuaUIMsg` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendMessage` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendMessageToAllyTeam` | i32,i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendMessageToPlayer` | i32,i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendMessageToSpectators` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendMessageToTeam` | i32,i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendPrivateChat` | i32,i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendPublicChat` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendSkirmishAIMessage` | i32,i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendSpectatorChat` | i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `SendToUnsynced` | i32->i64 | x |  | x |  |  | synced-visible | handwritten-reviewed |

## `config`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetConfigFloat` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `GetConfigInt` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `GetConfigParams` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetConfigString` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetLogSections` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `SetConfigFloat` | f32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetConfigInt` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetConfigString` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetLogSectionFilterLevel` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |

## `tracing`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `TraceRay` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `TraceRayBetweenPositions` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `TraceRayFeatures` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `TraceRayGroundBetweenPositions` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed-option |
| `TraceRayGroundInDirection` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed-option |
| `TraceRayInDirection` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `TraceRayUnits` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |

## `utils`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ClosestBuildPos` | i32,i32,f32,i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetCEGID` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetFeatureDefDimensions` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitDefDimensions` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `Pos2BuildPos` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `TestBuildOrder` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `TestMoveOrder` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `unsynced_ctrl`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AssignMouseCursor` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `DeselectUnitMap` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `DrawUnitCommands` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `ForceLayoutUpdate` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `ForceTesselationUpdate` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetWaterTexture` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `LoadCmdColorsConfig` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `LoadCtrlPanelConfig` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `LoadModelTextures` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `PauseDollyCamera` | f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `PreloadFeatureDefModel` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `PreloadUnitDefModel` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `ReplaceMouseCursor` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `ResumeDollyCamera` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `RunDollyCamera` | f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SDLSetTextInputRect` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SDLStartTextInput` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SDLStopTextInput` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SelectUnitMap` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetActiveCommand` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetAtmosphere` | i32->i64 | x | x | x | x | x | synced-visible | fixed-option |
| `SetAutoShowMetal` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetBoxSelectionByEngine` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetBuildFacing` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetBuildSpacing` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetCameraOffset` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetClipboard` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetCustomCommandDrawData` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-nested-adapted |
| `SetCustomPaletteColor` | i32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDollyCameraCurve` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-adapted |
| `SetDollyCameraLookCurve` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-adapted |
| `SetDollyCameraLookPosition` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDollyCameraLookUnit` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDollyCameraMode` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDollyCameraPosition` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDollyCameraRelativeMode` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDrawGround` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDrawGroundDeferred` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `SetDrawModelsDeferred` | i32,i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `SetDrawSelectionInfo` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDrawSky` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetDrawWater` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetEngineBuildSquareRendering` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetFeatureAlwaysUpdateMatrix` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetFeatureEngineDrawMask` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetFeatureFade` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetFeatureNoDraw` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetFeaturePaletteIndex` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetLastMessagePosition` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetLosViewColors` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetMapRenderingParams` | i32->i64 | x | x | x | x | x | synced-visible | fixed-option |
| `SetMapShader` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetMapShadingTexture` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetMiniMapRotation` | f32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `SetMouseCursor` | f32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetNanoProjectileParams` | f32,f32,f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetShockFrontFactors` | i32->i64 | x | x | x | x | x | synced-visible | fixed-option |
| `SetSkyBoxTexture` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetSunDirection` | f32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetSunLighting` | i32->i64 | x | x | x | x | x | synced-visible | fixed-option |
| `SetUnitAlwaysUpdateMatrix` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitDefIcon` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetUnitDefImage` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetUnitEngineDrawMask` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitIcon` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetUnitIconDraw` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitLeaveTracks` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitNoDraw` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitNoGroup` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitNoMinimap` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitNoSelect` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetUnitPaletteIndex` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetVideoCapturingMode` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetVideoCapturingTimeOffset` | f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetWMCaption` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetWMIcon` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetWaterParams` | i32->i64 | x | x | x | x | x | synced-visible | fixed-option |
| `SetWaterTexture` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetWindowGeometry` | i32,i32,i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetWindowMaximized` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetWindowMinimized` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `WarpMouse` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `gfx`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `ActiveFBO` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `ActiveShader` | i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `ActiveTexture` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `AddAtlasTexture` | i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `AddFallbackFont` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `AddFeatureDefsToSubmissionVAO` | i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `AddFeaturesToSubmissionVAO` | i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `AddUnitDefsToSubmissionVAO` | i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `AddUnitsToSubmissionVAO` | i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `AlphaTest` | i32,i32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `AlphaToCoverage` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `AttachIndexBufferVAO` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `AttachInstanceBufferVAO` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `AttachVertexBufferVAO` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `BeginEnd` | i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `BeginText` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Billboard` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `BindBufferRangeVBO` | i32,i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `BindImageTexture` | i32,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `BindTexture` | i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `BlendEquation` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `BlendEquationSeparate` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `BlendFunc` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `BlendFuncSeparate` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Blending` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `BlitFBO` | i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `CallList` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ChangeTextureParams` | i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `Clear` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ClearAttachmentFBO` | i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ClearFallbackFonts` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ClearSubmissionVAO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ClearVBO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ClipDistance` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ClipPlane` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Color` | f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ColorMask` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ConfigMiniMap` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `CopyToTexture` | i32,i32,i32,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `CopyToVBO` | i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `CreateFBO` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-nested-adapted |
| `CreateList` | i32->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `CreateQuery` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `CreateRBO` | i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `CreateShader` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `CreateTexture` | i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `CreateTextureAtlas` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `Culling` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DefineVBO` | i32,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-adapted |
| `DeleteFBO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DeleteFont` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DeleteList` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DeleteQuery` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DeleteRBO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DeleteShader` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DeleteTexture` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DeleteTextureAtlas` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DeleteTextureFBO` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DeleteVAO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DeleteVBO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DepthClamp` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DepthMask` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DepthTest` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DispatchCompute` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DownloadVBO` | i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `DrawArraysVAO` | i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DrawElementsVAO` | i32,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DrawFuncAtUnit` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `DrawGroundCircle` | f32,i32,i32,f32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DrawGroundQuad` | f32,f32,f32,f32,i32,f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DrawListAtUnit` | i32,i32,i32,f32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DrawMiniMap` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DumpDefinitionVBO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EdgeFlag` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EndText` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Feature` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeatureMultMatrix` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeaturePiece` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeaturePieceMatrix` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeaturePieceMultMatrix` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeatureRaw` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeatureShape` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeatureShapeTextures` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FeatureTextures` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FinalizeTextureAtlas` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `Finish` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Flush` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Fog` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FogCoord` | f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontBegin` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontBindTexture` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontEnd` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontGetTextHeight` | i32,f32,f32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `FontGetTextWidth` | i32,f32,f32,f32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `FontPrint` | i32,f32,f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `FontPrintWorld` | i32,f32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `FontSetAutoOutlineColor` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontSetOutlineColor` | i32,f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontSetTextColor` | i32,f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontSubmitBuffered` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `FontWrapText` | i32,f32,f32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `Frustum` | f32,f32,f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GenerateMipmap` | i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetActiveUniforms` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetAtlasTexture` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetAtmosphere` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetConsoleCommands` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetEngineAtlasTextures` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetEngineModelUniformDataDef` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetEngineModelUniformDataSize` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetEngineTextureNames` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetEngineUniformBufferDef` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetFixedState` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetFontInfo` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetGlobalTexCoords` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetGlobalTexNames` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetIDVBO` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetMapRendering` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetMatrixData` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetNumber` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetQuery` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetRBOInfo` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetScreenViewTrans` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetShaderLog` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetShadowMapParams` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetString` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetSubroutineIndex` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetSun` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetTextHeight` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetTextWidth` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetUniformLocation` | i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetVAO` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetVBO` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetVBOInfo` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetViewRange` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetViewSizes` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetWaterRendering` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `HasExtension` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `InstanceDataFromFeatureDefsVBO` | i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `InstanceDataFromFeaturesVBO` | i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `InstanceDataFromUnitDefsVBO` | i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `InstanceDataFromUnitsVBO` | i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `IsValidFBO` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Light` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Lighting` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `LineStipple` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `LineWidth` | f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `LoadFont` | i32,i32,f32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `LoadIdentity` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `LoadMatrix` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `LogicOp` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Material` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `MatrixDataFromProjectilesVBO` | i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `MatrixMode` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `MemoryBarrier` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ModelsVBO` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `MultMatrix` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `MultiTexCoord` | i32,f32,f32,f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `MultiTexEnv` | i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `MultiTexGen` | i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Normal` | f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ObjectLabel` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `Ortho` | f32,f32,f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PointParameter` | i32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PointSize` | f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PointSprite` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PolygonMode` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PolygonOffset` | f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PopAttrib` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PopDebugGroup` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PopMatrix` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PushAttrib` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PushDebugGroup` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `PushMatrix` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `PushPopMatrix` | i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `RawBindFBO` | i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ReadPixels` | i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `Rect` | f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `RemoveFromSubmissionVAO` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `RenderToTexture` | i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `ResetMatrices` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ResetState` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Rotate` | f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `RunQuery` | i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `SaveImage` | i32,i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `Scale` | f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Scissor` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SecondaryColor` | f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SetFBOAttachment` | i32,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `SetFBODrawBuffers` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `SetFBOReadBuffer` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SetFeatureBufferUniforms` | i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `SetGeometryShaderParameter` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SetTesselationShaderParameter` | i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SetUnitBufferUniforms` | i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ShadeModel` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Shape` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-adapted |
| `SlaveMiniMap` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `StencilFunc` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `StencilFuncSeparate` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `StencilMask` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `StencilMaskSeparate` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `StencilOp` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `StencilOpSeparate` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `StencilTest` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SubmitVAO` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SwapBuffers` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `TexCoord` | f32,f32,f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `TexEnv` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `TexGen` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `TexRect` | f32,f32,f32,f32,f32,f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Text` | f32,f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `TextEnv` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `TextureInfo` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `Translate` | f32,f32,f32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnbindBufferRangeVBO` | i32,i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `Uniform` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UniformArrayFloat` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `UniformArrayInt` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `UniformInt` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UniformMatrix` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `UniformSubroutine` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Unit` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitMultMatrix` | i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitPiece` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitPieceMatrix` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitPieceMultMatrix` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitRaw` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitShape` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitShapeTextures` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnitTextures` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `UnsafeState` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `UploadTexture` | i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `UploadVBO` | i32,i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `UseShader` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `Vertex` | f32,f32,f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Viewport` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |

## `lights`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddLightTrackingTarget` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `AddMapLight` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `AddModelLight` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetMapLightTrackingState` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetModelLightTrackingState` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `UpdateMapLight` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `UpdateModelLight` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `icons`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddUnitIcon` | f32,f32,i32,f32,f32,f32,f32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `FreeUnitIcon` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetAllIconDataArray` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetIconData` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `UnitIconGetDraw` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `UnitIconSetDraw` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `markers`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddWorldIcon` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `AddWorldText` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |
| `AddWorldUnit` | i32,i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `MarkerAddLine` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `MarkerAddPoint` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed-mixed-fixed |
| `MarkerErasePosition` | f32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `ground_decals`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `CreateGroundDecal` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `DestroyGroundDecal` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetAllGroundDecals` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetGroundDecalAlpha` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalCreationFrame` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalGlowParams` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalMiddlePos` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalMisc` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalNormal` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalOwner` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalQuadPos` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalRotation` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalSizeAndHeight` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalTexture` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetGroundDecalTextureParams` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalTextures` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetGroundDecalTint` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGroundDecalType` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetGroundDecalUserData` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `SetGroundDecalAlpha` | i32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalCreationFrame` | i32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalGlowParams` | i32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalMisc` | i32,f32,f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalNormal` | i32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalPosAndDims` | i32,f32,f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalQuadPosAndHeight` | i32,f32,f32,f32,f32,f32,f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalRotation` | i32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalTexture` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetGroundDecalTextureParams` | i32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalTint` | i32,f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `SetGroundDecalUserData` | i32,i32,f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |

## `system_control`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `CallAsTeam` | i32,i32->i64 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `ClearWatchDogTimer` | i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GarbageCollectCtrl` | i32,i32,i32,i32,f32,f32,f32,f32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetGameName` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetGameState` | f32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGatherMode` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetMenuName` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetReplayFilePath` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetReplayLength` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetReplayRecordingFilePath` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetVideoCapturingMode` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetWindowDisplayMode` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `IsReplay` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `Ping` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Quit` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Reload` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `RequestStartPosition` | i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `Restart` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SetShareLevel` | f32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `ShareResources` | i32,f32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `Start` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `Yield` | i32->i64 | x | x | x | x | x | synced-visible | fixed |

## `profiling`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DiffTimers` | i64,i64,i32->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `GetDrawSeconds` | i32->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `GetFrameTimer` | i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `GetLuaMemUsage` | i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `GetProfilerRecordNames` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetProfilerTimeRecord` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetSyncedGCInfo` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetTimer` | i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `GetTimerMicros` | i32,i32->i32 | x | x | x | x | x | synced-visible | handwritten-reviewed |
| `GetVidMemUsage` | i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |

## `rml_ui`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddTranslationString` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ClearDocumentPathRequests` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ClearTranslations` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextActivateTheme` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextAddEventListener` | i64,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `ContextCreateDataModel` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextCreateDocument` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextEnableMouseCursor` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextGetDensityIndependentPixelRatio` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextGetDimensions` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ContextGetDocument` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextGetElementAtPoint` | i64,f32,f32,i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ContextGetFocusElement` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ContextGetHoverElement` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ContextGetName` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ContextGetRootElement` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ContextIsMouseInteracting` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextIsThemeActive` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextLoadDocument` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextOpenDataModel` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextProcessKeyDown` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextProcessKeyUp` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextProcessMouseButtonDown` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextProcessMouseButtonUp` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextProcessMouseLeave` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextProcessMouseMove` | i64,f32,f32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextProcessMouseWheel` | i64,f32,f32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextProcessTextInput` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextPullDocumentToFront` | i64,i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextPullToFront` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextPushDocumentToBack` | i64,i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextRemoveDataModel` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextRemoveEventListener` | i64,i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ContextRender` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextSetDensityIndependentPixelRatio` | i64,f32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextSetDimensions` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextSetPointerCapture` | i64,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextTakePointerCaptureDelta` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ContextUnloadAllDocuments` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextUnloadDocument` | i64,i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ContextUpdate` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `CreateContext` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelBindBool` | i64,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelBindColor` | i64,i32,i32,i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelBindEvent` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `DataModelBindFloat` | i64,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelBindInt` | i64,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelBindPercent` | i64,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelBindPixels` | i64,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelBindRows` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-nested-adapted |
| `DataModelBindString` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelGetBool` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelGetColor` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelGetFloat` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelGetInt` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelGetPercent` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelGetPixels` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelGetString` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `DataModelSetBool` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelSetColor` | i64,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelSetFloat` | i64,f32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelSetInt` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelSetPercent` | i64,f32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelSetPixels` | i64,f32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DataModelSetRows` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-nested-adapted |
| `DataModelSetString` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DataModelUnbindEvent` | i64->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `DocumentAppendToStyleSheet` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DocumentClose` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DocumentCreateElement` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DocumentCreateTextNode` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DocumentGetContext` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `DocumentGetTitle` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `DocumentGetUrl` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `DocumentHide` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DocumentIsModal` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DocumentLoadExternalScript` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DocumentLoadInlineScript` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DocumentPullToFront` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DocumentPushToBack` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DocumentReloadStyleSheet` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `DocumentSetTitle` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `DocumentShow` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed-option |
| `DocumentUpdateDocument` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementAddEventListener` | i64,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `ElementAppendChild` | i64,i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ElementArePseudoClassesSet` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementBlur` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementClick` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementClone` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ElementClosest` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementDispatchEvent` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementFocus` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormControlInputGetSelection` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ElementFormControlInputSelect` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormControlInputSetSelection` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormControlSelectAdd` | i64,i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormControlSelectRemove` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormControlSelectRemoveAll` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormControlTextAreaGetSelection` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ElementFormControlTextAreaSelect` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormControlTextAreaSetSelection` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementFormSubmit` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementGetActivePseudoClasses` | i64,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `ElementGetAttribute` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `ElementGetChild` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ElementGetClassName` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ElementGetElementById` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementGetElementsByClassName` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `ElementGetElementsByClassNameCount` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementGetElementsByTagName` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `ElementGetElementsByTagNameCount` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementGetId` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ElementGetInnerRml` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ElementGetRect` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ElementGetScrollLeft` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementGetScrollTop` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementGetTagName` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ElementGetValue` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `ElementHasAttribute` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementHasChildNodes` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementInsertBefore` | i64,i64,i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ElementIsClassSet` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementIsPointWithinElement` | i64,f32,f32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementIsPseudoClassSet` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementIsVisible` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementMatches` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementProcessDefaultAction` | i64,i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementQuerySelector` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementQuerySelectorAll` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `ElementQuerySelectorAllCount` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementRemoveAttribute` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementRemoveChild` | i64,i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ElementRemoveEventListener` | i64,i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementReplaceChild` | i64,i64,i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `ElementScrollIntoView` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementSetAttribute` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementSetClass` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementSetClassName` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementSetId` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementSetInnerRml` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementSetPseudoClass` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementSetScrollLeft` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementSetScrollTop` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementTabSetRemoveTab` | i64,i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `ElementTabSetSetPanel` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `ElementTabSetSetTab` | i64,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `EventGetCurrent` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EventGetCurrentElement` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EventGetParameterBool` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `EventGetParameterFloat` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `EventGetParameterInt` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `EventGetParameterString` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `EventGetParameterType` | i64,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `EventGetPhase` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EventGetTargetElement` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EventGetType` | i64,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `EventIsImmediatePropagating` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EventIsInterruptible` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EventIsPropagating` | i64,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `EventListenerOnAttach` | i64,i64->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `EventListenerOnDetach` | i64,i64->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `EventListenerProcessEvent` | i64,i64->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `EventStopImmediatePropagation` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `EventStopPropagation` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetContext` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetDocumentPathRequests` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetVersion` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `IsReady` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `LoadFontFace` | i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `RegiserEventType` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `RegisterEventType` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed-mixed-fixed |
| `RemoveContext` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `RemoveContextByName` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `SetDebugContext` | i64->i64 |  | x |  | x | x | unsynced-only | fixed |
| `SetDebugContextByName` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `SetMouseCursorAlias` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `SolLuaDataModelSetDirty` | i64,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `Vector2fNew` | f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `Vector2iNew` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |

## `vfs`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AbortDownload` | i32->i64 |  |  |  |  | x | unsynced-only | fixed |
| `CalculateHash` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `CompressFolder` | i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `CreateDir` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `DirList` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `DownloadArchive` | i32->i32 |  |  |  |  | x | unsynced-only | variable-input-borrowed |
| `ExtractModArchiveFile` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `FileExists` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetAllArchives` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetArchiveChecksum` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetArchiveContainingFile` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetArchiveDependencies` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetArchiveInfo` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetArchivePath` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetArchiveReplaces` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetArchives` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetAvailableAIs` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetFileAbsolutePath` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `GetFileInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `GetFileSize` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `GetGames` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetLoadedArchives` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetMapSquareTexture` | i32,i32,i32,i32,i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `GetMapSquareTextureInfo` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetMaps` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetNameFromRapidTag` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |
| `HasArchive` | i32->i64 |  | x |  | x | x | unsynced-only | variable-input-borrowed |
| `IsDirectory` | i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `ListDir` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `LoadFile` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `PackF32` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `PackS16` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `PackS32` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `PackS8` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `PackU16` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `PackU32` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `PackU8` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `ReadFile` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `ReadFileAsString` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `ScanAllDirs` | i32->i32 |  |  |  |  | x | unsynced-only | fixed |
| `SetMapSquareTexture` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | variable-input-borrowed |
| `SubDirs` | i32,i32,i32->i32 | x | x | x | x | x | synced-visible | dynamic-output-caller-owned |
| `UnpackF32` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `UnpackS16` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `UnpackS32` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `UnpackS8` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `UnpackU16` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `UnpackU32` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `UnpackU8` | i32,i32,i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `UseArchive` | i32->i64 |  | x |  | x | x | unsynced-only | handwritten-reviewed |
| `ZlibCompress` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |
| `ZlibDecompress` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-io-borrowed-input-caller-owned-output |

## `unsynced_read`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetActiveCmdDesc` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetActiveCmdDescs` | i32,i32->i32 |  | x |  | x | x | unsynced-only | dynamic-output-caller-owned |
| `GetBoxSelectionByEngine` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetBuildFacing` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetBuildSpacing` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetClipboard` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetCmdDescIndex` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetCustomPaletteColor` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetDrawSelectionInfo` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeaturePaletteIndex` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetGameSecondsInterpolated` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetLastMessagePositions` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetNanoProjectileParams` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetPieceProjectileName` | i32,i32->i32 | x | x | x | x | x | synced-visible | variable-output-caller-owned |
| `GetPrevFrameSyncChecksum` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetTeamDamageStats` | i32,i32->i32 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPaletteIndex` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `IsUnitAllied` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `IsUnitSelected` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `SolveNURBSCurve` | i32,i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-io-borrowed-input-caller-owned-output |

## `team_control`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddTeamResource` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `AddTeamResourceExcessStats` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `AssignPlayerToTeam` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `GameOver` | i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `KillTeam` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetAlly` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetAllyTeamStartBox` | i32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetGlobalLos` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetPlayerReadyState` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetTeamResource` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetTeamShareLevel` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetTeamStartPosition` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `ShareTeamResource` | i32,i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `TransferTeamMaxUnits` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UseTeamResource` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |

## `unit_control`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddObjectDecal` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AddUnitDamage` | i32,f32,f32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AddUnitExperience` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AddUnitImpulse` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AddUnitResource` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `AddUnitSeismicPing` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `BuggerOff` | f32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed-mixed-fixed |
| `ClearUnitGoal` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `CreateUnit` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `DestroyUnit` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `EditUnitCmdDesc` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `ForceUnitCollisionUpdate` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `GetUnitFeatureSeparation` | i32,i32,i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitLeavesGhost` | i32->i64 | x | x | x | x | x | synced-visible | fixed |
| `GetUnitPhysicalState` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `GiveOrderArrayToUnit` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `GiveOrderArrayToUnitArray` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `GiveOrderToUnit` | i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | handwritten-reviewed |
| `GiveOrderToUnitArray` | i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `InsertUnitCmdDesc` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `RemoveObjectDecal` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `RemoveUnitCmdDesc` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFactoryBuggerOff` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitAlwaysVisible` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitArmored` | i32,i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitBlocking` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitBuildParams` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed-mixed-fixed |
| `SetUnitBuildSpeed` | i32,f32,f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitBuildeeRadius` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitCloak` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitCollisionVolumeData` | i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitCosts` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitCrashing` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitDirection` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitExperience` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitFlanking` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed-mixed-fixed |
| `SetUnitHarvestStorage` | i32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitHeading` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitHeadingAndUpDir` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitHealth` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitLandGoal` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitLeavesGhost` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitLoadingTransport` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitLosMask` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitLosState` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitMass` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitMaxHealth` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitMaxRange` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitMetalExtraction` | i32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitMidAndAimPos` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitMoveGoal` | i32,f32,f32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitNanoPieces` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetUnitNeutral` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPhysicalStateBit` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPhysics` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPieceCollisionVolumeData` | i32,i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPieceMatrix` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPieceParent` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPieceVisible` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPosErrorParams` | i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitPosition` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitRadiusAndHeight` | i32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitResourcing` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetUnitRotation` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitSeismicSignature` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitSelectionVolumeData` | i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitSensorRadius` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetUnitShieldRechargeDelay` | i32,i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitShieldState` | i32,i32,i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitSonarStealth` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitStealth` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitStockpile` | i32,i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitStorage` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetUnitTarget` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitTooltip` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetUnitUseAirLos` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitUseWeapons` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitVelocity` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetUnitWeaponDamages` | i32,i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetUnitWeaponState` | i32,i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `TransferUnit` | i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UnitAttach` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UnitDetach` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UnitDetachFromAir` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UnitFinishCommand` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UnitWeaponFire` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UnitWeaponHoldFire` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `UseUnitResource` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |

## `feature_control`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddFeatureDamage` | i32,f32,f32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `CreateFeature` | i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `CreateFeatureWreck` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `CreateUnitWreck` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `DestroyFeature` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureAlwaysVisible` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureBlocking` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureCollisionVolumeData` | i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureDirection` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureFireTime` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureHeadingAndUpDir` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureHealth` | i32,f32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureMass` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureMaxHealth` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureMidAndAimPos` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureMoveCtrl` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureNoSelect` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeaturePhysics` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeaturePieceCollisionVolumeData` | i32,i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeaturePieceMatrix` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeaturePieceVisible` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeaturePosition` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureRadiusAndHeight` | i32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureReclaim` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureResources` | i32,f32,f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureResurrect` | i32,i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `SetFeatureRotation` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureSelectionVolumeData` | i32,i32,i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureSmokeTime` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureUseAirLos` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetFeatureVelocity` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `TransferFeature` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |

## `terrain_control`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `AddGrass` | f32,f32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AddHeightMap` | f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AddOriginalHeightMap` | f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AddSmoothMesh` | f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AdjustHeightMap` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AdjustOriginalHeightMap` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `AdjustSmoothMesh` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `LevelHeightMap` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `LevelOriginalHeightMap` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `LevelSmoothMesh` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `RebuildSmoothMesh` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `RemoveGrass` | f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `RevertHeightMap` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `RevertOriginalHeightMap` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `RevertSmoothMesh` | f32,f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetHeightMap` | f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetHeightMapFunc` | i32->i64 | x |  | x |  |  | synced-visible | handwritten-reviewed |
| `SetMapSquareTerrainType` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetOriginalHeightMap` | f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetOriginalHeightMapFunc` | i32->i64 | x |  | x |  |  | synced-visible | handwritten-reviewed |
| `SetSmoothMesh` | f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetSmoothMeshFunc` | i32->i64 | x |  | x |  |  | synced-visible | handwritten-reviewed |
| `SetTerrainTypeData` | i32,f32,f32,f32,f32,f32,i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetTidal` | f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetWind` | f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |

## `projectile_control`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `DeleteProjectile` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetPieceProjectileParams` | i32,i32,f32,f32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileAlwaysVisible` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileCEG` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetProjectileCollision` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileDamages` | i32,i32,f32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |
| `SetProjectileGravity` | i32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileIgnoreTrackingError` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileIsIntercepted` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileMoveControl` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectilePosition` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileTarget` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileTimeToLive` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileUseAirLos` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetProjectileVelocity` | i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SpawnProjectile` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |

## `effects_control`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `SpawnCEG` | f32,f32,f32,i32,i32->i32 | x |  | x |  |  | synced-visible | variable-input-nested-adapted |
| `SpawnExplosion` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SpawnSFX` | i32,i32,f32,f32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |

## `game_config`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `SetCheatingEnabled` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetExperienceGrade` | f32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetGodMode` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetNoPause` | i32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetRadarErrorParams` | i32,f32,f32,f32->i64 | x |  | x |  |  | synced-visible | fixed |
| `SetSquareBuildingMask` | i32,i32,i32->i64 | x |  | x |  |  | synced-visible | fixed |

## `cob_script`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `CallCOBScript` | i32,i32,i32,i32->i32 | x |  | x |  |  | synced-visible | handwritten-reviewed |
| `GetCOBScriptID` | i32,i32->i64 | x |  | x |  |  | synced-visible | variable-input-borrowed |

## `unit_rendering`

| callout | signature | rules-synced | rules-unsynced | gaia-synced | gaia-unsynced | ui | sync | transport | mutating |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | ---: |
| `GetCameraRotation` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetCameraVectors` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetFeaturesInScreenRectangle` | f32,f32,f32,f32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetFrustumPlanes` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitAlwaysUpdateMatrix` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitDrawFlag` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitEngineDrawMask` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitIcon` | i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetUnitIconData` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetUnitLuaDraw` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitNoDraw` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitNoGroup` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitNoMinimap` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitNoSelect` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitSelectionVolumeData` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitTransformMatrix` | i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitViewPosition` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | fixed |
| `GetUnitsInScreenRectangle` | f32,f32,f32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetVisibleFeatures` | i32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetVisibleProjectiles` | i32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `GetVisibleUnits` | i32,f32,i32,i32->i32 |  | x |  | x | x | unsynced-only | variable-output-caller-owned |
| `IsUnitIcon` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `IsUnitInView` | i32->i64 |  | x |  | x | x | unsynced-only | fixed |
| `IsUnitVisible` | i32,f32,i32->i64 |  | x |  | x | x | unsynced-only | fixed |

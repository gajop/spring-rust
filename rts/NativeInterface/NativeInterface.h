/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>

// Include all API implementations
// - Callouts: Module calls Spring (Query/Result pattern - e.g., GetUnitPosition)
// - Callins: Spring calls Module (Event pattern - e.g., UnitCreated)
#include "NativeInterface/api/Camera.h"
#include "NativeInterface/api/Callins.h"
#include "NativeInterface/api/Config.h"
#include "NativeInterface/api/Display.h"
#include "NativeInterface/api/FeatureDefs.h"
#include "NativeInterface/api/Features.h"
#include "NativeInterface/api/Game.h"
#include "NativeInterface/api/Gfx.h"
#include "NativeInterface/api/Input.h"
#include "NativeInterface/api/LOS.h"
#include "NativeInterface/api/MathExtra.h"
#include "NativeInterface/api/Memory.h"
#include "NativeInterface/api/Messages.h"
#include "NativeInterface/api/MetalMap.h"
#include "NativeInterface/api/MoveCtrl.h"
#include "NativeInterface/api/PathFinder.h"
#include "NativeInterface/api/Platform.h"
#include "NativeInterface/api/Player.h"
#include "NativeInterface/api/Projectiles.h"
#include "NativeInterface/api/RulesParams.h"
#include "NativeInterface/api/RmlUi.h"
#include "NativeInterface/api/Selection.h"
#include "NativeInterface/api/Sound.h"
#include "NativeInterface/api/SyncedCtrl.h"
#include "NativeInterface/api/Teams.h"
#include "NativeInterface/api/Terrain.h"
#include "NativeInterface/api/Tracing.h"
#include "NativeInterface/api/UnitDefs.h"
#include "NativeInterface/api/UnitsCommands.h"
#include "NativeInterface/api/UnitsInfo.h"
#include "NativeInterface/api/UnitsPieces.h"
#include "NativeInterface/api/UnitsQuery.h"
#include "NativeInterface/api/UnitsWeapons.h"
#include "NativeInterface/api/UnsyncedCtrl.h"
#include "NativeInterface/api/UnsyncedRead.h"
#include "NativeInterface/api/Utils.h"
#include "NativeInterface/api/VFS.h"
#include "NativeInterface/api/WeaponDefs.h"
#include "NativeInterface/api/Lights.h"
#include "NativeInterface/api/Icons.h"
#include "NativeInterface/api/Markers.h"
#include "NativeInterface/api/GroundDecals.h"
#include "NativeInterface/api/SystemControl.h"
#include "NativeInterface/api/Profiling.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * NativeInterface - C ABI boundary for native module integration
 *
 * This structure provides a stable C interface that can be consumed
 * by native modules written in any language with C FFI support
 * (Rust, C, C++, Zig, etc.).
 *
 * Version information is passed via InitializeNativeModuleQuery and
 * modules export their version via the NativeModuleApiVersion symbol.
 */
struct NativeInterface {
	const MemoryApi* memory;
	const GameApi* game;
	const TerrainApi* terrain;
	const TeamsApi* teams;
	const UnitsQueryApi* unitsQuery;
	const UnitsInfoApi* unitsInfo;
	const UnitsWeaponsApi* unitsWeapons;
	const UnitsCommandsApi* unitsCommands;
	const UnitsPiecesApi* unitsPieces;
	const FeaturesApi* features;
	const ProjectilesApi* projectiles;
	const LOSApi* los;
	const UnitDefsApi* unitDefs;
	const FeatureDefsApi* featureDefs;
	const WeaponDefsApi* weaponDefs;
	const MetalMapApi* metalMap;
	const PathFinderApi* pathFinder;
	const PlatformApi* platform;
	const RulesParamsApi* rulesParams;
	const RmlUiApi* rmlUi;
	const MathExtraApi* mathExtra;
	const MoveCtrlApi* moveCtrl;
	const SyncedCtrlApi* syncedCtrl;
	const CameraApi* cameraApi;
	const InputApi* input;
	const DisplayApi* display;
	const SelectionApi* selection;
	const VFSApi* vfs;
	const SoundApi* soundApi;
	const MessagesApi* messages;
	const ConfigApi* config;
	const TracingApi* tracing;
	const UtilsApi* utils;
	const PlayerApi* player;
	const UnsyncedCtrlApi* unsyncedCtrl;
	const UnsyncedReadApi* unsyncedRead;
	const LightsApi* lights;
	const IconsApi* icons;
	const MarkersApi* markers;
	const GroundDecalsApi* groundDecals;
	const SystemControlApi* systemControl;
	const ProfilingApi* profiling;
	const GfxApi* gfx;
};

#ifdef __cplusplus
}
#endif

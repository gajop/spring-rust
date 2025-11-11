#include "RustSystem.h"

#include <type_traits>
#include <utility>
#include <filesystem>

#include "spring-rust-plugin.h"

#include "Game/Rust/api/Memory.h"
#include "Game/Rust/api/Game.h"
#include "Game/Rust/api/Terrain.h"
#include "Game/Rust/api/Teams.h"
#include "Game/Rust/api/UnitsQuery.h"
#include "Game/Rust/api/UnitsInfo.h"
#include "Game/Rust/api/UnitsWeapons.h"
#include "Game/Rust/api/UnitsCommands.h"
#include "Game/Rust/api/UnitsPieces.h"
#include "Game/Rust/api/Features.h"
#include "Game/Rust/api/Projectiles.h"
#include "Game/Rust/api/LOS.h"
#include "Game/Rust/api/UnitDefs.h"
#include "Game/Rust/api/FeatureDefs.h"
#include "Game/Rust/api/WeaponDefs.h"
#include "Game/Rust/api/MetalMap.h"
#include "Game/Rust/api/PathFinder.h"
#include "Game/Rust/api/RulesParams.h"
#include "Game/Rust/api/MathExtra.h"
#include "Game/Rust/api/MoveCtrl.h"
#include "Game/Rust/api/SyncedCtrl.h"
#include "Game/Rust/api/Camera.h"
#include "Game/Rust/api/Input.h"
#include "Game/Rust/api/Display.h"
#include "Game/Rust/api/Selection.h"
#include "Game/Rust/api/VFS.h"
#include "Game/Rust/api/Sound.h"
#include "Game/Rust/api/Messages.h"
#include "Game/Rust/api/Config.h"
#include "Game/Rust/api/Tracing.h"
#include "Game/Rust/api/Utils.h"
#include "Game/Rust/api/Player.h"

#include "System/Log/ILog.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"

#include "System/EventHandler.h"

// LuaSyncedRead.cpp includes
#include "ExternalAI/SkirmishAIHandler.h"
#include "Game/Game.h"
#include "Game/GameSetup.h"
#include "Game/Camera.h"
#include "Game/GameHelper.h"
#include "Game/GlobalUnsynced.h"
#include "Game/Players/Player.h"
#include "Game/Players/PlayerHandler.h"
#include "Map/Ground.h"
#include "Map/MapDamage.h"
#include "Map/MapInfo.h"
#include "Map/MapParser.h"
#include "Map/MetalMap.h"
#include "Map/ReadMap.h"
#include "Rendering/Env/GrassDrawer.h"
#include "Rendering/Models/IModelParser.h"
#include "Sim/Misc/DamageArrayHandler.h"
#include "Sim/Misc/SideParser.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/CollisionVolume.h"
#include "Sim/Misc/GroundBlockingObjectMap.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/SmoothHeightMesh.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/Wind.h"
#include "Sim/MoveTypes/StrafeAirMoveType.h"
#include "Sim/MoveTypes/GroundMoveType.h"
#include "Sim/MoveTypes/HoverAirMoveType.h"
#include "Sim/MoveTypes/ScriptMoveType.h"
#include "Sim/MoveTypes/StaticMoveType.h"
#include "Sim/MoveTypes/MoveDefHandler.h"
#include "Sim/Path/IPathManager.h"
#include "Sim/Projectiles/ExplosionGenerator.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/PieceProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/UnitLoader.h"
#include "Sim/Units/UnitToolTipMap.hpp"
#include "Sim/Units/UnitTypes/Builder.h"
#include "Sim/Units/UnitTypes/Factory.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Sim/Units/CommandAI/CommandAI.h"
#include "Sim/Units/CommandAI/FactoryCAI.h"
#include "Sim/Units/CommandAI/MobileCAI.h"
#include "Sim/Units/Scripts/UnitScript.h"
#include "Sim/Weapons/PlasmaRepulser.h"
#include "Sim/Weapons/Weapon.h"
#include "Sim/Weapons/WeaponDefHandler.h"
#include "System/MainDefines.h"
#include "System/SpringMath.h"
#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/FileSystem.h"
#include "System/StringUtil.h"

#include <cctype>
#include <type_traits>
//

// SyncedCtrl includes


#include <vector>
#include <algorithm>
#include <cctype>

#include "Game/Game.h"
#include "Game/GameSetup.h"
#include "Game/Camera.h"
#include "Game/GameHelper.h"
#include "Game/SelectedUnitsHandler.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/Players/Player.h"
#include "Net/GameServer.h"
#include "Map/Ground.h"
#include "Map/MapDamage.h"
#include "Map/MapInfo.h"
#include "Map/ReadMap.h"
#include "Rendering/Env/GrassDrawer.h"
#include "Rendering/Env/IGroundDecalDrawer.h"
#include "Rendering/Models/IModelParser.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Features/FeatureDefHandler.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/BuildingMaskMap.h"
#include "Sim/Misc/CollisionVolume.h"
#include "Sim/Misc/DamageArray.h"
#include "Sim/Misc/DamageArrayHandler.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/ModInfo.h"
#include "Sim/Misc/SmoothHeightMesh.h"
#include "Sim/Misc/Team.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/Wind.h"
#include "Sim/MoveTypes/AAirMoveType.h"

#include <vector>
#include <algorithm>
#include <cctype>

#include "Game/Game.h"
#include "Game/GameSetup.h"
#include "Sim/Path/IPathManager.h"
#include "Sim/Projectiles/ExplosionGenerator.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/PieceProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/MissileProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/StarburstProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/TorpedoProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectileFactory.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/UnitLoader.h"
#include "Sim/Units/UnitToolTipMap.hpp"
#include "Sim/Units/Scripts/CobInstance.h"
#include "Sim/Units/Scripts/LuaUnitScript.h"
#include "Sim/Units/UnitTypes/Builder.h"
#include "Sim/Units/UnitTypes/Factory.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandAI.h"
#include "Sim/Units/CommandAI/FactoryCAI.h"
#include "Sim/Units/UnitTypes/ExtractorBuilding.h"
#include "Sim/Weapons/PlasmaRepulser.h"
#include "Sim/Weapons/Weapon.h"
#include "Sim/Weapons/WeaponDefHandler.h"
#include "Sim/Weapons/WeaponTarget.h"
#include "System/EventHandler.h"
#include "System/ObjectDependenceTypes.h"
#include "System/Log/ILog.h"

//


#include <dlfcn.h>

RustSystem* RustSystem::s_instance = nullptr;

RustSystem::RustSystem() :
  CEventClient("[RustSystem]", 23253, false),
  m_NativeInterface {
    .f_HandleRustPanic = [](NativeInterface* ptr) {
        return ptr->bridge->HandleRustPanic();
    },

    .f_IsPosInMap = [](NativeInterface* ptr, float x, float z) {
      return ptr->bridge->IsPosInMap(x, z);
    },
    .f_GetGroundHeight = [](NativeInterface* ptr, float x, float z){
      return ptr->bridge->GetGroundHeight(x, z);
    },
    .f_GetGroundOrigHeight = [](NativeInterface* ptr, float x, float z){
      return ptr->bridge->GetGroundOrigHeight(x, z);
    },
    .f_GetGroundNormal = [](NativeInterface* ptr, const float x, const float z, bool *normal){
      return ptr->bridge->GetGroundNormal(x, z, normal);
    },
    .f_GetGroundInfo = [](NativeInterface* ptr, float x, float z){
      return ptr->bridge->GetGroundInfo(x, z);
    },
    .f_GetGroundBlocked = [](NativeInterface* ptr, float fx1, float fz1, const float *fx2, const float *fz2){
      return ptr->bridge->GetGroundBlocked(fx1, fz1, fx2, fz2);
    },
    .f_GetGroundExtremes = [](NativeInterface* ptr) {
      return ptr->bridge->GetGroundExtremes();
    },
    .f_GetTerrainTypeData = [](NativeInterface* ptr, int terrainTypeInfo){
      return ptr->bridge->GetTerrainTypeData(terrainTypeInfo);
    },
    .f_GetGrass = [](NativeInterface* ptr, float x, float z){
      return ptr->bridge->GetGrass(x, z);
    },
    .f_GetSmoothMeshHeight= [](NativeInterface* ptr, float x, float z){
      return ptr->bridge->GetSmoothMeshHeight(x, z);
    },
    // SyncedCtrl START
    .f_AddHeightMap = [](NativeInterface* ptr, float xl, float zl, float height) {
        return ptr->bridge->AddHeightMap(xl, zl, height);
    },
    .f_SetHeightMap = [](NativeInterface* ptr, float xl, float zl, float height, float* scalingFactor) {
        return ptr->bridge->SetHeightMap(xl, zl, height, scalingFactor);
    },
    .f_SetHeightMapFunc = [](NativeInterface* ptr, void (*function)(void* data), void* data) {
        return ptr->bridge->SetHeightMapFunc(function, data);
    },
    // .f_SetHeightMap = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) {
    //     return ptr->bridge->SetHeightMap(x1, z1, x2, z2, height);
    // },
    // .f_AddHeightMap = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) {
    //     return ptr->bridge->AddHeightMap(x1, z1, x2, z2, height);
    // },
    .f_RevertHeightMap = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height, float origFactor) {
        return ptr->bridge->RevertHeightMap(x1, z1, x2, z2, height, origFactor);
    },
    // .f_SetHeightMapArray = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) {
    //     return ptr->bridge->SetHeightMapArray(x1, z1, x2, z2, height);
    // },
    // .f_AddHeightMapArray = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) {
    //     return ptr->bridge->AddHeightMapArray(x1, z1, x2, z2, height);
    // },

    // .f_SetOriginalHeightMap = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) {
    //     return ptr->bridge->SetOriginalHeightMap(x1, z1, x2, z2, height);
    // },
    // .f_AddOriginalHeightMap = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) {
    //     return ptr->bridge->AddOriginalHeightMap(x1, z1, x2, z2, height);
    // },
    // .f_RevertOriginalHeightMap = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor) {
    //     return ptr->bridge->RevertOriginalHeightMap(x1, z1, x2, z2, origFactor);
    // },
    // .f_SetOriginalHeightMapArray = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) {
    //     return ptr->bridge->SetOriginalHeightMapArray(x1, z1, x2, z2, height);
    // },
    // .f_AddOriginalHeightMapArray = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) {
    //     return ptr->bridge->AddOriginalHeightMapArray(x1, z1, x2, z2, height);
    // },

    .f_SetSmoothMesh = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) {
        return ptr->bridge->SetSmoothMesh(x1, z1, x2, z2, height);
    },
    .f_AddSmoothMesh = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) {
        return ptr->bridge->AddSmoothMesh(x1, z1, x2, z2, height);
    },
    .f_RevertSmoothMesh = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor) {
        return ptr->bridge->RevertSmoothMesh(x1, z1, x2, z2, origFactor);
    },
    .f_SetSmoothMeshArray = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) {
        return ptr->bridge->SetSmoothMeshArray(x1, z1, x2, z2, height);
    },
    .f_AddSmoothMeshArray = [](NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) {
        return ptr->bridge->AddSmoothMeshArray(x1, z1, x2, z2, height);
    },

    .f_SetMapSquareTerrainType = [](NativeInterface* ptr, uint32_t x, uint32_t z, uint32_t newType) {
        return ptr->bridge->SetMapSquareTerrainType(x, z, newType);
    },
    .f_SetTerrainTypeData = [](NativeInterface* ptr, uint32_t typeIndex, float* tankSpeed, float* kbotSpeed, float* hoverSpeed, float* shipSpeed,
      float* hardness, bool* receiveTracks, const char* name) {
        return ptr->bridge->SetTerrainTypeData(typeIndex, tankSpeed, kbotSpeed, hoverSpeed, shipSpeed, hardness, receiveTracks, name);
    },
    // SyncedCtrl END

    .f_Echo = [](NativeInterface* ptr, const char* msg) {
      return ptr->bridge->Echo(msg);
    },
    .f_GameMapX = [](NativeInterface* ptr) {
      return ptr->bridge->GameMapX();
    },
    .f_GameMapY = [](NativeInterface* ptr) {
      return ptr->bridge->GameMapY();
    },
    .f_GameMapSizeX = [](NativeInterface* ptr) {
      return ptr->bridge->GameMapSizeX();
    },
    .f_GameMapSizeZ = [](NativeInterface* ptr) {
      return ptr->bridge->GameMapSizeZ();
    },

    // Memory management
    .memory = &MEMORY_API,

    // Core game state (synced read)
    .game = &GAME_API,
    .terrain = &TERRAIN_API,
    .teams = &TEAMS_API,
    .unitsQuery = &UNITS_QUERY_API,
    .unitsInfo = &UNITS_INFO_API,
    .unitsWeapons = &UNITS_WEAPONS_API,
    .unitsCommands = &UNITS_COMMANDS_API,
    .unitsPieces = &UNITS_PIECES_API,
    .features = &FEATURES_API,
    .projectiles = &PROJECTILES_API,
    .los = &LOS_API,

    // Definitions (static data)
    .unitDefs = &UNIT_DEFS_API,
    .featureDefs = &FEATURE_DEFS_API,
    .weaponDefs = &WEAPON_DEFS_API,

    // Specialized systems
    .metalMap = &METAL_MAP_API,
    .pathFinder = &PATH_FINDER_API,
    .rulesParams = &RULES_PARAMS_API,
    .mathExtra = &MATH_EXTRA_API,
    .moveCtrl = &MOVE_CTRL_API,

    // Control (synced write)
    .syncedCtrl = &SYNCED_CTRL_API,

    // UI/Rendering (unsynced)
    .camera = &CAMERA_API,
    .input = &INPUT_API,
    .display = &DISPLAY_API,
    .selection = &SELECTION_API,

    // System/IO
    .vfs = &VFS_API,
    .sound = &SOUND_API,
    .messages = &MESSAGES_API,
    .config = &CONFIG_API,

    // Utilities
    .tracing = &TRACING_API,
    .utils = &UTILS_API,
    .player = &PLAYER_API,

    .bridge=this
  } {
  LOG("Rust system function initialization...");

  Reload();

  // autoLinkEvents = true;
    // RegisterLinkedEvents(this);
    eventHandler.AddClient(this);

  s_instance = this;
}

#define LOAD_SYMBOL(SymbolName)                                                   \
  {                                                                               \
    m_##SymbolName##FuncPtr = reinterpret_cast<fptr::SymbolName##FuncPtr>(        \
        dlsym(m_handle, #SymbolName));                                            \
    if (m_##SymbolName##FuncPtr == nullptr) {                                     \
      LOG_L(L_ERROR, "Failed to load symbol " #SymbolName ": %s", dlerror());     \
    }                                                                             \
  }

// #define DECLARE_RUST_FUNCTION(FuncName, ...) \
//     void FuncName(__VA_ARGS__) { \
//         CallNativeFunction(&RustSystem::m_##FuncName##FuncPtr, __func__, ##__VA_ARGS__); \
//     }

// If you need to define these functions outside the class:

#define EXTRACT_ARGS(...) __VA_ARGS__

#define DEFINE_RUST_FUNCTION(FuncName, ...) \
    void RustSystem::FuncName(__VA_ARGS__) { \
        CallNativeFunction(&RustSystem::m_##FuncName##FuncPtr, __func__, EXTRACT_ARGS(__VA_ARGS__)); \
    }

#include <iostream>
#include <filesystem>
#include <fstream>
#include <random>
#include <ctime>

namespace fs = std::filesystem;

std::string generate_random_name(int length) {
    const char charset[] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const int charset_size = sizeof(charset) - 1;

    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, charset_size - 1);

    std::string result;
    result.reserve(length);
    for (int i = 0; i < length; ++i) {
        result += charset[dis(gen)];
    }
    return result;
}

std::optional<fs::path> copy_so_to_temp(const std::string& source_path) {
    try {
        // Check if source file exists
        if (!fs::exists(source_path)) {
            std::cerr << "Source file does not exist: " << source_path << std::endl;
            return std::nullopt;
        }

        // Generate random name for the temporary file

        std::string random_name = generate_random_name(10) + ".so";
        fs::path temp_dir = fs::temp_directory_path();
        fs::path dest_path = temp_dir / random_name;

        // Copy the file
        fs::copy_file(source_path, dest_path, fs::copy_options::overwrite_existing);

        std::cout << "File copied successfully to: " << dest_path << std::endl;
        return dest_path;
    } catch (const fs::filesystem_error& e) {
        std::cerr << "Filesystem error: " << e.what() << std::endl;
        return std::nullopt;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return std::nullopt;
    }
}

void RustSystem::Reload() {
  std::string path = "/home/gajop/projects/spring-projects/SBC.sdd/native//target/release/librust_plugin.so";


  if (m_handle != nullptr) {
    const int closed = dlclose(m_handle);
    if (closed != 0) {
      LOG_L(L_ERROR, "Failed to close plugin %s: %s", path.c_str(), dlerror());
    }
    m_handle = nullptr;
    m_data = nullptr;
  }

  // We need to copy the path to a temporary location, so reload works.
  // Otherwise, Linux (and maybe Windows too?) tends to reuse the old file on
  // subequent reloads, making hot reload not work.
  const auto orig_path = path;
  path = copy_so_to_temp(path).value();

  m_handle = dlopen(path.c_str(), RTLD_LAZY);
  // https://stackoverflow.com/a/72639000
  // m_handle = dlopen(path.c_str(), RTLD_LOCAL);
  if (!m_handle) {
    LOG_L(L_ERROR, "Failed to open plugin %s: %s", path.c_str(), dlerror());
    return;
  }

  LOG("Successfully opened plugin %s", orig_path.c_str());
  LOAD_SYMBOL(InitializeNativeModule);
  LOAD_SYMBOL(DownloadFailed);
  LOAD_SYMBOL(DownloadFinished);
  LOAD_SYMBOL(DownloadProgress);
  LOAD_SYMBOL(DownloadQueued);
  LOAD_SYMBOL(DownloadStarted);
  LOAD_SYMBOL(FeatureCreated);
  LOAD_SYMBOL(FeatureDestroyed);
  LOAD_SYMBOL(GameID);
  LOAD_SYMBOL(GamePaused);
  LOAD_SYMBOL(GamePreload);
  LOAD_SYMBOL(GameStart);
  LOAD_SYMBOL(PlayerAdded);
  LOAD_SYMBOL(PlayerChanged);
  LOAD_SYMBOL(PlayerRemoved);
  LOAD_SYMBOL(RenderUnitDestroyed);
  LOAD_SYMBOL(RunDelayedFunctions);
  LOAD_SYMBOL(Shutdown);
  LOAD_SYMBOL(TeamChanged);
  LOAD_SYMBOL(TeamDied);
  LOAD_SYMBOL(UnitCreated);
  LOAD_SYMBOL(UnitDestroyed);
  LOAD_SYMBOL(UnitExperience);
  LOAD_SYMBOL(UnitFinished);
  LOAD_SYMBOL(UnitFromFactory);
  LOAD_SYMBOL(UnitGiven);
  LOAD_SYMBOL(UnitLoaded);
  LOAD_SYMBOL(UnitStunned);
  LOAD_SYMBOL(UnitTaken);
  LOAD_SYMBOL(UnitUnloaded);
  LOAD_SYMBOL(HandleLuaMsg);
  LOAD_SYMBOL(HandleLuaCall);

  LOG("Initializing native module...");
  m_data = m_InitializeNativeModuleFuncPtr(&m_NativeInterface);
}

void RustSystem::DownloadFailed(int ID, int errorID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadFailedFuncPtr(&m_NativeInterface, m_data, ID, errorID);
}

void RustSystem::DownloadFinished(int ID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadFinishedFuncPtr(&m_NativeInterface, m_data, ID);
}

void RustSystem::DownloadProgress(int ID, long downloaded, long total) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadProgressFuncPtr(&m_NativeInterface, m_data, ID, downloaded, total);
}

void RustSystem::DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadQueuedFuncPtr(&m_NativeInterface, m_data, ID, archiveName.c_str(), archiveType.c_str());
}

void RustSystem::DownloadStarted(int ID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadStartedFuncPtr(&m_NativeInterface, m_data, ID);
}

void RustSystem::FeatureCreated(const CFeature* feature) {
  LOG_L(L_DEBUG, "NativeModule::%s ID: %d", __func__, feature->id);
  m_FeatureCreatedFuncPtr(&m_NativeInterface, m_data, feature->id);
}

void RustSystem::FeatureDestroyed(const CFeature* feature) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_FeatureDestroyedFuncPtr(&m_NativeInterface, m_data, feature->id);
}

void RustSystem::GameID(const unsigned char* gameID, unsigned int numBytes) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GameIDFuncPtr(&m_NativeInterface, m_data, gameID, numBytes);
}

void RustSystem::GamePaused(int playerID, bool paused) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GamePausedFuncPtr(&m_NativeInterface, m_data, playerID, paused);
}

void RustSystem::GamePreload() {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GamePreloadFuncPtr(&m_NativeInterface);
}

void RustSystem::GameStart() {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GameStartFuncPtr(&m_NativeInterface);
}

void RustSystem::PlayerAdded(int playerID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_PlayerAddedFuncPtr(&m_NativeInterface, m_data, playerID);
}

void RustSystem::PlayerChanged(int playerID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_PlayerChangedFuncPtr(&m_NativeInterface, m_data, playerID);
}

void RustSystem::PlayerRemoved(int playerID, int reason) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_PlayerRemovedFuncPtr(&m_NativeInterface, m_data, playerID, reason);
}

void RustSystem::RenderUnitDestroyed(const CUnit* unit) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_RenderUnitDestroyedFuncPtr(&m_NativeInterface, m_data, unit->id);
}

void RustSystem::TeamChanged(int teamID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_TeamChangedFuncPtr(&m_NativeInterface, m_data, teamID);
}

void RustSystem::TeamDied(int teamID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_TeamDiedFuncPtr(&m_NativeInterface, m_data, teamID);
}

void RustSystem::UnitCreated(const CUnit* unit, const CUnit* builder) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitCreatedFuncPtr(&m_NativeInterface, m_data, unit->id, builder != nullptr ? builder->id : -1);
}

void RustSystem::UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitDestroyedFuncPtr(&m_NativeInterface, m_data, unit->id, attacker != nullptr ? attacker->id : -1);
}

void RustSystem::UnitExperience(const CUnit* unit, float oldExperience) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitExperienceFuncPtr(&m_NativeInterface, m_data, unit->id, oldExperience);
}

void RustSystem::UnitFinished(const CUnit *unit) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitFinishedFuncPtr(&m_NativeInterface, m_data, unit->id);
}

void RustSystem::UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitFromFactoryFuncPtr(&m_NativeInterface, m_data, unit->id, factory->id, userOrders);
}

void RustSystem::UnitGiven(const CUnit* unit, int oldTeam, int newTeam) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitGivenFuncPtr(&m_NativeInterface, m_data, unit->id, oldTeam, newTeam);
}

void RustSystem::UnitLoaded(const CUnit* unit, const CUnit* transport) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitLoadedFuncPtr(&m_NativeInterface, m_data, unit->id, transport->id);
}

void RustSystem::UnitStunned(const CUnit* unit, bool stunned) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitStunnedFuncPtr(&m_NativeInterface, m_data, unit->id, stunned);
}

void RustSystem::UnitTaken(const CUnit* unit, int oldTeam, int newTeam) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitTakenFuncPtr(&m_NativeInterface, m_data, unit->id, oldTeam, newTeam);
}

void RustSystem::UnitUnloaded(const CUnit* unit, const CUnit* transport) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitUnloadedFuncPtr(&m_NativeInterface, m_data, unit->id, transport->id);
}

void RustSystem::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data)
{
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_HandleLuaMsgFuncPtr(&m_NativeInterface, m_data, playerID, script, mode, data.data(), data.size());
}

void RustSystem::HandleLuaCall(const char* msg)
{
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_HandleLuaCallFuncPtr(&m_NativeInterface, m_data, msg);
}

bool isSynced = true;

void RustSystem::HandleRustPanic() {
    LOG_L(L_WARNING, "Rust module panicked. Reloading...");
    Reload();
}

IsPosInMapReturn RustSystem::IsPosInMap(float x, float z) {
    return {};
}

float RustSystem::GetGroundHeight(float x, float z) {
  return CGround::GetHeightReal(x, z, isSynced);
}

float RustSystem::GetGroundOrigHeight(float x, float z) {
    return CGround::GetOrigHeight(x, z);
}

bool OptBool(bool* b, bool def) {
  return b != nullptr ? *b: def;
}

GetGroundNormalReturn RustSystem::GetGroundNormal(const float x, const float z, bool *isNormal) {
    // raw or smoothed center normal
    const float3& normal = OptBool(isNormal, false) ?
        CGround::GetNormal(x, z, isSynced):
        CGround::GetSmoothNormal(x, z, isSynced);

  return GetGroundNormalReturn {
    .x = normal.x,
    .y = normal.y,
    .z = normal.z,
    // slope derives from face normals, include it here
    .slope = CGround::GetSlope(x, z, isSynced)
  };
}

GetGroundInfoReturn RustSystem::GetGroundInfo(float x, float z) {
  return {};
}

bool RustSystem::GetGroundBlocked(float fx1, float fz1, const float *fx2, const float *fz2) {
  return {};
}

GetGroundExtremesReturn RustSystem::GetGroundExtremes() {
  return GetGroundExtremesReturn {
    .initMinHeight = readMap->GetInitMinHeight(),
    .initMaxHeight = readMap->GetInitMaxHeight(),
    .currMinHeight = readMap->GetCurrMinHeight(),
    .currMaxHeight = readMap->GetCurrMaxHeight()
  };
}

GetTerrainTypeDataReturn RustSystem::GetTerrainTypeData(int terrainTypeInfo) {
return {};
}

float RustSystem::GetGrass(float x, float z) {
    const float3 pos(x, 0.0f, z);
    return grassDrawer->GetGrass(pos.cClampInBounds());
}

float RustSystem::GetSmoothMeshHeight(float x, float z) {
  return {};
}

// SyncedCtrl START

/******************************************************************************
 * Heightmap
 * @section heightmap
 * Note that x & z coords are in worldspace (Game.mapSizeX/Z), still the heightmap resolution is Game.squareSize.
******************************************************************************/


/*** Can only be called in `Spring.SetHeightMapFunc`
 *
 * @function Spring.AddHeightMap
 * @number x
 * @number z
 * @number height
 * @treturn ?nil|number newHeight
 */
float RustSystem::AddHeightMap(float xl, float zl, float height)
{
	if (!m_heightMapChange.inHeightMap) {
    LOG_L(L_ERROR, "AddHeightMap() can only be called in SetHeightMapFunc()");
    return 0;
	}

	// quantize
	const int x = (int)(xl / SQUARE_SIZE);
	const int z = (int)(zl / SQUARE_SIZE);

	// discard invalid coordinates
	if ((x < 0) || (x > mapDims.mapx) ||
	    (z < 0) || (z > mapDims.mapy)) {
		return 0;
	}

	const int index = (z * mapDims.mapxp1) + x;
	const float oldHeight = readMap->GetCornerHeightMapSynced()[index];
	m_heightMapChange.amountChanged += math::fabsf(height);

	// update RecalcArea()
	if (x < m_heightMapChange.x1) { m_heightMapChange.x1 = x; }
	if (x > m_heightMapChange.x2) { m_heightMapChange.x2 = x; }
	if (z < m_heightMapChange.z1) { m_heightMapChange.z1 = z; }
	if (z > m_heightMapChange.z2) { m_heightMapChange.z2 = z; }

	readMap->AddHeight(index, height);
	// push the new height
  return oldHeight + height;
}


/***
 *
 * @function Spring.SetHeightMap
 *
 * Can only be called in `Spring.SetHeightMapFunc`. The terraform argument is
 *
 * @number x
 * @number z
 * @number height
 * @number[opt=1] terraform a scaling factor.
 * @treturn ?nil|number absHeightDiff =0 nothing will be changed (the terraform starts) and if =1 the terraform will be finished.
 *
 */
float RustSystem::SetHeightMap(float xl, float zl, float height, float* scalingFactor)
{
	if (!m_heightMapChange.inHeightMap) {
    LOG_L(L_ERROR, "SetHeightMap() can only be called in SetHeightMapFunc()");
    return 0;
	}

	// quantize
	const int x = (int)(xl / SQUARE_SIZE);
	const int z = (int)(zl / SQUARE_SIZE);

	// discard invalid coordinates
	if ((x < 0) || (x > mapDims.mapx) ||
	    (z < 0) || (z > mapDims.mapy)) {
		return 0;
	}

	const int index = (z * mapDims.mapxp1) + x;
	const float oldHeight = readMap->GetCornerHeightMapSynced()[index];
	float newHeight = oldHeight;

  if (scalingFactor != nullptr) {
		newHeight += (height - oldHeight) * *scalingFactor;
  } else {
		newHeight = height;
	}

	const float heightDiff = (newHeight - oldHeight);
	m_heightMapChange.amountChanged += math::fabsf(heightDiff);

	// update RecalcArea()
	if (x > m_heightMapChange.x2) { m_heightMapChange.x2 = x; }
	if (x < m_heightMapChange.x1) { m_heightMapChange.x1 = x; }
	if (z < m_heightMapChange.z1) { m_heightMapChange.z1 = z; }
	if (z > m_heightMapChange.z2) { m_heightMapChange.z2 = z; }

	readMap->SetHeight(index, newHeight);

  return heightDiff;
}

/***
 * @function Spring.SetHeightMapFunc
 *
 * Example code:
 *
 *     function Spring.SetHeightMapFunc(function()
 *       for z=0,Game.mapSizeZ, Game.squareSize do
 *         for x=0,Game.mapSizeX, Game.squareSize do
 *           Spring.SetHeightMap( x, z, 200 + 20 * math.cos((x + z) / 90) )
 *         end
 *       end
 *     end)
 *
 * @func lua_function
 * @param[opt] arg1
 * @param[opt] arg2
 * @param[opt] argn
 * @treturn ?nil|number absTotalHeightMapAmountChanged
 */
float RustSystem::SetHeightMapFunc(void (*function)(void* data), void* data)
{
	if (mapDamage->Disabled()) {
		return 0.0f;
	}

	if (m_heightMapChange.inHeightMap) {
		LOG_L(L_ERROR, "SetHeightMapFunc() recursion is not permitted");
    return 0.0f;
	}

	m_heightMapChange.x1 = mapDims.mapx;
	m_heightMapChange.x2 = -1;
	m_heightMapChange.z1 = mapDims.mapy;
	m_heightMapChange.z2 = 0;
	m_heightMapChange.amountChanged = 0.0f;

	m_heightMapChange.inHeightMap = true;
  function(data);
	m_heightMapChange.inHeightMap = false;


	if (m_heightMapChange.x2 > -1) {
		mapDamage->RecalcArea(m_heightMapChange.x1, m_heightMapChange.x2, m_heightMapChange.z1, m_heightMapChange.z2);
	}

  return m_heightMapChange.amountChanged;
}

// void RustSystem::SetHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height)
// {
//     if (mapDamage->Disabled()) {
//         return;
//     }

//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             readMap->SetHeight((z * mapDims.mapxp1) + x, height);
//         }
//     }

//     mapDamage->RecalcArea(x1, x2, z1, z2);
// }


// void RustSystem::AddHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height)
// {
//     if (mapDamage->Disabled()) {
//         return;
//     }

//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             readMap->AddHeight((z * mapDims.mapxp1) + x, height);
//         }
//     }

//     mapDamage->RecalcArea(x1, x2, z1, z2);
// }


void RustSystem::RevertHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height, float origFactor)
{
    if (mapDamage->Disabled()) {
        return;
    }

    const float* origMap = readMap->GetOriginalHeightMapSynced();
    const float* currMap = readMap->GetCornerHeightMapSynced();

    if (origFactor == 1.0f) {
        for (int z = z1; z <= z2; z++) {
            for (int x = x1; x <= x2; x++) {
                const int idx = (z * mapDims.mapxp1) + x;

                readMap->SetHeight(idx, origMap[idx]);
            }
        }
    }
    else {
        const float currFactor = (1.0f - origFactor);
        for (int z = z1; z <= z2; z++) {
            for (int x = x1; x <= x2; x++) {
                const int index = (z * mapDims.mapxp1) + x;
                const float ofh = origFactor * origMap[index];
                const float cfh = currFactor * currMap[index];
                readMap->SetHeight(index, ofh + cfh);
            }
        }
    }

    mapDamage->RecalcArea(x1, x2, z1, z2);
}


// void RustSystem::SetHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height)
// {
//     if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
//         x1 > x2 || z1 > z2) {
//         // TODO: Figure out how to deal with errors
//         return;
//     }

//     if (mapDamage->Disabled()) {
//         return;
//     }

//     uint32_t arrayIndex = 0;
//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             readMap->SetHeight(z * mapDims.mapxp1 + x, height[arrayIndex]);
//         }
//     }

//     mapDamage->RecalcArea(x1, x2, z1, z2);
// }


// void RustSystem::AddHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height)
// {
//     if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
//         x1 > x2 || z1 > z2) {
//         // TODO: Figure out how to deal with errors
//         return;
//     }

//     if (mapDamage->Disabled()) {
//         return;
//     }

//     uint32_t arrayIndex = 0;
//     for (uint32_t z = z1; z <= z2; z++) {
//         for (uint32_t x = x1; x <= x2; x++) {
//             const uint32_t index = (z * mapDims.mapxp1) + x;
//             readMap->AddHeight(index, height[arrayIndex++]);
//         }
//     }

//     mapDamage->RecalcArea(x1, x2, z1, z2);
// }


/******************************************************************************
 * Height Map/Smooth Mesh
 * @section heightmap
******************************************************************************/

// void RustSystem::SetOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height)
// {
//     if (mapDamage->Disabled()) {
//         return;
//     }

//     if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
//         x1 > x2 || z1 > z2) {
//         // TODO: Figure out how to deal with errors
//         return;
//     }


//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             readMap->SetOriginalHeight((z * mapDims.mapxp1) + x, height);
//         }
//     }
// }


// void RustSystem::AddOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height)
// {
//     if (mapDamage->Disabled()) {
//         return;
//     }

//     if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
//         x1 > x2 || z1 > z2) {
//         // TODO: Figure out how to deal with errors
//         return;
//     }

//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             readMap->AddOriginalHeight((z * mapDims.mapxp1) + x, height);
//         }
//     }
// }


// void RustSystem::RevertOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor)
// {
//     if (mapDamage->Disabled()) {
//         return;
//     }

//     if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
//         x1 > x2 || z1 > z2) {
//         // TODO: Figure out how to deal with errors
//         return;
//     }

//     const float* origMap = readMap->GetMapFileHeightMapSynced();
//     const float* currMap = readMap->GetOriginalHeightMapSynced();

//     if (origFactor == 1.0f) {
//         for (int z = z1; z <= z2; z++) {
//             for (int x = x1; x <= x2; x++) {
//                 const int idx = (z * mapDims.mapxp1) + x;

//                 readMap->SetOriginalHeight(idx, origMap[idx]);
//             }
//         }
//         return;
//     }

//     const float currFactor = 1.0f - origFactor;
//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             const int index = (z * mapDims.mapxp1) + x;
//             const float ofh = origFactor * origMap[index];
//             const float cfh = currFactor * currMap[index];
//             readMap->SetOriginalHeight(index, ofh + cfh);
//         }
//     }
// }

// void RustSystem::SetOriginalHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height)
// {
//     if (mapDamage->Disabled()) {
//         return;
//     }

//     if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
//         x1 > x2 || z1 > z2) {
//         // TODO: Figure out how to deal with errors
//         return;
//     }

//     uint32_t arrayIndex = 0;
//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             readMap->SetOriginalHeight((z * mapDims.mapxp1) + x, height[arrayIndex++]);
//         }
//     }
// }

// void RustSystem::AddOriginalHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height)
// {
//     if (mapDamage->Disabled()) {
//         return;
//     }

//     if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
//         x1 > x2 || z1 > z2) {
//         // TODO: Figure out how to deal with errors
//         return;
//     }

//     uint32_t arrayIndex = 0;
//     for (int z = z1; z <= z2; z++) {
//         for (int x = x1; x <= x2; x++) {
//             readMap->AddOriginalHeight((z * mapDims.mapxp1) + x, height[arrayIndex++]);
//         }
//     }
// }

void RustSystem::SetSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height)
{
    for (int z = z1; z <= z2; z++) {
        for (int x = x1; x <= x2; x++) {
            const int index = (z * smoothGround.GetMaxX()) + x;
            smoothGround.SetHeight(index, height);
        }
    }
}

void RustSystem::AddSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height)
{
    for (int z = z1; z <= z2; z++) {
        for (int x = x1; x <= x2; x++) {
            const int index = (z * smoothGround.GetMaxX()) + x;
            smoothGround.AddHeight(index, height);
        }
    }
}

void RustSystem::RevertSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor)
{
    const float* origMap = smoothGround.GetOriginalMeshData();
    const float* currMap = smoothGround.GetMeshData();

    if (origFactor == 1.0f) {
        for (int z = z1; z <= z2; z++) {
            for (int x = x1; x <= x2; x++) {
                const int idx = (z * smoothGround.GetMaxX()) + x;
                smoothGround.SetHeight(idx, origMap[idx]);
            }
        }
    }
    else {
        const float currFactor = (1.0f - origFactor);
        for (int z = z1; z <= z2; z++) {
            for (int x = x1; x <= x2; x++) {
                const int index = (z * smoothGround.GetMaxX()) + x;
                const float ofh = origFactor * origMap[index];
                const float cfh = currFactor * currMap[index];
                smoothGround.SetHeight(index, ofh + cfh);
            }
        }
    }
}

void RustSystem::SetSmoothMeshArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height)
{
    if (mapDamage->Disabled()) {
        return;
    }

    if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
        x1 > x2 || z1 > z2) {
        // TODO: Figure out how to deal with errors
        return;
    }

    uint32_t arrayIndex = 0;
    for (int z = z1; z <= z2; z++) {
        for (int x = x1; x <= x2; x++) {
            const int index = (z * smoothGround.GetMaxX()) + x;
            smoothGround.SetHeight(index, height[arrayIndex]);
        }
    }
}

void RustSystem::AddSmoothMeshArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height)
{
    if (mapDamage->Disabled()) {
        return;
    }

    if (x2 > mapDims.mapx || z2 > mapDims.mapy ||
        x1 > x2 || z1 > z2) {
        // TODO: Figure out how to deal with errors
        return;
    }

    uint32_t arrayIndex = 0;
    for (int z = z1; z <= z2; z++) {
        for (int x = x1; x <= x2; x++) {
            const int index = (z * smoothGround.GetMaxX()) + x;
            smoothGround.AddHeight(index, height[arrayIndex]);
        }
    }
}


/******************************************************************************
 * TerrainTypes
 * @section terraintypes
******************************************************************************/

void RustSystem::SetMapSquareTerrainType(uint32_t x, uint32_t z, uint32_t newType)
{
    if (x >= mapDims.mapx || z >= mapDims.mapy) {
        return;
    }

    const uint32_t tx = x >> 1;
    const uint32_t tz = z >> 1;

    const uint32_t ott = readMap->GetTypeMapSynced()[tz * mapDims.hmapx + tx];

    readMap->GetTypeMapSynced()[tz * mapDims.hmapx + tx] = std::clamp(newType, 0u, static_cast<uint32_t>(CMapInfo::NUM_TERRAIN_TYPES) - 1);
    pathManager->TerrainChange(x, z, x + 1, z + 1,  TERRAINCHANGE_SQUARE_TYPEMAP_INDEX);
}

bool RustSystem::SetTerrainTypeData(uint32_t typeIndex, float* tankSpeed, float* kbotSpeed, float* hoverSpeed, float* shipSpeed,
    float* hardness, bool* receiveTracks, const char* name)
{
    if (typeIndex < 0 || typeIndex >= CMapInfo::NUM_TERRAIN_TYPES) {
        return false;
    }

    CMapInfo::TerrainType* tt = const_cast<CMapInfo::TerrainType*>(&mapInfo->terrainTypes[typeIndex]);
    const CMapInfo::TerrainType ctt = *tt; // copy

    bool ttSpeedModChanged = false;
    bool ttHardnessChanged = false;

    if (tankSpeed != nullptr) ttSpeedModChanged |= (ctt. tankSpeed != (tt-> tankSpeed = *tankSpeed));
    if (kbotSpeed != nullptr) ttSpeedModChanged |= (ctt. kbotSpeed != (tt-> kbotSpeed = *kbotSpeed));
    if (hoverSpeed != nullptr) ttSpeedModChanged |= (ctt.hoverSpeed != (tt->hoverSpeed = *hoverSpeed));
    if (shipSpeed != nullptr) ttSpeedModChanged |= (ctt. shipSpeed != (tt-> shipSpeed = *shipSpeed));
    if (hardness != nullptr) ttHardnessChanged |= (ctt.  hardness != (tt->  hardness = *hardness));

    if (receiveTracks != nullptr) tt->receiveTracks = *receiveTracks;
    if (name != nullptr) tt->name = name;

    // hardness changes do not require repathing
    if (ttHardnessChanged)
        mapDamage->TerrainTypeHardnessChanged(typeIndex);
    if (ttSpeedModChanged)
        mapDamage->TerrainTypeSpeedModChanged(typeIndex);

    return true;
}


// SyncedCtrl END

// UnsyncedCtrl START
void RustSystem::Echo(const char* msg) const {
  LOG("%s", msg);
}
// UnsyncedCtrl END

// Game constants STAT
float RustSystem::GameMapX() const {
  return mapDims.mapx / 64;
}
float RustSystem::GameMapY() const {
  return mapDims.mapy / 64;
}
float RustSystem::GameMapSizeX() const {
  return mapDims.mapx * SQUARE_SIZE;
}
float RustSystem::GameMapSizeZ() const {
  return mapDims.mapy * SQUARE_SIZE;
}
// Game constants END

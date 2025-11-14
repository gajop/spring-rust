#include "NativeInterface.h"

#include <dlfcn.h>
#include <algorithm>
#include <ctime>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <random>
#include <vector>

#include "NativeInterface/api/Camera.h"
#include "NativeInterface/api/Config.h"
#include "NativeInterface/api/Display.h"
#include "NativeInterface/api/FeatureDefs.h"
#include "NativeInterface/api/Features.h"
#include "NativeInterface/api/Game.h"
#include "NativeInterface/api/Input.h"
#include "NativeInterface/api/LOS.h"
#include "NativeInterface/api/MathExtra.h"
#include "NativeInterface/api/Memory.h"
#include "NativeInterface/api/Messages.h"
#include "NativeInterface/api/MetalMap.h"
#include "NativeInterface/api/MoveCtrl.h"
#include "NativeInterface/api/PathFinder.h"
#include "NativeInterface/api/Player.h"
#include "NativeInterface/api/Projectiles.h"
#include "NativeInterface/api/RulesParams.h"
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
#include "NativeInterface/api/Utils.h"
#include "NativeInterface/api/VFS.h"
#include "NativeInterface/api/WeaponDefs.h"

#include "Sim/Features/Feature.h"
#include "Sim/Units/Unit.h"
#include "System/EventHandler.h"
#include "System/Log/ILog.h"

NativeInterfaceSystem* NativeInterfaceSystem::s_instance = nullptr;

NativeInterfaceSystem::NativeInterfaceSystem() :
  CEventClient("[NativeInterfaceSystem]", 23253, false),
  m_NativeInterface {
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
    .cameraApi = &CAMERA_API,
    .input = &INPUT_API,
    .display = &DISPLAY_API,
    .selection = &SELECTION_API,

    // System/IO
    .vfs = &VFS_API,
    .soundApi = &SOUND_API,
    .messages = &MESSAGES_API,
    .config = &CONFIG_API,

    // Utilities
    .tracing = &TRACING_API,
    .utils = &UTILS_API,
    .player = &PLAYER_API
  } {
  LOG("Native interface system function initialization...");

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

void NativeInterfaceSystem::Reload() {
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

void NativeInterfaceSystem::DownloadFailed(int ID, int errorID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadFailedFuncPtr(&m_NativeInterface, m_data, ID, errorID);
}

void NativeInterfaceSystem::DownloadFinished(int ID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadFinishedFuncPtr(&m_NativeInterface, m_data, ID);
}

void NativeInterfaceSystem::DownloadProgress(int ID, long downloaded, long total) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadProgressFuncPtr(&m_NativeInterface, m_data, ID, downloaded, total);
}

void NativeInterfaceSystem::DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadQueuedFuncPtr(&m_NativeInterface, m_data, ID, archiveName.c_str(), archiveType.c_str());
}

void NativeInterfaceSystem::DownloadStarted(int ID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_DownloadStartedFuncPtr(&m_NativeInterface, m_data, ID);
}

void NativeInterfaceSystem::FeatureCreated(const CFeature* feature) {
  LOG_L(L_DEBUG, "NativeModule::%s ID: %d", __func__, feature->id);
  m_FeatureCreatedFuncPtr(&m_NativeInterface, m_data, feature->id);
}

void NativeInterfaceSystem::FeatureDestroyed(const CFeature* feature) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_FeatureDestroyedFuncPtr(&m_NativeInterface, m_data, feature->id);
}

void NativeInterfaceSystem::GameID(const unsigned char* gameID, unsigned int numBytes) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GameIDFuncPtr(&m_NativeInterface, m_data, gameID, numBytes);
}

void NativeInterfaceSystem::GamePaused(int playerID, bool paused) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GamePausedFuncPtr(&m_NativeInterface, m_data, playerID, paused);
}

void NativeInterfaceSystem::GamePreload() {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GamePreloadFuncPtr(&m_NativeInterface);
}

void NativeInterfaceSystem::GameStart() {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_GameStartFuncPtr(&m_NativeInterface);
}

void NativeInterfaceSystem::PlayerAdded(int playerID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_PlayerAddedFuncPtr(&m_NativeInterface, m_data, playerID);
}

void NativeInterfaceSystem::PlayerChanged(int playerID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_PlayerChangedFuncPtr(&m_NativeInterface, m_data, playerID);
}

void NativeInterfaceSystem::PlayerRemoved(int playerID, int reason) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_PlayerRemovedFuncPtr(&m_NativeInterface, m_data, playerID, reason);
}

void NativeInterfaceSystem::RenderUnitDestroyed(const CUnit* unit) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_RenderUnitDestroyedFuncPtr(&m_NativeInterface, m_data, unit->id);
}

void NativeInterfaceSystem::TeamChanged(int teamID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_TeamChangedFuncPtr(&m_NativeInterface, m_data, teamID);
}

void NativeInterfaceSystem::TeamDied(int teamID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_TeamDiedFuncPtr(&m_NativeInterface, m_data, teamID);
}

void NativeInterfaceSystem::UnitCreated(const CUnit* unit, const CUnit* builder) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitCreatedFuncPtr(&m_NativeInterface, m_data, unit->id, builder != nullptr ? builder->id : -1);
}

void NativeInterfaceSystem::UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitDestroyedFuncPtr(&m_NativeInterface, m_data, unit->id, attacker != nullptr ? attacker->id : -1);
}

void NativeInterfaceSystem::UnitExperience(const CUnit* unit, float oldExperience) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitExperienceFuncPtr(&m_NativeInterface, m_data, unit->id, oldExperience);
}

void NativeInterfaceSystem::UnitFinished(const CUnit *unit) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitFinishedFuncPtr(&m_NativeInterface, m_data, unit->id);
}

void NativeInterfaceSystem::UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitFromFactoryFuncPtr(&m_NativeInterface, m_data, unit->id, factory->id, userOrders);
}

void NativeInterfaceSystem::UnitGiven(const CUnit* unit, int oldTeam, int newTeam) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitGivenFuncPtr(&m_NativeInterface, m_data, unit->id, oldTeam, newTeam);
}

void NativeInterfaceSystem::UnitLoaded(const CUnit* unit, const CUnit* transport) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitLoadedFuncPtr(&m_NativeInterface, m_data, unit->id, transport->id);
}

void NativeInterfaceSystem::UnitStunned(const CUnit* unit, bool stunned) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitStunnedFuncPtr(&m_NativeInterface, m_data, unit->id, stunned);
}

void NativeInterfaceSystem::UnitTaken(const CUnit* unit, int oldTeam, int newTeam) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitTakenFuncPtr(&m_NativeInterface, m_data, unit->id, oldTeam, newTeam);
}

void NativeInterfaceSystem::UnitUnloaded(const CUnit* unit, const CUnit* transport) {
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_UnitUnloadedFuncPtr(&m_NativeInterface, m_data, unit->id, transport->id);
}

void NativeInterfaceSystem::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data)
{
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_HandleLuaMsgFuncPtr(&m_NativeInterface, m_data, playerID, script, mode, data.data(), data.size());
}

void NativeInterfaceSystem::HandleLuaCall(const char* msg)
{
  LOG_L(L_DEBUG, "NativeModule::%s", __func__);
  m_HandleLuaCallFuncPtr(&m_NativeInterface, m_data, msg);
}

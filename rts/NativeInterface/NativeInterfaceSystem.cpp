/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceSystem.h"

#include <dlfcn.h>
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <random>

#include "NativeInterface.h"
#include "NativeInterfaceEventClient.h"

#include "System/EventHandler.h"
#include "System/Log/ILog.h"

namespace fs = std::filesystem;

NativeInterfaceSystem* NativeInterfaceSystem::s_instance = nullptr;

namespace {
	std::string GenerateRandomName(int length) {
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

	std::optional<fs::path> CopySOToTemp(const std::string& source_path) {
		try {
			if (!fs::exists(source_path)) {
				std::cerr << "Source file does not exist: " << source_path << std::endl;
				return std::nullopt;
			}

			std::string random_name = GenerateRandomName(10) + ".so";
			fs::path temp_dir = fs::temp_directory_path();
			fs::path dest_path = temp_dir / random_name;

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
}

/**
 * Pimpl implementation - contains all the implementation details
 */
class NativeInterfaceSystem::Impl {
public:
	NativeInterface nativeInterface;
	void* dllHandle = nullptr;
	std::unique_ptr<NativeInterfaceEventClient> eventClient;

	Impl() {
		// Initialize NativeInterface with all API pointers
		nativeInterface.memory = &MEMORY_API;
		nativeInterface.game = &GAME_API;
		nativeInterface.terrain = &TERRAIN_API;
		nativeInterface.teams = &TEAMS_API;
		nativeInterface.unitsQuery = &UNITS_QUERY_API;
		nativeInterface.unitsInfo = &UNITS_INFO_API;
		nativeInterface.unitsWeapons = &UNITS_WEAPONS_API;
		nativeInterface.unitsCommands = &UNITS_COMMANDS_API;
		nativeInterface.unitsPieces = &UNITS_PIECES_API;
		nativeInterface.features = &FEATURES_API;
		nativeInterface.projectiles = &PROJECTILES_API;
		nativeInterface.los = &LOS_API;
		nativeInterface.unitDefs = &UNIT_DEFS_API;
		nativeInterface.featureDefs = &FEATURE_DEFS_API;
		nativeInterface.weaponDefs = &WEAPON_DEFS_API;
		nativeInterface.metalMap = &METAL_MAP_API;
		nativeInterface.pathFinder = &PATH_FINDER_API;
		nativeInterface.rulesParams = &RULES_PARAMS_API;
		nativeInterface.mathExtra = &MATH_EXTRA_API;
		nativeInterface.moveCtrl = &MOVE_CTRL_API;
		nativeInterface.syncedCtrl = &SYNCED_CTRL_API;
		nativeInterface.cameraApi = &CAMERA_API;
		nativeInterface.input = &INPUT_API;
		nativeInterface.display = &DISPLAY_API;
		nativeInterface.selection = &SELECTION_API;
		nativeInterface.vfs = &VFS_API;
		nativeInterface.soundApi = &SOUND_API;
		nativeInterface.messages = &MESSAGES_API;
		nativeInterface.config = &CONFIG_API;
		nativeInterface.tracing = &TRACING_API;
		nativeInterface.utils = &UTILS_API;
		nativeInterface.player = &PLAYER_API;
	}

	~Impl() {
		// Unregister event client before cleanup
		if (eventClient) {
			eventHandler.RemoveClient(eventClient.get());
			eventClient.reset();
		}

		if (dllHandle != nullptr) {
			dlclose(dllHandle);
			dllHandle = nullptr;
		}
	}

	void LoadDLL(const std::string& path) {
		// Cleanup previous DLL if any
		if (eventClient) {
			eventHandler.RemoveClient(eventClient.get());
			eventClient.reset();
		}

		if (dllHandle != nullptr) {
			const int closed = dlclose(dllHandle);
			if (closed != 0) {
				LOG_L(L_ERROR, "Failed to close plugin %s: %s", path.c_str(), dlerror());
			}
			dllHandle = nullptr;
		}

		// Copy to temp location for hot reload support
		const auto orig_path = path;
		auto temp_path = CopySOToTemp(path);
		if (!temp_path.has_value()) {
			LOG_L(L_ERROR, "Failed to copy plugin to temp location");
			return;
		}

		dllHandle = dlopen(temp_path->c_str(), RTLD_LAZY);
		if (!dllHandle) {
			LOG_L(L_ERROR, "Failed to open plugin %s: %s", temp_path->c_str(), dlerror());
			return;
		}

		LOG("Successfully opened plugin %s", orig_path.c_str());

		// Check module API version BEFORE calling any module code
		uint32_t* moduleVersion = static_cast<uint32_t*>(dlsym(dllHandle, "NativeModuleApiVersion"));
		if (moduleVersion == nullptr) {
			LOG_L(L_ERROR, "Module does not export NativeModuleApiVersion symbol - incompatible module");
			dlclose(dllHandle);
			dllHandle = nullptr;
			return;
		}

		// Extract version components
		const uint32_t moduleMajor = moduleVersion[0];
		const uint32_t moduleMinor = moduleVersion[1];
		const uint32_t modulePatch = moduleVersion[2];

		// Get host version
		const uint32_t hostMajor = NATIVE_API_MAJOR(NATIVE_API_CURRENT_VERSION);
		const uint32_t hostMinor = NATIVE_API_MINOR(NATIVE_API_CURRENT_VERSION);
		const uint32_t hostPatch = NATIVE_API_PATCH(NATIVE_API_CURRENT_VERSION);

		LOG("Module API version: %u.%u.%u, Host API version: %u.%u.%u",
			moduleMajor, moduleMinor, modulePatch,
			hostMajor, hostMinor, hostPatch);

		// MUST reject incompatible major version
		if (moduleMajor != hostMajor) {
			LOG_L(L_ERROR, "Incompatible API version: module v%u.%u.%u, host v%u.%u.%u (major version mismatch)",
				moduleMajor, moduleMinor, modulePatch,
				hostMajor, hostMinor, hostPatch);
			dlclose(dllHandle);
			dllHandle = nullptr;
			return;
		}

		// Create event client, load symbols, and register with event handler
		eventClient = std::make_unique<NativeInterfaceEventClient>(&nativeInterface, dllHandle);
		eventClient->LoadSymbols();
		eventClient->Initialize();
		eventHandler.AddClient(eventClient.get());
	}
};

NativeInterfaceSystem::NativeInterfaceSystem()
	: pImpl(std::make_unique<Impl>())
{
	LOG("Native interface system initialization...");

	Reload();

	s_instance = this;
}

NativeInterfaceSystem::~NativeInterfaceSystem() {
	// pImpl destructor handles cleanup
}

void NativeInterfaceSystem::Reload() {
	std::string path = "/home/gajop/projects/spring-projects/SBC.sdd/native//target/release/librust_plugin.so";
	pImpl->LoadDLL(path);
}

void NativeInterfaceSystem::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data) {
	if (pImpl->eventClient)
		pImpl->eventClient->HandleLuaMsg(playerID, script, mode, data);
}

void NativeInterfaceSystem::HandleLuaCall(const char* msg) {
	if (pImpl->eventClient)
		pImpl->eventClient->HandleLuaCall(msg);
}

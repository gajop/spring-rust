/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceSystem.h"

#include <algorithm>
#include <charconv>
#include <cctype>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <random>
#include <utility>

#include "NativeInterface.h"
#include "NativeInterfaceEventClient.h"
#include "NativeModulePath.h"
#include "NativeInterface/api/RmlUi.h"
#include "Game/GameSetup.h"
#include "WasmInterface/WasmInterfaceSystem.h"

#include "Rml/Backends/RmlUi_Backend.h"

#include "System/EventHandler.h"
#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/VFSHandler.h"
#include "System/FileSystem/VFSModes.h"
#include "System/Log/ILog.h"
#include "System/Platform/SharedLib.h"

namespace fs = std::filesystem;

NativeInterfaceSystem* NativeInterfaceSystem::s_instance = nullptr;

namespace {
	std::uint64_t ReadWasmLimitOption(std::string_view name, std::uint64_t fallback)
	{
		const auto& options = CGameSetup::GetModOptions();
		const auto iter = options.find(std::string(name));
		if (iter == options.end() || iter->second.empty())
			return fallback;

		std::string value = iter->second;
		std::transform(value.begin(), value.end(), value.begin(), [](unsigned char character) {
			return static_cast<char>(std::tolower(character));
		});
		if (value == "0" || value == "off" || value == "false" || value == "unlimited")
			return 0;

		std::uint64_t parsed = 0;
		const auto result = std::from_chars(value.data(), value.data() + value.size(), parsed);
		if (result.ec == std::errc{} && result.ptr == value.data() + value.size())
			return parsed;

		LOG_L(L_WARNING, "Ignoring invalid Wasm limit mod option %s=%s", iter->first.c_str(),
			iter->second.c_str());
		return fallback;
	}

	WasmRuntimeConfig WasmRuntimeConfigFromModOptions()
	{
		WasmRuntimeConfig config;
		config.instructionFuel = ReadWasmLimitOption("wasm_instruction_fuel", config.instructionFuel);
		config.hostWorkLimit = ReadWasmLimitOption("wasm_host_work_limit", config.hostWorkLimit);
		return config;
	}

	void RemoveNativeRmlContext(uint64_t contextHandle) {
		RmlGui::RemoveContextImmediately(
			reinterpret_cast<Rml::Context*>(static_cast<uintptr_t>(contextHandle)));
	}

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

	std::optional<fs::path> CopyFileToTemp(const NativeModuleSource& source) {
		try {
			const std::string extension = fs::path(source.path).extension().string();
			std::string random_name = GenerateRandomName(10) + extension;
			fs::path temp_dir = fs::temp_directory_path();
			fs::path dest_path = temp_dir / random_name;

			if (source.isVfsFile) {
				CFileHandler moduleFile(source.path, SPRING_VFS_MOD);
				std::string moduleData;
				if (!moduleFile.FileExists() || !moduleFile.LoadStringData(moduleData)) {
					LOG_L(L_ERROR, "Native module does not exist in the selected game: %s", source.path.c_str());
					return std::nullopt;
				}

				std::ofstream output(dest_path, std::ios::binary | std::ios::trunc);
				output.write(moduleData.data(), static_cast<std::streamsize>(moduleData.size()));
				if (!output) {
					LOG_L(L_ERROR, "Failed to extract native module to: %s", dest_path.string().c_str());
					return std::nullopt;
				}
			} else {
				if (!fs::exists(source.path)) {
					LOG_L(L_ERROR, "Native module source does not exist: %s", source.path.c_str());
					return std::nullopt;
				}
				fs::copy_file(source.path, dest_path, fs::copy_options::overwrite_existing);
			}

			return dest_path;
		} catch (const fs::filesystem_error& e) {
			LOG_L(L_ERROR, "Failed to prepare native module: %s", e.what());
			return std::nullopt;
		} catch (const std::exception& e) {
			LOG_L(L_ERROR, "Failed to prepare native module: %s", e.what());
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
	std::unique_ptr<SharedLib> sharedLib;
	std::unique_ptr<NativeInterfaceEventClient> eventClient;
	std::vector<std::unique_ptr<NativeInterfaceEventClient>> benchmarkEventClients;
	std::unique_ptr<WasmInterfaceSystem> wasmSystem;

	Impl() {
		// Initialize all API pointers
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
		nativeInterface.platform = &PLATFORM_API;
		nativeInterface.rulesParams = &RULES_PARAMS_API;
		nativeInterface.rmlUi = &RMLUI_API;
		nativeInterface.mathExtra = &MATH_EXTRA_API;
		nativeInterface.moveCtrl = &MOVE_CTRL_API;
		nativeInterface.syncedCtrl = &SYNCED_CTRL_API;
		nativeInterface.cameraApi = &CAMERA_API;
		nativeInterface.input = &INPUT_API;
		nativeInterface.debugInput = &DEBUG_INPUT_API;
		nativeInterface.display = &DISPLAY_API;
		nativeInterface.selection = &SELECTION_API;
		nativeInterface.vfs = &VFS_API;
		nativeInterface.unsyncedCtrl = &UNSYNCED_CTRL_API;
		nativeInterface.unsyncedRead = &UNSYNCED_READ_API;
		nativeInterface.lights = &LIGHTS_API;
		nativeInterface.icons = &ICONS_API;
		nativeInterface.markers = &MARKERS_API;
		nativeInterface.groundDecals = &GROUND_DECALS_API;
		nativeInterface.systemControl = &SYSTEM_CONTROL_API;
		nativeInterface.profiling = &PROFILING_API;
		nativeInterface.gfx = &GFX_API;
		nativeInterface.soundApi = &SOUND_API;
		nativeInterface.messages = &MESSAGES_API;
		nativeInterface.config = &CONFIG_API;
		nativeInterface.encoding = &ENCODING_API;
		nativeInterface.tracing = &TRACING_API;
		nativeInterface.utils = &UTILS_API;
		nativeInterface.player = &PLAYER_API;
		wasmSystem = std::make_unique<WasmInterfaceSystem>(&nativeInterface,
			WasmRuntimeConfigFromModOptions());
	}

	~Impl() {
		// Stop callbacks, then let the module release resources while its shared
		// object (and therefore its shutdown function) is still loaded.
		UnloadEventClients();

		sharedLib.reset();
	}

	void UnloadEventClients() {
		for (auto& client : benchmarkEventClients)
			eventHandler.RemoveClient(client.get());
		if (eventClient)
			eventHandler.RemoveClient(eventClient.get());

		if (eventClient || !benchmarkEventClients.empty())
			NativeRmlUi::ClearAllContexts(RemoveNativeRmlContext);

		for (auto& client : benchmarkEventClients)
			client->Shutdown();
		benchmarkEventClients.clear();
		if (eventClient) {
			eventClient->Shutdown();
			eventClient.reset();
		}
	}

	void LoadDLL(const NativeModuleSource& source) {
		// Stop callbacks, then let the old module release resources before its
		// shared object is unloaded. This is essential for module-owned RmlUi
		// contexts and other host resources.
		UnloadEventClients();

		sharedLib.reset();

		// Copy to temp location for hot reload support
		auto temp_path = CopyFileToTemp(source);
		if (!temp_path.has_value()) {
			LOG_L(L_ERROR, "Failed to copy plugin to temp location");
			return;
		}

		sharedLib.reset(SharedLib::Instantiate(temp_path->string()));
		if (!sharedLib) {
			LOG_L(L_ERROR, "Failed to open plugin %s", temp_path->string().c_str());
			return;
		}

		LOG("Successfully opened native module %s", source.path.c_str());

		// Check module API version BEFORE calling any module code
		uint32_t* moduleVersion = static_cast<uint32_t*>(sharedLib->FindAddress("NativeModuleApiVersion"));
		if (moduleVersion == nullptr) {
			LOG_L(L_ERROR, "Module does not export NativeModuleApiVersion symbol - incompatible module");
			sharedLib.reset();
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
			sharedLib.reset();
			return;
		}

		if (moduleMinor > hostMinor) {
			LOG_L(L_ERROR, "Incompatible API version: module v%u.%u.%u, host v%u.%u.%u (module requires newer minor version)",
				moduleMajor, moduleMinor, modulePatch,
				hostMajor, hostMinor, hostPatch);
			sharedLib.reset();
			return;
		}

		// Create event client, load symbols, and register with event handler
		eventClient = std::make_unique<NativeInterfaceEventClient>(&nativeInterface, sharedLib.get(), wasmSystem.get());
		eventClient->LoadSymbols();
		eventClient->Initialize();
		eventHandler.AddClient(eventClient.get());

		// The normal native integration has one module instance. The benchmark
		// explicitly opts into four independent event clients so its fan-out row
		// measures real module lifecycle/dispatch behavior rather than repeating a
		// callback in the benchmark guest.
		std::size_t benchmarkModuleCount = 1;
		if (const char* value = std::getenv("SPRING_NATIVE_BENCHMARK_MODULES");
			value != nullptr && *value != '\0') {
			std::uint64_t parsed = 0;
			const auto [end, conversionError] = std::from_chars(
				value, value + std::char_traits<char>::length(value), parsed);
			if (conversionError == std::errc{} && end == value + std::char_traits<char>::length(value))
				benchmarkModuleCount = std::clamp<std::size_t>(parsed, 1, 16);
		}
		for (std::size_t index = 1; index < benchmarkModuleCount; ++index) {
			auto client = std::make_unique<NativeInterfaceEventClient>(
				&nativeInterface, sharedLib.get(), wasmSystem.get());
			client->LoadSymbols();
			client->Initialize();
			eventHandler.AddClient(client.get());
			benchmarkEventClients.push_back(std::move(client));
		}
	}

	void EnsureWasmEventClient() {
		if (eventClient)
			return;

		// Keep a managed event client alive even when no legacy native DLL is
		// installed. This lets Wasm-only content receive the same engine event
		// stream without changing the native module loading contract.
		eventClient = std::make_unique<NativeInterfaceEventClient>(
			&nativeInterface, nullptr, wasmSystem.get());
		eventHandler.AddClient(eventClient.get());
	}

	void RequestReload() {
		if (eventClient) {
			reloadRequested = true;
			return;
		}
		ReloadNow();
	}

	void ReloadNow() {
		const auto source = ResolveNativeModuleSource();
		if (!source.has_value()) {
			EnsureWasmEventClient();
			return;
		}

		LoadDLL(*source);
		if (!eventClient)
			EnsureWasmEventClient();
	}

	void Update() {
		if (reloadRequested) {
			reloadRequested = false;
			ReloadNow();
		}
		wasmSystem->Update();
	}

	bool reloadRequested = false;
};

NativeInterfaceSystem::NativeInterfaceSystem()
	: pImpl(std::make_unique<Impl>())
{
	LOG("Native interface system initialization...");

	// Note: Reload() is called later after sound system is initialized
	// to avoid interfering with sound thread startup

	s_instance = this;
}

NativeInterfaceSystem::~NativeInterfaceSystem() {
	s_instance = nullptr;
	// pImpl destructor handles cleanup
}

void NativeInterfaceSystem::Reload() {
	pImpl->RequestReload();
}

void NativeInterfaceSystem::Update() {
	pImpl->Update();
}

bool NativeInterfaceSystem::KeyPress(int keyCode, int scanCode, bool isRepeat) {
	if (!pImpl->eventClient)
		return false;

	const bool handled = pImpl->eventClient->KeyPress(keyCode, scanCode, isRepeat);
	if (!handled)
		pImpl->eventClient->SuppressNextKeyPress();
	return handled;
}

bool NativeInterfaceSystem::KeyRelease(int keyCode, int scanCode) {
	if (!pImpl->eventClient)
		return false;

	const bool handled = pImpl->eventClient->KeyRelease(keyCode, scanCode);
	if (!handled)
		pImpl->eventClient->SuppressNextKeyRelease();
	return handled;
}

void NativeInterfaceSystem::CancelKeyPressPreDispatch() {
	if (pImpl->eventClient)
		pImpl->eventClient->CancelSuppressedKeyPress();
}

void NativeInterfaceSystem::CancelKeyReleasePreDispatch() {
	if (pImpl->eventClient)
		pImpl->eventClient->CancelSuppressedKeyRelease();
}

void NativeInterfaceSystem::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data) {
	if (pImpl->eventClient)
		pImpl->eventClient->HandleLuaMsg(playerID, script, mode, data);
}

void NativeInterfaceSystem::HandleLuaCall(const char* msg, size_t msgLength, bool synced) {
	if (pImpl->eventClient)
		pImpl->eventClient->HandleLuaCall(msg, msgLength, synced);
}

bool NativeInterfaceSystem::DispatchWasmSyncedMessage(std::string_view message,
	std::string& error)
{
	return pImpl->wasmSystem->DispatchSyncedMessage(message, error);
}

bool NativeInterfaceSystem::LoadWasmModule(WasmModuleDescriptor descriptor, std::string& error) {
	return pImpl->wasmSystem->LoadModule(std::move(descriptor), error);
}

bool NativeInterfaceSystem::LoadWasmManifest(std::string_view manifestPath,
	std::string_view vfsModes, std::string& error)
{
	const std::string path(manifestPath);
	const std::string modes(vfsModes);
	CVFSHandler::Section section = CVFSHandler::Section::Error;
	for (const char mode : modes) {
		section = CVFSHandler::GetModeSection(mode);
		if (section != CVFSHandler::Section::Error)
			break;
	}
	if (vfsHandler != nullptr && section != CVFSHandler::Section::Error) {
		std::vector<std::string> archiveNames = vfsHandler->GetAllArchiveNames(section);
		std::sort(archiveNames.begin(), archiveNames.end());
		archiveNames.erase(std::unique(archiveNames.begin(), archiveNames.end()), archiveNames.end());
		std::vector<WasmManifestSource> sources;
		for (const auto& archive : archiveNames) {
			const int fileExists = vfsHandler->FileExistsInArchive(archive, path, section);
			if (fileExists < 0)
				continue;
			if (fileExists == 0)
				continue;
			std::vector<std::uint8_t> bytes;
			const int loadCode = vfsHandler->LoadFileFromArchive(archive, path, bytes, section);
			if (loadCode != 1) {
				error = "could not read Wasm manifest " + path + " from archive " + archive;
				return false;
			}
			sources.push_back({archive, std::string(bytes.begin(), bytes.end())});
		}
		if (!sources.empty()) {
			const bool loaded = pImpl->wasmSystem->LoadManifests(sources,
				[section](std::string_view archive, std::string_view modulePath,
					std::vector<std::uint8_t>& bytes, std::string& providerError) {
					const std::string archiveName(archive);
					const std::string path(modulePath);
					if (vfsHandler == nullptr) {
						providerError = "Wasm VFS is unavailable";
						return false;
					}
					const int loadCode = vfsHandler->LoadFileFromArchive(archiveName, path, bytes, section);
					if (loadCode != 1) {
						providerError = "Wasm module does not exist in archive " + archiveName + ": " + path;
						return false;
					}
					return true;
				}, error);
			if (!loaded)
				error = "failed to load Wasm manifests from " + path + ": " + error;
			return loaded;
		}
	}

	// Preserve the legacy single-source behavior for raw development files and
	// VFS implementations that do not expose archive enumeration.
	CFileHandler manifestFile(path, modes);
	if (!manifestFile.FileExists())
		return true;
	std::string manifest;
	if (!manifestFile.LoadStringData(manifest)) {
		error = "could not read Wasm manifest: " + path;
		return false;
	}
	const bool loaded = pImpl->wasmSystem->LoadManifest(manifest,
		[modes](std::string_view modulePath, std::vector<std::uint8_t>& bytes,
			std::string& providerError) {
			const std::string modulePathString(modulePath);
			CFileHandler moduleFile(modulePathString, modes);
			if (!moduleFile.FileExists()) {
				providerError = "Wasm module does not exist in the selected content: " + modulePathString;
				return false;
			}
			std::string data;
			if (!moduleFile.LoadStringData(data)) {
				providerError = "could not read Wasm module: " + modulePathString;
				return false;
			}
			bytes.assign(data.begin(), data.end());
			return true;
		}, error);
	if (!loaded)
		error = "failed to load Wasm manifest " + path + ": " + error;
	return loaded;
}

bool NativeInterfaceSystem::UnloadWasmModule(const std::string& moduleName) {
	return pImpl->wasmSystem->UnloadModule(moduleName);
}

void NativeInterfaceSystem::UnloadAllWasmModules() {
	pImpl->wasmSystem->UnloadAll();
}

WasmInterfaceSystem* NativeInterfaceSystem::GetWasmInterfaceSystem() {
	return pImpl->wasmSystem.get();
}

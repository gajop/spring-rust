/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmStandaloneEnvironment.h"

#include <filesystem>
#include <fstream>
#include <random>

#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/NativeInterfaceEventClient.h"
#include "NativeInterface/api/RmlUi.h"
#include "Rml/Backends/RmlUi_Backend.h"
#include "WasmInterface/system/WasmInterfaceSystem.h"
#include "WasmInterface/runtime/WasmEnvironment.h"

#include "System/EventHandler.h"
#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/VFSModes.h"
#include "System/Log/ILog.h"
#include "System/Platform/SharedLib.h"

namespace fs = std::filesystem;

extern const DisplayApi DISPLAY_API;
extern const ConfigApi CONFIG_API;
extern const VFSApi VFS_API;
extern const GfxApi GFX_API;
extern const SoundApi SOUND_API;
extern const InputApi INPUT_API;
extern const PlatformApi PLATFORM_API;
extern const EncodingApi ENCODING_API;
extern const MathExtraApi MATH_EXTRA_API;
extern const UtilsApi UTILS_API;
extern const SystemControlApi SYSTEM_CONTROL_API;
extern const MemoryApi MEMORY_API;
extern const UnsyncedCtrlApi UNSYNCED_CTRL_API;
extern const UnsyncedReadApi UNSYNCED_READ_API;
extern const PlayerApi PLAYER_API;
extern const MessagesApi MESSAGES_API;
extern const ProfilingApi PROFILING_API;
extern const TracingApi TRACING_API;
extern const RmlUiApi RMLUI_API;

namespace {

std::string PlatformLibraryPath(const std::string& pathStem)
{
	fs::path path(pathStem);
	if (path.has_extension())
		return path.generic_string();

	std::string filename = path.filename().string();

#if defined(_WIN32)
	filename += ".dll";
#elif defined(__APPLE__)
	filename = "lib" + filename + ".dylib";
#else
	filename = "lib" + filename + ".so";
#endif

	return (path.parent_path() / filename).generic_string();
}

std::string GenerateRandomName(int length)
{
	const char charset[] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
	const int charset_size = sizeof(charset) - 1;

	std::random_device rd;
	std::mt19937 gen(rd());
	std::uniform_int_distribution<> dis(0, charset_size - 1);

	std::string result;
	result.reserve(length);
	for (int i = 0; i < length; ++i)
		result += charset[dis(gen)];
	return result;
}

std::optional<fs::path> CopyToTemp(const std::string& sourcePath, const char* vfsModes)
{
	try {
		const std::string extension = fs::path(sourcePath).extension().string();
		std::string randomName = GenerateRandomName(10) + extension;
		fs::path destPath = fs::temp_directory_path() / randomName;

		if (vfsModes != nullptr) {
			CFileHandler moduleFile(sourcePath, vfsModes);
			std::string moduleData;
			if (!moduleFile.FileExists()) {
				return std::nullopt;
			}
			if (!moduleFile.LoadStringData(moduleData)) {
				LOG_L(L_ERROR, "Failed to read native module: %s", sourcePath.c_str());
				return std::nullopt;
			}

			std::ofstream output(destPath, std::ios::binary | std::ios::trunc);
			output.write(moduleData.data(), static_cast<std::streamsize>(moduleData.size()));
			if (!output) {
				LOG_L(L_ERROR, "Failed to extract native module to: %s", destPath.string().c_str());
				return std::nullopt;
			}
		} else {
			if (!fs::exists(sourcePath)) {
				return std::nullopt;
			}
			fs::copy_file(sourcePath, destPath, fs::copy_options::overwrite_existing);
		}

		return destPath;
	} catch (const std::exception& e) {
		LOG_L(L_ERROR, "Failed to prepare native module: %s", e.what());
		return std::nullopt;
	}
}

void RemoveNativeRmlContext(uint64_t contextHandle)
{
	RmlGui::RemoveContextImmediately(
		reinterpret_cast<Rml::Context*>(static_cast<uintptr_t>(contextHandle)));
}

} // namespace


WasmStandaloneEnvironment::WasmStandaloneEnvironment() = default;

WasmStandaloneEnvironment::~WasmStandaloneEnvironment()
{
	RemoveEventClient();
	m_sharedLib.reset();
	m_wasmSystem.reset();
	m_nativeInterface.reset();
}

std::unique_ptr<WasmStandaloneEnvironment> WasmStandaloneEnvironment::Create()
{
	auto env = std::unique_ptr<WasmStandaloneEnvironment>(new WasmStandaloneEnvironment());

	env->m_nativeInterface = std::make_unique<NativeInterface>();
	*env->m_nativeInterface = {};

	env->m_nativeInterface->memory = &MEMORY_API;
	env->m_nativeInterface->display = &DISPLAY_API;
	env->m_nativeInterface->config = &CONFIG_API;
	env->m_nativeInterface->vfs = &VFS_API;
	env->m_nativeInterface->gfx = &GFX_API;
	env->m_nativeInterface->soundApi = &SOUND_API;
	env->m_nativeInterface->input = &INPUT_API;
	env->m_nativeInterface->platform = &PLATFORM_API;
	env->m_nativeInterface->encoding = &ENCODING_API;
	env->m_nativeInterface->mathExtra = &MATH_EXTRA_API;
	env->m_nativeInterface->utils = &UTILS_API;
	env->m_nativeInterface->systemControl = &SYSTEM_CONTROL_API;
	env->m_nativeInterface->unsyncedCtrl = &UNSYNCED_CTRL_API;
	env->m_nativeInterface->unsyncedRead = &UNSYNCED_READ_API;
	env->m_nativeInterface->player = &PLAYER_API;
	env->m_nativeInterface->messages = &MESSAGES_API;
	env->m_nativeInterface->profiling = &PROFILING_API;
	env->m_nativeInterface->tracing = &TRACING_API;
	env->m_nativeInterface->rmlUi = &RMLUI_API;

	env->m_wasmSystem = std::make_unique<WasmInterfaceSystem>(env->m_nativeInterface.get());

	return env;
}

bool WasmStandaloneEnvironment::LoadManifest(const std::string& manifestPath)
{
	CFileHandler file(manifestPath, SPRING_VFS_RAW_FIRST);
	if (!file.FileExists())
		return false;

	std::string manifest;
	manifest.resize(file.FileSize());
	file.Read(manifest.data(), manifest.size());

	std::string error;
	if (!m_wasmSystem->LoadManifest(manifest,
			[](std::string_view path, std::vector<std::uint8_t>& bytes,
				std::string& providerError) {
				CFileHandler f(std::string(path), SPRING_VFS_RAW_FIRST);
				if (!f.FileExists()) {
					providerError = "file not found: " + std::string(path);
					return false;
				}
				bytes.resize(f.FileSize());
				f.Read(bytes.data(), bytes.size());
				return true;
			}, error)) {
		LOG_L(L_ERROR, "Failed to load manifest %s: %s", manifestPath.c_str(), error.c_str());
		return false;
	}

	LOG("Wasm manifest loaded from %s", manifestPath.c_str());
	return true;
}

bool WasmStandaloneEnvironment::TryLoadNativeDLL(const std::string& pathStem)
{
	std::string resolvedPath = PlatformLibraryPath(pathStem);
	// Menu modules live in the menu archive. Extract from the active menu VFS
	// before falling back to a process-filesystem path for developer installs.
	auto tempPath = CopyToTemp(resolvedPath, SPRING_VFS_MENU_BASE);
	if (!tempPath.has_value())
		tempPath = CopyToTemp(resolvedPath, nullptr);
	if (!tempPath.has_value())
		return false;

	auto lib = std::unique_ptr<SharedLib>(SharedLib::Instantiate(tempPath->string()));
	if (!lib) {
		LOG_L(L_ERROR, "Failed to open native module %s", tempPath->string().c_str());
		return false;
	}

	uint32_t* moduleVersion = static_cast<uint32_t*>(lib->FindAddress("NativeModuleApiVersion"));
	if (moduleVersion == nullptr) {
		LOG_L(L_ERROR, "Module does not export NativeModuleApiVersion - incompatible module");
		return false;
	}

	const uint32_t moduleMajor = moduleVersion[0];
	const uint32_t hostMajor = NATIVE_API_MAJOR(NATIVE_API_CURRENT_VERSION);
	if (moduleMajor != hostMajor) {
		LOG_L(L_ERROR, "Incompatible API version: module major %u, host major %u",
			moduleMajor, hostMajor);
		return false;
	}

	const uint32_t moduleMinor = moduleVersion[1];
	const uint32_t hostMinor = NATIVE_API_MINOR(NATIVE_API_CURRENT_VERSION);
	if (moduleMinor > hostMinor) {
		LOG_L(L_ERROR, "Incompatible API version: module minor %u > host minor %u",
			moduleMinor, hostMinor);
		return false;
	}

	RemoveEventClient();
	m_sharedLib = std::move(lib);
	m_nativeModuleLoaded = true;

	LOG("Successfully loaded native module %s", resolvedPath.c_str());

	m_eventClient = std::make_unique<NativeInterfaceEventClient>(
		m_nativeInterface.get(), m_sharedLib.get(), m_wasmSystem.get(), true);
	m_eventClient->LoadSymbols();
	m_eventClient->Initialize();
	if (!m_eventClient->IsInitialized()) {
		LOG_L(L_ERROR, "Native menu module failed to initialize: %s", resolvedPath.c_str());
		RemoveEventClient();
		m_sharedLib.reset();
		return false;
	}
	eventHandler.AddClient(m_eventClient.get());

	return true;
}

void WasmStandaloneEnvironment::EnsureEventClient()
{
	if (m_eventClient)
		return;

	m_eventClient = std::make_unique<NativeInterfaceEventClient>(
		m_nativeInterface.get(), nullptr, m_wasmSystem.get(), true);
	eventHandler.AddClient(m_eventClient.get());
}

void WasmStandaloneEnvironment::RemoveEventClient()
{
	if (!m_eventClient) {
		m_nativeModuleLoaded = false;
		return;
	}

	eventHandler.RemoveClient(m_eventClient.get());
	// Give a native menu its valid shutdown window first. Its shutdown hook may
	// still query or explicitly remove its contexts. Only then clear resources
	// the module left behind, while its callback code and module data remain
	// valid.
	if (m_sharedLib)
		m_eventClient->Shutdown();
	NativeRmlUi::ClearOwnerContexts(m_eventClient->ContextOwner(), RemoveNativeRmlContext);
	if (RmlGui::GetCurrentContextOwner() == m_eventClient->ContextOwner())
		RmlGui::SetCurrentContextOwner(nullptr, false);
	m_eventClient.reset();
	m_nativeModuleLoaded = false;
}

void WasmStandaloneEnvironment::ActivateMenu(const std::string& message)
{
	if (m_eventClient && m_nativeModuleLoaded)
		m_eventClient->ActivateMenu(message);
}

void WasmStandaloneEnvironment::ActivateGame()
{
	if (m_eventClient && m_nativeModuleLoaded)
		m_eventClient->ActivateGame();
}

void WasmStandaloneEnvironment::Update()
{
	if (m_wasmSystem)
		m_wasmSystem->Update();
}

bool WasmStandaloneEnvironment::HasModules(WasmEnvironment environment) const
{
	return m_wasmSystem && m_wasmSystem->HasModules(environment);
}

WasmInterfaceSystem* WasmStandaloneEnvironment::GetWasmSystem() const
{
	return m_wasmSystem.get();
}

NativeInterface* WasmStandaloneEnvironment::GetNativeInterface() const
{
	return m_nativeInterface.get();
}

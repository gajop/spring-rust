/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmDefsParser.h"
#include "WasmStandaloneEnvironment.h"

#include "Lua/LuaParser.h"
#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/api/Callins.h"
#include "WasmInterface/core/host/WasmCoreCallinId.h"
#include "WasmInterface/runtime/WasmEnvironment.h"
#include "WasmInterface/system/WasmInterfaceSystem.h"

#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/VFSModes.h"
#include "System/Log/ILog.h"

LuaParser* TryWasmDefsParser(std::string& errorOut)
{
	const std::string manifestPath = "gamedata/wasm_defs/manifest.json";
	CFileHandler probe(manifestPath, SPRING_VFS_MOD_BASE);
	if (!probe.FileExists())
		return nullptr;

	auto env = WasmStandaloneEnvironment::Create();

	std::string manifest;
	manifest.resize(probe.FileSize());
	probe.Read(manifest.data(), manifest.size());

	std::string loadError;
	if (!env->GetWasmSystem()->LoadManifest(manifest,
			[](std::string_view path, std::vector<std::uint8_t>& bytes,
				std::string& providerError) {
				CFileHandler f(std::string(path), SPRING_VFS_MOD_BASE);
				if (!f.FileExists()) {
					providerError = "file not found: " + std::string(path);
					return false;
				}
				bytes.resize(f.FileSize());
				f.Read(bytes.data(), bytes.size());
				return true;
			}, loadError)) {
		errorOut = "Failed to load wasm defs manifest: " + loadError;
		return nullptr;
	}

	if (!env->HasModules(WasmEnvironment::RulesSynced)) {
		LOG("Wasm defs manifest loaded but has no rules-synced modules, skipping");
		return nullptr;
	}

	LOG("Wasm defs module loaded, dispatching GenerateDefs");

	SimpleCallinQuery query = {};
	StringCallinResult result = {.error = nullptr, .value = nullptr};
	bool handled = false;
	std::string dispatchError;

	if (!WasmInterfaceSystem::DispatchActiveCoreCallin(
			CoreCallinOf("GenerateDefs"), &query, true, &result, handled, dispatchError)) {
		if (!dispatchError.empty()) {
			errorOut = "Wasm GenerateDefs failed: " + dispatchError;
			return nullptr;
		}
	}

	if (result.error != nullptr) {
		errorOut = std::string("Wasm GenerateDefs returned error: ") + result.error->message;
		return nullptr;
	}

	if (result.value == nullptr || result.value[0] == '\0') {
		LOG("Wasm GenerateDefs returned empty result, falling back to Lua defs");
		return nullptr;
	}

	std::string luaSource(result.value);
	LOG("Wasm GenerateDefs produced %zu bytes of Lua source", luaSource.size());

	auto* parser = new LuaParser(luaSource, SPRING_VFS_MOD_BASE, 0, {true}, {false});
	return parser;
}

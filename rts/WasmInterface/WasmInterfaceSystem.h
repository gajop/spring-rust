/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "WasmDispatch.h"
#include "WasmModuleManifest.h"
#include "WasmRuntime.h"

class WasmInterfaceSystem {
public:
	explicit WasmInterfaceSystem(WasmHostAdapter* hostAdapter = nullptr);
	~WasmInterfaceSystem();

	WasmInterfaceSystem(const WasmInterfaceSystem&) = delete;
	WasmInterfaceSystem& operator=(const WasmInterfaceSystem&) = delete;

	bool LoadModule(WasmModuleDescriptor descriptor, std::string& error);
	using ModuleBytesProvider = std::function<bool(std::string_view path,
		std::vector<std::uint8_t>& bytes, std::string& error)>;
	bool LoadManifest(std::string_view manifest, const ModuleBytesProvider& bytesProvider,
		std::string& error);
	using ArchiveModuleBytesProvider = std::function<bool(std::string_view archive,
		std::string_view path, std::vector<std::uint8_t>& bytes, std::string& error)>;
	// Parse and load declarations from all participating archives as one
	// transaction. Module names are global, while each module's bytes remain
	// relative to the archive that declared it.
	bool LoadManifests(const std::vector<WasmManifestSource>& sources,
		const ArchiveModuleBytesProvider& bytesProvider, std::string& error);
	bool UnloadModule(std::string_view moduleName);
	void UnloadAll();
	void Update();

	bool DispatchCallin(const WasmCallinEvent& event, WasmEnvironment environment,
		std::string& error);
	// Dispatch the generated Component Model callin world. The callin name is
	// the canonical NativeInterface spelling; the system maps it to the
	// environment-specific WIT export path.
	bool DispatchCallin(std::string_view name, const std::vector<WasmValue>& arguments,
		WasmEnvironment environment, std::string& error);
	// Dispatch a semantic callin and return its aggregated payload. The result
	// is the generated callin result record (not the outer result<..., error>
	// envelope). For fire-and-forget callins the result is unit.
	bool DispatchCallin(std::string_view name, const std::vector<WasmValue>& arguments,
		WasmEnvironment environment, WasmValue& result, std::string& error);
	bool DispatchCallin(std::string_view name, const std::vector<WasmValue>& arguments,
		const std::vector<WasmEnvironment>& environments, WasmValue& result,
		std::string& error);
	struct CallinInvocation {
		WasmEnvironment environment;
		std::vector<WasmValue> arguments;
		bool contributesResult = true;
	};
	bool DispatchCallin(std::string_view name,
		const std::vector<CallinInvocation>& invocations, WasmValue& result,
		std::string& error);

	std::size_t ModuleCount() const;
	const WasmRuntime& Runtime() const { return *runtime; }

	// Match configuration must enumerate these values for every synced module.
	std::vector<std::string> SyncedConfiguration() const;

private:
	void FaultSyncedModules(WasmEnvironment environment, std::string_view reason);

	std::unique_ptr<WasmRuntime> runtime;
	WasmHostAdapter* hostAdapter = nullptr;
	std::vector<std::unique_ptr<WasmModule>> modules;
	WasmInstanceID nextInstanceID = 1;
};

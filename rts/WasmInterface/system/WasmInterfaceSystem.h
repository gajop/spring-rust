/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <functional>
#include <memory>
#include <span>
#include <string>
#include <string_view>
#include <vector>

#include "WasmModuleManifest.h"
#include "WasmRuntime.h"

struct NativeInterface;
class WasmCoreHost;
enum class WasmCoreCallin : std::uint16_t;

class WasmInterfaceSystem {
public:
	explicit WasmInterfaceSystem(NativeInterface* nativeInterface,
		WasmRuntimeConfig runtimeConfig = {});
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
	bool LoadManifests(const std::vector<WasmManifestSource>& sources,
		const ArchiveModuleBytesProvider& bytesProvider, std::string& error);
	bool UnloadModule(std::string_view moduleName);
	void UnloadAll();
	void Update();

	static bool DispatchActiveCoreCallin(std::string_view name, const void* query,
		bool synced, void* nativeResult, bool& handled, std::string& error);
	struct CoreCallinInvocation {
		WasmEnvironment environment;
		const void* query = nullptr;
		bool contributesResult = true;
	};
	bool DispatchCoreCallin(std::string_view name,
		std::span<const CoreCallinInvocation> invocations, void* nativeResult,
		bool& handled, std::string& error);
	bool DispatchCoreCallin(WasmCoreCallin callin, std::string_view diagnosticName,
		std::span<const CoreCallinInvocation> invocations, void* nativeResult,
		bool& handled, std::string& error);
	bool DispatchSyncedMessage(std::string_view message, std::string& error);

	std::size_t ModuleCount() const;
	bool HasModules(WasmEnvironment environment) const;
	bool HasCoreModules(WasmEnvironment environment) const;
	const WasmRuntime& Runtime() const { return *runtime; }
	std::vector<std::string> SyncedConfiguration() const;

private:
	struct CoreModuleRecord {
		WasmModuleDescriptor descriptor;
		WasmModuleIdentity identity;
		WasmCoreHost* host = nullptr;
	};

	class CoreDispatchRegistration {
	public:
		explicit CoreDispatchRegistration(WasmInterfaceSystem* owner);
		~CoreDispatchRegistration();
		CoreDispatchRegistration(const CoreDispatchRegistration&) = delete;
		CoreDispatchRegistration& operator=(const CoreDispatchRegistration&) = delete;
	private:
		WasmInterfaceSystem* owner = nullptr;
		WasmInterfaceSystem* previous = nullptr;
	};

	std::unique_ptr<WasmRuntime> runtime;
	NativeInterface* nativeInterface = nullptr;
	std::vector<CoreModuleRecord> coreModules;
	std::uint32_t coreEnvironmentMask = 0;
	CoreDispatchRegistration coreDispatchRegistration{this};
};

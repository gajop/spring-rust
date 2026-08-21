/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <functional>
#include <memory>
#include <span>
#include <string>
#include <string_view>
#include <vector>

#include "WasmDispatch.h"
#include "WasmModuleManifest.h"
#include "WasmRuntime.h"

class WasmCoreHost;
enum class WasmCoreCallin : std::uint16_t;

class WasmInterfaceSystem {
public:
	explicit WasmInterfaceSystem(WasmHostAdapter* hostAdapter = nullptr,
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

	bool DispatchCallin(const WasmCallinEvent& event, WasmEnvironment environment,
		std::string& error);
	bool DispatchCallin(std::string_view name, const std::vector<WasmValue>& arguments,
		WasmEnvironment environment, std::string& error);
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
	static bool AggregateCallinResult(std::string_view aggregation,
		const WasmValue& value, bool& haveResult, WasmValue& result,
		std::string& error);

	struct CoreCallinInvocation {
		WasmEnvironment environment;
		const void* query = nullptr;
		bool contributesResult = true;
	};
	bool DispatchCoreCallin(std::string_view name,
		std::span<const CoreCallinInvocation> invocations,
		WasmValue* valueResult, void* nativeResult, bool& handled,
		std::string& error);
	// Internal hot overload: the outer event seam has already resolved the
	// string to the compact callin ID, so do not resolve it again.
	bool DispatchCoreCallin(WasmCoreCallin callin, std::string_view diagnosticName,
		std::span<const CoreCallinInvocation> invocations,
		WasmValue* valueResult, void* nativeResult, bool& handled,
		std::string& error);

	// The NativeInterface event seam already knows whether this dispatch is on
	// the synced or unsynced side. Pass that fact through instead of inferring it
	// from a small hardcoded callin-name set; many generated callins are valid on
	// both sides.
	static bool DispatchActiveCoreCallin(std::string_view name, const void* query,
		bool synced, void* nativeResult, bool& handled, std::string& error);

	bool DispatchSyncedMessage(std::string_view message, std::string& error);

	std::size_t ModuleCount() const;
	bool HasModules(WasmEnvironment environment) const;
	bool HasCoreModules(WasmEnvironment environment) const;
	bool HasComponentModules(WasmEnvironment environment) const;
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

	void FaultSyncedModules(WasmEnvironment environment, std::string_view reason);

	std::unique_ptr<WasmRuntime> runtime;
	WasmHostAdapter* hostAdapter = nullptr;
	std::vector<std::unique_ptr<WasmModule>> modules;
	std::vector<CoreModuleRecord> coreModules;
	WasmInstanceID nextInstanceID = 1;
	CoreDispatchRegistration coreDispatchRegistration{this};
};

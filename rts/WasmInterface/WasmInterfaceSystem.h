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

	// Native-query Core dispatch is the fast path used before a query is turned
	// into the owned WasmValue tree required by Component Model guests.  The
	// invocation list preserves the engine's environment ordering and permits a
	// visibility-filtered query copy for UI without imposing that copy on rules
	// worlds. `handled` is true only when at least one matching Core export ran.
	struct CoreCallinInvocation {
		WasmEnvironment environment;
		const void* query = nullptr;
		bool contributesResult = true;
	};
	bool DispatchCoreCallin(std::string_view name,
		const std::vector<CoreCallinInvocation>& invocations,
		WasmValue* valueResult, void* nativeResult, bool& handled,
		std::string& error);

	// The engine owns one live interface system. The alternative typed-host
	// shim reaches this registered instance so Core callins can retain their
	// pre-WasmValue fast path while still using the system's sorted module list,
	// environment selection and aggregation rules.
	static bool DispatchActiveCoreCallin(std::string_view name, const void* query,
		void* nativeResult, bool& handled, std::string& error);

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

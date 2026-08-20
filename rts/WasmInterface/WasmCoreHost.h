/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "WasmEnvironment.h"
#include "WasmRuntime.h"

struct NativeInterface;

enum class WasmCoreCallin : std::uint8_t {
	Invalid = 0,
	GameFrame,
	GameFramePost,
	Update,
	UnitCreated,
	UnitPreDamaged,
	AllowUnitCreation,
	AddConsoleLine,
	CommandNotify,
	DrawWorld,
};

// Native C++ host for the Spring Core-Wasm ABI. Each guest owns a separate
// Wasmtime store/linker/instance. Production dispatch keeps the host object
// behind a stable pointer owned by the static registry; vector reallocation
// moves unique_ptrs, not the pointed-to host.
class WasmCoreHost {
public:
	static bool Enabled();
	static bool Load(std::string moduleName, const std::vector<std::uint8_t>& moduleBytes,
		NativeInterface* nativeInterface, WasmEnvironment environment,
		const WasmRuntime& runtime, WasmModuleIdentity& identity, std::string& error);
	static void Unload(std::string_view moduleName);
	static void UnloadAll();
	static bool AnyActive();
	static bool AnyActive(WasmEnvironment environment);
	static bool HasModule(std::string_view moduleName);
	static bool ModuleFaulted(std::string_view moduleName);

	static WasmCoreCallin ResolveCallin(std::string_view name);

	// Resolve once and cache in WasmInterfaceSystem's CoreModuleRecord. The
	// pointer remains valid until that module is unloaded, at which point the
	// owning system removes the record as part of the same lifecycle operation.
	static WasmCoreHost* ModuleHandle(std::string_view moduleName)
	{
		return Find(moduleName);
	}
	static bool ModuleHasCallin(const WasmCoreHost* host, WasmCoreCallin callin)
	{
		return host != nullptr && host->HasCallin(callin);
	}
	static bool ModuleHasCallin(std::string_view moduleName, WasmCoreCallin callin)
	{
		return ModuleHasCallin(Find(moduleName), callin);
	}
	static bool ModuleHasCallin(std::string_view moduleName, std::string_view name)
	{
		return ModuleHasCallin(moduleName, ResolveCallin(name));
	}

	// Pointer + numeric-callin overload is the steady-state hot path. String
	// overloads remain only for legacy/diagnostic callers.
	static bool DispatchModule(WasmCoreHost* host, WasmCoreCallin callin,
		const void* query, void* result, std::string& error)
	{
		if (host == nullptr) {
			error = "Core Wasm module handle is null";
			return false;
		}
		return host->Invoke(callin, query, result, error);
	}
	static bool DispatchModule(std::string_view moduleName, WasmCoreCallin callin,
		const void* query, void* result, std::string& error);
	static bool DispatchModule(std::string_view moduleName, std::string_view name,
		const void* query, void* result, std::string& error);

	static bool DispatchEnvironment(WasmEnvironment environment, std::string_view name,
		const void* query, void* result, std::string& error);

	// Legacy benchmark seam: fan out to every active Core guest irrespective of
	// environment. Keep this only for existing alternate-host benchmarks.
	static bool DispatchCallin(std::string_view name, const void* query, void* result,
		std::string& error);

	static bool FaultModule(std::string_view moduleName, std::string reason);
	static std::size_t RemoveFaultedUnsynced();

	static bool ResetBudget(std::string_view moduleName, std::string& error);
	static bool FuelRemaining(std::string_view moduleName, std::uint64_t& fuel,
		std::string& error);

	~WasmCoreHost();
	WasmCoreHost(const WasmCoreHost&) = delete;
	WasmCoreHost& operator=(const WasmCoreHost&) = delete;

private:
	struct Backend;
	WasmCoreHost(std::string moduleName, WasmEnvironment environment,
		std::unique_ptr<Backend> backend);

	static WasmCoreHost* Find(std::string_view moduleName);
	bool HasCallin(WasmCoreCallin callin) const;
	bool Invoke(WasmCoreCallin callin, const void* query, void* result, std::string& error);
	bool InvokeGameFrame(const void* query, std::string& error);
	bool InvokeGameFramePost(const void* query, std::string& error);
	bool InvokeUpdate(const void* query, std::string& error);
	bool InvokeUnitCreated(const void* query, std::string& error);
	bool InvokeUnitPreDamaged(const void* query, void* result, std::string& error);
	bool InvokeAllowUnitCreation(const void* query, void* result, std::string& error);
	bool InvokeAddConsoleLine(const void* query, void* result, std::string& error);
	bool InvokeCommandNotify(const void* query, void* result, std::string& error);
	bool InvokeDrawWorld(std::string& error);
	bool ResetBudgetImpl(std::string& error);
	bool FuelRemainingImpl(std::uint64_t& fuel, std::string& error) const;
	void Fault(std::string reason);

	std::string moduleName;
	WasmEnvironment environment;
	std::unique_ptr<Backend> backend;
};

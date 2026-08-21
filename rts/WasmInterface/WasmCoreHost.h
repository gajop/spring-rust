/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "NativeInterface/WasmUiVisibility.h"
#include "WasmEnvironment.h"
#include "WasmRuntime.h"
#include "wasm/generated/WasmCallinRegistry.h"

struct NativeInterface;

namespace recoil::wasm::core::detail {

consteval std::uint16_t CoreCallinOrdinal(std::string_view name)
{
	for (std::size_t index = 0;
		index < sizeof(recoil::wasm::generated::kCallins) /
			sizeof(recoil::wasm::generated::kCallins[0]);
		++index) {
		if (name == recoil::wasm::generated::kCallins[index].name)
			return static_cast<std::uint16_t>(index + 1u);
	}
	return 0;
}

} // namespace recoil::wasm::core::detail

// Every generated Callins.def descriptor has a stable per-build numeric Core
// ID: generated-registry index + 1. Only the hand-specialized hot callins need
// named enum constants here; every other valid ID is represented by casting the
// generated ordinal returned by ResolveCallin(). This avoids a hand-maintained
// 126-entry enum while keeping the hot specialized comparisons compile-time.
enum class WasmCoreCallin : std::uint16_t {
	Invalid = 0,
	GameFrame = recoil::wasm::core::detail::CoreCallinOrdinal("GameFrame"),
	GameFramePost = recoil::wasm::core::detail::CoreCallinOrdinal("GameFramePost"),
	Update = recoil::wasm::core::detail::CoreCallinOrdinal("Update"),
	UnitCreated = recoil::wasm::core::detail::CoreCallinOrdinal("UnitCreated"),
	UnitPreDamaged = recoil::wasm::core::detail::CoreCallinOrdinal("UnitPreDamaged"),
	AllowUnitCreation = recoil::wasm::core::detail::CoreCallinOrdinal("AllowUnitCreation"),
	AddConsoleLine = recoil::wasm::core::detail::CoreCallinOrdinal("AddConsoleLine"),
	CommandNotify = recoil::wasm::core::detail::CoreCallinOrdinal("CommandNotify"),
	DrawWorld = recoil::wasm::core::detail::CoreCallinOrdinal("DrawWorld"),
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

	// Budget-window boundaries already hold the cached module handle. Reset
	// through that pointer so a frame boundary never repeats the module-name
	// lookup on the steady-state path.
	static bool ResetBudget(WasmCoreHost* host, std::string& error)
	{
		if (host == nullptr) {
			error = "Core Wasm module handle is null";
			return false;
		}
		return host->ResetBudgetImpl(error);
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
		// Do not construct the UI visibility scope for an unimplemented callin.
		// This is the negative-case fast path and also keeps its cost independent
		// of the guest's environment.
		if (!host->HasCallin(callin))
			return true;
		// Keep UI read visibility active for the whole guest invocation. All
		// nested Core imports and re-entrant callbacks inherit this perspective.
		WasmUiVisibility::ScopedContext uiContext(host->environment == WasmEnvironment::UI);
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

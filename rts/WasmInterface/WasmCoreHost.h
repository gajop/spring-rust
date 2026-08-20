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

// Internal numeric dispatch key. String names are resolved once at the outer
// engine seam; hot per-module Core dispatch must not repeatedly hash/compare
// callin names before reaching an already-cached Wasmtime export.
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
// Wasmtime store/linker/instance. The public static registry is keyed by the
// engine module name; production callers should prefer DispatchModule so
// WasmInterfaceSystem remains responsible for environment/order/aggregation.
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
	static bool ModuleHasCallin(std::string_view moduleName, WasmCoreCallin callin)
	{
		const WasmCoreHost* host = Find(moduleName);
		return host != nullptr && host->HasCallin(callin);
	}
	static bool ModuleHasCallin(std::string_view moduleName, std::string_view name)
	{
		return ModuleHasCallin(moduleName, ResolveCallin(name));
	}

	// Production dispatch: invoke exactly one already-ordered module. The
	// numeric overload is the hot path. The string overload is retained for
	// legacy callers and resolves the name once before dispatch.
	static bool DispatchModule(std::string_view moduleName, WasmCoreCallin callin,
		const void* query, void* result, std::string& error);
	static bool DispatchModule(std::string_view moduleName, std::string_view name,
		const void* query, void* result, std::string& error);

	// Convenience fan-out restricted to one environment. This intentionally
	// does not implement callin aggregation; callers needing Lua-compatible
	// aggregation should iterate modules in WasmInterfaceSystem and use
	// DispatchModule for each one.
	static bool DispatchEnvironment(WasmEnvironment environment, std::string_view name,
		const void* query, void* result, std::string& error);

	// Legacy benchmark seam: fan out to every active Core guest irrespective of
	// environment. Keep this only for existing alternate-host benchmarks.
	static bool DispatchCallin(std::string_view name, const void* query, void* result,
		std::string& error);

	// Fault state is sticky. Synced integration can retain a faulted host for
	// match-fatal reporting; unsynced integration can remove it at a safe point.
	static bool FaultModule(std::string_view moduleName, std::string reason);
	static std::size_t RemoveFaultedUnsynced();

	// Explicit deterministic accounting boundaries. ResetBudget restores both
	// the Spring host-work counter and Wasmtime fuel to the configured module
	// limits. It is never called implicitly by the host: the engine decides
	// whether the period is a frame, game tick, callback batch, or whole match.
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

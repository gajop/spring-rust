/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "NativeInterface/WasmUiVisibility.h"
#include "System/BenchmarkCallins.h"
#include "WasmCoreCallinId.h"
#include "WasmCoreDispatchPlan.h"
#include "WasmEnvironment.h"
#include "WasmRuntime.h"
#include "wasm/generated/WasmCallinRegistry.h"

struct NativeInterface;

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
	// Diagnostic name for a resolved module handle; dispatch carries the handle
	// and only needs the name when reporting an error.
	static std::string_view ModuleName(const WasmCoreHost* host);

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

	// Plan invokers for callins that cannot be reduced to filling raw slots:
	// variable payloads serialize into guest scratch memory first, and
	// generated callins marshal through the generated binding table.
	bool InvokeAddConsoleLine(const void* query, void* result, std::string& error);
	bool InvokeCommandNotify(const void* query, void* result, std::string& error);
	bool InvokeGenerated(WasmCoreCallin callin, const void* query, void* result,
		std::string& error);

	// The steady-state hot path. `plan` was resolved when the module was loaded
	// and holds, in one cache line, everything reaching the guest requires.
	static bool Dispatch(const recoil::wasm::core::WasmCoreDispatchPlan* plan,
		const void* query, void* result, std::string& error);

	// Used by the out-of-line plan error paths, which must not see Backend.
	static std::string FaultReason(const WasmCoreHost* host);
	static void FaultHost(WasmCoreHost* host, std::string reason);

	// Resolved dispatch plan for one callin, or null when the guest does not
	// export it. Callers cache this; it is stable for the module's lifetime.
	static const recoil::wasm::core::WasmCoreDispatchPlan* ModulePlan(
		const WasmCoreHost* host, WasmCoreCallin callin);

	static bool DispatchModule(WasmCoreHost* host, WasmCoreCallin callin,
		const void* query, void* result, std::string& error);
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

	// Faults are rare, so the sweep is flag-gated: an unsynced fault bumps this
	// counter and the next dispatch or Update() pays for the scan. Without a
	// pending fault the sweep must not touch the host registry at all.
	static std::size_t PendingUnsyncedFaults();
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
	static void RecountPendingUnsyncedFaults();
	bool HasCallin(WasmCoreCallin callin) const;
	const recoil::wasm::core::WasmCoreDispatchPlan* PlanFor(WasmCoreCallin callin) const;
	void BuildDispatchPlans();
	bool Invoke(WasmCoreCallin callin, const void* query, void* result, std::string& error);
	bool ResetBudgetImpl(std::string& error);
	bool FuelRemainingImpl(std::uint64_t& fuel, std::string& error) const;
	void Fault(std::string reason);

	std::string moduleName;
	WasmEnvironment environment;
	std::unique_ptr<Backend> backend;
};

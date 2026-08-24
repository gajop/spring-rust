/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>

#include "NativeInterface/WasmUiVisibility.h"
#include "System/BenchmarkCallins.h"
#include "WasmCoreAbi.h"
#include "WasmCoreCallinId.h"
#include "WasmInterface/runtime/WasmResources.h"

class WasmCoreHost;

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

// Mutable per-guest state that every dispatch touches. Grouped and
// cache-line-aligned so the budget charge and the fault check are one line
// rather than two ends of a multi-kilobyte backend object.
// The fault flag sits first so it shares a line with the budget counters the
// host-work charge reads; both are touched on every single dispatch.
struct alignas(64) HotGuestState {
	bool faulted;
	WasmExecutionBudget budget;
};

// Everything the steady-state dispatch of one callin into one guest needs,
// resolved when the module is loaded and packed into a single cache line.
//
// Without this, reaching a guest export meant walking the host object, its
// backend, the backend's binding table and the Wasmtime store in turn — a
// chain of dependent loads that is pure latency on a cold cache. Nothing the
// plan caches can change while the module is loaded.
struct alignas(64) WasmCoreDispatchPlan {
	using Invoke = bool (*)(const WasmCoreDispatchPlan& plan, const void* query,
		void* result, std::string& error);

	Invoke invoke = nullptr;
	wasmtime_context_t* context = nullptr;
	HotGuestState* hot = nullptr;
	WasmCoreHost* host = nullptr;
	wasmtime_func_t function{};
	WasmCoreCallin callin = WasmCoreCallin::Invalid;
	bool uiEnvironment = false;
};

#else

struct HotGuestState {
	bool faulted;
	WasmExecutionBudget budget;
};

struct WasmCoreDispatchPlan {
	WasmCoreHost* host = nullptr;
	HotGuestState* hot = nullptr;
	WasmCoreCallin callin = WasmCoreCallin::Invalid;
	bool uiEnvironment = false;
};

#endif

// Rejection, budget exhaustion and guest failure all need the owning host, so
// they stay out of line; the fast path below never touches it.
bool DispatchPlanRejected(const WasmCoreDispatchPlan* plan, std::string& error);
bool DispatchPlanExhausted(const WasmCoreDispatchPlan* plan, std::string& error);
bool DispatchPlanFailed(const WasmCoreDispatchPlan* plan, std::string& error);

#if defined(RECOIL_WASMTIME_AVAILABLE)

// Inline so the engine call site, the routing and the guest entry compile into
// one function. Every cross-translation-unit call on this path is a cold code
// line of its own once a frame of rendering has evicted the caches.
inline bool DispatchPlan(const WasmCoreDispatchPlan* plan, const void* query,
	void* result, std::string& error)
{
	if (plan == nullptr || plan->hot->faulted)
		return DispatchPlanRejected(plan, error);
	if (!plan->hot->budget.ChargeHost(1))
		return DispatchPlanExhausted(plan, error);
	// A Core callin is itself a guest callback from the engine's point of
	// view. Host imports such as create-unit may synchronously emit another
	// callin (UnitCreated, UnitPreDamaged, ...), so keep the callback nesting
	// state active for the whole guest invocation. Without this, a perfectly
	// ordinary authoritative mutation traps with "Wasm import re-entry
	// denied" before the nested callin can be dispatched.
	if (!plan->hot->budget.EnterCallback(true)) {
		error = "Core Wasm callback nesting limit rejected callin";
		return false;
	}
	struct CallbackScope {
		WasmExecutionBudget* budget;
		~CallbackScope() { budget->LeaveCallback(); }
	} callbackScope{&plan->hot->budget};

	// Keep UI read visibility active for the whole guest invocation. All
	// nested Core imports and re-entrant callbacks inherit this perspective.
	const auto visibilityStage = spring::benchmark_callins::BeginStage(
		spring::benchmark_callins::Stage::Visibility);
	WasmUiVisibility::ScopedContext uiContext(plan->uiEnvironment);
	spring::benchmark_callins::End(visibilityStage);

	if (plan->invoke(*plan, query, result, error))
		return true;
	return DispatchPlanFailed(plan, error);
}

#endif

// Error paths need the owning module; the hot path never reads this.
inline WasmCoreHost* PlanHost(const WasmCoreDispatchPlan* plan)
{
	return plan != nullptr ? plan->host : nullptr;
}

} // namespace recoil::wasm::core

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <array>
#include <cstdint>
#include <string>
#include <string_view>

#include "NativeInterface/WasmUiVisibility.h"
#include "WasmCoreBindings.h"

namespace recoil::wasm::core::generated {

inline wasm_trap_t* Trap(std::string_view message)
{
	return wasmtime_trap_new(message.data(), message.size());
}

inline std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

class ImportGuard {
public:
	ImportGuard(HostState* state, std::uint64_t work)
		: uiContext(state != nullptr && state->environment == WasmEnvironment::UI)
	{
		Initialize(state, work, ownedError);
	}

	ImportGuard(HostState* state, std::uint64_t work, std::string& error)
		: uiContext(state != nullptr && state->environment == WasmEnvironment::UI)
	{
		Initialize(state, work, error);
	}

	~ImportGuard()
	{
		if (entered && budget != nullptr)
			budget->LeaveImport();
	}

	ImportGuard(const ImportGuard&) = delete;
	ImportGuard& operator=(const ImportGuard&) = delete;
	ImportGuard(ImportGuard&&) = delete;
	ImportGuard& operator=(ImportGuard&&) = delete;

	bool Ok() const { return entered; }
	std::string_view Error() const { return ownedError; }

private:
	void Initialize(HostState* state, std::uint64_t work, std::string& error)
	{
		if (state == nullptr) {
			error = "core Wasm generated host state is null";
			return;
		}

		budget = state->budget;
		if (budget == nullptr) {
			entered = true;
			return;
		}
		const bool allowReentry = budget->CallbackDepth() != 0 &&
			budget->CallbackReentryAllowed();
		if (!budget->EnterImport(allowReentry)) {
			error = "Wasm import re-entry denied";
			return;
		}
		entered = true;
		if (!budget->ChargeHost(work)) {
			budget->LeaveImport();
			entered = false;
			error = "Wasm callout host-work budget exhausted";
		}
	}

	// UI modules install the same perspective as the Component adapter. For
	// non-UI modules this is a literal no-op, preserving any outer restricted
	// context and avoiding a thread-local save/restore on the hot path.
	WasmUiVisibility::ConditionalScopedContext uiContext;
	WasmExecutionBudget* budget = nullptr;
	bool entered = false;
	std::string ownedError;
};

inline bool EnsureMemory(HostState* state, wasmtime_caller_t* caller, std::string& error)
{
	if (state == nullptr) {
		error = "core Wasm generated host state is null";
		return false;
	}
	if (state->memory.IsBound())
		return true;
	return state->memory.BindFromCaller(caller, error);
}

class CallbackGuard {
public:
	explicit CallbackGuard(WasmExecutionBudget* executionBudget)
		: budget(executionBudget)
	{
		entered = budget == nullptr || budget->EnterCallback(true);
	}

	~CallbackGuard()
	{
		if (entered && budget != nullptr)
			budget->LeaveCallback();
	}

	bool Ok() const { return entered; }

private:
	WasmExecutionBudget* budget = nullptr;
	bool entered = false;
};

inline bool ResolveCallback(HostState& state, wasmtime_caller_t* caller,
	std::string& error)
{
	if (state.callbackDispatchBound)
		return true;
	if (caller == nullptr) {
		error = "Core Wasm callback has no active caller";
		return false;
	}
	constexpr char exportName[] = "spring:callback/dispatch";
	wasmtime_extern_t item{};
	if (!wasmtime_caller_export_get(caller, exportName, sizeof(exportName) - 1, &item)) {
		error = "Core Wasm callback requires export spring:callback/dispatch";
		return false;
	}
	if (item.kind != WASMTIME_EXTERN_FUNC) {
		wasmtime_extern_delete(&item);
		error = "Core Wasm callback dispatch export is not a function";
		return false;
	}
	const wasm_valkind_t params[] = {WASM_I32, WASM_I32};
	if (!FunctionHasSignature(wasmtime_caller_context(caller), item.of.func,
			params, 2, nullptr, 0)) {
		wasmtime_extern_delete(&item);
		error = "Core Wasm callback dispatch export must be (i32,i32)->()";
		return false;
	}
	state.callbackDispatch = item.of.func;
	state.callbackDispatchBound = true;
	wasmtime_extern_delete(&item);
	return true;
}

inline bool DispatchCallback(HostState& state, wasmtime_caller_t* caller,
	std::uint32_t callbackID, std::uint32_t userData, std::string& error)
{
	if (!ResolveCallback(state, caller, error))
		return false;
	CallbackGuard guard(state.budget);
	if (!guard.Ok()) {
		error = "Core Wasm callback nesting limit rejected callback";
		return false;
	}
	std::array<wasmtime_val_raw_t, 2> slots{};
	slots[0].i32 = static_cast<std::int32_t>(callbackID);
	slots[1].i32 = static_cast<std::int32_t>(userData);
	wasm_trap_t* trap = nullptr;
	if (wasmtime_error_t* callError = wasmtime_func_call_unchecked(
			wasmtime_caller_context(caller), &state.callbackDispatch,
			slots.data(), slots.size(), &trap);
		callError != nullptr) {
		error = "Core Wasm callback dispatch failed: " + ErrorMessage(callError);
		if (trap != nullptr)
			error += ": " + TrapMessage(trap);
		return false;
	}
	if (trap != nullptr) {
		error = "Core Wasm callback dispatch trapped: " + TrapMessage(trap);
		return false;
	}
	return true;
}

struct CallbackContext {
	HostState* state = nullptr;
	wasmtime_caller_t* caller = nullptr;
	std::uint32_t callbackID = 0;
	std::uint32_t userData = 0;
	bool success = true;
	std::string error;
};

inline void InvokeCallback(void* data)
{
	auto* context = static_cast<CallbackContext*>(data);
	if (context == nullptr || context->state == nullptr)
		return;
	context->success = DispatchCallback(*context->state, context->caller,
		context->callbackID, context->userData, context->error);
}

} // namespace recoil::wasm::core::generated
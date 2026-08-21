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
		, errorStorage(&ownedError)
	{
		Initialize(state, work);
	}

	ImportGuard(HostState* state, std::uint64_t work, std::string& error)
		: uiContext(state != nullptr && state->environment == WasmEnvironment::UI)
		, errorStorage(&error)
	{
		Initialize(state, work);
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

	bool Ok() const { return entered && !failed; }
	std::string_view Error() const
	{
		return errorStorage == nullptr ? std::string_view{} : std::string_view(*errorStorage);
	}

	// Variable-size Core inputs can make host work scale with guest-controlled
	// lengths after the import has already been entered. Charge that work before
	// any host allocation/adaptation/native iteration. This is intentionally a
	// separate cheap counter operation so zero-copy paths do not pay for bytes
	// they merely validate and borrow unless the native API can consume them.
	bool Charge(std::uint64_t work)
	{
		if (!entered || failed)
			return false;
		if (budget == nullptr || budget->ChargeHost(work))
			return true;
		failed = true;
		SetError("Wasm callout host-work budget exhausted");
		return false;
	}

private:
	void SetError(std::string_view message)
	{
		if (errorStorage != nullptr)
			errorStorage->assign(message.data(), message.size());
	}

	void Initialize(HostState* state, std::uint64_t work)
	{
		if (state == nullptr) {
			SetError("core Wasm generated host state is null");
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
			SetError("Wasm import re-entry denied");
			return;
		}
		entered = true;
		if (!budget->ChargeHost(work)) {
			budget->LeaveImport();
			entered = false;
			SetError("Wasm callout host-work budget exhausted");
		}
	}

	// UI modules install the same perspective as the Component adapter. For
	// non-UI modules this is a literal no-op, preserving any outer restricted
	// context and avoiding a thread-local save/restore on the hot path.
	WasmUiVisibility::ConditionalScopedContext uiContext;
	WasmExecutionBudget* budget = nullptr;
	bool entered = false;
	bool failed = false;
	std::string ownedError;
	std::string* errorStorage = nullptr;
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

inline bool CheckResultBytes(const HostState* state, std::size_t bytes)
{
	return state == nullptr || state->budget == nullptr || state->budget->CheckResultSize(bytes);
}

inline bool CheckResultNodes(const HostState* state, std::size_t nodes)
{
	return state == nullptr || nodes <= state->maxValueNodes;
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
	if (caller == nullptr) {
		error = "Core Wasm callback has no active caller";
		return false;
	}
	state.context = wasmtime_caller_context(caller);
	if (state.callbackDispatchBound)
		return true;
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
	if (!FunctionHasSignature(state.context, item.of.func, params, 2, nullptr, 0)) {
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
			state.context, &state.callbackDispatch, slots.data(), slots.size(), &trap);
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

inline bool DispatchRetainedCallback(HostState& state, std::uint32_t callbackID,
	std::uint32_t userData, std::string& error)
{
	if (!state.callbackDispatchBound || state.context == nullptr) {
		error = "Core Wasm retained callback dispatch is not bound";
		return false;
	}
	WasmUiVisibility::ConditionalScopedContext uiContext(
		state.environment == WasmEnvironment::UI);
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
			state.context, &state.callbackDispatch, slots.data(), slots.size(), &trap);
		callError != nullptr) {
		error = "Core Wasm retained callback dispatch failed: " + ErrorMessage(callError);
		if (trap != nullptr)
			error += ": " + TrapMessage(trap);
		return false;
	}
	if (trap != nullptr) {
		error = "Core Wasm retained callback dispatch trapped: " + TrapMessage(trap);
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

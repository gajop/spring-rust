/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>
#include <string_view>

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
	ImportGuard(HostState* state, std::uint64_t work, std::string& error)
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

	~ImportGuard()
	{
		if (entered && budget != nullptr)
			budget->LeaveImport();
	}

	bool Ok() const { return entered; }

private:
	WasmExecutionBudget* budget = nullptr;
	bool entered = false;
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

} // namespace recoil::wasm::core::generated

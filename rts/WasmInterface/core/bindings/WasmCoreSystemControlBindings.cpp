/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreSystemControlBindings.h"

#include <cstdint>

#include "WasmCoreGeneratedSupport.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::CallbackContext;
using generated::ImportGuard;
using generated::InvokeCallback;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

wasm_trap_t* CallAsTeam(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->systemControl == nullptr ||
		state->native->systemControl->CallAsTeam == nullptr)
		return Trap("CallAsTeam Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("CallAsTeam Core ABI signature mismatch");

	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	callback.state = state;
	callback.caller = caller;
	callback.callbackID = static_cast<std::uint32_t>(slots[1].i32);
	callback.userData = static_cast<std::uint32_t>(slots[2].i32);

	CallAsTeamQuery query{};
	query.teamID = slots[0].i32;
	query.callback = InvokeCallback;
	query.userData = &callback;
	CallAsTeamResult result{};
	state->native->systemControl->CallAsTeam(&query, &result);
	if (!callback.success)
		return Trap(callback.error.empty() ? "Core CallAsTeam callback failed" : callback.error);

	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, NativeErrorCode(result.error)));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:system-control", 21,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterSystemControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->systemControl == nullptr) {
		error = "cannot register SystemControl Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t results[] = {WASM_I64};
	return Define(linker, "call-as-team", MakeFuncType(params, 3, results, 1),
		CallAsTeam, state, error);
}

#endif

} // namespace recoil::wasm::core

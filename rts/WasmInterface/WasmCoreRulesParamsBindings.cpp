/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreRulesParamsBindings.h"

#include <bit>
#include <cstdint>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

void ReturnStatus(wasmtime_val_raw_t* slots, Status status)
{
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(0, static_cast<std::int32_t>(status)));
}

wasm_trap_t* GetUnitRulesParamF32(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->rulesParams == nullptr ||
		state->native->rulesParams->GetUnitRulesParam == nullptr)
		return Trap("GetUnitRulesParam Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitRulesParamF32 Core ABI signature mismatch");

	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> name;
	if (!name.Read(state->memory, static_cast<std::uint32_t>(slots[1].i32),
			static_cast<std::uint32_t>(slots[2].i32))) {
		ReturnStatus(slots, Status::OutOfBounds);
		return nullptr;
	}

	GetUnitRulesParamQuery query{};
	query.unitID = slots[0].i32;
	query.paramName = name.c_str();
	GetUnitRulesParamResult result{};
	state->native->rulesParams->GetUnitRulesParam(&query, &result);
	const std::int32_t nativeError = NativeErrorCode(result.error);
	if (nativeError != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, nativeError));
		return nullptr;
	}
	if (!result.exists) {
		ReturnStatus(slots, Status::NotFound);
		return nullptr;
	}
	if (result.value.type != RULESPARAM_TYPE_FLOAT) {
		ReturnStatus(slots, Status::InvalidArgument);
		return nullptr;
	}
	const std::uint32_t bits = std::bit_cast<std::uint32_t>(result.value.floatValue);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(bits, 0));
	return nullptr;
}

wasm_trap_t* SetUnitRulesParamF32(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->rulesParams == nullptr ||
		state->native->rulesParams->SetUnitRulesParam == nullptr)
		return Trap("SetUnitRulesParam Core binding is unavailable");
	if (slots == nullptr || slotCount != 5)
		return Trap("SetUnitRulesParamF32 Core ABI signature mismatch");

	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> name;
	if (!name.Read(state->memory, static_cast<std::uint32_t>(slots[1].i32),
			static_cast<std::uint32_t>(slots[2].i32))) {
		ReturnStatus(slots, Status::OutOfBounds);
		return nullptr;
	}

	SetUnitRulesParamQuery query{};
	query.unitID = slots[0].i32;
	query.paramName = name.c_str();
	query.value.type = RULESPARAM_TYPE_FLOAT;
	query.value.floatValue = slots[3].f32;
	query.los = slots[4].i32;
	SetUnitRulesParamResult result{};
	state->native->rulesParams->SetUnitRulesParam(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, NativeErrorCode(result.error)));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:rules-params", 19,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterRulesParamsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->rulesParams == nullptr) {
		error = "cannot register RulesParams Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t getParams[] = {WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t setParams[] = {WASM_I32, WASM_I32, WASM_I32, WASM_F32, WASM_I32};
	const wasm_valkind_t result[] = {WASM_I64};
	if (!Define(linker, "get-unit-rules-param-f32",
			MakeFuncType(getParams, 3, result, 1), GetUnitRulesParamF32, state, error))
		return false;
	return Define(linker, "set-unit-rules-param-f32",
		MakeFuncType(setParams, 5, result, 1), SetUnitRulesParamF32, state, error);
}

#endif

} // namespace recoil::wasm::core

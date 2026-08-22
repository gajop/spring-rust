/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreBenchmarkBindings.h"

#include <array>
#include <bit>
#include <cstdint>
#include <limits>
#include <string>
#include <string_view>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

std::int32_t ErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

wasm_trap_t* GetTimerMicros(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->profiling == nullptr ||
		state->native->profiling->GetTimerMicros == nullptr)
		return Trap("GetTimerMicros Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetTimerMicros Core ABI signature mismatch");

	ImportGuard guard(state, 1);
	if (!guard.Ok())
		return Trap(guard.Error());

	GetTimerMicrosQuery query{};
	GetTimerMicrosResult result{};
	state->native->profiling->GetTimerMicros(&query, &result);
	if (result.error != nullptr)
		return Trap("GetTimerMicros NativeInterface call failed");
	slots[0].i64 = static_cast<std::int64_t>(result.timer);
	return nullptr;
}

wasm_trap_t* SendLuaRulesMsg(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->messages == nullptr ||
		state->native->messages->SendLuaRulesMsg == nullptr)
		return Trap("SendLuaRulesMsg Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("SendLuaRulesMsg Core ABI signature mismatch");

	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GuestCString<> message;
	if (!message.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32),
			static_cast<std::uint32_t>(slots[1].i32))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	SendLuaRulesQuery query{message.c_str()};
	SendLuaRulesResult result{};
	state->native->messages->SendLuaRulesMsg(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, ErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* SendLuaUIMsg(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->messages == nullptr ||
		state->native->messages->SendLuaUIMsg == nullptr)
		return Trap("SendLuaUIMsg Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("SendLuaUIMsg Core ABI signature mismatch");

	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GuestCString<> message;
	GuestCString<> mode;
	if (!message.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32),
			static_cast<std::uint32_t>(slots[1].i32)) ||
		!mode.Read(state->memory, static_cast<std::uint32_t>(slots[2].i32),
			static_cast<std::uint32_t>(slots[3].i32))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	SendLuaUIQuery query{message.c_str(), mode.c_str()};
	SendLuaUIResult result{};
	state->native->messages->SendLuaUIMsg(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, ErrorCode(result.error)));
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
		return Trap("SetUnitRulesParam Core ABI signature mismatch");

	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GuestCString<> name;
	// Validate/copy all guest string input before mutating engine state.
	if (!name.Read(state->memory, static_cast<std::uint32_t>(slots[1].i32),
			static_cast<std::uint32_t>(slots[2].i32))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	RulesParamValue value{};
	value.type = RULESPARAM_TYPE_FLOAT;
	value.floatValue = slots[3].f32;
	SetUnitRulesParamQuery query{slots[0].i32, name.c_str(), value, slots[4].i32};
	SetUnitRulesParamResult result{};
	state->native->rulesParams->SetUnitRulesParam(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, ErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitRulesParamF32(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->rulesParams == nullptr ||
		state->native->rulesParams->GetUnitRulesParam == nullptr)
		return Trap("GetUnitRulesParam Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitRulesParam Core ABI signature mismatch");

	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GuestCString<> name;
	if (!name.Read(state->memory, static_cast<std::uint32_t>(slots[1].i32),
			static_cast<std::uint32_t>(slots[2].i32))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	GetUnitRulesParamQuery query{slots[0].i32, name.c_str()};
	GetUnitRulesParamResult result{};
	state->native->rulesParams->GetUnitRulesParam(&query, &result);
	const std::int32_t nativeError = ErrorCode(result.error);
	if (nativeError != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, nativeError));
		return nullptr;
	}
	if (!result.exists) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::NotFound)));
		return nullptr;
	}
	if (result.value.type != RULESPARAM_TYPE_FLOAT) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::InvalidState)));
		return nullptr;
	}
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(std::bit_cast<std::uint32_t>(result.value.floatValue), 0));
	return nullptr;
}

wasm_trap_t* GetGroundOrigHeight(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetGroundOrigHeight == nullptr)
		return Trap("GetGroundOrigHeight Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetGroundOrigHeight Core ABI signature mismatch");

	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());

	GetGroundOrigHeightQuery query{slots[0].f32, slots[1].f32};
	GetGroundOrigHeightResult result{};
	state->native->terrain->GetGroundOrigHeight(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(
		std::bit_cast<std::uint32_t>(result.height), ErrorCode(result.error)));
	return nullptr;
}

// Pure transport benchmark. This intentionally does not copy or inspect the
// payload; it measures the steady-state cost of an unchecked Core import plus
// the range validation production zero-copy inputs would still require.
wasm_trap_t* ConsumeString(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 2)
		return Trap("ConsumeString Core ABI signature mismatch");

	ImportGuard guard(state, 1);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t pointer = static_cast<std::uint32_t>(slots[0].i32);
	const std::uint32_t length = static_cast<std::uint32_t>(slots[1].i32);
	if (!state->memory.Contains(pointer, length)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	slots[0].i64 = static_cast<std::int64_t>(PackU32(length, 0));
	return nullptr;
}

wasm_trap_t* ConsumeF32List(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 2)
		return Trap("ConsumeF32List Core ABI signature mismatch");

	ImportGuard guard(state, 1);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t pointer = static_cast<std::uint32_t>(slots[0].i32);
	const std::uint32_t count = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint64_t bytes = static_cast<std::uint64_t>(count) * sizeof(float);
	if (bytes > std::numeric_limits<std::size_t>::max() ||
		!state->memory.Contains(pointer, static_cast<std::size_t>(bytes))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	slots[0].i64 = static_cast<std::int64_t>(PackU32(count, 0));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* moduleName, const char* functionName,
	wasm_functype_t* type, wasmtime_func_unchecked_callback_t callback,
	HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker,
		moduleName, std::char_traits<char>::length(moduleName),
		functionName, std::char_traits<char>::length(functionName),
		type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterBenchmarkImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr) {
		error = "cannot register Core benchmark imports without linker/host/native API";
		return false;
	}

	const wasm_valkind_t i64Result[] = {WASM_I64};
	if (!Define(linker, "spring:profiling", "get-timer-micros",
			MakeFuncType(nullptr, 0, i64Result, 1), GetTimerMicros, state, error))
		return false;

	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32};
		if (!Define(linker, "spring:messages", "send-lua-rules-msg",
				MakeFuncType(params, 2, i64Result, 1), SendLuaRulesMsg, state, error) ||
			!Define(linker, "spring:benchmark", "consume-string",
				MakeFuncType(params, 2, i64Result, 1), ConsumeString, state, error) ||
			!Define(linker, "spring:benchmark", "consume-f32-list",
				MakeFuncType(params, 2, i64Result, 1), ConsumeF32List, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
		if (!Define(linker, "spring:messages", "send-lua-ui-msg",
				MakeFuncType(params, 4, i64Result, 1), SendLuaUIMsg, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_F32, WASM_I32};
		if (!Define(linker, "spring:rules-params", "set-unit-rules-param-f32",
				MakeFuncType(params, 5, i64Result, 1), SetUnitRulesParamF32, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32};
		if (!Define(linker, "spring:rules-params", "get-unit-rules-param-f32",
				MakeFuncType(params, 3, i64Result, 1), GetUnitRulesParamF32, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_F32, WASM_F32};
		if (!Define(linker, "spring:terrain", "get-ground-orig-height",
				MakeFuncType(params, 2, i64Result, 1), GetGroundOrigHeight, state, error))
			return false;
	}
	return true;
}

#endif

} // namespace recoil::wasm::core

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreBindings.h"

#include <array>
#include <cstdint>
#include <cstring>
#include <string_view>

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

wasm_trap_t* Trap(std::string_view message)
{
	return wasmtime_trap_new(message.data(), message.size());
}

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

bool EnsureMemory(HostState* state, wasmtime_caller_t* caller, std::string& error)
{
	if (state == nullptr) {
		error = "core Wasm host state is null";
		return false;
	}
	if (state->memory.IsBound())
		return true;
	return state->memory.BindFromCaller(caller, error);
}

void EncodeF32LE(float value, std::uint8_t* output)
{
	static_assert(sizeof(float) == sizeof(std::uint32_t));
	std::uint32_t bits = 0;
	std::memcpy(&bits, &value, sizeof(bits));
	output[0] = static_cast<std::uint8_t>(bits);
	output[1] = static_cast<std::uint8_t>(bits >> 8);
	output[2] = static_cast<std::uint8_t>(bits >> 16);
	output[3] = static_cast<std::uint8_t>(bits >> 24);
}

wasm_trap_t* GetUnitDefID(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitDefID == nullptr)
		return Trap("GetUnitDefID host binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetUnitDefID core ABI signature mismatch");

	GetUnitDefIDQuery query{};
	query.unitID = slots[0].i32;
	GetUnitDefIDResult result{};
	state->native->unitsInfo->GetUnitDefID(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.unitDefID, NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitPosition(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitPosition == nullptr)
		return Trap("GetUnitPosition host binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitPosition core ABI signature mismatch");

	std::string memoryError;
	if (!EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t flags = static_cast<std::uint32_t>(slots[1].i32);
	if ((flags & ~(POSITION_MID | POSITION_AIM)) != 0) {
		slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument);
		return nullptr;
	}

	GetUnitPositionQuery query{};
	query.unitID = slots[0].i32;
	query.options.midPos = (flags & POSITION_MID) != 0;
	query.options.aimPos = (flags & POSITION_AIM) != 0;
	GetUnitPositionResult result{};
	state->native->unitsInfo->GetUnitPosition(&query, &result);

	const std::int32_t errorCode = NativeErrorCode(result.error);
	if (errorCode != 0) {
		slots[0].i32 = errorCode;
		return nullptr;
	}

	const std::uint32_t output = static_cast<std::uint32_t>(slots[2].i32);
	std::array<std::uint8_t, 12> wire{};
	EncodeF32LE(result.position.x, wire.data() + 0);
	EncodeF32LE(result.position.y, wire.data() + 4);
	EncodeF32LE(result.position.z, wire.data() + 8);
	if (!state->memory.Write(output, wire.data(), wire.size())) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}

	slots[0].i32 = 0;
	return nullptr;
}

bool DefineUnchecked(wasmtime_linker_t* linker, const char* moduleName,
	const char* functionName, wasm_functype_t* type, wasmtime_func_unchecked_callback_t callback,
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

bool RegisterFastImports(wasmtime_linker_t* linker, HostState* state, std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr) {
		error = "cannot register core Wasm imports without linker/host state/native API";
		return false;
	}

	{
		const wasm_valkind_t params[] = {WASM_I32};
		const wasm_valkind_t results[] = {WASM_I64};
		if (!DefineUnchecked(linker, "spring:units-info", "get-unit-def-id",
				MakeFuncType(params, 1, results, 1), GetUnitDefID, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32};
		const wasm_valkind_t results[] = {WASM_I32};
		if (!DefineUnchecked(linker, "spring:units-info", "get-unit-position",
				MakeFuncType(params, 3, results, 1), GetUnitPosition, state, error))
			return false;
	}

	return true;
}

bool BindGuestMemory(HostState& state, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, std::string& error)
{
	return state.memory.BindFromInstance(context, instance, error);
}

bool InstanceBindings::Bind(wasmtime_context_t* context, const wasmtime_instance_t& instance,
	std::string& error)
{
	if (!BindGuestMemory(host, context, instance, error))
		return false;
	constexpr char gameFrameName[] = "spring:callin/game-frame";
	return gameFrame.Resolve(context, instance, gameFrameName, sizeof(gameFrameName) - 1,
		true, error);
}

#endif

} // namespace recoil::wasm::core

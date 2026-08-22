/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreUnitControlBindings.h"

#include <array>
#include <cstdint>
#include <limits>
#include <span>
#include <vector>

#include "WasmCoreGeneratedSupport.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

wasm_trap_t* GiveOrderToUnit(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->syncedCtrl == nullptr ||
		state->native->syncedCtrl->unit == nullptr ||
		state->native->syncedCtrl->unit->GiveOrderToUnit == nullptr)
		return Trap("GiveOrderToUnit Core binding is unavailable");
	if (slots == nullptr || slotCount != 6)
		return Trap("GiveOrderToUnit Core ABI signature mismatch");

	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());

	const std::uint32_t paramsPointer = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint32_t paramCount = static_cast<std::uint32_t>(slots[3].i32);
	if (paramCount > std::numeric_limits<std::size_t>::max() / sizeof(float)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::InvalidArgument)));
		return nullptr;
	}

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	// The benchmark's move order has three parameters, so its hot path never
	// allocates. Larger generic commands retain the same ABI with a bounded
	// heap fallback owned only for this synchronous call.
	std::array<float, 32> inlineParams{};
	std::vector<float> heapParams;
	std::span<float> params;
	if (paramCount <= inlineParams.size()) {
		params = std::span<float>(inlineParams.data(), paramCount);
	} else {
		heapParams.resize(paramCount);
		params = std::span<float>(heapParams.data(), heapParams.size());
	}
	// Copy and endian-normalize every guest parameter before mutating the
	// engine. NativeInterface never retains a guest linear-memory pointer.
	if (!state->memory.ReadF32SliceLE(paramsPointer, params)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	GiveOrderToUnitQuery query{};
	query.unitID = slots[0].i32;
	query.cmdID = slots[1].i32;
	query.params = params.empty() ? nullptr : params.data();
	query.paramCount = paramCount;
	query.options = static_cast<std::uint32_t>(slots[4].i32);
	query.timeout = slots[5].i32;
	GiveOrderToUnitResult result{};
	state->native->syncedCtrl->unit->GiveOrderToUnit(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, NativeErrorCode(result.error)));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:unit-control", 19,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterUnitControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->syncedCtrl == nullptr || state->native->syncedCtrl->unit == nullptr) {
		error = "cannot register UnitControl Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t params[] = {
		WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
	};
	const wasm_valkind_t results[] = {WASM_I64};
	return Define(linker, "give-order-to-unit",
		MakeFuncType(params, 6, results, 1), GiveOrderToUnit, state, error);
}

#endif

} // namespace recoil::wasm::core

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreMathExtraBindings.h"

#include <cstdint>
#include <span>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreWire.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::EnsureMemory;
using generated::ImportGuard;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

wasm_trap_t* Normalize(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->mathExtra == nullptr ||
		state->native->mathExtra->Normalize == nullptr)
		return Trap("Normalize Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("Normalize Core ABI signature mismatch");

	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	Float3 value{slots[0].f32, slots[1].f32, slots[2].f32};
	NormalizeQuery query{&value};
	NormalizeResult result{};
	state->native->mathExtra->Normalize(&query, &result);
	const std::int32_t status = NativeErrorCode(result.error);
	if (status != 0) {
		slots[0].i32 = status;
		return nullptr;
	}

	std::span<std::uint8_t> output;
	if (!state->memory.MutableView(static_cast<std::uint32_t>(slots[3].i32), 16u, output)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	WireWriter writer(output);
	if (!writer.F32(value.x) || !writer.F32(value.y) || !writer.F32(value.z) ||
		!writer.F32(result.length) || !writer.Finish(4)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::Internal);
		return nullptr;
	}
	slots[0].i32 = 0;
	return nullptr;
}

} // namespace

bool RegisterMathExtraImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->mathExtra == nullptr) {
		error = "cannot register MathExtra Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t params[] = {WASM_F32, WASM_F32, WASM_F32, WASM_I32};
	const wasm_valkind_t results[] = {WASM_I32};
	wasm_functype_t* type = MakeFuncType(params, 4, results, 1);
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:math-extra", 17, "normalize", 9, type, Normalize, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

#endif

} // namespace recoil::wasm::core

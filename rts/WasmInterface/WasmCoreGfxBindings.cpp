/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreGfxBindings.h"

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

wasm_trap_t* Vertex(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->Vertex == nullptr)
		return Trap("Gfx Vertex Core binding is unavailable");
	if (slots == nullptr || slotCount != 5)
		return Trap("Gfx Vertex Core ABI signature mismatch");
	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());
	GfxVertexQuery query{
		slots[0].f32,
		slots[1].f32,
		slots[2].f32,
		slots[3].f32,
		static_cast<std::uint32_t>(slots[4].i32),
	};
	GfxEmptyResult result{};
	state->native->gfx->Vertex(&query, &result);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* BeginEnd(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->BeginEnd == nullptr)
		return Trap("Gfx BeginEnd Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("Gfx BeginEnd Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	callback.state = state;
	callback.caller = caller;
	callback.callbackID = static_cast<std::uint32_t>(slots[1].i32);
	callback.userData = static_cast<std::uint32_t>(slots[2].i32);
	GfxBeginEndQuery query{
		static_cast<std::uint32_t>(slots[0].i32),
		InvokeCallback,
		&callback,
	};
	GfxEmptyResult result{};
	state->native->gfx->BeginEnd(&query, &result);
	if (!callback.success)
		return Trap(callback.error.empty() ? "Core Gfx callback failed" : callback.error);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:gfx", 10,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterGfxImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->gfx == nullptr) {
		error = "cannot register Gfx Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t status[] = {WASM_I32};
	const wasm_valkind_t vertexParams[] = {
		WASM_F32, WASM_F32, WASM_F32, WASM_F32, WASM_I32,
	};
	if (!Define(linker, "vertex", MakeFuncType(vertexParams, 5, status, 1),
			Vertex, state, error))
		return false;
	const wasm_valkind_t beginEndParams[] = {WASM_I32, WASM_I32, WASM_I32};
	return Define(linker, "begin-end",
		MakeFuncType(beginEndParams, 3, status, 1), BeginEnd, state, error);
}

#endif

} // namespace recoil::wasm::core

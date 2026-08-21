/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreGfxBindings.h"

#include <cstdint>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"

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

void InitCallback(CallbackContext& callback, HostState* state,
	wasmtime_caller_t* caller, std::int32_t callbackID, std::int32_t userData)
{
	callback.state = state;
	callback.caller = caller;
	callback.callbackID = static_cast<std::uint32_t>(callbackID);
	callback.userData = static_cast<std::uint32_t>(userData);
}

wasm_trap_t* CallbackTrap(const CallbackContext& callback)
{
	return Trap(callback.error.empty() ? "Core Gfx callback failed" : callback.error);
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
	InitCallback(callback, state, caller, slots[1].i32, slots[2].i32);
	GfxBeginEndQuery query{
		static_cast<std::uint32_t>(slots[0].i32),
		InvokeCallback,
		&callback,
	};
	GfxEmptyResult result{};
	state->native->gfx->BeginEnd(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* ActiveFBO(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->ActiveFBO == nullptr)
		return Trap("Gfx ActiveFBO Core binding is unavailable");
	if (slots == nullptr || slotCount != 5)
		return Trap("Gfx ActiveFBO Core ABI signature mismatch");
	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[3].i32, slots[4].i32);
	GfxActiveFBOQuery query{
		static_cast<std::uint32_t>(slots[0].i32),
		static_cast<std::uint32_t>(slots[1].i32),
		slots[2].i32 != 0,
		InvokeCallback,
		&callback,
	};
	GfxEmptyResult result{};
	state->native->gfx->ActiveFBO(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* ActiveShader(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->ActiveShader == nullptr)
		return Trap("Gfx ActiveShader Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("Gfx ActiveShader Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[1].i32, slots[2].i32);
	GfxActiveShaderQuery query{
		static_cast<std::uint32_t>(slots[0].i32),
		InvokeCallback,
		&callback,
	};
	GfxEmptyResult result{};
	state->native->gfx->ActiveShader(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* CreateList(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->CreateList == nullptr)
		return Trap("Gfx CreateList Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("Gfx CreateList Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[0].i32, slots[1].i32);
	GfxCallbackQuery query{InvokeCallback, &callback};
	GfxUIntResult result{};
	state->native->gfx->CreateList(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.value, NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* DrawFuncAtUnit(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->DrawFuncAtUnit == nullptr)
		return Trap("Gfx DrawFuncAtUnit Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("Gfx DrawFuncAtUnit Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[2].i32, slots[3].i32);
	GfxDrawFuncAtUnitQuery query{
		slots[0].i32,
		slots[1].i32 != 0,
		InvokeCallback,
		&callback,
	};
	GfxEmptyResult result{};
	state->native->gfx->DrawFuncAtUnit(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* PushPopMatrix(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->PushPopMatrix == nullptr)
		return Trap("Gfx PushPopMatrix Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("Gfx PushPopMatrix Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[0].i32, slots[1].i32);
	GfxCallbackQuery query{InvokeCallback, &callback};
	GfxEmptyResult result{};
	state->native->gfx->PushPopMatrix(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* RenderToTexture(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->RenderToTexture == nullptr)
		return Trap("Gfx RenderToTexture Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("Gfx RenderToTexture Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> name;
	if (!name.Read(state->memory,
			static_cast<std::uint32_t>(slots[0].i32),
			static_cast<std::uint32_t>(slots[1].i32))) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[2].i32, slots[3].i32);
	GfxRenderToTextureQuery query{name.c_str(), InvokeCallback, &callback};
	GfxEmptyResult result{};
	state->native->gfx->RenderToTexture(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* RunQuery(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->RunQuery == nullptr)
		return Trap("Gfx RunQuery Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("Gfx RunQuery Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[1].i32, slots[2].i32);
	GfxRunQueryQuery query{
		static_cast<std::uint32_t>(slots[0].i32),
		InvokeCallback,
		&callback,
	};
	GfxEmptyResult result{};
	state->native->gfx->RunQuery(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
	slots[0].i32 = NativeErrorCode(result.error);
	return nullptr;
}

wasm_trap_t* UnsafeState(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->UnsafeState == nullptr)
		return Trap("Gfx UnsafeState Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("Gfx UnsafeState Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	InitCallback(callback, state, caller, slots[2].i32, slots[3].i32);
	GfxUnsafeStateQuery query{
		static_cast<std::uint32_t>(slots[0].i32),
		slots[1].i32 != 0,
		InvokeCallback,
		&callback,
	};
	GfxEmptyResult result{};
	state->native->gfx->UnsafeState(&query, &result);
	if (!callback.success)
		return CallbackTrap(callback);
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

bool DefineStatus(wasmtime_linker_t* linker, const char* name,
	const wasm_valkind_t* params, std::size_t paramCount,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	const wasm_valkind_t result[] = {WASM_I32};
	return Define(linker, name, MakeFuncType(params, paramCount, result, 1),
		callback, state, error);
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
	const wasm_valkind_t vertexParams[] = {
		WASM_F32, WASM_F32, WASM_F32, WASM_F32, WASM_I32,
	};
	if (!DefineStatus(linker, "vertex", vertexParams, 5, Vertex, state, error))
		return false;
	const wasm_valkind_t callback2[] = {WASM_I32, WASM_I32};
	const wasm_valkind_t callback3[] = {WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t callback4[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t callback5[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	if (!DefineStatus(linker, "begin-end", callback3, 3, BeginEnd, state, error) ||
		!DefineStatus(linker, "active-fbo", callback5, 5, ActiveFBO, state, error) ||
		!DefineStatus(linker, "active-shader", callback3, 3, ActiveShader, state, error) ||
		!DefineStatus(linker, "draw-func-at-unit", callback4, 4, DrawFuncAtUnit, state, error) ||
		!DefineStatus(linker, "push-pop-matrix", callback2, 2, PushPopMatrix, state, error) ||
		!DefineStatus(linker, "render-to-texture", callback4, 4, RenderToTexture, state, error) ||
		!DefineStatus(linker, "run-query", callback3, 3, RunQuery, state, error) ||
		!DefineStatus(linker, "unsafe-state", callback4, 4, UnsafeState, state, error))
		return false;

	const wasm_valkind_t createListResults[] = {WASM_I64};
	return Define(linker, "create-list", MakeFuncType(callback2, 2, createListResults, 1),
		CreateList, state, error);
}

#endif

} // namespace recoil::wasm::core

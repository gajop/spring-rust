/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreTerrainControlBindings.h"

#include <cstdint>
#include <string_view>

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

wasm_trap_t* SetHeightMap(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->syncedCtrl == nullptr ||
		state->native->syncedCtrl->terrain == nullptr ||
		state->native->syncedCtrl->terrain->SetHeightMap == nullptr)
		return Trap("SetHeightMap Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("SetHeightMap Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());
	SetHeightMapQuery query{slots[0].f32, slots[1].f32, slots[2].f32, slots[3].f32};
	SetHeightMapResult result{};
	state->native->syncedCtrl->terrain->SetHeightMap(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* LevelHeightMap(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->syncedCtrl == nullptr ||
		state->native->syncedCtrl->terrain == nullptr ||
		state->native->syncedCtrl->terrain->LevelHeightMap == nullptr)
		return Trap("LevelHeightMap Core binding is unavailable");
	if (slots == nullptr || slotCount != 5)
		return Trap("LevelHeightMap Core ABI signature mismatch");
	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());
	LevelHeightMapQuery query{
		slots[0].f32, slots[1].f32, slots[2].f32, slots[3].f32, slots[4].f32,
	};
	LevelHeightMapResult result{};
	state->native->syncedCtrl->terrain->LevelHeightMap(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, NativeErrorCode(result.error)));
	return nullptr;
}

template<typename Query, typename Result>
wasm_trap_t* InvokeTerrainCallbackImport(HostState* state, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*nativeFunction)(const Query*, Result*), std::string_view name)
{
	if (state == nullptr || nativeFunction == nullptr)
		return Trap(std::string(name) + " Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap(std::string(name) + " Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());

	CallbackContext callback{};
	callback.state = state;
	callback.caller = caller;
	callback.callbackID = static_cast<std::uint32_t>(slots[0].i32);
	callback.userData = static_cast<std::uint32_t>(slots[1].i32);
	Query query{InvokeCallback, &callback};
	Result result{};
	nativeFunction(&query, &result);
	if (!callback.success)
		return Trap(callback.error.empty() ? "Core terrain callback failed" : callback.error);
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(result.success ? 1u : 0u, NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* SetHeightMapFunc(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->syncedCtrl == nullptr ||
		state->native->syncedCtrl->terrain == nullptr)
		return Trap("SetHeightMapFunc Core binding is unavailable");
	return InvokeTerrainCallbackImport<SetHeightMapFuncQuery, SetHeightMapFuncResult>(
		state, caller, slots, slotCount,
		state->native->syncedCtrl->terrain->SetHeightMapFunc, "SetHeightMapFunc");
}

wasm_trap_t* SetOriginalHeightMapFunc(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->syncedCtrl == nullptr ||
		state->native->syncedCtrl->terrain == nullptr)
		return Trap("SetOriginalHeightMapFunc Core binding is unavailable");
	return InvokeTerrainCallbackImport<SetOriginalHeightMapFuncQuery, SetOriginalHeightMapFuncResult>(
		state, caller, slots, slotCount,
		state->native->syncedCtrl->terrain->SetOriginalHeightMapFunc, "SetOriginalHeightMapFunc");
}

wasm_trap_t* SetSmoothMeshFunc(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->syncedCtrl == nullptr ||
		state->native->syncedCtrl->terrain == nullptr)
		return Trap("SetSmoothMeshFunc Core binding is unavailable");
	return InvokeTerrainCallbackImport<SetSmoothMeshFuncQuery, SetSmoothMeshFuncResult>(
		state, caller, slots, slotCount,
		state->native->syncedCtrl->terrain->SetSmoothMeshFunc, "SetSmoothMeshFunc");
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:terrain-control", 22,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterTerrainControlImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->syncedCtrl == nullptr || state->native->syncedCtrl->terrain == nullptr) {
		error = "cannot register TerrainControl Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t i64Result[] = {WASM_I64};
	const wasm_valkind_t setParams[] = {WASM_F32, WASM_F32, WASM_F32, WASM_F32};
	if (!Define(linker, "set-height-map", MakeFuncType(setParams, 4, i64Result, 1),
			SetHeightMap, state, error))
		return false;
	const wasm_valkind_t levelParams[] = {
		WASM_F32, WASM_F32, WASM_F32, WASM_F32, WASM_F32,
	};
	if (!Define(linker, "level-height-map", MakeFuncType(levelParams, 5, i64Result, 1),
			LevelHeightMap, state, error))
		return false;
	const wasm_valkind_t callbackParams[] = {WASM_I32, WASM_I32};
	if (!Define(linker, "set-height-map-func",
			MakeFuncType(callbackParams, 2, i64Result, 1), SetHeightMapFunc, state, error))
		return false;
	if (!Define(linker, "set-original-height-map-func",
			MakeFuncType(callbackParams, 2, i64Result, 1), SetOriginalHeightMapFunc, state, error))
		return false;
	return Define(linker, "set-smooth-mesh-func",
		MakeFuncType(callbackParams, 2, i64Result, 1), SetSmoothMeshFunc, state, error);
}

#endif

} // namespace recoil::wasm::core

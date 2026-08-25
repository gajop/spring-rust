/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreTerrainReadBindings.h"

#include <bit>
#include <cstdint>
#include <span>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreWire.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

void ReturnF32(wasmtime_val_raw_t* slots, float value, const Error* error)
{
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(std::bit_cast<std::uint32_t>(value), NativeErrorCode(error)));
}

void ReturnBool(wasmtime_val_raw_t* slots, bool value, const Error* error)
{
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(value ? 1u : 0u, NativeErrorCode(error)));
}

bool PrepareOutput(HostState* state, wasmtime_caller_t* caller,
	std::uint32_t output, std::size_t bytes, wasmtime_val_raw_t* slots,
	wasm_trap_t*& trap)
{
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError)) {
		trap = Trap(memoryError);
		return false;
	}
	if (!state->memory.Contains(output, bytes)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return false;
	}
	return true;
}

bool WriteI32Pair(HostState* state, std::uint32_t output,
	std::int32_t first, std::int32_t second)
{
	std::span<std::uint8_t> bytes;
	if (!state->memory.MutableView(output, 8u, bytes))
		return false;
	WireWriter writer(bytes);
	return writer.I32(first) && writer.I32(second) && writer.Finish(4);
}

bool WriteF32Values(HostState* state, std::uint32_t output,
	std::span<const float> values)
{
	std::span<std::uint8_t> bytes;
	if (!state->memory.MutableView(output, values.size() * sizeof(float), bytes))
		return false;
	WireWriter writer(bytes);
	for (float value : values) {
		if (!writer.F32(value))
			return false;
	}
	return writer.Finish(4);
}

wasm_trap_t* IsPosInMap(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->IsPosInMap == nullptr)
		return Trap("IsPosInMap Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("IsPosInMap Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());
	IsPosInMapQuery query{slots[0].f32, slots[1].f32};
	IsPosInMapResult result{};
	state->native->terrain->IsPosInMap(&query, &result);
	const std::uint32_t flags = (result.inMap ? 1u : 0u) | (result.inPlayArea ? 2u : 0u);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(flags, NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetGroundHeight(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetGroundHeight == nullptr)
		return Trap("GetGroundHeight Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetGroundHeight Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok()) return Trap(guard.Error());
	GetGroundHeightQuery query{slots[0].f32, slots[1].f32};
	GetGroundHeightResult result{};
	state->native->terrain->GetGroundHeight(&query, &result);
	ReturnF32(slots, result.height, result.error);
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
	ImportGuard guard(state, 3);
	if (!guard.Ok()) return Trap(guard.Error());
	GetGroundOrigHeightQuery query{slots[0].f32, slots[1].f32};
	GetGroundOrigHeightResult result{};
	state->native->terrain->GetGroundOrigHeight(&query, &result);
	ReturnF32(slots, result.height, result.error);
	return nullptr;
}

wasm_trap_t* GetSmoothMeshHeight(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetSmoothMeshHeight == nullptr)
		return Trap("GetSmoothMeshHeight Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetSmoothMeshHeight Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok()) return Trap(guard.Error());
	GetSmoothMeshHeightQuery query{slots[0].f32, slots[1].f32};
	GetSmoothMeshHeightResult result{};
	state->native->terrain->GetSmoothMeshHeight(&query, &result);
	ReturnF32(slots, result.height, result.error);
	return nullptr;
}

wasm_trap_t* GetWaterPlaneLevel(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetWaterPlaneLevel == nullptr)
		return Trap("GetWaterPlaneLevel Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetWaterPlaneLevel Core ABI signature mismatch");
	ImportGuard guard(state, 1);
	if (!guard.Ok()) return Trap(guard.Error());
	GetWaterPlaneLevelQuery query{};
	GetWaterPlaneLevelResult result{};
	state->native->terrain->GetWaterPlaneLevel(&query, &result);
	ReturnF32(slots, result.level, result.error);
	return nullptr;
}

wasm_trap_t* GetWaterLevel(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetWaterLevel == nullptr)
		return Trap("GetWaterLevel Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetWaterLevel Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok()) return Trap(guard.Error());
	GetWaterLevelQuery query{slots[0].f32, slots[1].f32};
	GetWaterLevelResult result{};
	state->native->terrain->GetWaterLevel(&query, &result);
	ReturnF32(slots, result.level, result.error);
	return nullptr;
}

wasm_trap_t* GetHeightMapSize(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetHeightMapSize == nullptr)
		return Trap("GetHeightMapSize Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetHeightMapSize Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok()) return Trap(guard.Error());
	const std::uint32_t output = static_cast<std::uint32_t>(slots[0].i32);
	wasm_trap_t* trap = nullptr;
	if (!PrepareOutput(state, caller, output, 8u, slots, trap)) return trap;
	GetHeightMapSizeQuery query{};
	GetHeightMapSizeResult result{};
	state->native->terrain->GetHeightMapSize(&query, &result);
	const std::int32_t errorCode = NativeErrorCode(result.error);
	if (errorCode != 0) { slots[0].i32 = errorCode; return nullptr; }
	if (!WriteI32Pair(state, output, result.pointsX, result.pointsZ))
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
	else
		slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetGroundExtremes(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetGroundExtremes == nullptr)
		return Trap("GetGroundExtremes Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetGroundExtremes Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok()) return Trap(guard.Error());
	const std::uint32_t output = static_cast<std::uint32_t>(slots[0].i32);
	wasm_trap_t* trap = nullptr;
	if (!PrepareOutput(state, caller, output, 16u, slots, trap)) return trap;
	GetGroundExtremesQuery query{};
	GetGroundExtremesResult result{};
	state->native->terrain->GetGroundExtremes(&query, &result);
	const std::int32_t errorCode = NativeErrorCode(result.error);
	if (errorCode != 0) { slots[0].i32 = errorCode; return nullptr; }
	const float values[] = {result.initMinHeight, result.initMaxHeight,
		result.currMinHeight, result.currMaxHeight};
	if (!WriteF32Values(state, output, values))
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
	else
		slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetGroundNormal(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetGroundNormal == nullptr)
		return Trap("GetGroundNormal Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("GetGroundNormal Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok()) return Trap(guard.Error());
	const std::uint32_t output = static_cast<std::uint32_t>(slots[3].i32);
	wasm_trap_t* trap = nullptr;
	if (!PrepareOutput(state, caller, output, 16u, slots, trap)) return trap;
	GetGroundNormalQuery query{slots[0].f32, slots[1].f32, slots[2].i32 != 0};
	GetGroundNormalResult result{};
	state->native->terrain->GetGroundNormal(&query, &result);
	const std::int32_t errorCode = NativeErrorCode(result.error);
	if (errorCode != 0) { slots[0].i32 = errorCode; return nullptr; }
	const float values[] = {result.normal.x, result.normal.y, result.normal.z, result.slope};
	if (!WriteF32Values(state, output, values))
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
	else
		slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetGroundBlocked(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetGroundBlocked == nullptr)
		return Trap("GetGroundBlocked Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("GetGroundBlocked Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok()) return Trap(guard.Error());
	GetGroundBlockedQuery query{slots[0].f32, slots[1].f32, slots[2].f32, slots[3].f32};
	GetGroundBlockedResult result{};
	state->native->terrain->GetGroundBlocked(&query, &result);
	ReturnBool(slots, result.blocked, result.error);
	return nullptr;
}

wasm_trap_t* GetGrass(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->terrain == nullptr ||
		state->native->terrain->GetGrass == nullptr)
		return Trap("GetGrass Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetGrass Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok()) return Trap(guard.Error());
	GetGrassQuery query{slots[0].f32, slots[1].f32};
	GetGrassResult result{};
	state->native->terrain->GetGrass(&query, &result);
	ReturnF32(slots, result.grassLevel, result.error);
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:terrain", 14,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterTerrainReadImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr) {
		error = "cannot register Terrain Core imports without linker/host/API";
		return false;
	}
	if (state->native->terrain == nullptr)
		return true;
	const wasm_valkind_t f32f32[] = {WASM_F32, WASM_F32};
	const wasm_valkind_t f32x4[] = {WASM_F32, WASM_F32, WASM_F32, WASM_F32};
	const wasm_valkind_t i32[] = {WASM_I32};
	const wasm_valkind_t i64[] = {WASM_I64};
	const wasm_valkind_t i32Result[] = {WASM_I32};
	if (!Define(linker, "is-pos-in-map", MakeFuncType(f32f32, 2, i64, 1), IsPosInMap, state, error) ||
		!Define(linker, "get-ground-height", MakeFuncType(f32f32, 2, i64, 1), GetGroundHeight, state, error) ||
		!Define(linker, "get-ground-orig-height", MakeFuncType(f32f32, 2, i64, 1), GetGroundOrigHeight, state, error) ||
		!Define(linker, "get-smooth-mesh-height", MakeFuncType(f32f32, 2, i64, 1), GetSmoothMeshHeight, state, error) ||
		!Define(linker, "get-water-level", MakeFuncType(f32f32, 2, i64, 1), GetWaterLevel, state, error) ||
		!Define(linker, "get-ground-blocked", MakeFuncType(f32x4, 4, i64, 1), GetGroundBlocked, state, error) ||
		!Define(linker, "get-grass", MakeFuncType(f32f32, 2, i64, 1), GetGrass, state, error))
		return false;
	if (!Define(linker, "get-water-plane-level", MakeFuncType(nullptr, 0, i64, 1),
			GetWaterPlaneLevel, state, error) ||
		!Define(linker, "get-height-map-size", MakeFuncType(i32, 1, i32Result, 1),
			GetHeightMapSize, state, error) ||
		!Define(linker, "get-ground-extremes", MakeFuncType(i32, 1, i32Result, 1),
			GetGroundExtremes, state, error))
		return false;
	const wasm_valkind_t normalParams[] = {WASM_F32, WASM_F32, WASM_I32, WASM_I32};
	return Define(linker, "get-ground-normal", MakeFuncType(normalParams, 4, i32Result, 1),
		GetGroundNormal, state, error);
}

#endif

} // namespace recoil::wasm::core

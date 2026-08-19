/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreUnitsQueryBindings.h"

#include <cstdint>
#include <limits>
#include <span>
#include <string_view>

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

wasm_trap_t* BeginImport(HostState* state, std::uint64_t work,
	std::unique_ptr<ImportGuard>& guard)
{
	std::string error;
	guard = std::make_unique<ImportGuard>(state, work, error);
	return guard->Ok() ? nullptr : Trap(error);
}

bool PrepareListBuffer(HostState* state, wasmtime_caller_t* caller,
	std::uint32_t output, std::uint32_t capacity)
{
	std::string error;
	if (!generated::EnsureMemory(state, caller, error))
		return false;
	const std::uint64_t bytes64 = static_cast<std::uint64_t>(capacity) * sizeof(std::int32_t);
	if (bytes64 > std::numeric_limits<std::size_t>::max())
		return false;
	return state->memory.Contains(output, static_cast<std::size_t>(bytes64));
}

void ReturnList(wasmtime_val_raw_t* slots, HostState* state, const Error* nativeError,
	const std::int32_t* values, std::uint32_t count,
	std::uint32_t output, std::uint32_t capacity)
{
	const std::int32_t errorCode = NativeErrorCode(nativeError);
	if (errorCode != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0, errorCode));
		return;
	}
	if (count > capacity) {
		// No partial write. The low bits always report the complete required
		// element count so the guest can resize exactly and retry once.
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(count, static_cast<std::int32_t>(Status::BufferOverflow)));
		return;
	}
	if (count != 0 && values == nullptr) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::Internal)));
		return;
	}
	if (!state->memory.WriteI32SliceLE(output,
			std::span<const std::int32_t>(values, count))) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return;
	}
	slots[0].i64 = static_cast<std::int64_t>(PackU32(count, 0));
}

bool ValidateFlags(std::uint32_t flags, std::uint32_t allowed,
	wasmtime_val_raw_t* slots)
{
	if ((flags & ~allowed) == 0)
		return true;
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(0, static_cast<std::int32_t>(Status::InvalidArgument)));
	return false;
}

wasm_trap_t* ValidUnitID(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->ValidUnitID == nullptr)
		return Trap("ValidUnitID Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("ValidUnitID Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 2, guard))
		return trap;
	ValidUnitIDQuery query{slots[0].i32};
	ValidUnitIDResult result{};
	state->native->unitsQuery->ValidUnitID(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.valid ? 1u : 0u,
		NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetAllUnits(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetAllUnits == nullptr)
		return Trap("GetAllUnits Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetAllUnits Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 3, guard))
		return trap;
	const std::uint32_t output = static_cast<std::uint32_t>(slots[0].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[1].i32);
	if (!PrepareListBuffer(state, caller, output, capacity)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetAllUnitsQuery query{};
	GetAllUnitsResult result{};
	state->native->unitsQuery->GetAllUnits(&query, &result);
	ReturnList(slots, state, result.error, result.units, result.count, output, capacity);
	return nullptr;
}

wasm_trap_t* GetTeamUnits(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetTeamUnits == nullptr)
		return Trap("GetTeamUnits Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetTeamUnits Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 4, guard))
		return trap;
	const std::uint32_t output = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[2].i32);
	if (!PrepareListBuffer(state, caller, output, capacity)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetTeamUnitsQuery query{slots[0].i32};
	GetTeamUnitsResult result{};
	state->native->unitsQuery->GetTeamUnits(&query, &result);
	ReturnList(slots, state, result.error, result.units, result.count, output, capacity);
	return nullptr;
}

wasm_trap_t* GetTeamUnitDefCount(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetTeamUnitDefCount == nullptr)
		return Trap("GetTeamUnitDefCount Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetTeamUnitDefCount Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 3, guard))
		return trap;
	GetTeamUnitDefCountQuery query{slots[0].i32, slots[1].i32};
	GetTeamUnitDefCountResult result{};
	state->native->unitsQuery->GetTeamUnitDefCount(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.count,
		NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetTeamUnitCount(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetTeamUnitCount == nullptr)
		return Trap("GetTeamUnitCount Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetTeamUnitCount Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 2, guard))
		return trap;
	GetTeamUnitCountQuery query{slots[0].i32};
	GetTeamUnitCountResult result{};
	state->native->unitsQuery->GetTeamUnitCount(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.count,
		NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitsInRectangle(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetUnitsInRectangle == nullptr)
		return Trap("GetUnitsInRectangle Core binding is unavailable");
	if (slots == nullptr || slotCount != 7)
		return Trap("GetUnitsInRectangle Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 8, guard))
		return trap;
	const std::uint32_t output = static_cast<std::uint32_t>(slots[5].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[6].i32);
	if (!PrepareListBuffer(state, caller, output, capacity)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetUnitsInRectangleQuery query{slots[0].f32, slots[1].f32, slots[2].f32,
		slots[3].f32, slots[4].i32};
	GetUnitsInRectangleResult result{};
	state->native->unitsQuery->GetUnitsInRectangle(&query, &result);
	ReturnList(slots, state, result.error, result.units, result.count, output, capacity);
	return nullptr;
}

wasm_trap_t* GetUnitsInBox(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetUnitsInBox == nullptr)
		return Trap("GetUnitsInBox Core binding is unavailable");
	if (slots == nullptr || slotCount != 9)
		return Trap("GetUnitsInBox Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 10, guard))
		return trap;
	const std::uint32_t output = static_cast<std::uint32_t>(slots[7].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[8].i32);
	if (!PrepareListBuffer(state, caller, output, capacity)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetUnitsInBoxQuery query{slots[0].f32, slots[1].f32, slots[2].f32,
		slots[3].f32, slots[4].f32, slots[5].f32, slots[6].i32};
	GetUnitsInBoxResult result{};
	state->native->unitsQuery->GetUnitsInBox(&query, &result);
	ReturnList(slots, state, result.error, result.units, result.count, output, capacity);
	return nullptr;
}

wasm_trap_t* GetUnitsInSphere(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetUnitsInSphere == nullptr)
		return Trap("GetUnitsInSphere Core binding is unavailable");
	if (slots == nullptr || slotCount != 7)
		return Trap("GetUnitsInSphere Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 8, guard))
		return trap;
	const std::uint32_t output = static_cast<std::uint32_t>(slots[5].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[6].i32);
	if (!PrepareListBuffer(state, caller, output, capacity)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetUnitsInSphereQuery query{slots[0].f32, slots[1].f32, slots[2].f32,
		slots[3].f32, slots[4].i32};
	GetUnitsInSphereResult result{};
	state->native->unitsQuery->GetUnitsInSphere(&query, &result);
	ReturnList(slots, state, result.error, result.units, result.count, output, capacity);
	return nullptr;
}

wasm_trap_t* GetUnitsInCylinder(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetUnitsInCylinder == nullptr)
		return Trap("GetUnitsInCylinder Core binding is unavailable");
	if (slots == nullptr || slotCount != 6)
		return Trap("GetUnitsInCylinder Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 7, guard))
		return trap;
	const std::uint32_t output = static_cast<std::uint32_t>(slots[4].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[5].i32);
	if (!PrepareListBuffer(state, caller, output, capacity)) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}
	GetUnitsInCylinderQuery query{slots[0].f32, slots[1].f32, slots[2].f32,
		slots[3].i32};
	GetUnitsInCylinderResult result{};
	state->native->unitsQuery->GetUnitsInCylinder(&query, &result);
	ReturnList(slots, state, result.error, result.units, result.count, output, capacity);
	return nullptr;
}

wasm_trap_t* GetUnitNearestAlly(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetUnitNearestAlly == nullptr)
		return Trap("GetUnitNearestAlly Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetUnitNearestAlly Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 3, guard))
		return trap;
	GetUnitNearestAllyQuery query{slots[0].i32, slots[1].f32};
	GetUnitNearestAllyResult result{};
	state->native->unitsQuery->GetUnitNearestAlly(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.unitID,
		NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitNearestEnemy(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetUnitNearestEnemy == nullptr)
		return Trap("GetUnitNearestEnemy Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitNearestEnemy Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 4, guard))
		return trap;
	const std::uint32_t flags = static_cast<std::uint32_t>(slots[2].i32);
	if (!ValidateFlags(flags, 0x7u, slots))
		return nullptr;
	GetUnitNearestEnemyQuery query{};
	query.unitID = slots[0].i32;
	query.range = slots[1].f32;
	query.options.useLOS = (flags & 0x1u) != 0;
	query.options.sphereDistTest = (flags & 0x2u) != 0;
	query.options.checkSightDist = (flags & 0x4u) != 0;
	GetUnitNearestEnemyResult result{};
	state->native->unitsQuery->GetUnitNearestEnemy(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.unitID,
		NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitSeparation(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsQuery == nullptr ||
		state->native->unitsQuery->GetUnitSeparation == nullptr)
		return Trap("GetUnitSeparation Core binding is unavailable");
	if (slots == nullptr || slotCount != 3)
		return Trap("GetUnitSeparation Core ABI signature mismatch");
	std::unique_ptr<ImportGuard> guard;
	if (wasm_trap_t* trap = BeginImport(state, 4, guard))
		return trap;
	const std::uint32_t flags = static_cast<std::uint32_t>(slots[2].i32);
	if (!ValidateFlags(flags, 0x3u, slots))
		return nullptr;
	GetUnitSeparationQuery query{};
	query.unitID1 = slots[0].i32;
	query.unitID2 = slots[1].i32;
	query.options.positional = (flags & 0x1u) != 0;
	query.options.checkMap = (flags & 0x2u) != 0;
	GetUnitSeparationResult result{};
	state->native->unitsQuery->GetUnitSeparation(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackU32(
		std::bit_cast<std::uint32_t>(result.separation), NativeErrorCode(result.error)));
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:units-query", 18, name, std::char_traits<char>::length(name),
		type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterUnitsQueryImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->unitsQuery == nullptr) {
		error = "cannot register UnitsQuery Core imports without linker/host/API";
		return false;
	}

	const wasm_valkind_t i32[] = {WASM_I32};
	const wasm_valkind_t i64[] = {WASM_I64};
	const wasm_valkind_t i32i32[] = {WASM_I32, WASM_I32};
	const wasm_valkind_t i32i32i32[] = {WASM_I32, WASM_I32, WASM_I32};
	if (!Define(linker, "valid-unit-id", MakeFuncType(i32, 1, i64, 1), ValidUnitID, state, error) ||
		!Define(linker, "get-all-units", MakeFuncType(i32i32, 2, i64, 1), GetAllUnits, state, error) ||
		!Define(linker, "get-team-units", MakeFuncType(i32i32i32, 3, i64, 1), GetTeamUnits, state, error) ||
		!Define(linker, "get-team-unit-def-count", MakeFuncType(i32i32, 2, i64, 1), GetTeamUnitDefCount, state, error) ||
		!Define(linker, "get-team-unit-count", MakeFuncType(i32, 1, i64, 1), GetTeamUnitCount, state, error))
		return false;

	{
		const wasm_valkind_t params[] = {WASM_F32, WASM_F32, WASM_F32, WASM_F32,
			WASM_I32, WASM_I32, WASM_I32};
		if (!Define(linker, "get-units-in-rectangle", MakeFuncType(params, 7, i64, 1),
			GetUnitsInRectangle, state, error) ||
			!Define(linker, "get-units-in-sphere", MakeFuncType(params, 7, i64, 1),
				GetUnitsInSphere, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_F32, WASM_F32, WASM_F32, WASM_F32,
			WASM_F32, WASM_F32, WASM_I32, WASM_I32, WASM_I32};
		if (!Define(linker, "get-units-in-box", MakeFuncType(params, 9, i64, 1),
			GetUnitsInBox, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_F32, WASM_F32, WASM_F32,
			WASM_I32, WASM_I32, WASM_I32};
		if (!Define(linker, "get-units-in-cylinder", MakeFuncType(params, 6, i64, 1),
			GetUnitsInCylinder, state, error))
			return false;
	}
	{
		const wasm_valkind_t nearestAlly[] = {WASM_I32, WASM_F32};
		const wasm_valkind_t nearestEnemy[] = {WASM_I32, WASM_F32, WASM_I32};
		if (!Define(linker, "get-unit-nearest-ally", MakeFuncType(nearestAlly, 2, i64, 1),
			GetUnitNearestAlly, state, error) ||
			!Define(linker, "get-unit-nearest-enemy", MakeFuncType(nearestEnemy, 3, i64, 1),
				GetUnitNearestEnemy, state, error) ||
			!Define(linker, "get-unit-separation", MakeFuncType(i32i32i32, 3, i64, 1),
				GetUnitSeparation, state, error))
			return false;
	}
	return true;
}

#endif

} // namespace recoil::wasm::core

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// Columns: one property of many units in one call (`spring:columns`).
//
// A guest names a property and passes the unit IDs as an i32 column; the host
// fills (or applies) one plane of 4-byte values per component: all X values,
// then all Y values, and so on, so both sides can loop over plain arrays. Each
// entry runs the same native read or write as the matching per-unit import, so
// values, visibility and errors match it exactly; an entry the per-unit import
// would refuse is zeroed and marked 0 in the optional `found` byte column.
//
// get-units(property, ids, count, args, arg, out, out-bytes, found) -> i64
// set-units(property, ids, count, args, arg, values, values-bytes, done) -> i64   (synced only)
// get-units-rules-param(name, name-bytes, ids, count, out, found) -> i64
//
// `args` (an i32 column, or 0 to use `arg` for every entry) carries a
// property's argument: the ally team for in-los/in-radar, the weapon number
// (from 1) for weapon-target. The result packs the number of entries found (or
// applied) with a status, as the other Core imports do.

#include "WasmCoreBindings.h"

#include <array>
#include <bit>
#include <cstdint>
#include <cstring>
#include <span>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"
#include "NativeInterface/NativeInterface.h"

namespace recoil::wasm::core {
#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

// Property numbers are the ABI; never renumber them.
enum class UnitColumn : std::int32_t {
	Position = 1,      // f32 x3
	MidPosition = 2,   // f32 x3
	AimPosition = 3,   // f32 x3
	Velocity = 4,      // f32 x3
	Vectors = 5,       // f32 x9: front xyz, up xyz, right xyz
	AllyTeam = 6,      // i32
	Team = 7,          // i32
	DefId = 8,         // i32
	Alive = 9,         // i32 0/1: a valid unit not yet dead (always found)
	Cloaked = 10,      // i32 0/1
	Health = 11,       // f32 x2: health, max health
	Radius = 12,       // f32
	InLos = 13,        // i32 0/1, arg: ally team (always found; 0 where the per-unit import errs)
	InRadar = 14,      // i32 0/1, arg: ally team (as in-los)
	Stunned = 15,      // i32 0/1
	WeaponTarget = 16, // x5: type i32, unit i32, x y z f32; arg: weapon number from 1
	FireState = 17,    // i32
	MoveState = 18,    // i32
	MoveCtrlVelocity = 20, // set only, f32 x3: MoveCtrl.SetVelocity
	Direction = 21,        // set only, f32 x6: front xyz, right xyz (SetUnitDirection)
};

constexpr std::uint32_t MAX_ENTRIES = 1u << 20;

std::uint32_t Components(std::int32_t property, bool set)
{
	switch (static_cast<UnitColumn>(property)) {
		case UnitColumn::Position:
		case UnitColumn::MidPosition:
		case UnitColumn::AimPosition:
		case UnitColumn::Velocity: return set ? 0 : 3;
		case UnitColumn::Vectors: return set ? 0 : 9;
		case UnitColumn::AllyTeam:
		case UnitColumn::Team:
		case UnitColumn::DefId:
		case UnitColumn::Alive:
		case UnitColumn::Cloaked:
		case UnitColumn::Radius:
		case UnitColumn::InLos:
		case UnitColumn::InRadar:
		case UnitColumn::Stunned:
		case UnitColumn::FireState:
		case UnitColumn::MoveState: return set ? 0 : 1;
		case UnitColumn::Health: return set ? 0 : 2;
		case UnitColumn::WeaponTarget: return set ? 0 : 5;
		case UnitColumn::MoveCtrlVelocity: return set ? 3 : 0;
		case UnitColumn::Direction: return set ? 6 : 0;
	}
	return 0;
}

bool Failed(const Error* error) { return error != nullptr; }

// One plane per component, `count` 4-byte values each, in guest memory that
// need not be aligned.
struct Planes {
	std::uint8_t* base;
	std::uint32_t count;

	void F32(std::uint32_t plane, std::uint32_t i, float value) const
	{
		std::memcpy(base + (std::size_t(plane) * count + i) * 4, &value, 4);
	}
	void I32(std::uint32_t plane, std::uint32_t i, std::int32_t value) const
	{
		std::memcpy(base + (std::size_t(plane) * count + i) * 4, &value, 4);
	}
	float GetF32(std::uint32_t plane, std::uint32_t i) const
	{
		float value;
		std::memcpy(&value, base + (std::size_t(plane) * count + i) * 4, 4);
		return value;
	}
};

std::int32_t Load(std::span<const std::uint8_t> column, std::uint32_t i)
{
	std::int32_t value;
	std::memcpy(&value, column.data() + std::size_t(i) * 4, 4);
	return value;
}

// Reads entry `i` into the planes; false where the per-unit import errs.
bool ReadUnit(const NativeInterface& native, UnitColumn property, std::int32_t unit,
	std::int32_t arg, const Planes& out, std::uint32_t i)
{
	switch (property) {
		case UnitColumn::Position:
		case UnitColumn::MidPosition:
		case UnitColumn::AimPosition: {
			GetUnitPositionQuery query{};
			query.unitID = unit;
			query.options.midPos = property == UnitColumn::MidPosition;
			query.options.aimPos = property == UnitColumn::AimPosition;
			GetUnitPositionResult result{};
			native.unitsInfo->GetUnitPosition(&query, &result);
			if (Failed(result.error)) return false;
			out.F32(0, i, result.position.x);
			out.F32(1, i, result.position.y);
			out.F32(2, i, result.position.z);
			return true;
		}
		case UnitColumn::Velocity: {
			GetUnitVelocityQuery query{unit};
			GetUnitVelocityResult result{};
			native.unitsInfo->GetUnitVelocity(&query, &result);
			if (Failed(result.error)) return false;
			out.F32(0, i, result.velocity.x);
			out.F32(1, i, result.velocity.y);
			out.F32(2, i, result.velocity.z);
			return true;
		}
		case UnitColumn::Vectors: {
			GetUnitVectorsQuery query{unit};
			GetUnitVectorsResult result{};
			native.unitsInfo->GetUnitVectors(&query, &result);
			if (Failed(result.error)) return false;
			const Float3 axes[3] = {result.vectors.frontDir, result.vectors.upDir, result.vectors.rightDir};
			for (std::uint32_t a = 0; a < 3; ++a) {
				out.F32(a * 3 + 0, i, axes[a].x);
				out.F32(a * 3 + 1, i, axes[a].y);
				out.F32(a * 3 + 2, i, axes[a].z);
			}
			return true;
		}
		case UnitColumn::AllyTeam: {
			GetUnitAllyTeamQuery query{unit};
			GetUnitAllyTeamResult result{};
			native.unitsInfo->GetUnitAllyTeam(&query, &result);
			if (Failed(result.error)) return false;
			out.I32(0, i, result.allyTeamID);
			return true;
		}
		case UnitColumn::Team: {
			GetUnitTeamQuery query{unit};
			GetUnitTeamResult result{};
			native.unitsInfo->GetUnitTeam(&query, &result);
			if (Failed(result.error)) return false;
			out.I32(0, i, result.teamID);
			return true;
		}
		case UnitColumn::DefId: {
			GetUnitDefIDQuery query{unit};
			GetUnitDefIDResult result{};
			native.unitsInfo->GetUnitDefID(&query, &result);
			if (Failed(result.error)) return false;
			out.I32(0, i, result.unitDefID);
			return true;
		}
		case UnitColumn::Alive: {
			// valid-unit-id and not get-unit-is-dead, as a guest asks with both
			ValidUnitIDQuery validQuery{unit};
			ValidUnitIDResult valid{};
			native.unitsQuery->ValidUnitID(&validQuery, &valid);
			bool alive = !Failed(valid.error) && valid.valid;
			if (alive) {
				GetUnitIsDeadQuery deadQuery{unit};
				GetUnitIsDeadResult dead{};
				native.unitsInfo->GetUnitIsDead(&deadQuery, &dead);
				alive = !Failed(dead.error) && !dead.isDead;
			}
			out.I32(0, i, alive ? 1 : 0);
			return true;
		}
		case UnitColumn::Cloaked: {
			GetUnitIsCloakedQuery query{unit};
			GetUnitIsCloakedResult result{};
			native.unitsInfo->GetUnitIsCloaked(&query, &result);
			if (Failed(result.error)) return false;
			out.I32(0, i, result.isCloaked ? 1 : 0);
			return true;
		}
		case UnitColumn::Health: {
			GetUnitHealthQuery query{unit};
			GetUnitHealthResult result{};
			native.unitsInfo->GetUnitHealth(&query, &result);
			if (Failed(result.error)) return false;
			out.F32(0, i, result.health.health);
			out.F32(1, i, result.health.maxHealth);
			return true;
		}
		case UnitColumn::Radius: {
			GetUnitRadiusQuery query{unit};
			GetUnitRadiusResult result{};
			native.unitsInfo->GetUnitRadius(&query, &result);
			if (Failed(result.error)) return false;
			out.F32(0, i, result.radius);
			return true;
		}
		case UnitColumn::InLos: {
			IsUnitInLosQuery query{unit, arg};
			IsUnitInLosResult result{};
			native.los->IsUnitInLos(&query, &result);
			out.I32(0, i, (!Failed(result.error) && result.inLos) ? 1 : 0);
			return !Failed(result.error);
		}
		case UnitColumn::InRadar: {
			IsUnitInRadarQuery query{unit, arg};
			IsUnitInRadarResult result{};
			native.los->IsUnitInRadar(&query, &result);
			out.I32(0, i, (!Failed(result.error) && result.inRadar) ? 1 : 0);
			return !Failed(result.error);
		}
		case UnitColumn::Stunned: {
			GetUnitIsStunnedQuery query{unit};
			GetUnitIsStunnedResult result{};
			native.unitsInfo->GetUnitIsStunned(&query, &result);
			if (Failed(result.error)) return false;
			out.I32(0, i, result.isStunned ? 1 : 0);
			return true;
		}
		case UnitColumn::WeaponTarget: {
			GetUnitWeaponTargetQuery query{unit, arg};
			GetUnitWeaponTargetResult result{};
			native.unitsWeapons->GetUnitWeaponTarget(&query, &result);
			if (Failed(result.error)) return false;
			out.I32(0, i, result.target.targetType);
			out.I32(1, i, result.target.targetID);
			out.F32(2, i, result.target.targetPos.x);
			out.F32(3, i, result.target.targetPos.y);
			out.F32(4, i, result.target.targetPos.z);
			return true;
		}
		case UnitColumn::FireState:
		case UnitColumn::MoveState: {
			GetUnitStatesQuery query{};
			query.unitID = unit;
			GetUnitStatesResult result{};
			native.unitsInfo->GetUnitStates(&query, &result);
			if (Failed(result.error)) return false;
			out.I32(0, i, property == UnitColumn::FireState ? result.states.fireState : result.states.moveState);
			return true;
		}
		default:
			return false;
	}
}

bool WriteUnit(const NativeInterface& native, UnitColumn property, std::int32_t unit,
	const Planes& in, std::uint32_t i)
{
	switch (property) {
		case UnitColumn::MoveCtrlVelocity: {
			MoveCtrlFloat3Query query{};
			query.unitID = unit;
			query.value = {in.GetF32(0, i), in.GetF32(1, i), in.GetF32(2, i)};
			MoveCtrlResult result{};
			native.moveCtrl->SetVelocity(&query, &result);
			return !Failed(result.error) && result.success;
		}
		case UnitColumn::Direction: {
			SetUnitDirectionQuery query{};
			query.unitID = unit;
			query.frontDir = {in.GetF32(0, i), in.GetF32(1, i), in.GetF32(2, i)};
			query.rightDir = {in.GetF32(3, i), in.GetF32(4, i), in.GetF32(5, i)};
			SetUnitDirectionResult result{};
			native.syncedCtrl->unit->SetUnitDirection(&query, &result);
			return !Failed(result.error) && result.success;
		}
		default:
			return false;
	}
}

bool Available(const HostState* state)
{
	const NativeInterface* native = state->native;
	return native != nullptr && native->unitsInfo != nullptr && native->unitsQuery != nullptr &&
		native->unitsWeapons != nullptr && native->los != nullptr && native->moveCtrl != nullptr &&
		native->syncedCtrl != nullptr && native->syncedCtrl->unit != nullptr && native->rulesParams != nullptr;
}

void Return(wasmtime_val_raw_t* slots, std::uint32_t value, Status status)
{
	slots[0].i64 = static_cast<std::int64_t>(PackU32(value, static_cast<std::int32_t>(status)));
}

// The shared part of get-units and set-units: validate, then run `each` per entry.
template<typename Each>
wasm_trap_t* Column(void* environment, wasmtime_caller_t* caller, wasmtime_val_raw_t* slots,
	std::size_t slotCount, bool set, Each&& each)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || !Available(state))
		return Trap("Column host binding is unavailable");
	if (slots == nullptr || slotCount != 8)
		return Trap("Column core ABI signature mismatch");

	ImportGuard guard(state, 1);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::int32_t property = slots[0].i32;
	const auto idsPointer = static_cast<std::uint32_t>(slots[1].i32);
	const auto count = static_cast<std::uint32_t>(slots[2].i32);
	const auto argsPointer = static_cast<std::uint32_t>(slots[3].i32);
	const std::int32_t arg = slots[4].i32;
	const auto planesPointer = static_cast<std::uint32_t>(slots[5].i32);
	const auto planesBytes = static_cast<std::uint32_t>(slots[6].i32);
	const auto flagsPointer = static_cast<std::uint32_t>(slots[7].i32);

	const std::uint32_t components = Components(property, set);
	if (components == 0 || count > MAX_ENTRIES) {
		Return(slots, 0, Status::InvalidArgument);
		return nullptr;
	}
	const std::size_t needed = std::size_t(components) * count * 4;
	if (planesBytes < needed) {
		Return(slots, static_cast<std::uint32_t>(needed), Status::BufferOverflow);
		return nullptr;
	}
	std::span<const std::uint8_t> ids, args;
	std::span<std::uint8_t> planes, flags;
	if (!state->memory.View(idsPointer, std::size_t(count) * 4, ids) ||
		(argsPointer != 0 && !state->memory.View(argsPointer, std::size_t(count) * 4, args)) ||
		!state->memory.MutableView(planesPointer, needed, planes) ||
		(flagsPointer != 0 && !state->memory.MutableView(flagsPointer, count, flags))) {
		Return(slots, 0, Status::OutOfBounds);
		return nullptr;
	}
	// one per-unit import's work for every entry
	if (!guard.Charge(std::uint64_t(count) * (components + 2)))
		return Trap(guard.Error());

	const Planes columns{planes.data(), count};
	if (!set)
		std::memset(planes.data(), 0, needed);
	std::uint32_t done = 0;
	for (std::uint32_t i = 0; i < count; ++i) {
		const bool ok = each(*state->native, static_cast<UnitColumn>(property), Load(ids, i),
			args.empty() ? arg : Load(args, i), columns, i);
		if (!flags.empty())
			flags[i] = ok ? 1 : 0;
		done += ok ? 1 : 0;
	}
	Return(slots, done, Status::Ok);
	return nullptr;
}

wasm_trap_t* GetUnits(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	return Column(environment, caller, slots, slotCount, false,
		[](const NativeInterface& native, UnitColumn property, std::int32_t unit, std::int32_t arg,
			const Planes& out, std::uint32_t i) {
			return ReadUnit(native, property, unit, arg, out, i);
		});
}

wasm_trap_t* SetUnits(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	return Column(environment, caller, slots, slotCount, true,
		[](const NativeInterface& native, UnitColumn property, std::int32_t unit, std::int32_t,
			const Planes& in, std::uint32_t i) {
			return WriteUnit(native, property, unit, in, i);
		});
}

wasm_trap_t* GetUnitsRulesParam(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || !Available(state))
		return Trap("Column host binding is unavailable");
	if (slots == nullptr || slotCount != 6)
		return Trap("Column core ABI signature mismatch");

	ImportGuard guard(state, 1);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const auto idsPointer = static_cast<std::uint32_t>(slots[2].i32);
	const auto count = static_cast<std::uint32_t>(slots[3].i32);
	const auto outPointer = static_cast<std::uint32_t>(slots[4].i32);
	const auto foundPointer = static_cast<std::uint32_t>(slots[5].i32);
	if (count > MAX_ENTRIES) {
		Return(slots, 0, Status::InvalidArgument);
		return nullptr;
	}
	GuestCString<> name;
	std::span<const std::uint8_t> ids;
	std::span<std::uint8_t> out, found;
	if (!name.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32), static_cast<std::uint32_t>(slots[1].i32)) ||
		!state->memory.View(idsPointer, std::size_t(count) * 4, ids) ||
		!state->memory.MutableView(outPointer, std::size_t(count) * 4, out) ||
		(foundPointer != 0 && !state->memory.MutableView(foundPointer, count, found))) {
		Return(slots, 0, Status::OutOfBounds);
		return nullptr;
	}
	if (!guard.Charge(std::uint64_t(count) * 4))
		return Trap(guard.Error());

	const Planes values{out.data(), count};
	std::uint32_t done = 0;
	for (std::uint32_t i = 0; i < count; ++i) {
		GetUnitRulesParamQuery query{};
		query.unitID = Load(ids, i);
		query.paramName = name.c_str();
		GetUnitRulesParamResult result{};
		state->native->rulesParams->GetUnitRulesParam(&query, &result);
		// as get-unit-rules-param-f32: a float parameter that exists
		const bool ok = !Failed(result.error) && result.exists && result.value.type == RULESPARAM_TYPE_FLOAT;
		values.F32(0, i, ok ? result.value.floatValue : 0.0f);
		if (!found.empty())
			found[i] = ok ? 1 : 0;
		done += ok ? 1 : 0;
	}
	Return(slots, done, Status::Ok);
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, std::size_t params,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasm_valkind_t kinds[8];
	for (std::size_t i = 0; i < params; ++i)
		kinds[i] = WASM_I32;
	const wasm_valkind_t results[] = {WASM_I64};
	auto* type = MakeFuncType(kinds, params, results, 1);
	auto* failure = wasmtime_linker_define_func_unchecked(linker, "spring:columns", 14,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (failure) {
		error = ErrorMessage(failure);
		return false;
	}
	return true;
}

} // namespace

bool RegisterColumnsImports(wasmtime_linker_t* linker, HostState* state, std::string& error)
{
	return Define(linker, "get-units", 8, GetUnits, state, error) &&
		Define(linker, "set-units", 8, SetUnits, state, error) &&
		Define(linker, "get-units-rules-param", 6, GetUnitsRulesParam, state, error);
}

#endif
} // namespace recoil::wasm::core

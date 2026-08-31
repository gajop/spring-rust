/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreBindings.h"

#include "System/BenchmarkCallins.h"
#include "Sim/Units/Scripts/UnitScriptFactory.h"
#include "Sim/Units/Scripts/UnitScript.h"
#include "Sim/Units/Scripts/NativeUnitScript.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"

#include <array>
#include <bit>
#include <cstdint>
#include <cstring>
#include <span>
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

class ImportBudgetGuard {
public:
	ImportBudgetGuard(HostState* state, std::uint64_t work, std::string& error)
	{
		if (state == nullptr) {
			error = "core Wasm host state is null";
			return;
		}
		budget = state->budget;
		if (budget == nullptr) {
			entered = true;
			return;
		}
		const bool allowReentry = budget->CallbackDepth() != 0 &&
			budget->CallbackReentryAllowed();
		if (!budget->EnterImport(allowReentry)) {
			error = "Wasm import re-entry denied";
			return;
		}
		entered = true;
		if (!budget->ChargeHost(work)) {
			budget->LeaveImport();
			entered = false;
			error = "Wasm callout host-work budget exhausted";
		}
	}

	~ImportBudgetGuard()
	{
		if (entered && budget != nullptr)
			budget->LeaveImport();
	}

	bool Ok() const { return entered; }

private:
	WasmExecutionBudget* budget = nullptr;
	bool entered = false;
};

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
	const std::uint32_t bits = std::bit_cast<std::uint32_t>(value);
	output[0] = static_cast<std::uint8_t>(bits);
	output[1] = static_cast<std::uint8_t>(bits >> 8);
	output[2] = static_cast<std::uint8_t>(bits >> 16);
	output[3] = static_cast<std::uint8_t>(bits >> 24);
}

template<std::size_t N>
bool WriteF32s(Memory& memory, std::uint32_t output,
	const std::array<float, N>& values)
{
	std::array<std::uint8_t, N * 4> wire{};
	for (std::size_t index = 0; index < N; ++index)
		EncodeF32LE(values[index], wire.data() + index * 4);
	return memory.Write(output, wire.data(), wire.size());
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
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 2, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	GetUnitDefIDQuery query{};
	query.unitID = slots[0].i32;
	GetUnitDefIDResult result{};
	state->native->unitsInfo->GetUnitDefID(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.unitDefID, NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitTeam(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitTeam == nullptr)
		return Trap("GetUnitTeam host binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetUnitTeam core ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 2, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	GetUnitTeamQuery query{};
	query.unitID = slots[0].i32;
	GetUnitTeamResult result{};
	state->native->unitsInfo->GetUnitTeam(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.teamID, NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitIsDead(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitIsDead == nullptr)
		return Trap("GetUnitIsDead host binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetUnitIsDead core ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 2, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	GetUnitIsDeadQuery query{};
	query.unitID = slots[0].i32;
	GetUnitIsDeadResult result{};
	state->native->unitsInfo->GetUnitIsDead(&query, &result);
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.isDead ? 1 : 0,
		NativeErrorCode(result.error)));
	return nullptr;
}

wasm_trap_t* GetUnitExperience(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitExperience == nullptr)
		return Trap("GetUnitExperience host binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("GetUnitExperience core ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 2, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	GetUnitExperienceQuery query{};
	query.unitID = slots[0].i32;
	GetUnitExperienceResult result{};
	state->native->unitsInfo->GetUnitExperience(&query, &result);
	const std::int32_t bits = std::bit_cast<std::int32_t>(result.experience);
	slots[0].i64 = static_cast<std::int64_t>(PackI32(bits, NativeErrorCode(result.error)));
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
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 4, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
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
	const std::array<float, 3> values = {
		result.position.x, result.position.y, result.position.z,
	};
	if (!WriteF32s(state->memory, output, values)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetUnitVelocity(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitVelocity == nullptr)
		return Trap("GetUnitVelocity host binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetUnitVelocity core ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 3, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	std::string memoryError;
	if (!EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	GetUnitVelocityQuery query{};
	query.unitID = slots[0].i32;
	GetUnitVelocityResult result{};
	state->native->unitsInfo->GetUnitVelocity(&query, &result);
	const std::int32_t errorCode = NativeErrorCode(result.error);
	if (errorCode != 0) {
		slots[0].i32 = errorCode;
		return nullptr;
	}
	const std::array<float, 3> values = {
		result.velocity.x, result.velocity.y, result.velocity.z,
	};
	if (!WriteF32s(state->memory, static_cast<std::uint32_t>(slots[1].i32), values)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* GetUnitHealth(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsInfo == nullptr ||
		state->native->unitsInfo->GetUnitHealth == nullptr)
		return Trap("GetUnitHealth host binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap("GetUnitHealth core ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 3, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	std::string memoryError;
	if (!EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	GetUnitHealthQuery query{};
	query.unitID = slots[0].i32;
	GetUnitHealthResult result{};
	state->native->unitsInfo->GetUnitHealth(&query, &result);
	const std::int32_t errorCode = NativeErrorCode(result.error);
	if (errorCode != 0) {
		slots[0].i32 = errorCode;
		return nullptr;
	}
	const std::array<float, 5> values = {
		result.health.health,
		result.health.maxHealth,
		result.health.paralyzeDamage,
		result.health.captureProgress,
		result.health.buildProgress,
	};
	if (!WriteF32s(state->memory, static_cast<std::uint32_t>(slots[1].i32), values)) {
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

bool ResolveRaw(RawExport& target, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, const char* name,
	std::span<const wasm_valkind_t> params,
	std::span<const wasm_valkind_t> results, std::string& error)
{
	return target.Resolve(context, instance, name, std::char_traits<char>::length(name),
		params, results, true, error);
}

wasm_trap_t* CusAttach(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 4)
		return Trap("CUS attach ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 5, budgetError);
	// The attach path is only entered from a synced guest. Keep the result
	// packed so invalid IDs become ordinary API errors instead of Wasm traps.
	const auto fail = [&slots](std::int32_t status) {
		slots[0].i64 = static_cast<std::int64_t>(PackI32(0, status));
	};
	if (!budgetGuard.Ok()) {
		fail(static_cast<std::int32_t>(Status::Internal));
		return nullptr;
	}
	if (state->cusBackend == nullptr) {
		fail(static_cast<std::int32_t>(Status::NotAvailable));
		return nullptr;
	}
	const int unitID = slots[0].i32;
	const std::uint32_t instanceID = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint64_t capabilities =
		static_cast<std::uint64_t>(static_cast<std::uint32_t>(slots[2].i32)) |
		(static_cast<std::uint64_t>(static_cast<std::uint32_t>(slots[3].i32)) << 32);
	CUnit* unit = unitID >= 0 ? unitHandler.GetUnit(static_cast<unsigned int>(unitID)) : nullptr;
	if (unit == nullptr) {
		fail(static_cast<std::int32_t>(Status::InvalidId));
		return nullptr;
	}
	CUnitScript* script = CUnitScriptFactory::AttachCusScript(
		unit, state->cusBackend, instanceID, capabilities);
	if (script == nullptr) {
		fail(static_cast<std::int32_t>(Status::OperationFailed));
		return nullptr;
	}
	state->cusBackend->StartCreate(static_cast<CNativeUnitScript*>(script));
	slots[0].i64 = static_cast<std::int64_t>(PackI32(1, 0));
	return nullptr;
}

wasm_trap_t*CusOperation(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 10)
		return Trap("CUS operation ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 3, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	if (state->native == nullptr || state->native->cus == nullptr ||
		state->native->cus->Operation == nullptr) {
		slots[0].i64 = static_cast<std::int64_t>(PackI32(0,
			static_cast<std::int32_t>(Status::NotAvailable)));
		return nullptr;
	}
	CusOperationQuery query = {
		.unitID = slots[0].i32,
		.instanceID = static_cast<std::uint32_t>(slots[1].i32),
		.operation = static_cast<std::uint32_t>(slots[2].i32),
		.piece = slots[3].i32,
		.axis = slots[4].i32,
		.target = slots[5].i32,
		.value = slots[6].i32,
		.first = slots[7].f32,
		.second = slots[8].f32,
		.third = slots[9].f32,
	};
	CusOperationResult result{};
	state->native->cus->Operation(&query, &result);
	const std::int32_t status = result.error != nullptr ? result.error->code
		: (result.completed != 0 ? 0 : static_cast<std::int32_t>(Status::OperationFailed));
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.value, status));
	return nullptr;
}

wasm_trap_t* CusAnimationActive(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 5)
		return Trap("CUS animation ABI signature mismatch");
	std::string budgetError;
	ImportBudgetGuard budgetGuard(state, 2, budgetError);
	if (!budgetGuard.Ok())
		return Trap(budgetError);
	if (state->native == nullptr || state->native->cus == nullptr ||
		state->native->cus->AnimationActive == nullptr) {
		slots[0].i64 = static_cast<std::int64_t>(PackI32(0,
			static_cast<std::int32_t>(Status::NotAvailable)));
		return nullptr;
	}
	const CusAnimationQuery query = {
		.unitID = slots[0].i32,
		.instanceID = static_cast<std::uint32_t>(slots[1].i32),
		.animation = static_cast<std::uint32_t>(slots[2].i32),
		.piece = slots[3].i32,
		.axis = slots[4].i32,
	};
	CusAnimationResult result{};
	state->native->cus->AnimationActive(&query, &result);
	const std::int32_t status = result.error != nullptr ? result.error->code : 0;
	slots[0].i64 = static_cast<std::int64_t>(PackI32(result.active != 0, status));
	return nullptr;
}

} // namespace

bool RegisterFastImports(wasmtime_linker_t* linker, HostState* state, std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr) {
		error = "cannot register core Wasm imports without linker/host state/native API";
		return false;
	}

	const wasm_valkind_t i32ToI64Params[] = {WASM_I32};
	const wasm_valkind_t i64Result[] = {WASM_I64};
	if (!DefineUnchecked(linker, "spring:units-info", "get-unit-def-id",
			MakeFuncType(i32ToI64Params, 1, i64Result, 1), GetUnitDefID, state, error) ||
		!DefineUnchecked(linker, "spring:units-info", "get-unit-team",
			MakeFuncType(i32ToI64Params, 1, i64Result, 1), GetUnitTeam, state, error) ||
		!DefineUnchecked(linker, "spring:units-info", "get-unit-is-dead",
			MakeFuncType(i32ToI64Params, 1, i64Result, 1), GetUnitIsDead, state, error) ||
		!DefineUnchecked(linker, "spring:units-info", "get-unit-experience",
			MakeFuncType(i32ToI64Params, 1, i64Result, 1), GetUnitExperience, state, error))
		return false;

	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32};
		const wasm_valkind_t results[] = {WASM_I32};
		if (!DefineUnchecked(linker, "spring:units-info", "get-unit-position",
				MakeFuncType(params, 3, results, 1), GetUnitPosition, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32};
		const wasm_valkind_t results[] = {WASM_I32};
		if (!DefineUnchecked(linker, "spring:units-info", "get-unit-velocity",
				MakeFuncType(params, 2, results, 1), GetUnitVelocity, state, error) ||
			!DefineUnchecked(linker, "spring:units-info", "get-unit-health",
				MakeFuncType(params, 2, results, 1), GetUnitHealth, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
		const wasm_valkind_t results[] = {WASM_I64};
		if (!DefineUnchecked(linker, "spring:cus", "attach",
				MakeFuncType(params, 4, results, 1), CusAttach, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {
			WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
			WASM_I32, WASM_I32, WASM_F32, WASM_F32, WASM_F32,
		};
		const wasm_valkind_t results[] = {WASM_I64};
		if (!DefineUnchecked(linker, "spring:cus", "operation",
				MakeFuncType(params, 10, results, 1),CusOperation, state, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32};
		const wasm_valkind_t results[] = {WASM_I64};
		if (!DefineUnchecked(linker, "spring:cus", "animation-active",
				MakeFuncType(params, 5, results, 1), CusAnimationActive, state, error))
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
	if (!gameFrame.Resolve(context, instance, gameFrameName, sizeof(gameFrameName) - 1,
		true, error))
		return false;
	constexpr char gameFramePostName[] = "spring:callin/game-frame-post";
	if (!gameFramePost.Resolve(context, instance, gameFramePostName,
		sizeof(gameFramePostName) - 1, true, error))
		return false;
	{
		const wasm_valkind_t params[] = {WASM_F32};
		if (!ResolveRaw(update, context, instance, "spring:callin/update",
			std::span<const wasm_valkind_t>(params, 1), {}, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
		if (!ResolveRaw(unitCreated, context, instance, "spring:callin/unit-created",
			std::span<const wasm_valkind_t>(params, 4), {}, error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {
			WASM_I32, WASM_I32, WASM_I32, WASM_F32, WASM_I32,
			WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
		};
		const wasm_valkind_t results[] = {WASM_I64};
		if (!ResolveRaw(unitPreDamaged, context, instance, "spring:callin/unit-pre-damaged",
			std::span<const wasm_valkind_t>(params, 10),
			std::span<const wasm_valkind_t>(results, 1), error))
			return false;
	}
	{
		const wasm_valkind_t params[] = {
			WASM_I32, WASM_I32, WASM_I32, WASM_I32,
			WASM_F32, WASM_F32, WASM_F32, WASM_I32,
		};
		const wasm_valkind_t results[] = {WASM_I32};
		if (!ResolveRaw(allowUnitCreation, context, instance,
			"spring:callin/allow-unit-creation",
			std::span<const wasm_valkind_t>(params, 8),
			std::span<const wasm_valkind_t>(results, 1), error))
			return false;
	}
	if (!ResolveRaw(drawWorld, context, instance, "spring:callin/draw-world", {}, {}, error))
		return false;
	return true;
}

#endif

} // namespace recoil::wasm::core

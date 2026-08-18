/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// C++ side of the typed Rust Wasmtime host.
//
// One shim per callout the benchmark table exercises.  The dynamic C API path
// lowers a wasmtime_component_val_t into a WasmValue tree and then lowers that
// into the native query struct; measured on callout_vec3 the two trees cost
// 646 of its 881 ns.  Wasmtime's typed Rust API hands the host the argument
// already typed, so these shims take POD arguments and go straight to the
// NativeInterface function.
//
// Contract, matching rust/crates/spring-wasm-typed-host/src/ffi.rs:
//   - return 0 on success, otherwise the native error code (never 0)
//   - borrowed outputs (strings, lists) point into a thread-local scratch
//     buffer and stay valid until the next shim call on the same thread.
//     Guest calls are synchronous and single-threaded, and the Rust side
//     copies before returning to the guest, so one buffer per kind suffices.

#include <cstdint>
#include <string>
#include <vector>

#include "NativeInterface/NativeInterface.h"
#include "WasmInterface/WasmTypedHost.h"

namespace {

// A code of 0 would read as success, so an error carrying one is remapped.
constexpr std::int32_t kUnavailable = -2;

template<typename Result>
std::int32_t NativeStatus(const Result& result)
{
	if (result.error == nullptr)
		return 0;
	return result.error->code != 0 ? result.error->code : kUnavailable;
}

std::string& StringScratch()
{
	static thread_local std::string scratch;
	return scratch;
}

std::vector<std::int32_t>& IdScratch()
{
	static thread_local std::vector<std::int32_t> scratch;
	return scratch;
}

std::vector<SpringTypedCommand>& CommandScratch()
{
	static thread_local std::vector<SpringTypedCommand> scratch;
	return scratch;
}

std::vector<float>& ParamScratch()
{
	static thread_local std::vector<float> scratch;
	return scratch;
}

} // namespace

extern "C" {

// ------------------------------------------------------------------ messages

std::int32_t spring_wasm_typed_messages_send_lua_rules_msg(void* native,
	const char* message, std::size_t messageLength, bool* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->messages == nullptr ||
		nativeInterface->messages->SendLuaRulesMsg == nullptr)
		return kUnavailable;
	// The guest string is not NUL-terminated; the native API takes a C string.
	auto& owned = StringScratch();
	owned.assign(message, messageLength);
	SendLuaRulesQuery query{};
	SendLuaRulesResult result{};
	query.message = owned.c_str();
	nativeInterface->messages->SendLuaRulesMsg(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.success;
	return 0;
}

// ----------------------------------------------------------------- profiling

std::int32_t spring_wasm_typed_profiling_get_timer_micros(void* native,
	std::uint8_t unused, std::uint64_t* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->profiling == nullptr ||
		nativeInterface->profiling->GetTimerMicros == nullptr)
		return kUnavailable;
	GetTimerMicrosQuery query{};
	GetTimerMicrosResult result{};
	query._unused = unused;
	nativeInterface->profiling->GetTimerMicros(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.timer;
	return 0;
}

/// Writes the eight KB/allocation counters into `out`.
std::int32_t spring_wasm_typed_profiling_get_lua_mem_usage(void* native,
	std::uint8_t unused, float* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->profiling == nullptr ||
		nativeInterface->profiling->GetLuaMemUsage == nullptr)
		return kUnavailable;
	GetLuaMemUsageQuery query{};
	GetLuaMemUsageResult result{};
	query._unused = unused;
	nativeInterface->profiling->GetLuaMemUsage(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	out[0] = result.handleAllocedKB;
	out[1] = result.handleAllocsK;
	out[2] = result.globalAllocedKB;
	out[3] = result.globalAllocsK;
	out[4] = result.unsyncedAllocedKB;
	out[5] = result.unsyncedAllocsK;
	out[6] = result.syncedAllocedKB;
	out[7] = result.syncedAllocsK;
	return 0;
}

std::int32_t spring_wasm_typed_profiling_get_synced_gc_info(void* native,
	bool collect, float* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->profiling == nullptr ||
		nativeInterface->profiling->GetSyncedGCInfo == nullptr)
		return kUnavailable;
	GetSyncedGCInfoQuery query{collect};
	GetSyncedGCInfoResult result{};
	nativeInterface->profiling->GetSyncedGCInfo(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.gcKB;
	return 0;
}

// --------------------------------------------------------------- rules-params

std::int32_t spring_wasm_typed_rules_params_get_unit_rules_param(void* native,
	std::int32_t unitID, const char* name, std::size_t nameLength,
	std::int32_t* outType, bool* outBool, float* outFloat,
	const char** outString, std::size_t* outStringLength,
	std::int32_t* outLos, bool* outExists)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->rulesParams == nullptr ||
		nativeInterface->rulesParams->GetUnitRulesParam == nullptr)
		return kUnavailable;
	auto& owned = StringScratch();
	owned.assign(name, nameLength);
	GetUnitRulesParamQuery query{};
	GetUnitRulesParamResult result{};
	query.unitID = unitID;
	query.paramName = owned.c_str();
	nativeInterface->rulesParams->GetUnitRulesParam(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*outType = static_cast<std::int32_t>(result.value.type);
	*outBool = false;
	*outFloat = 0.0f;
	*outString = "";
	*outStringLength = 0;
	switch (result.value.type) {
		case RULESPARAM_TYPE_BOOL:
			*outBool = result.value.boolValue;
			break;
		case RULESPARAM_TYPE_STRING:
			if (result.value.stringValue != nullptr) {
				*outString = result.value.stringValue;
				*outStringLength = std::char_traits<char>::length(result.value.stringValue);
			}
			break;
		case RULESPARAM_TYPE_FLOAT:
		default:
			*outFloat = result.value.floatValue;
			break;
	}
	*outLos = result.los;
	*outExists = result.exists;
	return 0;
}

std::int32_t spring_wasm_typed_rules_params_set_unit_rules_param(void* native,
	std::int32_t unitID, const char* name, std::size_t nameLength,
	std::int32_t valueType, bool valueBool, float valueFloat,
	const char* valueString, std::size_t valueStringLength,
	std::int32_t los, bool* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->rulesParams == nullptr ||
		nativeInterface->rulesParams->SetUnitRulesParam == nullptr)
		return kUnavailable;
	// Both strings are borrowed from the guest and neither is NUL-terminated,
	// so they share the scratch buffer with an explicit split.
	auto& owned = StringScratch();
	owned.assign(name, nameLength);
	owned.push_back('\0');
	const std::size_t valueOffset = owned.size();
	owned.append(valueString, valueStringLength);
	owned.push_back('\0');
	SetUnitRulesParamQuery query{};
	SetUnitRulesParamResult result{};
	query.unitID = unitID;
	query.paramName = owned.c_str();
	query.los = los;
	query.value.type = static_cast<RulesParamType>(valueType);
	switch (query.value.type) {
		case RULESPARAM_TYPE_BOOL:
			query.value.boolValue = valueBool;
			break;
		case RULESPARAM_TYPE_STRING:
			query.value.stringValue = owned.c_str() + valueOffset;
			break;
		case RULESPARAM_TYPE_FLOAT:
		default:
			query.value.floatValue = valueFloat;
			break;
	}
	nativeInterface->rulesParams->SetUnitRulesParam(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.success;
	return 0;
}

// ------------------------------------------------------------------- terrain

std::int32_t spring_wasm_typed_terrain_get_ground_orig_height(void* native,
	float x, float z, float* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->terrain == nullptr ||
		nativeInterface->terrain->GetGroundOrigHeight == nullptr)
		return kUnavailable;
	GetGroundOrigHeightQuery query{};
	GetGroundOrigHeightResult result{};
	query.x = x;
	query.z = z;
	nativeInterface->terrain->GetGroundOrigHeight(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.height;
	return 0;
}

// ----------------------------------------------------------- terrain-control

std::int32_t spring_wasm_typed_terrain_control_level_height_map(void* native,
	float x1, float z1, float x2, float z2, float height, bool* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->syncedCtrl == nullptr ||
		nativeInterface->syncedCtrl->terrain == nullptr ||
		nativeInterface->syncedCtrl->terrain->LevelHeightMap == nullptr)
		return kUnavailable;
	LevelHeightMapQuery query{};
	LevelHeightMapResult result{};
	query.x1 = x1;
	query.z1 = z1;
	query.x2 = x2;
	query.z2 = z2;
	query.height = height;
	nativeInterface->syncedCtrl->terrain->LevelHeightMap(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.success;
	return 0;
}

std::int32_t spring_wasm_typed_terrain_control_set_height_map(void* native,
	float x, float z, float height, float terraform, bool* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->syncedCtrl == nullptr ||
		nativeInterface->syncedCtrl->terrain == nullptr ||
		nativeInterface->syncedCtrl->terrain->SetHeightMap == nullptr)
		return kUnavailable;
	SetHeightMapQuery query{};
	SetHeightMapResult result{};
	query.x = x;
	query.z = z;
	query.height = height;
	query.terraform = terraform;
	nativeInterface->syncedCtrl->terrain->SetHeightMap(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.success;
	return 0;
}

// The guest callback crosses C++ to Rust to guest: the native API takes a
// plain thunk plus context, and the Rust side supplies both so it can re-enter
// the component with the store it holds.
std::int32_t spring_wasm_typed_terrain_control_set_height_map_func(void* native,
	NativeCallback trampoline, void* trampolineContext, bool* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->syncedCtrl == nullptr ||
		nativeInterface->syncedCtrl->terrain == nullptr ||
		nativeInterface->syncedCtrl->terrain->SetHeightMapFunc == nullptr)
		return kUnavailable;
	SetHeightMapFuncQuery query{trampoline, trampolineContext};
	SetHeightMapFuncResult result{};
	nativeInterface->syncedCtrl->terrain->SetHeightMapFunc(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.success;
	return 0;
}

// -------------------------------------------------------------- unit-control

std::int32_t spring_wasm_typed_unit_control_give_order_to_unit(void* native,
	std::int32_t unitID, std::int32_t cmdID, const float* params,
	std::size_t paramCount, std::uint32_t options, std::int32_t timeout,
	bool* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->syncedCtrl == nullptr ||
		nativeInterface->syncedCtrl->unit == nullptr ||
		nativeInterface->syncedCtrl->unit->GiveOrderToUnit == nullptr)
		return kUnavailable;
	GiveOrderToUnitQuery query{};
	GiveOrderToUnitResult result{};
	query.unitID = unitID;
	query.cmdID = cmdID;
	// The guest list is already a flat f32 buffer, so it is passed through.
	query.params = const_cast<float*>(params);
	query.paramCount = static_cast<std::uint32_t>(paramCount);
	query.options = options;
	query.timeout = timeout;
	nativeInterface->syncedCtrl->unit->GiveOrderToUnit(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.success;
	return 0;
}

// ----------------------------------------------------------------- unit-defs

std::int32_t spring_wasm_typed_unit_defs_get_unit_def_name(void* native,
	std::int32_t unitDefID, const char** out, std::size_t* outLength)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->unitDefs == nullptr ||
		nativeInterface->unitDefs->GetUnitDefName == nullptr)
		return kUnavailable;
	GetUnitDefNameQuery query{};
	GetUnitDefNameResult result{};
	query.unitDefID = unitDefID;
	nativeInterface->unitDefs->GetUnitDefName(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	if (result.name == nullptr) {
		*out = "";
		*outLength = 0;
		return 0;
	}
	*out = result.name;
	*outLength = std::char_traits<char>::length(result.name);
	return 0;
}

// ------------------------------------------------------------ units-commands

std::int32_t spring_wasm_typed_units_commands_get_unit_commands(void* native,
	std::int32_t unitID, std::uint32_t maxCommands,
	const SpringTypedCommand** outCommands, std::size_t* outCommandCount,
	const float** outParams, std::size_t* outParamCount)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->unitsCommands == nullptr ||
		nativeInterface->unitsCommands->GetUnitCommands == nullptr)
		return kUnavailable;
	GetUnitCommandsQuery query{};
	GetUnitCommandsResult result{};
	query.unitID = unitID;
	query.maxCommands = maxCommands;
	nativeInterface->unitsCommands->GetUnitCommands(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	// Each native command owns a separate params pointer.  Flatten them into
	// one buffer so the Rust side can slice it without a second crossing.
	auto& commands = CommandScratch();
	auto& params = ParamScratch();
	commands.clear();
	params.clear();
	commands.reserve(result.count);
	for (std::uint32_t index = 0; index < result.count; ++index) {
		const CommandFFI& command = result.commands[index];
		SpringTypedCommand raw{};
		raw.cmdID = command.cmdID;
		raw.tag = command.tag;
		raw.aiCommandID = command.aiCommandID;
		raw.timeOut = command.timeOut;
		raw.options = command.options;
		raw.paramOffset = static_cast<std::uint32_t>(params.size());
		raw.paramCount = command.paramCount;
		if (command.params != nullptr && command.paramCount > 0)
			params.insert(params.end(), command.params, command.params + command.paramCount);
		else
			raw.paramCount = 0;
		commands.push_back(raw);
	}
	*outCommands = commands.data();
	*outCommandCount = commands.size();
	*outParams = params.data();
	*outParamCount = params.size();
	return 0;
}

// ---------------------------------------------------------------- units-info

std::int32_t spring_wasm_typed_units_info_get_unit_def_id(void* native,
	std::int32_t unitID, std::int32_t* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->unitsInfo == nullptr ||
		nativeInterface->unitsInfo->GetUnitDefID == nullptr)
		return kUnavailable;
	GetUnitDefIDQuery query{};
	GetUnitDefIDResult result{};
	query.unitID = unitID;
	nativeInterface->unitsInfo->GetUnitDefID(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	*out = result.unitDefID;
	return 0;
}

std::int32_t spring_wasm_typed_units_info_get_unit_health(void* native,
	std::int32_t unitID, float* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->unitsInfo == nullptr ||
		nativeInterface->unitsInfo->GetUnitHealth == nullptr)
		return kUnavailable;
	GetUnitHealthQuery query{};
	GetUnitHealthResult result{};
	query.unitID = unitID;
	nativeInterface->unitsInfo->GetUnitHealth(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	out[0] = result.health.health;
	out[1] = result.health.maxHealth;
	out[2] = result.health.paralyzeDamage;
	out[3] = result.health.captureProgress;
	out[4] = result.health.buildProgress;
	return 0;
}

std::int32_t spring_wasm_typed_units_info_get_unit_position(void* native,
	std::int32_t unitID, bool midPos, bool aimPos, float* out)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->unitsInfo == nullptr ||
		nativeInterface->unitsInfo->GetUnitPosition == nullptr)
		return kUnavailable;
	GetUnitPositionQuery query{};
	GetUnitPositionResult result{};
	query.unitID = unitID;
	query.options.midPos = midPos;
	query.options.aimPos = aimPos;
	nativeInterface->unitsInfo->GetUnitPosition(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	out[0] = result.position.x;
	out[1] = result.position.y;
	out[2] = result.position.z;
	return 0;
}

// --------------------------------------------------------------- units-query

std::int32_t spring_wasm_typed_units_query_get_team_units(void* native,
	std::int32_t teamID, const std::int32_t** out, std::size_t* outLength)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->unitsQuery == nullptr ||
		nativeInterface->unitsQuery->GetTeamUnits == nullptr)
		return kUnavailable;
	GetTeamUnitsQuery query{};
	GetTeamUnitsResult result{};
	query.teamID = teamID;
	nativeInterface->unitsQuery->GetTeamUnits(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	auto& scratch = IdScratch();
	scratch.assign(result.units, result.units + result.count);
	*out = scratch.data();
	*outLength = scratch.size();
	return 0;
}

std::int32_t spring_wasm_typed_units_query_get_units_in_cylinder(void* native,
	float x, float z, float radius, std::int32_t allegiance,
	const std::int32_t** out, std::size_t* outLength)
{
	auto* nativeInterface = static_cast<NativeInterface*>(native);
	if (nativeInterface == nullptr || nativeInterface->unitsQuery == nullptr ||
		nativeInterface->unitsQuery->GetUnitsInCylinder == nullptr)
		return kUnavailable;
	GetUnitsInCylinderQuery query{};
	GetUnitsInCylinderResult result{};
	query.x = x;
	query.z = z;
	query.radius = radius;
	query.allegiance = allegiance;
	nativeInterface->unitsQuery->GetUnitsInCylinder(&query, &result);
	if (const auto status = NativeStatus(result); status != 0)
		return status;
	auto& scratch = IdScratch();
	scratch.assign(result.units, result.units + result.count);
	*out = scratch.data();
	*outLength = scratch.size();
	return 0;
}

} // extern "C"

// The Rust host is loaded with dlopen and cannot resolve engine symbols by
// name, so it receives this table instead.  Field order must match
// `ShimTable` in the crate's ffi.rs; the designated initializers make a
// reordering a compile error rather than a silent mis-dispatch.
const SpringTypedShimTable& TypedHostShimTable()
{
	static const SpringTypedShimTable table{
		.messages_send_lua_rules_msg = &spring_wasm_typed_messages_send_lua_rules_msg,
		.profiling_get_timer_micros = &spring_wasm_typed_profiling_get_timer_micros,
		.rules_params_get_unit_rules_param = &spring_wasm_typed_rules_params_get_unit_rules_param,
		.rules_params_set_unit_rules_param = &spring_wasm_typed_rules_params_set_unit_rules_param,
		.terrain_get_ground_orig_height = &spring_wasm_typed_terrain_get_ground_orig_height,
		.terrain_control_level_height_map = &spring_wasm_typed_terrain_control_level_height_map,
		.terrain_control_set_height_map = &spring_wasm_typed_terrain_control_set_height_map,
		.unit_control_give_order_to_unit = &spring_wasm_typed_unit_control_give_order_to_unit,
		.unit_defs_get_unit_def_name = &spring_wasm_typed_unit_defs_get_unit_def_name,
		.units_commands_get_unit_commands = &spring_wasm_typed_units_commands_get_unit_commands,
		.units_info_get_unit_def_id = &spring_wasm_typed_units_info_get_unit_def_id,
		.units_info_get_unit_health = &spring_wasm_typed_units_info_get_unit_health,
		.units_info_get_unit_position = &spring_wasm_typed_units_info_get_unit_position,
		.units_query_get_team_units = &spring_wasm_typed_units_query_get_team_units,
		.units_query_get_units_in_cylinder = &spring_wasm_typed_units_query_get_units_in_cylinder,
		.terrain_control_set_height_map_func = &spring_wasm_typed_terrain_control_set_height_map_func,
		.profiling_get_lua_mem_usage = &spring_wasm_typed_profiling_get_lua_mem_usage,
		.profiling_get_synced_gc_info = &spring_wasm_typed_profiling_get_synced_gc_info,
	};
	return table;
}

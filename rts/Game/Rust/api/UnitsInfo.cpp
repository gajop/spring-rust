#include "UnitsInfo.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"

namespace {

static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "UnitsInfo API partially implemented - many functions are stubs"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

// Basic unit info - simplified implementations
static BoolResult NativeValidUnitID(int32_t unitID)
{
	BoolResult result = {};
	result.value = (unitHandler.GetUnit(unitID) != nullptr);
	return result;
}

static UInt32Result NativeGetUnitDefID(int32_t unitID)
{
	UInt32Result result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}
	result.value = static_cast<uint32_t>(unit->unitDef->id);
	return result;
}

static Int32Result NativeGetUnitTeam(int32_t unitID)
{
	Int32Result result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}
	result.value = unit->team;
	return result;
}

static Float3Result NativeGetUnitPosition(int32_t unitID)
{
	Float3Result result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}
	result.value.x = unit->pos.x;
	result.value.y = unit->pos.y;
	result.value.z = unit->pos.z;
	return result;
}

static FloatResult NativeGetUnitHealth(int32_t unitID)
{
	FloatResult result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}
	result.value = unit->health;
	return result;
}

// Many more functions would go here - stubbed for now
// Total: ~80 functions for complete UnitsInfo API
// Including: velocity, rotation, resources, states, etc.

} // namespace

const UnitsInfoApi UNITS_INFO_API = {
	.ValidUnitID = NativeValidUnitID,
	.GetUnitDefID = NativeGetUnitDefID,
	.GetUnitTeam = NativeGetUnitTeam,
	.GetUnitPosition = NativeGetUnitPosition,
	.GetUnitHealth = NativeGetUnitHealth,
	// ... ~75 more function pointers would be initialized here
};

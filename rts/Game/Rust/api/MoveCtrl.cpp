#include "MoveCtrl.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/MoveTypes/MoveType.h"
#include "Sim/MoveTypes/GroundMoveType.h"
#include "Sim/MoveTypes/AAirMoveType.h"
#include "System/StringUtil.h"
#include <vector>

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Move control system not ready"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

// Get move type data
static MoveTypeDataResult NativeGetUnitMoveTypeData(int32_t unitID)
{
	MoveTypeDataResult result = {};

	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	const AMoveType* mt = unit->moveType;

	// Use static storage for name
	static thread_local std::string moveTypeName;
	moveTypeName = "unknown";

	if (dynamic_cast<const CGroundMoveType*>(mt) != nullptr) {
		moveTypeName = "ground";
		const CGroundMoveType* gmt = static_cast<const CGroundMoveType*>(mt);

		result.data.turnRate = gmt->turnRate;
		result.data.accRate = gmt->accRate;
		result.data.decRate = gmt->decRate;
		result.data.maxReverseSpeed = gmt->maxReverseSpeed;
		result.data.wantedSpeed = gmt->wantedSpeed;
		result.data.currentSpeed = gmt->currentSpeed;
		result.data.deltaSpeed = gmt->deltaSpeed;
	} else if (dynamic_cast<const AAirMoveType*>(mt) != nullptr) {
		moveTypeName = "air";
		const AAirMoveType* amt = static_cast<const AAirMoveType*>(mt);

		result.data.maxBank = amt->maxBank;
		result.data.maxPitch = amt->maxPitch;
		result.data.maxAileron = amt->maxAileron;
		result.data.maxElevator = amt->maxElevator;
		result.data.maxRudder = amt->maxRudder;
	} else {
		moveTypeName = "static";
	}

	result.data.name = moveTypeName.c_str();
	result.data.maxSpeed = mt->maxSpeed;
	result.data.maxWantedSpeed = mt->maxWantedSpeed;
	result.data.goalX = mt->goalPos.x;
	result.data.goalY = mt->goalPos.y;
	result.data.goalZ = mt->goalPos.z;

	return result;
}

// Get estimated path
static EstimatedPathResult NativeGetUnitEstimatedPath(int32_t unitID)
{
	EstimatedPathResult result = {};

	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	// Simplified: return goal as single waypoint
	static thread_local std::vector<PathWaypoint> waypoints;
	waypoints.clear();

	PathWaypoint wp;
	wp.pos.x = unit->moveType->goalPos.x;
	wp.pos.y = unit->moveType->goalPos.y;
	wp.pos.z = unit->moveType->goalPos.z;
	wp.eta = 0.0f; // ETA not calculated

	waypoints.push_back(wp);

	result.waypoints = waypoints.data();
	result.count = static_cast<uint32_t>(waypoints.size());
	return result;
}

} // namespace

const MoveCtrlApi MOVE_CTRL_API = {
	.GetUnitMoveTypeData = NativeGetUnitMoveTypeData,
	.GetUnitEstimatedPath = NativeGetUnitEstimatedPath,
};

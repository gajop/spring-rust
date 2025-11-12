#include "MoveCtrl.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/MoveTypes/MoveType.h"
#include "Sim/MoveTypes/GroundMoveType.h"
#include "Sim/MoveTypes/AAirMoveType.h"
#include "System/StringUtil.h"

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Move control system not ready"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

// Get move type data
static void NativeGetUnitMoveTypeData(const GetUnitMoveTypeDataQuery* query, GetUnitMoveTypeDataResult* result)
{
	bufferPos = 0;

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const AMoveType* mt = unit->moveType;

	// Determine move type name and store in scratch buffer
	const char* moveTypeName = "unknown";
	if (dynamic_cast<const CGroundMoveType*>(mt) != nullptr) {
		moveTypeName = "ground";
		const CGroundMoveType* gmt = static_cast<const CGroundMoveType*>(mt);

		result->data.turnRate = gmt->turnRate;
		result->data.accRate = gmt->accRate;
		result->data.decRate = gmt->decRate;
		result->data.maxReverseSpeed = gmt->maxReverseSpeed;
		result->data.wantedSpeed = gmt->wantedSpeed;
		result->data.currentSpeed = gmt->currentSpeed;
		result->data.deltaSpeed = gmt->deltaSpeed;
	} else if (dynamic_cast<const AAirMoveType*>(mt) != nullptr) {
		moveTypeName = "air";
		const AAirMoveType* amt = static_cast<const AAirMoveType*>(mt);

		result->data.maxBank = amt->maxBank;
		result->data.maxPitch = amt->maxPitch;
		result->data.maxAileron = amt->maxAileron;
		result->data.maxElevator = amt->maxElevator;
		result->data.maxRudder = amt->maxRudder;
	} else {
		moveTypeName = "static";
	}

	result->error = nullptr;
	result->data.name = moveTypeName;
	result->data.maxSpeed = mt->maxSpeed;
	result->data.maxWantedSpeed = mt->maxWantedSpeed;
	result->data.goalX = mt->goalPos.x;
	result->data.goalY = mt->goalPos.y;
	result->data.goalZ = mt->goalPos.z;
}

// Get estimated path
static void NativeGetUnitEstimatedPath(const GetUnitEstimatedPathQuery* query, GetUnitEstimatedPathResult* result)
{
	bufferPos = 0;

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Simplified: return goal as single waypoint using scratch buffer
	PathWaypoint* waypoints = reinterpret_cast<PathWaypoint*>(&scratchBuffer[bufferPos]);

	if (bufferPos + sizeof(PathWaypoint) > sizeof(scratchBuffer)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	waypoints[0].pos.x = unit->moveType->goalPos.x;
	waypoints[0].pos.y = unit->moveType->goalPos.y;
	waypoints[0].pos.z = unit->moveType->goalPos.z;
	waypoints[0].eta = 0.0f; // ETA not calculated

	bufferPos += sizeof(PathWaypoint);

	result->error = nullptr;
	result->waypoints = waypoints;
	result->count = 1;
}

} // namespace

const MoveCtrlApi MOVE_CTRL_API = {
	.GetUnitMoveTypeData = NativeGetUnitMoveTypeData,
	.GetUnitEstimatedPath = NativeGetUnitEstimatedPath,
};

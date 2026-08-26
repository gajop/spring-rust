#include "MoveCtrl.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/MoveTypes/MoveType.h"
#include "Sim/MoveTypes/ScriptMoveType.h"
#include "Sim/MoveTypes/GroundMoveType.h"
#include "Sim/MoveTypes/AAirMoveType.h"
#include "Sim/MoveTypes/HoverAirMoveType.h"
#include "Sim/MoveTypes/StaticMoveType.h"
#include "Sim/MoveTypes/StrafeAirMoveType.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Path/IPathManager.h"
#include "Sim/Misc/GlobalConstants.h"
#include "System/StringUtil.h"
#include "System/SpringHash.h"

#include <cmath>
#include <cstring>
#include <vector>

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
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

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid movement speed"
};

static const Error INVALID_MOVE_TYPE_FIELD_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Move type field is not supported by this unit"
};

static const Error BUFFER_OVERFLOW_ERROR = {
	.code = ERROR_BUFFER_OVERFLOW,
	.message = "Buffer overflow"
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
	result->data = {};

	// Keep this classification and the scalar units identical to
	// LuaSyncedRead::GetUnitMoveTypeData.
	const char* moveTypeName = "unknown";
	if (const auto* gmt = dynamic_cast<const CGroundMoveType*>(mt); gmt != nullptr) {
		moveTypeName = "ground";

		result->data.turnRate = gmt->GetTurnRate();
		result->data.accRate = gmt->GetAccRate();
		result->data.decRate = gmt->GetDecRate();
		result->data.maxReverseSpeed = gmt->GetMaxReverseSpeed() * GAME_SPEED;
		result->data.wantedSpeed = gmt->GetWantedSpeed() * GAME_SPEED;
		result->data.currentSpeed = gmt->GetCurrentSpeed() * GAME_SPEED;
		result->data.deltaSpeed = gmt->GetDeltaSpeed();
	} else if (dynamic_cast<const CHoverAirMoveType*>(mt) != nullptr) {
		moveTypeName = "gunship";
	} else if (dynamic_cast<const CStrafeAirMoveType*>(mt) != nullptr) {
		moveTypeName = "airplane";

		const CStrafeAirMoveType* samt = dynamic_cast<const CStrafeAirMoveType*>(mt);
		if (samt != nullptr) {
			result->data.maxBank = samt->maxBank;
			result->data.maxPitch = samt->maxPitch;
			result->data.maxAileron = samt->maxAileron;
			result->data.maxElevator = samt->maxElevator;
			result->data.maxRudder = samt->maxRudder;
		}
	} else if (dynamic_cast<const CStaticMoveType*>(mt) != nullptr) {
		moveTypeName = "static";
	} else if (dynamic_cast<const CScriptMoveType*>(mt) != nullptr) {
		moveTypeName = "script";
	}

	result->error = nullptr;
	result->data.name = moveTypeName;
	result->data.maxSpeed = mt->GetMaxSpeed() * GAME_SPEED;
	result->data.maxWantedSpeed = mt->GetMaxWantedSpeed() * GAME_SPEED;
	result->data.goalX = mt->goalPos.x;
	result->data.goalY = mt->goalPos.y;
	result->data.goalZ = mt->goalPos.z;
}

// Get estimated path
static void NativeGetUnitEstimatedPath(const GetUnitEstimatedPathQuery* query, GetUnitEstimatedPathResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->waypoints = nullptr;
	result->count = 0;
	result->starts = nullptr;
	result->startCount = 0;

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const auto* gmt = dynamic_cast<const CGroundMoveType*>(unit->moveType);
	if (gmt == nullptr || gmt->GetPathID() == 0 || pathManager == nullptr)
		return;

	std::vector<float3> points;
	std::vector<int> starts;
	pathManager->GetPathWayPoints(gmt->GetPathID(), points, starts);

	if (!points.empty()) {
		if (bufferPos + points.size() * sizeof(PathWaypoint) > sizeof(scratchBuffer)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		result->waypoints = reinterpret_cast<PathWaypoint*>(&scratchBuffer[bufferPos]);
		bufferPos += points.size() * sizeof(PathWaypoint);
		result->count = static_cast<uint32_t>(points.size());

		for (size_t i = 0; i < points.size(); ++i) {
			result->waypoints[i].pos.x = points[i].x;
			result->waypoints[i].pos.y = points[i].y;
			result->waypoints[i].pos.z = points[i].z;
			result->waypoints[i].eta = 0.0f;
		}
	}

	if (!starts.empty()) {
		if (bufferPos + starts.size() * sizeof(int32_t) > sizeof(scratchBuffer)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->waypoints = nullptr;
			result->count = 0;
			return;
		}

		result->starts = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
		bufferPos += starts.size() * sizeof(int32_t);
		result->startCount = static_cast<uint32_t>(starts.size());

		for (size_t i = 0; i < starts.size(); ++i)
			result->starts[i] = starts[i] + 1;
	}
}

static void NativeMoveCtrl(const MoveCtrlQuery* query, MoveCtrlResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->enable) {
		unit->EnableScriptMoveType();
	} else {
		unit->DisableScriptMoveType();
	}

	result->success = true;
}

static void NativeIsMoveCtrlEnabled(const IsMoveCtrlEnabledQuery* query, IsMoveCtrlEnabledResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->enabled = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->enabled = unit->UsingScriptMoveType();
}

static void NativeSetMoveCtrlGravity(const SetMoveCtrlGravityQuery* query, SetMoveCtrlGravityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CScriptMoveType* moveType = dynamic_cast<CScriptMoveType*>(unit->moveType);
	if (moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	moveType->gravityFactor = query->gravityFactor;
	result->success = true;
}

// Typed equivalent of Lua's MoveCtrl.SetGroundMoveTypeData(unitID,
// {maxSpeed = value}). Keep Lua-facing units at this boundary; AMoveType
// stores speeds per simulation frame internally.
static void NativeSetGroundMoveTypeMaxSpeed(
	const SetGroundMoveTypeMaxSpeedQuery* query,
	SetGroundMoveTypeMaxSpeedResult* result
)
{
	result->error = nullptr;
	result->success = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}
	if (!std::isfinite(query->maxSpeed)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}
	if (dynamic_cast<CGroundMoveType*>(unit->moveType) == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	// SetMemberValue("maxSpeed", value) performs this same conversion and
	// clamps the internal value to the valid non-zero range.
	unit->moveType->SetMaxSpeed(query->maxSpeed / GAME_SPEED);
	result->success = true;
}

static const char* MoveTypeNumericFieldName(MoveTypeNumericField field)
{
	switch (field) {
		case MOVE_TYPE_MAX_SPEED: return "maxSpeed";
		case MOVE_TYPE_MAX_WANTED_SPEED: return "maxWantedSpeed";
		case MOVE_TYPE_MANEUVER_LEASH: return "maneuverLeash";
		case MOVE_TYPE_WATERLINE: return "waterline";

		case MOVE_TYPE_GROUND_TURN_RATE: return "turnRate";
		case MOVE_TYPE_GROUND_TURN_ACCEL: return "turnAccel";
		case MOVE_TYPE_GROUND_ACC_RATE: return "accRate";
		case MOVE_TYPE_GROUND_DEC_RATE: return "decRate";
		case MOVE_TYPE_GROUND_MY_GRAVITY: return "myGravity";
		case MOVE_TYPE_GROUND_MAX_REVERSE_DIST: return "maxReverseDist";
		case MOVE_TYPE_GROUND_MIN_REVERSE_ANGLE: return "minReverseAngle";
		case MOVE_TYPE_GROUND_MAX_REVERSE_SPEED: return "maxReverseSpeed";
		case MOVE_TYPE_GROUND_SQ_SKID_SPEED_MULT: return "sqSkidSpeedMult";
		case MOVE_TYPE_GROUND_MIN_SCRIPT_CHANGE_HEADING: return "minScriptChangeHeading";

		case MOVE_TYPE_GUNSHIP_WANTED_HEIGHT: return "wantedHeight";
		case MOVE_TYPE_GUNSHIP_ACC_RATE: return "accRate";
		case MOVE_TYPE_GUNSHIP_DEC_RATE: return "decRate";
		case MOVE_TYPE_GUNSHIP_TURN_RATE: return "turnRate";
		case MOVE_TYPE_GUNSHIP_ALTITUDE_RATE: return "altitudeRate";
		case MOVE_TYPE_GUNSHIP_CURRENT_BANK: return "currentBank";
		case MOVE_TYPE_GUNSHIP_CURRENT_PITCH: return "currentPitch";
		case MOVE_TYPE_GUNSHIP_MAX_DRIFT: return "maxDrift";

		case MOVE_TYPE_AIR_WANTED_HEIGHT: return "wantedHeight";
		case MOVE_TYPE_AIR_TURN_RADIUS: return "turnRadius";
		case MOVE_TYPE_AIR_ACC_RATE: return "accRate";
		case MOVE_TYPE_AIR_DEC_RATE: return "decRate";
		case MOVE_TYPE_AIR_MAX_ACC: return "maxAcc";
		case MOVE_TYPE_AIR_MAX_DEC: return "maxDec";
		case MOVE_TYPE_AIR_MAX_BANK: return "maxBank";
		case MOVE_TYPE_AIR_MAX_PITCH: return "maxPitch";
		case MOVE_TYPE_AIR_MAX_AILERON: return "maxAileron";
		case MOVE_TYPE_AIR_MAX_ELEVATOR: return "maxElevator";
		case MOVE_TYPE_AIR_MAX_RUDDER: return "maxRudder";
		case MOVE_TYPE_AIR_ATTACK_SAFETY_DISTANCE: return "attackSafetyDistance";
		case MOVE_TYPE_AIR_MY_GRAVITY: return "myGravity";
		case MOVE_TYPE_AIR_MANEUVER_BLOCK_TIME: return "maneuverBlockTime";
	}

	return nullptr;
}

static const char* MoveTypeBooleanFieldName(MoveTypeBooleanField field)
{
	switch (field) {
		case MOVE_TYPE_USE_WANTED_SPEED_INDIVIDUAL: return "useWantedSpeed[0]";
		case MOVE_TYPE_USE_WANTED_SPEED_FORMATION: return "useWantedSpeed[1]";

		case MOVE_TYPE_GROUND_AT_GOAL: return "atGoal";
		case MOVE_TYPE_GROUND_AT_END_OF_PATH: return "atEndOfPath";
		case MOVE_TYPE_GROUND_PUSH_RESISTANT: return "pushResistant";

		case MOVE_TYPE_GUNSHIP_COLLIDE: return "collide";
		case MOVE_TYPE_GUNSHIP_DONT_LAND: return "dontLand";
		case MOVE_TYPE_GUNSHIP_AIR_STRAFE: return "airStrafe";
		case MOVE_TYPE_GUNSHIP_USE_SMOOTH_MESH: return "useSmoothMesh";
		case MOVE_TYPE_GUNSHIP_BANKING_ALLOWED: return "bankingAllowed";

		case MOVE_TYPE_AIR_COLLIDE: return "collide";
		case MOVE_TYPE_AIR_USE_SMOOTH_MESH: return "useSmoothMesh";
		case MOVE_TYPE_AIR_LOOPBACK_ATTACK: return "loopbackAttack";
	}

	return nullptr;
}

static void NativeSetMoveTypeNumeric(
	const SetMoveTypeNumericQuery* query,
	SetMoveTypeNumericResult* result
)
{
	result->error = nullptr;
	result->success = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}
	if (!std::isfinite(query->value)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const char* fieldName = MoveTypeNumericFieldName(query->field);
	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}
	float value = query->value;
	if (fieldName == nullptr || !unit->moveType->SetMemberValue(
		spring::LiteHash(fieldName, std::strlen(fieldName), 0),
		&value)) {
		result->error = &INVALID_MOVE_TYPE_FIELD_ERROR;
		return;
	}

	result->success = true;
}

static void NativeSetMoveTypeBoolean(
	const SetMoveTypeBooleanQuery* query,
	SetMoveTypeBooleanResult* result
)
{
	result->error = nullptr;
	result->success = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const char* fieldName = MoveTypeBooleanFieldName(query->field);
	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}
	bool value = query->value;
	if (fieldName == nullptr || !unit->moveType->SetMemberValue(
		spring::LiteHash(fieldName, std::strlen(fieldName), 0),
		&value)) {
		result->error = &INVALID_MOVE_TYPE_FIELD_ERROR;
		return;
	}

	result->success = true;
}

} // namespace

const MoveCtrlApi MOVE_CTRL_API = {
	.GetUnitMoveTypeData = NativeGetUnitMoveTypeData,
	.GetUnitEstimatedPath = NativeGetUnitEstimatedPath,
	.MoveCtrl = NativeMoveCtrl,
	.IsMoveCtrlEnabled = NativeIsMoveCtrlEnabled,
	.SetMoveCtrlGravity = NativeSetMoveCtrlGravity,
	.SetGroundMoveTypeMaxSpeed = NativeSetGroundMoveTypeMaxSpeed,
	.SetMoveTypeNumeric = NativeSetMoveTypeNumeric,
	.SetMoveTypeBoolean = NativeSetMoveTypeBoolean,
};

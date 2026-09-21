#include "MoveCtrl.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/MoveTypes/MoveType.h"
#include "Sim/MoveTypes/MoveDefHandler.h"
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
#include <algorithm>
#include <vector>

namespace {

static thread_local std::vector<PathWaypoint> estimatedWaypoints;
static thread_local std::vector<int32_t> estimatedStarts;

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

static const Error INVALID_PROGRESS_STATE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid move control progress state"
};

template<typename Mutator>
static void ApplyScriptMoveType(int unitID, MoveCtrlResult* result, Mutator&& mutator);

// Get move type data
static void NativeGetUnitMoveTypeData(const GetUnitMoveTypeDataQuery* query, GetUnitMoveTypeDataResult* result)
{

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

	estimatedWaypoints.clear();
	estimatedWaypoints.reserve(points.size());
	for (const float3& point : points) {
		estimatedWaypoints.push_back({
			.pos = {point.x, point.y, point.z},
			.eta = 0.0f,
		});
	}
	result->waypoints = estimatedWaypoints.empty() ? nullptr : estimatedWaypoints.data();
	result->count = estimatedWaypoints.size();

	estimatedStarts.clear();
	estimatedStarts.reserve(starts.size());
	for (int start : starts)
		estimatedStarts.push_back(start + 1);
	result->starts = estimatedStarts.empty() ? nullptr : estimatedStarts.data();
	result->startCount = estimatedStarts.size();
}

static void NativeMoveCtrl(const MoveCtrlQuery* query, MoveCtrlResult* result)
{
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

static void NativeSetTag(const SetMoveCtrlTagQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.tag = query->tag;
	});
}

static void NativeGetTag(const GetMoveCtrlTagQuery* query, GetMoveCtrlTagResult* result)
{
	result->error = nullptr;
	result->tag = 0;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const auto* moveType = dynamic_cast<const CScriptMoveType*>(unit->moveType);
	if (moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->tag = moveType->tag;
}

static void NativeSetProgressState(const SetMoveCtrlProgressStateQuery* query, MoveCtrlResult* result)
{
	if (query->state < MOVE_CTRL_PROGRESS_DONE || query->state > MOVE_CTRL_PROGRESS_FAILED) {
		result->error = &INVALID_PROGRESS_STATE_ERROR;
		result->success = false;
		return;
	}

	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.progressState = static_cast<AMoveType::ProgressState>(query->state);
	});
}

static void NativeIsMoveCtrlEnabled(const IsMoveCtrlEnabledQuery* query, IsMoveCtrlEnabledResult* result)
{
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

static void NativeSetNoBlocking(const SetNoBlockingQuery* query, SetNoBlockingResult* result)
{
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

	moveType->SetNoBlocking(query->noBlocking);
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

template<typename Mutator>
static void ApplyScriptMoveType(int unitID, MoveCtrlResult* result, Mutator&& mutator)
{
	result->error = nullptr;
	result->success = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CScriptMoveType* moveType = dynamic_cast<CScriptMoveType*>(unit->moveType);
	if (moveType == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	mutator(*moveType);
	result->success = true;
}

static void NativeSetExtrapolate(const MoveCtrlBoolQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.extrapolate = query->value;
	});
}

static void NativeSetPhysics(const MoveCtrlPhysicsQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.SetPhysics(
			float3(query->position.x, query->position.y, query->position.z),
			float3(query->velocity.x, query->velocity.y, query->velocity.z),
			float3(query->rotation.x, query->rotation.y, query->rotation.z));
	});
}

static void NativeSetPosition(const MoveCtrlFloat3Query* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.SetPosition(float3(query->value.x, query->value.y, query->value.z));
	});
}

static void NativeSetVelocity(const MoveCtrlFloat3Query* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.SetVelocity(float3(query->value.x, query->value.y, query->value.z));
	});
}

static void NativeSetRelativeVelocity(const MoveCtrlFloat3Query* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.SetRelativeVelocity(float3(query->value.x, query->value.y, query->value.z));
	});
}

static void NativeSetRotation(const MoveCtrlFloat3Query* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.SetRotation(float3(query->value.x, query->value.y, query->value.z));
	});
}

static void NativeSetRotationVelocity(const MoveCtrlFloat3Query* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.SetRotationVelocity(float3(query->value.x, query->value.y, query->value.z));
	});
}

static void NativeSetHeading(const MoveCtrlHeadingQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.SetHeading(static_cast<short>(query->heading));
	});
}

static void NativeSetTrackSlope(const MoveCtrlBoolQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.trackSlope = query->value;
	});
}

static void NativeSetTrackGround(const MoveCtrlBoolQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.trackGround = query->value;
	});
}

static void NativeSetTrackLimits(const MoveCtrlBoolQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.trackLimits = query->value;
	});
}

static void NativeSetGroundOffset(const MoveCtrlFloatQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.groundOffset = query->value;
	});
}

static void NativeSetGravity(const MoveCtrlFloatQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.gravityFactor = query->value;
	});
}

static void NativeSetDrag(const MoveCtrlFloatQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.drag = query->value;
	});
}

static void NativeSetWindFactor(const MoveCtrlFloatQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.windFactor = query->value;
	});
}

static void NativeSetLimits(const MoveCtrlLimitsQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.mins = float3(query->mins.x, query->mins.y, query->mins.z);
		moveType.maxs = float3(query->maxs.x, query->maxs.y, query->maxs.z);
	});
}

static void NativeSetCollideStop(const MoveCtrlBoolQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.groundStop = query->value;
	});
}

static void NativeSetLimitsStop(const MoveCtrlBoolQuery* query, MoveCtrlResult* result)
{
	ApplyScriptMoveType(query->unitID, result, [query](CScriptMoveType& moveType) {
		moveType.limitsStop = query->value;
	});
}

static void NativeSetMoveDef(const MoveCtrlMoveDefQuery* query, MoveCtrlResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (gs == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->moveDef == nullptr || unit->moveType == nullptr)
		return;

	MoveDef* moveDef = nullptr;
	if (query->hasMoveDefName) {
		if (query->moveDefName != nullptr)
			moveDef = moveDefHandler.GetMoveDefByName(query->moveDefName);
	} else {
		const unsigned int moveDefCount = moveDefHandler.GetNumMoveDefs();
		if (moveDefCount != 0) {
			const int pathType = std::clamp(query->moveDefID, 0, static_cast<int>(moveDefCount) - 1);
			moveDef = moveDefHandler.GetMoveDefByPathType(pathType);
		}
	}

	if (moveDef == nullptr)
		return;

	if (unit->UsingScriptMoveType())
		unit->prevMoveType->StopMoving();
	else
		unit->moveType->StopMoving();

	unit->moveDef = moveDef;
	result->success = true;
}

} // namespace

const MoveCtrlApi MOVE_CTRL_API = {
	.GetUnitMoveTypeData = NativeGetUnitMoveTypeData,
	.GetUnitEstimatedPath = NativeGetUnitEstimatedPath,
	.MoveCtrl = NativeMoveCtrl,
	.SetTag = NativeSetTag,
	.GetTag = NativeGetTag,
	.SetProgressState = NativeSetProgressState,
	.SetMoveDef = NativeSetMoveDef,
	.IsMoveCtrlEnabled = NativeIsMoveCtrlEnabled,
	.SetMoveCtrlGravity = NativeSetMoveCtrlGravity,
	.SetGroundMoveTypeMaxSpeed = NativeSetGroundMoveTypeMaxSpeed,
	.SetMoveTypeNumeric = NativeSetMoveTypeNumeric,
	.SetMoveTypeBoolean = NativeSetMoveTypeBoolean,
	.SetNoBlocking = NativeSetNoBlocking,
	.SetExtrapolate = NativeSetExtrapolate,
	.SetPhysics = NativeSetPhysics,
	.SetPosition = NativeSetPosition,
	.SetVelocity = NativeSetVelocity,
	.SetRelativeVelocity = NativeSetRelativeVelocity,
	.SetRotation = NativeSetRotation,
	.SetRotationVelocity = NativeSetRotationVelocity,
	.SetHeading = NativeSetHeading,
	.SetTrackSlope = NativeSetTrackSlope,
	.SetTrackGround = NativeSetTrackGround,
	.SetTrackLimits = NativeSetTrackLimits,
	.SetGroundOffset = NativeSetGroundOffset,
	.SetGravity = NativeSetGravity,
	.SetDrag = NativeSetDrag,
	.SetWindFactor = NativeSetWindFactor,
	.SetLimits = NativeSetLimits,
	.SetCollideStop = NativeSetCollideStop,
	.SetLimitsStop = NativeSetLimitsStop,
};

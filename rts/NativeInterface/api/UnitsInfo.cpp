#include "UnitsInfo.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Features/Feature.h"
#include "Sim/Units/UnitTypes/Builder.h"
#include "Sim/Units/UnitTypes/Factory.h"
#include "Sim/Units/CommandAI/BuilderCAI.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/MobileCAI.h"
#include "Sim/MoveTypes/MoveDefHandler.h"
#include "Sim/MoveTypes/AAirMoveType.h"
#include "Sim/MoveTypes/HoverAirMoveType.h"
#include "Sim/MoveTypes/StrafeAirMoveType.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Weapons/PlasmaRepulser.h"
#include "System/float3.h"
#include "System/SpringMath.h"
#include "Rendering/Units/UnitDrawer.h"
#include "Rendering/Models/3DModel.hpp"
#include "Rendering/Models/3DModelPiece.hpp"
#include "Sim/Units/UnitToolTipMap.hpp"
#include <algorithm>
#include <cstddef>
#include <cstring>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };
static const Error INVALID_ALLY_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ally team ID" };

static bool IsReady() {
	return (gs != nullptr);
}

static char* AllocScratch(size_t size, size_t alignment = alignof(std::max_align_t)) {
	const size_t alignedPos = (bufferPos + alignment - 1) & ~(alignment - 1);
	if (alignedPos + size > sizeof(scratchBuffer)) {
		return nullptr;
	}

	char* ptr = &scratchBuffer[alignedPos];
	bufferPos = alignedPos + size;
	return ptr;
}

static const char* CopyString(const std::string& str) {
	const size_t len = str.size() + 1;
	char* ptr = AllocScratch(len, alignof(char));
	if (ptr == nullptr) {
		return "";
	}

	memcpy(ptr, str.c_str(), len);
	return ptr;
}

static void NativeGetUnitTooltip(const GetUnitTooltipQuery* query, GetUnitTooltipResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->tooltip = "";

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->tooltip = unitToolTipMap.Get(unit->id).c_str();
}

static void NativeGetUnitDefID(const GetUnitDefIDQuery* query, GetUnitDefIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->unitDefID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->unitDefID = unit->unitDef->id;
}

static void NativeGetUnitMoveDefID(const GetUnitMoveDefIDQuery* query, GetUnitMoveDefIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->moveDefID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->moveDef != nullptr)
		result->moveDefID = static_cast<int32_t>(unit->moveDef->pathType);
}

static void NativeGetUnitTeam(const GetUnitTeamQuery* query, GetUnitTeamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->teamID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->teamID = unit->team;
}

static void NativeGetUnitAllyTeam(const GetUnitAllyTeamQuery* query, GetUnitAllyTeamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->allyTeamID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->allyTeamID = unit->allyteam;
}

static void NativeGetUnitNeutral(const GetUnitNeutralQuery* query, GetUnitNeutralResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->neutral = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->neutral = unit->IsNeutral();
}

static void NativeGetUnitCrashing(const GetUnitCrashingQuery* query, GetUnitCrashingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->isAircraft = false;
	result->crashing = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const AAirMoveType* amt = dynamic_cast<AAirMoveType*>(unit->moveType);
	if (amt == nullptr) {
		// Not an aircraft: crashing does not apply (isAircraft stays false).
		return;
	}

	result->isAircraft = true;
	result->crashing = (amt->aircraftState == AAirMoveType::AIRCRAFT_CRASHING);
}

static void NativeGetUnitHealth(const GetUnitHealthQuery* query, GetUnitHealthResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->health.health = unit->health;
	result->health.maxHealth = unit->maxHealth;
	result->health.paralyzeDamage = unit->paralyzeDamage;
	result->health.captureProgress = unit->captureProgress;
	result->health.buildProgress = unit->buildProgress;
}

static void NativeGetUnitIsDead(const GetUnitIsDeadQuery* query, GetUnitIsDeadResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->isDead = true;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		return; // Dead/doesn't exist
	}

	result->isDead = unit->isDead;
}

static void NativeGetUnitIsStunned(const GetUnitIsStunnedQuery* query, GetUnitIsStunnedResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->isStunned = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->isStunned = unit->IsStunned();
}

static void NativeGetUnitIsBeingBuilt(const GetUnitIsBeingBuiltQuery* query, GetUnitIsBeingBuiltResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->isBeingBuilt = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->isBeingBuilt = unit->beingBuilt;
}

static void NativeGetUnitCosts(const GetUnitCostsQuery* query, GetUnitCostsResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->costs.metalCost = unit->cost.metal;
	result->costs.energyCost = unit->cost.energy;
	result->costs.buildTime = unit->buildTime;
}

static void NativeGetUnitCostTable(const GetUnitCostTableQuery* query, GetUnitCostTableResult* result) {
	// Same as GetUnitCosts
	NativeGetUnitCosts(reinterpret_cast<const GetUnitCostsQuery*>(query), reinterpret_cast<GetUnitCostsResult*>(result));
}

static void NativeGetUnitResources(const GetUnitResourcesQuery* query, GetUnitResourcesResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const UnitDef* ud = unit->unitDef;
	result->resources.metalMake = ud->resourceMake.metal;
	result->resources.metalUse = ud->upkeep.metal;
	result->resources.energyMake = ud->resourceMake.energy;
	result->resources.energyUse = ud->upkeep.energy;
	result->resources.metalIncome = ud->resourceMake.metal - ud->upkeep.metal;
	result->resources.energyIncome = ud->resourceMake.energy - ud->upkeep.energy;
}

static void NativeGetUnitStorage(const GetUnitStorageQuery* query, GetUnitStorageResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->storage.metalStorage = unit->storage.metal;
	result->storage.energyStorage = unit->storage.energy;
}

static void NativeGetUnitMetalExtraction(const GetUnitMetalExtractionQuery* query, GetUnitMetalExtractionResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->metalExtraction = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->metalExtraction = unit->metalExtract;
}

static void NativeGetUnitExperience(const GetUnitExperienceQuery* query, GetUnitExperienceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->experience = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->experience = unit->experience;
}

static void NativeGetUnitStates(const GetUnitStatesQuery* query, GetUnitStatesResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Lua can choose between a table and a variable-length positional return,
	// and can independently omit the binary or air-move-type fields.  The
	// native ABI returns one typed UnitStates value instead, so it always
	// materializes the complete state; accepting the same selectors keeps the
	// call contract compatible while giving Rust callers a stable result shape.
	(void)query->retTable;
	(void)query->hasRetTable;
	(void)query->binState;
	(void)query->hasBinState;
	(void)query->amtState;
	(void)query->hasAmtState;

	result->states.fireState = unit->fireState;
	result->states.moveState = unit->moveState;
	result->states.autoRepairLevel = -1.0f;
	result->states.repeat = unit->commandAI->repeatOrders;
	result->states.cloak = unit->wantCloak;
	result->states.active = unit->activated;
	result->states.trajectory = unit->useHighTrajectory;
	result->states.autoLand = false;
	result->states.loopbackAttack = false;

	if (const CMobileCAI* mCAI = dynamic_cast<const CMobileCAI*>(unit->commandAI)) {
		result->states.autoRepairLevel = mCAI->repairBelowHealth;
	}
	if (const CHoverAirMoveType* hAMT = dynamic_cast<const CHoverAirMoveType*>(unit->moveType)) {
		result->states.autoLand = hAMT->autoLand;
		result->states.loopbackAttack = false;
	} else if (const CStrafeAirMoveType* sAMT = dynamic_cast<const CStrafeAirMoveType*>(unit->moveType)) {
		result->states.autoLand = sAMT->autoLand;
		result->states.loopbackAttack = sAMT->loopbackAttack;
	}
}

static void NativeGetUnitArmored(const GetUnitArmoredQuery* query, GetUnitArmoredResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->armoredState.armored = unit->armoredState;
	result->armoredState.armorMultiple = unit->armoredMultiple;
}

static void NativeGetUnitIsActive(const GetUnitIsActiveQuery* query, GetUnitIsActiveResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->isActive = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->isActive = unit->activated;
}

static void NativeGetUnitIsCloaked(const GetUnitIsCloakedQuery* query, GetUnitIsCloakedResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->isCloaked = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->isCloaked = unit->IsCloaked();
}

static void NativeGetUnitSeismicSignature(const GetUnitSeismicSignatureQuery* query, GetUnitSeismicSignatureResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->seismicSignature = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->seismicSignature = unit->seismicSignature;
}

static void NativeGetUnitSensorRadius(const GetUnitSensorRadiusQuery* query, GetUnitSensorRadiusResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->radius = {};

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const char* type = (query->type != nullptr) ? query->type : "";
	if (strcmp(type, "los") == 0) result->radius.los = unit->losRadius;
	else if (strcmp(type, "airLos") == 0) result->radius.airLos = unit->airLosRadius;
	else if (strcmp(type, "radar") == 0) result->radius.radar = unit->radarRadius;
	else if (strcmp(type, "sonar") == 0) result->radius.sonar = unit->sonarRadius;
	else if (strcmp(type, "seismic") == 0) result->radius.seismic = unit->seismicRadius;
	else if (strcmp(type, "radarJammer") == 0) result->radius.radarJammer = unit->jammerRadius;
	else if (strcmp(type, "sonarJammer") == 0) result->radius.sonarJammer = unit->sonarJamRadius;
}

static void NativeGetUnitPosErrorParams(const GetUnitPosErrorParamsQuery* query, GetUnitPosErrorParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int allyTeam = std::clamp(query->allyTeamID, 0, teamHandler.ActiveAllyTeams());
	result->params.posErrorVector.x = unit->posErrorVector.x;
	result->params.posErrorVector.y = unit->posErrorVector.y;
	result->params.posErrorVector.z = unit->posErrorVector.z;
	result->params.posErrorDelta.x = unit->posErrorDelta.x;
	result->params.posErrorDelta.y = unit->posErrorDelta.y;
	result->params.posErrorDelta.z = unit->posErrorDelta.z;
	result->params.nextPosErrorUpdate = unit->nextPosErrorUpdate;
	result->params.posErrorBit = unit->GetPosErrorBit(allyTeam);
}

static void NativeGetUnitHeight(const GetUnitHeightQuery* query, GetUnitHeightResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->height = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->height = unit->height;
}

static void NativeGetUnitRadius(const GetUnitRadiusQuery* query, GetUnitRadiusResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->radius = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->radius = unit->radius;
}

static void NativeGetUnitBuildeeRadius(const GetUnitBuildeeRadiusQuery* query, GetUnitBuildeeRadiusResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->radius = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->radius = unit->buildeeRadius;
}

static void NativeGetUnitMass(const GetUnitMassQuery* query, GetUnitMassResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->mass = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->mass = unit->mass;
}

static void NativeGetUnitPosition(const GetUnitPositionQuery* query, GetUnitPositionResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 pos = query->midPos ? float3(unit->midPos) : (query->aimPos ? float3(unit->aimPos) : unit->pos);
	result->position.x = pos.x;
	result->position.y = pos.y;
	result->position.z = pos.z;
}

static void NativeGetUnitBasePosition(const GetUnitBasePositionQuery* query, GetUnitBasePositionResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Base position is midPos for units
	result->position.x = unit->midPos.x;
	result->position.y = unit->midPos.y;
	result->position.z = unit->midPos.z;
}

static void NativeGetUnitVectors(const GetUnitVectorsQuery* query, GetUnitVectorsResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->vectors.frontDir.x = unit->frontdir.x;
	result->vectors.frontDir.y = unit->frontdir.y;
	result->vectors.frontDir.z = unit->frontdir.z;

	result->vectors.upDir.x = unit->updir.x;
	result->vectors.upDir.y = unit->updir.y;
	result->vectors.upDir.z = unit->updir.z;

	result->vectors.rightDir.x = unit->rightdir.x;
	result->vectors.rightDir.y = unit->rightdir.y;
	result->vectors.rightDir.z = unit->rightdir.z;
}

static void NativeGetUnitRotation(const GetUnitRotationQuery* query, GetUnitRotationResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CMatrix44f& matrix = unit->GetTransformMatrix(true);
	const float3 angles = matrix.GetEulerAnglesLftHand();
	result->rotation.pitch = angles[CMatrix44f::ANGLE_P];
	result->rotation.yaw = angles[CMatrix44f::ANGLE_Y];
	result->rotation.roll = angles[CMatrix44f::ANGLE_R];
}

static void NativeGetUnitDirection(const GetUnitDirectionQuery* query, GetUnitDirectionResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->direction.x = unit->frontdir.x;
	result->direction.y = unit->frontdir.y;
	result->direction.z = unit->frontdir.z;
}

static void NativeGetUnitHeading(const GetUnitHeadingQuery* query, GetUnitHeadingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->heading = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float heading = unit->heading;
	result->heading = query->convertToRadians ? ClampRad(math::PI / 32768.0f * heading) : heading;
}

static void NativeGetUnitVelocity(const GetUnitVelocityQuery* query, GetUnitVelocityResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->velocity.x = unit->speed.x;
	result->velocity.y = unit->speed.y;
	result->velocity.z = unit->speed.z;
}

static void NativeGetUnitBuildFacing(const GetUnitBuildFacingQuery* query, GetUnitBuildFacingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->facing = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->facing = unit->buildFacing;
}

static void NativeGetUnitIsBuilding(const GetUnitIsBuildingQuery* query, GetUnitIsBuildingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->buildeeID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CBuilder* builder = dynamic_cast<const CBuilder*>(unit);
	if (builder != nullptr && builder->curBuild != nullptr) {
		result->buildeeID = builder->curBuild->id;
		return;
	}

	const CFactory* factory = dynamic_cast<const CFactory*>(unit);
	if (factory != nullptr && factory->curBuild != nullptr) {
		result->buildeeID = factory->curBuild->id;
	}
}

static void NativeGetUnitWorkerTask(const GetUnitWorkerTaskQuery* query, GetUnitWorkerTaskResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->task.cmdID = 0;
	result->task.targetID = 0;
	result->task.hasTask = false;
	result->task.hasTarget = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CBuilder* builder = dynamic_cast<const CBuilder*>(unit);
	if (builder != nullptr) {
		if (builder->curBuild != nullptr) {
			result->task.cmdID = builder->curBuild->beingBuilt ? -builder->curBuild->unitDef->id : CMD_REPAIR;
			result->task.targetID = builder->curBuild->id;
			result->task.hasTask = true;
			result->task.hasTarget = true;
		} else if (builder->curCapture != nullptr) {
			result->task.cmdID = CMD_CAPTURE;
			result->task.targetID = builder->curCapture->id;
			result->task.hasTask = true;
			result->task.hasTarget = true;
		} else if (builder->curResurrect != nullptr) {
			result->task.cmdID = CMD_RESURRECT;
			result->task.targetID = builder->curResurrect->id + unitHandler.MaxUnits();
			result->task.hasTask = true;
			result->task.hasTarget = true;
		} else if (builder->curReclaim != nullptr) {
			result->task.cmdID = CMD_RECLAIM;
			if (builder->reclaimingUnit) {
				const CUnit* reclaimee = dynamic_cast<const CUnit*>(builder->curReclaim);
				result->task.targetID = (reclaimee != nullptr) ? reclaimee->id : 0;
			} else {
				const CFeature* reclaimee = dynamic_cast<const CFeature*>(builder->curReclaim);
				result->task.targetID = (reclaimee != nullptr) ? reclaimee->id + unitHandler.MaxUnits() : 0;
			}
			result->task.hasTask = true;
			result->task.hasTarget = true;
		} else if (builder->helpTerraform || builder->terraforming) {
			result->task.cmdID = CMD_RESTORE;
			result->task.hasTask = true;
			result->task.hasTarget = false;
		}
		return;
	}

	const CFactory* factory = dynamic_cast<const CFactory*>(unit);
	if (factory != nullptr && factory->curBuild != nullptr) {
		result->task.cmdID = factory->curBuild->beingBuilt ? -factory->curBuild->unitDef->id : CMD_REPAIR;
		result->task.targetID = factory->curBuild->id;
		result->task.hasTask = true;
		result->task.hasTarget = true;
	}
}

static void NativeGetUnitEffectiveBuildRange(const GetUnitEffectiveBuildRangeQuery* query, GetUnitEffectiveBuildRangeResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->range = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CBuilderCAI* builderCAI = dynamic_cast<const CBuilderCAI*>(unit->commandAI);
	if (builderCAI == nullptr) {
		result->range = unit->unitDef->buildDistance;
		return;
	}

	if (query->buildeeDefID <= 0) {
		result->range = builderCAI->GetBuildRange(0.0f);
		return;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query->buildeeDefID);
	if (unitDef == nullptr) {
		result->range = builderCAI->GetBuildRange(0.0f);
		return;
	}

	const S3DModel* model = unitDef->LoadModel();
	const float radius = (model != nullptr) ? std::max(0.0f, model->radius) : 0.0f;
	result->range = builderCAI->GetBuildRange(radius);
}

static void NativeGetUnitCurrentBuildPower(const GetUnitCurrentBuildPowerQuery* query, GetUnitCurrentBuildPowerResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->buildPower = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const NanoPieceCache* pieceCache = nullptr;

	const CBuilder* builder = dynamic_cast<const CBuilder*>(unit);
	if (builder != nullptr)
		pieceCache = &builder->GetNanoPieceCache();

	const CFactory* factory = dynamic_cast<const CFactory*>(unit);
	if (factory != nullptr)
		pieceCache = &factory->GetNanoPieceCache();

	if (pieceCache != nullptr)
		result->buildPower = pieceCache->GetBuildPower();
}

static void NativeGetUnitBuildParams(const GetUnitBuildParamsQuery* query, GetUnitBuildParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = {};
	result->hasValue = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CBuilder* builder = dynamic_cast<const CBuilder*>(unit);
	if (builder == nullptr || query->paramName == nullptr) {
		return;
	}

	switch (hashString(query->paramName)) {
		case hashString("buildRange"):
		case hashString("buildDistance"):
			result->value.number = builder->buildDistance;
			result->value.useBoolean = false;
			result->hasValue = true;
			break;
		case hashString("buildRange3D"):
			result->value.boolean = builder->range3D;
			result->value.useBoolean = true;
			result->hasValue = true;
			break;
		default:
			break;
	}
}

static void NativeGetUnitInBuildStance(const GetUnitInBuildStanceQuery* query, GetUnitInBuildStanceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->inBuildStance = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->inBuildStance = unit->inBuildStance;
}

static void NativeGetUnitNanoPieces(const GetUnitNanoPiecesQuery* query, GetUnitNanoPiecesResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->pieces = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const NanoPieceCache* pieceCache = nullptr;
	const std::vector<int>* nanoPieces = nullptr;

	// Try to get nano pieces from builder
	const CBuilder* builder = dynamic_cast<const CBuilder*>(unit);
	if (builder != nullptr) {
		pieceCache = &builder->GetNanoPieceCache();
		nanoPieces = &pieceCache->GetNanoPieces();
	}

	// Try to get nano pieces from factory
	const CFactory* factory = dynamic_cast<const CFactory*>(unit);
	if (factory != nullptr) {
		pieceCache = &factory->GetNanoPieceCache();
		nanoPieces = &pieceCache->GetNanoPieces();
	}

	// Return empty if unit is not a builder/factory or has no nano pieces
	if (nanoPieces == nullptr || nanoPieces->empty()) {
		return;
	}

	// Copy nano pieces to scratch buffer
	const size_t count = nanoPieces->size();
	const size_t bytesNeeded = count * sizeof(int32_t);

	if (bufferPos + bytesNeeded > sizeof(scratchBuffer)) {
		static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	int32_t* piecesBuf = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	for (size_t i = 0; i < count; i++) {
		piecesBuf[i] = (*nanoPieces)[i] + 1;  // Convert from 0-indexed C++ to 1-indexed Lua
	}
	bufferPos += bytesNeeded;

	result->pieces = piecesBuf;
	result->count = static_cast<uint32_t>(count);
}

static void NativeGetUnitTransporter(const GetUnitTransporterQuery* query, GetUnitTransporterResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->transporterID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->GetTransporter() != nullptr) {
		result->transporterID = unit->GetTransporter()->id;
	}
}

static void NativeGetUnitIsTransporting(const GetUnitIsTransportingQuery* query, GetUnitIsTransportingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->isTransporting = false;
	result->unitIDs = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (!unit->unitDef->IsTransportUnit()) {
		return;
	}

	result->isTransporting = true;
	result->count = unit->transportedUnits.size();
	if (result->count == 0) {
		return;
	}

	const size_t bytesNeeded = result->count * sizeof(int32_t);
	result->unitIDs = reinterpret_cast<int32_t*>(AllocScratch(bytesNeeded, alignof(int32_t)));
	if (result->unitIDs == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->count = 0;
		return;
	}

	for (uint32_t i = 0; i < result->count; ++i) {
		result->unitIDs[i] = unit->transportedUnits[i].unit->id;
	}
}

static void NativeGetUnitStockpile(const GetUnitStockpileQuery* query, GetUnitStockpileResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->hasStockpile = false;
	result->stockpile.stockpile = 0;
	result->stockpile.stockpileQueueSize = 0;
	result->stockpile.buildPercent = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CWeapon* weapon = unit->stockpileWeapon;
	if (weapon == nullptr) {
		return;
	}

	result->hasStockpile = true;
	result->stockpile.stockpile = weapon->numStockpiled;
	result->stockpile.stockpileQueueSize = weapon->numStockpileQued;
	result->stockpile.buildPercent = weapon->buildPercent;
}

static void NativeGetUnitSelfDTime(const GetUnitSelfDTimeQuery* query, GetUnitSelfDTimeResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->selfDTime = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->selfDTime = unit->selfDCountdown;
}

static void NativeGetUnitShieldState(const GetUnitShieldStateQuery* query, GetUnitShieldStateResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->hasShield = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Check if unit has a shield weapon
	for (CWeapon* w : unit->weapons) {
		CPlasmaRepulser* shield = dynamic_cast<CPlasmaRepulser*>(w);
		if (shield != nullptr) {
			result->hasShield = true;
			result->shield.shieldEnabled = shield->IsEnabled();
			result->shield.shieldPower = shield->GetCurPower();
			result->shield.shieldAlpha = 1.0f;
			break;
		}
	}
}

static void NativeGetUnitFlanking(const GetUnitFlankingQuery* query, GetUnitFlankingResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->flanking.flankingMode = unit->flankingBonusMode;
	result->flanking.moveFactor = unit->flankingBonusMobilityAdd;
	result->flanking.minDamage = unit->flankingBonusAvgDamage - unit->flankingBonusDifDamage;
	result->flanking.maxDamage = unit->flankingBonusAvgDamage + unit->flankingBonusDifDamage;
	result->flanking.direction.x = unit->flankingBonusDir.x;
	result->flanking.direction.y = unit->flankingBonusDir.y;
	result->flanking.direction.z = unit->flankingBonusDir.z;
	result->flanking.mobility = unit->flankingBonusMobility;
}

static void NativeGetUnitTravel(const GetUnitTravelQuery* query, GetUnitTravelResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->travel.travelPeriod = 0.0f; // Not directly available
	result->travel.travelTime = 0.0f;   // Not directly available
}

static void NativeGetUnitFuel(const GetUnitFuelQuery* query, GetUnitFuelResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->unitDef == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Fuel is not directly tracked in Spring - use 0 as it's not available
	result->fuel.fuel = 0.0f;
	result->fuel.maxFuel = 0.0f;
}

static void NativeGetUnitLastAttacker(const GetUnitLastAttackerQuery* query, GetUnitLastAttackerResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->hasAttacker = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->lastAttacker != nullptr) {
		result->hasAttacker = true;
		result->attacker.attackerID = unit->lastAttacker->id;
		result->attacker.attackerDefID = unit->lastAttacker->unitDef ? unit->lastAttacker->unitDef->id : -1;
		result->attacker.attackerTeam = unit->lastAttacker->team;
	}
}

static void NativeGetUnitLastAttackedPiece(const GetUnitLastAttackedPieceQuery* query, GetUnitLastAttackedPieceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->piece.name = "";
	result->piece.pieceNum = -1;
	result->piece.frame = -1;
	result->piece.wasHit = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModelPiece* piece = unit->hitModelPieces[true];
	if (piece == nullptr || piece->original == nullptr) {
		return;
	}

	result->piece.name = CopyString(piece->original->name);
	result->piece.pieceNum = piece->GetLModelPieceIndex() + 1;
	result->piece.frame = unit->pieceHitFrames[true];
	result->piece.wasHit = true;
}

static void NativeGetUnitLosState(const GetUnitLosStateQuery* query, GetUnitLosStateResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->losState.rawMask = 0;
	result->losState.los = false;
	result->losState.radar = false;
	result->losState.typed = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int allyTeam = query->allyTeamID;
	if (!teamHandler.IsValidAllyTeam(allyTeam) || allyTeam >= static_cast<int>(unit->losStatus.size())) {
		result->error = &INVALID_ALLY_TEAM_ERROR;
		return;
	}

	const uint8_t losStatus = unit->losStatus[allyTeam] & LOS_ALL_BITS;
	constexpr uint8_t prevMask = LOS_PREVLOS | LOS_CONTRADAR;

	result->losState.rawMask = losStatus;
	result->losState.los = (losStatus & LOS_INLOS) != 0;
	result->losState.radar = (losStatus & LOS_INRADAR) != 0;
	result->losState.typed = result->losState.los || ((losStatus & prevMask) == prevMask);
}

static void NativeGetUnitCollisionVolumeData(const GetUnitCollisionVolumeDataQuery* query, GetUnitCollisionVolumeDataResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CollisionVolume& cv = unit->collisionVolume;
	result->volume.scaleX = cv.GetScales().x;
	result->volume.scaleY = cv.GetScales().y;
	result->volume.scaleZ = cv.GetScales().z;
	result->volume.offsetX = cv.GetOffsets().x;
	result->volume.offsetY = cv.GetOffsets().y;
	result->volume.offsetZ = cv.GetOffsets().z;
	result->volume.volumeType = cv.GetVolumeType();
	result->volume.testType = cv.UseContHitTest() ? 1 : 0;  // 1=continuous, 0=discrete
	result->volume.primaryAxis = cv.GetPrimaryAxis();
	result->volume.disabled = cv.IgnoreHits();
}

static void NativeGetUnitPieceCollisionVolumeData(const GetUnitPieceCollisionVolumeDataQuery* query, GetUnitPieceCollisionVolumeDataResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Piece collision volumes would require script/model integration
	// Return unit's main volume as fallback
	const CollisionVolume& cv = unit->collisionVolume;
	result->volume.scaleX = cv.GetScales().x;
	result->volume.scaleY = cv.GetScales().y;
	result->volume.scaleZ = cv.GetScales().z;
	result->volume.offsetX = cv.GetOffsets().x;
	result->volume.offsetY = cv.GetOffsets().y;
	result->volume.offsetZ = cv.GetOffsets().z;
	result->volume.volumeType = cv.GetVolumeType();
	result->volume.testType = cv.UseContHitTest() ? 1 : 0;  // 1=continuous, 0=discrete
	result->volume.primaryAxis = cv.GetPrimaryAxis();
	result->volume.disabled = cv.IgnoreHits();
}

static void NativeGetUnitBlocking(const GetUnitBlockingQuery* query, GetUnitBlockingResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->blockingState.isBlocking = unit->HasPhysicalStateBit(CSolidObject::PSTATE_BIT_BLOCKING);
	result->blockingState.isSolidObjectCollidable = unit->HasCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	result->blockingState.isProjectileCollidable = unit->HasCollidableStateBit(CSolidObject::CSTATE_BIT_PROJECTILES);
	result->blockingState.isRaySegmentCollidable = unit->HasCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS);
	result->blockingState.crushable = unit->crushable;
	result->blockingState.blockEnemyPushing = unit->blockEnemyPushing;
	result->blockingState.blockHeightChanges = unit->blockHeightChanges;
}

static void NativeGetUnitHarvestStorage(const GetUnitHarvestStorageQuery* query, GetUnitHarvestStorageResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	std::memset(&result->storage, 0, sizeof(result->storage));

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->storage.storedMetal = unit->harvested.metal;
	result->storage.maxStoredMetal = unit->harvestStorage.metal;
	result->storage.storedEnergy = unit->harvested.energy;
	result->storage.maxStoredEnergy = unit->harvestStorage.energy;
}

static void NativeClearUnitsPreviousDrawFlag(const ClearUnitsPreviousDrawFlagQuery* query, ClearUnitsPreviousDrawFlagResult* result) {
	bufferPos = 0;
	(void)query;

	result->error = nullptr;
	result->success = false;

	if (!IsReady() || unitDrawer == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	unitDrawer->ClearPreviousDrawFlags();
	result->success = true;
}

} // namespace

const UnitsInfoApi UNITS_INFO_API = {
	.GetUnitTooltip = NativeGetUnitTooltip,
	.GetUnitDefID = NativeGetUnitDefID,
	.GetUnitMoveDefID = NativeGetUnitMoveDefID,
	.GetUnitTeam = NativeGetUnitTeam,
	.GetUnitAllyTeam = NativeGetUnitAllyTeam,
	.GetUnitNeutral = NativeGetUnitNeutral,
	.GetUnitHealth = NativeGetUnitHealth,
	.GetUnitIsDead = NativeGetUnitIsDead,
	.GetUnitIsStunned = NativeGetUnitIsStunned,
	.GetUnitIsBeingBuilt = NativeGetUnitIsBeingBuilt,
	.GetUnitCosts = NativeGetUnitCosts,
	.GetUnitCostTable = NativeGetUnitCostTable,
	.GetUnitResources = NativeGetUnitResources,
	.GetUnitStorage = NativeGetUnitStorage,
	.GetUnitMetalExtraction = NativeGetUnitMetalExtraction,
	.GetUnitExperience = NativeGetUnitExperience,
	.GetUnitStates = NativeGetUnitStates,
	.GetUnitArmored = NativeGetUnitArmored,
	.GetUnitIsActive = NativeGetUnitIsActive,
	.GetUnitIsCloaked = NativeGetUnitIsCloaked,
	.GetUnitSeismicSignature = NativeGetUnitSeismicSignature,
	.GetUnitSensorRadius = NativeGetUnitSensorRadius,
	.GetUnitPosErrorParams = NativeGetUnitPosErrorParams,
	.GetUnitHeight = NativeGetUnitHeight,
	.GetUnitRadius = NativeGetUnitRadius,
	.GetUnitBuildeeRadius = NativeGetUnitBuildeeRadius,
	.GetUnitMass = NativeGetUnitMass,
	.GetUnitPosition = NativeGetUnitPosition,
	.GetUnitBasePosition = NativeGetUnitBasePosition,
	.GetUnitVectors = NativeGetUnitVectors,
	.GetUnitRotation = NativeGetUnitRotation,
	.GetUnitDirection = NativeGetUnitDirection,
	.GetUnitHeading = NativeGetUnitHeading,
	.GetUnitVelocity = NativeGetUnitVelocity,
	.GetUnitBuildFacing = NativeGetUnitBuildFacing,
	.GetUnitIsBuilding = NativeGetUnitIsBuilding,
	.GetUnitWorkerTask = NativeGetUnitWorkerTask,
	.GetUnitEffectiveBuildRange = NativeGetUnitEffectiveBuildRange,
	.GetUnitCurrentBuildPower = NativeGetUnitCurrentBuildPower,
	.GetUnitBuildParams = NativeGetUnitBuildParams,
	.GetUnitInBuildStance = NativeGetUnitInBuildStance,
	.GetUnitNanoPieces = NativeGetUnitNanoPieces,
	.GetUnitTransporter = NativeGetUnitTransporter,
	.GetUnitIsTransporting = NativeGetUnitIsTransporting,
	.GetUnitStockpile = NativeGetUnitStockpile,
	.GetUnitSelfDTime = NativeGetUnitSelfDTime,
	.GetUnitShieldState = NativeGetUnitShieldState,
	.GetUnitFlanking = NativeGetUnitFlanking,
	.GetUnitTravel = NativeGetUnitTravel,
	.GetUnitFuel = NativeGetUnitFuel,
	.GetUnitLastAttacker = NativeGetUnitLastAttacker,
	.GetUnitLastAttackedPiece = NativeGetUnitLastAttackedPiece,
	.GetUnitLosState = NativeGetUnitLosState,
	.GetUnitCollisionVolumeData = NativeGetUnitCollisionVolumeData,
	.GetUnitPieceCollisionVolumeData = NativeGetUnitPieceCollisionVolumeData,
	.GetUnitBlocking = NativeGetUnitBlocking,
	.GetUnitHarvestStorage = NativeGetUnitHarvestStorage,
	.ClearUnitsPreviousDrawFlag = NativeClearUnitsPreviousDrawFlag,
	.GetUnitCrashing = NativeGetUnitCrashing,
};

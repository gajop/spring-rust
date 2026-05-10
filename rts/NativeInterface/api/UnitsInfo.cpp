#include "UnitsInfo.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Units/UnitTypes/Builder.h"
#include "Sim/Units/UnitTypes/Factory.h"
#include "Sim/MoveTypes/MoveDefHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Weapons/PlasmaRepulser.h"
#include "System/float3.h"
#include "Rendering/Units/UnitDrawer.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Unit system not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };

static bool IsReady() {
	return (gs != nullptr);
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

	result->tooltip = unit->unitDef->humanName.c_str();
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

	result->storage.metalStorage = unit->unitDef->storage.metal;
	result->storage.energyStorage = unit->unitDef->storage.energy;
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

	result->states.fireState = unit->fireState;
	result->states.moveState = unit->moveState;
	result->states.repeat = false; // Would need command queue
	result->states.cloak = unit->wantCloak;
	result->states.active = unit->activated;
	result->states.trajectory = unit->useHighTrajectory;
	result->states.autoLand = !unit->unitDef->DontLand();
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

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->radius.los = unit->losRadius;
	result->radius.airLos = unit->airLosRadius;
	result->radius.radar = unit->radarRadius;
	result->radius.sonar = unit->sonarRadius;
	result->radius.seismic = unit->seismicRadius;
	result->radius.radarJammer = unit->jammerRadius;
	result->radius.sonarJammer = unit->sonarJamRadius;
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

	const float3 errVec = unit->GetErrorVector(query->allyTeamID);
	result->params.posError.x = errVec.x;
	result->params.posError.y = errVec.y;
	result->params.posError.z = errVec.z;
	result->params.nextPosError.x = 0.0f;
	result->params.nextPosError.y = 0.0f;
	result->params.nextPosError.z = 0.0f;
	result->params.errorScale = 1.0f;
	result->params.errorMult = 1.0f;
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

	// Return buildee's radius if it exists, otherwise unit's radius
	const CBuilder* builder = dynamic_cast<const CBuilder*>(unit);
	if (builder != nullptr && builder->curBuild != nullptr) {
		result->radius = builder->curBuild->radius;
	} else {
		result->radius = unit->radius;
	}
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

	result->position.x = unit->pos.x;
	result->position.y = unit->pos.y;
	result->position.z = unit->pos.z;
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

	result->rotation.col1.x = unit->rightdir.x;
	result->rotation.col1.y = unit->rightdir.y;
	result->rotation.col1.z = unit->rightdir.z;

	result->rotation.col2.x = unit->updir.x;
	result->rotation.col2.y = unit->updir.y;
	result->rotation.col2.z = unit->updir.z;

	result->rotation.col3.x = unit->frontdir.x;
	result->rotation.col3.y = unit->frontdir.y;
	result->rotation.col3.z = unit->frontdir.z;
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

	result->heading = unit->heading;
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
	}
}

static void NativeGetUnitWorkerTask(const GetUnitWorkerTaskQuery* query, GetUnitWorkerTaskResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->task = "";

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Determine task based on current state
	const CBuilder* builder = dynamic_cast<const CBuilder*>(unit);
	if (builder != nullptr && builder->curBuild != nullptr) {
		result->task = "building";
	} else if (builder != nullptr && builder->curReclaim != nullptr) {
		result->task = "reclaiming";
	} else if (builder != nullptr && builder->curResurrect != nullptr) {
		result->task = "resurrecting";
	} else if (builder != nullptr && builder->curCapture != nullptr) {
		result->task = "capturing";
	} else {
		result->task = "idle";
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

	result->range = unit->unitDef->buildDistance;
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

	result->buildPower = unit->unitDef->buildSpeed;
}

static void NativeGetUnitBuildParams(const GetUnitBuildParamsQuery* query, GetUnitBuildParamsResult* result) {
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
	result->params.buildDistance = ud->buildDistance;
	result->params.buildSpeed = ud->buildSpeed;
	result->params.repairSpeed = ud->repairSpeed;
	result->params.reclaimSpeed = ud->reclaimSpeed;
	result->params.resurrectSpeed = ud->resurrectSpeed;
	result->params.captureSpeed = ud->captureSpeed;
	result->params.terraformSpeed = ud->terraformSpeed;
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

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->isTransporting = !unit->transportedUnits.empty();
}

static void NativeGetUnitStockpile(const GetUnitStockpileQuery* query, GetUnitStockpileResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->weapons.empty()) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Check first weapon for stockpile
	const CWeapon* weapon = unit->weapons[0];
	if (weapon != nullptr) {
		result->stockpile.stockpile = weapon->numStockpiled;
		result->stockpile.stockpileQueueSize = weapon->numStockpileQued;
	} else {
		result->stockpile.stockpile = 0;
		result->stockpile.stockpileQueueSize = 0;
	}
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
	result->flanking.minDamage = unit->flankingBonusAvgDamage - unit->flankingBonusDifDamage;
	result->flanking.maxDamage = unit->flankingBonusAvgDamage + unit->flankingBonusDifDamage;
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
	result->pieceNum = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->pieceNum = -1;  // lastAttackedPiece no longer available
}

static void NativeGetUnitLosState(const GetUnitLosStateQuery* query, GetUnitLosStateResult* result) {
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

	// Get LOS state for allyteam 0 as default
	int allyTeam = 0;
	if (allyTeam < static_cast<int>(unit->losStatus.size())) {
		const uint8_t losStatus = unit->losStatus[allyTeam];
		result->losState.los = (losStatus & LOS_INLOS) != 0;
		result->losState.prevLos = (losStatus & LOS_PREVLOS) != 0;
		result->losState.radar = (losStatus & LOS_INRADAR) != 0;
		result->losState.sonar = false;
		result->losState.seismic = false;
		result->losState.jammer = false;
		result->losState.typed = false;
	}
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

	result->blockingState.isBlocking = unit->immobile;  // blocking field removed, use immobile
	result->blockingState.isSolidObjectCollidable = !unit->collisionVolume.IgnoreHits();
	result->blockingState.isProjectileCollidable = !unit->collisionVolume.IgnoreHits();
	result->blockingState.isRaySegmentCollidable = !unit->collisionVolume.IgnoreHits();
	result->blockingState.crushable = unit->crushResistance > 0.0f;
	result->blockingState.blockEnemyPushing = unit->blockEnemyPushing;
	result->blockingState.blockHeightChanges = false;  // levelGround no longer available
}

static void NativeGetUnitHarvestStorage(const GetUnitHarvestStorageQuery* query, GetUnitHarvestStorageResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->harvestStorage = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->harvestStorage = unit->harvested.metal;
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
};

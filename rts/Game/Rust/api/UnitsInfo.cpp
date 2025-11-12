#include "UnitsInfo.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "UnitsInfo API not yet fully implemented" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit ID" };

// All functions use stub implementations for now
static void NativeGetUnitTooltip(const GetUnitTooltipQuery* query, GetUnitTooltipResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->tooltip = "";
}

static void NativeGetUnitDefID(const GetUnitDefIDQuery* query, GetUnitDefIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->unitDefID = -1;
}

static void NativeGetUnitTeam(const GetUnitTeamQuery* query, GetUnitTeamResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->teamID = -1;
}

static void NativeGetUnitAllyTeam(const GetUnitAllyTeamQuery* query, GetUnitAllyTeamResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->allyTeamID = -1;
}

static void NativeGetUnitNeutral(const GetUnitNeutralQuery* query, GetUnitNeutralResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->neutral = false;
}

static void NativeGetUnitHealth(const GetUnitHealthQuery* query, GetUnitHealthResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitIsDead(const GetUnitIsDeadQuery* query, GetUnitIsDeadResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->isDead = false;
}

static void NativeGetUnitIsStunned(const GetUnitIsStunnedQuery* query, GetUnitIsStunnedResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->isStunned = false;
}

static void NativeGetUnitIsBeingBuilt(const GetUnitIsBeingBuiltQuery* query, GetUnitIsBeingBuiltResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->isBeingBuilt = false;
}

static void NativeGetUnitCosts(const GetUnitCostsQuery* query, GetUnitCostsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitCostTable(const GetUnitCostTableQuery* query, GetUnitCostTableResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitResources(const GetUnitResourcesQuery* query, GetUnitResourcesResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitStorage(const GetUnitStorageQuery* query, GetUnitStorageResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitMetalExtraction(const GetUnitMetalExtractionQuery* query, GetUnitMetalExtractionResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->metalExtraction = 0.0f;
}

static void NativeGetUnitExperience(const GetUnitExperienceQuery* query, GetUnitExperienceResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->experience = 0.0f;
}

static void NativeGetUnitStates(const GetUnitStatesQuery* query, GetUnitStatesResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitArmored(const GetUnitArmoredQuery* query, GetUnitArmoredResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->armoredState.armored = false;
	result->armoredState.armorMultiple = 1.0f;
}

static void NativeGetUnitIsActive(const GetUnitIsActiveQuery* query, GetUnitIsActiveResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->isActive = false;
}

static void NativeGetUnitIsCloaked(const GetUnitIsCloakedQuery* query, GetUnitIsCloakedResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->isCloaked = false;
}

static void NativeGetUnitSeismicSignature(const GetUnitSeismicSignatureQuery* query, GetUnitSeismicSignatureResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->seismicSignature = 0.0f;
}

static void NativeGetUnitSensorRadius(const GetUnitSensorRadiusQuery* query, GetUnitSensorRadiusResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitPosErrorParams(const GetUnitPosErrorParamsQuery* query, GetUnitPosErrorParamsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitHeight(const GetUnitHeightQuery* query, GetUnitHeightResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->height = 0.0f;
}

static void NativeGetUnitRadius(const GetUnitRadiusQuery* query, GetUnitRadiusResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->radius = 0.0f;
}

static void NativeGetUnitBuildeeRadius(const GetUnitBuildeeRadiusQuery* query, GetUnitBuildeeRadiusResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->radius = 0.0f;
}

static void NativeGetUnitMass(const GetUnitMassQuery* query, GetUnitMassResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->mass = 0.0f;
}

static void NativeGetUnitPosition(const GetUnitPositionQuery* query, GetUnitPositionResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->position.x = 0.0f;
	result->position.y = 0.0f;
	result->position.z = 0.0f;
}

static void NativeGetUnitBasePosition(const GetUnitBasePositionQuery* query, GetUnitBasePositionResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->position.x = 0.0f;
	result->position.y = 0.0f;
	result->position.z = 0.0f;
}

static void NativeGetUnitVectors(const GetUnitVectorsQuery* query, GetUnitVectorsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitRotation(const GetUnitRotationQuery* query, GetUnitRotationResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitDirection(const GetUnitDirectionQuery* query, GetUnitDirectionResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->direction.x = 0.0f;
	result->direction.y = 0.0f;
	result->direction.z = 0.0f;
}

static void NativeGetUnitHeading(const GetUnitHeadingQuery* query, GetUnitHeadingResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->heading = 0;
}

static void NativeGetUnitVelocity(const GetUnitVelocityQuery* query, GetUnitVelocityResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->velocity.x = 0.0f;
	result->velocity.y = 0.0f;
	result->velocity.z = 0.0f;
}

static void NativeGetUnitBuildFacing(const GetUnitBuildFacingQuery* query, GetUnitBuildFacingResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->facing = 0;
}

static void NativeGetUnitIsBuilding(const GetUnitIsBuildingQuery* query, GetUnitIsBuildingResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->buildeeID = -1;
}

static void NativeGetUnitWorkerTask(const GetUnitWorkerTaskQuery* query, GetUnitWorkerTaskResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->task = "";
}

static void NativeGetUnitEffectiveBuildRange(const GetUnitEffectiveBuildRangeQuery* query, GetUnitEffectiveBuildRangeResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->range = 0.0f;
}

static void NativeGetUnitCurrentBuildPower(const GetUnitCurrentBuildPowerQuery* query, GetUnitCurrentBuildPowerResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->buildPower = 0.0f;
}

static void NativeGetUnitBuildParams(const GetUnitBuildParamsQuery* query, GetUnitBuildParamsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitInBuildStance(const GetUnitInBuildStanceQuery* query, GetUnitInBuildStanceResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->inBuildStance = false;
}

static void NativeGetUnitNanoPieces(const GetUnitNanoPiecesQuery* query, GetUnitNanoPiecesResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->pieces = nullptr;
	result->count = 0;
}

static void NativeGetUnitTransporter(const GetUnitTransporterQuery* query, GetUnitTransporterResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->transporterID = -1;
}

static void NativeGetUnitIsTransporting(const GetUnitIsTransportingQuery* query, GetUnitIsTransportingResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->isTransporting = false;
}

static void NativeGetUnitStockpile(const GetUnitStockpileQuery* query, GetUnitStockpileResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitSelfDTime(const GetUnitSelfDTimeQuery* query, GetUnitSelfDTimeResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->selfDTime = 0.0f;
}

static void NativeGetUnitShieldState(const GetUnitShieldStateQuery* query, GetUnitShieldStateResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->hasShield = false;
}

static void NativeGetUnitFlanking(const GetUnitFlankingQuery* query, GetUnitFlankingResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitTravel(const GetUnitTravelQuery* query, GetUnitTravelResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitFuel(const GetUnitFuelQuery* query, GetUnitFuelResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitLastAttacker(const GetUnitLastAttackerQuery* query, GetUnitLastAttackerResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->hasAttacker = false;
}

static void NativeGetUnitLastAttackedPiece(const GetUnitLastAttackedPieceQuery* query, GetUnitLastAttackedPieceResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->pieceNum = -1;
}

static void NativeGetUnitLosState(const GetUnitLosStateQuery* query, GetUnitLosStateResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitCollisionVolumeData(const GetUnitCollisionVolumeDataQuery* query, GetUnitCollisionVolumeDataResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitPieceCollisionVolumeData(const GetUnitPieceCollisionVolumeDataQuery* query, GetUnitPieceCollisionVolumeDataResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitBlocking(const GetUnitBlockingQuery* query, GetUnitBlockingResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitHarvestStorage(const GetUnitHarvestStorageQuery* query, GetUnitHarvestStorageResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->harvestStorage = 0.0f;
}

} // namespace

const UnitsInfoApi UNITS_INFO_API = {
	.GetUnitTooltip = NativeGetUnitTooltip,
	.GetUnitDefID = NativeGetUnitDefID,
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
};

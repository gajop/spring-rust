#include "SyncedCtrl.h"
#include <cstring>

namespace {

// Thread-local scratch buffer for dynamic data
thread_local uint8_t scratchBuffer[8192];
thread_local size_t bufferPos = 0;

// Error messages
static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "SyncedCtrl API not yet fully implemented - stubs only"
};

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Game not ready"
};

static Error* MakeError(int32_t code, const char* message) {
	const size_t msgLen = strlen(message);
	const size_t totalSize = sizeof(Error) + msgLen + 1;

	if (bufferPos + totalSize > sizeof(scratchBuffer)) {
		static Error bufferOverflow = {
			.code = ERROR_BUFFER_OVERFLOW,
			.message = "Scratch buffer overflow"
		};
		return &bufferOverflow;
	}

	Error* error = reinterpret_cast<Error*>(scratchBuffer + bufferPos);
	bufferPos += sizeof(Error);

	char* msgBuffer = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(msgBuffer, message, msgLen + 1);
	bufferPos += msgLen + 1;

	error->code = code;
	error->message = msgBuffer;

	return error;
}

// ============================================================================
// Team Control Implementation
// ============================================================================

static void NativeSetAlly(const SetAllyQuery* query, SetAllyResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetAllyTeamStartBox(const SetAllyTeamStartBoxQuery* query, SetAllyTeamStartBoxResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeKillTeam(const KillTeamQuery* query, KillTeamResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeAssignPlayerToTeam(const AssignPlayerToTeamQuery* query, AssignPlayerToTeamResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeGameOver(const GameOverQuery* query, GameOverResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetGlobalLos(const SetGlobalLosQuery* query, SetGlobalLosResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeAddTeamResource(const AddTeamResourceQuery* query, AddTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeUseTeamResource(const UseTeamResourceQuery* query, UseTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetTeamResource(const SetTeamResourceQuery* query, SetTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetTeamShareLevel(const SetTeamShareLevelQuery* query, SetTeamShareLevelResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeShareTeamResource(const ShareTeamResourceQuery* query, ShareTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static const TeamControlApi TEAM_CONTROL_API = {
	.SetAlly = NativeSetAlly,
	.SetAllyTeamStartBox = NativeSetAllyTeamStartBox,
	.KillTeam = NativeKillTeam,
	.AssignPlayerToTeam = NativeAssignPlayerToTeam,
	.GameOver = NativeGameOver,
	.SetGlobalLos = NativeSetGlobalLos,
	.AddTeamResource = NativeAddTeamResource,
	.UseTeamResource = NativeUseTeamResource,
	.SetTeamResource = NativeSetTeamResource,
	.SetTeamShareLevel = NativeSetTeamShareLevel,
	.ShareTeamResource = NativeShareTeamResource
};

// ============================================================================
// Unit Control Implementation
// ============================================================================

static void NativeCreateUnit(const CreateUnitQuery* query, CreateUnitResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->unitID = -1;
}

static void NativeDestroyUnit(const DestroyUnitQuery* query, DestroyUnitResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeTransferUnit(const TransferUnitQuery* query, TransferUnitResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeGiveOrderToUnit(const GiveOrderToUnitQuery* query, GiveOrderToUnitResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeGiveOrderToUnitArray(const GiveOrderToUnitArrayQuery* query, GiveOrderToUnitArrayResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeUnitFinishCommand(const UnitFinishCommandQuery* query, UnitFinishCommandResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitHealth(const SetUnitHealthQuery* query, SetUnitHealthResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitMaxHealth(const SetUnitMaxHealthQuery* query, SetUnitMaxHealthResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitExperience(const SetUnitExperienceQuery* query, SetUnitExperienceResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeAddUnitExperience(const AddUnitExperienceQuery* query, AddUnitExperienceResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitNeutral(const SetUnitNeutralQuery* query, SetUnitNeutralResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitResourcing(const SetUnitResourcingQuery* query, SetUnitResourcingResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitMetalExtraction(const SetUnitMetalExtractionQuery* query, SetUnitMetalExtractionResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitPosition(const SetUnitPositionQuery* query, SetUnitPositionResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitVelocity(const SetUnitVelocityQuery* query, SetUnitVelocityResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitRotation(const SetUnitRotationQuery* query, SetUnitRotationResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetUnitPhysics(const SetUnitPhysicsQuery* query, SetUnitPhysicsResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeAddUnitDamage(const AddUnitDamageQuery* query, AddUnitDamageResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeAddUnitImpulse(const AddUnitImpulseQuery* query, AddUnitImpulseResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static const UnitControlApi UNIT_CONTROL_API = {
	.CreateUnit = NativeCreateUnit,
	.DestroyUnit = NativeDestroyUnit,
	.TransferUnit = NativeTransferUnit,
	.GiveOrderToUnit = NativeGiveOrderToUnit,
	.GiveOrderToUnitArray = NativeGiveOrderToUnitArray,
	.UnitFinishCommand = NativeUnitFinishCommand,
	.SetUnitHealth = NativeSetUnitHealth,
	.SetUnitMaxHealth = NativeSetUnitMaxHealth,
	.SetUnitExperience = NativeSetUnitExperience,
	.AddUnitExperience = NativeAddUnitExperience,
	.SetUnitNeutral = NativeSetUnitNeutral,
	.SetUnitResourcing = NativeSetUnitResourcing,
	.SetUnitMetalExtraction = NativeSetUnitMetalExtraction,
	.SetUnitPosition = NativeSetUnitPosition,
	.SetUnitVelocity = NativeSetUnitVelocity,
	.SetUnitRotation = NativeSetUnitRotation,
	.SetUnitPhysics = NativeSetUnitPhysics,
	.AddUnitDamage = NativeAddUnitDamage,
	.AddUnitImpulse = NativeAddUnitImpulse
};

// ============================================================================
// Feature Control Implementation
// ============================================================================

static void NativeCreateFeature(const CreateFeatureQuery* query, CreateFeatureResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->featureID = -1;
}

static void NativeDestroyFeature(const DestroyFeatureQuery* query, DestroyFeatureResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeTransferFeature(const TransferFeatureQuery* query, TransferFeatureResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetFeatureHealth(const SetFeatureHealthQuery* query, SetFeatureHealthResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetFeaturePosition(const SetFeaturePositionQuery* query, SetFeaturePositionResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetFeatureDirection(const SetFeatureDirectionQuery* query, SetFeatureDirectionResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetFeatureVelocity(const SetFeatureVelocityQuery* query, SetFeatureVelocityResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetFeatureResources(const SetFeatureResourcesQuery* query, SetFeatureResourcesResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static const FeatureControlApi FEATURE_CONTROL_API = {
	.CreateFeature = NativeCreateFeature,
	.DestroyFeature = NativeDestroyFeature,
	.TransferFeature = NativeTransferFeature,
	.SetFeatureHealth = NativeSetFeatureHealth,
	.SetFeaturePosition = NativeSetFeaturePosition,
	.SetFeatureDirection = NativeSetFeatureDirection,
	.SetFeatureVelocity = NativeSetFeatureVelocity,
	.SetFeatureResources = NativeSetFeatureResources
};

// ============================================================================
// Terrain Control Implementation
// ============================================================================

static void NativeAddHeightMap(const AddHeightMapQuery* query, AddHeightMapResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetHeightMap(const SetHeightMapQuery* query, SetHeightMapResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeRevertHeightMap(const RevertHeightMapQuery* query, RevertHeightMapResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeAddSmoothMesh(const AddSmoothMeshQuery* query, AddSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetSmoothMesh(const SetSmoothMeshQuery* query, SetSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeRevertSmoothMesh(const RevertSmoothMeshQuery* query, RevertSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetMapSquareTerrainType(const SetMapSquareTerrainTypeQuery* query, SetMapSquareTerrainTypeResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetTerrainTypeData(const SetTerrainTypeDataQuery* query, SetTerrainTypeDataResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetTidal(const SetTidalQuery* query, SetTidalResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetWind(const SetWindQuery* query, SetWindResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static const TerrainControlApi TERRAIN_CONTROL_API = {
	.AddHeightMap = NativeAddHeightMap,
	.SetHeightMap = NativeSetHeightMap,
	.RevertHeightMap = NativeRevertHeightMap,
	.AddSmoothMesh = NativeAddSmoothMesh,
	.SetSmoothMesh = NativeSetSmoothMesh,
	.RevertSmoothMesh = NativeRevertSmoothMesh,
	.SetMapSquareTerrainType = NativeSetMapSquareTerrainType,
	.SetTerrainTypeData = NativeSetTerrainTypeData,
	.SetTidal = NativeSetTidal,
	.SetWind = NativeSetWind
};

// ============================================================================
// Projectile Control Implementation
// ============================================================================

static void NativeSpawnProjectile(const SpawnProjectileQuery* query, SpawnProjectileResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->projectileID = -1;
}

static void NativeDeleteProjectile(const DeleteProjectileQuery* query, DeleteProjectileResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetProjectilePosition(const SetProjectilePositionQuery* query, SetProjectilePositionResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetProjectileVelocity(const SetProjectileVelocityQuery* query, SetProjectileVelocityResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetProjectileGravity(const SetProjectileGravityQuery* query, SetProjectileGravityResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static void NativeSetProjectileTarget(const SetProjectileTargetQuery* query, SetProjectileTargetResult* result)
{
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->success = false;
}

static const ProjectileControlApi PROJECTILE_CONTROL_API = {
	.SpawnProjectile = NativeSpawnProjectile,
	.DeleteProjectile = NativeDeleteProjectile,
	.SetProjectilePosition = NativeSetProjectilePosition,
	.SetProjectileVelocity = NativeSetProjectileVelocity,
	.SetProjectileGravity = NativeSetProjectileGravity,
	.SetProjectileTarget = NativeSetProjectileTarget
};

} // namespace

// ============================================================================
// Public API Export
// ============================================================================

const SyncedCtrlApi SYNCED_CTRL_API = {
	.team = &TEAM_CONTROL_API,
	.unit = &UNIT_CONTROL_API,
	.feature = &FEATURE_CONTROL_API,
	.terrain = &TERRAIN_CONTROL_API,
	.projectile = &PROJECTILE_CONTROL_API
};

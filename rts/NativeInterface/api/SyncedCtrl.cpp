#include "SyncedCtrl.h"
#include <algorithm>
#include <cstring>
#include <cmath>
#include <cstdlib>
#include <vector>

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitLoader.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/CommandAI/CommandAI.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Sim/Units/UnitTypes/ExtractorBuilding.h"
#include "Sim/Units/UnitTypes/Factory.h"
#include "Sim/Units/UnitTypes/Builder.h"
#include "Sim/Units/UnitToolTipMap.hpp"
#include "Rendering/Models/3DModel.hpp"
#include "Rendering/Models/IModelParser.h"
#include "Rendering/Env/IGroundDecalDrawer.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Features/FeatureDefHandler.h"
#include "Sim/Features/FeatureHandler.h"
#include "Rendering/Env/GrassDrawer.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectileFactory.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectileTypes.h"
#include "Sim/Projectiles/WeaponProjectiles/MissileProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/StarburstProjectile.h"
#include "Sim/Misc/GlobalConstants.h"
#include "Sim/Projectiles/WeaponProjectiles/TorpedoProjectile.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/Team.h"
#include "Sim/Misc/AllyTeam.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/ModInfo.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/DamageArray.h"
#include "Sim/Misc/SmoothHeightMesh.h"
#include "Sim/Misc/Wind.h"
#include "Sim/Weapons/WeaponDefHandler.h"
#include "Sim/Weapons/WeaponDef.h"
#include "Sim/Weapons/Weapon.h"
#include "Sim/Weapons/PlasmaRepulser.h"
#include "Sim/MoveTypes/MoveType.h"
#include "Sim/MoveTypes/AAirMoveType.h"
#include "Sim/Projectiles/ExplosionGenerator.h"
#include "Sim/Projectiles/PieceProjectile.h"
#include "Sim/Misc/BuildingMaskMap.h"
#include "Sim/Misc/QuadField.h"
#include "Net/GameServer.h"
#include "Game/Game.h"
#include "Game/GameHelper.h"
#include "Game/GameSetup.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/Players/Player.h"
#include "Lua/LuaConfig.h"
#include "Lua/LuaUI.h"
#include "Map/ReadMap.h"
#include "Map/MapDamage.h"
#include "Map/MapInfo.h"
#include "Map/MapDimensions.h"
#include "Map/Ground.h"
#include "Sim/Path/IPathManager.h"
#include "Sim/Misc/GroundBlockingObjectMap.h"
#include "Sim/Misc/CollisionVolume.h"
#include "Sim/Misc/GlobalConstants.h"
#include "System/EventHandler.h"
#include "System/StringHash.h"
#include "System/float3.h"
#include "System/Matrix44f.h"
#include "System/creg/STL_Map.h"
#include "System/Log/ILog.h"
#include "Sim/Units/Scripts/CobInstance.h"
#include "Sim/Units/Scripts/NullUnitScript.h"

namespace {

// Thread-local scratch buffer for dynamic data
thread_local uint8_t scratchBuffer[1024];
thread_local size_t bufferPos = 0;

// Error messages
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Game not ready"
};

static const Error NOT_AVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Not available in Native API"
};

static const Error INVALID_TEAM_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid team ID"
};

static const Error INVALID_ALLYTEAM_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid ally team ID"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

static const Error UNIT_CREATION_FAILED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Unit loader could not create the requested unit"
};

static const Error INVALID_UNITDEF_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit definition ID"
};

static const Error INVALID_FEATURE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid feature ID"
};

static const Error INVALID_FEATUREDEF_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid feature definition ID"
};

static const Error INVALID_WEAPONDEF_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid weapon definition ID"
};

static const Error INVALID_PROJECTILE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid projectile ID"
};

static const Error INVALID_PLAYER_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid player ID"
};

static const Error INVALID_RESOURCE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid resource type (use 'metal' or 'energy')"
};

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static const Error BUFFER_OVERFLOW_ERROR = {
	.code = ERROR_BUFFER_OVERFLOW,
	.message = "Scratch buffer overflow"
};

static bool IsReady() {
	return (gs != nullptr);
}

static Error* MakeError(int32_t code, const char* message) {
	const size_t msgLen = strlen(message);
	const size_t totalSize = sizeof(Error) + msgLen + 1;

	if (bufferPos + totalSize > sizeof(scratchBuffer)) {
		return const_cast<Error*>(&BUFFER_OVERFLOW_ERROR);
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

// Helper to get resource index from string
static int GetResourceIndex(const char* type) {
	if (strcmp(type, "metal") == 0 || strcmp(type, "m") == 0) return 0;
	if (strcmp(type, "energy") == 0 || strcmp(type, "e") == 0) return 1;
	return -1;
}

// ============================================================================
// Team Control Implementation
// ============================================================================

static void NativeSetAlly(const SetAllyQuery* query, SetAllyResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidAllyTeam(query->firstAllyTeamID) ||
	    !teamHandler.IsValidAllyTeam(query->secondAllyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}

	teamHandler.SetAlly(query->firstAllyTeamID, query->secondAllyTeamID, query->allied);
	result->success = true;
}

static void NativeSetAllyTeamStartBox(const SetAllyTeamStartBoxQuery* query, SetAllyTeamStartBoxResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}

	if (gameSetup == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Set the start box for the ally team (convert to normalized coordinates)
	AllyTeam& allyTeam = teamHandler.GetAllyTeam(query->allyTeamID);
	allyTeam.startRectLeft   = query->minX / float(mapDims.mapx * SQUARE_SIZE);
	allyTeam.startRectRight  = query->maxX / float(mapDims.mapx * SQUARE_SIZE);
	allyTeam.startRectTop    = query->minZ / float(mapDims.mapy * SQUARE_SIZE);
	allyTeam.startRectBottom = query->maxZ / float(mapDims.mapy * SQUARE_SIZE);
	result->success = true;
}

static void NativeKillTeam(const KillTeamQuery* query, KillTeamResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	// Match Spring.KillTeam: the Lua path uses the normal-death behavior,
	// including the associated player/AI state transitions.
	team->Died();
	result->success = true;
}

static void NativeAssignPlayerToTeam(const AssignPlayerToTeamQuery* query, AssignPlayerToTeamResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!playerHandler.IsValidPlayer(query->playerID)) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid player ID");
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	CPlayer* player = playerHandler.Player(query->playerID);
	if (player == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid player");
		return;
	}

	player->SetControlledTeams();
	player->team = query->teamID;
	player->SetControlledTeams();
	result->success = true;
}

static void NativeGameOver(const GameOverQuery* query, GameOverResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Match Spring.GameOver: filter invalid ally-team IDs and let CGame perform
	// the complete game-end transition and event notification.
	std::vector<unsigned char> winningAllyTeams;
	winningAllyTeams.reserve(query->count);
	for (uint32_t i = 0; i < query->count; ++i) {
		const int allyTeamID = query->winningAllyTeams[i];
		if (teamHandler.ValidAllyTeam(allyTeamID))
			winningAllyTeams.push_back(static_cast<unsigned char>(allyTeamID));
	}

	game->GameEnd(winningAllyTeams);

	result->success = true;
}

static void NativeSetGlobalLos(const SetGlobalLosQuery* query, SetGlobalLosResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}

	if (losHandler != nullptr) {
		losHandler->SetGlobalLOS(query->allyTeamID, query->enabled);
		result->success = true;
	}
}

static void NativeAddTeamResource(const AddTeamResourceQuery* query, AddTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	int resIdx = GetResourceIndex(query->resourceType);
	if (resIdx < 0) {
		result->error = &INVALID_RESOURCE_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	const float amount = std::max(0.0f, query->amount);
	if (resIdx == 0)
		team->AddResources({amount, 0.0f});
	else
		team->AddResources({0.0f, amount});

	result->success = true;
}

static void NativeAddTeamResourceExcessStats(const AddTeamResourceExcessStatsQuery* query, AddTeamResourceExcessStatsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	const int resIdx = GetResourceIndex(query->resourceType);
	if (resIdx < 0) {
		result->error = &INVALID_RESOURCE_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	const float amount = std::max(0.0f, query->amount);
	float& resourceExcess = (resIdx == 0) ? team->resPrevExcess.metal : team->resPrevExcess.energy;
	TeamStatistics& statistics = team->GetCurrentStats();
	float& statisticsExcess = (resIdx == 0) ? statistics.metalExcess : statistics.energyExcess;

	resourceExcess += amount;
	statisticsExcess += amount;
	result->success = true;
}

static void NativeUseTeamResource(const UseTeamResourceQuery* query, UseTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	int resIdx = GetResourceIndex(query->resourceType);
	if (resIdx < 0) {
		result->error = &INVALID_RESOURCE_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	const float amount = std::max(0.0f, query->amount);
	if (resIdx == 0) {
		team->resPull.metal += amount;
		result->success = team->UseResources({amount, 0.0f});
	} else {
		team->resPull.energy += amount;
		result->success = team->UseResources({0.0f, amount});
	}
}

static void NativeSetTeamResource(const SetTeamResourceQuery* query, SetTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	const bool metalStorage = (strcmp(query->resourceType, "metalStorage") == 0);
	const bool energyStorage = (strcmp(query->resourceType, "energyStorage") == 0);
	int resIdx = GetResourceIndex(query->resourceType);
	if (resIdx < 0 && !metalStorage && !energyStorage) {
		result->error = &INVALID_RESOURCE_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	const float amount = std::max(0.0f, query->amount);
	if (metalStorage) {
		team->resStorage.metal = amount;
		team->res.metal = std::min(team->res.metal, team->resStorage.metal);
	} else if (energyStorage) {
		team->resStorage.energy = amount;
		team->res.energy = std::min(team->res.energy, team->resStorage.energy);
	} else if (resIdx == 0) {
		team->res.metal = std::min(team->resStorage.metal, amount);
	} else {
		team->res.energy = std::min(team->resStorage.energy, amount);
	}

	result->success = true;
}

static void NativeSetTeamShareLevel(const SetTeamShareLevelQuery* query, SetTeamShareLevelResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	int resIdx = GetResourceIndex(query->resourceType);
	if (resIdx < 0) {
		result->error = &INVALID_RESOURCE_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	float shareLevel = std::max(0.0f, std::min(1.0f, query->shareLevel));

	if (resIdx == 0) {
		team->resShare.metal = shareLevel;
	} else {
		team->resShare.energy = shareLevel;
	}

	result->success = true;
}

static void NativeShareTeamResource(const ShareTeamResourceQuery* query, ShareTeamResourceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID) || !teamHandler.IsValidTeam(query->targetTeamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	int resIdx = GetResourceIndex(query->resourceType);
	if (resIdx < 0) {
		result->error = &INVALID_RESOURCE_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	CTeam* targetTeam = teamHandler.Team(query->targetTeamID);

	if (team == nullptr || targetTeam == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	float* sourceRes = (resIdx == 0) ? &team->res.metal : &team->res.energy;
	float* targetRes = (resIdx == 0) ? &targetTeam->res.metal : &targetTeam->res.energy;

	float amount = std::min(query->amount, *sourceRes);
	*sourceRes -= amount;
	*targetRes += amount;

	if (resIdx == 0) {
		team->resSent.metal += amount;
		targetTeam->resReceived.metal += amount;
		team->GetCurrentStats().metalSent += amount;
		targetTeam->GetCurrentStats().metalReceived += amount;
	} else {
		team->resSent.energy += amount;
		targetTeam->resReceived.energy += amount;
		team->GetCurrentStats().energySent += amount;
		targetTeam->GetCurrentStats().energyReceived += amount;
	}

	result->success = true;
}

static void NativeSetTeamStartPosition(const SetTeamStartPositionQuery* query, SetTeamStartPositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	CTeam* team = teamHandler.Team(query->teamID);
	float3 pickPos(query->pos.x, query->pos.y, query->pos.z);
	team->ClampStartPosInStartBox(&pickPos);
	team->SetStartPos(pickPos);

	result->success = true;
}

static void NativeSetPlayerReadyState(const SetPlayerReadyStateQuery* query, SetPlayerReadyStateResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!playerHandler.IsValidPlayer(query->playerID)) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid player ID");
		return;
	}

	playerHandler.Player(query->playerID)->SetReadyToStart(query->ready);
	result->success = true;
}

static void NativeTransferTeamMaxUnits(const TransferTeamMaxUnitsQuery* query, TransferTeamMaxUnitsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->fromTeamID)) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid from team ID");
		return;
	}

	if (!teamHandler.IsValidTeam(query->toTeamID)) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid to team ID");
		return;
	}

	CTeam* fromTeam = teamHandler.Team(query->fromTeamID);
	CTeam* toTeam = teamHandler.Team(query->toTeamID);

	if (fromTeam == nullptr || toTeam == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	result->success = teamHandler.TransferTeamMaxUnits(fromTeam, toTeam, query->amount);
}

static const TeamControlApi TEAM_CONTROL_API = {
	.SetAlly = NativeSetAlly,
	.SetAllyTeamStartBox = NativeSetAllyTeamStartBox,
	.KillTeam = NativeKillTeam,
	.AssignPlayerToTeam = NativeAssignPlayerToTeam,
	.GameOver = NativeGameOver,
	.SetGlobalLos = NativeSetGlobalLos,
	.AddTeamResource = NativeAddTeamResource,
	.AddTeamResourceExcessStats = NativeAddTeamResourceExcessStats,
	.UseTeamResource = NativeUseTeamResource,
	.SetTeamResource = NativeSetTeamResource,
	.SetTeamShareLevel = NativeSetTeamShareLevel,
	.ShareTeamResource = NativeShareTeamResource,
	.SetTeamStartPosition = NativeSetTeamStartPosition,
	.SetPlayerReadyState = NativeSetPlayerReadyState,
	.TransferTeamMaxUnits = NativeTransferTeamMaxUnits
};

// ============================================================================
// Unit Control Implementation
// ============================================================================

static void NativeCreateUnit(const CreateUnitQuery* query, CreateUnitResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	const UnitDef* unitDef = (query->unitDef.id >= 0)
		? unitDefHandler->GetUnitDefByID(query->unitDef.id)
		: ((query->unitDef.name != nullptr) ? unitDefHandler->GetUnitDefByName(query->unitDef.name) : nullptr);
	if (unitDef == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);

	const CUnit* builder = nullptr;
	if (query->options.builderID >= 0) {
		builder = unitHandler.GetUnit(query->options.builderID);
	}

	UnitLoadParams params;
	params.unitDef = unitDef;
	params.builder = builder;
	params.pos = pos;
	params.speed = ZeroVector;
	params.unitID = (query->options.unitID >= 0) ? query->options.unitID : -1;
	params.teamID = query->teamID;
	params.facing = query->facing;
	params.beingBuilt = query->options.build;
	params.flattenGround = query->options.flattenGround;

	CUnit* unit = unitLoader->LoadUnit(params);

	if (unit != nullptr) {
		result->unitID = unit->id;
		if (builder != nullptr && unitDef != nullptr) {
			unit->SetSoloBuilder(const_cast<CUnit*>(builder), unitDef);
		}
	} else {
		result->error = &UNIT_CREATION_FAILED_ERROR;
		LOG_L(L_WARNING, "NativeCreateUnit: UnitLoader returned null for '%s' at (%f, %f, %f), team=%d",
			unitDef->name.c_str(), pos.x, pos.y, pos.z, query->teamID);
	}
}

static void NativeDestroyUnit(const DestroyUnitQuery* query, DestroyUnitResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CUnit* attacker = nullptr;
	if (query->options.attackerID >= 0) {
		attacker = unitHandler.GetUnit(query->options.attackerID);
	}

	if (query->options.selfd) {
		unit->KillUnit(attacker, true, query->options.reclaimed);
	} else {
		unit->KillUnit(attacker, false, query->options.reclaimed);
	}

	if (query->options.recycleID) {
		unitHandler.GarbageCollectUnit(unit->id);
	}

	result->success = true;
}

static void NativeTransferUnit(const TransferUnitQuery* query, TransferUnitResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->newTeamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	CTeam* oldTeam = teamHandler.Team(unit->team);
	CTeam* newTeam = teamHandler.Team(query->newTeamID);
	if (oldTeam == nullptr || newTeam == nullptr) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}
	if (query->adjustUnitLimit) {
		newTeam->maxUnits++;
		oldTeam->maxUnits--;
	}
	result->success = unit->ChangeTeam(query->newTeamID, query->given ? CUnit::ChangeGiven : CUnit::ChangeCaptured);
	if (query->adjustUnitLimit && !result->success) {
		newTeam->maxUnits--;
		oldTeam->maxUnits++;
	}
}

static void NativeGiveOrderToUnit(const GiveOrderToUnitQuery* query, GiveOrderToUnitResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->commandAI == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	Command cmd(query->cmdID, query->options);
	if (query->timeout > 0)
		cmd.SetTimeOut(query->timeout);

	for (uint32_t i = 0; i < query->paramCount; ++i) {
		cmd.PushParam(query->params[i]);
	}

	unit->commandAI->GiveCommand(cmd);
	result->success = true;
}

static void NativeGiveOrderToUnitArray(const GiveOrderToUnitArrayQuery* query, GiveOrderToUnitArrayResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	Command cmd(query->cmdID, query->options);
	if (query->timeout > 0)
		cmd.SetTimeOut(query->timeout);

	for (uint32_t i = 0; i < query->paramCount; ++i) {
		cmd.PushParam(query->params[i]);
	}

	for (uint32_t i = 0; i < query->count; ++i) {
		CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
		if (unit != nullptr && unit->commandAI != nullptr) {
			unit->commandAI->GiveCommand(cmd);
		}
	}

	result->success = true;
}

static void NativeGiveOrderArrayToUnit(const GiveOrderArrayToUnitQuery* query, GiveOrderArrayToUnitResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->commandAI == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->commands == nullptr || query->commandCount == 0) {
		result->success = false;
		return;
	}

	// Give each command to the unit
	for (uint32_t i = 0; i < query->commandCount; ++i) {
		const NativeCommand& nativeCmd = query->commands[i];
		Command cmd(nativeCmd.cmdID, nativeCmd.options);
		if (nativeCmd.timeout > 0)
			cmd.SetTimeOut(nativeCmd.timeout);

		for (uint32_t j = 0; j < nativeCmd.paramCount; ++j) {
			cmd.PushParam(nativeCmd.params[j]);
		}

		unit->commandAI->GiveCommand(cmd);
	}

	result->success = true;
}

static void NativeGiveOrderArrayToUnitArray(const GiveOrderArrayToUnitArrayQuery* query, GiveOrderArrayToUnitArrayResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitsOrdered = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->unitIDs == nullptr || query->unitCount == 0) {
		return;
	}

	if (query->commands == nullptr || query->commandCount == 0) {
		return;
	}

	if (query->pairwise) {
		// Pairwise mode: unit[i] gets command[i]
		const uint32_t count = std::min(query->unitCount, query->commandCount);
		for (uint32_t i = 0; i < count; ++i) {
			CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
			if (unit != nullptr && unit->commandAI != nullptr) {
				const NativeCommand& nativeCmd = query->commands[i];
				Command cmd(nativeCmd.cmdID, nativeCmd.options);
				if (nativeCmd.timeout > 0)
					cmd.SetTimeOut(nativeCmd.timeout);

				for (uint32_t j = 0; j < nativeCmd.paramCount; ++j) {
					cmd.PushParam(nativeCmd.params[j]);
				}

				unit->commandAI->GiveCommand(cmd);
				result->unitsOrdered++;
			}
		}
	} else {
		// Broadcast mode: each unit gets all commands
		for (uint32_t i = 0; i < query->unitCount; ++i) {
			CUnit* unit = unitHandler.GetUnit(query->unitIDs[i]);
			if (unit != nullptr && unit->commandAI != nullptr) {
				for (uint32_t c = 0; c < query->commandCount; ++c) {
					const NativeCommand& nativeCmd = query->commands[c];
					Command cmd(nativeCmd.cmdID, nativeCmd.options);
					if (nativeCmd.timeout > 0)
						cmd.SetTimeOut(nativeCmd.timeout);

					for (uint32_t j = 0; j < nativeCmd.paramCount; ++j) {
						cmd.PushParam(nativeCmd.params[j]);
					}

					unit->commandAI->GiveCommand(cmd);
				}
				result->unitsOrdered++;
			}
		}
	}
}

static void NativeUnitFinishCommand(const UnitFinishCommandQuery* query, UnitFinishCommandResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unit->commandAI == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (!unit->commandAI->commandQue.empty()) {
		unit->commandAI->FinishCommand();
		result->success = true;
	}
}

static void NativeSetUnitHealth(const SetUnitHealthQuery* query, SetUnitHealthResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (!query->value.useAmounts) {
		unit->health = std::min(unit->maxHealth, query->value.health);
	} else {
		unit->health = std::min(unit->maxHealth, query->value.health);
		unit->captureProgress = query->value.capture;
		const float refValue = modInfo.paralyzeOnMaxHealth ? unit->maxHealth : unit->health;
		if ((unit->paralyzeDamage = std::max(0.0f, query->value.paralyze)) > refValue) {
			unit->SetStunned(true);
		} else if (query->value.paralyze < 0.0f) {
			unit->SetStunned(false);
		}
		if ((unit->buildProgress = query->value.build) >= 1.0f)
			unit->FinishedBuilding(false);
		else
			unit->TurnIntoNanoframe();
	}
	result->success = true;
}

static void NativeSetUnitMaxHealth(const SetUnitMaxHealthQuery* query, SetUnitMaxHealthResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->maxHealth = std::max(0.1f, query->maxHealth);
	unit->health = std::min(unit->health, unit->maxHealth);
	result->success = true;
}

static void NativeSetUnitExperience(const SetUnitExperienceQuery* query, SetUnitExperienceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->AddExperience(std::max(0.0f, query->experience) - unit->experience);

	result->success = true;
}

static void NativeAddUnitExperience(const AddUnitExperienceQuery* query, AddUnitExperienceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->AddExperience(query->experience);
	result->success = true;
}

static void NativeSetUnitNeutral(const SetUnitNeutralQuery* query, SetUnitNeutralResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->SetNeutral(query->neutral);
	result->success = true;
}

static void NativeSetUnitResourcing(const SetUnitResourcingQuery* query, SetUnitResourcingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const char* type = query->type;
	const float amount = query->amount * 0.5f;
	if (strcmp(type, "uum") == 0) {
		unit->resourcesUncondUse.metal = amount;
	} else if (strcmp(type, "uue") == 0) {
		unit->resourcesUncondUse.energy = amount;
	} else if (strcmp(type, "umm") == 0) {
		unit->resourcesUncondMake.metal = amount;
	} else if (strcmp(type, "ume") == 0) {
		unit->resourcesUncondMake.energy = amount;
	} else if (strcmp(type, "cum") == 0) {
		unit->resourcesCondUse.metal = amount;
	} else if (strcmp(type, "cue") == 0) {
		unit->resourcesCondUse.energy = amount;
	} else if (strcmp(type, "cmm") == 0) {
		unit->resourcesCondMake.metal = amount;
	} else if (strcmp(type, "cme") == 0) {
		unit->resourcesCondMake.energy = amount;
	} else {
		// Lua's SetUnitResourcing silently ignores an unknown resource key.
		return;
	}

	result->success = true;
}

static void NativeSetUnitMetalExtraction(const SetUnitMetalExtractionQuery* query, SetUnitMetalExtractionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CExtractorBuilding* extractor = dynamic_cast<CExtractorBuilding*>(unit);
	if (extractor != nullptr) {
		extractor->ResetExtraction();
		extractor->SetExtractionRangeAndDepth(query->range, query->depth);
		result->success = true;
	}
}

static void NativeSetUnitPosition(const SetUnitPositionQuery* query, SetUnitPositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	// Match Spring.SetUnitPosition: ForcedMove updates the blocking state
	// around the relocation, whereas Move only changes coordinates and leaves
	// a unit that was blocking at its old location logically blocked.
	unit->ForcedMove(pos);
	result->success = true;
}

static void NativeSetUnitVelocity(const SetUnitVelocityQuery* query, SetUnitVelocityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->speed.x = query->velocity.x;
	unit->speed.y = query->velocity.y;
	unit->speed.z = query->velocity.z;

	result->success = true;
}

static void NativeSetUnitRotation(const SetUnitRotationQuery* query, SetUnitRotationResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 rot(query->rotation.x, query->rotation.y, query->rotation.z);
	unit->SetDirVectorsEuler(rot);
	unit->UpdateMidAndAimPos();

	result->success = true;
}

static void NativeSetUnitPhysics(const SetUnitPhysicsQuery* query, SetUnitPhysicsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	float3 pos(query->pos.x, query->pos.y, query->pos.z);
	unit->Move(pos, false);

	unit->SetVelocityAndSpeed(float3(query->velocity.x, query->velocity.y, query->velocity.z));

	const float3 rot(query->rotation.x, query->rotation.y, query->rotation.z);
	CMatrix44f rotMatrix;
	rotMatrix.RotateEulerYXZ(-rot);

	unit->frontdir = rotMatrix.GetZ();
	unit->updir = rotMatrix.GetY();
	unit->rightdir = rotMatrix.GetX();
	unit->dragScales.x = std::clamp(query->drag.x, 0.0f, 1.0f);
	unit->dragScales.y = std::clamp(query->drag.y, 0.0f, 1.0f);
	unit->dragScales.z = std::clamp(query->drag.z, 0.0f, 1.0f);

	unit->UpdateMidAndAimPos();
	unit->ForcedMove(pos);

	result->success = true;
}

static void NativeAddUnitDamage(const AddUnitDamageQuery* query, AddUnitDamageResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CUnit* attacker = nullptr;
	if (query->attackerID >= 0) {
		attacker = unitHandler.GetUnit(query->attackerID);
	}

	const WeaponDef* weaponDef = nullptr;
	if (query->weaponDefID >= 0) {
		weaponDef = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	}

	DamageArray damages;
	if (weaponDef != nullptr) {
		damages = weaponDef->damages;
		damages = damages * (query->damage / damages.GetDefault());
	} else {
		damages.SetDefaultDamage(query->damage);
	}

	if (query->paralyzeTime > 0.0f) {
		damages.paralyzeDamageTime = query->paralyzeTime;
	}

	const float3 impulse(query->impulse.x, query->impulse.y, query->impulse.z);
	unit->DoDamage(damages, impulse, attacker, weaponDef != nullptr ? weaponDef->id : -1, -1);

	result->success = true;
}

static void NativeAddUnitImpulse(const AddUnitImpulseQuery* query, AddUnitImpulseResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 impulse(
		std::clamp(query->impulse.x, -MAX_EXPLOSION_IMPULSE, MAX_EXPLOSION_IMPULSE),
		std::clamp(query->impulse.y, -MAX_EXPLOSION_IMPULSE, MAX_EXPLOSION_IMPULSE),
		std::clamp(query->impulse.z, -MAX_EXPLOSION_IMPULSE, MAX_EXPLOSION_IMPULSE)
	);
	unit->ApplyImpulse(impulse);

	result->success = true;
}

static void NativeSetUnitCloak(const SetUnitCloakQuery* query, SetUnitCloakResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->wantCloak = query->cloak.useBoolean ? query->cloak.boolean : (query->cloak.number != 0.0f);
	if (query->cloakArg.useBoolean) {
		if (unit->unitDef != nullptr) {
			unit->decloakDistance = query->cloakArg.boolean ? math::fabsf(unit->unitDef->decloakDistance) : unit->unitDef->decloakDistance;
		}
	} else {
		unit->decloakDistance = query->cloakArg.number;
	}

	result->success = true;
}

static void NativeSetUnitStealth(const SetUnitStealthQuery* query, SetUnitStealthResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->stealth = query->stealth;
	result->success = true;
}

static void NativeSetUnitSonarStealth(const SetUnitSonarStealthQuery* query, SetUnitSonarStealthResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->sonarStealth = query->sonarStealth;
	result->success = true;
}

static void NativeSetUnitSeismicSignature(const SetUnitSeismicSignatureQuery* query, SetUnitSeismicSignatureResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->seismicSignature = query->seismicSignature;
	result->success = true;
}

static void NativeSetUnitArmored(const SetUnitArmoredQuery* query, SetUnitArmoredResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->armoredState = query->armoredState;
	unit->armoredMultiple = query->armoredMultiple;
	unit->curArmorMultiple = query->armoredState ? query->armoredMultiple : 1.0f;
	result->success = true;
}

static void NativeSetUnitBlocking(const SetUnitBlockingQuery* query, SetUnitBlockingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Update solid objects collidable bit
	if (query->options.solidObjects) {
		unit->SetCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	} else {
		unit->ClearCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	}

	// Update blocking bit (do this after changing the SO-bit so it is reversible)
	if (query->options.blocking) {
		unit->Block();
	} else {
		unit->UnBlock();
	}

	// Update other collidable bits
	unit->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_PROJECTILES, query->options.projectiles);
	unit->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS, query->options.quadMapRays);

	// Update other blocking properties
	unit->crushable = query->options.crushable;
	unit->blockEnemyPushing = query->options.blockEnemyPushing;
	unit->blockHeightChanges = query->options.blockHeightChanges;

	result->success = true;
}

static void NativeSetUnitMass(const SetUnitMassQuery* query, SetUnitMassResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->SetMass(query->mass);
	result->success = true;
}

static void NativeSetUnitLeavesGhost(const SetUnitLeavesGhostQuery* query, SetUnitLeavesGhostResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->SetLeavesGhost(query->options.leavesGhost, query->options.leaveDeadGhost);
	result->success = true;
}

static void NativeSetUnitAlwaysVisible(const SetUnitAlwaysVisibleQuery* query, SetUnitAlwaysVisibleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->alwaysVisible = query->alwaysVisible;
	result->success = true;
}

static void NativeSetUnitUseAirLos(const SetUnitUseAirLosQuery* query, SetUnitUseAirLosResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->useAirLos = query->useAirLos;
	result->success = true;
}

static void NativeGetUnitLeavesGhost(const GetUnitLeavesGhostQuery* query, GetUnitLeavesGhostResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->leavesGhost = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->leavesGhost = unit->leavesGhost;
}

static void NativeGetUnitPhysicalState(const GetUnitPhysicalStateQuery* query, GetUnitPhysicalStateResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->physicalState = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->physicalState = unit->physicalState;
}

static void NativeGetUnitFeatureSeparation(const GetUnitFeatureSeparationQuery* query, GetUnitFeatureSeparationResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->distance = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const float3 unitPos = unit->midPos;
	const float3 featurePos = feature->midPos;

	if (query->ignoreY) {
		result->distance = unitPos.distance2D(featurePos);
	} else {
		result->distance = unitPos.distance(featurePos);
	}
}

// Helper to convert NativeCommandDescription to SCommandDescription
static void ApplyNativeCommandDescription(const NativeCommandDescription* native, SCommandDescription& cd)
{
	cd.id = native->id;
	cd.type = native->type;
	cd.queueing = native->queueing;
	cd.hidden = native->hidden;
	cd.disabled = native->disabled;
	cd.showUnique = native->showUnique;
	cd.onlyTexture = native->onlyTexture;

	if (native->name != nullptr)
		cd.name = native->name;
	if (native->action != nullptr)
		cd.action = native->action;
	if (native->iconname != nullptr)
		cd.iconname = native->iconname;
	if (native->mouseicon != nullptr)
		cd.mouseicon = native->mouseicon;
	if (native->tooltip != nullptr)
		cd.tooltip = native->tooltip;

	if (native->params != nullptr && native->paramCount > 0) {
		cd.params.clear();
		for (uint32_t i = 0; i < native->paramCount; i++) {
			if (native->params[i] != nullptr)
				cd.params.push_back(native->params[i]);
		}
	}
}

static void NativeEditUnitCmdDesc(const EditUnitCmdDescQuery* query, EditUnitCmdDescResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const auto& cmdDescs = unit->commandAI->GetPossibleCommands();
	// Command-description indices are 1-based at the Lua/native API boundary.
	// Convert to the command AI's 0-based vector index before accessing it.
	const int cmdDescIndex = static_cast<int>(query->cmdDescIndex) - CMD_INDEX_OFFSET;
	if (cmdDescIndex < 0 || cmdDescIndex >= static_cast<int>(cmdDescs.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid command description index");
		return;
	}

	if (query->cmdDesc == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Command description is null");
		return;
	}

	// Make a copy of the existing command description
	SCommandDescription cmdDesc = *cmdDescs[cmdDescIndex];

	// Apply changes from native description
	ApplyNativeCommandDescription(query->cmdDesc, cmdDesc);

	// Update the command description
	unit->commandAI->UpdateCommandDescription(cmdDescIndex, std::move(cmdDesc));

	result->success = true;
}

static void NativeInsertUnitCmdDesc(const InsertUnitCmdDescQuery* query, InsertUnitCmdDescResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->cmdDesc == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Command description is null");
		return;
	}

	SCommandDescription cmdDesc;
	ApplyNativeCommandDescription(query->cmdDesc, cmdDesc);

	// -1 means append at end; other command-description indices are 1-based
	// at the Lua/native API boundary and 0-based inside the command AI.
	int cmdDescIdx = -1;
	if (query->cmdDescIndex >= 0) {
		cmdDescIdx = query->cmdDescIndex - CMD_INDEX_OFFSET;
		if (cmdDescIdx < 0 || cmdDescIdx > static_cast<int>(unit->commandAI->possibleCommands.size())) {
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid command description index");
			return;
		}
	}

	unit->commandAI->InsertCommandDescription(cmdDescIdx, std::move(cmdDesc));

	result->success = true;
}

static void NativeRemoveUnitCmdDesc(const RemoveUnitCmdDescQuery* query, RemoveUnitCmdDescResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// -1 means remove last; other command-description indices are 1-based
	// at the Lua/native API boundary and 0-based inside the command AI.
	int cmdDescIdx;
	if (query->cmdDescIndex < 0) {
		cmdDescIdx = static_cast<int>(unit->commandAI->possibleCommands.size()) - 1;
	} else {
		cmdDescIdx = query->cmdDescIndex - CMD_INDEX_OFFSET;
		if (cmdDescIdx < 0 || cmdDescIdx >= static_cast<int>(unit->commandAI->possibleCommands.size())) {
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid command description index");
			return;
		}
	}

	unit->commandAI->RemoveCommandDescription(cmdDescIdx);

	result->success = true;
}

static void NativeSetUnitCosts(const SetUnitCostsQuery* query, SetUnitCostsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->buildTime = std::max(1.0f, query->costs.buildTime);
	unit->cost.metal = std::max(1.0f, query->costs.metalCost);
	unit->cost.energy = std::max(1.0f, query->costs.energyCost);

	result->success = true;
}

static void NativeSetUnitBuildSpeed(const SetUnitBuildSpeedQuery* query, SetUnitBuildSpeedResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// INV_GAME_SPEED = 1.0f / GAME_SPEED
	constexpr float INV_GAME_SPEED = 1.0f / 30.0f;

	// Check if it's a factory
	CFactory* factory = dynamic_cast<CFactory*>(unit);
	if (factory != nullptr) {
		factory->buildSpeed = INV_GAME_SPEED * std::max(0.0f, query->buildSpeed);
		result->success = true;
		return;
	}

	// Check if it's a builder
	CBuilder* builder = dynamic_cast<CBuilder*>(unit);
	if (builder == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not a builder or factory");
		return;
	}

	builder->buildSpeed = INV_GAME_SPEED * std::max(0.0f, query->buildSpeed);

	if (query->repairSpeed >= 0.0f)
		builder->repairSpeed = INV_GAME_SPEED * std::max(0.0f, query->repairSpeed);
	if (query->reclaimSpeed >= 0.0f)
		builder->reclaimSpeed = INV_GAME_SPEED * std::max(0.0f, query->reclaimSpeed);
	if (query->resurrectSpeed >= 0.0f)
		builder->resurrectSpeed = INV_GAME_SPEED * std::max(0.0f, query->resurrectSpeed);
	if (query->captureSpeed >= 0.0f)
		builder->captureSpeed = INV_GAME_SPEED * std::max(0.0f, query->captureSpeed);
	if (query->terraformSpeed >= 0.0f)
		builder->terraformSpeed = INV_GAME_SPEED * std::max(0.0f, query->terraformSpeed);

	result->success = true;
}

static void NativeSetUnitCollisionVolumeData(const SetUnitCollisionVolumeDataQuery* query, SetUnitCollisionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 scales(query->scales.x, query->scales.y, query->scales.z);
	const float3 offsets(query->offsets.x, query->offsets.y, query->offsets.z);

	unit->collisionVolume.InitShape(
		scales,
		offsets,
		query->volumeType,
		query->testType,
		query->primaryAxis
	);

	result->success = true;
}

static void NativeSetUnitSelectionVolumeData(const SetUnitSelectionVolumeDataQuery* query, SetUnitSelectionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 scales(query->scales.x, query->scales.y, query->scales.z);
	const float3 offsets(query->offsets.x, query->offsets.y, query->offsets.z);

	unit->selectionVolume.InitShape(
		scales,
		offsets,
		query->volumeType,
		query->testType,
		query->primaryAxis
	);

	result->success = true;
}

static void NativeSetUnitPieceCollisionVolumeData(const SetUnitPieceCollisionVolumeDataQuery* query, SetUnitPieceCollisionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	LocalModel& localModel = unit->localModel;
	if (query->pieceIndex < 0 || static_cast<size_t>(query->pieceIndex) >= localModel.pieces.size()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
		return;
	}

	LocalModelPiece* lmp = &localModel.pieces[query->pieceIndex];

	if (query->enable) {
		const float3 scales(query->scales.x, query->scales.y, query->scales.z);
		const float3 offsets(query->offsets.x, query->offsets.y, query->offsets.z);

		// Create/initialize the collision volume for this piece
		lmp->GetCollisionVolume()->InitShape(
			scales,
			offsets,
			query->volumeType,
			CollisionVolume::COLVOL_HITTEST_CONT,
			query->primaryAxis
		);
		lmp->SetScriptVisible(!lmp->GetScriptVisible());
		lmp->SetScriptVisible(!lmp->GetScriptVisible());
	}

	result->success = true;
}

// Weapon numbers on the native surface follow the Lua convention
// (LUA_WEAPON_BASE_INDEX): 1 selects the unit's first weapon. Anything below 1
// resolves to a negative index, which each caller reads as "no specific weapon"
// the way Lua does. UnitsWeapons.cpp::GetLuaWeapon applies the same rule to the
// reader callouts.
static int LuaWeaponIndex(int32_t luaWeaponNum)
{
	return static_cast<int>(luaWeaponNum) - LUA_WEAPON_BASE_INDEX;
}

static void NativeSetUnitTarget(const SetUnitTargetQuery* query, SetUnitTargetResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (!query->target.isGroundTarget && query->target.targetID == -1) {
		unit->DropCurrentAttackTarget();
		result->success = true;
		return;
	}

	const float3 targetPos(query->target.pos.x, query->target.pos.y, query->target.pos.z);

	if (query->target.isGroundTarget) {
		const int weaponIndex = LuaWeaponIndex(query->weaponNum);
		if (weaponIndex < 0) {
			result->success = unit->AttackGround(targetPos, query->options.userTarget, query->options.manualFire);
		} else if (static_cast<size_t>(weaponIndex) < unit->weapons.size()) {
			SWeaponTarget trg(targetPos, query->options.userTarget);
			trg.isManualFire = query->options.manualFire;
			result->success = unit->weapons[weaponIndex]->Attack(trg);
		}
		return;
	}

	// Unit target
	CUnit* target = unitHandler.GetUnit(query->target.targetID);
	if (target == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid target unit");
		return;
	}

	if (target == unit) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit cannot attack itself");
		return;
	}

	const int weaponIndex = LuaWeaponIndex(query->weaponNum);
	if (weaponIndex < 0) {
		result->success = unit->AttackUnit(target, query->options.userTarget, query->options.manualFire);
	} else if (static_cast<size_t>(weaponIndex) < unit->weapons.size()) {
		SWeaponTarget trg(target, query->options.userTarget);
		trg.isManualFire = query->options.manualFire;
		result->success = unit->weapons[weaponIndex]->Attack(trg);
	}
}

static void NativeSetUnitShieldState(const SetUnitShieldStateQuery* query, SetUnitShieldStateResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CPlasmaRepulser* shield = static_cast<CPlasmaRepulser*>(unit->shieldWeapon);

	const int weaponIndex = LuaWeaponIndex(query->weaponNum);
	if (weaponIndex >= 0 && static_cast<size_t>(weaponIndex) < unit->weapons.size()) {
		shield = dynamic_cast<CPlasmaRepulser*>(unit->weapons[weaponIndex]);
	}

	if (shield == nullptr) {
		// Lua treats a unit without a matching shield as a no-op.
		result->success = true;
		return;
	}

	shield->SetEnabled(query->enabled);
	if (query->power >= 0.0f) {
		shield->SetCurPower(query->power);
	}

	result->success = true;
}

static void NativeSetUnitShieldRechargeDelay(const SetUnitShieldRechargeDelayQuery* query, SetUnitShieldRechargeDelayResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CPlasmaRepulser* shield = static_cast<CPlasmaRepulser*>(unit->shieldWeapon);

	const int weaponIndex = LuaWeaponIndex(query->weaponNum);
	if (weaponIndex >= 0 && static_cast<size_t>(weaponIndex) < unit->weapons.size()) {
		shield = dynamic_cast<CPlasmaRepulser*>(unit->weapons[weaponIndex]);
	}

	if (shield == nullptr) {
		// Lua treats a unit without a matching shield as a no-op.
		result->success = true;
		return;
	}

	if (query->rechargeDelay >= 0.0f) {
		const int frames = static_cast<int>(query->rechargeDelay * GAME_SPEED);
		shield->SetRechargeDelay(frames, true);
	} else {
		shield->SetRechargeDelay(shield->weaponDef->shieldRechargeDelay, false);
	}

	result->success = true;
}

static void NativeSetUnitFlanking(const SetUnitFlankingQuery* query, SetUnitFlankingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->type == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Missing flanking type");
		return;
	}

	switch (hashString(query->type)) {
		case hashString("mode"):
			unit->flankingBonusMode = static_cast<int>(query->args.x);
			break;
		case hashString("dir"):
			unit->flankingBonusDir = float3(query->args.x, query->args.y, query->args.z).Normalize();
			break;
		case hashString("moveFactor"):
			unit->flankingBonusMobilityAdd = query->args.x;
			break;
		case hashString("minDamage"): {
			const float maxDamage = unit->flankingBonusAvgDamage + unit->flankingBonusDifDamage;
			unit->flankingBonusAvgDamage = (maxDamage + query->args.x) * 0.5f;
			unit->flankingBonusDifDamage = (maxDamage - query->args.x) * 0.5f;
		} break;
		case hashString("maxDamage"): {
			const float minDamage = unit->flankingBonusAvgDamage - unit->flankingBonusDifDamage;
			unit->flankingBonusAvgDamage = (query->args.x + minDamage) * 0.5f;
			unit->flankingBonusDifDamage = (query->args.x - minDamage) * 0.5f;
		} break;
		default:
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown flanking type");
			return;
	}

	result->success = true;
}

static void NativeSetUnitMidAndAimPos(const SetUnitMidAndAimPosQuery* query, SetUnitMidAndAimPosResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 newMidPos(query->midPos.x, query->midPos.y, query->midPos.z);
	const float3 newAimPos(query->aimPos.x, query->aimPos.y, query->aimPos.z);

	unit->SetMidAndAimPos(newMidPos, newAimPos, query->setRelative);

	result->success = true;
}

static void NativeSetUnitRadiusAndHeight(const SetUnitRadiusAndHeightQuery* query, SetUnitRadiusAndHeightResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float newRadius = std::max(1.0f, query->radius);
	const float newHeight = std::max(1.0f, query->height);

	unit->SetRadiusAndHeight(newRadius, newHeight);
	result->success = true;
}

static void NativeSetUnitMoveGoal(const SetUnitMoveGoalQuery* query, SetUnitMoveGoalResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->moveType == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit has no move type");
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const float speed = (query->speed > 0.0f) ? query->speed : unit->moveType->GetMaxSpeed();

	if (query->raw) {
		unit->moveType->StartMovingRaw(pos, query->radius, speed);
	} else {
		unit->moveType->StartMoving(pos, query->radius, speed);
	}

	result->success = true;
}

static void NativeSetUnitLandGoal(const SetUnitLandGoalQuery* query, SetUnitLandGoalResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	AAirMoveType* amt = dynamic_cast<AAirMoveType*>(unit->moveType);
	if (amt == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not a flying unit");
		return;
	}

	const float3 landPos(query->pos.x, query->pos.y, query->pos.z);
	amt->LandAt(landPos, query->radiusSq);

	result->success = true;
}

static void NativeClearUnitGoal(const ClearUnitGoalQuery* query, ClearUnitGoalResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->moveType == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit has no move type");
		return;
	}

	unit->moveType->StopMoving(false, false, query->cancelRaw);
	result->success = true;
}

static void NativeSetUnitStockpile(const SetUnitStockpileQuery* query, SetUnitStockpileResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CWeapon* w = unit->stockpileWeapon;
	if (w == nullptr) {
		// Lua treats a unit without a stockpile weapon as a no-op.
		result->success = true;
		return;
	}

	if (query->stockpile >= 0) {
		w->numStockpiled = query->stockpile;
		unit->commandAI->UpdateStockpileIcon();
	}

	if (query->buildPercent >= 0.0f) {
		w->buildPercent = std::clamp(query->buildPercent, 0.0f, 1.0f);
	}

	result->success = true;
}

static void NativeSetUnitDirection(const SetUnitDirectionQuery* query, SetUnitDirectionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	float3 dir(query->frontDir.x, query->frontDir.y, query->frontDir.z);
	float3 rightDir(query->rightDir.x, query->rightDir.y, query->rightDir.z);
	dir.SafeNormalize();
	rightDir.SafeNormalize();

	if (math::fabsf(dir.SqLength() - 1.0f) > float3::cmp_eps() || math::fabsf(rightDir.SqLength() - 1.0f) > float3::cmp_eps()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid direction vector");
		return;
	}

	unit->ForcedSpin(dir, rightDir);
	result->success = true;
}

static void NativeUnitAttach(const UnitAttachQuery* query, UnitAttachResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* transporter = unitHandler.GetUnit(query->transporterID);
	if (transporter == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid transporter unit ID");
		return;
	}

	CUnit* transportee = unitHandler.GetUnit(query->transporteeID);
	if (transportee == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid transportee unit ID");
		return;
	}

	if (transporter == transportee) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Cannot attach unit to itself");
		return;
	}

	int piece = query->pieceNum;
	const auto& pieces = transporter->localModel.pieces;

	if (piece >= static_cast<int>(pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece number");
		return;
	}

	if (piece >= 0) {
		piece = pieces[piece].scriptPieceIndex;
	}

	transporter->AttachUnit(transportee, piece, !transporter->unitDef->IsTransportUnit());
	result->success = true;
}

static void NativeUnitDetach(const UnitDetachQuery* query, UnitDetachResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* transportee = unitHandler.GetUnit(query->transporteeID);
	if (transportee == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CUnit* transporter = transportee->GetTransporter();
	if (transporter == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not being transported");
		return;
	}

	transporter->DetachUnit(transportee);
	result->success = true;
}

static void NativeUnitDetachFromAir(const UnitDetachFromAirQuery* query, UnitDetachFromAirResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* transportee = unitHandler.GetUnit(query->transporteeID);
	if (transportee == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CUnit* transporter = transportee->GetTransporter();
	if (transporter == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not being transported");
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);

	transporter->DetachUnitFromAir(transportee, pos);
	result->success = true;
}

static void NativeSetUnitLoadingTransport(const SetUnitLoadingTransportQuery* query, SetUnitLoadingTransportResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->transportID < 0) {
		unit->loadingTransportId = -1;
		result->success = true;
		return;
	}

	CUnit* transport = unitHandler.GetUnit(query->transportID);
	if (transport == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid transport unit ID");
		return;
	}

	unit->loadingTransportId = transport->id;
	result->success = true;
}

static void NativeSetUnitCrashing(const SetUnitCrashingQuery* query, SetUnitCrashingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->stateChanged = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	AAirMoveType* amt = dynamic_cast<AAirMoveType*>(unit->moveType);
	if (amt == nullptr) {
		// Lua returns false for non-air units instead of raising an error.
		return;
	}

	const AAirMoveType::AircraftState aircraftState = amt->aircraftState;

	// For simplicity, can only set a non-landed aircraft to start crashing,
	// or a crashing aircraft to start flying
	if (query->wantCrash && (aircraftState != AAirMoveType::AIRCRAFT_LANDED)) {
		amt->SetState(AAirMoveType::AIRCRAFT_CRASHING);
	}

	if (!query->wantCrash && (aircraftState == AAirMoveType::AIRCRAFT_CRASHING)) {
		amt->SetState(AAirMoveType::AIRCRAFT_FLYING);
	}

	result->stateChanged = (amt->aircraftState != aircraftState);
}

// Helper function to set a single weapon state property
static bool SetSingleWeaponState(CWeapon* weapon, const char* key, float value)
{
	switch (hashString(key)) {
		case hashString("reloadState"):
		case hashString("reloadFrame"):
			weapon->reloadStatus = static_cast<int>(value);
			break;
		case hashString("reloadTime"):
			weapon->reloadTime = std::max(1, static_cast<int>(value * GAME_SPEED));
			break;
		case hashString("reaimTime"):
			weapon->reaimTime = std::max(1, static_cast<int>(value));
			break;
		case hashString("accuracy"):
			weapon->accuracyError = value;
			break;
		case hashString("sprayAngle"):
			weapon->sprayAngle = value;
			break;
		case hashString("range"):
			weapon->UpdateRange(value);
			break;
		case hashString("projectileSpeed"):
			weapon->UpdateProjectileSpeed(value);
			break;
		case hashString("autoTargetRangeBoost"):
			weapon->autoTargetRangeBoost = std::max(0.0f, value);
			break;
		case hashString("burst"):
			weapon->salvoSize = static_cast<int>(value);
			break;
		case hashString("burstRate"):
			weapon->salvoDelay = static_cast<int>(value * GAME_SPEED);
			break;
		case hashString("windup"):
			weapon->salvoWindup = static_cast<int>(value * GAME_SPEED);
			break;
		case hashString("projectiles"):
			weapon->projectilesPerShot = static_cast<int>(value);
			break;
		case hashString("salvoLeft"):
			weapon->salvoLeft = static_cast<int>(value);
			break;
		case hashString("nextSalvo"):
			weapon->nextSalvo = static_cast<int>(value);
			break;
		case hashString("aimReady"):
			weapon->angleGood = (value != 0.0f);
			break;
		case hashString("forceAim"):
			weapon->lastAimedFrame -= static_cast<int>(value > 0.0f ? value : weapon->reaimTime);
			break;
		case hashString("avoidFlags"):
			weapon->avoidFlags = static_cast<int>(value);
			break;
		case hashString("collisionFlags"):
			weapon->collisionFlags = static_cast<int>(value);
			break;
		case hashString("ttl"):
			weapon->ttl = static_cast<int>(value * GAME_SPEED);
			break;
		default:
			return false;
	}
	return true;
}

static void NativeSetUnitWeaponState(const SetUnitWeaponStateQuery* query, SetUnitWeaponStateResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int weaponIndex = LuaWeaponIndex(query->weaponNum);
	if (weaponIndex < 0 || static_cast<size_t>(weaponIndex) >= unit->weapons.size()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid weapon number");
		return;
	}

	CWeapon* weapon = unit->weapons[weaponIndex];
	if (query->key == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Key is null");
		return;
	}

	result->success = SetSingleWeaponState(weapon, query->key, query->value);
	if (!result->success) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown weapon state key");
	}
}

static void NativeUnitWeaponFire(const UnitWeaponFireQuery* query, UnitWeaponFireResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int weaponIndex = LuaWeaponIndex(query->weaponNum);
	if (weaponIndex < 0 || static_cast<size_t>(weaponIndex) >= unit->weapons.size()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid weapon number");
		return;
	}

	unit->weapons[weaponIndex]->Fire(false);
	result->success = true;
}

static void NativeUnitWeaponHoldFire(const UnitWeaponHoldFireQuery* query, UnitWeaponHoldFireResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int weaponIndex = LuaWeaponIndex(query->weaponNum);
	if (weaponIndex < 0 || static_cast<size_t>(weaponIndex) >= unit->weapons.size()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid weapon number");
		return;
	}

	unit->weapons[weaponIndex]->DropCurrentTarget();
	result->success = true;
}

static void NativeSetUnitUseWeapons(const SetUnitUseWeaponsQuery* query, SetUnitUseWeaponsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->forceUseWeapons = query->options.forceUseWeapons;
	unit->allowUseWeapons = query->options.allowUseWeapons;

	result->success = true;
}

static void NativeSetUnitMaxRange(const SetUnitMaxRangeQuery* query, SetUnitMaxRangeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->maxRange = std::max(0.0f, query->maxRange);
	result->success = true;
}

static void NativeSetUnitPhysicalStateBit(const SetUnitPhysicalStateBitQuery* query, SetUnitPhysicalStateBitResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->SetPhysicalStateBit(query->stateBit);
	result->success = true;
}

static void NativeSetUnitPosErrorParams(const SetUnitPosErrorParamsQuery* query, SetUnitPosErrorParamsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->posErrorVector.x = query->posErrorVector.x;
	unit->posErrorVector.y = query->posErrorVector.y;
	unit->posErrorVector.z = query->posErrorVector.z;
	unit->posErrorDelta.x = query->posErrorDelta.x;
	unit->posErrorDelta.y = query->posErrorDelta.y;
	unit->posErrorDelta.z = query->posErrorDelta.z;
	unit->nextPosErrorUpdate = query->nextPosErrorUpdate;

	if (query->allyTeamID >= 0 && query->allyTeamID < teamHandler.ActiveAllyTeams()) {
		unit->SetPosErrorBit(query->allyTeamID, query->setPosErrorBit);
	}

	result->success = true;
}

// Helper function to set a single damage property
static bool SetSingleDynDamagesKey(DynDamageArray* damages, const char* key, float value)
{
	switch (hashString(key)) {
		case hashString("paralyzeDamageTime"):
			damages->paralyzeDamageTime = std::max(static_cast<int>(value), 0);
			break;
		case hashString("impulseFactor"):
			damages->impulseFactor = value;
			break;
		case hashString("impulseBoost"):
			damages->impulseBoost = value;
			break;
		case hashString("craterMult"):
			damages->craterMult = value;
			break;
		case hashString("craterBoost"):
			damages->craterBoost = value;
			break;
		case hashString("dynDamageExp"):
			damages->dynDamageExp = value;
			break;
		case hashString("dynDamageMin"):
			damages->dynDamageMin = value;
			break;
		case hashString("dynDamageRange"):
			damages->dynDamageRange = value;
			break;
		case hashString("dynDamageInverted"):
			damages->dynDamageInverted = (value != 0.0f);
			break;
		case hashString("craterAreaOfEffect"):
			damages->craterAreaOfEffect = value;
			break;
		case hashString("damageAreaOfEffect"):
			damages->damageAreaOfEffect = value;
			break;
		case hashString("edgeEffectiveness"):
			damages->edgeEffectiveness = std::min(value, 1.0f);
			break;
		case hashString("explosionSpeed"):
			damages->explosionSpeed = value;
			break;
		default:
			return false;
	}
	return true;
}

static void NativeSetUnitWeaponDamages(const SetUnitWeaponDamagesQuery* query, SetUnitWeaponDamagesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	DynDamageArray* damages = nullptr;

	if (query->weaponNum == -1) {
		// "explode"
		damages = DynDamageArray::GetMutable(unit->deathExpDamages);
	} else if (query->weaponNum == -2) {
		// "selfDestruct"
		damages = DynDamageArray::GetMutable(unit->selfdExpDamages);
	} else {
		const int weaponIndex = LuaWeaponIndex(query->weaponNum);
		if (weaponIndex < 0 || static_cast<size_t>(weaponIndex) >= unit->weapons.size()) {
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid weapon number");
			return;
		}
		damages = DynDamageArray::GetMutable(unit->weapons[weaponIndex]->damages);
	}

	if (damages == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Could not get damage array");
		return;
	}

	if (query->damageKey == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Damage key is null");
		return;
	}

	// Check if it's an armor type index (numeric string)
	char* endptr;
	long armType = strtol(query->damageKey, &endptr, 10);
	if (*endptr == '\0' && armType >= 0) {
		// It's a numeric armor type
		if (static_cast<unsigned>(armType) < damages->GetNumTypes()) {
			damages->Set(static_cast<unsigned>(armType), query->damageValue);
			result->success = true;
		} else {
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid armor type index");
		}
		return;
	}

	// It's a named property
	result->success = SetSingleDynDamagesKey(damages, query->damageKey, query->damageValue);
	if (!result->success) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown damage key");
	}
}

static void NativeForceUnitCollisionUpdate(const ForceUnitCollisionUpdateQuery* query, ForceUnitCollisionUpdateResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (unit->moveType == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit has no move type");
		return;
	}

	unit->moveType->UpdateCollisionMap(true);
	result->success = true;
}

static void NativeSetUnitHeading(const SetUnitHeadingQuery* query, SetUnitHeadingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->heading = static_cast<short>(query->heading);
	unit->SetFacingFromHeading();
	unit->UpdateMidAndAimPos();

	result->success = true;
}

static void NativeSetUnitHeadingAndUpDir(const SetUnitHeadingAndUpDirQuery* query, SetUnitHeadingAndUpDirResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 upDir = float3(query->upDir.x, query->upDir.y, query->upDir.z).SafeNormalize();
	if (std::fabs(upDir.SqLength() - 1.0f) > float3::cmp_eps()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid up direction (must be non-zero)");
		return;
	}

	unit->heading = static_cast<short>(query->heading);
	unit->UpdateDirVectors(upDir);
	unit->SetFacingFromHeading();
	unit->UpdateMidAndAimPos();

	result->success = true;
}

static void NativeAddObjectDecal(const AddObjectDecalQuery* query, AddObjectDecalResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	groundDecals->AddSolidObject(unit);
	result->success = true;
}

static void NativeRemoveObjectDecal(const RemoveObjectDecalQuery* query, RemoveObjectDecalResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	groundDecals->ForceRemoveSolidObject(unit);
	result->success = true;
}

static void NativeSetUnitBuildeeRadius(const SetUnitBuildeeRadiusQuery* query, SetUnitBuildeeRadiusResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->buildeeRadius = std::max(0.0f, query->radius);
	result->success = true;
}

static void NativeSetUnitSensorRadius(const SetUnitSensorRadiusQuery* query, SetUnitSensorRadiusResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->newRadius = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->sensorType == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Sensor type is null");
		return;
	}

	const int radius = std::clamp(query->radius, 0, MAX_UNIT_SENSOR_RADIUS);

	switch (hashString(query->sensorType)) {
		case hashString("los"):
			unit->ChangeLos(unit->realLosRadius = radius, unit->realAirLosRadius);
			result->newRadius = unit->losRadius;
			break;
		case hashString("airLos"):
			unit->ChangeLos(unit->realLosRadius, unit->realAirLosRadius = radius);
			result->newRadius = unit->airLosRadius;
			break;
		case hashString("radar"):
			result->newRadius = unit->radarRadius = radius;
			break;
		case hashString("sonar"):
			result->newRadius = unit->sonarRadius = radius;
			break;
		case hashString("seismic"):
			result->newRadius = unit->seismicRadius = radius;
			break;
		case hashString("radarJammer"):
			result->newRadius = unit->jammerRadius = radius;
			break;
		case hashString("sonarJammer"):
			result->newRadius = unit->sonarJamRadius = radius;
			break;
		default:
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown sensor type");
			return;
	}
}

static void NativeSetUnitHarvestStorage(const SetUnitHarvestStorageQuery* query, SetUnitHarvestStorageResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->harvested.metal = query->storedMetal;
	unit->harvestStorage.metal = query->maxStoredMetal;
	unit->harvested.energy = query->storedEnergy;
	unit->harvestStorage.energy = query->maxStoredEnergy;

	result->success = true;
}

static void NativeSetUnitBuildParams(const SetUnitBuildParamsQuery* query, SetUnitBuildParamsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CBuilder* builder = dynamic_cast<CBuilder*>(unit);
	if (builder == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not a builder");
		return;
	}

	if (query->paramName == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Param name is null");
		return;
	}

	switch (hashString(query->paramName)) {
		case hashString("buildRange"):
		case hashString("buildDistance"):
			builder->buildDistance = query->value.number;
			break;
		case hashString("buildRange3D"):
			builder->range3D = query->value.useBoolean ? query->value.boolean : (query->value.number != 0.0f);
			break;
		default:
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown build param");
			return;
	}

	result->success = true;
}

static void NativeSetUnitLosMask(const SetUnitLosMaskQuery* query, SetUnitLosMaskResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}

	const unsigned char losStatus = unit->losStatus[query->allyTeamID];
	const unsigned char newMask = query->losMask & 0x0F;
	const unsigned char state = (newMask << LOS_MASK_SHIFT) | (losStatus & 0x0F);

	unit->losStatus[query->allyTeamID] = state;
	unit->SetLosStatus(query->allyTeamID, unit->CalcLosStatus(query->allyTeamID));

	result->success = true;
}

static void NativeSetUnitLosState(const SetUnitLosStateQuery* query, SetUnitLosStateResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}

	const unsigned char losStatus = unit->losStatus[query->allyTeamID];
	const unsigned char newState = query->losState & 0x0F;

	unit->SetLosStatus(query->allyTeamID, (losStatus & 0xF0) | newState);

	result->success = true;
}

static void NativeSetUnitStorage(const SetUnitStorageQuery* query, SetUnitStorageResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	SResourcePack newStorage = unit->storage;
	if (query->resource == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	switch (query->resource[0]) {
		case 'm': newStorage.metal = query->amount; break;
		case 'e': newStorage.energy = query->amount; break;
		default:
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
	}
	unit->SetStorage(newStorage);

	result->success = true;
}

static void NativeSetUnitTooltip(const SetUnitTooltipQuery* query, SetUnitTooltipResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->tooltip != nullptr) {
		unitToolTipMap.Set(unit->id, std::string(query->tooltip));
	} else {
		unitToolTipMap.Set(unit->id, "");
	}

	result->success = true;
}

static void NativeSetFactoryBuggerOff(const SetFactoryBuggerOffQuery* query, SetFactoryBuggerOffResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->perform = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CFactory* factory = dynamic_cast<CFactory*>(unit);
	if (factory == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not a factory");
		return;
	}

	factory->boPerform = query->options.perform;
	factory->boOffset = query->options.offset;
	factory->boRadius = query->options.radius;
	factory->boRelHeading = query->options.relHeading;
	factory->boSherical = query->options.spherical;
	factory->boForced = query->options.forced;

	result->perform = factory->boPerform;
}

static void NativeBuggerOff(const BuggerOffQuery* query, BuggerOffResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	float3 pos(query->pos.x, query->pos.y, query->pos.z);

	CUnit* excludeUnit = nullptr;
	if (query->options.excludeUnitID >= 0) {
		excludeUnit = unitHandler.GetUnit(query->options.excludeUnitID);
	}

	if (query->excludeUnitDefIDs != nullptr && query->excludeUnitDefCount > 0) {
		std::vector<const UnitDef*> exclDefs;
		exclDefs.reserve(query->excludeUnitDefCount);
		for (uint32_t i = 0; i < query->excludeUnitDefCount; ++i) {
			const int defID = query->excludeUnitDefIDs[i];
			if (unitDefHandler->IsValidUnitDefID(defID)) {
				exclDefs.push_back(unitDefHandler->GetUnitDefByID(defID));
			}
		}
		CGameHelper::BuggerOff(pos, query->radius, query->options.spherical, query->options.forced, query->teamID, excludeUnit, exclDefs);
	} else {
		CGameHelper::BuggerOff(pos, query->radius, query->options.spherical, query->options.forced, query->teamID, excludeUnit);
	}
	result->success = true;
}

static void NativeAddUnitSeismicPing(const AddUnitSeismicPingQuery* query, AddUnitSeismicPingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->DoSeismicPing(query->pingSize);
	result->success = true;
}

static void NativeAddUnitResource(const AddUnitResourceQuery* query, AddUnitResourceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->resourceType == nullptr || query->resourceType[0] == '\0') {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Resource type is null or empty");
		return;
	}

	const float amount = std::max(0.0f, query->amount);
	switch (query->resourceType[0]) {
		case 'm':
			unit->AddMetal(amount);
			result->success = true;
			break;
		case 'e':
			unit->AddEnergy(amount);
			result->success = true;
			break;
		default:
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown resource type");
			break;
	}
}

static void NativeUseUnitResource(const UseUnitResourceQuery* query, UseUnitResourceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (query->resourceType == nullptr || query->resourceType[0] == '\0') {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Resource type is null or empty");
		return;
	}

	const float amount = std::max(0.0f, query->amount);
	switch (query->resourceType[0]) {
		case 'm':
			result->success = unit->UseMetal(amount);
			break;
		case 'e':
			result->success = unit->UseEnergy(amount);
			break;
		default:
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown resource type");
			break;
	}
}

static void NativeSetUnitPieceVisible(const SetUnitPieceVisibleQuery* query, SetUnitPieceVisibleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	LocalModel& localModel = unit->localModel;
	if (query->pieceIndex < 0 || query->pieceIndex >= static_cast<int>(localModel.pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
		return;
	}

	localModel.pieces[query->pieceIndex].SetScriptVisible(query->visible);
	result->success = true;
}

static void NativeSetUnitPieceParent(const SetUnitPieceParentQuery* query, SetUnitPieceParentResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	LocalModel& localModel = unit->localModel;
	if (query->childPieceIndex < 0 || query->childPieceIndex >= static_cast<int>(localModel.pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid child piece index");
		return;
	}
	if (query->parentPieceIndex < 0 || query->parentPieceIndex >= static_cast<int>(localModel.pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid parent piece index");
		return;
	}

	LocalModelPiece* childPiece = &localModel.pieces[query->childPieceIndex];
	LocalModelPiece* parentPiece = &localModel.pieces[query->parentPieceIndex];

	// Cannot change the root piece's parent
	if (childPiece == localModel.GetRoot()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Cannot change root piece's parent");
		return;
	}

	childPiece->parent->RemoveChild(childPiece);
	childPiece->SetParent(parentPiece);
	parentPiece->AddChild(childPiece);
	result->success = true;
}

static void NativeSetUnitPieceMatrix(const SetUnitPieceMatrixQuery* query, SetUnitPieceMatrixResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->blockScriptAnims = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	LocalModel& localModel = unit->localModel;
	if (query->pieceIndex < 0 || query->pieceIndex >= static_cast<int>(localModel.pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
		return;
	}

	LocalModelPiece* lmp = &localModel.pieces[query->pieceIndex];

	CMatrix44f mat;
	for (int i = 0; i < 16; ++i) {
		mat.m[i] = query->matrix[i];
	}

	if (lmp->SetPieceSpaceMatrix(mat))
		lmp->SetDirty();

	result->blockScriptAnims = lmp->blockScriptAnims;
}

static void NativeSetUnitNanoPieces(const SetUnitNanoPiecesQuery* query, SetUnitNanoPiecesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	NanoPieceCache* pieceCache = nullptr;
	std::vector<int>* nanoPieces = nullptr;

	// Check if Builder
	CBuilder* builder = dynamic_cast<CBuilder*>(unit);
	if (builder != nullptr) {
		pieceCache = &builder->GetNanoPieceCache();
		nanoPieces = &pieceCache->GetNanoPieces();
	}

	// Check if Factory
	CFactory* factory = dynamic_cast<CFactory*>(unit);
	if (factory != nullptr) {
		pieceCache = &factory->GetNanoPieceCache();
		nanoPieces = &pieceCache->GetNanoPieces();
	}

	if (nanoPieces == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not a builder or factory");
		return;
	}

	nanoPieces->clear();
	pieceCache->StopPolling();

	for (uint32_t i = 0; i < query->pieceCount; ++i) {
		const int modelPieceNum = query->pieceIndices[i];  // Already 0-indexed

		if (unit->localModel.HasPiece(modelPieceNum)) {
			nanoPieces->push_back(modelPieceNum);
		} else {
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
			return;
		}
	}

	result->success = true;
}

static const UnitControlApi UNIT_CONTROL_API = {
	.CreateUnit = NativeCreateUnit,
	.DestroyUnit = NativeDestroyUnit,
	.TransferUnit = NativeTransferUnit,
	.GiveOrderToUnit = NativeGiveOrderToUnit,
	.GiveOrderToUnitArray = NativeGiveOrderToUnitArray,
	.GiveOrderArrayToUnit = NativeGiveOrderArrayToUnit,
	.GiveOrderArrayToUnitArray = NativeGiveOrderArrayToUnitArray,
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
	.AddUnitImpulse = NativeAddUnitImpulse,
	.SetUnitCloak = NativeSetUnitCloak,
	.SetUnitStealth = NativeSetUnitStealth,
	.SetUnitSonarStealth = NativeSetUnitSonarStealth,
	.SetUnitSeismicSignature = NativeSetUnitSeismicSignature,
	.SetUnitArmored = NativeSetUnitArmored,
	.SetUnitBlocking = NativeSetUnitBlocking,
	.SetUnitMass = NativeSetUnitMass,
	.SetUnitLeavesGhost = NativeSetUnitLeavesGhost,
	.SetUnitAlwaysVisible = NativeSetUnitAlwaysVisible,
	.SetUnitUseAirLos = NativeSetUnitUseAirLos,
	.GetUnitLeavesGhost = NativeGetUnitLeavesGhost,
	.GetUnitPhysicalState = NativeGetUnitPhysicalState,
	.GetUnitFeatureSeparation = NativeGetUnitFeatureSeparation,
	.EditUnitCmdDesc = NativeEditUnitCmdDesc,
	.InsertUnitCmdDesc = NativeInsertUnitCmdDesc,
	.RemoveUnitCmdDesc = NativeRemoveUnitCmdDesc,
	.SetUnitCosts = NativeSetUnitCosts,
	.SetUnitBuildSpeed = NativeSetUnitBuildSpeed,
	.SetUnitCollisionVolumeData = NativeSetUnitCollisionVolumeData,
	.SetUnitSelectionVolumeData = NativeSetUnitSelectionVolumeData,
	.SetUnitPieceCollisionVolumeData = NativeSetUnitPieceCollisionVolumeData,
	.SetUnitTarget = NativeSetUnitTarget,
	.SetUnitShieldState = NativeSetUnitShieldState,
	.SetUnitShieldRechargeDelay = NativeSetUnitShieldRechargeDelay,
	.SetUnitFlanking = NativeSetUnitFlanking,
	.SetUnitMidAndAimPos = NativeSetUnitMidAndAimPos,
	.SetUnitRadiusAndHeight = NativeSetUnitRadiusAndHeight,
	.SetUnitMoveGoal = NativeSetUnitMoveGoal,
	.SetUnitLandGoal = NativeSetUnitLandGoal,
	.ClearUnitGoal = NativeClearUnitGoal,
	.SetUnitStockpile = NativeSetUnitStockpile,
	.SetUnitDirection = NativeSetUnitDirection,
	.UnitAttach = NativeUnitAttach,
	.UnitDetach = NativeUnitDetach,
	.UnitDetachFromAir = NativeUnitDetachFromAir,
	.SetUnitLoadingTransport = NativeSetUnitLoadingTransport,
	.SetUnitCrashing = NativeSetUnitCrashing,
	.SetUnitWeaponState = NativeSetUnitWeaponState,
	.UnitWeaponFire = NativeUnitWeaponFire,
	.UnitWeaponHoldFire = NativeUnitWeaponHoldFire,
	.SetUnitUseWeapons = NativeSetUnitUseWeapons,
	.SetUnitMaxRange = NativeSetUnitMaxRange,
	.SetUnitPhysicalStateBit = NativeSetUnitPhysicalStateBit,
	.SetUnitPosErrorParams = NativeSetUnitPosErrorParams,
	.SetUnitWeaponDamages = NativeSetUnitWeaponDamages,
	.ForceUnitCollisionUpdate = NativeForceUnitCollisionUpdate,
	.SetUnitHeading = NativeSetUnitHeading,
	.SetUnitHeadingAndUpDir = NativeSetUnitHeadingAndUpDir,
	.AddObjectDecal = NativeAddObjectDecal,
	.RemoveObjectDecal = NativeRemoveObjectDecal,
	.SetUnitBuildeeRadius = NativeSetUnitBuildeeRadius,
	.SetUnitSensorRadius = NativeSetUnitSensorRadius,
	.SetUnitHarvestStorage = NativeSetUnitHarvestStorage,
	.SetUnitBuildParams = NativeSetUnitBuildParams,
	.SetUnitLosMask = NativeSetUnitLosMask,
	.SetUnitLosState = NativeSetUnitLosState,
	.SetUnitStorage = NativeSetUnitStorage,
	.SetUnitTooltip = NativeSetUnitTooltip,
	.SetFactoryBuggerOff = NativeSetFactoryBuggerOff,
	.BuggerOff = NativeBuggerOff,
	.AddUnitSeismicPing = NativeAddUnitSeismicPing,
	.AddUnitResource = NativeAddUnitResource,
	.UseUnitResource = NativeUseUnitResource,
	.SetUnitPieceVisible = NativeSetUnitPieceVisible,
	.SetUnitPieceParent = NativeSetUnitPieceParent,
	.SetUnitPieceMatrix = NativeSetUnitPieceMatrix,
	.SetUnitNanoPieces = NativeSetUnitNanoPieces
};

// ============================================================================
// Feature Control Implementation
// ============================================================================

static void NativeCreateFeature(const CreateFeatureQuery* query, CreateFeatureResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->featureID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* featureDef = (query->featureDef.id >= 0)
		? featureDefHandler->GetFeatureDefByID(query->featureDef.id)
		: ((query->featureDef.name != nullptr) ? featureDefHandler->GetFeatureDef(query->featureDef.name) : nullptr);
	if (featureDef == nullptr) {
		result->error = &INVALID_FEATUREDEF_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const int teamID = (teamHandler.IsValidTeam(query->teamID)) ? query->teamID : -1;

	FeatureLoadParams params;
	params.parentObj = nullptr;
	params.featureDef = featureDef;
	params.pos = pos;
	params.speed = ZeroVector;
	params.featureID = query->featureID;
	params.teamID = teamID;
	params.allyTeamID = (teamID < 0) ? -1 : teamHandler.AllyTeam(teamID);
	params.heading = query->facing;
	params.facing = query->facing;
	params.wreckLevels = 0;
	params.smokeTime = 0;

	CFeature* feature = featureHandler.LoadFeature(params);

	if (feature != nullptr) {
		result->featureID = feature->id;
	}
}

static void NativeDestroyFeature(const DestroyFeatureQuery* query, DestroyFeatureResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	featureHandler.DeleteFeature(feature);
	result->success = true;
}

static void NativeTransferFeature(const TransferFeatureQuery* query, TransferFeatureResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->newTeamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	feature->ChangeTeam(query->newTeamID);
	result->success = true;
}

static void NativeSetFeatureHealth(const SetFeatureHealthQuery* query, SetFeatureHealthResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->health = std::min(query->health, feature->maxHealth);
	if (feature->health <= 0.0f && query->checkDestruction) {
		featureHandler.DeleteFeature(feature);
	}
	result->success = true;
}

static void NativeSetFeaturePosition(const SetFeaturePositionQuery* query, SetFeaturePositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	float3 pos(query->pos.x, query->pos.y, query->pos.z);
	if (query->snapToGround) {
		pos.y = CGround::GetHeightReal(pos.x, pos.z);
	}
	feature->ForcedMove(pos);
	result->success = true;
}

static void NativeSetFeatureDirection(const SetFeatureDirectionQuery* query, SetFeatureDirectionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	float3 dir(query->frontDir.x, query->frontDir.y, query->frontDir.z);
	float3 rightDir(query->rightDir.x, query->rightDir.y, query->rightDir.z);
	dir.SafeNormalize();
	rightDir.SafeNormalize();

	// Use ForcedSpin (as NativeSetUnitDirection and Lua's SetFeatureDirection do)
	// so updir and heading are recomputed from front/right. Assigning frontdir/
	// rightdir directly leaves a stale updir — an inconsistent basis the feature
	// fights every physical update, making it spin until something re-grounds it.
	feature->ForcedSpin(dir, rightDir);

	result->success = true;
}

static void NativeSetFeatureVelocity(const SetFeatureVelocityQuery* query, SetFeatureVelocityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	float3 speed;
	speed.x = std::clamp(query->velocity.x, -MAX_UNIT_SPEED, MAX_UNIT_SPEED);
	speed.y = std::clamp(query->velocity.y, -MAX_UNIT_SPEED, MAX_UNIT_SPEED);
	speed.z = std::clamp(query->velocity.z, -MAX_UNIT_SPEED, MAX_UNIT_SPEED);
	feature->SetVelocityAndSpeed(speed);

	result->success = true;
}

static void NativeSetFeatureResources(const SetFeatureResourcesQuery* query, SetFeatureResourcesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->defResources.metal = std::max(0.0f, query->featureDefMetal);
	feature->defResources.energy = std::max(0.0f, query->featureDefEnergy);
	feature->resources.metal = std::clamp(query->metal, 0.0f, feature->defResources.metal);
	feature->resources.energy = std::clamp(query->energy, 0.0f, feature->defResources.energy);
	feature->reclaimTime = std::clamp(query->reclaimTime, 1.0f, 1000000.0f);
	feature->reclaimLeft = std::clamp(query->reclaimLeft, 0.0f, 1.0f);

	result->success = true;
}

static void NativeAddFeatureDamage(const AddFeatureDamageQuery* query, AddFeatureDamageResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	CUnit* attacker = nullptr;
	if (query->attackerID >= 0) {
		attacker = unitHandler.GetUnit(query->attackerID);
	}

	const WeaponDef* weaponDef = nullptr;
	if (query->weaponDefID >= 0) {
		weaponDef = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	}

	DamageArray damages;
	if (weaponDef != nullptr) {
		damages = weaponDef->damages;
		damages = damages * (query->damage / damages.GetDefault());
	} else {
		damages.SetDefaultDamage(query->damage);
	}

	if (query->paralyzeTime > 0.0f) {
		damages.paralyzeDamageTime = query->paralyzeTime;
	}

	const float3 impulse(query->impulse.x, query->impulse.y, query->impulse.z);
	feature->DoDamage(damages, impulse, attacker, weaponDef != nullptr ? weaponDef->id : -1, -1);

	result->success = true;
}

static void NativeSetFeatureBlocking(const SetFeatureBlockingQuery* query, SetFeatureBlockingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	// Update solid objects collidable bit
	if (query->options.solidObjects) {
		feature->SetCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	} else {
		feature->ClearCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	}

	// Update blocking bit
	if (query->options.blocking) {
		feature->Block();
	} else {
		feature->UnBlock();
	}

	// Update other collidable bits
	feature->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_PROJECTILES, query->options.projectiles);
	feature->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS, query->options.quadMapRays);

	// Update other blocking properties
	feature->crushable = query->options.crushable;
	feature->blockEnemyPushing = query->options.blockEnemyPushing;
	feature->blockHeightChanges = query->options.blockHeightChanges;

	result->success = true;
}

static void NativeSetFeatureMass(const SetFeatureMassQuery* query, SetFeatureMassResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->SetMass(query->mass);
	result->success = true;
}

static void NativeSetFeatureMaxHealth(const SetFeatureMaxHealthQuery* query, SetFeatureMaxHealthResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->maxHealth = std::max(0.1f, query->maxHealth);
	feature->health = std::min(feature->health, feature->maxHealth);
	result->success = true;
}

static void NativeSetFeatureReclaim(const SetFeatureReclaimQuery* query, SetFeatureReclaimResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->reclaimLeft = query->reclaimLeft;
	result->success = true;
}

static void NativeSetFeatureResurrect(const SetFeatureResurrectQuery* query, SetFeatureResurrectResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const UnitDef* ud = (query->unitDef.id >= 0)
		? unitDefHandler->GetUnitDefByID(query->unitDef.id)
		: ((query->unitDef.name != nullptr) ? unitDefHandler->GetUnitDefByName(query->unitDef.name) : nullptr);
	feature->udef = ud;

	// Set facing direction
	if (query->facing >= 0 && query->facing < 4) {
		feature->buildFacing = query->facing;
	}
	feature->resurrectProgress = std::clamp(query->progress, 0.0f, 1.0f);

	result->success = true;
}

static void NativeSetFeaturePhysics(const SetFeaturePhysicsQuery* query, SetFeaturePhysicsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const float3 rot(query->rotation.x, query->rotation.y, query->rotation.z);
	feature->dragScales.x = std::clamp(query->drag.x, 0.0f, 1.0f);
	feature->dragScales.y = std::clamp(query->drag.y, 0.0f, 1.0f);
	feature->dragScales.z = std::clamp(query->drag.z, 0.0f, 1.0f);

	// Keep this sequence identical to LuaSyncedCtrl::SetSolidObjectPhysicalState.
	// In particular, ForcedMove must be the operation that changes the position:
	// moving first and then calling ForcedMove makes the quad-field removal use
	// the new position and leaves a stale pointer in the old quad.
	feature->SetDirVectorsEuler(rot);
	feature->ForcedMove(pos);
	feature->SetVelocityAndSpeed(float3(query->velocity.x, query->velocity.y, query->velocity.z));

	result->success = true;
}

static void NativeSetFeatureMoveCtrl(const SetFeatureMoveCtrlQuery* query, SetFeatureMoveCtrlResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	CFeature::MoveCtrl& moveCtrl = feature->moveCtrl;
	moveCtrl.enabled = query->enable;
	if (query->enable) {
		featureHandler.SetFeatureUpdateable(feature);
		moveCtrl.velVector = float3(query->velocityOrMask.x, query->velocityOrMask.y, query->velocityOrMask.z);
		moveCtrl.accVector = float3(query->accelerationOrImpulseMask.x, query->accelerationOrImpulseMask.y, query->accelerationOrImpulseMask.z);
	} else {
		moveCtrl.velocityMask.x = (query->velocityOrMask.x != 0.0f);
		moveCtrl.velocityMask.y = (query->velocityOrMask.y != 0.0f);
		moveCtrl.velocityMask.z = (query->velocityOrMask.z != 0.0f);
		moveCtrl.impulseMask.x = (query->accelerationOrImpulseMask.x != 0.0f);
		moveCtrl.impulseMask.y = (query->accelerationOrImpulseMask.y != 0.0f);
		moveCtrl.impulseMask.z = (query->accelerationOrImpulseMask.z != 0.0f);
		moveCtrl.movementMask.x = (query->movementMask.x != 0.0f);
		moveCtrl.movementMask.y = (query->movementMask.y != 0.0f);
		moveCtrl.movementMask.z = (query->movementMask.z != 0.0f);
	}

	result->success = true;
}

static void NativeSetFeatureHeadingAndUpDir(const SetFeatureHeadingAndUpDirQuery* query, SetFeatureHeadingAndUpDirResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	float3 upDir(query->upDir.x, query->upDir.y, query->upDir.z);
	upDir.SafeNormalize();

	feature->heading = query->heading;
	// Keep this sequence identical to LuaSyncedCtrl's
	// SetSolidObjectHeadingAndUpDir.  In particular, UpdateDirVectors(const
	// float3&) preserves the requested heading while applying the new up
	// direction; the bool overload has different ground/object-normal
	// semantics.
	feature->UpdateDirVectors(upDir);
	feature->SetFacingFromHeading();
	feature->UpdateMidAndAimPos();
	feature->UpdateTransform(feature->pos, true);

	result->success = true;
}

static void NativeSetFeatureRotation(const SetFeatureRotationQuery* query, SetFeatureRotationResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const float3 rot(query->rotation.x, query->rotation.y, query->rotation.z);
	feature->SetDirVectorsEuler(rot);
	// Lua's SetFeatureRotation performs exactly these two operations.  Do not
	// replace the transform update with ForcedSpin: that recomputes the basis,
	// heading, and mid/aim state and is an observable extra side effect.
	feature->UpdateTransform(feature->pos, true);

	result->success = true;
}

static void NativeSetFeatureAlwaysVisible(const SetFeatureAlwaysVisibleQuery* query, SetFeatureAlwaysVisibleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->alwaysVisible = query->alwaysVisible;
	result->success = true;
}

static void NativeSetFeatureUseAirLos(const SetFeatureUseAirLosQuery* query, SetFeatureUseAirLosResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->useAirLos = query->useAirLos;
	result->success = true;
}

static void NativeSetFeatureNoSelect(const SetFeatureNoSelectQuery* query, SetFeatureNoSelectResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->noSelect = query->noSelect;
	result->success = true;
}

static void NativeSetFeatureMidAndAimPos(const SetFeatureMidAndAimPosQuery* query, SetFeatureMidAndAimPosResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const float3 newMidPos(query->midPos.x, query->midPos.y, query->midPos.z);
	const float3 newAimPos(query->aimPos.x, query->aimPos.y, query->aimPos.z);
	const bool updateQuads = (newMidPos != feature->midPos);

	if (updateQuads)
		quadField.RemoveFeature(feature);

	feature->SetMidAndAimPos(newMidPos, newAimPos, query->setRelative);

	if (updateQuads)
		quadField.AddFeature(feature);

	result->success = true;
}

static void NativeSetFeatureRadiusAndHeight(const SetFeatureRadiusAndHeightQuery* query, SetFeatureRadiusAndHeightResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const float newRadius = std::max(1.0f, query->radius);
	const float newHeight = std::max(1.0f, query->height);
	const bool updateQuads = (newRadius != feature->radius);

	if (updateQuads)
		quadField.RemoveFeature(feature);

	feature->SetRadiusAndHeight(newRadius, newHeight);

	if (updateQuads)
		quadField.AddFeature(feature);

	result->success = true;
}

static void NativeSetFeatureCollisionVolumeData(const SetFeatureCollisionVolumeDataQuery* query, SetFeatureCollisionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const float3 scales(query->scales.x, query->scales.y, query->scales.z);
	const float3 offsets(query->offsets.x, query->offsets.y, query->offsets.z);

	feature->collisionVolume.InitShape(
		scales,
		offsets,
		query->volumeType,
		query->testType,
		query->primaryAxis
	);

	result->success = true;
}

static void NativeSetFeatureSelectionVolumeData(const SetFeatureSelectionVolumeDataQuery* query, SetFeatureSelectionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const float3 scales(query->scales.x, query->scales.y, query->scales.z);
	const float3 offsets(query->offsets.x, query->offsets.y, query->offsets.z);

	feature->selectionVolume.InitShape(
		scales,
		offsets,
		query->volumeType,
		query->useContHitTest ? CollisionVolume::COLVOL_HITTEST_CONT : CollisionVolume::COLVOL_HITTEST_DISC,
		query->primaryAxis
	);

	result->success = true;
}

static void NativeSetFeatureFireTime(const SetFeatureFireTimeQuery* query, SetFeatureFireTimeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->fireTime = static_cast<int>(query->fireTime * GAME_SPEED);
	result->success = true;
}

static void NativeSetFeatureSmokeTime(const SetFeatureSmokeTimeQuery* query, SetFeatureSmokeTimeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	feature->smokeTime = static_cast<int>(query->smokeTime * GAME_SPEED);
	result->success = true;
}

static void NativeCreateUnitWreck(const CreateUnitWreckQuery* query, CreateUnitWreckResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->featureID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int wreckLevel = std::max(1, query->wreckLevel);
	CFeature* wreck = unit->CreateWreck(wreckLevel, query->doSmoke);

	if (wreck != nullptr) {
		result->featureID = wreck->id;
	}
}

static void NativeCreateFeatureWreck(const CreateFeatureWreckQuery* query, CreateFeatureWreckResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->featureID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const int wreckLevel = std::max(1, query->wreckLevel);
	CFeature* wreck = feature->CreateWreck(wreckLevel, query->doSmoke);

	if (wreck != nullptr) {
		result->featureID = wreck->id;
	}
}

static void NativeSetFeaturePieceVisible(const SetFeaturePieceVisibleQuery* query, SetFeaturePieceVisibleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	LocalModel& localModel = feature->localModel;
	if (query->pieceIndex < 0 || query->pieceIndex >= static_cast<int>(localModel.pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
		return;
	}

	localModel.pieces[query->pieceIndex].SetScriptVisible(query->visible);
	result->success = true;
}

static void NativeSetFeaturePieceMatrix(const SetFeaturePieceMatrixQuery* query, SetFeaturePieceMatrixResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->blockScriptAnims = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	LocalModel& localModel = feature->localModel;
	if (query->pieceIndex < 0 || query->pieceIndex >= static_cast<int>(localModel.pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
		return;
	}

	LocalModelPiece* lmp = &localModel.pieces[query->pieceIndex];

	CMatrix44f mat;
	for (int i = 0; i < 16; ++i) {
		mat.m[i] = query->matrix[i];
	}

	if (lmp->SetPieceSpaceMatrix(mat))
		lmp->SetDirty();

	result->blockScriptAnims = lmp->blockScriptAnims;
}

static void NativeSetFeaturePieceCollisionVolumeData(const SetFeaturePieceCollisionVolumeDataQuery* query, SetFeaturePieceCollisionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	LocalModel& localModel = feature->localModel;
	if (query->pieceIndex < 0 || query->pieceIndex >= static_cast<int>(localModel.pieces.size())) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
		return;
	}

	LocalModelPiece* lmp = &localModel.pieces[query->pieceIndex];

	const float3 scales(query->scales.x, query->scales.y, query->scales.z);
	const float3 offsets(query->offsets.x, query->offsets.y, query->offsets.z);

	CollisionVolume* vol = lmp->GetCollisionVolume();
	vol->InitShape(scales, offsets, query->volumeType, CollisionVolume::COLVOL_HITTEST_CONT, query->primaryAxis);
	vol->SetIgnoreHits(!query->enable);
	result->success = true;
}

static const FeatureControlApi FEATURE_CONTROL_API = {
	.CreateFeature = NativeCreateFeature,
	.DestroyFeature = NativeDestroyFeature,
	.TransferFeature = NativeTransferFeature,
	.SetFeatureHealth = NativeSetFeatureHealth,
	.SetFeaturePosition = NativeSetFeaturePosition,
	.SetFeatureDirection = NativeSetFeatureDirection,
	.SetFeatureVelocity = NativeSetFeatureVelocity,
	.SetFeatureResources = NativeSetFeatureResources,
	.AddFeatureDamage = NativeAddFeatureDamage,
	.SetFeatureBlocking = NativeSetFeatureBlocking,
	.SetFeatureMass = NativeSetFeatureMass,
	.SetFeatureMaxHealth = NativeSetFeatureMaxHealth,
	.SetFeatureReclaim = NativeSetFeatureReclaim,
	.SetFeatureResurrect = NativeSetFeatureResurrect,
	.SetFeaturePhysics = NativeSetFeaturePhysics,
	.SetFeatureMoveCtrl = NativeSetFeatureMoveCtrl,
	.SetFeatureHeadingAndUpDir = NativeSetFeatureHeadingAndUpDir,
	.SetFeatureRotation = NativeSetFeatureRotation,
	.SetFeatureAlwaysVisible = NativeSetFeatureAlwaysVisible,
	.SetFeatureUseAirLos = NativeSetFeatureUseAirLos,
	.SetFeatureNoSelect = NativeSetFeatureNoSelect,
	.SetFeatureMidAndAimPos = NativeSetFeatureMidAndAimPos,
	.SetFeatureRadiusAndHeight = NativeSetFeatureRadiusAndHeight,
	.SetFeatureCollisionVolumeData = NativeSetFeatureCollisionVolumeData,
	.SetFeatureSelectionVolumeData = NativeSetFeatureSelectionVolumeData,
	.SetFeatureFireTime = NativeSetFeatureFireTime,
	.SetFeatureSmokeTime = NativeSetFeatureSmokeTime,
	.CreateUnitWreck = NativeCreateUnitWreck,
	.CreateFeatureWreck = NativeCreateFeatureWreck,
	.SetFeaturePieceVisible = NativeSetFeaturePieceVisible,
	.SetFeaturePieceMatrix = NativeSetFeaturePieceMatrix,
	.SetFeaturePieceCollisionVolumeData = NativeSetFeaturePieceCollisionVolumeData
};

// ============================================================================
// Terrain Control Implementation
// ============================================================================

// Terrain edit batching, mirroring LuaSyncedCtrl's Set*Func wrappers.
static bool inHeightMapEdit = false;
static int heightMapEditX1 = 0, heightMapEditX2 = -1;
static int heightMapEditZ1 = 0, heightMapEditZ2 = 0;
static bool inOriginalHeightMapEdit = false;
static bool inSmoothMeshEdit = false;

static void TrackHeightMapEdit(int x, int z)
{
	if (!inHeightMapEdit)
		return;
	if (x < heightMapEditX1) heightMapEditX1 = x;
	if (x > heightMapEditX2) heightMapEditX2 = x;
	if (z < heightMapEditZ1) heightMapEditZ1 = z;
	if (z > heightMapEditZ2) heightMapEditZ2 = z;
}

static void NativeAddHeightMap(const AddHeightMapQuery* query, AddHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!inHeightMapEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	// Convert world coordinates to heightmap coordinates
	const int x = query->x / SQUARE_SIZE;
	const int z = query->z / SQUARE_SIZE;

	if (x >= 0 && x <= mapDims.mapx && z >= 0 && z <= mapDims.mapy) {
		const int idx = z * mapDims.mapxp1 + x;
		if (query->height != 0.0f) {
			readMap->AddHeight(idx, query->height);
			TrackHeightMapEdit(x, z);
		}
		result->success = true;
	}
}

static void NativeSetHeightMap(const SetHeightMapQuery* query, SetHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!inHeightMapEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const int x = query->x / SQUARE_SIZE;
	const int z = query->z / SQUARE_SIZE;

	if (x >= 0 && x <= mapDims.mapx && z >= 0 && z <= mapDims.mapy) {
		const int idx = z * mapDims.mapxp1 + x;
		const float oldHeight = readMap->GetCornerHeightMapSynced()[idx];
		const float height = oldHeight + (query->height - oldHeight) * query->terraform;
		if (height != oldHeight) {
			readMap->SetHeight(idx, height);
			TrackHeightMapEdit(x, z);
		}
		result->success = true;
	}
}

static void NativeRevertHeightMap(const RevertHeightMapQuery* query, RevertHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (readMap != nullptr && mapDamage != nullptr) {
		float x1 = query->x1;
		float x2 = query->x2;
		float z1 = query->z1;
		float z2 = query->z2;
		if (x1 > x2) std::swap(x1, x2);
		if (z1 > z2) std::swap(z1, z2);

		const int hx1 = std::clamp(static_cast<int>(x1 / SQUARE_SIZE), 0, mapDims.mapx);
		const int hx2 = std::clamp(static_cast<int>(x2 / SQUARE_SIZE), 0, mapDims.mapx);
		const int hz1 = std::clamp(static_cast<int>(z1 / SQUARE_SIZE), 0, mapDims.mapy);
		const int hz2 = std::clamp(static_cast<int>(z2 / SQUARE_SIZE), 0, mapDims.mapy);
		const float* origMap = readMap->GetOriginalHeightMapSynced();
		const float* currMap = readMap->GetCornerHeightMapSynced();

		for (int z = hz1; z <= hz2; ++z) {
			for (int x = hx1; x <= hx2; ++x) {
				const int idx = (z * mapDims.mapxp1) + x;
				readMap->SetHeight(idx, origMap[idx] * query->origFactor + currMap[idx] * (1.0f - query->origFactor));
			}
		}

		mapDamage->RecalcArea(hx1, hx2, hz1, hz2);
		result->success = true;
	}
}

static void NativeAddSmoothMesh(const AddSmoothMeshQuery* query, AddSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!inSmoothMeshEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const int x = static_cast<int>(query->x / smoothGround.GetResolution());
	const int z = static_cast<int>(query->z / smoothGround.GetResolution());

	if (x < 0 || x > smoothGround.GetMaxX() - 1 || z < 0 || z > smoothGround.GetMaxY() - 1) {
		result->success = false;
		return;
	}

	const int idx = z * smoothGround.GetMaxX() + x;
	smoothGround.AddHeight(idx, query->height);
	result->success = true;
}

static void NativeSetSmoothMesh(const SetSmoothMeshQuery* query, SetSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!inSmoothMeshEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const int x = static_cast<int>(query->x / smoothGround.GetResolution());
	const int z = static_cast<int>(query->z / smoothGround.GetResolution());

	if (x < 0 || x >= smoothGround.GetMaxX() || z < 0 || z >= smoothGround.GetMaxY()) {
		return;
	}

	const int idx = z * smoothGround.GetMaxX() + x;
	const float oldHeight = smoothGround.GetMeshData()[idx];
	const float height = oldHeight + (query->height - oldHeight) * query->terraform;
	smoothGround.SetHeight(idx, height);
	result->success = true;
}

static void NativeRevertSmoothMesh(const RevertSmoothMeshQuery* query, RevertSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	float x1 = query->x1;
	float x2 = query->x2;
	float z1 = query->z1;
	float z2 = query->z2;
	if (x1 > x2) std::swap(x1, x2);
	if (z1 > z2) std::swap(z1, z2);

	const int minx = std::clamp(static_cast<int>(x1 / smoothGround.GetResolution()), 0, smoothGround.GetMaxX() - 1);
	const int maxx = std::clamp(static_cast<int>(x2 / smoothGround.GetResolution()), 0, smoothGround.GetMaxX() - 1);
	const int minz = std::clamp(static_cast<int>(z1 / smoothGround.GetResolution()), 0, smoothGround.GetMaxY() - 1);
	const int maxz = std::clamp(static_cast<int>(z2 / smoothGround.GetResolution()), 0, smoothGround.GetMaxY() - 1);
	const float* origMesh = smoothGround.GetOriginalMeshData();
	const float* currMesh = smoothGround.GetMeshData();
	for (int z = minz; z <= maxz; ++z) {
		for (int x = minx; x <= maxx; ++x) {
			const int idx = z * smoothGround.GetMaxX() + x;
			if (idx >= 0 && idx < smoothGround.GetMaxX() * smoothGround.GetMaxY()) {
				smoothGround.SetHeight(idx, origMesh[idx] * query->origFactor + currMesh[idx] * (1.0f - query->origFactor));
			}
		}
	}
	result->success = true;
}

static void NativeSetMapSquareTerrainType(const SetMapSquareTerrainTypeQuery* query, SetMapSquareTerrainTypeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (readMap != nullptr) {
		uint8_t* typeMap = readMap->GetTypeMapSynced();
		const int hx = query->x / SQUARE_SIZE;
		const int hz = query->z / SQUARE_SIZE;

		if (hx >= 0 && hx < mapDims.mapx && hz >= 0 && hz < mapDims.mapy) {
			const int tx = hx >> 1;
			const int tz = hz >> 1;
			const int idx = tz * mapDims.hmapx + tx;
			typeMap[idx] = std::clamp(query->terrainType, 0, CMapInfo::NUM_TERRAIN_TYPES - 1);
			pathManager->TerrainChange(hx, hz, hx + 1, hz + 1, TERRAINCHANGE_SQUARE_TYPEMAP_INDEX);
			result->success = true;
		}
	}
}

static void NativeSetTerrainTypeData(const SetTerrainTypeDataQuery* query, SetTerrainTypeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapInfo == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->typeIndex < 0 || query->typeIndex >= CMapInfo::NUM_TERRAIN_TYPES) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid terrain type index");
		return;
	}

	CMapInfo::TerrainType* tt = const_cast<CMapInfo::TerrainType*>(&mapInfo->terrainTypes[query->typeIndex]);
	tt->tankSpeed = query->tankSpeed;
	tt->kbotSpeed = query->kbotSpeed;
	tt->hoverSpeed = query->hoverSpeed;
	tt->shipSpeed = query->shipSpeed;
	tt->hardness = query->hardness;
	tt->receiveTracks = query->receiveTracks;
	if (query->name != nullptr)
		tt->name = query->name;

	result->success = true;
}

static void NativeSetTidal(const SetTidalQuery* query, SetTidalResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Note: mapInfo is const, cannot modify tidalStrength at runtime
	// The tidal strength can only be set during map initialization via LoadTidal()
	if (mapInfo != nullptr) {
		envResHandler.LoadTidal(query->tidal);
		result->success = true;
	}
}

static void NativeSetWind(const SetWindQuery* query, SetWindResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	envResHandler.LoadWind(query->minWind, query->maxWind);
	result->success = true;
}

static void NativeAddGrass(const AddGrassQuery* query, AddGrassResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (grassDrawer == nullptr) {
		result->error = &NOT_AVAILABLE_ERROR;
		return;
	}

	grassDrawer->AddGrass(float3(query->x, 0.0f, query->z).cClampInBounds(), query->grassValue);

	result->success = true;
}

static void NativeRemoveGrass(const RemoveGrassQuery* query, RemoveGrassResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (grassDrawer == nullptr) {
		result->error = &NOT_AVAILABLE_ERROR;
		return;
	}

	grassDrawer->RemoveGrass(float3(query->x, 0.0f, query->z).cClampInBounds());

	result->success = true;
}

static void NativeAdjustHeightMap(const AdjustHeightMapQuery* query, AdjustHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	float x1 = query->x1;
	float x2 = query->x2;
	float z1 = query->z1;
	float z2 = query->z2;
	if (x1 > x2) std::swap(x1, x2);
	if (z1 > z2) std::swap(z1, z2);

	const int hx1 = std::clamp(static_cast<int>(x1 / SQUARE_SIZE), 0, mapDims.mapx);
	const int hx2 = std::clamp(static_cast<int>(x2 / SQUARE_SIZE), 0, mapDims.mapx);
	const int hz1 = std::clamp(static_cast<int>(z1 / SQUARE_SIZE), 0, mapDims.mapy);
	const int hz2 = std::clamp(static_cast<int>(z2 / SQUARE_SIZE), 0, mapDims.mapy);

	for (int z = hz1; z <= hz2; ++z) {
		for (int x = hx1; x <= hx2; ++x) {
			readMap->AddHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

	mapDamage->RecalcArea(hx1, hx2, hz1, hz2);
	result->success = true;
}

static void NativeLevelHeightMap(const LevelHeightMapQuery* query, LevelHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	float fx1 = query->x1;
	float fx2 = query->x2;
	float fz1 = query->z1;
	float fz2 = query->z2;
	if (fx1 > fx2) std::swap(fx1, fx2);
	if (fz1 > fz2) std::swap(fz1, fz2);

	const int x1 = std::clamp(static_cast<int>(fx1 / SQUARE_SIZE), 0, mapDims.mapx);
	const int z1 = std::clamp(static_cast<int>(fz1 / SQUARE_SIZE), 0, mapDims.mapy);
	const int x2 = std::clamp(static_cast<int>(fx2 / SQUARE_SIZE), 0, mapDims.mapx);
	const int z2 = std::clamp(static_cast<int>(fz2 / SQUARE_SIZE), 0, mapDims.mapy);

	bool changed = false;
	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			const int index = (z * mapDims.mapxp1) + x;
			if (readMap->GetCornerHeightMapSynced()[index] == query->height)
				continue;

			readMap->SetHeight(index, query->height);
			changed = true;
		}
	}

	if (changed)
		mapDamage->RecalcArea(x1, x2, z1, z2);
	result->success = true;
}

static void NativeAddOriginalHeightMap(const AddOriginalHeightMapQuery* query, AddOriginalHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	if (!inOriginalHeightMapEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->success = false;
		return;
	}

	const int x = static_cast<int>(query->x / SQUARE_SIZE);
	const int z = static_cast<int>(query->z / SQUARE_SIZE);
	if (x < 0 || x > mapDims.mapx || z < 0 || z > mapDims.mapy) {
		result->success = true;
		return;
	}

	const int idx = (z * mapDims.mapxp1) + x;

	readMap->AddOriginalHeight(idx, query->height);
	result->success = true;
}

static void NativeSetOriginalHeightMap(const SetOriginalHeightMapQuery* query, SetOriginalHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	if (!inOriginalHeightMapEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->success = false;
		return;
	}

	const int x = static_cast<int>(query->x / SQUARE_SIZE);
	const int z = static_cast<int>(query->z / SQUARE_SIZE);

	if (x >= 0 && x <= mapDims.mapx && z >= 0 && z <= mapDims.mapy) {
		const int idx = (z * mapDims.mapxp1) + x;
		const float oldHeight = readMap->GetOriginalHeightMapSynced()[idx];
		const float height = oldHeight + (query->height - oldHeight) * query->factor;
		readMap->SetOriginalHeight(idx, height);
		result->success = true;
	}
}

static void NativeRevertOriginalHeightMap(const RevertOriginalHeightMapQuery* query, RevertOriginalHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	float fx1 = query->x1;
	float fx2 = query->x2;
	float fz1 = query->z1;
	float fz2 = query->z2;
	if (fx1 > fx2) std::swap(fx1, fx2);
	if (fz1 > fz2) std::swap(fz1, fz2);

	const int x1 = std::clamp(static_cast<int>(fx1 / SQUARE_SIZE), 0, mapDims.mapx);
	const int z1 = std::clamp(static_cast<int>(fz1 / SQUARE_SIZE), 0, mapDims.mapy);
	const int x2 = std::clamp(static_cast<int>(fx2 / SQUARE_SIZE), 0, mapDims.mapx);
	const int z2 = std::clamp(static_cast<int>(fz2 / SQUARE_SIZE), 0, mapDims.mapy);
	const float* origMap = readMap->GetMapFileHeightMapSynced();
	const float* currMap = readMap->GetOriginalHeightMapSynced();

	if (query->origFactor == 1.0f) {
		for (int z = z1; z <= z2; ++z) {
			for (int x = x1; x <= x2; ++x) {
				const int idx = (z * mapDims.mapxp1) + x;
				readMap->SetOriginalHeight(idx, origMap[idx]);
			}
		}
	} else {
		const float currFactor = (1.0f - query->origFactor);
		for (int z = z1; z <= z2; ++z) {
			for (int x = x1; x <= x2; ++x) {
				const int idx = (z * mapDims.mapxp1) + x;
				const float ofh = query->origFactor * origMap[idx];
				const float cfh = currFactor * currMap[idx];
				readMap->SetOriginalHeight(idx, ofh + cfh);
			}
		}
	}

	result->success = true;
}

static void NativeAdjustOriginalHeightMap(const AdjustOriginalHeightMapQuery* query, AdjustOriginalHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	float x1 = query->x1;
	float x2 = query->x2;
	float z1 = query->z1;
	float z2 = query->z2;
	if (x1 > x2) std::swap(x1, x2);
	if (z1 > z2) std::swap(z1, z2);

	const int hx1 = std::clamp(static_cast<int>(x1 / SQUARE_SIZE), 0, mapDims.mapx);
	const int hx2 = std::clamp(static_cast<int>(x2 / SQUARE_SIZE), 0, mapDims.mapx);
	const int hz1 = std::clamp(static_cast<int>(z1 / SQUARE_SIZE), 0, mapDims.mapy);
	const int hz2 = std::clamp(static_cast<int>(z2 / SQUARE_SIZE), 0, mapDims.mapy);

	for (int z = hz1; z <= hz2; ++z) {
		for (int x = hx1; x <= hx2; ++x) {
			readMap->AddOriginalHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

	mapDamage->RecalcArea(hx1, hx2, hz1, hz2);
	result->success = true;
}

static void NativeLevelOriginalHeightMap(const LevelOriginalHeightMapQuery* query, LevelOriginalHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	float fx1 = query->x1;
	float fx2 = query->x2;
	float fz1 = query->z1;
	float fz2 = query->z2;
	if (fx1 > fx2) std::swap(fx1, fx2);
	if (fz1 > fz2) std::swap(fz1, fz2);

	const int x1 = std::clamp(static_cast<int>(fx1 / SQUARE_SIZE), 0, mapDims.mapx);
	const int z1 = std::clamp(static_cast<int>(fz1 / SQUARE_SIZE), 0, mapDims.mapy);
	const int x2 = std::clamp(static_cast<int>(fx2 / SQUARE_SIZE), 0, mapDims.mapx);
	const int z2 = std::clamp(static_cast<int>(fz2 / SQUARE_SIZE), 0, mapDims.mapy);

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			readMap->SetOriginalHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

	result->success = true;
}

static void NativeAdjustSmoothMesh(const AdjustSmoothMeshQuery* query, AdjustSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	float x1 = query->x1;
	float x2 = query->x2;
	float z1 = query->z1;
	float z2 = query->z2;
	if (x1 > x2) std::swap(x1, x2);
	if (z1 > z2) std::swap(z1, z2);

	const int hx1 = std::clamp(static_cast<int>(x1 / smoothGround.GetResolution()), 0, smoothGround.GetMaxX() - 1);
	const int hx2 = std::clamp(static_cast<int>(x2 / smoothGround.GetResolution()), 0, smoothGround.GetMaxX() - 1);
	const int hz1 = std::clamp(static_cast<int>(z1 / smoothGround.GetResolution()), 0, smoothGround.GetMaxY() - 1);
	const int hz2 = std::clamp(static_cast<int>(z2 / smoothGround.GetResolution()), 0, smoothGround.GetMaxY() - 1);

	for (int z = hz1; z <= hz2; ++z) {
		for (int x = hx1; x <= hx2; ++x) {
			const int index = (z * smoothGround.GetMaxX()) + x;
			smoothGround.AddHeight(index, query->height);
		}
	}

	result->success = true;
}

static void NativeLevelSmoothMesh(const LevelSmoothMeshQuery* query, LevelSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	float fx1 = query->x1;
	float fx2 = query->x2;
	float fz1 = query->z1;
	float fz2 = query->z2;
	if (fx1 > fx2) std::swap(fx1, fx2);
	if (fz1 > fz2) std::swap(fz1, fz2);

	const int x1 = std::clamp(static_cast<int>(fx1 / smoothGround.GetResolution()), 0, smoothGround.GetMaxX() - 1);
	const int z1 = std::clamp(static_cast<int>(fz1 / smoothGround.GetResolution()), 0, smoothGround.GetMaxY() - 1);
	const int x2 = std::clamp(static_cast<int>(fx2 / smoothGround.GetResolution()), 0, smoothGround.GetMaxX() - 1);
	const int z2 = std::clamp(static_cast<int>(fz2 / smoothGround.GetResolution()), 0, smoothGround.GetMaxY() - 1);

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			const int index = (z * smoothGround.GetMaxX()) + x;
			smoothGround.SetHeight(index, query->height);
		}
	}

	result->success = true;
}

static void NativeSetHeightMapFunc(const SetHeightMapFuncQuery* query, SetHeightMapFuncResult* result)
{
	bufferPos = 0;
	result->success = false;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->callback == nullptr || readMap == nullptr || mapDamage == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}
	if (inHeightMapEdit) {
		// no recursion
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	heightMapEditX1 = mapDims.mapx;
	heightMapEditX2 = -1;
	heightMapEditZ1 = mapDims.mapy;
	heightMapEditZ2 = 0;
	inHeightMapEdit = true;

	query->callback(query->userData);

	inHeightMapEdit = false;

	if (heightMapEditX2 > -1) {
		readMap->MarkHeightMapUpdated();
		mapDamage->RecalcArea(heightMapEditX1, heightMapEditX2, heightMapEditZ1, heightMapEditZ2);
	}

	result->success = true;
}

static void NativeSetOriginalHeightMapFunc(const SetOriginalHeightMapFuncQuery* query, SetOriginalHeightMapFuncResult* result)
{
	bufferPos = 0;
	result->success = false;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (mapDamage->Disabled()) {
		result->success = true;
		return;
	}

	if (query->callback == nullptr || readMap == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	if (inOriginalHeightMapEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	inOriginalHeightMapEdit = true;
	query->callback(query->userData);
	inOriginalHeightMapEdit = false;

	result->success = true;
}

static void NativeSetSmoothMeshFunc(const SetSmoothMeshFuncQuery* query, SetSmoothMeshFuncResult* result)
{
	bufferPos = 0;
	result->success = false;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	if (inSmoothMeshEdit) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	inSmoothMeshEdit = true;
	query->callback(query->userData);
	inSmoothMeshEdit = false;

	result->success = true;
}

static void NativeRebuildSmoothMesh(const RebuildSmoothMeshQuery* query, RebuildSmoothMeshResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Rebuild the entire smooth mesh from the height map
	// The query parameters are ignored as MakeSmoothMesh rebuilds everything
	smoothGround.MakeSmoothMesh();

	result->success = true;
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
	.SetWind = NativeSetWind,
	.AddGrass = NativeAddGrass,
	.RemoveGrass = NativeRemoveGrass,
	.AdjustHeightMap = NativeAdjustHeightMap,
	.LevelHeightMap = NativeLevelHeightMap,
	.AddOriginalHeightMap = NativeAddOriginalHeightMap,
	.SetOriginalHeightMap = NativeSetOriginalHeightMap,
	.RevertOriginalHeightMap = NativeRevertOriginalHeightMap,
	.AdjustOriginalHeightMap = NativeAdjustOriginalHeightMap,
	.LevelOriginalHeightMap = NativeLevelOriginalHeightMap,
	.AdjustSmoothMesh = NativeAdjustSmoothMesh,
	.LevelSmoothMesh = NativeLevelSmoothMesh,
	.RebuildSmoothMesh = NativeRebuildSmoothMesh,
	.SetHeightMapFunc = NativeSetHeightMapFunc,
	.SetOriginalHeightMapFunc = NativeSetOriginalHeightMapFunc,
	.SetSmoothMeshFunc = NativeSetSmoothMeshFunc
};

// ============================================================================
// Projectile Control Implementation
// ============================================================================

static void NativeSpawnProjectile(const SpawnProjectileQuery* query, SpawnProjectileResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->projectileID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const WeaponDef* weaponDef = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
	if (weaponDef == nullptr) {
		result->error = &INVALID_WEAPONDEF_ERROR;
		return;
	}

	const NativeProjectileParams& nativeParams = query->projectileParams;
	if (nativeParams.team >= 0 && !teamHandler.IsValidTeam(nativeParams.team)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	ProjectileParams params;
	params.pos = float3(nativeParams.pos.x, nativeParams.pos.y, nativeParams.pos.z);
	params.end = float3(nativeParams.end.x, nativeParams.end.y, nativeParams.end.z);
	params.speed = float3(nativeParams.speed.x, nativeParams.speed.y, nativeParams.speed.z);
	params.spread = float3(nativeParams.spread.x, nativeParams.spread.y, nativeParams.spread.z);
	params.error = float3(nativeParams.error.x, nativeParams.error.y, nativeParams.error.z);
	params.ownerID = (nativeParams.owner >= 0) ? static_cast<unsigned int>(nativeParams.owner) : -1u;
	params.teamID = (nativeParams.team >= 0) ? static_cast<unsigned int>(nativeParams.team) : teamHandler.GaiaTeamID();
	params.weaponNum = (nativeParams.weaponNum >= 0) ? static_cast<unsigned int>(nativeParams.weaponNum) : -1u;
	params.ttl = static_cast<int>(nativeParams.ttl);
	params.gravity = nativeParams.gravity;
	params.tracking = nativeParams.tracking;
	params.maxRange = nativeParams.maxRange;
	params.upTime = nativeParams.upTime;
	params.startAlpha = nativeParams.startAlpha;
	params.endAlpha = nativeParams.endAlpha;
	params.weaponDef = weaponDef;

	if (nativeParams.model != nullptr && nativeParams.model[0] != '\0')
		params.model = modelLoader.LoadModel(nativeParams.model);
	if (nativeParams.cegTag != nullptr && nativeParams.cegTag[0] != '\0')
		params.cegID = explGenHandler.LoadGeneratorID(nativeParams.cegTag);

	const unsigned int projectileID = WeaponProjectileFactory::LoadProjectile(params);
	result->projectileID = static_cast<int32_t>(projectileID);
}

static void NativeDeleteProjectile(const DeleteProjectileQuery* query, DeleteProjectileResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* projectile = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (projectile == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	projectile->Delete();

	result->success = true;
}

static void NativeSetProjectilePosition(const SetProjectilePositionQuery* query, SetProjectilePositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* projectile = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (projectile == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	projectile->pos = pos;
	projectile->SetPosition(pos);

	result->success = true;
}

static void NativeSetProjectileVelocity(const SetProjectileVelocityQuery* query, SetProjectileVelocityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* projectile = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (projectile == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	const float3 velocity(query->velocity.x, query->velocity.y, query->velocity.z);
	projectile->SetVelocityAndSpeed(velocity);

	result->success = true;
}

static void NativeSetProjectileGravity(const SetProjectileGravityQuery* query, SetProjectileGravityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* projectile = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (projectile == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	projectile->mygravity = query->gravity;

	result->success = true;
}

static void NativeSetProjectileTarget(const SetProjectileTargetQuery* query, SetProjectileTargetResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* projectile = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (projectile == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	CWeaponProjectile* weaponProj = dynamic_cast<CWeaponProjectile*>(projectile);
	if (weaponProj == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Projectile is not a weapon projectile");
		return;
	}

	CWorldObject* oldTargetObject = weaponProj->GetTargetObject();
	CWorldObject* newTargetObject = nullptr;

	if (query->target.isGroundTarget) {
		const float3 targetPos(query->target.pos.x, query->target.pos.y, query->target.pos.z);
		if (oldTargetObject != nullptr) {
			weaponProj->DeleteDeathDependence(oldTargetObject, DEPENDENCE_WEAPONTARGET);
			weaponProj->DeleteDeathDependence(oldTargetObject, DEPENDENCE_INTERCEPTTARGET);
		}
		weaponProj->SetTargetObject(nullptr);
		weaponProj->SetTargetPos(targetPos);
	} else {
		switch (query->target.targetType) {
			case 'u': newTargetObject = unitHandler.GetUnit(query->target.targetID); break;
			case 'f': newTargetObject = featureHandler.GetFeature(query->target.targetID); break;
			case 'p': newTargetObject = projectileHandler.GetProjectileBySyncedID(query->target.targetID); break;
			default: break;
		}

		if (oldTargetObject != nullptr) {
			weaponProj->DeleteDeathDependence(oldTargetObject, DEPENDENCE_WEAPONTARGET);
			weaponProj->DeleteDeathDependence(oldTargetObject, DEPENDENCE_INTERCEPTTARGET);
		}
		weaponProj->SetTargetObject(nullptr);
		if (newTargetObject != nullptr) {
			if (dynamic_cast<CSolidObject*>(newTargetObject) != nullptr)
				weaponProj->AddDeathDependence(newTargetObject, DEPENDENCE_WEAPONTARGET);
			else if (dynamic_cast<CWeaponProjectile*>(newTargetObject) != nullptr)
				weaponProj->AddDeathDependence(newTargetObject, DEPENDENCE_INTERCEPTTARGET);
			weaponProj->SetTargetObject(newTargetObject);
		}
	}

	result->success = (oldTargetObject != nullptr || newTargetObject != nullptr || query->target.isGroundTarget);
}

static void NativeSetProjectileDamages(const SetProjectileDamagesQuery* query, SetProjectileDamagesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr || !proj->weapon) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	CWeaponProjectile* wpro = static_cast<CWeaponProjectile*>(proj);
	DynDamageArray* damages = DynDamageArray::GetMutable(wpro->damages);
	(void)query->unused;
	if (query->damageKey == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Damage key is null");
		return;
	}

	char* endptr = nullptr;
	const long armType = std::strtol(query->damageKey, &endptr, 10);
	if (*endptr == '\0' && armType >= 0) {
		if (static_cast<unsigned long>(armType) >= static_cast<unsigned long>(damages->GetNumTypes())) {
			result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid armor type index");
			return;
		}
		damages->Set(static_cast<int>(armType), query->damageValue);
		result->success = true;
		return;
	}

	result->success = SetSingleDynDamagesKey(damages, query->damageKey, query->damageValue);
	if (!result->success)
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unknown damage key");
}

static void NativeSetProjectileTimeToLive(const SetProjectileTimeToLiveQuery* query, SetProjectileTimeToLiveResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr || !proj->weapon) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	CWeaponProjectile* wproj = static_cast<CWeaponProjectile*>(proj);
	wproj->SetTimeToLive(query->timeToLive);
	result->success = true;
}

static void NativeSetProjectileIsIntercepted(const SetProjectileIsInterceptedQuery* query, SetProjectileIsInterceptedResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	if (CWeaponProjectile* wproj = dynamic_cast<CWeaponProjectile*>(proj)) {
		wproj->SetBeingIntercepted(query->intercepted);
	}
	result->success = true;
}

static void NativeSetProjectileCollision(const SetProjectileCollisionQuery* query, SetProjectileCollisionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	proj->Collision();
	result->success = true;
}

static void NativeSetProjectileCEG(const SetProjectileCEGQuery* query, SetProjectileCEGResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->cegID = static_cast<int32_t>(CExplosionGeneratorHandler::EXPGEN_ID_INVALID);

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}
	if (!proj->weapon && !proj->piece)
		return;
	if (query->cegName == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "CEG name is null");
		return;
	}

	const unsigned int cegID = explGenHandler.LoadCustomGeneratorID(query->cegName);
	result->cegID = static_cast<int32_t>(cegID);
	if (explGenHandler.GetGenerator(cegID) != nullptr)
		proj->SetCustomExpGenID(cegID);
}

static void NativeSetProjectileAlwaysVisible(const SetProjectileAlwaysVisibleQuery* query, SetProjectileAlwaysVisibleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	proj->alwaysVisible = query->alwaysVisible;
	result->success = true;
}

static void NativeSetProjectileUseAirLos(const SetProjectileUseAirLosQuery* query, SetProjectileUseAirLosResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	proj->useAirLos = query->useAirLos;
	result->success = true;
}

static void NativeSetProjectileMoveControl(const SetProjectileMoveControlQuery* query, SetProjectileMoveControlResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}
	if (proj->weapon || proj->piece)
		proj->luaMoveCtrl = query->enable;
	result->success = true;
}

static void NativeSetProjectileIgnoreTrackingError(const SetProjectileIgnoreTrackingErrorQuery* query, SetProjectileIgnoreTrackingErrorResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr || !proj->weapon) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	switch (proj->GetProjectileType()) {
		case WEAPON_MISSILE_PROJECTILE: {
			static_cast<CMissileProjectile*>(proj)->SetIgnoreError(query->ignore);
		} break;
		case WEAPON_STARBURST_PROJECTILE: {
			static_cast<CStarburstProjectile*>(proj)->SetIgnoreError(query->ignore);
		} break;
		case WEAPON_TORPEDO_PROJECTILE: {
			static_cast<CTorpedoProjectile*>(proj)->SetIgnoreError(query->ignore);
		} break;
		default:
			break;
	}
	result->success = true;
}

static void NativeSetPieceProjectileParams(const SetPieceProjectileParamsQuery* query, SetPieceProjectileParamsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CProjectile* proj = projectileHandler.GetProjectileBySyncedID(query->projectileID);
	if (proj == nullptr) {
		result->error = &INVALID_PROJECTILE_ERROR;
		return;
	}

	if (!proj->piece) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Projectile is not a piece projectile");
		return;
	}

	CPieceProjectile* pproj = static_cast<CPieceProjectile*>(proj);
	pproj->explFlags = query->explFlags;
	pproj->spinAngle = query->spinAngle;
	pproj->spinSpeed = query->spinSpeed;
	pproj->spinVec = float3(query->spinVec.x, query->spinVec.y, query->spinVec.z);

	result->success = true;
}

static const ProjectileControlApi PROJECTILE_CONTROL_API = {
	.SpawnProjectile = NativeSpawnProjectile,
	.DeleteProjectile = NativeDeleteProjectile,
	.SetProjectilePosition = NativeSetProjectilePosition,
	.SetProjectileVelocity = NativeSetProjectileVelocity,
	.SetProjectileGravity = NativeSetProjectileGravity,
	.SetProjectileTarget = NativeSetProjectileTarget,
	.SetProjectileDamages = NativeSetProjectileDamages,
	.SetProjectileTimeToLive = NativeSetProjectileTimeToLive,
	.SetProjectileIsIntercepted = NativeSetProjectileIsIntercepted,
	.SetProjectileCollision = NativeSetProjectileCollision,
	.SetProjectileCEG = NativeSetProjectileCEG,
	.SetProjectileAlwaysVisible = NativeSetProjectileAlwaysVisible,
	.SetProjectileUseAirLos = NativeSetProjectileUseAirLos,
	.SetProjectileMoveControl = NativeSetProjectileMoveControl,
	.SetProjectileIgnoreTrackingError = NativeSetProjectileIgnoreTrackingError,
	.SetPieceProjectileParams = NativeSetPieceProjectileParams
};

// ============================================================================
// Effects Control Implementation
// ============================================================================

static void NativeSpawnExplosion(const SpawnExplosionQuery* query, SpawnExplosionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const float3 dir(query->dir.x, query->dir.y, query->dir.z);

	const NativeExplosionParams& nativeParams = query->explosionParams;
	DamageArray damages(nativeParams.damages);

	CExplosionParams params = {
		.pos = pos,
		.dir = dir,
		.damages = damages,
		.weaponDef = (nativeParams.weaponDefID >= 0) ? weaponDefHandler->GetWeaponDefByID(nativeParams.weaponDefID) : nullptr,
		.owner = (nativeParams.ownerID >= 0) ? unitHandler.GetUnit(nativeParams.ownerID) : nullptr,
		.hitObject = ExplosionHitObject(),
		.craterAreaOfEffect = nativeParams.craterAreaOfEffect,
		.damageAreaOfEffect = nativeParams.damageAreaOfEffect,
		.edgeEffectiveness = std::min(nativeParams.edgeEffectiveness, 1.0f),
		.explosionSpeed = nativeParams.explosionSpeed,
		.gfxMod = nativeParams.gfxMod,
		.maxGroundDeformation = 0.0f,
		.impactOnly = nativeParams.impactOnly,
		.ignoreOwner = nativeParams.ignoreOwner,
		.damageGround = nativeParams.damageGround,
		.projectileID = (nativeParams.projectileID >= 0) ? static_cast<uint32_t>(nativeParams.projectileID) : static_cast<uint32_t>(-1)
	};

	if (nativeParams.hitUnitID >= 0) {
		params.hitObject = unitHandler.GetUnit(nativeParams.hitUnitID);
	} else if (nativeParams.hitFeatureID >= 0) {
		params.hitObject = featureHandler.GetFeature(nativeParams.hitFeatureID);
	}

	helper->Explosion(params);
	result->success = true;
}

static void NativeSpawnCEG(const SpawnCEGQuery* query, SpawnCEGResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;
	result->cegID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const float3 dir(query->dir.x, query->dir.y, query->dir.z);

	unsigned int cegID;
	if (query->ceg.id >= 0) {
		cegID = static_cast<unsigned int>(query->ceg.id);
	} else if (query->ceg.name != nullptr) {
		cegID = explGenHandler.LoadCustomGeneratorID(query->ceg.name);
	} else {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid CEG reference");
		return;
	}

	result->success = explGenHandler.GenExplosion(
		cegID,
		pos,
		dir,
		query->damage,
		query->radius,
		query->dmgMod,
		nullptr,
		ExplosionHitObject()
	);
	result->cegID = static_cast<int32_t>(cegID);
}

static void NativeSpawnSFX(const SpawnSFXQuery* query, SpawnSFXResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const float3 dir(query->dir.x, query->dir.y, query->dir.z);

	// Keep this in lockstep with LuaSyncedCtrl::SpawnSFX.  The radius and
	// damage fields are accepted by the Lua ABI for compatibility but are not
	// consumed by EmitAbsSFX/EmitRelSFX.
	if (query->absolute) {
		result->success = unit->script->EmitAbsSFX(query->sfxID, pos, dir);
	} else {
		result->success = unit->script->EmitRelSFX(query->sfxID, pos, dir);
	}
}

static const EffectsControlApi EFFECTS_CONTROL_API = {
	.SpawnExplosion = NativeSpawnExplosion,
	.SpawnCEG = NativeSpawnCEG,
	.SpawnSFX = NativeSpawnSFX
};

// ============================================================================
// Game Config Control Implementation
// ============================================================================

static void NativeSetNoPause(const SetNoPauseQuery* query, SetNoPauseResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Only works in server mode
	if (gameServer != nullptr) {
		gameServer->SetGamePausable(!query->noPause);
		result->success = true;
	} else {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Not in server mode");
	}
}

static void NativeSetCheatingEnabled(const SetCheatingEnabledQuery* query, SetCheatingEnabledResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	gs->cheatEnabled = query->enabled;
	result->success = true;
}

static void NativeSetGodMode(const SetGodModeQuery* query, SetGodModeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	gs->godMode = 0;
	if (query->options.controlAllies)
		gs->godMode |= GODMODE_ATC_BIT;
	if (query->options.controlEnemies)
		gs->godMode |= GODMODE_ETC_BIT;

	CLuaUI::UpdateTeams();
	CPlayer::UpdateControlledTeams();
	result->success = true;
}

static void NativeSetExperienceGrade(const SetExperienceGradeQuery* query, SetExperienceGradeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	globalUnitParams.expGrade = query->expGrade;

	// Additional params only apply if cheats are enabled
	if (gs->cheatEnabled) {
		if (query->expPowerScale >= 0.0f)
			globalUnitParams.expPowerScale = query->expPowerScale;
		if (query->expHealthScale >= 0.0f)
			globalUnitParams.expHealthScale = query->expHealthScale;
		if (query->expReloadScale >= 0.0f)
			globalUnitParams.expReloadScale = query->expReloadScale;
	}

	result->success = true;
}

static void NativeSetRadarErrorParams(const SetRadarErrorParamsQuery* query, SetRadarErrorParamsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) {
		result->error = &INVALID_ALLYTEAM_ERROR;
		return;
	}

	losHandler->SetAllyTeamRadarErrorSize(query->allyTeamID, query->allyTeamErrorSize);

	if (query->baseErrorSize >= 0.0f)
		losHandler->SetBaseRadarErrorSize(query->baseErrorSize);
	if (query->baseErrorMult >= 0.0f)
		losHandler->SetBaseRadarErrorMult(query->baseErrorMult);

	result->success = true;
}

static void NativeSetSquareBuildingMask(const SetSquareBuildingMaskQuery* query, SetSquareBuildingMaskResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!buildingMaskMap.SetTileMask(query->x, query->z, query->mask)) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid tile coordinates");
		return;
	}

	result->success = true;
}

static const GameConfigApi GAME_CONFIG_API = {
	.SetNoPause = NativeSetNoPause,
	.SetCheatingEnabled = NativeSetCheatingEnabled,
	.SetGodMode = NativeSetGodMode,
	.SetExperienceGrade = NativeSetExperienceGrade,
	.SetRadarErrorParams = NativeSetRadarErrorParams,
	.SetSquareBuildingMask = NativeSetSquareBuildingMask
};

// ============================================================================
// COB Script Control Implementation
// ============================================================================

// Thread-local storage for COB return values
thread_local int32_t cobReturnValues[MAX_COB_ARGS];
thread_local std::vector<float> unitScriptReturnValues;
static constexpr uint32_t MAX_UNIT_SCRIPT_RETURN_VALUES = 256;

static void NativeCallCOBScript(const CallCOBScriptQuery* query, CallCOBScriptResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->retCode = 0;
	result->retValues = nullptr;
	result->retCount = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CCobInstance* cob = dynamic_cast<CCobInstance*>(unit->script);
	if (cob == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit is not running a COB script");
		return;
	}

	// Prepare arguments - cobArgs[0] holds the count
	std::array<int, 1 + MAX_COB_ARGS> cobArgs;
	cobArgs[0] = static_cast<int>(std::min(static_cast<uint32_t>(MAX_COB_ARGS), query->argCount));
	for (int i = 0; i < cobArgs[0]; ++i) {
		cobArgs[1 + i] = query->args[i];
	}

	int retCode = 0;
	if (query->func.id >= 0) {
		retCode = cob->RawCall(query->func.id, cobArgs);
	} else if (query->func.name != nullptr) {
		retCode = cob->Call(query->func.name, cobArgs);
	} else {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "No function ID or name provided");
		return;
	}

	result->retCode = retCode;

	const int numRetVals = std::min(static_cast<int>(query->retArgs), std::min(static_cast<int>(MAX_COB_ARGS), cobArgs[0]));
	for (int i = 0; i < numRetVals; ++i) {
		cobReturnValues[i] = cobArgs[i];
	}
	result->retValues = (numRetVals > 0) ? cobReturnValues : nullptr;
	result->retCount = numRetVals;
}

static void NativeGetCOBScriptID(const GetCOBScriptIDQuery* query, GetCOBScriptIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->funcID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	CCobInstance* cob = dynamic_cast<CCobInstance*>(unit->script);
	if (cob == nullptr) {
		// Not an error - allows checking if unit runs COB or LUS
		return;
	}

	if (query->funcName == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Function name is null");
		return;
	}

	result->funcID = cob->GetFunctionId(query->funcName);
}

static const COBScriptApi COB_SCRIPT_API = {
	.CallCOBScript = NativeCallCOBScript,
	.GetCOBScriptID = NativeGetCOBScriptID
};

static void NativeCallUnitScript(const CallUnitScriptQuery* query, CallUnitScriptResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->functionFound = false;
	result->success = false;
	result->retValues = nullptr;
	result->retCount = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->functionName == nullptr || (query->argCount > 0 && query->args == nullptr)) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid unit-script call arguments");
		return;
	}
	if (query->retCapacity > MAX_UNIT_SCRIPT_RETURN_VALUES) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit-script return capacity is too large");
		return;
	}

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unitScriptReturnValues.resize(query->retCapacity);
	uint32_t retCount = 0;
	bool found = false;
	const bool success = unit->script->CallFunctionByName(
		query->functionName,
		query->args,
		query->argCount,
		unitScriptReturnValues.data(),
		query->retCapacity,
		retCount,
		found
	);
	result->functionFound = found;
	result->success = success;
	if (success && retCount > 0) {
		result->retValues = unitScriptReturnValues.data();
		result->retCount = retCount;
	}
}

static const UnitScriptApi UNIT_SCRIPT_API = {
	.CallUnitScript = NativeCallUnitScript,
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
	.projectile = &PROJECTILE_CONTROL_API,
	.effects = &EFFECTS_CONTROL_API,
	.gameConfig = &GAME_CONFIG_API,
	.cobScript = &COB_SCRIPT_API,
	.unitScript = &UNIT_SCRIPT_API,
};

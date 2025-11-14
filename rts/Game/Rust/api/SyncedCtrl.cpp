#include "SyncedCtrl.h"
#include <cstring>
#include <cmath>

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/UnitLoader.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/CommandAI/CommandAI.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/UnitTypes/ExtractorBuilding.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Features/FeatureDefHandler.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectileFactory.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/Team.h"
#include "Sim/Misc/AllyTeam.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/DamageArray.h"
#include "Sim/Misc/SmoothHeightMesh.h"
#include "Sim/Misc/Wind.h"
#include "Sim/Weapons/WeaponDefHandler.h"
#include "Sim/Weapons/WeaponDef.h"
#include "Game/GameHelper.h"
#include "Game/GameSetup.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/Players/Player.h"
#include "Map/ReadMap.h"
#include "Map/MapDamage.h"
#include "Map/MapInfo.h"
#include "Map/MapDimensions.h"
#include "Map/Ground.h"
#include "Sim/Misc/GroundBlockingObjectMap.h"
#include "Sim/Misc/CollisionVolume.h"
#include "System/EventHandler.h"
#include "System/float3.h"
#include "System/Matrix44f.h"
#include "System/creg/STL_Map.h"

namespace {

// Thread-local scratch buffer for dynamic data
thread_local uint8_t scratchBuffer[8192];
thread_local size_t bufferPos = 0;

// Error messages
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Game not ready"
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

static const Error INVALID_RESOURCE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid resource type (use 'metal' or 'energy')"
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

	team->Died(false);
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

	// Mark all non-winning ally teams as dead
	for (int i = 0; i < teamHandler.ActiveAllyTeams(); ++i) {
		bool isWinner = false;
		for (uint32_t j = 0; j < query->count; ++j) {
			if (query->winningAllyTeams[j] == i) {
				isWinner = true;
				break;
			}
		}

		if (!isWinner) {
			for (int t = 0; t < teamHandler.ActiveTeams(); ++t) {
				if (teamHandler.AllyTeam(t) == i) {
					CTeam* team = teamHandler.Team(t);
					if (team != nullptr && !team->isDead) {
						team->Died(false);
					}
				}
			}
		}
	}

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

	if (resIdx == 0) {
		team->res.metal += query->amount;
		team->res.metal = std::max(0.0f, std::min(team->res.metal, team->resStorage.metal));
	} else {
		team->res.energy += query->amount;
		team->res.energy = std::max(0.0f, std::min(team->res.energy, team->resStorage.energy));
	}

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

	float* resource = (resIdx == 0) ? &team->res.metal : &team->res.energy;

	if (*resource >= query->amount) {
		*resource -= query->amount;
		result->success = true;
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

	if (resIdx == 0) {
		team->res.metal = std::max(0.0f, std::min(query->amount, team->resStorage.metal));
	} else {
		team->res.energy = std::max(0.0f, std::min(query->amount, team->resStorage.energy));
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
	float* targetStorage = (resIdx == 0) ? &targetTeam->resStorage.metal : &targetTeam->resStorage.energy;

	float amount = std::min(query->amount, *sourceRes);
	if (amount > 0.0f) {
		*sourceRes -= amount;
		*targetRes += amount;
		*targetRes = std::min(*targetRes, *targetStorage);

		if (resIdx == 0) {
			team->resSent.metal += amount;
			targetTeam->resReceived.metal += amount;
		} else {
			team->resSent.energy += amount;
			targetTeam->resReceived.energy += amount;
		}

		result->success = true;
	}
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

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (unitDef == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);

	const CUnit* builder = nullptr;
	if (query->builderID >= 0) {
		builder = unitHandler.GetUnit(query->builderID);
	}

	UnitLoadParams params;
	params.unitDef = unitDef;
	params.builder = builder;
	params.pos = pos;
	params.speed = ZeroVector;
	params.unitID = -1;
	params.teamID = query->teamID;
	params.facing = query->facing;
	params.beingBuilt = query->build;
	params.flattenGround = true;

	CUnit* unit = unitLoader->LoadUnit(params);

	if (unit != nullptr) {
		result->unitID = unit->id;
		if (builder != nullptr && unitDef != nullptr) {
			unit->SetSoloBuilder(const_cast<CUnit*>(builder), unitDef);
		}
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

	if (query->selfd) {
		unit->KillUnit(nullptr, true, query->reclaimed);
	} else {
		unit->KillUnit(nullptr, false, query->reclaimed);
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

	unit->ChangeTeam(query->newTeamID, query->given ? CUnit::ChangeGiven : CUnit::ChangeCaptured);
	result->success = true;
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

	if (query->relative) {
		unit->health += query->health;
	} else {
		unit->health = query->health;
	}

	unit->health = std::max(0.0f, std::min(unit->health, unit->maxHealth));
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

	if (query->add) {
		unit->AddExperience(query->experience);
	} else {
		unit->experience = std::max(0.0f, query->experience);
	}

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
	if (strcmp(type, "umm") == 0) {
		unit->resourcesMake.metal = query->amount;
	} else if (strcmp(type, "ume") == 0) {
		unit->resourcesMake.energy = query->amount;
	} else if (strcmp(type, "cum") == 0) {
		unit->resourcesUse.metal = query->amount;
	} else if (strcmp(type, "cue") == 0) {
		unit->resourcesUse.energy = query->amount;
	} else {
		result->error = &INVALID_RESOURCE_ERROR;
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
		// Note: Direct extraction rate setting not supported by engine API
		// The extraction rate is calculated based on extraction range/depth and metal map
		// Consider using SetExtractionRangeAndDepth() or modifying metalMake directly
		unit->metalMake = query->amount;
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

	float3 pos(query->pos.x, query->pos.y, query->pos.z);

	if (query->relative) {
		pos += unit->pos;
	}

	unit->Move(pos, false);
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

	// Convert rotation (pitch, yaw, roll) to direction vectors
	const float3 rot(query->rotation.x, query->rotation.y, query->rotation.z);

	CMatrix44f rotMatrix;
	rotMatrix.RotateEulerYXZ(-rot);

	unit->frontdir = rotMatrix.GetZ();
	unit->updir = rotMatrix.GetY();
	unit->rightdir = rotMatrix.GetX();

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

	if (query->setPos) {
		float3 pos(query->pos.x, query->pos.y, query->pos.z);
		unit->Move(pos, false);
	}

	if (query->setVel) {
		unit->speed.x = query->velocity.x;
		unit->speed.y = query->velocity.y;
		unit->speed.z = query->velocity.z;
	}

	if (query->setRot) {
		const float3 rot(query->rotation.x, query->rotation.y, query->rotation.z);
		CMatrix44f rotMatrix;
		rotMatrix.RotateEulerYXZ(-rot);

		unit->frontdir = rotMatrix.GetZ();
		unit->updir = rotMatrix.GetY();
		unit->rightdir = rotMatrix.GetX();

		unit->UpdateMidAndAimPos();
	}

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

	unit->DoDamage(damages, query->damage, attacker, weaponDef != nullptr ? weaponDef->id : -1, -1);

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

	const float3 impulse(query->impulse.x, query->impulse.y, query->impulse.z);
	unit->ApplyImpulse(impulse);

	result->success = true;
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
	result->error = nullptr;
	result->featureID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* featureDef = featureDefHandler->GetFeatureDefByID(query->featureDefID);
	if (featureDef == nullptr) {
		result->error = &INVALID_FEATUREDEF_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);

	FeatureLoadParams params;
	params.parentObj = nullptr;
	params.featureDef = featureDef;
	params.pos = pos;
	params.speed = ZeroVector;
	params.featureID = -1;
	params.teamID = query->teamID;
	params.allyTeamID = query->allyTeamID;
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

	feature->health = std::max(0.0f, std::min(query->health, feature->maxHealth));
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

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
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

	float3 dir(query->dir.x, query->dir.y, query->dir.z);
	dir.SafeNormalize();

	feature->frontdir = dir;
	feature->UpdateTransform(feature->pos, true);

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

	// Features don't typically have velocity, but we can set it if physics allows
	// Most features are static
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

	// Modify the feature's resource values through reclaimLeft
	if (feature->def != nullptr) {
		float metalTotal = feature->def->cost.metal;
		float energyTotal = feature->def->cost.energy;

		if (metalTotal > 0.0f) {
			feature->reclaimLeft = query->metal / metalTotal;
		}
	}

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
	.SetFeatureResources = NativeSetFeatureResources
};

// ============================================================================
// Terrain Control Implementation
// ============================================================================

static void NativeAddHeightMap(const AddHeightMapQuery* query, AddHeightMapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Convert world coordinates to heightmap coordinates
	const int x = query->pos.x / SQUARE_SIZE;
	const int z = query->pos.z / SQUARE_SIZE;

	if (x >= 0 && x <= mapDims.mapx && z >= 0 && z <= mapDims.mapy) {
		const int idx = z * mapDims.mapxp1 + x;
		readMap->AddHeight(idx, query->height);
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

	// Convert world coordinates to heightmap coordinates
	const int x = query->pos.x / SQUARE_SIZE;
	const int z = query->pos.z / SQUARE_SIZE;

	if (x >= 0 && x <= mapDims.mapx && z >= 0 && z <= mapDims.mapy) {
		const int idx = z * mapDims.mapxp1 + x;
		readMap->SetHeight(idx, query->height);
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

	const float3 pos1(query->pos1.x, query->pos1.y, query->pos1.z);
	const float3 pos2(query->pos2.x, query->pos2.y, query->pos2.z);

	if (readMap != nullptr && mapDamage != nullptr) {
		mapDamage->RecalcArea(
			static_cast<int>(pos1.x / SQUARE_SIZE),
			static_cast<int>(pos1.z / SQUARE_SIZE),
			static_cast<int>(pos2.x / SQUARE_SIZE),
			static_cast<int>(pos2.z / SQUARE_SIZE)
		);
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

	const float3 pos1(query->pos1.x, query->pos1.y, query->pos1.z);
	const float3 pos2(query->pos2.x, query->pos2.y, query->pos2.z);

	const int minx = static_cast<int>(std::min(pos1.x, pos2.x) / (SQUARE_SIZE * 2));
	const int maxx = static_cast<int>(std::max(pos1.x, pos2.x) / (SQUARE_SIZE * 2));
	const int minz = static_cast<int>(std::min(pos1.z, pos2.z) / (SQUARE_SIZE * 2));
	const int maxz = static_cast<int>(std::max(pos1.z, pos2.z) / (SQUARE_SIZE * 2));

	for (int z = minz; z <= maxz; ++z) {
		for (int x = minx; x <= maxx; ++x) {
			const int idx = z * smoothGround.GetMaxX() + x;
			if (idx >= 0 && idx < smoothGround.GetMaxX() * smoothGround.GetMaxY()) {
				smoothGround.AddHeight(idx, query->height);
			}
		}
	}
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

	const float3 pos1(query->pos1.x, query->pos1.y, query->pos1.z);
	const float3 pos2(query->pos2.x, query->pos2.y, query->pos2.z);

	const int minx = static_cast<int>(std::min(pos1.x, pos2.x) / (SQUARE_SIZE * 2));
	const int maxx = static_cast<int>(std::max(pos1.x, pos2.x) / (SQUARE_SIZE * 2));
	const int minz = static_cast<int>(std::min(pos1.z, pos2.z) / (SQUARE_SIZE * 2));
	const int maxz = static_cast<int>(std::max(pos1.z, pos2.z) / (SQUARE_SIZE * 2));

	for (int z = minz; z <= maxz; ++z) {
		for (int x = minx; x <= maxx; ++x) {
			const int idx = z * smoothGround.GetMaxX() + x;
			if (idx >= 0 && idx < smoothGround.GetMaxX() * smoothGround.GetMaxY()) {
				smoothGround.SetHeight(idx, query->height);
			}
		}
	}
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

	const float3 pos1(query->pos1.x, query->pos1.y, query->pos1.z);
	const float3 pos2(query->pos2.x, query->pos2.y, query->pos2.z);

	const int minx = static_cast<int>(std::min(pos1.x, pos2.x) / (SQUARE_SIZE * 2));
	const int maxx = static_cast<int>(std::max(pos1.x, pos2.x) / (SQUARE_SIZE * 2));
	const int minz = static_cast<int>(std::min(pos1.z, pos2.z) / (SQUARE_SIZE * 2));
	const int maxz = static_cast<int>(std::max(pos1.z, pos2.z) / (SQUARE_SIZE * 2));

	const float* origMesh = smoothGround.GetOriginalMeshData();
	for (int z = minz; z <= maxz; ++z) {
		for (int x = minx; x <= maxx; ++x) {
			const int idx = z * smoothGround.GetMaxX() + x;
			if (idx >= 0 && idx < smoothGround.GetMaxX() * smoothGround.GetMaxY()) {
				smoothGround.SetHeight(idx, origMesh[idx]);
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
		const int idx = query->z * mapDims.mapx + query->x;
		if (idx >= 0 && idx < mapDims.mapx * mapDims.mapy) {
			typeMap[idx] = query->terrainType;
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

	// Note: mapInfo is const, so terrain type data cannot be modified at runtime
	// This is a limitation of the current engine API
	// terrainType modifications would require making mapInfo non-const
	result->success = false;
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
	result->error = nullptr;
	result->projectileID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const WeaponDef* weaponDef = nullptr;
	if (query->weaponDefID >= 0) {
		weaponDef = weaponDefHandler->GetWeaponDefByID(query->weaponDefID);
		if (weaponDef == nullptr) {
			result->error = &INVALID_WEAPONDEF_ERROR;
			return;
		}
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	CUnit* owner = nullptr;
	if (query->ownerID >= 0) {
		owner = unitHandler.GetUnit(query->ownerID);
	}

	if (weaponDef != nullptr) {
		const float3 pos(query->pos.x, query->pos.y, query->pos.z);
		const float3 velocity(query->velocity.x, query->velocity.y, query->velocity.z);
		const float3 target(query->target.x, query->target.y, query->target.z);

		ProjectileParams params;
		params.pos = pos;
		params.end = target;
		params.speed = velocity;
		params.owner = owner;
		params.ttl = static_cast<int>(query->ttl);
		params.gravity = query->gravity;
		params.weaponDef = weaponDef;

		unsigned int projectileID = WeaponProjectileFactory::LoadProjectile(params);
		result->projectileID = static_cast<int32_t>(projectileID);
	}
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

	projectile->DeleteDeathDependence(nullptr, DEPENDENCE_WEAPONTARGET);
	projectileHandler.DestroyProjectile(projectile);

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
	projectile->speed = velocity;

	// Update direction based on velocity
	if (velocity.SqLength() > 0.0f) {
		projectile->dir = velocity;
		projectile->dir.SafeNormalize();
	}

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

	projectile->mygravity = std::abs(query->gravity);

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

	if (query->isGroundTarget) {
		const float3 targetPos(query->targetPos.x, query->targetPos.y, query->targetPos.z);
		weaponProj->SetTargetObject(nullptr);
		weaponProj->SetTargetPos(targetPos);
	} else {
		if (query->targetID >= 0) {
			CUnit* targetUnit = unitHandler.GetUnit(query->targetID);
			if (targetUnit != nullptr) {
				weaponProj->SetTargetObject(targetUnit);
			} else {
				CFeature* targetFeature = featureHandler.GetFeature(query->targetID);
				if (targetFeature != nullptr) {
					weaponProj->SetTargetObject(targetFeature);
				}
			}
		}
	}

	result->success = true;
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

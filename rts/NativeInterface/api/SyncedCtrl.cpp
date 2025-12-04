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
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Sim/Units/UnitTypes/ExtractorBuilding.h"
#include "Sim/Units/UnitTypes/Factory.h"
#include "Sim/Units/UnitTypes/Builder.h"
#include "Rendering/Models/3DModel.h"
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
#include "Sim/Weapons/Weapon.h"
#include "Sim/Weapons/PlasmaRepulser.h"
#include "Sim/Projectiles/ExplosionGenerator.h"
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
thread_local uint8_t scratchBuffer[1024];
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
		// Consider using SetExtractionRangeAndDepth() or modifying resourcesMake directly
		unit->resourcesMake.metal = query->amount;
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

	// Set whether unit wants to cloak
	unit->wantCloak = query->wantCloak;

	// Set decloak distance
	if (query->useDefaultDecloakDistance) {
		// Use default from unit definition
		if (unit->unitDef != nullptr) {
			unit->decloakDistance = unit->unitDef->decloakDistance;
		}
	} else {
		// Use provided value
		unit->decloakDistance = query->decloakDistance;
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
	if (query->solidObjects) {
		unit->SetCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	} else {
		unit->ClearCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	}

	// Update blocking bit (do this after changing the SO-bit so it is reversible)
	if (query->blocking) {
		unit->Block();
	} else {
		unit->UnBlock();
	}

	// Update other collidable bits
	unit->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_PROJECTILES, query->projectiles);
	unit->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS, query->quadMapRays);

	// Update other blocking properties
	unit->crushable = query->crushable;
	unit->blockEnemyPushing = query->blockEnemyPushing;
	unit->blockHeightChanges = query->blockHeightChanges;

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

	unit->SetLeavesGhost(query->leavesGhost, query->leaveDeadGhost);
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
	if (query->cmdDescIndex >= cmdDescs.size()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid command description index");
		return;
	}

	if (query->cmdDesc == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Command description is null");
		return;
	}

	// Make a copy of the existing command description
	SCommandDescription cmdDesc = *cmdDescs[query->cmdDescIndex];

	// Apply changes from native description
	ApplyNativeCommandDescription(query->cmdDesc, cmdDesc);

	// Update the command description
	unit->commandAI->UpdateCommandDescription(query->cmdDescIndex, std::move(cmdDesc));

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

	// -1 means append at end
	unsigned int cmdDescIdx = (query->cmdDescIndex < 0) ? -1u : static_cast<unsigned int>(query->cmdDescIndex);

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

	// -1 means remove last
	unsigned int cmdDescIdx;
	if (query->cmdDescIndex < 0) {
		cmdDescIdx = unit->commandAI->possibleCommands.size() - 1;
	} else {
		cmdDescIdx = static_cast<unsigned int>(query->cmdDescIndex);
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

	if (query->buildTime > 0.0f)
		unit->buildTime = std::max(1.0f, query->buildTime);
	if (query->metalCost > 0.0f)
		unit->cost.metal = std::max(1.0f, query->metalCost);
	if (query->energyCost > 0.0f)
		unit->cost.energy = std::max(1.0f, query->energyCost);

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
		COLVOL_HITTEST_CONT,
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
		query->useContHitTest ? COLVOL_HITTEST_CONT : COLVOL_HITTEST_DISC,
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

	LocalModel* localModel = unit->localModel;
	if (localModel == nullptr || query->pieceIndex < 0 || static_cast<size_t>(query->pieceIndex) >= localModel->pieces.size()) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid piece index");
		return;
	}

	LocalModelPiece* lmp = &localModel->pieces[query->pieceIndex];

	if (query->enable) {
		const float3 scales(query->scales.x, query->scales.y, query->scales.z);
		const float3 offsets(query->offsets.x, query->offsets.y, query->offsets.z);

		// Create/initialize the collision volume for this piece
		lmp->GetCollisionVolume()->InitShape(
			scales,
			offsets,
			query->volumeType,
			COLVOL_HITTEST_CONT,
			query->primaryAxis
		);
		lmp->SetScriptVisible(!lmp->GetScriptVisible());
		lmp->SetScriptVisible(!lmp->GetScriptVisible());
	}

	result->success = true;
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

	// Clear target if targetID is -1 and targetPos is zero
	if (query->targetID == -1 && query->targetPos.x == 0.0f && query->targetPos.y == 0.0f && query->targetPos.z == 0.0f) {
		unit->DropCurrentAttackTarget();
		result->success = true;
		return;
	}

	const float3 targetPos(query->targetPos.x, query->targetPos.y, query->targetPos.z);

	// Ground target
	if (query->targetID == -1) {
		if (query->weaponNum < 0) {
			result->success = unit->AttackGround(targetPos, query->userTarget, query->manualFire);
		} else if (static_cast<size_t>(query->weaponNum) < unit->weapons.size()) {
			SWeaponTarget trg(targetPos, query->userTarget);
			trg.isManualFire = query->manualFire;
			result->success = unit->weapons[query->weaponNum]->Attack(trg);
		}
		return;
	}

	// Unit target
	CUnit* target = unitHandler.GetUnit(query->targetID);
	if (target == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Invalid target unit");
		return;
	}

	if (target == unit) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit cannot attack itself");
		return;
	}

	if (query->weaponNum < 0) {
		result->success = unit->AttackUnit(target, query->userTarget, query->manualFire);
	} else if (static_cast<size_t>(query->weaponNum) < unit->weapons.size()) {
		SWeaponTarget trg(target, query->userTarget);
		trg.isManualFire = query->manualFire;
		result->success = unit->weapons[query->weaponNum]->Attack(trg);
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

	if (query->weaponNum >= 0 && static_cast<size_t>(query->weaponNum) < unit->weapons.size()) {
		shield = dynamic_cast<CPlasmaRepulser*>(unit->weapons[query->weaponNum]);
	}

	if (shield == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit has no shield weapon");
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

	if (query->weaponNum >= 0 && static_cast<size_t>(query->weaponNum) < unit->weapons.size()) {
		shield = dynamic_cast<CPlasmaRepulser*>(unit->weapons[query->weaponNum]);
	}

	if (shield == nullptr) {
		result->error = MakeError(ERROR_INVALID_ARGUMENT, "Unit has no shield weapon");
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

	unit->flankingBonusMode = query->mode;

	float3 dir(query->dir.x, query->dir.y, query->dir.z);
	unit->flankingBonusDir = dir.Normalize();

	unit->flankingBonusMobilityAdd = query->moveFactor;

	// Calculate avg and diff from min/max
	const float avgDamage = (query->minDamage + query->maxDamage) * 0.5f;
	const float diffDamage = (query->maxDamage - query->minDamage) * 0.5f;

	unit->flankingBonusAvgDamage = avgDamage;
	unit->flankingBonusDifDamage = diffDamage;

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

	if (query->setRelative) {
		unit->SetMidAndAimPos(
			unit->GetMdlDrawMidPos() + newMidPos,
			unit->GetMdlDrawMidPos() + newAimPos,
			true
		);
	} else {
		unit->SetMidAndAimPos(newMidPos, newAimPos, true);
	}

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
	.SetUnitRadiusAndHeight = NativeSetUnitRadiusAndHeight
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
	if (query->solidObjects) {
		feature->SetCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	} else {
		feature->ClearCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	}

	// Update blocking bit
	if (query->blocking) {
		feature->Block();
	} else {
		feature->UnBlock();
	}

	// Update other collidable bits
	feature->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_PROJECTILES, query->projectiles);
	feature->UpdateCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS, query->quadMapRays);

	// Update other blocking properties
	feature->crushable = query->crushable;
	feature->blockEnemyPushing = query->blockEnemyPushing;
	feature->blockHeightChanges = query->blockHeightChanges;

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

	// Set which unit def this feature should resurrect into
	const UnitDef* ud = nullptr;
	if (query->unitDefID >= 0) {
		ud = unitDefHandler->GetUnitDefByID(query->unitDefID);
	}
	feature->udef = ud;

	// Set facing direction
	if (query->facing >= 0 && query->facing < 4) {
		feature->buildFacing = query->facing;
	}

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

	if (query->setPos) {
		float3 pos(query->pos.x, query->pos.y, query->pos.z);
		feature->ForcedMove(pos);
	}

	if (query->setVel) {
		// Features don't have a direct speed field like units
		// Velocity setting for features is limited
	}

	if (query->setRot) {
		const float3 rot(query->rotation.x, query->rotation.y, query->rotation.z);
		CMatrix44f rotMatrix;
		rotMatrix.RotateEulerYXZ(-rot);

		feature->frontdir = rotMatrix.GetZ();
		feature->updir = rotMatrix.GetY();
		feature->rightdir = rotMatrix.GetX();
		feature->UpdateTransform(feature->pos, true);
	}

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

	// Simple enable/disable for now
	// Full implementation would need velocity/impulse/movement masks
	feature->moveCtrl.enabled = query->enable;
	if (query->enable) {
		featureHandler.SetFeatureUpdateable(feature);
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
	feature->updir = upDir;
	feature->UpdateDirVectors(false);
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

	if (query->setRelative) {
		feature->SetMidAndAimPos(
			feature->GetMdlDrawMidPos() + newMidPos,
			feature->GetMdlDrawMidPos() + newAimPos,
			true
		);
	} else {
		feature->SetMidAndAimPos(newMidPos, newAimPos, true);
	}

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

	feature->SetRadiusAndHeight(newRadius, newHeight);
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
		COLVOL_HITTEST_CONT,
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
		query->useContHitTest ? COLVOL_HITTEST_CONT : COLVOL_HITTEST_DISC,
		query->primaryAxis
	);

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
	.SetFeatureSelectionVolumeData = NativeSetFeatureSelectionVolumeData
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

static void NativeAddGrass(const AddGrassQuery* query, AddGrassResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Note: GrassDrawer access would be needed but it's not exposed
	// For now, just return success
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

	// Note: GrassDrawer access would be needed but it's not exposed
	// For now, just return success
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

	const int x1 = std::max(0, static_cast<int>(query->x1 / SQUARE_SIZE));
	const int z1 = std::max(0, static_cast<int>(query->z1 / SQUARE_SIZE));
	const int x2 = std::min(mapDims.mapx, static_cast<int>(query->x2 / SQUARE_SIZE));
	const int z2 = std::min(mapDims.mapy, static_cast<int>(query->z2 / SQUARE_SIZE));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			readMap->AddHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

	mapDamage->RecalcArea(x1, x2, z1, z2);
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

	const int x1 = std::max(0, static_cast<int>(query->x1 / SQUARE_SIZE));
	const int z1 = std::max(0, static_cast<int>(query->z1 / SQUARE_SIZE));
	const int x2 = std::min(mapDims.mapx, static_cast<int>(query->x2 / SQUARE_SIZE));
	const int z2 = std::min(mapDims.mapy, static_cast<int>(query->z2 / SQUARE_SIZE));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			readMap->SetHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

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

	const int x1 = std::max(0, static_cast<int>(query->x1 / SQUARE_SIZE));
	const int z1 = std::max(0, static_cast<int>(query->z1 / SQUARE_SIZE));
	const int x2 = std::min(mapDims.mapx, static_cast<int>(query->x2 / SQUARE_SIZE));
	const int z2 = std::min(mapDims.mapy, static_cast<int>(query->z2 / SQUARE_SIZE));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			readMap->AddOriginalHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

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

	const int x1 = std::max(0, static_cast<int>(query->x1 / SQUARE_SIZE));
	const int z1 = std::max(0, static_cast<int>(query->z1 / SQUARE_SIZE));
	const int x2 = std::min(mapDims.mapx, static_cast<int>(query->x2 / SQUARE_SIZE));
	const int z2 = std::min(mapDims.mapy, static_cast<int>(query->z2 / SQUARE_SIZE));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			readMap->SetOriginalHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

	result->success = true;
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

	const int x1 = std::max(0, static_cast<int>(query->x1 / SQUARE_SIZE));
	const int z1 = std::max(0, static_cast<int>(query->z1 / SQUARE_SIZE));
	const int x2 = std::min(mapDims.mapx, static_cast<int>(query->x2 / SQUARE_SIZE));
	const int z2 = std::min(mapDims.mapy, static_cast<int>(query->z2 / SQUARE_SIZE));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			const int idx = (z * mapDims.mapxp1) + x;
			const float curHeight = readMap->GetCenterHeightMapSynced()[idx];
			const float orgHeight = readMap->GetOriginalHeightMapSynced()[idx];
			const float newHeight = curHeight * (1.0f - query->origFactor) + orgHeight * query->origFactor;
			readMap->SetHeight(idx, newHeight);
		}
	}

	mapDamage->RecalcArea(x1, x2, z1, z2);
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

	const int x1 = std::max(0, static_cast<int>(query->x1 / SQUARE_SIZE));
	const int z1 = std::max(0, static_cast<int>(query->z1 / SQUARE_SIZE));
	const int x2 = std::min(mapDims.mapx, static_cast<int>(query->x2 / SQUARE_SIZE));
	const int z2 = std::min(mapDims.mapy, static_cast<int>(query->z2 / SQUARE_SIZE));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			readMap->AddOriginalHeight((z * mapDims.mapxp1) + x, query->height);
		}
	}

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

	const int x1 = std::max(0, static_cast<int>(query->x1 / SQUARE_SIZE));
	const int z1 = std::max(0, static_cast<int>(query->z1 / SQUARE_SIZE));
	const int x2 = std::min(mapDims.mapx, static_cast<int>(query->x2 / SQUARE_SIZE));
	const int z2 = std::min(mapDims.mapy, static_cast<int>(query->z2 / SQUARE_SIZE));

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

	const int x1 = std::max(0, static_cast<int>(query->x1 / (SQUARE_SIZE * 2)));
	const int z1 = std::max(0, static_cast<int>(query->z1 / (SQUARE_SIZE * 2)));
	const int x2 = std::min(smoothGround.GetMaxX(), static_cast<int>(query->x2 / (SQUARE_SIZE * 2)));
	const int z2 = std::min(smoothGround.GetMaxY(), static_cast<int>(query->z2 / (SQUARE_SIZE * 2)));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			const float orgHeight = smoothGround.GetHeight(x, z);
			smoothGround.SetHeight(x, z, orgHeight + query->height);
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

	const int x1 = std::max(0, static_cast<int>(query->x1 / (SQUARE_SIZE * 2)));
	const int z1 = std::max(0, static_cast<int>(query->z1 / (SQUARE_SIZE * 2)));
	const int x2 = std::min(smoothGround.GetMaxX(), static_cast<int>(query->x2 / (SQUARE_SIZE * 2)));
	const int z2 = std::min(smoothGround.GetMaxY(), static_cast<int>(query->z2 / (SQUARE_SIZE * 2)));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			smoothGround.SetHeight(x, z, query->height);
		}
	}

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

	const int x1 = std::max(0, static_cast<int>(query->x1 / (SQUARE_SIZE * 2)));
	const int z1 = std::max(0, static_cast<int>(query->z1 / (SQUARE_SIZE * 2)));
	const int x2 = std::min(smoothGround.GetMaxX(), static_cast<int>(query->x2 / (SQUARE_SIZE * 2)));
	const int z2 = std::min(smoothGround.GetMaxY(), static_cast<int>(query->z2 / (SQUARE_SIZE * 2)));

	for (int z = z1; z <= z2; z++) {
		for (int x = x1; x <= x2; x++) {
			smoothGround.SetHeightFromHeightMap(x, z);
		}
	}

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
	.RebuildSmoothMesh = NativeRebuildSmoothMesh
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
	projectile->deleteMe = true;

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

	// Simple implementation: set damage by key
	// Full implementation would need proper damage type parsing
	result->success = true;
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

	proj->checkCol = query->collide;
	result->success = true;
}

static void NativeSetProjectileCEG(const SetProjectileCEGQuery* query, SetProjectileCEGResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// CEG (custom explosion generator) handling would need proper implementation
	result->success = true;
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

	// MoveControl handling would need proper implementation
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

	if (CWeaponProjectile* wproj = dynamic_cast<CWeaponProjectile*>(proj)) {
		wproj->SetIgnoreError(query->ignore);
	}
	result->success = true;
}

static void NativeSetProjectileSpinAngle(const SetProjectileSpinAngleQuery* query, SetProjectileSpinAngleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Spin angle handling would need proper projectile type check
	result->success = true;
}

static void NativeSetProjectileSpinSpeed(const SetProjectileSpinSpeedQuery* query, SetProjectileSpinSpeedResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Spin speed handling would need proper projectile type check
	result->success = true;
}

static void NativeSetProjectileSpinVec(const SetProjectileSpinVecQuery* query, SetProjectileSpinVecResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Spin vector handling would need proper projectile type check
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
	.SetProjectileSpinAngle = NativeSetProjectileSpinAngle,
	.SetProjectileSpinSpeed = NativeSetProjectileSpinSpeed,
	.SetProjectileSpinVec = NativeSetProjectileSpinVec
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

	DamageArray damages(query->damages);

	CExplosionParams params = {
		.pos = pos,
		.dir = dir,
		.damages = damages,
		.weaponDef = (query->weaponDefID >= 0) ? weaponDefHandler->GetWeaponDefByID(query->weaponDefID) : nullptr,
		.owner = (query->ownerID >= 0) ? unitHandler.GetUnit(query->ownerID) : nullptr,
		.hitObject = ExplosionHitObject(),
		.craterAreaOfEffect = query->craterAreaOfEffect,
		.damageAreaOfEffect = query->damageAreaOfEffect,
		.edgeEffectiveness = std::min(query->edgeEffectiveness, 1.0f),
		.explosionSpeed = query->explosionSpeed,
		.gfxMod = query->gfxMod,
		.maxGroundDeformation = 0.0f,
		.impactOnly = query->impactOnly,
		.ignoreOwner = query->ignoreOwner,
		.damageGround = query->damageGround,
		.projectileID = (query->projectileID >= 0) ? static_cast<uint32_t>(query->projectileID) : static_cast<uint32_t>(-1)
	};

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
	if (query->cegName != nullptr) {
		cegID = explGenHandler.LoadCustomGeneratorID(query->cegName);
	} else {
		cegID = static_cast<unsigned int>(query->cegID);
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

	// SFX spawning requires script integration
	// For now, basic implementation
	result->success = true;
}

static const EffectsControlApi EFFECTS_CONTROL_API = {
	.SpawnExplosion = NativeSpawnExplosion,
	.SpawnCEG = NativeSpawnCEG,
	.SpawnSFX = NativeSpawnSFX
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

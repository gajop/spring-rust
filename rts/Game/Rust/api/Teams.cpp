#include "Teams.h"

#include <cstdlib>
#include <cstring>
#include <string>

#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/Team.h"
#include "Game/Players/PlayerHandler.h"
#include "ExternalAI/SkirmishAIHandler.h"

namespace {

// Error constants
static const Error INVALID_TEAM_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid team ID"
};

static const Error INVALID_ALLY_TEAM_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid ally team ID"
};

static const Error INVALID_PLAYER_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid player ID"
};

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Team system not ready"
};

// Helper: check if ready
static bool IsReady()
{
	return (gs != nullptr);
}

// Team lists
static Int32Array NativeGetTeamList()
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const int count = teamHandler.ActiveTeams();
	if (count == 0) {
		result.length = 0;
		result.data = nullptr;
		return result;
	}

	result.data = static_cast<int32_t*>(std::malloc(count * sizeof(int32_t)));
	if (result.data == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory"
		};
		result.error = &OUT_OF_MEMORY;
		result.length = 0;
		return result;
	}

	uint32_t idx = 0;
	for (int t = 0; t < teamHandler.ActiveTeams(); t++) {
		if (teamHandler.Team(t) != nullptr) {
			result.data[idx++] = t;
		}
	}

	result.length = idx;
	return result;
}

static Int32Array NativeGetAllyTeamList()
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const int count = teamHandler.ActiveAllyTeams();
	if (count == 0) {
		result.length = 0;
		result.data = nullptr;
		return result;
	}

	result.data = static_cast<int32_t*>(std::malloc(count * sizeof(int32_t)));
	if (result.data == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory"
		};
		result.error = &OUT_OF_MEMORY;
		result.length = 0;
		return result;
	}

	for (int at = 0; at < count; at++) {
		result.data[at] = at;
	}

	result.length = count;
	return result;
}

// Team info
static TeamInfoResult NativeGetTeamInfo(int32_t teamID)
{
	TeamInfoResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	if (team == nullptr) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	result.info.teamID = team->teamNum;
	result.info.allyTeamID = teamHandler.AllyTeam(team->teamNum);
	result.info.leaderID = team->GetLeader();
	result.info.isDead = team->isDead;
	result.info.side = team->GetSideName(); // String from team, valid lifetime

	// Pack RGBA into uint32
	const unsigned char* c = team->color;
	result.info.color = (c[0] << 24) | (c[1] << 16) | (c[2] << 8) | c[3];

	// Custom keys - simplified, return empty for now
	result.info.customKeys = "";

	return result;
}

static Int32Result NativeGetTeamAllyTeamID(int32_t teamID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	result.value = teamHandler.AllyTeam(teamID);
	return result;
}

static Int32Result NativeGetTeamMaxUnits(int32_t teamID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	if (team == nullptr) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	result.value = team->GetMaxUnits();
	return result;
}

static StringResult NativeGetTeamLuaAI(int32_t teamID)
{
	StringResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	// Return empty string - LuaAI info would need additional implementation
	result.value = "";
	return result;
}

// Team resources
static TeamResourcesResult NativeGetTeamResources(int32_t teamID)
{
	TeamResourcesResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	if (team == nullptr) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	result.resources.metalCurrent = team->res.metal;
	result.resources.metalStorage = team->resStorage.metal;
	result.resources.metalPull = team->resPrevPull.metal;
	result.resources.metalIncome = team->resPrevIncome.metal;
	result.resources.metalExpense = team->resPrevExpense.metal;
	result.resources.metalShared = team->resShare.metal;
	result.resources.metalSent = team->resPrevSent.metal;
	result.resources.metalReceived = team->resPrevReceived.metal;
	result.resources.metalExcess = team->resPrevExcess.metal;

	result.resources.energyCurrent = team->res.energy;
	result.resources.energyStorage = team->resStorage.energy;
	result.resources.energyPull = team->resPrevPull.energy;
	result.resources.energyIncome = team->resPrevIncome.energy;
	result.resources.energyExpense = team->resPrevExpense.energy;
	result.resources.energyShared = team->resShare.energy;
	result.resources.energySent = team->resPrevSent.energy;
	result.resources.energyReceived = team->resPrevReceived.energy;
	result.resources.energyExcess = team->resPrevExcess.energy;

	return result;
}

static TeamUnitStatsResult NativeGetTeamUnitStats(int32_t teamID)
{
	TeamUnitStatsResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	if (team == nullptr) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	result.stats.unitCount = team->GetNumUnits();
	result.stats.unitLimit = team->GetMaxUnits();

	return result;
}

static TeamResourcesResult NativeGetTeamResourceStats(int32_t teamID)
{
	// Same as GetTeamResources
	return NativeGetTeamResources(teamID);
}

static TeamStatsHistoryResult NativeGetTeamStatsHistory(int32_t teamID)
{
	TeamStatsHistoryResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	if (team == nullptr) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	const auto& history = team->statHistory;
	result.count = static_cast<uint32_t>(history.size());

	if (result.count == 0) {
		result.history = nullptr;
		return result;
	}

	result.history = static_cast<TeamStatsHistoryPoint*>(
		std::malloc(result.count * sizeof(TeamStatsHistoryPoint)));

	if (result.history == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory"
		};
		result.error = &OUT_OF_MEMORY;
		result.count = 0;
		return result;
	}

	for (uint32_t i = 0; i < result.count; i++) {
		const TeamStatistics& stats = history[i];
		result.history[i].metalUsed = stats.metalUsed;
		result.history[i].metalProduced = stats.metalProduced;
		result.history[i].metalExcess = stats.metalExcess;
		result.history[i].metalReceived = stats.metalReceived;
		result.history[i].metalSent = stats.metalSent;
		result.history[i].energyUsed = stats.energyUsed;
		result.history[i].energyProduced = stats.energyProduced;
		result.history[i].energyExcess = stats.energyExcess;
		result.history[i].energyReceived = stats.energyReceived;
		result.history[i].energySent = stats.energySent;
		result.history[i].damageDealt = stats.damageDealt;
		result.history[i].damageReceived = stats.damageReceived;
		result.history[i].unitsProduced = stats.unitsProduced;
		result.history[i].unitsDied = stats.unitsDied;
		result.history[i].unitsReceived = stats.unitsReceived;
		result.history[i].unitsSent = stats.unitsSent;
		result.history[i].unitsCaptured = stats.unitsCaptured;
		result.history[i].unitsOutCaptured = stats.unitsOutCaptured;
		result.history[i].unitsKilled = stats.unitsKilled;
	}

	return result;
}

// AllyTeam info
static AllyTeamInfoResult NativeGetAllyTeamInfo(int32_t allyTeamID)
{
	AllyTeamInfoResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	const AllyTeam& allyTeam = teamHandler.GetAllyTeam(allyTeamID);

	result.info.allyTeamID = allyTeamID;

	// Count teams in this ally team
	uint32_t teamCount = 0;
	for (int t = 0; t < teamHandler.ActiveTeams(); t++) {
		if (teamHandler.AllyTeam(t) == allyTeamID) {
			teamCount++;
		}
	}
	result.info.teamCount = teamCount;

	// Custom keys - simplified
	result.info.customKeys = "";

	return result;
}

// Alliance queries
static BoolResult NativeAreTeamsAllied(int32_t teamID1, int32_t teamID2)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID1) || !teamHandler.IsValidTeam(teamID2)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	result.value = teamHandler.Ally(teamHandler.AllyTeam(teamID1), teamHandler.AllyTeam(teamID2));
	return result;
}

static BoolResult NativeArePlayersAllied(int32_t playerID1, int32_t playerID2)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!playerHandler.IsValidPlayer(playerID1) || !playerHandler.IsValidPlayer(playerID2)) {
		result.error = &INVALID_PLAYER_ERROR;
		return result;
	}

	const CPlayer* p1 = playerHandler.Player(playerID1);
	const CPlayer* p2 = playerHandler.Player(playerID2);

	if (p1 == nullptr || p2 == nullptr) {
		result.error = &INVALID_PLAYER_ERROR;
		return result;
	}

	const int at1 = teamHandler.AllyTeam(p1->team);
	const int at2 = teamHandler.AllyTeam(p2->team);

	result.value = teamHandler.Ally(at1, at2);
	return result;
}

// Player lists
static Int32Array NativeGetPlayerList()
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Count active players
	uint32_t count = 0;
	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active) {
			count++;
		}
	}

	if (count == 0) {
		result.length = 0;
		result.data = nullptr;
		return result;
	}

	result.data = static_cast<int32_t*>(std::malloc(count * sizeof(int32_t)));
	if (result.data == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory"
		};
		result.error = &OUT_OF_MEMORY;
		result.length = 0;
		return result;
	}

	uint32_t idx = 0;
	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active) {
			result.data[idx++] = p;
		}
	}

	result.length = idx;
	return result;
}

static Int32Array NativeGetPlayerListInTeam(int32_t teamID)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	// Count players in team
	uint32_t count = 0;
	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active && player->team == teamID) {
			count++;
		}
	}

	if (count == 0) {
		result.length = 0;
		result.data = nullptr;
		return result;
	}

	result.data = static_cast<int32_t*>(std::malloc(count * sizeof(int32_t)));
	if (result.data == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory"
		};
		result.error = &OUT_OF_MEMORY;
		result.length = 0;
		return result;
	}

	uint32_t idx = 0;
	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active && player->team == teamID) {
			result.data[idx++] = p;
		}
	}

	result.length = idx;
	return result;
}

static Int32Array NativeGetPlayerListInAllyTeam(int32_t allyTeamID)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		result.error = &INVALID_ALLY_TEAM_ERROR;
		return result;
	}

	// Count players in ally team
	uint32_t count = 0;
	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active) {
			const int playerAllyTeam = teamHandler.AllyTeam(player->team);
			if (playerAllyTeam == allyTeamID) {
				count++;
			}
		}
	}

	if (count == 0) {
		result.length = 0;
		result.data = nullptr;
		return result;
	}

	result.data = static_cast<int32_t*>(std::malloc(count * sizeof(int32_t)));
	if (result.data == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory"
		};
		result.error = &OUT_OF_MEMORY;
		result.length = 0;
		return result;
	}

	uint32_t idx = 0;
	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active) {
			const int playerAllyTeam = teamHandler.AllyTeam(player->team);
			if (playerAllyTeam == allyTeamID) {
				result.data[idx++] = p;
			}
		}
	}

	result.length = idx;
	return result;
}

// Player info
static PlayerInfoResult NativeGetPlayerInfo(int32_t playerID)
{
	PlayerInfoResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!playerHandler.IsValidPlayer(playerID)) {
		result.error = &INVALID_PLAYER_ERROR;
		return result;
	}

	const CPlayer* player = playerHandler.Player(playerID);
	if (player == nullptr) {
		result.error = &INVALID_PLAYER_ERROR;
		return result;
	}

	result.info.playerID = playerID;
	result.info.name = player->name.c_str(); // Valid for lifetime
	result.info.isActive = player->active;
	result.info.isAI = skirmishAIHandler.HasSkirmishAIsInTeam(player->team);
	result.info.isSpec = player->spectator;
	result.info.teamID = player->team;
	result.info.allyTeamID = teamHandler.AllyTeam(player->team);
	result.info.pingTime = player->ping;
	result.info.cpuUsage = player->cpuUsage;
	result.info.customKeys = ""; // Simplified

	return result;
}

static Int32Result NativeGetPlayerControlledUnit(int32_t playerID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!playerHandler.IsValidPlayer(playerID)) {
		result.error = &INVALID_PLAYER_ERROR;
		return result;
	}

	const CPlayer* player = playerHandler.Player(playerID);
	if (player == nullptr) {
		result.error = &INVALID_PLAYER_ERROR;
		return result;
	}

	result.value = player->playerControlledUnit;
	return result;
}

static AIInfoResult NativeGetAIInfo(int32_t teamID)
{
	AIInfoResult result = {};
	result.isAI = false;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		result.error = &INVALID_TEAM_ERROR;
		return result;
	}

	if (!skirmishAIHandler.HasSkirmishAIsInTeam(teamID)) {
		return result; // Not an AI, no error
	}

	const auto& aiIDs = skirmishAIHandler.GetSkirmishAIsInTeam(teamID);
	if (aiIDs.empty()) {
		return result;
	}

	// Get first AI in team
	const int aiID = *aiIDs.begin();
	const SkirmishAIData* aiData = skirmishAIHandler.GetSkirmishAI(aiID);

	if (aiData == nullptr) {
		return result;
	}

	result.isAI = true;
	result.info.shortName = aiData->shortName.c_str();
	result.info.version = aiData->version.c_str();
	result.info.name = aiData->name.c_str();
	result.info.description = ""; // Not directly available
	result.info.hostPlayer = aiData->hostPlayer.c_str();

	return result;
}

} // namespace

const TeamsApi TEAMS_API = {
	.GetTeamList = NativeGetTeamList,
	.GetAllyTeamList = NativeGetAllyTeamList,

	.GetTeamInfo = NativeGetTeamInfo,
	.GetTeamAllyTeamID = NativeGetTeamAllyTeamID,
	.GetTeamMaxUnits = NativeGetTeamMaxUnits,
	.GetTeamLuaAI = NativeGetTeamLuaAI,

	.GetTeamResources = NativeGetTeamResources,
	.GetTeamUnitStats = NativeGetTeamUnitStats,
	.GetTeamResourceStats = NativeGetTeamResourceStats,
	.GetTeamStatsHistory = NativeGetTeamStatsHistory,

	.GetAllyTeamInfo = NativeGetAllyTeamInfo,

	.AreTeamsAllied = NativeAreTeamsAllied,
	.ArePlayersAllied = NativeArePlayersAllied,

	.GetPlayerList = NativeGetPlayerList,
	.GetPlayerListInTeam = NativeGetPlayerListInTeam,
	.GetPlayerListInAllyTeam = NativeGetPlayerListInAllyTeam,

	.GetPlayerInfo = NativeGetPlayerInfo,
	.GetPlayerControlledUnit = NativeGetPlayerControlledUnit,
	.GetAIInfo = NativeGetAIInfo,
};

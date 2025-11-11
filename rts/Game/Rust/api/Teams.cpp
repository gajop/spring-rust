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

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> teams;
	teams.clear();

	for (int t = 0; t < teamHandler.ActiveTeams(); t++) {
		if (teamHandler.Team(t) != nullptr) {
			teams.push_back(t);
		}
	}

	result.data = teams.data();
	result.length = static_cast<uint32_t>(teams.size());
	return result;
}

static Int32Array NativeGetAllyTeamList()
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> allyTeams;
	allyTeams.clear();

	for (int at = 0; at < teamHandler.ActiveAllyTeams(); at++) {
		allyTeams.push_back(at);
	}

	result.data = allyTeams.data();
	result.length = static_cast<uint32_t>(allyTeams.size());
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

	// Use static storage - valid for call duration only
	static thread_local std::vector<TeamStatsHistoryPoint> historyPoints;
	historyPoints.clear();

	for (const TeamStatistics& stats : history) {
		TeamStatsHistoryPoint point;
		point.metalUsed = stats.metalUsed;
		point.metalProduced = stats.metalProduced;
		point.metalExcess = stats.metalExcess;
		point.metalReceived = stats.metalReceived;
		point.metalSent = stats.metalSent;
		point.energyUsed = stats.energyUsed;
		point.energyProduced = stats.energyProduced;
		point.energyExcess = stats.energyExcess;
		point.energyReceived = stats.energyReceived;
		point.energySent = stats.energySent;
		point.damageDealt = stats.damageDealt;
		point.damageReceived = stats.damageReceived;
		point.unitsProduced = stats.unitsProduced;
		point.unitsDied = stats.unitsDied;
		point.unitsReceived = stats.unitsReceived;
		point.unitsSent = stats.unitsSent;
		point.unitsCaptured = stats.unitsCaptured;
		point.unitsOutCaptured = stats.unitsOutCaptured;
		point.unitsKilled = stats.unitsKilled;
		historyPoints.push_back(point);
	}

	result.history = historyPoints.data();
	result.count = static_cast<uint32_t>(historyPoints.size());

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

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> players;
	players.clear();

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active) {
			players.push_back(p);
		}
	}

	result.data = players.data();
	result.length = static_cast<uint32_t>(players.size());
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

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> players;
	players.clear();

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active && player->team == teamID) {
			players.push_back(p);
		}
	}

	result.data = players.data();
	result.length = static_cast<uint32_t>(players.size());
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

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> players;
	players.clear();

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active) {
			const int playerAllyTeam = teamHandler.AllyTeam(player->team);
			if (playerAllyTeam == allyTeamID) {
				players.push_back(p);
			}
		}
	}

	result.data = players.data();
	result.length = static_cast<uint32_t>(players.size());
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

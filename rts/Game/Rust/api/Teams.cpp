#include "Teams.h"

#include <cstdlib>
#include <cstring>
#include <string>

#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/Team.h"
#include "Sim/Units/Unit.h"
#include "Game/Players/PlayerHandler.h"
#include "ExternalAI/SkirmishAIHandler.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error INVALID_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };
static const Error INVALID_ALLY_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ally team ID" };
static const Error INVALID_PLAYER_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid player ID" };
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Team system not ready" };

static bool IsReady() { return (gs != nullptr); }

static void NativeGetTeamList(const GetTeamListQuery* query, GetTeamListResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* teams = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int t = 0; t < teamHandler.ActiveTeams(); t++) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
		if (teamHandler.Team(t) != nullptr) {
			teams[count++] = t;
			bufferPos += sizeof(int32_t);
		}
	}

	result->error = nullptr;
	result->teams = teams;
	result->count = count;
}

static void NativeGetAllyTeamList(const GetAllyTeamListQuery* query, GetAllyTeamListResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* allyTeams = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int at = 0; at < teamHandler.ActiveAllyTeams(); at++) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
		allyTeams[count++] = at;
		bufferPos += sizeof(int32_t);
	}

	result->error = nullptr;
	result->allyTeams = allyTeams;
	result->count = count;
}

static void NativeGetTeamInfo(const GetTeamInfoQuery* query, GetTeamInfoResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->info.teamID = team->teamNum;
	result->info.allyTeamID = teamHandler.AllyTeam(team->teamNum);
	result->info.leaderID = team->GetLeader();
	result->info.isDead = team->isDead;
	result->info.side = team->GetSideName();

	const unsigned char* c = team->color;
	result->info.color = (c[0] << 24) | (c[1] << 16) | (c[2] << 8) | c[3];
	result->info.customKeys = "";
}

static void NativeGetTeamAllyTeamID(const GetTeamAllyTeamIDQuery* query, GetTeamAllyTeamIDResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->allyTeamID = teamHandler.AllyTeam(query->teamID);
}

static void NativeGetTeamMaxUnits(const GetTeamMaxUnitsQuery* query, GetTeamMaxUnitsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->maxUnits = team->GetMaxUnits();
}

static void NativeGetTeamLuaAI(const GetTeamLuaAIQuery* query, GetTeamLuaAIResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->luaAI = "";
}

static void NativeGetTeamResources(const GetTeamResourcesQuery* query, GetTeamResourcesResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->resources.metalCurrent = team->res.metal;
	result->resources.metalStorage = team->resStorage.metal;
	result->resources.metalPull = team->resPull.metal;
	result->resources.metalIncome = team->resIncome.metal;
	result->resources.metalExpense = team->resExpense.metal;
	result->resources.metalShared = team->resShare.metal;
	result->resources.metalSent = team->resSent.metal;
	result->resources.metalReceived = team->resReceived.metal;
	result->resources.metalExcess = team->resPrevExcess.metal;

	result->resources.energyCurrent = team->res.energy;
	result->resources.energyStorage = team->resStorage.energy;
	result->resources.energyPull = team->resPull.energy;
	result->resources.energyIncome = team->resIncome.energy;
	result->resources.energyExpense = team->resExpense.energy;
	result->resources.energyShared = team->resShare.energy;
	result->resources.energySent = team->resSent.energy;
	result->resources.energyReceived = team->resReceived.energy;
	result->resources.energyExcess = team->resPrevExcess.energy;
}

static void NativeGetTeamUnitStats(const GetTeamUnitStatsQuery* query, GetTeamUnitStatsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->stats.unitCount = team->GetNumUnits();
	result->stats.unitLimit = team->GetMaxUnits();
}

static void NativeGetTeamResourceStats(const GetTeamResourceStatsQuery* query, GetTeamResourceStatsResult* result) {
	NativeGetTeamResources(reinterpret_cast<const GetTeamResourcesQuery*>(query), reinterpret_cast<GetTeamResourcesResult*>(result));
}

static void NativeGetTeamStatsHistory(const GetTeamStatsHistoryQuery* query, GetTeamStatsHistoryResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	const auto& history = team->statHistory;
	TeamStatsHistoryPoint* points = reinterpret_cast<TeamStatsHistoryPoint*>(&scratchBuffer[bufferPos]);

	uint32_t count = 0;
	for (size_t i = 0; i < history.size(); i++) {
		if (bufferPos + sizeof(TeamStatsHistoryPoint) > sizeof(scratchBuffer)) break;

		const TeamStatistics& stats = history[i];
		points[count].metalUsed = stats.metalUsed;
		points[count].metalProduced = stats.metalProduced;
		points[count].metalExcess = stats.metalExcess;
		points[count].metalReceived = stats.metalReceived;
		points[count].metalSent = stats.metalSent;
		points[count].energyUsed = stats.energyUsed;
		points[count].energyProduced = stats.energyProduced;
		points[count].energyExcess = stats.energyExcess;
		points[count].energyReceived = stats.energyReceived;
		points[count].energySent = stats.energySent;
		points[count].damageDealt = stats.damageDealt;
		points[count].damageReceived = stats.damageReceived;
		points[count].unitsProduced = static_cast<uint32_t>(stats.unitsProduced);
		points[count].unitsDied = static_cast<uint32_t>(stats.unitsDied);
		points[count].unitsReceived = static_cast<uint32_t>(stats.unitsReceived);
		points[count].unitsSent = static_cast<uint32_t>(stats.unitsSent);
		points[count].unitsCaptured = static_cast<uint32_t>(stats.unitsCaptured);
		points[count].unitsOutCaptured = static_cast<uint32_t>(stats.unitsOutCaptured);
		points[count].unitsKilled = static_cast<uint32_t>(stats.unitsKilled);

		bufferPos += sizeof(TeamStatsHistoryPoint);
		count++;
	}

	result->error = nullptr;
	result->history = points;
	result->count = count;
}

static void NativeGetAllyTeamInfo(const GetAllyTeamInfoQuery* query, GetAllyTeamInfoResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) { result->error = &INVALID_ALLY_TEAM_ERROR; return; }

	result->error = nullptr;
	result->info.allyTeamID = query->allyTeamID;
	result->info.teamCount = 0;
	for (int t = 0; t < teamHandler.ActiveTeams(); t++) {
		if (teamHandler.AllyTeam(t) == query->allyTeamID) result->info.teamCount++;
	}
	result->info.customKeys = "";
}

static void NativeAreTeamsAllied(const AreTeamsAlliedQuery* query, AreTeamsAlliedResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->allied = teamHandler.AlliedTeams(query->teamID1, query->teamID2);
}

static void NativeArePlayersAllied(const ArePlayersAlliedQuery* query, ArePlayersAlliedResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	const CPlayer* player1 = playerHandler.Player(query->playerID1);
	const CPlayer* player2 = playerHandler.Player(query->playerID2);

	if (player1 == nullptr || player2 == nullptr) {
		result->error = &INVALID_PLAYER_ERROR;
		return;
	}

	result->error = nullptr;
	result->allied = teamHandler.AlliedTeams(player1->team, player2->team);
}

static void NativeGetPlayerList(const GetPlayerListQuery* query, GetPlayerListResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* players = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
		if (playerHandler.Player(p) != nullptr && playerHandler.Player(p)->active) {
			players[count++] = p;
			bufferPos += sizeof(int32_t);
		}
	}

	result->error = nullptr;
	result->players = players;
	result->count = count;
}

static void NativeGetPlayerListInTeam(const GetPlayerListInTeamQuery* query, GetPlayerListInTeamResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* players = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active && player->team == query->teamID) {
			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
			players[count++] = p;
			bufferPos += sizeof(int32_t);
		}
	}

	result->error = nullptr;
	result->players = players;
	result->count = count;
}

static void NativeGetPlayerListInAllyTeam(const GetPlayerListInAllyTeamQuery* query, GetPlayerListInAllyTeamResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	int32_t* players = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active && teamHandler.AllyTeam(player->team) == query->allyTeamID) {
			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
			players[count++] = p;
			bufferPos += sizeof(int32_t);
		}
	}

	result->error = nullptr;
	result->players = players;
	result->count = count;
}

static void NativeGetPlayerInfo(const GetPlayerInfoQuery* query, GetPlayerInfoResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!playerHandler.IsValidPlayer(query->playerID)) { result->error = &INVALID_PLAYER_ERROR; return; }

	const CPlayer* player = playerHandler.Player(query->playerID);
	if (player == nullptr) { result->error = &INVALID_PLAYER_ERROR; return; }

	result->error = nullptr;
	result->info.playerID = query->playerID;
	result->info.name = player->name.c_str();
	result->info.isActive = player->active;
	result->info.isAI = false;
	result->info.isSpec = player->spectator;
	result->info.teamID = player->team;
	result->info.allyTeamID = teamHandler.AllyTeam(player->team);
	result->info.pingTime = player->ping;
	result->info.cpuUsage = player->cpuUsage;
	result->info.customKeys = "";
}

static void NativeGetPlayerControlledUnit(const GetPlayerControlledUnitQuery* query, GetPlayerControlledUnitResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!playerHandler.IsValidPlayer(query->playerID)) { result->error = &INVALID_PLAYER_ERROR; return; }

	const CPlayer* player = playerHandler.Player(query->playerID);
	if (player == nullptr) { result->error = &INVALID_PLAYER_ERROR; return; }

	const CUnit* controllee = player->fpsController.GetControllee();

	result->error = nullptr;
	result->unitID = (controllee != nullptr) ? controllee->id : -1;
}

static void NativeGetAIInfo(const GetAIInfoQuery* query, GetAIInfoResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->isAI = false;
	result->info.shortName = "";
	result->info.version = "";
	result->info.name = "";
	result->info.description = "";
	result->info.hostPlayer = "";
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

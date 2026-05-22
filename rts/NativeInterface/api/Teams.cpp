#include "Teams.h"

#include <algorithm>
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
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error INVALID_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };
static const Error INVALID_ALLY_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ally team ID" };
static const Error INVALID_PLAYER_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid player ID" };
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Team system not ready" };
static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };

static bool IsReady() { return (gs != nullptr); }

static void NativeGetTeamList(const GetTeamListQuery* query, GetTeamListResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (query->allyTeamID >= 0 && !teamHandler.IsValidAllyTeam(query->allyTeamID)) { result->error = &INVALID_ALLY_TEAM_ERROR; return; }

	int32_t* teams = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int t = 0; t < teamHandler.ActiveTeams(); t++) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) { result->error = &NOT_READY_ERROR; return; }
		if (teamHandler.Team(t) != nullptr) {
			if (query->allyTeamID >= 0 && query->allyTeamID != teamHandler.AllyTeam(t)) {
				continue;
			}
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
	result->info.customKeys = query->getTeamKeys ? "" : nullptr;
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
	result->luaAI = nullptr;
}

static void NativeGetTeamResources(const GetTeamResourcesQuery* query, GetTeamResourcesResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	std::memset(&result->resources, 0, sizeof(result->resources));
	const char res = (query->resource != nullptr) ? query->resource[0] : '\0';
	if (res == 'm') {
		result->resources.metalCurrent = team->res.metal;
		result->resources.metalStorage = team->resStorage.metal;
		result->resources.metalPull = team->resPrevPull.metal;
		result->resources.metalIncome = team->resPrevIncome.metal;
		result->resources.metalExpense = team->resPrevExpense.metal;
		result->resources.metalShared = team->resShare.metal;
		result->resources.metalSent = team->resPrevSent.metal;
		result->resources.metalReceived = team->resPrevReceived.metal;
		result->resources.metalExcess = team->resPrevExcess.metal;
		return;
	}
	if (res == 'e') {
		result->resources.energyCurrent = team->res.energy;
		result->resources.energyStorage = team->resStorage.energy;
		result->resources.energyPull = team->resPrevPull.energy;
		result->resources.energyIncome = team->resPrevIncome.energy;
		result->resources.energyExpense = team->resPrevExpense.energy;
		result->resources.energyShared = team->resShare.energy;
		result->resources.energySent = team->resPrevSent.energy;
		result->resources.energyReceived = team->resPrevReceived.energy;
		result->resources.energyExcess = team->resPrevExcess.energy;
		return;
	}

	result->error = &INVALID_TEAM_ERROR;
}

static void NativeGetTeamUnitStats(const GetTeamUnitStatsQuery* query, GetTeamUnitStatsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	const TeamStatistics& stats = team->GetCurrentStats();
	result->stats.killed = static_cast<uint32_t>(stats.unitsKilled);
	result->stats.died = static_cast<uint32_t>(stats.unitsDied);
	result->stats.capturedBy = static_cast<uint32_t>(stats.unitsCaptured);
	result->stats.capturedFrom = static_cast<uint32_t>(stats.unitsOutCaptured);
	result->stats.received = static_cast<uint32_t>(stats.unitsReceived);
	result->stats.sent = static_cast<uint32_t>(stats.unitsSent);
}

static void NativeGetTeamResourceStats(const GetTeamResourceStatsQuery* query, GetTeamResourceStatsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	const TeamStatistics& stats = team->GetCurrentStats();
	result->error = nullptr;
	std::memset(&result->resources, 0, sizeof(result->resources));

	const char res = (query->resource != nullptr) ? query->resource[0] : '\0';
	if (res == 'm') {
		result->resources.metalCurrent = stats.metalUsed;
		result->resources.metalStorage = stats.metalProduced;
		result->resources.metalPull = stats.metalExcess;
		result->resources.metalIncome = stats.metalReceived;
		result->resources.metalExpense = stats.metalSent;
		return;
	}
	if (res == 'e') {
		result->resources.energyCurrent = stats.energyUsed;
		result->resources.energyStorage = stats.energyProduced;
		result->resources.energyPull = stats.energyExcess;
		result->resources.energyIncome = stats.energyReceived;
		result->resources.energyExpense = stats.energySent;
		return;
	}

	result->error = &INVALID_TEAM_ERROR;
}

static void NativeGetTeamStatsHistory(const GetTeamStatsHistoryQuery* query, GetTeamStatsHistoryResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	const auto& history = team->statHistory;
	const int statCount = static_cast<int>(history.size());
	int start = std::clamp(query->startIndex - 1, 0, std::max(0, statCount - 1));
	int end = std::clamp(query->endIndex - 1, 0, std::max(0, statCount - 1));
	if (query->endIndex <= 0)
		end = start;

	TeamStatsHistoryPoint* points = reinterpret_cast<TeamStatsHistoryPoint*>(&scratchBuffer[bufferPos]);

	uint32_t count = 0;
	for (int i = start; i <= end && i < statCount; i++) {
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
	result->info.keys = nullptr;
	result->info.values = nullptr;
	result->info.count = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) { result->error = &INVALID_ALLY_TEAM_ERROR; return; }

	const AllyTeam& allyTeam = teamHandler.GetAllyTeam(query->allyTeamID);
	const AllyTeam::customOpts& customOpts = allyTeam.GetAllValues();
	const size_t pointerBytes = customOpts.size() * sizeof(const char*) * 2;
	if (bufferPos + pointerBytes > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	const char** keys = reinterpret_cast<const char**>(scratchBuffer + bufferPos);
	bufferPos += customOpts.size() * sizeof(const char*);
	const char** values = reinterpret_cast<const char**>(scratchBuffer + bufferPos);
	bufferPos += customOpts.size() * sizeof(const char*);

	uint32_t count = 0;
	for (const auto& [key, value] : customOpts) {
		keys[count] = key.c_str();
		values[count] = value.c_str();
		count++;
	}

	result->error = nullptr;
	result->info.keys = keys;
	result->info.values = values;
	result->info.count = count;
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
	if (query->teamID >= teamHandler.ActiveTeams()) { result->error = &INVALID_TEAM_ERROR; return; }

	int32_t* players = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) break;
		const CPlayer* player = playerHandler.Player(p);
		if (player == nullptr) continue;
		if (query->active && !player->active) continue;
		if (query->teamID >= 0 && (player->spectator || player->team != query->teamID)) continue;

		players[count++] = p;
		bufferPos += sizeof(int32_t);
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
	result->info.customKeys = query->getPlayerOpts ? "" : nullptr;
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
	result->hasUnit = (controllee != nullptr);
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

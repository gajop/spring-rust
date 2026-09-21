#include "Teams.h"

#include "NativeInterface/WasmUiVisibility.h"

#include <algorithm>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/GlobalConstants.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/Team.h"
#include "Sim/Units/Unit.h"
#include "Game/Players/PlayerHandler.h"
#include "ExternalAI/SkirmishAIHandler.h"

namespace {

static thread_local std::vector<TeamStatsHistoryPoint> historyBuffer;
static thread_local std::vector<int32_t> teamIDs;
static thread_local std::vector<int32_t> allyTeamIDs;
static thread_local std::vector<int32_t> playerIDs;
static thread_local std::vector<const char*> allyTeamKeys;
static thread_local std::vector<const char*> allyTeamValues;
static thread_local std::vector<AIOption> aiOptions;
static thread_local std::vector<std::string> aiStringStorage;

// Static errors
static const Error INVALID_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };
static const Error INVALID_ALLY_TEAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid ally team ID" };
static const Error INVALID_PLAYER_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid player ID" };
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Team system not ready" };

static bool IsReady() { return (gs != nullptr); }

static const char* StoreAIString(const std::string& value)
{
	aiStringStorage.push_back(value);
	return aiStringStorage.back().c_str();
}

static void NativeGetTeamList(const GetTeamListQuery* query, GetTeamListResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (query->allyTeamID >= 0 && !teamHandler.IsValidAllyTeam(query->allyTeamID)) { result->error = &INVALID_ALLY_TEAM_ERROR; return; }

	teamIDs.clear();

	for (int t = 0; t < teamHandler.ActiveTeams(); t++) {
		if (teamHandler.Team(t) != nullptr) {
			if (query->allyTeamID >= 0 && query->allyTeamID != teamHandler.AllyTeam(t)) {
				continue;
			}
			teamIDs.push_back(t);
		}
	}

	result->error = nullptr;
	result->teams = teamIDs.empty() ? nullptr : teamIDs.data();
	result->count = teamIDs.size();
}

static void NativeGetAllyTeamList(const GetAllyTeamListQuery* query, GetAllyTeamListResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	allyTeamIDs.clear();

	for (int at = 0; at < teamHandler.ActiveAllyTeams(); at++) {
		allyTeamIDs.push_back(at);
	}

	result->error = nullptr;
	result->allyTeams = allyTeamIDs.empty() ? nullptr : allyTeamIDs.data();
	result->count = allyTeamIDs.size();
}

static void NativeGetTeamInfo(const GetTeamInfoQuery* query, GetTeamInfoResult* result) {
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
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->allyTeamID = teamHandler.AllyTeam(query->teamID);
}

static void NativeGetTeamMaxUnits(const GetTeamMaxUnitsQuery* query, GetTeamMaxUnitsResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->maxUnits = team->GetMaxUnits();
}

static void NativeGetTeamLuaAI(const GetTeamLuaAIQuery* query, GetTeamLuaAIResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->luaAI = nullptr;
}

static void NativeGetTeamResources(const GetTeamResourcesQuery* query, GetTeamResourcesResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }
	if (!WasmUiVisibility::IsTeamVisible(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

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
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }
	if (!WasmUiVisibility::IsTeamVisible(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

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
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }
	if (!WasmUiVisibility::IsTeamVisible(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

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
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INVALID_TEAM_ERROR; return; }
	if (!WasmUiVisibility::IsTeamVisible(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	const auto& history = team->statHistory;
	const int statCount = static_cast<int>(history.size());
	int start = std::clamp(query->startIndex - 1, 0, std::max(0, statCount - 1));
	int end = std::clamp(query->endIndex - 1, 0, std::max(0, statCount - 1));
	if (query->endIndex <= 0)
		end = start;

	historyBuffer.clear();
	historyBuffer.reserve(std::max(0, end - start + 1));

	for (int i = start; i <= end && i < statCount; i++) {
		const TeamStatistics& stats = history[i];
		const int historyFrame = (i + 1 == statCount) ? gs->GetLuaSimFrame() : stats.frame;
		TeamStatsHistoryPoint& point = historyBuffer.emplace_back();
		point.time = static_cast<float>(historyFrame) / GAME_SPEED;
		point.frame = historyFrame;
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
		point.unitsProduced = static_cast<uint32_t>(stats.unitsProduced);
		point.unitsDied = static_cast<uint32_t>(stats.unitsDied);
		point.unitsReceived = static_cast<uint32_t>(stats.unitsReceived);
		point.unitsSent = static_cast<uint32_t>(stats.unitsSent);
		point.unitsCaptured = static_cast<uint32_t>(stats.unitsCaptured);
		point.unitsOutCaptured = static_cast<uint32_t>(stats.unitsOutCaptured);
		point.unitsKilled = static_cast<uint32_t>(stats.unitsKilled);
	}

	result->error = nullptr;
	result->history = historyBuffer.empty() ? nullptr : historyBuffer.data();
	result->count = historyBuffer.size();
}

static void NativeGetAllyTeamInfo(const GetAllyTeamInfoQuery* query, GetAllyTeamInfoResult* result) {
	result->info.keys = nullptr;
	result->info.values = nullptr;
	result->info.count = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) { result->error = &INVALID_ALLY_TEAM_ERROR; return; }

	const AllyTeam& allyTeam = teamHandler.GetAllyTeam(query->allyTeamID);
	const AllyTeam::customOpts& customOpts = allyTeam.GetAllValues();
	allyTeamKeys.clear();
	allyTeamValues.clear();
	allyTeamKeys.reserve(customOpts.size());
	allyTeamValues.reserve(customOpts.size());
	for (const auto& [key, value] : customOpts) {
		allyTeamKeys.push_back(key.c_str());
		allyTeamValues.push_back(value.c_str());
	}

	result->error = nullptr;
	result->info.keys = allyTeamKeys.empty() ? nullptr : allyTeamKeys.data();
	result->info.values = allyTeamValues.empty() ? nullptr : allyTeamValues.data();
	result->info.count = allyTeamKeys.size();
}

static void NativeAreTeamsAllied(const AreTeamsAlliedQuery* query, AreTeamsAlliedResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID1) || !teamHandler.IsValidTeam(query->teamID2)) {
		result->error = &INVALID_TEAM_ERROR;
		return;
	}

	result->error = nullptr;
	result->allied = teamHandler.AlliedTeams(query->teamID1, query->teamID2);
}

static void NativeArePlayersAllied(const ArePlayersAlliedQuery* query, ArePlayersAlliedResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!playerHandler.IsValidPlayer(query->playerID1) || !playerHandler.IsValidPlayer(query->playerID2)) {
		result->error = &INVALID_PLAYER_ERROR;
		return;
	}

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
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (query->teamID >= teamHandler.ActiveTeams()) { result->error = &INVALID_TEAM_ERROR; return; }

	playerIDs.clear();

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player == nullptr) continue;
		if (query->active && !player->active) continue;
		if (query->teamID >= 0 && (player->spectator || player->team != query->teamID)) continue;

		playerIDs.push_back(p);
	}

	result->error = nullptr;
	result->players = playerIDs.empty() ? nullptr : playerIDs.data();
	result->count = playerIDs.size();
}

static void NativeGetPlayerListInTeam(const GetPlayerListInTeamQuery* query, GetPlayerListInTeamResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	playerIDs.clear();

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active && player->team == query->teamID) {
			playerIDs.push_back(p);
		}
	}

	result->error = nullptr;
	result->players = playerIDs.empty() ? nullptr : playerIDs.data();
	result->count = playerIDs.size();
}

static void NativeGetPlayerListInAllyTeam(const GetPlayerListInAllyTeamQuery* query, GetPlayerListInAllyTeamResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	playerIDs.clear();

	for (int p = 0; p < playerHandler.ActivePlayers(); p++) {
		const CPlayer* player = playerHandler.Player(p);
		if (player != nullptr && player->active &&
			(query->allyTeamID < 0 || teamHandler.AllyTeam(player->team) == query->allyTeamID)) {
			playerIDs.push_back(p);
		}
	}

	result->error = nullptr;
	result->players = playerIDs.empty() ? nullptr : playerIDs.data();
	result->count = playerIDs.size();
}

static void NativeGetPlayerInfo(const GetPlayerInfoQuery* query, GetPlayerInfoResult* result) {
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
	result->info.pingTime = player->ping * 0.001f;
	result->info.cpuUsage = player->cpuUsage;
	result->info.country = player->countryCode.c_str();
	result->info.rank = player->rank;
	result->info.hasSkirmishAIsInTeam = skirmishAIHandler.HasSkirmishAIsInTeam(player->team);
	result->info.customKeys = query->getPlayerOpts ? "" : nullptr;
	result->info.desynced = player->desynced;
}

static void NativeGetPlayerControlledUnit(const GetPlayerControlledUnitQuery* query, GetPlayerControlledUnitResult* result) {
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!playerHandler.IsValidPlayer(query->playerID)) { result->error = &INVALID_PLAYER_ERROR; return; }

	const CPlayer* player = playerHandler.Player(query->playerID);
	if (player == nullptr) { result->error = &INVALID_PLAYER_ERROR; return; }

	const CUnit* controllee = player->fpsController.GetControllee();
	if (controllee != nullptr && !WasmUiVisibility::IsUnitAlly(controllee))
		controllee = nullptr;

	result->error = nullptr;
	result->unitID = (controllee != nullptr) ? controllee->id : -1;
	result->hasUnit = (controllee != nullptr);
}

static void NativeGetAIInfo(const GetAIInfoQuery* query, GetAIInfoResult* result) {
	aiOptions.clear();
	aiStringStorage.clear();
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM_ERROR; return; }

	result->error = nullptr;
	result->isAI = false;
	result->info = {};
	result->info.skirmishAIID = -1;
	result->info.hostingPlayerID = -1;
	result->info.name = "";
	result->info.shortName = "";
	result->info.version = "";
	result->info.options = nullptr;
	result->info.optionCount = 0;

	const std::vector<uint8_t>& teamAIs = skirmishAIHandler.GetSkirmishAIsInTeam(query->teamID);
	if (teamAIs.empty())
		return;

	const size_t skirmishAIID = teamAIs[0];
	const SkirmishAIData* aiData = skirmishAIHandler.GetSkirmishAI(skirmishAIID);
	if (aiData == nullptr)
		return;

	const bool exposeUnsyncedInfo = !WasmUiVisibility::Active() ||
		WasmUiVisibility::FullRead() || skirmishAIHandler.IsLocalSkirmishAI(skirmishAIID);
	result->info.skirmishAIID = static_cast<int32_t>(skirmishAIID);
	aiStringStorage.reserve(3 + aiData->options.size() * 2);
	aiOptions.clear();
	aiOptions.reserve(aiData->options.size());
	result->info.name = StoreAIString(aiData->name);
	result->info.hostingPlayerID = aiData->hostPlayer;
	result->info.shortName = exposeUnsyncedInfo ? StoreAIString(aiData->shortName) : "UNKNOWN";
	result->info.version = exposeUnsyncedInfo ? StoreAIString(aiData->version) : "UNKNOWN";

	if (exposeUnsyncedInfo && !aiData->options.empty()) {
		uint32_t optionCount = 0;
		for (const auto& option: aiData->options) {
			aiOptions.push_back({
				.key = StoreAIString(option.first),
				.value = StoreAIString(option.second),
			});
			optionCount++;
		}
		result->info.options = aiOptions.data();
		result->info.optionCount = optionCount;
	}

	result->isAI = true;
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

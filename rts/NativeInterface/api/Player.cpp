#include "Player.h"

#include <cstdlib>
#include <cstring>

#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/UI/PlayerRoster.h"
#include "Sim/Misc/TeamHandler.h"

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error PLAYER_NOT_AVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Player system is not available"
};

static const Error INVALID_PLAYER_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid player ID"
};

// Helper: check if player system is ready
static bool PlayerSystemReady()
{
	return (game != nullptr) && (gu != nullptr);
}

// Local player info
static void NativeGetLocalPlayerID(const GetLocalPlayerIDQuery* query, GetLocalPlayerIDResult* result)
{
	bufferPos = 0;

	if (!PlayerSystemReady()) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	result->error = nullptr;
	result->playerID = gu->myPlayerNum;
}

static void NativeGetLocalTeamID(const GetLocalTeamIDQuery* query, GetLocalTeamIDResult* result)
{
	bufferPos = 0;

	if (!PlayerSystemReady()) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	result->error = nullptr;
	result->teamID = gu->myTeam;
}

static void NativeGetLocalAllyTeamID(const GetLocalAllyTeamIDQuery* query, GetLocalAllyTeamIDResult* result)
{
	bufferPos = 0;

	if (!PlayerSystemReady()) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	result->error = nullptr;
	result->allyTeamID = gu->myAllyTeam;
}

static void NativeGetSpectatingState(const GetSpectatingStateQuery* query, GetSpectatingStateResult* result)
{
	bufferPos = 0;

	if (!PlayerSystemReady()) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	result->error = nullptr;
	result->spectating = gu->spectating;
}

// Player roster
static void NativeGetPlayerRoster(const GetPlayerRosterQuery* query, GetPlayerRosterResult* result)
{
	bufferPos = 0;

	if (!PlayerSystemReady()) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	const PlayerRoster::SortType oldSortType = playerRoster.GetSortType();
	playerRoster.SetSortTypeByCode(static_cast<PlayerRoster::SortType>(query->sortMode));

	const std::vector<int>& playerIndices = playerRoster.GetIndices(query->showPathingPlayers);
	playerRoster.SetSortTypeByCode(oldSortType);

	// Write roster entries to scratch buffer
	RosterEntry* entries = reinterpret_cast<RosterEntry*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (size_t i = 0; i < playerIndices.size(); i++) {
		if (bufferPos + sizeof(RosterEntry) > sizeof(scratchBuffer)) {
			result->error = &PLAYER_NOT_AVAILABLE_ERROR;
			return;
		}

		const int playerID = playerIndices[i];
		const CPlayer* p = playerHandler.Player(playerID);

		if (p == nullptr || !p->active) {
			continue;
		}

		RosterEntry& entry = entries[count];
		entry.name = p->name.c_str();
		entry.playerID = playerID;
		entry.teamID = p->team;
		entry.allyTeamID = teamHandler.AllyTeam(p->team);
		entry.isAI = false;
		entry.isSpec = p->spectator;
		entry.isActive = p->active;
		entry.pingTime = p->ping;
		entry.cpuUsage = p->cpuUsage;
		entry.country = p->countryCode.empty() ? nullptr : p->countryCode.c_str();
		entry.rank = p->rank;

		bufferPos += sizeof(RosterEntry);
		count++;
	}

	result->error = nullptr;
	result->entries = entries;
	result->count = count;
}

// Player traffic
static void NativeGetPlayerTraffic(const GetPlayerTrafficQuery* query, GetPlayerTrafficResult* result)
{
	bufferPos = 0;

	if (!PlayerSystemReady()) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	const auto& traffic = game->GetPlayerTraffic();
	const auto it = traffic.find(query->playerID);

	if (it == traffic.end()) {
		result->error = nullptr;
		result->count = 0;
		result->traffic = nullptr;
		return;
	}

	// Write traffic data to scratch buffer
	PlayerTraffic* trafficData = reinterpret_cast<PlayerTraffic*>(&scratchBuffer[bufferPos]);
	if (bufferPos + sizeof(PlayerTraffic) > sizeof(scratchBuffer)) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	const CGame::PlayerTrafficInfo& pti = it->second;
	trafficData->playerID = query->playerID;
	trafficData->packetsSent = 0;
	trafficData->packetsReceived = 0;
	trafficData->bytesSent = pti.total;
	if (query->packetID != -1) {
		const auto pit = pti.packets.find(query->packetID);
		trafficData->bytesSent = (pit != pti.packets.end()) ? pit->second : static_cast<uint32_t>(-1);
	}
	trafficData->bytesReceived = 0;

	bufferPos += sizeof(PlayerTraffic);

	result->error = nullptr;
	result->traffic = trafficData;
	result->count = 1;
}

// Player statistics
static void NativeGetPlayerStatistics(const GetPlayerStatisticsQuery* query, GetPlayerStatisticsResult* result)
{
	bufferPos = 0;

	if (!PlayerSystemReady()) {
		result->error = &PLAYER_NOT_AVAILABLE_ERROR;
		return;
	}

	if (!playerHandler.IsValidPlayer(query->playerID)) {
		result->error = &INVALID_PLAYER_ERROR;
		return;
	}

	const CPlayer* player = playerHandler.Player(query->playerID);
	if (player == nullptr) {
		result->error = &INVALID_PLAYER_ERROR;
		return;
	}

	const PlayerStatistics& pStats = player->currentStats;
	result->error = nullptr;
	result->stats.mousePixels = pStats.mousePixels;
	result->stats.mouseClicks = pStats.mouseClicks;
	result->stats.keyPresses = pStats.keyPresses;
	result->stats.unitCommands = pStats.unitCommands;
	result->stats.avgCommandSize = (pStats.numCommands > 0) ?
		(static_cast<float>(pStats.unitCommands) / static_cast<float>(pStats.numCommands)) : 0.0f;
}

} // namespace

const PlayerApi PLAYER_API = {
	.GetLocalPlayerID = NativeGetLocalPlayerID,
	.GetLocalTeamID = NativeGetLocalTeamID,
	.GetLocalAllyTeamID = NativeGetLocalAllyTeamID,
	.GetSpectatingState = NativeGetSpectatingState,

	.GetPlayerRoster = NativeGetPlayerRoster,
	.GetPlayerTraffic = NativeGetPlayerTraffic,
	.GetPlayerStatistics = NativeGetPlayerStatistics,
};

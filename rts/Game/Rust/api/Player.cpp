#include "Player.h"

#include <cstdlib>
#include <cstring>

#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/Players/PlayerRoster.h"
#include "Sim/Misc/TeamHandler.h"

namespace {

// Error constants
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
static Int32Result NativeGetLocalPlayerID()
{
	Int32Result result = {};
	if (!PlayerSystemReady()) {
		result.error = &PLAYER_NOT_AVAILABLE_ERROR;
		return result;
	}
	result.value = gu->myPlayerNum;
	return result;
}

static Int32Result NativeGetLocalTeamID()
{
	Int32Result result = {};
	if (!PlayerSystemReady()) {
		result.error = &PLAYER_NOT_AVAILABLE_ERROR;
		return result;
	}
	result.value = gu->myTeam;
	return result;
}

static Int32Result NativeGetLocalAllyTeamID()
{
	Int32Result result = {};
	if (!PlayerSystemReady()) {
		result.error = &PLAYER_NOT_AVAILABLE_ERROR;
		return result;
	}
	result.value = gu->myAllyTeam;
	return result;
}

static BoolResult NativeGetSpectatingState()
{
	BoolResult result = {};
	if (!PlayerSystemReady()) {
		result.error = &PLAYER_NOT_AVAILABLE_ERROR;
		return result;
	}
	result.value = gu->spectating;
	return result;
}

// Player roster
static PlayerRosterResult NativeGetPlayerRoster(int32_t sortMode)
{
	PlayerRosterResult result = {};
	if (!PlayerSystemReady()) {
		result.error = &PLAYER_NOT_AVAILABLE_ERROR;
		return result;
	}

	const PlayerRoster::SortType oldSortType = playerRoster.GetSortType();
	playerRoster.SetSortTypeByCode(static_cast<PlayerRoster::SortType>(sortMode));

	const std::vector<int>& playerIndices = playerRoster.GetIndices(false);
	playerRoster.SetSortTypeByCode(oldSortType);

	// Use static storage - valid for call duration only
	static thread_local std::vector<RosterEntry> entries;
	entries.clear();

	for (size_t i = 0; i < playerIndices.size(); i++) {
		const int playerID = playerIndices[i];
		const CPlayer* p = playerHandler.Player(playerID);

		if (p == nullptr || !p->active) {
			continue;
		}

		RosterEntry entry;
		entry.name = p->name.c_str(); // Point to internal CPlayer string
		entry.playerID = playerID;
		entry.teamID = p->team;
		entry.allyTeamID = teamHandler.AllyTeam(p->team);
		entry.isAI = false; // TODO: detect AI players
		entry.isSpec = p->spectator;
		entry.isActive = p->active;
		entry.pingTime = p->ping;
		entry.cpuUsage = p->cpuUsage;
		entry.country = p->countryCode.empty() ? nullptr : p->countryCode.c_str();
		entry.rank = p->rank;

		entries.push_back(entry);
	}

	result.entries = entries.data();
	result.count = static_cast<uint32_t>(entries.size());

	return result;
}

// Player traffic
static PlayerTrafficResult NativeGetPlayerTraffic(int32_t playerID)
{
	PlayerTrafficResult result = {};
	if (!PlayerSystemReady()) {
		result.error = &PLAYER_NOT_AVAILABLE_ERROR;
		return result;
	}

	const auto& traffic = game->GetPlayerTraffic();
	const auto it = traffic.find(playerID);

	if (it == traffic.end()) {
		result.count = 0;
		result.traffic = nullptr;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local PlayerTraffic trafficData;

	const CGame::PlayerTrafficInfo& pti = it->second;
	trafficData.playerID = playerID;
	trafficData.packetsSent = 0; // Not directly available
	trafficData.packetsReceived = 0; // Not directly available
	trafficData.bytesSent = pti.total;
	trafficData.bytesReceived = 0; // Not directly available

	result.traffic = &trafficData;
	result.count = 1;

	return result;
}

// Player statistics
static PlayerStatsResult NativeGetPlayerStatistics(int32_t playerID)
{
	PlayerStatsResult result = {};
	if (!PlayerSystemReady()) {
		result.error = &PLAYER_NOT_AVAILABLE_ERROR;
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

	const PlayerStatistics& pStats = player->currentStats;
	result.stats.mousePixels = pStats.mousePixels;
	result.stats.mouseClicks = pStats.mouseClicks;
	result.stats.keyPresses = pStats.keyPresses;
	result.stats.unitCommands = pStats.unitCommands;
	result.stats.avgCommandSize = (pStats.numCommands > 0) ?
		(static_cast<float>(pStats.unitCommands) / static_cast<float>(pStats.numCommands)) : 0.0f;

	return result;
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

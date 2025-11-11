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

	// Count active players
	uint32_t count = 0;
	for (size_t i = 0; i < playerIndices.size(); i++) {
		const CPlayer* p = playerHandler.Player(playerIndices[i]);
		if (p != nullptr && p->active) {
			count++;
		}
	}

	if (count == 0) {
		result.count = 0;
		result.entries = nullptr;
		return result;
	}

	// Allocate array
	result.entries = static_cast<RosterEntry*>(std::malloc(count * sizeof(RosterEntry)));
	if (result.entries == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory for roster"
		};
		result.error = &OUT_OF_MEMORY;
		result.count = 0;
		return result;
	}

	// Fill array
	uint32_t idx = 0;
	for (size_t i = 0; i < playerIndices.size() && idx < count; i++) {
		const int playerID = playerIndices[i];
		const CPlayer* p = playerHandler.Player(playerID);

		if (p == nullptr || !p->active) {
			continue;
		}

		RosterEntry& entry = result.entries[idx++];

		// Allocate and copy name string
		const size_t nameLen = p->name.length() + 1;
		char* nameCopy = static_cast<char*>(std::malloc(nameLen));
		if (nameCopy == nullptr) {
			// Free previously allocated entries on error
			for (uint32_t j = 0; j < idx - 1; j++) {
				std::free(const_cast<char*>(result.entries[j].name));
				if (result.entries[j].country != nullptr) {
					std::free(const_cast<char*>(result.entries[j].country));
				}
			}
			std::free(result.entries);

			static const Error OUT_OF_MEMORY = {
				.code = ERROR_INTERNAL,
				.message = "Failed to allocate memory for player name"
			};
			result.error = &OUT_OF_MEMORY;
			result.entries = nullptr;
			result.count = 0;
			return result;
		}
		std::memcpy(nameCopy, p->name.c_str(), nameLen);
		entry.name = nameCopy;

		entry.playerID = playerID;
		entry.teamID = p->team;
		entry.allyTeamID = teamHandler.AllyTeam(p->team);
		entry.isAI = false; // TODO: detect AI players
		entry.isSpec = p->spectator;
		entry.isActive = p->active;
		entry.pingTime = p->ping;
		entry.cpuUsage = p->cpuUsage;

		// Country string (may be empty)
		if (!p->countryCode.empty()) {
			const size_t countryLen = p->countryCode.length() + 1;
			char* countryCopy = static_cast<char*>(std::malloc(countryLen));
			if (countryCopy != nullptr) {
				std::memcpy(countryCopy, p->countryCode.c_str(), countryLen);
				entry.country = countryCopy;
			} else {
				entry.country = nullptr;
			}
		} else {
			entry.country = nullptr;
		}

		entry.rank = p->rank;
	}

	result.count = idx;
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

	// Allocate single entry
	result.traffic = static_cast<PlayerTraffic*>(std::malloc(sizeof(PlayerTraffic)));
	if (result.traffic == nullptr) {
		static const Error OUT_OF_MEMORY = {
			.code = ERROR_INTERNAL,
			.message = "Failed to allocate memory for traffic"
		};
		result.error = &OUT_OF_MEMORY;
		result.count = 0;
		return result;
	}

	const CGame::PlayerTrafficInfo& pti = it->second;
	result.traffic->playerID = playerID;
	result.traffic->packetsSent = 0; // Not directly available
	result.traffic->packetsReceived = 0; // Not directly available
	result.traffic->bytesSent = pti.total;
	result.traffic->bytesReceived = 0; // Not directly available
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

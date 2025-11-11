#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Player/Local API
// @see rts/Lua/LuaUnsyncedRead.cpp
//
// Local player information (unsynced)
// ============================================================================

// Player roster entry
struct RosterEntry {
	const char* name;
	int32_t playerID;
	int32_t teamID;
	int32_t allyTeamID;
	bool isAI;
	bool isSpec;
	bool isActive;
	uint32_t pingTime;
	uint32_t cpuUsage;
	const char* country;
	int32_t rank;
};

struct PlayerRosterResult {
	const Error* error;
	RosterEntry* entries;
	uint32_t count;
};

// Player traffic stats
struct PlayerTraffic {
	int32_t playerID;
	uint32_t packetsSent;
	uint32_t packetsReceived;
	uint32_t bytesSent;
	uint32_t bytesReceived;
};

struct PlayerTrafficResult {
	const Error* error;
	PlayerTraffic* traffic;
	uint32_t count;
};

// Player statistics
struct PlayerStats {
	int32_t mousePixels;
	int32_t mouseClicks;
	int32_t keyPresses;
	uint32_t unitCommands;
	float avgCommandSize;
};

struct PlayerStatsResult {
	const Error* error;
	PlayerStats stats;
};

// API structure
struct PlayerApi {
	// Local player
	Int32Result (*GetLocalPlayerID)();
	Int32Result (*GetLocalTeamID)();
	Int32Result (*GetLocalAllyTeamID)();
	BoolResult (*GetSpectatingState)();

	// Player roster
	PlayerRosterResult (*GetPlayerRoster)(int32_t sortMode);

	// Player traffic
	PlayerTrafficResult (*GetPlayerTraffic)(int32_t playerID);

	// Player statistics
	PlayerStatsResult (*GetPlayerStatistics)(int32_t playerID);
};

extern const PlayerApi PLAYER_API;

#ifdef __cplusplus
}
#endif

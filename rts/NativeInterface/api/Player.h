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
	float pingTime; // seconds, matching Spring.GetPlayerRoster
	float cpuUsage; // normalized load, matching CPlayer and Spring.GetPlayerRoster
	const char* country;
	int32_t rank;
};

// Player traffic stats
struct PlayerTraffic {
	int32_t playerID;
	uint32_t packetsSent;
	uint32_t packetsReceived;
	uint32_t bytesSent;
	uint32_t bytesReceived;
};

// Player statistics
struct PlayerStats {
	int32_t mousePixels;
	int32_t mouseClicks;
	int32_t keyPresses;
	uint32_t unitCommands;
	float avgCommandSize;
};

// Queries
struct GetLocalPlayerIDQuery {
	uint8_t _unused;
};

struct GetLocalPlayerIDResult {
	const Error* error;
	int32_t playerID;
};

struct GetLocalTeamIDQuery {
	uint8_t _unused;
};

struct GetLocalTeamIDResult {
	const Error* error;
	int32_t teamID;
};

struct GetLocalAllyTeamIDQuery {
	uint8_t _unused;
};

struct GetLocalAllyTeamIDResult {
	const Error* error;
	int32_t allyTeamID;
};

struct GetSpectatingStateQuery {
	uint8_t _unused;
};

struct GetSpectatingStateResult {
	const Error* error;
	bool spectating;
};

struct GetPlayerRosterQuery {
	int32_t sortMode;
	bool showPathingPlayers;
};

struct GetPlayerRosterResult {
	const Error* error;
	RosterEntry* entries;
	uint32_t count;
};

struct GetPlayerTrafficQuery {
	int32_t playerID;
	int32_t packetID;
};

struct GetPlayerTrafficResult {
	const Error* error;
	PlayerTraffic* traffic;
	uint32_t count;
};

struct GetPlayerStatisticsQuery {
	int32_t playerID;
};

struct GetPlayerStatisticsResult {
	const Error* error;
	PlayerStats stats;
};

// API structure
struct PlayerApi {
	void (*GetLocalPlayerID)(
		const GetLocalPlayerIDQuery* query,
		GetLocalPlayerIDResult* result
	);

	void (*GetLocalTeamID)(
		const GetLocalTeamIDQuery* query,
		GetLocalTeamIDResult* result
	);

	void (*GetLocalAllyTeamID)(
		const GetLocalAllyTeamIDQuery* query,
		GetLocalAllyTeamIDResult* result
	);

	void (*GetSpectatingState)(
		const GetSpectatingStateQuery* query,
		GetSpectatingStateResult* result
	);

	void (*GetPlayerRoster)(
		const GetPlayerRosterQuery* query,
		GetPlayerRosterResult* result
	);

	void (*GetPlayerTraffic)(
		const GetPlayerTrafficQuery* query,
		GetPlayerTrafficResult* result
	);

	void (*GetPlayerStatistics)(
		const GetPlayerStatisticsQuery* query,
		GetPlayerStatisticsResult* result
	);
};

extern const PlayerApi PLAYER_API;

#ifdef __cplusplus
}
#endif

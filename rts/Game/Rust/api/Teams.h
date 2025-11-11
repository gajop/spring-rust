#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Teams, Players, and AllyTeams API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Queries about teams, players, ally teams, and their relationships
// ============================================================================

// Team info
struct TeamInfo {
	int32_t teamID;
	int32_t allyTeamID;
	int32_t leaderID;  // Player ID of team leader
	bool isDead;
	const char* side;  // Faction name
	uint32_t color;    // RGBA color
	const char* customKeys;  // Comma-separated custom keys
};

struct TeamInfoResult {
	const Error* error;
	TeamInfo info;
};

// Team resources
struct TeamResources {
	float metalCurrent;
	float metalStorage;
	float metalPull;
	float metalIncome;
	float metalExpense;
	float metalShared;
	float metalSent;
	float metalReceived;
	float metalExcess;

	float energyCurrent;
	float energyStorage;
	float energyPull;
	float energyIncome;
	float energyExpense;
	float energyShared;
	float energySent;
	float energyReceived;
	float energyExcess;
};

struct TeamResourcesResult {
	const Error* error;
	TeamResources resources;
};

// Team unit stats
struct TeamUnitStats {
	uint32_t unitCount;
	uint32_t unitLimit;
};

struct TeamUnitStatsResult {
	const Error* error;
	TeamUnitStats stats;
};

// Player info
struct PlayerInfo {
	int32_t playerID;
	const char* name;
	bool isActive;
	bool isAI;
	bool isSpec;
	int32_t teamID;
	int32_t allyTeamID;
	uint32_t pingTime;
	uint32_t cpuUsage;
	const char* customKeys;  // Comma-separated custom keys
};

struct PlayerInfoResult {
	const Error* error;
	PlayerInfo info;
};

// AllyTeam info
struct AllyTeamInfo {
	int32_t allyTeamID;
	uint32_t teamCount;
	const char* customKeys;
};

struct AllyTeamInfoResult {
	const Error* error;
	AllyTeamInfo info;
};

// AI info
struct AIInfo {
	const char* shortName;
	const char* version;
	const char* name;
	const char* description;
	const char* hostPlayer;  // Player name hosting this AI
};

struct AIInfoResult {
	const Error* error;
	AIInfo info;
	bool isAI;
};

// Team stats history point
struct TeamStatsHistoryPoint {
	float metalUsed;
	float metalProduced;
	float metalExcess;
	float metalReceived;
	float metalSent;
	float energyUsed;
	float energyProduced;
	float energyExcess;
	float energyReceived;
	float energySent;
	float damageDealt;
	float damageReceived;
	uint32_t unitsProduced;
	uint32_t unitsDied;
	uint32_t unitsReceived;
	uint32_t unitsSent;
	uint32_t unitsCaptured;
	uint32_t unitsOutCaptured;
	uint32_t unitsKilled;
};

struct TeamStatsHistoryResult {
	const Error* error;
	TeamStatsHistoryPoint* history;
	uint32_t count;
};

// API structure
struct TeamsApi {
	// Team lists
	Int32Array (*GetTeamList)();
	Int32Array (*GetAllyTeamList)();

	// Team info
	TeamInfoResult (*GetTeamInfo)(int32_t teamID);
	Int32Result (*GetTeamAllyTeamID)(int32_t teamID);
	Int32Result (*GetTeamMaxUnits)(int32_t teamID);
	StringResult (*GetTeamLuaAI)(int32_t teamID);

	// Team resources
	TeamResourcesResult (*GetTeamResources)(int32_t teamID);
	TeamUnitStatsResult (*GetTeamUnitStats)(int32_t teamID);
	TeamResourcesResult (*GetTeamResourceStats)(int32_t teamID);
	TeamStatsHistoryResult (*GetTeamStatsHistory)(int32_t teamID);

	// AllyTeam info
	AllyTeamInfoResult (*GetAllyTeamInfo)(int32_t allyTeamID);

	// Alliance queries
	BoolResult (*AreTeamsAllied)(int32_t teamID1, int32_t teamID2);
	BoolResult (*ArePlayersAllied)(int32_t playerID1, int32_t playerID2);

	// Player lists
	Int32Array (*GetPlayerList)();
	Int32Array (*GetPlayerListInTeam)(int32_t teamID);
	Int32Array (*GetPlayerListInAllyTeam)(int32_t allyTeamID);

	// Player info
	PlayerInfoResult (*GetPlayerInfo)(int32_t playerID);
	Int32Result (*GetPlayerControlledUnit)(int32_t playerID);
	AIInfoResult (*GetAIInfo)(int32_t teamID);
};

extern const TeamsApi TEAMS_API;

#ifdef __cplusplus
}
#endif

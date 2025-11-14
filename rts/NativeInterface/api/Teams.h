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
	int32_t leaderID;
	bool isDead;
	const char* side;
	uint32_t color;
	const char* customKeys;
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

// Team unit stats
struct TeamUnitStats {
	uint32_t unitCount;
	uint32_t unitLimit;
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
	const char* customKeys;
};

// AllyTeam info
struct AllyTeamInfo {
	int32_t allyTeamID;
	uint32_t teamCount;
	const char* customKeys;
};

// AI info
struct AIInfo {
	const char* shortName;
	const char* version;
	const char* name;
	const char* description;
	const char* hostPlayer;
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

// Queries
struct GetTeamListQuery { uint8_t _unused; };
struct GetTeamListResult { const Error* error; int32_t* teams; uint32_t count; };

struct GetAllyTeamListQuery { uint8_t _unused; };
struct GetAllyTeamListResult { const Error* error; int32_t* allyTeams; uint32_t count; };

struct GetTeamInfoQuery { int32_t teamID; };
struct GetTeamInfoResult { const Error* error; TeamInfo info; };

struct GetTeamAllyTeamIDQuery { int32_t teamID; };
struct GetTeamAllyTeamIDResult { const Error* error; int32_t allyTeamID; };

struct GetTeamMaxUnitsQuery { int32_t teamID; };
struct GetTeamMaxUnitsResult { const Error* error; int32_t maxUnits; };

struct GetTeamLuaAIQuery { int32_t teamID; };
struct GetTeamLuaAIResult { const Error* error; const char* luaAI; };

struct GetTeamResourcesQuery { int32_t teamID; };
struct GetTeamResourcesResult { const Error* error; TeamResources resources; };

struct GetTeamUnitStatsQuery { int32_t teamID; };
struct GetTeamUnitStatsResult { const Error* error; TeamUnitStats stats; };

struct GetTeamResourceStatsQuery { int32_t teamID; };
struct GetTeamResourceStatsResult { const Error* error; TeamResources resources; };

struct GetTeamStatsHistoryQuery { int32_t teamID; };
struct GetTeamStatsHistoryResult { const Error* error; TeamStatsHistoryPoint* history; uint32_t count; };

struct GetAllyTeamInfoQuery { int32_t allyTeamID; };
struct GetAllyTeamInfoResult { const Error* error; AllyTeamInfo info; };

struct AreTeamsAlliedQuery { int32_t teamID1; int32_t teamID2; };
struct AreTeamsAlliedResult { const Error* error; bool allied; };

struct ArePlayersAlliedQuery { int32_t playerID1; int32_t playerID2; };
struct ArePlayersAlliedResult { const Error* error; bool allied; };

struct GetPlayerListQuery { uint8_t _unused; };
struct GetPlayerListResult { const Error* error; int32_t* players; uint32_t count; };

struct GetPlayerListInTeamQuery { int32_t teamID; };
struct GetPlayerListInTeamResult { const Error* error; int32_t* players; uint32_t count; };

struct GetPlayerListInAllyTeamQuery { int32_t allyTeamID; };
struct GetPlayerListInAllyTeamResult { const Error* error; int32_t* players; uint32_t count; };

struct GetPlayerInfoQuery { int32_t playerID; };
struct GetPlayerInfoResult { const Error* error; PlayerInfo info; };

struct GetPlayerControlledUnitQuery { int32_t playerID; };
struct GetPlayerControlledUnitResult { const Error* error; int32_t unitID; };

struct GetAIInfoQuery { int32_t teamID; };
struct GetAIInfoResult { const Error* error; AIInfo info; bool isAI; };

// API structure
struct TeamsApi {
	void (*GetTeamList)(const GetTeamListQuery* query, GetTeamListResult* result);
	void (*GetAllyTeamList)(const GetAllyTeamListQuery* query, GetAllyTeamListResult* result);
	void (*GetTeamInfo)(const GetTeamInfoQuery* query, GetTeamInfoResult* result);
	void (*GetTeamAllyTeamID)(const GetTeamAllyTeamIDQuery* query, GetTeamAllyTeamIDResult* result);
	void (*GetTeamMaxUnits)(const GetTeamMaxUnitsQuery* query, GetTeamMaxUnitsResult* result);
	void (*GetTeamLuaAI)(const GetTeamLuaAIQuery* query, GetTeamLuaAIResult* result);
	void (*GetTeamResources)(const GetTeamResourcesQuery* query, GetTeamResourcesResult* result);
	void (*GetTeamUnitStats)(const GetTeamUnitStatsQuery* query, GetTeamUnitStatsResult* result);
	void (*GetTeamResourceStats)(const GetTeamResourceStatsQuery* query, GetTeamResourceStatsResult* result);
	void (*GetTeamStatsHistory)(const GetTeamStatsHistoryQuery* query, GetTeamStatsHistoryResult* result);
	void (*GetAllyTeamInfo)(const GetAllyTeamInfoQuery* query, GetAllyTeamInfoResult* result);
	void (*AreTeamsAllied)(const AreTeamsAlliedQuery* query, AreTeamsAlliedResult* result);
	void (*ArePlayersAllied)(const ArePlayersAlliedQuery* query, ArePlayersAlliedResult* result);
	void (*GetPlayerList)(const GetPlayerListQuery* query, GetPlayerListResult* result);
	void (*GetPlayerListInTeam)(const GetPlayerListInTeamQuery* query, GetPlayerListInTeamResult* result);
	void (*GetPlayerListInAllyTeam)(const GetPlayerListInAllyTeamQuery* query, GetPlayerListInAllyTeamResult* result);
	void (*GetPlayerInfo)(const GetPlayerInfoQuery* query, GetPlayerInfoResult* result);
	void (*GetPlayerControlledUnit)(const GetPlayerControlledUnitQuery* query, GetPlayerControlledUnitResult* result);
	void (*GetAIInfo)(const GetAIInfoQuery* query, GetAIInfoResult* result);
};

extern const TeamsApi TEAMS_API;

#ifdef __cplusplus
}
#endif

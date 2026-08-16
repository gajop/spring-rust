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
	uint32_t killed;
	uint32_t died;
	uint32_t capturedBy;
	uint32_t capturedFrom;
	uint32_t received;
	uint32_t sent;
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
	float pingTime; // seconds, matching Spring.GetPlayerInfo
	float cpuUsage; // normalized load, matching CPlayer and Spring.GetPlayerInfo
	const char* country;
	int32_t rank;
	bool hasSkirmishAIsInTeam;
	const char* customKeys;
	bool desynced;
};

// AllyTeam info
struct AllyTeamInfo {
	// The key/value arrays are parallel and share count.  Explicitly describe
	// both arrays so generators can produce owned string lists without ever
	// transporting a native pointer.
	RECOIL_WASM_LIST("string", "count") const char** keys;
	RECOIL_WASM_LIST("string", "count") const char** values;
	uint32_t count;
};

// AI info
struct AIOption {
	const char* key;
	const char* value;
};

struct AIInfo {
	int32_t skirmishAIID;
	const char* name;
	int32_t hostingPlayerID;
	const char* shortName;
	const char* version;
	AIOption* options;
	uint32_t optionCount;
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
struct GetTeamListQuery { int32_t allyTeamID; };
struct GetTeamListResult { const Error* error; int32_t* teams; uint32_t count; };

struct GetAllyTeamListQuery { uint8_t _unused; };
struct GetAllyTeamListResult { const Error* error; int32_t* allyTeams; uint32_t count; };

struct GetTeamInfoQuery { int32_t teamID; bool getTeamKeys; };
struct GetTeamInfoResult { const Error* error; TeamInfo info; };

struct GetTeamAllyTeamIDQuery { int32_t teamID; };
struct GetTeamAllyTeamIDResult { const Error* error; int32_t allyTeamID; };

struct GetTeamMaxUnitsQuery { int32_t teamID; };
struct GetTeamMaxUnitsResult { const Error* error; int32_t maxUnits; };

struct GetTeamLuaAIQuery { int32_t teamID; };
struct GetTeamLuaAIResult { const Error* error; const char* luaAI; };

struct GetTeamResourcesQuery { int32_t teamID; const char* resource; };
struct GetTeamResourcesResult { const Error* error; TeamResources resources; };

struct GetTeamUnitStatsQuery { int32_t teamID; };
struct GetTeamUnitStatsResult { const Error* error; TeamUnitStats stats; };

struct GetTeamResourceStatsQuery { int32_t teamID; const char* resource; };
struct GetTeamResourceStatsResult { const Error* error; TeamResources resources; };

struct GetTeamStatsHistoryQuery { int32_t teamID; int32_t startIndex; int32_t endIndex; };
struct GetTeamStatsHistoryResult { const Error* error; TeamStatsHistoryPoint* history; uint32_t count; };

struct GetAllyTeamInfoQuery { int32_t allyTeamID; };
struct GetAllyTeamInfoResult { const Error* error; AllyTeamInfo info; };

struct AreTeamsAlliedQuery { int32_t teamID1; int32_t teamID2; };
struct AreTeamsAlliedResult { const Error* error; bool allied; };

struct ArePlayersAlliedQuery { int32_t playerID1; int32_t playerID2; };
struct ArePlayersAlliedResult { const Error* error; bool allied; };

struct GetPlayerListQuery { int32_t teamID; bool active; };
struct GetPlayerListResult { const Error* error; int32_t* players; uint32_t count; };

struct GetPlayerListInTeamQuery { int32_t teamID; };
struct GetPlayerListInTeamResult { const Error* error; int32_t* players; uint32_t count; };

struct GetPlayerListInAllyTeamQuery { int32_t allyTeamID; };
struct GetPlayerListInAllyTeamResult { const Error* error; int32_t* players; uint32_t count; };

struct GetPlayerInfoQuery { int32_t playerID; bool getPlayerOpts; };
struct GetPlayerInfoResult { const Error* error; PlayerInfo info; };

struct GetPlayerControlledUnitQuery { int32_t playerID; };
struct GetPlayerControlledUnitResult { const Error* error; int32_t unitID; bool hasUnit; };

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

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Game State API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Core game state queries (frame, time, game settings, etc.)
// ============================================================================

// Side data (faction)
struct SideData {
	const char* sideName;
	const char* caseName;
	const char* startUnit;
	uint32_t sideIndex;
};

// Start box for an ally team
struct StartBox {
	float minX;
	float minZ;
	float maxX;
	float maxZ;
};

// Map start position
struct StartPosition {
	Float3 pos;
	int32_t teamID;
};

// Wind/tidal values
struct WindData {
	float min;
	float max;
	float current;
};

struct GameSetupInfo {
	int32_t startPosType;
	bool ghostedBuildings;
	const char* demoPlayName;
};

struct GameMapInfo {
	const char* mapName;
	const char* mapDescription;
	const char* mapChecksum;
	float mapHardness;
	float extractorRadius;
	float tidal;
	float waterDamage;
	float gravity;
	int32_t mapX;
	int32_t mapY;
	int32_t mapSizeX;
	int32_t mapSizeZ;
	bool mapDamage;
};

struct GameModInfo {
	const char* gameName;
	const char* gameShortName;
	const char* gameVersion;
	const char* gameMutator;
	const char* gameDesc;
	const char* modName;
	const char* modShortName;
	const char* modVersion;
	const char* modMutator;
	const char* modDesc;
	const char* modChecksum;
};

struct GameRulesInfo {
	int32_t maxUnits;
	bool constructionDecay;
	int32_t constructionDecayTime;
	float constructionDecaySpeed;
	int32_t multiReclaim;
	int32_t reclaimMethod;
	int32_t reclaimUnitMethod;
	float reclaimUnitEnergyCostFactor;
	float reclaimUnitEfficiency;
	float reclaimFeatureEnergyCostFactor;
	bool reclaimUnitDrainHealth;
	bool reclaimAllowEnemies;
	bool reclaimAllowAllies;
	float repairEnergyCostFactor;
	float resurrectEnergyCostFactor;
	float captureEnergyCostFactor;
	int32_t transportAir;
	int32_t transportShip;
	int32_t transportHover;
	int32_t transportGround;
	int32_t fireAtKilled;
	int32_t fireAtCrashing;
	int32_t requireSonarUnderWater;
	bool paralyzeOnMaxHealth;
	float paralyzeDeclineRate;
	bool allowEnginePlayerlist;
	bool nativeExcessSharing;
};

struct GameRulesResourceInfo {
	ResourcePack reclaimUnitCostFactor;
	ResourcePack reclaimUnitEfficiency;
	ResourcePack reclaimFeatureCostFactor;
	ResourcePack repairCostFactor;
	ResourcePack resurrectCostFactor;
	ResourcePack captureCostFactor;
};

// Queries
struct IsCheatingEnabledQuery { uint8_t _unused; };
struct IsCheatingEnabledResult { const Error* error; bool enabled; };

struct IsGodModeEnabledQuery { uint8_t _unused; };
struct IsGodModeEnabledResult { const Error* error; bool enabled; };

struct IsDevLuaEnabledQuery { uint8_t _unused; };
struct IsDevLuaEnabledResult { const Error* error; bool enabled; };

struct IsEditDefsEnabledQuery { uint8_t _unused; };
struct IsEditDefsEnabledResult { const Error* error; bool enabled; };

struct IsNoCostEnabledQuery { uint8_t _unused; };
struct IsNoCostEnabledResult { const Error* error; bool enabled; };

struct GetGlobalLosQuery { int32_t allyTeamID; };
struct GetGlobalLosResult { const Error* error; int32_t los; };

struct AreHelperAIsEnabledQuery { uint8_t _unused; };
struct AreHelperAIsEnabledResult { const Error* error; bool enabled; };

struct FixedAlliesQuery { uint8_t _unused; };
struct FixedAlliesResult { const Error* error; bool fixed; };

struct IsGameOverQuery { uint8_t _unused; };
struct IsGameOverResult { const Error* error; bool gameOver; };

struct GetGameFrameQuery { uint8_t _unused; };
struct GetGameFrameResult { const Error* error; uint32_t low16; uint32_t high16; };

struct GetGameSecondsQuery { uint8_t _unused; };
struct GetGameSecondsResult { const Error* error; float seconds; };

struct GetGaiaTeamIDQuery { uint8_t _unused; };
struct GetGaiaTeamIDResult { const Error* error; int32_t teamID; };

struct GetGameSetupInfoQuery { uint8_t _unused; };
struct GetGameSetupInfoResult { const Error* error; GameSetupInfo info; };

struct GetGameMapInfoQuery { uint8_t _unused; };
struct GetGameMapInfoResult { const Error* error; GameMapInfo info; };

struct GetGameModInfoQuery { uint8_t _unused; };
struct GetGameModInfoResult { const Error* error; GameModInfo info; };

struct GetGameRulesInfoQuery { uint8_t _unused; };
struct GetGameRulesInfoResult { const Error* error; GameRulesInfo info; };

struct GetGameRulesResourceInfoQuery { uint8_t _unused; };
struct GetGameRulesResourceInfoResult { const Error* error; GameRulesResourceInfo info; };

struct GetMapOptionQuery { const char* key; };
struct GetMapOptionResult { const Error* error; const char* value; bool exists; };

struct GetMapOptionsQuery { uint8_t _unused; };
struct GetMapOptionsResult { const Error* error; const char** keys; uint32_t count; };

struct GetModOptionQuery { const char* key; };
struct GetModOptionResult { const Error* error; const char* value; bool exists; };

struct GetModOptionsQuery { uint8_t _unused; };
struct GetModOptionsResult { const Error* error; const char** keys; uint32_t count; };

struct GetTidalQuery { uint8_t _unused; };
struct GetTidalResult { const Error* error; float strength; };

struct GetWindQuery { uint8_t _unused; };
struct GetWindResult { const Error* error; WindData data; };

struct GetHeadingFromVectorQuery { float x; float z; };
struct GetHeadingFromVectorResult { const Error* error; int32_t heading; };

struct GetVectorFromHeadingQuery { int32_t heading; };
struct GetVectorFromHeadingResult { const Error* error; Float2 vector; };

struct GetFacingFromHeadingQuery { int32_t heading; };
struct GetFacingFromHeadingResult { const Error* error; int32_t facing; };

struct GetHeadingFromFacingQuery { int32_t facing; };
struct GetHeadingFromFacingResult { const Error* error; int32_t heading; };

struct GetSideDataQuery { const char* sideName; };
struct GetSideDataResult { const Error* error; SideData data; };

struct GetSideDataByIndexQuery { uint32_t sideIndex; };
struct GetSideDataByIndexResult { const Error* error; SideData data; };

struct GetSideDataCountQuery { uint8_t _unused; };
struct GetSideDataCountResult { const Error* error; uint32_t count; };

struct GetAllyTeamStartBoxQuery { int32_t allyTeamID; };
struct GetAllyTeamStartBoxResult { const Error* error; StartBox box; bool exists; };

struct GetTeamStartPositionQuery { int32_t teamID; };
// Lua's GetTeamStartPosition returns the validity bit alongside the
// position.  Keep it in the native result so the Wasm and native semantic
// surfaces do not have to infer validity from coordinates.
struct GetTeamStartPositionResult { const Error* error; Float3 position; bool valid; };

struct GetMapStartPositionsQuery { uint8_t _unused; };
struct GetMapStartPositionsResult { const Error* error; StartPosition* positions; uint32_t count; };

// API structure
struct GameApi {
	void (*IsCheatingEnabled)(const IsCheatingEnabledQuery* query, IsCheatingEnabledResult* result);
	void (*IsGodModeEnabled)(const IsGodModeEnabledQuery* query, IsGodModeEnabledResult* result);
	void (*IsDevLuaEnabled)(const IsDevLuaEnabledQuery* query, IsDevLuaEnabledResult* result);
	void (*IsEditDefsEnabled)(const IsEditDefsEnabledQuery* query, IsEditDefsEnabledResult* result);
	void (*IsNoCostEnabled)(const IsNoCostEnabledQuery* query, IsNoCostEnabledResult* result);
	void (*GetGlobalLos)(const GetGlobalLosQuery* query, GetGlobalLosResult* result);
	void (*AreHelperAIsEnabled)(const AreHelperAIsEnabledQuery* query, AreHelperAIsEnabledResult* result);
	void (*FixedAllies)(const FixedAlliesQuery* query, FixedAlliesResult* result);
	void (*IsGameOver)(const IsGameOverQuery* query, IsGameOverResult* result);
	void (*GetGameFrame)(const GetGameFrameQuery* query, GetGameFrameResult* result);
	void (*GetGameSeconds)(const GetGameSecondsQuery* query, GetGameSecondsResult* result);
	void (*GetGaiaTeamID)(const GetGaiaTeamIDQuery* query, GetGaiaTeamIDResult* result);
	void (*GetGameSetupInfo)(const GetGameSetupInfoQuery* query, GetGameSetupInfoResult* result);
	void (*GetGameMapInfo)(const GetGameMapInfoQuery* query, GetGameMapInfoResult* result);
	void (*GetGameModInfo)(const GetGameModInfoQuery* query, GetGameModInfoResult* result);
	void (*GetGameRulesInfo)(const GetGameRulesInfoQuery* query, GetGameRulesInfoResult* result);
	void (*GetMapOption)(const GetMapOptionQuery* query, GetMapOptionResult* result);
	void (*GetMapOptions)(const GetMapOptionsQuery* query, GetMapOptionsResult* result);
	void (*GetModOption)(const GetModOptionQuery* query, GetModOptionResult* result);
	void (*GetModOptions)(const GetModOptionsQuery* query, GetModOptionsResult* result);
	void (*GetTidal)(const GetTidalQuery* query, GetTidalResult* result);
	void (*GetWind)(const GetWindQuery* query, GetWindResult* result);
	void (*GetHeadingFromVector)(const GetHeadingFromVectorQuery* query, GetHeadingFromVectorResult* result);
	void (*GetVectorFromHeading)(const GetVectorFromHeadingQuery* query, GetVectorFromHeadingResult* result);
	void (*GetFacingFromHeading)(const GetFacingFromHeadingQuery* query, GetFacingFromHeadingResult* result);
	void (*GetHeadingFromFacing)(const GetHeadingFromFacingQuery* query, GetHeadingFromFacingResult* result);
	void (*GetSideData)(const GetSideDataQuery* query, GetSideDataResult* result);
	void (*GetSideDataByIndex)(const GetSideDataByIndexQuery* query, GetSideDataByIndexResult* result);
	void (*GetSideDataCount)(const GetSideDataCountQuery* query, GetSideDataCountResult* result);
	void (*GetAllyTeamStartBox)(const GetAllyTeamStartBoxQuery* query, GetAllyTeamStartBoxResult* result);
	void (*GetTeamStartPosition)(const GetTeamStartPositionQuery* query, GetTeamStartPositionResult* result);
	void (*GetMapStartPositions)(const GetMapStartPositionsQuery* query, GetMapStartPositionsResult* result);
	void (*GetGameRulesResourceInfo)(const GetGameRulesResourceInfoQuery* query, GetGameRulesResourceInfoResult* result);
};

extern const GameApi GAME_API;

#ifdef __cplusplus
}
#endif

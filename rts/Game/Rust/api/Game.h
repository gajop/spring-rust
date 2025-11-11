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

// Game options query
struct GameOptionQuery {
	const char* key;
};

struct GameOptionResult {
	const Error* error;
	const char* value;  // NULL if not found
	bool exists;
};

// Side data (faction)
struct SideData {
	const char* sideName;
	const char* caseName;  // Lowercase version
	uint32_t sideIndex;
};

struct SideDataResult {
	const Error* error;
	SideData data;
};

// Start box for an ally team
struct StartBox {
	float minX;
	float minZ;
	float maxX;
	float maxZ;
};

struct StartBoxResult {
	const Error* error;
	StartBox box;
	bool exists;
};

// Map start position
struct StartPosition {
	Float3 pos;
	int32_t teamID;
};

struct StartPositionsResult {
	const Error* error;
	StartPosition* positions;
	uint32_t count;
};

// Wind/tidal values
struct WindData {
	float min;
	float max;
	float current;
};

struct WindDataResult {
	const Error* error;
	WindData data;
};

// Heading/vector conversions
struct HeadingConversion {
	float x;
	float z;
};

// API structure
struct GameApi {
	// Game state
	BoolResult (*IsCheatingEnabled)();
	BoolResult (*IsGodModeEnabled)();
	BoolResult (*IsDevLuaEnabled)();
	BoolResult (*IsEditDefsEnabled)();
	BoolResult (*IsNoCostEnabled)();
	Int32Result (*GetGlobalLos)(int32_t allyTeamID);
	BoolResult (*AreHelperAIsEnabled)();
	BoolResult (*FixedAllies)();
	BoolResult (*IsGameOver)();

	// Frame and time
	UInt32Result (*GetGameFrame)();
	FloatResult (*GetGameSeconds)();

	// Gaia team
	Int32Result (*GetGaiaTeamID)();

	// Map and mod options
	GameOptionResult (*GetMapOption)(const char* key);
	StringArray (*GetMapOptions)();
	GameOptionResult (*GetModOption)(const char* key);
	StringArray (*GetModOptions)();

	// Environmental
	FloatResult (*GetTidal)();
	WindDataResult (*GetWind)();

	// Heading/vector conversions
	Int32Result (*GetHeadingFromVector)(float x, float z);
	Float2Result (*GetVectorFromHeading)(int32_t heading);
	Int32Result (*GetFacingFromHeading)(int32_t heading);
	Int32Result (*GetHeadingFromFacing)(int32_t facing);

	// Side (faction) data
	SideDataResult (*GetSideData)(const char* sideName);

	// Start positions
	StartBoxResult (*GetAllyTeamStartBox)(int32_t allyTeamID);
	Float3Result (*GetTeamStartPosition)(int32_t teamID);
	StartPositionsResult (*GetMapStartPositions)();
};

extern const GameApi GAME_API;

#ifdef __cplusplus
}
#endif

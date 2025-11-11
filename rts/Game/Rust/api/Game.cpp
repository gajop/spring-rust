#include "Game/Rust/api/Game.h"

#include <algorithm>
#include <cstring>
#include <cstdlib>

#include "Game/Game.h"
#include "Game/GameSetup.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/Wind.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/SideParser.h"
#include "Map/ReadMap.h"
#include "Map/MapInfo.h"
#include "Map/MapDimensions.h"
#include "Map/MapParser.h"
#include "System/SpringMath.h"
#include "System/StringUtil.h"

namespace {

// Error constants
static const Error GAME_NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Game is not initialized"
};

static const Error INVALID_OPTION_ERROR = {
	.code = ERROR_NOT_FOUND,
	.message = "Option key not found"
};

// Helper: check if game is ready
static bool GameReady()
{
	return (game != nullptr) && (gs != nullptr);
}

// Game state queries
static BoolResult NativeIsCheatingEnabled()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = gs->cheatEnabled;
	return result;
}

static BoolResult NativeIsGodModeEnabled()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = gs->godMode;
	return result;
}

static BoolResult NativeIsDevLuaEnabled()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = gs->devLuaEnabled;
	return result;
}

static BoolResult NativeIsEditDefsEnabled()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = gs->editDefsEnabled;
	return result;
}

static BoolResult NativeIsNoCostEnabled()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = gs->noCostEnabled;
	return result;
}

static Int32Result NativeGetGlobalLos(int32_t allyTeamID)
{
	Int32Result result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	if (allyTeamID < 0 || allyTeamID >= gs->activeAllyTeams) {
		result.error = &INVALID_OPTION_ERROR;
		return result;
	}
	result.value = gs->globalLOS[allyTeamID];
	return result;
}

static BoolResult NativeAreHelperAIsEnabled()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = !gs->noHelperAIs;
	return result;
}

static BoolResult NativeFixedAllies()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = (gameSetup != nullptr) && gameSetup->fixedAllies;
	return result;
}

static BoolResult NativeIsGameOver()
{
	BoolResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = game->IsGameOver();
	return result;
}

// Frame and time
static UInt32Result NativeGetGameFrame()
{
	UInt32Result result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = gs->frameNum;
	return result;
}

static FloatResult NativeGetGameSeconds()
{
	UInt32Result result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = gs->frameNum / static_cast<float>(GAME_SPEED);
	return result;
}

// Gaia team
static Int32Result NativeGetGaiaTeamID()
{
	Int32Result result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = teamHandler.GaiaTeamID();
	return result;
}

// Map and mod options
static GameOptionResult NativeGetMapOption(const char* key)
{
	GameOptionResult result = {};
	if (!GameReady() || !gameSetup) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	const auto& options = gameSetup->GetMapOptions();
	auto it = options.find(key);
	if (it != options.end()) {
		result.value = it->second.c_str();
		result.exists = true;
	} else {
		result.exists = false;
	}
	return result;
}

static StringArray NativeGetMapOptions()
{
	StringArray result = {};
	if (!GameReady() || !gameSetup) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	const auto& options = gameSetup->GetMapOptions();
	result.length = static_cast<uint32_t>(options.size());

	if (result.length == 0) {
		result.data = nullptr;
		return result;
	}

	// Allocate array of string pointers (caller must free with Memory API)
	// Strings themselves point to internal gameSetup data (valid for call duration)
	static thread_local std::vector<const char*> optionKeys;
	optionKeys.clear();
	optionKeys.reserve(options.size());

	for (const auto& [key, value] : options) {
		(void)value; // We only return keys
		optionKeys.push_back(key.c_str());
	}

	result.data = optionKeys.data();
	result.length = static_cast<uint32_t>(optionKeys.size());

	return result;
}

static GameOptionResult NativeGetModOption(const char* key)
{
	GameOptionResult result = {};
	if (!GameReady() || !gameSetup) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	const auto& options = gameSetup->GetModOptions();
	auto it = options.find(key);
	if (it != options.end()) {
		result.value = it->second.c_str();
		result.exists = true;
	} else {
		result.exists = false;
	}
	return result;
}

static StringArray NativeGetModOptions()
{
	StringArray result = {};
	if (!GameReady() || !gameSetup) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	const auto& options = gameSetup->GetModOptions();
	result.length = static_cast<uint32_t>(options.size());

	if (result.length == 0) {
		result.data = nullptr;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<const char*> optionKeys;
	optionKeys.clear();
	optionKeys.reserve(options.size());

	for (const auto& [key, value] : options) {
		(void)value; // We only return keys
		optionKeys.push_back(key.c_str());
	}

	result.data = optionKeys.data();
	result.length = static_cast<uint32_t>(optionKeys.size());

	return result;
}

// Environmental
static FloatResult NativeGetTidal()
{
	FloatResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.value = mapInfo->map.tidalStrength;
	return result;
}

static WindDataResult NativeGetWind()
{
	WindDataResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}
	result.data.min = envResHandler.GetMinWindStrength();
	result.data.max = envResHandler.GetMaxWindStrength();
	result.data.current = envResHandler.GetCurrentWindStrength();
	return result;
}

// Heading/vector conversions
static Int32Result NativeGetHeadingFromVector(float x, float z)
{
	Int32Result result = {};
	result.value = GetHeadingFromVector(x, z);
	return result;
}

static Float2Result NativeGetVectorFromHeading(int32_t heading)
{
	Float2Result result = {};
	float3 dir = GetVectorFromHeading(heading);
	result.value.x = dir.x;
	result.value.y = dir.z;
	return result;
}

static Int32Result NativeGetFacingFromHeading(int32_t heading)
{
	Int32Result result = {};
	result.value = GetFacingFromHeading(heading);
	return result;
}

static Int32Result NativeGetHeadingFromFacing(int32_t facing)
{
	Int32Result result = {};
	result.value = GetHeadingFromFacing(facing);
	return result;
}

// Side (faction) data
static SideDataResult NativeGetSideData(const char* sideName)
{
	SideDataResult result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	if (sideName == nullptr) {
		static const Error INVALID_ARG = {
			.code = ERROR_INVALID_ARGUMENT,
			.message = "sideName cannot be null"
		};
		result.error = &INVALID_ARG;
		return result;
	}

	const std::string& startUnit = sideParser.GetStartUnit(sideName);
	if (startUnit.empty()) {
		static const Error NOT_FOUND = {
			.code = ERROR_NOT_FOUND,
			.message = "Side not found"
		};
		result.error = &NOT_FOUND;
		return result;
	}

	// Store lowercase version in static storage (valid for call duration)
	static thread_local std::string lowerSideName;
	lowerSideName = StringToLower(sideName);

	result.data.sideName = lowerSideName.c_str();
	result.data.caseName = sideParser.GetCaseName(sideName).c_str();
	result.data.sideIndex = 0; // Would need to search for index if needed

	return result;
}

// Start box for ally team
static StartBoxResult NativeGetAllyTeamStartBox(int32_t allyTeamID)
{
	StartBoxResult result = {};
	result.exists = false;

	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidAllyTeam(allyTeamID)) {
		static const Error INVALID_TEAM = {
			.code = ERROR_INVALID_ARGUMENT,
			.message = "Invalid ally team ID"
		};
		result.error = &INVALID_TEAM;
		return result;
	}

	const AllyTeam& allyTeam = teamHandler.GetAllyTeam(allyTeamID);
	result.box.minX = (mapDims.mapx * SQUARE_SIZE) * allyTeam.startRectLeft;
	result.box.minZ = (mapDims.mapy * SQUARE_SIZE) * allyTeam.startRectTop;
	result.box.maxX = (mapDims.mapx * SQUARE_SIZE) * allyTeam.startRectRight;
	result.box.maxZ = (mapDims.mapy * SQUARE_SIZE) * allyTeam.startRectBottom;
	result.exists = true;

	return result;
}

// Team start position
static Float3Result NativeGetTeamStartPosition(int32_t teamID)
{
	Float3Result result = {};
	if (!GameReady()) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	if (!teamHandler.IsValidTeam(teamID)) {
		static const Error INVALID_TEAM = {
			.code = ERROR_INVALID_ARGUMENT,
			.message = "Invalid team ID"
		};
		result.error = &INVALID_TEAM;
		return result;
	}

	const CTeam* team = teamHandler.Team(teamID);
	if (team == nullptr) {
		static const Error INTERNAL = {
			.code = ERROR_INTERNAL,
			.message = "Failed to get team"
		};
		result.error = &INTERNAL;
		return result;
	}

	const float3& pos = team->GetStartPos();
	result.value.x = pos.x;
	result.value.y = pos.y;
	result.value.z = pos.z;

	return result;
}

// Map start positions
static StartPositionsResult NativeGetMapStartPositions()
{
	StartPositionsResult result = {};
	if (!GameReady() || !gameSetup) {
		result.error = &GAME_NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<StartPosition> positions;
	positions.clear();

	gameSetup->LoadStartPositionsFromMap(MAX_TEAMS, [&](MapParser& mapParser, int teamNum) {
		float3 pos;
		if (mapParser.GetStartPos(teamNum, pos)) {
			StartPosition sp;
			sp.pos.x = pos.x;
			sp.pos.y = pos.y;
			sp.pos.z = pos.z;
			sp.teamID = teamNum;
			positions.push_back(sp);
		}
		return true;
	});

	result.positions = positions.data();
	result.count = static_cast<uint32_t>(positions.size());

	return result;
}

} // namespace

// Export the API
const GameApi GAME_API = {
	.IsCheatingEnabled = NativeIsCheatingEnabled,
	.IsGodModeEnabled = NativeIsGodModeEnabled,
	.IsDevLuaEnabled = NativeIsDevLuaEnabled,
	.IsEditDefsEnabled = NativeIsEditDefsEnabled,
	.IsNoCostEnabled = NativeIsNoCostEnabled,
	.GetGlobalLos = NativeGetGlobalLos,
	.AreHelperAIsEnabled = NativeAreHelperAIsEnabled,
	.FixedAllies = NativeFixedAllies,
	.IsGameOver = NativeIsGameOver,

	.GetGameFrame = NativeGetGameFrame,
	.GetGameSeconds = NativeGetGameSeconds,

	.GetGaiaTeamID = NativeGetGaiaTeamID,

	.GetMapOption = NativeGetMapOption,
	.GetMapOptions = NativeGetMapOptions,
	.GetModOption = NativeGetModOption,
	.GetModOptions = NativeGetModOptions,

	.GetTidal = NativeGetTidal,
	.GetWind = NativeGetWind,

	.GetHeadingFromVector = NativeGetHeadingFromVector,
	.GetVectorFromHeading = NativeGetVectorFromHeading,
	.GetFacingFromHeading = NativeGetFacingFromHeading,
	.GetHeadingFromFacing = NativeGetHeadingFromFacing,

	.GetSideData = NativeGetSideData,
	.GetAllyTeamStartBox = NativeGetAllyTeamStartBox,
	.GetTeamStartPosition = NativeGetTeamStartPosition,
	.GetMapStartPositions = NativeGetMapStartPositions,
};

#include "NativeInterface/api/Game.h"

#include <algorithm>
#include <cstring>
#include <cstdlib>

#include "Game/Game.h"
#include "Game/GameSetup.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/LosHandler.h"
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

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error GAME_NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game is not initialized" };
static const Error INVALID_OPTION_ERROR = { .code = ERROR_NOT_FOUND, .message = "Option key not found" };
static const Error INVALID_ARG = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid argument" };
static const Error INVALID_TEAM = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };
static const Error NOT_FOUND = { .code = ERROR_NOT_FOUND, .message = "Not found" };
static const Error INTERNAL = { .code = ERROR_INTERNAL, .message = "Internal error" };

static bool GameReady() { return (game != nullptr) && (gs != nullptr); }

#define IMPL_SIMPLE_QUERY(name, check, value) \
	static void Native##name(const name##Query* query, name##Result* result) { \
		bufferPos = 0; \
		if (!(check)) { result->error = &GAME_NOT_READY_ERROR; return; } \
		result->error = nullptr; \
		value; \
	}

IMPL_SIMPLE_QUERY(IsCheatingEnabled, GameReady(), result->enabled = gs->cheatEnabled)
IMPL_SIMPLE_QUERY(IsGodModeEnabled, GameReady(), result->enabled = gs->godMode)
// devLuaEnabled not available in engine
IMPL_SIMPLE_QUERY(IsDevLuaEnabled, GameReady(), result->enabled = false)
IMPL_SIMPLE_QUERY(IsEditDefsEnabled, GameReady(), result->enabled = gs->editDefsEnabled)
// noCostEnabled not available in engine
IMPL_SIMPLE_QUERY(IsNoCostEnabled, GameReady(), result->enabled = false)
IMPL_SIMPLE_QUERY(AreHelperAIsEnabled, GameReady(), result->enabled = !gs->noHelperAIs)
IMPL_SIMPLE_QUERY(FixedAllies, GameReady(), result->fixed = (gameSetup != nullptr) && gameSetup->fixedAllies)
IMPL_SIMPLE_QUERY(IsGameOver, GameReady(), result->gameOver = game->IsGameOver())
IMPL_SIMPLE_QUERY(GetGameFrame, GameReady(), result->frame = gs->frameNum)
IMPL_SIMPLE_QUERY(GetGameSeconds, GameReady(), result->seconds = gs->frameNum / static_cast<float>(GAME_SPEED))
IMPL_SIMPLE_QUERY(GetGaiaTeamID, GameReady(), result->teamID = teamHandler.GaiaTeamID())
IMPL_SIMPLE_QUERY(GetTidal, GameReady(), result->strength = mapInfo->map.tidalStrength)

static void NativeGetGlobalLos(const GetGlobalLosQuery* query, GetGlobalLosResult* result) {
	bufferPos = 0;
	if (!GameReady()) { result->error = &GAME_NOT_READY_ERROR; return; }
	if (query->allyTeamID < 0 || query->allyTeamID >= teamHandler.ActiveAllyTeams()) { result->error = &INVALID_OPTION_ERROR; return; }
	result->error = nullptr;
	result->los = losHandler->GetGlobalLOS(query->allyTeamID);
}

static void NativeGetMapOption(const GetMapOptionQuery* query, GetMapOptionResult* result) {
	bufferPos = 0;
	if (!GameReady() || !gameSetup) { result->error = &GAME_NOT_READY_ERROR; return; }
	const auto& options = gameSetup->GetMapOptions();
	auto it = options.find(query->key);
	result->error = nullptr;
	if (it != options.end()) {
		result->value = it->second.c_str();
		result->exists = true;
	} else {
		result->exists = false;
	}
}

static void NativeGetMapOptions(const GetMapOptionsQuery* query, GetMapOptionsResult* result) {
	bufferPos = 0;
	if (!GameReady() || !gameSetup) { result->error = &GAME_NOT_READY_ERROR; return; }
	const auto& options = gameSetup->GetMapOptions();

	const char** keys = reinterpret_cast<const char**>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (const auto& [key, value] : options) {
		if (bufferPos + sizeof(const char*) > sizeof(scratchBuffer)) { result->error = &INTERNAL; return; }
		keys[count++] = key.c_str();
		bufferPos += sizeof(const char*);
	}

	result->error = nullptr;
	result->keys = keys;
	result->count = count;
}

static void NativeGetModOption(const GetModOptionQuery* query, GetModOptionResult* result) {
	bufferPos = 0;
	if (!GameReady() || !gameSetup) { result->error = &GAME_NOT_READY_ERROR; return; }
	const auto& options = gameSetup->GetModOptions();
	auto it = options.find(query->key);
	result->error = nullptr;
	if (it != options.end()) {
		result->value = it->second.c_str();
		result->exists = true;
	} else {
		result->exists = false;
	}
}

static void NativeGetModOptions(const GetModOptionsQuery* query, GetModOptionsResult* result) {
	bufferPos = 0;
	if (!GameReady() || !gameSetup) { result->error = &GAME_NOT_READY_ERROR; return; }
	const auto& options = gameSetup->GetModOptions();

	const char** keys = reinterpret_cast<const char**>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (const auto& [key, value] : options) {
		if (bufferPos + sizeof(const char*) > sizeof(scratchBuffer)) { result->error = &INTERNAL; return; }
		keys[count++] = key.c_str();
		bufferPos += sizeof(const char*);
	}

	result->error = nullptr;
	result->keys = keys;
	result->count = count;
}

static void NativeGetWind(const GetWindQuery* query, GetWindResult* result) {
	bufferPos = 0;
	if (!GameReady()) { result->error = &GAME_NOT_READY_ERROR; return; }
	result->error = nullptr;
	result->data.min = envResHandler.GetMinWindStrength();
	result->data.max = envResHandler.GetMaxWindStrength();
	result->data.current = envResHandler.GetCurrentWindStrength();
}

static void NativeGetHeadingFromVector(const GetHeadingFromVectorQuery* query, GetHeadingFromVectorResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->heading = GetHeadingFromVector(query->x, query->z);
}

static void NativeGetVectorFromHeading(const GetVectorFromHeadingQuery* query, GetVectorFromHeadingResult* result) {
	bufferPos = 0;
	float3 dir = GetVectorFromHeading(query->heading);
	result->error = nullptr;
	result->vector.x = dir.x;
	result->vector.y = dir.z;
}

static void NativeGetFacingFromHeading(const GetFacingFromHeadingQuery* query, GetFacingFromHeadingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->facing = GetFacingFromHeading(query->heading);
}

static void NativeGetHeadingFromFacing(const GetHeadingFromFacingQuery* query, GetHeadingFromFacingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->heading = GetHeadingFromFacing(query->facing);
}

static void NativeGetSideData(const GetSideDataQuery* query, GetSideDataResult* result) {
	bufferPos = 0;
	if (!GameReady()) { result->error = &GAME_NOT_READY_ERROR; return; }
	if (query->sideName == nullptr) { result->error = &INVALID_ARG; return; }

	const std::string& startUnit = sideParser.GetStartUnit(query->sideName);
	if (startUnit.empty()) { result->error = &NOT_FOUND; return; }

	// Copy strings to scratch buffer
	std::string lowerName = StringToLower(query->sideName);
	char* nameBuf = &scratchBuffer[bufferPos];
	size_t len = lowerName.length();
	if (bufferPos + len + 1 > sizeof(scratchBuffer)) { result->error = &INTERNAL; return; }
	memcpy(nameBuf, lowerName.c_str(), len + 1);
	bufferPos += len + 1;

	result->error = nullptr;
	result->data.sideName = nameBuf;
	result->data.caseName = sideParser.GetCaseName(query->sideName).c_str();
	result->data.sideIndex = 0;
}

static void NativeGetAllyTeamStartBox(const GetAllyTeamStartBoxQuery* query, GetAllyTeamStartBoxResult* result) {
	bufferPos = 0;
	if (!GameReady()) { result->error = &GAME_NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidAllyTeam(query->allyTeamID)) { result->error = &INVALID_TEAM; return; }

	const AllyTeam& allyTeam = teamHandler.GetAllyTeam(query->allyTeamID);
	result->error = nullptr;
	result->box.minX = (mapDims.mapx * SQUARE_SIZE) * allyTeam.startRectLeft;
	result->box.minZ = (mapDims.mapy * SQUARE_SIZE) * allyTeam.startRectTop;
	result->box.maxX = (mapDims.mapx * SQUARE_SIZE) * allyTeam.startRectRight;
	result->box.maxZ = (mapDims.mapy * SQUARE_SIZE) * allyTeam.startRectBottom;
	result->exists = true;
}

static void NativeGetTeamStartPosition(const GetTeamStartPositionQuery* query, GetTeamStartPositionResult* result) {
	bufferPos = 0;
	if (!GameReady()) { result->error = &GAME_NOT_READY_ERROR; return; }
	if (!teamHandler.IsValidTeam(query->teamID)) { result->error = &INVALID_TEAM; return; }

	const CTeam* team = teamHandler.Team(query->teamID);
	if (team == nullptr) { result->error = &INTERNAL; return; }

	const float3& pos = team->GetStartPos();
	result->error = nullptr;
	result->position.x = pos.x;
	result->position.y = pos.y;
	result->position.z = pos.z;
}

static void NativeGetMapStartPositions(const GetMapStartPositionsQuery* query, GetMapStartPositionsResult* result) {
	bufferPos = 0;
	if (!GameReady() || !gameSetup) { result->error = &GAME_NOT_READY_ERROR; return; }

	StartPosition* positions = reinterpret_cast<StartPosition*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	gameSetup->LoadStartPositionsFromMap(MAX_TEAMS, [&](MapParser& mapParser, int teamNum) {
		if (bufferPos + sizeof(StartPosition) > sizeof(scratchBuffer)) return false;

		float3 pos;
		if (mapParser.GetStartPos(teamNum, pos)) {
			positions[count].pos.x = pos.x;
			positions[count].pos.y = pos.y;
			positions[count].pos.z = pos.z;
			positions[count].teamID = teamNum;
			bufferPos += sizeof(StartPosition);
			count++;
		}
		return true;
	});

	result->error = nullptr;
	result->positions = positions;
	result->count = count;
}

} // namespace

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

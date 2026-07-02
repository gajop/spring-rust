#include "NativeInterface/api/Game.h"
#include "NativeInterface/api/Constants.h"

#include <algorithm>
#include <cstring>
#include <cstdlib>

#include "Game/Game.h"
#include "Game/GameSetup.h"
#include "Game/GlobalUnsynced.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/ModInfo.h"
#include "Sim/Misc/Wind.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/SideParser.h"
#include "Sim/Units/UnitHandler.h"
#include "Map/MapDamage.h"
#include "Map/MetalMap.h"
#include "Map/ReadMap.h"
#include "Map/MapInfo.h"
#include "Map/MapDimensions.h"
#include "Map/MapParser.h"
#include "System/FileSystem/ArchiveScanner.h"
#include "System/FileSystem/FileSystem.h"
#include "System/SpringMath.h"
#include "System/Sync/SHA512.hpp"
#include "System/StringUtil.h"

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;
static thread_local sha512::hex_digest mapChecksumDigest;
static thread_local sha512::hex_digest modChecksumDigest;

// Static errors
static const Error GAME_NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game is not initialized" };
static const Error INVALID_OPTION_ERROR = { .code = ERROR_NOT_FOUND, .message = "Option key not found" };
static const Error INVALID_ARG = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid argument" };
static const Error INVALID_TEAM = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid team ID" };
static const Error NOT_FOUND = { .code = ERROR_NOT_FOUND, .message = "Not found" };
static const Error INTERNAL = { .code = ERROR_INTERNAL, .message = "Internal error" };

static_assert(GAME_MAX_TEAMS == MAX_TEAMS);
static_assert(GAME_MAX_PLAYERS == MAX_PLAYERS);
static_assert(GAME_MAX_AIS == MAX_AIS);
static_assert(GAME_MAX_UNITS == MAX_UNITS);
static_assert(GAME_MAX_FEATURES == MAX_FEATURES);
static_assert(GAME_MAX_PROJECTILES == MAX_PROJECTILES);
static_assert(GAME_MAX_WEAPONS_PER_UNIT == MAX_WEAPONS_PER_UNIT);
static_assert(GAME_SQUARE_SIZE == SQUARE_SIZE);
static_assert(GAME_METAL_MAP_SQUARE_SIZE == static_cast<int>(METAL_MAP_SQUARE_SIZE));
static_assert(GAME_BUILD_SQUARE_SIZE == BUILD_SQUARE_SIZE);
static_assert(GAME_BUILD_GRID_RESOLUTION == BUILD_GRID_RESOLUTION);
static_assert(GAME_FOOTPRINT_SCALE == SPRING_FOOTPRINT_SCALE);
static_assert(GAME_GAME_SPEED == GAME_SPEED);
static_assert(GAME_UNIT_SLOWUPDATE_RATE == UNIT_SLOWUPDATE_RATE);
static_assert(GAME_TEAM_SLOWUPDATE_RATE == TEAM_SLOWUPDATE_RATE);

static bool GameReady() { return (game != nullptr) && (gs != nullptr); }

static const char* CopyScratchString(const std::string& value)
{
	char* buffer = &scratchBuffer[bufferPos];
	const size_t len = value.length();
	if (bufferPos + len + 1 > sizeof(scratchBuffer))
		return nullptr;

	memcpy(buffer, value.c_str(), len + 1);
	bufferPos += len + 1;
	return buffer;
}

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
IMPL_SIMPLE_QUERY(GetGameFrame, GameReady(), result->low16 = (gs->frameNum & 0xFFFFu); result->high16 = (gs->frameNum >> 16))
IMPL_SIMPLE_QUERY(GetGameSeconds, GameReady(), result->seconds = gs->frameNum / static_cast<float>(GAME_SPEED))
IMPL_SIMPLE_QUERY(GetGaiaTeamID, GameReady(), result->teamID = teamHandler.GaiaTeamID())
IMPL_SIMPLE_QUERY(GetTidal, GameReady(), result->strength = envResHandler.GetCurrentTidalStrength())

static void NativeGetGameSetupInfo(const GetGameSetupInfoQuery* /*query*/, GetGameSetupInfoResult* result) {
	bufferPos = 0;
	if (gameSetup == nullptr) { result->error = &GAME_NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->info.startPosType = gameSetup->startPosType;
	result->info.ghostedBuildings = gameSetup->ghostedBuildings;
	result->info.demoPlayName = nullptr;

	if (gameSetup->hostDemo) {
		result->info.demoPlayName = CopyScratchString(FileSystem::GetBasename(gameSetup->demoName));
		if (result->info.demoPlayName == nullptr) { result->error = &INTERNAL; return; }
	}
}

static void NativeGetGameMapInfo(const GetGameMapInfoQuery* /*query*/, GetGameMapInfoResult* result) {
	bufferPos = 0;
	if (readMap == nullptr || mapInfo == nullptr) { result->error = &GAME_NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->info.mapName = mapInfo->map.name.c_str();
	result->info.mapDescription = mapInfo->map.description.c_str();
	result->info.mapChecksum = nullptr;
	result->info.mapHardness = mapInfo->map.hardness;
	result->info.extractorRadius = mapInfo->map.extractorRadius;
	result->info.tidal = mapInfo->map.tidalStrength;
	result->info.waterDamage = mapInfo->water.damage;
	result->info.gravity = -mapInfo->map.gravity * GAME_SPEED * GAME_SPEED;
	result->info.mapX = mapDims.mapx / 64;
	result->info.mapY = mapDims.mapy / 64;
	result->info.mapSizeX = mapDims.mapx * SQUARE_SIZE;
	result->info.mapSizeZ = mapDims.mapy * SQUARE_SIZE;
	result->info.mapDamage = (mapDamage != nullptr) && !mapDamage->Disabled();

	if (archiveScanner != nullptr) {
		sha512::dump_digest(archiveScanner->GetArchiveCompleteChecksumBytes(mapInfo->map.name), mapChecksumDigest);
		result->info.mapChecksum = mapChecksumDigest.data();
	}
}

static void NativeGetGameModInfo(const GetGameModInfoQuery* /*query*/, GetGameModInfoResult* result) {
	bufferPos = 0;
	if (modInfo.filename.empty()) { result->error = &GAME_NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->info.gameName = modInfo.humanName.c_str();
	result->info.gameShortName = modInfo.shortName.c_str();
	result->info.gameVersion = modInfo.version.c_str();
	result->info.gameMutator = modInfo.mutator.c_str();
	result->info.gameDesc = modInfo.description.c_str();
	result->info.modName = modInfo.humanNameVersioned.c_str();
	result->info.modShortName = modInfo.shortName.c_str();
	result->info.modVersion = modInfo.version.c_str();
	result->info.modMutator = modInfo.mutator.c_str();
	result->info.modDesc = modInfo.description.c_str();
	result->info.modChecksum = nullptr;

	if (archiveScanner != nullptr) {
		sha512::dump_digest(archiveScanner->GetArchiveCompleteChecksumBytes(modInfo.filename), modChecksumDigest);
		result->info.modChecksum = modChecksumDigest.data();
	}
}

static ResourcePack MakeResourcePack(const SResourcePack& pack)
{
	return {
		.metal = pack.metal,
		.energy = pack.energy,
	};
}

static void NativeGetGameRulesInfo(const GetGameRulesInfoQuery* /*query*/, GetGameRulesInfoResult* result) {
	bufferPos = 0;
	if (modInfo.filename.empty()) { result->error = &GAME_NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->info.maxUnits = unitHandler.MaxUnits();
	result->info.constructionDecay = modInfo.constructionDecay;
	result->info.constructionDecayTime = modInfo.constructionDecayTime;
	result->info.constructionDecaySpeed = modInfo.constructionDecaySpeed;
	result->info.multiReclaim = modInfo.multiReclaim;
	result->info.reclaimMethod = modInfo.reclaimMethod;
	result->info.reclaimUnitMethod = modInfo.reclaimUnitMethod;
	result->info.reclaimUnitEnergyCostFactor = modInfo.reclaimUnitCostFactor.energy;
	result->info.reclaimUnitEfficiency = modInfo.reclaimUnitEfficiency.metal;
	result->info.reclaimFeatureEnergyCostFactor = modInfo.reclaimFeatureCostFactor.energy;
	result->info.reclaimUnitDrainHealth = modInfo.reclaimUnitDrainHealth;
	result->info.reclaimAllowEnemies = modInfo.reclaimAllowEnemies;
	result->info.reclaimAllowAllies = modInfo.reclaimAllowAllies;
	result->info.repairEnergyCostFactor = modInfo.repairCostFactor.energy;
	result->info.resurrectEnergyCostFactor = modInfo.resurrectCostFactor.energy;
	result->info.captureEnergyCostFactor = modInfo.captureCostFactor.energy;
	result->info.transportAir = modInfo.transportAir;
	result->info.transportShip = modInfo.transportShip;
	result->info.transportHover = modInfo.transportHover;
	result->info.transportGround = modInfo.transportGround;
	result->info.fireAtKilled = modInfo.fireAtKilled;
	result->info.fireAtCrashing = modInfo.fireAtCrashing;
	result->info.requireSonarUnderWater = modInfo.requireSonarUnderWater;
	result->info.paralyzeOnMaxHealth = modInfo.paralyzeOnMaxHealth;
	result->info.paralyzeDeclineRate = modInfo.paralyzeDeclineRate;
	result->info.allowEnginePlayerlist = modInfo.allowEnginePlayerlist;
	result->info.nativeExcessSharing = modInfo.nativeExcessSharing;
}

static void NativeGetGameRulesResourceInfo(const GetGameRulesResourceInfoQuery* /*query*/, GetGameRulesResourceInfoResult* result)
{
	bufferPos = 0;
	if (modInfo.filename.empty()) { result->error = &GAME_NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->info.reclaimUnitCostFactor = MakeResourcePack(modInfo.reclaimUnitCostFactor);
	result->info.reclaimUnitEfficiency = MakeResourcePack(modInfo.reclaimUnitEfficiency);
	result->info.reclaimFeatureCostFactor = MakeResourcePack(modInfo.reclaimFeatureCostFactor);
	result->info.repairCostFactor = MakeResourcePack(modInfo.repairCostFactor);
	result->info.resurrectCostFactor = MakeResourcePack(modInfo.resurrectCostFactor);
	result->info.captureCostFactor = MakeResourcePack(modInfo.captureCostFactor);
}

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

	const std::string lowerName = StringToLower(query->sideName);
	unsigned int sideIndex = 0;
	const unsigned int sideCount = sideParser.GetCount();
	for (; sideIndex < sideCount; ++sideIndex) {
		if (sideParser.GetSideName(sideIndex) == lowerName)
			break;
	}

	const std::string& fallbackSideName = lowerName;
	const std::string& sideName = sideIndex < sideCount
		? sideParser.GetSideName(sideIndex)
		: fallbackSideName;
	const std::string& caseName = sideParser.GetCaseName(query->sideName);

	char* nameBuf = &scratchBuffer[bufferPos];
	size_t len = sideName.length();
	if (bufferPos + len + 1 > sizeof(scratchBuffer)) { result->error = &INTERNAL; return; }
	memcpy(nameBuf, sideName.c_str(), len + 1);
	bufferPos += len + 1;

	result->error = nullptr;
	result->data.sideName = nameBuf;
	result->data.caseName = caseName.c_str();
	result->data.startUnit = startUnit.c_str();
	result->data.sideIndex = sideIndex < sideCount ? sideIndex : 0;
}

static void NativeGetSideDataByIndex(const GetSideDataByIndexQuery* query, GetSideDataByIndexResult* result) {
	bufferPos = 0;
	if (!GameReady()) { result->error = &GAME_NOT_READY_ERROR; return; }
	if (!sideParser.ValidSide(query->sideIndex)) { result->error = &NOT_FOUND; return; }

	result->error = nullptr;
	result->data.sideName = sideParser.GetSideName(query->sideIndex).c_str();
	result->data.caseName = sideParser.GetCaseName(query->sideIndex).c_str();
	result->data.startUnit = sideParser.GetStartUnit(query->sideIndex).c_str();
	result->data.sideIndex = query->sideIndex;
}

static void NativeGetSideDataCount(const GetSideDataCountQuery* query, GetSideDataCountResult* result) {
	bufferPos = 0;
	if (!GameReady()) { result->error = &GAME_NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->count = sideParser.GetCount();
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
	.GetGameSetupInfo = NativeGetGameSetupInfo,
	.GetGameMapInfo = NativeGetGameMapInfo,
	.GetGameModInfo = NativeGetGameModInfo,
	.GetGameRulesInfo = NativeGetGameRulesInfo,
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
	.GetSideDataByIndex = NativeGetSideDataByIndex,
	.GetSideDataCount = NativeGetSideDataCount,
	.GetAllyTeamStartBox = NativeGetAllyTeamStartBox,
	.GetTeamStartPosition = NativeGetTeamStartPosition,
	.GetMapStartPositions = NativeGetMapStartPositions,
	.GetGameRulesResourceInfo = NativeGetGameRulesResourceInfo,
};

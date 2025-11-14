/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Callin Event Structures
// These define the interface for events from Spring Engine to native modules
// ============================================================================

// Initialization
struct InitializeNativeModuleQuery {
	// No input parameters
};

struct InitializeNativeModuleResult {
	const Error* error;
	void* moduleData;  // Opaque pointer returned to module
};

// Download events
struct DownloadFailedQuery {
	int32_t downloadID;
	int32_t errorID;
};

struct DownloadFailedResult {
	const Error* error;
};

struct DownloadFinishedQuery {
	int32_t downloadID;
};

struct DownloadFinishedResult {
	const Error* error;
};

struct DownloadProgressQuery {
	int32_t downloadID;
	int64_t downloaded;
	int64_t total;
};

struct DownloadProgressResult {
	const Error* error;
};

struct DownloadQueuedQuery {
	int32_t downloadID;
	const char* archiveName;
	const char* archiveType;
};

struct DownloadQueuedResult {
	const Error* error;
};

struct DownloadStartedQuery {
	int32_t downloadID;
};

struct DownloadStartedResult {
	const Error* error;
};

// Feature events
struct FeatureCreatedQuery {
	int32_t featureID;
};

struct FeatureCreatedResult {
	const Error* error;
};

struct FeatureDestroyedQuery {
	int32_t featureID;
};

struct FeatureDestroyedResult {
	const Error* error;
};

// Game events
struct GameIDQuery {
	const uint8_t* gameID;
	uint32_t numBytes;
};

struct GameIDResult {
	const Error* error;
};

struct GamePausedQuery {
	int32_t playerID;
	bool paused;
};

struct GamePausedResult {
	const Error* error;
};

struct GamePreloadQuery {
	// No input parameters
};

struct GamePreloadResult {
	const Error* error;
};

struct GameStartQuery {
	// No input parameters
};

struct GameStartResult {
	const Error* error;
};

// Player events
struct PlayerAddedQuery {
	int32_t playerID;
};

struct PlayerAddedResult {
	const Error* error;
};

struct PlayerChangedQuery {
	int32_t playerID;
};

struct PlayerChangedResult {
	const Error* error;
};

struct PlayerRemovedQuery {
	int32_t playerID;
	int32_t reason;
};

struct PlayerRemovedResult {
	const Error* error;
};

// Team events
struct TeamChangedQuery {
	int32_t teamID;
};

struct TeamChangedResult {
	const Error* error;
};

struct TeamDiedQuery {
	int32_t teamID;
};

struct TeamDiedResult {
	const Error* error;
};

// Unit events
struct UnitCreatedQuery {
	int32_t unitID;
	int32_t builderID;  // -1 if none
};

struct UnitCreatedResult {
	const Error* error;
};

struct UnitDestroyedQuery {
	int32_t unitID;
	int32_t attackerID;  // -1 if none
};

struct UnitDestroyedResult {
	const Error* error;
};

struct UnitExperienceQuery {
	int32_t unitID;
	float oldExperience;
};

struct UnitExperienceResult {
	const Error* error;
};

struct UnitFinishedQuery {
	int32_t unitID;
};

struct UnitFinishedResult {
	const Error* error;
};

struct UnitFromFactoryQuery {
	int32_t unitID;
	int32_t factoryID;
	bool userOrders;
};

struct UnitFromFactoryResult {
	const Error* error;
};

struct UnitGivenQuery {
	int32_t unitID;
	int32_t oldTeam;
	int32_t newTeam;
};

struct UnitGivenResult {
	const Error* error;
};

struct UnitLoadedQuery {
	int32_t unitID;
	int32_t transportID;
};

struct UnitLoadedResult {
	const Error* error;
};

struct UnitStunnedQuery {
	int32_t unitID;
	bool stunned;
};

struct UnitStunnedResult {
	const Error* error;
};

struct UnitTakenQuery {
	int32_t unitID;
	int32_t oldTeam;
	int32_t newTeam;
};

struct UnitTakenResult {
	const Error* error;
};

struct UnitUnloadedQuery {
	int32_t unitID;
	int32_t transportID;
};

struct UnitUnloadedResult {
	const Error* error;
};

struct RenderUnitDestroyedQuery {
	int32_t unitID;
};

struct RenderUnitDestroyedResult {
	const Error* error;
};

// Special events
struct HandleLuaMsgQuery {
	int32_t playerID;
	int32_t script;
	int32_t mode;
	const uint8_t* data;
	int32_t dataLength;
};

struct HandleLuaMsgResult {
	const Error* error;
};

struct HandleLuaCallQuery {
	const char* message;
};

struct HandleLuaCallResult {
	const Error* error;
};

struct ShutdownQuery {
	// No input parameters
};

struct ShutdownResult {
	const Error* error;
};

#ifdef __cplusplus
}
#endif

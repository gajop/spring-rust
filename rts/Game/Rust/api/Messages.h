#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Messages API
// @see rts/Lua/LuaUnsyncedCtrl.cpp
//
// Chat, console, and messaging (unsynced)
// ============================================================================

// Console buffer entry
struct ConsoleEntry {
	const char* text;
	uint32_t priority;  // Log level
};

struct ConsoleBufferResult {
	const Error* error;
	ConsoleEntry* entries;
	uint32_t count;
};

// API structure
struct MessagesApi {
	// Console output
	BoolResult (*Echo)(const char* message);
	BoolResult (*Log)(const char* section, int32_t level, const char* message);

	// Chat
	BoolResult (*SendMessage)(const char* message);
	BoolResult (*SendMessageToPlayer)(int32_t playerID, const char* message);
	BoolResult (*SendMessageToTeam)(int32_t teamID, const char* message);
	BoolResult (*SendMessageToAllyTeam)(int32_t allyTeamID, const char* message);
	BoolResult (*SendMessageToSpectators)(const char* message);

	// Inter-Lua messaging
	BoolResult (*SendLuaUIMsg)(const char* message);
	BoolResult (*SendLuaGaiaMsg)(const char* message);
	BoolResult (*SendLuaRulesMsg)(const char* message);

	// Console
	ConsoleBufferResult (*GetConsoleBuffer)(uint32_t maxLines);
	StringResult (*GetCurrentTooltip)();
	BoolResult (*IsUserWriting)();
};

extern const MessagesApi MESSAGES_API;

#ifdef __cplusplus
}
#endif

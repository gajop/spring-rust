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

// Queries
struct EchoQuery { const char* message; const char* rest; };
struct EchoResult { const Error* error; bool success; };

struct LogQuery { const char* section; int32_t level; const char* message; };
struct LogResult { const Error* error; bool success; };

struct SendMessageQuery { const char* message; };
struct SendMessageResult { const Error* error; bool success; };

struct SendMessageToPlayerQuery { int32_t playerID; const char* message; };
struct SendMessageToPlayerResult { const Error* error; bool success; };

struct SendMessageToTeamQuery { int32_t teamID; const char* message; };
struct SendMessageToTeamResult { const Error* error; bool success; };

struct SendMessageToAllyTeamQuery { int32_t allyTeamID; const char* message; };
struct SendMessageToAllyTeamResult { const Error* error; bool success; };

struct SendMessageToSpectatorsQuery { const char* message; };
struct SendMessageToSpectatorsResult { const Error* error; bool success; };

struct SendPublicChatQuery { const char* message; };
struct SendPublicChatResult { const Error* error; bool success; };

struct SendAllyChatQuery { const char* message; };
struct SendAllyChatResult { const Error* error; bool success; };

struct SendSpectatorChatQuery { const char* message; };
struct SendSpectatorChatResult { const Error* error; bool success; };

struct SendPrivateChatQuery { const char* message; int32_t playerID; };
struct SendPrivateChatResult { const Error* error; bool success; };

struct SendCommandsQuery { const char* command; const char* rest; };
struct SendCommandsResult { const Error* error; bool success; };

struct SendLuaMenuMsgQuery { const char* message; };
struct SendLuaMenuMsgResult { const Error* error; bool success; };

struct SendSkirmishAIMessageQuery { int32_t aiID; const char* message; };
struct SendSkirmishAIMessageResult { const Error* error; bool success; };

struct SendLuaUIQuery { const char* message; const char* mode; };
struct SendLuaUIResult { const Error* error; bool success; };

struct SendLuaGaiaQuery { const char* message; };
struct SendLuaGaiaResult { const Error* error; bool success; };

struct SendLuaRulesQuery { const char* message; };
struct SendLuaRulesResult { const Error* error; bool success; };
// Synced-only bridge matching Lua's SendToUnsynced(string) callout.
struct SendToUnsyncedQuery { const char* message; };
struct SendToUnsyncedResult { const Error* error; bool success; };

struct GetConsoleBufferQuery { uint32_t maxLines; };
struct GetConsoleBufferResult { const Error* error; ConsoleEntry* entries; uint32_t count; };

struct GetCurrentTooltipQuery { uint8_t _unused; };
struct GetCurrentTooltipResult { const Error* error; const char* tooltip; };

struct IsUserWritingQuery { uint8_t _unused; };
struct IsUserWritingResult { const Error* error; bool writing; };

// API structure
struct MessagesApi {
	void (*Echo)(const EchoQuery* query, EchoResult* result);
	void (*Log)(const LogQuery* query, LogResult* result);
	void (*SendMessage)(const SendMessageQuery* query, SendMessageResult* result);
	void (*SendMessageToPlayer)(const SendMessageToPlayerQuery* query, SendMessageToPlayerResult* result);
	void (*SendMessageToTeam)(const SendMessageToTeamQuery* query, SendMessageToTeamResult* result);
	void (*SendMessageToAllyTeam)(const SendMessageToAllyTeamQuery* query, SendMessageToAllyTeamResult* result);
	void (*SendMessageToSpectators)(const SendMessageToSpectatorsQuery* query, SendMessageToSpectatorsResult* result);
	void (*SendPublicChat)(const SendPublicChatQuery* query, SendPublicChatResult* result);
	void (*SendAllyChat)(const SendAllyChatQuery* query, SendAllyChatResult* result);
	void (*SendSpectatorChat)(const SendSpectatorChatQuery* query, SendSpectatorChatResult* result);
	void (*SendPrivateChat)(const SendPrivateChatQuery* query, SendPrivateChatResult* result);
	void (*SendCommands)(const SendCommandsQuery* query, SendCommandsResult* result);
	void (*SendLuaMenuMsg)(const SendLuaMenuMsgQuery* query, SendLuaMenuMsgResult* result);
	void (*SendSkirmishAIMessage)(const SendSkirmishAIMessageQuery* query, SendSkirmishAIMessageResult* result);
	void (*SendLuaUIMsg)(const SendLuaUIQuery* query, SendLuaUIResult* result);
	void (*SendLuaGaiaMsg)(const SendLuaGaiaQuery* query, SendLuaGaiaResult* result);
	void (*SendLuaRulesMsg)(const SendLuaRulesQuery* query, SendLuaRulesResult* result);
	void (*SendToUnsynced)(const SendToUnsyncedQuery* query, SendToUnsyncedResult* result);
	void (*GetConsoleBuffer)(const GetConsoleBufferQuery* query, GetConsoleBufferResult* result);
	void (*GetCurrentTooltip)(const GetCurrentTooltipQuery* query, GetCurrentTooltipResult* result);
	void (*IsUserWriting)(const IsUserWritingQuery* query, IsUserWritingResult* result);
};

extern const MessagesApi MESSAGES_API;

#ifdef __cplusplus
}
#endif

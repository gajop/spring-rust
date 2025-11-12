#include "Messages.h"

#include "System/Log/ILog.h"
#include "Game/GlobalUnsynced.h"
#include "Game/UI/TooltipConsole.h"
#include "Game/UI/MouseHandler.h"
#include "System/EventClient.h"
#include "Lua/LuaHandle.h"
#include <string>
#include <vector>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Message system not ready" };

static void NativeEcho(const EchoQuery* query, EchoResult* result) {
	bufferPos = 0;
	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	LOG("%s", query->message);
	result->error = nullptr;
	result->success = true;
}

static void NativeLog(const LogQuery* query, LogResult* result) {
	bufferPos = 0;
	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	// Simplified: just log to default section
	LOG("%s", query->message);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendMessage(const SendMessageQuery* query, SendMessageResult* result) {
	bufferPos = 0;
	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	LOG("%s", query->message);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendMessageToPlayer(const SendMessageToPlayerQuery* query, SendMessageToPlayerResult* result) {
	bufferPos = 0;
	if (query->message == nullptr || gu == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	// Only display if it's for the local player
	if (query->playerID == gu->myPlayerNum) {
		LOG("%s", query->message);
	}

	result->error = nullptr;
	result->success = true;
}

static void NativeSendMessageToTeam(const SendMessageToTeamQuery* query, SendMessageToTeamResult* result) {
	bufferPos = 0;
	if (query->message == nullptr || gu == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	// Only display if it's for the local player's team
	if (query->teamID == gu->myTeam) {
		LOG("%s", query->message);
	}

	result->error = nullptr;
	result->success = true;
}

static void NativeSendMessageToAllyTeam(const SendMessageToAllyTeamQuery* query, SendMessageToAllyTeamResult* result) {
	bufferPos = 0;
	if (query->message == nullptr || gu == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	// Only display if it's for the local player's ally team
	if (query->allyTeamID == gu->myAllyTeam) {
		LOG("%s", query->message);
	}

	result->error = nullptr;
	result->success = true;
}

static void NativeSendMessageToSpectators(const SendMessageToSpectatorsQuery* query, SendMessageToSpectatorsResult* result) {
	bufferPos = 0;
	if (query->message == nullptr || gu == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	// Only display if local player is spectating
	if (gu->spectating) {
		LOG("%s", query->message);
	}

	result->error = nullptr;
	result->success = true;
}

static void NativeSendLuaUIMsg(const SendLuaUIQuery* query, SendLuaUIResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = (query->message != nullptr);
}

static void NativeSendLuaGaiaMsg(const SendLuaGaiaQuery* query, SendLuaGaiaResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = (query->message != nullptr);
}

static void NativeSendLuaRulesMsg(const SendLuaRulesQuery* query, SendLuaRulesResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = (query->message != nullptr);
}

static void NativeGetConsoleBuffer(const GetConsoleBufferQuery* query, GetConsoleBufferResult* result) {
	bufferPos = 0;
	// Console buffer access would require interfacing with the logging system
	// which stores messages internally - simplified implementation
	result->error = nullptr;
	result->entries = nullptr;
	result->count = 0;
}

static void NativeGetCurrentTooltip(const GetCurrentTooltipQuery* query, GetCurrentTooltipResult* result) {
	bufferPos = 0;

	if (tooltip == nullptr) {
		result->error = nullptr;
		result->tooltip = "";
		return;
	}

	// Use scratch buffer for tooltip text
	static thread_local std::string tooltipText;
	tooltipText = tooltip->GetTooltip();

	// Copy to scratch buffer if needed
	const size_t len = tooltipText.length() + 1;
	if (bufferPos + len <= sizeof(scratchBuffer)) {
		char* str = &scratchBuffer[bufferPos];
		memcpy(str, tooltipText.c_str(), len);
		bufferPos += len;
		result->tooltip = str;
	} else {
		result->tooltip = "";
	}

	result->error = nullptr;
}

static void NativeIsUserWriting(const IsUserWritingQuery* query, IsUserWritingResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->writing = (mouse != nullptr) && mouse->locked;
}

} // namespace

const MessagesApi MESSAGES_API = {
	.Echo = NativeEcho,
	.Log = NativeLog,
	.SendMessage = NativeSendMessage,
	.SendMessageToPlayer = NativeSendMessageToPlayer,
	.SendMessageToTeam = NativeSendMessageToTeam,
	.SendMessageToAllyTeam = NativeSendMessageToAllyTeam,
	.SendMessageToSpectators = NativeSendMessageToSpectators,
	.SendLuaUIMsg = NativeSendLuaUIMsg,
	.SendLuaGaiaMsg = NativeSendLuaGaiaMsg,
	.SendLuaRulesMsg = NativeSendLuaRulesMsg,
	.GetConsoleBuffer = NativeGetConsoleBuffer,
	.GetCurrentTooltip = NativeGetCurrentTooltip,
	.IsUserWriting = NativeIsUserWriting,
};

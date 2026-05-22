#include "Messages.h"

#include "System/Log/ILog.h"
#include "Game/GlobalUnsynced.h"
#include "Game/Game.h"
#include "Game/ChatMessage.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/TooltipConsole.h"
#include "Game/UI/MouseHandler.h"
#include "ExternalAI/EngineOutHandler.h"
#include "System/EventClient.h"
#include "Lua/LuaHandle.h"
#include "Lua/LuaMenu.h"
#include <string>
#include <vector>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Message system not ready" };
static const Error INVALID_PLAYER_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid player" };
static const Error GUI_NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "GUI handler not ready" };

static void NativeEcho(const EchoQuery* query, EchoResult* result) {
	bufferPos = 0;
	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	std::string composed = query->message;
	if (query->rest != nullptr) {
		composed += " ";
		composed += query->rest;
	}

	LOG("%s", composed.c_str());
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

static void NativeSendPublicChat(const SendPublicChatQuery* query, SendPublicChatResult* result) {
	bufferPos = 0;

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (game == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}

	game->SendNetChat(query->message, ChatMessage::TO_EVERYONE);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendAllyChat(const SendAllyChatQuery* query, SendAllyChatResult* result) {
	bufferPos = 0;

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (game == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}

	game->SendNetChat(query->message, ChatMessage::TO_ALLIES);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendSpectatorChat(const SendSpectatorChatQuery* query, SendSpectatorChatResult* result) {
	bufferPos = 0;

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (game == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}

	game->SendNetChat(query->message, ChatMessage::TO_SPECTATORS);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendPrivateChat(const SendPrivateChatQuery* query, SendPrivateChatResult* result) {
	bufferPos = 0;

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (game == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}
	if (!playerHandler.IsValidPlayer(query->playerID)) {
		result->error = &INVALID_PLAYER_ERROR;
		result->success = false;
		return;
	}

	game->SendNetChat(query->message, query->playerID);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendCommands(const SendCommandsQuery* query, SendCommandsResult* result) {
	bufferPos = 0;

	if (query->command == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (guihandler == nullptr) {
		result->error = &GUI_NOT_READY_ERROR;
		result->success = false;
		return;
	}

	std::string command = query->command;
	if (query->rest != nullptr && query->rest[0] != '\0') {
		command += "\n";
		command += query->rest;
	}
	if (!command.empty() && command[0] != '@')
		command = "@@" + command;

	guihandler->RunCustomCommands({command}, false);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendLuaMenuMsg(const SendLuaMenuMsgQuery* query, SendLuaMenuMsgResult* result) {
	bufferPos = 0;

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (luaMenu == nullptr || gu == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}

	luaMenu->RecvLuaMsg(query->message, gu->myPlayerNum);
	result->error = nullptr;
	result->success = true;
}

static void NativeSendSkirmishAIMessage(const SendSkirmishAIMessageQuery* query, SendSkirmishAIMessageResult* result) {
	bufferPos = 0;

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (eoh == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}

	std::vector<const char*> outData;
	result->success = eoh->SendLuaMessages(query->aiID, query->message, outData);
	result->error = nullptr;
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
	tooltipText = tooltip->GetTooltip(0, 0);

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
	.SendPublicChat = NativeSendPublicChat,
	.SendAllyChat = NativeSendAllyChat,
	.SendSpectatorChat = NativeSendSpectatorChat,
	.SendPrivateChat = NativeSendPrivateChat,
	.SendCommands = NativeSendCommands,
	.SendLuaMenuMsg = NativeSendLuaMenuMsg,
	.SendSkirmishAIMessage = NativeSendSkirmishAIMessage,
	.SendLuaUIMsg = NativeSendLuaUIMsg,
	.SendLuaGaiaMsg = NativeSendLuaGaiaMsg,
	.SendLuaRulesMsg = NativeSendLuaRulesMsg,
	.GetConsoleBuffer = NativeGetConsoleBuffer,
	.GetCurrentTooltip = NativeGetCurrentTooltip,
	.IsUserWriting = NativeIsUserWriting,
};

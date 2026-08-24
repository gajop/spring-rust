#include "Messages.h"

#include "System/Log/ILog.h"
#include "Game/GlobalUnsynced.h"
#include "Game/Game.h"
#include "Game/ChatMessage.h"
#include "Game/Players/PlayerHandler.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/InfoConsole.h"
#include "Game/UI/TooltipConsole.h"
#include "Game/UI/MouseHandler.h"
#include "ExternalAI/EngineOutHandler.h"
#include "System/EventClient.h"
#include "NativeInterface/NativeInterfaceSystem.h"
#include "Lua/LuaHandle.h"
#include "Lua/LuaMenu.h"
#include "Lua/LuaRules.h"
#include "Lua/LuaUI.h"
#include "Sim/Misc/TeamHandler.h"
#include <algorithm>
#include <cstring>
#include <string>
#include <vector>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;
static thread_local std::vector<std::string> consoleTextBuffer;
static thread_local std::vector<ConsoleEntry> consoleEntryBuffer;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Message system not ready" };
static const Error INVALID_PLAYER_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid player" };
static const Error INVALID_LUA_UI_MODE_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid LuaUI message mode" };
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
	if (luaMenu != nullptr && gu != nullptr) {
		luaMenu->RecvLuaMsg(query->message, gu->myPlayerNum);
	}

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

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (luaUI == nullptr || gu == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}

	const char mode = (query->mode != nullptr) ? query->mode[0] : 0;
	if (mode != 0 && mode != 'a' && mode != 's') {
		result->error = &INVALID_LUA_UI_MODE_ERROR;
		result->success = false;
		return;
	}

	bool sendMsg = false;
	switch (mode) {
		case 0: {
			sendMsg = true;
		} break;
		case 's': {
			sendMsg = gu->spectating;
		} break;
		case 'a': {
			if (gu->spectatingFullView) {
				sendMsg = true;
			} else {
				sendMsg = teamHandler.Ally(gu->myAllyTeam, gu->myAllyTeam);
			}
		} break;
	}

	if (sendMsg)
		luaUI->RecvLuaMsg(query->message, gu->myPlayerNum);

	result->error = nullptr;
	result->success = true;
}

static void NativeSendLuaGaiaMsg(const SendLuaGaiaQuery* query, SendLuaGaiaResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = (query->message != nullptr);
}

static void NativeSendLuaRulesMsg(const SendLuaRulesQuery* query, SendLuaRulesResult* result) {
	bufferPos = 0;

	if (query->message == nullptr) {
		result->error = nullptr;
		result->success = false;
		return;
	}
	if (gu == nullptr || (luaRules == nullptr && NativeInterfaceSystem::s_instance == nullptr)) {
		result->error = &NOT_READY_ERROR;
		result->success = false;
		return;
	}

	if (luaRules != nullptr)
		luaRules->RecvLuaMsg(query->message, gu->myPlayerNum);
	// LuaRules normally owns RecvLuaMsg. When the rules implementation is a
	// Core-WASM module there may be no Lua callin to receive the message, so
	// route the same generic message through the native event bridge as well.
	if (NativeInterfaceSystem::s_instance != nullptr) {
		const auto* begin = reinterpret_cast<const std::uint8_t*>(query->message);
		const auto* end = begin + std::strlen(query->message);
		NativeInterfaceSystem::s_instance->HandleLuaMsg(
			gu->myPlayerNum, LUA_HANDLE_ORDER_RULES, 0, std::vector<std::uint8_t>(begin, end));
	}
	result->error = nullptr;
	result->success = true;
}

static void NativeSendToUnsynced(const SendToUnsyncedQuery* query, SendToUnsyncedResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (query->message == nullptr) {
		return;
	}
	if (luaRules != nullptr) {
		if (!luaRules->SendToUnsyncedMessage(query->message)) {
			result->error = &NOT_READY_ERROR;
			return;
		}
	} else if (NativeInterfaceSystem::s_instance != nullptr) {
		std::string error;
		if (!NativeInterfaceSystem::s_instance->DispatchWasmSyncedMessage(query->message, error)) {
			result->error = &NOT_READY_ERROR;
			return;
		}
	} else {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->success = true;
}

static void NativeGetConsoleBuffer(const GetConsoleBufferQuery* query, GetConsoleBufferResult* result) {
	bufferPos = 0;

	if (infoConsole == nullptr) {
		result->error = nullptr;
		result->entries = nullptr;
		result->count = 0;
		return;
	}

	std::vector<CInfoConsole::RawLine> lines;
	infoConsole->GetRawLines(lines);

	const size_t lineCount = lines.size();
	size_t startLine = 0;

	if (query != nullptr && query->maxLines > 0) {
		startLine = lineCount - std::min(lineCount, size_t(query->maxLines));
	}

	consoleTextBuffer.clear();
	consoleEntryBuffer.clear();
	consoleTextBuffer.reserve(lineCount - startLine);
	consoleEntryBuffer.reserve(lineCount - startLine);

	for (size_t i = startLine; i < lineCount; ++i) {
		consoleTextBuffer.push_back(lines[i].text);
		consoleEntryBuffer.push_back({
			.text = consoleTextBuffer.back().c_str(),
			.priority = uint32_t(lines[i].level),
		});
	}

	result->error = nullptr;
	result->entries = consoleEntryBuffer.empty() ? nullptr : consoleEntryBuffer.data();
	result->count = uint32_t(consoleEntryBuffer.size());
}

static void NativeGetCurrentTooltip(const GetCurrentTooltipQuery* query, GetCurrentTooltipResult* result) {
	bufferPos = 0;

	if (mouse == nullptr) {
		result->error = nullptr;
		result->tooltip = "";
		return;
	}

	// Use scratch buffer for tooltip text
	static thread_local std::string tooltipText;
	tooltipText = mouse->GetCurrentTooltip();

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
	.SendToUnsynced = NativeSendToUnsynced,
	.GetConsoleBuffer = NativeGetConsoleBuffer,
	.GetCurrentTooltip = NativeGetCurrentTooltip,
	.IsUserWriting = NativeIsUserWriting,
};

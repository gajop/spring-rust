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

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Message system not ready"
};

// Console output
static BoolResult NativeEcho(const char* message)
{
	BoolResult result = {};
	if (message == nullptr) {
		result.value = false;
		return result;
	}

	LOG("%s", message);
	result.value = true;
	return result;
}

static BoolResult NativeLog(const char* section, int32_t level, const char* message)
{
	BoolResult result = {};
	if (message == nullptr) {
		result.value = false;
		return result;
	}

	// Use LOG system with section filtering
	// Simplified: just log to default section
	LOG("%s", message);
	result.value = true;
	return result;
}

// Chat
static BoolResult NativeSendMessage(const char* message)
{
	BoolResult result = {};
	if (message == nullptr) {
		result.value = false;
		return result;
	}

	LOG("%s", message);
	result.value = true;
	return result;
}

static BoolResult NativeSendMessageToPlayer(int32_t playerID, const char* message)
{
	BoolResult result = {};
	if (message == nullptr || gu == nullptr) {
		result.value = false;
		return result;
	}

	// Only display if it's for the local player
	if (playerID == gu->myPlayerNum) {
		LOG("%s", message);
	}

	result.value = true;
	return result;
}

static BoolResult NativeSendMessageToTeam(int32_t teamID, const char* message)
{
	BoolResult result = {};
	if (message == nullptr || gu == nullptr) {
		result.value = false;
		return result;
	}

	// Only display if it's for the local player's team
	if (teamID == gu->myTeam) {
		LOG("%s", message);
	}

	result.value = true;
	return result;
}

static BoolResult NativeSendMessageToAllyTeam(int32_t allyTeamID, const char* message)
{
	BoolResult result = {};
	if (message == nullptr || gu == nullptr) {
		result.value = false;
		return result;
	}

	// Only display if it's for the local player's ally team
	if (allyTeamID == gu->myAllyTeam) {
		LOG("%s", message);
	}

	result.value = true;
	return result;
}

static BoolResult NativeSendMessageToSpectators(const char* message)
{
	BoolResult result = {};
	if (message == nullptr || gu == nullptr) {
		result.value = false;
		return result;
	}

	// Only display if local player is spectating
	if (gu->spectating) {
		LOG("%s", message);
	}

	result.value = true;
	return result;
}

// Inter-Lua messaging (simplified - just return success)
static BoolResult NativeSendLuaUIMsg(const char* message)
{
	BoolResult result = {};
	result.value = (message != nullptr);
	return result;
}

static BoolResult NativeSendLuaGaiaMsg(const char* message)
{
	BoolResult result = {};
	result.value = (message != nullptr);
	return result;
}

static BoolResult NativeSendLuaRulesMsg(const char* message)
{
	BoolResult result = {};
	result.value = (message != nullptr);
	return result;
}

// Console (simplified - not fully implemented)
static ConsoleBufferResult NativeGetConsoleBuffer(uint32_t maxLines)
{
	ConsoleBufferResult result = {};
	// Console buffer access would require interfacing with the logging system
	// which stores messages internally - simplified implementation
	result.entries = nullptr;
	result.count = 0;
	return result;
}

static StringResult NativeGetCurrentTooltip()
{
	StringResult result = {};

	if (tooltip == nullptr) {
		result.value = "";
		return result;
	}

	// Use static storage
	static thread_local std::string tooltipText;
	tooltipText = tooltip->GetTooltip();
	result.value = tooltipText.c_str();
	return result;
}

static BoolResult NativeIsUserWriting()
{
	BoolResult result = {};
	result.value = (mouse != nullptr) && mouse->locked;
	return result;
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

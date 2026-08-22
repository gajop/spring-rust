/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreMessagesBindings.h"

#include <cstdint>
#include <string_view>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

std::int32_t NativeErrorCode(const Error* error)
{
	return error == nullptr ? 0 : error->code;
}

void ReturnBool(wasmtime_val_raw_t* slots, const Error* error, bool value)
{
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(value ? 1u : 0u, NativeErrorCode(error)));
}

void ReturnStatus(wasmtime_val_raw_t* slots, Status status)
{
	slots[0].i64 = static_cast<std::int64_t>(
		PackU32(0, static_cast<std::int32_t>(status)));
}

template<typename Query, typename Result, typename Fill>
wasm_trap_t* CallOneString(HostState* state, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*function)(const Query*, Result*), Fill&& fill, std::string_view name)
{
	if (slots == nullptr || slotCount != 2)
		return Trap(std::string(name) + " Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> first;
	if (!first.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32),
			static_cast<std::uint32_t>(slots[1].i32))) {
		ReturnStatus(slots, Status::OutOfBounds);
		return nullptr;
	}
	Query query{};
	fill(query, first.c_str());
	Result result{};
	function(&query, &result);
	ReturnBool(slots, result.error, result.success);
	return nullptr;
}

template<typename Query, typename Result, typename Fill>
wasm_trap_t* CallTwoStrings(HostState* state, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*function)(const Query*, Result*), Fill&& fill, std::string_view name)
{
	if (slots == nullptr || slotCount != 4)
		return Trap(std::string(name) + " Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> first;
	GuestCString<> second;
	if (!first.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32),
			static_cast<std::uint32_t>(slots[1].i32)) ||
		!second.Read(state->memory, static_cast<std::uint32_t>(slots[2].i32),
			static_cast<std::uint32_t>(slots[3].i32))) {
		ReturnStatus(slots, Status::OutOfBounds);
		return nullptr;
	}
	Query query{};
	fill(query, first.c_str(), second.c_str());
	Result result{};
	function(&query, &result);
	ReturnBool(slots, result.error, result.success);
	return nullptr;
}

template<typename Query, typename Result, typename Fill>
wasm_trap_t* CallIdString(HostState* state, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*function)(const Query*, Result*), Fill&& fill, std::string_view name)
{
	if (slots == nullptr || slotCount != 3)
		return Trap(std::string(name) + " Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> message;
	if (!message.Read(state->memory, static_cast<std::uint32_t>(slots[1].i32),
			static_cast<std::uint32_t>(slots[2].i32))) {
		ReturnStatus(slots, Status::OutOfBounds);
		return nullptr;
	}
	Query query{};
	fill(query, slots[0].i32, message.c_str());
	Result result{};
	function(&query, &result);
	ReturnBool(slots, result.error, result.success);
	return nullptr;
}

template<typename Query, typename Result, typename Fill>
wasm_trap_t* CallStringId(HostState* state, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*function)(const Query*, Result*), Fill&& fill, std::string_view name)
{
	if (slots == nullptr || slotCount != 3)
		return Trap(std::string(name) + " Core ABI signature mismatch");
	ImportGuard guard(state, 4);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> message;
	if (!message.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32),
			static_cast<std::uint32_t>(slots[1].i32))) {
		ReturnStatus(slots, Status::OutOfBounds);
		return nullptr;
	}
	Query query{};
	fill(query, message.c_str(), slots[2].i32);
	Result result{};
	function(&query, &result);
	ReturnBool(slots, result.error, result.success);
	return nullptr;
}

template<typename Query, typename Result, typename Fill>
wasm_trap_t* CallStringIntString(HostState* state, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*function)(const Query*, Result*), Fill&& fill, std::string_view name)
{
	if (slots == nullptr || slotCount != 5)
		return Trap(std::string(name) + " Core ABI signature mismatch");
	ImportGuard guard(state, 6);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	GuestCString<> first;
	GuestCString<> second;
	if (!first.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32),
			static_cast<std::uint32_t>(slots[1].i32)) ||
		!second.Read(state->memory, static_cast<std::uint32_t>(slots[3].i32),
			static_cast<std::uint32_t>(slots[4].i32))) {
		ReturnStatus(slots, Status::OutOfBounds);
		return nullptr;
	}
	Query query{};
	fill(query, first.c_str(), slots[2].i32, second.c_str());
	Result result{};
	function(&query, &result);
	ReturnBool(slots, result.error, result.success);
	return nullptr;
}

#define MESSAGE_ONE_STRING(CALLBACK, MEMBER, QUERY, RESULT, FIELD) \
wasm_trap_t* CALLBACK(void* environment, wasmtime_caller_t* caller, \
	wasmtime_val_raw_t* slots, std::size_t slotCount) \
{ \
	auto* state = static_cast<HostState*>(environment); \
	if (state == nullptr || state->native == nullptr || state->native->messages == nullptr || \
		state->native->messages->MEMBER == nullptr) \
		return Trap(#MEMBER " Core binding is unavailable"); \
	return CallOneString<QUERY, RESULT>(state, caller, slots, slotCount, \
		state->native->messages->MEMBER, [](QUERY& query, const char* value) { query.FIELD = value; }, #MEMBER); \
}

#define MESSAGE_TWO_STRINGS(CALLBACK, MEMBER, QUERY, RESULT, FIRST, SECOND) \
wasm_trap_t* CALLBACK(void* environment, wasmtime_caller_t* caller, \
	wasmtime_val_raw_t* slots, std::size_t slotCount) \
{ \
	auto* state = static_cast<HostState*>(environment); \
	if (state == nullptr || state->native == nullptr || state->native->messages == nullptr || \
		state->native->messages->MEMBER == nullptr) \
		return Trap(#MEMBER " Core binding is unavailable"); \
	return CallTwoStrings<QUERY, RESULT>(state, caller, slots, slotCount, \
		state->native->messages->MEMBER, [](QUERY& query, const char* first, const char* second) { \
			query.FIRST = first; query.SECOND = second; \
		}, #MEMBER); \
}

#define MESSAGE_ID_STRING(CALLBACK, MEMBER, QUERY, RESULT, ID_FIELD, STRING_FIELD) \
wasm_trap_t* CALLBACK(void* environment, wasmtime_caller_t* caller, \
	wasmtime_val_raw_t* slots, std::size_t slotCount) \
{ \
	auto* state = static_cast<HostState*>(environment); \
	if (state == nullptr || state->native == nullptr || state->native->messages == nullptr || \
		state->native->messages->MEMBER == nullptr) \
		return Trap(#MEMBER " Core binding is unavailable"); \
	return CallIdString<QUERY, RESULT>(state, caller, slots, slotCount, \
		state->native->messages->MEMBER, [](QUERY& query, std::int32_t id, const char* value) { \
			query.ID_FIELD = id; query.STRING_FIELD = value; \
		}, #MEMBER); \
}

MESSAGE_TWO_STRINGS(Echo, Echo, EchoQuery, EchoResult, message, rest)

wasm_trap_t* Log(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->messages == nullptr ||
		state->native->messages->Log == nullptr)
		return Trap("Log Core binding is unavailable");
	return CallStringIntString<LogQuery, LogResult>(state, caller, slots, slotCount,
		state->native->messages->Log,
		[](LogQuery& query, const char* section, std::int32_t level, const char* message) {
			query.section = section;
			query.level = level;
			query.message = message;
		}, "Log");
}

MESSAGE_ONE_STRING(SendMessage, SendMessage, SendMessageQuery, SendMessageResult, message)
MESSAGE_ID_STRING(SendMessageToPlayer, SendMessageToPlayer, SendMessageToPlayerQuery,
	SendMessageToPlayerResult, playerID, message)
MESSAGE_ID_STRING(SendMessageToTeam, SendMessageToTeam, SendMessageToTeamQuery,
	SendMessageToTeamResult, teamID, message)
MESSAGE_ID_STRING(SendMessageToAllyTeam, SendMessageToAllyTeam, SendMessageToAllyTeamQuery,
	SendMessageToAllyTeamResult, allyTeamID, message)
MESSAGE_ONE_STRING(SendMessageToSpectators, SendMessageToSpectators,
	SendMessageToSpectatorsQuery, SendMessageToSpectatorsResult, message)
MESSAGE_ONE_STRING(SendPublicChat, SendPublicChat, SendPublicChatQuery, SendPublicChatResult, message)
MESSAGE_ONE_STRING(SendAllyChat, SendAllyChat, SendAllyChatQuery, SendAllyChatResult, message)
MESSAGE_ONE_STRING(SendSpectatorChat, SendSpectatorChat, SendSpectatorChatQuery,
	SendSpectatorChatResult, message)

wasm_trap_t* SendPrivateChat(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->messages == nullptr ||
		state->native->messages->SendPrivateChat == nullptr)
		return Trap("SendPrivateChat Core binding is unavailable");
	return CallStringId<SendPrivateChatQuery, SendPrivateChatResult>(state, caller, slots, slotCount,
		state->native->messages->SendPrivateChat,
		[](SendPrivateChatQuery& query, const char* message, std::int32_t playerID) {
			query.message = message;
			query.playerID = playerID;
		}, "SendPrivateChat");
}

MESSAGE_TWO_STRINGS(SendCommands, SendCommands, SendCommandsQuery, SendCommandsResult, command, rest)
MESSAGE_ONE_STRING(SendLuaMenuMsg, SendLuaMenuMsg, SendLuaMenuMsgQuery, SendLuaMenuMsgResult, message)
MESSAGE_ID_STRING(SendSkirmishAIMessage, SendSkirmishAIMessage, SendSkirmishAIMessageQuery,
	SendSkirmishAIMessageResult, aiID, message)
MESSAGE_TWO_STRINGS(SendLuaUIMsg, SendLuaUIMsg, SendLuaUIQuery, SendLuaUIResult, message, mode)
MESSAGE_ONE_STRING(SendLuaGaiaMsg, SendLuaGaiaMsg, SendLuaGaiaQuery, SendLuaGaiaResult, message)
MESSAGE_ONE_STRING(SendLuaRulesMsg, SendLuaRulesMsg, SendLuaRulesQuery, SendLuaRulesResult, message)
MESSAGE_ONE_STRING(SendToUnsynced, SendToUnsynced, SendToUnsyncedQuery, SendToUnsyncedResult, message)

#undef MESSAGE_ID_STRING
#undef MESSAGE_TWO_STRINGS
#undef MESSAGE_ONE_STRING

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:messages", 15,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterMessagesImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->messages == nullptr) {
		error = "cannot register Messages Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t oneString[] = {WASM_I32, WASM_I32};
	const wasm_valkind_t twoStrings[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t idString[] = {WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t logParams[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t result[] = {WASM_I64};

#define DEFINE_MESSAGE(NAME, IMPORT, PARAMS) \
	if (!Define(linker, IMPORT, MakeFuncType(PARAMS, sizeof(PARAMS) / sizeof(PARAMS[0]), result, 1), \
			NAME, state, error)) return false

	DEFINE_MESSAGE(Echo, "echo", twoStrings);
	DEFINE_MESSAGE(Log, "log", logParams);
	DEFINE_MESSAGE(SendMessage, "send-message", oneString);
	DEFINE_MESSAGE(SendMessageToPlayer, "send-message-to-player", idString);
	DEFINE_MESSAGE(SendMessageToTeam, "send-message-to-team", idString);
	DEFINE_MESSAGE(SendMessageToAllyTeam, "send-message-to-ally-team", idString);
	DEFINE_MESSAGE(SendMessageToSpectators, "send-message-to-spectators", oneString);
	DEFINE_MESSAGE(SendPublicChat, "send-public-chat", oneString);
	DEFINE_MESSAGE(SendAllyChat, "send-ally-chat", oneString);
	DEFINE_MESSAGE(SendSpectatorChat, "send-spectator-chat", oneString);
	DEFINE_MESSAGE(SendPrivateChat, "send-private-chat", idString);
	DEFINE_MESSAGE(SendCommands, "send-commands", twoStrings);
	DEFINE_MESSAGE(SendLuaMenuMsg, "send-lua-menu-msg", oneString);
	DEFINE_MESSAGE(SendSkirmishAIMessage, "send-skirmish-ai-message", idString);
	DEFINE_MESSAGE(SendLuaUIMsg, "send-lua-ui-msg", twoStrings);
	DEFINE_MESSAGE(SendLuaGaiaMsg, "send-lua-gaia-msg", oneString);
	DEFINE_MESSAGE(SendLuaRulesMsg, "send-lua-rules-msg", oneString);
	DEFINE_MESSAGE(SendToUnsynced, "send-to-unsynced", oneString);

#undef DEFINE_MESSAGE
	return true;
}

#endif

} // namespace recoil::wasm::core

#include "Input.h"

#include <SDL_keyboard.h>

#include "Game/UI/MouseHandler.h"
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/MiniMap.h"
#include "Game/Camera.h"
#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Rendering/GlobalRendering.h"
#include "System/Input/KeyInput.h"
#include "System/float3.h"
#include "Lua/LuaConfig.h"
#include <string>
#include <cstring>
#include <vector>

#ifndef SDL_BUTTON_LEFT
#define SDL_BUTTON_LEFT 1
#define SDL_BUTTON_MIDDLE 2
#define SDL_BUTTON_RIGHT 3
#endif

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Input system not ready"
};

static const Error INVALID_ARG_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static const Error BUFFER_OVERFLOW_ERROR = {
	.code = ERROR_BUFFER_OVERFLOW,
	.message = "Buffer overflow"
};

static const char* CopyString(const char* str)
{
	if (str == nullptr)
		return "";

	const size_t len = strlen(str) + 1;
	if (bufferPos + len > sizeof(scratchBuffer))
		return "";

	char* out = &scratchBuffer[bufferPos];
	memcpy(out, str, len);
	bufferPos += len;
	return out;
}

static const char* CopyString(const std::string& str)
{
	return CopyString(str.c_str());
}

// Helper: check if ready
static bool IsReady()
{
	return (mouse != nullptr) && (globalRendering != nullptr);
}

// Mouse
static void NativeGetMouseState(const GetMouseStateQuery* query, GetMouseStateResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->state.x = static_cast<float>(mouse->lastx - globalRendering->viewPosX);
	result->state.y = static_cast<float>(globalRendering->viewSizeY - mouse->lasty - 1);
	result->state.dx = 0.0f; // Delta not directly available
	result->state.dy = 0.0f;
	result->state.left = mouse->buttons[SDL_BUTTON_LEFT].pressed;
	result->state.middle = mouse->buttons[SDL_BUTTON_MIDDLE].pressed;
	result->state.right = mouse->buttons[SDL_BUTTON_RIGHT].pressed;
	result->state.offscreen = mouse->offscreen;
}

static void NativeGetMouseCursor(const GetMouseCursorQuery* query, GetMouseCursorResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Copy cursor name to scratch buffer
	std::string cursorName = mouse->GetCurrentCursor();
	char* strBuf = &scratchBuffer[bufferPos];
	size_t len = cursorName.length();

	if (bufferPos + len + 1 > sizeof(scratchBuffer)) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	memcpy(strBuf, cursorName.c_str(), len + 1);
	bufferPos += len + 1;

	result->error = nullptr;
	result->cursor = strBuf;
}

static void NativeGetMouseButtonsPressed(const GetMouseButtonsPressedQuery* query, GetMouseButtonsPressedResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		result->pressed = nullptr;
		result->count = 0;
		return;
	}

	if (query->buttons == nullptr || query->count == 0) {
		result->error = &INVALID_ARG_ERROR;
		result->pressed = nullptr;
		result->count = 0;
		return;
	}

	const size_t needed = query->count * sizeof(bool);
	if (bufferPos + needed > sizeof(scratchBuffer)) {
		result->error = &INVALID_ARG_ERROR;
		result->pressed = nullptr;
		result->count = 0;
		return;
	}

	bool* states = reinterpret_cast<bool*>(&scratchBuffer[bufferPos]);
	for (uint32_t i = 0; i < query->count; ++i) {
		const int32_t button = query->buttons[i];
		if (button <= 0 || button > NUM_BUTTONS) {
			states[i] = false;
		} else {
			states[i] = mouse->buttons[button].pressed;
		}
	}

	bufferPos += needed;
	result->error = nullptr;
	result->pressed = states;
	result->count = query->count;
}

static void NativeGetMouseStartPosition(const GetMouseStartPositionQuery* query, GetMouseStartPositionResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Validate button index (1-based in Spring)
	if (query->button <= 0 || query->button > NUM_BUTTONS) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	const CMouseHandler::ButtonPressEvt& bp = mouse->buttons[query->button];
	result->error = nullptr;
	result->position.x = static_cast<float>(bp.x);
	result->position.y = static_cast<float>(bp.y);
}

// Keyboard
static void NativeGetKeyState(const GetKeyStateQuery* query, GetKeyStateResult* result)
{
	bufferPos = 0;

	result->error = nullptr;
	result->pressed = KeyInput::IsKeyPressed(query->keyCode);
}

static void NativeGetPressedKeys(const GetPressedKeysQuery* query, GetPressedKeysResult* result)
{
	bufferPos = 0;

	const auto& pressedKeys = KeyInput::GetPressedKeys();

	// Write keys to scratch buffer
	int32_t* keys = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (const auto& pair : pressedKeys) {
		if (pair.second) {
			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) {
				result->error = &INVALID_ARG_ERROR;
				return;
			}
			keys[count++] = pair.first;
			bufferPos += sizeof(int32_t);
		}
	}

	result->error = nullptr;
	result->keys = keys;
	result->count = count;
}

static void NativeGetPressedScans(const GetPressedScansQuery* query, GetPressedScansResult* result)
{
	bufferPos = 0;

	const auto& pressedScans = KeyInput::GetPressedScans();

	// Write scans to scratch buffer
	int32_t* scans = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;

	for (const auto& pair : pressedScans) {
		if (pair.second) {
			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) {
				result->error = &INVALID_ARG_ERROR;
				return;
			}
			scans[count++] = pair.first;
			bufferPos += sizeof(int32_t);
		}
	}

	result->error = nullptr;
	result->scans = scans;
	result->count = count;
}

// Modifier keys
static void NativeGetModKeyState(const GetModKeyStateQuery* query, GetModKeyStateResult* result)
{
	bufferPos = 0;

	// Return as bitfield: shift | ctrl | alt | meta
	uint32_t modState = 0;
	if (KeyInput::GetKeyModState(KMOD_SHIFT)) modState |= (1 << 0);
	if (KeyInput::GetKeyModState(KMOD_CTRL))  modState |= (1 << 1);
	if (KeyInput::GetKeyModState(KMOD_ALT))   modState |= (1 << 2);
	if (KeyInput::GetKeyModState(KMOD_GUI))   modState |= (1 << 3);

	result->error = nullptr;
	result->modState = modState;
}

// Selection
static void NativeGetSelectionBox(const GetSelectionBoxQuery* query, GetSelectionBoxResult* result)
{
	bufferPos = 0;

	if (!IsReady() || camera == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	float3 bl, br, tl, tr;
	if (!mouse->GetSelectionBoxVertices(bl, br, tl, tr)) {
		result->error = nullptr;
		result->box.active = false;
		return;
	}

	const float3 bottomLeft = camera->CalcViewPortCoordinates(bl);
	const float3 topRight = camera->CalcViewPortCoordinates(tr);

	result->error = nullptr;
	result->box.left = bottomLeft.x;
	result->box.top = topRight.y;
	result->box.right = topRight.x;
	result->box.bottom = bottomLeft.y;
	result->box.active = true;
}

static void NativeGetInvertQueueKey(const GetInvertQueueKeyQuery* query, GetInvertQueueKeyResult* result)
{
	bufferPos = 0;

	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->invert = false;
		return;
	}

	result->error = nullptr;
	result->invert = guihandler->GetInvertQueueKey();
}

static void NativeIsAboveMiniMap(const IsAboveMiniMapQuery* query, IsAboveMiniMapResult* result)
{
	bufferPos = 0;

	if (minimap == nullptr || !IsReady()) {
		result->error = nullptr;
		result->above = false;
		return;
	}

	if (minimap->GetMinimized() || (game != nullptr && game->hideInterface)) {
		result->error = nullptr;
		result->above = false;
		return;
	}

	const int x = static_cast<int>(query->screenX) + globalRendering->viewPosX;
	const int y = static_cast<int>(query->screenY) + globalRendering->viewPosY;

	const int x0 = minimap->GetPosX();
	const int y0 = minimap->GetPosY();
	const int x1 = x0 + minimap->GetSizeX();
	const int y1 = y0 + minimap->GetSizeY();

	result->error = nullptr;
	result->above = (x >= x0) && (x < x1) && (y >= y0) && (y < y1);
}

// Active command
static void NativeGetActiveCommand(const GetActiveCommandQuery* query, GetActiveCommandResult* result)
{
	bufferPos = 0;
	(void)query;

	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	result->commandID = 0;
	result->commandType = 0;
	result->commandName = nullptr;

	const int inCommand = guihandler->inCommand;
	result->commandIndex = inCommand + CMD_INDEX_OFFSET;

	const std::vector<SCommandDescription>& cmdDescs = guihandler->commands;
	if (inCommand < 0 || inCommand >= static_cast<int>(cmdDescs.size()))
		return;

	const SCommandDescription& desc = cmdDescs[inCommand];
	result->commandID = desc.id;
	result->commandType = desc.type;
	result->commandName = CopyString(desc.name);
	if (result->commandName == nullptr)
		result->error = &BUFFER_OVERFLOW_ERROR;
}

static void NativeGetActionHotKeys(const GetActionHotKeysQuery* query, GetActionHotKeysResult* result)
{
	bufferPos = 0;
	(void)query;

	result->error = nullptr;
	result->hotkeys = nullptr;
	result->count = 0;
}

static void NativeGetKeyBindings(const GetKeyBindingsQuery* query, GetKeyBindingsResult* result)
{
	bufferPos = 0;
	(void)query;

	result->error = nullptr;
	result->bindings = nullptr;
	result->count = 0;
}

static void NativeGetKeyCode(const GetKeyCodeQuery* query, GetKeyCodeResult* result)
{
	bufferPos = 0;
	(void)query;

	result->error = nullptr;
	result->keyCode = 0;
}

static void NativeGetKeySymbol(const GetKeySymbolQuery* query, GetKeySymbolResult* result)
{
	bufferPos = 0;
	(void)query;

	result->error = nullptr;
	result->keyCodeName = "";
	result->keyCodeDefaultName = "";
}

static void NativeGetScanSymbol(const GetScanSymbolQuery* query, GetScanSymbolResult* result)
{
	bufferPos = 0;
	(void)query;

	result->error = nullptr;
	result->scanCodeName = "";
	result->scanCodeDefaultName = "";
}

static void NativeGetKeyFromScanSymbol(const GetKeyFromScanSymbolQuery* query, GetKeyFromScanSymbolResult* result)
{
	bufferPos = 0;
	(void)query;

	result->error = nullptr;
	result->keyCode = 0;
}

static void NativeGetActivePage(const GetActivePageQuery* query, GetActivePageResult* result)
{
	bufferPos = 0;

	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->activePage = 0;
		result->maxPage = 0;
		return;
	}

	result->error = nullptr;
	result->activePage = guihandler->GetActivePage();
	result->maxPage = guihandler->GetMaxPage();
}

static void NativeGetDefaultCommand(const GetDefaultCommandQuery* query, GetDefaultCommandResult* result)
{
	bufferPos = 0;

	if (guihandler == nullptr || !IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->error = nullptr;
	const int defCmd = guihandler->GetDefaultCommand(mouse->lastx, mouse->lasty);
	result->commandIndex = defCmd + CMD_INDEX_OFFSET;

	const auto& cmdDescs = guihandler->commands;
	if (defCmd < 0 || defCmd >= (int)cmdDescs.size()) {
		result->commandID = -1;
		result->commandType = -1;
		result->commandName = nullptr;
		return;
	}

	const SCommandDescription& cd = cmdDescs[defCmd];
	result->commandID = cd.id;
	result->commandType = cd.type;
	result->commandName = cd.name.c_str();
}

} // namespace

const InputApi INPUT_API = {
	.GetMouseState = NativeGetMouseState,
	.GetMouseCursor = NativeGetMouseCursor,
	.GetMouseButtonsPressed = NativeGetMouseButtonsPressed,
	.GetMouseStartPosition = NativeGetMouseStartPosition,

	.GetKeyState = NativeGetKeyState,
	.GetPressedKeys = NativeGetPressedKeys,
	.GetPressedScans = NativeGetPressedScans,

	.GetModKeyState = NativeGetModKeyState,

	.GetSelectionBox = NativeGetSelectionBox,
	.GetInvertQueueKey = NativeGetInvertQueueKey,
	.IsAboveMiniMap = NativeIsAboveMiniMap,

	.GetActiveCommand = NativeGetActiveCommand,
	.GetActionHotKeys = NativeGetActionHotKeys,
	.GetKeyBindings = NativeGetKeyBindings,
	.GetKeyCode = NativeGetKeyCode,
	.GetKeySymbol = NativeGetKeySymbol,
	.GetScanSymbol = NativeGetScanSymbol,
	.GetKeyFromScanSymbol = NativeGetKeyFromScanSymbol,
	.GetActivePage = NativeGetActivePage,
	.GetDefaultCommand = NativeGetDefaultCommand,
};

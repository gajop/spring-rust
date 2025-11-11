#include "Input.h"

#include "Game/UI/MouseHandler.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/MiniMap.h"
#include "Game/Camera.h"
#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Rendering/GlobalRendering.h"
#include "System/Input/KeyInput.h"
#include "System/float3.h"
#include <vector>

#ifndef SDL_BUTTON_LEFT
#define SDL_BUTTON_LEFT 1
#define SDL_BUTTON_MIDDLE 2
#define SDL_BUTTON_RIGHT 3
#endif

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Input system not ready"
};

// Helper: check if ready
static bool IsReady()
{
	return (mouse != nullptr) && (globalRendering != nullptr);
}

// Mouse
static MouseStateResult NativeGetMouseState()
{
	MouseStateResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.state.x = static_cast<float>(mouse->lastx - globalRendering->viewPosX);
	result.state.y = static_cast<float>(globalRendering->viewSizeY - mouse->lasty - 1);
	result.state.dx = 0.0f; // Delta not directly available
	result.state.dy = 0.0f;
	result.state.left = mouse->buttons[SDL_BUTTON_LEFT].pressed;
	result.state.middle = mouse->buttons[SDL_BUTTON_MIDDLE].pressed;
	result.state.right = mouse->buttons[SDL_BUTTON_RIGHT].pressed;
	result.state.offscreen = mouse->offscreen;

	return result;
}

static StringResult NativeGetMouseCursor()
{
	StringResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage
	static thread_local std::string cursorName;
	cursorName = mouse->GetCurrentCursor();
	result.value = cursorName.c_str();
	return result;
}

static Float2Result NativeGetMouseStartPosition(int32_t button)
{
	Float2Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Validate button index (1-based in Spring)
	if (button <= 0 || button > NUM_BUTTONS) {
		result.value.x = 0.0f;
		result.value.y = 0.0f;
		return result;
	}

	const CMouseHandler::ButtonPressEvt& bp = mouse->buttons[button];
	result.value.x = static_cast<float>(bp.x);
	result.value.y = static_cast<float>(bp.y);
	return result;
}

// Keyboard
static BoolResult NativeGetKeyState(int32_t keyCode)
{
	BoolResult result = {};
	result.value = KeyInput::IsKeyPressed(keyCode);
	return result;
}

static Int32Array NativeGetPressedKeys()
{
	Int32Array result = {};

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> keys;
	keys.clear();

	const auto& pressedKeys = KeyInput::GetPressedKeys();
	for (const auto& pair : pressedKeys) {
		if (pair.second) {
			keys.push_back(pair.first);
		}
	}

	result.data = keys.data();
	result.length = static_cast<uint32_t>(keys.size());
	return result;
}

static Int32Array NativeGetPressedScans()
{
	Int32Array result = {};

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> scans;
	scans.clear();

	const auto& pressedScans = KeyInput::GetPressedScans();
	for (const auto& pair : pressedScans) {
		if (pair.second) {
			scans.push_back(pair.first);
		}
	}

	result.data = scans.data();
	result.length = static_cast<uint32_t>(scans.size());
	return result;
}

// Modifier keys
static BoolResult NativeGetModKeyState()
{
	BoolResult result = {};
	// Return as bitfield: shift | ctrl | alt | meta
	uint32_t modState = 0;
	if (KeyInput::GetKeyModState(KMOD_SHIFT)) modState |= (1 << 0);
	if (KeyInput::GetKeyModState(KMOD_CTRL))  modState |= (1 << 1);
	if (KeyInput::GetKeyModState(KMOD_ALT))   modState |= (1 << 2);
	if (KeyInput::GetKeyModState(KMOD_GUI))   modState |= (1 << 3);

	result.value = (modState != 0);
	return result;
}

// Selection
static SelectionBoxResult NativeGetSelectionBox()
{
	SelectionBoxResult result = {};
	if (!IsReady() || camera == nullptr) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	float3 bl, br, tl, tr;
	if (!mouse->GetSelectionBoxVertices(bl, br, tl, tr)) {
		result.box.active = false;
		return result;
	}

	const float3 bottomLeft = camera->CalcViewPortCoordinates(bl);
	const float3 topRight = camera->CalcViewPortCoordinates(tr);

	result.box.left = bottomLeft.x;
	result.box.top = topRight.y;
	result.box.right = topRight.x;
	result.box.bottom = bottomLeft.y;
	result.box.active = true;

	return result;
}

static BoolResult NativeIsAboveMiniMap(float screenX, float screenY)
{
	BoolResult result = {};
	if (minimap == nullptr || !IsReady()) {
		result.value = false;
		return result;
	}

	if (minimap->GetMinimized() || (game != nullptr && game->hideInterface)) {
		result.value = false;
		return result;
	}

	const int x = static_cast<int>(screenX) + globalRendering->viewPosX;
	const int y = static_cast<int>(screenY) + globalRendering->viewPosY;

	const int x0 = minimap->GetPosX();
	const int y0 = minimap->GetPosY();
	const int x1 = x0 + minimap->GetSizeX();
	const int y1 = y0 + minimap->GetSizeY();

	result.value = (x >= x0) && (x < x1) && (y >= y0) && (y < y1);
	return result;
}

// Active command
static Int32Result NativeGetActiveCommand()
{
	Int32Result result = {};
	if (guihandler == nullptr) {
		result.value = -1;
		return result;
	}

	const int inCommand = guihandler->inCommand;
	result.value = inCommand; // Return index
	return result;
}

static Int32Result NativeGetDefaultCommand()
{
	Int32Result result = {};
	if (guihandler == nullptr || !IsReady()) {
		result.value = -1;
		return result;
	}

	const int defCmd = guihandler->GetDefaultCommand(mouse->lastx, mouse->lasty);
	result.value = defCmd;
	return result;
}

} // namespace

const InputApi INPUT_API = {
	.GetMouseState = NativeGetMouseState,
	.GetMouseCursor = NativeGetMouseCursor,
	.GetMouseStartPosition = NativeGetMouseStartPosition,

	.GetKeyState = NativeGetKeyState,
	.GetPressedKeys = NativeGetPressedKeys,
	.GetPressedScans = NativeGetPressedScans,

	.GetModKeyState = NativeGetModKeyState,

	.GetSelectionBox = NativeGetSelectionBox,
	.IsAboveMiniMap = NativeIsAboveMiniMap,

	.GetActiveCommand = NativeGetActiveCommand,
	.GetDefaultCommand = NativeGetDefaultCommand,
};

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "DebugInput.h"

#include <SDL_keyboard.h>
#include <SDL_keycode.h>

#include <set>

#include "Game/GameController.h"
#include "Game/UI/KeyBindings.h"
#include "Game/UI/KeyCodes.h"
#include "Game/UI/MouseHandler.h"
#include "Game/UI/ScanCodes.h"
#include "System/Input/KeyInput.h"
#include "System/Input/MouseInput.h"
#include "System/Platform/SDL1_keysym.h"

namespace {

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Debug input system not ready"
};

static const Error INVALID_ARG_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static bool IsReady(bool requireController, bool requireMouse, bool requireMouseInput)
{
	return (!requireController || activeController != nullptr) &&
		(!requireMouse || mouse != nullptr) &&
		(!requireMouseInput || mouseInput != nullptr);
}

static bool IsPhysicalKeyDown(SDL_Scancode scanCode)
{
	int numKeys = 0;
	const uint8_t* keyboardState = SDL_GetKeyboardState(&numKeys);
	return (static_cast<int>(scanCode) < numKeys) && (keyboardState != nullptr) && (keyboardState[scanCode] != 0);
}

static void NativeEmulateKey(const EmulateKeyQuery* query, EmulateKeyResult* result)
{
	result->error = nullptr;

	if (!IsReady(true, false, false)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const int rawKey = SDL12_keysyms(query->keyCode);
	if (rawKey == SDLK_UNKNOWN) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	const SDL_Scancode scanCode = SDL_GetScancodeFromKey(static_cast<SDL_Keycode>(rawKey));
	const int eventKey = CKeyCodes::GetNormalizedSymbol(rawKey);
	const int eventScanCode = CScanCodes::GetNormalizedSymbol(scanCode);

	const bool physicalDown = IsPhysicalKeyDown(scanCode);
	const bool wasDown = physicalDown || KeyInput::IsKeyEmulated(rawKey);

	KeyInput::SetKeyEmulated(rawKey, query->pressed);
	KeyInput::Update(keyBindings.GetFakeMetaKey());

	if (query->pressed) {
		if (!wasDown)
			activeController->KeyPressed(eventKey, eventScanCode, false);
	} else if (wasDown && !physicalDown) {
		activeController->KeyReleased(eventKey, eventScanCode);
	}
}

static void NativeEmulateMouseButton(const EmulateMouseButtonQuery* query, EmulateMouseButtonResult* result)
{
	result->error = nullptr;

	if (!IsReady(false, true, false)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->button < 1 || query->button > NUM_BUTTONS) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	mouse->SetButtonEmulated(query->button, query->pressed);
}

static void NativeEmulateMouseMove(const EmulateMouseMoveQuery* query, EmulateMouseMoveResult* result)
{
	result->error = nullptr;

	if (!IsReady(false, true, true)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const int2 previousPosition = mouseInput->GetPos();
	mouseInput->SetPos(int2(query->x, query->y));
	mouse->MouseMove(query->x, query->y, query->x - previousPosition.x, query->y - previousPosition.y);
}

static void NativeEmulateMouseWheel(const EmulateMouseWheelQuery* query, EmulateMouseWheelResult* result)
{
	result->error = nullptr;

	if (!IsReady(false, true, false)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	mouse->MouseWheel(query->delta);
}

static void NativeEmulateTextInput(const EmulateTextInputQuery* query, EmulateTextInputResult* result)
{
	result->error = nullptr;
	result->consumed = false;

	if (!IsReady(true, false, false)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->utf8Text == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	result->consumed = activeController->TextInput(query->utf8Text) != 0;
}

static void NativeEmulateTextEditing(const EmulateTextEditingQuery* query, EmulateTextEditingResult* result)
{
	result->error = nullptr;
	result->consumed = false;

	if (!IsReady(true, false, false)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->utf8Text == nullptr) {
		result->error = &INVALID_ARG_ERROR;
		return;
	}

	result->consumed = activeController->TextEditing(query->utf8Text, query->start, query->length) != 0;
}

static void NativeClearEmulatedInput(const ClearEmulatedInputQuery* query, ClearEmulatedInputResult* result)
{
	result->error = nullptr;

	const std::set<int> emulatedKeys = KeyInput::GetEmulatedKeys();

	KeyInput::ClearEmulatedKeys();
	KeyInput::Update(keyBindings.GetFakeMetaKey());

	if (query->fireReleases && activeController != nullptr) {
		for (const int rawKey : emulatedKeys) {
			const SDL_Scancode scanCode = SDL_GetScancodeFromKey(static_cast<SDL_Keycode>(rawKey));

			if (IsPhysicalKeyDown(scanCode))
				continue;

			activeController->KeyReleased(
				CKeyCodes::GetNormalizedSymbol(rawKey),
				CScanCodes::GetNormalizedSymbol(scanCode)
			);
		}
	}

	// Clear is intentionally best-effort: the mouse may already have been
	// destroyed during teardown after the key state was still available.
	if (mouse == nullptr)
		return;

	if (!query->fireReleases) {
		mouse->ClearEmulatedButtons();
		return;
	}

	for (int button = 1; button <= NUM_BUTTONS; ++button) {
		if (mouse->IsButtonEmulated(button))
			mouse->SetButtonEmulated(button, false);
	}
}

} // namespace

const DebugInputApi DEBUG_INPUT_API = {
	.EmulateKey = NativeEmulateKey,
	.EmulateMouseButton = NativeEmulateMouseButton,
	.EmulateMouseMove = NativeEmulateMouseMove,
	.EmulateMouseWheel = NativeEmulateMouseWheel,
	.EmulateTextInput = NativeEmulateTextInput,
	.EmulateTextEditing = NativeEmulateTextEditing,
	.ClearEmulatedInput = NativeClearEmulatedInput,
};

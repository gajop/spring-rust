#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Input API
// @see rts/Lua/LuaUnsyncedRead.cpp
//
// Mouse and keyboard input queries (unsynced)
// ============================================================================

// Mouse state
struct MouseState {
	float x;
	float y;
	float dx;
	float dy;
	bool left;
	bool middle;
	bool right;
	bool offscreen;
};

struct MouseStateResult {
	const Error* error;
	MouseState state;
};

// Key state
struct KeyState {
	bool pressed;
	bool released;
};

// Selection box
struct SelectionBox {
	float left;
	float top;
	float right;
	float bottom;
	bool active;
};

struct SelectionBoxResult {
	const Error* error;
	SelectionBox box;
};

// API structure
struct InputApi {
	// Mouse
	MouseStateResult (*GetMouseState)();
	StringResult (*GetMouseCursor)();
	Float2Result (*GetMouseStartPosition)(int32_t button);

	// Keyboard
	BoolResult (*GetKeyState)(int32_t keyCode);
	Int32Array (*GetPressedKeys)();
	Int32Array (*GetPressedScans)();

	// Modifier keys
	BoolResult (*GetModKeyState)();  // Returns shift/ctrl/alt/meta as bitfield

	// Selection
	SelectionBoxResult (*GetSelectionBox)();
	BoolResult (*IsAboveMiniMap)(float screenX, float screenY);

	// Active command
	Int32Result (*GetActiveCommand)();
	Int32Result (*GetDefaultCommand)();
};

extern const InputApi INPUT_API;

#ifdef __cplusplus
}
#endif

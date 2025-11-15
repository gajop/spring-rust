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

// Selection box
struct SelectionBox {
	float left;
	float top;
	float right;
	float bottom;
	bool active;
};

// Queries
struct GetMouseStateQuery {
	uint8_t _unused;
};

struct GetMouseStateResult {
	const Error* error;
	MouseState state;
};

struct GetMouseCursorQuery {
	uint8_t _unused;
};

struct GetMouseCursorResult {
	const Error* error;
	const char* cursor;
};

struct GetMouseStartPositionQuery {
	int32_t button;
};

struct GetMouseStartPositionResult {
	const Error* error;
	Float2 position;
};

struct GetKeyStateQuery {
	int32_t keyCode;
};

struct GetKeyStateResult {
	const Error* error;
	bool pressed;
};

struct GetPressedKeysQuery {
	uint8_t _unused;
};

struct GetPressedKeysResult {
	const Error* error;
	int32_t* keys;
	uint32_t count;
};

struct GetPressedScansQuery {
	uint8_t _unused;
};

struct GetPressedScansResult {
	const Error* error;
	int32_t* scans;
	uint32_t count;
};

struct GetModKeyStateQuery {
	uint8_t _unused;
};

struct GetModKeyStateResult {
	const Error* error;
	uint32_t modState;  // Bitfield: shift | ctrl | alt | meta
};

struct GetSelectionBoxQuery {
	uint8_t _unused;
};

struct GetSelectionBoxResult {
	const Error* error;
	SelectionBox box;
};

struct IsAboveMiniMapQuery {
	float screenX;
	float screenY;
};

struct IsAboveMiniMapResult {
	const Error* error;
	bool above;
};

struct GetActiveCommandQuery {
	uint8_t _unused;
};

struct GetActiveCommandResult {
	const Error* error;
	int32_t commandIndex;
};

struct GetDefaultCommandQuery {
	uint8_t _unused;
};

struct GetDefaultCommandResult {
	const Error* error;
	int32_t commandIndex;
};

// API structure
struct InputApi {
	void (*GetMouseState)(
		const GetMouseStateQuery* query,
		GetMouseStateResult* result
	);

	void (*GetMouseCursor)(
		const GetMouseCursorQuery* query,
		GetMouseCursorResult* result
	);

	void (*GetMouseStartPosition)(
		const GetMouseStartPositionQuery* query,
		GetMouseStartPositionResult* result
	);

	void (*GetKeyState)(
		const GetKeyStateQuery* query,
		GetKeyStateResult* result
	);

	void (*GetPressedKeys)(
		const GetPressedKeysQuery* query,
		GetPressedKeysResult* result
	);

	void (*GetPressedScans)(
		const GetPressedScansQuery* query,
		GetPressedScansResult* result
	);

	void (*GetModKeyState)(
		const GetModKeyStateQuery* query,
		GetModKeyStateResult* result
	);

	void (*GetSelectionBox)(
		const GetSelectionBoxQuery* query,
		GetSelectionBoxResult* result
	);

	void (*IsAboveMiniMap)(
		const IsAboveMiniMapQuery* query,
		IsAboveMiniMapResult* result
	);

	void (*GetActiveCommand)(
		const GetActiveCommandQuery* query,
		GetActiveCommandResult* result
	);

	void (*GetDefaultCommand)(
		const GetDefaultCommandQuery* query,
		GetDefaultCommandResult* result
	);
};

extern const InputApi INPUT_API;

#ifdef __cplusplus
}
#endif

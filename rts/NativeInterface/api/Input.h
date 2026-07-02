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

struct GetMouseButtonsPressedQuery {
	const int32_t* buttons;
	uint32_t count;
};

struct GetMouseButtonsPressedResult {
	const Error* error;
	bool* pressed;
	uint32_t count;
};

struct GetMouseStartPositionQuery {
	int32_t button;
};

struct GetMouseStartPositionResult {
	const Error* error;
	Float2 position;
	Float3 camPos;
	Float3 dir;
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

struct GetInvertQueueKeyQuery { uint8_t _unused; };
struct GetInvertQueueKeyResult { const Error* error; bool invert; };

struct IsAboveMiniMapQuery {
	float screenX;
	float screenY;
};

struct IsAboveMiniMapResult {
	const Error* error;
	bool above;
};

struct GetActiveCommandQuery { uint8_t _unused; };

struct GetActiveCommandResult {
	const Error* error;
	int32_t commandIndex;
	int32_t commandID;
	int32_t commandType;
	const char* commandName;
};

struct GetActionHotKeysQuery { const char* action; };
struct GetActionHotKeysResult { const Error* error; const char** hotkeys; uint32_t count; };

struct KeyBindingEntry {
	const char* command;
	const char* extra;
	const char* boundWith;
};

struct GetKeyBindingsQuery { const char* keySet1; const char* keySet2; };
struct GetKeyBindingsResult { const Error* error; KeyBindingEntry* bindings; uint32_t count; };

struct GetKeyCodeQuery { const char* keySym; };
struct GetKeyCodeResult { const Error* error; int32_t keyCode; };

struct GetKeySymbolQuery { int32_t keyCode; };
struct GetKeySymbolResult { const Error* error; const char* keyCodeName; const char* keyCodeDefaultName; };

struct GetScanSymbolQuery { int32_t scanCode; };
struct GetScanSymbolResult { const Error* error; const char* scanCodeName; const char* scanCodeDefaultName; };

struct GetKeyFromScanSymbolQuery { const char* scanSymbol; };
struct GetKeyFromScanSymbolResult { const Error* error; const char* keyName; };

struct GetActivePageQuery { uint8_t _unused; };
struct GetActivePageResult { const Error* error; int32_t activePage; int32_t maxPage; };

struct GetDefaultCommandQuery {
	uint8_t _unused;
};

struct GetDefaultCommandResult {
	const Error* error;
	int32_t commandIndex;
	int32_t commandID;
	int32_t commandType;
	const char* commandName;
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

	void (*GetMouseButtonsPressed)(
		const GetMouseButtonsPressedQuery* query,
		GetMouseButtonsPressedResult* result
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

	void (*GetInvertQueueKey)(
		const GetInvertQueueKeyQuery* query,
		GetInvertQueueKeyResult* result
	);

	void (*IsAboveMiniMap)(
		const IsAboveMiniMapQuery* query,
		IsAboveMiniMapResult* result
	);

	void (*GetActiveCommand)(
		const GetActiveCommandQuery* query,
		GetActiveCommandResult* result
	);

	void (*GetActionHotKeys)(
		const GetActionHotKeysQuery* query,
		GetActionHotKeysResult* result
	);

	void (*GetKeyBindings)(
		const GetKeyBindingsQuery* query,
		GetKeyBindingsResult* result
	);

	void (*GetKeyCode)(
		const GetKeyCodeQuery* query,
		GetKeyCodeResult* result
	);

	void (*GetKeySymbol)(
		const GetKeySymbolQuery* query,
		GetKeySymbolResult* result
	);

	void (*GetScanSymbol)(
		const GetScanSymbolQuery* query,
		GetScanSymbolResult* result
	);

	void (*GetKeyFromScanSymbol)(
		const GetKeyFromScanSymbolQuery* query,
		GetKeyFromScanSymbolResult* result
	);

	void (*GetActivePage)(
		const GetActivePageQuery* query,
		GetActivePageResult* result
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

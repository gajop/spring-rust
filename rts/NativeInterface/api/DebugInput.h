#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// DebugInput API
//
// Feeds input to the engine as if it came from real hardware, for automated
// tests and tooling. Emulated presses are held in a store separate from the
// hardware state and OR'd into it; an event fires only when the combined
// (physical-or-emulated) state actually changes, so consumers never observe
// two Presses or two Releases in a row.
//
// Threading: main thread only. A driver receiving commands off-thread must
// queue them and drain from the Update callin.
// ============================================================================

// keyCode is an SDL1.2 keysym, matching InputApi::GetKeyState and
// InputApi::GetPressedKeys. Do NOT pass a raw SDL2 keycode.
struct EmulateKeyQuery  { int32_t keyCode; bool pressed; };
struct EmulateKeyResult { const Error* error; };

// button is 1..NUM_BUTTONS (1 = left, 2 = middle, 3 = right)
struct EmulateMouseButtonQuery  { int32_t button; bool pressed; };
struct EmulateMouseButtonResult { const Error* error; };

// x/y are TOP-LEFT origin pixels: the engine/SDL convention. No flip is
// applied. Does not move the OS cursor.
struct EmulateMouseMoveQuery  { int32_t x; int32_t y; };
struct EmulateMouseMoveResult { const Error* error; };

struct EmulateMouseWheelQuery  { float delta; };
struct EmulateMouseWheelResult { const Error* error; };

// Routed through activeController, i.e. the same path a real SDL_TEXTINPUT
// takes. `consumed` reports whether the pipeline handled the text.
struct EmulateTextInputQuery  { const char* utf8Text; };
struct EmulateTextInputResult { const Error* error; bool consumed; };

struct EmulateTextEditingQuery  { const char* utf8Text; uint32_t start; uint32_t length; };
struct EmulateTextEditingResult { const Error* error; bool consumed; };

// Releases everything currently held emulated. fireReleases=true dispatches
// release events for keys/buttons that end up effectively up; false drops the
// flags silently (for teardown, when handles are already being destroyed).
struct ClearEmulatedInputQuery  { bool fireReleases; };
struct ClearEmulatedInputResult { const Error* error; };

struct DebugInputApi {
	void (*EmulateKey)(
		const EmulateKeyQuery* query,
		EmulateKeyResult* result
	);

	void (*EmulateMouseButton)(
		const EmulateMouseButtonQuery* query,
		EmulateMouseButtonResult* result
	);

	void (*EmulateMouseMove)(
		const EmulateMouseMoveQuery* query,
		EmulateMouseMoveResult* result
	);

	void (*EmulateMouseWheel)(
		const EmulateMouseWheelQuery* query,
		EmulateMouseWheelResult* result
	);

	void (*EmulateTextInput)(
		const EmulateTextInputQuery* query,
		EmulateTextInputResult* result
	);

	void (*EmulateTextEditing)(
		const EmulateTextEditingQuery* query,
		EmulateTextEditingResult* result
	);

	void (*ClearEmulatedInput)(
		const ClearEmulatedInputQuery* query,
		ClearEmulatedInputResult* result
	);
};

extern const DebugInputApi DEBUG_INPUT_API;

#ifdef __cplusplus
}
#endif

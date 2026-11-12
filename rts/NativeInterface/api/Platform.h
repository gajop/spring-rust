/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

struct GetArchitectureQuery { uint8_t _unused; };
struct GetArchitectureResult { const Error* error; const char* architecture; };

struct IsHeadlessQuery { uint8_t _unused; };
struct IsHeadlessResult { const Error* error; bool isHeadless; };

// NEW (Lua's os.date): the local wall-clock date and time, for names and
// stamps a player reads, such as an output file's. Not for synced code
// (UNSYNCED_ONLY in WasmAnnotations.def): it differs between machines. Month
// 1-12, day 1-31, hour 0-23; second 0-60 (a leap second).
struct GetLocalTimeQuery { uint8_t _unused; };
struct GetLocalTimeResult { const Error* error; int32_t year; int32_t month; int32_t day; int32_t hour; int32_t minute; int32_t second; };

struct PlatformApi {
	void (*GetArchitecture)(const GetArchitectureQuery* query, GetArchitectureResult* result);
	void (*IsHeadless)(const IsHeadlessQuery* query, IsHeadlessResult* result);
	// NEW (no Spring Lua call; Lua uses os.date) — see GetLocalTimeQuery above.
	void (*GetLocalTime)(const GetLocalTimeQuery* query, GetLocalTimeResult* result);
};

extern const PlatformApi PLATFORM_API;

#ifdef __cplusplus
}
#endif

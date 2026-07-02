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

struct PlatformApi {
	void (*GetArchitecture)(const GetArchitectureQuery* query, GetArchitectureResult* result);
	void (*IsHeadless)(const IsHeadlessQuery* query, IsHeadlessResult* result);
};

extern const PlatformApi PLATFORM_API;

#ifdef __cplusplus
}
#endif

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

struct AddWorldIconQuery { int32_t cmdID; Float3 pos; };
struct AddWorldIconResult { const Error* error; bool success; };

struct AddWorldTextQuery { const char* text; Float3 pos; };
struct AddWorldTextResult { const Error* error; bool success; };

struct AddWorldUnitQuery { int32_t unitDefID; Float3 pos; int32_t teamID; int32_t facing; };
struct AddWorldUnitResult { const Error* error; bool success; };

struct MarkerAddPointQuery { Float3 pos; const char* text; bool localOnly; int32_t playerID; };
struct MarkerAddPointResult { const Error* error; bool success; };

struct MarkerAddLineQuery { Float3 from; Float3 to; bool localOnly; int32_t playerID; };
struct MarkerAddLineResult { const Error* error; bool success; };

struct MarkerErasePositionQuery { Float3 pos; float unused; bool localOnly; int32_t playerID; bool alwaysErase; };
struct MarkerErasePositionResult { const Error* error; bool success; };

struct MarkersApi {
	void (*AddWorldIcon)(const AddWorldIconQuery* query, AddWorldIconResult* result);
	void (*AddWorldText)(const AddWorldTextQuery* query, AddWorldTextResult* result);
	void (*AddWorldUnit)(const AddWorldUnitQuery* query, AddWorldUnitResult* result);
	void (*MarkerAddPoint)(const MarkerAddPointQuery* query, MarkerAddPointResult* result);
	void (*MarkerAddLine)(const MarkerAddLineQuery* query, MarkerAddLineResult* result);
	void (*MarkerErasePosition)(const MarkerErasePositionQuery* query, MarkerErasePositionResult* result);
};

extern const MarkersApi MARKERS_API;

#ifdef __cplusplus
}
#endif

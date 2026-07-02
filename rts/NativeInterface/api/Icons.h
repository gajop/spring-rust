/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

struct IconDataEntry {
	const char* name;
	float atlasTexCoords[4]; // x1,y1,x2,y2
	float size;
	float distance;
	bool radiusAdjust;
};

struct AddUnitIconQuery {
	const char* iconName;
	const char* texFile;
	float size;
	float distance;
	bool radiusAdjust;
	float u0, v0, u1, v1;
};
struct AddUnitIconResult { const Error* error; bool success; };

struct FreeUnitIconQuery { const char* iconName; };
struct FreeUnitIconResult { const Error* error; bool success; };

struct GetIconDataQuery { const char* iconName; bool fullData; };
struct GetIconDataResult { const Error* error; IconDataEntry data; };

struct GetAllIconDataArrayQuery { bool fullData; };
struct GetAllIconDataArrayResult { const Error* error; IconDataEntry* entries; uint32_t count; };

struct UnitIconGetDrawQuery { int32_t unitID; };
struct UnitIconGetDrawResult { const Error* error; bool drawIcon; };

struct UnitIconSetDrawQuery { int32_t unitID; bool drawIcon; };
struct UnitIconSetDrawResult { const Error* error; bool success; };

struct IconsApi {
	void (*AddUnitIcon)(const AddUnitIconQuery* query, AddUnitIconResult* result);
	void (*FreeUnitIcon)(const FreeUnitIconQuery* query, FreeUnitIconResult* result);
	void (*GetIconData)(const GetIconDataQuery* query, GetIconDataResult* result);
	void (*GetAllIconDataArray)(const GetAllIconDataArrayQuery* query, GetAllIconDataArrayResult* result);
	void (*UnitIconGetDraw)(const UnitIconGetDrawQuery* query, UnitIconGetDrawResult* result);
	void (*UnitIconSetDraw)(const UnitIconSetDrawQuery* query, UnitIconSetDrawResult* result);
};

extern const IconsApi ICONS_API;

#ifdef __cplusplus
}
#endif

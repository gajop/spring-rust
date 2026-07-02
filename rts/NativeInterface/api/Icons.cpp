/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Icons.h"

#include <cstring>

#include "Rendering/IconHandler.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "System/float4.h"

namespace {

thread_local uint8_t scratchBuffer[4096];
thread_local size_t bufferPos = 0;

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Icon system not available"
};

static const Error INVALID_ICON_NAME_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Icon name is null"
};

static const Error BUFFER_OVERFLOW_ERROR = {
	.code = ERROR_BUFFER_OVERFLOW,
	.message = "Scratch buffer overflow"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

static bool CopyString(const std::string& src, const char** outPtr)
{
	const size_t len = src.size();
	if (bufferPos + len + 1 > sizeof(scratchBuffer))
		return false;

	char* dest = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	std::memcpy(dest, src.c_str(), len + 1);
	bufferPos += len + 1;
	*outPtr = dest;
	return true;
}

static void FillIconData(const icon::IconData& src, bool fullData, IconDataEntry* dst, const Error** errorOut)
{
	if (!CopyString(src.GetName(), &dst->name)) {
		*errorOut = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	const float4& tc = src.GetSrcTexCoords();
	dst->atlasTexCoords[0] = tc.x;
	dst->atlasTexCoords[1] = tc.y;
	dst->atlasTexCoords[2] = tc.z;
	dst->atlasTexCoords[3] = tc.w;

	if (fullData) {
		dst->size = src.GetSize();
		dst->distance = src.GetDistance();
		dst->radiusAdjust = src.GetRadiusAdjust();
	} else {
		dst->size = 0.0f;
		dst->distance = 0.0f;
		dst->radiusAdjust = false;
	}
}

static void NativeAddUnitIcon(const AddUnitIconQuery* query, AddUnitIconResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (query->iconName == nullptr || query->texFile == nullptr) {
		result->error = &INVALID_ICON_NAME_ERROR;
		return;
	}

	result->success = icon::iconHandler.AddIcon(
		1,
		query->iconName,
		query->texFile,
		query->size,
		query->distance,
		query->radiusAdjust,
		query->u0,
		query->v0,
		query->u1,
		query->v1
	);
}

static void NativeFreeUnitIcon(const FreeUnitIconQuery* query, FreeUnitIconResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (query->iconName == nullptr) {
		result->error = &INVALID_ICON_NAME_ERROR;
		return;
	}

	result->success = icon::iconHandler.FreeIcon(query->iconName);
}

static void NativeGetIconData(const GetIconDataQuery* query, GetIconDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->data = {};

	if (query->iconName == nullptr) {
		result->error = &INVALID_ICON_NAME_ERROR;
		return;
	}

	const auto found = icon::iconHandler.FindIconIdx(query->iconName);
	if (!found.first) {
		result->error = &INVALID_ICON_NAME_ERROR;
		return;
	}

	const icon::IconData& data = icon::iconHandler.GetIconData(found.second->second);
	FillIconData(data, query->fullData, &result->data, &result->error);
}

static void NativeGetAllIconDataArray(const GetAllIconDataArrayQuery* query, GetAllIconDataArrayResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->entries = nullptr;
	result->count = 0;

	const auto& icons = icon::iconHandler.GetIconsData();
	const size_t count = icons.size();
	if (count == 0)
		return;

	const size_t bytesNeeded = count * sizeof(IconDataEntry);
	if (bytesNeeded > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	IconDataEntry* entries = reinterpret_cast<IconDataEntry*>(scratchBuffer + bufferPos);
	size_t base = bufferPos;
	bufferPos += bytesNeeded;

	for (size_t i = 0; i < count; ++i) {
		FillIconData(icons[i], query->fullData, &entries[i], &result->error);
		if (result->error != nullptr) {
			return;
		}
	}

	result->entries = entries;
	result->count = static_cast<uint32_t>(count);
}

static void NativeUnitIconGetDraw(const UnitIconGetDrawQuery* query, UnitIconGetDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->drawIcon = false;

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->drawIcon = unit->drawIcon;
}

static void NativeUnitIconSetDraw(const UnitIconSetDrawQuery* query, UnitIconSetDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	unit->drawIcon = query->drawIcon;
	result->success = true;
}

} // namespace

const IconsApi ICONS_API = {
	.AddUnitIcon = NativeAddUnitIcon,
	.FreeUnitIcon = NativeFreeUnitIcon,
	.GetIconData = NativeGetIconData,
	.GetAllIconDataArray = NativeGetAllIconDataArray,
	.UnitIconGetDraw = NativeUnitIconGetDraw,
	.UnitIconSetDraw = NativeUnitIconSetDraw,
};

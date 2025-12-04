#include "UnsyncedRead.h"
#include <cstring>

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Rendering/Models/3DModel.h"
#include "Rendering/Icon/IconHandler.h"
#include "System/Matrix44f.h"
#include "System/float3.h"

namespace {

// Thread-local scratch buffer for dynamic data
thread_local uint8_t scratchBuffer[4096];
thread_local size_t bufferPos = 0;

// Error messages
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Game not ready"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

static bool IsReady() {
	return (gs != nullptr);
}

// ============================================================================
// Unit Rendering State Implementation
// ============================================================================

static void NativeGetUnitNoDraw(const GetUnitNoDrawQuery* query, GetUnitNoDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noDraw = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noDraw = unit->noDraw;
}

static void NativeGetUnitLuaDraw(const GetUnitLuaDrawQuery* query, GetUnitLuaDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->luaDraw = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->luaDraw = unit->luaDraw;
}

static void NativeGetUnitEngineDrawMask(const GetUnitEngineDrawMaskQuery* query, GetUnitEngineDrawMaskResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->engineDrawMask = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->engineDrawMask = unit->engineDrawMask;
}

static void NativeGetUnitAlwaysUpdateMatrix(const GetUnitAlwaysUpdateMatrixQuery* query, GetUnitAlwaysUpdateMatrixResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->alwaysUpdateMatrix = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->alwaysUpdateMatrix = unit->alwaysUpdateMat;
}

static void NativeGetUnitDrawFlag(const GetUnitDrawFlagQuery* query, GetUnitDrawFlagResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->drawFlag = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->drawFlag = unit->drawFlag;
}

static void NativeGetUnitNoSelect(const GetUnitNoSelectQuery* query, GetUnitNoSelectResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noSelect = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noSelect = unit->noSelect;
}

static void NativeGetUnitNoMinimap(const GetUnitNoMinimapQuery* query, GetUnitNoMinimapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noMinimap = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noMinimap = unit->noMinimap;
}

static void NativeGetUnitNoGroup(const GetUnitNoGroupQuery* query, GetUnitNoGroupResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noGroup = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noGroup = unit->noGroup;
}

static void NativeGetUnitViewPosition(const GetUnitViewPositionQuery* query, GetUnitViewPositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->position = {0.0f, 0.0f, 0.0f};

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Get unit position (with or without midPos adjustment)
	const float3 unitPos = query->useMidPos ? unit->GetObjDrawMidPos() : unit->drawPos;

	// Note: In Lua, this function also adds error vector for ally team visibility
	// For Native API, we return the actual position without error vector
	// If error vector is needed, it should be handled on the client side
	result->position.x = unitPos.x;
	result->position.y = unitPos.y;
	result->position.z = unitPos.z;
}

static void NativeGetUnitTransformMatrix(const GetUnitTransformMatrixQuery* query, GetUnitTransformMatrixResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	memset(result->matrix, 0, sizeof(result->matrix));

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Get the transform matrix
	const CMatrix44f& mat = unit->GetTransformMatrix(false);

	// Copy matrix data (CMatrix44f stores in column-major order)
	for (int i = 0; i < 16; ++i) {
		result->matrix[i] = mat[i];
	}
}

static void NativeGetUnitSelectionVolumeData(const GetUnitSelectionVolumeDataQuery* query, GetUnitSelectionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->scales = {0.0f, 0.0f, 0.0f};
	result->offsets = {0.0f, 0.0f, 0.0f};
	result->volumeType = 0;
	result->useContHitTest = false;
	result->primaryAxis = 0;
	result->ignoreHits = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CollisionVolume& vol = unit->selectionVolume;

	result->scales.x = vol.GetScales().x;
	result->scales.y = vol.GetScales().y;
	result->scales.z = vol.GetScales().z;

	result->offsets.x = vol.GetOffsets().x;
	result->offsets.y = vol.GetOffsets().y;
	result->offsets.z = vol.GetOffsets().z;

	result->volumeType = vol.GetVolumeType();
	result->useContHitTest = vol.UseContHitTest();
	result->primaryAxis = vol.GetPrimaryAxis();
	result->ignoreHits = vol.IgnoreHits();
}

static void NativeGetUnitIconData(const GetUnitIconDataQuery* query, GetUnitIconDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->iconName = "";
	result->atlasTexCoords[0] = 0.0f;
	result->atlasTexCoords[1] = 0.0f;
	result->atlasTexCoords[2] = 0.0f;
	result->atlasTexCoords[3] = 0.0f;
	result->size = 0.0f;
	result->distance = 0.0f;
	result->radiusAdjust = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const auto iconIdx = unit->currentIconIndex;

	if (iconIdx < 0 || iconIdx >= icon::iconHandler.GetIconsData().size()) {
		// Invalid icon index, return empty data
		return;
	}

	const auto& iconData = icon::iconHandler.GetIconsData()[iconIdx];

	// Allocate space for icon name in scratch buffer
	const size_t nameLen = iconData.name.length();
	if (bufferPos + nameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &(Error{ERROR_BUFFER_OVERFLOW, "Scratch buffer overflow"});
		return;
	}

	char* nameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(nameBuf, iconData.name.c_str(), nameLen + 1);
	bufferPos += nameLen + 1;

	result->iconName = nameBuf;
	result->atlasTexCoords[0] = iconData.atlasTexCoords.x1;
	result->atlasTexCoords[1] = iconData.atlasTexCoords.y1;
	result->atlasTexCoords[2] = iconData.atlasTexCoords.x2;
	result->atlasTexCoords[3] = iconData.atlasTexCoords.y2;

	if (query->fullData) {
		result->size = iconData.size;
		result->distance = iconData.distance;
		result->radiusAdjust = iconData.radiusAdjust;
	}
}

static const UnitRenderingApi UNIT_RENDERING_API = {
	.GetUnitNoDraw = NativeGetUnitNoDraw,
	.GetUnitLuaDraw = NativeGetUnitLuaDraw,
	.GetUnitEngineDrawMask = NativeGetUnitEngineDrawMask,
	.GetUnitAlwaysUpdateMatrix = NativeGetUnitAlwaysUpdateMatrix,
	.GetUnitDrawFlag = NativeGetUnitDrawFlag,
	.GetUnitNoSelect = NativeGetUnitNoSelect,
	.GetUnitNoMinimap = NativeGetUnitNoMinimap,
	.GetUnitNoGroup = NativeGetUnitNoGroup,
	.GetUnitViewPosition = NativeGetUnitViewPosition,
	.GetUnitTransformMatrix = NativeGetUnitTransformMatrix,
	.GetUnitSelectionVolumeData = NativeGetUnitSelectionVolumeData,
	.GetUnitIconData = NativeGetUnitIconData
};

} // namespace

// ============================================================================
// Public API Export
// ============================================================================

const UnsyncedReadApi UNSYNCED_READ_API = {
	.unitRendering = &UNIT_RENDERING_API
};

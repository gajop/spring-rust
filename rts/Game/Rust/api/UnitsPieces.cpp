#include "UnitsPieces.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Rendering/Models/3DModel.h"
#include <vector>
#include <string>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Function not yet fully implemented" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit or feature ID" };

static void NativeGetModelRootPiece(const GetModelRootPieceQuery* query, GetModelRootPieceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->rootPiece = 0; // Root piece
}

static void NativeGetUnitRootPiece(const GetUnitRootPieceQuery* query, GetUnitRootPieceResult* result) {
	bufferPos = 0;
	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}
	result->error = nullptr;
	result->rootPiece = 0; // Root piece
}

static void NativeGetFeatureRootPiece(const GetFeatureRootPieceQuery* query, GetFeatureRootPieceResult* result) {
	bufferPos = 0;
	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}
	result->error = nullptr;
	result->rootPiece = 0; // Root piece
}

static void NativeGetModelPieceList(const GetModelPieceListQuery* query, GetModelPieceListResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->pieces = nullptr;
	result->count = 0;
}

static void NativeGetModelPieceMap(const GetModelPieceMapQuery* query, GetModelPieceMapResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->names = nullptr;
	result->count = 0;
}

static void NativeGetUnitPieceList(const GetUnitPieceListQuery* query, GetUnitPieceListResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->pieces = nullptr;
	result->count = 0;
}

static void NativeGetUnitPieceMap(const GetUnitPieceMapQuery* query, GetUnitPieceMapResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->names = nullptr;
	result->count = 0;
}

static void NativeGetFeaturePieceList(const GetFeaturePieceListQuery* query, GetFeaturePieceListResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->pieces = nullptr;
	result->count = 0;
}

static void NativeGetFeaturePieceMap(const GetFeaturePieceMapQuery* query, GetFeaturePieceMapResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->names = nullptr;
	result->count = 0;
}

static void NativeGetUnitPieceInfo(const GetUnitPieceInfoQuery* query, GetUnitPieceInfoResult* result) {
	bufferPos = 0;
	result->exists = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetFeaturePieceInfo(const GetFeaturePieceInfoQuery* query, GetFeaturePieceInfoResult* result) {
	bufferPos = 0;
	result->exists = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitPiecePosition(const GetUnitPiecePositionQuery* query, GetUnitPiecePositionResult* result) {
	bufferPos = 0;
	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->error = nullptr;
	result->position.x = unit->pos.x;
	result->position.y = unit->pos.y;
	result->position.z = unit->pos.z;
}

static void NativeGetUnitPieceDirection(const GetUnitPieceDirectionQuery* query, GetUnitPieceDirectionResult* result) {
	bufferPos = 0;
	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->error = nullptr;
	result->direction.x = unit->frontdir.x;
	result->direction.y = unit->frontdir.y;
	result->direction.z = unit->frontdir.z;
}

static void NativeGetUnitPiecePosDir(const GetUnitPiecePosDirQuery* query, GetUnitPiecePosDirResult* result) {
	bufferPos = 0;
	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->error = nullptr;
	result->posDir.position.x = unit->pos.x;
	result->posDir.position.y = unit->pos.y;
	result->posDir.position.z = unit->pos.z;
	result->posDir.direction.x = unit->frontdir.x;
	result->posDir.direction.y = unit->frontdir.y;
	result->posDir.direction.z = unit->frontdir.z;
}

static void NativeGetFeaturePiecePosition(const GetFeaturePiecePositionQuery* query, GetFeaturePiecePositionResult* result) {
	bufferPos = 0;
	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->error = nullptr;
	result->position.x = feature->pos.x;
	result->position.y = feature->pos.y;
	result->position.z = feature->pos.z;
}

static void NativeGetFeaturePieceDirection(const GetFeaturePieceDirectionQuery* query, GetFeaturePieceDirectionResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetFeaturePiecePosDir(const GetFeaturePiecePosDirQuery* query, GetFeaturePiecePosDirResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitPieceMatrix(const GetUnitPieceMatrixQuery* query, GetUnitPieceMatrixResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetFeaturePieceMatrix(const GetFeaturePieceMatrixQuery* query, GetFeaturePieceMatrixResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitScriptPiece(const GetUnitScriptPieceQuery* query, GetUnitScriptPieceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->pieceNum = query->scriptNum; // Simplified: assume 1:1 mapping
}

static void NativeGetUnitScriptNames(const GetUnitScriptNamesQuery* query, GetUnitScriptNamesResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->names = nullptr;
	result->count = 0;
}

} // namespace

const UnitsPiecesApi UNITS_PIECES_API = {
	.GetModelRootPiece = NativeGetModelRootPiece,
	.GetUnitRootPiece = NativeGetUnitRootPiece,
	.GetFeatureRootPiece = NativeGetFeatureRootPiece,
	.GetModelPieceList = NativeGetModelPieceList,
	.GetModelPieceMap = NativeGetModelPieceMap,
	.GetUnitPieceList = NativeGetUnitPieceList,
	.GetUnitPieceMap = NativeGetUnitPieceMap,
	.GetFeaturePieceList = NativeGetFeaturePieceList,
	.GetFeaturePieceMap = NativeGetFeaturePieceMap,
	.GetUnitPieceInfo = NativeGetUnitPieceInfo,
	.GetFeaturePieceInfo = NativeGetFeaturePieceInfo,
	.GetUnitPiecePosition = NativeGetUnitPiecePosition,
	.GetUnitPieceDirection = NativeGetUnitPieceDirection,
	.GetUnitPiecePosDir = NativeGetUnitPiecePosDir,
	.GetFeaturePiecePosition = NativeGetFeaturePiecePosition,
	.GetFeaturePieceDirection = NativeGetFeaturePieceDirection,
	.GetFeaturePiecePosDir = NativeGetFeaturePiecePosDir,
	.GetUnitPieceMatrix = NativeGetUnitPieceMatrix,
	.GetFeaturePieceMatrix = NativeGetFeaturePieceMatrix,
	.GetUnitScriptPiece = NativeGetUnitScriptPiece,
	.GetUnitScriptNames = NativeGetUnitScriptNames,
};

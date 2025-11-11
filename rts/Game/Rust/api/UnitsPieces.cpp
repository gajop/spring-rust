#include "UnitsPieces.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Rendering/Models/3DModel.h"
#include <vector>
#include <string>

namespace {

// Error constants
static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Function not yet fully implemented"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit or feature ID"
};

// Model root piece (simplified - always return 0 as root)
static Int32Result NativeGetModelRootPiece(const char* modelName, int32_t modelType)
{
	Int32Result result = {};
	result.value = 0; // Root piece
	return result;
}

static Int32Result NativeGetUnitRootPiece(int32_t unitID)
{
	Int32Result result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}
	result.value = 0; // Root piece
	return result;
}

static Int32Result NativeGetFeatureRootPiece(int32_t featureID)
{
	Int32Result result = {};
	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}
	result.value = 0; // Root piece
	return result;
}

// Piece lists and maps (simplified - return empty)
static Int32Array NativeGetModelPieceList(const char* modelName, int32_t modelType)
{
	Int32Array result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static StringArray NativeGetModelPieceMap(const char* modelName, int32_t modelType)
{
	StringArray result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static Int32Array NativeGetUnitPieceList(int32_t unitID)
{
	Int32Array result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static StringArray NativeGetUnitPieceMap(int32_t unitID)
{
	StringArray result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static Int32Array NativeGetFeaturePieceList(int32_t featureID)
{
	Int32Array result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static StringArray NativeGetFeaturePieceMap(int32_t featureID)
{
	StringArray result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Piece info (simplified)
static PieceInfoResult NativeGetUnitPieceInfo(int32_t unitID, int32_t pieceNum)
{
	PieceInfoResult result = {};
	result.exists = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static PieceInfoResult NativeGetFeaturePieceInfo(int32_t featureID, int32_t pieceNum)
{
	PieceInfoResult result = {};
	result.exists = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Piece position and direction (simplified - return unit/feature position)
static Float3Result NativeGetUnitPiecePosition(int32_t unitID, int32_t pieceNum)
{
	Float3Result result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.value.x = unit->pos.x;
	result.value.y = unit->pos.y;
	result.value.z = unit->pos.z;
	return result;
}

static Float3Result NativeGetUnitPieceDirection(int32_t unitID, int32_t pieceNum)
{
	Float3Result result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.value.x = unit->frontdir.x;
	result.value.y = unit->frontdir.y;
	result.value.z = unit->frontdir.z;
	return result;
}

static PiecePosDirResult NativeGetUnitPiecePosDir(int32_t unitID, int32_t pieceNum)
{
	PiecePosDirResult result = {};
	const CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.posDir.position.x = unit->pos.x;
	result.posDir.position.y = unit->pos.y;
	result.posDir.position.z = unit->pos.z;
	result.posDir.direction.x = unit->frontdir.x;
	result.posDir.direction.y = unit->frontdir.y;
	result.posDir.direction.z = unit->frontdir.z;
	return result;
}

static Float3Result NativeGetFeaturePiecePosition(int32_t featureID, int32_t pieceNum)
{
	Float3Result result = {};
	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_UNIT_ERROR;
		return result;
	}

	result.value.x = feature->pos.x;
	result.value.y = feature->pos.y;
	result.value.z = feature->pos.z;
	return result;
}

static Float3Result NativeGetFeaturePieceDirection(int32_t featureID, int32_t pieceNum)
{
	Float3Result result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static PiecePosDirResult NativeGetFeaturePiecePosDir(int32_t featureID, int32_t pieceNum)
{
	PiecePosDirResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Piece matrix (simplified - return identity matrix)
static PieceMatrixResult NativeGetUnitPieceMatrix(int32_t unitID, int32_t pieceNum)
{
	PieceMatrixResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

static PieceMatrixResult NativeGetFeaturePieceMatrix(int32_t featureID, int32_t pieceNum)
{
	PieceMatrixResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Script piece mapping (simplified)
static Int32Result NativeGetUnitScriptPiece(int32_t unitID, int32_t scriptNum)
{
	Int32Result result = {};
	result.value = scriptNum; // Simplified: assume 1:1 mapping
	return result;
}

static StringArray NativeGetUnitScriptNames(int32_t unitID)
{
	StringArray result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
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

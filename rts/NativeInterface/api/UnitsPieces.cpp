#include "UnitsPieces.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Rendering/Models/3DModel.h"
#include "System/Matrix44f.h"
#include <vector>
#include <string>
#include <cstring>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game not ready" };
static const Error INVALID_UNIT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid unit or feature ID" };
static const Error INVALID_PIECE_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid piece number" };
static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };

static bool IsReady() {
	return (gs != nullptr);
}

// Helper to allocate from scratch buffer
template<typename T>
static T* AllocateArray(size_t count) {
	size_t needed = count * sizeof(T);
	if (bufferPos + needed > sizeof(scratchBuffer)) {
		return nullptr;
	}
	T* ptr = reinterpret_cast<T*>(&scratchBuffer[bufferPos]);
	bufferPos += needed;
	return ptr;
}

// Helper to copy string to scratch buffer
static const char* CopyString(const std::string& str) {
	size_t len = str.length() + 1;
	if (bufferPos + len > sizeof(scratchBuffer)) {
		return nullptr;
	}
	char* ptr = &scratchBuffer[bufferPos];
	memcpy(ptr, str.c_str(), len);
	bufferPos += len;
	return ptr;
}

static void NativeGetModelRootPiece(const GetModelRootPieceQuery* query, GetModelRootPieceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->rootPiece = 0; // Root piece is always index 0
}

static void NativeGetUnitRootPiece(const GetUnitRootPieceQuery* query, GetUnitRootPieceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->rootPiece = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->rootPiece = 0; // Root piece is always index 0
}

static void NativeGetFeatureRootPiece(const GetFeatureRootPieceQuery* query, GetFeatureRootPieceResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->rootPiece = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->rootPiece = 0; // Root piece is always index 0
}

static void NativeGetModelPieceList(const GetModelPieceListQuery* query, GetModelPieceListResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->pieces = nullptr;
	result->count = 0;

	// Model piece lists are not easily accessible without a specific model instance
	// This would require model loading which is complex
}

static void NativeGetModelPieceMap(const GetModelPieceMapQuery* query, GetModelPieceMapResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	// Model piece maps are not easily accessible without a specific model instance
}

static void NativeGetUnitPieceList(const GetUnitPieceListQuery* query, GetUnitPieceListResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->pieces = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.Initialized()) {
		return;
	}

	uint32_t count = static_cast<uint32_t>(localModel.pieces.size());
	if (count == 0) {
		return;
	}

	result->pieces = AllocateArray<int32_t>(count);
	if (result->pieces == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		result->pieces[i] = static_cast<int32_t>(i);
	}

	result->count = count;
}

static void NativeGetUnitPieceMap(const GetUnitPieceMapQuery* query, GetUnitPieceMapResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.Initialized()) {
		return;
	}

	uint32_t count = static_cast<uint32_t>(localModel.pieces.size());
	if (count == 0) {
		return;
	}

	result->names = AllocateArray<const char*>(count);
	if (result->names == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		const LocalModelPiece& piece = localModel.pieces[i];
		if (piece.original != nullptr) {
			result->names[i] = CopyString(piece.original->name);
			if (result->names[i] == nullptr) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				result->count = i;
				return;
			}
		} else {
			result->names[i] = CopyString("");
		}
	}

	result->count = count;
}

static void NativeGetFeaturePieceList(const GetFeaturePieceListQuery* query, GetFeaturePieceListResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->pieces = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = feature->localModel;
	if (!localModel.Initialized()) {
		return;
	}

	uint32_t count = static_cast<uint32_t>(localModel.pieces.size());
	if (count == 0) {
		return;
	}

	result->pieces = AllocateArray<int32_t>(count);
	if (result->pieces == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		result->pieces[i] = static_cast<int32_t>(i);
	}

	result->count = count;
}

static void NativeGetFeaturePieceMap(const GetFeaturePieceMapQuery* query, GetFeaturePieceMapResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = feature->localModel;
	if (!localModel.Initialized()) {
		return;
	}

	uint32_t count = static_cast<uint32_t>(localModel.pieces.size());
	if (count == 0) {
		return;
	}

	result->names = AllocateArray<const char*>(count);
	if (result->names == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		const LocalModelPiece& piece = localModel.pieces[i];
		if (piece.original != nullptr) {
			result->names[i] = CopyString(piece.original->name);
			if (result->names[i] == nullptr) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				result->count = i;
				return;
			}
		} else {
			result->names[i] = CopyString("");
		}
	}

	result->count = count;
}

static void NativeGetUnitPieceInfo(const GetUnitPieceInfoQuery* query, GetUnitPieceInfoResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->exists = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	if (piece == nullptr || piece->original == nullptr) {
		return;
	}

	result->info.name = CopyString(piece->original->name);
	if (result->info.name == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	result->info.pieceNum = query->pieceNum;
	result->info.offset.x = piece->original->offset.x;
	result->info.offset.y = piece->original->offset.y;
	result->info.offset.z = piece->original->offset.z;

	float3 emitDir = piece->original->GetEmitDir();
	result->info.emitDir.x = emitDir.x;
	result->info.emitDir.y = emitDir.y;
	result->info.emitDir.z = emitDir.z;

	result->exists = true;
}

static void NativeGetFeaturePieceInfo(const GetFeaturePieceInfoQuery* query, GetFeaturePieceInfoResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->exists = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = feature->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	if (piece == nullptr || piece->original == nullptr) {
		return;
	}

	result->info.name = CopyString(piece->original->name);
	if (result->info.name == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	result->info.pieceNum = query->pieceNum;
	result->info.offset.x = piece->original->offset.x;
	result->info.offset.y = piece->original->offset.y;
	result->info.offset.z = piece->original->offset.z;

	float3 emitDir = piece->original->GetEmitDir();
	result->info.emitDir.x = emitDir.x;
	result->info.emitDir.y = emitDir.y;
	result->info.emitDir.z = emitDir.z;

	result->exists = true;
}

static void NativeGetUnitPiecePosition(const GetUnitPiecePositionQuery* query, GetUnitPiecePositionResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	float3 pos = piece->GetAbsolutePos() + unit->pos;

	result->position.x = pos.x;
	result->position.y = pos.y;
	result->position.z = pos.z;
}

static void NativeGetUnitPieceDirection(const GetUnitPieceDirectionQuery* query, GetUnitPieceDirectionResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	const float3& dir = piece->GetDirection();

	result->direction.x = dir.x;
	result->direction.y = dir.y;
	result->direction.z = dir.z;
}

static void NativeGetUnitPiecePosDir(const GetUnitPiecePosDirQuery* query, GetUnitPiecePosDirResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	float3 pos = piece->GetAbsolutePos() + unit->pos;
	const float3& dir = piece->GetDirection();

	result->posDir.position.x = pos.x;
	result->posDir.position.y = pos.y;
	result->posDir.position.z = pos.z;
	result->posDir.direction.x = dir.x;
	result->posDir.direction.y = dir.y;
	result->posDir.direction.z = dir.z;
}

static void NativeGetFeaturePiecePosition(const GetFeaturePiecePositionQuery* query, GetFeaturePiecePositionResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = feature->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	float3 pos = piece->GetAbsolutePos() + feature->pos;

	result->position.x = pos.x;
	result->position.y = pos.y;
	result->position.z = pos.z;
}

static void NativeGetFeaturePieceDirection(const GetFeaturePieceDirectionQuery* query, GetFeaturePieceDirectionResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = feature->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	const float3& dir = piece->GetDirection();

	result->direction.x = dir.x;
	result->direction.y = dir.y;
	result->direction.z = dir.z;
}

static void NativeGetFeaturePiecePosDir(const GetFeaturePiecePosDirQuery* query, GetFeaturePiecePosDirResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = feature->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	float3 pos = piece->GetAbsolutePos() + feature->pos;
	const float3& dir = piece->GetDirection();

	result->posDir.position.x = pos.x;
	result->posDir.position.y = pos.y;
	result->posDir.position.z = pos.z;
	result->posDir.direction.x = dir.x;
	result->posDir.direction.y = dir.y;
	result->posDir.direction.z = dir.z;
}

static void NativeGetUnitPieceMatrix(const GetUnitPieceMatrixQuery* query, GetUnitPieceMatrixResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	const CMatrix44f& mat = piece->GetModelSpaceMatrix();

	// Copy matrix in column-major order
	for (int i = 0; i < 16; ++i) {
		result->matrix.m[i] = mat.m[i];
	}
}

static void NativeGetFeaturePieceMatrix(const GetFeaturePieceMatrixQuery* query, GetFeaturePieceMatrixResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = feature->localModel;
	if (!localModel.HasPiece(query->pieceNum)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum);
	const CMatrix44f& mat = piece->GetModelSpaceMatrix();

	// Copy matrix in column-major order
	for (int i = 0; i < 16; ++i) {
		result->matrix.m[i] = mat.m[i];
	}
}

static void NativeGetUnitScriptPiece(const GetUnitScriptPieceQuery* query, GetUnitScriptPieceResult* result) {
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Script piece numbers map to local model piece numbers
	// This is a simplified mapping - actual mapping may be more complex
	result->pieceNum = query->scriptNum;
}

static void NativeGetUnitScriptNames(const GetUnitScriptNamesQuery* query, GetUnitScriptNamesResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const LocalModel& localModel = unit->localModel;
	if (!localModel.Initialized()) {
		return;
	}

	uint32_t count = static_cast<uint32_t>(localModel.pieces.size());
	if (count == 0) {
		return;
	}

	result->names = AllocateArray<const char*>(count);
	if (result->names == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		const LocalModelPiece& piece = localModel.pieces[i];
		if (piece.original != nullptr) {
			result->names[i] = CopyString(piece.original->name);
			if (result->names[i] == nullptr) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				result->count = i;
				return;
			}
		} else {
			result->names[i] = CopyString("");
		}
	}

	result->count = count;
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

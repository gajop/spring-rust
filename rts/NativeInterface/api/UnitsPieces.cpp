#include "UnitsPieces.h"

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Units/Scripts/UnitScript.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Rendering/Models/IModelParser.h"
#include "Rendering/Models/3DModel.hpp"
#include "Rendering/Models/3DModelPiece.hpp"
#include "System/Matrix44f.h"
#include <vector>
#include <string>
#include <cstring>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
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
	result->rootPiece = 0;

	const auto* model = modelLoader.LoadModel(query->modelName);
	if (model != nullptr)
		result->rootPiece = static_cast<int32_t>(model->GetRootPieceIndex() + 1);
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

	if (unit->localModel.Initialized())
		result->rootPiece = static_cast<int32_t>(unit->localModel.GetRoot()->GetLModelPieceIndex() + 1);
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

	if (feature->localModel.Initialized())
		result->rootPiece = static_cast<int32_t>(feature->localModel.GetRoot()->GetLModelPieceIndex() + 1);
}

static void NativeGetModelPieceList(const GetModelPieceListQuery* query, GetModelPieceListResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	const auto* model = modelLoader.LoadModel(query->modelName);
	if (model == nullptr)
		return;

	const uint32_t count = static_cast<uint32_t>(model->pieceObjects.size());
	if (count == 0)
		return;

	result->names = AllocateArray<const char*>(count);
	if (result->names == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		result->names[i] = CopyString(model->pieceObjects[i]->name);
		if (result->names[i] == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = i;
			return;
		}
	}

	result->count = count;
}

static void NativeGetModelPieceMap(const GetModelPieceMapQuery* query, GetModelPieceMapResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->entries = nullptr;
	result->count = 0;

	if (query->modelName == nullptr || query->modelName[0] == '\0')
		return;

	const auto* model = modelLoader.LoadModel(query->modelName);
	if (model == nullptr)
		return;

	const uint32_t count = static_cast<uint32_t>(model->pieceObjects.size());
	if (count == 0)
		return;

	result->entries = AllocateArray<PieceMapEntry>(count);
	if (result->entries == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		result->entries[i].name = CopyString(model->pieceObjects[i]->name);
		result->entries[i].pieceNum = static_cast<int32_t>(i + 1);
		if (result->entries[i].name == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = i;
			return;
		}
	}

	result->count = count;
}

static void NativeGetUnitPieceList(const GetUnitPieceListQuery* query, GetUnitPieceListResult* result) {
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
	if (!localModel.Initialized())
		return;

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
		const std::string name = (piece.original != nullptr) ? piece.original->name : "";
		result->names[i] = CopyString(name);
		if (result->names[i] == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = i;
			return;
		}
	}

	result->count = count;
}

static void NativeGetUnitPieceMap(const GetUnitPieceMapQuery* query, GetUnitPieceMapResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->entries = nullptr;
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

	result->entries = AllocateArray<PieceMapEntry>(count);
	if (result->entries == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		const LocalModelPiece& piece = localModel.pieces[i];
		const std::string name = (piece.original != nullptr) ? piece.original->name : "";
		result->entries[i].name = CopyString(name);
		result->entries[i].pieceNum = static_cast<int32_t>(i + 1);
		if (result->entries[i].name == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = i;
			return;
		}
	}

	result->count = count;
}

static void NativeGetFeaturePieceList(const GetFeaturePieceListQuery* query, GetFeaturePieceListResult* result) {
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
		const std::string name = (piece.original != nullptr) ? piece.original->name : "";
		result->names[i] = CopyString(name);
		if (result->names[i] == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = i;
			return;
		}
	}

	result->count = count;
}

static void NativeGetFeaturePieceMap(const GetFeaturePieceMapQuery* query, GetFeaturePieceMapResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->entries = nullptr;
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

	result->entries = AllocateArray<PieceMapEntry>(count);
	if (result->entries == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		const LocalModelPiece& piece = localModel.pieces[i];
		const std::string name = (piece.original != nullptr) ? piece.original->name : "";
		result->entries[i].name = CopyString(name);
		result->entries[i].pieceNum = static_cast<int32_t>(i + 1);
		if (result->entries[i].name == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = i;
			return;
		}
	}

	result->count = count;
}

static bool FillPieceInfo(const S3DModelPiece* piece, int32_t pieceNum, PieceInfo& info)
{
	if (piece == nullptr)
		return false;

	info.name = CopyString(piece->name);
	info.parent = CopyString(piece->parent != nullptr ? piece->parent->name : "[null]");
	info.childCount = static_cast<uint32_t>(piece->children.size());
	info.children = nullptr;
	if (info.childCount != 0) {
		info.children = AllocateArray<const char*>(info.childCount);
		if (info.children == nullptr)
			return false;
		for (uint32_t i = 0; i < info.childCount; ++i) {
			info.children[i] = CopyString(piece->children[i]->name);
			if (info.children[i] == nullptr)
				return false;
		}
	}

	info.isEmpty = !piece->HasGeometryData();
	info.min.x = piece->mins.x;
	info.min.y = piece->mins.y;
	info.min.z = piece->mins.z;
	info.max.x = piece->maxs.x;
	info.max.y = piece->maxs.y;
	info.max.z = piece->maxs.z;
	info.pieceNum = pieceNum;
	info.offset.x = piece->offset.x;
	info.offset.y = piece->offset.y;
	info.offset.z = piece->offset.z;
	const float3 emitDir = piece->GetEmitDir();
	info.emitDir.x = emitDir.x;
	info.emitDir.y = emitDir.y;
	info.emitDir.z = emitDir.z;
	return info.name != nullptr && info.parent != nullptr;
}

static void NativeGetUnitPieceInfo(const GetUnitPieceInfoQuery* query, GetUnitPieceInfoResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->info = {};
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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
	if (piece == nullptr || piece->original == nullptr) {
		return;
	}

	if (!FillPieceInfo(piece->original, query->pieceNum, result->info)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	result->exists = true;
}

static void NativeGetFeaturePieceInfo(const GetFeaturePieceInfoQuery* query, GetFeaturePieceInfoResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->info = {};
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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
	if (piece == nullptr || piece->original == nullptr) {
		return;
	}

	if (!FillPieceInfo(piece->original, query->pieceNum, result->info)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
	float3 pos = piece->GetAbsolutePos();

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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
	float3 pos;
	float3 dir;
	piece->GetEmitDirPos(pos, dir);
	pos = unit->GetObjectSpacePos(pos);
	dir = unit->GetObjectSpaceVec(dir);

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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
	float3 pos = piece->GetAbsolutePos();

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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
	float3 pos;
	float3 dir;
	piece->GetEmitDirPos(pos, dir);
	pos = feature->GetObjectSpacePos(pos);
	dir = feature->GetObjectSpaceVec(dir);

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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
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
	if (query->pieceNum <= 0 || !localModel.HasPiece(query->pieceNum - 1)) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const LocalModelPiece* piece = localModel.GetPiece(query->pieceNum - 1);
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

	if (query->scriptNum < 0 || unit->script == nullptr) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	const int piece = unit->script->ScriptToModel(query->scriptNum);
	if (piece < 0) {
		result->error = &INVALID_PIECE_ERROR;
		return;
	}

	result->pieceNum = piece + 1;
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

	if (unit->script == nullptr) {
		return;
	}

	const std::vector<LocalModelPiece*>& pieces = unit->script->pieces;
	uint32_t count = static_cast<uint32_t>(pieces.size());
	if (count == 0) {
		return;
	}

	result->names = AllocateArray<const char*>(count);
	if (result->names == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (uint32_t i = 0; i < count; ++i) {
		const LocalModelPiece* piece = pieces[i];
		if (piece != nullptr && piece->original != nullptr) {
			result->names[i] = CopyString(piece->original->name);
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

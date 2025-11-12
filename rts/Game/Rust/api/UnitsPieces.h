#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Unit Model Pieces API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Unit and feature model piece manipulation (3D model hierarchy)
// ============================================================================

// Piece info
struct PieceInfo {
	const char* name;
	int32_t pieceNum;
	Float3 offset;
	Float3 emitDir;
};

// Piece position/direction
struct PiecePosDir {
	Float3 position;
	Float3 direction;
};

// Piece matrix (full transformation)
struct PieceMatrix {
	float m[16];  // 4x4 matrix in column-major order
};

// Queries
struct GetModelRootPieceQuery { const char* modelName; int32_t modelType; };
struct GetModelRootPieceResult { const Error* error; int32_t rootPiece; };

struct GetUnitRootPieceQuery { int32_t unitID; };
struct GetUnitRootPieceResult { const Error* error; int32_t rootPiece; };

struct GetFeatureRootPieceQuery { int32_t featureID; };
struct GetFeatureRootPieceResult { const Error* error; int32_t rootPiece; };

struct GetModelPieceListQuery { const char* modelName; int32_t modelType; };
struct GetModelPieceListResult { const Error* error; int32_t* pieces; uint32_t count; };

struct GetModelPieceMapQuery { const char* modelName; int32_t modelType; };
struct GetModelPieceMapResult { const Error* error; const char** names; uint32_t count; };

struct GetUnitPieceListQuery { int32_t unitID; };
struct GetUnitPieceListResult { const Error* error; int32_t* pieces; uint32_t count; };

struct GetUnitPieceMapQuery { int32_t unitID; };
struct GetUnitPieceMapResult { const Error* error; const char** names; uint32_t count; };

struct GetFeaturePieceListQuery { int32_t featureID; };
struct GetFeaturePieceListResult { const Error* error; int32_t* pieces; uint32_t count; };

struct GetFeaturePieceMapQuery { int32_t featureID; };
struct GetFeaturePieceMapResult { const Error* error; const char** names; uint32_t count; };

struct GetUnitPieceInfoQuery { int32_t unitID; int32_t pieceNum; };
struct GetUnitPieceInfoResult { const Error* error; PieceInfo info; bool exists; };

struct GetFeaturePieceInfoQuery { int32_t featureID; int32_t pieceNum; };
struct GetFeaturePieceInfoResult { const Error* error; PieceInfo info; bool exists; };

struct GetUnitPiecePositionQuery { int32_t unitID; int32_t pieceNum; };
struct GetUnitPiecePositionResult { const Error* error; Float3 position; };

struct GetUnitPieceDirectionQuery { int32_t unitID; int32_t pieceNum; };
struct GetUnitPieceDirectionResult { const Error* error; Float3 direction; };

struct GetUnitPiecePosDirQuery { int32_t unitID; int32_t pieceNum; };
struct GetUnitPiecePosDirResult { const Error* error; PiecePosDir posDir; };

struct GetFeaturePiecePositionQuery { int32_t featureID; int32_t pieceNum; };
struct GetFeaturePiecePositionResult { const Error* error; Float3 position; };

struct GetFeaturePieceDirectionQuery { int32_t featureID; int32_t pieceNum; };
struct GetFeaturePieceDirectionResult { const Error* error; Float3 direction; };

struct GetFeaturePiecePosDirQuery { int32_t featureID; int32_t pieceNum; };
struct GetFeaturePiecePosDirResult { const Error* error; PiecePosDir posDir; };

struct GetUnitPieceMatrixQuery { int32_t unitID; int32_t pieceNum; };
struct GetUnitPieceMatrixResult { const Error* error; PieceMatrix matrix; };

struct GetFeaturePieceMatrixQuery { int32_t featureID; int32_t pieceNum; };
struct GetFeaturePieceMatrixResult { const Error* error; PieceMatrix matrix; };

struct GetUnitScriptPieceQuery { int32_t unitID; int32_t scriptNum; };
struct GetUnitScriptPieceResult { const Error* error; int32_t pieceNum; };

struct GetUnitScriptNamesQuery { int32_t unitID; };
struct GetUnitScriptNamesResult { const Error* error; const char** names; uint32_t count; };

// API structure
struct UnitsPiecesApi {
	void (*GetModelRootPiece)(const GetModelRootPieceQuery* query, GetModelRootPieceResult* result);
	void (*GetUnitRootPiece)(const GetUnitRootPieceQuery* query, GetUnitRootPieceResult* result);
	void (*GetFeatureRootPiece)(const GetFeatureRootPieceQuery* query, GetFeatureRootPieceResult* result);
	void (*GetModelPieceList)(const GetModelPieceListQuery* query, GetModelPieceListResult* result);
	void (*GetModelPieceMap)(const GetModelPieceMapQuery* query, GetModelPieceMapResult* result);
	void (*GetUnitPieceList)(const GetUnitPieceListQuery* query, GetUnitPieceListResult* result);
	void (*GetUnitPieceMap)(const GetUnitPieceMapQuery* query, GetUnitPieceMapResult* result);
	void (*GetFeaturePieceList)(const GetFeaturePieceListQuery* query, GetFeaturePieceListResult* result);
	void (*GetFeaturePieceMap)(const GetFeaturePieceMapQuery* query, GetFeaturePieceMapResult* result);
	void (*GetUnitPieceInfo)(const GetUnitPieceInfoQuery* query, GetUnitPieceInfoResult* result);
	void (*GetFeaturePieceInfo)(const GetFeaturePieceInfoQuery* query, GetFeaturePieceInfoResult* result);
	void (*GetUnitPiecePosition)(const GetUnitPiecePositionQuery* query, GetUnitPiecePositionResult* result);
	void (*GetUnitPieceDirection)(const GetUnitPieceDirectionQuery* query, GetUnitPieceDirectionResult* result);
	void (*GetUnitPiecePosDir)(const GetUnitPiecePosDirQuery* query, GetUnitPiecePosDirResult* result);
	void (*GetFeaturePiecePosition)(const GetFeaturePiecePositionQuery* query, GetFeaturePiecePositionResult* result);
	void (*GetFeaturePieceDirection)(const GetFeaturePieceDirectionQuery* query, GetFeaturePieceDirectionResult* result);
	void (*GetFeaturePiecePosDir)(const GetFeaturePiecePosDirQuery* query, GetFeaturePiecePosDirResult* result);
	void (*GetUnitPieceMatrix)(const GetUnitPieceMatrixQuery* query, GetUnitPieceMatrixResult* result);
	void (*GetFeaturePieceMatrix)(const GetFeaturePieceMatrixQuery* query, GetFeaturePieceMatrixResult* result);
	void (*GetUnitScriptPiece)(const GetUnitScriptPieceQuery* query, GetUnitScriptPieceResult* result);
	void (*GetUnitScriptNames)(const GetUnitScriptNamesQuery* query, GetUnitScriptNamesResult* result);
};

extern const UnitsPiecesApi UNITS_PIECES_API;

#ifdef __cplusplus
}
#endif

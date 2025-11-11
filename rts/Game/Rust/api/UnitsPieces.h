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

struct PieceInfoResult {
	const Error* error;
	PieceInfo info;
	bool exists;
};

// Piece position/direction
struct PiecePosDir {
	Float3 position;
	Float3 direction;
};

struct PiecePosDirResult {
	const Error* error;
	PiecePosDir posDir;
};

// Piece matrix (full transformation)
struct PieceMatrix {
	float m[16];  // 4x4 matrix in column-major order
};

struct PieceMatrixResult {
	const Error* error;
	PieceMatrix matrix;
};

// Script piece query
struct ScriptPieceQuery {
	int32_t unitID;
	int32_t scriptNum;  // Script piece number (from COB/Lua scripts)
};

// API structure
struct UnitsPiecesApi {
	// Model root piece
	Int32Result (*GetModelRootPiece)(const char* modelName, int32_t modelType);
	Int32Result (*GetUnitRootPiece)(int32_t unitID);
	Int32Result (*GetFeatureRootPiece)(int32_t featureID);

	// Piece lists and maps
	Int32Array (*GetModelPieceList)(const char* modelName, int32_t modelType);
	StringArray (*GetModelPieceMap)(const char* modelName, int32_t modelType);  // Returns piece names
	Int32Array (*GetUnitPieceList)(int32_t unitID);
	StringArray (*GetUnitPieceMap)(int32_t unitID);
	Int32Array (*GetFeaturePieceList)(int32_t featureID);
	StringArray (*GetFeaturePieceMap)(int32_t featureID);

	// Piece info
	PieceInfoResult (*GetUnitPieceInfo)(int32_t unitID, int32_t pieceNum);
	PieceInfoResult (*GetFeaturePieceInfo)(int32_t featureID, int32_t pieceNum);

	// Piece position and direction
	Float3Result (*GetUnitPiecePosition)(int32_t unitID, int32_t pieceNum);
	Float3Result (*GetUnitPieceDirection)(int32_t unitID, int32_t pieceNum);
	PiecePosDirResult (*GetUnitPiecePosDir)(int32_t unitID, int32_t pieceNum);

	Float3Result (*GetFeaturePiecePosition)(int32_t featureID, int32_t pieceNum);
	Float3Result (*GetFeaturePieceDirection)(int32_t featureID, int32_t pieceNum);
	PiecePosDirResult (*GetFeaturePiecePosDir)(int32_t featureID, int32_t pieceNum);

	// Piece matrix
	PieceMatrixResult (*GetUnitPieceMatrix)(int32_t unitID, int32_t pieceNum);
	PieceMatrixResult (*GetFeaturePieceMatrix)(int32_t featureID, int32_t pieceNum);

	// Script piece mapping
	Int32Result (*GetUnitScriptPiece)(int32_t unitID, int32_t scriptNum);
	StringArray (*GetUnitScriptNames)(int32_t unitID);
};

extern const UnitsPiecesApi UNITS_PIECES_API;

#ifdef __cplusplus
}
#endif

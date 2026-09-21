/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>

#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// Typed counterpart of Spring.UnitRendering and Spring.FeatureRendering.
// @see rts/Lua/LuaObjectRendering.cpp

enum ObjectRenderingObjectType {
	OBJECT_RENDERING_UNIT = 0,
	OBJECT_RENDERING_FEATURE = 1,
};

enum ObjectRenderingMaterialType {
	OBJECT_RENDERING_ALPHA = 0,
	OBJECT_RENDERING_OPAQUE = 1,
	OBJECT_RENDERING_ALPHA_REFLECT = 2,
	OBJECT_RENDERING_OPAQUE_REFLECT = 3,
	OBJECT_RENDERING_SHADOW = 4,
};

struct ObjectMaterialDescriptor {
	// Use shaderID >= 0 for a custom Gfx shader. Otherwise use the engine
	// shader name "3DO", "S3O", or "ASS".
	int32_t shaderID;
	const char* shader;
	int32_t order;
	bool useCamera;
	uint32_t culling;
	RECOIL_WASM_LIST("u32", "textureCount") const uint32_t* textureUnits;
	RECOIL_WASM_LIST("string", "textureCount") const char** textureNames;
	uint32_t textureCount;
	uint32_t preList;
	uint32_t postList;
};

struct ObjectRenderingResult {
	const Error* error;
	bool success;
};

struct GetObjectLODQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
};

struct GetObjectLODResult {
	const Error* error;
	uint32_t lodCount;
	uint32_t currentLOD;
};

struct SetObjectLODCountQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	uint32_t lodCount;
};

struct SetObjectLODDistanceQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	uint32_t lodLevel;
	float distance;
};

struct SetObjectLODLengthQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	uint32_t lodLevel;
	float length;
};

struct SetObjectPieceListQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	uint32_t lodLevel;
	// Public model-piece numbers are 1-based, like UnitsPieces and Lua.
	uint32_t piece;
	uint32_t displayList;
};

struct SetObjectMaterialQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	uint32_t lodLevel;
	ObjectRenderingMaterialType materialType;
	ObjectMaterialDescriptor material;
};

struct SetObjectMaterialLastLODQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	ObjectRenderingMaterialType materialType;
	uint32_t lodLevel;
};

struct SetObjectMaterialDisplayListsQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	uint32_t lodLevel;
	ObjectRenderingMaterialType materialType;
	uint32_t preList;
	uint32_t postList;
};

enum ObjectRenderingUniformType {
	OBJECT_RENDERING_UNIFORM_FLOAT = 0,
	OBJECT_RENDERING_UNIFORM_FLOAT2 = 1,
	OBJECT_RENDERING_UNIFORM_FLOAT3 = 2,
	OBJECT_RENDERING_UNIFORM_FLOAT4 = 3,
	OBJECT_RENDERING_UNIFORM_FLOAT_MAT3 = 4,
	OBJECT_RENDERING_UNIFORM_FLOAT_MAT4 = 5,
	OBJECT_RENDERING_UNIFORM_INT = 6,
	OBJECT_RENDERING_UNIFORM_INT2 = 7,
	OBJECT_RENDERING_UNIFORM_INT3 = 8,
	OBJECT_RENDERING_UNIFORM_INT4 = 9,
};

struct ObjectMaterialUniform {
	const char* name;
	ObjectRenderingUniformType type;
	float floatValues[32];
	int32_t intValues[32];
	uint32_t valueCount;
};

struct SetObjectMaterialUniformQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	ObjectRenderingMaterialType materialType;
	uint32_t lodLevel;
	ObjectMaterialUniform uniform;
};

struct ClearObjectMaterialUniformQuery {
	ObjectRenderingObjectType objectType;
	int32_t objectID;
	ObjectRenderingMaterialType materialType;
	uint32_t lodLevel;
	const char* name;
};

struct ObjectRenderingApi {
	void (*GetLOD)(const GetObjectLODQuery* query, GetObjectLODResult* result);
	void (*SetLODCount)(const SetObjectLODCountQuery* query, ObjectRenderingResult* result);
	void (*SetLODLength)(const SetObjectLODLengthQuery* query, ObjectRenderingResult* result);
	void (*SetLODDistance)(const SetObjectLODDistanceQuery* query, ObjectRenderingResult* result);
	void (*SetPieceList)(const SetObjectPieceListQuery* query, ObjectRenderingResult* result);
	void (*SetMaterial)(const SetObjectMaterialQuery* query, ObjectRenderingResult* result);
	void (*SetMaterialLastLOD)(const SetObjectMaterialLastLODQuery* query, ObjectRenderingResult* result);
	void (*SetMaterialDisplayLists)(const SetObjectMaterialDisplayListsQuery* query, ObjectRenderingResult* result);
	void (*SetForwardMaterialUniform)(const SetObjectMaterialUniformQuery* query, ObjectRenderingResult* result);
	void (*SetDeferredMaterialUniform)(const SetObjectMaterialUniformQuery* query, ObjectRenderingResult* result);
	void (*ClearForwardMaterialUniform)(const ClearObjectMaterialUniformQuery* query, ObjectRenderingResult* result);
	void (*ClearDeferredMaterialUniform)(const ClearObjectMaterialUniformQuery* query, ObjectRenderingResult* result);
};

extern const ObjectRenderingApi OBJECT_RENDERING_API;

#ifdef __cplusplus
}
#endif

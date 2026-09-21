/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "ObjectRendering.h"

#include <algorithm>
#include <cstring>

#include "Lua/LuaMaterial.h"
#include "Lua/LuaOpenGLUtils.h"
#include "NativeInterface/api/Gfx.h"
#include "Rendering/GL/myGL.h"
#include "Rendering/LuaObjectDrawer.h"
#include "Rendering/Models/LocalModel.hpp"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Objects/SolidObject.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"

namespace {

static const Error INVALID_OBJECT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid object ID",
};

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid object rendering argument",
};

static const Error NOT_FOUND_ERROR = {
	.code = ERROR_NOT_FOUND,
	.message = "Object rendering resource was not found",
};

static LuaObjType ToLuaObjectType(ObjectRenderingObjectType objectType)
{
	return (objectType == OBJECT_RENDERING_FEATURE) ? LUAOBJ_FEATURE : LUAOBJ_UNIT;
}

static CSolidObject* GetObject(ObjectRenderingObjectType objectType, int objectID)
{
	if (objectType == OBJECT_RENDERING_FEATURE)
		return featureHandler.GetFeature(objectID);

	return unitHandler.GetUnit(objectID);
}

static LuaMatType ToLuaMaterialType(ObjectRenderingMaterialType materialType)
{
	switch (materialType) {
		case OBJECT_RENDERING_ALPHA:          return LUAMAT_ALPHA;
		case OBJECT_RENDERING_OPAQUE:         return LUAMAT_OPAQUE;
		case OBJECT_RENDERING_ALPHA_REFLECT:  return LUAMAT_ALPHA_REFLECT;
		case OBJECT_RENDERING_OPAQUE_REFLECT: return LUAMAT_OPAQUE_REFLECT;
		case OBJECT_RENDERING_SHADOW:         return LUAMAT_SHADOW;
	}

	return LuaMatType(-1);
}

static LuaObjectMaterial* GetObjectMaterial(
	CSolidObject* object,
	ObjectRenderingMaterialType materialType)
{
	const LuaMatType luaMaterialType = ToLuaMaterialType(materialType);
	if (luaMaterialType < 0 || luaMaterialType >= LUAMAT_TYPE_COUNT)
		return nullptr;

	return object->GetLuaMaterialData()->GetLuaMaterial(luaMaterialType);
}

static LuaObjectLODMaterial* GetObjectLODMaterial(
	CSolidObject* object,
	ObjectRenderingMaterialType materialType,
	uint32_t lodLevel)
{
	if (lodLevel == 0)
		return nullptr;

	LuaObjectMaterial* objectMaterial = GetObjectMaterial(object, materialType);
	return (objectMaterial != nullptr) ? objectMaterial->GetMaterial(lodLevel - 1) : nullptr;
}

static bool BuildMaterial(const ObjectMaterialDescriptor& descriptor, LuaMatType materialType, LuaMaterial& material)
{
	if (descriptor.shaderID < 0 && descriptor.shader == nullptr)
		return false;

	material = LuaMaterial(materialType);
	for (int pass = 0; pass < LuaMatShader::LUASHADER_PASS_CNT; ++pass) {
		if (descriptor.shaderID >= 0) {
			uint32_t glProgramID = 0;
			if (!GetNativeGfxShaderProgram(static_cast<uint32_t>(descriptor.shaderID), &glProgramID))
				return false;
			material.shaders[pass].SetCustomTypeFromID(glProgramID);
		} else {
			material.shaders[pass].SetEngineTypeFromKey(descriptor.shader);
		}
	}

	material.useCamera = descriptor.useCamera;
	material.order = descriptor.order;
	material.cullingMode = descriptor.culling;
	if (descriptor.textureCount > CGlobalRendering::MAX_TEXTURE_UNITS)
		return false;

	for (uint32_t texture = 0; texture < descriptor.textureCount; ++texture) {
		if (descriptor.textureUnits == nullptr || descriptor.textureNames == nullptr ||
			descriptor.textureNames[texture] == nullptr ||
			descriptor.textureUnits[texture] >= CGlobalRendering::MAX_TEXTURE_UNITS)
			return false;
		const uint32_t unit = descriptor.textureUnits[texture];
		if (!LuaOpenGLUtils::ParseTextureImage(nullptr, material.textures[unit], descriptor.textureNames[texture]))
			return false;
		material.textures[unit].Enable(true);
	}

	if (descriptor.preList != 0 && !GetNativeGfxDisplayList(descriptor.preList, &material.preList))
		return false;
	if (descriptor.postList != 0 && !GetNativeGfxDisplayList(descriptor.postList, &material.postList))
		return false;

	material.Finalize();
	return true;
}

static void SetFailure(ObjectRenderingResult* result, const Error* error)
{
	result->error = error;
	result->success = false;
}

static void NativeGetLOD(const GetObjectLODQuery* query, GetObjectLODResult* result)
{
	result->error = nullptr;
	result->lodCount = 0;
	result->currentLOD = 0;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	if (object == nullptr) {
		result->error = &INVALID_OBJECT_ERROR;
		return;
	}

	const LuaObjectMaterialData* materialData = object->GetLuaMaterialData();
	result->lodCount = materialData->GetLODCount();
	result->currentLOD = materialData->GetCurrentLOD();
}

static void NativeSetLODCount(const SetObjectLODCountQuery* query, ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}
	if (query->lodCount > 1024) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	LuaObjectDrawer::SetObjectLOD(object, ToLuaObjectType(query->objectType), query->lodCount);
	result->success = true;
}

static void NativeSetLODDistance(const SetObjectLODDistanceQuery* query, ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	if (object == nullptr || query->lodLevel == 0) {
		SetFailure(result, (object == nullptr) ? &INVALID_OBJECT_ERROR : &INVALID_ARGUMENT_ERROR);
		return;
	}

	// Same projection scale as LuaObjectRendering::SetLODDistance.
	object->GetLuaMaterialData()->SetLODLength(
		query->lodLevel - 1, query->distance * 0.0010786811520132682f);
	result->success = true;
}

static void NativeSetLODLength(const SetObjectLODLengthQuery* query, ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}

	object->GetLuaMaterialData()->SetLODLength(query->lodLevel - 1, query->length);
	result->success = true;
}

static void NativeSetPieceList(const SetObjectPieceListQuery* query, ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}
	if (query->lodLevel == 0 || query->piece == 0 || !object->localModel.HasPiece(query->piece - 1)) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	LocalModelPiece* piece = object->localModel.GetPiece(query->piece - 1);
	if (piece->lodDispLists.size() <= query->lodLevel - 1)
		piece->SetLODCount(query->lodLevel);

	if (query->displayList != 0) {
		uint32_t glID = 0;
		if (!GetNativeGfxDisplayList(query->displayList, &glID)) {
			SetFailure(result, &NOT_FOUND_ERROR);
			return;
		}
		piece->lodDispLists[query->lodLevel - 1] = glID;
	} else {
		piece->lodDispLists[query->lodLevel - 1] = 0;
	}

	result->success = true;
}

static void NativeSetMaterial(const SetObjectMaterialQuery* query, ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	LuaObjectLODMaterial* lodMaterial = (object != nullptr)
		? GetObjectLODMaterial(object, query->materialType, query->lodLevel)
		: nullptr;
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}
	if (lodMaterial == nullptr) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	LuaMaterial material;
	if (!BuildMaterial(query->material, ToLuaMaterialType(query->materialType), material)) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	lodMaterial->matref = luaMatHandler.GetRef(material);
	lodMaterial->preDisplayList = material.preList;
	lodMaterial->postDisplayList = material.postList;
	result->success = true;
}

static void NativeSetMaterialLastLOD(const SetObjectMaterialLastLODQuery* query, ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	LuaObjectMaterial* material = (object != nullptr) ? GetObjectMaterial(object, query->materialType) : nullptr;
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}
	if (material == nullptr || query->lodLevel == 0) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	material->SetLastLOD(query->lodLevel - 1);
	result->success = true;
}

static void NativeSetMaterialDisplayLists(const SetObjectMaterialDisplayListsQuery* query, ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}

	LuaObjectLODMaterial* lodMaterial = GetObjectLODMaterial(object, query->materialType, query->lodLevel);
	if (lodMaterial == nullptr) {
		SetFailure(result, &NOT_FOUND_ERROR);
		return;
	}

	GLuint preList = 0;
	GLuint postList = 0;
	if ((query->preList != 0 && !GetNativeGfxDisplayList(query->preList, &preList)) ||
		(query->postList != 0 && !GetNativeGfxDisplayList(query->postList, &postList))) {
		SetFailure(result, &NOT_FOUND_ERROR);
		return;
	}

	lodMaterial->preDisplayList = preList;
	lodMaterial->postDisplayList = postList;
	result->success = true;
}

static GLenum UniformGLType(ObjectRenderingUniformType type, uint32_t* components, bool* integer)
{
	*integer = false;
	switch (type) {
		case OBJECT_RENDERING_UNIFORM_FLOAT:      *components = 1; return GL_FLOAT;
		case OBJECT_RENDERING_UNIFORM_FLOAT2:     *components = 2; return GL_FLOAT_VEC2;
		case OBJECT_RENDERING_UNIFORM_FLOAT3:     *components = 3; return GL_FLOAT_VEC3;
		case OBJECT_RENDERING_UNIFORM_FLOAT4:     *components = 4; return GL_FLOAT_VEC4;
		case OBJECT_RENDERING_UNIFORM_FLOAT_MAT3: *components = 9; return GL_FLOAT_MAT3;
		case OBJECT_RENDERING_UNIFORM_FLOAT_MAT4: *components = 16; return GL_FLOAT_MAT4;
		case OBJECT_RENDERING_UNIFORM_INT:  *components = 1; *integer = true; return GL_INT;
		case OBJECT_RENDERING_UNIFORM_INT2: *components = 2; *integer = true; return GL_INT_VEC2;
		case OBJECT_RENDERING_UNIFORM_INT3: *components = 3; *integer = true; return GL_INT_VEC3;
		case OBJECT_RENDERING_UNIFORM_INT4: *components = 4; *integer = true; return GL_INT_VEC4;
	}

	return 0;
}

static bool MakeUniform(const ObjectMaterialUniform& source, LuaMatUniform& target)
{
	if (source.name == nullptr || std::strlen(source.name) >= sizeof(target.name))
		return false;

	uint32_t components = 0;
	bool integer = false;
	target.type = UniformGLType(source.type, &components, &integer);
	if (target.type == 0 || source.valueCount == 0 || source.valueCount > 32 ||
		source.valueCount % components != 0)
		return false;

	std::strncpy(target.name, source.name, sizeof(target.name) - 1);
	target.size = source.valueCount / components;
	target.loc = -2;
	if (integer)
		std::memcpy(target.data.i, source.intValues, sizeof(target.data.i));
	else
		std::memcpy(target.data.f, source.floatValues, sizeof(target.data.f));
	return true;
}

static void NativeSetMaterialUniform(
	const SetObjectMaterialUniformQuery* query,
	LuaMatShader::Pass pass,
	ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	LuaObjectLODMaterial* lodMaterial = (object != nullptr)
		? GetObjectLODMaterial(object, query->materialType, query->lodLevel)
		: nullptr;
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}
	if (lodMaterial == nullptr || lodMaterial->matref.GetBin() == nullptr) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	LuaMatUniform uniform;
	if (!MakeUniform(query->uniform, uniform)) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	LuaMatUniforms& uniforms = lodMaterial->matref.GetBin()->uniforms[pass];
	uniforms.ClearObjectUniform(query->objectID, ToLuaObjectType(query->objectType), query->uniform.name);
	result->success = uniforms.AddObjectUniform(
		query->objectID, ToLuaObjectType(query->objectType), uniform);
}

static void NativeSetForwardMaterialUniform(const SetObjectMaterialUniformQuery* query, ObjectRenderingResult* result)
{
	NativeSetMaterialUniform(query, LuaMatShader::LUASHADER_PASS_FWD, result);
}

static void NativeSetDeferredMaterialUniform(const SetObjectMaterialUniformQuery* query, ObjectRenderingResult* result)
{
	NativeSetMaterialUniform(query, LuaMatShader::LUASHADER_PASS_DFR, result);
}

static void NativeClearMaterialUniform(
	const ClearObjectMaterialUniformQuery* query,
	LuaMatShader::Pass pass,
	ObjectRenderingResult* result)
{
	result->error = nullptr;
	result->success = false;

	CSolidObject* object = GetObject(query->objectType, query->objectID);
	LuaObjectLODMaterial* lodMaterial = (object != nullptr)
		? GetObjectLODMaterial(object, query->materialType, query->lodLevel)
		: nullptr;
	if (object == nullptr) {
		SetFailure(result, &INVALID_OBJECT_ERROR);
		return;
	}
	if (lodMaterial == nullptr || lodMaterial->matref.GetBin() == nullptr || query->name == nullptr) {
		SetFailure(result, &INVALID_ARGUMENT_ERROR);
		return;
	}

	result->success = lodMaterial->matref.GetBin()->uniforms[pass].ClearObjectUniform(
		query->objectID, ToLuaObjectType(query->objectType), query->name);
}

static void NativeClearForwardMaterialUniform(const ClearObjectMaterialUniformQuery* query, ObjectRenderingResult* result)
{
	NativeClearMaterialUniform(query, LuaMatShader::LUASHADER_PASS_FWD, result);
}

static void NativeClearDeferredMaterialUniform(const ClearObjectMaterialUniformQuery* query, ObjectRenderingResult* result)
{
	NativeClearMaterialUniform(query, LuaMatShader::LUASHADER_PASS_DFR, result);
}

} // namespace

const ObjectRenderingApi OBJECT_RENDERING_API = {
	.GetLOD = NativeGetLOD,
	.SetLODCount = NativeSetLODCount,
	.SetLODLength = NativeSetLODLength,
	.SetLODDistance = NativeSetLODDistance,
	.SetPieceList = NativeSetPieceList,
	.SetMaterial = NativeSetMaterial,
	.SetMaterialLastLOD = NativeSetMaterialLastLOD,
	.SetMaterialDisplayLists = NativeSetMaterialDisplayLists,
	.SetForwardMaterialUniform = NativeSetForwardMaterialUniform,
	.SetDeferredMaterialUniform = NativeSetDeferredMaterialUniform,
	.ClearForwardMaterialUniform = NativeClearForwardMaterialUniform,
	.ClearDeferredMaterialUniform = NativeClearDeferredMaterialUniform,
};

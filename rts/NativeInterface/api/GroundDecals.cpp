/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "GroundDecals.h"

#include <cmath>
#include <cstring>
#include <optional>
#include <string>

#include "Rendering/Env/IGroundDecalDrawer.h"
#include "Rendering/Env/Decals/GroundDecal.h"
#include "Sim/Features/Feature.h"
#include "Sim/Objects/SolidObject.h"
#include "Sim/Misc/GlobalConstants.h"
#include "Sim/Units/UnitHandler.h"
#include "System/type2.h"
#include "System/float3.h"
#include "System/float4.h"

namespace {

static thread_local uint8_t scratchBuffer[8192];
static thread_local size_t bufferPos = 0;

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Ground decal system not available"
};

static const Error INVALID_DECAL_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid decal ID"
};

static const Error BUFFER_OVERFLOW_ERROR = {
	.code = ERROR_BUFFER_OVERFLOW,
	.message = "Scratch buffer overflow"
};

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static void ResetBuffer()
{
	bufferPos = 0;
}

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

static GroundDecal* GetDecal(uint32_t id, const Error** error)
{
	if (groundDecals == nullptr) {
		*error = &NOT_READY_ERROR;
		return nullptr;
	}

	GroundDecal* decal = groundDecals->GetDecalById(id);
	if (decal == nullptr || !decal->IsValid()) {
		*error = &INVALID_DECAL_ERROR;
		return nullptr;
	}

	return decal;
}

static const GroundDecal* GetDecalConst(uint32_t id, const Error** error)
{
	return GetDecal(id, error);
}

static void NativeCreateGroundDecal(const CreateGroundDecalQuery* query, CreateGroundDecalResult* result)
{
	ResetBuffer();
	(void)query;
	result->error = nullptr;
	result->success = false;
	result->decalID = 0;

	if (groundDecals == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const uint32_t id = groundDecals->CreateLuaDecal();

	result->decalID = id;
	result->success = (id > 0);
}

static void NativeDestroyGroundDecal(const DestroyGroundDecalQuery* query, DestroyGroundDecalResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	if (groundDecals == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->success = groundDecals->DeleteLuaDecal(query->decalID);
	if (!result->success) {
		result->error = &INVALID_DECAL_ERROR;
	}
}

static void NativeGetAllGroundDecals(const GetAllGroundDecalsQuery*, GetAllGroundDecalsResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->decalIDs = nullptr;
	result->count = 0;

	if (groundDecals == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& decals = groundDecals->GetAllDecals();
	uint32_t validCount = 0;
	for (const auto& d : decals) {
		validCount += d.IsValid();
	}

	if (validCount == 0)
		return;

	const size_t bytesNeeded = validCount * sizeof(uint32_t);
	if (bytesNeeded > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	uint32_t* ids = reinterpret_cast<uint32_t*>(scratchBuffer + bufferPos);
	size_t base = bufferPos;
	bufferPos += bytesNeeded;

	uint32_t idx = 0;
	for (const auto& d : decals) {
		if (!d.IsValid())
			continue;
		ids[idx++] = d.info.id;
	}

	bufferPos = base + bytesNeeded;
	result->decalIDs = ids;
	result->count = validCount;
}

static void NativeGetGroundDecalType(const GetGroundDecalTypeQuery* query, GetGroundDecalTypeResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->type = 0;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->type = decal->info.type;
}

static void NativeGetGroundDecalOwner(const GetGroundDecalOwnerQuery* query, GetGroundDecalOwnerResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->hasOwner = false;
	result->ownerID = -1;

	if (groundDecals == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CSolidObject* owner = groundDecals->GetDecalSolidObjectOwner(query->decalID);
	if (owner == nullptr)
		return;

	result->hasOwner = true;
	if (const auto* f = dynamic_cast<const CFeature*>(owner); f != nullptr)
		result->ownerID = unitHandler.MaxUnits() + owner->id;
	else
		result->ownerID = owner->id;
}

static void NativeGetGroundDecalTextures(const GetGroundDecalTexturesQuery* query, GetGroundDecalTexturesResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->textures = nullptr;
	result->textureCount = 0;
	result->filenames = nullptr;
	result->filenameCount = 0;

	if (groundDecals == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& texNames = groundDecals->GetDecalTextures(query->mainTex ? std::optional<bool>(true) : std::optional<bool>(false));
	const auto texCount = texNames.size();

	if (texCount > 0) {
		const size_t pointerBytes = texCount * sizeof(const char*);
		if (bufferPos + pointerBytes > sizeof(scratchBuffer)) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		const char** names = reinterpret_cast<const char**>(scratchBuffer + bufferPos);
		bufferPos += pointerBytes;

		for (size_t i = 0; i < texCount; ++i) {
			if (!CopyString(texNames[i], &names[i])) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				return;
			}
		}

		result->textures = names;
		result->textureCount = static_cast<uint32_t>(texCount);
	}

	if (!query->includeFilenames || texCount == 0)
		return;

	const auto& texFileNames = groundDecals->GetDecalTextureFileNames(texNames);
	const auto fileCount = texFileNames.size();
	if (fileCount == 0)
		return;

	const size_t pointerBytes = fileCount * sizeof(const char*);
	if (bufferPos + pointerBytes > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	const char** names = reinterpret_cast<const char**>(scratchBuffer + bufferPos);
	bufferPos += pointerBytes;

	for (size_t i = 0; i < fileCount; ++i) {
		if (!CopyString(texFileNames[i], &names[i])) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}
	}

	result->filenames = names;
	result->filenameCount = static_cast<uint32_t>(fileCount);
}

static void NativeGetGroundDecalTexture(const GetGroundDecalTextureQuery* query, GetGroundDecalTextureResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->texture = nullptr;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	const auto& texName = groundDecals->GetDecalTexture(query->decalID, query->mainTex);
	if (!texName.empty() && !CopyString(texName, &result->texture)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
	}
}

static void NativeGetGroundDecalTextureParams(const GetGroundDecalTextureParamsQuery* query, GetGroundDecalTextureParamsResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->texWrapDistance = 0.0f;
	result->texTraveledDistance = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->texWrapDistance = decal->uvWrapDistance;
	result->texTraveledDistance = decal->uvTraveledDistance;
}

static void NativeGetGroundDecalAlpha(const GetGroundDecalAlphaQuery* query, GetGroundDecalAlphaResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->alpha = 0.0f;
	result->alphaFalloff = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->alpha = decal->alpha;
	result->alphaFalloff = decal->alphaFalloff * GAME_SPEED;
}

static void NativeGetGroundDecalTint(const GetGroundDecalTintQuery* query, GetGroundDecalTintResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->tint[0] = result->tint[1] = result->tint[2] = result->tint[3] = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	float4 tint = decal->tintColor;
	result->tint[0] = tint.r;
	result->tint[1] = tint.g;
	result->tint[2] = tint.b;
	result->tint[3] = tint.a;
}

static void NativeGetGroundDecalNormal(const GetGroundDecalNormalQuery* query, GetGroundDecalNormalResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->normal[0] = result->normal[1] = result->normal[2] = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->normal[0] = decal->forcedNormal.x;
	result->normal[1] = decal->forcedNormal.y;
	result->normal[2] = decal->forcedNormal.z;
}

static void NativeGetGroundDecalGlowParams(const GetGroundDecalGlowParamsQuery* query, GetGroundDecalGlowParamsResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->glow = 0.0f;
	result->glowFalloff = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->glow = decal->glow;
	result->glowFalloff = decal->glowFalloff * GAME_SPEED;
}

static void NativeGetGroundDecalMisc(const GetGroundDecalMiscQuery* query, GetGroundDecalMiscResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->dotElimExp = 0.0f;
	result->refHeight = 0.0f;
	result->minHeight = 0.0f;
	result->maxHeight = 0.0f;
	result->forceHeightMode = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->dotElimExp = decal->dotElimExp;
	result->refHeight = decal->refHeight;
	result->minHeight = decal->minHeight;
	result->maxHeight = decal->maxHeight;
	result->forceHeightMode = decal->forceHeightMode;
}

static void NativeGetGroundDecalCreationFrame(const GetGroundDecalCreationFrameQuery* query, GetGroundDecalCreationFrameResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->creationFrameMin = 0.0f;
	result->creationFrameMax = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->creationFrameMin = decal->createFrameMin;
	result->creationFrameMax = decal->createFrameMax;
}

static void NativeGetGroundDecalUserData(const GetGroundDecalUserDataQuery* query, GetGroundDecalUserDataResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;
	result->values[0] = result->values[1] = result->values[2] = result->values[3] = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	if (query->quadIndex >= GroundDecal::NUM_USERDATA) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const float4& data = decal->userDefined[query->quadIndex];
	for (size_t i = 0; i < 4; ++i)
		result->values[i] = data[i];

	result->success = true;
}

static void NativeGetGroundDecalMiddlePos(const GetGroundDecalMiddlePosQuery* query, GetGroundDecalMiddlePosResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;
	result->midPos[0] = result->midPos[1] = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	const float2 midPoint = (decal->posTL + decal->posTR + decal->posBR + decal->posBL) * 0.25f;
	result->midPos[0] = midPoint.x;
	result->midPos[1] = midPoint.y;
	result->success = true;
}

static void NativeGetGroundDecalQuadPos(const GetGroundDecalQuadPosQuery* query, GetGroundDecalQuadPosResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->positions[0] = decal->posTL.x;
	result->positions[1] = decal->posTL.y;
	result->positions[2] = decal->posTR.x;
	result->positions[3] = decal->posTR.y;
	result->positions[4] = decal->posBR.x;
	result->positions[5] = decal->posBR.y;
	result->positions[6] = decal->posBL.x;
	result->positions[7] = decal->posBL.y;
	result->success = true;
}

static void NativeGetGroundDecalRotation(const GetGroundDecalRotationQuery* query, GetGroundDecalRotationResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;
	result->rotation = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->rotation = decal->rot;
	result->success = true;
}

static void NativeGetGroundDecalSizeAndHeight(const GetGroundDecalSizeAndHeightQuery* query, GetGroundDecalSizeAndHeightResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;
	result->sizeX = 0.0f;
	result->sizeZ = 0.0f;
	result->height = 0.0f;

	const GroundDecal* decal = GetDecalConst(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	result->sizeX = (decal->posTL.Distance(decal->posTR) + decal->posBL.Distance(decal->posBR)) * 0.25f * 2.0f;
	result->sizeZ = (decal->posTL.Distance(decal->posBL) + decal->posTR.Distance(decal->posBR)) * 0.25f * 2.0f;
	result->height = decal->height;
	result->success = true;
}

static void NativeSetGroundDecalPosAndDims(const SetGroundDecalPosAndDimsQuery* query, SetGroundDecalPosAndDimsResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	const float2 midPoint { query->midPosX, query->midPosZ };

	const float sizex = query->sizeX;
	const float sizez = query->sizeZ;

	const auto posTL = midPoint + float2(-sizex, -sizez);
	const auto posTR = midPoint + float2( sizex, -sizez);
	const auto posBR = midPoint + float2( sizex,  sizez);
	const auto posBL = midPoint + float2(-sizex,  sizez);

	decal->posTL = posTL;
	decal->posTR = posTR;
	decal->posBR = posBR;
	decal->posBL = posBL;

	const float computedHeight = std::sqrt(sizex * sizex + sizez * sizez);
	decal->height = (query->projCubeHeight > 0.0f) ? query->projCubeHeight : computedHeight;

	result->success = true;
}

static void NativeSetGroundDecalQuadPosAndHeight(const SetGroundDecalQuadPosAndHeightQuery* query, SetGroundDecalQuadPosAndHeightResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	decal->posTL = float2{ query->posTLX, query->posTLY };
	decal->posTR = float2{ query->posTRX, query->posTRY };
	decal->posBR = float2{ query->posBRX, query->posBRY };
	decal->posBL = float2{ query->posBLX, query->posBLY };

	const float sizex = (decal->posTL.Distance(decal->posTR) + decal->posBL.Distance(decal->posBR)) * 0.25f;
	const float sizez = (decal->posTL.Distance(decal->posBL) + decal->posTR.Distance(decal->posBR)) * 0.25f;
	const float computedHeight = std::sqrt(sizex * sizex + sizez * sizez);
	decal->height = (query->projCubeHeight > 0.0f) ? query->projCubeHeight : computedHeight;

	result->success = true;
}

static void NativeSetGroundDecalRotation(const SetGroundDecalRotationQuery* query, SetGroundDecalRotationResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	decal->rot = query->rotation;
	result->success = true;
}

static void NativeSetGroundDecalTexture(const SetGroundDecalTextureQuery* query, SetGroundDecalTextureResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	if (groundDecals == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->textureName == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->success = groundDecals->SetDecalTexture(query->decalID, query->textureName, query->mainTex);
	if (!result->success) {
		result->error = &INVALID_DECAL_ERROR;
	}
}

static void NativeSetGroundDecalTextureParams(const SetGroundDecalTextureParamsQuery* query, SetGroundDecalTextureParamsResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	decal->uvWrapDistance = query->texWrapDistance;
	decal->uvTraveledDistance = query->texTraveledDistance;
	result->success = true;
}

static void NativeSetGroundDecalAlpha(const SetGroundDecalAlphaQuery* query, SetGroundDecalAlphaResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	decal->alpha = query->alpha;
	decal->alphaFalloff = query->alphaFalloff / GAME_SPEED;
	result->success = true;
}

static void NativeSetGroundDecalTint(const SetGroundDecalTintQuery* query, SetGroundDecalTintResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	float4 tint { query->tintR, query->tintG, query->tintB, query->tintA };
	decal->tintColor = SColor{ tint };
	result->success = true;
}

static void NativeSetGroundDecalNormal(const SetGroundDecalNormalQuery* query, SetGroundDecalNormalResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	float3 normal { query->normalX, query->normalY, query->normalZ };
	if (normal.SqLength() > 0.0f)
		normal.SafeNormalize();
	decal->forcedNormal = normal;

	result->success = true;
}

static void NativeSetGroundDecalGlowParams(const SetGroundDecalGlowParamsQuery* query, SetGroundDecalGlowParamsResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	decal->glow = query->glow;
	decal->glowFalloff = query->glowFalloff / GAME_SPEED;
	result->success = true;
}

static void NativeSetGroundDecalMisc(const SetGroundDecalMiscQuery* query, SetGroundDecalMiscResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	decal->dotElimExp = query->dotElimExp;
	decal->refHeight = query->refHeight;
	decal->minHeight = query->minHeight;
	decal->maxHeight = query->maxHeight;
	decal->forceHeightMode = query->forceHeightMode;

	result->success = true;
}

static void NativeSetGroundDecalCreationFrame(const SetGroundDecalCreationFrameQuery* query, SetGroundDecalCreationFrameResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	decal->createFrameMin = query->creationFrameMin;
	decal->createFrameMax = query->creationFrameMax;
	result->success = true;
}

static void NativeSetGroundDecalUserData(const SetGroundDecalUserDataQuery* query, SetGroundDecalUserDataResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->success = false;

	GroundDecal* decal = GetDecal(query->decalID, &result->error);
	if (decal == nullptr)
		return;

	if (query->quadIndex >= GroundDecal::NUM_USERDATA) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	float4& data = decal->userDefined[query->quadIndex];
	data.x = query->valueX;
	data.y = query->valueY;
	data.z = query->valueZ;
	data.w = query->valueW;

	result->success = true;
}

} // namespace

const GroundDecalsApi GROUND_DECALS_API = {
	.CreateGroundDecal = NativeCreateGroundDecal,
	.DestroyGroundDecal = NativeDestroyGroundDecal,
	.GetAllGroundDecals = NativeGetAllGroundDecals,
	.GetGroundDecalType = NativeGetGroundDecalType,
	.GetGroundDecalOwner = NativeGetGroundDecalOwner,
	.GetGroundDecalTextures = NativeGetGroundDecalTextures,
	.GetGroundDecalTexture = NativeGetGroundDecalTexture,
	.GetGroundDecalTextureParams = NativeGetGroundDecalTextureParams,
	.GetGroundDecalAlpha = NativeGetGroundDecalAlpha,
	.GetGroundDecalTint = NativeGetGroundDecalTint,
	.GetGroundDecalNormal = NativeGetGroundDecalNormal,
	.GetGroundDecalGlowParams = NativeGetGroundDecalGlowParams,
	.GetGroundDecalMisc = NativeGetGroundDecalMisc,
	.GetGroundDecalCreationFrame = NativeGetGroundDecalCreationFrame,
	.GetGroundDecalUserData = NativeGetGroundDecalUserData,
	.GetGroundDecalMiddlePos = NativeGetGroundDecalMiddlePos,
	.GetGroundDecalQuadPos = NativeGetGroundDecalQuadPos,
	.GetGroundDecalRotation = NativeGetGroundDecalRotation,
	.GetGroundDecalSizeAndHeight = NativeGetGroundDecalSizeAndHeight,
	.SetGroundDecalPosAndDims = NativeSetGroundDecalPosAndDims,
	.SetGroundDecalQuadPosAndHeight = NativeSetGroundDecalQuadPosAndHeight,
	.SetGroundDecalRotation = NativeSetGroundDecalRotation,
	.SetGroundDecalTexture = NativeSetGroundDecalTexture,
	.SetGroundDecalTextureParams = NativeSetGroundDecalTextureParams,
	.SetGroundDecalAlpha = NativeSetGroundDecalAlpha,
	.SetGroundDecalTint = NativeSetGroundDecalTint,
	.SetGroundDecalNormal = NativeSetGroundDecalNormal,
	.SetGroundDecalGlowParams = NativeSetGroundDecalGlowParams,
	.SetGroundDecalMisc = NativeSetGroundDecalMisc,
	.SetGroundDecalCreationFrame = NativeSetGroundDecalCreationFrame,
	.SetGroundDecalUserData = NativeSetGroundDecalUserData,
};

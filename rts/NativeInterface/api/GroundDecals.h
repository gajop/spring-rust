/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

struct CreateGroundDecalQuery { uint8_t _unused; };
struct CreateGroundDecalResult { const Error* error; uint32_t decalID; bool success; };

struct DestroyGroundDecalQuery { uint32_t decalID; };
struct DestroyGroundDecalResult { const Error* error; bool success; };

struct GetAllGroundDecalsQuery { uint8_t _unused; };
struct GetAllGroundDecalsResult { const Error* error; const uint32_t* decalIDs; uint32_t count; };

struct GetGroundDecalTypeQuery { uint32_t decalID; };
struct GetGroundDecalTypeResult { const Error* error; uint8_t type; };

struct GetGroundDecalOwnerQuery { uint32_t decalID; };
struct GetGroundDecalOwnerResult { const Error* error; bool hasOwner; int32_t ownerID; };

struct GetGroundDecalTexturesQuery { bool mainTex; bool includeFilenames; };
struct GetGroundDecalTexturesResult { const Error* error; const char** textures; uint32_t textureCount; const char** filenames; uint32_t filenameCount; };

struct GetGroundDecalTextureQuery { uint32_t decalID; bool mainTex; };
struct GetGroundDecalTextureResult { const Error* error; const char* texture; };

struct GetGroundDecalTextureParamsQuery { uint32_t decalID; };
struct GetGroundDecalTextureParamsResult { const Error* error; float texWrapDistance; float texTraveledDistance; };

struct GetGroundDecalAlphaQuery { uint32_t decalID; };
struct GetGroundDecalAlphaResult { const Error* error; float alpha; float alphaFalloff; };

struct GetGroundDecalTintQuery { uint32_t decalID; };
struct GetGroundDecalTintResult { const Error* error; float tint[4]; };

struct GetGroundDecalNormalQuery { uint32_t decalID; };
struct GetGroundDecalNormalResult { const Error* error; float normal[3]; };

struct GetGroundDecalGlowParamsQuery { uint32_t decalID; };
struct GetGroundDecalGlowParamsResult { const Error* error; float glow; float glowFalloff; };

struct GetGroundDecalMiscQuery { uint32_t decalID; };
struct GetGroundDecalMiscResult { const Error* error; float dotElimExp; float refHeight; float minHeight; float maxHeight; float forceHeightMode; };

struct GetGroundDecalCreationFrameQuery { uint32_t decalID; };
struct GetGroundDecalCreationFrameResult { const Error* error; float creationFrameMin; float creationFrameMax; };

struct GetGroundDecalUserDataQuery { uint32_t decalID; uint32_t quadIndex; };
struct GetGroundDecalUserDataResult { const Error* error; float values[4]; bool success; };

struct GetGroundDecalMiddlePosQuery { uint32_t decalID; };
struct GetGroundDecalMiddlePosResult { const Error* error; float midPos[2]; bool success; };

struct GetGroundDecalQuadPosQuery { uint32_t decalID; };
struct GetGroundDecalQuadPosResult { const Error* error; float positions[8]; bool success; };

struct GetGroundDecalRotationQuery { uint32_t decalID; };
struct GetGroundDecalRotationResult { const Error* error; float rotation; bool success; };

struct GetGroundDecalSizeAndHeightQuery { uint32_t decalID; };
struct GetGroundDecalSizeAndHeightResult { const Error* error; float sizeX; float sizeZ; float height; bool success; };

struct SetGroundDecalPosAndDimsQuery { uint32_t decalID; float midPosX; float midPosZ; float sizeX; float sizeZ; float projCubeHeight; };
struct SetGroundDecalPosAndDimsResult { const Error* error; bool success; };

struct SetGroundDecalQuadPosAndHeightQuery {
	uint32_t decalID;
	float posTLX;
	float posTLY;
	float posTRX;
	float posTRY;
	float posBRX;
	float posBRY;
	float posBLX;
	float posBLY;
	float projCubeHeight;
};
struct SetGroundDecalQuadPosAndHeightResult { const Error* error; bool success; };

struct SetGroundDecalRotationQuery { uint32_t decalID; float rotation; };
struct SetGroundDecalRotationResult { const Error* error; bool success; };

struct SetGroundDecalTextureQuery { uint32_t decalID; const char* textureName; bool mainTex; };
struct SetGroundDecalTextureResult { const Error* error; bool success; };

struct SetGroundDecalTextureParamsQuery { uint32_t decalID; float texWrapDistance; float texTraveledDistance; };
struct SetGroundDecalTextureParamsResult { const Error* error; bool success; };

struct SetGroundDecalAlphaQuery { uint32_t decalID; float alpha; float alphaFalloff; };
struct SetGroundDecalAlphaResult { const Error* error; bool success; };

struct SetGroundDecalTintQuery { uint32_t decalID; float tintR; float tintG; float tintB; float tintA; };
struct SetGroundDecalTintResult { const Error* error; bool success; };

struct SetGroundDecalNormalQuery { uint32_t decalID; float normalX; float normalY; float normalZ; };
struct SetGroundDecalNormalResult { const Error* error; bool success; };

struct SetGroundDecalGlowParamsQuery { uint32_t decalID; float glow; float glowFalloff; };
struct SetGroundDecalGlowParamsResult { const Error* error; bool success; };

struct SetGroundDecalMiscQuery { uint32_t decalID; float dotElimExp; float refHeight; float minHeight; float maxHeight; float forceHeightMode; };
struct SetGroundDecalMiscResult { const Error* error; bool success; };

struct SetGroundDecalCreationFrameQuery { uint32_t decalID; float creationFrameMin; float creationFrameMax; };
struct SetGroundDecalCreationFrameResult { const Error* error; bool success; };

struct SetGroundDecalUserDataQuery { uint32_t decalID; uint32_t quadIndex; float valueX; float valueY; float valueZ; float valueW; };
struct SetGroundDecalUserDataResult { const Error* error; bool success; };

struct GroundDecalsApi {
	void (*CreateGroundDecal)(const CreateGroundDecalQuery* query, CreateGroundDecalResult* result);
	void (*DestroyGroundDecal)(const DestroyGroundDecalQuery* query, DestroyGroundDecalResult* result);
	void (*GetAllGroundDecals)(const GetAllGroundDecalsQuery* query, GetAllGroundDecalsResult* result);
	void (*GetGroundDecalType)(const GetGroundDecalTypeQuery* query, GetGroundDecalTypeResult* result);
	void (*GetGroundDecalOwner)(const GetGroundDecalOwnerQuery* query, GetGroundDecalOwnerResult* result);
	void (*GetGroundDecalTextures)(const GetGroundDecalTexturesQuery* query, GetGroundDecalTexturesResult* result);
	void (*GetGroundDecalTexture)(const GetGroundDecalTextureQuery* query, GetGroundDecalTextureResult* result);
	void (*GetGroundDecalTextureParams)(const GetGroundDecalTextureParamsQuery* query, GetGroundDecalTextureParamsResult* result);
	void (*GetGroundDecalAlpha)(const GetGroundDecalAlphaQuery* query, GetGroundDecalAlphaResult* result);
	void (*GetGroundDecalTint)(const GetGroundDecalTintQuery* query, GetGroundDecalTintResult* result);
	void (*GetGroundDecalNormal)(const GetGroundDecalNormalQuery* query, GetGroundDecalNormalResult* result);
	void (*GetGroundDecalGlowParams)(const GetGroundDecalGlowParamsQuery* query, GetGroundDecalGlowParamsResult* result);
	void (*GetGroundDecalMisc)(const GetGroundDecalMiscQuery* query, GetGroundDecalMiscResult* result);
	void (*GetGroundDecalCreationFrame)(const GetGroundDecalCreationFrameQuery* query, GetGroundDecalCreationFrameResult* result);
	void (*GetGroundDecalUserData)(const GetGroundDecalUserDataQuery* query, GetGroundDecalUserDataResult* result);
	void (*GetGroundDecalMiddlePos)(const GetGroundDecalMiddlePosQuery* query, GetGroundDecalMiddlePosResult* result);
	void (*GetGroundDecalQuadPos)(const GetGroundDecalQuadPosQuery* query, GetGroundDecalQuadPosResult* result);
	void (*GetGroundDecalRotation)(const GetGroundDecalRotationQuery* query, GetGroundDecalRotationResult* result);
	void (*GetGroundDecalSizeAndHeight)(const GetGroundDecalSizeAndHeightQuery* query, GetGroundDecalSizeAndHeightResult* result);
	void (*SetGroundDecalPosAndDims)(const SetGroundDecalPosAndDimsQuery* query, SetGroundDecalPosAndDimsResult* result);
	void (*SetGroundDecalQuadPosAndHeight)(const SetGroundDecalQuadPosAndHeightQuery* query, SetGroundDecalQuadPosAndHeightResult* result);
	void (*SetGroundDecalRotation)(const SetGroundDecalRotationQuery* query, SetGroundDecalRotationResult* result);
	void (*SetGroundDecalTexture)(const SetGroundDecalTextureQuery* query, SetGroundDecalTextureResult* result);
	void (*SetGroundDecalTextureParams)(const SetGroundDecalTextureParamsQuery* query, SetGroundDecalTextureParamsResult* result);
	void (*SetGroundDecalAlpha)(const SetGroundDecalAlphaQuery* query, SetGroundDecalAlphaResult* result);
	void (*SetGroundDecalTint)(const SetGroundDecalTintQuery* query, SetGroundDecalTintResult* result);
	void (*SetGroundDecalNormal)(const SetGroundDecalNormalQuery* query, SetGroundDecalNormalResult* result);
	void (*SetGroundDecalGlowParams)(const SetGroundDecalGlowParamsQuery* query, SetGroundDecalGlowParamsResult* result);
	void (*SetGroundDecalMisc)(const SetGroundDecalMiscQuery* query, SetGroundDecalMiscResult* result);
	void (*SetGroundDecalCreationFrame)(const SetGroundDecalCreationFrameQuery* query, SetGroundDecalCreationFrameResult* result);
	void (*SetGroundDecalUserData)(const SetGroundDecalUserDataQuery* query, SetGroundDecalUserDataResult* result);
};

extern const GroundDecalsApi GROUND_DECALS_API;

#ifdef __cplusplus
}
#endif

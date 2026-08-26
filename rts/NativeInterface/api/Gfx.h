/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

// @see rts/Lua/LuaOpenGL.cpp

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Graphics API (unsynced, draw-call context)
// Subset of Lua's gl.* surface. Calls that mutate GL state should be issued
// from a draw callin such as DrawScreen.
// ============================================================================

struct GfxEmptyQuery { uint8_t _unused; };
struct GfxEmptyResult { const Error* error; };

struct GfxBoolQuery { bool value; };
struct GfxIntQuery { int32_t value; };
struct GfxUIntQuery { uint32_t value; };
struct GfxFloatQuery { float value; };
struct GfxStringQuery { const char* value; };

// The numeric values are the OpenGL values exposed by Lua's GL constants.
// Keep this separate from the boolean Culling operation: Lua accepts both
// gl.Culling(false) and gl.Culling(GL_FRONT), and a typed WASM caller should
// not have to pass an unvalidated integer for the latter form.
enum GfxCullFace : uint32_t {
	GFX_CULL_FACE_FRONT = 0x0404,
	GFX_CULL_FACE_BACK = 0x0405,
	GFX_CULL_FACE_FRONT_AND_BACK = 0x0408,
};
struct GfxCullFaceQuery { GfxCullFace face; };

struct GfxBoolResult { const Error* error; bool value; };
struct GfxIntResult { const Error* error; int32_t value; };
struct GfxUIntResult { const Error* error; uint32_t value; };
struct GfxFloatResult { const Error* error; float value; };
struct GfxStringResult { const Error* error; const char* value; };

struct GfxGetNumberQuery { uint32_t pname; uint32_t maxValues; };
struct GfxGetNumberResult { const Error* error; float values[16]; uint32_t count; };

struct GfxGetStringQuery { uint32_t pname; };

struct GfxViewSizesResult { const Error* error; int32_t viewSizeX; int32_t viewSizeY; };
struct GfxViewRangeQuery { int32_t cameraType; };
struct GfxViewRangeResult { const Error* error; float nearPlaneDist; float farPlaneDist; float minViewRange; float maxViewRange; };
struct GfxShadowMapParamsResult { const Error* error; Float4 params; };

struct GfxMatrixQuery { float values[16]; };
struct GfxMatrixModeQuery { uint32_t mode; };
struct GfxRotateQuery { float degrees; float x; float y; float z; };
struct GfxTranslateQuery { float x; float y; float z; };
struct GfxTranslateResult { const Error* error; float x; float y; float z; };
struct GfxScaleQuery { float x; float y; float z; };
struct GfxOrthoQuery { float left; float right; float bottom; float top; float nearVal; float farVal; };
struct GfxFrustumQuery { float left; float right; float bottom; float top; float nearVal; float farVal; };
struct GfxGetMatrixDataQuery { uint32_t mode; };
struct GfxGetMatrixDataResult { const Error* error; float values[16]; };
struct GfxVertexQuery { float x; float y; float z; float w; uint32_t count; };
struct GfxMultiTexCoordQuery { int32_t texNum; float s; float t; float r; float q; uint32_t count; };
struct GfxColorQuery { float r; float g; float b; float a; };
struct GfxTexEnvQuery { uint32_t target; uint32_t pname; float values[4]; uint32_t count; };
struct GfxMultiTexEnvQuery { int32_t texNum; uint32_t target; uint32_t pname; float values[4]; uint32_t count; };
struct GfxTexGenOptions { bool setState; bool state; };
struct GfxTexGenQuery { uint32_t target; GfxTexGenOptions options; uint32_t pname; float values[4]; uint32_t count; };
struct GfxMultiTexGenOptions { bool setState; bool state; };
struct GfxMultiTexGenQuery { int32_t texNum; uint32_t target; GfxMultiTexGenOptions options; uint32_t pname; float values[4]; uint32_t count; };
struct GfxLightOptions { bool setState; bool state; };
struct GfxLightQuery { int32_t light; GfxLightOptions options; uint32_t pname; float values[4]; uint32_t count; };
struct GfxMaterialQuery { uint32_t pname; float values[4]; uint32_t count; };
struct GfxDispatchComputeQuery { uint32_t numGroupX; uint32_t numGroupY; uint32_t numGroupZ; uint32_t barriers; };
struct GfxMemoryBarrierQuery { uint32_t barriers; };
struct GfxActiveTextureQuery { int32_t texNum; };
struct GfxObjectLabelQuery { uint32_t identifier; uint32_t objectID; const char* label; };
struct GfxPushDebugGroupQuery { uint32_t id; const char* message; bool sourceIsThirdParty; };
struct GfxCreateShaderOptions {
	bool hasGeoInputType;
	uint32_t geoInputType;
	bool hasGeoOutputType;
	uint32_t geoOutputType;
	bool hasGeoOutputVerts;
	int32_t geoOutputVerts;
};
struct GfxCreateShaderQuery {
	const char* definitions;
	const char* vertex;
	const char* tcs;
	const char* tes;
	const char* geometry;
	const char* fragment;
	const char* compute;
	GfxCreateShaderOptions options;
};
struct GfxCreateShaderResult { const Error* error; uint32_t shaderID; uint32_t glProgramID; };
struct GfxShaderQuery { uint32_t shaderID; };
struct GfxActiveShaderQuery { uint32_t shaderID; NativeCallback callback; void* userData; };
struct GfxUseShaderResult { const Error* error; bool linked; };
struct GfxUniformLocationQuery { uint32_t shaderID; const char* name; };
struct GfxUniformLocationResult { const Error* error; int32_t location; };
struct GfxUniformFloatQuery { int32_t location; float values[4]; uint32_t count; };
struct GfxUniformIntQuery { int32_t location; int32_t values[4]; uint32_t count; };
struct GfxUniformArrayFloatQuery { int32_t location; const float* values; uint32_t count; };
struct GfxUniformArrayIntQuery { int32_t location; const int32_t* values; uint32_t count; };
struct GfxUniformMatrixQuery { int32_t location; const float* values; uint32_t count; bool transpose; };
struct GfxSubroutineIndexQuery { uint32_t shaderID; uint32_t shaderType; const char* name; };
struct GfxSubroutineIndexResult { const Error* error; int32_t index; bool success; };
struct GfxUniformSubroutineQuery { uint32_t shaderType; uint32_t index; };
struct GfxGeometryShaderParameterQuery { uint32_t shaderID; uint32_t param; int32_t value; };
struct GfxTesselationShaderParameterQuery { uint32_t param; int32_t value; float values[4]; uint32_t valueCount; bool useFloatArray; };
struct GfxEngineUniformBufferDefQuery { int32_t index; };
struct GfxEngineModelUniformDataSizeResult { const Error* error; uint32_t sizeInElements; uint32_t sizeInBytesOnCPU; };
struct GfxObjectBufferUniformsQuery { int32_t objectID; const float* values; uint32_t count; uint32_t offset; };
struct GfxObjectBufferUniformsResult { const Error* error; uint32_t count; };
struct GfxActiveUniformEntry { const char* name; const char* type; uint32_t glType; int32_t length; int32_t size; int32_t location; };
struct GfxActiveUniformsResult { const Error* error; const GfxActiveUniformEntry* entries; uint32_t count; };
struct GfxTextureParams {
	uint32_t target;
	uint32_t format;
	int32_t border;
	uint32_t minFilter;
	uint32_t magFilter;
	uint32_t wrapS;
	uint32_t wrapT;
	uint32_t wrapR;
	uint32_t compareFunc;
	float lodBias;
	float aniso;
	uint32_t samples;
	bool fbo;
	bool fboDepth;
};
struct GfxCreateTextureQuery { int32_t xsize; int32_t ysize; int32_t zsize; GfxTextureParams params; };
struct GfxTextureNameQuery { const char* name; };
struct GfxTextureBindQuery { const char* name; int32_t texNum; bool enable; };
struct GfxChangeTextureParamsQuery { const char* name; GfxTextureParams params; };
struct GfxTextureInfoResult { const Error* error; int32_t xsize; int32_t ysize; int32_t zsize; uint32_t id; uint32_t target; uint32_t fbo; };
struct GfxEngineTextureNamesResult { const Error* error; const char** names; uint32_t count; };
struct GfxConsoleCommandEntry { const char* command; const char* description; bool synced; bool cheat; };
struct GfxConsoleCommandsResult { const Error* error; const GfxConsoleCommandEntry* entries; uint32_t count; };

#ifdef __cplusplus
bool GetNativeGfxTextureInfo(const char* name, uint32_t* id, int32_t* xsize, int32_t* ysize, uint32_t* target);
#endif
struct GfxCopyToTextureQuery { const char* name; int32_t xoff; int32_t yoff; int32_t x; int32_t y; int32_t width; int32_t height; uint32_t target; uint32_t level; };
struct GfxUploadTextureQuery {
	const char* name;
	uint32_t target;
	int32_t level;
	int32_t xoff;
	int32_t yoff;
	int32_t zoff;
	int32_t width;
	int32_t height;
	int32_t depth;
	uint32_t format;
	uint32_t pixelType;
	const uint8_t* data;
	uint32_t dataSize;
};
struct GfxBindImageTextureQuery { uint32_t unit; const char* name; int32_t level; int32_t layer; bool layered; uint32_t access; uint32_t format; };
struct GfxReadPixelsQuery { int32_t x; int32_t y; int32_t width; int32_t height; uint32_t format; };
struct GfxReadPixelsResult { const Error* error; const float* values; uint32_t count; uint32_t components; };
struct GfxRBOCreateQuery { int32_t xsize; int32_t ysize; uint32_t target; uint32_t format; int32_t samples; };
struct GfxRBOInfoQuery { uint32_t rboID; };
struct GfxRBOInfoResult {
	const Error* error;
	bool valid;
	uint32_t target;
	uint32_t format;
	int32_t xsize;
	int32_t ysize;
	int32_t samples;
};
struct GfxFBOAttachment {
	uint32_t attachment;
	const char* textureName;
	uint32_t textureTarget;
	int32_t mipLevel;
	uint32_t rboID;
	bool useRBO;
};
struct GfxFBOAttachmentQuery {
	uint32_t fboID;
	uint32_t attachment;
	const char* textureName;
	uint32_t textureTarget;
	int32_t mipLevel;
	uint32_t rboID;
	bool useRBO;
};
struct GfxFBODrawBuffersQuery { uint32_t fboID; const uint32_t* buffers; uint32_t bufferCount; };
struct GfxFBOReadBufferQuery { uint32_t fboID; uint32_t buffer; };
struct GfxFBOCreateQuery {
	uint32_t target;
	const GfxFBOAttachment* attachments;
	uint32_t attachmentCount;
	const uint32_t* drawBuffers;
	uint32_t drawBufferCount;
	uint32_t readBuffer;
};
struct GfxFBOQuery { uint32_t fboID; uint32_t target; };
struct GfxFBOResult { const Error* error; uint32_t fboID; uint32_t rawID; };
struct GfxFBOStatusResult { const Error* error; bool valid; uint32_t status; };
struct GfxActiveFBOQuery { uint32_t fboID; uint32_t target; bool identities; NativeCallback callback; void* userData; };
struct GfxRawBindFBOQuery { bool bindDefault; uint32_t fboID; uint32_t target; uint32_t rawFboID; };
struct GfxRawBindFBOResult { const Error* error; uint32_t previouslyBoundRawFboID; bool hasPrevious; };
struct GfxBlitFBOQuery {
	uint32_t srcFBOID;
	uint32_t dstFBOID;
	int32_t x0Src;
	int32_t y0Src;
	int32_t x1Src;
	int32_t y1Src;
	int32_t x0Dst;
	int32_t y0Dst;
	int32_t x1Dst;
	int32_t y1Dst;
	uint32_t mask;
	uint32_t filter;
};
struct GfxClearAttachmentFBOQuery { uint32_t target; uint32_t attachment; float values[4]; uint32_t count; };
struct GfxVAOResult { const Error* error; uint32_t vaoID; uint32_t rawID; };
struct GfxVBOQuery { uint32_t target; bool freqUpdated; };
struct GfxVBOResult { const Error* error; uint32_t vboID; uint32_t rawID; uint32_t target; };
struct GfxVBOAttributeOptions {
	int32_t id;
	uint32_t type;
	int32_t size;
	bool normalized;
};
struct GfxVBODefineQuery {
	uint32_t vboID;
	int32_t elementsCount;
	bool elementArray;
	uint32_t indexType;
	bool useDefaultAttributes;
	uint32_t defaultAttributeCount;
	const GfxVBOAttributeOptions* attributes;
	uint32_t attributeCount;
};
struct GfxVBOInfoQuery { uint32_t vboID; };
struct GfxVBOInfoResult {
	const Error* error;
	uint32_t elementsCount;
	uint32_t bufferSizeInBytes;
	uint32_t gpuBufferSizeInBytes;
	uint32_t elemSizeInBytes;
	uint32_t attributesCount;
	uint32_t primitiveRestartIndex;
};
struct GfxVBOUploadQuery {
	uint32_t vboID;
	const float* data;
	uint32_t dataCount;
	int32_t attributeIndex;
	int32_t elementOffset;
	int32_t dataStartIndex;
	int32_t dataFinishIndex;
};
struct GfxVBOUploadResult { const Error* error; uint32_t bytesWritten; };
struct GfxVBODownloadQuery {
	uint32_t vboID;
	int32_t attributeIndex;
	int32_t elementOffset;
	int32_t elementCount;
	bool forceGPURead;
};
struct GfxVBODownloadResult { const Error* error; const float* values; uint32_t count; };
struct GfxVBOInstanceDataQuery {
	uint32_t vboID;
	const uint32_t* ids;
	uint32_t idCount;
	int32_t attributeIndex;
	int32_t teamID;
	int32_t elementOffset;
};
struct GfxVBOCopyQuery { uint32_t sourceVBOID; uint32_t destinationVBOID; int32_t copySizeInBytes; };
struct GfxVBOBindRangeQuery {
	uint32_t vboID;
	uint32_t bindingIndex;
	int32_t elementOffset;
	int32_t elementCount;
	uint32_t target;
	bool bind;
};
struct GfxVAOBufferQuery { uint32_t vaoID; uint32_t vboID; };
struct GfxVAODrawArraysQuery {
	uint32_t vaoID;
	uint32_t mode;
	int32_t vertexCount;
	int32_t vertexFirst;
	int32_t instanceCount;
	int32_t instanceFirst;
};
struct GfxVAODrawElementsQuery {
	uint32_t vaoID;
	uint32_t mode;
	int32_t drawCount;
	int32_t baseIndex;
	int32_t instanceCount;
	int32_t baseVertex;
	int32_t baseInstance;
};
struct GfxVAOSubmissionQuery { uint32_t vaoID; const uint32_t* ids; uint32_t idCount; };
struct GfxVAORemoveSubmissionQuery { uint32_t vaoID; int32_t index; };
struct GfxCreateTextureAtlasQuery { int32_t xsize; int32_t ysize; int32_t allocType; };
struct GfxAtlasTextureQuery { const char* atlasName; const char* textureName; };
struct GfxAtlasTextureResult { const Error* error; float x1; float x2; float y1; float y2; int32_t pageNum; };
struct GfxAtlasTextureEntry { const char* name; float x1; float x2; float y1; float y2; int32_t pageNum; };
struct GfxAtlasTexturesResult { const Error* error; const GfxAtlasTextureEntry* entries; uint32_t count; };
struct GfxVertexData {
	float vertex[3];
	float normal[3];
	float texCoord[2];
	float color[4];
	bool hasVertex;
	bool hasNormal;
	bool hasTexCoord;
	bool hasColor;
};
struct GfxShapeQuery { uint32_t primitive; const GfxVertexData* vertices; uint32_t vertexCount; };
struct GfxRectQuery { float x1; float y1; float x2; float y2; };
struct GfxTexRectQuery { float x1; float y1; float x2; float y2; float s1; float t1; float s2; float t2; };

struct GfxCallbackQuery { NativeCallback callback; void* userData; };
struct GfxBeginEndQuery { uint32_t primitive; NativeCallback callback; void* userData; };
struct GfxRenderToTextureQuery { const char* name; NativeCallback callback; void* userData; };
struct GfxUnsafeStateQuery { uint32_t state; bool reverse; NativeCallback callback; void* userData; };
struct GfxRunQueryQuery { uint32_t id; NativeCallback callback; void* userData; };
struct GfxSaveImageOptions { bool alpha; bool yflip; bool grayscale16bit; };
struct GfxSaveImageQuery { int32_t x; int32_t y; int32_t width; int32_t height; const char* filename; GfxSaveImageOptions options; uint32_t readBuffer; };
struct GfxTextQuery { const char* text; float x; float y; float size; const char* options; };
struct GfxTextHeightResult { const Error* error; float height; float descender; int32_t lines; };
struct GfxLoadFontQuery { const char* path; int32_t size; int32_t outlineWidth; float outlineWeight; };
struct GfxFontQuery { uint32_t fontID; };
struct GfxFontResult { const Error* error; uint32_t fontID; };
struct GfxFontInfoResult {
	const Error* error;
	const char* path;
	const char* family;
	const char* style;
	float size;
	float lineHeight;
	float descender;
	float outlineWidth;
	float outlineWeight;
	int32_t textureWidth;
	int32_t textureHeight;
};
struct GfxFontBeginQuery { uint32_t fontID; bool userDefinedBlending; };
struct GfxFontTextQuery { uint32_t fontID; const char* text; float x; float y; float size; const char* options; };
struct GfxFontWorldTextQuery { uint32_t fontID; const char* text; Float3 pos; float size; const char* options; };
struct GfxFontSubmitBufferedOptions { bool noBillboarding; bool userDefinedBlending; };
struct GfxFontSubmitBufferedQuery { uint32_t fontID; GfxFontSubmitBufferedOptions options; };
struct GfxFontWrapTextQuery { uint32_t fontID; const char* text; float maxWidth; float maxHeight; float size; };
struct GfxFontWrapTextResult { const Error* error; const char* text; int32_t lines; };
struct GfxFontColorQuery { uint32_t fontID; float r; float g; float b; float a; };
struct GfxFontAutoOutlineColorQuery { uint32_t fontID; bool enable; };
struct GfxGroundCircleQuery { Float3 pos; float radius; int32_t resolution; bool ballistic; float slope; float gravity; int32_t weaponDefID; };
struct GfxGroundQuadQuery { float x0; float z0; float x1; float z1; bool useTexCoords; float tu0; float tv0; float tu1; float tv1; };
struct GfxMiniMapConfigQuery { int32_t px; int32_t py; int32_t sx; int32_t sy; };
struct GfxFixedStateQuery { const char* param; };
struct GfxFixedStateResult { const Error* error; bool bools[8]; uint32_t boolCount; int32_t ints[16]; uint32_t intCount; float floats[16]; uint32_t floatCount; };
struct GfxUnitDrawOptions { bool applyTransform; bool doRawDraw; bool noLuaCall; bool fullModel; };
struct GfxUnitDrawQuery { int32_t unitID; GfxUnitDrawOptions options; };
struct GfxFeatureDrawOptions { bool applyTransform; bool doRawDraw; bool noLuaCall; };
struct GfxFeatureDrawQuery { int32_t featureID; GfxFeatureDrawOptions options; };
struct GfxObjectTextureStateQuery { int32_t objectID; bool push; };
struct GfxObjectShapeOptions { bool rawState; bool toScreen; bool opaque; };
struct GfxObjectShapeQuery { int32_t defID; int32_t teamID; GfxObjectShapeOptions options; };
struct GfxObjectPieceQuery { int32_t objectID; int32_t pieceID; };
struct GfxDrawListAtUnitQuery { int32_t unitID; uint32_t listID; bool useMidPos; Float3 scale; float degrees; Float3 rot; };
struct GfxDrawFuncAtUnitQuery { int32_t unitID; bool useMidPos; NativeCallback callback; void* userData; };
struct GfxClearQuery { uint32_t bits; float values[4]; uint32_t count; };
struct GfxDepthTestOptions { bool enable; bool setFunc; uint32_t func; };
struct GfxDepthTestQuery { GfxDepthTestOptions options; };
struct GfxBlendFuncQuery { uint32_t src; uint32_t dst; };
struct GfxBlendFuncSeparateQuery { uint32_t srcRGB; uint32_t dstRGB; uint32_t srcAlpha; uint32_t dstAlpha; };
struct GfxBlendEquationQuery { uint32_t mode; };
struct GfxBlendEquationSeparateQuery { uint32_t modeRGB; uint32_t modeAlpha; };
struct GfxColorMaskOptions { bool red; bool green; bool blue; bool alpha; };
struct GfxColorMaskQuery { GfxColorMaskOptions options; };
struct GfxScissorQuery { int32_t x; int32_t y; int32_t width; int32_t height; };
struct GfxViewportQuery { int32_t x; int32_t y; int32_t width; int32_t height; };
struct GfxAlphaTestQuery { bool enable; uint32_t func; float ref; };
struct GfxStencilTestQuery { bool enable; };
struct GfxStencilFuncQuery { uint32_t func; int32_t ref; uint32_t mask; };
struct GfxStencilFuncSeparateQuery { uint32_t face; uint32_t func; int32_t ref; uint32_t mask; };
struct GfxStencilMaskQuery { uint32_t mask; };
struct GfxStencilMaskSeparateQuery { uint32_t face; uint32_t mask; };
struct GfxStencilOpQuery { uint32_t fail; uint32_t zfail; uint32_t zpass; };
struct GfxStencilOpSeparateQuery { uint32_t face; uint32_t fail; uint32_t zfail; uint32_t zpass; };
struct GfxPolygonModeQuery { uint32_t face; uint32_t mode; };
struct GfxPolygonOffsetQuery { float factor; float units; };
struct GfxLogicOpQuery { bool enable; uint32_t opcode; };
struct GfxShadeModelQuery { uint32_t mode; };
struct GfxLineStippleQuery { int32_t factor; uint16_t pattern; };
struct GfxPointParameterQuery { uint32_t pname; float value; float values[4]; uint32_t count; };
struct GfxClipPlaneQuery { uint32_t plane; float equation[4]; };
struct GfxClipDistanceQuery { uint32_t index; bool enable; };

struct GfxValueQuery { const char* key; const char* mode; };
struct GfxValueResult {
	const Error* error;
	float values[4];
	uint32_t count;
	bool boolValue;
	bool hasBool;
	const char* stringValue;
};

struct GfxApi {
	void (*HasExtension)(const GfxStringQuery* query, GfxBoolResult* result);
	void (*GetNumber)(const GfxGetNumberQuery* query, GfxGetNumberResult* result);
	void (*GetString)(const GfxGetStringQuery* query, GfxStringResult* result);
	void (*GetViewSizes)(const GfxEmptyQuery* query, GfxViewSizesResult* result);
	void (*GetViewRange)(const GfxViewRangeQuery* query, GfxViewRangeResult* result);
	void (*GetShadowMapParams)(const GfxEmptyQuery* query, GfxShadowMapParamsResult* result);
	void (*GetAtmosphere)(const GfxValueQuery* query, GfxValueResult* result);
	void (*GetSun)(const GfxValueQuery* query, GfxValueResult* result);
	void (*GetWaterRendering)(const GfxValueQuery* query, GfxValueResult* result);
	void (*GetMapRendering)(const GfxValueQuery* query, GfxValueResult* result);

	void (*ResetState)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*Clear)(const GfxClearQuery* query, GfxEmptyResult* result);
	void (*Flush)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*Finish)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*SwapBuffers)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*ResetMatrices)(const GfxEmptyQuery* query, GfxEmptyResult* result);

	void (*DepthTest)(const GfxDepthTestQuery* query, GfxEmptyResult* result);
	void (*DepthMask)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*Culling)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*CullFace)(const GfxCullFaceQuery* query, GfxEmptyResult* result);
	void (*Blending)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*BlendFunc)(const GfxBlendFuncQuery* query, GfxEmptyResult* result);
	void (*BlendFuncSeparate)(const GfxBlendFuncSeparateQuery* query, GfxEmptyResult* result);
	void (*BlendEquation)(const GfxBlendEquationQuery* query, GfxEmptyResult* result);
	void (*BlendEquationSeparate)(const GfxBlendEquationSeparateQuery* query, GfxEmptyResult* result);
	void (*ColorMask)(const GfxColorMaskQuery* query, GfxEmptyResult* result);
	void (*AlphaTest)(const GfxAlphaTestQuery* query, GfxEmptyResult* result);
	void (*AlphaToCoverage)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*StencilTest)(const GfxStencilTestQuery* query, GfxEmptyResult* result);
	void (*StencilFunc)(const GfxStencilFuncQuery* query, GfxEmptyResult* result);
	void (*StencilFuncSeparate)(const GfxStencilFuncSeparateQuery* query, GfxEmptyResult* result);
	void (*StencilMask)(const GfxStencilMaskQuery* query, GfxEmptyResult* result);
	void (*StencilMaskSeparate)(const GfxStencilMaskSeparateQuery* query, GfxEmptyResult* result);
	void (*StencilOp)(const GfxStencilOpQuery* query, GfxEmptyResult* result);
	void (*StencilOpSeparate)(const GfxStencilOpSeparateQuery* query, GfxEmptyResult* result);
	void (*PolygonMode)(const GfxPolygonModeQuery* query, GfxEmptyResult* result);
	void (*PolygonOffset)(const GfxPolygonOffsetQuery* query, GfxEmptyResult* result);
	void (*LogicOp)(const GfxLogicOpQuery* query, GfxEmptyResult* result);
	void (*ShadeModel)(const GfxShadeModelQuery* query, GfxEmptyResult* result);
	void (*Scissor)(const GfxScissorQuery* query, GfxEmptyResult* result);
	void (*Viewport)(const GfxViewportQuery* query, GfxEmptyResult* result);
	void (*LineWidth)(const GfxFloatQuery* query, GfxEmptyResult* result);
	void (*LineStipple)(const GfxLineStippleQuery* query, GfxEmptyResult* result);
	void (*PointSize)(const GfxFloatQuery* query, GfxEmptyResult* result);
	void (*PointSprite)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*PointParameter)(const GfxPointParameterQuery* query, GfxEmptyResult* result);
	void (*ClipPlane)(const GfxClipPlaneQuery* query, GfxEmptyResult* result);
	void (*ClipDistance)(const GfxClipDistanceQuery* query, GfxEmptyResult* result);
	void (*PushAttrib)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*PopAttrib)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*DepthClamp)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*Fog)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*Lighting)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*Light)(const GfxLightQuery* query, GfxEmptyResult* result);
	void (*Material)(const GfxMaterialQuery* query, GfxEmptyResult* result);
	void (*TexEnv)(const GfxTexEnvQuery* query, GfxEmptyResult* result);
	void (*TextEnv)(const GfxTexEnvQuery* query, GfxEmptyResult* result);
	void (*MultiTexEnv)(const GfxMultiTexEnvQuery* query, GfxEmptyResult* result);
	void (*TexGen)(const GfxTexGenQuery* query, GfxEmptyResult* result);
	void (*MultiTexGen)(const GfxMultiTexGenQuery* query, GfxEmptyResult* result);
	void (*DispatchCompute)(const GfxDispatchComputeQuery* query, GfxEmptyResult* result);
	void (*MemoryBarrier)(const GfxMemoryBarrierQuery* query, GfxEmptyResult* result);
	void (*ActiveTexture)(const GfxActiveTextureQuery* query, GfxEmptyResult* result);
	void (*ObjectLabel)(const GfxObjectLabelQuery* query, GfxEmptyResult* result);
	void (*PushDebugGroup)(const GfxPushDebugGroupQuery* query, GfxEmptyResult* result);
	void (*PopDebugGroup)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*CreateShader)(const GfxCreateShaderQuery* query, GfxCreateShaderResult* result);
	void (*DeleteShader)(const GfxShaderQuery* query, GfxBoolResult* result);
	void (*UseShader)(const GfxShaderQuery* query, GfxUseShaderResult* result);
	void (*ActiveShader)(const GfxActiveShaderQuery* query, GfxEmptyResult* result);
	void (*GetShaderLog)(const GfxEmptyQuery* query, GfxStringResult* result);
	void (*GetUniformLocation)(const GfxUniformLocationQuery* query, GfxUniformLocationResult* result);
	void (*GetActiveUniforms)(const GfxShaderQuery* query, GfxActiveUniformsResult* result);
	void (*Uniform)(const GfxUniformFloatQuery* query, GfxEmptyResult* result);
	void (*UniformInt)(const GfxUniformIntQuery* query, GfxEmptyResult* result);
	void (*UniformArrayFloat)(const GfxUniformArrayFloatQuery* query, GfxEmptyResult* result);
	void (*UniformArrayInt)(const GfxUniformArrayIntQuery* query, GfxEmptyResult* result);
	void (*UniformMatrix)(const GfxUniformMatrixQuery* query, GfxEmptyResult* result);
	void (*GetSubroutineIndex)(const GfxSubroutineIndexQuery* query, GfxSubroutineIndexResult* result);
	void (*UniformSubroutine)(const GfxUniformSubroutineQuery* query, GfxEmptyResult* result);
	void (*SetGeometryShaderParameter)(const GfxGeometryShaderParameterQuery* query, GfxEmptyResult* result);
	void (*SetTesselationShaderParameter)(const GfxTesselationShaderParameterQuery* query, GfxEmptyResult* result);
	void (*GetEngineUniformBufferDef)(const GfxEngineUniformBufferDefQuery* query, GfxStringResult* result);
	void (*GetEngineModelUniformDataDef)(const GfxEmptyQuery* query, GfxStringResult* result);
	void (*GetEngineModelUniformDataSize)(const GfxEmptyQuery* query, GfxEngineModelUniformDataSizeResult* result);
	void (*SetUnitBufferUniforms)(const GfxObjectBufferUniformsQuery* query, GfxObjectBufferUniformsResult* result);
	void (*SetFeatureBufferUniforms)(const GfxObjectBufferUniformsQuery* query, GfxObjectBufferUniformsResult* result);

	void (*CreateTexture)(const GfxCreateTextureQuery* query, GfxStringResult* result);
	void (*DeleteTexture)(const GfxTextureNameQuery* query, GfxBoolResult* result);
	void (*DeleteTextureFBO)(const GfxTextureNameQuery* query, GfxBoolResult* result);
	void (*BindTexture)(const GfxTextureBindQuery* query, GfxBoolResult* result);
	void (*TextureInfo)(const GfxTextureNameQuery* query, GfxTextureInfoResult* result);
	void (*GetEngineTextureNames)(const GfxEmptyQuery* query, GfxEngineTextureNamesResult* result);
	void (*GetConsoleCommands)(const GfxEmptyQuery* query, GfxConsoleCommandsResult* result);
	void (*ChangeTextureParams)(const GfxChangeTextureParamsQuery* query, GfxEmptyResult* result);
	void (*CopyToTexture)(const GfxCopyToTextureQuery* query, GfxEmptyResult* result);
	void (*UploadTexture)(const GfxUploadTextureQuery* query, GfxEmptyResult* result);
	void (*GenerateMipmap)(const GfxTextureNameQuery* query, GfxEmptyResult* result);
	void (*BindImageTexture)(const GfxBindImageTextureQuery* query, GfxEmptyResult* result);
	void (*ReadPixels)(const GfxReadPixelsQuery* query, GfxReadPixelsResult* result);
	void (*CreateRBO)(const GfxRBOCreateQuery* query, GfxUIntResult* result);
	void (*DeleteRBO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*GetRBOInfo)(const GfxRBOInfoQuery* query, GfxRBOInfoResult* result);
	void (*CreateFBO)(const GfxFBOCreateQuery* query, GfxFBOResult* result);
	void (*SetFBOAttachment)(const GfxFBOAttachmentQuery* query, GfxEmptyResult* result);
	void (*SetFBODrawBuffers)(const GfxFBODrawBuffersQuery* query, GfxEmptyResult* result);
	void (*SetFBOReadBuffer)(const GfxFBOReadBufferQuery* query, GfxEmptyResult* result);
	void (*DeleteFBO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*IsValidFBO)(const GfxFBOQuery* query, GfxFBOStatusResult* result);
	void (*ActiveFBO)(const GfxActiveFBOQuery* query, GfxEmptyResult* result);
	void (*RawBindFBO)(const GfxRawBindFBOQuery* query, GfxRawBindFBOResult* result);
	void (*BlitFBO)(const GfxBlitFBOQuery* query, GfxEmptyResult* result);
	void (*ClearAttachmentFBO)(const GfxClearAttachmentFBOQuery* query, GfxBoolResult* result);
	void (*GetVAO)(const GfxEmptyQuery* query, GfxVAOResult* result);
	void (*DeleteVAO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*AttachVertexBufferVAO)(const GfxVAOBufferQuery* query, GfxEmptyResult* result);
	void (*AttachInstanceBufferVAO)(const GfxVAOBufferQuery* query, GfxEmptyResult* result);
	void (*AttachIndexBufferVAO)(const GfxVAOBufferQuery* query, GfxEmptyResult* result);
	void (*DrawArraysVAO)(const GfxVAODrawArraysQuery* query, GfxEmptyResult* result);
	void (*DrawElementsVAO)(const GfxVAODrawElementsQuery* query, GfxEmptyResult* result);
	void (*ClearSubmissionVAO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*AddUnitsToSubmissionVAO)(const GfxVAOSubmissionQuery* query, GfxUIntResult* result);
	void (*AddFeaturesToSubmissionVAO)(const GfxVAOSubmissionQuery* query, GfxUIntResult* result);
	void (*AddUnitDefsToSubmissionVAO)(const GfxVAOSubmissionQuery* query, GfxUIntResult* result);
	void (*AddFeatureDefsToSubmissionVAO)(const GfxVAOSubmissionQuery* query, GfxUIntResult* result);
	void (*RemoveFromSubmissionVAO)(const GfxVAORemoveSubmissionQuery* query, GfxEmptyResult* result);
	void (*SubmitVAO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*GetVBO)(const GfxVBOQuery* query, GfxVBOResult* result);
	void (*DeleteVBO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*DefineVBO)(const GfxVBODefineQuery* query, GfxEmptyResult* result);
	void (*GetVBOInfo)(const GfxVBOInfoQuery* query, GfxVBOInfoResult* result);
	void (*UploadVBO)(const GfxVBOUploadQuery* query, GfxVBOUploadResult* result);
	void (*DownloadVBO)(const GfxVBODownloadQuery* query, GfxVBODownloadResult* result);
	void (*ClearVBO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*ModelsVBO)(const GfxUIntQuery* query, GfxUIntResult* result);
	void (*InstanceDataFromUnitDefsVBO)(const GfxVBOInstanceDataQuery* query, GfxUIntResult* result);
	void (*InstanceDataFromFeatureDefsVBO)(const GfxVBOInstanceDataQuery* query, GfxUIntResult* result);
	void (*InstanceDataFromUnitsVBO)(const GfxVBOInstanceDataQuery* query, GfxUIntResult* result);
	void (*InstanceDataFromFeaturesVBO)(const GfxVBOInstanceDataQuery* query, GfxUIntResult* result);
	void (*MatrixDataFromProjectilesVBO)(const GfxVBOInstanceDataQuery* query, GfxUIntResult* result);
	void (*BindBufferRangeVBO)(const GfxVBOBindRangeQuery* query, GfxIntResult* result);
	void (*UnbindBufferRangeVBO)(const GfxVBOBindRangeQuery* query, GfxIntResult* result);
	void (*DumpDefinitionVBO)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*CopyToVBO)(const GfxVBOCopyQuery* query, GfxBoolResult* result);
	void (*GetIDVBO)(const GfxUIntQuery* query, GfxUIntResult* result);
	void (*RenderToTexture)(const GfxRenderToTextureQuery* query, GfxEmptyResult* result);
	void (*CreateTextureAtlas)(const GfxCreateTextureAtlasQuery* query, GfxStringResult* result);
	void (*FinalizeTextureAtlas)(const GfxTextureNameQuery* query, GfxBoolResult* result);
	void (*DeleteTextureAtlas)(const GfxTextureNameQuery* query, GfxBoolResult* result);
	void (*AddAtlasTexture)(const GfxAtlasTextureQuery* query, GfxEmptyResult* result);
	void (*GetAtlasTexture)(const GfxAtlasTextureQuery* query, GfxAtlasTextureResult* result);
	void (*GetEngineAtlasTextures)(const GfxTextureNameQuery* query, GfxAtlasTexturesResult* result);
	void (*SaveImage)(const GfxSaveImageQuery* query, GfxBoolResult* result);
	void (*CreateList)(const GfxCallbackQuery* query, GfxUIntResult* result);
	void (*CallList)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*DeleteList)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*CreateQuery)(const GfxEmptyQuery* query, GfxUIntResult* result);
	void (*DeleteQuery)(const GfxUIntQuery* query, GfxEmptyResult* result);
	void (*RunQuery)(const GfxRunQueryQuery* query, GfxEmptyResult* result);
	void (*GetQuery)(const GfxUIntQuery* query, GfxUIntResult* result);
	void (*GetGlobalTexNames)(const GfxEmptyQuery* query, GfxAtlasTexturesResult* result);
	void (*GetGlobalTexCoords)(const GfxStringQuery* query, GfxAtlasTextureResult* result);
	void (*BeginText)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*Text)(const GfxTextQuery* query, GfxEmptyResult* result);
	void (*EndText)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*GetTextWidth)(const GfxStringQuery* query, GfxFloatResult* result);
	void (*GetTextHeight)(const GfxStringQuery* query, GfxTextHeightResult* result);
	void (*AddFallbackFont)(const GfxStringQuery* query, GfxBoolResult* result);
	void (*ClearFallbackFonts)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*LoadFont)(const GfxLoadFontQuery* query, GfxFontResult* result);
	void (*DeleteFont)(const GfxFontQuery* query, GfxEmptyResult* result);
	void (*GetFontInfo)(const GfxFontQuery* query, GfxFontInfoResult* result);
	void (*FontBegin)(const GfxFontBeginQuery* query, GfxEmptyResult* result);
	void (*FontEnd)(const GfxFontQuery* query, GfxEmptyResult* result);
	void (*FontPrint)(const GfxFontTextQuery* query, GfxEmptyResult* result);
	void (*FontPrintWorld)(const GfxFontWorldTextQuery* query, GfxEmptyResult* result);
	void (*FontSubmitBuffered)(const GfxFontSubmitBufferedQuery* query, GfxEmptyResult* result);
	void (*FontWrapText)(const GfxFontWrapTextQuery* query, GfxFontWrapTextResult* result);
	void (*FontGetTextWidth)(const GfxFontTextQuery* query, GfxFloatResult* result);
	void (*FontGetTextHeight)(const GfxFontTextQuery* query, GfxTextHeightResult* result);
	void (*FontSetTextColor)(const GfxFontColorQuery* query, GfxEmptyResult* result);
	void (*FontSetOutlineColor)(const GfxFontColorQuery* query, GfxEmptyResult* result);
	void (*FontSetAutoOutlineColor)(const GfxFontAutoOutlineColorQuery* query, GfxEmptyResult* result);
	void (*FontBindTexture)(const GfxFontQuery* query, GfxEmptyResult* result);
	void (*BeginEnd)(const GfxBeginEndQuery* query, GfxEmptyResult* result);
	void (*PushPopMatrix)(const GfxCallbackQuery* query, GfxEmptyResult* result);
	void (*UnsafeState)(const GfxUnsafeStateQuery* query, GfxEmptyResult* result);
	void (*DrawGroundCircle)(const GfxGroundCircleQuery* query, GfxEmptyResult* result);
	void (*DrawGroundQuad)(const GfxGroundQuadQuery* query, GfxEmptyResult* result);
	void (*GetFixedState)(const GfxFixedStateQuery* query, GfxFixedStateResult* result);
	void (*GetScreenViewTrans)(const GfxEmptyQuery* query, GfxTranslateResult* result);
	void (*SlaveMiniMap)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*ConfigMiniMap)(const GfxMiniMapConfigQuery* query, GfxEmptyResult* result);
	void (*DrawMiniMap)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*Unit)(const GfxUnitDrawQuery* query, GfxEmptyResult* result);
	void (*UnitRaw)(const GfxUnitDrawQuery* query, GfxEmptyResult* result);
	void (*UnitTextures)(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result);
	void (*UnitShape)(const GfxObjectShapeQuery* query, GfxEmptyResult* result);
	void (*UnitShapeTextures)(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result);
	void (*UnitMultMatrix)(const GfxIntQuery* query, GfxEmptyResult* result);
	void (*UnitPiece)(const GfxObjectPieceQuery* query, GfxEmptyResult* result);
	void (*UnitPieceMatrix)(const GfxObjectPieceQuery* query, GfxEmptyResult* result);
	void (*UnitPieceMultMatrix)(const GfxObjectPieceQuery* query, GfxEmptyResult* result);
	void (*Feature)(const GfxFeatureDrawQuery* query, GfxEmptyResult* result);
	void (*FeatureRaw)(const GfxFeatureDrawQuery* query, GfxEmptyResult* result);
	void (*FeatureTextures)(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result);
	void (*FeatureShape)(const GfxObjectShapeQuery* query, GfxEmptyResult* result);
	void (*FeatureShapeTextures)(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result);
	void (*FeatureMultMatrix)(const GfxIntQuery* query, GfxEmptyResult* result);
	void (*FeaturePiece)(const GfxObjectPieceQuery* query, GfxEmptyResult* result);
	void (*FeaturePieceMatrix)(const GfxObjectPieceQuery* query, GfxEmptyResult* result);
	void (*FeaturePieceMultMatrix)(const GfxObjectPieceQuery* query, GfxEmptyResult* result);
	void (*DrawListAtUnit)(const GfxDrawListAtUnitQuery* query, GfxEmptyResult* result);
	void (*DrawFuncAtUnit)(const GfxDrawFuncAtUnitQuery* query, GfxEmptyResult* result);

	void (*MatrixMode)(const GfxMatrixModeQuery* query, GfxEmptyResult* result);
	void (*LoadIdentity)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*LoadMatrix)(const GfxMatrixQuery* query, GfxEmptyResult* result);
	void (*MultMatrix)(const GfxMatrixQuery* query, GfxEmptyResult* result);
	void (*PushMatrix)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*PopMatrix)(const GfxEmptyQuery* query, GfxEmptyResult* result);
	void (*Translate)(const GfxTranslateQuery* query, GfxEmptyResult* result);
	void (*Scale)(const GfxScaleQuery* query, GfxEmptyResult* result);
	void (*Rotate)(const GfxRotateQuery* query, GfxEmptyResult* result);
	void (*Ortho)(const GfxOrthoQuery* query, GfxEmptyResult* result);
	void (*Frustum)(const GfxFrustumQuery* query, GfxEmptyResult* result);
	void (*GetMatrixData)(const GfxGetMatrixDataQuery* query, GfxGetMatrixDataResult* result);

	void (*Vertex)(const GfxVertexQuery* query, GfxEmptyResult* result);
	void (*Normal)(const GfxTranslateQuery* query, GfxEmptyResult* result);
	void (*TexCoord)(const GfxVertexQuery* query, GfxEmptyResult* result);
	void (*MultiTexCoord)(const GfxMultiTexCoordQuery* query, GfxEmptyResult* result);
	void (*Color)(const GfxColorQuery* query, GfxEmptyResult* result);
	void (*SecondaryColor)(const GfxTranslateQuery* query, GfxEmptyResult* result);
	void (*FogCoord)(const GfxFloatQuery* query, GfxEmptyResult* result);
	void (*EdgeFlag)(const GfxBoolQuery* query, GfxEmptyResult* result);
	void (*Rect)(const GfxRectQuery* query, GfxEmptyResult* result);
	void (*TexRect)(const GfxTexRectQuery* query, GfxEmptyResult* result);
	void (*Shape)(const GfxShapeQuery* query, GfxEmptyResult* result);
	void (*Billboard)(const GfxEmptyQuery* query, GfxEmptyResult* result);
};

extern const GfxApi GFX_API;

#ifdef __cplusplus
}
#endif

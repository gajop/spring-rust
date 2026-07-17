/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Gfx.h"

#include <algorithm>
#include <cstring>
#include <limits>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "Game/Camera.h"
#include "Game/SyncedGameCommands.h"
#include "Game/UnsyncedGameCommands.h"
#include "Game/CameraHandler.h"
#include "Game/UI/MiniMap.h"
#include "Map/MapInfo.h"
#include "Map/ReadMap.h"
#include "Rendering/Map/InfoTexture/IInfoTextureHandler.h"
#include "Rendering/Env/ISky.h"
#include "Rendering/Env/MapRendering.h"
#include "Rendering/Env/Particles/ProjectileDrawer.h"
#include "Rendering/Env/SunLighting.h"
#include "Rendering/Env/WaterRendering.h"
#include "Rendering/Fonts/CFontTexture.h"
#include "Rendering/GL/FBO.h"
#include "Rendering/GL/glExtra.h"
#include "Rendering/GL/TexBind.h"
#include "Rendering/GL/myGL.h"
#include "Rendering/GlobalRendering.h"
#include "Rendering/Fonts/glFont.h"
#include "Rendering/Common/ModelDrawerHelpers.h"
#include "Rendering/Features/FeatureDrawer.h"
#include "Rendering/Models/3DModel.hpp"
#include "Rendering/Models/3DModelMisc.hpp"
#include "Rendering/Models/3DModelPiece.hpp"
#include "Rendering/Models/LocalModel.hpp"
#include "Rendering/Models/ModelsMemStorage.h"
#include "Rendering/ShadowHandler.h"
#include "Rendering/Textures/3DOTextureHandler.h"
#include "Rendering/Textures/Bitmap.h"
#include "Rendering/Textures/TextureAtlas.h"
#include "Rendering/Textures/TextureFormat.h"
#include "Lua/LuaOpenGLUtils.h"
#include "Constants.h"

// Verify every exposed GLConstant (api/Constants.h) matches the real GL_* value
// from the loader, so a wrong literal in the mirror fails the build here rather
// than silently handing native callers a bad value.
#define SPRING_GL_VERIFY_ENTRY(name, value) \
	static_assert(static_cast<unsigned>(GLC_##name) == static_cast<unsigned>(GL_##name), \
		"GLConstant GLC_" #name " does not match GL_" #name);
SPRING_GL_CONSTANTS(SPRING_GL_VERIFY_ENTRY)
#undef SPRING_GL_VERIFY_ENTRY
#include "System/Config/ConfigHandler.h"
#include "System/StringHash.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Features/FeatureDefHandler.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Objects/SolidObject.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Weapons/WeaponDefHandler.h"
#include "Rendering/Units/UnitDrawer.h"
#include "Rendering/UniformConstants.h"

namespace {

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Graphics system not ready",
};

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid graphics argument",
};

static const Error NOT_FOUND_ERROR = {
	.code = ERROR_NOT_FOUND,
	.message = "Graphics object not found",
};

static const Error OPERATION_FAILED_ERROR = {
	.code = ERROR_INTERNAL,
	.message = "Graphics operation failed",
};

struct NativeTexture {
	GLuint id = 0;
	GLuint fbo = 0;
	GLuint fboDepth = 0;
	GLenum target = GL_TEXTURE_2D;
	GLenum format = GL_RGBA8;
	GLsizei xsize = 0;
	GLsizei ysize = 0;
	GLsizei zsize = 0;
	GLsizei samples = 0;
	GLint border = 0;
	GLenum minFilter = GL_LINEAR;
	GLenum magFilter = GL_LINEAR;
	GLenum wrapS = GL_REPEAT;
	GLenum wrapT = GL_REPEAT;
	GLenum wrapR = GL_REPEAT;
	GLenum compareFunc = GL_NONE;
	GLfloat lodBias = 0.0f;
	GLfloat aniso = 0.0f;
};

struct NativeShaderObject {
	GLuint id = 0;
	GLenum type = 0;
};

struct NativeShaderProgram {
	GLuint id = 0;
	std::vector<NativeShaderObject> objects;
};

struct NativeRBO {
	GLuint id = 0;
	GLenum target = GL_RENDERBUFFER_EXT;
	GLenum format = GL_RGBA8;
	GLsizei xsize = 0;
	GLsizei ysize = 0;
	GLsizei samples = 0;
};

struct NativeFBO {
	GLuint id = 0;
	GLenum target = GL_FRAMEBUFFER_EXT;
	GLsizei xsize = 0;
	GLsizei ysize = 0;
};

static std::unordered_map<std::string, NativeTexture> nativeTextures;
static std::unordered_map<std::string, size_t> nativeAtlasMap;
static std::vector<CTextureAtlas> nativeAtlases;
static std::unordered_map<uint32_t, GLuint> nativeDisplayLists;
static std::unordered_map<uint32_t, GLuint> nativeQueries;
static std::unordered_map<uint32_t, NativeShaderProgram> nativeShaders;
static std::unordered_map<uint32_t, NativeRBO> nativeRBOs;
static std::unordered_map<uint32_t, NativeFBO> nativeFBOs;
static std::unordered_map<uint32_t, GLuint> nativeVAOs;
static std::unordered_map<uint32_t, GLuint> nativeVBOs;
static std::unordered_map<uint32_t, GLenum> nativeVBOTargets;
static std::unordered_map<uint32_t, std::shared_ptr<CglFont>> nativeFonts;
static uint32_t nativeTextureCounter = 0;
static uint32_t nativeAtlasCounter = 0;
static uint32_t nativeDisplayListCounter = 0;
static uint32_t nativeQueryCounter = 0;
static uint32_t nativeShaderCounter = 0;
static uint32_t nativeRBOCounter = 0;
static uint32_t nativeFBOCounter = 0;
static uint32_t nativeVAOCounter = 0;
static uint32_t nativeVBOCounter = 0;
static uint32_t nativeFontCounter = 0;
static uint32_t activeNativeShader = 0;
static thread_local std::string stringResult;
static thread_local std::string fontWrapResult;
static thread_local std::vector<float> readPixelsResult;
static thread_local std::vector<GfxAtlasTextureEntry> atlasTextureEntries;
static thread_local std::vector<std::string> activeUniformNames;
static thread_local std::vector<std::string> activeUniformTypes;
static thread_local std::vector<GfxActiveUniformEntry> activeUniformEntries;
static std::string nativeShaderLog;

static const char* UniformTypeString(GLenum type)
{
	switch (type) {
		case GL_FLOAT: return "float";
		case GL_FLOAT_VEC2: return "float_vec2";
		case GL_FLOAT_VEC3: return "float_vec3";
		case GL_FLOAT_VEC4: return "float_vec4";
		case GL_FLOAT_MAT2: return "float_mat2";
		case GL_FLOAT_MAT3: return "float_mat3";
		case GL_FLOAT_MAT4: return "float_mat4";
		case GL_SAMPLER_1D: return "sampler_1d";
		case GL_SAMPLER_2D: return "sampler_2d";
		case GL_SAMPLER_3D: return "sampler_3d";
		case GL_SAMPLER_CUBE: return "sampler_cube";
		case GL_SAMPLER_1D_SHADOW: return "sampler_1d_shadow";
		case GL_SAMPLER_2D_SHADOW: return "sampler_2d_shadow";
		case GL_INT: return "int";
		case GL_INT_VEC2: return "int_vec2";
		case GL_INT_VEC3: return "int_vec3";
		case GL_INT_VEC4: return "int_vec4";
		case GL_BOOL: return "bool";
		case GL_BOOL_VEC2: return "bool_vec2";
		case GL_BOOL_VEC3: return "bool_vec3";
		case GL_BOOL_VEC4: return "bool_vec4";
		default: return "unknown_type";
	}
}

static GLuint CompileShaderObject(const char* definitions, const char* source, GLenum type, bool* success)
{
	*success = true;

	if (source == nullptr || source[0] == '\0')
		return 0;

	GLuint obj = glCreateShader(type);
	if (obj == 0) {
		nativeShaderLog = "Could not create shader object";
		*success = false;
		return 0;
	}

	const GLchar* sources[2] = { definitions != nullptr ? definitions : "", source };
	glShaderSource(obj, 2, sources, nullptr);
	glCompileShader(obj);

	GLint status = GL_FALSE;
	glGetShaderiv(obj, GL_COMPILE_STATUS, &status);

	GLchar log[4096] = { 0 };
	GLsizei logSize = 0;
	glGetShaderInfoLog(obj, sizeof(log), &logSize, log);
	nativeShaderLog = log;

	if (status != GL_TRUE) {
		if (nativeShaderLog.empty())
			nativeShaderLog = "Shader compile failed without an info log";
		glDeleteShader(obj);
		*success = false;
		return 0;
	}

	return obj;
}

static void DeleteShaderProgram(NativeShaderProgram& program)
{
	if (program.id == 0)
		return;

	for (const NativeShaderObject& object: program.objects) {
		glDetachShader(program.id, object.id);
		glDeleteShader(object.id);
	}

	glDeleteProgram(program.id);
	program.objects.clear();
	program.id = 0;
}

static void ResetValueResult(GfxValueResult* result)
{
	result->error = nullptr;
	result->count = 0;
	result->boolValue = false;
	result->hasBool = false;
	result->stringValue = nullptr;
	std::fill(std::begin(result->values), std::end(result->values), 0.0f);
}

static const char* SafeKey(const GfxValueQuery* query)
{
	return (query != nullptr && query->key != nullptr) ? query->key : "";
}

static const char* SafeMode(const GfxValueQuery* query, const char* fallback)
{
	return (query != nullptr && query->mode != nullptr) ? query->mode : fallback;
}

static void SetFloats(GfxValueResult* result, const float* values, uint32_t count)
{
	result->count = std::min<uint32_t>(count, 4);
	for (uint32_t i = 0; i < result->count; ++i) {
		result->values[i] = values[i];
	}
}

static void SetFloat(GfxValueResult* result, float value)
{
	result->values[0] = value;
	result->count = 1;
}

static void SetBool(GfxValueResult* result, bool value)
{
	result->boolValue = value;
	result->hasBool = true;
	result->count = 1;
}

static uint32_t MatrixModeToPName(uint32_t mode)
{
	switch (mode) {
		case GL_MODELVIEW: return GL_MODELVIEW_MATRIX;
		case GL_PROJECTION: return GL_PROJECTION_MATRIX;
		case GL_TEXTURE: return GL_TEXTURE_MATRIX;
		default: return mode;
	}
}

static bool IsValidTextureUnit(int32_t texNum)
{
	return texNum >= 0 && texNum < CGlobalRendering::MAX_TEXTURE_UNITS;
}

static bool IsValidNativeTextureTarget(GLenum target)
{
	switch (target) {
		case GL_TEXTURE_1D:
		case GL_TEXTURE_2D:
		case GL_TEXTURE_3D:
		case GL_TEXTURE_2D_ARRAY:
		case GL_TEXTURE_CUBE_MAP:
		case GL_TEXTURE_2D_MULTISAMPLE:
			return true;
		default:
			return false;
	}
}

static NativeTexture TextureFromParams(const GfxCreateTextureQuery* query)
{
	NativeTexture tex;
	tex.target = query->params.target != 0 ? query->params.target : GL_TEXTURE_2D;
	tex.format = query->params.format != 0 ? query->params.format : GL_RGBA8;
	tex.xsize = query->xsize;
	tex.ysize = query->ysize;
	tex.zsize = query->zsize;
	tex.border = query->params.border;
	tex.minFilter = query->params.minFilter != 0 ? query->params.minFilter : GL_LINEAR;
	tex.magFilter = query->params.magFilter != 0 ? query->params.magFilter : GL_LINEAR;
	tex.wrapS = query->params.wrapS != 0 ? query->params.wrapS : GL_REPEAT;
	tex.wrapT = query->params.wrapT != 0 ? query->params.wrapT : GL_REPEAT;
	tex.wrapR = query->params.wrapR != 0 ? query->params.wrapR : GL_REPEAT;
	tex.compareFunc = query->params.compareFunc;
	tex.lodBias = query->params.lodBias;
	tex.aniso = query->params.aniso;
	tex.samples = query->params.samples;
	return tex;
}

static void ApplyTextureParams(const NativeTexture& tex)
{
	glTexParameteri(tex.target, GL_TEXTURE_WRAP_S, tex.wrapS);
	glTexParameteri(tex.target, GL_TEXTURE_WRAP_T, tex.wrapT);
	glTexParameteri(tex.target, GL_TEXTURE_WRAP_R, tex.wrapR);
	glTexParameteri(tex.target, GL_TEXTURE_MIN_FILTER, tex.minFilter);
	glTexParameteri(tex.target, GL_TEXTURE_MAG_FILTER, tex.magFilter);

	if (tex.compareFunc != GL_NONE && tex.compareFunc != 0) {
		glTexParameteri(tex.target, GL_TEXTURE_COMPARE_MODE, GL_COMPARE_REF_TO_TEXTURE);
		glTexParameteri(tex.target, GL_TEXTURE_COMPARE_FUNC, tex.compareFunc);
	} else {
		glTexParameteri(tex.target, GL_TEXTURE_COMPARE_MODE, GL_NONE);
		glTexParameteri(tex.target, GL_TEXTURE_COMPARE_FUNC, GL_LEQUAL);
	}

	if (tex.lodBias != 0.0f)
		glTexParameterf(tex.target, GL_TEXTURE_LOD_BIAS, tex.lodBias);

	if (tex.aniso != 0.0f && GLAD_GL_EXT_texture_filter_anisotropic && globalRendering != nullptr)
		glTexParameterf(tex.target, GL_TEXTURE_MAX_ANISOTROPY_EXT, std::clamp(tex.aniso, 1.0f, globalRendering->maxTexAnisoLvl));
}

static NativeTexture* GetNativeTexture(const char* name)
{
	if (name == nullptr)
		return nullptr;

	const auto it = nativeTextures.find(name);
	return (it != nativeTextures.end()) ? &it->second : nullptr;
}

extern "C" bool GetNativeGfxTextureInfo(const char* name, uint32_t* id, int32_t* xsize, int32_t* ysize, uint32_t* target)
{
	NativeTexture* tex = GetNativeTexture(name);
	if (tex == nullptr)
		return false;

	if (id != nullptr) *id = tex->id;
	if (xsize != nullptr) *xsize = tex->xsize;
	if (ysize != nullptr) *ysize = tex->ysize;
	if (target != nullptr) *target = tex->target;
	return true;
}

static CTextureAtlas* GetNativeAtlas(const char* name)
{
	if (name == nullptr)
		return nullptr;

	const auto it = nativeAtlasMap.find(name);
	return (it != nativeAtlasMap.end()) ? &nativeAtlases[it->second] : nullptr;
}

struct ResolvedTexture {
	GLuint id = 0;
	GLenum target = 0;
	GLsizei xsize = 0;
	GLsizei ysize = 0;
	GLsizei zsize = 0;
	NativeTexture* native = nullptr;
	CTextureAtlas* atlas = nullptr;
};

static bool ResolveTexture(const char* name, ResolvedTexture& resolved)
{
	resolved = {};

	if (NativeTexture* tex = GetNativeTexture(name); tex != nullptr) {
		resolved.id = tex->id;
		resolved.target = tex->target;
		resolved.xsize = tex->xsize;
		resolved.ysize = tex->ysize;
		resolved.zsize = tex->zsize;
		resolved.native = tex;
		return true;
	}

	if (CTextureAtlas* atlas = GetNativeAtlas(name); atlas != nullptr) {
		const int2 atlasSize = atlas->GetSize();
		resolved.id = atlas->GetTexID();
		resolved.target = atlas->GetTexTarget();
		resolved.xsize = atlasSize.x;
		resolved.ysize = atlasSize.y;
		resolved.zsize = atlas->GetNumPages();
		resolved.atlas = atlas;
		return true;
	}

	LuaMatTexture matTex;
	if (!LuaOpenGLUtils::ParseTextureImage(nullptr, matTex, name != nullptr ? name : ""))
		return false;

	const auto [xsize, ysize, zsize] = matTex.GetSize();
	resolved.id = matTex.GetTextureID();
	resolved.target = matTex.GetTextureTarget();
	resolved.xsize = xsize;
	resolved.ysize = ysize;
	resolved.zsize = zsize;
	return true;
}

static GLenum GetFBOBindingEnum(GLenum target)
{
	switch (target) {
		case GL_FRAMEBUFFER_EXT: return GL_FRAMEBUFFER_BINDING_EXT;
		case GL_DRAW_FRAMEBUFFER_EXT: return GL_FRAMEBUFFER_BINDING_EXT;
		case GL_READ_FRAMEBUFFER_EXT: return GL_FRAMEBUFFER_BINDING_EXT;
		default: return 0;
	}
}

static NativeFBO* GetNativeFBO(uint32_t fboID)
{
	const auto it = nativeFBOs.find(fboID);
	return (it != nativeFBOs.end()) ? &it->second : nullptr;
}

static NativeRBO* GetNativeRBO(uint32_t rboID)
{
	const auto it = nativeRBOs.find(rboID);
	return (it != nativeRBOs.end()) ? &it->second : nullptr;
}

static bool DeleteNativeAtlas(const char* name)
{
	if (name == nullptr)
		return false;

	const auto it = nativeAtlasMap.find(name);
	if (it == nativeAtlasMap.end())
		return false;

	const size_t index = it->second;
	if (index != nativeAtlases.size() - 1) {
		nativeAtlasMap[nativeAtlases.back().GetName()] = index;
		std::swap(nativeAtlases[index], nativeAtlases.back());
	}

	nativeAtlases.pop_back();
	nativeAtlasMap.erase(it);
	return true;
}

static void DeleteNativeTexture(NativeTexture& tex)
{
	if (tex.id != 0)
		glDeleteTextures(1, &tex.id);

	if (FBO::IsSupported()) {
		if (tex.fbo != 0)
			glDeleteFramebuffersEXT(1, &tex.fbo);
		if (tex.fboDepth != 0)
			glDeleteRenderbuffersEXT(1, &tex.fboDepth);
	}
}

static void SetTexGenState(uint32_t target, bool state)
{
	if (target < GL_S || target > GL_Q)
		return;

	const GLenum pname = GL_TEXTURE_GEN_S + (target - GL_S);
	state ? glEnable(pname) : glDisable(pname);
}

static void ResetGLState()
{
	glDisable(GL_DEPTH_TEST);
	glDepthFunc(GL_LEQUAL);
	glDepthMask(GL_FALSE);
	if (GLAD_GL_ARB_depth_clamp)
		glDisable(GL_DEPTH_CLAMP);

	glColorMask(GL_TRUE, GL_TRUE, GL_TRUE, GL_TRUE);
	glEnable(GL_BLEND);
	if (IS_GL_FUNCTION_AVAILABLE(glBlendEquation))
		glBlendEquation(GL_FUNC_ADD);
	glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

	glDisable(GL_ALPHA_TEST);
	glAlphaFunc(GL_GREATER, 0.5f);
	glDisable(GL_LIGHTING);
	glShadeModel(GL_SMOOTH);
	glDisable(GL_COLOR_LOGIC_OP);
	glLogicOp(GL_INVERT);
	glDisable(GL_CULL_FACE);
	glCullFace(GL_BACK);
	glDisable(GL_SCISSOR_TEST);
	glDisable(GL_STENCIL_TEST);
	glStencilMask(~0);
	if (GLAD_GL_EXT_stencil_two_side)
		glDisable(GL_STENCIL_TEST_TWO_SIDE_EXT);

	glDisable(GL_TEXTURE_2D);
	glDisable(GL_TEXTURE_GEN_S);
	glDisable(GL_TEXTURE_GEN_T);
	glDisable(GL_TEXTURE_GEN_R);
	glDisable(GL_TEXTURE_GEN_Q);
	glTexEnvi(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE);
	glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
	glDisable(GL_POLYGON_OFFSET_FILL);
	glDisable(GL_POLYGON_OFFSET_LINE);
	glDisable(GL_POLYGON_OFFSET_POINT);
	glDisable(GL_LINE_STIPPLE);
	glDisable(GL_CLIP_PLANE4);
	glDisable(GL_CLIP_PLANE5);
	glLineWidth(1.0f);
	glPointSize(1.0f);
	glDisable(GL_POINT_SPRITE);

	const GLfloat atten[3] = { 1.0f, 0.0f, 0.0f };
	glPointParameterfv(GL_POINT_DISTANCE_ATTENUATION, atten);
	glPointParameterf(GL_POINT_SIZE_MIN, 0.0f);
	glPointParameterf(GL_POINT_SIZE_MAX, 1.0e9f);
	glPointParameterf(GL_POINT_FADE_THRESHOLD_SIZE, 1.0f);

	glColor4f(1.0f, 1.0f, 1.0f, 1.0f);
	const float ambient[4] = { 0.2f, 0.2f, 0.2f, 1.0f };
	const float diffuse[4] = { 0.8f, 0.8f, 0.8f, 1.0f };
	const float black[4] = { 0.0f, 0.0f, 0.0f, 1.0f };
	glMaterialfv(GL_FRONT_AND_BACK, GL_AMBIENT, ambient);
	glMaterialfv(GL_FRONT_AND_BACK, GL_DIFFUSE, diffuse);
	glMaterialfv(GL_FRONT_AND_BACK, GL_EMISSION, black);
	glMaterialfv(GL_FRONT_AND_BACK, GL_SPECULAR, black);
	glMaterialf(GL_FRONT_AND_BACK, GL_SHININESS, 0.0f);

	if (IS_GL_FUNCTION_AVAILABLE(glUseProgram))
		glUseProgram(0);
}

static bool SkyReady(GfxValueResult* result)
{
	if (ISky::GetSky() != nullptr && ISky::GetSky()->GetLight() != nullptr)
		return true;

	result->error = &NOT_READY_ERROR;
	return false;
}

static void HasExtension(const GfxStringQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;

	if (globalRendering == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->value = globalRendering->IsExtensionSupported(query->value);
}

static void GetNumber(const GfxGetNumberQuery* query, GfxGetNumberResult* result)
{
	result->error = nullptr;
	result->count = std::min<uint32_t>(std::max<uint32_t>(query->maxValues, 1), 16);
	std::fill(std::begin(result->values), std::end(result->values), 0.0f);
	glGetFloatv(query->pname, result->values);
}

static void GetString(const GfxGetStringQuery* query, GfxStringResult* result)
{
	result->error = nullptr;
	const GLubyte* value = glGetString(query->pname);
	result->value = (value != nullptr) ? reinterpret_cast<const char*>(value) : "[NULL]";
}

static void GetViewSizes(const GfxEmptyQuery*, GfxViewSizesResult* result)
{
	result->error = nullptr;

	if (globalRendering == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->viewSizeX = 0;
		result->viewSizeY = 0;
		return;
	}

	result->viewSizeX = globalRendering->viewSizeX;
	result->viewSizeY = globalRendering->viewSizeY;
}

static void GetViewRange(const GfxViewRangeQuery* query, GfxViewRangeResult* result)
{
	result->error = nullptr;

	if (globalRendering == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const int camType = std::clamp<int>(query->cameraType, CCamera::CAMTYPE_PLAYER, CCamera::CAMTYPE_ACTIVE);
	const CCamera* cam = CCameraHandler::GetCamera(camType);

	if (cam == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->nearPlaneDist = cam->GetNearPlaneDist();
	result->farPlaneDist = cam->GetFarPlaneDist();
	result->minViewRange = globalRendering->minViewRange;
	result->maxViewRange = globalRendering->maxViewRange;
}

static void GetShadowMapParams(const GfxEmptyQuery*, GfxShadowMapParamsResult* result)
{
	result->error = nullptr;
	const float4& params = shadowHandler.GetShadowParams();
	result->params = { params.x, params.y, params.z, params.w };
}

static void GetAtmosphere(const GfxValueQuery* query, GfxValueResult* result)
{
	ResetValueResult(result);
	if (!SkyReady(result))
		return;

	const auto& sky = ISky::GetSky();
	const char* key = SafeKey(query);

	if (key[0] == '\0' || std::strcmp(key, "pos") == 0) {
		SetFloats(result, &sky->GetLight()->GetLightDir().x, 3);
		return;
	}

	switch (hashString(key)) {
		case hashString("fogStart"): SetFloat(result, sky->fogStart); return;
		case hashString("fogEnd"): SetFloat(result, sky->fogEnd); return;
		case hashString("fogColor"): SetFloats(result, &sky->fogColor.x, 4); return;
		case hashString("skyColor"): SetFloats(result, &sky->skyColor.x, 3); return;
		case hashString("sunColor"): SetFloats(result, &sky->sunColor.x, 3); return;
		case hashString("cloudColor"): SetFloats(result, &sky->cloudColor.x, 3); return;
		case hashString("skyAxisAngle"): SetFloats(result, &sky->GetSkyAxisAngle().x, 4); return;
		default: result->error = &INVALID_ARGUMENT_ERROR; return;
	}
}

static void GetSun(const GfxValueQuery* query, GfxValueResult* result)
{
	ResetValueResult(result);
	if (!SkyReady(result))
		return;

	const auto& sky = ISky::GetSky();
	const char* key = SafeKey(query);
	const char* mode = SafeMode(query, "ground");
	const bool model = (mode[0] == 'u');

	if (key[0] == '\0' || std::strcmp(key, "pos") == 0 || std::strcmp(key, "dir") == 0) {
		SetFloats(result, &sky->GetLight()->GetLightDir().x, 3);
		return;
	}

	switch (hashString(key)) {
		case hashString("specularExponent"): SetFloat(result, sunLighting->specularExponent); return;
		case hashString("shadowDensity"): SetFloat(result, model ? sunLighting->modelShadowDensity : sunLighting->groundShadowDensity); return;
		case hashString("diffuse"): SetFloats(result, model ? &sunLighting->modelDiffuseColor.x : &sunLighting->groundDiffuseColor.x, 3); return;
		case hashString("ambient"): SetFloats(result, model ? &sunLighting->modelAmbientColor.x : &sunLighting->groundAmbientColor.x, 3); return;
		case hashString("specular"): SetFloats(result, model ? &sunLighting->modelSpecularColor.x : &sunLighting->groundSpecularColor.x, 3); return;
		default: result->error = &INVALID_ARGUMENT_ERROR; return;
	}
}

static void GetWaterRendering(const GfxValueQuery* query, GfxValueResult* result)
{
	ResetValueResult(result);
	const char* key = SafeKey(query);

	switch (hashString(key)) {
		case hashString("absorb"): SetFloats(result, &waterRendering->absorb.x, 3); return;
		case hashString("baseColor"): SetFloats(result, &waterRendering->baseColor.x, 3); return;
		case hashString("minColor"): SetFloats(result, &waterRendering->minColor.x, 3); return;
		case hashString("surfaceColor"): SetFloats(result, &waterRendering->surfaceColor.x, 3); return;
		case hashString("diffuseColor"): SetFloats(result, &waterRendering->diffuseColor.x, 3); return;
		case hashString("specularColor"): SetFloats(result, &waterRendering->specularColor.x, 3); return;
		case hashString("planeColor"): SetFloats(result, &waterRendering->planeColor.x, 3); return;
		case hashString("texture"): result->stringValue = waterRendering->texture.c_str(); return;
		case hashString("foamTexture"): result->stringValue = waterRendering->foamTexture.c_str(); return;
		case hashString("normalTexture"): result->stringValue = waterRendering->normalTexture.c_str(); return;
		case hashString("repeatX"): SetFloat(result, waterRendering->repeatX); return;
		case hashString("repeatY"): SetFloat(result, waterRendering->repeatY); return;
		case hashString("surfaceAlpha"): SetFloat(result, waterRendering->surfaceAlpha); return;
		case hashString("ambientFactor"): SetFloat(result, waterRendering->ambientFactor); return;
		case hashString("diffuseFactor"): SetFloat(result, waterRendering->diffuseFactor); return;
		case hashString("specularFactor"): SetFloat(result, waterRendering->specularFactor); return;
		case hashString("specularPower"): SetFloat(result, waterRendering->specularPower); return;
		case hashString("fresnelMin"): SetFloat(result, waterRendering->fresnelMin); return;
		case hashString("fresnelMax"): SetFloat(result, waterRendering->fresnelMax); return;
		case hashString("fresnelPower"): SetFloat(result, waterRendering->fresnelPower); return;
		case hashString("reflectionDistortion"): SetFloat(result, waterRendering->reflDistortion); return;
		case hashString("blurBase"): SetFloat(result, waterRendering->blurBase); return;
		case hashString("blurExponent"): SetFloat(result, waterRendering->blurExponent); return;
		case hashString("perlinStartFreq"): SetFloat(result, waterRendering->perlinStartFreq); return;
		case hashString("perlinLacunarity"): SetFloat(result, waterRendering->perlinLacunarity); return;
		case hashString("perlinAmplitude"): SetFloat(result, waterRendering->perlinAmplitude); return;
		case hashString("windSpeed"): SetFloat(result, waterRendering->windSpeed); return;
		case hashString("waveOffsetFactor"): SetFloat(result, waterRendering->waveOffsetFactor); return;
		case hashString("waveLength"): SetFloat(result, waterRendering->waveLength); return;
		case hashString("waveFoamDistortion"): SetFloat(result, waterRendering->waveFoamDistortion); return;
		case hashString("waveFoamIntensity"): SetFloat(result, waterRendering->waveFoamIntensity); return;
		case hashString("causticsResolution"): SetFloat(result, waterRendering->causticsResolution); return;
		case hashString("causticsStrength"): SetFloat(result, waterRendering->causticsStrength); return;
		case hashString("numTiles"): SetFloat(result, waterRendering->numTiles); return;
		case hashString("shoreWaves"): SetBool(result, waterRendering->shoreWaves); return;
		case hashString("forceRendering"): SetBool(result, waterRendering->forceRendering); return;
		case hashString("hasWaterPlane"): SetBool(result, waterRendering->hasWaterPlane); return;
		default: result->error = &INVALID_ARGUMENT_ERROR; return;
	}
}

static void GetMapRendering(const GfxValueQuery* query, GfxValueResult* result)
{
	ResetValueResult(result);
	const char* key = SafeKey(query);

	switch (hashString(key)) {
		case hashString("splatTexScales"): SetFloats(result, &mapRendering->splatTexScales.x, 4); return;
		case hashString("splatTexMults"): SetFloats(result, &mapRendering->splatTexMults.x, 4); return;
		case hashString("voidWater"): SetBool(result, mapRendering->voidWater); return;
		case hashString("voidGround"): SetBool(result, mapRendering->voidGround); return;
		case hashString("splatDetailNormalDiffuseAlpha"): SetBool(result, mapRendering->splatDetailNormalDiffuseAlpha); return;
		default: result->error = &INVALID_ARGUMENT_ERROR; return;
	}
}

static void ResetState(const GfxEmptyQuery*, GfxEmptyResult* result) { result->error = nullptr; ResetGLState(); }
static void Clear(const GfxClearQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->count == 1) {
		switch (query->bits) {
			case GL_DEPTH_BUFFER_BIT: glClearDepth(query->values[0]); break;
			case GL_STENCIL_BUFFER_BIT: glClearStencil(static_cast<GLint>(query->values[0])); break;
			default: break;
		}
	} else if (query->count >= 4) {
		switch (query->bits) {
			case GL_COLOR_BUFFER_BIT: glClearColor(query->values[0], query->values[1], query->values[2], query->values[3]); break;
			case GL_ACCUM_BUFFER_BIT: glClearAccum(query->values[0], query->values[1], query->values[2], query->values[3]); break;
			default: break;
		}
	}

	glClear(query->bits);
}
static void Flush(const GfxEmptyQuery*, GfxEmptyResult* result) { result->error = nullptr; glFlush(); }
static void Finish(const GfxEmptyQuery*, GfxEmptyResult* result) { result->error = nullptr; glFinish(); }
static void SwapBuffers(const GfxEmptyQuery*, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (globalRendering == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	globalRendering->SwapBuffers(true, true);
}

static void ResetMatrices(const GfxEmptyQuery*, GfxEmptyResult* result)
{
	result->error = nullptr;
	glMatrixMode(GL_TEXTURE);
	glLoadIdentity();
	glMatrixMode(GL_PROJECTION);
	glLoadIdentity();
	glMatrixMode(GL_MODELVIEW);
	glLoadIdentity();
}

static void DepthTest(const GfxDepthTestQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	query->enable ? glEnable(GL_DEPTH_TEST) : glDisable(GL_DEPTH_TEST);
	if (query->setFunc)
		glDepthFunc(query->func);
}

static void DepthMask(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; glDepthMask(query->value ? GL_TRUE : GL_FALSE); }
static void Culling(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->value ? glEnable(GL_CULL_FACE) : glDisable(GL_CULL_FACE); }
static void Blending(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->value ? glEnable(GL_BLEND) : glDisable(GL_BLEND); }
static void BlendFunc(const GfxBlendFuncQuery* query, GfxEmptyResult* result) { result->error = nullptr; glBlendFunc(query->src, query->dst); }
static void BlendFuncSeparate(const GfxBlendFuncSeparateQuery* query, GfxEmptyResult* result) { result->error = nullptr; glBlendFuncSeparate(query->srcRGB, query->dstRGB, query->srcAlpha, query->dstAlpha); }
static void BlendEquation(const GfxBlendEquationQuery* query, GfxEmptyResult* result) { result->error = nullptr; glBlendEquation(query->mode); }
static void BlendEquationSeparate(const GfxBlendEquationSeparateQuery* query, GfxEmptyResult* result) { result->error = nullptr; glBlendEquationSeparate(query->modeRGB, query->modeAlpha); }
static void ColorMask(const GfxColorMaskQuery* query, GfxEmptyResult* result) { result->error = nullptr; glColorMask(query->red, query->green, query->blue, query->alpha); }
static void AlphaTest(const GfxAlphaTestQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->enable ? glEnable(GL_ALPHA_TEST) : glDisable(GL_ALPHA_TEST); glAlphaFunc(query->func, query->ref); }
static void AlphaToCoverage(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->value ? glEnable(GL_SAMPLE_ALPHA_TO_COVERAGE) : glDisable(GL_SAMPLE_ALPHA_TO_COVERAGE); }
static void StencilTest(const GfxStencilTestQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->enable ? glEnable(GL_STENCIL_TEST) : glDisable(GL_STENCIL_TEST); }
static void StencilFunc(const GfxStencilFuncQuery* query, GfxEmptyResult* result) { result->error = nullptr; glStencilFunc(query->func, query->ref, query->mask); }
static void StencilFuncSeparate(const GfxStencilFuncSeparateQuery* query, GfxEmptyResult* result) { result->error = nullptr; glStencilFuncSeparate(query->face, query->func, query->ref, query->mask); }
static void StencilMask(const GfxStencilMaskQuery* query, GfxEmptyResult* result) { result->error = nullptr; glStencilMask(query->mask); }
static void StencilMaskSeparate(const GfxStencilMaskSeparateQuery* query, GfxEmptyResult* result) { result->error = nullptr; glStencilMaskSeparate(query->face, query->mask); }
static void StencilOp(const GfxStencilOpQuery* query, GfxEmptyResult* result) { result->error = nullptr; glStencilOp(query->fail, query->zfail, query->zpass); }
static void StencilOpSeparate(const GfxStencilOpSeparateQuery* query, GfxEmptyResult* result) { result->error = nullptr; glStencilOpSeparate(query->face, query->fail, query->zfail, query->zpass); }
static void PolygonMode(const GfxPolygonModeQuery* query, GfxEmptyResult* result) { result->error = nullptr; glPolygonMode(query->face, query->mode); }
static void PolygonOffset(const GfxPolygonOffsetQuery* query, GfxEmptyResult* result) { result->error = nullptr; glPolygonOffset(query->factor, query->units); }
static void LogicOp(const GfxLogicOpQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->enable ? glEnable(GL_COLOR_LOGIC_OP) : glDisable(GL_COLOR_LOGIC_OP); glLogicOp(query->opcode); }
static void ShadeModel(const GfxShadeModelQuery* query, GfxEmptyResult* result) { result->error = nullptr; glShadeModel(query->mode); }
static void Scissor(const GfxScissorQuery* query, GfxEmptyResult* result) { result->error = nullptr; glScissor(query->x, query->y, query->width, query->height); }
static void Viewport(const GfxViewportQuery* query, GfxEmptyResult* result) { result->error = nullptr; glViewport(query->x, query->y, query->width, query->height); }
static void LineWidth(const GfxFloatQuery* query, GfxEmptyResult* result) { result->error = nullptr; glLineWidth(query->value); }
static void LineStipple(const GfxLineStippleQuery* query, GfxEmptyResult* result) { result->error = nullptr; glLineStipple(query->factor, query->pattern); }
static void PointSize(const GfxFloatQuery* query, GfxEmptyResult* result) { result->error = nullptr; glPointSize(query->value); }
static void PointSprite(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->value ? glEnable(GL_POINT_SPRITE) : glDisable(GL_POINT_SPRITE); }

static void PointParameter(const GfxPointParameterQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (query->count <= 1) {
		glPointParameterf(query->pname, query->value);
	} else {
		glPointParameterfv(query->pname, query->values);
	}
}

static void ClipPlane(const GfxClipPlaneQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const GLdouble equation[4] = {
		query->equation[0],
		query->equation[1],
		query->equation[2],
		query->equation[3],
	};
	glClipPlane(query->plane, equation);
}

static void ClipDistance(const GfxClipDistanceQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const uint32_t cap = GL_CLIP_DISTANCE0 + query->index;
	query->enable ? glEnable(cap) : glDisable(cap);
}

static void PushAttrib(const GfxUIntQuery* query, GfxEmptyResult* result) { result->error = nullptr; glPushAttrib(query->value); }
static void PopAttrib(const GfxEmptyQuery*, GfxEmptyResult* result) { result->error = nullptr; glPopAttrib(); }
static void DepthClamp(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->value ? glEnable(GL_DEPTH_CLAMP) : glDisable(GL_DEPTH_CLAMP); }
static void Fog(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->value ? glEnable(GL_FOG) : glDisable(GL_FOG); }
static void Lighting(const GfxBoolQuery* query, GfxEmptyResult* result) { result->error = nullptr; query->value ? glEnable(GL_LIGHTING) : glDisable(GL_LIGHTING); }

static void Light(const GfxLightQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const GLenum light = GL_LIGHT0 + query->light;

	if (light < GL_LIGHT0 || light > GL_LIGHT7) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	if (query->setState) {
		query->state ? glEnable(light) : glDisable(light);
		return;
	}

	if (query->count <= 1) {
		glLightf(light, query->pname, query->values[0]);
	} else {
		glLightfv(light, query->pname, query->values);
	}
}

static void Material(const GfxMaterialQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->pname == GL_SHININESS || query->count <= 1) {
		glMaterialf(GL_FRONT_AND_BACK, query->pname, query->values[0]);
	} else {
		glMaterialfv(GL_FRONT_AND_BACK, query->pname, query->values);
	}
}

static void TexEnv(const GfxTexEnvQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (query->count <= 1) {
		glTexEnvf(query->target, query->pname, query->values[0]);
	} else {
		glTexEnvfv(query->target, query->pname, query->values);
	}
}

static void MultiTexEnv(const GfxMultiTexEnvQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (!IsValidTextureUnit(query->texNum)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	glActiveTexture(GL_TEXTURE0 + query->texNum);
	if (query->count <= 1) {
		glTexEnvf(query->target, query->pname, query->values[0]);
	} else {
		glTexEnvfv(query->target, query->pname, query->values);
	}
	glActiveTexture(GL_TEXTURE0);
}

static void TexGen(const GfxTexGenQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->setState) {
		SetTexGenState(query->target, query->state);
		return;
	}

	if (query->count <= 1) {
		glTexGenf(query->target, query->pname, query->values[0]);
	} else {
		glTexGenfv(query->target, query->pname, query->values);
	}
	SetTexGenState(query->target, true);
}

static void MultiTexGen(const GfxMultiTexGenQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (!IsValidTextureUnit(query->texNum)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	glActiveTexture(GL_TEXTURE0 + query->texNum);
	if (query->setState) {
		SetTexGenState(query->target, query->state);
	} else if (query->count <= 1) {
		glTexGenf(query->target, query->pname, query->values[0]);
		SetTexGenState(query->target, true);
	} else {
		glTexGenfv(query->target, query->pname, query->values);
		SetTexGenState(query->target, true);
	}
	glActiveTexture(GL_TEXTURE0);
}

static void DispatchCompute(const GfxDispatchComputeQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glDispatchCompute(query->numGroupX, query->numGroupY, query->numGroupZ);
	if (query->barriers != 0)
		glMemoryBarrier(query->barriers);
}

static void MemoryBarrier(const GfxMemoryBarrierQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (query->barriers != 0)
		glMemoryBarrier(query->barriers);
}

static void ActiveTexture(const GfxActiveTextureQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (!IsValidTextureUnit(query->texNum)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	glActiveTexture(GL_TEXTURE0 + query->texNum);
}

static void ObjectLabel(const GfxObjectLabelQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	switch (query->identifier) {
		case GL_BUFFER:
		case GL_SHADER:
		case GL_PROGRAM:
		case GL_VERTEX_ARRAY:
		case GL_QUERY:
		case GL_PROGRAM_PIPELINE:
		case GL_TRANSFORM_FEEDBACK:
		case GL_TEXTURE:
		case GL_RENDERBUFFER:
		case GL_FRAMEBUFFER:
			break;
		default:
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
	}

	glObjectLabel(query->identifier, query->objectID, -1, query->label != nullptr ? query->label : "");
}

static void PushDebugGroup(const GfxPushDebugGroupQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	GLint maxLength = 0;
	glGetIntegerv(GL_MAX_DEBUG_MESSAGE_LENGTH, &maxLength);
	if (maxLength <= 0)
		return;

	std::string message = query->message != nullptr ? query->message : "";
	if (message.size() >= static_cast<size_t>(maxLength)) {
		static constexpr const char* trim = "(...)";
		message.resize(maxLength - std::strlen(trim) - 1);
		message += trim;
	}

	glPushDebugGroup(query->sourceIsThirdParty ? GL_DEBUG_SOURCE_THIRD_PARTY : GL_DEBUG_SOURCE_APPLICATION, query->id, -1, message.c_str());
}

static void PopDebugGroup(const GfxEmptyQuery*, GfxEmptyResult* result)
{
	result->error = nullptr;
	glPopDebugGroup();
}

static void CreateShader(const GfxCreateShaderQuery* query, GfxCreateShaderResult* result)
{
	result->error = nullptr;
	result->shaderID = 0;
	result->glProgramID = 0;
	nativeShaderLog.clear();

	const bool graphicsEmpty =
		(query->vertex == nullptr || query->vertex[0] == '\0') &&
		(query->tcs == nullptr || query->tcs[0] == '\0') &&
		(query->tes == nullptr || query->tes[0] == '\0') &&
		(query->geometry == nullptr || query->geometry[0] == '\0') &&
		(query->fragment == nullptr || query->fragment[0] == '\0');
	const bool computeEmpty = query->compute == nullptr || query->compute[0] == '\0';

	if ((graphicsEmpty && computeEmpty) || (!graphicsEmpty && !computeEmpty)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	bool success = true;
	NativeShaderProgram program;
	const char* definitions = query->definitions != nullptr ? query->definitions : "";

	constexpr struct { GLenum type; const char* GfxCreateShaderQuery::*source; } stages[] = {
		{ GL_VERTEX_SHADER, &GfxCreateShaderQuery::vertex },
		{ GL_TESS_CONTROL_SHADER, &GfxCreateShaderQuery::tcs },
		{ GL_TESS_EVALUATION_SHADER, &GfxCreateShaderQuery::tes },
		{ GL_GEOMETRY_SHADER, &GfxCreateShaderQuery::geometry },
		{ GL_FRAGMENT_SHADER, &GfxCreateShaderQuery::fragment },
		{ GL_COMPUTE_SHADER, &GfxCreateShaderQuery::compute },
	};

	for (const auto& stage: stages) {
		const GLuint objectID = CompileShaderObject(definitions, query->*(stage.source), stage.type, &success);
		if (!success) {
			DeleteShaderProgram(program);
			result->error = &OPERATION_FAILED_ERROR;
			return;
		}
		if (objectID != 0)
			program.objects.push_back({ objectID, stage.type });
	}

	program.id = glCreateProgram();
	if (program.id == 0) {
		nativeShaderLog = "Could not create shader program";
		DeleteShaderProgram(program);
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	for (const NativeShaderObject& object: program.objects)
		glAttachShader(program.id, object.id);

	if (query->hasGeoInputType && IS_GL_FUNCTION_AVAILABLE(glProgramParameteriEXT))
		glProgramParameteriEXT(program.id, GL_GEOMETRY_INPUT_TYPE_EXT, query->geoInputType);
	if (query->hasGeoOutputType && IS_GL_FUNCTION_AVAILABLE(glProgramParameteriEXT))
		glProgramParameteriEXT(program.id, GL_GEOMETRY_OUTPUT_TYPE_EXT, query->geoOutputType);
	if (query->hasGeoOutputVerts && IS_GL_FUNCTION_AVAILABLE(glProgramParameteriEXT))
		glProgramParameteriEXT(program.id, GL_GEOMETRY_VERTICES_OUT_EXT, query->geoOutputVerts);

	glLinkProgram(program.id);
	GLint linkStatus = GL_FALSE;
	glGetProgramiv(program.id, GL_LINK_STATUS, &linkStatus);

	glValidateProgram(program.id);
	GLint validStatus = GL_FALSE;
	glGetProgramiv(program.id, GL_VALIDATE_STATUS, &validStatus);

	if (linkStatus != GL_TRUE || validStatus != GL_TRUE) {
		GLchar log[4096] = { 0 };
		GLsizei logSize = 0;
		glGetProgramInfoLog(program.id, sizeof(log), &logSize, log);
		nativeShaderLog = log;
		DeleteShaderProgram(program);
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	const uint32_t shaderID = ++nativeShaderCounter;
	result->shaderID = shaderID;
	result->glProgramID = program.id;
	nativeShaders[shaderID] = std::move(program);
}

static void DeleteShader(const GfxShaderQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;

	auto it = nativeShaders.find(query->shaderID);
	if (it == nativeShaders.end())
		return;

	if (activeNativeShader == query->shaderID) {
		glUseProgram(0);
		activeNativeShader = 0;
	}

	DeleteShaderProgram(it->second);
	nativeShaders.erase(it);
	result->value = true;
}

static void UseShader(const GfxShaderQuery* query, GfxUseShaderResult* result)
{
	result->error = nullptr;
	result->linked = false;

	if (query->shaderID == 0) {
		glUseProgram(0);
		activeNativeShader = 0;
		result->linked = true;
		return;
	}

	const auto it = nativeShaders.find(query->shaderID);
	if (it == nativeShaders.end())
		return;

	glUseProgram(it->second.id);
	activeNativeShader = query->shaderID;
	result->linked = true;
}

static void ActiveShader(const GfxActiveShaderQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint currentProgram = 0;
	glGetIntegerv(GL_CURRENT_PROGRAM, &currentProgram);
	const uint32_t previousNativeShader = activeNativeShader;

	GLuint programID = 0;
	if (query->shaderID != 0) {
		const auto it = nativeShaders.find(query->shaderID);
		if (it == nativeShaders.end()) {
			result->error = &NOT_FOUND_ERROR;
			return;
		}

		programID = it->second.id;
	}

	glUseProgram(programID);
	activeNativeShader = query->shaderID;
	query->callback(query->userData);
	activeNativeShader = previousNativeShader;
	glUseProgram(currentProgram);
}

static void GetShaderLog(const GfxEmptyQuery*, GfxStringResult* result)
{
	result->error = nullptr;
	result->value = nativeShaderLog.c_str();
}

static void GetUniformLocation(const GfxUniformLocationQuery* query, GfxUniformLocationResult* result)
{
	result->error = nullptr;
	result->location = -1;

	if (query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const auto it = nativeShaders.find(query->shaderID);
	if (it == nativeShaders.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->location = glGetUniformLocation(it->second.id, query->name);
}

static void GetActiveUniforms(const GfxShaderQuery* query, GfxActiveUniformsResult* result)
{
	result->error = nullptr;
	result->entries = nullptr;
	result->count = 0;
	activeUniformEntries.clear();
	activeUniformNames.clear();
	activeUniformTypes.clear();

	const auto it = nativeShaders.find(query->shaderID);
	if (it == nativeShaders.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	GLint numUniforms = 0;
	glGetProgramiv(it->second.id, GL_ACTIVE_UNIFORMS, &numUniforms);
	activeUniformEntries.reserve(numUniforms);
	activeUniformNames.reserve(numUniforms);
	activeUniformTypes.reserve(numUniforms);

	std::vector<char> nameBuffer(512, 0);
	for (GLint i = 0; i < numUniforms; ++i) {
		GLsizei uniformLen = 0;
		GLint size = 0;
		GLenum type = 0;
		std::fill(nameBuffer.begin(), nameBuffer.end(), 0);
		glGetActiveUniform(it->second.id, i, nameBuffer.size() - 1, &uniformLen, &size, &type, nameBuffer.data());

		if (std::strncmp(nameBuffer.data(), "gl_", 3) == 0)
			continue;

		std::string name(nameBuffer.data(), uniformLen);
		if (!name.empty() && name.back() == ']' && name.size() >= 3)
			name = name.substr(0, name.size() - 3);

		activeUniformNames.push_back(name);
		activeUniformTypes.emplace_back(UniformTypeString(type));
		activeUniformEntries.push_back({
			activeUniformNames.back().c_str(),
			activeUniformTypes.back().c_str(),
			type,
			static_cast<int32_t>(name.size()),
			size,
			glGetUniformLocation(it->second.id, name.c_str()),
		});
	}

	result->entries = activeUniformEntries.data();
	result->count = activeUniformEntries.size();
}

static void Uniform(const GfxUniformFloatQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	switch (query->count) {
		case 1: glUniform1f(query->location, query->values[0]); break;
		case 2: glUniform2f(query->location, query->values[0], query->values[1]); break;
		case 3: glUniform3f(query->location, query->values[0], query->values[1], query->values[2]); break;
		case 4: glUniform4f(query->location, query->values[0], query->values[1], query->values[2], query->values[3]); break;
		default: result->error = &INVALID_ARGUMENT_ERROR; break;
	}
}

static void UniformInt(const GfxUniformIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	switch (query->count) {
		case 1: glUniform1i(query->location, query->values[0]); break;
		case 2: glUniform2i(query->location, query->values[0], query->values[1]); break;
		case 3: glUniform3i(query->location, query->values[0], query->values[1], query->values[2]); break;
		case 4: glUniform4i(query->location, query->values[0], query->values[1], query->values[2], query->values[3]); break;
		default: result->error = &INVALID_ARGUMENT_ERROR; break;
	}
}

static void UniformArrayFloat(const GfxUniformArrayFloatQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (query->values == nullptr && query->count > 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	glUniform1fv(query->location, query->count, query->values);
}

static void UniformArrayInt(const GfxUniformArrayIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (query->values == nullptr && query->count > 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	glUniform1iv(query->location, query->count, query->values);
}

static void UniformMatrix(const GfxUniformMatrixQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (query->values == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	switch (query->count) {
		case 4: glUniformMatrix2fv(query->location, 1, query->transpose, query->values); break;
		case 9: glUniformMatrix3fv(query->location, 1, query->transpose, query->values); break;
		case 16: glUniformMatrix4fv(query->location, 1, query->transpose, query->values); break;
		default: result->error = &INVALID_ARGUMENT_ERROR; break;
	}
}

static void GetSubroutineIndex(const GfxSubroutineIndexQuery* query, GfxSubroutineIndexResult* result)
{
	result->error = nullptr;
	result->index = 0;
	result->success = false;

	if (!IS_GL_FUNCTION_AVAILABLE(glGetSubroutineIndex))
		return;
	if (query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const auto it = nativeShaders.find(query->shaderID);
	if (it == nativeShaders.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->index = glGetSubroutineIndex(it->second.id, query->shaderType, query->name);
	result->success = true;
}

static void UniformSubroutine(const GfxUniformSubroutineQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (!IS_GL_FUNCTION_AVAILABLE(glUniformSubroutinesuiv))
		return;

	const GLuint index = query->index;
	glUniformSubroutinesuiv(query->shaderType, 1, &index);
}

static void SetGeometryShaderParameter(const GfxGeometryShaderParameterQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (!IS_GL_FUNCTION_AVAILABLE(glProgramParameteriEXT))
		return;

	const auto it = nativeShaders.find(query->shaderID);
	if (it == nativeShaders.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	glProgramParameteriEXT(it->second.id, query->param, query->value);
}

static void SetTesselationShaderParameter(const GfxTesselationShaderParameterQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (!IS_GL_FUNCTION_AVAILABLE(glPatchParameteri))
		return;

	if (query->useFloatArray) {
		if (query->valueCount == 0 || query->valueCount > 4) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}

		glPatchParameterfv(query->param, query->values);
		return;
	}

	glPatchParameteri(query->param, query->value);
}

static void GetEngineUniformBufferDef(const GfxEngineUniformBufferDefQuery* query, GfxStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;

	if (globalRendering == nullptr || !globalRendering->haveGL4)
		return;
	if (query->index < 0 || query->index > 1) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	stringResult = UniformConstants::GetInstance().GetGLSLDefinition(query->index);
	result->value = stringResult.c_str();
}

static void GetEngineModelUniformDataDef(const GfxEmptyQuery*, GfxStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;

	if (globalRendering == nullptr || !globalRendering->haveGL4)
		return;

	stringResult = ModelUniformData::GetGLSLDefinition();
	result->value = stringResult.c_str();
}

static void GetEngineModelUniformDataSize(const GfxEmptyQuery*, GfxEngineModelUniformDataSizeResult* result)
{
	result->error = nullptr;
	result->sizeInElements = 0;
	result->sizeInBytesOnCPU = 0;

	if (globalRendering == nullptr || !globalRendering->haveGL4)
		return;

	result->sizeInElements = modelUniformsStorage.GetSize();
	result->sizeInBytesOnCPU = result->sizeInElements * sizeof(ModelUniformData);
}

template<typename T>
static void SetObjectBufferUniforms(const GfxObjectBufferUniformsQuery* query, GfxObjectBufferUniformsResult* result, T* object)
{
	result->error = nullptr;
	result->count = 0;

	if (object == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	if (query->values == nullptr && query->count > 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (query->offset >= ModelUniformData::MAX_MODEL_UD_UNIFORMS)
		return;

	const uint32_t count = std::min<uint32_t>(query->count, ModelUniformData::MAX_MODEL_UD_UNIFORMS - query->offset);
	if (count == 0)
		return;

	ModelUniformData& uniformData = modelUniformsStorage.GetObjUniformsArray(object);
	std::copy(query->values, query->values + count, std::begin(uniformData.userDefined) + query->offset);
	result->count = count;
}

static void SetUnitBufferUniforms(const GfxObjectBufferUniformsQuery* query, GfxObjectBufferUniformsResult* result)
{
	SetObjectBufferUniforms(query, result, unitHandler.GetUnit(query->objectID));
}

static void SetFeatureBufferUniforms(const GfxObjectBufferUniformsQuery* query, GfxObjectBufferUniformsResult* result)
{
	SetObjectBufferUniforms(query, result, featureHandler.GetFeature(query->objectID));
}

static void CreateTexture(const GfxCreateTextureQuery* query, GfxStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;

	if (query->xsize <= 0 || query->ysize <= 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	NativeTexture tex = TextureFromParams(query);
	if (!IsValidNativeTextureTarget(tex.target) || (tex.target != GL_TEXTURE_1D && tex.target != GL_TEXTURE_2D && tex.zsize <= 0 && (tex.target == GL_TEXTURE_3D || tex.target == GL_TEXTURE_2D_ARRAY))) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const GLenum bindingQuery = GL::GetBindingQueryFromTarget(tex.target);
	if (bindingQuery == 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint currentBinding = 0;
	glGetIntegerv(bindingQuery, &currentBinding);

	glGenTextures(1, &tex.id);
	glBindTexture(tex.target, tex.id);
	glClearErrors("NativeGfx", __func__, globalRendering != nullptr && globalRendering->glDebugErrors);

	const GLenum dataFormat = GL::GetDataFormatFromInternalFormat(tex.format);
	const GLenum dataType = GL::GetDataTypeFromInternalFormat(tex.format);

	switch (tex.target) {
		case GL_TEXTURE_1D:
			glTexImage1D(tex.target, 0, tex.format, tex.xsize, tex.border, dataFormat, dataType, nullptr);
			break;
		case GL_TEXTURE_2D:
			glTexImage2D(tex.target, 0, tex.format, tex.xsize, tex.ysize, tex.border, dataFormat, dataType, nullptr);
			break;
		case GL_TEXTURE_CUBE_MAP:
			for (GLenum face = GL_TEXTURE_CUBE_MAP_POSITIVE_X; face <= GL_TEXTURE_CUBE_MAP_NEGATIVE_Z; ++face)
				glTexImage2D(face, 0, tex.format, tex.xsize, tex.ysize, tex.border, dataFormat, dataType, nullptr);
			break;
		case GL_TEXTURE_2D_ARRAY:
		case GL_TEXTURE_3D:
			glTexImage3D(tex.target, 0, tex.format, tex.xsize, tex.ysize, tex.zsize, tex.border, dataFormat, dataType, nullptr);
			break;
		case GL_TEXTURE_2D_MULTISAMPLE:
			if (globalRendering == nullptr || !globalRendering->supportMSAAFrameBuffer || tex.samples <= 1) {
				glBindTexture(tex.target, currentBinding);
				glDeleteTextures(1, &tex.id);
				result->error = &INVALID_ARGUMENT_ERROR;
				return;
			}
			glTexImage2DMultisample(tex.target, tex.samples, tex.format, tex.xsize, tex.ysize, GL_TRUE);
			break;
		default:
			break;
	}

	if (glGetError() != GL_NO_ERROR) {
		glBindTexture(tex.target, currentBinding);
		glDeleteTextures(1, &tex.id);
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	ApplyTextureParams(tex);
	glBindTexture(tex.target, currentBinding);

	if (query->params.fbo) {
		if (!FBO::IsSupported() || tex.target != GL_TEXTURE_2D) {
			glDeleteTextures(1, &tex.id);
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}

		GLint currentFBO = 0;
		glGetIntegerv(GL_FRAMEBUFFER_BINDING_EXT, &currentFBO);
		glGenFramebuffersEXT(1, &tex.fbo);
		glBindFramebufferEXT(GL_FRAMEBUFFER_EXT, tex.fbo);

		if (query->params.fboDepth) {
			glGenRenderbuffersEXT(1, &tex.fboDepth);
			glBindRenderbufferEXT(GL_RENDERBUFFER_EXT, tex.fboDepth);
			const GLenum depthFormat = static_cast<GLenum>(CGlobalRendering::DepthBitsToFormat(globalRendering->supportDepthBufferBitDepth));
			glRenderbufferStorageEXT(GL_RENDERBUFFER_EXT, depthFormat, tex.xsize, tex.ysize);
			glFramebufferRenderbufferEXT(GL_FRAMEBUFFER_EXT, GL_DEPTH_ATTACHMENT_EXT, GL_RENDERBUFFER_EXT, tex.fboDepth);
		}

		glFramebufferTexture2DEXT(GL_FRAMEBUFFER_EXT, GL_COLOR_ATTACHMENT0_EXT, tex.target, tex.id, 0);
		if (glCheckFramebufferStatus(GL_FRAMEBUFFER_EXT) != GL_FRAMEBUFFER_COMPLETE_EXT) {
			DeleteNativeTexture(tex);
			glBindFramebufferEXT(GL_FRAMEBUFFER_EXT, currentFBO);
			result->error = &OPERATION_FAILED_ERROR;
			return;
		}
		glBindFramebufferEXT(GL_FRAMEBUFFER_EXT, currentFBO);
	}

	stringResult = "!native" + std::to_string(++nativeTextureCounter);
	nativeTextures[stringResult] = tex;
	result->value = stringResult.c_str();
}

static void DeleteTexture(const GfxTextureNameQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;

	const auto it = nativeTextures.find(query->name != nullptr ? query->name : "");
	if (it == nativeTextures.end())
		return;

	DeleteNativeTexture(it->second);
	nativeTextures.erase(it);
	result->value = true;
}

static void DeleteTextureFBO(const GfxTextureNameQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;

	NativeTexture* tex = GetNativeTexture(query->name);
	if (tex == nullptr || !FBO::IsSupported())
		return;

	if (tex->fbo != 0)
		glDeleteFramebuffersEXT(1, &tex->fbo);
	if (tex->fboDepth != 0)
		glDeleteRenderbuffersEXT(1, &tex->fboDepth);

	tex->fbo = 0;
	tex->fboDepth = 0;
	result->value = true;
}

static void BindTexture(const GfxTextureBindQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;

	ResolvedTexture texture;
	if (!ResolveTexture(query->name, texture)) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	const bool switchUnit = query->texNum >= 0;
	if (switchUnit) {
		if (!IsValidTextureUnit(query->texNum)) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		glActiveTexture(GL_TEXTURE0 + query->texNum);
	}

	query->enable ? glEnable(texture.target) : glDisable(texture.target);
	glBindTexture(texture.target, query->enable ? texture.id : 0);

	if (switchUnit)
		glActiveTexture(GL_TEXTURE0);

	result->value = true;
}

static void TextureInfo(const GfxTextureNameQuery* query, GfxTextureInfoResult* result)
{
	result->error = nullptr;
	result->xsize = 0;
	result->ysize = 0;
	result->zsize = 0;
	result->id = 0;
	result->target = 0;
	result->fbo = 0;

	ResolvedTexture texture;
	if (!ResolveTexture(query->name, texture)) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->xsize = texture.xsize;
	result->ysize = texture.ysize;
	result->zsize = texture.zsize;
	result->id = texture.id;
	result->target = texture.target;
	result->fbo = texture.native != nullptr ? texture.native->fbo : 0;
}

static void GetEngineTextureNames(const GfxEmptyQuery*, GfxEngineTextureNamesResult* result)
{
	static thread_local std::vector<const char*> names;
	const auto& source = LuaOpenGLUtils::GetEngineTextureNames();
	names.clear();
	names.reserve(source.size());
	for (const std::string& name: source)
		names.push_back(name.c_str());
	if (infoTextureHandler != nullptr) {
		// Named info textures are resolved by the same parser as Lua's
		// `$info_<mode>`/`$extra_<mode>` syntax.  Report only modes the
		// running map actually provides.
		static thread_local std::vector<std::string> dynamicNames;
		dynamicNames.clear();
		for (const std::string& mode: infoTextureHandler->GetModes()) {
			dynamicNames.push_back("$info_" + mode);
			dynamicNames.push_back("$extra_" + mode);
		}
		for (const std::string& name: dynamicNames)
			names.push_back(name.c_str());
	}
	result->error = nullptr;
	result->names = names.data();
	result->count = uint32_t(names.size());
}

static void GetConsoleCommands(const GfxEmptyQuery*, GfxConsoleCommandsResult* result)
{
	static thread_local std::vector<GfxConsoleCommandEntry> commands;
	commands.clear();
	if (syncedGameCommands == nullptr || unsyncedGameCommands == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->entries = nullptr;
		result->count = 0;
		return;
	}
	const auto append = [](const auto& executors) {
		for (const auto& pair: executors) {
			const auto* executor = pair.second;
			commands.push_back({
				.command = executor->GetCommand().c_str(),
				.description = executor->GetDescription().c_str(),
				.synced = executor->IsSynced(),
				.cheat = executor->IsCheatRequired(),
			});
		}
	};
	append(syncedGameCommands->GetSortedActionExecutors());
	append(unsyncedGameCommands->GetSortedActionExecutors());
	result->error = nullptr;
	result->entries = commands.data();
	result->count = uint32_t(commands.size());
}

static void ChangeTextureParams(const GfxChangeTextureParamsQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	NativeTexture* tex = GetNativeTexture(query->name);
	if (tex == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	tex->minFilter = query->params.minFilter != 0 ? query->params.minFilter : tex->minFilter;
	tex->magFilter = query->params.magFilter != 0 ? query->params.magFilter : tex->magFilter;
	tex->wrapS = query->params.wrapS != 0 ? query->params.wrapS : tex->wrapS;
	tex->wrapT = query->params.wrapT != 0 ? query->params.wrapT : tex->wrapT;
	tex->wrapR = query->params.wrapR != 0 ? query->params.wrapR : tex->wrapR;
	tex->compareFunc = query->params.compareFunc;
	tex->lodBias = query->params.lodBias;
	tex->aniso = query->params.aniso;

	auto bind = GL::TexBind(tex->target, tex->id);
	ApplyTextureParams(*tex);
}

static void CopyToTexture(const GfxCopyToTextureQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	NativeTexture* tex = GetNativeTexture(query->name);
	if (tex == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	glBindTexture(tex->target, tex->id);
	glEnable(tex->target);
	glCopyTexSubImage2D(query->target != 0 ? query->target : tex->target, query->level, query->xoff, query->yoff, query->x, query->y, query->width, query->height);
	if (tex->target != GL_TEXTURE_2D)
		glDisable(tex->target);
}

static bool IsCubeMapFace(GLenum target)
{
	return target >= GL_TEXTURE_CUBE_MAP_POSITIVE_X && target <= GL_TEXTURE_CUBE_MAP_NEGATIVE_Z;
}

static uint32_t PixelFormatComponentCount(GLenum format)
{
	switch (format) {
		case GL_RED:  return 1;
		case GL_RG:   return 2;
		case GL_RGB:  return 3;
		case GL_RGBA: return 4;
		default:      return 0;
	}
}

static uint32_t PixelTypeSize(GLenum type)
{
	switch (type) {
		case GL_BYTE:
		case GL_UNSIGNED_BYTE:
			return 1;
		case GL_SHORT:
		case GL_UNSIGNED_SHORT:
		case GL_HALF_FLOAT:
			return 2;
		case GL_INT:
		case GL_UNSIGNED_INT:
		case GL_FLOAT:
			return 4;
		default:
			return 0;
	}
}

static bool CheckedMultiply(uint64_t& value, uint64_t factor)
{
	if (factor != 0 && value > std::numeric_limits<uint64_t>::max() / factor)
		return false;

	value *= factor;
	return true;
}

static void UploadTexture(const GfxUploadTextureQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	NativeTexture* tex = GetNativeTexture(query->name);
	if (tex == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	if (query->level < 0 || query->xoff < 0 || query->yoff < 0 || query->zoff < 0 ||
	    query->width <= 0 || query->height <= 0 || query->depth <= 0 ||
	    tex->target == GL_TEXTURE_2D_MULTISAMPLE) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLenum uploadTarget = tex->target;
	if (tex->target == GL_TEXTURE_CUBE_MAP) {
		if (!IsCubeMapFace(query->target)) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		uploadTarget = query->target;
	} else if (query->target != 0 && query->target != tex->target) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	switch (tex->target) {
		case GL_TEXTURE_1D:
			if (query->yoff != 0 || query->zoff != 0 || query->height != 1 || query->depth != 1) {
				result->error = &INVALID_ARGUMENT_ERROR;
				return;
			}
			break;
		case GL_TEXTURE_2D:
		case GL_TEXTURE_CUBE_MAP:
			if (query->zoff != 0 || query->depth != 1) {
				result->error = &INVALID_ARGUMENT_ERROR;
				return;
			}
			break;
		case GL_TEXTURE_2D_ARRAY:
		case GL_TEXTURE_3D:
			break;
		default:
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
	}

	const uint32_t componentCount = PixelFormatComponentCount(query->format);
	const uint32_t componentSize = PixelTypeSize(query->pixelType);
	uint64_t requiredSize = static_cast<uint64_t>(query->width);
	if (componentCount == 0 || componentSize == 0 ||
	    !CheckedMultiply(requiredSize, query->height) ||
	    !CheckedMultiply(requiredSize, query->depth) ||
	    !CheckedMultiply(requiredSize, componentCount) ||
	    !CheckedMultiply(requiredSize, componentSize) ||
	    requiredSize > std::numeric_limits<uint32_t>::max() ||
	    requiredSize != query->dataSize || query->data == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const GLenum bindingQuery = GL::GetBindingQueryFromTarget(tex->target);
	if (bindingQuery == 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint previousBinding = 0;
	GLint previousUnpackBuffer = 0;
	GLint previousAlignment = 0;
	GLint previousRowLength = 0;
	GLint previousImageHeight = 0;
	GLint previousSkipPixels = 0;
	GLint previousSkipRows = 0;
	GLint previousSkipImages = 0;
	GLint previousSwapBytes = 0;
	GLint previousLsbFirst = 0;
	glGetIntegerv(bindingQuery, &previousBinding);
	glGetIntegerv(GL_PIXEL_UNPACK_BUFFER_BINDING, &previousUnpackBuffer);
	glGetIntegerv(GL_UNPACK_ALIGNMENT, &previousAlignment);
	glGetIntegerv(GL_UNPACK_ROW_LENGTH, &previousRowLength);
	glGetIntegerv(GL_UNPACK_IMAGE_HEIGHT, &previousImageHeight);
	glGetIntegerv(GL_UNPACK_SKIP_PIXELS, &previousSkipPixels);
	glGetIntegerv(GL_UNPACK_SKIP_ROWS, &previousSkipRows);
	glGetIntegerv(GL_UNPACK_SKIP_IMAGES, &previousSkipImages);
	glGetIntegerv(GL_UNPACK_SWAP_BYTES, &previousSwapBytes);
	glGetIntegerv(GL_UNPACK_LSB_FIRST, &previousLsbFirst);

	auto restoreState = [&]() {
		glBindTexture(tex->target, previousBinding);
		glBindBuffer(GL_PIXEL_UNPACK_BUFFER, previousUnpackBuffer);
		glPixelStorei(GL_UNPACK_ALIGNMENT, previousAlignment);
		glPixelStorei(GL_UNPACK_ROW_LENGTH, previousRowLength);
		glPixelStorei(GL_UNPACK_IMAGE_HEIGHT, previousImageHeight);
		glPixelStorei(GL_UNPACK_SKIP_PIXELS, previousSkipPixels);
		glPixelStorei(GL_UNPACK_SKIP_ROWS, previousSkipRows);
		glPixelStorei(GL_UNPACK_SKIP_IMAGES, previousSkipImages);
		glPixelStorei(GL_UNPACK_SWAP_BYTES, previousSwapBytes);
		glPixelStorei(GL_UNPACK_LSB_FIRST, previousLsbFirst);
	};

	glClearErrors("NativeGfx", __func__, globalRendering != nullptr && globalRendering->glDebugErrors);
	glBindTexture(tex->target, tex->id);
	glBindBuffer(GL_PIXEL_UNPACK_BUFFER, 0);
	glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
	glPixelStorei(GL_UNPACK_ROW_LENGTH, 0);
	glPixelStorei(GL_UNPACK_IMAGE_HEIGHT, 0);
	glPixelStorei(GL_UNPACK_SKIP_PIXELS, 0);
	glPixelStorei(GL_UNPACK_SKIP_ROWS, 0);
	glPixelStorei(GL_UNPACK_SKIP_IMAGES, 0);
	glPixelStorei(GL_UNPACK_SWAP_BYTES, GL_FALSE);
	glPixelStorei(GL_UNPACK_LSB_FIRST, GL_FALSE);

	GLint levelWidth = 0;
	GLint levelHeight = 1;
	GLint levelDepth = 1;
	glGetTexLevelParameteriv(uploadTarget, query->level, GL_TEXTURE_WIDTH, &levelWidth);
	if (tex->target != GL_TEXTURE_1D)
		glGetTexLevelParameteriv(uploadTarget, query->level, GL_TEXTURE_HEIGHT, &levelHeight);
	if (tex->target == GL_TEXTURE_2D_ARRAY || tex->target == GL_TEXTURE_3D)
		glGetTexLevelParameteriv(uploadTarget, query->level, GL_TEXTURE_DEPTH, &levelDepth);

	const bool inBounds = levelWidth > 0 && levelHeight > 0 && levelDepth > 0 &&
		static_cast<int64_t>(query->xoff) + query->width <= levelWidth &&
		static_cast<int64_t>(query->yoff) + query->height <= levelHeight &&
		static_cast<int64_t>(query->zoff) + query->depth <= levelDepth;
	if (!inBounds || glGetError() != GL_NO_ERROR) {
		restoreState();
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	switch (tex->target) {
		case GL_TEXTURE_1D:
			glTexSubImage1D(uploadTarget, query->level, query->xoff, query->width, query->format, query->pixelType, query->data);
			break;
		case GL_TEXTURE_2D:
		case GL_TEXTURE_CUBE_MAP:
			glTexSubImage2D(uploadTarget, query->level, query->xoff, query->yoff, query->width, query->height, query->format, query->pixelType, query->data);
			break;
		case GL_TEXTURE_2D_ARRAY:
		case GL_TEXTURE_3D:
			glTexSubImage3D(uploadTarget, query->level, query->xoff, query->yoff, query->zoff, query->width, query->height, query->depth, query->format, query->pixelType, query->data);
			break;
	}

	const GLenum uploadError = glGetError();
	restoreState();
	if (uploadError != GL_NO_ERROR)
		result->error = &OPERATION_FAILED_ERROR;
}

static void GenerateMipmap(const GfxTextureNameQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	NativeTexture* tex = GetNativeTexture(query->name);
	if (tex == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	auto bind = GL::TexBind(tex->target, tex->id);
	glGenerateMipmapEXT(tex->target);
}

static void BindImageTexture(const GfxBindImageTextureQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	GLint maxUnit = 0;
	glGetIntegerv(GL_MAX_IMAGE_UNITS, &maxUnit);
	if (maxUnit <= 0 || query->unit >= static_cast<uint32_t>(maxUnit)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLuint texID = 0;
	if (query->name != nullptr && query->name[0] != '\0') {
		ResolvedTexture texture;
		if (!ResolveTexture(query->name, texture)) {
			result->error = &NOT_FOUND_ERROR;
			return;
		}
		texID = texture.id;
	}

	glBindImageTexture(query->unit, texID, query->level, query->layered ? GL_TRUE : GL_FALSE, query->layer, query->access != 0 ? query->access : GL_READ_WRITE, query->format);
}

static void ReadPixels(const GfxReadPixelsQuery* query, GfxReadPixelsResult* result)
{
	result->error = nullptr;
	result->values = nullptr;
	result->count = 0;
	result->components = 0;

	if (query->width <= 0 || query->height <= 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	int components = 4;
	switch (query->format) {
		case GL_RED:
		case GL_GREEN:
		case GL_BLUE:
		case GL_ALPHA:
		case GL_DEPTH_COMPONENT:
			components = 1;
			break;
		case GL_RG:
			components = 2;
			break;
		case GL_RGB:
		case GL_BGR:
			components = 3;
			break;
		case GL_RGBA:
		case GL_BGRA:
		default:
			components = 4;
			break;
	}

	readPixelsResult.resize(static_cast<size_t>(query->width) * query->height * components);
	glReadPixels(query->x, query->y, query->width, query->height, query->format != 0 ? query->format : GL_RGBA, GL_FLOAT, readPixelsResult.data());
	result->values = readPixelsResult.data();
	result->count = readPixelsResult.size();
	result->components = components;
}

static void CreateRBO(const GfxRBOCreateQuery* query, GfxUIntResult* result)
{
	result->error = nullptr;
	result->value = 0;

	if (!FBO::IsSupported() || query->xsize <= 0 || query->ysize <= 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	NativeRBO rbo;
	rbo.target = query->target != 0 ? query->target : GL_RENDERBUFFER_EXT;
	rbo.format = query->format != 0 ? query->format : GL_RGBA8;
	rbo.xsize = query->xsize;
	rbo.ysize = query->ysize;
	rbo.samples = std::max(query->samples, 0);

	glGenRenderbuffersEXT(1, &rbo.id);
	glBindRenderbufferEXT(rbo.target, rbo.id);
	if (rbo.samples > 1)
		glRenderbufferStorageMultisampleEXT(rbo.target, rbo.samples, rbo.format, rbo.xsize, rbo.ysize);
	else
		glRenderbufferStorageEXT(rbo.target, rbo.format, rbo.xsize, rbo.ysize);
	glBindRenderbufferEXT(rbo.target, 0);

	if (rbo.id == 0) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	const uint32_t rboID = ++nativeRBOCounter;
	nativeRBOs[rboID] = rbo;
	result->value = rboID;
}

static void DeleteRBO(const GfxUIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const auto it = nativeRBOs.find(query->value);
	if (it == nativeRBOs.end())
		return;

	glDeleteRenderbuffersEXT(1, &it->second.id);
	nativeRBOs.erase(it);
}

static void CreateFBO(const GfxFBOCreateQuery* query, GfxFBOResult* result)
{
	result->error = nullptr;
	result->fboID = 0;
	result->rawID = 0;

	if (!FBO::IsSupported()) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	NativeFBO fbo;
	fbo.target = query->target != 0 ? query->target : GL_FRAMEBUFFER_EXT;

	const GLenum bindTarget = GetFBOBindingEnum(fbo.target);
	if (bindTarget == 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint currentFBO = 0;
	glGetIntegerv(bindTarget, &currentFBO);
	glGenFramebuffersEXT(1, &fbo.id);
	glBindFramebufferEXT(fbo.target, fbo.id);

	for (uint32_t i = 0; i < query->attachmentCount; ++i) {
		const GfxFBOAttachment& attachment = query->attachments[i];
		if (attachment.useRBO) {
			NativeRBO* rbo = GetNativeRBO(attachment.rboID);
			if (rbo == nullptr) {
				glBindFramebufferEXT(fbo.target, currentFBO);
				glDeleteFramebuffersEXT(1, &fbo.id);
				result->error = &NOT_FOUND_ERROR;
				return;
			}

			glFramebufferRenderbufferEXT(fbo.target, attachment.attachment, rbo->target, rbo->id);
			fbo.xsize = std::max(fbo.xsize, rbo->xsize);
			fbo.ysize = std::max(fbo.ysize, rbo->ysize);
			continue;
		}

		NativeTexture* tex = GetNativeTexture(attachment.textureName);
		if (tex == nullptr) {
			glBindFramebufferEXT(fbo.target, currentFBO);
			glDeleteFramebuffersEXT(1, &fbo.id);
			result->error = &NOT_FOUND_ERROR;
			return;
		}

		const GLenum texTarget = attachment.textureTarget != 0 ? attachment.textureTarget : tex->target;
		glFramebufferTexture2DEXT(fbo.target, attachment.attachment, texTarget, tex->id, attachment.mipLevel);
		fbo.xsize = std::max(fbo.xsize, tex->xsize);
		fbo.ysize = std::max(fbo.ysize, tex->ysize);
	}

	if (query->drawBuffers != nullptr && query->drawBufferCount > 0)
		glDrawBuffers(query->drawBufferCount, query->drawBuffers);
	if (query->readBuffer != 0)
		glReadBuffer(query->readBuffer);

	glBindFramebufferEXT(fbo.target, currentFBO);

	if (fbo.id == 0) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	const uint32_t fboID = ++nativeFBOCounter;
	result->fboID = fboID;
	result->rawID = fbo.id;
	nativeFBOs[fboID] = fbo;
}

static void DeleteFBO(const GfxUIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const auto it = nativeFBOs.find(query->value);
	if (it == nativeFBOs.end())
		return;

	glDeleteFramebuffersEXT(1, &it->second.id);
	nativeFBOs.erase(it);
}

static void IsValidFBO(const GfxFBOQuery* query, GfxFBOStatusResult* result)
{
	result->error = nullptr;
	result->valid = false;
	result->status = 0;

	NativeFBO* fbo = GetNativeFBO(query->fboID);
	if (fbo == nullptr)
		return;

	const GLenum target = query->target != 0 ? query->target : fbo->target;
	const GLenum bindTarget = GetFBOBindingEnum(target);
	if (bindTarget == 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint currentFBO = 0;
	glGetIntegerv(bindTarget, &currentFBO);
	glBindFramebufferEXT(target, fbo->id);
	result->status = glCheckFramebufferStatus(target);
	result->valid = (result->status == GL_FRAMEBUFFER_COMPLETE_EXT);
	glBindFramebufferEXT(target, currentFBO);
}

static void ActiveFBO(const GfxActiveFBOQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	NativeFBO* fbo = GetNativeFBO(query->fboID);
	if (fbo == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const GLenum target = query->target != 0 ? query->target : fbo->target;
	const GLenum bindTarget = GetFBOBindingEnum(target);
	if (bindTarget == 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint currentFBO = 0;
	glGetIntegerv(bindTarget, &currentFBO);
	glPushAttrib(GL_VIEWPORT_BIT);
	if (fbo->xsize > 0 && fbo->ysize > 0)
		glViewport(0, 0, fbo->xsize, fbo->ysize);
	if (query->identities) {
		glMatrixMode(GL_PROJECTION); glPushMatrix(); glLoadIdentity();
		glMatrixMode(GL_MODELVIEW);  glPushMatrix(); glLoadIdentity();
	}

	glBindFramebufferEXT(target, fbo->id);
	query->callback(query->userData);
	glBindFramebufferEXT(target, currentFBO);

	if (query->identities) {
		glMatrixMode(GL_PROJECTION); glPopMatrix();
		glMatrixMode(GL_MODELVIEW);  glPopMatrix();
	}
	glPopAttrib();
}

static void RawBindFBO(const GfxRawBindFBOQuery* query, GfxRawBindFBOResult* result)
{
	result->error = nullptr;
	result->previouslyBoundRawFboID = 0;
	result->hasPrevious = false;

	const GLenum target = query->target != 0 ? query->target : GL_FRAMEBUFFER_EXT;
	if (query->bindDefault) {
		glBindFramebufferEXT(target, query->rawFboID);
		return;
	}

	NativeFBO* fbo = GetNativeFBO(query->fboID);
	if (fbo == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	GLint currentFBO = 0;
	glGetIntegerv(GL_FRAMEBUFFER_BINDING_EXT, &currentFBO);
	result->previouslyBoundRawFboID = currentFBO;
	result->hasPrevious = true;
	glBindFramebufferEXT(target, fbo->id);
}

static void BlitFBO(const GfxBlitFBOQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const NativeFBO* src = (query->srcFBOID == 0) ? nullptr : GetNativeFBO(query->srcFBOID);
	const NativeFBO* dst = (query->dstFBOID == 0) ? nullptr : GetNativeFBO(query->dstFBOID);
	if ((query->srcFBOID != 0 && src == nullptr) || (query->dstFBOID != 0 && dst == nullptr)) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	GLint currentFBO = 0;
	glGetIntegerv(GL_FRAMEBUFFER_BINDING_EXT, &currentFBO);
	glBindFramebufferEXT(GL_READ_FRAMEBUFFER_EXT, src != nullptr ? src->id : 0);
	glBindFramebufferEXT(GL_DRAW_FRAMEBUFFER_EXT, dst != nullptr ? dst->id : 0);
	glBlitFramebufferEXT(query->x0Src, query->y0Src, query->x1Src, query->y1Src, query->x0Dst, query->y0Dst, query->x1Dst, query->y1Dst, query->mask != 0 ? query->mask : GL_COLOR_BUFFER_BIT, query->filter != 0 ? query->filter : GL_NEAREST);
	glBindFramebufferEXT(GL_FRAMEBUFFER_EXT, currentFBO);
}

static void ClearAttachmentFBO(const GfxClearAttachmentFBOQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;
	if (!GLAD_GL_VERSION_3_0 || query->count > 4) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	float values[4] = { 0.0f, 0.0f, 0.0f, 0.0f };
	for (uint32_t i = 0; i < query->count; ++i)
		values[i] = query->values[i];

	const GLenum attachment = query->attachment;
	if (attachment == GL_DEPTH || attachment == GL_DEPTH_ATTACHMENT) {
		glClearBufferfv(GL_DEPTH, 0, values);
		result->value = true;
		return;
	}
	if (attachment == GL_STENCIL || attachment == GL_STENCIL_ATTACHMENT) {
		const GLint stencilValue = values[0];
		glClearBufferiv(GL_STENCIL, 0, &stencilValue);
		result->value = true;
		return;
	}

	const GLint drawBuffer = (attachment >= GL_COLOR_ATTACHMENT0 && attachment <= GL_COLOR_ATTACHMENT15) ? (attachment - GL_COLOR_ATTACHMENT0) : 0;
	glClearBufferfv(GL_COLOR, drawBuffer, values);
	result->value = true;
}

static void GetVAO(const GfxEmptyQuery*, GfxVAOResult* result)
{
	result->error = nullptr;
	result->vaoID = 0;
	result->rawID = 0;

	if (!GLAD_GL_ARB_vertex_array_object || !GLAD_GL_ARB_vertex_buffer_object || !GLAD_GL_ARB_instanced_arrays || !GLAD_GL_ARB_draw_elements_base_vertex || !GLAD_GL_ARB_multi_draw_indirect) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	GLuint vao = 0;
	glGenVertexArrays(1, &vao);
	if (vao == 0) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	const uint32_t vaoID = ++nativeVAOCounter;
	nativeVAOs[vaoID] = vao;
	result->vaoID = vaoID;
	result->rawID = vao;
}

static void DeleteVAO(const GfxUIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const auto it = nativeVAOs.find(query->value);
	if (it == nativeVAOs.end())
		return;

	glDeleteVertexArrays(1, &it->second);
	nativeVAOs.erase(it);
}

static void GetVBO(const GfxVBOQuery* query, GfxVBOResult* result)
{
	result->error = nullptr;
	result->vboID = 0;
	result->rawID = 0;
	result->target = query->target != 0 ? query->target : GL_ARRAY_BUFFER;

	if (!GLAD_GL_ARB_vertex_buffer_object) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	switch (result->target) {
		case GL_ARRAY_BUFFER:
		case GL_ELEMENT_ARRAY_BUFFER:
		case GL_UNIFORM_BUFFER:
		case GL_SHADER_STORAGE_BUFFER:
			break;
		default:
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
	}

	GLuint vbo = 0;
	glGenBuffers(1, &vbo);
	if (vbo == 0) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	const uint32_t vboID = ++nativeVBOCounter;
	nativeVBOs[vboID] = vbo;
	nativeVBOTargets[vboID] = result->target;
	result->vboID = vboID;
	result->rawID = vbo;
}

static void DeleteVBO(const GfxUIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const auto it = nativeVBOs.find(query->value);
	if (it == nativeVBOs.end())
		return;

	glDeleteBuffers(1, &it->second);
	nativeVBOTargets.erase(query->value);
	nativeVBOs.erase(it);
}

static void RenderToTexture(const GfxRenderToTextureQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	NativeTexture* tex = GetNativeTexture(query->name);
	if (tex == nullptr || tex->fbo == 0) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint currentFBO = 0;
	glGetIntegerv(GL_FRAMEBUFFER_BINDING_EXT, &currentFBO);
	glBindFramebufferEXT(GL_FRAMEBUFFER_EXT, tex->fbo);

	glPushAttrib(GL_VIEWPORT_BIT);
	glViewport(0, 0, tex->xsize, tex->ysize);
	glMatrixMode(GL_PROJECTION); glPushMatrix(); glLoadIdentity();
	glMatrixMode(GL_MODELVIEW);  glPushMatrix(); glLoadIdentity();

	query->callback(query->userData);

	glMatrixMode(GL_PROJECTION); glPopMatrix();
	glMatrixMode(GL_MODELVIEW);  glPopMatrix();
	glPopAttrib();
	glBindFramebufferEXT(GL_FRAMEBUFFER_EXT, currentFBO);
}

static void CreateTextureAtlas(const GfxCreateTextureAtlasQuery* query, GfxStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;

	constexpr int minSize = 256;
	const int maxSizeX = configHandler->GetInt("MaxTextureAtlasSizeX");
	const int maxSizeY = configHandler->GetInt("MaxTextureAtlasSizeY");

	if (query->xsize < minSize || query->ysize < minSize || query->xsize > maxSizeX || query->ysize > maxSizeY) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const int allocType = std::clamp(query->allocType, static_cast<int>(CTextureAtlas::ATLAS_ALLOC_LEGACY), static_cast<int>(CTextureAtlas::ATLAS_ALLOC_ROW));
	stringResult = "*native" + std::to_string(++nativeAtlasCounter);
	nativeAtlases.emplace_back(CTextureAtlas(allocType, query->xsize, query->ysize, stringResult));
	nativeAtlasMap[stringResult] = nativeAtlases.size() - 1;
	result->value = stringResult.c_str();
}

static void FinalizeTextureAtlas(const GfxTextureNameQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;

	CTextureAtlas* atlas = GetNativeAtlas(query->name);
	if (atlas == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->value = atlas->Finalize();
}

static void DeleteTextureAtlas(const GfxTextureNameQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = DeleteNativeAtlas(query->name);
}

static void AddAtlasTexture(const GfxAtlasTextureQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	CTextureAtlas* atlas = GetNativeAtlas(query->atlasName);
	NativeTexture* tex = GetNativeTexture(query->textureName);
	if (atlas == nullptr || tex == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	if (tex->target != GL_TEXTURE_2D || tex->id == 0 || tex->xsize <= 0 || tex->ysize <= 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLint currentBinding = 0;
	glGetIntegerv(GL_TEXTURE_BINDING_2D, &currentBinding);

	std::vector<uint8_t> buffer(static_cast<size_t>(tex->xsize) * tex->ysize * sizeof(uint32_t));
	glBindTexture(GL_TEXTURE_2D, tex->id);
	glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_BYTE, buffer.data());
	glBindTexture(GL_TEXTURE_2D, currentBinding);

	const char* subName = (query->textureName != nullptr && query->textureName[0] != '\0') ? query->textureName : "texture";
	atlas->AddTexFromMem(subName, tex->xsize, tex->ysize, CTextureAtlas::RGBA32, buffer.data());
}

static void GetAtlasTexture(const GfxAtlasTextureQuery* query, GfxAtlasTextureResult* result)
{
	result->error = nullptr;
	result->x1 = 0.0f;
	result->x2 = 0.0f;
	result->y1 = 0.0f;
	result->y2 = 0.0f;
	result->pageNum = 0;

	CTextureAtlas* atlas = GetNativeAtlas(query->atlasName);
	if (atlas == nullptr || query->textureName == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	const AtlasedTexture tex = atlas->GetTexture(query->textureName);
	if (tex == AtlasedTexture::DefaultAtlasTexture) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->x1 = tex.x1;
	result->x2 = tex.x2;
	result->y1 = tex.y1;
	result->y2 = tex.y2;
	result->pageNum = tex.pageNum;
}

static void FillAtlasTextureEntries(const spring::unordered_map<std::string, IAtlasAllocator::SAtlasEntry>& textures, GfxAtlasTexturesResult* result)
{
	atlasTextureEntries.clear();
	atlasTextureEntries.reserve(textures.size());

	for (const auto& texture : textures) {
		const AtlasedTexture& tex = texture.second.texCoords;
		atlasTextureEntries.push_back({
			.name = texture.first.c_str(),
			.x1 = tex.x1,
			.x2 = tex.x2,
			.y1 = tex.y1,
			.y2 = tex.y2,
			.pageNum = static_cast<int32_t>(tex.pageNum),
		});
	}

	result->entries = atlasTextureEntries.data();
	result->count = atlasTextureEntries.size();
}

static void GetEngineAtlasTextures(const GfxTextureNameQuery* query, GfxAtlasTexturesResult* result)
{
	result->error = nullptr;
	result->entries = nullptr;
	result->count = 0;

	if (projectileDrawer == nullptr || projectileDrawer->textureAtlas == nullptr || projectileDrawer->groundFXAtlas == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const char* name = query->name != nullptr ? query->name : "";
	switch (hashString(name)) {
		case hashString("$explosions"): FillAtlasTextureEntries(projectileDrawer->textureAtlas->GetTextures(), result); return;
		case hashString("$groundfx"): FillAtlasTextureEntries(projectileDrawer->groundFXAtlas->GetTextures(), result); return;
		default: result->error = &INVALID_ARGUMENT_ERROR; return;
	}
}

static void SaveImage(const GfxSaveImageQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;

	if (query->filename == nullptr || query->width <= 0 || query->height <= 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	GLenum curReadBuffer = 0;
	if (query->readBuffer != 0) {
		glGetIntegerv(GL_READ_BUFFER, reinterpret_cast<GLint*>(&curReadBuffer));
		glReadBuffer(query->readBuffer);
	}

	CBitmap bitmap;
	bitmap.Alloc(query->width, query->height);
	glReadPixels(query->x, query->y, query->width, query->height, GL_RGBA, GL_UNSIGNED_BYTE, bitmap.GetRawMem());

	if (query->yflip)
		bitmap.ReverseYAxis();

	result->value = query->grayscale16bit ? bitmap.SaveGrayScale(query->filename) : bitmap.Save(query->filename, !query->alpha);

	if (query->readBuffer != 0)
		glReadBuffer(curReadBuffer);
}

static void CreateList(const GfxCallbackQuery* query, GfxUIntResult* result)
{
	result->error = nullptr;
	result->value = 0;

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const GLuint list = glGenLists(1);
	if (list == 0) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	glNewList(list, GL_COMPILE);
	query->callback(query->userData);
	glEndList();

	result->value = ++nativeDisplayListCounter;
	nativeDisplayLists[result->value] = list;
}

static void CallList(const GfxUIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	const auto it = nativeDisplayLists.find(query->value);
	if (it == nativeDisplayLists.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	glCallList(it->second);
}

static void DeleteList(const GfxUIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	const auto it = nativeDisplayLists.find(query->value);
	if (it == nativeDisplayLists.end())
		return;

	glDeleteLists(it->second, 1);
	nativeDisplayLists.erase(it);
}

static void CreateQuery(const GfxEmptyQuery*, GfxUIntResult* result)
{
	result->error = nullptr;
	result->value = 0;

	GLuint id = 0;
	glGenQueries(1, &id);
	if (id == 0) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	result->value = ++nativeQueryCounter;
	nativeQueries[result->value] = id;
}

static void DeleteQuery(const GfxUIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	const auto it = nativeQueries.find(query->value);
	if (it == nativeQueries.end())
		return;

	glDeleteQueries(1, &it->second);
	nativeQueries.erase(it);
}

static void RunQuery(const GfxRunQueryQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	const auto it = nativeQueries.find(query->id);
	if (it == nativeQueries.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	glBeginQuery(GL_SAMPLES_PASSED, it->second);
	query->callback(query->userData);
	glEndQuery(GL_SAMPLES_PASSED);
}

static void GetQuery(const GfxUIntQuery* query, GfxUIntResult* result)
{
	result->error = nullptr;
	result->value = 0;

	const auto it = nativeQueries.find(query->value);
	if (it == nativeQueries.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	glGetQueryObjectuiv(it->second, GL_QUERY_RESULT, &result->value);
}

static void GetGlobalTexNames(const GfxEmptyQuery*, GfxAtlasTexturesResult* result)
{
	result->error = nullptr;
	result->entries = nullptr;
	result->count = 0;
	atlasTextureEntries.clear();

	const auto& textures = textureHandler3DO.GetAtlasTextures();
	atlasTextureEntries.reserve(textures.size());
	for (const auto& texture : textures) {
		atlasTextureEntries.push_back({
			.name = texture.first.c_str(),
			.x1 = texture.second.xstart,
			.x2 = texture.second.xend,
			.y1 = texture.second.ystart,
			.y2 = texture.second.yend,
			.pageNum = 0,
		});
	}

	result->entries = atlasTextureEntries.data();
	result->count = atlasTextureEntries.size();
}

static void GetGlobalTexCoords(const GfxStringQuery* query, GfxAtlasTextureResult* result)
{
	result->error = nullptr;
	result->x1 = 0.0f;
	result->x2 = 0.0f;
	result->y1 = 0.0f;
	result->y2 = 0.0f;
	result->pageNum = 0;

	if (query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const C3DOTextureHandler::UnitTexture* texCoords = textureHandler3DO.Get3DOTexture(query->value);
	if (texCoords == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->x1 = texCoords->xstart;
	result->x2 = texCoords->xend;
	result->y1 = texCoords->ystart;
	result->y2 = texCoords->yend;
}

static int ParseFontOptions(const char* chars)
{
	int options = FONT_NEAREST;
	if (chars == nullptr)
		return options;

	while (*chars != 0) {
		switch (*chars) {
			case 'c': options |= FONT_CENTER; break;
			case 'r': options |= FONT_RIGHT; break;
			case 'a': options |= FONT_ASCENDER; break;
			case 't': options |= FONT_TOP; break;
			case 'v': options |= FONT_VCENTER; break;
			case 'x': options |= FONT_BASELINE; break;
			case 'b': options |= FONT_BOTTOM; break;
			case 'd': options |= FONT_DESCENDER; break;
			case 's': options |= FONT_SHADOW; break;
			case 'o':
			case 'O': options |= FONT_OUTLINE; break;
			case 'n': options ^= FONT_NEAREST; break;
			default: break;
		}
		++chars;
	}

	return options;
}

static bool FontReady(GfxEmptyResult* result)
{
	if (font != nullptr)
		return true;

	result->error = &NOT_READY_ERROR;
	return false;
}

static CglFont* GetNativeFont(uint32_t fontID)
{
	const auto it = nativeFonts.find(fontID);
	return (it != nativeFonts.end()) ? it->second.get() : nullptr;
}

static void BeginText(const GfxBoolQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (!FontReady(result))
		return;

	font->Begin(query->value);
}

static void Text(const GfxTextQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (!FontReady(result))
		return;

	if (query->text == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const int options = ParseFontOptions(query->options);
	if (query->options != nullptr && std::strchr(query->options, 'O') != nullptr) {
		font->SetOutlineColor(0.95f, 0.95f, 0.95f, 0.8f);
	} else if (query->options != nullptr && std::strchr(query->options, 'o') != nullptr) {
		font->SetOutlineColor(0.15f, 0.15f, 0.15f, 0.8f);
	}

	float color[4] = { 1.0f, 1.0f, 1.0f, 1.0f };
	glGetFloatv(GL_CURRENT_COLOR, color);
	font->SetTextColor(color[0], color[1], color[2], color[3]);
	font->glPrint(query->x, query->y, query->size, options, query->text);
}

static void EndText(const GfxEmptyQuery*, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (!FontReady(result))
		return;

	font->End();
}

static void GetTextWidth(const GfxStringQuery* query, GfxFloatResult* result)
{
	result->error = nullptr;
	result->value = 0.0f;

	if (font == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->value = font->GetTextWidth(query->value);
}

static void GetTextHeight(const GfxStringQuery* query, GfxTextHeightResult* result)
{
	result->error = nullptr;
	result->height = 0.0f;
	result->descender = 0.0f;
	result->lines = 0;

	if (font == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	int lines = 0;
	result->height = font->GetTextHeight(query->value, &result->descender, &lines);
	result->lines = lines;
}

static void AddFallbackFont(const GfxStringQuery* query, GfxBoolResult* result)
{
	result->error = nullptr;
	result->value = false;
	if (query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->value = CFontTexture::AddFallbackFont(query->value);
}

static void ClearFallbackFonts(const GfxEmptyQuery*, GfxEmptyResult* result)
{
	result->error = nullptr;
	CFontTexture::ClearFallbackFonts();
}

static void LoadFont(const GfxLoadFontQuery* query, GfxFontResult* result)
{
	result->error = nullptr;
	result->fontID = 0;
	if (query->path == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::shared_ptr<CglFont> loadedFont = CglFont::LoadFont(
		query->path,
		query->size > 0 ? query->size : 14,
		query->outlineWidth >= 0 ? query->outlineWidth : 2,
		query->outlineWeight > 0.0f ? query->outlineWeight : 15.0f
	);
	if (loadedFont == nullptr) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	const uint32_t fontID = ++nativeFontCounter;
	nativeFonts[fontID] = std::move(loadedFont);
	result->fontID = fontID;
}

static void DeleteFont(const GfxFontQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	nativeFonts.erase(query->fontID);
}

static void GetFontInfo(const GfxFontQuery* query, GfxFontInfoResult* result)
{
	result->error = nullptr;
	result->path = nullptr;
	result->family = nullptr;
	result->style = nullptr;
	result->size = 0.0f;
	result->lineHeight = 0.0f;
	result->descender = 0.0f;
	result->outlineWidth = 0.0f;
	result->outlineWeight = 0.0f;
	result->textureWidth = 0;
	result->textureHeight = 0;

	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	result->path = f->GetFilePath().c_str();
	result->family = f->GetFamily().c_str();
	result->style = f->GetStyle().c_str();
	result->size = f->GetSize();
	result->lineHeight = f->GetLineHeight();
	result->descender = f->GetDescender();
	result->outlineWidth = f->GetOutlineWidth();
	result->outlineWeight = f->GetOutlineWeight();
	result->textureWidth = f->GetTextureWidth();
	result->textureHeight = f->GetTextureHeight();
}

static void FontBegin(const GfxFontBeginQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	f->Begin(query->userDefinedBlending);
}

static void FontEnd(const GfxFontQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	f->End();
}

static void FontPrint(const GfxFontTextQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	if (query->text == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	f->glPrint(query->x, query->y, query->size > 0.0f ? query->size : f->GetSize(), ParseFontOptions(query->options), query->text);
}

static void FontPrintWorld(const GfxFontWorldTextQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	if (query->text == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const float3 pos = { query->pos.x, query->pos.y, query->pos.z };
	f->glWorldPrint(pos, query->size > 0.0f ? query->size : f->GetSize(), query->text, ParseFontOptions(query->options));
}

static void FontSubmitBuffered(const GfxFontSubmitBufferedQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	if (query->noBillboarding)
		f->DrawBuffered(query->userDefinedBlending);
	else
		f->DrawWorldBuffered(query->userDefinedBlending);
}

static void FontWrapText(const GfxFontWrapTextQuery* query, GfxFontWrapTextResult* result)
{
	result->error = nullptr;
	result->text = nullptr;
	result->lines = 0;

	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	if (query->text == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	fontWrapResult = query->text;
	result->lines = f->WrapInPlace(fontWrapResult, query->size > 0.0f ? query->size : f->GetSize(), query->maxWidth, query->maxHeight > 0.0f ? query->maxHeight : CglFont::MAX_HEIGHT_DEFAULT);
	result->text = fontWrapResult.c_str();
}

static void FontGetTextWidth(const GfxFontTextQuery* query, GfxFloatResult* result)
{
	result->error = nullptr;
	result->value = 0.0f;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	if (query->text == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->value = f->GetTextWidth(query->text);
}

static void FontGetTextHeight(const GfxFontTextQuery* query, GfxTextHeightResult* result)
{
	result->error = nullptr;
	result->height = 0.0f;
	result->descender = 0.0f;
	result->lines = 0;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	if (query->text == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	int lines = 0;
	result->height = f->GetTextHeight(query->text, &result->descender, &lines);
	result->lines = lines;
}

static void FontSetTextColor(const GfxFontColorQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	f->SetTextColor(query->r, query->g, query->b, query->a);
}

static void FontSetOutlineColor(const GfxFontColorQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	f->SetOutlineColor(query->r, query->g, query->b, query->a);
}

static void FontSetAutoOutlineColor(const GfxFontAutoOutlineColorQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	f->SetAutoOutlineColor(query->enable);
}

static void FontBindTexture(const GfxFontQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	CglFont* f = GetNativeFont(query->fontID);
	if (f == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	glBindTexture(GL_TEXTURE_2D, f->GetTexture());
	glEnable(GL_TEXTURE_2D);
}

static void BeginEnd(const GfxBeginEndQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	glBegin(query->primitive);
	query->callback(query->userData);
	glEnd();
}

static void PushPopMatrix(const GfxCallbackQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	glPushMatrix();
	query->callback(query->userData);
	glPopMatrix();
}

static void UnsafeState(const GfxUnsafeStateQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	query->reverse ? glDisable(query->state) : glEnable(query->state);
	query->callback(query->userData);
	query->reverse ? glEnable(query->state) : glDisable(query->state);
}

static void DrawGroundCircle(const GfxGroundCircleQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	float color[4] = { 1.0f, 1.0f, 1.0f, 1.0f };
	glGetFloatv(GL_CURRENT_COLOR, color);

	if (query->ballistic) {
		const WeaponDef* wd = weaponDefHandler != nullptr ? weaponDefHandler->GetWeaponDefByID(query->weaponDefID) : nullptr;
		if (wd == nullptr) {
			result->error = &NOT_FOUND_ERROR;
			return;
		}

		const float gravity = query->gravity != 0.0f ? query->gravity : mapInfo->map.gravity;
		glBallisticCircleLua(wd, { color }, query->resolution, { query->pos.x, query->pos.y, query->pos.z }, { query->radius, query->slope, gravity });
		return;
	}

	glSurfaceCircleLua({ query->pos.x, query->pos.y, query->pos.z }, query->radius, { color }, query->resolution);
}

static void DrawGroundQuad(const GfxGroundQuadQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (readMap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const int mapxi = mapDims.mapxp1;
	const int mapzi = mapDims.mapyp1;
	const float* heightmap = readMap->GetCornerHeightMapUnsynced();
	const float xs = std::clamp(query->x0, 0.0f, float3::maxxpos);
	const float xe = std::clamp(query->x1, 0.0f, float3::maxxpos);
	const float zs = std::clamp(query->z0, 0.0f, float3::maxzpos);
	const float ze = std::clamp(query->z1, 0.0f, float3::maxzpos);
	const int xis = std::clamp(static_cast<int>((xs + 0.5f) / SQUARE_SIZE), 0, mapxi);
	const int xie = std::clamp(static_cast<int>((xe + 0.5f) / SQUARE_SIZE), 0, mapxi);
	const int zis = std::clamp(static_cast<int>((zs + 0.5f) / SQUARE_SIZE), 0, mapzi);
	const int zie = std::clamp(static_cast<int>((ze + 0.5f) / SQUARE_SIZE), 0, mapzi);

	if (xis >= xie || zis >= zie)
		return;

	const float tuStep = query->useTexCoords ? (query->tu1 - query->tu0) / float(xie - xis) : 0.0f;
	const float tvStep = query->useTexCoords ? (query->tv1 - query->tv0) / float(zie - zis) : 0.0f;
	float tub = query->tu0;

	for (int xib = xis; xib < xie; ++xib) {
		const int xit = xib + 1;
		const float xb = xib * SQUARE_SIZE;
		const float xt = xb + SQUARE_SIZE;
		const float tut = tub + tuStep;
		float tv = query->tv0;

		glBegin(GL_TRIANGLE_STRIP);
		for (int zi = zis; zi <= zie; ++zi) {
			const int ziOff = zi * mapxi;
			const float yb = heightmap[ziOff + xib];
			const float yt = heightmap[ziOff + xit];
			const float z = zi * SQUARE_SIZE;
			if (query->useTexCoords)
				glTexCoord2f(tut, tv);
			glVertex3f(xt, yt, z);
			if (query->useTexCoords)
				glTexCoord2f(tub, tv);
			glVertex3f(xb, yb, z);
			tv += tvStep;
		}
		glEnd();
		tub += tuStep;
	}
}

static void ResetFixedStateResult(GfxFixedStateResult* result)
{
	result->error = nullptr;
	result->boolCount = 0;
	result->intCount = 0;
	result->floatCount = 0;
	std::fill(std::begin(result->bools), std::end(result->bools), false);
	std::fill(std::begin(result->ints), std::end(result->ints), 0);
	std::fill(std::begin(result->floats), std::end(result->floats), 0.0f);
}

static void GetFixedState(const GfxFixedStateQuery* query, GfxFixedStateResult* result)
{
	ResetFixedStateResult(result);
	const char* param = query->param != nullptr ? query->param : "";

	switch (hashString(param)) {
		case hashString("blending"):
			result->bools[result->boolCount++] = glIsEnabled(GL_BLEND);
			glGetIntegerv(GL_BLEND_SRC_RGB, &result->ints[result->intCount++]);
			glGetIntegerv(GL_BLEND_SRC_ALPHA, &result->ints[result->intCount++]);
			glGetIntegerv(GL_BLEND_DST_RGB, &result->ints[result->intCount++]);
			glGetIntegerv(GL_BLEND_DST_ALPHA, &result->ints[result->intCount++]);
			glGetIntegerv(GL_BLEND_EQUATION_RGB, &result->ints[result->intCount++]);
			glGetIntegerv(GL_BLEND_EQUATION_ALPHA, &result->ints[result->intCount++]);
			return;
		case hashString("depth"):
			result->bools[result->boolCount++] = glIsEnabled(GL_DEPTH_TEST);
			result->bools[result->boolCount++] = glIsEnabled(GL_DEPTH_WRITEMASK);
			glGetIntegerv(GL_DEPTH_FUNC, &result->ints[result->intCount++]);
			return;
		case hashString("shadeModel"):
		case hashString("shademodel"):
			glGetIntegerv(GL_SHADE_MODEL, &result->ints[result->intCount++]);
			return;
		case hashString("scissor"):
			result->bools[result->boolCount++] = glIsEnabled(GL_SCISSOR_TEST);
			glGetIntegerv(GL_SCISSOR_BOX, result->ints);
			result->intCount = 4;
			return;
		case hashString("lighting"):
			result->bools[result->boolCount++] = glIsEnabled(GL_LIGHTING);
			return;
		case hashString("colorMask"):
		case hashString("colormask"): {
			GLboolean mask[4] = {};
			glGetBooleanv(GL_COLOR_WRITEMASK, mask);
			for (int i = 0; i < 4; ++i)
				result->bools[result->boolCount++] = mask[i];
			return;
		}
		case hashString("culling"):
			result->bools[result->boolCount++] = glIsEnabled(GL_CULL_FACE);
			glGetIntegerv(GL_CULL_FACE_MODE, &result->ints[result->intCount++]);
			return;
		case hashString("logicOp"):
		case hashString("logicop"):
			result->bools[result->boolCount++] = glIsEnabled(GL_COLOR_LOGIC_OP);
			glGetIntegerv(GL_LOGIC_OP_MODE, &result->ints[result->intCount++]);
			return;
		case hashString("alphaTest"):
		case hashString("alphatest"):
			result->bools[result->boolCount++] = glIsEnabled(GL_ALPHA_TEST);
			glGetIntegerv(GL_ALPHA_TEST_FUNC, &result->ints[result->intCount++]);
			glGetFloatv(GL_ALPHA_TEST_REF, &result->floats[result->floatCount++]);
			return;
		case hashString("fog"):
			result->bools[result->boolCount++] = glIsEnabled(GL_FOG);
			glGetFloatv(GL_FOG_COLOR, result->floats);
			result->floatCount = 4;
			glGetFloatv(GL_FOG_DENSITY, &result->floats[result->floatCount++]);
			glGetFloatv(GL_FOG_START, &result->floats[result->floatCount++]);
			glGetFloatv(GL_FOG_END, &result->floats[result->floatCount++]);
			glGetIntegerv(GL_FOG_MODE, &result->ints[result->intCount++]);
			return;
		case hashString("edgeFlag"):
		case hashString("edgeflag"): {
			GLboolean edgeFlag = GL_FALSE;
			glGetBooleanv(GL_EDGE_FLAG, &edgeFlag);
			result->bools[result->boolCount++] = edgeFlag;
			return;
		}
		case hashString("lineStripple"):
		case hashString("linestripple"):
			result->bools[result->boolCount++] = glIsEnabled(GL_LINE_STIPPLE);
			glGetIntegerv(GL_LINE_STIPPLE_PATTERN, &result->ints[result->intCount++]);
			glGetIntegerv(GL_LINE_STIPPLE_REPEAT, &result->ints[result->intCount++]);
			return;
		case hashString("polygonMode"):
		case hashString("polygonmode"):
			glGetIntegerv(GL_POLYGON_MODE, result->ints);
			result->intCount = 2;
			return;
		case hashString("polygonOffset"):
		case hashString("polygonoffset"):
			result->bools[result->boolCount++] = glIsEnabled(GL_POLYGON_OFFSET_FILL);
			result->bools[result->boolCount++] = glIsEnabled(GL_POLYGON_OFFSET_LINE);
			result->bools[result->boolCount++] = glIsEnabled(GL_POLYGON_OFFSET_POINT);
			glGetFloatv(GL_POLYGON_OFFSET_FACTOR, &result->floats[result->floatCount++]);
			glGetFloatv(GL_POLYGON_OFFSET_UNITS, &result->floats[result->floatCount++]);
			return;
		case hashString("stencil"):
			result->bools[result->boolCount++] = glIsEnabled(GL_STENCIL_TEST);
			glGetIntegerv(GL_STENCIL_WRITEMASK, &result->ints[result->intCount++]);
			glGetIntegerv(GL_STENCIL_BITS, &result->ints[result->intCount++]);
			glGetIntegerv(GL_STENCIL_VALUE_MASK, &result->ints[result->intCount++]);
			glGetIntegerv(GL_STENCIL_REF, &result->ints[result->intCount++]);
			glGetIntegerv(GL_STENCIL_FUNC, &result->ints[result->intCount++]);
			if (GLAD_GL_EXT_stencil_two_side) {
				glGetIntegerv(GL_STENCIL_BACK_WRITEMASK, &result->ints[result->intCount++]);
				glGetIntegerv(GL_STENCIL_BACK_VALUE_MASK, &result->ints[result->intCount++]);
				glGetIntegerv(GL_STENCIL_BACK_REF, &result->ints[result->intCount++]);
				glGetIntegerv(GL_STENCIL_BACK_FUNC, &result->ints[result->intCount++]);
			}
			return;
		case hashString("lineWidth"):
		case hashString("linewidth"):
			glGetFloatv(GL_LINE_WIDTH, &result->floats[result->floatCount++]);
			return;
		case hashString("pointSize"):
		case hashString("pointsize"):
			result->bools[result->boolCount++] = glIsEnabled(GL_PROGRAM_POINT_SIZE);
			glGetFloatv(GL_POINT_SIZE, &result->floats[result->floatCount++]);
			return;
		case hashString("pointSmooth"):
		case hashString("pointsmooth"): {
			GLboolean pointSmoothFlag = GL_FALSE;
			glGetBooleanv(GL_POINT_SMOOTH, &pointSmoothFlag);
			result->bools[result->boolCount++] = pointSmoothFlag;
			return;
		}
		default:
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
	}
}

static void GetScreenViewTrans(const GfxEmptyQuery*, GfxTranslateResult* result)
{
	result->error = nullptr;
	result->x = 0.0f;
	result->y = 0.0f;
	result->z = 0.0f;
}

static void SlaveMiniMap(const GfxBoolQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (minimap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}
	minimap->SetSlaveMode(query->value);
}

static void ConfigMiniMap(const GfxMiniMapConfigQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (minimap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}
	minimap->SetGeometry(query->px, query->py, query->sx, query->sy);
}

static void DrawMiniMap(const GfxBoolQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	if (minimap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (!minimap->GetSlaveMode()) {
		result->error = &OPERATION_FAILED_ERROR;
		return;
	}

	if (query->value) {
		glPushMatrix();
		glScalef(globalRendering->viewSizeX, globalRendering->viewSizeY, 1.0f);
		minimap->DrawForReal(true, false, true);
		glPopMatrix();
	} else {
		minimap->DrawForReal(false, false, true);
	}
}

static void ObjectTextures(const CSolidObject* obj, bool push, GfxEmptyResult* result)
{
	if (obj == nullptr || obj->model == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	push ? CModelDrawerHelper::PushModelRenderState(obj->model) : CModelDrawerHelper::PopModelRenderState(obj->model);
}

static void ObjectShapeTextures(const SolidObjectDef* def, bool push, GfxEmptyResult* result)
{
	if (def == nullptr || def->LoadModel() == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	push ? CModelDrawerHelper::PushModelRenderState(def->model) : CModelDrawerHelper::PopModelRenderState(def->model);
}

static void ObjectShape(const SolidObjectDef* def, const GfxObjectShapeQuery* query, GfxEmptyResult* result)
{
	if (def == nullptr || def->LoadModel() == nullptr || unitDrawer == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	if (query->opaque) {
		unitDrawer->DrawIndividualDefOpaque(def, query->teamID, query->rawState, query->toScreen);
	} else {
		unitDrawer->DrawIndividualDefAlpha(def, query->teamID, query->rawState, query->toScreen);
	}
}

static const LocalModelPiece* ObjectPiece(const CSolidObject* obj, int32_t pieceID, GfxEmptyResult* result)
{
	if (obj == nullptr || !obj->localModel.HasPiece(pieceID)) {
		result->error = &NOT_FOUND_ERROR;
		return nullptr;
	}

	const LocalModelPiece* lmp = obj->localModel.GetPiece(pieceID);
	if (lmp == nullptr || lmp->original == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return nullptr;
	}

	return lmp;
}

static void DrawObjectPiece(const CSolidObject* obj, int32_t pieceID, GfxEmptyResult* result)
{
	const LocalModelPiece* lmp = ObjectPiece(obj, pieceID, result);
	if (lmp == nullptr)
		return;

	S3DModelHelpers::BindLegacyAttrVBOs();
	lmp->original->DrawElements();
	S3DModelHelpers::UnbindLegacyAttrVBOs();
}

static void MultObjectPieceMatrix(const CSolidObject* obj, int32_t pieceID, GfxEmptyResult* result)
{
	const LocalModelPiece* lmp = ObjectPiece(obj, pieceID, result);
	if (lmp != nullptr)
		glMultMatrixf(lmp->GetModelSpaceMatrix());
}

static void UnitCommon(const GfxUnitDrawQuery* query, bool applyTransform, bool defaultNoLuaCall, GfxEmptyResult* result)
{
	result->error = nullptr;
	CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr || unitDrawer == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	glPushAttrib(GL_ENABLE_BIT);
	if (query->doRawDraw) {
		if (applyTransform) {
			unitDrawer->DrawUnitTrans(unit, 0, 0, query->fullModel, query->noLuaCall);
		} else {
			unitDrawer->DrawUnitNoTrans(unit, 0, 0, query->fullModel, query->noLuaCall || defaultNoLuaCall);
		}
	} else {
		if (applyTransform) {
			unitDrawer->DrawIndividual(unit, query->noLuaCall);
		} else {
			unitDrawer->DrawIndividualNoTrans(unit, query->noLuaCall || defaultNoLuaCall);
		}
	}
	glPopAttrib();
}

static void Unit(const GfxUnitDrawQuery* query, GfxEmptyResult* result) { UnitCommon(query, true, false, result); }
static void UnitRaw(const GfxUnitDrawQuery* query, GfxEmptyResult* result) { UnitCommon(query, false, true, result); }

static void UnitTextures(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	ObjectTextures(unitHandler.GetUnit(query->objectID), query->push, result);
}

static void UnitShape(const GfxObjectShapeQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	ObjectShape(unitDefHandler != nullptr ? unitDefHandler->GetUnitDefByID(query->defID) : nullptr, query, result);
}

static void UnitShapeTextures(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	ObjectShapeTextures(unitDefHandler != nullptr ? unitDefHandler->GetUnitDefByID(query->objectID) : nullptr, query->push, result);
}

static void UnitMultMatrix(const GfxIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const CUnit* unit = unitHandler.GetUnit(query->value);
	if (unit == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	glMultMatrixf(unit->GetTransformMatrix());
}

static void UnitPiece(const GfxObjectPieceQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	DrawObjectPiece(unitHandler.GetUnit(query->objectID), query->pieceID, result);
}

static void UnitPieceMatrix(const GfxObjectPieceQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	MultObjectPieceMatrix(unitHandler.GetUnit(query->objectID), query->pieceID, result);
}

static void UnitPieceMultMatrix(const GfxObjectPieceQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	MultObjectPieceMatrix(unitHandler.GetUnit(query->objectID), query->pieceID, result);
}

static void FeatureCommon(const GfxFeatureDrawQuery* query, bool applyTransform, bool defaultNoLuaCall, GfxEmptyResult* result)
{
	result->error = nullptr;
	CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr || feature->model == nullptr || featureDrawer == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	glPushAttrib(GL_ENABLE_BIT);
	if (query->doRawDraw) {
		if (applyTransform) {
			featureDrawer->DrawFeatureTrans(feature, 0, 0, false, query->noLuaCall);
		} else {
			featureDrawer->DrawFeatureNoTrans(feature, 0, 0, false, query->noLuaCall || defaultNoLuaCall);
		}
	} else {
		if (applyTransform) {
			featureDrawer->DrawIndividual(feature, query->noLuaCall);
		} else {
			featureDrawer->DrawIndividualNoTrans(feature, query->noLuaCall || defaultNoLuaCall);
		}
	}
	glPopAttrib();
}

static void Feature(const GfxFeatureDrawQuery* query, GfxEmptyResult* result) { FeatureCommon(query, true, false, result); }
static void FeatureRaw(const GfxFeatureDrawQuery* query, GfxEmptyResult* result) { FeatureCommon(query, false, true, result); }

static void FeatureTextures(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	ObjectTextures(featureHandler.GetFeature(query->objectID), query->push, result);
}

static void FeatureShape(const GfxObjectShapeQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	ObjectShape(featureDefHandler != nullptr ? featureDefHandler->GetFeatureDefByID(query->defID) : nullptr, query, result);
}

static void FeatureShapeTextures(const GfxObjectTextureStateQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	ObjectShapeTextures(featureDefHandler != nullptr ? featureDefHandler->GetFeatureDefByID(query->objectID) : nullptr, query->push, result);
}

static void FeatureMultMatrix(const GfxIntQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const CFeature* feature = featureHandler.GetFeature(query->value);
	if (feature == nullptr) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}
	glMultMatrixf(feature->GetTransformMatrixRef());
}

static void FeaturePiece(const GfxObjectPieceQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	DrawObjectPiece(featureHandler.GetFeature(query->objectID), query->pieceID, result);
}

static void FeaturePieceMatrix(const GfxObjectPieceQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	MultObjectPieceMatrix(featureHandler.GetFeature(query->objectID), query->pieceID, result);
}

static void FeaturePieceMultMatrix(const GfxObjectPieceQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	MultObjectPieceMatrix(featureHandler.GetFeature(query->objectID), query->pieceID, result);
}

static const CUnit* DrawUnitForTransform(int32_t unitID)
{
	const CUnit* unit = unitHandler.GetUnit(unitID);
	while (unit != nullptr && unit->GetTransporter() != nullptr) {
		unit = unit->GetTransporter();
	}
	return unit;
}

static void DrawListAtUnit(const GfxDrawListAtUnitQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const CUnit* unit = DrawUnitForTransform(query->unitID);
	const auto listIt = nativeDisplayLists.find(query->listID);
	if (unit == nullptr || listIt == nativeDisplayLists.end()) {
		result->error = &NOT_FOUND_ERROR;
		return;
	}

	const float3 drawPos = query->useMidPos ? unit->drawMidPos : unit->drawPos;
	glPushMatrix();
	glTranslatef(drawPos.x, drawPos.y, drawPos.z);
	glRotatef(query->degrees, query->rot.x, query->rot.y, query->rot.z);
	glScalef(query->scale.x, query->scale.y, query->scale.z);
	glCallList(listIt->second);
	glPopMatrix();
}

static void DrawFuncAtUnit(const GfxDrawFuncAtUnitQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	const CUnit* unit = DrawUnitForTransform(query->unitID);
	if (unit == nullptr || query->callback == nullptr) {
		result->error = unit == nullptr ? &NOT_FOUND_ERROR : &INVALID_ARGUMENT_ERROR;
		return;
	}

	const float3 drawPos = query->useMidPos ? unit->drawMidPos : unit->drawPos;
	glPushMatrix();
	glTranslatef(drawPos.x, drawPos.y, drawPos.z);
	query->callback(query->userData);
	glPopMatrix();
}

static void MatrixMode(const GfxMatrixModeQuery* query, GfxEmptyResult* result) { result->error = nullptr; glMatrixMode(query->mode); }
static void LoadIdentity(const GfxEmptyQuery*, GfxEmptyResult* result) { result->error = nullptr; glLoadIdentity(); }
static void LoadMatrix(const GfxMatrixQuery* query, GfxEmptyResult* result) { result->error = nullptr; glLoadMatrixf(query->values); }
static void MultMatrix(const GfxMatrixQuery* query, GfxEmptyResult* result) { result->error = nullptr; glMultMatrixf(query->values); }
static void PushMatrix(const GfxEmptyQuery*, GfxEmptyResult* result) { result->error = nullptr; glPushMatrix(); }
static void PopMatrix(const GfxEmptyQuery*, GfxEmptyResult* result) { result->error = nullptr; glPopMatrix(); }
static void Translate(const GfxTranslateQuery* query, GfxEmptyResult* result) { result->error = nullptr; glTranslatef(query->x, query->y, query->z); }
static void Scale(const GfxScaleQuery* query, GfxEmptyResult* result) { result->error = nullptr; glScalef(query->x, query->y, query->z); }
static void Rotate(const GfxRotateQuery* query, GfxEmptyResult* result) { result->error = nullptr; glRotatef(query->degrees, query->x, query->y, query->z); }
static void Ortho(const GfxOrthoQuery* query, GfxEmptyResult* result) { result->error = nullptr; glOrtho(query->left, query->right, query->bottom, query->top, query->nearVal, query->farVal); }
static void Frustum(const GfxFrustumQuery* query, GfxEmptyResult* result) { result->error = nullptr; glFrustum(query->left, query->right, query->bottom, query->top, query->nearVal, query->farVal); }

static void GetMatrixData(const GfxGetMatrixDataQuery* query, GfxGetMatrixDataResult* result)
{
	result->error = nullptr;
	std::fill(std::begin(result->values), std::end(result->values), 0.0f);
	glGetFloatv(MatrixModeToPName(query->mode), result->values);
}

static void Vertex(const GfxVertexQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	switch (query->count) {
		case 2: glVertex2f(query->x, query->y); return;
		case 4: glVertex4f(query->x, query->y, query->z, query->w); return;
		default: glVertex3f(query->x, query->y, query->z); return;
	}
}

static void Normal(const GfxTranslateQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glNormal3f(query->x, query->y, query->z);
}

static void TexCoord(const GfxVertexQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	switch (query->count) {
		case 1: glTexCoord1f(query->x); return;
		case 2: glTexCoord2f(query->x, query->y); return;
		case 3: glTexCoord3f(query->x, query->y, query->z); return;
		default: glTexCoord4f(query->x, query->y, query->z, query->w); return;
	}
}

static void MultiTexCoord(const GfxMultiTexCoordQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (globalRendering != nullptr && (query->texNum < 0 || query->texNum >= CGlobalRendering::MAX_TEXTURE_UNITS)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const GLenum texUnit = GL_TEXTURE0 + query->texNum;
	switch (query->count) {
		case 1: glMultiTexCoord1f(texUnit, query->s); return;
		case 2: glMultiTexCoord2f(texUnit, query->s, query->t); return;
		case 3: glMultiTexCoord3f(texUnit, query->s, query->t, query->r); return;
		default: glMultiTexCoord4f(texUnit, query->s, query->t, query->r, query->q); return;
	}
}

static void Color(const GfxColorQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glColor4f(query->r, query->g, query->b, query->a);
}

static void SecondaryColor(const GfxTranslateQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glSecondaryColor3f(query->x, query->y, query->z);
}

static void FogCoord(const GfxFloatQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glFogCoordf(query->value);
}

static void EdgeFlag(const GfxBoolQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glEdgeFlag(query->value);
}

static void Rect(const GfxRectQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glRectf(query->x1, query->y1, query->x2, query->y2);
}

static void TexRect(const GfxTexRectQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;
	glBegin(GL_QUADS);
	glTexCoord2f(query->s1, query->t1); glVertex2f(query->x1, query->y1);
	glTexCoord2f(query->s2, query->t1); glVertex2f(query->x2, query->y1);
	glTexCoord2f(query->s2, query->t2); glVertex2f(query->x2, query->y2);
	glTexCoord2f(query->s1, query->t2); glVertex2f(query->x1, query->y2);
	glEnd();
}

static void Shape(const GfxShapeQuery* query, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (query->vertices == nullptr && query->vertexCount > 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	glBegin(query->primitive);
	for (uint32_t i = 0; i < query->vertexCount; ++i) {
		const GfxVertexData& vertex = query->vertices[i];
		if (vertex.hasColor)
			glColor4fv(vertex.color);
		if (vertex.hasTexCoord)
			glTexCoord2fv(vertex.texCoord);
		if (vertex.hasNormal)
			glNormal3fv(vertex.normal);
		if (vertex.hasVertex)
			glVertex3fv(vertex.vertex);
	}
	glEnd();
}

static void Billboard(const GfxEmptyQuery*, GfxEmptyResult* result)
{
	result->error = nullptr;

	if (camera == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	glMultMatrixf(camera->GetBillBoardMatrix());
}

} // namespace

const GfxApi GFX_API = {
	.HasExtension = HasExtension,
	.GetNumber = GetNumber,
	.GetString = GetString,
	.GetViewSizes = GetViewSizes,
	.GetViewRange = GetViewRange,
	.GetShadowMapParams = GetShadowMapParams,
	.GetAtmosphere = GetAtmosphere,
	.GetSun = GetSun,
	.GetWaterRendering = GetWaterRendering,
	.GetMapRendering = GetMapRendering,
	.ResetState = ResetState,
	.Clear = Clear,
	.Flush = Flush,
	.Finish = Finish,
	.SwapBuffers = SwapBuffers,
	.ResetMatrices = ResetMatrices,
	.DepthTest = DepthTest,
	.DepthMask = DepthMask,
	.Culling = Culling,
	.Blending = Blending,
	.BlendFunc = BlendFunc,
	.BlendFuncSeparate = BlendFuncSeparate,
	.BlendEquation = BlendEquation,
	.BlendEquationSeparate = BlendEquationSeparate,
	.ColorMask = ColorMask,
	.AlphaTest = AlphaTest,
	.AlphaToCoverage = AlphaToCoverage,
	.StencilTest = StencilTest,
	.StencilFunc = StencilFunc,
	.StencilFuncSeparate = StencilFuncSeparate,
	.StencilMask = StencilMask,
	.StencilMaskSeparate = StencilMaskSeparate,
	.StencilOp = StencilOp,
	.StencilOpSeparate = StencilOpSeparate,
	.PolygonMode = PolygonMode,
	.PolygonOffset = PolygonOffset,
	.LogicOp = LogicOp,
	.ShadeModel = ShadeModel,
	.Scissor = Scissor,
	.Viewport = Viewport,
	.LineWidth = LineWidth,
	.LineStipple = LineStipple,
	.PointSize = PointSize,
	.PointSprite = PointSprite,
	.PointParameter = PointParameter,
	.ClipPlane = ClipPlane,
	.ClipDistance = ClipDistance,
	.PushAttrib = PushAttrib,
	.PopAttrib = PopAttrib,
	.DepthClamp = DepthClamp,
	.Fog = Fog,
	.Lighting = Lighting,
	.Light = Light,
	.Material = Material,
	.TexEnv = TexEnv,
	.TextEnv = TexEnv,
	.MultiTexEnv = MultiTexEnv,
	.TexGen = TexGen,
	.MultiTexGen = MultiTexGen,
	.DispatchCompute = DispatchCompute,
	.MemoryBarrier = MemoryBarrier,
	.ActiveTexture = ActiveTexture,
	.ObjectLabel = ObjectLabel,
	.PushDebugGroup = PushDebugGroup,
	.PopDebugGroup = PopDebugGroup,
	.CreateShader = CreateShader,
	.DeleteShader = DeleteShader,
	.UseShader = UseShader,
	.ActiveShader = ActiveShader,
	.GetShaderLog = GetShaderLog,
	.GetUniformLocation = GetUniformLocation,
	.GetActiveUniforms = GetActiveUniforms,
	.Uniform = Uniform,
	.UniformInt = UniformInt,
	.UniformArrayFloat = UniformArrayFloat,
	.UniformArrayInt = UniformArrayInt,
	.UniformMatrix = UniformMatrix,
	.GetSubroutineIndex = GetSubroutineIndex,
	.UniformSubroutine = UniformSubroutine,
	.SetGeometryShaderParameter = SetGeometryShaderParameter,
	.SetTesselationShaderParameter = SetTesselationShaderParameter,
	.GetEngineUniformBufferDef = GetEngineUniformBufferDef,
	.GetEngineModelUniformDataDef = GetEngineModelUniformDataDef,
	.GetEngineModelUniformDataSize = GetEngineModelUniformDataSize,
	.SetUnitBufferUniforms = SetUnitBufferUniforms,
	.SetFeatureBufferUniforms = SetFeatureBufferUniforms,
	.CreateTexture = CreateTexture,
	.DeleteTexture = DeleteTexture,
	.DeleteTextureFBO = DeleteTextureFBO,
	.BindTexture = BindTexture,
	.TextureInfo = TextureInfo,
	.GetEngineTextureNames = GetEngineTextureNames,
	.GetConsoleCommands = GetConsoleCommands,
	.ChangeTextureParams = ChangeTextureParams,
	.CopyToTexture = CopyToTexture,
	.UploadTexture = UploadTexture,
	.GenerateMipmap = GenerateMipmap,
	.BindImageTexture = BindImageTexture,
	.ReadPixels = ReadPixels,
	.CreateRBO = CreateRBO,
	.DeleteRBO = DeleteRBO,
	.CreateFBO = CreateFBO,
	.DeleteFBO = DeleteFBO,
	.IsValidFBO = IsValidFBO,
	.ActiveFBO = ActiveFBO,
	.RawBindFBO = RawBindFBO,
	.BlitFBO = BlitFBO,
	.ClearAttachmentFBO = ClearAttachmentFBO,
	.GetVAO = GetVAO,
	.DeleteVAO = DeleteVAO,
	.GetVBO = GetVBO,
	.DeleteVBO = DeleteVBO,
	.RenderToTexture = RenderToTexture,
	.CreateTextureAtlas = CreateTextureAtlas,
	.FinalizeTextureAtlas = FinalizeTextureAtlas,
	.DeleteTextureAtlas = DeleteTextureAtlas,
	.AddAtlasTexture = AddAtlasTexture,
	.GetAtlasTexture = GetAtlasTexture,
	.GetEngineAtlasTextures = GetEngineAtlasTextures,
	.SaveImage = SaveImage,
	.CreateList = CreateList,
	.CallList = CallList,
	.DeleteList = DeleteList,
	.CreateQuery = CreateQuery,
	.DeleteQuery = DeleteQuery,
	.RunQuery = RunQuery,
	.GetQuery = GetQuery,
	.GetGlobalTexNames = GetGlobalTexNames,
	.GetGlobalTexCoords = GetGlobalTexCoords,
	.BeginText = BeginText,
	.Text = Text,
	.EndText = EndText,
	.GetTextWidth = GetTextWidth,
	.GetTextHeight = GetTextHeight,
	.AddFallbackFont = AddFallbackFont,
	.ClearFallbackFonts = ClearFallbackFonts,
	.LoadFont = LoadFont,
	.DeleteFont = DeleteFont,
	.GetFontInfo = GetFontInfo,
	.FontBegin = FontBegin,
	.FontEnd = FontEnd,
	.FontPrint = FontPrint,
	.FontPrintWorld = FontPrintWorld,
	.FontSubmitBuffered = FontSubmitBuffered,
	.FontWrapText = FontWrapText,
	.FontGetTextWidth = FontGetTextWidth,
	.FontGetTextHeight = FontGetTextHeight,
	.FontSetTextColor = FontSetTextColor,
	.FontSetOutlineColor = FontSetOutlineColor,
	.FontSetAutoOutlineColor = FontSetAutoOutlineColor,
	.FontBindTexture = FontBindTexture,
	.BeginEnd = BeginEnd,
	.PushPopMatrix = PushPopMatrix,
	.UnsafeState = UnsafeState,
	.DrawGroundCircle = DrawGroundCircle,
	.DrawGroundQuad = DrawGroundQuad,
	.GetFixedState = GetFixedState,
	.GetScreenViewTrans = GetScreenViewTrans,
	.SlaveMiniMap = SlaveMiniMap,
	.ConfigMiniMap = ConfigMiniMap,
	.DrawMiniMap = DrawMiniMap,
	.Unit = Unit,
	.UnitRaw = UnitRaw,
	.UnitTextures = UnitTextures,
	.UnitShape = UnitShape,
	.UnitShapeTextures = UnitShapeTextures,
	.UnitMultMatrix = UnitMultMatrix,
	.UnitPiece = UnitPiece,
	.UnitPieceMatrix = UnitPieceMatrix,
	.UnitPieceMultMatrix = UnitPieceMultMatrix,
	.Feature = Feature,
	.FeatureRaw = FeatureRaw,
	.FeatureTextures = FeatureTextures,
	.FeatureShape = FeatureShape,
	.FeatureShapeTextures = FeatureShapeTextures,
	.FeatureMultMatrix = FeatureMultMatrix,
	.FeaturePiece = FeaturePiece,
	.FeaturePieceMatrix = FeaturePieceMatrix,
	.FeaturePieceMultMatrix = FeaturePieceMultMatrix,
	.DrawListAtUnit = DrawListAtUnit,
	.DrawFuncAtUnit = DrawFuncAtUnit,
	.MatrixMode = MatrixMode,
	.LoadIdentity = LoadIdentity,
	.LoadMatrix = LoadMatrix,
	.MultMatrix = MultMatrix,
	.PushMatrix = PushMatrix,
	.PopMatrix = PopMatrix,
	.Translate = Translate,
	.Scale = Scale,
	.Rotate = Rotate,
	.Ortho = Ortho,
	.Frustum = Frustum,
	.GetMatrixData = GetMatrixData,
	.Vertex = Vertex,
	.Normal = Normal,
	.TexCoord = TexCoord,
	.MultiTexCoord = MultiTexCoord,
	.Color = Color,
	.SecondaryColor = SecondaryColor,
	.FogCoord = FogCoord,
	.EdgeFlag = EdgeFlag,
	.Rect = Rect,
	.TexRect = TexRect,
	.Shape = Shape,
	.Billboard = Billboard,
};

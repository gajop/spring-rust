/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */


#include "Rendering/GL/myGL.h"
#include "S3OTextureHandler.h"

#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/SimpleParser.h"
#include "Rendering/ShadowHandler.h"
#include "Rendering/Units/UnitDrawer.h"
#include "Rendering/Models/3DModel.hpp"
#include "Rendering/Models/ModelsLock.h"
#include "Rendering/Textures/Bitmap.h"
#include "System/StringUtil.h"
#include "System/Exceptions.h"
#include "System/Log/ILog.h"
#include "System/Config/ConfigHandler.h"
#include "System/Platform/Threading.h"
#include "System/Threading/ThreadPool.h"

#include <algorithm>
#include <cctype>
#include <set>
#include <sstream>

#include "System/Misc/TracyDefs.h"

#define LOG_SECTION_TEXTURE "Texture"
LOG_REGISTER_SECTION_GLOBAL(LOG_SECTION_TEXTURE)
#ifdef LOG_SECTION_CURRENT
        #undef LOG_SECTION_CURRENT
#endif
#define LOG_SECTION_CURRENT LOG_SECTION_TEXTURE

#define TEX_MAT_UID(pTxID, sTxID) ((std::uint64_t(pTxID) << 32u) | sTxID)


// The S3O texture handler uses two textures.
// The first contains diffuse color (RGB) and teamcolor (A)
// The second contains glow (R), reflectivity (G) and 1-bit Alpha (A).

//////////////////////////////////////////////////////////////////////
// Construction/Destruction
//////////////////////////////////////////////////////////////////////

CS3OTextureHandler textureHandlerS3O;

CONFIG(bool, ModelTextureStreaming)
	.defaultValue(false)
	.description("Decode the maps of a model loaded while the game runs (a unit or feature made whose model nobody queued) on the worker threads instead of in the frame that makes it: the model first draws with a single-pixel map, which is replaced, one map a frame, as each is decoded. Off, the frame waits for the decode (tens of milliseconds a 4096 x 4096 map). Models queued with PreloadModel are not affected.");

void CS3OTextureHandler::Init()
{
	RECOIL_DETAILED_TRACY_ZONE;
	textures.reserve(128);

	// dummies
	textures.emplace_back();
	textures.emplace_back();
}

void CS3OTextureHandler::Kill()
{
	RECOIL_DETAILED_TRACY_ZONE;
	// a decode in flight uses the bitmap memory pool: let it finish
	for (StreamedTex& tex: streaming) {
		if (tex.job.valid())
			tex.job.wait();
	}
	streaming.clear();

	for (S3OTexMat& texture: textures) {
		glDeleteTextures(1, &(texture.tex1));
		glDeleteTextures(1, &(texture.tex2));
	}

	textures.clear();
	textureCache.clear();
	textureTable.clear();
	bitmapCache.clear();
}

// TextureMipSkip and TextureMipFloor, as for a model's DDS textures: an image the engine decodes
// itself (PNG, TGA, ...) leaves out its finest levels too, box-filtered down, its longer side never
// below the floor. A large decoded map (a planet's, 4096 x 2048 RGBA8 with its levels: 43 MiB)
// otherwise kept its full size at every preset.
static void SkipFinestLevels(CBitmap& bitmap)
{
#ifndef HEADLESS
	if (bitmap.compressed)
		return;

	const unsigned int floor = configHandler->GetInt("TextureMipFloor");
	const int skip = TextureMipSkipLevels(std::max(bitmap.xsize, bitmap.ysize), floor);
	if (skip > 0)
		bitmap = bitmap.CreateRescaled(std::max(1, bitmap.xsize >> skip), std::max(1, bitmap.ysize >> skip));
#endif
}

void CS3OTextureHandler::Reload()
{
	RECOIL_DETAILED_TRACY_ZONE;
	auto lock = CModelsLock::GetScopedLock(); //needed?

	for (auto& [texName, texData] : textureCache) {
		if (texData.texID == 0)
			continue;

		CBitmap bitmap;
		if (!bitmap.Load(texName) && !bitmap.Load("unittextures/" + texName))
			continue;

		{
			if (texData.invertAlpha)
				bitmap.InvertAlpha();
			if (texData.invertAxis)
				bitmap.ReverseYAxis();
			SkipFinestLevels(bitmap);

			uint32_t newTexId = bitmap.CreateMipMapTexture(0.0f, 0.0f, 0, texData.texID);
			assert(newTexId == texData.texID);
		}
	}
}


void CS3OTextureHandler::PreloadTexture(S3DModel* model, bool invertAxis, bool invertAlpha)
{
	RECOIL_DETAILED_TRACY_ZONE;

	// A model's maps are decoded without the models lock, which main-thread work needs at any time
	// (a unit or feature made, a model uploaded): decoding a 4096 x 4096 map takes tens of
	// milliseconds, and the lock is held only to look in and fill the caches.
	const bool invertAlphas[2] = {invertAlpha, false}; // never invert alpha for tex2
	// on the main thread (a model nobody queued, loaded as a unit or feature is made) the maps are
	// decoded on the worker threads and stand in as single pixels until they are
#ifndef HEADLESS
	const bool stream = Threading::IsMainThread() && ThreadPool::HasThreads() && configHandler->GetBool("ModelTextureStreaming");
#else
	const bool stream = false;
#endif
	for (unsigned int texNum = 0; texNum < 2; ++texNum) {
		const std::string textureName = model->texs[texNum];
		{
			auto lock = CModelsLock::GetScopedLock();
			const auto textureIt = textureCache.find(textureName);
			if ((textureIt != textureCache.end() && textureIt->second.texID > 0) || bitmapCache.find(textureName) != bitmapCache.end())
				continue;
		}

		if (stream && StreamTexture(model, texNum, invertAxis, invertAlphas[texNum]))
			continue;

		CBitmap bitmap;
		if (!bitmap.Load(textureName) && !bitmap.Load("unittextures/" + textureName)) {
			if (texNum == 0)
				LOG_L(L_WARNING, "[%s] could not load primary texture \"%s\" from model \"%s\"", __func__, textureName.c_str(), model->name.c_str());

			// file not found (or headless build), set a single pixel so model is visible
			bitmap.AllocDummy(SColor(255 * (texNum == 0), 0, 0, 255 * (1 - invertAlphas[texNum])));
		}

		if (invertAxis)
			bitmap.ReverseYAxis();
		if (invertAlphas[texNum])
			bitmap.InvertAlpha();
		SkipFinestLevels(bitmap);

		auto lock = CModelsLock::GetScopedLock();
		bitmapCache.emplace(textureName, std::move(bitmap)); // another thread's copy of the same map wins
		LoadAndCacheTexture(model, texNum, invertAxis, invertAlphas[texNum], true);
	}
}


bool CS3OTextureHandler::StreamTexture(const S3DModel* model, unsigned int texNum, bool invertAxis, bool invertAlpha)
{
	RECOIL_DETAILED_TRACY_ZONE;
	const std::string& textureName = model->texs[texNum];

	auto lock = CModelsLock::GetScopedLock();
	for (const StreamedTex& tex: streaming) {
		if (tex.name == textureName)
			return true; // another model's map, already coming
	}

	StreamedTex tex;
	tex.name = textureName;
	tex.invertAxis = invertAxis;
	tex.invertAlpha = invertAlpha;
	tex.image = std::make_shared<StreamedTex::Image>();
	tex.job = ThreadPool::Enqueue([image = tex.image, textureName, modelName = model->name, texNum, invertAxis, invertAlpha]() {
		CBitmap& bitmap = image->bitmap;
		if (!bitmap.Load(textureName) && !bitmap.Load("unittextures/" + textureName)) {
			if (texNum == 0)
				LOG_L(L_WARNING, "[CS3OTextureHandler::StreamTexture] could not load primary texture \"%s\" from model \"%s\"", textureName.c_str(), modelName.c_str());

			// file not found, set a single pixel so model is visible
			bitmap.AllocDummy(SColor(255 * (texNum == 0), 0, 0, 255 * (1 - invertAlpha)));
		}

		if (invertAxis)
			bitmap.ReverseYAxis();
		if (invertAlpha)
			bitmap.InvertAlpha();
		SkipFinestLevels(bitmap);
	});

	// what LoadAndCacheTexture fills in when the model goes up; the map's size is not known yet
	textureCache.emplace(textureName, CachedS3OTex{0, 1, 1, invertAxis, invertAlpha});
	streaming.push_back(std::move(tex));
	return true;
}

// Makes the texture of a streamed map from its decoded image: in `tex.texID` when the
// model's material already uses a stand-in for it, a new one otherwise.
// The caller holds the models lock.
unsigned int CS3OTextureHandler::FinishStreamedTexture(StreamedTex& tex)
{
	RECOIL_DETAILED_TRACY_ZONE;
	const CBitmap& bitmap = tex.image->bitmap;
	const auto tdbg0 = spring_gettime();
	const unsigned int texID = bitmap.CreateMipMapTexture(0.0f, 0.0f, 0, tex.texID);
#ifndef HEADLESS
	if (texID > 0 && glObjectLabel != nullptr)
		glObjectLabel(GL_TEXTURE, texID, -1, ("model texture: " + tex.name).c_str());
#endif

	textureCache[tex.name] = {
		texID,
		static_cast<uint32_t>(bitmap.xsize),
		static_cast<uint32_t>(bitmap.ysize),
		tex.invertAxis,
		tex.invertAlpha
	};

	for (S3OTexMat& mat: textures) {
		if (mat.tex1 == texID) {
			mat.tex1SizeX = bitmap.xsize;
			mat.tex1SizeY = bitmap.ysize;
		}
		if (mat.tex2 == texID) {
			mat.tex2SizeX = bitmap.xsize;
			mat.tex2SizeY = bitmap.ysize;
		}
	}

	return texID;
}

void CS3OTextureHandler::Update()
{
	RECOIL_DETAILED_TRACY_ZONE;
	if (streaming.empty())
		return;

	// the loading threads hold the lock while they fill the caches; nothing is lost by trying next frame
	auto lock = CModelsLock::GetTryLock();
	if (!lock.owns_lock())
		return;

	// one map a frame: each is an upload of its whole chain of levels
	for (auto it = streaming.begin(); it != streaming.end(); ++it) {
		if (it->texID == 0)
			continue; // its model is not on the GPU yet
		if (it->job.wait_for(std::chrono::seconds(0)) != std::future_status::ready)
			continue;

		FinishStreamedTexture(*it);
		streaming.erase(it);
		return;
	}
}

void CS3OTextureHandler::LoadTexture(S3DModel* model)
{
	RECOIL_DETAILED_TRACY_ZONE;
	auto lock = CModelsLock::GetScopedLock();

	const unsigned int tex1ID = LoadAndCacheTexture(model, 0, false, false, false);
	const unsigned int tex2ID = LoadAndCacheTexture(model, 1, false, false, false);

	const auto texTableIter = textureTable.find(TEX_MAT_UID(tex1ID, tex2ID));

	// even if both textures were already loaded as parts of
	// other models, their pair might form a unique material
	if (texTableIter == textureTable.end()) {
		model->textureType = InsertTextureMat(model);
	} else {
		model->textureType = texTableIter->second;
	}
}

unsigned int CS3OTextureHandler::LoadAndCacheTexture(
	const S3DModel* model,
	unsigned int texNum,
	bool invertAxis,
	bool invertAlpha,
	bool preloadCall
) {
	RECOIL_DETAILED_TRACY_ZONE;
	CBitmap* bitmap = nullptr;

	const auto& textureName = model->texs[texNum];
	const auto textureIt = textureCache.find(textureName);

	if (textureIt != textureCache.end() && textureIt->second.texID > 0)
		return textureIt->second.texID;

	if (!preloadCall) {
		const auto streamed = std::find_if(streaming.begin(), streaming.end(), [&](const StreamedTex& tex) { return tex.name == textureName; });
		if (streamed != streaming.end()) {
			// decoded already: made now. Not yet: a single pixel stands in, Update() replaces it
			if (streamed->job.wait_for(std::chrono::seconds(0)) == std::future_status::ready) {
				const unsigned int texID = FinishStreamedTexture(*streamed);
				streaming.erase(streamed);
				return texID;
			}

			CBitmap standIn;
			standIn.AllocDummy(texNum == 0 ? SColor(128, 128, 128, 255) : SColor(0, 0, 0, 255));
			streamed->texID = standIn.CreateMipMapTexture();
#ifndef HEADLESS
			if (streamed->texID > 0 && glObjectLabel != nullptr)
				glObjectLabel(GL_TEXTURE, streamed->texID, -1, ("model texture (stand-in): " + textureName).c_str());
#endif
			textureCache[textureName].texID = streamed->texID;
			return streamed->texID;
		}
	}

	const auto bitmapIt = bitmapCache.find(textureName);

	if (bitmapIt != bitmapCache.end()) {
		// bitmap was previously preloaded but not yet loaded
		// if !preloading, we will now turn the bitmap into a
		// texture and cache it
		bitmap = &(bitmapIt->second);
	} else {
		// bitmap was not yet preloaded, meaning we are the
		// first to (all non-3DO model textures are always
		// preloaded)
#ifndef HEADLESS //?
		assert(preloadCall);
#endif

		auto pair = bitmapCache.emplace(textureName, CBitmap{});
		auto iter = pair.first;

		bitmap = &(iter->second);

		if (!bitmap->Load(textureName) && !bitmap->Load("unittextures/" + textureName)) {
			if (texNum == 0)
				LOG_L(L_WARNING, "[%s] could not load primary texture \"%s\" from model \"%s\"", __func__, textureName.c_str(), model->name.c_str());

			// file not found (or headless build), set a single pixel so model is visible
			bitmap->AllocDummy(SColor(255 * (texNum == 0), 0, 0, 255 * (1 - invertAlpha)));
		}

		if (invertAxis)
			bitmap->ReverseYAxis();
		if (invertAlpha)
			bitmap->InvertAlpha();
		SkipFinestLevels(*bitmap);
	}

	const unsigned int texID = preloadCall ? 0 : bitmap->CreateMipMapTexture();
#ifndef HEADLESS
	assert(preloadCall || texID > 0);

	// named for debuggers and debuginfo gpumemory, as named textures are
	if (texID > 0 && glObjectLabel != nullptr)
		glObjectLabel(GL_TEXTURE, texID, -1, ("model texture: " + textureName).c_str());
#endif

	if (textureIt != textureCache.end() && texID > 0) {
		assert(!preloadCall);
		textureIt->second.texID = texID;
	}
	else {
		//save main params from the preloadCall pass, such that data is stored correctly for Reload()
#ifndef HEADLESS //?
		assert( preloadCall);
#endif
		textureCache[textureName] = {
			texID,
			static_cast<uint32_t>(bitmap->xsize),
			static_cast<uint32_t>(bitmap->ysize),
			invertAxis,
			invertAlpha
		};
	}

	// don't generate a texture yet, just save the bitmap for later
	if (preloadCall)
		return 0;

	bitmapCache.erase(textureName);
	return texID;
}


unsigned int CS3OTextureHandler::InsertTextureMat(const S3DModel* model)
{
	const CachedS3OTex& tex1 = textureCache[ model->texs[0] ];
	const CachedS3OTex& tex2 = textureCache[ model->texs[1] ];

	textures.emplace_back();

	S3OTexMat& texMat = textures.back();

	texMat.num       = textures.size() - 1;
	texMat.tex1      = tex1.texID;
	texMat.tex2      = tex2.texID;
	texMat.tex1SizeX = tex1.xsize;
	texMat.tex1SizeY = tex1.ysize;
	texMat.tex2SizeX = tex2.xsize;
	texMat.tex2SizeY = tex2.ysize;

	textureTable[TEX_MAT_UID(texMat.tex1, texMat.tex2)] = texMat.num;

	return texMat.num;
}


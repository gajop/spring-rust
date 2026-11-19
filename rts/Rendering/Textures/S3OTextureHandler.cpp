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
#include "Rendering/Textures/TextureUploads.h"
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
	.description("Make the maps of a model loaded while the game runs without stalling the frame that needs them: a map nobody decoded ahead (a unit or feature made whose model nobody queued) is decoded on the worker threads, and every such map goes up over the next drawn frames within TextureUploadRate, coarsest level first. The model draws with a single-pixel map until its first level is up. Off, the frame that makes the model decodes and uploads its maps (tens of milliseconds a 4096 x 4096 map).");

// Maps smaller than this are made whole when their model goes up: they cost the frame little.
static constexpr size_t STREAM_MIN_BYTES = 1024 * 1024;

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
	// a decode in flight uses the bitmap memory pool: let it finish; a map being made is dropped
	// (its stand-in texture goes below)
	for (StreamedTex& tex: streaming) {
		if (tex.job.valid())
			tex.job.wait();
		if (tex.ticket != 0)
			TextureUploads::Cancel(tex.ticket);
	}
	streaming.clear();
	streamUploads = false;

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

// A streamed map's texture is complete: its size goes in the cache and in the materials using
// it, and it stops being streamed. The caller holds the models lock.
void CS3OTextureHandler::FinishStreamedTexture(const std::string& name, unsigned int texID, const CBitmap& bitmap)
{
	RECOIL_DETAILED_TRACY_ZONE;
#ifndef HEADLESS
	if (texID > 0 && glObjectLabel != nullptr)
		glObjectLabel(GL_TEXTURE, texID, -1, ("model texture: " + name).c_str());
#endif

	auto& cached = textureCache[name];
	cached.texID = texID;
	cached.xsize = static_cast<uint32_t>(bitmap.xsize);
	cached.ysize = static_cast<uint32_t>(bitmap.ysize);

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

	std::erase_if(streaming, [&](const StreamedTex& tex) { return tex.name == name; });
}

bool CS3OTextureHandler::StreamUploads() const
{
#ifndef HEADLESS
	return streamUploads && Threading::IsMainThread() && configHandler->GetBool("ModelTextureStreaming");
#else
	return false;
#endif
}

void CS3OTextureHandler::Update()
{
	RECOIL_DETAILED_TRACY_ZONE;
	// a game runs: the models loaded from now on are made without stalling a frame
	streamUploads = true;

	if (streaming.empty())
		return;

	// the loading threads hold the lock while they fill the caches; nothing is lost by trying next frame
	auto lock = CModelsLock::GetTryLock();
	if (!lock.owns_lock())
		return;

	for (StreamedTex& tex: streaming) {
		if (tex.texID == 0 || tex.ticket != 0)
			continue; // its model is not on the GPU yet, or it is being made
		if (tex.job.valid() && tex.job.wait_for(std::chrono::seconds(0)) != std::future_status::ready)
			continue;

		// made in the stand-in's texture, which the model's material already uses
		TextureUploads::Request request;
		request.image = std::shared_ptr<const CBitmap>(tex.image, &tex.image->bitmap);
		request.texID = tex.texID;
#ifndef HEADLESS
		request.mipSkip = static_cast<uint32_t>(configHandler->GetInt("TextureMipSkip"));
		request.mipFloor = static_cast<uint32_t>(configHandler->GetInt("TextureMipFloor"));
#endif
		request.makeWhole = [this, name = tex.name](const CBitmap& bitmap, uint32_t texID) {
			auto lock = CModelsLock::GetScopedLock();
			texID = bitmap.CreateMipMapTexture(0.0f, 0.0f, 0, texID);
			FinishStreamedTexture(name, texID, bitmap);
			return texID;
		};
		request.done = [this, name = tex.name, image = request.image](uint32_t texID) {
			auto lock = CModelsLock::GetScopedLock();
			FinishStreamedTexture(name, texID, *image);
		};
		tex.ticket = TextureUploads::Queue(std::move(request));
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
		auto streamed = std::find_if(streaming.begin(), streaming.end(), [&](const StreamedTex& tex) { return tex.name == textureName; });

		// a large map decoded ahead (a queued model's), made while the game runs: in steps too
		if (const auto bitmapIt = bitmapCache.find(textureName); streamed == streaming.end() && bitmapIt != bitmapCache.end() && StreamUploads()) {
			const CBitmap& decoded = bitmapIt->second;
#ifndef HEADLESS
			const size_t bytes = decoded.compressed? static_cast<size_t>(decoded.ddsimage.get_size()): decoded.GetMemSize();
#else
			const size_t bytes = 0;
#endif
			if (bytes >= STREAM_MIN_BYTES) {
				StreamedTex tex;
				tex.name = textureName;
				tex.invertAxis = invertAxis;
				tex.invertAlpha = invertAlpha;
				tex.image = std::make_shared<StreamedTex::Image>();
				tex.image->bitmap = std::move(bitmapIt->second);
				bitmapCache.erase(bitmapIt);
				streaming.push_back(std::move(tex));
				streamed = std::prev(streaming.end());
			}
		}

		if (streamed != streaming.end()) {
			// a single pixel stands in until Update() has the map made in it, over a few frames
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


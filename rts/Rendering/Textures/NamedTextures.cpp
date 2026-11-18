/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// must be included before streflop! else we get streflop/cmath resolve conflicts in its hash implementation files
#include <atomic>
#include <bit>
#include <chrono>
#include <future>
#include <memory>
#include <vector>
#include "NamedTextures.h"

#include "Rendering/GL/myGL.h"
#include "Bitmap.h"
#include "Rendering/GlobalRendering.h"
#include "System/type2.h"
#include "System/Log/ILog.h"
#include "System/Config/ConfigHandler.h"
#include "System/Threading/SpringThreading.h"
#include "System/Threading/ThreadPool.h"
#include "System/UnorderedMap.hpp"

#include "System/Misc/TracyDefs.h"



namespace CNamedTextures {
	// maps names to texInfoVec indices
	static spring::unordered_map<std::string, size_t> texInfoMap;

	static std::vector<CNamedTextures::TexInfo> texInfoVec;
	static std::vector<size_t> freeIndices;
	static std::vector<std::string> waitingTextures;

	static spring::recursive_mutex mutex;

	// images decoded ahead of the texture being made (Preload), by the name Load is given
	struct DecodedImage {
		CBitmap bitmap;
		bool decoded = false;
	};
	struct PreloadedImage {
		std::shared_future<void> job;
		std::shared_ptr<DecodedImage> image; // the job's own reference, so the job does not own its entry
	};
	static spring::unordered_map<std::string, std::shared_ptr<PreloadedImage>> preloaded;
	static spring::mutex preloadMutex;

	/******************************************************************************/

	void Init()
	{
	RECOIL_DETAILED_TRACY_ZONE;
		texInfoMap.clear();
		texInfoMap.reserve(128);
		texInfoVec.clear();
		texInfoVec.reserve(128);

		freeIndices.clear();

		waitingTextures.clear();
		waitingTextures.reserve(16);
	}

	void Kill(bool shutdown)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		decltype(texInfoMap) tempMap;

		{
			// a decode in flight uses the bitmap memory pool: let it finish before anything goes
			const std::lock_guard<spring::mutex> lck(preloadMutex);
			for (const auto& [texName, image]: preloaded) {
				if (image->job.valid())
					image->job.wait();
			}
			preloaded.clear();
		}

		const std::lock_guard<spring::recursive_mutex> lck(mutex);

		for (const auto& [texName, texIdx]: texInfoMap) {
			const GLuint texID = texInfoVec[texIdx].id;

			if (shutdown || !texInfoVec[texIdx].persist) {
				glDeleteTextures(1, &texID);
				// always recycle non-persistent textures
				freeIndices.push_back(texIdx);
			} else {
				tempMap[texName] = texIdx;
			}
		}

		std::swap(texInfoMap, tempMap);
		waitingTextures.clear();
	}

	void Reload()
	{
	RECOIL_DETAILED_TRACY_ZONE;
		const std::lock_guard<spring::recursive_mutex> lck(mutex); //needed?

		for (const auto& [texName, texIdx] : texInfoMap) {
			const GLuint texID = texInfoVec[texIdx].id;
			if (texID == 0)
				continue;

			Load(texName, texID, false);
		}
	}


	/******************************************************************************/

	static void InsertTex(const std::string& texName, const TexInfo& texInfo, bool loadTex)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		// caller (GenInsertTex) already has lock
		if (!loadTex)
			waitingTextures.push_back(texName);

		if (freeIndices.empty()) {
			texInfoMap[texName] = texInfoVec.size();
			texInfoVec.push_back(texInfo);
		} else {
			// recycle
			texInfoMap[texName] = freeIndices.back();
			texInfoVec[freeIndices.back()] = texInfo;
			freeIndices.pop_back();
		}
	}

	static TexInfo GenTex(bool bindTex, bool persistTex)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		GLuint texID = 0;
		glGenTextures(1, &texID);

		if (bindTex)
			glBindTexture(GL_TEXTURE_2D, texID);

		TexInfo texInfo;
		texInfo.id = texID;
		texInfo.persist = persistTex;
		return texInfo;
	}

	static void GenInsertTex(const std::string& texName, const TexInfo& texInfo, bool genTex, bool bindTex, bool loadTex, bool persistTex)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		const std::lock_guard<spring::recursive_mutex> lck(mutex);

		if (!genTex) {
			InsertTex(texName, texInfo, loadTex);
			return;
		}

		InsertTex(texName, GenTex(bindTex, persistTex), loadTex);
	}

	static bool EraseTex(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		const std::lock_guard<spring::recursive_mutex> lck(mutex);

		const auto it = texInfoMap.find(texName);

		if (it != texInfoMap.end()) {
			const size_t texIdx = it->second;
			const GLuint texID = texInfoVec[texIdx].id;

			glDeleteTextures(1, &texID);

			freeIndices.push_back(texIdx);
			texInfoMap.erase(it);
			return true;
		}

		return false;
	}



	// the qualifiers a texture name carries (":n:name": nearest filtering, a resize, ...) and the file it names
	struct NameSpec {
		std::string filename;
		bool border  = false;
		bool clamped = false;
		bool nearest = false;
		bool linear  = false;
		bool aniso   = false;
		bool invert  = false;
		bool greyed  = false;
		bool mipnear = false;
		bool tint    = false;
		float tintColor[3];
		bool resize  = false;
		int2 resizeDimensions;
	};

	static NameSpec ParseName(const std::string& texName)
	{
		NameSpec spec;
		spec.filename = texName;
		std::string& filename = spec.filename;

		if (filename[0] == ':') {
			size_t p;
			for (p = 1; p < filename.size(); p++) {
				const char ch = filename[p];

				if (ch == ':')      { break; }
				else if (ch == 'n') { spec.nearest = true; }
				else if (ch == 'l') { spec.linear  = true; }
				else if (ch == 'a') { spec.aniso   = true; }
				else if (ch == 'i') { spec.invert  = true; }
				else if (ch == 'g') { spec.greyed  = true; }
				else if (ch == 'c') { spec.clamped = true; }
				else if (ch == 'b') { spec.border  = true; }
				else if (ch == 'm') { spec.mipnear = true; }
				else if (ch == 't') {
					const char* cstr = filename.c_str() + p + 1;
					const char* start = cstr;
					char* endptr;
					spec.tintColor[0] = (float)strtod(start, &endptr);
					if ((start != endptr) && (*endptr == ',')) {
						start = endptr + 1;
						spec.tintColor[1] = (float)strtod(start, &endptr);
						if ((start != endptr) && (*endptr == ',')) {
							start = endptr + 1;
							spec.tintColor[2] = (float)strtod(start, &endptr);
							if (start != endptr) {
								spec.tint = true;
								p += (endptr - cstr);
							}
						}
					}
				}
				else if (ch == 'r') {
					const char* cstr = filename.c_str() + p + 1;
					const char* start = cstr;
					char* endptr;
					spec.resizeDimensions.x = (int)strtoul(start, &endptr, 10);
					if ((start != endptr) && (*endptr == ',')) {
						start = endptr + 1;
						spec.resizeDimensions.y = (int)strtoul(start, &endptr, 10);
						if (start != endptr) {
							spec.resize = true;
							p += (endptr - cstr);
						}
					}
				}
			}

			if (p < filename.size()) {
				filename = filename.substr(p + 1);
			} else {
				filename.clear();
			}
		}

		return spec;
	}

	// the image Preload decoded for `texName`, if there is one (waiting for a decode still running)
	static bool TakePreloaded(const std::string& texName, CBitmap& bitmap)
	{
		std::shared_ptr<PreloadedImage> image;
		{
			const std::lock_guard<spring::mutex> lck(preloadMutex);
			const auto it = preloaded.find(texName);
			if (it == preloaded.end())
				return false;

			image = it->second;
			preloaded.erase(it);
		}

		if (image->job.valid())
			image->job.wait();
		if (!image->image->decoded)
			return false;

		bitmap = std::move(image->image->bitmap);
		return true;
	}

	static bool Load(const std::string& texName, unsigned int texID, bool genInsert)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		const NameSpec spec = ParseName(texName);
		const std::string& filename = spec.filename;
		const bool nearest = spec.nearest;
		const bool linear  = spec.linear;
		const bool mipnear = spec.mipnear;

		// get the image
		CBitmap bitmap;
		TexInfo texInfo;

		if (!TakePreloaded(texName, bitmap) && !bitmap.Load(filename, 1.0f, 4u, 0u)) {
			LOG_L(L_WARNING, "Couldn't find texture \"%s\"!", filename.c_str());
			GenInsertTex(texName, texInfo, false, false, true, false);
			return false;
		}

		const bool needMipMaps = (!(nearest || linear)) || mipnear;

		GL::TextureCreationParams tcp;
		tcp.texID = texID;
		tcp.linearMipMapFilter = !mipnear;
		tcp.linearTextureFilter = !nearest;
		tcp.reqNumLevels = needMipMaps ? 0 : 1;
		if (spec.aniso)
			tcp.aniso = globalRendering->maxTexAnisoLvl;

		if (bitmap.compressed) {
			texID = bitmap.CreateDDSTexture(tcp);
		} else {
			if (spec.resize) {
				bitmap = bitmap.CreateRescaled(spec.resizeDimensions.x, spec.resizeDimensions.y);
			}
			#ifndef HEADLESS
			else if (needMipMaps) {
				// TextureMipSkip, as for DDS textures: the finest levels of a large image are
				// left out (box-filtered down), never going below 1024 texels a side (effect
				// sheets and interface images keep their detail where hull maps give theirs up)
				const int skip = TextureMipSkipLevels(std::max(bitmap.xsize, bitmap.ysize), 1024);
				if (skip > 0)
					bitmap = bitmap.CreateRescaled(std::max(1, bitmap.xsize >> skip), std::max(1, bitmap.ysize >> skip));
			}
			#endif
			if (spec.invert) bitmap.InvertColors();
			if (spec.greyed) bitmap.MakeGrayScale();
			if (spec.tint)   bitmap.Tint(spec.tintColor);

			// verify if still broken
			if (globalRendering->amdHacks && nearest) {
				bitmap = bitmap.CreateRescaled(std::bit_ceil <uint32_t>(bitmap.xsize), std::bit_ceil <uint32_t>(bitmap.ysize));
			}

			texID = bitmap.CreateTexture(tcp);

			// specify extra params
			glBindTexture(GL_TEXTURE_2D, texID);

			if (spec.clamped) {
				glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
				glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
			}

			if (spec.border) {
				GLfloat white[4] = { 1.0f, 1.0f, 1.0f, 1.0f };
				glTexParameterfv(GL_TEXTURE_2D, GL_TEXTURE_BORDER_COLOR, white);
			}

			glBindTexture(GL_TEXTURE_2D, 0);
		}

		texInfo.id    = texID;
		texInfo.xsize = bitmap.xsize;
		texInfo.ysize = bitmap.ysize;	

		#ifndef HEADLESS
		// names the texture by its file in GL debuggers and `debuginfo gpumemory`
		if (glObjectLabel != nullptr)
			glObjectLabel(GL_TEXTURE, texID, -1, filename.c_str());
		#endif

		#ifndef HEADLESS
			switch (bitmap.textype) {
				case GL_TEXTURE_2D_ARRAY:  { texInfo.texType = GL_TEXTURE_2D_ARRAY; } break;
				case GL_TEXTURE_3D:        { texInfo.texType = GL_TEXTURE_3D;       } break;
				case GL_TEXTURE_CUBE_MAP:  { texInfo.texType = GL_TEXTURE_CUBE_MAP; } break;
				default:                   { texInfo.texType = GL_TEXTURE_2D;       } break;
			}	
		#endif

		if (genInsert)
			GenInsertTex(texName, texInfo, false, false, true, false);

		return true;
	}

	static bool GenLoadTex(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		GLuint texID = 0;
		glGenTextures(1, &texID);
		return (Load(texName, texID));
	}


	bool Bind(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		if (texName.empty())
			return false;

		// cached
		const auto it = texInfoMap.find(texName);

		if (it != texInfoMap.end()) {
			const size_t texIdx = it->second;
			const GLuint texID = texInfoVec[texIdx].id;
			glBindTexture(GL_TEXTURE_2D, texID);
			return (texID != 0);
		}

		// load texture
		GLboolean inListCompile;
		glGetBooleanv(GL_LIST_INDEX, &inListCompile);
		if (inListCompile) {
			GenInsertTex(texName, {}, true, true, false, false);
			return true;
		}

		return (GenLoadTex(texName));
	}


	void Update()
	{
	RECOIL_DETAILED_TRACY_ZONE;
		if (waitingTextures.empty())
			return;

		const std::lock_guard<spring::recursive_mutex> lck(mutex);

		glPushAttrib(GL_TEXTURE_BIT);

		for (const std::string& texString: waitingTextures) {
			const auto mit = texInfoMap.find(texString);

			if (mit == texInfoMap.end())
				continue;

			Load(texString, texInfoVec[mit->second].id);
		}

		glPopAttrib();
		waitingTextures.clear();
	}


	bool Free(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		if (texName.empty())
			return false;

		DropPreloaded(texName);
		return (EraseTex(texName));
	}


	bool DropPreloaded(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		// a decode still running keeps its own reference to its image
		const std::lock_guard<spring::mutex> lck(preloadMutex);
		return preloaded.erase(texName) > 0;
	}


	bool Preload(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		if (texName.empty())
			return false;

		{
			const std::lock_guard<spring::recursive_mutex> lck(mutex);
			if (texInfoMap.find(texName) != texInfoMap.end())
				return false;
		}

		auto entry = std::make_shared<PreloadedImage>();
		entry->image = std::make_shared<DecodedImage>();
		{
			const std::lock_guard<spring::mutex> lck(preloadMutex);
			if (!preloaded.emplace(texName, entry).second)
				return false;
		}

		const std::string filename = ParseName(texName).filename;
		const auto decode = [image = entry->image, filename]() {
			image->decoded = image->bitmap.Load(filename, 1.0f, 4u, 0u);
		};

		if (ThreadPool::HasThreads()) {
			entry->job = ThreadPool::Enqueue(decode);
		} else {
			decode();
		}

		return true;
	}


	PreloadState GetPreloadState(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		{
			const std::lock_guard<spring::recursive_mutex> lck(mutex);
			if (texInfoMap.find(texName) != texInfoMap.end())
				return PreloadState::Loaded;
		}

		const std::lock_guard<spring::mutex> lck(preloadMutex);
		const auto it = preloaded.find(texName);
		if (it == preloaded.end())
			return PreloadState::None;

		const auto& job = it->second->job;
		if (job.valid() && job.wait_for(std::chrono::seconds(0)) != std::future_status::ready)
			return PreloadState::Pending;

		return PreloadState::Ready;
	}


	size_t GetInfoIndex(const std::string& texName)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		const auto it = texInfoMap.find(texName);

		if (it != texInfoMap.end())
			return (it->second);

		return (size_t(-1));
	}

	const TexInfo* GetInfo(size_t texIdx) { return ((texIdx < texInfoVec.size())? &texInfoVec[texIdx]: nullptr); }
	const TexInfo* GetInfo(const std::string& texName, bool forceLoad, bool persist, bool secondaryGLContext)
	{
	RECOIL_DETAILED_TRACY_ZONE;
		if (texName.empty())
			return nullptr;

		const size_t texIdx = GetInfoIndex(texName);

		if (texIdx != size_t(-1)) {
			texInfoVec[texIdx].persist |= persist;
			return &texInfoVec[texIdx];
		}

		if (forceLoad) {
			// load texture
			GLboolean inListCompile;
			glGetBooleanv(GL_LIST_INDEX, &inListCompile);

			if (inListCompile) {
				GenInsertTex(texName, {}, true, secondaryGLContext, false, persist);
			} else {
				GenLoadTex(texName);
			}

			return &texInfoVec[ texInfoMap[texName] ];
		}

		return nullptr;
	}


	/******************************************************************************/

} // namespace CNamedTextures

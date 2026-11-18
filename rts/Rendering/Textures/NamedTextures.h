/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef NAMED_TEXTURES_H
#define NAMED_TEXTURES_H

#include <string>

namespace CNamedTextures {
	void Init();
	void Kill(bool shutdown = false);
	void Reload();

	static bool Load(const std::string& texName, unsigned int texID, bool genInsert = true);

	/**
	 * Reload textures we could not load because Bind() was called
	 * when compiling a DList.
	 * Otherwise, it would re-upload the texture-data on each call
	 * of the DList, so we delay it and load them here.
	 */
	void Update();

	bool Bind(const std::string& texName);
	bool Free(const std::string& texName);

	/**
	 * Asynchronous decode: reads and decodes the image of a named texture on the worker
	 * threads, so that making the texture (Load, from GetInfo(forceLoad) or Bind) takes the
	 * decoded image and costs the upload only, not the file read and the decode, which are
	 * the larger part for a block-compressed map. Returns true when a decode was queued.
	 * Without worker threads the decode happens in the call. Main-thread only.
	 */
	bool Preload(const std::string& texName);

	enum class PreloadState {
		None,    ///< nothing queued and not loaded
		Pending, ///< being decoded
		Ready,   ///< decoded, waiting for the texture to be made
		Loaded,  ///< the texture exists
	};
	PreloadState GetPreloadState(const std::string& texName);
	/// Drops an image Preload decoded and nothing used; true when there was one.
	bool DropPreloaded(const std::string& texName);

	struct TexInfo {
		TexInfo()
			: id(0), xsize(-1), ysize(-1), texType(0), alpha(false), persist(false) {}
		unsigned int id;
		int xsize;
		int ysize;
		unsigned int texType;
		bool alpha;
		bool persist;
	};

	size_t GetInfoIndex(const std::string& texName);

	const TexInfo* GetInfo(const std::string& texName, bool forceLoad = false, bool persist = false, bool secondaryGLContext = false);
	const TexInfo* GetInfo(size_t texIdx);
}

#endif /* NAMED_TEXTURES_H */

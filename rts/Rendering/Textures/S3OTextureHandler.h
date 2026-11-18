/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef S3O_TEXTURE_HANDLER_H
#define S3O_TEXTURE_HANDLER_H

#include <future>
#include <memory>
#include <string>
#include <vector>

#include "Bitmap.h"
#include "System/Threading/SpringThreading.h"
#include "System/UnorderedMap.hpp"

struct S3DModel;
class CBitmap;

class CS3OTextureHandler
{
public:
	struct S3OTexMat {
		int num;

		unsigned int tex1;
		unsigned int tex2;

		unsigned int tex1SizeX;
		unsigned int tex1SizeY;

		unsigned int tex2SizeX;
		unsigned int tex2SizeY;
	};

	struct CachedS3OTex {
		unsigned int texID;
		unsigned int xsize;
		unsigned int ysize;
		bool invertAxis;
		bool invertAlpha;
	};

	void Init();
	void Kill();
	void Reload();

	void LoadTexture(S3DModel* model);
	void PreloadTexture(S3DModel* model, bool invertAxis, bool invertAlpha);

	/// Finishes at most one of the maps being streamed in (see ModelTextureStreaming): the
	/// decoded image goes into the texture that stood in for it. Called once a drawn frame.
	void Update();
	/// Maps still being decoded or waiting to be made.
	size_t NumStreaming() const { return streaming.size(); }

public:
	const S3OTexMat* GetTexture(unsigned int num) {
		if (num < textures.size())
			return &textures[num];

		return nullptr;
	}

private:
	/// A map of a model loaded while the game runs, decoded on a worker thread; until it is
	/// made, the model's texture is a single pixel.
	struct StreamedTex {
		struct Image {
			CBitmap bitmap;
		};

		std::string name;
		unsigned int texID = 0;
		bool invertAxis = false;
		bool invertAlpha = false;
		std::shared_future<void> job;
		std::shared_ptr<Image> image;
	};

	bool StreamTexture(const S3DModel* model, unsigned int texNum, bool invertAxis, bool invertAlpha);
	unsigned int FinishStreamedTexture(StreamedTex& tex);
	unsigned int LoadAndCacheTexture(
		const S3DModel* model,
		unsigned int texNum,
		bool invertAxis,
		bool invertAlpha,
		bool preloadCall
	);
	unsigned int InsertTextureMat(const S3DModel* model);

private:
	typedef spring::unsynced_map<std::string, CachedS3OTex> TextureCache;
	typedef spring::unsynced_map<std::string, CBitmap> BitmapCache;
	typedef spring::unsynced_map<std::uint64_t, unsigned int> TextureTable;

	TextureCache textureCache; // stores individual primary- and secondary-textures by name
	TextureTable textureTable; // stores (primary, secondary) texture-pairs by unique ident
	BitmapCache bitmapCache;

	std::vector<StreamedTex> streaming;
	std::vector<S3OTexMat> textures;
};

extern CS3OTextureHandler textureHandlerS3O;

#endif /* S3O_TEXTURE_HANDLER_H */

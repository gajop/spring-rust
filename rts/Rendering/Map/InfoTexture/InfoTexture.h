/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include "Rendering/GL/myGL.h"
#include "Rendering/Textures/Texture.hpp"
#include "System/type2.h"
#include <string>

class CInfoTexture
{
public:
	CInfoTexture();
	CInfoTexture(const std::string& name, GL::Texture2D&& texture, int2 texSize);
	virtual ~CInfoTexture() {}

public:
	/// binding a view through here marks it in use (see RecentlyUsed)
	virtual GLuint GetTexture();
	/// Whether anything bound this view in the last few seconds. On a null map, views that upload
	/// a whole LOS-sized grid on every update skip it otherwise: the grids grow with the map, and
	/// the upload alone stalls drawing on large maps. A view bound again catches up on its next update.
	bool RecentlyUsed() const;
	int2 GetTexSize()     const { return texSize; }
	const std::string& GetName() const { return name; }
protected:
	friend class IInfoTextureHandler;

	GL::Texture2D texture;
	std::string name;
	int2 texSize;

	bool everUsed = false;
	unsigned int lastUseDrawFrame = 0;
	// textures created when first bound, and updated only while in use (see RecentlyUsed)
	bool createdOnUse = false;
};

class CDummyInfoTexture: public CInfoTexture {
public:
	CDummyInfoTexture() : CInfoTexture("dummy", {}, int2(0, 0)) {}
};
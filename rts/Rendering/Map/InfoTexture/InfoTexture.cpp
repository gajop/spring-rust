/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "InfoTexture.h"
#include "Rendering/GlobalRendering.h"


CInfoTexture::CInfoTexture()
	: texture{}
	, texSize(0, 0)
{}

CInfoTexture::CInfoTexture(const std::string& _name, GL::Texture2D&& _texture, int2 _texSize)
	: texture(std::move(_texture))
	, name(_name)
	, texSize(_texSize)
{}

GLuint CInfoTexture::GetTexture()
{
	everUsed = true;
	lastUseDrawFrame = globalRendering->drawFrame;
	return texture.GetId();
}

bool CInfoTexture::RecentlyUsed() const
{
	// about two seconds at 60 frames per second; info views update at most 30 times a second
	return (everUsed && (globalRendering->drawFrame - lastUseDrawFrame) < 120);
}

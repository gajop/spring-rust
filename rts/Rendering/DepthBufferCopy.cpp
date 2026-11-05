#include "DepthBufferCopy.h"

#include <array>
#include <cassert>

#include "System/EventHandler.h"
#include "Rendering/GlobalRendering.h"
#include "Rendering/GL/myGL.h"
#include "Rendering/GL/FBO.h"
#include "System/Log/ILog.h"

std::unique_ptr<DepthBufferCopy> depthBufferCopy = nullptr;

DepthBufferCopy::DepthBufferCopy()
	: CEventClient("[DepthBufferCopy]", 012345, false)
{
	eventHandler.AddClient(this);
	ViewResize();
}

DepthBufferCopy::~DepthBufferCopy()
{
	assert(consumersCount[false] == 0);
	assert(consumersCount[true ] == 0);

	eventHandler.RemoveClient(this);
	autoLinkedEvents.clear();
}

void DepthBufferCopy::Init()
{
	depthBufferCopy = std::make_unique<DepthBufferCopy>();
}

void DepthBufferCopy::Kill()
{
	depthBufferCopy = nullptr;
}

void DepthBufferCopy::AddConsumer(bool ms)
{
	if (consumersCount[ms] == 0)
		CreateTextureAndFBO(ms);

	consumersCount[ms]++;
}

void DepthBufferCopy::DelConsumer(bool ms)
{
	consumersCount[ms]--;

	if (consumersCount[ms] == 0)
		DestroyTextureAndFBO(ms);
}

void DepthBufferCopy::ViewResize()
{
	for (size_t ms = 0; ms < depthFBOs.size(); ++ms) {
		if (consumersCount[ms] == 0)
			continue;

		RecreateTextureAndFBO(static_cast<bool>(ms));
	}
}

bool DepthBufferCopy::IsValid(bool ms) const {
	const auto& depthFBO = depthFBOs[ms];
	return !failed[ms] && depthFBO && depthFBO->IsValid() && depthTextures[ms] > 0;
}

DepthBufferCopy::SourceFormat DepthBufferCopy::QuerySourceFormat(uint32_t fboId)
{
	SourceFormat format;

	GLint previous = 0;
	glGetIntegerv(GL_READ_FRAMEBUFFER_BINDING, &previous);
	glBindFramebuffer(GL_READ_FRAMEBUFFER, fboId);

	// the window's buffers are named GL_DEPTH and GL_STENCIL, an FBO's by attachment
	const GLenum depth   = (fboId == 0)? GL_DEPTH  : GL_DEPTH_ATTACHMENT;
	const GLenum stencil = (fboId == 0)? GL_STENCIL: GL_STENCIL_ATTACHMENT;

	GLint type = GL_NONE;
	glGetFramebufferAttachmentParameteriv(GL_READ_FRAMEBUFFER, depth, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, &type);
	if (type != GL_NONE) {
		GLint component = GL_NONE;
		glGetFramebufferAttachmentParameteriv(GL_READ_FRAMEBUFFER, depth, GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE, &format.depthBits);
		glGetFramebufferAttachmentParameteriv(GL_READ_FRAMEBUFFER, depth, GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE, &component);
		format.floatDepth = (component == GL_FLOAT);
	}

	type = GL_NONE;
	glGetFramebufferAttachmentParameteriv(GL_READ_FRAMEBUFFER, stencil, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, &type);
	if (type != GL_NONE)
		glGetFramebufferAttachmentParameteriv(GL_READ_FRAMEBUFFER, stencil, GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE, &format.stencilBits);

	glBindFramebuffer(GL_READ_FRAMEBUFFER, previous);
	return format;
}

void DepthBufferCopy::MakeDepthBufferCopy()
{
	// the world's framebuffer, bound now
	GLint currentFBO = 0;
	GLint currentReadFBO = 0;
	glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &currentFBO);
	glGetIntegerv(GL_READ_FRAMEBUFFER_BINDING, &currentReadFBO);

	const bool haveMS = (consumersCount[true] > 0) && CopyInto(true, currentFBO);

	if (consumersCount[false] > 0) {
		// from the multisampled copy when it holds the depth, else from the world's framebuffer
		const int32_t srcFboID = (haveMS && depthFBOs[true]) ? depthFBOs[true]->GetId() : currentFBO;
		CopyInto(false, srcFboID);
	}

	glBindFramebuffer(GL_DRAW_FRAMEBUFFER, currentFBO);
	glBindFramebuffer(GL_READ_FRAMEBUFFER, currentReadFBO);
}

bool DepthBufferCopy::CopyInto(bool ms, int32_t srcFboID)
{
	if (sourceFBOs[ms] != srcFboID) {
		// a new source: make the copy in its depth and stencil layout, since
		// depth blits between differing layouts fail on strict drivers
		sourceFBOs[ms] = srcFboID;
		const SourceFormat format = QuerySourceFormat(srcFboID);

		if (format != sourceFormats[ms]) {
			sourceFormats[ms] = format;
			failed[ms] = false;
			RecreateTextureAndFBO(ms);

			// the single-sampled copy is taken from this one: look at it again
			if (ms)
				sourceFBOs[false] = -1;
		}
	}

	if (failed[ms] || !depthFBOs[ms])
		return false;

	const int x0 = globalRendering->viewPosX;
	const int y0 = globalRendering->viewPosY;
	const int sx = globalRendering->viewSizeX;
	const int sy = globalRendering->viewSizeY;

	if (sx <= 0 || sy <= 0)
		return false;

	const bool verify = unverified[ms];

	if (verify) {
		// earlier errors belong to someone else; drained only for this one check
		for (int i = 0; i < 64 && glGetError() != GL_NO_ERROR; ++i) {}
	}

	glBindFramebuffer(GL_READ_FRAMEBUFFER, srcFboID);
	glBindFramebuffer(GL_DRAW_FRAMEBUFFER, depthFBOs[ms]->GetId());
	glBlitFramebuffer(x0, y0, x0 + sx, y0 + sy, 0, 0, sx, sy, GL_DEPTH_BUFFER_BIT, GL_NEAREST);

	if (!verify)
		return true;

	unverified[ms] = false;

	const SourceFormat& source = sourceFormats[ms];
	const GLenum error = glGetError();

	if (error == GL_NO_ERROR) {
		LOG("[DepthBufferCopy] %s depth copy: source framebuffer %d (depth %d bits%s, stencil %d bits), copy format 0x%04x",
			ms ? "multisampled" : "single-sampled", srcFboID, source.depthBits, source.floatDepth ? " float" : "", source.stencilBits, copyFormats[ms]);
		return true;
	}

	// The copy texture would keep its first contents (nearest depth on some
	// drivers), and everything soft particles draw would fade out entirely.
	failed[ms] = true;
	LOG_L(L_WARNING, "[DepthBufferCopy] copying the depth buffer fails on this driver (GL error 0x%04x; source framebuffer %d: depth %d bits%s, stencil %d bits; copy format 0x%04x): soft particles are drawn unsoftened",
		error, srcFboID, source.depthBits, source.floatDepth ? " float" : "", source.stencilBits, copyFormats[ms]);
	return false;
}

void DepthBufferCopy::DestroyTextureAndFBO(bool ms)
{
	auto& depthFBO = depthFBOs[ms];
	auto& depthTexture = depthTextures[ms];

	assert(depthFBO);
	if (depthFBO) {
		if (depthFBO->IsValid()) {
			depthFBO->Bind();
			depthFBO->DetachAll();
			depthFBO->Unbind();
		}
		depthFBO->Kill();
		depthFBO = nullptr;
	}

	assert(depthTexture);
	if (depthTexture) {
		glDeleteTextures(1, &depthTexture);
		depthTexture = 0u;
	}
}

void DepthBufferCopy::CreateTextureAndFBO(bool ms)
{
	auto& depthTexture = depthTextures[ms];
	auto& depthFBO     = depthFBOs[ms];
	const auto target = ms ? GL_TEXTURE_2D_MULTISAMPLE : GL_TEXTURE_2D;

	assert(depthTexture == 0);
	glGenTextures(1, &depthTexture);

	glEnable(target);
	glBindTexture(target, depthTexture);

	glTexParameteri(target, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
	glTexParameteri(target, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
	glTexParameteri(target, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
	glTexParameteri(target, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
	glTexParameteri(target, GL_DEPTH_TEXTURE_MODE, GL_LUMINANCE);
	glTexParameteri(target, GL_TEXTURE_BASE_LEVEL, 0);
	glTexParameteri(target, GL_TEXTURE_MAX_LEVEL, 0);
	glTexParameteri(target, GL_TEXTURE_COMPARE_MODE, GL_NONE);

	// The copy is blitted from the world's framebuffer, and depth blits need
	// the same depth and stencil layout on both sides: copy in the source's
	// layout. A depth-only copy of a DEPTH24_STENCIL8 window fails on AMD's
	// drivers (and a 32-bit copy of a 24-bit window on Mesa), and every blit
	// failing faded soft particles out completely.
	const SourceFormat& source = sourceFormats[ms];
	GLint depthFormat = GL_DEPTH_COMPONENT24;
	GLenum dataFormat = GL_DEPTH_COMPONENT;
	GLenum dataType = GL_FLOAT;
	GLenum attachment = GL_DEPTH_ATTACHMENT;

	if (source.depthBits <= 0) {
		// not copied yet: the window's depth size, until the first copy names the source
		const int depthBits = (globalRendering->windowDepthBufferBitDepth > 0)?
			globalRendering->windowDepthBufferBitDepth:
			globalRendering->supportDepthBufferBitDepth;
		depthFormat = static_cast<GLint>(CGlobalRendering::DepthBitsToFormat(depthBits));
	} else if (source.stencilBits > 0) {
		dataFormat = GL_DEPTH_STENCIL;
		attachment = GL_DEPTH_STENCIL_ATTACHMENT;
		depthFormat = source.floatDepth ? GL_DEPTH32F_STENCIL8 : GL_DEPTH24_STENCIL8;
		dataType = source.floatDepth ? GL_FLOAT_32_UNSIGNED_INT_24_8_REV : GL_UNSIGNED_INT_24_8;
	} else if (source.floatDepth) {
		depthFormat = GL_DEPTH_COMPONENT32F;
	} else {
		depthFormat = static_cast<GLint>(CGlobalRendering::DepthBitsToFormat(source.depthBits));
	}

	copyFormats[ms] = static_cast<uint32_t>(depthFormat);
	unverified[ms] = true;

	if (target == GL_TEXTURE_2D)
		glTexImage2D(target, 0, depthFormat, globalRendering->viewSizeX, globalRendering->viewSizeY, 0, dataFormat, dataType, nullptr);
	else
		glTexImage2DMultisample(target, globalRendering->msaaLevel, depthFormat, globalRendering->viewSizeX, globalRendering->viewSizeY, GL_TRUE);

	glBindTexture(target, 0);
	glDisable(target);

	assert(depthFBO == nullptr);
	depthFBO = std::make_unique<FBO>(true);
	depthFBO->Init(false);

	depthFBO->Bind();
	depthFBO->AttachTexture(depthTexture, target, attachment);
	glDrawBuffer(GL_NONE);
	depthFBO->CheckStatus(ms ? "DEPTH-BUFFER-COPY-FBO-MULTISAMPLED" : "DEPTH-BUFFER-COPY-FBO");
	depthFBO->Unbind();
}

#pragma once

#include <memory>
#include <array>
#include <cstdint>

#include "System/UnorderedSet.hpp"
#include "System/EventClient.h"
#include "Rendering/GL/FBO.h"

struct ScopedDepthBufferCopy;

class DepthBufferCopy : public CEventClient {
public:
	friend struct ScopedDepthBufferCopy;
	DepthBufferCopy();
	~DepthBufferCopy() override;

	static void Init();
	static void Kill();

	bool WantsEvent(const std::string& eventName) override {
		return
			(eventName == "ViewResize");
	}

	void ViewResize() override;

	/// False also once a copy has failed on this driver: consumers then draw
	/// without the depth copy (soft particles unsoftened) instead of reading
	/// a texture that was never written.
	bool IsValid(bool ms) const;

	void MakeDepthBufferCopy();
	/// Instead of a copy when no ground is drawn: the copy is let go (IsValid turns false
	/// and its consumers draw unsoftened) until a copy is made again.
	void MakeEmptyDepthBufferCopy();
	uint32_t GetDepthBufferTexture(bool ms) const { return depthTextures[ms]; }
private:
	// to be accessed with ScopedDepthBufferCopy
	void AddConsumer(bool ms);
	void DelConsumer(bool ms);
private:
	/// Depth and stencil layout of the framebuffer a copy is blitted from.
	/// Depth blits need the copy in exactly that layout (AMD's drivers refuse
	/// a DEPTH24_STENCIL8 window into a DEPTH_COMPONENT24 copy).
	struct SourceFormat {
		int depthBits = 0;
		int stencilBits = 0;
		bool floatDepth = false;
		bool operator == (const SourceFormat&) const = default;
	};
	static SourceFormat QuerySourceFormat(uint32_t fboId);

	void DestroyTextureAndFBO(bool ms);
	void CreateTextureAndFBO(bool ms);
	bool CopyInto(bool ms, int32_t srcFboID);
	void ClearToEmpty(bool ms);
	void RecreateTextureAndFBO(bool ms) {
		DestroyTextureAndFBO(ms);
		CreateTextureAndFBO(ms);
	}

	std::array<uint32_t, 2> consumersCount = {};
	std::array<uint32_t, 2> depthTextures = {};
	std::array<std::unique_ptr<FBO>, 2> depthFBOs = {};

	/// The source layout each copy was made for; -1 until the first copy.
	std::array<int64_t, 2> sourceFBOs = {-1, -1};
	std::array<SourceFormat, 2> sourceFormats = {};
	/// The first blit into a (re)made copy is checked for a GL error.
	std::array<bool, 2> unverified = {};
	std::array<bool, 2> failed = {};
	std::array<uint32_t, 2> copyFormats = {};
	bool reportedEmpty = false;
};

extern std::unique_ptr<DepthBufferCopy> depthBufferCopy;

struct ScopedDepthBufferCopy {
	ScopedDepthBufferCopy(bool ms)
		: multisampled(ms)
	{
		depthBufferCopy->AddConsumer(multisampled);
	}
	ScopedDepthBufferCopy(const ScopedDepthBufferCopy&) = delete;
	ScopedDepthBufferCopy& operator = (const ScopedDepthBufferCopy&) = delete;

	ScopedDepthBufferCopy(ScopedDepthBufferCopy&& other) noexcept { *this = std::move(other); }
	ScopedDepthBufferCopy& operator = (ScopedDepthBufferCopy&& other) noexcept {
		std::swap(multisampled, other.multisampled);
		return *this;
	}

	~ScopedDepthBufferCopy()
	{
		depthBufferCopy->DelConsumer(multisampled);
	}
	bool multisampled;
};
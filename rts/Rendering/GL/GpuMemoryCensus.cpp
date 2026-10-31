/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "GpuMemoryCensus.h"

#include <algorithm>
#include <bit>
#include <cstdint>
#include <map>
#include <string>
#include <tuple>
#include <vector>

#include "Rendering/GL/myGL.h"
#include "System/Log/ILog.h"

#ifndef HEADLESS
namespace {
// Names are scanned up to this far past the last one in use.
constexpr GLuint SCAN_PAST_LAST = 65536;
constexpr GLuint SCAN_LIMIT = 1u << 22;

// Bits a texel of one level takes as the driver stores it: three-channel and
// odd sizes are padded to the next power of two, as drivers lay them out.
std::uint64_t PaddedBits(std::uint64_t bits)
{
	if (bits == 24) return 32;
	if (bits == 48) return 64;
	if (bits == 96) return 128;
	return bits;
}

struct Object {
	const char* kind;
	GLenum target = 0;
	GLint format = 0;
	GLint w = 0, h = 0, d = 0;
	GLint levels = 0;
	GLint samples = 0;
	std::uint64_t bytes = 0;
	std::string label;
};

std::string Label(GLenum identifier, GLuint name)
{
	char text[128] = {0};
	GLsizei length = 0;
	glGetObjectLabel(identifier, name, sizeof(text), &length, text);
	return std::string(text, std::clamp<GLsizei>(length, 0, sizeof(text) - 1));
}

std::uint64_t LevelBytes(GLuint name, GLint level, GLenum target)
{
	GLint compressed = 0;
	glGetTextureLevelParameteriv(name, level, GL_TEXTURE_COMPRESSED, &compressed);
	GLint w = 0, h = 0, d = 0, samples = 0;
	glGetTextureLevelParameteriv(name, level, GL_TEXTURE_WIDTH, &w);
	glGetTextureLevelParameteriv(name, level, GL_TEXTURE_HEIGHT, &h);
	glGetTextureLevelParameteriv(name, level, GL_TEXTURE_DEPTH, &d);
	glGetTextureLevelParameteriv(name, level, GL_TEXTURE_SAMPLES, &samples);
	const std::uint64_t faces = (target == GL_TEXTURE_CUBE_MAP) ? 6 : 1;
	if (compressed) {
		GLint size = 0;
		glGetTextureLevelParameteriv(name, level, GL_TEXTURE_COMPRESSED_IMAGE_SIZE, &size);
		// Some drivers report one face of a cube map, some all six: block formats take at most
		// a byte a texel, so a cube map's size at or below w*h is one face.
		const std::uint64_t one = std::uint64_t(std::max(size, 0));
		return (faces == 6 && one <= std::uint64_t(std::max(w, 0)) * std::max(h, 0)) ? one * faces : one;
	}
	std::uint64_t bits = 0;
	for (const GLenum component: {GL_TEXTURE_RED_SIZE, GL_TEXTURE_GREEN_SIZE, GL_TEXTURE_BLUE_SIZE,
		GL_TEXTURE_ALPHA_SIZE, GL_TEXTURE_DEPTH_SIZE, GL_TEXTURE_STENCIL_SIZE, GL_TEXTURE_SHARED_SIZE}) {
		GLint size = 0;
		glGetTextureLevelParameteriv(name, level, component, &size);
		bits += std::uint64_t(std::max(size, 0));
	}
	return std::uint64_t(std::max(w, 0)) * std::max(h, 1) * std::max(d, 1) * std::max(samples, 1) * faces * PaddedBits(bits) / 8;
}

void Texture(GLuint name, std::vector<Object>& out)
{
	Object o{"texture"};
	GLint target = 0;
	glGetTextureParameteriv(name, GL_TEXTURE_TARGET, &target);
	o.target = GLenum(target);
	if (o.target == GL_TEXTURE_BUFFER)
		return; // its storage is a buffer, counted there
	glGetTextureLevelParameteriv(name, 0, GL_TEXTURE_INTERNAL_FORMAT, &o.format);
	glGetTextureLevelParameteriv(name, 0, GL_TEXTURE_WIDTH, &o.w);
	glGetTextureLevelParameteriv(name, 0, GL_TEXTURE_HEIGHT, &o.h);
	glGetTextureLevelParameteriv(name, 0, GL_TEXTURE_DEPTH, &o.d);
	glGetTextureLevelParameteriv(name, 0, GL_TEXTURE_SAMPLES, &o.samples);
	for (GLint level = 0; level < 16; ++level) {
		GLint w = 0;
		glGetTextureLevelParameteriv(name, level, GL_TEXTURE_WIDTH, &w);
		if (w <= 0) break;
		o.bytes += LevelBytes(name, level, o.target);
		o.levels = level + 1;
	}
	o.label = Label(GL_TEXTURE, name);
	out.push_back(std::move(o));
}

const char* TargetName(GLenum target)
{
	switch (target) {
		case GL_TEXTURE_1D: return "1d";
		case GL_TEXTURE_2D: return "2d";
		case GL_TEXTURE_3D: return "3d";
		case GL_TEXTURE_2D_ARRAY: return "2d-array";
		case GL_TEXTURE_CUBE_MAP: return "cube";
		case GL_TEXTURE_CUBE_MAP_ARRAY: return "cube-array";
		case GL_TEXTURE_RECTANGLE: return "rect";
		case GL_TEXTURE_2D_MULTISAMPLE: return "2d-ms";
		case GL_TEXTURE_2D_MULTISAMPLE_ARRAY: return "2d-ms-array";
		case GL_RENDERBUFFER: return "rbo";
		case GL_ARRAY_BUFFER: return "buffer";
		default: return "other";
	}
}
}
#endif

void LogGpuMemoryCensus(int minKiB)
{
#ifndef HEADLESS
	if (!GLAD_GL_VERSION_4_5) {
		LOG_L(L_WARNING, "[gpu-memory] needs GL 4.5 direct-state access");
		return;
	}
	glClearErrors("GpuMemoryCensus", __func__, false);
	std::vector<Object> objects;
	std::uint64_t counts[3] = {0, 0, 0};
	for (GLuint name = 1, last = 0; name < SCAN_LIMIT && name <= last + SCAN_PAST_LAST; ++name) {
		if (glIsTexture(name)) { Texture(name, objects); last = name; ++counts[0]; }
		if (glIsRenderbuffer(name)) {
			Object o{"renderbuffer", GL_RENDERBUFFER};
			glGetNamedRenderbufferParameteriv(name, GL_RENDERBUFFER_INTERNAL_FORMAT, &o.format);
			glGetNamedRenderbufferParameteriv(name, GL_RENDERBUFFER_WIDTH, &o.w);
			glGetNamedRenderbufferParameteriv(name, GL_RENDERBUFFER_HEIGHT, &o.h);
			glGetNamedRenderbufferParameteriv(name, GL_RENDERBUFFER_SAMPLES, &o.samples);
			std::uint64_t bits = 0;
			for (const GLenum component: {GL_RENDERBUFFER_RED_SIZE, GL_RENDERBUFFER_GREEN_SIZE, GL_RENDERBUFFER_BLUE_SIZE,
				GL_RENDERBUFFER_ALPHA_SIZE, GL_RENDERBUFFER_DEPTH_SIZE, GL_RENDERBUFFER_STENCIL_SIZE}) {
				GLint size = 0;
				glGetNamedRenderbufferParameteriv(name, component, &size);
				bits += std::uint64_t(std::max(size, 0));
			}
			o.levels = 1; o.d = 1;
			o.bytes = std::uint64_t(std::max(o.w, 0)) * std::max(o.h, 0) * std::max(o.samples, 1) * PaddedBits(bits) / 8;
			o.label = Label(GL_RENDERBUFFER, name);
			objects.push_back(std::move(o));
			last = name; ++counts[1];
		}
		if (glIsBuffer(name)) {
			Object o{"buffer", GL_ARRAY_BUFFER};
			GLint64 size = 0;
			glGetNamedBufferParameteri64v(name, GL_BUFFER_SIZE, &size);
			o.bytes = std::uint64_t(std::max<GLint64>(size, 0));
			o.label = Label(GL_BUFFER, name);
			objects.push_back(std::move(o));
			last = name; ++counts[2];
		}
	}
	glClearErrors("GpuMemoryCensus", __func__, false);

	std::map<std::string, std::uint64_t> byKind;
	std::uint64_t total = 0;
	for (const auto& o: objects) { byKind[o.kind] += o.bytes; total += o.bytes; }
	const auto mib = [](std::uint64_t b) { return double(b) / (1024.0 * 1024.0); };
	LOG("[gpu-memory] total %.1f MiB: textures %.1f MiB (%llu), renderbuffers %.1f MiB (%llu), buffers %.1f MiB (%llu)",
		mib(total), mib(byKind["texture"]), (unsigned long long)counts[0], mib(byKind["renderbuffer"]),
		(unsigned long long)counts[1], mib(byKind["buffer"]), (unsigned long long)counts[2]);

	if (GLAD_GL_NVX_gpu_memory_info) {
		GLint dedicated = 0, available = 0, evicted = 0;
		glGetIntegerv(GL_GPU_MEMORY_INFO_TOTAL_AVAILABLE_MEMORY_NVX, &dedicated);
		glGetIntegerv(GL_GPU_MEMORY_INFO_CURRENT_AVAILABLE_VIDMEM_NVX, &available);
		glGetIntegerv(GL_GPU_MEMORY_INFO_EVICTED_MEMORY_NVX, &evicted);
		LOG("[gpu-memory] driver: %.1f MiB of %.1f MiB in use on the device, %.1f MiB evicted",
			(dedicated - available) / 1024.0, dedicated / 1024.0, evicted / 1024.0);
	}

	// Group objects that look alike: same kind, target, format, size, levels and label.
	using Key = std::tuple<std::string, GLenum, GLint, GLint, GLint, GLint, GLint, GLint, std::string>;
	struct Group { std::uint64_t count = 0, bytes = 0; };
	std::map<Key, Group> groups;
	for (const auto& o: objects) {
		// Buffers group by size class (powers of two) so thousands of small ones read as one line.
		const GLint w = (o.target == GL_ARRAY_BUFFER) ? GLint(std::bit_ceil(std::max<std::uint64_t>(o.bytes, 1))) : o.w;
		auto& g = groups[Key{o.kind, o.target, o.format, w, o.h, o.d, o.levels, o.samples, o.label}];
		g.count++; g.bytes += o.bytes;
	}
	std::vector<std::pair<Key, Group>> sorted(groups.begin(), groups.end());
	std::sort(sorted.begin(), sorted.end(), [](const auto& a, const auto& b) { return a.second.bytes > b.second.bytes; });
	std::uint64_t listed = 0;
	for (const auto& [key, g]: sorted) {
		if (g.bytes < std::uint64_t(minKiB) * 1024) break;
		const auto& [kind, target, format, w, h, d, levels, samples, label] = key;
		listed += g.bytes;
		if (target == GL_ARRAY_BUFFER) {
			LOG("[gpu-memory] %8.2f MiB  %5llu x buffer up to %d bytes%s%s", mib(g.bytes), (unsigned long long)g.count, w,
				label.empty() ? "" : "  ", label.c_str());
			continue;
		}
		LOG("[gpu-memory] %8.2f MiB  %5llu x %s %dx%dx%d levels %d samples %d format 0x%04x%s%s", mib(g.bytes),
			(unsigned long long)g.count, TargetName(target), w, h, d, levels, samples, unsigned(format),
			label.empty() ? "" : "  ", label.c_str());
	}
	LOG("[gpu-memory] listed %.1f MiB; %.1f MiB in groups under %d KiB", mib(listed), mib(total - listed), minKiB);
#else
	(void)minKiB;
#endif
}

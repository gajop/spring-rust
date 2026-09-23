/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

// Tracy GPU (OpenGL timer query) zones. Every SCOPED_GL_DEBUGGROUP opens one,
// so draw passes show up on Tracy's GPU timeline. No-ops unless Tracy is
// enabled; never used in headless builds (no GL context).

#if defined(TRACY_ENABLE) && !defined(HEADLESS)
	#include "Rendering/GL/myGL.h"
	#include <tracy/TracyOpenGL.hpp>

	#define SPRING_TRACY_GPU 1
	// inactive until InitTracyGpuContext has run
	#define SCOPED_TRACY_GPU_ZONE(name) TracyGpuNamedZone(_UTIL_CONCAT(__tracyGpuZone, __LINE__), name, (tracy::GetGpuCtx().ptr != nullptr))
#else
	#define SPRING_TRACY_GPU 0
	#define SCOPED_TRACY_GPU_ZONE(name)
#endif

namespace GL {
	/// Create Tracy's GPU context; call once with the GL context current
	void InitTracyGpuContext();
	/// Resolve finished GPU timer queries; call once per frame
	void CollectTracyGpuZones();
}

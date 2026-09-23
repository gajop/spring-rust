/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

/*
 * The engine's known, accepted LeakSanitizer reports. LeakSanitizer reads this
 * list on its own in an ASan build (-DUSE_ASAN=ON), so engine developers and
 * games get it without passing a suppression file. Games can still add their
 * own entries with LSAN_OPTIONS=suppressions=<file>; both lists apply.
 *
 * Only list process-lifetime allocations the engine cannot free. A leak owned by
 * engine code must be fixed, not listed here. LeakSanitizer matches a pattern
 * against every frame of the allocation stack, so keep patterns specific.
 *
 * Include this header from exactly one source file of each executable (the
 * file defining main).
 */

#if defined(__SANITIZE_ADDRESS__)
	#define RECOIL_LSAN_SUPPRESSIONS 1
#elif defined(__has_feature)
	#if __has_feature(address_sanitizer)
		#define RECOIL_LSAN_SUPPRESSIONS 1
	#endif
#endif

#ifdef RECOIL_LSAN_SUPPRESSIONS
// Default visibility: the sanitizer runtime (libasan.so) looks the symbol up
// dynamically, and the engine builds with -fvisibility=hidden.
extern "C" __attribute__((visibility("default"), used)) const char* __lsan_default_suppressions()
{
	return
		// The proprietary NVIDIA driver's allocations for the GL context, shaders,
		// buffers and textures; they live until the process exits.
		"leak:libnvidia-glcore.so\n"
		// NVIDIA registers with the session bus while the driver initializes;
		// there is no teardown API for it.
		"leak:dbus_bus_register\n"
		// fontconfig's font directory scan (FcConfigBuildFonts) leaks FreeType
		// libraries and face data. The bundled FreeType has no frame pointers, so
		// with the default fast unwinder these stacks end at the FreeType
		// allocator and only the FreeType frames can be matched. This also covers
		// the engine's own FreeType allocations; its faces are freed by FontFace.
		"leak:FcFileScanConfig\n"
		"leak:ft_mem_alloc\n"
		"leak:FT_New_Memory\n"
		// SDL keeps its audio device bookkeeping, allocated on the OpenAL sound
		// thread, after SDL_QuitSubSystem(SDL_INIT_AUDIO).
		"leak:CSound::OpenSdlDevice\n"
		"leak:CSound::Cleanup\n";
}
#endif

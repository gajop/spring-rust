/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#if defined(__SSE__) || defined(_M_X64)
#include <xmmintrin.h>
#elif !defined(__aarch64__)
#include <cfenv>
#endif

// WebAssembly requires nearest-even rounding and gradual underflow. Native
// callers and host callbacks must not select different guest arithmetic.
// Control registers are thread-local; nested scopes restore their caller.
class WasmFloatEnvironment {
public:
	WasmFloatEnvironment()
	{
#if defined(__SSE__) || defined(_M_X64)
		saved = _mm_getcsr();
		// Mask FP exceptions, clear rounding/FTZ/DAZ, retain sticky status bits.
		const auto required = (saved & ~0xe040u) | 0x1f80u;
		if (required != saved) _mm_setcsr(required);
#elif defined(__aarch64__)
		asm volatile("mrs %0, fpcr" : "=r" (saved));
		// FPCR=0 selects nearest-even, no flushing or alternative FP modes,
		// and masked exceptions. See lib/streflop/FPUSettings.h's register map.
		if (saved != 0) asm volatile("msr fpcr, %0" : : "r" (0ull));
#else
		std::fegetenv(&saved);
		std::fesetenv(FE_DFL_ENV);
#endif
	}

	~WasmFloatEnvironment()
	{
#if defined(__SSE__) || defined(_M_X64)
		const auto current = _mm_getcsr();
		// The common case needs no register write. Host code may have changed
		// controls even when no adjustment was needed on entry.
		if (((current ^ saved) & ~0x3fu) != 0)
			_mm_setcsr((current & 0x3fu) | (saved & ~0x3fu));
#elif defined(__aarch64__)
		unsigned long long current;
		asm volatile("mrs %0, fpcr" : "=r" (current));
		if (current != saved) asm volatile("msr fpcr, %0" : : "r" (saved));
#else
		std::fesetenv(&saved);
#endif
	}

	WasmFloatEnvironment(const WasmFloatEnvironment&) = delete;
	WasmFloatEnvironment& operator=(const WasmFloatEnvironment&) = delete;
private:
#if defined(__SSE__) || defined(_M_X64)
	unsigned saved;
#elif defined(__aarch64__)
	unsigned long long saved;
#else
	std::fenv_t saved;
#endif
};

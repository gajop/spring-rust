#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Math Extensions API
// @see rts/Lua/LuaMathExtra.cpp
//
// Extended math functions beyond standard C math library
// ============================================================================

// API structure
struct MathExtraApi {
	// Hypotenuse: sqrt(x^2 + y^2) with better numerical stability
	float (*Hypot)(float x, float y);

	// Diagonal: length of n-dimensional vector
	float (*Diag)(const float* values, uint32_t count);

	// Clamp value between min and max
	float (*Clamp)(float value, float min, float max);

	// Sign function: returns -1, 0, or 1
	float (*Sgn)(float value);

	// Linear interpolation: mix(a, b, t) = a * (1-t) + b * t
	float (*Mix)(float a, float b, float t);

	// Round to nearest integer
	float (*Round)(float value);

	// Error function
	float (*Erf)(float value);

	// Smooth step interpolation
	float (*SmoothStep)(float edge0, float edge1, float x);

	// Normalize vector (returns length)
	FloatResult (*Normalize)(Float3* vec);  // Modifies vec in place

	// Bitwise operations (on 24-bit integers represented as floats)
	uint32_t (*BitOr)(uint32_t a, uint32_t b);
	uint32_t (*BitAnd)(uint32_t a, uint32_t b);
	uint32_t (*BitXor)(uint32_t a, uint32_t b);
	uint32_t (*BitInv)(uint32_t a);
	uint32_t (*BitBits)(uint32_t value, uint32_t startBit, uint32_t endBit);
};

extern const MathExtraApi MATH_EXTRA_API;

#ifdef __cplusplus
}
#endif

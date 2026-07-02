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

// Queries
struct HypotQuery { float x; float y; };
struct HypotResult { const Error* error; float value; };

struct DiagQuery { const float* values; uint32_t count; };
struct DiagResult { const Error* error; float length; };

struct ClampQuery { float value; float min; float max; };
struct ClampResult { const Error* error; float clamped; };

struct SgnQuery { float value; };
struct SgnResult { const Error* error; float sign; };

struct MixQuery { float a; float b; float t; };
struct MixResult { const Error* error; float mixed; };

struct RoundQuery { float value; };
struct RoundResult { const Error* error; float rounded; };

struct ErfQuery { float value; };
struct ErfResult { const Error* error; float result; };

struct SmoothStepQuery { float edge0; float edge1; float x; };
struct SmoothStepResult { const Error* error; float value; };

struct NormalizeQuery { Float3* vec; };  // Modifies vec in place
struct NormalizeResult { const Error* error; float length; };

struct BitOrQuery { uint32_t a; uint32_t b; };
struct BitOrResult { const Error* error; uint32_t value; };

struct BitAndQuery { uint32_t a; uint32_t b; };
struct BitAndResult { const Error* error; uint32_t value; };

struct BitXorQuery { uint32_t a; uint32_t b; };
struct BitXorResult { const Error* error; uint32_t value; };

struct BitInvQuery { uint32_t a; };
struct BitInvResult { const Error* error; uint32_t value; };

struct BitBitsQuery { const uint32_t* bits; uint32_t count; };
struct BitBitsResult { const Error* error; uint32_t bits; };

// API structure
struct MathExtraApi {
	void (*Hypot)(const HypotQuery* query, HypotResult* result);
	void (*Diag)(const DiagQuery* query, DiagResult* result);
	void (*Clamp)(const ClampQuery* query, ClampResult* result);
	void (*Sgn)(const SgnQuery* query, SgnResult* result);
	void (*Mix)(const MixQuery* query, MixResult* result);
	void (*Round)(const RoundQuery* query, RoundResult* result);
	void (*Erf)(const ErfQuery* query, ErfResult* result);
	void (*SmoothStep)(const SmoothStepQuery* query, SmoothStepResult* result);
	void (*Normalize)(const NormalizeQuery* query, NormalizeResult* result);
	void (*BitOr)(const BitOrQuery* query, BitOrResult* result);
	void (*BitAnd)(const BitAndQuery* query, BitAndResult* result);
	void (*BitXor)(const BitXorQuery* query, BitXorResult* result);
	void (*BitInv)(const BitInvQuery* query, BitInvResult* result);
	void (*BitBits)(const BitBitsQuery* query, BitBitsResult* result);
};

extern const MathExtraApi MATH_EXTRA_API;

#ifdef __cplusplus
}
#endif

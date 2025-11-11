#include "MathExtra.h"

#include <cmath>
#include <algorithm>

namespace {

static float NativeHypot(float x, float y)
{
	return std::hypot(x, y);
}

static float NativeDiag(const float* values, uint32_t count)
{
	if (values == nullptr || count == 0) {
		return 0.0f;
	}

	float sumSquares = 0.0f;
	for (uint32_t i = 0; i < count; i++) {
		sumSquares += values[i] * values[i];
	}

	return std::sqrt(sumSquares);
}

static float NativeClamp(float value, float min, float max)
{
	return std::clamp(value, min, max);
}

static float NativeSgn(float value)
{
	if (value > 0.0f) return 1.0f;
	if (value < 0.0f) return -1.0f;
	return 0.0f;
}

static float NativeMix(float a, float b, float t)
{
	return a * (1.0f - t) + b * t;
}

static float NativeRound(float value)
{
	return std::round(value);
}

static float NativeErf(float value)
{
	return std::erf(value);
}

static float NativeSmoothStep(float edge0, float edge1, float x)
{
	// Standard smoothstep function
	float t = std::clamp((x - edge0) / (edge1 - edge0), 0.0f, 1.0f);
	return t * t * (3.0f - 2.0f * t);
}

static FloatResult NativeNormalize(Float3* vec)
{
	FloatResult result = {};

	if (vec == nullptr) {
		static const Error NULL_PTR = {
			.code = ERROR_INVALID_ARGUMENT,
			.message = "Vector pointer is null"
		};
		result.error = &NULL_PTR;
		return result;
	}

	const float length = std::sqrt(vec->x * vec->x + vec->y * vec->y + vec->z * vec->z);

	if (length > 0.0f) {
		const float invLength = 1.0f / length;
		vec->x *= invLength;
		vec->y *= invLength;
		vec->z *= invLength;
	}

	result.value = length;
	return result;
}

// Bitwise operations
static uint32_t NativeBitOr(uint32_t a, uint32_t b)
{
	return a | b;
}

static uint32_t NativeBitAnd(uint32_t a, uint32_t b)
{
	return a & b;
}

static uint32_t NativeBitXor(uint32_t a, uint32_t b)
{
	return a ^ b;
}

static uint32_t NativeBitInv(uint32_t a)
{
	return ~a;
}

static uint32_t NativeBitBits(uint32_t value, uint32_t startBit, uint32_t endBit)
{
	if (startBit > endBit || endBit > 31) {
		return 0;
	}

	const uint32_t numBits = endBit - startBit + 1;
	const uint32_t mask = (numBits == 32) ? 0xFFFFFFFF : ((1u << numBits) - 1);

	return (value >> startBit) & mask;
}

} // namespace

const MathExtraApi MATH_EXTRA_API = {
	.Hypot = NativeHypot,
	.Diag = NativeDiag,
	.Clamp = NativeClamp,
	.Sgn = NativeSgn,
	.Mix = NativeMix,
	.Round = NativeRound,
	.Erf = NativeErf,
	.SmoothStep = NativeSmoothStep,
	.Normalize = NativeNormalize,
	.BitOr = NativeBitOr,
	.BitAnd = NativeBitAnd,
	.BitXor = NativeBitXor,
	.BitInv = NativeBitInv,
	.BitBits = NativeBitBits,
};

#include "MathExtra.h"

#include <cmath>
#include <algorithm>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NULL_PTR_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Vector pointer is null" };

static void NativeHypot(const HypotQuery* query, HypotResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = std::hypot(query->x, query->y);
}

static void NativeDiag(const DiagQuery* query, DiagResult* result) {
	bufferPos = 0;
	if (query->values == nullptr || query->count == 0) {
		result->error = nullptr;
		result->length = 0.0f;
		return;
	}

	float sumSquares = 0.0f;
	for (uint32_t i = 0; i < query->count; i++) {
		sumSquares += query->values[i] * query->values[i];
	}

	result->error = nullptr;
	result->length = std::sqrt(sumSquares);
}

static void NativeClamp(const ClampQuery* query, ClampResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->clamped = std::clamp(query->value, query->min, query->max);
}

static void NativeSgn(const SgnQuery* query, SgnResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	if (query->value > 0.0f) {
		result->sign = 1.0f;
	} else if (query->value < 0.0f) {
		result->sign = -1.0f;
	} else {
		result->sign = 0.0f;
	}
}

static void NativeMix(const MixQuery* query, MixResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->mixed = query->a * (1.0f - query->t) + query->b * query->t;
}

static void NativeRound(const RoundQuery* query, RoundResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->rounded = std::round(query->value);
}

static void NativeErf(const ErfQuery* query, ErfResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->result = std::erf(query->value);
}

static void NativeSmoothStep(const SmoothStepQuery* query, SmoothStepResult* result) {
	bufferPos = 0;
	float t = std::clamp((query->x - query->edge0) / (query->edge1 - query->edge0), 0.0f, 1.0f);
	result->error = nullptr;
	result->value = t * t * (3.0f - 2.0f * t);
}

static void NativeNormalize(const NormalizeQuery* query, NormalizeResult* result) {
	bufferPos = 0;
	if (query->vec == nullptr) {
		result->error = &NULL_PTR_ERROR;
		return;
	}

	const float length = std::sqrt(query->vec->x * query->vec->x +
	                               query->vec->y * query->vec->y +
	                               query->vec->z * query->vec->z);

	if (length > 0.0f) {
		const float invLength = 1.0f / length;
		query->vec->x *= invLength;
		query->vec->y *= invLength;
		query->vec->z *= invLength;
	}

	result->error = nullptr;
	result->length = length;
}

static void NativeBitOr(const BitOrQuery* query, BitOrResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = query->a | query->b;
}

static void NativeBitAnd(const BitAndQuery* query, BitAndResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = query->a & query->b;
}

static void NativeBitXor(const BitXorQuery* query, BitXorResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = query->a ^ query->b;
}

static void NativeBitInv(const BitInvQuery* query, BitInvResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = ~query->a;
}

static void NativeBitBits(const BitBitsQuery* query, BitBitsResult* result) {
	bufferPos = 0;
	if (query->startBit > query->endBit || query->endBit > 31) {
		result->error = nullptr;
		result->bits = 0;
		return;
	}

	const uint32_t numBits = query->endBit - query->startBit + 1;
	const uint32_t mask = (numBits == 32) ? 0xFFFFFFFF : ((1u << numBits) - 1);

	result->error = nullptr;
	result->bits = (query->value >> query->startBit) & mask;
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

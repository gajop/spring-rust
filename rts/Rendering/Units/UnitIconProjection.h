/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cmath>

#include "System/Matrix44f.h"
#include "System/simd_compat.h"

// One immutable camera snapshot shared by the unit-icon update batch.
class UnitIconProjection {
public:
	UnitIconProjection(const CMatrix44f& matrix, const float3& cameraRight, int viewportWidth)
		: right(cameraRight)
		, width(_mm_set1_ps(viewportWidth))
	{
		// Lanes hold {center.x, center.w, radius.x, radius.w}. Keep the
		// multiply/add order of CMatrix44f::operator*(float4).
		for (int i = 0; i < 4; ++i)
			columns[i] = _mm_setr_ps(matrix.m[4 * i], matrix.m[4 * i + 3], matrix.m[4 * i], matrix.m[4 * i + 3]);
	}

	float GetProjectedRadius(const float3& position, float radius) const
	{
		const float3 radiusPosition = position + right * radius;
		__m128 clip = _mm_mul_ps(columns[0], _mm_setr_ps(position.x, position.x, radiusPosition.x, radiusPosition.x));
		clip = _mm_add_ps(clip, _mm_mul_ps(columns[1], _mm_setr_ps(position.y, position.y, radiusPosition.y, radiusPosition.y)));
		clip = _mm_add_ps(clip, _mm_mul_ps(columns[2], _mm_setr_ps(position.z, position.z, radiusPosition.z, radiusPosition.z)));
		clip = _mm_add_ps(clip, _mm_mul_ps(columns[3], _mm_set1_ps(1.0f)));

		// CalcViewPortCoordinates uses multiplication by 1/w, followed by
		// width * (clip.x + 1) * 0.5. Do not cancel the common translation:
		// the rounded screen coordinates also determine icon picking radius.
		const __m128 w = _mm_shuffle_ps(clip, clip, _MM_SHUFFLE(3, 3, 1, 1));
		const __m128 ndc = _mm_mul_ps(clip, _mm_div_ps(_mm_set1_ps(1.0f), w));
		const __m128 pixels = _mm_mul_ps(_mm_mul_ps(width, _mm_add_ps(ndc, _mm_set1_ps(1.0f))), _mm_set1_ps(0.5f));
		return std::abs(_mm_cvtss_f32(pixels) - _mm_cvtss_f32(_mm_movehl_ps(pixels, pixels)));
	}

private:
	float3 right;
	__m128 width;
	__m128 columns[4];
};

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <array>
#include <bit>
#include <cstdint>
#include <limits>

#include "Rendering/Units/UnitIconProjection.h"

#include <catch_amalgamated.hpp>

namespace {
	// The previous screen-icon path called CalcViewPortCoordinates twice.
	float ReferenceScreenX(const CMatrix44f& matrix, const float3& position, int width)
	{
		const float4 projected = matrix * float4(position, 1.0f);
		const float3 clip = float3(projected) / projected.w;
		return width * (clip.x + 1.0f) * 0.5f;
	}

	float ReferenceRadius(const CMatrix44f& matrix, const float3& right, const float3& position, float radius, int width)
	{
		const float3 radiusPosition = position + right * radius;
		return std::abs(ReferenceScreenX(matrix, position, width) - ReferenceScreenX(matrix, radiusPosition, width));
	}

	void CheckSameFloat(float actual, float expected)
	{
		if (std::isnan(expected)) {
			CHECK(std::isnan(actual));
		} else {
			CHECK(std::bit_cast<uint32_t>(actual) == std::bit_cast<uint32_t>(expected));
		}
	}

	void CheckRadius(const CMatrix44f& matrix, const float3& right, const float3& position, float radius, int width)
	{
		const UnitIconProjection projection(matrix, right, width);
		const float actual = projection.GetProjectedRadius(position, radius);
		const float expected = ReferenceRadius(matrix, right, position, radius, width);
		CheckSameFloat(actual, expected);

		// Preserve picking-radius and strict icon-threshold behavior, including
		// zero extent, points behind the camera, and the exact boundary.
		for (const float limit: {0.0f, 1.0f, 32.0f, expected}) {
			CheckSameFloat(radius * ((limit * 0.9) / actual), radius * ((limit * 0.9) / expected));
			CHECK((actual < limit * 0.9) == (expected < limit * 0.9));
		}
	}
}

TEST_CASE("UnitIconProjection matches full projection for camera and viewport changes")
{
	const CMatrix44f view = CMatrix44f::LookAtViewDirection({16384.0f, 12000.0f, 16384.0f}, {0.0f, -1.0f, 0.0f}, {0.0f, 0.0f, -1.0f});
	const std::array matrices = {
		CMatrix44f{},
		CMatrix44f::PerspProj(-1.6f, 1.6f, -0.9f, 0.9f, 1.0f, 100000.0f) * view,
		CMatrix44f::PerspProj(-0.7f, 1.3f, -1.1f, 0.8f, 0.01f, 50000.0f),
		CMatrix44f::OrthoProj(-10000.0f, 10000.0f, -6000.0f, 6000.0f, 0.1f, 100000.0f) * view,
	};
	for (const auto& matrix: matrices) {
		for (const int width: {0, 1, 8, 511, 1280, 3840, 7680}) {
			for (const float3& right: {float3{1.0f, 0.0f, 0.0f}, float3{0.70710677f, 0.0f, 0.70710677f}}) {
				for (const float3& position: {float3{}, float3{16384.0f, 300.0f, 16384.0f}, float3{-32768.0f, -1000.0f, 40000.0f}}) {
					for (const float radius: {0.0f, 0.001f, 1.0f, 100.0f, 4096.0f})
						CheckRadius(matrix, right, position, radius, width);
				}
			}
		}
	}
}

TEST_CASE("UnitIconProjection preserves floating point edges and snapshots its inputs")
{
	const float inf = std::numeric_limits<float>::infinity();
	const float nan = std::numeric_limits<float>::quiet_NaN();
	for (const float w: {0.0f, -0.0f, 1.0f, -1.0f, std::numeric_limits<float>::min(), inf, nan}) {
		CMatrix44f matrix;
		matrix.m[15] = w;
		for (const float x: {0.0f, -0.0f, 1.0f, -1.0f, std::numeric_limits<float>::denorm_min(), std::numeric_limits<float>::max(), inf, -inf, nan}) {
			for (const float radius: {0.0f, 1.0f, inf, nan})
				CheckRadius(matrix, {1.0f, 0.0f, 0.0f}, {x, 0.0f, 1.0f}, radius, 1280);
		}
	}

	CMatrix44f matrix;
	float3 right{1.0f, 0.0f, 0.0f};
	const UnitIconProjection snapshot(matrix, right, 1280);
	const float expected = ReferenceRadius(matrix, right, {}, 1.0f, 1280);
	matrix.m[0] = 10.0f;
	right.x = 20.0f;
	CheckSameFloat(snapshot.GetProjectedRadius({}, 1.0f), expected);
	CheckRadius(matrix, right, {}, 1.0f, 1280);
}

TEST_CASE("UnitIconProjection matches arbitrary projective matrices")
{
	uint32_t state = 0x44f12345;
	auto next = [&state]() {
		state = state * 1664525u + 1013904223u;
		return (float(state >> 8) / float(1u << 24) - 0.5f);
	};
	for (int i = 0; i < 4096; ++i) {
		CMatrix44f matrix;
		for (float& coefficient: matrix.m)
			coefficient = next() * 10.0f;
		const float3 position{next() * 65536.0f, next() * 65536.0f, next() * 65536.0f};
		const float3 right{next(), next(), next()};
		CheckRadius(matrix, right, position, next() * 4096.0f, 1 + i % 7680);
	}
}

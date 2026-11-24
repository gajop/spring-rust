/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>

#include "System/float3.h"
#include "Sim/Misc/QuadFieldUnitMembership.h"

// Shared by the engine and the differential test. The old scan chooses the last
// equal-distance candidate in team / row-major quad / insertion order. Rings
// cannot reproduce that order locally: replay the old scan only for a final tie.
template<typename Unit>
struct NearestUnitResult {
	float closeSqDist;
	Unit* closeUnit = nullptr;
	bool tied = false;

	explicit NearestUnitResult(float radiusSquared): closeSqDist(radiusSquared) {}

	void Consider(Unit* unit, float distanceSquared) {
		if (distanceSquared <= closeSqDist) {
			tied = (closeUnit != nullptr && distanceSquared == closeSqDist);
			closeSqDist = distanceSquared;
			closeUnit = unit;
		}
	}
};

// Visits exactly GetQuads' circle cells, without materializing the whole circle.
// maxOffset bounds the distance between a unit's quad-indexed pos and midPos.
// The centre must be in bounds; callers fall back for unsupported inputs.
template<typename Field, typename Visit, typename Best>
void VisitNearestUnitQuads(const Field& field, const float3& pos, float radius,
	const QuadFieldUnitQueryRectangle& area, float maxOffset, Visit visit, Best best)
{
	const int size = field.GetQuadSizeX(); // square quads, as in GetQuads
	const int cx = std::clamp(int(pos.x / size), area.minX, area.maxX);
	const int cz = std::clamp(int(pos.z / size), area.minZ, area.maxZ);
	const int last = std::max({cx - area.minX, area.maxX - cx, cz - area.minZ, area.maxZ - cz});
	const float circleSquared = (radius + size * 0.72f) * (radius + size * 0.72f);
	auto cell = [&](int x, int z) {
		if (x < area.minX || x > area.maxX || z < area.minZ || z > area.maxZ)
			return;
		const float3 centre(x * size + size * 0.5f, 0, z * size + size * 0.5f);
		if (pos.SqDistance2D(centre) < circleSquared)
			visit(z * field.GetNumQuadsX() + x);
	};
	for (int ring = 0; ring <= last; ++ring) {
		if (ring > 0) {
			// Every unvisited origin lies beyond one of these four edges.
			// One elmo of slack also covers float rounding on the largest maps.
			const float inner = std::min({
				pos.x - (cx - ring + 1) * size, (cx + ring) * size - pos.x,
				pos.z - (cz - ring + 1) * size, (cz + ring) * size - pos.z
			}) - maxOffset - 1.0f;
			if (inner > 0.0f && inner * inner > best())
				break;
		}
		const int loX = cx - ring, hiX = cx + ring;
		const int loZ = cz - ring, hiZ = cz + ring;
		for (int x = std::max(loX, area.minX); x <= std::min(hiX, area.maxX); ++x) {
			cell(x, loZ);
			if (ring != 0) cell(x, hiZ);
		}
		for (int z = std::max(loZ + 1, area.minZ); z <= std::min(hiZ - 1, area.maxZ); ++z) {
			cell(loX, z);
			if (ring != 0) cell(hiX, z);
		}
	}
}

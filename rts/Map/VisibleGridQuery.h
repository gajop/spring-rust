/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>
#include <cstddef>

namespace visiblegrid {

// Bounds are in quads; frustum lines are in terrain squares. Row visitors may
// stop the query, allowing sparse callers to cap work before visiting empty cells.
struct Bounds {
	int width, height;
	int firstX, lastX, firstY, lastY;
};

template<typename Line, typename VisitRow>
bool VisitRows(const Bounds& bounds, int quadSize, int extraSize, const Line* negLines, const Line* posLines, VisitRow&& visit)
{
	for (int y = bounds.firstY; y <= bounds.lastY; ++y) {
		int sx = bounds.firstX;
		int ex = bounds.lastX;
		for (int i = 0; i < negLines[4].sign; ++i) {
			const auto& line = negLines[i];
			float x = std::min(line.base + line.dir * (y * quadSize), line.base + line.dir * ((y + 1) * quadSize));
			x = std::clamp(x / quadSize, -1.0f, float(bounds.width) + 1.0f);
			if ((x - extraSize) > sx)
				sx = int(x) - extraSize;
		}
		for (int i = 0; i < posLines[4].sign; ++i) {
			const auto& line = posLines[i];
			float x = std::max(line.base + line.dir * (y * quadSize), line.base + line.dir * ((y + 1) * quadSize));
			x = std::clamp(x / quadSize, -1.0f, float(bounds.width) + 1.0f);
			if ((x + extraSize) < ex)
				ex = int(x) + extraSize;
		}
		if (!visit(sx, ex, y))
			return false;
	}
	return true;
}

template<typename Line, typename Drawer>
bool CollectBounded(const Bounds& bounds, int quadSize, const Line* negLines, const Line* posLines, Drawer& drawer, size_t population)
{
	// Empty-cell lookup is cheaper than filtering an object, and large objects
	// can occur in several cell lists. Do not mistake references for unique IDs.
	const size_t cellBudget = std::max(size_t(64), population);
	const size_t candidateBudget = population * 2;
	size_t cells = 0;
	return VisitRows(bounds, quadSize, 0, negLines, posLines, [&](int sx, int ex, int y) {
		const size_t run = size_t(std::max(0, ex - sx + 1));
		if (run > cellBudget - cells)
			return false;
		cells += run;
		for (int x = sx; x <= ex; ++x) {
			drawer.DrawQuad(x, y);
			if (drawer.GetObjectCount() >= candidateBudget)
				return false;
		}
		return true;
	});
}

} // namespace visiblegrid

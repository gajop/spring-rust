/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <array>
#include <utility>
#include <vector>

#include <catch_amalgamated.hpp>
#include "Map/VisibleGridQuery.h"
#include "Sim/Misc/QuadFieldStorage.h"

namespace {

struct Line { float base = 0, dir = 0; int sign = 0; };
using Lines = std::array<Line, 5>;

struct Drawer {
	size_t references = 0;
	size_t referencesPerCell = 0;
	std::vector<std::pair<int, int>> cells;
	void DrawQuad(int x, int y) {
		cells.emplace_back(x, y);
		references += referencesPerCell;
	}
	size_t GetObjectCount() const { return references; }
};

} // namespace

TEST_CASE("Null-map storage uses dense cells only within a fixed metadata budget")
{
	for (const size_t side: {32, 64, 128, 256})
		CHECK_FALSE(quadstorage::UseSparse(0, side * side, 168));
	CHECK(quadstorage::UseSparse(0, 512 * 512, 168));
	CHECK(quadstorage::UseSparse(0, 4096 * 4096, 168));
	CHECK(quadstorage::UseSparse(0, 256 * 256, 120 + 64 * 24));
	CHECK(quadstorage::UseSparse(2, 1, 168));
	CHECK_FALSE(quadstorage::UseSparse(1, 4096 * 4096, 168));
	CHECK(quadstorage::UseSparse(0, size_t(-1), 168));
}

TEST_CASE("Visible grid rows retain ordered clipping, padding and empty rows")
{
	const visiblegrid::Bounds bounds{9, 7, 0, 8, 0, 6};
	Lines neg{}, pos{};
	neg[0].base = 4;
	pos[0].base = 10;
	neg[4].sign = pos[4].sign = 1;
	for (int padding: {0, 1, 2}) {
		std::vector<std::pair<int, int>> cells;
		CHECK(visiblegrid::VisitRows(bounds, 2, padding, neg.data(), pos.data(), [&](int sx, int ex, int y) {
			for (int x = sx; x <= ex; ++x) cells.emplace_back(x, y);
			return true;
		}));
		std::vector<std::pair<int, int>> expected;
		for (int y = 0; y < 7; ++y)
			for (int x = 2 - padding; x <= 5 + padding; ++x) expected.emplace_back(x, y);
		CHECK(cells == expected);
	}
	// This angled wedge lies off-map on its final rows. Both line endpoints
	// matter: the left uses the minimum and the right uses the maximum.
	neg[0] = {0, 1, 0};
	pos[0] = {4, -1, 0};
	Drawer wedge;
	CHECK(visiblegrid::VisitRows(bounds, 2, 0, neg.data(), pos.data(), [&](int sx, int ex, int y) {
		for (int x = sx; x <= ex; ++x) wedge.DrawQuad(x, y);
		return true;
	}));
	const std::vector<std::pair<int, int>> expected{{0, 0}, {1, 0}, {2, 0}, {1, 1}};
	CHECK(wedge.cells == expected);
}

TEST_CASE("Sparse visibility keeps crowded close views despite duplicated references")
{
	Lines neg{}, pos{};
	neg[0].base = 10;
	pos[0].base = 11;
	neg[4].sign = pos[4].sign = 1;
	Drawer drawer;
	drawer.referencesPerCell = 600;
	// 1200 references can represent far fewer than 1000 distinct objects.
	// The previous 3/4-population budget needlessly rejected this grid.
	CHECK(visiblegrid::CollectBounded({32, 1, 0, 31, 0, 0}, 1, neg.data(), pos.data(), drawer, 1000));
	CHECK(drawer.cells.size() == 2);
	CHECK(drawer.references == 1200);

	// A moderately sized view also must not trip the old population/8 limit.
	Drawer region;
	Lines all{};
	CHECK(visiblegrid::CollectBounded({20, 20, 0, 19, 0, 19}, 1, all.data(), all.data(), region, 1000));
	CHECK(region.cells.size() == 400);
}

TEST_CASE("Sparse visibility bounds work on huge empty maps and dense clusters")
{
	Lines all{};
	for (const size_t population: {1000, 4000, 10000, 32000}) {
		Drawer empty;
		CHECK_FALSE(visiblegrid::CollectBounded({4096, 4096, 0, 4095, 0, 4095}, 1, all.data(), all.data(), empty, population));
		CHECK(empty.cells.size() <= population);
	}
	Drawer cluster;
	cluster.referencesPerCell = 1000;
	CHECK_FALSE(visiblegrid::CollectBounded({32, 32, 0, 31, 0, 31}, 1, all.data(), all.data(), cluster, 1000));
	CHECK(cluster.cells.size() == 2);
	CHECK(cluster.references == 2000);

	// Empty/off-map frusta complete with no candidates rather than underflowing
	// the run count or converting an empty run into a huge unsigned length.
	Lines neg{}, pos{};
	neg[0].base = 10000;
	pos[0].base = -10000;
	neg[4].sign = pos[4].sign = 1;
	Drawer outside;
	CHECK(visiblegrid::CollectBounded({32, 32, 0, 31, 0, 31}, 1, neg.data(), pos.data(), outside, 1000));
	CHECK(outside.cells.empty());
}

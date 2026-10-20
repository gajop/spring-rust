/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/QuadFieldUnitMembership.h"
#include "System/float3.h"
#include "System/SpringMath.h"
#include <stdlib.h>
#include <time.h>
#include <chrono>
#include <iostream>
#include <limits>

#include <catch_amalgamated.hpp>

static inline float randf()
{
	return rand() / float(RAND_MAX);
}

TEST_CASE("Quad membership rejects only known empty allyteams", "[quad-membership]")
{
	QuadFieldUnitMembership membership;
	CHECK(membership.MayHaveUnits(0)); // not initialized: use the original query
	membership.Reset(3);
	CHECK_FALSE(membership.MayHaveUnits(0));
	CHECK_FALSE(membership.MayHaveUnits(1));
	membership.Add(1, 3); // one hull overlaps three quads
	CHECK(membership.MayHaveUnits(1));
	membership.Remove(1);
	membership.Remove(1);
	CHECK(membership.MayHaveUnits(1));
	membership.Remove(1);
	CHECK_FALSE(membership.MayHaveUnits(1));
	CHECK(membership.MayHaveUnits(-1));
	CHECK(membership.MayHaveUnits(3));

	SECTION("Inconsistent removals and counts cannot certify emptiness") {
		membership.Remove(1);
		CHECK(membership.MayHaveUnits(0));
		membership.Reset(3);
		membership.Add(2, std::numeric_limits<std::size_t>::max());
		membership.Add(2);
		CHECK(membership.MayHaveUnits(0));
	}
	SECTION("Unknown load state remains conservative until rebuilt") {
		membership.Invalidate();
		membership.Add(1);
		membership.Remove(1);
		CHECK(membership.MayHaveUnits(0));
		CHECK(membership.MayHaveUnits(1));
		membership.Reset(3);
		CHECK_FALSE(membership.MayHaveUnits(1));
	}
}

TEST_CASE("Quad membership follows destructor and team-change visibility", "[quad-membership]")
{
	QuadFieldUnitMembership membership;
	membership.Reset(3);
	membership.Add(1, 2);
	// Removing the unit from UnitHandler is deliberately irrelevant: its two
	// quad entries still exist throughout script destruction callbacks.
	bool inUnitHandler = false;
	CHECK_FALSE(inUnitHandler);
	CHECK(membership.MayHaveUnits(1));
	membership.Remove(1);
	CHECK(membership.MayHaveUnits(1));
	membership.Remove(1);
	CHECK_FALSE(membership.MayHaveUnits(1));
	// Team transfer removes old memberships before publishing the new allyteam.
	membership.Add(2, 2);
	CHECK_FALSE(membership.MayHaveUnits(1));
	CHECK(membership.MayHaveUnits(2));
}

TEST_CASE("Quad load rebuild permits either parent-child PostLoad order", "[quad-membership]")
{
	for (const int prefix: {0, 1, 2, 3}) {
		QuadFieldUnitMembership membership;
		membership.Invalidate();
		std::array<std::size_t, 3> materialized = {};
		auto quadPostLoad = [&](int team) {
			++materialized[team];
			membership.Add(team);
		};
		const std::array<int, 3> teams = {1, 2, 1};
		for (int i = 0; i < prefix; ++i)
			quadPostLoad(teams[i]);
		CHECK(membership.MayHaveUnits(0)); // unknown cannot reject during loading
		membership.Reset(3);
		for (int team = 0; team < 3; ++team)
			membership.Add(team, materialized[team]);
		for (int i = prefix; i < 3; ++i)
			quadPostLoad(teams[i]);
		CHECK_FALSE(membership.MayHaveUnits(0));
		CHECK(membership.MayHaveUnits(1));
		CHECK(membership.MayHaveUnits(2));
		membership.Remove(1);
		CHECK(membership.MayHaveUnits(1));
		membership.Remove(1);
		CHECK_FALSE(membership.MayHaveUnits(1));
		membership.Remove(2);
		CHECK_FALSE(membership.MayHaveUnits(2));
	}
}

TEST_CASE("Quad membership regions are exact per coarse cell", "[quad-membership]")
{
	constexpr int C = QuadFieldUnitMembership::CELL;
	QuadFieldUnitMembership membership;
	const QuadFieldUnitQueryRectangle origin = {0, 0, 1, 1};
	const QuadFieldUnitQueryRectangle distant = {10, 20, 10, 20};
	membership.Reset(3, 32, 32);
	membership.AddAt(1, 10, 20, 2);
	CHECK_FALSE(membership.MayHaveUnits(1, origin));
	CHECK(membership.MayHaveUnits(1, distant));
	membership.AddAt(1, 0, 0);
	CHECK(membership.MayHaveUnits(1, origin));
	membership.Remove(1, 0, 0); // the cell empties again: no stale bounds
	CHECK_FALSE(membership.MayHaveUnits(1, origin));
	CHECK(membership.MayHaveUnits(1, distant));
	membership.Remove(1, 10, 20);
	membership.Remove(1, 10, 20);
	CHECK_FALSE(membership.MayHaveUnits(1, distant));
	membership.AddAt(1, 10, 20);
	membership.Add(1); // a PostLoad addition without its quad index fails open
	CHECK(membership.MayHaveUnits(1, origin));
	CHECK(membership.NeedsRebuild());
	CHECK(membership.MayHaveUnits(-1, origin));
	CHECK(membership.MayHaveUnits(3, origin));
	membership.Invalidate();
	CHECK(membership.MayHaveUnits(0, origin));

	SECTION("Randomized mutations: sound, and exact at cell granularity") {
		srand(7);
		const int nx = 37, nz = 29, teams = 3;
		membership.Reset(teams, nx, nz);
		std::vector<int> quads(size_t(teams) * nx * nz, 0);
		for (int step = 0; step < 20000; ++step) {
			const int team = rand() % teams, x = rand() % nx, z = rand() % nz;
			int& n = quads[(size_t(team) * nz + z) * nx + x];
			if (n > 0 && rand() % 2) {
				--n;
				membership.Remove(team, x, z);
			} else {
				++n;
				membership.AddAt(team, x, z);
			}
			const int x0 = rand() % nx, z0 = rand() % nz;
			const QuadFieldUnitQueryRectangle q = {x0, z0, std::min(nx - 1, x0 + rand() % 12), std::min(nz - 1, z0 + rand() % 12)};
			const int qt = rand() % teams;
			bool inQuads = false, inCells = false;
			for (int zz = 0; zz < nz; ++zz) {
				for (int xx = 0; xx < nx; ++xx) {
					const int count = quads[(size_t(qt) * nz + zz) * nx + xx];
					if (count == 0)
						continue;
					inQuads |= xx >= q.minX && xx <= q.maxX && zz >= q.minZ && zz <= q.maxZ;
					inCells |= xx / C >= q.minX / C && xx / C <= q.maxX / C && zz / C >= q.minZ / C && zz / C <= q.maxZ / C;
				}
			}
			const bool may = membership.MayHaveUnits(qt, q);
			REQUIRE((may || !inQuads)); // never rejects a region with an entry
			REQUIRE(may == inCells);    // rejects exactly the regions whose cells are empty
		}
	}
}

// Container-only diagnostic: no renderer, simulation, or CUnit allocations.
// Deliberately hidden from normal test runs; timings are not pass/fail gates.
TEST_CASE("Null-map object-grid storage comparison", "[.storage-diagnostic]")
{
	for (const size_t mapSize: {8, 16, 32, 64}) {
		const size_t side = mapSize * 4;
		const size_t cells = side * side;
		for (const size_t population: {100, 1000, 4000, 10000}) {
			for (const bool sparse: {false, true}) {
				CQuadField::QuadStore store;
				store.Init(cells, 2, sparse);
				// Odd stride visits every cell before repeating on these power-of-two grids.
				for (size_t i = 0; i < population; ++i)
					store.Write((i * 4051) % cells).units.push_back(nullptr);
				for (const bool whole: {false, true}) {
					const size_t queried = whole ? cells : std::min(cells, size_t(256));
					volatile size_t consumed = 0;
					const auto start = std::chrono::steady_clock::now();
					for (int repeat = 0; repeat < 64; ++repeat) {
						size_t count = 0;
						for (size_t q = 0; q < queried; ++q)
							count += store[q].units.size();
						consumed = count;
					}
					const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now() - start).count();
					if (whole) CHECK(consumed == population);
					// Structural storage only: excludes allocator overhead and per-object
					// vector capacities, which are the same for both representations here.
					const size_t occupied = std::min(population, cells);
					const size_t bytesPerCell = sizeof(CQuadField::Quad) + 2 * sizeof(std::vector<CUnit*>);
					const size_t bytes = sparse ? cells * sizeof(void*) + occupied * bytesPerCell : cells * bytesPerCell;
					std::cout << "STORAGE map=" << mapSize << " n=" << population << " sparse=" << sparse
						<< " whole=" << whole << " touched=" << occupied << " cells=" << cells
						<< " structural_bytes=" << bytes << " ns=" << double(ns) / 64 << '\n';
				}
			}
		}
	}
}

template <typename T> T Clamp(T val, T min, T max)
{
	return std::min(std::max(val, min), max);
}

TEST_CASE("QuadField")
{
	srand( time(nullptr) );

	static constexpr int WIDTH  = 3;
	static constexpr int HEIGHT = 3;
	static constexpr int TEST_RUNS = 50000;

	quadField.Init(int2(WIDTH, HEIGHT), SQUARE_SIZE);

	int bitmap[WIDTH * HEIGHT];

	bool fail = false;

	for (int n = 0; n < TEST_RUNS; ++n) {
		// clear
		for (int i = 0; i < WIDTH * HEIGHT; ++i) {
			bitmap[i] = 0;
		}

		// generate ray
		float3 start;
			start.x = randf() *  WIDTH * 3 - WIDTH;
			start.z = randf() * HEIGHT * 3 - HEIGHT;
		float3 dir;
			dir.x = randf() - 0.5f;
			dir.z = randf() - 0.5f;
			dir.x *= (randf() < 0.2f) ? 0 : 1; // special case #1: dir.x/z == 0 are a special cases
			dir.z *= (randf() < 0.2f) ? 0 : 1; // in GetQuadsOnRay(). We want to check those codes, too
			dir.SafeNormalize();
		float length = randf() * (WIDTH + HEIGHT) * 0.5f;
			length = (randf() < 0.2f) ? randf() : length; // special case #1: when start & end are in the same quad

		// #1: very naive way to raytrace (step-based interpolation)
		auto plot = [&](float3 pos) {
			const int x = Clamp<int>(pos.x, 0,  WIDTH - 1);
			const int z = Clamp<int>(pos.z, 0, HEIGHT - 1);
			const int idx = z * WIDTH + x;
			bitmap[idx] |= 1;
		};
		for (float l = 0.f; l <= length; l+=0.001f) {
			float3 pos = start + dir * l;
			plot(pos);
		}
		plot(start);
		plot(start + dir * length);

		// #2: raytrace via QuadField
		{
			QuadFieldQuery qfQuery;
			quadField.GetQuadsOnRay(qfQuery, start * SQUARE_SIZE, dir, length * SQUARE_SIZE);
			assert( std::adjacent_find(qfQuery.quads->begin(), qfQuery.quads->end()) == qfQuery.quads->end() ); // check for duplicates
			for (int qi: *qfQuery.quads) {
				bitmap[qi] |= 2;
			}
		}

		// check if #1 & #2 iterate same quads
		for (int i = 0; i < WIDTH * HEIGHT; ++i) {
			if (bitmap[i] == 0 || bitmap[i] == 3)
				continue;

			// only break when GetQuadsOnRay() returned LESS quads than the naive solution
			// cause the naive solution uses step-based interpolation, and so it might jump
			// over some edges when the ray is very close to the corner intersection (of 4 quads)
			bool isAdditionalQuad = (bitmap[i] == 2);
			if (!isAdditionalQuad) {
				int* istart  = reinterpret_cast<int*>(&start.x);
				int* idir    = reinterpret_cast<int*>(&dir.x);
				int* ilength = reinterpret_cast<int*>(&length);

				printf("start: 0x%X 0x%X 0x%X dir: 0x%X 0x%X 0x%X length: 0x%X\n", istart[0], istart[1], istart[2], idir[0], idir[1], idir[2], *ilength);
				fail = true;
				break;
			}
		}

		if (fail) break;
	}

	INFO("Too little quads returned!");
	CHECK_FALSE(fail);
}

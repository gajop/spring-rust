/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <array>
#include <limits>
#include <random>
#include <vector>

#include "Game/NearestUnitQuery.h"
#include "Sim/Misc/QuadField.h"

namespace {
	constexpr int TEAMS = 4;

	struct Unit {
		float3 midPos;
		int team;
		int seen;
		bool neutral, aircraft, valid;
		float sight;
	};

	struct Layout {
		// QuadFieldQuery returns buffers to the engine singleton.
		CQuadField& field = quadField;
		std::vector<Unit> units;
		std::vector<std::array<std::vector<Unit*>, TEAMS>> members;
		float maxOffset = 0;
		const float oldMaxX = float3::maxxpos, oldMaxZ = float3::maxzpos;

		Layout() {
			field.Init(int2(1024, 768), 128, true);
			float3::maxxpos = 8191;
			float3::maxzpos = 6143;
			members.resize(field.GetNumQuadsX() * field.GetNumQuadsZ());
			units.reserve(512);
		}
		~Layout() {
			field.Kill();
			float3::maxxpos = oldMaxX;
			float3::maxzpos = oldMaxZ;
		}

		void Add(float3 pos, float3 offset, float radius, int team = 2,
			int seen = 3, bool neutral = false, bool aircraft = true, bool valid = true, float sight = 10000)
		{
			REQUIRE(units.size() < units.capacity());
			units.push_back({pos + offset, team, seen, neutral, aircraft, valid, sight});
			maxOffset = std::max(maxOffset, offset.Length());
			QuadFieldQuery quads;
			field.GetQuads(quads, pos, radius);
			for (int qi: *quads.quads)
				members[qi][team].push_back(&units.back());
		}
	};

	enum class Mode { Enemy, Friendly, Aircraft, ValidTarget, Sight };

	struct Search {
		Mode mode = Mode::Enemy;
		int ally = 0;
		Unit* exclude = nullptr;

		bool Team(int team) const {
			// Two allied allyteams, two enemies: exercise legacy outer team order.
			return ((team / 2 == ally / 2) == (mode == Mode::Friendly));
		}
		bool Accept(const Unit* u, float distanceSquared) const {
			if (u == exclude) return false;
			if (mode == Mode::Friendly) return true;
			if (u->neutral) return false;
			if (mode == Mode::Sight) return distanceSquared <= u->sight * u->sight;
			return (u->seen & (1 << ally)) != 0
				&& (mode != Mode::Aircraft || u->aircraft)
				&& (mode != Mode::ValidTarget || u->valid);
		}
	};

	Unit* Find(Layout& layout, float3 pos, float radius, Search search, bool rings, int* visited = nullptr, bool* replayed = nullptr)
	{
		NearestUnitResult<Unit> result(radius * radius);
		std::vector<Unit*> seen;
		auto visitTeam = [&](int qi, int team) {
			for (Unit* u: layout.members[qi][team]) {
				if (std::find(seen.begin(), seen.end(), u) != seen.end()) continue;
				seen.push_back(u);
				const float distanceSquared = (pos - u->midPos).SqLength2D();
				if (distanceSquared <= result.closeSqDist && search.Accept(u, distanceSquared))
					result.Consider(u, distanceSquared);
			}
		};
		QuadFieldUnitQueryRectangle area;
		if (rings && pos.x >= 0 && pos.x <= float3::maxxpos && pos.z >= 0 && pos.z <= float3::maxzpos
			&& QuadFieldGetUnitQueryRectangle(layout.field, pos, radius, area)) {
			VisitNearestUnitQuads(layout.field, pos, radius, area, layout.maxOffset, [&](int qi) {
				if (visited != nullptr) ++*visited;
				for (int team = 0; team < TEAMS; ++team)
					if (search.Team(team)) visitTeam(qi, team);
			}, [&] { return result.closeSqDist; });
			if (!result.tied) return result.closeUnit;
			if (replayed != nullptr) *replayed = true;
			result = NearestUnitResult<Unit>(radius * radius);
			seen.clear();
		}
		// Unmodified legacy order over the actual engine's GetQuads output.
		QuadFieldQuery quads;
		layout.field.GetQuads(quads, pos, radius);
		for (int team = 0; team < TEAMS; ++team) {
			if (!search.Team(team)) continue;
			for (int qi: *quads.quads) visitTeam(qi, team);
		}
		return result.closeUnit;
	}
}

TEST_CASE("Nearest rings match legacy cells and unit choice on random layouts", "[nearest-unit]")
{
	std::mt19937 random(7);
	auto next = [&](int n) { return int(random() % n); };
	for (int scene = 0; scene < 24; ++scene) {
		Layout layout;
		for (int i = 0; i < 240; ++i) {
			layout.Add(float3(next(8704) - 256, next(1000), next(6656) - 256),
				float3(next(801) - 400, next(401) - 200, next(801) - 400), next(400),
				next(TEAMS), next(16), next(7) == 0, next(2), next(2), next(6000));
		}
		for (int query = 0; query < 100; ++query) {
			const float3 pos(next(8192), next(2000), next(6144));
			const float radius = next(12000);
			QuadFieldUnitQueryRectangle area;
			REQUIRE(QuadFieldGetUnitQueryRectangle(layout.field, pos, radius, area));
			std::vector<int> cells;
			VisitNearestUnitQuads(layout.field, pos, radius, area, layout.maxOffset,
				[&](int qi) { cells.push_back(qi); }, [] { return std::numeric_limits<float>::infinity(); });
			std::sort(cells.begin(), cells.end());
			QuadFieldQuery old;
			layout.field.GetQuads(old, pos, radius);
			REQUIRE(cells == *old.quads); // also checks no duplicate visits
			for (Mode mode: {Mode::Enemy, Mode::Friendly, Mode::Aircraft, Mode::ValidTarget, Mode::Sight}) {
				Search search{mode, next(TEAMS), &layout.units[next(layout.units.size())]};
				INFO("scene=" << scene << " query=" << query << " mode=" << int(mode));
				REQUIRE(Find(layout, pos, radius, search, true) == Find(layout, pos, radius, search, false));
			}
		}
	}
}

TEST_CASE("Nearest rings preserve exact edge and final tie order", "[nearest-unit]")
{
	Layout layout;
	const float3 at(4096, 0, 3072);
	SECTION("Radius edge is inclusive, across enemy teams") {
		layout.Add(at + float3(128, 0, 0), ZeroVector, 200, 2);
		layout.Add(at - float3(128, 0, 0), ZeroVector, 200, 3);
		bool replayed = false;
		CHECK(Find(layout, at, 128, {}, true, nullptr, &replayed) == &layout.units[1]);
		CHECK(replayed);
	}
	SECTION("Same team ties keep last legacy quad and insertion order") {
		layout.Add(at + float3(128, 0, 0), ZeroVector, 300);
		layout.Add(at - float3(128, 0, 0), ZeroVector, 300);
		layout.Add(at, float3(0, 0, 128), 300);
		CHECK(Find(layout, at, 128, {}, true) == Find(layout, at, 128, {}, false));
	}
	SECTION("A nearer unit clears a previous tie") {
		const float3 centre = at + float3(64, 0, 64);
		layout.Add(centre + float3(20, 0, 0), ZeroVector, 0);
		layout.Add(centre - float3(20, 0, 0), ZeroVector, 0);
		layout.Add(centre, float3(0, 0, 1), 0);
		bool replayed = false;
		CHECK(Find(layout, centre, 1000, {}, true, nullptr, &replayed) == &layout.units[2]);
		CHECK_FALSE(replayed);
	}
	SECTION("A tied unit in the next ring must not be pruned") {
		const float3 centre = at + float3(64, 0, 64);
		layout.Add(centre - float3(64, 0, 0), ZeroVector, 0);
		layout.Add(centre + float3(64, 0, 0), ZeroVector, 0, 3);
		bool replayed = false;
		CHECK(Find(layout, centre, 2000, {}, true, nullptr, &replayed) == &layout.units[1]);
		CHECK(replayed);
	}
	SECTION("Zero radius and empty searches") {
		CHECK(Find(layout, at, 0, {}, true) == nullptr);
		layout.Add(at, ZeroVector, 0);
		CHECK(Find(layout, at, 0, {}, true) == &layout.units[0]);
	}
	SECTION("Model centre offset prevents an early stop") {
		layout.Add(at + float3(50, 0, 0), ZeroVector, 0);
		layout.Add(at + float3(600, 0, 0), float3(-599, 0, 0), 0);
		CHECK(Find(layout, at, 2000, {}, true) == &layout.units[1]);
	}
	SECTION("Queries outside map bounds retain the legacy clamped scan") {
		layout.Add(float3(0, 0, 3072), ZeroVector, 0);
		const float3 outside(-50, 0, 3072);
		CHECK(Find(layout, outside, 200, {}, true) == Find(layout, outside, 200, {}, false));
	}
}

TEST_CASE("Nearest rings stop locally without circle storage on a 1024 map", "[nearest-unit]")
{
	struct LargeField {
		int GetQuadSizeX() const { return 128; }
		int GetNumQuadsX() const { return 4096; }
	} field;
	const float3 pos(262208, 0, 262208);
	const QuadFieldUnitQueryRectangle area{1982, 1982, 2114, 2114};
	int visited = 0;
	VisitNearestUnitQuads(field, pos, 8400, area, 0, [&](int) { ++visited; }, [] { return 1.0f; });
	CHECK(visited == 1);
}

TEST_CASE("Nearest offset bound survives smaller updates and resets with the field", "[nearest-unit]")
{
	Layout layout;
	layout.field.TrackSolidMidPosOffset(float3(3, 4, 0));
	CHECK(layout.field.GetMaxSolidMidPosOffset() == 5);
	layout.field.TrackSolidMidPosOffset(float3(0, 0, 2));
	CHECK(layout.field.GetMaxSolidMidPosOffset() == 5);
	layout.field.Init(int2(1024, 768), 128, true);
	CHECK(layout.field.GetMaxSolidMidPosOffset() == 0);
}

TEST_CASE("Nearest rings reject unsupported query rectangles", "[nearest-unit]")
{
	Layout layout;
	QuadFieldUnitQueryRectangle area;
	CHECK_FALSE(QuadFieldGetUnitQueryRectangle(layout.field, float3(0, 0, 0), -1, area));
	CHECK_FALSE(QuadFieldGetUnitQueryRectangle(layout.field, float3(0, 0, 0), std::numeric_limits<float>::infinity(), area));
	CHECK_FALSE(QuadFieldGetUnitQueryRectangle(layout.field, float3(0, std::numeric_limits<float>::quiet_NaN(), 0), 1, area));
}

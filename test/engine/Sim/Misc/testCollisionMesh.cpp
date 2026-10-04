/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Sim/Misc/CollisionMesh.h"
#include "System/Misc/SpringTime.h"

#include <vector>

#include <catch_amalgamated.hpp>

InitSpringTime ist;

// a closed unit cube, 0..1 on each axis
static CollisionMesh Cube()
{
	const std::vector<float3> v = {
		{0,0,0}, {1,0,0}, {1,1,0}, {0,1,0},
		{0,0,1}, {1,0,1}, {1,1,1}, {0,1,1},
	};
	const std::vector<uint32_t> i = {
		0,1,2, 0,2,3,  4,6,5, 4,7,6,
		0,4,5, 0,5,1,  3,2,6, 3,6,7,
		0,3,7, 0,7,4,  1,5,6, 1,6,2,
	};
	return CollisionMesh::Build(v, i);
}

TEST_CASE("CollisionMesh")
{
	const CollisionMesh cube = Cube();
	CollisionMesh::Hit hit;

	CHECK(cube.NumTriangles() == 12);

	SECTION("a segment through the cube hits its near face") {
		REQUIRE(cube.Intersect({-1.0f, 0.5f, 0.5f}, {3.0f, 0.5f, 0.5f}, &hit));
		CHECK(hit.pos.x == Catch::Approx(0.0f).margin(1e-5f));
		CHECK(hit.t == Catch::Approx(0.25f).margin(1e-5f));
		CHECK(hit.normal.x == Catch::Approx(-1.0f).margin(1e-5f));
	}
	SECTION("a segment that stops short misses") {
		CHECK_FALSE(cube.Intersect({-1.0f, 0.5f, 0.5f}, {-0.1f, 0.5f, 0.5f}, &hit));
	}
	SECTION("a segment beside the cube misses") {
		CHECK_FALSE(cube.Intersect({-1.0f, 1.5f, 0.5f}, {3.0f, 1.5f, 0.5f}, &hit));
	}
	SECTION("a segment starting inside hits the face it leaves through") {
		REQUIRE(cube.Intersect({0.5f, 0.5f, 0.5f}, {0.5f, 0.5f, 3.0f}, &hit));
		CHECK(hit.pos.z == Catch::Approx(1.0f).margin(1e-5f));
	}
	SECTION("many triangles: the nearest of a stack of planes") {
		std::vector<float3> v;
		std::vector<uint32_t> i;
		for (int k = 0; k < 200; k++) {
			const float z = 10.0f + k;
			const uint32_t b = uint32_t(v.size());
			v.insert(v.end(), {{-1, -1, z}, {1, -1, z}, {1, 1, z}, {-1, 1, z}});
			i.insert(i.end(), {b, b + 1, b + 2, b, b + 2, b + 3});
		}
		const CollisionMesh planes = CollisionMesh::Build(v, i);
		REQUIRE(planes.Intersect({0.0f, 0.0f, 500.0f}, {0.0f, 0.0f, 0.0f}, &hit));
		CHECK(hit.pos.z == Catch::Approx(209.0f).margin(1e-3f));
		REQUIRE(planes.Intersect({0.2f, 0.3f, 0.0f}, {0.2f, 0.3f, 500.0f}, &hit));
		CHECK(hit.pos.z == Catch::Approx(10.0f).margin(1e-3f));
	}
}

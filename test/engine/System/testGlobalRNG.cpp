/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#include <catch_amalgamated.hpp>
#include "System/GlobalRNG.h"

TEST_CASE("Explicit unsynced seeds repeat across object addresses") {
	CGlobalUnsyncedRNG first;
	CGlobalUnsyncedRNG second;
	const uint64_t seeds[] = {0, 1, 424242, UINT64_MAX};
	for (const uint64_t seed: seeds) {
		first.SetDeterministicSeed(seed);
		second.SetDeterministicSeed(seed);
		for (int i = 0; i < 500; ++i)
			CHECK(first.NextFloat() == second.NextFloat());
		CHECK(first.GetLastSeed() == seed);
	}
}

TEST_CASE("Explicit stream keeps synced RNG's established sequence") {
	CGlobalSyncedRNG synced;
	CGlobalUnsyncedRNG unsynced;
	synced.Seed(424242);
	unsynced.SetDeterministicSeed(424242);
	for (int i = 0; i < 500; ++i)
		CHECK(synced.NextInt() == unsynced.NextInt());
}

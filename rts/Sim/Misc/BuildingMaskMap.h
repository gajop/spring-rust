/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef BUILDINGMASKMAP_H
#define BUILDINGMASKMAP_H

#include <cstdint>
#include <vector>

#include "System/creg/creg_cond.h"

class BuildingMaskMap
{
	CR_DECLARE_STRUCT(BuildingMaskMap)

public:
	void Init(unsigned int numSquares) {
		maskMap.clear();
		maskMap.resize(numSquares, DEFAULT_MASK);
	};
	void Kill() {
		maskMap.clear();
	}

	bool SetTileMask(unsigned int x, unsigned int z, std::uint16_t value);
	bool TestTileMask(unsigned int x, unsigned int z, std::uint16_t value) const { return (CheckBounds(x, z) && TestTileMaskUnsafe(x, z, value)); }
	bool TestTileMaskUnsafe(unsigned int x, unsigned int z, std::uint16_t value) const;

private:
	bool CheckBounds(unsigned int x, unsigned int z) const;

private:
	static constexpr std::uint16_t DEFAULT_MASK = 1; // 1st bit set to 1 indicates a normal tile

	std::vector<std::uint16_t> maskMap; // empty on a null map
};

extern BuildingMaskMap buildingMaskMap;

#endif

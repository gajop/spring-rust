/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>
#include <cstddef>
#include <limits>
#include <vector>

// Counts actual allyteam-vector entries, including a unit's overlapping quads, per allyteam and
// per coarse cell of CELL x CELL quads. A query is rejected only when every coarse cell it
// overlaps holds no entry of an eligible allyteam, so the test is exact at cell granularity and
// does not degrade as units spread. Unknown or inconsistent metadata never rejects.
struct QuadFieldUnitQueryRectangle {
	int minX, minZ, maxX, maxZ;
};

class QuadFieldUnitMembership {
public:
	static constexpr int CELL = 4;

	// Without quad dimensions only allyteam totals are kept; regional queries fail open.
	void Reset(std::size_t teams, int quadsX = 0, int quadsZ = 0) {
		totals.assign(teams, 0);
		numQuadsX = std::max(0, quadsX);
		numQuadsZ = std::max(0, quadsZ);
		cellsX = (numQuadsX + CELL - 1) / CELL;
		cellsZ = (numQuadsZ + CELL - 1) / CELL;
		cells.assign(teams * std::size_t(cellsX) * cellsZ, 0);
		known = true;
		gridKnown = cellsX > 0 && cellsZ > 0;
	}
	void Invalidate() { known = false; }
	// A known count whose quads are not known: regional queries fail open until a rebuild.
	std::size_t Teams() const { return totals.size(); }
	bool NeedsRebuild() const { return known && !gridKnown && cellsX > 0 && cellsZ > 0; }

	void Add(int team, std::size_t count = 1) {
		if (!CanAdd(team, count) || count == 0)
			return;
		totals[team] += count;
		gridKnown = false;
	}

	void AddAt(int team, int x, int z, std::size_t count = 1) {
		if (!CanAdd(team, count) || count == 0)
			return;
		totals[team] += count;
		if (!gridKnown)
			return;
		if (!InQuads(x, z)) {
			gridKnown = false;
			return;
		}
		Cell(team, x, z) += count;
	}

	void Remove(int team, int x = -1, int z = -1) {
		if (!known)
			return;
		if (team < 0 || std::size_t(team) >= totals.size() || totals[team] == 0) {
			Invalidate();
			return;
		}
		--totals[team];
		if (!gridKnown)
			return;
		if (!InQuads(x, z) || Cell(team, x, z) == 0) {
			gridKnown = false;
			return;
		}
		--Cell(team, x, z);
	}

	bool MayHaveUnits(int team) const {
		return !known || team < 0 || std::size_t(team) >= totals.size() || totals[team] != 0;
	}

	bool MayHaveUnits(int team, const QuadFieldUnitQueryRectangle& query) const {
		if (!known || team < 0 || std::size_t(team) >= totals.size())
			return true;
		if (totals[team] == 0)
			return false;
		if (!gridKnown)
			return true;
		if (query.maxX < query.minX || query.maxZ < query.minZ)
			return false;
		const int minX = std::clamp(query.minX, 0, numQuadsX - 1) / CELL;
		const int maxX = std::clamp(query.maxX, 0, numQuadsX - 1) / CELL;
		const int minZ = std::clamp(query.minZ, 0, numQuadsZ - 1) / CELL;
		const int maxZ = std::clamp(query.maxZ, 0, numQuadsZ - 1) / CELL;
		const std::size_t base = std::size_t(team) * cellsX * cellsZ;
		for (int cz = minZ; cz <= maxZ; ++cz) {
			for (int cx = minX; cx <= maxX; ++cx) {
				if (cells[base + std::size_t(cz) * cellsX + cx] != 0)
					return true;
			}
		}
		return false;
	}

private:
	bool CanAdd(int team, std::size_t count) {
		if (!known)
			return false;
		if (team < 0 || std::size_t(team) >= totals.size() || count > std::numeric_limits<std::size_t>::max() - totals[team]) {
			Invalidate();
			return false;
		}
		return true;
	}
	bool InQuads(int x, int z) const { return x >= 0 && z >= 0 && x < numQuadsX && z < numQuadsZ; }
	std::size_t& Cell(int team, int x, int z) {
		return cells[std::size_t(team) * cellsX * cellsZ + std::size_t(z / CELL) * cellsX + x / CELL];
	}

	std::vector<std::size_t> totals;
	std::vector<std::size_t> cells;
	int numQuadsX = 0, numQuadsZ = 0, cellsX = 0, cellsZ = 0;
	bool known = false;
	bool gridKnown = false;
};

class CQuadField;
struct float3;
// Derived metadata is tied to the actual field instance; an unknown owner fails open.
bool QuadFieldMayHaveUnitsInAllyTeam(const CQuadField& field, int team);
bool QuadFieldMayHaveUnitsInAllyTeam(const CQuadField& field, int team, const QuadFieldUnitQueryRectangle& query);
// Matches GetQuads' integer rectangle; false leaves unsupported inputs on the original path.
bool QuadFieldGetUnitQueryRectangle(const CQuadField& field, float3 pos, float radius, QuadFieldUnitQueryRectangle& query);

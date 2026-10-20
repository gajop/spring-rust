/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>
#include <cstddef>
#include <limits>
#include <vector>

// Counts actual allyteam-vector entries, including a unit's overlapping quads.
// Only zero is used to reject a query. Unknown or inconsistent metadata never rejects.
struct QuadFieldUnitQueryRectangle {
	int minX, minZ, maxX, maxZ;
};

class QuadFieldUnitMembership {
public:
	void Reset(std::size_t teams) { entries.assign(teams, {}); known = true; }
	void Invalidate() { known = false; }

	void Add(int team, std::size_t count = 1) {
		Entry* entry = GetForAdd(team, count);
		if (entry == nullptr || count == 0)
			return;
		entry->count += count;
		entry->boundsKnown = false;
	}

	void AddAt(int team, int x, int z, std::size_t count = 1) {
		Entry* entry = GetForAdd(team, count);
		if (entry == nullptr || count == 0)
			return;
		if (entry->count == 0) {
			entry->bounds = {x, z, x, z};
			entry->boundsKnown = true;
		} else if (entry->boundsKnown) {
			entry->bounds.minX = std::min(entry->bounds.minX, x);
			entry->bounds.minZ = std::min(entry->bounds.minZ, z);
			entry->bounds.maxX = std::max(entry->bounds.maxX, x);
			entry->bounds.maxZ = std::max(entry->bounds.maxZ, z);
		}
		entry->count += count;
	}

	void Remove(int team) {
		if (!known)
			return;
		if (team < 0 || std::size_t(team) >= entries.size() || entries[team].count == 0) {
			Invalidate();
			return;
		}
		if (--entries[team].count == 0)
			entries[team] = {};
	}

	bool MayHaveUnits(int team) const {
		return !known || team < 0 || std::size_t(team) >= entries.size() || entries[team].count != 0;
	}

	bool MayHaveUnits(int team, const QuadFieldUnitQueryRectangle& query) const {
		if (!known || team < 0 || std::size_t(team) >= entries.size())
			return true;
		const Entry& entry = entries[team];
		if (entry.count == 0)
			return false;
		if (!entry.boundsKnown)
			return true;
		if (query.maxX < query.minX || query.maxZ < query.minZ)
			return false;
		return query.maxX >= entry.bounds.minX && query.minX <= entry.bounds.maxX
			&& query.maxZ >= entry.bounds.minZ && query.minZ <= entry.bounds.maxZ;
	}

private:
	struct Entry {
		std::size_t count = 0;
		QuadFieldUnitQueryRectangle bounds = {};
		bool boundsKnown = false;
	};

	Entry* GetForAdd(int team, std::size_t count) {
		if (!known)
			return nullptr;
		if (team < 0 || std::size_t(team) >= entries.size() || count > std::numeric_limits<std::size_t>::max() - entries[team].count) {
			Invalidate();
			return nullptr;
		}
		return &entries[team];
	}

	std::vector<Entry> entries;
	bool known = false;
};

class CQuadField;
struct float3;
// Derived metadata is tied to the actual field instance; an unknown owner fails open.
bool QuadFieldMayHaveUnitsInAllyTeam(const CQuadField& field, int team);
bool QuadFieldMayHaveUnitsInAllyTeam(const CQuadField& field, int team, const QuadFieldUnitQueryRectangle& query);
// Matches GetQuads' integer rectangle; false leaves unsupported inputs on the original path.
bool QuadFieldGetUnitQueryRectangle(const CQuadField& field, float3 pos, float radius, QuadFieldUnitQueryRectangle& query);

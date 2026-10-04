/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef COLLISION_MESH_H
#define COLLISION_MESH_H

#include <cstdint>
#include <string>
#include <vector>

#include "System/float3.h"

/**
 * A triangle mesh in a model's own space, tested by segment through a bounding-volume
 * hierarchy. Attached to a collision volume, it refines that volume: the volume stays the
 * broad phase (bounding sphere, then its box early-out), and only a segment that reaches the
 * volume is tested against the triangles, so most projectiles never pay for the mesh.
 *
 * One mesh is loaded per file and shared by every object that names it.
 *
 * File format (little-endian): "RCOLMSH1", uint32 vertexCount, uint32 triangleCount,
 * float32[3 * vertexCount] positions in model space, uint32[3 * triangleCount] indices.
 */
class CollisionMesh {
public:
	struct Hit {
		float t = 0.0f;   ///< fraction along the segment, 0 at its start and 1 at its end
		float3 pos;       ///< in model space
		float3 normal;    ///< in model space, facing the segment's start
	};

	static const CollisionMesh* Load(const std::string& path);
	/// Builds from vertices and indices; empty if the input holds no triangles.
	static CollisionMesh Build(const std::vector<float3>& vertices, const std::vector<uint32_t>& indices);

	/// The first triangle the segment p0 -> p1 crosses, in model space.
	bool Intersect(const float3& p0, const float3& p1, Hit* hit) const;

	size_t NumTriangles() const { return triangles.size(); }
	size_t NumNodes() const { return nodes.size(); }
	const float3& GetMins() const { return mins; }
	const float3& GetMaxs() const { return maxs; }
	const std::string& GetPath() const { return path; }

	/// Segment tests since start, for statistics.
	static uint64_t numTests;

private:
	struct Triangle { float3 v0, e1, e2; };
	struct Node {
		float3 mins;
		float3 maxs;
		uint32_t first = 0; ///< first triangle (leaf) or right child (inner)
		uint32_t count = 0; ///< triangles in a leaf, 0 for an inner node
	};

	void BuildNode(uint32_t node, uint32_t first, uint32_t count, std::vector<float3>& centres);

	std::vector<Triangle> triangles;
	std::vector<Node> nodes;
	float3 mins;
	float3 maxs;
	std::string path;
};

#endif

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "CollisionMesh.h"

#include <algorithm>
#include <cstring>

uint64_t CollisionMesh::numTests = 0;

static constexpr uint32_t LEAF_TRIANGLES = 4;

CollisionMesh CollisionMesh::Build(const std::vector<float3>& vertices, const std::vector<uint32_t>& indices)
{
	CollisionMesh mesh;
	std::vector<float3> centres;

	mesh.triangles.reserve(indices.size() / 3);
	centres.reserve(indices.size() / 3);

	for (size_t i = 0; i + 2 < indices.size(); i += 3) {
		const float3& a = vertices[indices[i + 0]];
		const float3& b = vertices[indices[i + 1]];
		const float3& c = vertices[indices[i + 2]];
		const float3 e1 = b - a;
		const float3 e2 = c - a;

		// degenerate triangles can never be hit
		if (e1.cross(e2).SqLength() <= 0.0f)
			continue;

		mesh.triangles.push_back({a, e1, e2});
		centres.push_back((a + b + c) / 3.0f);
	}

	if (mesh.triangles.empty())
		return mesh;

	mesh.nodes.reserve(2 * mesh.triangles.size() / LEAF_TRIANGLES + 1);
	mesh.nodes.emplace_back();
	mesh.BuildNode(0, 0, uint32_t(mesh.triangles.size()), centres);
	mesh.mins = mesh.nodes[0].mins;
	mesh.maxs = mesh.nodes[0].maxs;
	return mesh;
}

void CollisionMesh::BuildNode(uint32_t node, uint32_t first, uint32_t count, std::vector<float3>& centres)
{
	float3 lo( 1e30f,  1e30f,  1e30f);
	float3 hi(-1e30f, -1e30f, -1e30f);
	float3 clo = lo;
	float3 chi = hi;

	for (uint32_t i = first; i < first + count; i++) {
		const Triangle& tri = triangles[i];
		const float3 b = tri.v0 + tri.e1;
		const float3 c = tri.v0 + tri.e2;
		lo = float3::min(lo, float3::min(tri.v0, float3::min(b, c)));
		hi = float3::max(hi, float3::max(tri.v0, float3::max(b, c)));
		clo = float3::min(clo, centres[i]);
		chi = float3::max(chi, centres[i]);
	}

	nodes[node].mins = lo;
	nodes[node].maxs = hi;

	if (count <= LEAF_TRIANGLES) {
		nodes[node].first = first;
		nodes[node].count = count;
		return;
	}

	// split the longest axis of the centres' box at its median
	const float3 extent = chi - clo;
	const int axis = (extent.x >= extent.y && extent.x >= extent.z)? 0: ((extent.y >= extent.z)? 1: 2);
	const uint32_t half = count / 2;

	std::vector<uint32_t> order(count);
	for (uint32_t i = 0; i < count; i++)
		order[i] = first + i;

	std::nth_element(order.begin(), order.begin() + half, order.end(), [&](uint32_t a, uint32_t b) {
		return (centres[a][axis] < centres[b][axis]);
	});

	std::vector<Triangle> tris(count);
	std::vector<float3> cents(count);
	for (uint32_t i = 0; i < count; i++) {
		tris[i] = triangles[order[i]];
		cents[i] = centres[order[i]];
	}
	std::copy(tris.begin(), tris.end(), triangles.begin() + first);
	std::copy(cents.begin(), cents.end(), centres.begin() + first);

	// left child follows its parent; the right child's index is stored in the parent
	const uint32_t left = uint32_t(nodes.size());
	nodes.emplace_back();
	BuildNode(left, first, half, centres);

	const uint32_t right = uint32_t(nodes.size());
	nodes.emplace_back();
	BuildNode(right, first + half, count - half, centres);

	nodes[node].first = right;
	nodes[node].count = 0;
}

static inline bool SegmentHitsBox(const float3& origin, const float3& inv, float tmax, const float3& lo, const float3& hi, float* tnear)
{
	float t0 = 0.0f;
	float t1 = tmax;

	for (int a = 0; a < 3; a++) {
		float tl = (lo[a] - origin[a]) * inv[a];
		float th = (hi[a] - origin[a]) * inv[a];
		if (tl > th)
			std::swap(tl, th);
		// NaN from 0 * inf (a segment lying in a slab's plane) must not reject the box
		if (tl > t0) t0 = tl;
		if (th < t1) t1 = th;
		if (t0 > t1)
			return false;
	}

	*tnear = t0;
	return true;
}

bool CollisionMesh::Intersect(const float3& p0, const float3& p1, Hit* hit) const
{
	numTests += 1;

	if (nodes.empty())
		return false;

	const float3 dir = p1 - p0;
	const float3 inv(
		(dir.x != 0.0f)? 1.0f / dir.x: 1e30f,
		(dir.y != 0.0f)? 1.0f / dir.y: 1e30f,
		(dir.z != 0.0f)? 1.0f / dir.z: 1e30f
	);

	float best = 1.0f;
	int bestTri = -1;

	uint32_t stack[64];
	uint32_t depth = 0;
	stack[depth++] = 0;

	while (depth > 0) {
		const Node& node = nodes[stack[--depth]];
		float tnear = 0.0f;

		if (!SegmentHitsBox(p0, inv, best, node.mins, node.maxs, &tnear))
			continue;

		if (node.count == 0) {
			if (depth + 2 > 64)
				continue;
			// left child is the next node, the right is stored
			stack[depth++] = node.first;
			stack[depth++] = uint32_t(&node - nodes.data()) + 1;
			continue;
		}

		for (uint32_t i = node.first; i < node.first + node.count; i++) {
			// Moller-Trumbore, two-sided
			const Triangle& tri = triangles[i];
			const float3 pv = dir.cross(tri.e2);
			const float det = tri.e1.dot(pv);

			if (det > -1e-12f && det < 1e-12f)
				continue;

			const float invDet = 1.0f / det;
			const float3 tv = p0 - tri.v0;
			const float u = tv.dot(pv) * invDet;

			if (u < 0.0f || u > 1.0f)
				continue;

			const float3 qv = tv.cross(tri.e1);
			const float v = dir.dot(qv) * invDet;

			if (v < 0.0f || (u + v) > 1.0f)
				continue;

			const float t = tri.e2.dot(qv) * invDet;

			if (t < 0.0f || t > best)
				continue;

			best = t;
			bestTri = int(i);
		}
	}

	if (bestTri < 0)
		return false;

	if (hit != nullptr) {
		const Triangle& tri = triangles[bestTri];
		float3 n = tri.e1.cross(tri.e2);
		if (n.dot(dir) > 0.0f)
			n = -n;

		hit->t = best;
		hit->pos = p0 + dir * best;
		hit->normal = n.SafeNormalize();
	}

	return true;
}

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "CollisionMesh.h"

#include <cstring>
#include <memory>
#include <unordered_map>

#include "System/FileSystem/FileHandler.h"
#include "System/Log/ILog.h"

const CollisionMesh* CollisionMesh::Load(const std::string& path)
{
	static std::unordered_map<std::string, std::unique_ptr<CollisionMesh>> cache;

	if (path.empty())
		return nullptr;

	const auto it = cache.find(path);
	if (it != cache.end())
		return it->second.get();

	std::unique_ptr<CollisionMesh>& slot = cache[path];
	CFileHandler file(path, SPRING_VFS_ZIP_FIRST);

	if (!file.FileExists()) {
		LOG_L(L_WARNING, "[CollisionMesh] %s not found; the volume is used alone", path.c_str());
		return nullptr;
	}

	std::string bytes;
	if (!file.LoadStringData(bytes) || bytes.size() < 16 || std::memcmp(bytes.data(), "RCOLMSH1", 8) != 0) {
		LOG_L(L_WARNING, "[CollisionMesh] %s is not a collision mesh; the volume is used alone", path.c_str());
		return nullptr;
	}

	uint32_t numVerts = 0;
	uint32_t numTris = 0;
	std::memcpy(&numVerts, bytes.data() +  8, 4);
	std::memcpy(&numTris , bytes.data() + 12, 4);

	const size_t need = 16 + size_t(numVerts) * 12 + size_t(numTris) * 12;
	if (bytes.size() < need) {
		LOG_L(L_WARNING, "[CollisionMesh] %s is truncated; the volume is used alone", path.c_str());
		return nullptr;
	}

	std::vector<float3> vertices(numVerts);
	std::vector<uint32_t> indices(size_t(numTris) * 3);

	for (uint32_t i = 0; i < numVerts; i++) {
		float xyz[3];
		std::memcpy(xyz, bytes.data() + 16 + size_t(i) * 12, 12);
		vertices[i] = float3(xyz[0], xyz[1], xyz[2]);
	}
	std::memcpy(indices.data(), bytes.data() + 16 + size_t(numVerts) * 12, indices.size() * 4);

	for (const uint32_t index: indices) {
		if (index >= numVerts) {
			LOG_L(L_WARNING, "[CollisionMesh] %s has an index out of range; the volume is used alone", path.c_str());
			return nullptr;
		}
	}

	slot = std::make_unique<CollisionMesh>(Build(vertices, indices));
	slot->path = path;

	LOG("[CollisionMesh] %s: %u triangles, %u nodes, bounds (%.1f %.1f %.1f) .. (%.1f %.1f %.1f)",
		path.c_str(), unsigned(slot->triangles.size()), unsigned(slot->nodes.size()),
		slot->mins.x, slot->mins.y, slot->mins.z, slot->maxs.x, slot->maxs.y, slot->maxs.z);

	return slot.get();
}


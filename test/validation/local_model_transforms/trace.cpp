/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// Source template for check.py; compiled after injecting each tree's real code.
#include <algorithm>
#include <array>
#include <bit>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <cstddef>
#include <cstring>
#include <cstdlib>
#include <optional>
#include <utility>
#include <vector>
#include <deque>

#include "System/Transform.hpp"
#include "System/Matrix44f.h"
#include "Rendering/Models/3DModelDefs.hpp"

#define RECOIL_DETAILED_TRACY_ZONE do {} while (0)
#define ZoneScoped do {} while (0)

namespace spring {
template<typename T> void SafeDelete(T*& pointer) { delete pointer; pointer = nullptr; }
}

constexpr float3 WORLD_TO_OBJECT_SPACE = {-1.0f, 1.0f, 1.0f};
struct CollisionVolume {
	float3 scales;
	float3 offsets;
	CollisionVolume() = default;
	CollisionVolume(const CollisionVolume* other): scales(other->scales), offsets(other->offsets) {}
	void InitBox(const float3& s, const float3& o) { scales = s; offsets = o; }
	float3 GetOffsets() const { return offsets; }
	float3 GetHScales() const { return scales * 0.5f; }
	float GetBoundingRadius() const { return 0; }
};

static uint64_t composeCount = 0;
static uint64_t previousSaveCalls = 0;
static bool tracePreviousCopies = true;
static uint64_t breadthVisits = 0;
struct S3DModelPiece {
	float3 offset;
	float scale = 1.0f;
	float3 mins{-1, -2, -3};
	float3 maxs{ 3,  4,  5};
	std::optional<Transform> bakedTransform;
	std::vector<S3DModelPiece*> children;
	CollisionVolume volume;
	const CollisionVolume* GetCollisionVolume() const { return &volume; }
	float3 GetEmitPos() const { return {2, 3, 4}; }
	float3 GetEmitDir() const { return {1, 0.5f, -2}; }
	bool HasGeometryData() const { return true; }
	Transform ComposeTransform(const float3& t, const float3& r, float s) const;
};

// INSERT_PIECE_HEADER

struct S3DModel {
	size_t numPieces = 0;
	std::vector<S3DModelPiece*> pieces;
	S3DModelPiece* GetPiece(size_t i) const { return pieces[i]; }
	S3DModelPiece* GetRootPiece() const { return pieces[0]; }
};

struct LuaObjectMaterialData {
	bool Enabled() const { return false; }
	uint32_t GetCurrentLOD() const { return 0; }
};

// INSERT_LOCAL_HEADER

// INSERT_METHODS

static void Print(float value) { std::printf(" %08x", std::bit_cast<uint32_t>(value)); }
static void Print(const float3& value) { Print(value.x); Print(value.y); Print(value.z); }
static void Print(const Transform& value) {
	Print(value.r.x); Print(value.r.y); Print(value.r.z); Print(value.r.r);
	Print(value.t); Print(value.s);
}
static void Print(const CMatrix44f& value) { for (float v: value.m) Print(v); }

static size_t traceSteps = 0;
static void Snapshot(const char* label, LocalModel& model)
{
	// INSERT_INVARIANT
	std::printf("%zu %s %d", traceSteps++, label, model.needsBoundariesRecalc);
	Print(model.boundingVolume.scales); Print(model.boundingVolume.offsets);
	for (const auto& piece: model.pieces) {
		std::printf(" | %d %d %d %d %d %d %d %d", piece.dirty,
			piece.wasUpdated[0], piece.wasUpdated[1], piece.noInterpolation[0],
			piece.noInterpolation[1], piece.noInterpolation[2],
			piece.blockScriptAnims, piece.scriptSetVisible);
		Print(piece.GetPosition()); Print(piece.GetRotation()); Print(piece.GetScaling());
		Print(piece.pieceSpaceTra); Print(piece.GetModelSpaceTransformRaw());
		Print(piece.modelSpaceMat); Print(piece.prevModelSpaceTra);
		Print(piece.GetEffectivePrevModelSpaceTransform());
	}
	std::puts("");
}

static void Save(LocalModel& model) {
	// INSERT_SAVE
}
static void Reset(LocalModel& model) {
	for (auto& piece: model.pieces) piece.ResetWasUpdated();
}
static void BreadthFirst(LocalModel& model) {
	LocalModelPiece* rootPiece = &model.pieces[0];
	// INSERT_BREADTH_FIRST
}

int main(int argc, char** argv)
{
	if (argc == 2 && std::strcmp(argv[1], "--layout") == 0) {
		std::printf("sizeof %zu alignof %zu\n", sizeof(LocalModelPiece), alignof(LocalModelPiece));
		#define PRINT_OFFSET(field) std::printf(#field " %zu\n", offsetof(LocalModelPiece, field))
		PRINT_OFFSET(modelSpaceMat); PRINT_OFFSET(pieceSpaceTra); PRINT_OFFSET(modelSpaceTra);
		PRINT_OFFSET(pos); PRINT_OFFSET(rot); PRINT_OFFSET(scale); PRINT_OFFSET(noInterpolation);
		PRINT_OFFSET(dirty); PRINT_OFFSET(prevModelSpaceTra); PRINT_OFFSET(colvol);
		PRINT_OFFSET(dir); PRINT_OFFSET(wasUpdated); PRINT_OFFSET(scriptSetVisible);
		PRINT_OFFSET(blockScriptAnims); PRINT_OFFSET(lmodelPieceIndex); PRINT_OFFSET(scriptPieceIndex);
		PRINT_OFFSET(children); PRINT_OFFSET(parent); PRINT_OFFSET(lodDispLists);
		PRINT_OFFSET(original); PRINT_OFFSET(localModel);
		#undef PRINT_OFFSET
		return 0;
	}
	streflop::streflop_init<streflop::Simple>();
	std::array<S3DModelPiece, 4> originals;
	std::array<S3DModelPiece, 4> remapped;
	for (size_t i = 0; i < originals.size(); ++i) {
		originals[i].offset = float3(i * 3.0f, i * -2.0f, i * 0.75f);
		originals[i].scale = 0.75f + i * 0.1f;
		originals[i].bakedTransform = Transform(CQuaternion::FromEulerYPR({0.1f, 0.2f, -0.3f}), {2, -3, 4});
		remapped[i] = originals[i];
		remapped[i].bakedTransform = Transform(CQuaternion::FromEulerYPR({-0.4f, 0.5f, 0.6f}), {-7, 8, 9}, 1.125f);
	}
	originals[0].children = {&originals[1], &originals[3]};
	originals[1].children = {&originals[2]};
	S3DModel source{4, {&originals[0], &originals[1], &originals[2], &originals[3]}};
	S3DModel replacement{4, {&remapped[0], &remapped[1], &remapped[2], &remapped[3]}};
	LocalModel model;
	model.SetModel(&source);
	Snapshot("constructed", model);
	auto& root = model.pieces[0];
	auto& child = model.pieces[1];
	auto& leaf = model.pieces[2];
	auto& sibling = model.pieces[3];
	for (int frame = 0; frame < 32; ++frame) {
		Save(model); Reset(model);
		root.SetPosition({frame * 0.125f, -frame * 0.25f, frame * 0.375f});
		root.SetRotation({frame * 0.01f, frame * -0.02f, frame * 0.03f});
		BreadthFirst(model);
		Snapshot("animated-root", model);
	}
	for (int frame = 0; frame < 4; ++frame) {
		Save(model); Reset(model); BreadthFirst(model);
		Snapshot("settled", model);
	}
	const uint64_t beforeIdle = previousSaveCalls;
	for (int frame = 0; frame < 64; ++frame) { Save(model); Reset(model); }
	// INSERT_IDLE_CHECK
	Snapshot("aggregate-idle-64", model);
	// Same-value writes (and +/- zero) must retain their original latent effect
	// when a later ancestor update or explicit invalidation composes this piece.
	child.SetRotation({0, 0, 0}); child.GetModelSpaceMatrix();
	child.SetRotation({-0.0f, 0.0f, -0.0f});
	Snapshot("signed-zero-clean", model);
	root.SetPosition({100, 200, -300});
	leaf.GetModelSpaceMatrix();
	Snapshot("lazy-descendant", model);
	child.SetScaling(0.0f); child.SetScaling(-0.0f);
	child.SetPosition({-0.0f, 0.0f, -0.0f});
	BreadthFirst(model); Snapshot("signed-zero-dirty", model);
	child.SetScaling(1.0f); BreadthFirst(model);
	// Blocked setters are ignored; pending accepted input survives block/unblock.
	child.SetRotation({0.2f, -0.3f, 0.4f});
	CMatrix44f block;
	if (child.SetPieceSpaceMatrix(block)) child.SetDirty();
	child.SetPosition({900, 800, 700}); child.SetScaling(0.125f);
	BreadthFirst(model); Snapshot("blocked", model);
	CMatrix44f unblock;
	unblock.Scale(2.0f);
	child.SetPieceSpaceMatrix(unblock);
	root.SetDirty(); BreadthFirst(model); Snapshot("unblocked", model);
	child.blockScriptAnims = true;
	root.SetRotation({0.1f, 0.2f, 0.3f});
	child.UpdateParentMatricesRec();
	child.blockScriptAnims = false;
	child.SetDirtyRaw(true); child.UpdateParentMatricesRec();
	Snapshot("raw-and-direct", model);
	// Exercise both explicit model-space writers and renderer interpolation bits.
	child.UpdatePieceSpaceTransform();
	child.UpdateModelSpaceTransform(Transform({-10, 20, -30}));
	Save(model); child.UpdateModelSpaceTransform(&root);
	child.SetRotationNoInterpolation(true); child.SetScalingNoInterpolation(true);
	Snapshot("interpolation-current", model);
	child.SetPositionNoInterpolation(true); child.SetPosition({7, 8, 9});
	Snapshot("interpolation-lazy", model);
	Save(model); Reset(model); Snapshot("interpolation-reset", model);
	Save(model); Reset(model); Snapshot("interpolation-settle", model);
	// Restore persisted state as creg does, then run the production post-load hook.
	LocalModelPiece persisted = child;
	child.SetRotation({0.7f, 0.8f, 0.9f}); BreadthFirst(model); Save(model);
	child = persisted; persisted.colvol = nullptr;
	child.PostLoad();
	// INSERT_MODEL_POSTLOAD
	Snapshot("post-load", model);
	model.SetModel(&replacement, false);
	Snapshot("model-remap-clean", model);
	root.SetPosition({-100, 200, -300}); BreadthFirst(model);
	Snapshot("model-remap-dirty", model);
	// Reparent without changing existing dirty semantics, then explicit recalc.
	sibling.SetRotation({0.7f, 0.8f, 0.9f});
	root.RemoveChild(&sibling); child.AddChild(&sibling); sibling.SetParent(&child);
	BreadthFirst(model); Snapshot("reparented-dirty-leaf", model);
	root.SetDirty(); root.UpdateChildTransformRec(false);
	Snapshot("reparented", model);
	root.UpdateChildTransformRec(true); Snapshot("forced-recursive", model);
	// Quiet/signaling NaNs and payloads exercise conservative accepted writes.
	for (uint32_t bits: {0x7fc00001u, 0x7fc00102u, 0xffc00003u, 0x7f800001u}) {
		const float nan = std::bit_cast<float>(bits);
		child.SetRotation({nan, -0.0f, 0.0f}); BreadthFirst(model);
		Save(model); Snapshot("nan-rotation", model);
		child.SetScaling(nan); child.GetModelSpaceMatrix();
		Snapshot("nan-scaling", model);
		child.SetPosition({nan, nan, nan}); root.SetDirty();
		leaf.GetModelSpaceTransform(); Save(model); Snapshot("nan-position", model);
		child.SetRotation({0, 0, 0}); child.SetScaling(1); child.SetPosition({1, 2, 3});
		root.UpdateChildTransformRec(false); Save(model);
	}
	// Broad, deep and balanced hierarchies exercise ancestor/descendant updates
	// with owner-wide scan suppression independently from the original fixture.
	for (int hierarchy = 0; hierarchy < 3; ++hierarchy) {
		std::array<S3DModelPiece, 16> nodes;
		S3DModel definition;
		definition.numPieces = nodes.size();
		for (size_t index = 0; index < nodes.size(); ++index) {
			nodes[index].offset = float3(index * 2.0f, index * -0.125f, index * 0.75f);
			if (index != 0) {
				size_t parentIndex = hierarchy == 0 ? 0 : (hierarchy == 1 ? index - 1 : (index - 1) / 2);
				nodes[parentIndex].children.push_back(&nodes[index]);
			}
		}
		auto flatten = [&](auto&& self, S3DModelPiece* node) -> void {
			definition.pieces.push_back(node);
			for (auto* childNode: node->children) self(self, childNode);
		};
		flatten(flatten, &nodes[0]);
		LocalModel varied;
		varied.SetModel(&definition);
		for (int frame = 0; frame < 12; ++frame) {
			Save(varied); Reset(varied);
			varied.pieces[0].SetRotation({frame * 0.02f, frame * -0.03f, frame * 0.01f});
			varied.pieces.back().SetPosition({frame * 0.1f, frame * -0.1f, frame * 0.2f});
			if (frame & 1) BreadthFirst(varied);
			else varied.pieces.back().GetModelSpaceMatrix();
			Snapshot("varied-hierarchy", varied);
		}
		for (int frame = 0; frame < 4; ++frame) { Save(varied); Reset(varied); }
		Snapshot("varied-hierarchy-settled", varied);
		const uint64_t leafPreviousStart = previousSaveCalls;
		for (int frame = 0; frame < 64; ++frame) {
			Save(varied); Reset(varied);
			varied.pieces.back().SetRotation({0, frame * 0.02f, 0});
			BreadthFirst(varied);
			Snapshot("active-leaf", varied);
		}
		std::fprintf(stderr, "active_leaf_%d_save_visits=%llu\n", hierarchy,
			static_cast<unsigned long long>(previousSaveCalls - leafPreviousStart));
	}
	// A stale logical piece ID cannot affect the physical-array interval.
	Save(model); Save(model);
	const uint32_t oldIndex = leaf.GetLModelPieceIndex();
	leaf.SetLModelPieceIndex(0xfffffff0u);
	leaf.UpdateModelSpaceTransform(Transform({31, -17, 23}));
	Save(model);
	leaf.SetLModelPieceIndex(oldIndex);
	Snapshot("logical-id-independent", model);
	// An owner marker from an object outside its pieces array must conservatively
	// request the original full sweep instead of indexing an unrelated pointer.
	{
		LocalModelPiece outside(&originals[0]);
		outside.SetDirtyRaw(false);
		outside.SetLocalModel(&model);
		const uint64_t begin = previousSaveCalls;
		Save(model);
		std::printf("unknown-member-save-visits %llu\n", static_cast<unsigned long long>(previousSaveCalls - begin));
		outside.SetLocalModel(nullptr);
	}
	// Explicit bound check: no historic maximum-model-size assumption is used.
	// More than 65536 pieces cannot fit the compact interval and must fall back.
	{
		tracePreviousCopies = false;
		S3DModelPiece trivial;
		LocalModel oversized;
		oversized.pieces.reserve(65537);
		for (size_t index = 0; index < 65537; ++index) {
			auto& piece = oversized.pieces.emplace_back(&trivial);
			piece.SetLocalModel(&oversized);
			piece.SetDirtyRaw(false);
		}
		Save(oversized);
		oversized.pieces.back().UpdateModelSpaceTransform(Transform({-11, 13, 17}));
		const uint64_t begin = previousSaveCalls;
		Save(oversized);
		std::printf("oversized-save-visits %llu", static_cast<unsigned long long>(previousSaveCalls - begin));
		Print(oversized.pieces.back().GetModelSpaceTransformRaw());
		Print(oversized.pieces.back().GetEffectivePrevModelSpaceTransform());
		std::puts("");
		oversized.pieces.pop_back(); // Now exactly 65536 entries: last index fits.
		oversized.pieces.back().UpdateModelSpaceTransform(Transform({19, -23, 29}));
		const uint64_t boundaryBegin = previousSaveCalls;
		Save(oversized);
		// INSERT_BOUNDARY_INTERVAL_CHECK
		std::fprintf(stderr, "maximum_interval_index_visits=%llu\n",
			static_cast<unsigned long long>(previousSaveCalls - boundaryBegin));
		std::printf("maximum-interval-index-state");
		Print(oversized.pieces.back().GetModelSpaceTransformRaw());
		Print(oversized.pieces.back().GetEffectivePrevModelSpaceTransform());
		std::puts("");
		tracePreviousCopies = true;
	}
	// Reproducible interleaving catches combinations between all public paths.
	uint32_t random = 0x58ea1427;
	for (int step = 0; step < 4096; ++step) {
		random = random * 1664525u + 1013904223u;
		auto& piece = model.pieces[(random >> 12) % 4];
		const float value = static_cast<int>((random >> 16) & 255) * 0.001f;
		switch (random % 21) {
			case 0: Save(model); break;
			case 1: Reset(model); break;
			case 2: piece.SetPosition({value, -value, value}); break;
			case 3: piece.SetRotation({value, value, -value}); break;
			case 4: piece.SetScaling(value); break;
			case 5: piece.SetPosition(piece.GetPosition()); break;
			case 6: piece.SetRotation(piece.GetRotation()); break;
			case 7: piece.SetDirty(); break;
			case 8: piece.SetDirtyRaw(random & 0x8000); break;
			case 9: piece.GetModelSpaceTransform(); break;
			case 10: piece.GetModelSpaceMatrix(); break;
			case 11: BreadthFirst(model); break;
			case 12: root.UpdateChildTransformRec(false); break;
			case 13: root.UpdateChildTransformRec(true); break;
			case 14: piece.UpdateParentMatricesRec(); break;
			case 15: piece.UpdatePieceSpaceTransform(); piece.UpdateModelSpaceTransform(piece.parent); break;
			case 16: piece.SetScriptVisible(random & 0x8000); break;
			case 17: piece.SetPositionNoInterpolation(random & 0x8000); break;
			case 18: piece.blockScriptAnims = random & 0x8000; break;
			case 19: piece.PostLoad(); break;
			case 20: { float3 pos, dir; piece.GetEmitDirPos(pos, dir); Print(pos); Print(dir); std::puts(""); } break;
		}
		Snapshot("interleaved", model);
	}
	std::fprintf(stderr, "compose_count=%llu previous_save_calls=%llu breadth_visits=%llu trace_steps=%zu\n", static_cast<unsigned long long>(composeCount), static_cast<unsigned long long>(previousSaveCalls), static_cast<unsigned long long>(breadthVisits), traceSteps);
}

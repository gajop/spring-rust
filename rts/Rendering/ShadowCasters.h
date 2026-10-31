/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

class CSolidObject;

// Which models cast in the shadow pass, and at which level of detail. Each part is a config key,
// off by default, read once per shadow pass:
//
// - ShadowCasterLOD: a Lua-material model's level is chosen by the world length one shadow-map
//   texel covers (times LODScaleShadow), or by the length one screen pixel covers at the model
//   (times LODScale) where that is coarser: a silhouette finer than either is lost in the map or
//   on screen. Off: the shadow camera's own projected distance, which for an orthographic light
//   camera centred on the ground is zero or negative for anything above it (the finest level).
// - ShadowCasterMinTexels: a model whose diameter spans fewer texels than this does not cast.
// - ShadowCasterReceivers: a model casts only if it is in the player's view itself, or if its
//   shadow, swept away from the light, reaches a unit or feature in the player's view. For a
//   game where models are the only receivers: the map and anything else reading the shadow map
//   lose the shadows of models left out.
//
// SPRING_SHADOW_CASTER_STATS=1 logs, every 300 shadow passes, how many casters were left out
// and at which level the rest were drawn.
namespace ShadowCasters {
	// At the start of the shadow pass, before the model drawers: reads the keys and, with
	// ShadowCasterReceivers, gathers the units and features in the player's view.
	void Begin();
	void End();

	// Whether `obj` (flagged for the shadow pass) casts at all.
	bool Wanted(const CSolidObject* obj);

	// With ShadowCasterLOD, the length per texel that chooses `obj`'s level; otherwise < 0.
	float LevelLength(const CSolidObject* obj, int objType);

	// Counts the level a Lua-material caster was drawn at (stats only).
	void CountLevel(unsigned int level);
}

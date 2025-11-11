# Rust Native API Port Status

This document tracks the progress of porting all Lua APIs to native Rust FFI interfaces.

## Overall Progress

- **Total APIs**: 45+ (split into modular sub-APIs)
- **Headers Complete**: 15/45 (33%)
- **Implementations Complete**: 1/45 (2%)

## Headers Generated (15/45)

1. ✅ CommonTypes.h - Shared types (Float2/3/4, arrays, results)
2. ✅ MetalMap.h - Metal map API
3. ✅ PathFinder.h - Pathfinding API
4. ✅ RulesParams.h - Rules parameters
5. ✅ MathExtra.h - Math utilities
6. ✅ Game.h - Game state queries
7. ✅ Terrain.h - Terrain/ground queries
8. ✅ Teams.h - Teams/Players/AllyTeams
9. ✅ UnitsQuery.h - Unit lists and spatial queries
10. ✅ UnitsInfo.h - Unit properties
11. ✅ UnitsWeapons.h - Unit weapons
12. ✅ UnitsCommands.h - Unit command queues
13. ✅ Features.h - Feature queries
14. ✅ Projectiles.h - Projectile queries

## Common Components

| Component | Status | Notes |
|-----------|--------|-------|
| Error handling base (Common.h) | ✅ Done | Basic Error struct exists |
| CommonTypes.h | ✅ Done | Float2/3/4, arrays, result types |
| Extended error codes | ✅ Done | CommonErrorCode enum in CommonTypes.h |
| String return helpers | ✅ Done | StringResult, StringArray types |
| Array return helpers | ✅ Done | FloatArray, Int32Array, etc. |
| Vector helpers (float3, float4) | ✅ Done | Float2/3/4 structs |
| Callback infrastructure | ⬜ Todo | For async operations |
| Memory pool for results | ⬜ Todo | Efficient result allocation |

## API Port Status

### Core Game State (Synced)

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaMetalMap | 4 | ✅ Done | ✅ Done | High | Reference implementation |
| LuaSyncedRead | ~300 | ✅ Partial | ⬜ Todo | High | Split into: Game, Terrain, Teams, Units*, Features, Projectiles |
| LuaSyncedCtrl | ~150 | ⬜ Todo | ⬜ Todo | High | Massive - need to split |
| LuaSyncedMoveCtrl | ~20 | ⬜ Todo | ⬜ Todo | Medium | Unit movement control |
| LuaRulesParams | ~4 | ✅ Done | ⬜ Todo | Medium | Custom parameters |
| LuaPathFinder | 6 | ✅ Done | ⬜ Todo | Medium | Pathfinding |
| LuaMathExtra | ~15 | ✅ Done | ⬜ Todo | Medium | Math utilities |
| LuaSyncedTable | ~3 | ⬜ Todo | ⬜ Todo | Low | Synced data tables |

### Core UI State (Unsynced)

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaUnsyncedRead | ~220 | ⬜ Todo | ⬜ Todo | High | UI/rendering queries |
| LuaUnsyncedCtrl | ~230 | ⬜ Todo | ⬜ Todo | High | UI/rendering control |

### Definition APIs

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaUnitDefs | ~50 | ⬜ Todo | ⬜ Todo | High | Unit definitions |
| LuaFeatureDefs | ~20 | ⬜ Todo | ⬜ Todo | Medium | Feature definitions |
| LuaWeaponDefs | ~30 | ⬜ Todo | ⬜ Todo | Medium | Weapon definitions |

### Pathfinding

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaPathFinder | 6 | ⬜ Todo | ⬜ Todo | Medium | Pathfinding API |

### Rendering APIs

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaOpenGL | ~600 | ⬜ Todo | ⬜ Todo | Low | Massive OpenGL API - maybe skip? |
| LuaShaders | ~15 | ⬜ Todo | ⬜ Todo | Low | Shader management |
| LuaFBOs | ~20 | ⬜ Todo | ⬜ Todo | Low | Framebuffer objects |
| LuaTextures | ~15 | ⬜ Todo | ⬜ Todo | Low | Texture management |
| LuaVBO | ~10 | ⬜ Todo | ⬜ Todo | Low | Vertex buffers |
| LuaVBOImpl | ~30 | ⬜ Todo | ⬜ Todo | Low | VBO implementation |
| LuaVAO | ~5 | ⬜ Todo | ⬜ Todo | Low | Vertex arrays |
| LuaVAOImpl | ~20 | ⬜ Todo | ⬜ Todo | Low | VAO implementation |
| LuaRBOs | ~8 | ⬜ Todo | ⬜ Todo | Low | Renderbuffer objects |
| LuaMaterial | ~20 | ⬜ Todo | ⬜ Todo | Low | Material system |
| LuaObjectRendering | ~15 | ⬜ Todo | ⬜ Todo | Low | Custom rendering |
| LuaFonts | ~20 | ⬜ Todo | ⬜ Todo | Low | Font rendering |
| LuaOpenGLUtils | ~40 | ⬜ Todo | ⬜ Todo | Low | OpenGL utilities |
| LuaAtlasTextures | ~5 | ⬜ Todo | ⬜ Todo | Low | Atlas textures |
| LuaDisplayLists | ~5 | ⬜ Todo | ⬜ Todo | Low | Display lists (deprecated) |

### File System APIs

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaVFS | ~30 | ⬜ Todo | ⬜ Todo | Medium | Virtual file system |
| LuaVFSDownload | ~8 | ⬜ Todo | ⬜ Todo | Low | Download management |
| LuaArchive | ~10 | ⬜ Todo | ⬜ Todo | Low | Archive access |
| LuaZip | ~12 | ⬜ Todo | ⬜ Todo | Low | ZIP operations |
| LuaIO | ~8 | ⬜ Todo | ⬜ Todo | Low | General I/O |

### Utility & Math

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaMathExtra | ~15 | ⬜ Todo | ⬜ Todo | Medium | Extended math |
| LuaUtils | ~40 | ⬜ Todo | ⬜ Todo | Medium | General utilities |
| LuaEncoding | ~5 | ⬜ Todo | ⬜ Todo | Low | String encoding |
| LuaTableExtra | ~3 | ⬜ Todo | ⬜ Todo | Low | Table utilities |

### Constants

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaConstCMD | ~50 | ⬜ Todo | ⬜ Todo | High | Command constants |
| LuaConstCMDTYPE | ~10 | ⬜ Todo | ⬜ Todo | High | Command type constants |
| LuaConstCOB | ~30 | ⬜ Todo | ⬜ Todo | Medium | COB constants |
| LuaConstEngine | ~20 | ⬜ Todo | ⬜ Todo | Medium | Engine constants |
| LuaConstGame | ~50 | ⬜ Todo | ⬜ Todo | Medium | Game constants |
| LuaConstGL | ~200 | ⬜ Todo | ⬜ Todo | Low | OpenGL constants |
| LuaConstPlatform | ~15 | ⬜ Todo | ⬜ Todo | Low | Platform constants |

### Specialized

| API File | Functions | Header | Implementation | Priority | Notes |
|----------|-----------|--------|----------------|----------|-------|
| LuaInterCall | ~4 | ⬜ Todo | ⬜ Todo | Low | Inter-Lua communication |
| LuaParser | ~20 | ⬜ Todo | ⬜ Todo | Low | Lua table parser |
| LuaGarbageCollectCtrl | ~3 | ⬜ Todo | ⬜ Todo | Low | GC control |

## Priority Levels

- **High**: Core gameplay functionality needed for AI/logic
- **Medium**: Useful but not critical for basic functionality
- **Low**: Nice-to-have, rendering/UI focused, or rarely used

## Notes

- LuaSyncedRead and LuaSyncedCtrl are massive and should be split into logical sub-APIs:
  - Units API
  - Features API
  - Projectiles API
  - Terrain API
  - Teams/Players API
  - Commands API
  - etc.
- Rendering APIs (LuaOpenGL, etc.) may be skipped or deprioritized for Rust AI development
- Constants can be auto-generated from existing Lua const files

# Rust Native API Port Status

This document tracks the progress of porting all Lua APIs to native Rust FFI interfaces.

## Overall Progress

- **Total API Headers**: 34 (generated from 45+ Lua API files)
- **Headers Complete**: 34/34 (100%) ✅
- **Implementations Complete**: 7/34 (21%)
  - Game.cpp ✅ (582 lines)
  - Memory.cpp ✅ (82 lines)
  - Player.cpp ✅ (268 lines)
  - Teams.cpp ✅ (709 lines)
  - MathExtra.cpp ✅ (137 lines)
  - Terrain.cpp ✅ (264 lines)
  - MetalMap.cpp ✅ (existing)
- **Total Lines of Code**:
  - Headers: 3,727 lines
  - Implementations: ~2,042 lines (new)

## All Generated Headers (34 files)

### Infrastructure (2 files)
1. ✅ **Common.h** - Base Error struct (already existed)
2. ✅ **CommonTypes.h** - Foundation types (Float2/3/4, arrays, result wrappers, error codes)

### Core Game State - Synced Read (11 files)
3. ✅ **Game.h** - Game state, frame, time, options, environmental
4. ✅ **Terrain.h** - Height maps, normals, blocking, terrain types, water
5. ✅ **Teams.h** - Teams, players, ally teams, resources, stats
6. ✅ **UnitsQuery.h** - Unit lists, spatial queries, filtering
7. ✅ **UnitsInfo.h** - Unit properties (~80 functions: health, position, state, sensors)
8. ✅ **UnitsWeapons.h** - Weapon state, targeting, damages
9. ✅ **UnitsCommands.h** - Command queues, factory queues, cmd descriptions
10. ✅ **UnitsPieces.h** - Model piece queries and manipulation
11. ✅ **Features.h** - Feature queries (wrecks, rocks, trees)
12. ✅ **Projectiles.h** - Projectile queries (bullets, missiles, beams)
13. ✅ **LOS.h** - Line of sight, radar, visibility queries

### Definitions - Static Game Data (3 files)
14. ✅ **UnitDefs.h** - Unit definition queries
15. ✅ **FeatureDefs.h** - Feature definition queries
16. ✅ **WeaponDefs.h** - Weapon definition queries

### Specialized Systems (5 files)
17. ✅ **PathFinder.h** - Pathfinding requests, node cost overlays
18. ✅ **RulesParams.h** - Custom parameters with LOS visibility
19. ✅ **MathExtra.h** - Extended math (hypot, clamp, smoothstep, bitwise)
20. ✅ **MetalMap.h** - Metal resource queries/modifications (✅ HAS IMPLEMENTATION)
21. ✅ **MoveCtrl.h** - Movement control and queries

### Control APIs - Synced Write (1 file, split into sub-APIs)
22. ✅ **SyncedCtrl.h** - Game modifications (team, unit, feature, terrain, projectile control)

### UI/Rendering - Unsynced (4 files)
23. ✅ **Camera.h** - Camera queries and control
24. ✅ **Input.h** - Mouse and keyboard input
25. ✅ **Display.h** - Display, window, rendering state
26. ✅ **Selection.h** - Unit selection queries and control

### System/IO (4 files)
27. ✅ **VFS.h** - Virtual file system access
28. ✅ **Sound.h** - Sound playback control
29. ✅ **Messages.h** - Chat, console, messaging
30. ✅ **Config.h** - Engine configuration

### Utilities (4 files)
31. ✅ **Tracing.h** - Ray tracing and collision testing
32. ✅ **Utils.h** - Build testing, CEG, utility functions
33. ✅ **Player.h** - Local player info, roster, traffic, stats
34. ✅ **Constants.h** - Command IDs, fire states, unit categories, COB constants

## Implementation Status

| API Header | Implementation | Priority | Notes |
|-----------|----------------|----------|-------|
| MetalMap.h | ✅ Complete | High | Reference implementation (MetalMap.cpp) |
| CommonTypes.h | N/A | - | Header-only types |
| Constants.h | N/A | - | Header-only constants |
| Memory.h | ✅ Complete | High | Memory management for FFI arrays |
| Game.h | ✅ Complete | High | Core game queries (20+ functions) |
| Terrain.h | ✅ Complete | High | Terrain queries (14 functions) |
| Teams.h | ✅ Complete | High | Team/player queries (20 functions) |
| Player.h | ✅ Complete | High | Player roster, traffic, stats (7 functions) |
| MathExtra.h | ✅ Complete | Medium | Extended math (14 functions) |
| UnitsQuery.h | ⬜ Todo | High | Unit spatial queries |
| UnitsInfo.h | ⬜ Todo | High | Unit property queries (~80 functions) |
| UnitsCommands.h | ⬜ Todo | Medium | Command queue queries |
| UnitsWeapons.h | ⬜ Todo | Medium | Weapon queries |
| Features.h | ⬜ Todo | Medium | Feature queries |
| Projectiles.h | ⬜ Todo | Medium | Projectile queries |
| UnitDefs.h | ⬜ Todo | High | Static unit data |
| PathFinder.h | ⬜ Todo | Medium | Pathfinding |
| RulesParams.h | ⬜ Todo | Medium | Custom params |
| SyncedCtrl.h | ⬜ Todo | High | Game modifications |
| (Others) | ⬜ Todo | Low-Med | UI, sound, config, etc. |

## Design Patterns Established

All headers follow consistent C FFI conventions:

1. **Error Handling**: Explicit `Error*` in result structs
2. **Memory Management**: No hidden allocations in API signatures
3. **Ownership**: Clear semantics (caller owns arrays, API owns strings)
4. **Modularity**: Organized by domain (units split into Query/Info/Weapons/Commands)
5. **Type Safety**: Strongly typed enums and structs
6. **Compatibility**: C-compatible (`extern "C"`) for Rust FFI

## Next Steps

1. **Implement high-priority .cpp files**:
   - Game.cpp
   - Terrain.cpp (merge with existing NativeInterface)
   - Teams.cpp
   - UnitsQuery.cpp
   - UnitsInfo.cpp
   - UnitDefs.cpp

2. **Integrate with NativeInterface**:
   - Add new API pointers to NativeInterface struct
   - Export global API instances

3. **Testing**:
   - Create Rust test harness
   - Validate API behavior matches Lua equivalents
   - Performance benchmarks

## File Organization

```
rts/Game/Rust/api/
├── Common.h                 # Base error type
├── CommonTypes.h            # Shared types
├── Constants.h              # Game constants
├── Game.h                   # Core game state
├── Terrain.h                # Terrain queries
├── Teams.h                  # Teams/players
├── Units*.h (5 files)       # Unit APIs
├── Features.h               # Features
├── Projectiles.h            # Projectiles
├── *Defs.h (3 files)        # Definitions
├── PathFinder.h             # Pathfinding
├── RulesParams.h            # Custom params
├── MathExtra.h              # Math utilities
├── MetalMap.h + .cpp        # Metal map (complete)
├── MoveCtrl.h               # Movement
├── LOS.h                    # Line of sight
├── SyncedCtrl.h             # Control APIs
├── Camera.h                 # Camera
├── Input.h                  # Input
├── Display.h                # Display
├── Selection.h              # Selection
├── VFS.h                    # File system
├── Sound.h                  # Sound
├── Messages.h               # Messaging
├── Config.h                 # Configuration
├── Tracing.h                # Ray tracing
├── Utils.h                  # Utilities
└── Player.h                 # Player info
```

Total: 34 header files, 3,727 lines of C FFI interface definitions

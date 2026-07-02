# spring-native

Safe Rust bindings for the Spring RTS Engine Native Interface.

This crate provides type-safe, idiomatic Rust wrappers around the C FFI defined in `rts/NativeInterface`. All code is automatically generated at build time from the C++ header files, ensuring the bindings stay synchronized with the engine.

## Features

- **Automatic code generation** - Bindings are generated from C++ headers using libclang
- **Type safety** - Query structs are flattened into strongly-typed function parameters
- **Idiomatic Rust** - Uses `Result<T, Error>`, `&[T]` slices, and `&str` strings
- **Zero unsafe in user code** - All FFI unsafe code is encapsulated in the generated wrappers
- **32 API modules** - Complete coverage of all Spring Native Interface APIs

## Building Native Modules

To create a Spring native module in Rust, you need two things:

1. **Implement the `NativeModule` trait** with your game logic
2. **Export FFI callbacks** for Spring to call

### Quick Example

**Cargo.toml:**
```toml
[package]
name = "my-spring-module"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Required: builds a dynamic library

[dependencies]
spring-native = "0.1"
```

**src/lib.rs:**
```rust
use spring_native::prelude::*;

// Your module implementation - just a normal Rust struct!
// Store the interface to call Spring APIs from your callbacks
struct MyModule {
    interface: NativeInterfaceRef,
    unit_count: u32,
}

impl NativeModule for MyModule {
    fn new(interface: NativeInterfaceRef) -> Self {
        println!("Module loaded!");
        MyModule {
            interface,
            unit_count: 0,
        }
    }

    fn game_start(&mut self) -> Result<(), Error> {
        println!("Game started!");

        // Call Spring APIs from callbacks!
        if let Some(game) = self.interface.game() {
            let frame = game.get_game_frame()?;
            println!("Starting at frame {}", frame);
        }
        Ok(())
    }

    fn unit_created(&mut self, unit_id: i32, builder_id: i32) -> Result<(), Error> {
        self.unit_count += 1;
        println!("Unit {} created (total: {})", unit_id, self.unit_count);

        // Query unit info using the Spring API
        if let Some(units_info) = self.interface.units_info() {
            let pos = units_info.get_unit_position(unit_id)?;
            println!("  Position: ({:.1}, {:.1}, {:.1})", pos.x, pos.y, pos.z);
        }
        Ok(())
    }

    fn unit_destroyed(&mut self, unit_id: i32, _attacker_id: i32) -> Result<(), Error> {
        self.unit_count -= 1;
        println!("Unit {} destroyed (remaining: {})", unit_id, self.unit_count);
        Ok(())
    }
}

// That's it! This one line exports everything Spring needs:
spring_native::export_module!(MyModule);
```

**Notice:**
- ✅ No unsafe code
- ✅ No FFI types
- ✅ No pointer management
- ✅ Bidirectional: receive callbacks AND call Spring APIs
- ✅ Just pure Rust!

The `export_module!` macro generates all 30+ callback exports, panic handler setup, and type conversions automatically.

See `examples/echo_module.rs` for a complete working example.

**For a detailed guide**, see [BUILDING_MODULES.md](BUILDING_MODULES.md).

## Usage (Calling Spring APIs)

Once your module is loaded, you can use the Spring APIs from within your callbacks:

```rust
use spring_native::{NativeInterfaceRef, Error};

// Get the interface from the engine (provided by Spring at runtime)
let interface = unsafe { NativeInterfaceRef::from_ptr(native_interface_ptr) }
    .ok_or("Invalid interface pointer")?;

// Query units
if let Some(units_query) = interface.units_query() {
    // Get all units on a team
    let team_units = units_query.get_team_units(0)?;
    println!("Team 0 has {} units", team_units.len());

    // Check if a unit ID is valid
    if units_query.valid_unit_id(42)? {
        println!("Unit 42 exists!");
    }
}

// Get unit information
if let Some(units_info) = interface.units_info() {
    let tooltip = units_info.get_unit_tooltip(42)?;
    println!("Unit tooltip: {:?}", tooltip);

    let health = units_info.get_unit_health(42)?;
    println!("Health: {}/{}", health.current, health.max);
}

// Terrain queries
if let Some(terrain) = interface.terrain() {
    let height = terrain.get_ground_height(100.0, 200.0)?;
    let (in_map, in_play_area) = terrain.is_pos_in_map(100.0, 200.0)?;
}

// Game state modification (synced)
if let Some(synced) = interface.synced_ctrl() {
    // Create a unit
    if let Some(unit_ctrl) = synced.unit() {
        let pos = sys::Float3 { x: 100.0, y: 0.0, z: 200.0 };
        let unit_id = unit_ctrl.create_unit(42, pos, 0, 0, false, -1)?;

        // Give it an order
        let params = vec![500.0, 0.0, 600.0];
        unit_ctrl.give_order_to_unit(unit_id, 10, &params, 0)?;
    }

    // Set alliance
    if let Some(team_ctrl) = synced.team() {
        team_ctrl.set_ally(0, 1, true)?;
    }

    // Modify terrain
    if let Some(terrain_ctrl) = synced.terrain() {
        let pos = sys::Float3 { x: 100.0, y: 10.0, z: 200.0 };
        terrain_ctrl.add_height_map(pos, 5.0)?;
    }
}
```

## API Overview

### Units

- **UnitsQuery** - Spatial queries, filtering, team/ally queries
  - `get_all_units() -> Vec<i32>`
  - `get_team_units(team_id: i32) -> Vec<i32>`
  - `get_units_in_rectangle(rect: RectangleQuery) -> Vec<i32>`
  - `valid_unit_id(unit_id: i32) -> bool`

- **UnitsInfo** - Unit properties, state, sensors
  - `get_unit_tooltip(unit_id: i32) -> Option<String>`
  - `get_unit_health(unit_id: i32) -> UnitHealth`
  - `get_unit_position(unit_id: i32) -> Float3`
  - ~80 functions covering all unit state

- **UnitsWeapons** - Weapon state and targeting
  - `get_unit_weapons(unit_id: i32) -> Vec<WeaponInfo>`
  - `get_weapon_target(unit_id: i32, weapon_num: i32) -> TargetInfo`

- **UnitsCommands** - Command queues and factory queues
  - `get_unit_commands(unit_id: i32) -> Vec<Command>`
  - `get_factory_commands(unit_id: i32) -> Vec<Command>`

- **UnitsPieces** - Model piece queries
  - `get_unit_piece_info(unit_id: i32, piece: i32) -> PieceInfo`

### Game State

- **Game** - Core game state, timing, options
  - `get_game_frame() -> i32`
  - `get_game_speed() -> f32`
  - `get_wind() -> (f32, f32, f32)`
  - `get_tidal_strength() -> f32`

- **Terrain** - Height maps, normals, blocking
  - `get_ground_height(x: f32, z: f32) -> f32`
  - `is_pos_in_map(x: f32, z: f32) -> (bool, bool)`
  - `get_ground_normal(x: f32, z: f32) -> Float3`

- **Teams** - Team/ally team/player management
  - `get_team_info(team_id: i32) -> TeamInfo`
  - `get_team_resources(team_id: i32) -> TeamResources`
  - `get_player_info(player_id: i32) -> PlayerInfo`

- **Player** - Local player info and roster
  - `get_my_player_id() -> i32`
  - `get_my_team_id() -> i32`
  - `get_player_roster() -> Vec<PlayerInfo>`

### Definitions

- **UnitDefs** - Static unit definition data
- **FeatureDefs** - Static feature definition data
- **WeaponDefs** - Static weapon definition data

### Game Objects

- **Features** - Feature queries (wrecks, rocks, trees)
- **Projectiles** - Projectile queries (bullets, missiles)
- **Los** - Line of sight and radar queries
  - `is_pos_in_los(pos: Float3, ally_team: i32) -> bool`
  - `is_pos_in_radar(pos: Float3, ally_team: i32) -> bool`

### Utilities

- **MathExtra** - Extended math functions
  - `normalize(vec: &mut Float3) -> f32`
  - `hypot(x: f32, y: f32) -> f32`
  - `clamp(value: f32, min: f32, max: f32) -> f32`

- **MetalMap** - Metal resource queries
- **PathFinder** - Pathfinding requests
- **RulesParams** - Custom game parameters
- **MoveCtrl** - Unit movement control
- **SyncedCtrl** - Synced game state modification (composite API):
  - `team()` - **TeamControl** (11 functions): Alliances, resources, start boxes, game over
  - `unit()` - **UnitControl** (19 functions): Create/destroy units, give orders, modify health/position/velocity
  - `feature()` - **FeatureControl** (8 functions): Create/destroy features, modify properties
  - `terrain()` - **TerrainControl** (10 functions): Height maps, smooth mesh, terrain types, wind/tidal
  - `projectile()` - **ProjectileControl** (6 functions): Spawn/delete projectiles, modify trajectory

### UI & Rendering

- **Camera** - Camera queries and control
- **Input** - Mouse and keyboard input
- **Display** - Display and rendering state
- **Selection** - Unit selection queries

### I/O & System

- **Vfs** - Virtual file system access
- **Sound** - Sound playback control
- **Messages** - Chat and console messaging
- **Config** - Engine configuration
- **Memory** - FFI memory management
- **Tracing** - Ray tracing and collision
- **Utils** - Miscellaneous utilities

## Type Mappings

The code generator automatically translates C types to Rust:

| C Type | Rust Type | Example |
|--------|-----------|---------|
| `const T*` + `uint32_t count` | `&[T]` | Array parameters |
| `T**` + `count` (output) | `Vec<T>` | Array returns |
| `const char*` | `&str` | String parameters |
| `const char*` (output) | `Option<String>` | String returns |
| `bool` | `bool` | Booleans |
| `int32_t` | `i32` | Integers |
| `float` | `f32` | Floats |
| `Float3` | `sys::Float3` | Structs |
| `T*` (single, mutable) | `&mut T` | Mutable references |
| `void*` | `*mut c_void` | Raw pointers |

## Error Handling

All API functions return `Result<T, Error>`. The `Error` type contains:
- Error code (from the engine)
- Error message

```rust
match units_query.get_team_units(team_id) {
    Ok(units) => println!("Found {} units", units.len()),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Examples

See `examples/mock_units_query.rs` for a complete example showing how to:
- Create a mock `NativeInterface` for testing
- Wire up API function pointers
- Call the safe Rust wrappers
- Handle errors

Run the example:
```bash
cargo run --example mock_units_query
```

## Architecture

This crate consists of three parts:

1. **spring-native-sys** - Raw C bindings via `bindgen`
2. **spring-native-codegen** - Code generator using libclang
3. **spring-native** - Safe wrappers (this crate)

The build process:
1. `spring-native-sys` runs bindgen on `NativeInterface.h`
2. `spring-native` build script runs `spring-native-codegen` for each API
3. Generated code is included via `include!(concat!(env!("OUT_DIR"), "/<module>_generated.rs"))`

See `doc/rust_native_interface.md` for full architecture documentation.

## Development

To rebuild the bindings:
```bash
cargo build
```

To generate documentation:
```bash
cargo doc --open
```

To add a new API module:
1. Create `src/<module>.rs` (copy pattern from existing modules)
2. Add generator function to `spring-native-codegen/src/lib.rs`
3. Update `build.rs` to call the generator
4. Add to `lib.rs` exports and `interface.rs` accessors

## License

GPL v2 or later (matches Spring engine license)

//! # spring-native
//!
//! Safe Rust bindings for the Spring RTS Engine Native Interface.
//!
//! This crate provides type-safe, idiomatic Rust wrappers around the C FFI defined in
//! `rts/NativeInterface`. All code is automatically generated at build time from the C++
//! header files, ensuring the bindings stay synchronized with the engine.
//!
//! ## Features
//!
//! - **Automatic code generation** - Bindings are generated from C++ headers using libclang
//! - **Type safety** - Query structs are flattened into strongly-typed function parameters
//! - **Idiomatic Rust** - Uses `Result<T, Error>`, `&[T]` slices, and `&str` strings
//! - **Zero unsafe in user code** - All FFI unsafe code is encapsulated in the generated wrappers
//! - **46 API modules** - Complete coverage of all Spring Native Interface APIs (38 main + 8 sub-APIs)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use spring_native::{NativeInterfaceRef, Error};
//!
//! # fn example(native_interface_ptr: *const spring_native::sys::NativeInterface) -> Result<(), Error> {
//! // Get the interface from the engine (provided by Spring at runtime)
//! let interface = unsafe { NativeInterfaceRef::from_ptr(native_interface_ptr) }
//!     .ok_or_else(|| Error::new(1, "Invalid interface pointer".to_string()))?;
//!
//! // Query units
//! if let Some(units_query) = interface.units_query() {
//!     let team_units = units_query.get_team_units(0)?;
//!     println!("Team 0 has {} units", team_units.len());
//! }
//!
//! // Get unit information
//! if let Some(units_info) = interface.units_info() {
//!     if let Some(tooltip) = units_info.get_unit_tooltip(42)? {
//!         println!("Unit tooltip: {}", tooltip);
//!     }
//! }
//!
//! // Modify game state (synced)
//! if let Some(synced) = interface.synced_ctrl() {
//!     if let Some(unit_ctrl) = synced.unit() {
//!         let pos = spring_native::sys::Float3 { x: 100.0, y: 0.0, z: 200.0 };
//!         let unit_id = unit_ctrl.create_unit(42, pos, 0, 0, false, -1)?;
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## API Modules
//!
//! ### Units
//! - [`UnitsQuery`] - Spatial queries, filtering, team/ally queries
//! - [`UnitsInfo`] - Unit properties, state, sensors (~80 functions)
//! - [`UnitsWeapons`] - Weapon state and targeting
//! - [`UnitsCommands`] - Command queues and factory queues
//! - [`UnitsPieces`] - Model piece queries
//!
//! ### Game State
//! - [`Game`] - Core game state, timing, options
//! - [`Terrain`] - Height maps, normals, blocking
//! - [`Teams`] - Team/ally team/player management
//! - [`Player`] - Local player info and roster
//!
//! ### Definitions
//! - [`UnitDefs`] - Static unit definition data
//! - [`FeatureDefs`] - Static feature definition data
//! - [`WeaponDefs`] - Static weapon definition data
//!
//! ### Game Objects
//! - [`Features`] - Feature queries (wrecks, rocks, trees)
//! - [`Projectiles`] - Projectile queries
//! - [`Los`] - Line of sight and radar queries
//!
//! ### Control (Synced)
//! - [`SyncedCtrl`] - Composite API with 8 sub-APIs:
//!   - [`TeamControl`] - Alliances, resources (11 functions)
//!   - [`UnitControl`] - Create/destroy units, orders (19 functions)
//!   - [`FeatureControl`] - Feature modification (8 functions)
//!   - [`TerrainControl`] - Height maps, terrain types (10 functions)
//!   - [`ProjectileControl`] - Spawn/modify projectiles (6 functions)
//!   - [`EffectsControl`] - Spawn explosions/CEGs/SFX (3 functions)
//!   - [`GameConfig`] - Game rule toggles and radar error settings (4 functions)
//!   - [`CobScript`] - Call COB scripts and resolve IDs (2 functions)
//!
//! ### Utilities
//! - [`MathExtra`] - Extended math functions
//! - [`MetalMap`] - Metal resource queries
//! - [`PathFinder`] - Pathfinding requests
//! - [`RulesParams`] - Custom game parameters
//! - [`MoveCtrl`] - Unit movement control
//!
//! ### UI & Rendering
//! - [`Camera`] - Camera queries and control
//! - [`Input`] - Mouse and keyboard input
//! - [`Display`] - Display and rendering state
//! - [`Selection`] - Unit selection queries
//! - [`Icons`] - Unit icon definitions and draw toggles
//! - [`Markers`] - Map/world marker helpers
//! - [`GroundDecals`] - Dynamic ground decals (create, query, modify)
//! - [`UnsyncedRead`] - Client-only rendering and UI state (UnitRendering sub-API)
//! - [`UnsyncedCtrl`] - Client-only rendering and UI control (unit flags, minimap)
//!
//! ### I/O & System
//! - [`Vfs`] - Virtual file system access
//! - [`Sound`] - Sound playback control
//! - [`Messages`] - Chat and console messaging
//! - [`Config`] - Engine configuration
//! - [`Memory`] - FFI memory management
//! - [`Tracing`] - Ray tracing and collision
//! - [`Lights`] - Dynamic map/model lights
//! - [`SystemControl`] - Game lifecycle, team sharing, watchdog/yield
//! - [`Profiling`] - Timers, profiler records, Lua/VRAM usage
//! - [`Utils`] - Miscellaneous utilities
//!
//! ## Error Handling
//!
//! All API functions return `Result<T, Error>`. The [`Error`] type contains the error code
//! and message from the engine.
//!
//! ## Type Mappings
//!
//! The code generator automatically translates C types to Rust:
//!
//! - `const T*` + `uint32_t count` → `&[T]` (array parameters)
//! - `T**` + `count` (output) → `Vec<T>` (array returns)
//! - `const char*` → `&str` (string parameters)
//! - `const char*` (output) → `Option<String>` (string returns)
//! - Primitive types map directly (i32, f32, bool, etc.)
//!
//! See the [architecture documentation](../../../doc/rust_native_interface.md) for details.

mod callbacks;
mod camera;
pub mod module_entry;
mod config;

// Include build-time generated version constants
mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}

// Re-export version constants
pub use version::{NATIVE_API_VERSION_MAJOR, NATIVE_API_VERSION_MINOR, NATIVE_API_VERSION_PATCH};
mod display;
mod error;
mod feature_defs;
mod features;
mod game;
mod ground_decals;
mod input;
mod icons;
mod interface;
mod los;
mod lights;
mod math_extra;
mod profiling;
mod markers;
mod memory;
mod messages;
mod metal_map;
mod move_ctrl;
mod path_finder;
mod player;
mod system_control;
pub mod prelude;
mod projectiles;
mod rules_params;
mod selection;
mod sound;
mod synced_ctrl;
mod teams;
mod terrain;
mod tracing;
mod unsynced_ctrl;
mod unsynced_read;
mod unit_defs;
mod units_commands;
mod units_info;
mod units_pieces;
mod units_query;
mod units_weapons;
mod utils;
mod vfs;
mod weapon_defs;

pub use callbacks::{ModuleData, NativeModule};
pub use camera::Camera;
pub use config::Config;
pub use display::Display;
pub use error::Error;
pub use feature_defs::FeatureDefs;
pub use features::Features;
pub use game::Game;
pub use ground_decals::GroundDecals;
pub use profiling::Profiling;
pub use input::Input;
pub use icons::Icons;
pub use interface::NativeInterfaceRef;
pub use los::Los;
pub use math_extra::MathExtra;
pub use markers::Markers;
pub use memory::Memory;
pub use messages::Messages;
pub use metal_map::MetalMap;
pub use move_ctrl::MoveCtrl;
pub use path_finder::PathFinder;
pub use player::Player;
pub use system_control::SystemControl;
pub use projectiles::Projectiles;
pub use rules_params::RulesParams;
pub use selection::Selection;
pub use sound::Sound;
pub use lights::Lights;
pub use synced_ctrl::{
    CobScript, EffectsControl, FeatureControl, GameConfig, ProjectileControl, SyncedCtrl,
    TeamControl, TerrainControl, UnitControl,
};
pub use teams::Teams;
pub use terrain::Terrain;
pub use tracing::Tracing;
pub use unsynced_ctrl::UnsyncedCtrl;
pub use unsynced_read::{UnitRendering, UnsyncedRead};
pub use unit_defs::UnitDefs;
pub use units_commands::UnitsCommands;
pub use units_info::UnitsInfo;
pub use units_pieces::UnitsPieces;
pub use units_query::{RectangleQueryExt, UnitsQuery};
pub use units_weapons::UnitsWeapons;
pub use utils::Utils;
pub use vfs::Vfs;
pub use weapon_defs::WeaponDefs;

pub mod sys {
    pub use spring_native_sys::*;
}

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
//! - **Type safety** - Query structs become strongly-typed parameters and named option descriptors
//! - **Idiomatic Rust** - Uses `Result<T, Error>`, `&[T]` slices, and `&str` strings
//! - **Zero unsafe in user code** - All FFI unsafe code is encapsulated in the generated wrappers
//! - **47 API modules** - Complete coverage of all Spring Native Interface APIs (39 main + 8 sub-APIs)
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
//! let units_query = interface.units_query();
//! let team_units = units_query.get_team_units(0)?;
//! println!("Team 0 has {} units", team_units.len());
//!
//! // Get unit information
//! let units_info = interface.units_info();
//! if let Some(tooltip) = units_info.get_unit_tooltip(42)? {
//!     println!("Unit tooltip: {}", tooltip);
//! }
//!
//! // Modify game state (synced)
//! let synced = interface.synced_ctrl();
//! let unit_ctrl = synced.unit();
//! let unit_def = spring_native::sys::DefRef { name: std::ptr::null(), id: 42 };
//! let pos = spring_native::sys::Float3 { x: 100.0, y: 0.0, z: 200.0 };
//! let unit_id = unit_ctrl.create_unit(
//!     unit_def,
//!     pos,
//!     0,
//!     0,
//!     spring_native::CreateUnitOptions::default(),
//! )?;
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
//! - [`Platform`] - Host platform and build properties
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
mod config;
pub mod constants;
pub mod module_entry;

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
mod gfx;
mod ground_decals;
mod icons;
mod input;
mod interface;
mod lights;
mod los;
mod markers;
mod math_extra;
mod memory;
mod messages;
mod metal_map;
mod move_ctrl;
mod path_finder;
mod platform;
mod player;
pub mod prelude;
mod profiling;
mod projectiles;
mod raw;
mod rml_ui;
mod rules_params;
mod selection;
mod sound;
mod synced_ctrl;
mod system_control;
mod teams;
mod terrain;
mod tracing;
mod unit_defs;
mod units_commands;
mod units_info;

// Descriptor types used by generated APIs with optional or flag-heavy query
// arguments. Re-export them at the crate root so callers do not need to know
// which private API module owns the method.
pub use camera::{SetCameraTargetOptions, TraceScreenRayOptions};
pub use gfx::{
    GfxColorMaskOptions, GfxCreateShaderOptions, GfxDepthTestOptions, GfxFeatureDrawOptions,
    GfxFontSubmitBufferedOptions, GfxLightOptions, GfxMultiTexGenOptions, GfxObjectShapeOptions,
    GfxSaveImageOptions, GfxTexGenOptions, GfxUnitDrawOptions,
};
pub use ground_decals::GetGroundDecalTexturesOptions;
pub use markers::MarkerErasePositionOptions;
pub use profiling::DiffTimersOptions;
pub use projectiles::{
    GetAllProjectilesOptions, GetProjectilesInRectangleOptions, GetProjectilesInSphereOptions,
};
pub use rml_ui::{RmlDocumentShowOptions, RmlRegisterEventTypeOptions};
pub use synced_ctrl::{
    BuggerOffOptions, CreateUnitOptions, DestroyUnitOptions, SetFactoryBuggerOffOptions,
    SetFeatureBlockingOptions, SetGodModeOptions, SetUnitBlockingOptions,
    SetUnitLeavesGhostOptions, SetUnitTargetOptions, SetUnitUseWeaponsOptions,
};
pub use tracing::{
    TraceRayGroundBetweenPositionsOptions, TraceRayGroundInDirectionOptions,
    TraceRayInDirectionOptions,
};
pub use units_info::{GetUnitPositionOptions, UnitStatesOptions};
pub use units_query::{
    GetClosestEnemyUnitOptions, GetUnitNearestEnemyOptions, GetUnitSeparationOptions,
};
pub use units_weapons::{
    GetUnitWeaponHaveFreeLineOfFireOptions, GetUnitWeaponTestTargetOptions,
    GetUnitWeaponTryTargetOptions,
};
pub use unsynced_ctrl::{
    SetActiveCommandOptions, SetShockFrontFactorsOptions, SetWindowGeometryOptions,
};
pub use unsynced_read::{GetVisibleFeaturesOptions, GetVisibleProjectilesOptions};
pub use utils::TestMoveOrderOptions;
mod units_pieces;
mod units_query;
mod units_weapons;
mod unsynced_ctrl;
mod unsynced_read;
mod utils;
mod vfs;
mod weapon_defs;

pub use callbacks::{ModuleData, NativeModule};
pub use camera::Camera;
pub use config::{Config, ConfigParameter, ConfigValueType};
pub use display::Display;
pub use error::Error;
pub use feature_defs::{FeatureDefInfo, FeatureDefs};
pub use features::Features;
pub use game::{Game, GameModInfo, SideData};
pub use gfx::{ConsoleCommand, Gfx};
pub use ground_decals::GroundDecals;
pub use icons::Icons;
pub use input::Input;
pub use interface::NativeInterfaceRef;
pub use lights::Lights;
pub use los::Los;
pub use markers::Markers;
pub use math_extra::MathExtra;
pub use memory::Memory;
pub use messages::{ConsoleEntry, Messages};
pub use metal_map::MetalMap;
pub use move_ctrl::MoveCtrl;
pub use path_finder::PathFinder;
pub use platform::Platform;
pub use player::{Player, RosterEntry};
pub use profiling::Profiling;
pub use projectiles::Projectiles;
pub use rml_ui::{
    RmlChoiceRow, RmlColor, RmlDataChoiceRows, RmlDataGridRows, RmlDataIconRows, RmlDataLogRows,
    RmlDataModel, RmlDataNotificationRows, RmlDataOptionRows, RmlDataStatusRows, RmlDataSwatchRows,
    RmlDataTextRows, RmlDataValue, RmlDataVariable, RmlGridRow, RmlIconRow, RmlLogRow,
    RmlLogSeverity, RmlNotificationRow, RmlOptionRow, RmlPercent, RmlPixels,
    RmlPointerCaptureDelta, RmlPointerCaptureStatus, RmlStatusRow, RmlSwatchRow, RmlTextRow, RmlUi,
};
pub use rules_params::{RulesParamValue, RulesParams};
pub use selection::Selection;
pub use sound::Sound;
pub use synced_ctrl::{
    CobScript, EffectsControl, FeatureControl, GameConfig, ProjectileControl, SyncedCtrl,
    TeamControl, TerrainControl, UnitControl,
};
pub use system_control::SystemControl;
pub use teams::{PlayerInfo, TeamInfo, Teams};
pub use terrain::Terrain;
pub use tracing::Tracing;
pub use unit_defs::{UnitDefBasicInfo, UnitDefParamKey, UnitDefParamType, UnitDefs};
pub use units_commands::{CommandDescription, CommandType, UnitsCommands};
pub use units_info::UnitsInfo;
pub use units_pieces::UnitsPieces;
pub use units_query::{RectangleQueryExt, UnitsQuery};
pub use units_weapons::UnitsWeapons;
pub use unsynced_ctrl::UnsyncedCtrl;
pub use unsynced_read::{UnitRendering, UnsyncedRead};
pub use utils::Utils;
pub use vfs::{DirectoryEntry, Vfs};
pub use weapon_defs::WeaponDefs;

pub mod sys {
    pub use spring_native_sys::*;
}

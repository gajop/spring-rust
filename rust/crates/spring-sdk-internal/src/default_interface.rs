//! Hide the engine's built-in interface for games that draw their own.
//!
//! Hiding is idempotent, works from the first frame, does not go through the
//! console-command parser and does not write the player's config. Showing a
//! part again restores the state it had when it was hidden. Available to
//! `ui`, `rules-unsynced` and `gaia-unsynced` modules.
//!
//! ```ignore
//! spring::default_interface::hide(default_interface::ALL)?;
//! ```
//!
//! Health and resurrection bars are not engine elements (they come from Lua
//! widgets), so there is nothing to hide for them here.

/// In-game message console.
pub const CONSOLE: u32 = 1 << 0;
/// Resource bar.
pub const RESOURCE_BAR: u32 = 1 << 1;
/// Tooltip box.
pub const TOOLTIP: u32 = 1 << 2;
/// Game clock.
pub const CLOCK: u32 = 1 << 3;
/// FPS counter.
pub const FPS: u32 = 1 << 4;
/// Simulation speed indicator.
pub const SPEED: u32 = 1 << 5;
/// Player roster (`/info`).
pub const PLAYER_INFO: u32 = 1 << 6;
/// Minimap: neither drawn nor handling input.
pub const MINIMAP: u32 = 1 << 7;
/// Command button panel: neither drawn nor clickable.
pub const COMMAND_MENU: u32 = 1 << 8;
/// End-of-game statistics graph.
pub const END_GRAPH: u32 = 1 << 9;
/// Units turning into distance icons when zoomed out.
pub const UNIT_ICONS: u32 = 1 << 10;
/// Map border.
pub const MAP_BORDER: u32 = 1 << 11;
/// Everything above.
pub const ALL: u32 = (1 << 12) - 1;

/// Hide `parts`; returns the mask of parts now hidden.
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub fn hide(parts: u32) -> crate::Result<u32> {
    crate::unsynced_ctrl::set_default_interface_visible(parts, false)
}

/// Show `parts` again; returns the mask of parts still hidden.
#[cfg(all(feature = "alloc", target_arch = "wasm32"))]
pub fn show(parts: u32) -> crate::Result<u32> {
    crate::unsynced_ctrl::set_default_interface_visible(parts, true)
}

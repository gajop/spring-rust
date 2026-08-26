/// Example: Echo Module - A Simple Spring RTS Native Module
///
/// This example demonstrates how to create a Spring native module with NO unsafe code!
///
/// The module:
/// - Tracks units as they're created and destroyed
/// - Responds to game events
/// - Echoes Lua messages
///
/// Notice: The user only writes safe Rust. All FFI boilerplate is handled by the
/// `export_module!` macro.
use spring_native::prelude::*;

/// Our module implementation - just a normal Rust struct!
///
/// The module stores a reference to the Spring interface, allowing it to
/// call Spring APIs from within callbacks.
struct EchoModule {
    interface: NativeInterfaceRef,
    unit_count: u32,
}

/// Implement the NativeModule trait to handle Spring callbacks
///
/// All methods are optional - only implement what you need.
/// Default implementations do nothing, so you won't get errors if you
/// don't implement a callback.
impl NativeModule for EchoModule {
    /// Called when Spring loads the module
    ///
    /// The interface parameter provides access to all Spring APIs.
    /// Store it in your module to use in callbacks.
    fn new(interface: NativeInterfaceRef) -> Self {
        println!("[EchoModule] Module initialized!");
        EchoModule {
            interface,
            unit_count: 0,
        }
    }

    /// Called when the game starts
    fn game_start(&mut self) -> Result<(), Error> {
        println!("[EchoModule] Game has started! Let's play!");

        // Example: Query game information using the Spring API
        let (frame_low, frame_high) = self.interface.game().get_game_frame()?;
        let frame = frame_low | (frame_high << 16);
        println!("[EchoModule] Starting at game frame: {}", frame);

        Ok(())
    }

    /// Called when a unit is created
    fn unit_created(
        &mut self,
        unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        builder_id: i32,
    ) -> Result<(), Error> {
        self.unit_count += 1;

        if builder_id >= 0 {
            println!(
                "[EchoModule] Unit #{} built by unit #{} (total units: {})",
                unit_id, builder_id, self.unit_count
            );
        } else {
            println!(
                "[EchoModule] Unit #{} spawned (total units: {})",
                unit_id, self.unit_count
            );
        }

        // Example: Query unit information using the Spring API
        let units_info = self.interface.units_info();
        if let Ok(pos) = units_info.get_unit_position(
            unit_id,
            spring_native::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        ) {
            println!(
                "[EchoModule] Unit #{} position: ({:.1}, {:.1}, {:.1})",
                unit_id, pos.x, pos.y, pos.z
            );
        }

        if let Ok(health) = units_info.get_unit_health(unit_id) {
            println!(
                "[EchoModule] Unit #{} health: {:.0}/{:.0}",
                unit_id, health.health, health.maxHealth
            );
        }

        Ok(())
    }

    /// Called when a unit is destroyed
    fn unit_destroyed(
        &mut self,
        unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        attacker_id: Option<i32>,
        _attacker_def_id: Option<i32>,
        _attacker_team: Option<i32>,
        _weapon_def_id: i32,
    ) -> Result<(), Error> {
        self.unit_count = self.unit_count.saturating_sub(1);

        if let Some(attacker_id) = attacker_id {
            println!(
                "[EchoModule] Unit #{} destroyed by #{} (remaining: {})",
                unit_id, attacker_id, self.unit_count
            );
        } else {
            println!(
                "[EchoModule] Unit #{} destroyed (remaining: {})",
                unit_id, self.unit_count
            );
        }

        Ok(())
    }

    /// Called when a unit finishes construction
    fn unit_finished(
        &mut self,
        unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
    ) -> Result<(), Error> {
        println!("[EchoModule] Unit #{} construction complete!", unit_id);
        Ok(())
    }

    /// Called when Lua code sends a message to the module
    fn handle_lua_call(&mut self, message: &str) -> Result<(), Error> {
        println!("[EchoModule] Received Lua message: '{}'", message);
        println!("[EchoModule] Echoing back: 'You said: {}'", message);

        // In a real module, you could send a response back to Spring here
        // using the Spring interface APIs

        Ok(())
    }

    /// Called when a player pauses or unpauses the game
    fn game_paused(&mut self, player_id: i32, paused: bool) -> Result<(), Error> {
        if paused {
            println!("[EchoModule] Player {} paused the game", player_id);
        } else {
            println!("[EchoModule] Player {} unpaused the game", player_id);
        }
        Ok(())
    }

    /// Called when the module is shutting down
    fn shutdown(&mut self) -> Result<(), Error> {
        println!(
            "[EchoModule] Shutting down. Final unit count: {}",
            self.unit_count
        );
        println!("[EchoModule] Goodbye!");
        Ok(())
    }
}

// ============================================================================
// THAT'S IT! This one line exports everything Spring needs:
// ============================================================================

spring_native::export_module!(EchoModule);

//
// The macro above generates:
// - InitializeNativeModule (entry point)
// - All 30+ callback exports (GameStart, UnitCreated, etc.)
// - Panic handler setup
// - Module data management
// - Type conversions (C strings, byte arrays, etc.)
//
// You don't have to write ANY of that yourself!
//

// ============================================================================
// Main function (just for demonstration - not used by Spring)
// ============================================================================

fn main() {
    println!("=== Spring RTS Native Module Example ===");
    println!();
    println!("This is a template for building Spring modules in Rust.");
    println!("To use this in Spring:");
    println!();
    println!("1. Add to your Cargo.toml:");
    println!("   [lib]");
    println!("   crate-type = [\"cdylib\"]");
    println!();
    println!("2. Build with: cargo build --release");
    println!();
    println!("3. The output will be: target/release/libecho_module.so");
    println!("   (or .dll on Windows, .dylib on macOS)");
    println!();
    println!("4. Configure Spring to load your module");
    println!();
    println!("Notice how clean the code is:");
    println!("  - No unsafe blocks");
    println!("  - No FFI types in your code");
    println!("  - No pointer management");
    println!("  - Just pure Rust!");
    println!();
    println!("The module demonstrates:");
    println!("  - Receiving callbacks from Spring (unit_created, game_start, etc.)");
    println!(
        "  - Calling Spring APIs from callbacks (game.get_game_frame, units_info.get_unit_position)"
    );
    println!("  - Storing the interface reference for easy access");
    println!();
    println!("All the complexity is hidden by the export_module! macro.");
}

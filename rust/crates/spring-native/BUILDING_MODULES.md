# Building Native Spring Modules in Rust

This guide shows you how to create a native Spring RTS module using the `spring-native` bindings.

## Overview

A native Spring module has two parts:

1. **Callbacks** (Spring → Rust) - Spring calls your code when events happen
2. **API Calls** (Rust → Spring) - Your code queries/modifies game state

The `spring-native` crate provides:
- Safe Rust wrappers for all Spring APIs (37 modules)
- `NativeModule` trait for implementing callbacks
- Helper types to reduce FFI boilerplate

## Project Setup

### 1. Create a New Library Crate

```bash
cargo new --lib my-spring-module
cd my-spring-module
```

### 2. Configure Cargo.toml

```toml
[package]
name = "my-spring-module"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # REQUIRED: builds .so dynamic library

[dependencies]
spring-native = "0.1"
```

The `crate-type = ["cdylib"]` is **critical** - it tells Rust to build a C-compatible dynamic library that Spring can load.

## Implementing Your Module

### Step 1: Define Your Module Struct

```rust
use spring_native::prelude::*;

pub struct MyModule {
    // Store the interface to call Spring APIs
    interface: NativeInterfaceRef,
    // Your state here
    unit_count: u32,
}

impl NativeModule for MyModule {
    fn new(interface: NativeInterfaceRef) -> Self {
        println!("[MyModule] Initializing");
        MyModule {
            interface,
            unit_count: 0,
        }
    }

    // Implement callbacks you care about
    fn game_start(&mut self) -> Result<(), Error> {
        println!("[MyModule] Game started!");

        // Use the interface to call Spring APIs
        if let Some(game) = self.interface.game() {
            let frame = game.get_game_frame()?;
            println!("[MyModule] Starting at frame {}", frame);
        }

        Ok(())
    }

    fn unit_created(&mut self, unit_id: i32, builder_id: i32) -> Result<(), Error> {
        self.unit_count += 1;
        println!("[MyModule] Unit #{} created (total: {})", unit_id, self.unit_count);

        // Query unit information from Spring
        if let Some(units_info) = self.interface.units_info() {
            if let Ok(pos) = units_info.get_unit_position(unit_id) {
                println!("[MyModule]   Position: ({:.1}, {:.1}, {:.1})", pos.x, pos.y, pos.z);
            }
        }

        Ok(())
    }

    fn unit_destroyed(&mut self, unit_id: i32, _attacker_id: i32) -> Result<(), Error> {
        self.unit_count = self.unit_count.saturating_sub(1);
        println!("[MyModule] Unit #{} destroyed (remaining: {})", unit_id, self.unit_count);
        Ok(())
    }
}
```

### Step 2: Export the Module

That's it! Just one line:

```rust
// This exports all 30+ callbacks automatically
spring_native::export_module!(MyModule);
```

The `export_module!` macro handles everything:
- Exports `InitializeNativeModule` entry point
- Exports all callback functions (GameStart, UnitCreated, etc.)
- Sets up panic handler
- Manages module data and type conversions
- Exports API version for compatibility checking

## Available Callbacks

The `NativeModule` trait provides these callbacks (all optional):

### Download Events
- `download_failed(download_id, error_id)`
- `download_finished(download_id)`
- `download_progress(download_id, downloaded, total)`
- `download_queued(download_id, archive_name, archive_type)`
- `download_started(download_id)`

### Feature Events
- `feature_created(feature_id)`
- `feature_destroyed(feature_id)`

### Game Events
- `game_id(game_id: &[u8])`
- `game_paused(player_id, paused)`
- `game_preload()`
- `game_start()`

### Player Events
- `player_added(player_id)`
- `player_changed(player_id)`
- `player_removed(player_id, reason)`

### Team Events
- `team_changed(team_id)`
- `team_died(team_id)`

### Unit Events
- `unit_created(unit_id, builder_id)`
- `unit_destroyed(unit_id, attacker_id)`
- `unit_experience(unit_id, old_experience)`
- `unit_finished(unit_id)`
- `unit_from_factory(unit_id, factory_id, user_orders)`
- `unit_given(unit_id, old_team, new_team)`
- `unit_loaded(unit_id, transport_id)`
- `unit_stunned(unit_id, stunned)`
- `unit_taken(unit_id, old_team, new_team)`
- `unit_unloaded(unit_id, transport_id)`
- `render_unit_destroyed(unit_id)`

### Special Events
- `handle_lua_msg(player_id, script, mode, data: &[u8])`
- `handle_lua_call(message: &str)`
- `shutdown()`

## Building and Testing

### Build the Module

```bash
cargo build --release
```

This creates `target/release/libmy_spring_module.so` (Linux) or equivalent on other platforms.

### Load in Spring

Copy the `.so` file to your Spring game directory and configure Spring to load it.

## Complete Example

See `examples/echo_module.rs` in the spring-native repository for a complete, documented example showing:

- Module struct with state
- Implementing NativeModule trait
- Exporting all necessary callbacks
- Handling string parameters
- Error handling

## Future Improvements

Currently, you must manually export each callback function. Future versions may include:

- `#[spring_module]` procedural macro to auto-generate exports
- Simplified callback registration
- Built-in interface access from callbacks

For now, the pattern shown above gives you full control with minimal boilerplate (~10-20 lines per callback).

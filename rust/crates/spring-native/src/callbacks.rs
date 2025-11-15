use crate::{error::Error, interface::NativeInterfaceRef};

/// Trait for native modules to implement Spring Engine callbacks.
///
/// All methods have default implementations that do nothing, so modules only need
/// to implement the callbacks they care about.
///
/// # Example
///
/// ```rust,no_run
/// use spring_native::{NativeModule, Error};
///
/// struct MyModule;
///
/// impl NativeModule for MyModule {
///     fn new() -> Self {
///         MyModule
///     }
///
///     fn game_start(&mut self) -> Result<(), Error> {
///         println!("Game started!");
///         Ok(())
///     }
///
///     fn unit_created(&mut self, unit_id: i32, builder_id: i32) -> Result<(), Error> {
///         println!("Unit {} created by {}", unit_id, builder_id);
///         Ok(())
///     }
/// }
/// ```
pub trait NativeModule: Sized {
    /// Create a new instance of this module.
    ///
    /// Called once when Spring loads the module via `InitializeNativeModule`.
    /// The `interface` parameter provides access to all Spring APIs and should
    /// typically be stored in your module for use in callbacks.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use spring_native::{NativeModule, NativeInterfaceRef, Error};
    /// struct MyModule {
    ///     interface: NativeInterfaceRef,
    ///     unit_count: u32,
    /// }
    ///
    /// impl NativeModule for MyModule {
    ///     fn new(interface: NativeInterfaceRef) -> Self {
    ///         MyModule {
    ///             interface,
    ///             unit_count: 0,
    ///         }
    ///     }
    ///
    ///     fn unit_created(&mut self, unit_id: i32, builder_id: i32) -> Result<(), Error> {
    ///         // Access Spring APIs via self.interface
    ///         if let Some(units_info) = self.interface.units_info() {
    ///             let pos = units_info.get_unit_position(unit_id)?;
    ///             println!("Unit at ({}, {}, {})", pos.x, pos.y, pos.z);
    ///         }
    ///         Ok(())
    ///     }
    /// }
    /// ```
    fn new(interface: NativeInterfaceRef) -> Self;

    // ========================================================================
    // Download Events
    // ========================================================================

    /// Called when a download fails.
    fn download_failed(&mut self, download_id: i32, error_id: i32) -> Result<(), Error> {
        let _ = (download_id, error_id);
        Ok(())
    }

    /// Called when a download finishes successfully.
    fn download_finished(&mut self, download_id: i32) -> Result<(), Error> {
        let _ = download_id;
        Ok(())
    }

    /// Called to report download progress.
    fn download_progress(
        &mut self,
        download_id: i32,
        downloaded: i64,
        total: i64,
    ) -> Result<(), Error> {
        let _ = (download_id, downloaded, total);
        Ok(())
    }

    /// Called when a download is queued.
    fn download_queued(
        &mut self,
        download_id: i32,
        archive_name: &str,
        archive_type: &str,
    ) -> Result<(), Error> {
        let _ = (download_id, archive_name, archive_type);
        Ok(())
    }

    /// Called when a download starts.
    fn download_started(&mut self, download_id: i32) -> Result<(), Error> {
        let _ = download_id;
        Ok(())
    }

    // ========================================================================
    // Feature Events
    // ========================================================================

    /// Called when a feature is created.
    fn feature_created(&mut self, feature_id: i32) -> Result<(), Error> {
        let _ = feature_id;
        Ok(())
    }

    /// Called when a feature is destroyed.
    fn feature_destroyed(&mut self, feature_id: i32) -> Result<(), Error> {
        let _ = feature_id;
        Ok(())
    }

    // ========================================================================
    // Game Events
    // ========================================================================

    /// Called with the game ID (unique identifier for this game session).
    fn game_id(&mut self, game_id: &[u8]) -> Result<(), Error> {
        let _ = game_id;
        Ok(())
    }

    /// Called when the game is paused or unpaused.
    fn game_paused(&mut self, player_id: i32, paused: bool) -> Result<(), Error> {
        let _ = (player_id, paused);
        Ok(())
    }

    /// Called during game preload phase.
    fn game_preload(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Called when the game starts.
    fn game_start(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // ========================================================================
    // Player Events
    // ========================================================================

    /// Called when a player is added to the game.
    fn player_added(&mut self, player_id: i32) -> Result<(), Error> {
        let _ = player_id;
        Ok(())
    }

    /// Called when a player's state changes.
    fn player_changed(&mut self, player_id: i32) -> Result<(), Error> {
        let _ = player_id;
        Ok(())
    }

    /// Called when a player is removed from the game.
    fn player_removed(&mut self, player_id: i32, reason: i32) -> Result<(), Error> {
        let _ = (player_id, reason);
        Ok(())
    }

    // ========================================================================
    // Team Events
    // ========================================================================

    /// Called when a team's state changes.
    fn team_changed(&mut self, team_id: i32) -> Result<(), Error> {
        let _ = team_id;
        Ok(())
    }

    /// Called when a team is defeated.
    fn team_died(&mut self, team_id: i32) -> Result<(), Error> {
        let _ = team_id;
        Ok(())
    }

    // ========================================================================
    // Unit Events
    // ========================================================================

    /// Called when a unit is created.
    ///
    /// `builder_id` is -1 if the unit was not built by another unit.
    fn unit_created(&mut self, unit_id: i32, builder_id: i32) -> Result<(), Error> {
        let _ = (unit_id, builder_id);
        Ok(())
    }

    /// Called when a unit is destroyed.
    ///
    /// `attacker_id` is -1 if there was no attacker.
    fn unit_destroyed(&mut self, unit_id: i32, attacker_id: i32) -> Result<(), Error> {
        let _ = (unit_id, attacker_id);
        Ok(())
    }

    /// Called when a unit gains experience.
    fn unit_experience(&mut self, unit_id: i32, old_experience: f32) -> Result<(), Error> {
        let _ = (unit_id, old_experience);
        Ok(())
    }

    /// Called when a unit finishes construction.
    fn unit_finished(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    /// Called when a unit exits a factory.
    fn unit_from_factory(
        &mut self,
        unit_id: i32,
        factory_id: i32,
        user_orders: bool,
    ) -> Result<(), Error> {
        let _ = (unit_id, factory_id, user_orders);
        Ok(())
    }

    /// Called when a unit is given to another team.
    fn unit_given(&mut self, unit_id: i32, old_team: i32, new_team: i32) -> Result<(), Error> {
        let _ = (unit_id, old_team, new_team);
        Ok(())
    }

    /// Called when a unit is loaded into a transport.
    fn unit_loaded(&mut self, unit_id: i32, transport_id: i32) -> Result<(), Error> {
        let _ = (unit_id, transport_id);
        Ok(())
    }

    /// Called when a unit is stunned or unstunned.
    fn unit_stunned(&mut self, unit_id: i32, stunned: bool) -> Result<(), Error> {
        let _ = (unit_id, stunned);
        Ok(())
    }

    /// Called when a unit is captured by another team.
    fn unit_taken(&mut self, unit_id: i32, old_team: i32, new_team: i32) -> Result<(), Error> {
        let _ = (unit_id, old_team, new_team);
        Ok(())
    }

    /// Called when a unit is unloaded from a transport.
    fn unit_unloaded(&mut self, unit_id: i32, transport_id: i32) -> Result<(), Error> {
        let _ = (unit_id, transport_id);
        Ok(())
    }

    /// Called when a unit is destroyed (rendering-specific).
    fn render_unit_destroyed(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    // ========================================================================
    // Special Events
    // ========================================================================

    /// Called to handle a Lua message (binary data).
    fn handle_lua_msg(
        &mut self,
        player_id: i32,
        script: i32,
        mode: i32,
        data: &[u8],
    ) -> Result<(), Error> {
        let _ = (player_id, script, mode, data);
        Ok(())
    }

    /// Called to handle a Lua call (text message).
    fn handle_lua_call(&mut self, message: &str) -> Result<(), Error> {
        let _ = message;
        Ok(())
    }

    /// Called when the module is being shut down.
    fn shutdown(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

/// Type-safe wrapper for module data passed through the C FFI boundary.
///
/// This holds both the user's module implementation and a reference to the
/// Spring interface, ensuring type safety when callbacks are invoked.
pub struct ModuleData<T: NativeModule> {
    /// The user's module implementation
    module: Box<T>,
    /// Reference to the Spring interface for making API calls
    interface: NativeInterfaceRef,
}

impl<T: NativeModule> ModuleData<T> {
    /// Create a new module data wrapper.
    ///
    /// # Safety
    ///
    /// The `interface_ptr` must be a valid pointer to a `NativeInterface` struct
    /// that remains valid for the lifetime of this `ModuleData`.
    pub unsafe fn new(interface_ptr: *const crate::sys::NativeInterface) -> Box<Self> {
        let interface = NativeInterfaceRef::from_ptr(interface_ptr)
            .expect("Invalid NativeInterface pointer");

        Box::new(ModuleData {
            module: Box::new(T::new(interface.clone())),
            interface,
        })
    }

    /// Get a reference to the module implementation.
    pub fn module(&mut self) -> &mut T {
        &mut self.module
    }

    /// Get a reference to the Spring interface.
    pub fn interface(&self) -> &NativeInterfaceRef {
        &self.interface
    }
}

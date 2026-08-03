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
///     fn new(_interface: spring_native::NativeInterfaceRef) -> Self {
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
    ///         let units_info = self.interface.units_info();
    ///         let pos = units_info.get_unit_position(unit_id, false, false)?;
    ///         println!("Unit at ({}, {}, {})", pos.x, pos.y, pos.z);
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

    /// Called when the game ends.
    fn game_over(&mut self, winning_ally_teams: &[u8]) -> Result<(), Error> {
        let _ = winning_ally_teams;
        Ok(())
    }

    /// Called once per simulation frame. Does not fire while the sim is paused.
    fn game_frame(&mut self, game_frame: i32) -> Result<(), Error> {
        let _ = game_frame;
        Ok(())
    }

    /// Called after each simulation frame. Does not fire while the sim is paused.
    fn game_frame_post(&mut self, game_frame: i32) -> Result<(), Error> {
        let _ = game_frame;
        Ok(())
    }

    /// Called once per drawn frame (unsynced). Fires even while the sim is
    /// paused — the native equivalent of `widget:Update`. Use for polling and
    /// deferred work that doesn't need a GL context.
    fn update(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Called during the screen draw pass (unsynced) with a valid GL context —
    /// the native equivalent of SpringBoard's `delayGL`. GfxApi operations are
    /// only valid when issued from here.
    fn draw_screen(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_genesis(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_world(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_world_pre_unit(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_pre_decals(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_world_pre_particles(
        &mut self,
        draw_above_water: bool,
        draw_below_water: bool,
        draw_reflection: bool,
        draw_refraction: bool,
    ) -> Result<(), Error> {
        let _ = (
            draw_above_water,
            draw_below_water,
            draw_reflection,
            draw_refraction,
        );
        Ok(())
    }

    fn draw_water_post(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_world_shadow(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_shadow_pass_transparent(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_world_reflection(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_world_refraction(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_ground_pre_forward(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_ground_post_forward(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_ground_pre_deferred(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_ground_deferred(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_ground_post_deferred(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_units_post_deferred(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_features_post_deferred(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_screen_effects(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_screen_post(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_in_minimap(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_in_minimap_background(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Called when the minimap rotation changes.
    fn minimap_rotation_changed(&mut self, new_rot: f32, old_rot: f32) -> Result<(), Error> {
        let _ = (new_rot, old_rot);
        Ok(())
    }

    /// Called when the minimap minimized/maximized/slaved state changes.
    fn minimap_state_changed(
        &mut self,
        is_minimized: bool,
        is_maximized: bool,
        is_slaved: bool,
    ) -> Result<(), Error> {
        let _ = (is_minimized, is_maximized, is_slaved);
        Ok(())
    }

    /// Called when the minimap geometry changes.
    fn minimap_geometry_changed(
        &mut self,
        new_pos_x: i32,
        new_pos_y: i32,
        new_dim_x: i32,
        new_dim_y: i32,
        old_pos_x: i32,
        old_pos_y: i32,
        old_dim_x: i32,
        old_dim_y: i32,
    ) -> Result<(), Error> {
        let _ = (
            new_pos_x, new_pos_y, new_dim_x, new_dim_y, old_pos_x, old_pos_y, old_dim_x, old_dim_y,
        );
        Ok(())
    }

    /// Called while the engine renders a unit. Returning true suppresses the
    /// engine's default draw, matching Lua's `UnsyncedCallins:DrawUnit`.
    fn draw_unit(&mut self, unit_id: i32, draw_mode: i32) -> Result<bool, Error> {
        let _ = (unit_id, draw_mode);
        Ok(false)
    }

    /// Called while the engine renders a feature. Returning true suppresses
    /// the engine's default draw.
    fn draw_feature(&mut self, feature_id: i32, draw_mode: i32) -> Result<bool, Error> {
        let _ = (feature_id, draw_mode);
        Ok(false)
    }

    /// Called while the engine renders a shield. Returning true suppresses the
    /// engine's default draw.
    fn draw_shield(&mut self, unit_id: i32, weapon_id: i32, draw_mode: i32) -> Result<bool, Error> {
        let _ = (unit_id, weapon_id, draw_mode);
        Ok(false)
    }

    /// Called while the engine renders a projectile. Returning true suppresses
    /// the engine's default draw.
    fn draw_projectile(&mut self, projectile_id: i32, draw_mode: i32) -> Result<bool, Error> {
        let _ = (projectile_id, draw_mode);
        Ok(false)
    }

    /// Called while the engine renders a Lua material. Returning true
    /// suppresses the engine's default draw.
    fn draw_material(&mut self, uuid: i32, draw_mode: i32) -> Result<bool, Error> {
        let _ = (uuid, draw_mode);
        Ok(false)
    }

    fn draw_build_square(
        &mut self,
        unit_def_id: i32,
        x: i32,
        z: i32,
        facing: i32,
        statuses: &[u8],
    ) -> Result<(), Error> {
        let _ = (unit_def_id, x, z, facing, statuses);
        Ok(())
    }

    fn draw_opaque_units_lua(
        &mut self,
        deferred_pass: bool,
        draw_reflection: bool,
        draw_refraction: bool,
    ) -> Result<(), Error> {
        let _ = (deferred_pass, draw_reflection, draw_refraction);
        Ok(())
    }

    fn draw_opaque_features_lua(
        &mut self,
        deferred_pass: bool,
        draw_reflection: bool,
        draw_refraction: bool,
    ) -> Result<(), Error> {
        let _ = (deferred_pass, draw_reflection, draw_refraction);
        Ok(())
    }

    fn draw_alpha_units_lua(
        &mut self,
        draw_reflection: bool,
        draw_refraction: bool,
    ) -> Result<(), Error> {
        let _ = (draw_reflection, draw_refraction);
        Ok(())
    }

    fn draw_alpha_features_lua(
        &mut self,
        draw_reflection: bool,
        draw_refraction: bool,
    ) -> Result<(), Error> {
        let _ = (draw_reflection, draw_refraction);
        Ok(())
    }

    fn draw_shadow_units_lua(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn draw_shadow_features_lua(&mut self) -> Result<(), Error> {
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

    /// Called while a unit is reverse-built.
    fn unit_reverse_built(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    /// Called when a partially-built unit decays.
    fn unit_construction_decayed(
        &mut self,
        unit_id: i32,
        time_since_last_build: f32,
        iteration_period: f32,
        part: f32,
    ) -> Result<(), Error> {
        let _ = (unit_id, time_since_last_build, iteration_period, part);
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

    /// Called when a unit becomes idle.
    fn unit_idle(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_command(
        &mut self,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        command: crate::sys::NativeCallinCommand,
        player_num: i32,
        from_synced: bool,
        from_lua: bool,
    ) -> Result<(), Error> {
        let _ = (
            unit_id,
            unit_def_id,
            unit_team,
            command,
            player_num,
            from_synced,
            from_lua,
        );
        Ok(())
    }

    fn unit_cmd_done(
        &mut self,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        command: crate::sys::NativeCallinCommand,
    ) -> Result<(), Error> {
        let _ = (unit_id, unit_def_id, unit_team, command);
        Ok(())
    }

    fn unit_damaged(
        &mut self,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        damage: f32,
        paralyzer: bool,
        weapon_def_id: i32,
        projectile_id: i32,
        attacker_id: i32,
        attacker_def_id: i32,
        attacker_team: i32,
    ) -> Result<(), Error> {
        let _ = (
            unit_id,
            unit_def_id,
            unit_team,
            damage,
            paralyzer,
            weapon_def_id,
            projectile_id,
            attacker_id,
            attacker_def_id,
            attacker_team,
        );
        Ok(())
    }

    /// Called when a unit's harvest storage is full.
    fn unit_harvest_storage_full(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_seismic_ping(
        &mut self,
        pos: crate::sys::Float3,
        strength: f32,
        ally_team: i32,
        unit_id: i32,
        unit_def_id: i32,
    ) -> Result<(), Error> {
        let _ = (pos, strength, ally_team, unit_id, unit_def_id);
        Ok(())
    }

    fn unit_entered_radar(&mut self, unit_id: i32, ally_team: i32) -> Result<(), Error> {
        let _ = (unit_id, ally_team);
        Ok(())
    }

    fn unit_entered_los(&mut self, unit_id: i32, ally_team: i32) -> Result<(), Error> {
        let _ = (unit_id, ally_team);
        Ok(())
    }

    fn unit_left_radar(&mut self, unit_id: i32, ally_team: i32) -> Result<(), Error> {
        let _ = (unit_id, ally_team);
        Ok(())
    }

    fn unit_left_los(&mut self, unit_id: i32, ally_team: i32) -> Result<(), Error> {
        let _ = (unit_id, ally_team);
        Ok(())
    }

    fn unit_entered_underwater(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_entered_water(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_entered_air(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_left_underwater(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_left_water(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_left_air(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
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

    fn unit_cloaked(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_decloaked(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_moved(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_move_failed(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_arrived_at_goal(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn unit_unit_collision(&mut self, collider_id: i32, collidee_id: i32) -> Result<bool, Error> {
        let _ = (collider_id, collidee_id);
        Ok(false)
    }

    fn unit_feature_collision(
        &mut self,
        collider_id: i32,
        collidee_id: i32,
    ) -> Result<bool, Error> {
        let _ = (collider_id, collidee_id);
        Ok(false)
    }

    /// Called when a unit is destroyed (rendering-specific).
    fn render_unit_destroyed(&mut self, unit_id: i32) -> Result<(), Error> {
        let _ = unit_id;
        Ok(())
    }

    fn feature_moved(&mut self, feature_id: i32, old_pos: crate::sys::Float3) -> Result<(), Error> {
        let _ = (feature_id, old_pos);
        Ok(())
    }

    fn feature_damaged(
        &mut self,
        feature_id: i32,
        feature_def_id: i32,
        feature_team: i32,
        damage: f32,
        weapon_def_id: i32,
        projectile_id: i32,
        attacker_id: i32,
        attacker_def_id: i32,
        attacker_team: i32,
    ) -> Result<(), Error> {
        let _ = (
            feature_id,
            feature_def_id,
            feature_team,
            damage,
            weapon_def_id,
            projectile_id,
            attacker_id,
            attacker_def_id,
            attacker_team,
        );
        Ok(())
    }

    fn projectile_created(&mut self, projectile_id: i32) -> Result<(), Error> {
        let _ = projectile_id;
        Ok(())
    }

    fn projectile_destroyed(&mut self, projectile_id: i32) -> Result<(), Error> {
        let _ = projectile_id;
        Ok(())
    }

    fn last_message_position(&mut self, pos: crate::sys::Float3) -> Result<(), Error> {
        let _ = pos;
        Ok(())
    }

    fn view_resize(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn sun_changed(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn fonts_changed(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn game_progress(&mut self, game_frame: i32) -> Result<(), Error> {
        let _ = game_frame;
        Ok(())
    }

    fn stockpile_changed(
        &mut self,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        weapon_num: i32,
        old_count: i32,
        new_count: i32,
    ) -> Result<(), Error> {
        let _ = (
            unit_id,
            unit_def_id,
            unit_team,
            weapon_num,
            old_count,
            new_count,
        );
        Ok(())
    }

    fn unsynced_height_map_update(
        &mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
    ) -> Result<(), Error> {
        let _ = (x1, z1, x2, z2);
        Ok(())
    }

    fn camera_rotation_changed(&mut self, rot: crate::sys::Float3) -> Result<(), Error> {
        let _ = rot;
        Ok(())
    }

    fn camera_position_changed(&mut self, pos: crate::sys::Float3) -> Result<(), Error> {
        let _ = pos;
        Ok(())
    }

    fn key_map_changed(&mut self) -> Result<bool, Error> {
        Ok(false)
    }

    fn key_press(&mut self, key_code: i32, scan_code: i32, is_repeat: bool) -> Result<bool, Error> {
        let _ = (key_code, scan_code, is_repeat);
        Ok(false)
    }

    fn key_release(&mut self, key_code: i32, scan_code: i32) -> Result<bool, Error> {
        let _ = (key_code, scan_code);
        Ok(false)
    }

    fn text_input(&mut self, utf8: &str) -> Result<bool, Error> {
        let _ = utf8;
        Ok(false)
    }

    fn text_editing(&mut self, utf8: &str, start: u32, length: u32) -> Result<bool, Error> {
        let _ = (utf8, start, length);
        Ok(false)
    }

    fn mouse_move(&mut self, x: i32, y: i32, dx: i32, dy: i32, button: i32) -> Result<bool, Error> {
        let _ = (x, y, dx, dy, button);
        Ok(false)
    }

    fn mouse_press(&mut self, x: i32, y: i32, button: i32) -> Result<bool, Error> {
        let _ = (x, y, button);
        Ok(false)
    }

    fn mouse_release(&mut self, x: i32, y: i32, button: i32) -> Result<(), Error> {
        let _ = (x, y, button);
        Ok(())
    }

    fn mouse_wheel(&mut self, up: bool, value: f32) -> Result<bool, Error> {
        let _ = (up, value);
        Ok(false)
    }

    fn is_above(&mut self, x: i32, y: i32) -> Result<bool, Error> {
        let _ = (x, y);
        Ok(false)
    }

    fn active_command_changed(
        &mut self,
        cmd_id: i32,
        cmd_type: i32,
        name: &str,
        action: &str,
        tooltip: &str,
    ) -> Result<(), Error> {
        let _ = (cmd_id, cmd_type, name, action, tooltip);
        Ok(())
    }

    fn command_notify(&mut self, command: crate::sys::NativeCallinCommand) -> Result<bool, Error> {
        let _ = command;
        Ok(false)
    }

    fn add_console_line(
        &mut self,
        message: &str,
        section: &str,
        level: i32,
    ) -> Result<bool, Error> {
        let _ = (message, section, level);
        Ok(false)
    }

    fn group_changed(&mut self, group_id: i32) -> Result<bool, Error> {
        let _ = group_id;
        Ok(false)
    }

    fn default_command(
        &mut self,
        unit_id: i32,
        feature_id: i32,
        current_command: i32,
    ) -> Result<Option<i32>, Error> {
        let _ = (unit_id, feature_id, current_command);
        Ok(None)
    }

    fn map_draw_cmd(
        &mut self,
        player_id: i32,
        draw_type: i32,
        pos0: Option<crate::sys::Float3>,
        pos1: Option<crate::sys::Float3>,
        label: Option<&str>,
    ) -> Result<bool, Error> {
        let _ = (player_id, draw_type, pos0, pos1, label);
        Ok(false)
    }

    fn explosion(
        &mut self,
        weapon_def_id: i32,
        pos: crate::sys::Float3,
        owner_id: i32,
        projectile_id: i32,
    ) -> Result<bool, Error> {
        let _ = (weapon_def_id, pos, owner_id, projectile_id);
        Ok(false)
    }

    fn load(&mut self, archive: *mut std::ffi::c_void) -> Result<(), Error> {
        let _ = archive;
        Ok(())
    }

    fn save(&mut self, archive: *mut std::ffi::c_void) -> Result<(), Error> {
        let _ = archive;
        Ok(())
    }

    fn get_tooltip(&mut self, x: i32, y: i32) -> Result<Option<String>, Error> {
        let _ = (x, y);
        Ok(None)
    }

    fn world_tooltip(
        &mut self,
        kind: i32,
        unit_id: i32,
        feature_id: i32,
        ground_pos: crate::sys::Float3,
    ) -> Result<Option<String>, Error> {
        let _ = (kind, unit_id, feature_id, ground_pos);
        Ok(None)
    }

    fn game_setup(&mut self, state: &str, ready: bool) -> Result<Option<bool>, Error> {
        let _ = (state, ready);
        Ok(None)
    }

    fn collect_garbage(&mut self, forced: bool) -> Result<(), Error> {
        let _ = forced;
        Ok(())
    }

    fn pong(
        &mut self,
        ping_tag: u8,
        packet_send_time_millis: i64,
        packet_recv_time_millis: i64,
    ) -> Result<(), Error> {
        let _ = (ping_tag, packet_send_time_millis, packet_recv_time_millis);
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
        let interface =
            NativeInterfaceRef::from_ptr(interface_ptr).expect("Invalid NativeInterface pointer");

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

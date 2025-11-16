# Rust API Functions

Total APIs: 37

Total Functions: 480

---

## Camera (10 functions)

- `Camera.get_camera_direction` → `Result<sys::Float3, Error>`
- `Camera.get_camera_fov` → `Result<f32, Error>`
- `Camera.get_camera_names` → `Result<Vec<String>, Error>`
- `Camera.get_camera_position` → `Result<sys::Float3, Error>`
- `Camera.get_camera_state` → `Result<sys::CameraState, Error>`
- `Camera.get_pixel_dir` → `Result<sys::Float3, Error>`
- `Camera.set_camera_state` → `Result<bool, Error>`
- `Camera.set_camera_target` → `Result<bool, Error>`
- `Camera.trace_screen_ray` → `Result<(i32, i32, sys::Float3), Error>`
- `Camera.world_to_screen_coords` → `Result<(sys::Float3, bool), Error>`

## Config (9 functions)

- `Config.get_config_float` → `Result<f32, Error>`
- `Config.get_config_int` → `Result<i32, Error>`
- `Config.get_config_params` → `Result<Vec<sys::ConfigParam>, Error>`
- `Config.get_config_string` → `Result<Option<String>, Error>`
- `Config.get_log_sections` → `Result<Vec<String>, Error>`
- `Config.set_config_float` → `Result<bool, Error>`
- `Config.set_config_int` → `Result<bool, Error>`
- `Config.set_config_string` → `Result<bool, Error>`
- `Config.set_log_section_filter_level` → `Result<bool, Error>`

## Display (18 functions)

- `Display.get_draw_frame` → `Result<u32, Error>`
- `Display.get_fps` → `Result<u32, Error>`
- `Display.get_frame_time_offset` → `Result<f32, Error>`
- `Display.get_game_speed` → `Result<f32, Error>`
- `Display.get_last_update_seconds` → `Result<f32, Error>`
- `Display.get_mini_map_geometry` → `Result<sys::MinimapGeometry, Error>`
- `Display.get_num_displays` → `Result<u32, Error>`
- `Display.get_screen_geometry` → `Result<sys::ViewGeometry, Error>`
- `Display.get_team_color` → `Result<sys::TeamColor, Error>`
- `Display.get_team_orig_color` → `Result<sys::TeamColor, Error>`
- `Display.get_view_geometry` → `Result<sys::ViewGeometry, Error>`
- `Display.get_window_geometry` → `Result<sys::ViewGeometry, Error>`
- `Display.have_adv_shading` → `Result<bool, Error>`
- `Display.have_shadows` → `Result<bool, Error>`
- `Display.is_aabbin_view` → `Result<bool, Error>`
- `Display.is_guihidden` → `Result<bool, Error>`
- `Display.is_sphere_in_view` → `Result<bool, Error>`
- `Display.set_team_color` → `Result<bool, Error>`

## FeatureControl (8 functions)

- `FeatureControl.create_feature` → `Result<i32, Error>`
- `FeatureControl.destroy_feature` → `Result<bool, Error>`
- `FeatureControl.set_feature_direction` → `Result<bool, Error>`
- `FeatureControl.set_feature_health` → `Result<bool, Error>`
- `FeatureControl.set_feature_position` → `Result<bool, Error>`
- `FeatureControl.set_feature_resources` → `Result<bool, Error>`
- `FeatureControl.set_feature_velocity` → `Result<bool, Error>`
- `FeatureControl.transfer_feature` → `Result<bool, Error>`

## FeatureDefs (10 functions)

- `FeatureDefs.get_feature_def_by_id` → `Result<(sys::FeatureDefInfo, bool), Error>`
- `FeatureDefs.get_feature_def_count` → `Result<u32, Error>`
- `FeatureDefs.get_feature_def_custom_param` → `Result<Option<String>, Error>`
- `FeatureDefs.get_feature_def_custom_param_keys` → `Result<Vec<String>, Error>`
- `FeatureDefs.get_feature_def_energy` → `Result<f32, Error>`
- `FeatureDefs.get_feature_def_idby_name` → `Result<i32, Error>`
- `FeatureDefs.get_feature_def_ids` → `Result<Vec<i32>, Error>`
- `FeatureDefs.get_feature_def_metal` → `Result<f32, Error>`
- `FeatureDefs.get_feature_def_name` → `Result<Option<String>, Error>`
- `FeatureDefs.valid_feature_def_id` → `Result<bool, Error>`

## Features (25 functions)

- `Features.get_all_features` → `Result<Vec<i32>, Error>`
- `Features.get_feature_ally_team` → `Result<i32, Error>`
- `Features.get_feature_blocking` → `Result<sys::FeatureBlockingState, Error>`
- `Features.get_feature_collision_volume_data` → `Result<sys::CollisionVolumeData, Error>`
- `Features.get_feature_def_id` → `Result<i32, Error>`
- `Features.get_feature_direction` → `Result<sys::Float3, Error>`
- `Features.get_feature_heading` → `Result<i32, Error>`
- `Features.get_feature_health` → `Result<sys::FeatureHealth, Error>`
- `Features.get_feature_height` → `Result<f32, Error>`
- `Features.get_feature_last_attacked_piece` → `Result<i32, Error>`
- `Features.get_feature_mass` → `Result<f32, Error>`
- `Features.get_feature_no_select` → `Result<bool, Error>`
- `Features.get_feature_piece_collision_volume_data` → `Result<sys::CollisionVolumeData, Error>`
- `Features.get_feature_position` → `Result<sys::Float3, Error>`
- `Features.get_feature_radius` → `Result<f32, Error>`
- `Features.get_feature_resources` → `Result<sys::FeatureResources, Error>`
- `Features.get_feature_resurrect` → `Result<(sys::FeatureResurrect, bool), Error>`
- `Features.get_feature_rotation` → `Result<sys::FeatureRotation, Error>`
- `Features.get_feature_separation` → `Result<f32, Error>`
- `Features.get_feature_team` → `Result<i32, Error>`
- `Features.get_feature_velocity` → `Result<sys::Float3, Error>`
- `Features.get_features_in_cylinder` → `Result<Vec<i32>, Error>`
- `Features.get_features_in_rectangle` → `Result<Vec<i32>, Error>`
- `Features.get_features_in_sphere` → `Result<Vec<i32>, Error>`
- `Features.valid_feature_id` → `Result<bool, Error>`

## Game (26 functions)

- `Game.are_helper_ais_enabled` → `Result<bool, Error>`
- `Game.fixed_allies` → `Result<bool, Error>`
- `Game.get_ally_team_start_box` → `Result<(sys::StartBox, bool), Error>`
- `Game.get_facing_from_heading` → `Result<i32, Error>`
- `Game.get_gaia_team_id` → `Result<i32, Error>`
- `Game.get_game_frame` → `Result<u32, Error>`
- `Game.get_game_seconds` → `Result<f32, Error>`
- `Game.get_global_los` → `Result<i32, Error>`
- `Game.get_heading_from_facing` → `Result<i32, Error>`
- `Game.get_heading_from_vector` → `Result<i32, Error>`
- `Game.get_map_option` → `Result<(Option<String>, bool), Error>`
- `Game.get_map_options` → `Result<Vec<String>, Error>`
- `Game.get_map_start_positions` → `Result<Vec<sys::StartPosition>, Error>`
- `Game.get_mod_option` → `Result<(Option<String>, bool), Error>`
- `Game.get_mod_options` → `Result<Vec<String>, Error>`
- `Game.get_side_data` → `Result<sys::SideData, Error>`
- `Game.get_team_start_position` → `Result<sys::Float3, Error>`
- `Game.get_tidal` → `Result<f32, Error>`
- `Game.get_vector_from_heading` → `Result<sys::Float2, Error>`
- `Game.get_wind` → `Result<sys::WindData, Error>`
- `Game.is_cheating_enabled` → `Result<bool, Error>`
- `Game.is_dev_lua_enabled` → `Result<bool, Error>`
- `Game.is_edit_defs_enabled` → `Result<bool, Error>`
- `Game.is_game_over` → `Result<bool, Error>`
- `Game.is_god_mode_enabled` → `Result<bool, Error>`
- `Game.is_no_cost_enabled` → `Result<bool, Error>`

## Input (11 functions)

- `Input.get_active_command` → `Result<i32, Error>`
- `Input.get_default_command` → `Result<i32, Error>`
- `Input.get_key_state` → `Result<bool, Error>`
- `Input.get_mod_key_state` → `Result<u32, Error>`
- `Input.get_mouse_cursor` → `Result<Option<String>, Error>`
- `Input.get_mouse_start_position` → `Result<sys::Float2, Error>`
- `Input.get_mouse_state` → `Result<sys::MouseState, Error>`
- `Input.get_pressed_keys` → `Result<Vec<i32>, Error>`
- `Input.get_pressed_scans` → `Result<Vec<i32>, Error>`
- `Input.get_selection_box` → `Result<sys::SelectionBox, Error>`
- `Input.is_above_mini_map` → `Result<bool, Error>`

## Los (10 functions)

- `Los.get_closest_valid_position` → `Result<sys::Float3, Error>`
- `Los.get_position_los_state` → `Result<sys::PositionLosState, Error>`
- `Los.get_radar_error_params` → `Result<sys::RadarErrorParams, Error>`
- `Los.is_pos_in_air_los` → `Result<bool, Error>`
- `Los.is_pos_in_los` → `Result<bool, Error>`
- `Los.is_pos_in_radar` → `Result<bool, Error>`
- `Los.is_unit_in_air_los` → `Result<bool, Error>`
- `Los.is_unit_in_jammer` → `Result<bool, Error>`
- `Los.is_unit_in_los` → `Result<bool, Error>`
- `Los.is_unit_in_radar` → `Result<bool, Error>`

## MathExtra (14 functions)

- `MathExtra.bit_and` → `Result<u32, Error>`
- `MathExtra.bit_bits` → `Result<u32, Error>`
- `MathExtra.bit_inv` → `Result<u32, Error>`
- `MathExtra.bit_or` → `Result<u32, Error>`
- `MathExtra.bit_xor` → `Result<u32, Error>`
- `MathExtra.clamp` → `Result<f32, Error>`
- `MathExtra.diag` → `Result<f32, Error>`
- `MathExtra.erf` → `Result<f32, Error>`
- `MathExtra.hypot` → `Result<f32, Error>`
- `MathExtra.mix` → `Result<f32, Error>`
- `MathExtra.normalize` → `Result<f32, Error>`
- `MathExtra.round` → `Result<f32, Error>`
- `MathExtra.sgn` → `Result<f32, Error>`
- `MathExtra.smooth_step` → `Result<f32, Error>`

## Memory (9 functions)

- `Memory.free` → `Result<(), Error>`
- `Memory.free_float2_array` → `Result<(), Error>`
- `Memory.free_float3_array` → `Result<(), Error>`
- `Memory.free_float4_array` → `Result<(), Error>`
- `Memory.free_float_array` → `Result<(), Error>`
- `Memory.free_int32_array` → `Result<(), Error>`
- `Memory.free_int3_array` → `Result<(), Error>`
- `Memory.free_string_array` → `Result<(), Error>`
- `Memory.free_uint32_array` → `Result<(), Error>`

## Messages (13 functions)

- `Messages.echo` → `Result<bool, Error>`
- `Messages.get_console_buffer` → `Result<Vec<sys::ConsoleEntry>, Error>`
- `Messages.get_current_tooltip` → `Result<Option<String>, Error>`
- `Messages.is_user_writing` → `Result<bool, Error>`
- `Messages.log` → `Result<bool, Error>`
- `Messages.send_lua_gaia_msg` → `Result<bool, Error>`
- `Messages.send_lua_rules_msg` → `Result<bool, Error>`
- `Messages.send_lua_uimsg` → `Result<bool, Error>`
- `Messages.send_message` → `Result<bool, Error>`
- `Messages.send_message_to_ally_team` → `Result<bool, Error>`
- `Messages.send_message_to_player` → `Result<bool, Error>`
- `Messages.send_message_to_spectators` → `Result<bool, Error>`
- `Messages.send_message_to_team` → `Result<bool, Error>`

## MetalMap (4 functions)

- `MetalMap.get_metal_amount` → `Result<f32, Error>`
- `MetalMap.get_metal_extraction` → `Result<f32, Error>`
- `MetalMap.get_metal_map_size` → `Result<(i32, i32), Error>`
- `MetalMap.set_metal_amount` → `Result<(), Error>`

## MoveCtrl (2 functions)

- `MoveCtrl.get_unit_estimated_path` → `Result<Vec<sys::PathWaypoint>, Error>`
- `MoveCtrl.get_unit_move_type_data` → `Result<sys::MoveTypeData, Error>`

## PathFinder (10 functions)

- `PathFinder.delete_path` → `Result<bool, Error>`
- `PathFinder.free_path_node_costs_array` → `Result<bool, Error>`
- `PathFinder.get_next_way_point` → `Result<(sys::Float3, bool), Error>`
- `PathFinder.get_path_node_cost` → `Result<f32, Error>`
- `PathFinder.get_path_node_costs` → `Result<Vec<f32>, Error>`
- `PathFinder.get_path_way_points` → `Result<Vec<sys::Float3>, Error>`
- `PathFinder.init_path_node_costs_array` → `Result<bool, Error>`
- `PathFinder.request_path` → `Result<u32, Error>`
- `PathFinder.set_path_node_cost` → `Result<bool, Error>`
- `PathFinder.set_path_node_costs` → `Result<bool, Error>`

## Player (7 functions)

- `Player.get_local_ally_team_id` → `Result<i32, Error>`
- `Player.get_local_player_id` → `Result<i32, Error>`
- `Player.get_local_team_id` → `Result<i32, Error>`
- `Player.get_player_roster` → `Result<Vec<sys::RosterEntry>, Error>`
- `Player.get_player_statistics` → `Result<sys::PlayerStats, Error>`
- `Player.get_player_traffic` → `Result<Vec<sys::PlayerTraffic>, Error>`
- `Player.get_spectating_state` → `Result<bool, Error>`

## ProjectileControl (6 functions)

- `ProjectileControl.delete_projectile` → `Result<bool, Error>`
- `ProjectileControl.set_projectile_gravity` → `Result<bool, Error>`
- `ProjectileControl.set_projectile_position` → `Result<bool, Error>`
- `ProjectileControl.set_projectile_target` → `Result<bool, Error>`
- `ProjectileControl.set_projectile_velocity` → `Result<bool, Error>`
- `ProjectileControl.spawn_projectile` → `Result<i32, Error>`

## Projectiles (16 functions)

- `Projectiles.get_piece_projectile_params` → `Result<(sys::PieceProjectileParams, bool), Error>`
- `Projectiles.get_projectile_ally_team_id` → `Result<i32, Error>`
- `Projectiles.get_projectile_damages` → `Result<sys::ProjectileDamages, Error>`
- `Projectiles.get_projectile_def_id` → `Result<i32, Error>`
- `Projectiles.get_projectile_direction` → `Result<sys::Float3, Error>`
- `Projectiles.get_projectile_gravity` → `Result<sys::Float3, Error>`
- `Projectiles.get_projectile_is_intercepted` → `Result<bool, Error>`
- `Projectiles.get_projectile_owner_id` → `Result<i32, Error>`
- `Projectiles.get_projectile_position` → `Result<sys::Float3, Error>`
- `Projectiles.get_projectile_target` → `Result<sys::ProjectileTarget, Error>`
- `Projectiles.get_projectile_team_id` → `Result<i32, Error>`
- `Projectiles.get_projectile_time_to_live` → `Result<f32, Error>`
- `Projectiles.get_projectile_type` → `Result<u32, Error>`
- `Projectiles.get_projectile_velocity` → `Result<sys::Float3, Error>`
- `Projectiles.get_projectiles_in_rectangle` → `Result<Vec<i32>, Error>`
- `Projectiles.get_projectiles_in_sphere` → `Result<Vec<i32>, Error>`

## RulesParams (15 functions)

- `RulesParams.get_feature_rules_param` → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_feature_rules_params` → `Result<Vec<String>, Error>`
- `RulesParams.get_game_rules_param` → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_game_rules_params` → `Result<Vec<String>, Error>`
- `RulesParams.get_player_rules_param` → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_player_rules_params` → `Result<Vec<String>, Error>`
- `RulesParams.get_team_rules_param` → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_team_rules_params` → `Result<Vec<String>, Error>`
- `RulesParams.get_unit_rules_param` → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_unit_rules_params` → `Result<Vec<String>, Error>`
- `RulesParams.set_feature_rules_param` → `Result<bool, Error>`
- `RulesParams.set_game_rules_param` → `Result<bool, Error>`
- `RulesParams.set_player_rules_param` → `Result<bool, Error>`
- `RulesParams.set_team_rules_param` → `Result<bool, Error>`
- `RulesParams.set_unit_rules_param` → `Result<bool, Error>`

## Selection (13 functions)

- `Selection.deselect_unit` → `Result<bool, Error>`
- `Selection.deselect_unit_array` → `Result<bool, Error>`
- `Selection.get_group_list` → `Result<Vec<i32>, Error>`
- `Selection.get_group_units` → `Result<Vec<i32>, Error>`
- `Selection.get_selected_group` → `Result<i32, Error>`
- `Selection.get_selected_units` → `Result<Vec<i32>, Error>`
- `Selection.get_selected_units_count` → `Result<u32, Error>`
- `Selection.get_selected_units_counts` → `Result<sys::SelectionCounts, Error>`
- `Selection.get_selected_units_sorted` → `Result<Vec<i32>, Error>`
- `Selection.get_unit_group` → `Result<i32, Error>`
- `Selection.select_unit` → `Result<bool, Error>`
- `Selection.select_unit_array` → `Result<bool, Error>`
- `Selection.set_unit_group` → `Result<bool, Error>`

## Sound (7 functions)

- `Sound.get_sound_stream_time` → `Result<f32, Error>`
- `Sound.load_sound_def` → `Result<bool, Error>`
- `Sound.pause_sound_stream` → `Result<bool, Error>`
- `Sound.play_sound_file` → `Result<bool, Error>`
- `Sound.play_sound_stream` → `Result<bool, Error>`
- `Sound.set_sound_stream_volume` → `Result<bool, Error>`
- `Sound.stop_sound_stream` → `Result<bool, Error>`

## SyncedCtrl (5 functions)

- `SyncedCtrl.feature` → `FeatureControl<'_>`
- `SyncedCtrl.projectile` → `ProjectileControl<'_>`
- `SyncedCtrl.team` → `TeamControl<'_>`
- `SyncedCtrl.terrain` → `TerrainControl<'_>`
- `SyncedCtrl.unit` → `UnitControl<'_>`

## TeamControl (11 functions)

- `TeamControl.add_team_resource` → `Result<bool, Error>`
- `TeamControl.assign_player_to_team` → `Result<bool, Error>`
- `TeamControl.game_over` → `Result<bool, Error>`
- `TeamControl.kill_team` → `Result<bool, Error>`
- `TeamControl.set_ally` → `Result<bool, Error>`
- `TeamControl.set_ally_team_start_box` → `Result<bool, Error>`
- `TeamControl.set_global_los` → `Result<bool, Error>`
- `TeamControl.set_team_resource` → `Result<bool, Error>`
- `TeamControl.set_team_share_level` → `Result<bool, Error>`
- `TeamControl.share_team_resource` → `Result<bool, Error>`
- `TeamControl.use_team_resource` → `Result<bool, Error>`

## Teams (19 functions)

- `Teams.are_players_allied` → `Result<bool, Error>`
- `Teams.are_teams_allied` → `Result<bool, Error>`
- `Teams.get_aiinfo` → `Result<(sys::AIInfo, bool), Error>`
- `Teams.get_ally_team_info` → `Result<sys::AllyTeamInfo, Error>`
- `Teams.get_ally_team_list` → `Result<Vec<i32>, Error>`
- `Teams.get_player_controlled_unit` → `Result<i32, Error>`
- `Teams.get_player_info` → `Result<sys::PlayerInfo, Error>`
- `Teams.get_player_list` → `Result<Vec<i32>, Error>`
- `Teams.get_player_list_in_ally_team` → `Result<Vec<i32>, Error>`
- `Teams.get_player_list_in_team` → `Result<Vec<i32>, Error>`
- `Teams.get_team_ally_team_id` → `Result<i32, Error>`
- `Teams.get_team_info` → `Result<sys::TeamInfo, Error>`
- `Teams.get_team_list` → `Result<Vec<i32>, Error>`
- `Teams.get_team_lua_ai` → `Result<Option<String>, Error>`
- `Teams.get_team_max_units` → `Result<i32, Error>`
- `Teams.get_team_resource_stats` → `Result<sys::TeamResources, Error>`
- `Teams.get_team_resources` → `Result<sys::TeamResources, Error>`
- `Teams.get_team_stats_history` → `Result<Vec<sys::TeamStatsHistoryPoint>, Error>`
- `Teams.get_team_unit_stats` → `Result<sys::TeamUnitStats, Error>`

## Terrain (12 functions)

- `Terrain.get_grass` → `Result<f32, Error>`
- `Terrain.get_ground_blocked` → `Result<bool, Error>`
- `Terrain.get_ground_extremes` → `Result<(f32, f32, f32, f32), Error>`
- `Terrain.get_ground_height` → `Result<f32, Error>`
- `Terrain.get_ground_info` → `Result<(i32, Option<String>, f32, f32, f32, f32, f32, f32, bool), Error>`
- `Terrain.get_ground_normal` → `Result<(sys::Float3, f32), Error>`
- `Terrain.get_ground_orig_height` → `Result<f32, Error>`
- `Terrain.get_smooth_mesh_height` → `Result<f32, Error>`
- `Terrain.get_terrain_type_data` → `Result<(i32, Option<String>, f32, f32, f32, f32, f32, bool), Error>`
- `Terrain.get_water_level` → `Result<f32, Error>`
- `Terrain.get_water_plane_level` → `Result<f32, Error>`
- `Terrain.is_pos_in_map` → `Result<(bool, bool), Error>`

## TerrainControl (10 functions)

- `TerrainControl.add_height_map` → `Result<bool, Error>`
- `TerrainControl.add_smooth_mesh` → `Result<bool, Error>`
- `TerrainControl.revert_height_map` → `Result<bool, Error>`
- `TerrainControl.revert_smooth_mesh` → `Result<bool, Error>`
- `TerrainControl.set_height_map` → `Result<bool, Error>`
- `TerrainControl.set_map_square_terrain_type` → `Result<bool, Error>`
- `TerrainControl.set_smooth_mesh` → `Result<bool, Error>`
- `TerrainControl.set_terrain_type_data` → `Result<bool, Error>`
- `TerrainControl.set_tidal` → `Result<bool, Error>`
- `TerrainControl.set_wind` → `Result<bool, Error>`

## Tracing (5 functions)

- `Tracing.trace_ray` → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_features` → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_ground_between_positions` → `Result<(bool, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_ground_in_direction` → `Result<(bool, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_units` → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`

## UnitControl (19 functions)

- `UnitControl.add_unit_damage` → `Result<bool, Error>`
- `UnitControl.add_unit_experience` → `Result<bool, Error>`
- `UnitControl.add_unit_impulse` → `Result<bool, Error>`
- `UnitControl.create_unit` → `Result<i32, Error>`
- `UnitControl.destroy_unit` → `Result<bool, Error>`
- `UnitControl.give_order_to_unit` → `Result<bool, Error>`
- `UnitControl.give_order_to_unit_array` → `Result<bool, Error>`
- `UnitControl.set_unit_experience` → `Result<bool, Error>`
- `UnitControl.set_unit_health` → `Result<bool, Error>`
- `UnitControl.set_unit_max_health` → `Result<bool, Error>`
- `UnitControl.set_unit_metal_extraction` → `Result<bool, Error>`
- `UnitControl.set_unit_neutral` → `Result<bool, Error>`
- `UnitControl.set_unit_physics` → `Result<bool, Error>`
- `UnitControl.set_unit_position` → `Result<bool, Error>`
- `UnitControl.set_unit_resourcing` → `Result<bool, Error>`
- `UnitControl.set_unit_rotation` → `Result<bool, Error>`
- `UnitControl.set_unit_velocity` → `Result<bool, Error>`
- `UnitControl.transfer_unit` → `Result<bool, Error>`
- `UnitControl.unit_finish_command` → `Result<bool, Error>`

## UnitDefs (12 functions)

- `UnitDefs.get_unit_def_by_id` → `Result<(bool, sys::UnitDefBasicInfo, sys::UnitDefCosts, sys::UnitDefPhysics, sys::UnitDefWeapons, sys::UnitDefBuildOptions, sys::UnitDefSensors, sys::UnitDefHealth), Error>`
- `UnitDefs.get_unit_def_costs` → `Result<sys::UnitDefCosts, Error>`
- `UnitDefs.get_unit_def_count` → `Result<u32, Error>`
- `UnitDefs.get_unit_def_custom_param` → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_custom_param_keys` → `Result<Vec<String>, Error>`
- `UnitDefs.get_unit_def_health` → `Result<f32, Error>`
- `UnitDefs.get_unit_def_human_name` → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_idby_name` → `Result<i32, Error>`
- `UnitDefs.get_unit_def_ids` → `Result<Vec<i32>, Error>`
- `UnitDefs.get_unit_def_name` → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_speed` → `Result<f32, Error>`
- `UnitDefs.valid_unit_def_id` → `Result<bool, Error>`

## UnitsCommands (12 functions)

- `UnitsCommands.find_unit_cmd_desc` → `Result<(sys::CommandDescription, bool), Error>`
- `UnitsCommands.get_command_queue` → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_factory_bugger_off` → `Result<(bool, sys::Float3, f32), Error>`
- `UnitsCommands.get_factory_command_count` → `Result<u32, Error>`
- `UnitsCommands.get_factory_commands` → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_factory_counts` → `Result<sys::FactoryQueueInfo, Error>`
- `UnitsCommands.get_full_build_queue` → `Result<Vec<sys::BuildQueueEntry>, Error>`
- `UnitsCommands.get_real_build_queue` → `Result<Vec<i32>, Error>`
- `UnitsCommands.get_unit_cmd_descs` → `Result<Vec<sys::CommandDescription>, Error>`
- `UnitsCommands.get_unit_command_count` → `Result<u32, Error>`
- `UnitsCommands.get_unit_commands` → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_unit_current_command` → `Result<(sys::CommandFFI, bool), Error>`

## UnitsInfo (56 functions)

- `UnitsInfo.get_unit_ally_team` → `Result<i32, Error>`
- `UnitsInfo.get_unit_armored` → `Result<sys::UnitArmoredState, Error>`
- `UnitsInfo.get_unit_base_position` → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_blocking` → `Result<sys::UnitBlockingState, Error>`
- `UnitsInfo.get_unit_build_facing` → `Result<i32, Error>`
- `UnitsInfo.get_unit_build_params` → `Result<sys::UnitBuildParams, Error>`
- `UnitsInfo.get_unit_buildee_radius` → `Result<f32, Error>`
- `UnitsInfo.get_unit_collision_volume_data` → `Result<sys::CollisionVolumeData, Error>`
- `UnitsInfo.get_unit_cost_table` → `Result<sys::UnitCosts, Error>`
- `UnitsInfo.get_unit_costs` → `Result<sys::UnitCosts, Error>`
- `UnitsInfo.get_unit_current_build_power` → `Result<f32, Error>`
- `UnitsInfo.get_unit_def_id` → `Result<i32, Error>`
- `UnitsInfo.get_unit_direction` → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_effective_build_range` → `Result<f32, Error>`
- `UnitsInfo.get_unit_experience` → `Result<f32, Error>`
- `UnitsInfo.get_unit_flanking` → `Result<sys::UnitFlanking, Error>`
- `UnitsInfo.get_unit_fuel` → `Result<sys::UnitFuel, Error>`
- `UnitsInfo.get_unit_harvest_storage` → `Result<f32, Error>`
- `UnitsInfo.get_unit_heading` → `Result<i32, Error>`
- `UnitsInfo.get_unit_health` → `Result<sys::UnitHealth, Error>`
- `UnitsInfo.get_unit_height` → `Result<f32, Error>`
- `UnitsInfo.get_unit_in_build_stance` → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_active` → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_being_built` → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_building` → `Result<i32, Error>`
- `UnitsInfo.get_unit_is_cloaked` → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_dead` → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_stunned` → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_transporting` → `Result<bool, Error>`
- `UnitsInfo.get_unit_last_attacked_piece` → `Result<i32, Error>`
- `UnitsInfo.get_unit_last_attacker` → `Result<(sys::UnitLastAttacker, bool), Error>`
- `UnitsInfo.get_unit_los_state` → `Result<sys::UnitLosState, Error>`
- `UnitsInfo.get_unit_mass` → `Result<f32, Error>`
- `UnitsInfo.get_unit_metal_extraction` → `Result<f32, Error>`
- `UnitsInfo.get_unit_nano_pieces` → `Result<Vec<i32>, Error>`
- `UnitsInfo.get_unit_neutral` → `Result<bool, Error>`
- `UnitsInfo.get_unit_piece_collision_volume_data` → `Result<sys::CollisionVolumeData, Error>`
- `UnitsInfo.get_unit_pos_error_params` → `Result<sys::UnitPosErrorParams, Error>`
- `UnitsInfo.get_unit_position` → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_radius` → `Result<f32, Error>`
- `UnitsInfo.get_unit_resources` → `Result<sys::UnitResources, Error>`
- `UnitsInfo.get_unit_rotation` → `Result<sys::UnitRotation, Error>`
- `UnitsInfo.get_unit_seismic_signature` → `Result<f32, Error>`
- `UnitsInfo.get_unit_self_dtime` → `Result<f32, Error>`
- `UnitsInfo.get_unit_sensor_radius` → `Result<sys::UnitSensorRadius, Error>`
- `UnitsInfo.get_unit_shield_state` → `Result<(sys::UnitShieldState, bool), Error>`
- `UnitsInfo.get_unit_states` → `Result<sys::UnitStates, Error>`
- `UnitsInfo.get_unit_stockpile` → `Result<sys::UnitStockpile, Error>`
- `UnitsInfo.get_unit_storage` → `Result<sys::UnitStorage, Error>`
- `UnitsInfo.get_unit_team` → `Result<i32, Error>`
- `UnitsInfo.get_unit_tooltip` → `Result<Option<String>, Error>`
- `UnitsInfo.get_unit_transporter` → `Result<i32, Error>`
- `UnitsInfo.get_unit_travel` → `Result<sys::UnitTravel, Error>`
- `UnitsInfo.get_unit_vectors` → `Result<sys::UnitVectors, Error>`
- `UnitsInfo.get_unit_velocity` → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_worker_task` → `Result<Option<String>, Error>`

## UnitsPieces (21 functions)

- `UnitsPieces.get_feature_piece_direction` → `Result<sys::Float3, Error>`
- `UnitsPieces.get_feature_piece_info` → `Result<(sys::PieceInfo, bool), Error>`
- `UnitsPieces.get_feature_piece_list` → `Result<Vec<i32>, Error>`
- `UnitsPieces.get_feature_piece_map` → `Result<Vec<String>, Error>`
- `UnitsPieces.get_feature_piece_matrix` → `Result<sys::PieceMatrix, Error>`
- `UnitsPieces.get_feature_piece_pos_dir` → `Result<sys::PiecePosDir, Error>`
- `UnitsPieces.get_feature_piece_position` → `Result<sys::Float3, Error>`
- `UnitsPieces.get_feature_root_piece` → `Result<i32, Error>`
- `UnitsPieces.get_model_piece_list` → `Result<Vec<i32>, Error>`
- `UnitsPieces.get_model_piece_map` → `Result<Vec<String>, Error>`
- `UnitsPieces.get_model_root_piece` → `Result<i32, Error>`
- `UnitsPieces.get_unit_piece_direction` → `Result<sys::Float3, Error>`
- `UnitsPieces.get_unit_piece_info` → `Result<(sys::PieceInfo, bool), Error>`
- `UnitsPieces.get_unit_piece_list` → `Result<Vec<i32>, Error>`
- `UnitsPieces.get_unit_piece_map` → `Result<Vec<String>, Error>`
- `UnitsPieces.get_unit_piece_matrix` → `Result<sys::PieceMatrix, Error>`
- `UnitsPieces.get_unit_piece_pos_dir` → `Result<sys::PiecePosDir, Error>`
- `UnitsPieces.get_unit_piece_position` → `Result<sys::Float3, Error>`
- `UnitsPieces.get_unit_root_piece` → `Result<i32, Error>`
- `UnitsPieces.get_unit_script_names` → `Result<Vec<String>, Error>`
- `UnitsPieces.get_unit_script_piece` → `Result<i32, Error>`

## UnitsQuery (18 functions)

- `UnitsQuery.get_all_units` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_team_unit_count` → `Result<u32, Error>`
- `UnitsQuery.get_team_unit_def_count` → `Result<u32, Error>`
- `UnitsQuery.get_team_units` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_team_units_by_defs` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_team_units_counts` → `Result<Vec<sys::UnitDefCount>, Error>`
- `UnitsQuery.get_team_units_sorted` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_unit_array_centroid` → `Result<sys::Float3, Error>`
- `UnitsQuery.get_unit_map_centroid` → `Result<sys::Float3, Error>`
- `UnitsQuery.get_unit_nearest_ally` → `Result<i32, Error>`
- `UnitsQuery.get_unit_nearest_enemy` → `Result<i32, Error>`
- `UnitsQuery.get_unit_separation` → `Result<f32, Error>`
- `UnitsQuery.get_units_in_box` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_cylinder` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_planes` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_rectangle` → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_sphere` → `Result<Vec<i32>, Error>`
- `UnitsQuery.valid_unit_id` → `Result<bool, Error>`

## UnitsWeapons (11 functions)

- `UnitsWeapons.get_unit_max_range` → `Result<f32, Error>`
- `UnitsWeapons.get_unit_weapon_can_fire` → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_count` → `Result<u32, Error>`
- `UnitsWeapons.get_unit_weapon_damages` → `Result<sys::UnitWeaponDamages, Error>`
- `UnitsWeapons.get_unit_weapon_have_free_line_of_fire` → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_state` → `Result<sys::UnitWeaponState, Error>`
- `UnitsWeapons.get_unit_weapon_target` → `Result<sys::UnitWeaponTarget, Error>`
- `UnitsWeapons.get_unit_weapon_test_range` → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_test_target` → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_try_target` → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_vectors` → `Result<sys::UnitWeaponVectors, Error>`

## Utils (6 functions)

- `Utils.closest_build_pos` → `Result<sys::Float3, Error>`
- `Utils.get_cegid` → `Result<i32, Error>`
- `Utils.get_unit_def_dimensions` → `Result<sys::Float3, Error>`
- `Utils.pos2_build_pos` → `Result<sys::Float3, Error>`
- `Utils.test_build_order` → `Result<(bool, i32), Error>`
- `Utils.test_move_order` → `Result<bool, Error>`

## Vfs (10 functions)

- `Vfs.file_exists` → `Result<bool, Error>`
- `Vfs.get_archives` → `Result<Vec<String>, Error>`
- `Vfs.get_file_info` → `Result<(sys::FileInfo, bool), Error>`
- `Vfs.get_file_size` → `Result<u32, Error>`
- `Vfs.get_games` → `Result<Vec<String>, Error>`
- `Vfs.get_maps` → `Result<Vec<String>, Error>`
- `Vfs.is_directory` → `Result<bool, Error>`
- `Vfs.list_dir` → `Result<Vec<sys::DirEntry>, Error>`
- `Vfs.read_file` → `Result<Vec<u8>, Error>`
- `Vfs.read_file_as_string` → `Result<Option<String>, Error>`

## WeaponDefs (10 functions)

- `WeaponDefs.get_weapon_def_by_id` → `Result<(sys::WeaponDefInfo, bool), Error>`
- `WeaponDefs.get_weapon_def_count` → `Result<u32, Error>`
- `WeaponDefs.get_weapon_def_custom_param` → `Result<Option<String>, Error>`
- `WeaponDefs.get_weapon_def_custom_param_keys` → `Result<Vec<String>, Error>`
- `WeaponDefs.get_weapon_def_damage` → `Result<f32, Error>`
- `WeaponDefs.get_weapon_def_id` → `Result<i32, Error>`
- `WeaponDefs.get_weapon_def_ids` → `Result<Vec<i32>, Error>`
- `WeaponDefs.get_weapon_def_name` → `Result<Option<String>, Error>`
- `WeaponDefs.get_weapon_def_range` → `Result<f32, Error>`
- `WeaponDefs.valid_weapon_def_id` → `Result<bool, Error>`


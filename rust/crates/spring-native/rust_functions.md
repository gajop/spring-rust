# Rust API Functions

Total APIs: 49

Total Functions: 878

---

## Camera (10 functions)

- `Camera.get_camera_direction` (params: ) → `Result<sys::Float3, Error>`
- `Camera.get_camera_fov` (params: ) → `Result<f32, Error>`
- `Camera.get_camera_names` (params: ) → `Result<Vec<String>, Error>`
- `Camera.get_camera_position` (params: ) → `Result<sys::Float3, Error>`
- `Camera.get_camera_state` (params: use_table:bool) → `Result<sys::CameraState, Error>`
- `Camera.get_pixel_dir` (params: screen_x:f32, screen_y:f32) → `Result<sys::Float3, Error>`
- `Camera.set_camera_state` (params: state:sys::CameraState, transition_time:f32, transition_time_factor:f32, transition_time_exponent:f32) → `Result<bool, Error>`
- `Camera.set_camera_target` (params: target:sys::Float3, transition_time:f32) → `Result<bool, Error>`
- `Camera.trace_screen_ray` (params: screen_x:f32, screen_y:f32, only_coords:bool, use_minimap:bool, include_sky:bool, ignore_water:bool, height_offset:f32) → `Result<(i32, i32, sys::Float3), Error>`
- `Camera.world_to_screen_coords` (params: world_pos:sys::Float3) → `Result<(sys::Float3, bool), Error>`

## CobScript (2 functions)

- `CobScript.call_cobscript` (params: unit_id:i32, func:sys::CobFunctionRef, ret_args:u32, args:&[i32]) → `Result<(i32, Vec<i32>), Error>`
- `CobScript.get_cobscript_id` (params: unit_id:i32, func_name:&str) → `Result<i32, Error>`

## Config (9 functions)

- `Config.get_config_float` (params: key:&str, default_value:f32) → `Result<f32, Error>`
- `Config.get_config_int` (params: key:&str, default_value:i32) → `Result<i32, Error>`
- `Config.get_config_params` (params: ) → `Result<Vec<sys::ConfigParam>, Error>`
- `Config.get_config_string` (params: key:&str, default_value:&str) → `Result<Option<String>, Error>`
- `Config.get_log_sections` (params: ) → `Result<Vec<String>, Error>`
- `Config.set_config_float` (params: key:&str, value:f32, use_overlay:bool) → `Result<bool, Error>`
- `Config.set_config_int` (params: key:&str, value:i32, use_overlay:bool) → `Result<bool, Error>`
- `Config.set_config_string` (params: key:&str, value:&str, use_overlay:bool) → `Result<bool, Error>`
- `Config.set_log_section_filter_level` (params: section:&str, level:i32) → `Result<bool, Error>`

## Display (24 functions)

- `Display.get_draw_frame` (params: ) → `Result<(u32, u32), Error>`
- `Display.get_dual_view_geometry` (params: ) → `Result<sys::ViewGeometry, Error>`
- `Display.get_fps` (params: ) → `Result<u32, Error>`
- `Display.get_frame_time_offset` (params: ) → `Result<f32, Error>`
- `Display.get_game_speed` (params: ) → `Result<(f32, f32, bool), Error>`
- `Display.get_last_update_seconds` (params: ) → `Result<f32, Error>`
- `Display.get_los_view_colors` (params: ) → `Result<(sys::Float3, sys::Float3, sys::Float3, sys::Float3, sys::Float3), Error>`
- `Display.get_map_draw_mode` (params: ) → `Result<Option<String>, Error>`
- `Display.get_mini_map_dual_screen` (params: ) → `Result<(Option<String>, bool), Error>`
- `Display.get_mini_map_geometry` (params: ) → `Result<sys::MinimapGeometry, Error>`
- `Display.get_mini_map_rotation` (params: ) → `Result<f32, Error>`
- `Display.get_num_displays` (params: ) → `Result<u32, Error>`
- `Display.get_screen_geometry` (params: screen_num:i32, query_usable:bool) → `Result<sys::ViewGeometry, Error>`
- `Display.get_team_color` (params: team_id:i32) → `Result<sys::TeamColor, Error>`
- `Display.get_team_orig_color` (params: team_id:i32) → `Result<sys::TeamColor, Error>`
- `Display.get_view_geometry` (params: ) → `Result<sys::ViewGeometry, Error>`
- `Display.get_water_mode` (params: ) → `Result<(i32, Option<String>), Error>`
- `Display.get_window_geometry` (params: ) → `Result<sys::ViewGeometry, Error>`
- `Display.have_adv_shading` (params: ) → `Result<bool, Error>`
- `Display.have_shadows` (params: ) → `Result<bool, Error>`
- `Display.is_aabbin_view` (params: mins:sys::Float3, maxs:sys::Float3) → `Result<bool, Error>`
- `Display.is_guihidden` (params: ) → `Result<bool, Error>`
- `Display.is_sphere_in_view` (params: center:sys::Float3, radius:f32) → `Result<bool, Error>`
- `Display.set_team_color` (params: team_id:i32, color:sys::TeamColor) → `Result<bool, Error>`

## EffectsControl (3 functions)

- `EffectsControl.spawn_ceg` (params: ceg:sys::DefRef, pos:sys::Float3, dir:sys::Float3, radius:f32, damage:f32, dmg_mod:f32) → `Result<(bool, i32), Error>`
- `EffectsControl.spawn_explosion` (params: pos:sys::Float3, dir:sys::Float3, explosion_params:sys::NativeExplosionParams) → `Result<bool, Error>`
- `EffectsControl.spawn_sfx` (params: unit_id:i32, sfx_id:i32, pos:sys::Float3, dir:sys::Float3, radius:f32, damage:f32, absolute:bool) → `Result<bool, Error>`

## FeatureControl (32 functions)

- `FeatureControl.add_feature_damage` (params: feature_id:i32, damage:f32, paralyze_time:f32, weapon_def_id:i32, attacker_id:i32, impulse:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.create_feature` (params: feature_def:sys::DefRef, pos:sys::Float3, facing:i32, team_id:i32, feature_id:i32) → `Result<i32, Error>`
- `FeatureControl.create_feature_wreck` (params: feature_id:i32, wreck_level:i32, do_smoke:bool) → `Result<i32, Error>`
- `FeatureControl.create_unit_wreck` (params: unit_id:i32, wreck_level:i32, do_smoke:bool) → `Result<i32, Error>`
- `FeatureControl.destroy_feature` (params: feature_id:i32) → `Result<bool, Error>`
- `FeatureControl.set_feature_always_visible` (params: feature_id:i32, always_visible:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_blocking` (params: feature_id:i32, blocking:bool, solid_objects:bool, projectiles:bool, quad_map_rays:bool, crushable:bool, block_enemy_pushing:bool, block_height_changes:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_collision_volume_data` (params: feature_id:i32, scales:sys::Float3, offsets:sys::Float3, volume_type:i32, test_type:i32, primary_axis:i32) → `Result<bool, Error>`
- `FeatureControl.set_feature_direction` (params: feature_id:i32, front_dir:sys::Float3, right_dir:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.set_feature_fire_time` (params: feature_id:i32, fire_time:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_heading_and_up_dir` (params: feature_id:i32, heading:i32, up_dir:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.set_feature_health` (params: feature_id:i32, health:f32, check_destruction:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_mass` (params: feature_id:i32, mass:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_max_health` (params: feature_id:i32, max_health:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_mid_and_aim_pos` (params: feature_id:i32, mid_pos:sys::Float3, aim_pos:sys::Float3, set_relative:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_move_ctrl` (params: feature_id:i32, enable:bool, velocity_or_mask:sys::Float3, acceleration_or_impulse_mask:sys::Float3, movement_mask:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.set_feature_no_select` (params: feature_id:i32, no_select:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_physics` (params: feature_id:i32, pos:sys::Float3, velocity:sys::Float3, rotation:sys::Float3, drag:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.set_feature_piece_collision_volume_data` (params: feature_id:i32, piece_index:i32, enable:bool, scales:sys::Float3, offsets:sys::Float3, volume_type:i32, primary_axis:i32) → `Result<bool, Error>`
- `FeatureControl.set_feature_piece_matrix` (params: feature_id:i32, piece_index:i32, matrix:[f32; 16]) → `Result<bool, Error>`
- `FeatureControl.set_feature_piece_visible` (params: feature_id:i32, piece_index:i32, visible:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_position` (params: feature_id:i32, pos:sys::Float3, snap_to_ground:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_radius_and_height` (params: feature_id:i32, radius:f32, height:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_reclaim` (params: feature_id:i32, reclaim_left:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_resources` (params: feature_id:i32, metal:f32, energy:f32, reclaim_time:f32, reclaim_left:f32, feature_def_metal:f32, feature_def_energy:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_resurrect` (params: feature_id:i32, unit_def:sys::DefRef, facing:i32, progress:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_rotation` (params: feature_id:i32, rotation:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.set_feature_selection_volume_data` (params: feature_id:i32, scales:sys::Float3, offsets:sys::Float3, volume_type:i32, primary_axis:i32, use_cont_hit_test:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_smoke_time` (params: feature_id:i32, smoke_time:f32) → `Result<bool, Error>`
- `FeatureControl.set_feature_use_air_los` (params: feature_id:i32, use_air_los:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_velocity` (params: feature_id:i32, velocity:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.transfer_feature` (params: feature_id:i32, new_team_id:i32) → `Result<bool, Error>`

## FeatureDefs (10 functions)

- `FeatureDefs.get_feature_def_by_id` (params: feature_def_id:i32) → `Result<(sys::FeatureDefInfo, bool), Error>`
- `FeatureDefs.get_feature_def_count` (params: ) → `Result<u32, Error>`
- `FeatureDefs.get_feature_def_custom_param` (params: feature_def_id:i32, key:&str) → `Result<Option<String>, Error>`
- `FeatureDefs.get_feature_def_custom_param_keys` (params: feature_def_id:i32) → `Result<Vec<String>, Error>`
- `FeatureDefs.get_feature_def_energy` (params: feature_def_id:i32) → `Result<f32, Error>`
- `FeatureDefs.get_feature_def_idby_name` (params: feature_def_name:&str) → `Result<i32, Error>`
- `FeatureDefs.get_feature_def_ids` (params: ) → `Result<Vec<i32>, Error>`
- `FeatureDefs.get_feature_def_metal` (params: feature_def_id:i32) → `Result<f32, Error>`
- `FeatureDefs.get_feature_def_name` (params: feature_def_id:i32) → `Result<Option<String>, Error>`
- `FeatureDefs.valid_feature_def_id` (params: feature_def_id:i32) → `Result<bool, Error>`

## Features (37 functions)

- `Features.clear_features_previous_draw_flag` (params: ) → `Result<bool, Error>`
- `Features.get_all_features` (params: ) → `Result<Vec<i32>, Error>`
- `Features.get_feature_ally_team` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_always_update_matrix` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_blocking` (params: feature_id:i32) → `Result<sys::FeatureBlockingState, Error>`
- `Features.get_feature_collision_volume_data` (params: feature_id:i32) → `Result<sys::CollisionVolumeData, Error>`
- `Features.get_feature_def_id` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_direction` (params: feature_id:i32) → `Result<sys::Float3, Error>`
- `Features.get_feature_draw_flag` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_engine_draw_mask` (params: feature_id:i32) → `Result<u32, Error>`
- `Features.get_feature_fire_time` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_heading` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_health` (params: feature_id:i32) → `Result<sys::FeatureHealth, Error>`
- `Features.get_feature_height` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_last_attacked_piece` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_lua_draw` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_mass` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_no_draw` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_no_select` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_piece_collision_volume_data` (params: feature_id:i32) → `Result<sys::CollisionVolumeData, Error>`
- `Features.get_feature_position` (params: feature_id:i32) → `Result<sys::Float3, Error>`
- `Features.get_feature_radius` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_resources` (params: feature_id:i32) → `Result<sys::FeatureResources, Error>`
- `Features.get_feature_resurrect` (params: feature_id:i32) → `Result<(sys::FeatureResurrect, bool), Error>`
- `Features.get_feature_rotation` (params: feature_id:i32) → `Result<sys::FeatureRotation, Error>`
- `Features.get_feature_selection_volume_data` (params: feature_id:i32) → `Result<sys::FeatureSelectionVolumeData, Error>`
- `Features.get_feature_separation` (params: feature_id1:i32, feature_id2:i32, positional:bool) → `Result<f32, Error>`
- `Features.get_feature_smoke_time` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_team` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_transform_matrix` (params: feature_id:i32) → `Result<sys::FeatureTransformMatrix, Error>`
- `Features.get_feature_velocity` (params: feature_id:i32) → `Result<sys::Float3, Error>`
- `Features.get_features_in_cylinder` (params: x:f32, z:f32, radius:f32, height:f32) → `Result<Vec<i32>, Error>`
- `Features.get_features_in_rectangle` (params: min_x:f32, min_z:f32, max_x:f32, max_z:f32) → `Result<Vec<i32>, Error>`
- `Features.get_features_in_sphere` (params: center:sys::Float3, radius:f32) → `Result<Vec<i32>, Error>`
- `Features.get_render_features` (params: draw_mask:i32, send_mask:bool) → `Result<Vec<i32>, Error>`
- `Features.get_render_features_draw_flag_changed` (params: send_mask:bool) → `Result<Vec<i32>, Error>`
- `Features.valid_feature_id` (params: feature_id:i32) → `Result<bool, Error>`

## Game (28 functions)

- `Game.are_helper_ais_enabled` (params: ) → `Result<bool, Error>`
- `Game.fixed_allies` (params: ) → `Result<bool, Error>`
- `Game.get_ally_team_start_box` (params: ally_team_id:i32) → `Result<(sys::StartBox, bool), Error>`
- `Game.get_facing_from_heading` (params: heading:i32) → `Result<i32, Error>`
- `Game.get_gaia_team_id` (params: ) → `Result<i32, Error>`
- `Game.get_game_frame` (params: ) → `Result<(u32, u32), Error>`
- `Game.get_game_seconds` (params: ) → `Result<f32, Error>`
- `Game.get_global_los` (params: ally_team_id:i32) → `Result<i32, Error>`
- `Game.get_heading_from_facing` (params: facing:i32) → `Result<i32, Error>`
- `Game.get_heading_from_vector` (params: x:f32, z:f32) → `Result<i32, Error>`
- `Game.get_map_option` (params: key:&str) → `Result<(Option<String>, bool), Error>`
- `Game.get_map_options` (params: ) → `Result<Vec<String>, Error>`
- `Game.get_map_start_positions` (params: ) → `Result<Vec<sys::StartPosition>, Error>`
- `Game.get_mod_option` (params: key:&str) → `Result<(Option<String>, bool), Error>`
- `Game.get_mod_options` (params: ) → `Result<Vec<String>, Error>`
- `Game.get_side_data` (params: side_name:&str) → `Result<sys::SideData, Error>`
- `Game.get_side_data_by_index` (params: side_index:u32) → `Result<sys::SideData, Error>`
- `Game.get_side_data_count` (params: ) → `Result<u32, Error>`
- `Game.get_team_start_position` (params: team_id:i32) → `Result<sys::Float3, Error>`
- `Game.get_tidal` (params: ) → `Result<f32, Error>`
- `Game.get_vector_from_heading` (params: heading:i32) → `Result<sys::Float2, Error>`
- `Game.get_wind` (params: ) → `Result<sys::WindData, Error>`
- `Game.is_cheating_enabled` (params: ) → `Result<bool, Error>`
- `Game.is_dev_lua_enabled` (params: ) → `Result<bool, Error>`
- `Game.is_edit_defs_enabled` (params: ) → `Result<bool, Error>`
- `Game.is_game_over` (params: ) → `Result<bool, Error>`
- `Game.is_god_mode_enabled` (params: ) → `Result<bool, Error>`
- `Game.is_no_cost_enabled` (params: ) → `Result<bool, Error>`

## GameConfig (6 functions)

- `GameConfig.set_cheating_enabled` (params: enabled:bool) → `Result<bool, Error>`
- `GameConfig.set_experience_grade` (params: exp_grade:f32, exp_power_scale:f32, exp_health_scale:f32, exp_reload_scale:f32) → `Result<bool, Error>`
- `GameConfig.set_god_mode` (params: control_allies:bool, control_enemies:bool) → `Result<bool, Error>`
- `GameConfig.set_no_pause` (params: no_pause:bool) → `Result<bool, Error>`
- `GameConfig.set_radar_error_params` (params: ally_team_id:i32, ally_team_error_size:f32, base_error_size:f32, base_error_mult:f32) → `Result<bool, Error>`
- `GameConfig.set_square_building_mask` (params: x:i32, z:i32, mask:u16) → `Result<bool, Error>`

## GroundDecals (31 functions)

- `GroundDecals.create_ground_decal` (params: ) → `Result<(u32, bool), Error>`
- `GroundDecals.destroy_ground_decal` (params: decal_id:u32) → `Result<bool, Error>`
- `GroundDecals.get_all_ground_decals` (params: ) → `Result<Vec<u32>, Error>`
- `GroundDecals.get_ground_decal_alpha` (params: decal_id:u32) → `Result<(f32, f32), Error>`
- `GroundDecals.get_ground_decal_creation_frame` (params: decal_id:u32) → `Result<(f32, f32), Error>`
- `GroundDecals.get_ground_decal_glow_params` (params: decal_id:u32) → `Result<(f32, f32), Error>`
- `GroundDecals.get_ground_decal_middle_pos` (params: decal_id:u32) → `Result<([f32`
- `GroundDecals.get_ground_decal_misc` (params: decal_id:u32) → `Result<(f32, f32, f32, f32, f32), Error>`
- `GroundDecals.get_ground_decal_normal` (params: decal_id:u32) → `Result<[f32`
- `GroundDecals.get_ground_decal_owner` (params: decal_id:u32) → `Result<(bool, i32), Error>`
- `GroundDecals.get_ground_decal_quad_pos` (params: decal_id:u32) → `Result<([f32`
- `GroundDecals.get_ground_decal_rotation` (params: decal_id:u32) → `Result<(f32, bool), Error>`
- `GroundDecals.get_ground_decal_size_and_height` (params: decal_id:u32) → `Result<(f32, f32, f32, bool), Error>`
- `GroundDecals.get_ground_decal_texture` (params: decal_id:u32, main_tex:bool) → `Result<Option<String>, Error>`
- `GroundDecals.get_ground_decal_texture_params` (params: decal_id:u32) → `Result<(f32, f32), Error>`
- `GroundDecals.get_ground_decal_textures` (params: main_tex:bool, include_filenames:bool) → `Result<(Vec<String>, Vec<String>), Error>`
- `GroundDecals.get_ground_decal_tint` (params: decal_id:u32) → `Result<[f32`
- `GroundDecals.get_ground_decal_type` (params: decal_id:u32) → `Result<u8, Error>`
- `GroundDecals.get_ground_decal_user_data` (params: decal_id:u32, quad_index:u32) → `Result<([f32`
- `GroundDecals.set_ground_decal_alpha` (params: decal_id:u32, alpha:f32, alpha_falloff:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_creation_frame` (params: decal_id:u32, creation_frame_min:f32, creation_frame_max:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_glow_params` (params: decal_id:u32, glow:f32, glow_falloff:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_misc` (params: decal_id:u32, dot_elim_exp:f32, ref_height:f32, min_height:f32, max_height:f32, force_height_mode:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_normal` (params: decal_id:u32, normal_x:f32, normal_y:f32, normal_z:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_pos_and_dims` (params: decal_id:u32, mid_pos_x:f32, mid_pos_z:f32, size_x:f32, size_z:f32, proj_cube_height:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_quad_pos_and_height` (params: decal_id:u32, pos_tlx:f32, pos_tly:f32, pos_trx:f32, pos_try:f32, pos_brx:f32, pos_bry:f32, pos_blx:f32, pos_bly:f32, proj_cube_height:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_rotation` (params: decal_id:u32, rotation:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_texture` (params: decal_id:u32, texture_name:&str, main_tex:bool) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_texture_params` (params: decal_id:u32, tex_wrap_distance:f32, tex_traveled_distance:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_tint` (params: decal_id:u32, tint_r:f32, tint_g:f32, tint_b:f32, tint_a:f32) → `Result<bool, Error>`
- `GroundDecals.set_ground_decal_user_data` (params: decal_id:u32, quad_index:u32, value_x:f32, value_y:f32, value_z:f32, value_w:f32) → `Result<bool, Error>`

## Icons (6 functions)

- `Icons.add_unit_icon` (params: icon_name:&str, tex_file:&str, size:f32, distance:f32, radius_adjust:bool, u0:f32, v0:f32, u1:f32, v1:f32) → `Result<bool, Error>`
- `Icons.free_unit_icon` (params: icon_name:&str) → `Result<bool, Error>`
- `Icons.get_all_icon_data_array` (params: full_data:bool) → `Result<Vec<sys::IconDataEntry>, Error>`
- `Icons.get_icon_data` (params: icon_name:&str, full_data:bool) → `Result<sys::IconDataEntry, Error>`
- `Icons.unit_icon_get_draw` (params: unit_id:i32) → `Result<bool, Error>`
- `Icons.unit_icon_set_draw` (params: unit_id:i32, draw_icon:bool) → `Result<bool, Error>`

## Input (20 functions)

- `Input.get_action_hot_keys` (params: action:&str) → `Result<Vec<String>, Error>`
- `Input.get_active_command` (params: ) → `Result<(i32, i32, i32, Option<String>), Error>`
- `Input.get_active_page` (params: ) → `Result<(i32, i32), Error>`
- `Input.get_default_command` (params: ) → `Result<(i32, i32, i32, Option<String>), Error>`
- `Input.get_invert_queue_key` (params: ) → `Result<bool, Error>`
- `Input.get_key_bindings` (params: key_set1:&str, key_set2:&str) → `Result<Vec<String>, Error>`
- `Input.get_key_code` (params: key_sym:&str) → `Result<i32, Error>`
- `Input.get_key_from_scan_symbol` (params: scan_symbol:&str) → `Result<i32, Error>`
- `Input.get_key_state` (params: key_code:i32) → `Result<bool, Error>`
- `Input.get_key_symbol` (params: key_code:i32) → `Result<(Option<String>, Option<String>), Error>`
- `Input.get_mod_key_state` (params: ) → `Result<u32, Error>`
- `Input.get_mouse_buttons_pressed` (params: buttons:&[i32]) → `Result<Vec<bool>, Error>`
- `Input.get_mouse_cursor` (params: ) → `Result<Option<String>, Error>`
- `Input.get_mouse_start_position` (params: button:i32) → `Result<sys::Float2, Error>`
- `Input.get_mouse_state` (params: ) → `Result<sys::MouseState, Error>`
- `Input.get_pressed_keys` (params: ) → `Result<Vec<i32>, Error>`
- `Input.get_pressed_scans` (params: ) → `Result<Vec<i32>, Error>`
- `Input.get_scan_symbol` (params: scan_code:i32) → `Result<(Option<String>, Option<String>), Error>`
- `Input.get_selection_box` (params: ) → `Result<sys::SelectionBox, Error>`
- `Input.is_above_mini_map` (params: screen_x:f32, screen_y:f32) → `Result<bool, Error>`

## Lights (7 functions)

- `Lights.add_light_tracking_target` (params: light_handle:u32, object_id:i32, track_unit:bool, enable_tracking:bool) → `Result<bool, Error>`
- `Lights.add_map_light` (params: params:sys::LightParams) → `Result<u32, Error>`
- `Lights.add_model_light` (params: params:sys::LightParams) → `Result<u32, Error>`
- `Lights.set_map_light_tracking_state` (params: light_handle:u32, object_id:i32, enable_tracking:bool, track_unit:bool) → `Result<bool, Error>`
- `Lights.set_model_light_tracking_state` (params: light_handle:u32, object_id:i32, enable_tracking:bool, track_unit:bool) → `Result<bool, Error>`
- `Lights.update_map_light` (params: light_handle:u32, params:sys::LightParams) → `Result<bool, Error>`
- `Lights.update_model_light` (params: light_handle:u32, params:sys::LightParams) → `Result<bool, Error>`

## Los (10 functions)

- `Los.get_closest_valid_position` (params: pos:sys::Float3, radius:f32, unit_def_id:i32, team_id:i32) → `Result<sys::Float3, Error>`
- `Los.get_position_los_state` (params: pos:sys::Float3, ally_team_id:i32) → `Result<sys::PositionLosState, Error>`
- `Los.get_radar_error_params` (params: ally_team_id:i32) → `Result<sys::RadarErrorParams, Error>`
- `Los.is_pos_in_air_los` (params: pos:sys::Float3, ally_team_id:i32) → `Result<bool, Error>`
- `Los.is_pos_in_los` (params: pos:sys::Float3, ally_team_id:i32) → `Result<bool, Error>`
- `Los.is_pos_in_radar` (params: pos:sys::Float3, ally_team_id:i32) → `Result<bool, Error>`
- `Los.is_unit_in_air_los` (params: unit_id:i32, ally_team_id:i32) → `Result<bool, Error>`
- `Los.is_unit_in_jammer` (params: unit_id:i32, ally_team_id:i32) → `Result<bool, Error>`
- `Los.is_unit_in_los` (params: unit_id:i32, ally_team_id:i32) → `Result<bool, Error>`
- `Los.is_unit_in_radar` (params: unit_id:i32, ally_team_id:i32) → `Result<bool, Error>`

## Markers (6 functions)

- `Markers.add_world_icon` (params: cmd_id:i32, pos:sys::Float3) → `Result<bool, Error>`
- `Markers.add_world_text` (params: text:&str, pos:sys::Float3) → `Result<bool, Error>`
- `Markers.add_world_unit` (params: unit_def_id:i32, pos:sys::Float3, team_id:i32, facing:i32) → `Result<bool, Error>`
- `Markers.marker_add_line` (params: from:sys::Float3, to:sys::Float3, local_only:bool, player_id:i32) → `Result<bool, Error>`
- `Markers.marker_add_point` (params: pos:sys::Float3, text:&str, local_only:bool, player_id:i32) → `Result<bool, Error>`
- `Markers.marker_erase_position` (params: pos:sys::Float3, unused:f32, local_only:bool, player_id:i32, always_erase:bool) → `Result<bool, Error>`

## MathExtra (14 functions)

- `MathExtra.bit_and` (params: a:u32, b:u32) → `Result<u32, Error>`
- `MathExtra.bit_bits` (params: value:u32, start_bit:u32, end_bit:u32) → `Result<u32, Error>`
- `MathExtra.bit_inv` (params: a:u32) → `Result<u32, Error>`
- `MathExtra.bit_or` (params: a:u32, b:u32) → `Result<u32, Error>`
- `MathExtra.bit_xor` (params: a:u32, b:u32) → `Result<u32, Error>`
- `MathExtra.clamp` (params: value:f32, min:f32, max:f32) → `Result<f32, Error>`
- `MathExtra.diag` (params: values:&[f32]) → `Result<f32, Error>`
- `MathExtra.erf` (params: value:f32) → `Result<f32, Error>`
- `MathExtra.hypot` (params: x:f32, y:f32) → `Result<f32, Error>`
- `MathExtra.mix` (params: a:f32, b:f32, t:f32) → `Result<f32, Error>`
- `MathExtra.normalize` (params: vec:&mut sys::Float3) → `Result<f32, Error>`
- `MathExtra.round` (params: value:f32) → `Result<f32, Error>`
- `MathExtra.sgn` (params: value:f32) → `Result<f32, Error>`
- `MathExtra.smooth_step` (params: edge0:f32, edge1:f32, x:f32) → `Result<f32, Error>`

## Memory (9 functions)

- `Memory.free` (params: ptr:*mut std::ffi::c_void) → `Result<(), Error>`
- `Memory.free_float2_array` (params: data:&[sys::Float2]) → `Result<(), Error>`
- `Memory.free_float3_array` (params: data:&[sys::Float3]) → `Result<(), Error>`
- `Memory.free_float4_array` (params: data:&[sys::Float4]) → `Result<(), Error>`
- `Memory.free_float_array` (params: data:&[f32]) → `Result<(), Error>`
- `Memory.free_int32_array` (params: data:&[i32]) → `Result<(), Error>`
- `Memory.free_int3_array` (params: data:&[sys::Int3]) → `Result<(), Error>`
- `Memory.free_string_array` (params: data:*mut *const i8, length:u32) → `Result<(), Error>`
- `Memory.free_uint32_array` (params: data:&[u32]) → `Result<(), Error>`

## Messages (20 functions)

- `Messages.echo` (params: message:&str, rest:&str) → `Result<bool, Error>`
- `Messages.get_console_buffer` (params: max_lines:u32) → `Result<Vec<sys::ConsoleEntry>, Error>`
- `Messages.get_current_tooltip` (params: ) → `Result<Option<String>, Error>`
- `Messages.is_user_writing` (params: ) → `Result<bool, Error>`
- `Messages.log` (params: section:&str, level:i32, message:&str) → `Result<bool, Error>`
- `Messages.send_ally_chat` (params: message:&str) → `Result<bool, Error>`
- `Messages.send_commands` (params: command:&str, rest:&str) → `Result<bool, Error>`
- `Messages.send_lua_gaia_msg` (params: message:&str) → `Result<bool, Error>`
- `Messages.send_lua_menu_msg` (params: message:&str) → `Result<bool, Error>`
- `Messages.send_lua_rules_msg` (params: message:&str) → `Result<bool, Error>`
- `Messages.send_lua_uimsg` (params: message:&str, mode:&str) → `Result<bool, Error>`
- `Messages.send_message` (params: message:&str) → `Result<bool, Error>`
- `Messages.send_message_to_ally_team` (params: ally_team_id:i32, message:&str) → `Result<bool, Error>`
- `Messages.send_message_to_player` (params: player_id:i32, message:&str) → `Result<bool, Error>`
- `Messages.send_message_to_spectators` (params: message:&str) → `Result<bool, Error>`
- `Messages.send_message_to_team` (params: team_id:i32, message:&str) → `Result<bool, Error>`
- `Messages.send_private_chat` (params: message:&str, player_id:i32) → `Result<bool, Error>`
- `Messages.send_public_chat` (params: message:&str) → `Result<bool, Error>`
- `Messages.send_skirmish_aimessage` (params: ai_id:i32, message:&str) → `Result<bool, Error>`
- `Messages.send_spectator_chat` (params: message:&str) → `Result<bool, Error>`

## MetalMap (4 functions)

- `MetalMap.get_metal_amount` (params: x:i32, z:i32) → `Result<f32, Error>`
- `MetalMap.get_metal_extraction` (params: x:i32, z:i32) → `Result<f32, Error>`
- `MetalMap.get_metal_map_size` (params: ) → `Result<(i32, i32), Error>`
- `MetalMap.set_metal_amount` (params: x:i32, z:i32, amount:f32) → `Result<(), Error>`

## MoveCtrl (3 functions)

- `MoveCtrl.get_unit_estimated_path` (params: unit_id:i32) → `Result<Vec<sys::PathWaypoint>, Error>`
- `MoveCtrl.get_unit_move_type_data` (params: unit_id:i32) → `Result<sys::MoveTypeData, Error>`
- `MoveCtrl.move_ctrl` (params: unit_id:i32, enable:bool) → `Result<bool, Error>`

## PathFinder (10 functions)

- `PathFinder.delete_path` (params: path_id:u32) → `Result<bool, Error>`
- `PathFinder.free_path_node_costs_array` (params: overlay_index:u32) → `Result<bool, Error>`
- `PathFinder.get_next_way_point` (params: path_id:u32, caller_pos:sys::Float3, min_dist:f32) → `Result<(sys::Float3, bool), Error>`
- `PathFinder.get_path_node_cost` (params: x:u32, z:u32) → `Result<f32, Error>`
- `PathFinder.get_path_node_costs` (params: overlay_index:u32) → `Result<Vec<f32>, Error>`
- `PathFinder.get_path_way_points` (params: path_id:u32) → `Result<(Vec<sys::Float3>, Vec<i32>), Error>`
- `PathFinder.init_path_node_costs_array` (params: overlay_index:u32, size_x:u32, size_z:u32) → `Result<bool, Error>`
- `PathFinder.request_path` (params: move_def_id:u32, move_def_name:&str, start_pos:sys::Float3, end_pos:sys::Float3, radius:f32) → `Result<u32, Error>`
- `PathFinder.set_path_node_cost` (params: overlay_index:u32, cost_index:u32, cost:f32) → `Result<bool, Error>`
- `PathFinder.set_path_node_costs` (params: overlay_index:u32) → `Result<bool, Error>`

## Player (7 functions)

- `Player.get_local_ally_team_id` (params: ) → `Result<i32, Error>`
- `Player.get_local_player_id` (params: ) → `Result<i32, Error>`
- `Player.get_local_team_id` (params: ) → `Result<i32, Error>`
- `Player.get_player_roster` (params: sort_mode:i32, show_pathing_players:bool) → `Result<Vec<sys::RosterEntry>, Error>`
- `Player.get_player_statistics` (params: player_id:i32) → `Result<sys::PlayerStats, Error>`
- `Player.get_player_traffic` (params: player_id:i32, packet_id:i32) → `Result<Vec<sys::PlayerTraffic>, Error>`
- `Player.get_spectating_state` (params: ) → `Result<bool, Error>`

## Profiling (10 functions)

- `Profiling.diff_timers` (params: end_timer:u64, start_timer:u64, return_ms:bool, from_micro_secs:bool) → `Result<f32, Error>`
- `Profiling.get_draw_seconds` (params: ) → `Result<f32, Error>`
- `Profiling.get_frame_timer` (params: last_frame_time:bool) → `Result<u64, Error>`
- `Profiling.get_lua_mem_usage` (params: ) → `Result<(f32, f32, f32, f32, f32, f32, f32, f32), Error>`
- `Profiling.get_profiler_record_names` (params: ) → `Result<Vec<String>, Error>`
- `Profiling.get_profiler_time_record` (params: name:&str, include_frame_data:bool) → `Result<(f32, f32, f32, f32, f32, Vec<f32>), Error>`
- `Profiling.get_synced_gcinfo` (params: collect:bool) → `Result<f32, Error>`
- `Profiling.get_timer` (params: ) → `Result<u64, Error>`
- `Profiling.get_timer_micros` (params: ) → `Result<u64, Error>`
- `Profiling.get_vid_mem_usage` (params: ) → `Result<(f32, f32), Error>`

## ProjectileControl (19 functions)

- `ProjectileControl.delete_projectile` (params: projectile_id:i32) → `Result<bool, Error>`
- `ProjectileControl.set_piece_projectile_params` (params: projectile_id:i32, expl_flags:i32, spin_angle:f32, spin_speed:f32, spin_vec:sys::Float3) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_always_visible` (params: projectile_id:i32, always_visible:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_ceg` (params: projectile_id:i32, ceg_name:&str) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_collision` (params: projectile_id:i32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_damages` (params: projectile_id:i32, unused:i32, damage_key:&str, damage_value:f32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_gravity` (params: projectile_id:i32, gravity:f32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_ignore_tracking_error` (params: projectile_id:i32, ignore:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_is_intercepted` (params: projectile_id:i32, intercepted:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_move_control` (params: projectile_id:i32, enable:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_position` (params: projectile_id:i32, pos:sys::Float3) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_spin_angle` (params: projectile_id:i32, angle:f32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_spin_speed` (params: projectile_id:i32, speed:f32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_spin_vec` (params: projectile_id:i32, spin_vec:sys::Float3) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_target` (params: projectile_id:i32, target:sys::ProjectileTargetRef) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_time_to_live` (params: projectile_id:i32, time_to_live:i32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_use_air_los` (params: projectile_id:i32, use_air_los:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_velocity` (params: projectile_id:i32, velocity:sys::Float3) → `Result<bool, Error>`
- `ProjectileControl.spawn_projectile` (params: weapon_def_id:i32, projectile_params:sys::NativeProjectileParams) → `Result<i32, Error>`

## Projectiles (17 functions)

- `Projectiles.get_all_projectiles` (params: synced:bool, weapon:bool) → `Result<Vec<i32>, Error>`
- `Projectiles.get_piece_projectile_params` (params: projectile_id:i32) → `Result<(sys::PieceProjectileParams, bool), Error>`
- `Projectiles.get_projectile_ally_team_id` (params: projectile_id:i32) → `Result<i32, Error>`
- `Projectiles.get_projectile_damages` (params: projectile_id:i32, tag:&str) → `Result<sys::ProjectileDamages, Error>`
- `Projectiles.get_projectile_def_id` (params: projectile_id:i32) → `Result<i32, Error>`
- `Projectiles.get_projectile_direction` (params: projectile_id:i32) → `Result<sys::Float3, Error>`
- `Projectiles.get_projectile_gravity` (params: projectile_id:i32) → `Result<sys::Float3, Error>`
- `Projectiles.get_projectile_is_intercepted` (params: projectile_id:i32) → `Result<bool, Error>`
- `Projectiles.get_projectile_owner_id` (params: projectile_id:i32) → `Result<i32, Error>`
- `Projectiles.get_projectile_position` (params: projectile_id:i32) → `Result<sys::Float3, Error>`
- `Projectiles.get_projectile_target` (params: projectile_id:i32) → `Result<sys::ProjectileTarget, Error>`
- `Projectiles.get_projectile_team_id` (params: projectile_id:i32) → `Result<i32, Error>`
- `Projectiles.get_projectile_time_to_live` (params: projectile_id:i32) → `Result<f32, Error>`
- `Projectiles.get_projectile_type` (params: projectile_id:i32) → `Result<u32, Error>`
- `Projectiles.get_projectile_velocity` (params: projectile_id:i32) → `Result<sys::Float3, Error>`
- `Projectiles.get_projectiles_in_rectangle` (params: min_x:f32, min_z:f32, max_x:f32, max_z:f32, synced:bool, weapon:bool) → `Result<Vec<i32>, Error>`
- `Projectiles.get_projectiles_in_sphere` (params: center:sys::Float3, radius:f32, synced:bool, weapon:bool) → `Result<Vec<i32>, Error>`

## RulesParams (15 functions)

- `RulesParams.get_feature_rules_param` (params: feature_id:i32, param_name:&str) → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_feature_rules_params` (params: feature_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.get_game_rules_param` (params: param_name:&str) → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_game_rules_params` (params: ) → `Result<Vec<String>, Error>`
- `RulesParams.get_player_rules_param` (params: player_id:i32, param_name:&str) → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_player_rules_params` (params: player_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.get_team_rules_param` (params: team_id:i32, param_name:&str) → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_team_rules_params` (params: team_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.get_unit_rules_param` (params: unit_id:i32, param_name:&str) → `Result<(sys::RulesParamValue, i32, bool), Error>`
- `RulesParams.get_unit_rules_params` (params: unit_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.set_feature_rules_param` (params: feature_id:i32, param_name:&str, value:sys::RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_game_rules_param` (params: param_name:&str, value:sys::RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_player_rules_param` (params: player_id:i32, param_name:&str, value:sys::RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_team_rules_param` (params: team_id:i32, param_name:&str, value:sys::RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_unit_rules_param` (params: unit_id:i32, param_name:&str, value:sys::RulesParamValue, los:i32) → `Result<bool, Error>`

## Selection (16 functions)

- `Selection.deselect_unit` (params: unit_id:i32) → `Result<bool, Error>`
- `Selection.deselect_unit_array` (params: unit_ids:&[i32]) → `Result<bool, Error>`
- `Selection.get_group_list` (params: ) → `Result<Vec<i32>, Error>`
- `Selection.get_group_units` (params: group_id:i32) → `Result<Vec<i32>, Error>`
- `Selection.get_group_units_count` (params: group_id:i32) → `Result<u32, Error>`
- `Selection.get_group_units_counts` (params: group_id:i32) → `Result<sys::SelectionCounts, Error>`
- `Selection.get_group_units_sorted` (params: group_id:i32) → `Result<Vec<i32>, Error>`
- `Selection.get_selected_group` (params: ) → `Result<i32, Error>`
- `Selection.get_selected_units` (params: ) → `Result<Vec<i32>, Error>`
- `Selection.get_selected_units_count` (params: ) → `Result<u32, Error>`
- `Selection.get_selected_units_counts` (params: ) → `Result<sys::SelectionCounts, Error>`
- `Selection.get_selected_units_sorted` (params: ) → `Result<Vec<i32>, Error>`
- `Selection.get_unit_group` (params: unit_id:i32) → `Result<i32, Error>`
- `Selection.select_unit` (params: unit_id:i32, append:bool) → `Result<bool, Error>`
- `Selection.select_unit_array` (params: unit_ids:&[i32], append:bool) → `Result<bool, Error>`
- `Selection.set_unit_group` (params: unit_id:i32, group_id:i32) → `Result<bool, Error>`

## Sound (11 functions)

- `Sound.get_sound_devices` (params: ) → `Result<Vec<String>, Error>`
- `Sound.get_sound_effect_params` (params: ) → `Result<bool, Error>`
- `Sound.get_sound_stream_time` (params: ) → `Result<f32, Error>`
- `Sound.load_sound_def` (params: sound_name:&str) → `Result<bool, Error>`
- `Sound.pause_sound_stream` (params: ) → `Result<bool, Error>`
- `Sound.play_sound_file` (params: sound_file:&str, volume:f32, pos:sys::Float3, velocity:sys::Float3, channel:i32) → `Result<bool, Error>`
- `Sound.play_sound_stream` (params: ogg_file:&str, volume:f32, enqueue:bool) → `Result<bool, Error>`
- `Sound.preload_sound_item` (params: sound_name:&str) → `Result<bool, Error>`
- `Sound.set_sound_effect_params` (params: params:sys::SoundEffectParams) → `Result<bool, Error>`
- `Sound.set_sound_stream_volume` (params: volume:f32) → `Result<bool, Error>`
- `Sound.stop_sound_stream` (params: ) → `Result<bool, Error>`

## SyncedCtrl (8 functions)

- `SyncedCtrl.cob_script` (params: ) → `CobScript<'_>`
- `SyncedCtrl.effects` (params: ) → `EffectsControl<'_>`
- `SyncedCtrl.feature` (params: ) → `FeatureControl<'_>`
- `SyncedCtrl.game_config` (params: ) → `GameConfig<'_>`
- `SyncedCtrl.projectile` (params: ) → `ProjectileControl<'_>`
- `SyncedCtrl.team` (params: ) → `TeamControl<'_>`
- `SyncedCtrl.terrain` (params: ) → `TerrainControl<'_>`
- `SyncedCtrl.unit` (params: ) → `UnitControl<'_>`

## SystemControl (22 functions)

- `SystemControl.call_as_team` (params: team_id:i32, func:sys::LuaFunctionRef, args:sys::NativeLuaArgs) → `Result<bool, Error>`
- `SystemControl.clear_watch_dog_timer` (params: thread_name:&str, keep_stopped:bool) → `Result<bool, Error>`
- `SystemControl.garbage_collect_ctrl` (params: iters_per_batch:i32, num_steps_per_iter:i32, min_steps_per_iter:i32, max_steps_per_iter:i32, min_loop_run_time:f32, max_loop_run_time:f32, base_run_time_mult:f32, base_mem_load_mult:f32) → `Result<bool, Error>`
- `SystemControl.get_game_name` (params: ) → `Result<Option<String>, Error>`
- `SystemControl.get_game_state` (params: max_latency:f32) → `Result<(bool, bool, bool, bool), Error>`
- `SystemControl.get_gather_mode` (params: ) → `Result<i32, Error>`
- `SystemControl.get_menu_name` (params: ) → `Result<Option<String>, Error>`
- `SystemControl.get_replay_file_path` (params: ) → `Result<(Option<String>, bool), Error>`
- `SystemControl.get_replay_length` (params: ) → `Result<(f32, bool), Error>`
- `SystemControl.get_replay_recording_file_path` (params: ) → `Result<(Option<String>, bool), Error>`
- `SystemControl.get_video_capturing_mode` (params: ) → `Result<bool, Error>`
- `SystemControl.get_window_display_mode` (params: ) → `Result<(i32, i32, i32, i32, Option<String>, bool), Error>`
- `SystemControl.is_replay` (params: ) → `Result<bool, Error>`
- `SystemControl.ping` (params: tag:u32) → `Result<bool, Error>`
- `SystemControl.quit` (params: ) → `Result<bool, Error>`
- `SystemControl.reload` (params: start_script:&str) → `Result<bool, Error>`
- `SystemControl.request_start_position` (params: pos:sys::Float3, ready:bool) → `Result<bool, Error>`
- `SystemControl.restart` (params: cmd_args:&str, start_script:&str) → `Result<bool, Error>`
- `SystemControl.set_share_level` (params: resource:&str, level:f32) → `Result<bool, Error>`
- `SystemControl.share_resources` (params: team_id:i32, resource:&str, amount:f32) → `Result<bool, Error>`
- `SystemControl.start` (params: cmd_args:&str, start_script:&str) → `Result<bool, Error>`
- `SystemControl.yield` (params: ) → `Result<bool, Error>`

## TeamControl (14 functions)

- `TeamControl.add_team_resource` (params: team_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`
- `TeamControl.assign_player_to_team` (params: player_id:i32, team_id:i32) → `Result<bool, Error>`
- `TeamControl.game_over` (params: winning_ally_teams:&[i32]) → `Result<bool, Error>`
- `TeamControl.kill_team` (params: team_id:i32) → `Result<bool, Error>`
- `TeamControl.set_ally` (params: first_ally_team_id:i32, second_ally_team_id:i32, allied:bool) → `Result<bool, Error>`
- `TeamControl.set_ally_team_start_box` (params: ally_team_id:i32, min_x:f32, min_z:f32, max_x:f32, max_z:f32) → `Result<bool, Error>`
- `TeamControl.set_global_los` (params: ally_team_id:i32, enabled:bool) → `Result<bool, Error>`
- `TeamControl.set_player_ready_state` (params: player_id:i32, ready:bool) → `Result<bool, Error>`
- `TeamControl.set_team_resource` (params: team_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`
- `TeamControl.set_team_share_level` (params: team_id:i32, resource_type:&str, share_level:f32) → `Result<bool, Error>`
- `TeamControl.set_team_start_position` (params: team_id:i32, pos:sys::Float3) → `Result<bool, Error>`
- `TeamControl.share_team_resource` (params: team_id:i32, target_team_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`
- `TeamControl.transfer_team_max_units` (params: from_team_id:i32, to_team_id:i32, amount:i32) → `Result<bool, Error>`
- `TeamControl.use_team_resource` (params: team_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`

## Teams (19 functions)

- `Teams.are_players_allied` (params: player_id1:i32, player_id2:i32) → `Result<bool, Error>`
- `Teams.are_teams_allied` (params: team_id1:i32, team_id2:i32) → `Result<bool, Error>`
- `Teams.get_aiinfo` (params: team_id:i32) → `Result<(sys::AIInfo, bool), Error>`
- `Teams.get_ally_team_info` (params: ally_team_id:i32) → `Result<sys::AllyTeamInfo, Error>`
- `Teams.get_ally_team_list` (params: ) → `Result<Vec<i32>, Error>`
- `Teams.get_player_controlled_unit` (params: player_id:i32) → `Result<(i32, bool), Error>`
- `Teams.get_player_info` (params: player_id:i32, get_player_opts:bool) → `Result<sys::PlayerInfo, Error>`
- `Teams.get_player_list` (params: team_id:i32, active:bool) → `Result<Vec<i32>, Error>`
- `Teams.get_player_list_in_ally_team` (params: ally_team_id:i32) → `Result<Vec<i32>, Error>`
- `Teams.get_player_list_in_team` (params: team_id:i32) → `Result<Vec<i32>, Error>`
- `Teams.get_team_ally_team_id` (params: team_id:i32) → `Result<i32, Error>`
- `Teams.get_team_info` (params: team_id:i32, get_team_keys:bool) → `Result<sys::TeamInfo, Error>`
- `Teams.get_team_list` (params: ally_team_id:i32) → `Result<Vec<i32>, Error>`
- `Teams.get_team_lua_ai` (params: team_id:i32) → `Result<Option<String>, Error>`
- `Teams.get_team_max_units` (params: team_id:i32) → `Result<i32, Error>`
- `Teams.get_team_resource_stats` (params: team_id:i32, resource:&str) → `Result<sys::TeamResources, Error>`
- `Teams.get_team_resources` (params: team_id:i32, resource:&str) → `Result<sys::TeamResources, Error>`
- `Teams.get_team_stats_history` (params: team_id:i32, start_index:i32, end_index:i32) → `Result<Vec<sys::TeamStatsHistoryPoint>, Error>`
- `Teams.get_team_unit_stats` (params: team_id:i32) → `Result<sys::TeamUnitStats, Error>`

## Terrain (12 functions)

- `Terrain.get_grass` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_ground_blocked` (params: x1:f32, z1:f32, x2:f32, z2:f32) → `Result<bool, Error>`
- `Terrain.get_ground_extremes` (params: ) → `Result<(f32, f32, f32, f32), Error>`
- `Terrain.get_ground_height` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_ground_info` (params: x:f32, z:f32) → `Result<(i32, Option<String>, f32, f32, f32, f32, f32, f32, bool), Error>`
- `Terrain.get_ground_normal` (params: x:f32, z:f32, smoothed:bool) → `Result<(sys::Float3, f32), Error>`
- `Terrain.get_ground_orig_height` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_smooth_mesh_height` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_terrain_type_data` (params: terrain_type_index:i32) → `Result<(i32, Option<String>, f32, f32, f32, f32, f32, bool), Error>`
- `Terrain.get_water_level` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_water_plane_level` (params: ) → `Result<f32, Error>`
- `Terrain.is_pos_in_map` (params: x:f32, z:f32) → `Result<(bool, bool), Error>`

## TerrainControl (25 functions)

- `TerrainControl.add_grass` (params: x:f32, z:f32) → `Result<bool, Error>`
- `TerrainControl.add_height_map` (params: x:f32, z:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.add_original_height_map` (params: x:f32, z:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.add_smooth_mesh` (params: x:f32, z:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.adjust_height_map` (params: x1:f32, z1:f32, x2:f32, z2:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.adjust_original_height_map` (params: x1:f32, z1:f32, x2:f32, z2:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.adjust_smooth_mesh` (params: x1:f32, z1:f32, x2:f32, z2:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.level_height_map` (params: x1:f32, z1:f32, x2:f32, z2:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.level_original_height_map` (params: x1:f32, z1:f32, x2:f32, z2:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.level_smooth_mesh` (params: x1:f32, z1:f32, x2:f32, z2:f32, height:f32) → `Result<bool, Error>`
- `TerrainControl.rebuild_smooth_mesh` (params: ) → `Result<bool, Error>`
- `TerrainControl.remove_grass` (params: x:f32, z:f32) → `Result<bool, Error>`
- `TerrainControl.revert_height_map` (params: x1:f32, z1:f32, x2:f32, z2:f32, orig_factor:f32) → `Result<bool, Error>`
- `TerrainControl.revert_original_height_map` (params: x1:f32, z1:f32, x2:f32, z2:f32, orig_factor:f32) → `Result<bool, Error>`
- `TerrainControl.revert_smooth_mesh` (params: x1:f32, z1:f32, x2:f32, z2:f32, orig_factor:f32) → `Result<bool, Error>`
- `TerrainControl.set_height_map` (params: x:f32, z:f32, height:f32, terraform:f32) → `Result<bool, Error>`
- `TerrainControl.set_height_map_func` (params: lua_function:sys::LuaFunctionRef, arg:f32, args:sys::NativeLuaArgs) → `Result<bool, Error>`
- `TerrainControl.set_map_square_terrain_type` (params: x:i32, z:i32, terrain_type:i32) → `Result<bool, Error>`
- `TerrainControl.set_original_height_map` (params: x:f32, z:f32, height:f32, factor:f32) → `Result<bool, Error>`
- `TerrainControl.set_original_height_map_func` (params: height_map_func:sys::LuaFunctionRef) → `Result<bool, Error>`
- `TerrainControl.set_smooth_mesh` (params: x:f32, z:f32, height:f32, terraform:f32) → `Result<bool, Error>`
- `TerrainControl.set_smooth_mesh_func` (params: lua_function:sys::LuaFunctionRef, arg:sys::NativeLuaValue, args:sys::NativeLuaArgs) → `Result<bool, Error>`
- `TerrainControl.set_terrain_type_data` (params: type_index:i32, tank_speed:f32, kbot_speed:f32, hover_speed:f32, ship_speed:f32, hardness:f32, receive_tracks:bool, name:&str) → `Result<bool, Error>`
- `TerrainControl.set_tidal` (params: tidal:f32) → `Result<bool, Error>`
- `TerrainControl.set_wind` (params: min_wind:f32, max_wind:f32) → `Result<bool, Error>`

## Tracing (5 functions)

- `Tracing.trace_ray` (params: ray:sys::Ray) → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_features` (params: ray:sys::Ray) → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_ground_between_positions` (params: start:sys::Float3, end:sys::Float3, test_water:bool) → `Result<(bool, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_ground_in_direction` (params: start:sys::Float3, dir:sys::Float3, length:f32) → `Result<(bool, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_units` (params: ray:sys::Ray) → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`

## UnitControl (88 functions)

- `UnitControl.add_object_decal` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitControl.add_unit_damage` (params: unit_id:i32, damage:f32, paralyze_time:f32, weapon_def_id:i32, attacker_id:i32, impulse:sys::Float3) → `Result<bool, Error>`
- `UnitControl.add_unit_experience` (params: unit_id:i32, experience:f32) → `Result<bool, Error>`
- `UnitControl.add_unit_impulse` (params: unit_id:i32, impulse:sys::Float3, decay_rate:f32) → `Result<bool, Error>`
- `UnitControl.add_unit_resource` (params: unit_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`
- `UnitControl.add_unit_seismic_ping` (params: unit_id:i32, ping_size:f32) → `Result<bool, Error>`
- `UnitControl.bugger_off` (params: pos:sys::Float3, radius:f32, team_id:i32, spherical:bool, forced:bool, exclude_unit_id:i32, exclude_unit_def_ids:&[i32]) → `Result<bool, Error>`
- `UnitControl.clear_unit_goal` (params: unit_id:i32, cancel_raw:bool) → `Result<bool, Error>`
- `UnitControl.create_unit` (params: unit_def:sys::DefRef, pos:sys::Float3, facing:i32, team_id:i32, build:bool, flatten_ground:bool, unit_id:i32, builder_id:i32) → `Result<i32, Error>`
- `UnitControl.destroy_unit` (params: unit_id:i32, selfd:bool, reclaimed:bool, attacker_id:i32, recycle_id:bool) → `Result<bool, Error>`
- `UnitControl.edit_unit_cmd_desc` (params: unit_id:i32, cmd_desc_index:u32, cmd_desc:&sys::NativeCommandDescription) → `Result<bool, Error>`
- `UnitControl.force_unit_collision_update` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitControl.get_unit_feature_separation` (params: unit_id:i32, feature_id:i32, ignore_y:bool) → `Result<f32, Error>`
- `UnitControl.get_unit_leaves_ghost` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitControl.get_unit_physical_state` (params: unit_id:i32) → `Result<u32, Error>`
- `UnitControl.give_order_array_to_unit` (params: unit_id:i32, commands:&[sys::NativeCommand]) → `Result<bool, Error>`
- `UnitControl.give_order_array_to_unit_array` (params: unit_ids:&[i32], commands:&[sys::NativeCommand], pairwise:bool) → `Result<i32, Error>`
- `UnitControl.give_order_to_unit` (params: unit_id:i32, cmd_id:i32, params:&[f32], options:u32, timeout:i32) → `Result<bool, Error>`
- `UnitControl.give_order_to_unit_array` (params: unit_ids:&[i32], cmd_id:i32, params:&[f32], options:u32, timeout:i32) → `Result<bool, Error>`
- `UnitControl.insert_unit_cmd_desc` (params: unit_id:i32, cmd_desc_index:i32, cmd_desc:&sys::NativeCommandDescription) → `Result<bool, Error>`
- `UnitControl.remove_object_decal` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitControl.remove_unit_cmd_desc` (params: unit_id:i32, cmd_desc_index:i32) → `Result<bool, Error>`
- `UnitControl.set_factory_bugger_off` (params: unit_id:i32, perform:bool, offset:f32, radius:f32, rel_heading:i32, spherical:bool, forced:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_always_visible` (params: unit_id:i32, always_visible:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_armored` (params: unit_id:i32, armored_state:bool, armored_multiple:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_blocking` (params: unit_id:i32, blocking:bool, solid_objects:bool, projectiles:bool, quad_map_rays:bool, crushable:bool, block_enemy_pushing:bool, block_height_changes:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_build_params` (params: unit_id:i32, param_name:&str, value:sys::NumberOrBool) → `Result<bool, Error>`
- `UnitControl.set_unit_build_speed` (params: unit_id:i32, build_speed:f32, repair_speed:f32, reclaim_speed:f32, resurrect_speed:f32, capture_speed:f32, terraform_speed:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_buildee_radius` (params: unit_id:i32, radius:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_cloak` (params: unit_id:i32, cloak:sys::NumberOrBool, cloak_arg:sys::NumberOrBool) → `Result<bool, Error>`
- `UnitControl.set_unit_collision_volume_data` (params: unit_id:i32, scales:sys::Float3, offsets:sys::Float3, volume_type:i32, test_type:i32, primary_axis:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_costs` (params: unit_id:i32, costs:sys::UnitCostOverrides) → `Result<bool, Error>`
- `UnitControl.set_unit_crashing` (params: unit_id:i32, want_crash:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_direction` (params: unit_id:i32, front_dir:sys::Float3, right_dir:sys::Float3) → `Result<bool, Error>`
- `UnitControl.set_unit_experience` (params: unit_id:i32, experience:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_flanking` (params: unit_id:i32, r#type:&str, args:sys::Float3) → `Result<bool, Error>`
- `UnitControl.set_unit_harvest_storage` (params: unit_id:i32, stored_metal:f32, max_stored_metal:f32, stored_energy:f32, max_stored_energy:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_heading` (params: unit_id:i32, heading:i32, use_smoothing:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_heading_and_up_dir` (params: unit_id:i32, heading:i32, up_dir:sys::Float3) → `Result<bool, Error>`
- `UnitControl.set_unit_health` (params: unit_id:i32, value:sys::UnitHealthValue) → `Result<bool, Error>`
- `UnitControl.set_unit_land_goal` (params: unit_id:i32, pos:sys::Float3, radius_sq:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_leaves_ghost` (params: unit_id:i32, leaves_ghost:bool, leave_dead_ghost:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_loading_transport` (params: unit_id:i32, transport_id:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_los_mask` (params: unit_id:i32, ally_team_id:i32, los_mask:u8) → `Result<bool, Error>`
- `UnitControl.set_unit_los_state` (params: unit_id:i32, ally_team_id:i32, los_state:u8) → `Result<bool, Error>`
- `UnitControl.set_unit_mass` (params: unit_id:i32, mass:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_max_health` (params: unit_id:i32, max_health:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_max_range` (params: unit_id:i32, max_range:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_metal_extraction` (params: unit_id:i32, depth:f32, range:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_mid_and_aim_pos` (params: unit_id:i32, mid_pos:sys::Float3, aim_pos:sys::Float3, set_relative:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_move_goal` (params: unit_id:i32, pos:sys::Float3, radius:f32, speed:f32, raw:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_nano_pieces` (params: unit_id:i32, piece_indices:&[i32]) → `Result<bool, Error>`
- `UnitControl.set_unit_neutral` (params: unit_id:i32, neutral:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_physical_state_bit` (params: unit_id:i32, state_bit:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_physics` (params: unit_id:i32, pos:sys::Float3, velocity:sys::Float3, rotation:sys::Float3, drag:sys::Float3) → `Result<bool, Error>`
- `UnitControl.set_unit_piece_collision_volume_data` (params: unit_id:i32, piece_index:i32, enable:bool, scales:sys::Float3, offsets:sys::Float3, volume_type:i32, primary_axis:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_piece_matrix` (params: unit_id:i32, piece_index:i32, matrix:[f32; 16]) → `Result<bool, Error>`
- `UnitControl.set_unit_piece_parent` (params: unit_id:i32, child_piece_index:i32, parent_piece_index:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_piece_visible` (params: unit_id:i32, piece_index:i32, visible:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_pos_error_params` (params: unit_id:i32, pos_error_vector:sys::Float3, pos_error_delta:sys::Float3, next_pos_error_update:i32, ally_team_id:i32, set_pos_error_bit:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_position` (params: unit_id:i32, pos:sys::Float3) → `Result<bool, Error>`
- `UnitControl.set_unit_radius_and_height` (params: unit_id:i32, radius:f32, height:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_resourcing` (params: unit_id:i32, r#type:&str, amount:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_rotation` (params: unit_id:i32, rotation:sys::Float3) → `Result<bool, Error>`
- `UnitControl.set_unit_seismic_signature` (params: unit_id:i32, seismic_signature:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_selection_volume_data` (params: unit_id:i32, scales:sys::Float3, offsets:sys::Float3, volume_type:i32, test_type:i32, primary_axis:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_sensor_radius` (params: unit_id:i32, sensor_type:&str, radius:i32) → `Result<i32, Error>`
- `UnitControl.set_unit_shield_recharge_delay` (params: unit_id:i32, weapon_num:i32, recharge_delay:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_shield_state` (params: unit_id:i32, weapon_num:i32, enabled:bool, power:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_sonar_stealth` (params: unit_id:i32, sonar_stealth:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_stealth` (params: unit_id:i32, stealth:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_stockpile` (params: unit_id:i32, stockpile:i32, build_percent:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_storage` (params: unit_id:i32, resource:&str, amount:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_target` (params: unit_id:i32, target:sys::UnitTargetRef, manual_fire:bool, user_target:bool, weapon_num:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_tooltip` (params: unit_id:i32, tooltip:&str) → `Result<bool, Error>`
- `UnitControl.set_unit_use_air_los` (params: unit_id:i32, use_air_los:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_use_weapons` (params: unit_id:i32, force_use_weapons:bool, allow_use_weapons:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_velocity` (params: unit_id:i32, velocity:sys::Float3) → `Result<bool, Error>`
- `UnitControl.set_unit_weapon_damages` (params: unit_id:i32, weapon_num:i32, damage_key:&str, damage_value:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_weapon_state` (params: unit_id:i32, weapon_num:i32, key:&str, value:f32) → `Result<bool, Error>`
- `UnitControl.transfer_unit` (params: unit_id:i32, new_team_id:i32, given:bool, adjust_unit_limit:bool) → `Result<bool, Error>`
- `UnitControl.unit_attach` (params: transporter_id:i32, transportee_id:i32, piece_num:i32) → `Result<bool, Error>`
- `UnitControl.unit_detach` (params: transportee_id:i32) → `Result<bool, Error>`
- `UnitControl.unit_detach_from_air` (params: transportee_id:i32, pos:sys::Float3) → `Result<bool, Error>`
- `UnitControl.unit_finish_command` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitControl.unit_weapon_fire` (params: unit_id:i32, weapon_num:i32) → `Result<bool, Error>`
- `UnitControl.unit_weapon_hold_fire` (params: unit_id:i32, weapon_num:i32) → `Result<bool, Error>`
- `UnitControl.use_unit_resource` (params: unit_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`

## UnitDefs (12 functions)

- `UnitDefs.get_unit_def_by_id` (params: unit_def_id:i32) → `Result<(bool, sys::UnitDefBasicInfo, sys::UnitDefCosts, sys::UnitDefPhysics, sys::UnitDefWeapons, sys::UnitDefBuildOptions, sys::UnitDefSensors, sys::UnitDefHealth), Error>`
- `UnitDefs.get_unit_def_costs` (params: unit_def_id:i32) → `Result<sys::UnitDefCosts, Error>`
- `UnitDefs.get_unit_def_count` (params: ) → `Result<u32, Error>`
- `UnitDefs.get_unit_def_custom_param` (params: unit_def_id:i32, key:&str) → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_custom_param_keys` (params: unit_def_id:i32) → `Result<Vec<String>, Error>`
- `UnitDefs.get_unit_def_health` (params: unit_def_id:i32) → `Result<f32, Error>`
- `UnitDefs.get_unit_def_human_name` (params: unit_def_id:i32) → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_idby_name` (params: unit_def_name:&str) → `Result<i32, Error>`
- `UnitDefs.get_unit_def_ids` (params: ) → `Result<Vec<i32>, Error>`
- `UnitDefs.get_unit_def_name` (params: unit_def_id:i32) → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_speed` (params: unit_def_id:i32) → `Result<f32, Error>`
- `UnitDefs.valid_unit_def_id` (params: unit_def_id:i32) → `Result<bool, Error>`

## UnitRendering (24 functions)

- `UnitRendering.get_camera_rotation` (params: ) → `Result<(f32, f32, f32), Error>`
- `UnitRendering.get_camera_vectors` (params: ) → `Result<(sys::Float3, sys::Float3, sys::Float3), Error>`
- `UnitRendering.get_features_in_screen_rectangle` (params: left:f32, top:f32, right:f32, bottom:f32) → `Result<Vec<i32>, Error>`
- `UnitRendering.get_frustum_planes` (params: ) → `Result<[f32`
- `UnitRendering.get_unit_always_update_matrix` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.get_unit_draw_flag` (params: unit_id:i32) → `Result<u8, Error>`
- `UnitRendering.get_unit_engine_draw_mask` (params: unit_id:i32) → `Result<u32, Error>`
- `UnitRendering.get_unit_icon` (params: unit_id:i32) → `Result<(Option<String>, [f32`
- `UnitRendering.get_unit_icon_data` (params: unit_id:i32, full_data:bool) → `Result<(Option<String>, [f32`
- `UnitRendering.get_unit_lua_draw` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.get_unit_no_draw` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.get_unit_no_group` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.get_unit_no_minimap` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.get_unit_no_select` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.get_unit_selection_volume_data` (params: unit_id:i32) → `Result<(sys::Float3, sys::Float3, i32, bool, i32, bool), Error>`
- `UnitRendering.get_unit_transform_matrix` (params: unit_id:i32) → `Result<[f32`
- `UnitRendering.get_unit_view_position` (params: unit_id:i32, use_mid_pos:bool) → `Result<sys::Float3, Error>`
- `UnitRendering.get_units_in_screen_rectangle` (params: left:f32, top:f32, right:f32, bottom:f32, allegiance:i32) → `Result<Vec<i32>, Error>`
- `UnitRendering.get_visible_features` (params: ally_team_id:i32, radius:f32, include_icons:bool, include_geos:bool) → `Result<Vec<i32>, Error>`
- `UnitRendering.get_visible_projectiles` (params: ally_team_id:i32, include_synced_projectiles:bool, include_weapon_projectiles:bool, include_piece_projectiles:bool) → `Result<Vec<i32>, Error>`
- `UnitRendering.get_visible_units` (params: team_id:i32, radius:f32, include_icons:bool) → `Result<Vec<i32>, Error>`
- `UnitRendering.is_unit_icon` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.is_unit_in_view` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.is_unit_visible` (params: unit_id:i32, radius:f32, check_icon:bool) → `Result<bool, Error>`

## UnitsCommands (15 functions)

- `UnitsCommands.find_unit_cmd_desc` (params: unit_id:i32, cmd_id:i32) → `Result<(i32, bool), Error>`
- `UnitsCommands.get_command_queue` (params: unit_id:i32, max_commands:u32) → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_factory_bugger_off` (params: unit_id:i32) → `Result<(bool, sys::Float3, f32), Error>`
- `UnitsCommands.get_factory_command_count` (params: unit_id:i32) → `Result<u32, Error>`
- `UnitsCommands.get_factory_commands` (params: unit_id:i32, max_commands:u32) → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_factory_counts` (params: unit_id:i32, count:i32, add_cmds:bool) → `Result<sys::FactoryQueueInfo, Error>`
- `UnitsCommands.get_full_build_queue` (params: unit_id:i32) → `Result<Vec<sys::BuildQueueEntry>, Error>`
- `UnitsCommands.get_real_build_queue` (params: unit_id:i32) → `Result<Vec<i32>, Error>`
- `UnitsCommands.get_unit_cmd_descs` (params: unit_id:i32) → `Result<Vec<sys::CommandDescription>, Error>`
- `UnitsCommands.get_unit_command_count` (params: unit_id:i32) → `Result<u32, Error>`
- `UnitsCommands.get_unit_commands` (params: unit_id:i32, max_commands:u32) → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_unit_current_command` (params: unit_id:i32, cmd_index:i32) → `Result<(sys::CommandFFI, bool), Error>`
- `UnitsCommands.give_order` (params: cmd_id:i32, params:&[f32], options:u32, timeout:i32) → `Result<bool, Error>`
- `UnitsCommands.give_order_array_to_unit_map` (params: unit_ids:&[i32], commands:&[sys::CommandFFI]) → `Result<i32, Error>`
- `UnitsCommands.give_order_to_unit_map` (params: unit_ids:&[i32], cmd_id:i32, params:&[f32], options:u32, timeout:i32) → `Result<i32, Error>`

## UnitsInfo (58 functions)

- `UnitsInfo.clear_units_previous_draw_flag` (params: ) → `Result<bool, Error>`
- `UnitsInfo.get_unit_ally_team` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_armored` (params: unit_id:i32) → `Result<sys::UnitArmoredState, Error>`
- `UnitsInfo.get_unit_base_position` (params: unit_id:i32) → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_blocking` (params: unit_id:i32) → `Result<sys::UnitBlockingState, Error>`
- `UnitsInfo.get_unit_build_facing` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_build_params` (params: unit_id:i32, param_name:&str) → `Result<(sys::NumberOrBool, bool), Error>`
- `UnitsInfo.get_unit_buildee_radius` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_collision_volume_data` (params: unit_id:i32) → `Result<sys::CollisionVolumeData, Error>`
- `UnitsInfo.get_unit_cost_table` (params: unit_id:i32) → `Result<sys::UnitCosts, Error>`
- `UnitsInfo.get_unit_costs` (params: unit_id:i32) → `Result<sys::UnitCosts, Error>`
- `UnitsInfo.get_unit_current_build_power` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_def_id` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_direction` (params: unit_id:i32) → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_effective_build_range` (params: unit_id:i32, buildee_def_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_experience` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_flanking` (params: unit_id:i32) → `Result<sys::UnitFlanking, Error>`
- `UnitsInfo.get_unit_fuel` (params: unit_id:i32) → `Result<sys::UnitFuel, Error>`
- `UnitsInfo.get_unit_harvest_storage` (params: unit_id:i32) → `Result<sys::UnitHarvestStorage, Error>`
- `UnitsInfo.get_unit_heading` (params: unit_id:i32, convert_to_radians:bool) → `Result<f32, Error>`
- `UnitsInfo.get_unit_health` (params: unit_id:i32) → `Result<sys::UnitHealth, Error>`
- `UnitsInfo.get_unit_height` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_in_build_stance` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_active` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_being_built` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_building` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_is_cloaked` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_dead` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_stunned` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_is_transporting` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_last_attacked_piece` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_last_attacker` (params: unit_id:i32) → `Result<(sys::UnitLastAttacker, bool), Error>`
- `UnitsInfo.get_unit_los_state` (params: unit_id:i32, ally_team_id:i32, raw:bool) → `Result<sys::UnitLosState, Error>`
- `UnitsInfo.get_unit_mass` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_metal_extraction` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_move_def_id` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_nano_pieces` (params: unit_id:i32) → `Result<Vec<i32>, Error>`
- `UnitsInfo.get_unit_neutral` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_piece_collision_volume_data` (params: unit_id:i32, piece_num:i32) → `Result<sys::CollisionVolumeData, Error>`
- `UnitsInfo.get_unit_pos_error_params` (params: unit_id:i32, ally_team_id:i32) → `Result<sys::UnitPosErrorParams, Error>`
- `UnitsInfo.get_unit_position` (params: unit_id:i32, mid_pos:bool, aim_pos:bool) → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_radius` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_resources` (params: unit_id:i32) → `Result<sys::UnitResources, Error>`
- `UnitsInfo.get_unit_rotation` (params: unit_id:i32) → `Result<sys::UnitRotation, Error>`
- `UnitsInfo.get_unit_seismic_signature` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_self_dtime` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_sensor_radius` (params: unit_id:i32, r#type:&str) → `Result<sys::UnitSensorRadius, Error>`
- `UnitsInfo.get_unit_shield_state` (params: unit_id:i32, weapon_num:i32) → `Result<(sys::UnitShieldState, bool), Error>`
- `UnitsInfo.get_unit_states` (params: unit_id:i32) → `Result<sys::UnitStates, Error>`
- `UnitsInfo.get_unit_stockpile` (params: unit_id:i32) → `Result<sys::UnitStockpile, Error>`
- `UnitsInfo.get_unit_storage` (params: unit_id:i32) → `Result<sys::UnitStorage, Error>`
- `UnitsInfo.get_unit_team` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_tooltip` (params: unit_id:i32) → `Result<Option<String>, Error>`
- `UnitsInfo.get_unit_transporter` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_travel` (params: unit_id:i32) → `Result<sys::UnitTravel, Error>`
- `UnitsInfo.get_unit_vectors` (params: unit_id:i32) → `Result<sys::UnitVectors, Error>`
- `UnitsInfo.get_unit_velocity` (params: unit_id:i32) → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_worker_task` (params: unit_id:i32) → `Result<sys::UnitWorkerTask, Error>`

## UnitsPieces (21 functions)

- `UnitsPieces.get_feature_piece_direction` (params: feature_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_feature_piece_info` (params: feature_id:i32, piece_num:i32) → `Result<(sys::PieceInfo, bool), Error>`
- `UnitsPieces.get_feature_piece_list` (params: feature_id:i32) → `Result<Vec<i32>, Error>`
- `UnitsPieces.get_feature_piece_map` (params: feature_id:i32) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_feature_piece_matrix` (params: feature_id:i32, piece_num:i32) → `Result<sys::PieceMatrix, Error>`
- `UnitsPieces.get_feature_piece_pos_dir` (params: feature_id:i32, piece_num:i32) → `Result<sys::PiecePosDir, Error>`
- `UnitsPieces.get_feature_piece_position` (params: feature_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_feature_root_piece` (params: feature_id:i32) → `Result<i32, Error>`
- `UnitsPieces.get_model_piece_list` (params: model_name:&str) → `Result<Vec<i32>, Error>`
- `UnitsPieces.get_model_piece_map` (params: model_name:&str) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_model_root_piece` (params: model_name:&str) → `Result<i32, Error>`
- `UnitsPieces.get_unit_piece_direction` (params: unit_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_unit_piece_info` (params: unit_id:i32, piece_num:i32) → `Result<(sys::PieceInfo, bool), Error>`
- `UnitsPieces.get_unit_piece_list` (params: unit_id:i32) → `Result<Vec<i32>, Error>`
- `UnitsPieces.get_unit_piece_map` (params: unit_id:i32) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_unit_piece_matrix` (params: unit_id:i32, piece_num:i32) → `Result<sys::PieceMatrix, Error>`
- `UnitsPieces.get_unit_piece_pos_dir` (params: unit_id:i32, piece_num:i32) → `Result<sys::PiecePosDir, Error>`
- `UnitsPieces.get_unit_piece_position` (params: unit_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_unit_root_piece` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsPieces.get_unit_script_names` (params: unit_id:i32) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_unit_script_piece` (params: unit_id:i32, script_num:i32) → `Result<i32, Error>`

## UnitsQuery (21 functions)

- `UnitsQuery.get_all_units` (params: ) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_closest_enemy_unit` (params: pos:sys::Float3, range:f32, ally_team_id:i32, use_los:bool, sphere_dist_test:bool, check_sight_dist:bool) → `Result<i32, Error>`
- `UnitsQuery.get_render_units` (params: draw_mask:i32, send_mask:bool) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_render_units_draw_flag_changed` (params: send_mask:bool) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_team_unit_count` (params: team_id:i32) → `Result<u32, Error>`
- `UnitsQuery.get_team_unit_def_count` (params: team_id:i32, unit_def_id:i32) → `Result<u32, Error>`
- `UnitsQuery.get_team_units` (params: team_id:i32) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_team_units_by_defs` (params: team_id:i32, unit_def_ids:&[i32]) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_team_units_counts` (params: team_id:i32) → `Result<Vec<sys::UnitDefCount>, Error>`
- `UnitsQuery.get_team_units_sorted` (params: team_id:i32) → `Result<Vec<sys::TeamUnitsByDef>, Error>`
- `UnitsQuery.get_unit_array_centroid` (params: unit_ids:&[i32]) → `Result<sys::Float3, Error>`
- `UnitsQuery.get_unit_map_centroid` (params: unit_ids:&[i32]) → `Result<sys::Float3, Error>`
- `UnitsQuery.get_unit_nearest_ally` (params: unit_id:i32, range:f32) → `Result<i32, Error>`
- `UnitsQuery.get_unit_nearest_enemy` (params: unit_id:i32, range:f32, use_los:bool, sphere_dist_test:bool, check_sight_dist:bool) → `Result<i32, Error>`
- `UnitsQuery.get_unit_separation` (params: unit_id1:i32, unit_id2:i32, positional:bool, check_map:bool) → `Result<f32, Error>`
- `UnitsQuery.get_units_in_box` (params: xmin:f32, ymin:f32, zmin:f32, xmax:f32, ymax:f32, zmax:f32, allegiance:i32) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_cylinder` (params: x:f32, z:f32, radius:f32, allegiance:i32) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_planes` (params: planes:sys::PlanesQuery, allegiance:i32) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_rectangle` (params: xmin:f32, zmin:f32, xmax:f32, zmax:f32, allegiance:i32) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_units_in_sphere` (params: x:f32, y:f32, z:f32, radius:f32, allegiance:i32) → `Result<Vec<i32>, Error>`
- `UnitsQuery.valid_unit_id` (params: unit_id:i32) → `Result<bool, Error>`

## UnitsWeapons (11 functions)

- `UnitsWeapons.get_unit_max_range` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsWeapons.get_unit_weapon_can_fire` (params: unit_id:i32, weapon_num:i32) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_count` (params: unit_id:i32) → `Result<u32, Error>`
- `UnitsWeapons.get_unit_weapon_damages` (params: unit_id:i32, weapon_num:i32) → `Result<sys::UnitWeaponDamages, Error>`
- `UnitsWeapons.get_unit_weapon_have_free_line_of_fire` (params: unit_id:i32, weapon_num:i32, target_id:i32, target_pos:sys::Float3, is_ground_target:bool) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_state` (params: unit_id:i32, weapon_num:i32, key:&str) → `Result<sys::UnitWeaponState, Error>`
- `UnitsWeapons.get_unit_weapon_target` (params: unit_id:i32, weapon_num:i32) → `Result<sys::UnitWeaponTarget, Error>`
- `UnitsWeapons.get_unit_weapon_test_range` (params: unit_id:i32, weapon_num:i32, target_pos:sys::Float3) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_test_target` (params: unit_id:i32, weapon_num:i32, target_id:i32, target_pos:sys::Float3, is_ground_target:bool) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_try_target` (params: unit_id:i32, weapon_num:i32, target_id:i32, target_pos:sys::Float3, user_target:bool, is_ground_target:bool) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_vectors` (params: unit_id:i32, weapon_num:i32) → `Result<sys::UnitWeaponVectors, Error>`

## UnsyncedCtrl (79 functions)

- `UnsyncedCtrl.assign_mouse_cursor` (params: command_name:&str, cursor_file_name:&str, overwrite:bool, hot_spot_top_left:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.deselect_unit_map` (params: unit_ids:&[i32]) → `Result<bool, Error>`
- `UnsyncedCtrl.draw_unit_commands` (params: unit_ids:&[i32], table_or_array:bool, queue_draw_depth:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.force_layout_update` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.force_tesselation_update` (params: normal:bool, shadow:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.load_cmd_colors_config` (params: filename:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.load_ctrl_panel_config` (params: filename:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.load_model_textures` (params: model_name:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.pause_dolly_camera` (params: percent:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.preload_feature_def_model` (params: def_id:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.preload_unit_def_model` (params: def_id:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.replace_mouse_cursor` (params: old_cursor_file_name:&str, new_cursor_file_name:&str, hot_spot_top_left:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.resume_dolly_camera` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.run_dolly_camera` (params: runtime_ms:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.sdlset_text_input_rect` (params: x:i32, y:i32, w:i32, h:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.sdlstart_text_input` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.sdlstop_text_input` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.select_unit_map` (params: unit_ids:&[i32], append:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_active_command` (params: cmd_index:i32, button:i32, left_click:bool, right_click:bool, alt:bool, ctrl:bool, meta:bool, shift:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_atmosphere` (params: params:sys::AtmosphereParams) → `Result<bool, Error>`
- `UnsyncedCtrl.set_auto_show_metal` (params: enable:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_box_selection_by_engine` (params: state:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_build_facing` (params: facing:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_build_spacing` (params: spacing:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_camera_offset` (params: pos_offset:sys::Float3, tilt_offset:sys::Float3) → `Result<bool, Error>`
- `UnsyncedCtrl.set_clipboard` (params: text:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_custom_command_draw_data` (params: cmd_id:i32, cmd_reference:sys::DefRef, color:sys::Float4, show_area:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_custom_palette_color` (params: index:i32, r:f32, g:f32, b:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_dolly_camera_curve` (params: degree:i32, control_points:&[sys::Float4], knots:&[f32]) → `Result<bool, Error>`
- `UnsyncedCtrl.set_dolly_camera_look_curve` (params: degree:i32, control_points:&[sys::Float4], knots:&[f32]) → `Result<bool, Error>`
- `UnsyncedCtrl.set_dolly_camera_look_position` (params: position:sys::Float3) → `Result<bool, Error>`
- `UnsyncedCtrl.set_dolly_camera_look_unit` (params: unit_id:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_dolly_camera_mode` (params: mode:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_dolly_camera_position` (params: position:sys::Float3) → `Result<bool, Error>`
- `UnsyncedCtrl.set_dolly_camera_relative_mode` (params: mode:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_draw_ground` (params: draw_ground:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_draw_ground_deferred` (params: draw_deferred:bool, draw_forward:bool) → `Result<(bool, bool, bool), Error>`
- `UnsyncedCtrl.set_draw_models_deferred` (params: draw_units_deferred:bool, draw_features_deferred:bool, draw_units_forward:bool, draw_features_forward:bool) → `Result<(bool, bool, bool, bool, bool), Error>`
- `UnsyncedCtrl.set_draw_selection_info` (params: draw:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_draw_sky` (params: draw_sky:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_draw_water` (params: draw_water:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_engine_build_square_rendering` (params: enabled:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_feature_always_update_matrix` (params: feature_id:i32, enable:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_feature_engine_draw_mask` (params: feature_id:i32, mask:u32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_feature_fade` (params: feature_id:i32, allow:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_feature_no_draw` (params: feature_id:i32, no_draw:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_feature_palette_index` (params: feature_id:i32, custom_index:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_last_message_position` (params: pos:sys::Float3) → `Result<bool, Error>`
- `UnsyncedCtrl.set_los_view_colors` (params: always:sys::RgbColor, los:sys::RgbColor, radar:sys::RgbColor, jam:sys::RgbColor, radar2:sys::RgbColor) → `Result<bool, Error>`
- `UnsyncedCtrl.set_map_rendering_params` (params: params:sys::MapRenderingParams) → `Result<bool, Error>`
- `UnsyncedCtrl.set_map_shader` (params: standard_shader_id:i32, deferred_shader_id:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_map_shading_texture` (params: tex_type:&str, tex_name:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_mini_map_rotation` (params: radians:f32) → `Result<(bool, i32), Error>`
- `UnsyncedCtrl.set_mouse_cursor` (params: cursor_name:&str, scale:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_nano_projectile_params` (params: r:f32, v:f32, a:f32, rand_r:f32, rand_v:f32, rand_a:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_sky_box_texture` (params: tex_name:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_sun_direction` (params: dir:sys::Float3, intensity:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_sun_lighting` (params: params:sys::SunLightingParams) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_always_update_matrix` (params: unit_id:i32, always_update_matrix:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_def_icon` (params: unit_def_id:i32, icon_name:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_def_image` (params: unit_def_id:i32, image:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_engine_draw_mask` (params: unit_id:i32, draw_mask:u32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_icon` (params: unit_id:i32, icon_name:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_icon_draw` (params: unit_id:i32, draw_icon:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_leave_tracks` (params: unit_id:i32, leave_tracks:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_no_draw` (params: unit_id:i32, no_draw:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_no_group` (params: unit_id:i32, no_group:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_no_minimap` (params: unit_id:i32, no_minimap:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_no_select` (params: unit_id:i32, no_select:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_unit_palette_index` (params: unit_id:i32, custom_index:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_video_capturing_mode` (params: allow_capture_mode:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_video_capturing_time_offset` (params: time_offset:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_water_params` (params: params:sys::WaterParams) → `Result<bool, Error>`
- `UnsyncedCtrl.set_window_geometry` (params: display_index:i32, window_pos_x:i32, window_pos_y:i32, window_size_x:i32, window_size_y:i32, full_screen:bool, borderless:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.set_window_maximized` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.set_window_minimized` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.set_wmcaption` (params: title:&str, title_short:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_wmicon` (params: icon_file_name:&str, force_resolution:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.warp_mouse` (params: x:i32, y:i32) → `Result<bool, Error>`

## UnsyncedRead (18 functions)

- `UnsyncedRead.get_active_cmd_desc` (params: unit_id:i32) → `Result<bool, Error>`
- `UnsyncedRead.get_active_cmd_descs` (params: ) → `Result<bool, Error>`
- `UnsyncedRead.get_box_selection_by_engine` (params: ) → `Result<bool, Error>`
- `UnsyncedRead.get_build_facing` (params: ) → `Result<i32, Error>`
- `UnsyncedRead.get_build_spacing` (params: ) → `Result<i32, Error>`
- `UnsyncedRead.get_clipboard` (params: ) → `Result<Option<String>, Error>`
- `UnsyncedRead.get_cmd_desc_index` (params: cmd_id:i32) → `Result<i32, Error>`
- `UnsyncedRead.get_custom_palette_color` (params: index:i32) → `Result<(f32, f32, f32, bool), Error>`
- `UnsyncedRead.get_draw_selection_info` (params: ) → `Result<bool, Error>`
- `UnsyncedRead.get_feature_palette_index` (params: feature_id:i32) → `Result<(i32, bool), Error>`
- `UnsyncedRead.get_last_message_positions` (params: ) → `Result<Vec<sys::Float3>, Error>`
- `UnsyncedRead.get_nano_projectile_params` (params: ) → `Result<(f32, f32, f32, f32, f32, f32), Error>`
- `UnsyncedRead.get_piece_projectile_name` (params: projectile_id:i32) → `Result<Option<String>, Error>`
- `UnsyncedRead.get_team_damage_stats` (params: team_id:i32) → `Result<(f32, f32, bool), Error>`
- `UnsyncedRead.get_unit_palette_index` (params: unit_id:i32) → `Result<(i32, bool), Error>`
- `UnsyncedRead.is_unit_allied` (params: unit_id:i32) → `Result<bool, Error>`
- `UnsyncedRead.is_unit_selected` (params: unit_id:i32) → `Result<bool, Error>`
- `UnsyncedRead.solve_nurbscurve` (params: degree:i32, points:&[sys::Float4], knots:&[f32], segments:i32) → `Result<(Vec<sys::Float3>, bool), Error>`

## Utils (6 functions)

- `Utils.closest_build_pos` (params: team_id:i32, unit_def_id:i32, pos:sys::Float3, search_radius:f32, min_dist:i32, facing:i32) → `Result<sys::Float3, Error>`
- `Utils.get_cegid` (params: ceg_name:&str) → `Result<i32, Error>`
- `Utils.get_unit_def_dimensions` (params: unit_def_id:i32) → `Result<sys::UnitDefDimensions, Error>`
- `Utils.pos2_build_pos` (params: unit_def_id:i32, pos:sys::Float3, facing:i32) → `Result<sys::Float3, Error>`
- `Utils.test_build_order` (params: unit_def_id:i32, pos:sys::Float3, facing:i32) → `Result<(i32, bool, i32), Error>`
- `Utils.test_move_order` (params: unit_def_id:i32, pos:sys::Float3, dir:sys::Float3, test_terrain:bool, test_objects:bool, center_only:bool) → `Result<bool, Error>`

## Vfs (14 functions)

- `Vfs.create_dir` (params: path:&str) → `Result<bool, Error>`
- `Vfs.extract_mod_archive_file` (params: path:&str) → `Result<bool, Error>`
- `Vfs.file_exists` (params: path:&str) → `Result<bool, Error>`
- `Vfs.get_archives` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.get_file_info` (params: path:&str) → `Result<(sys::FileInfo, bool), Error>`
- `Vfs.get_file_size` (params: path:&str) → `Result<u32, Error>`
- `Vfs.get_games` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.get_map_square_texture` (params: tex_square_x:i32, tex_square_y:i32, lod_min:i32, texture_name:&str, lod_max:i32) → `Result<bool, Error>`
- `Vfs.get_maps` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.is_directory` (params: path:&str) → `Result<bool, Error>`
- `Vfs.list_dir` (params: path:&str, pattern:&str) → `Result<Vec<sys::DirEntry>, Error>`
- `Vfs.read_file` (params: path:&str) → `Result<Vec<u8>, Error>`
- `Vfs.read_file_as_string` (params: path:&str) → `Result<Option<String>, Error>`
- `Vfs.set_map_square_texture` (params: tex_square_x:i32, tex_square_y:i32, texture_name:&str) → `Result<bool, Error>`

## WeaponDefs (10 functions)

- `WeaponDefs.get_weapon_def_by_id` (params: weapon_def_id:i32) → `Result<(sys::WeaponDefInfo, bool), Error>`
- `WeaponDefs.get_weapon_def_count` (params: ) → `Result<u32, Error>`
- `WeaponDefs.get_weapon_def_custom_param` (params: weapon_def_id:i32, key:&str) → `Result<Option<String>, Error>`
- `WeaponDefs.get_weapon_def_custom_param_keys` (params: weapon_def_id:i32) → `Result<Vec<String>, Error>`
- `WeaponDefs.get_weapon_def_damage` (params: weapon_def_id:i32) → `Result<f32, Error>`
- `WeaponDefs.get_weapon_def_id` (params: weapon_def_name:&str) → `Result<i32, Error>`
- `WeaponDefs.get_weapon_def_ids` (params: ) → `Result<Vec<i32>, Error>`
- `WeaponDefs.get_weapon_def_name` (params: weapon_def_id:i32) → `Result<Option<String>, Error>`
- `WeaponDefs.get_weapon_def_range` (params: weapon_def_id:i32) → `Result<f32, Error>`
- `WeaponDefs.valid_weapon_def_id` (params: weapon_def_id:i32) → `Result<bool, Error>`


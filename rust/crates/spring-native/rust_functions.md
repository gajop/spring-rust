# Rust API Functions

Total APIs: 53

Total Functions: 1422

---

## Camera (10 functions)

- `Camera.get_camera_direction` (params: ) → `Result<sys::Float3, Error>`
- `Camera.get_camera_fov` (params: ) → `Result<f32, Error>`
- `Camera.get_camera_names` (params: ) → `Result<Vec<String>, Error>`
- `Camera.get_camera_position` (params: ) → `Result<sys::Float3, Error>`
- `Camera.get_camera_state` (params: use_table:bool) → `Result<sys::CameraState, Error>`
- `Camera.get_pixel_dir` (params: screen_x:f32, screen_y:f32) → `Result<sys::Float3, Error>`
- `Camera.set_camera_state` (params: state:sys::CameraState, transition_time:f32, transition_time_factor:f32, transition_time_exponent:f32) → `Result<bool, Error>`
- `Camera.set_camera_target` (params: target:sys::Float3, options:SetCameraTargetOptions) → `Result<bool, Error>`
- `Camera.trace_screen_ray` (params: screen_x:f32, screen_y:f32, options:TraceScreenRayOptions) → `Result<(i32, i32, sys::Float3), Error>`
- `Camera.world_to_screen_coords` (params: world_pos:sys::Float3) → `Result<(sys::Float3, bool), Error>`

## CobScript (2 functions)

- `CobScript.call_cobscript` (params: unit_id:i32, func:sys::CobFunctionRef, ret_args:u32, args:&[i32]) → `Result<(i32, Vec<i32>), Error>`
- `CobScript.get_cobscript_id` (params: unit_id:i32, func_name:&str) → `Result<i32, Error>`

## Config (10 functions)

- `Config.get_config_float` (params: key:&str, default_value:Option<f32>) → `Result<(f32, bool), Error>`
- `Config.get_config_int` (params: key:&str, default_value:Option<i32>) → `Result<(i32, bool), Error>`
- `Config.get_config_parameters` (params: ) → `Result<Vec<ConfigParameter>, Error>`
- `Config.get_config_params` (params: ) → `Result<Vec<sys::ConfigParam>, Error>`
- `Config.get_config_string` (params: key:&str, default_value:Option<&str>) → `Result<(Option<String>, bool), Error>`
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

## Encoding (6 functions)

- `Encoding.decode_base64` (params: text:&str) → `Result<Vec<u8>, Error>`
- `Encoding.decode_base64_url` (params: text:&str) → `Result<Vec<u8>, Error>`
- `Encoding.encode_base64` (params: text:&[u8], strip_padding:bool) → `Result<Option<String>, Error>`
- `Encoding.encode_base64_url` (params: text:&[u8]) → `Result<Option<String>, Error>`
- `Encoding.is_valid_base64` (params: text:&str) → `Result<bool, Error>`
- `Encoding.is_valid_base64_url` (params: text:&str) → `Result<bool, Error>`

## FeatureControl (32 functions)

- `FeatureControl.add_feature_damage` (params: feature_id:i32, damage:f32, paralyze_time:f32, weapon_def_id:i32, attacker_id:i32, impulse:sys::Float3) → `Result<bool, Error>`
- `FeatureControl.create_feature` (params: feature_def:sys::DefRef, pos:sys::Float3, facing:i32, team_id:i32, feature_id:i32) → `Result<i32, Error>`
- `FeatureControl.create_feature_wreck` (params: feature_id:i32, wreck_level:i32, do_smoke:bool) → `Result<i32, Error>`
- `FeatureControl.create_unit_wreck` (params: unit_id:i32, wreck_level:i32, do_smoke:bool) → `Result<i32, Error>`
- `FeatureControl.destroy_feature` (params: feature_id:i32) → `Result<bool, Error>`
- `FeatureControl.set_feature_always_visible` (params: feature_id:i32, always_visible:bool) → `Result<bool, Error>`
- `FeatureControl.set_feature_blocking` (params: feature_id:i32, options:SetFeatureBlockingOptions) → `Result<bool, Error>`
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

## FeatureDefs (11 functions)

- `FeatureDefs.get_feature_def_by_id` (params: feature_def_id:i32) → `Result<(sys::FeatureDefInfo, bool), Error>`
- `FeatureDefs.get_feature_def_count` (params: ) → `Result<u32, Error>`
- `FeatureDefs.get_feature_def_custom_param` (params: feature_def_id:i32, key:&str) → `Result<Option<String>, Error>`
- `FeatureDefs.get_feature_def_custom_param_keys` (params: feature_def_id:i32) → `Result<Vec<String>, Error>`
- `FeatureDefs.get_feature_def_energy` (params: feature_def_id:i32) → `Result<f32, Error>`
- `FeatureDefs.get_feature_def_idby_name` (params: feature_def_name:&str) → `Result<i32, Error>`
- `FeatureDefs.get_feature_def_ids` (params: ) → `Result<Vec<i32>, Error>`
- `FeatureDefs.get_feature_def_info` (params: feature_def_id:i32) → `Result<Option<FeatureDefInfo>, Error>`
- `FeatureDefs.get_feature_def_metal` (params: feature_def_id:i32) → `Result<f32, Error>`
- `FeatureDefs.get_feature_def_name` (params: feature_def_id:i32) → `Result<Option<String>, Error>`
- `FeatureDefs.valid_feature_def_id` (params: feature_def_id:i32) → `Result<bool, Error>`

## Features (38 functions)

- `Features.clear_features_previous_draw_flag` (params: ) → `Result<bool, Error>`
- `Features.get_all_features` (params: ) → `Result<Vec<i32>, Error>`
- `Features.get_feature_ally_team` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_always_update_matrix` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_blocking` (params: feature_id:i32) → `Result<sys::FeatureBlockingState, Error>`
- `Features.get_feature_collision_volume_data` (params: feature_id:i32) → `Result<sys::CollisionVolumeData, Error>`
- `Features.get_feature_def_id` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_direction` (params: feature_id:i32) → `Result<sys::Float3, Error>`
- `Features.get_feature_draw_flag` (params: feature_id:i32) → `Result<u8, Error>`
- `Features.get_feature_engine_draw_mask` (params: feature_id:i32) → `Result<u32, Error>`
- `Features.get_feature_fire_time` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_heading` (params: feature_id:i32) → `Result<i32, Error>`
- `Features.get_feature_health` (params: feature_id:i32) → `Result<sys::FeatureHealth, Error>`
- `Features.get_feature_height` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_last_attacked_piece` (params: feature_id:i32) → `Result<sys::FeatureLastHitPiece, Error>`
- `Features.get_feature_lua_draw` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_mass` (params: feature_id:i32) → `Result<f32, Error>`
- `Features.get_feature_no_draw` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_no_select` (params: feature_id:i32) → `Result<bool, Error>`
- `Features.get_feature_piece_collision_volume_data` (params: feature_id:i32, piece_num:i32) → `Result<sys::CollisionVolumeData, Error>`
- `Features.get_feature_position` (params: feature_id:i32) → `Result<sys::Float3, Error>`
- `Features.get_feature_position_ext` (params: feature_id:i32) → `Result<sys::FeaturePositionExt, Error>`
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

## Game (37 functions)

- `Game.are_helper_ais_enabled` (params: ) → `Result<bool, Error>`
- `Game.fixed_allies` (params: ) → `Result<bool, Error>`
- `Game.get_ally_team_start_box` (params: ally_team_id:i32) → `Result<(sys::StartBox, bool), Error>`
- `Game.get_facing_from_heading` (params: heading:i32) → `Result<i32, Error>`
- `Game.get_gaia_team_id` (params: ) → `Result<i32, Error>`
- `Game.get_game_frame` (params: ) → `Result<(u32, u32), Error>`
- `Game.get_game_map_info` (params: ) → `Result<sys::GameMapInfo, Error>`
- `Game.get_game_map_info_owned` (params: ) → `Result<GameMapInfo, Error>`
- `Game.get_game_mod_info` (params: ) → `Result<sys::GameModInfo, Error>`
- `Game.get_game_mod_info_owned` (params: ) → `Result<GameModInfo, Error>`
- `Game.get_game_rules_info` (params: ) → `Result<sys::GameRulesInfo, Error>`
- `Game.get_game_rules_resource_info` (params: ) → `Result<sys::GameRulesResourceInfo, Error>`
- `Game.get_game_seconds` (params: ) → `Result<f32, Error>`
- `Game.get_game_setup_info` (params: ) → `Result<sys::GameSetupInfo, Error>`
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
- `Game.get_side_data_by_index_owned` (params: side_index:u32) → `Result<SideData, Error>`
- `Game.get_side_data_count` (params: ) → `Result<u32, Error>`
- `Game.get_side_data_owned` (params: side_name:&str) → `Result<SideData, Error>`
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
- `GameConfig.set_god_mode` (params: options:SetGodModeOptions) → `Result<bool, Error>`
- `GameConfig.set_no_pause` (params: no_pause:bool) → `Result<bool, Error>`
- `GameConfig.set_radar_error_params` (params: ally_team_id:i32, ally_team_error_size:f32, base_error_size:f32, base_error_mult:f32) → `Result<bool, Error>`
- `GameConfig.set_square_building_mask` (params: x:i32, z:i32, mask:u16) → `Result<bool, Error>`

## Gfx (237 functions)

- `Gfx.active_fbo` (params: fbo_id:u32, target:u32, identities:bool, callback:F) → `Result<(), Error>`
- `Gfx.active_shader` (params: shader_id:u32, callback:F) → `Result<(), Error>`
- `Gfx.active_texture` (params: tex_num:i32) → `Result<(), Error>`
- `Gfx.add_atlas_texture` (params: atlas_name:&str, texture_name:&str) → `Result<(), Error>`
- `Gfx.add_fallback_font` (params: value:&str) → `Result<bool, Error>`
- `Gfx.add_feature_defs_to_submission_vao` (params: vao_id:u32, ids:&[u32]) → `Result<u32, Error>`
- `Gfx.add_features_to_submission_vao` (params: vao_id:u32, ids:&[u32]) → `Result<u32, Error>`
- `Gfx.add_unit_defs_to_submission_vao` (params: vao_id:u32, ids:&[u32]) → `Result<u32, Error>`
- `Gfx.add_units_to_submission_vao` (params: vao_id:u32, ids:&[u32]) → `Result<u32, Error>`
- `Gfx.alpha_test` (params: enable:bool, func:u32, r#ref:f32) → `Result<(), Error>`
- `Gfx.alpha_to_coverage` (params: value:bool) → `Result<(), Error>`
- `Gfx.attach_index_buffer_vao` (params: vao_id:u32, vbo_id:u32) → `Result<(), Error>`
- `Gfx.attach_instance_buffer_vao` (params: vao_id:u32, vbo_id:u32) → `Result<(), Error>`
- `Gfx.attach_vertex_buffer_vao` (params: vao_id:u32, vbo_id:u32) → `Result<(), Error>`
- `Gfx.begin_end` (params: primitive:u32, callback:F) → `Result<(), Error>`
- `Gfx.begin_text` (params: value:bool) → `Result<(), Error>`
- `Gfx.billboard` (params: ) → `Result<(), Error>`
- `Gfx.bind_buffer_range_vbo` (params: vbo_id:u32, binding_index:u32, element_offset:i32, element_count:i32, target:u32, bind:bool) → `Result<i32, Error>`
- `Gfx.bind_image_texture` (params: unit:u32, name:&str, level:i32, layer:i32, layered:bool, access:u32, format:u32) → `Result<(), Error>`
- `Gfx.bind_texture` (params: name:&str, tex_num:i32, enable:bool) → `Result<bool, Error>`
- `Gfx.blend_equation` (params: mode:u32) → `Result<(), Error>`
- `Gfx.blend_equation_separate` (params: mode_rgb:u32, mode_alpha:u32) → `Result<(), Error>`
- `Gfx.blend_func` (params: src:u32, dst:u32) → `Result<(), Error>`
- `Gfx.blend_func_separate` (params: src_rgb:u32, dst_rgb:u32, src_alpha:u32, dst_alpha:u32) → `Result<(), Error>`
- `Gfx.blending` (params: value:bool) → `Result<(), Error>`
- `Gfx.blit_fbo` (params: src_fboid:u32, dst_fboid:u32, x0_src:i32, y0_src:i32, x1_src:i32, y1_src:i32, x0_dst:i32, y0_dst:i32, x1_dst:i32, y1_dst:i32, mask:u32, filter:u32) → `Result<(), Error>`
- `Gfx.call_list` (params: value:u32) → `Result<(), Error>`
- `Gfx.change_texture_params` (params: name:&str, params:sys::GfxTextureParams) → `Result<(), Error>`
- `Gfx.clear` (params: bits:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.clear_attachment_fbo` (params: target:u32, attachment:u32, values:[f32; 4], count:u32) → `Result<bool, Error>`
- `Gfx.clear_fallback_fonts` (params: ) → `Result<(), Error>`
- `Gfx.clear_submission_vao` (params: value:u32) → `Result<(), Error>`
- `Gfx.clear_vbo` (params: value:u32) → `Result<(), Error>`
- `Gfx.clip_distance` (params: index:u32, enable:bool) → `Result<(), Error>`
- `Gfx.clip_plane` (params: plane:u32, equation:[f32; 4]) → `Result<(), Error>`
- `Gfx.color` (params: r:f32, g:f32, b:f32, a:f32) → `Result<(), Error>`
- `Gfx.color_mask` (params: options:GfxColorMaskOptions) → `Result<(), Error>`
- `Gfx.config_mini_map` (params: px:i32, py:i32, sx:i32, sy:i32) → `Result<(), Error>`
- `Gfx.copy_to_texture` (params: name:&str, xoff:i32, yoff:i32, x:i32, y:i32, width:i32, height:i32, target:u32, level:u32) → `Result<(), Error>`
- `Gfx.copy_to_vbo` (params: source_vboid:u32, destination_vboid:u32, copy_size_in_bytes:i32) → `Result<bool, Error>`
- `Gfx.create_fbo` (params: target:u32, attachments:&[sys::GfxFBOAttachment], draw_buffers:&[u32], read_buffer:u32) → `Result<(u32, u32), Error>`
- `Gfx.create_list` (params: callback:F) → `Result<u32, Error>`
- `Gfx.create_query` (params: ) → `Result<u32, Error>`
- `Gfx.create_rbo` (params: xsize:i32, ysize:i32, target:u32, format:u32, samples:i32) → `Result<u32, Error>`
- `Gfx.create_shader` (params: definitions:&str, vertex:&str, tcs:&str, tes:&str, geometry:&str, fragment:&str, compute:&str, options:GfxCreateShaderOptions) → `Result<(u32, u32), Error>`
- `Gfx.create_texture` (params: xsize:i32, ysize:i32, zsize:i32, params:sys::GfxTextureParams) → `Result<Option<String>, Error>`
- `Gfx.create_texture_atlas` (params: xsize:i32, ysize:i32, alloc_type:i32) → `Result<Option<String>, Error>`
- `Gfx.culling` (params: value:bool) → `Result<(), Error>`
- `Gfx.define_vbo` (params: vbo_id:u32, elements_count:i32, element_array:bool, index_type:u32, use_default_attributes:bool, default_attribute_count:u32, attributes:&[sys::GfxVBOAttributeOptions]) → `Result<(), Error>`
- `Gfx.delete_fbo` (params: value:u32) → `Result<(), Error>`
- `Gfx.delete_font` (params: font_id:u32) → `Result<(), Error>`
- `Gfx.delete_list` (params: value:u32) → `Result<(), Error>`
- `Gfx.delete_query` (params: value:u32) → `Result<(), Error>`
- `Gfx.delete_rbo` (params: value:u32) → `Result<(), Error>`
- `Gfx.delete_shader` (params: shader_id:u32) → `Result<bool, Error>`
- `Gfx.delete_texture` (params: name:&str) → `Result<bool, Error>`
- `Gfx.delete_texture_atlas` (params: name:&str) → `Result<bool, Error>`
- `Gfx.delete_texture_fbo` (params: name:&str) → `Result<bool, Error>`
- `Gfx.delete_vao` (params: value:u32) → `Result<(), Error>`
- `Gfx.delete_vbo` (params: value:u32) → `Result<(), Error>`
- `Gfx.depth_clamp` (params: value:bool) → `Result<(), Error>`
- `Gfx.depth_mask` (params: value:bool) → `Result<(), Error>`
- `Gfx.depth_test` (params: options:GfxDepthTestOptions) → `Result<(), Error>`
- `Gfx.dispatch_compute` (params: num_group_x:u32, num_group_y:u32, num_group_z:u32, barriers:u32) → `Result<(), Error>`
- `Gfx.download_vbo` (params: vbo_id:u32, attribute_index:i32, element_offset:i32, element_count:i32, force_gpuread:bool) → `Result<Vec<f32>, Error>`
- `Gfx.draw_arrays_vao` (params: vao_id:u32, mode:u32, vertex_count:i32, vertex_first:i32, instance_count:i32, instance_first:i32) → `Result<(), Error>`
- `Gfx.draw_elements_vao` (params: vao_id:u32, mode:u32, draw_count:i32, base_index:i32, instance_count:i32, base_vertex:i32, base_instance:i32) → `Result<(), Error>`
- `Gfx.draw_func_at_unit` (params: unit_id:i32, use_mid_pos:bool, callback:F) → `Result<(), Error>`
- `Gfx.draw_ground_circle` (params: pos:sys::Float3, radius:f32, resolution:i32, ballistic:bool, slope:f32, gravity:f32, weapon_def_id:i32) → `Result<(), Error>`
- `Gfx.draw_ground_quad` (params: x0:f32, z0:f32, x1:f32, z1:f32, use_tex_coords:bool, tu0:f32, tv0:f32, tu1:f32, tv1:f32) → `Result<(), Error>`
- `Gfx.draw_list_at_unit` (params: unit_id:i32, list_id:u32, use_mid_pos:bool, scale:sys::Float3, degrees:f32, rot:sys::Float3) → `Result<(), Error>`
- `Gfx.draw_mini_map` (params: value:bool) → `Result<(), Error>`
- `Gfx.dump_definition_vbo` (params: value:u32) → `Result<(), Error>`
- `Gfx.edge_flag` (params: value:bool) → `Result<(), Error>`
- `Gfx.end_text` (params: ) → `Result<(), Error>`
- `Gfx.feature` (params: feature_id:i32, options:GfxFeatureDrawOptions) → `Result<(), Error>`
- `Gfx.feature_mult_matrix` (params: value:i32) → `Result<(), Error>`
- `Gfx.feature_piece` (params: object_id:i32, piece_id:i32) → `Result<(), Error>`
- `Gfx.feature_piece_matrix` (params: object_id:i32, piece_id:i32) → `Result<(), Error>`
- `Gfx.feature_piece_mult_matrix` (params: object_id:i32, piece_id:i32) → `Result<(), Error>`
- `Gfx.feature_raw` (params: feature_id:i32, options:GfxFeatureDrawOptions) → `Result<(), Error>`
- `Gfx.feature_shape` (params: def_id:i32, team_id:i32, options:GfxObjectShapeOptions) → `Result<(), Error>`
- `Gfx.feature_shape_textures` (params: object_id:i32, push:bool) → `Result<(), Error>`
- `Gfx.feature_textures` (params: object_id:i32, push:bool) → `Result<(), Error>`
- `Gfx.finalize_texture_atlas` (params: name:&str) → `Result<bool, Error>`
- `Gfx.finish` (params: ) → `Result<(), Error>`
- `Gfx.flush` (params: ) → `Result<(), Error>`
- `Gfx.fog` (params: value:bool) → `Result<(), Error>`
- `Gfx.fog_coord` (params: value:f32) → `Result<(), Error>`
- `Gfx.font_begin` (params: font_id:u32, user_defined_blending:bool) → `Result<(), Error>`
- `Gfx.font_bind_texture` (params: font_id:u32) → `Result<(), Error>`
- `Gfx.font_end` (params: font_id:u32) → `Result<(), Error>`
- `Gfx.font_get_text_height` (params: font_id:u32, text:&str, x:f32, y:f32, size:f32, options:&str) → `Result<(f32, f32, i32), Error>`
- `Gfx.font_get_text_width` (params: font_id:u32, text:&str, x:f32, y:f32, size:f32, options:&str) → `Result<f32, Error>`
- `Gfx.font_print` (params: font_id:u32, text:&str, x:f32, y:f32, size:f32, options:&str) → `Result<(), Error>`
- `Gfx.font_print_world` (params: font_id:u32, text:&str, pos:sys::Float3, size:f32, options:&str) → `Result<(), Error>`
- `Gfx.font_set_auto_outline_color` (params: font_id:u32, enable:bool) → `Result<(), Error>`
- `Gfx.font_set_outline_color` (params: font_id:u32, r:f32, g:f32, b:f32, a:f32) → `Result<(), Error>`
- `Gfx.font_set_text_color` (params: font_id:u32, r:f32, g:f32, b:f32, a:f32) → `Result<(), Error>`
- `Gfx.font_submit_buffered` (params: font_id:u32, options:GfxFontSubmitBufferedOptions) → `Result<(), Error>`
- `Gfx.font_wrap_text` (params: font_id:u32, text:&str, max_width:f32, max_height:f32, size:f32) → `Result<(Option<String>, i32), Error>`
- `Gfx.frustum` (params: left:f32, right:f32, bottom:f32, top:f32, near_val:f32, far_val:f32) → `Result<(), Error>`
- `Gfx.generate_mipmap` (params: name:&str) → `Result<(), Error>`
- `Gfx.get_active_uniforms` (params: shader_id:u32) → `Result<Vec<sys::GfxActiveUniformEntry>, Error>`
- `Gfx.get_atlas_texture` (params: atlas_name:&str, texture_name:&str) → `Result<(f32, f32, f32, f32, i32), Error>`
- `Gfx.get_atmosphere` (params: key:&str, mode:&str) → `Result<([f32`
- `Gfx.get_console_commands` (params: ) → `Result<Vec<sys::GfxConsoleCommandEntry>, Error>`
- `Gfx.get_engine_atlas_textures` (params: name:&str) → `Result<Vec<sys::GfxAtlasTextureEntry>, Error>`
- `Gfx.get_engine_model_uniform_data_def` (params: ) → `Result<Option<String>, Error>`
- `Gfx.get_engine_model_uniform_data_size` (params: ) → `Result<(u32, u32), Error>`
- `Gfx.get_engine_texture_names` (params: ) → `Result<Vec<String>, Error>`
- `Gfx.get_engine_uniform_buffer_def` (params: index:i32) → `Result<Option<String>, Error>`
- `Gfx.get_fixed_state` (params: param:&str) → `Result<([bool`
- `Gfx.get_font_info` (params: font_id:u32) → `Result<(Option<String>, Option<String>, Option<String>, f32, f32, f32, f32, f32, i32, i32), Error>`
- `Gfx.get_global_tex_coords` (params: value:&str) → `Result<(f32, f32, f32, f32, i32), Error>`
- `Gfx.get_global_tex_names` (params: ) → `Result<Vec<sys::GfxAtlasTextureEntry>, Error>`
- `Gfx.get_idvbo` (params: value:u32) → `Result<u32, Error>`
- `Gfx.get_map_rendering` (params: key:&str, mode:&str) → `Result<([f32`
- `Gfx.get_matrix_data` (params: mode:u32) → `Result<[f32`
- `Gfx.get_number` (params: pname:u32, max_values:u32) → `Result<([f32`
- `Gfx.get_query` (params: value:u32) → `Result<u32, Error>`
- `Gfx.get_rboinfo` (params: rbo_id:u32) → `Result<(bool, u32, u32, i32, i32, i32), Error>`
- `Gfx.get_screen_view_trans` (params: ) → `Result<(f32, f32, f32), Error>`
- `Gfx.get_shader_log` (params: ) → `Result<Option<String>, Error>`
- `Gfx.get_shadow_map_params` (params: ) → `Result<sys::Float4, Error>`
- `Gfx.get_string` (params: pname:u32) → `Result<Option<String>, Error>`
- `Gfx.get_subroutine_index` (params: shader_id:u32, shader_type:u32, name:&str) → `Result<(i32, bool), Error>`
- `Gfx.get_sun` (params: key:&str, mode:&str) → `Result<([f32`
- `Gfx.get_text_height` (params: value:&str) → `Result<(f32, f32, i32), Error>`
- `Gfx.get_text_width` (params: value:&str) → `Result<f32, Error>`
- `Gfx.get_uniform_location` (params: shader_id:u32, name:&str) → `Result<i32, Error>`
- `Gfx.get_vao` (params: ) → `Result<(u32, u32), Error>`
- `Gfx.get_vbo` (params: target:u32, freq_updated:bool) → `Result<(u32, u32, u32), Error>`
- `Gfx.get_vboinfo` (params: vbo_id:u32) → `Result<(u32, u32, u32, u32, u32, u32), Error>`
- `Gfx.get_view_range` (params: camera_type:i32) → `Result<(f32, f32, f32, f32), Error>`
- `Gfx.get_view_sizes` (params: ) → `Result<(i32, i32), Error>`
- `Gfx.get_water_rendering` (params: key:&str, mode:&str) → `Result<([f32`
- `Gfx.has_extension` (params: value:&str) → `Result<bool, Error>`
- `Gfx.instance_data_from_feature_defs_vbo` (params: vbo_id:u32, ids:&[u32], attribute_index:i32, team_id:i32, element_offset:i32) → `Result<u32, Error>`
- `Gfx.instance_data_from_features_vbo` (params: vbo_id:u32, ids:&[u32], attribute_index:i32, team_id:i32, element_offset:i32) → `Result<u32, Error>`
- `Gfx.instance_data_from_unit_defs_vbo` (params: vbo_id:u32, ids:&[u32], attribute_index:i32, team_id:i32, element_offset:i32) → `Result<u32, Error>`
- `Gfx.instance_data_from_units_vbo` (params: vbo_id:u32, ids:&[u32], attribute_index:i32, team_id:i32, element_offset:i32) → `Result<u32, Error>`
- `Gfx.is_valid_fbo` (params: fbo_id:u32, target:u32) → `Result<(bool, u32), Error>`
- `Gfx.light` (params: light:i32, options:GfxLightOptions, pname:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.lighting` (params: value:bool) → `Result<(), Error>`
- `Gfx.line_stipple` (params: factor:i32, pattern:u16) → `Result<(), Error>`
- `Gfx.line_width` (params: value:f32) → `Result<(), Error>`
- `Gfx.load_font` (params: path:&str, size:i32, outline_width:i32, outline_weight:f32) → `Result<u32, Error>`
- `Gfx.load_identity` (params: ) → `Result<(), Error>`
- `Gfx.load_matrix` (params: values:[f32; 16]) → `Result<(), Error>`
- `Gfx.logic_op` (params: enable:bool, opcode:u32) → `Result<(), Error>`
- `Gfx.material` (params: pname:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.matrix_data_from_projectiles_vbo` (params: vbo_id:u32, ids:&[u32], attribute_index:i32, team_id:i32, element_offset:i32) → `Result<u32, Error>`
- `Gfx.matrix_mode` (params: mode:u32) → `Result<(), Error>`
- `Gfx.memory_barrier` (params: barriers:u32) → `Result<(), Error>`
- `Gfx.models_vbo` (params: value:u32) → `Result<u32, Error>`
- `Gfx.mult_matrix` (params: values:[f32; 16]) → `Result<(), Error>`
- `Gfx.multi_tex_coord` (params: tex_num:i32, s:f32, t:f32, r:f32, q:f32, count:u32) → `Result<(), Error>`
- `Gfx.multi_tex_env` (params: tex_num:i32, target:u32, pname:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.multi_tex_gen` (params: tex_num:i32, target:u32, options:GfxMultiTexGenOptions, pname:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.normal` (params: x:f32, y:f32, z:f32) → `Result<(), Error>`
- `Gfx.object_label` (params: identifier:u32, object_id:u32, label:&str) → `Result<(), Error>`
- `Gfx.ortho` (params: left:f32, right:f32, bottom:f32, top:f32, near_val:f32, far_val:f32) → `Result<(), Error>`
- `Gfx.point_parameter` (params: pname:u32, value:f32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.point_size` (params: value:f32) → `Result<(), Error>`
- `Gfx.point_sprite` (params: value:bool) → `Result<(), Error>`
- `Gfx.polygon_mode` (params: face:u32, mode:u32) → `Result<(), Error>`
- `Gfx.polygon_offset` (params: factor:f32, units:f32) → `Result<(), Error>`
- `Gfx.pop_attrib` (params: ) → `Result<(), Error>`
- `Gfx.pop_debug_group` (params: ) → `Result<(), Error>`
- `Gfx.pop_matrix` (params: ) → `Result<(), Error>`
- `Gfx.push_attrib` (params: value:u32) → `Result<(), Error>`
- `Gfx.push_debug_group` (params: id:u32, message:&str, source_is_third_party:bool) → `Result<(), Error>`
- `Gfx.push_matrix` (params: ) → `Result<(), Error>`
- `Gfx.push_pop_matrix` (params: callback:F) → `Result<(), Error>`
- `Gfx.raw_bind_fbo` (params: bind_default:bool, fbo_id:u32, target:u32, raw_fbo_id:u32) → `Result<(u32, bool), Error>`
- `Gfx.read_pixels` (params: x:i32, y:i32, width:i32, height:i32, format:u32) → `Result<(Vec<f32>, u32), Error>`
- `Gfx.rect` (params: x1:f32, y1:f32, x2:f32, y2:f32) → `Result<(), Error>`
- `Gfx.remove_from_submission_vao` (params: vao_id:u32, index:i32) → `Result<(), Error>`
- `Gfx.render_to_texture` (params: name:&str, callback:F) → `Result<(), Error>`
- `Gfx.reset_matrices` (params: ) → `Result<(), Error>`
- `Gfx.reset_state` (params: ) → `Result<(), Error>`
- `Gfx.rotate` (params: degrees:f32, x:f32, y:f32, z:f32) → `Result<(), Error>`
- `Gfx.run_query` (params: id:u32, callback:F) → `Result<(), Error>`
- `Gfx.save_image` (params: x:i32, y:i32, width:i32, height:i32, filename:&str, options:GfxSaveImageOptions, read_buffer:u32) → `Result<bool, Error>`
- `Gfx.scale` (params: x:f32, y:f32, z:f32) → `Result<(), Error>`
- `Gfx.scissor` (params: x:i32, y:i32, width:i32, height:i32) → `Result<(), Error>`
- `Gfx.secondary_color` (params: x:f32, y:f32, z:f32) → `Result<(), Error>`
- `Gfx.set_fboattachment` (params: fbo_id:u32, attachment:u32, texture_name:&str, texture_target:u32, mip_level:i32, rbo_id:u32, use_rbo:bool) → `Result<(), Error>`
- `Gfx.set_fbodraw_buffers` (params: fbo_id:u32, buffers:&[u32]) → `Result<(), Error>`
- `Gfx.set_fboread_buffer` (params: fbo_id:u32, buffer:u32) → `Result<(), Error>`
- `Gfx.set_feature_buffer_uniforms` (params: object_id:i32, values:&[f32], offset:u32) → `Result<u32, Error>`
- `Gfx.set_geometry_shader_parameter` (params: shader_id:u32, param:u32, value:i32) → `Result<(), Error>`
- `Gfx.set_tesselation_shader_parameter` (params: param:u32, value:i32, values:[f32; 4], value_count:u32, use_float_array:bool) → `Result<(), Error>`
- `Gfx.set_unit_buffer_uniforms` (params: object_id:i32, values:&[f32], offset:u32) → `Result<u32, Error>`
- `Gfx.shade_model` (params: mode:u32) → `Result<(), Error>`
- `Gfx.shape` (params: primitive:u32, vertices:&[sys::GfxVertexData]) → `Result<(), Error>`
- `Gfx.slave_mini_map` (params: value:bool) → `Result<(), Error>`
- `Gfx.stencil_func` (params: func:u32, r#ref:i32, mask:u32) → `Result<(), Error>`
- `Gfx.stencil_func_separate` (params: face:u32, func:u32, r#ref:i32, mask:u32) → `Result<(), Error>`
- `Gfx.stencil_mask` (params: mask:u32) → `Result<(), Error>`
- `Gfx.stencil_mask_separate` (params: face:u32, mask:u32) → `Result<(), Error>`
- `Gfx.stencil_op` (params: fail:u32, zfail:u32, zpass:u32) → `Result<(), Error>`
- `Gfx.stencil_op_separate` (params: face:u32, fail:u32, zfail:u32, zpass:u32) → `Result<(), Error>`
- `Gfx.stencil_test` (params: enable:bool) → `Result<(), Error>`
- `Gfx.submit_vao` (params: value:u32) → `Result<(), Error>`
- `Gfx.swap_buffers` (params: ) → `Result<(), Error>`
- `Gfx.tex_coord` (params: x:f32, y:f32, z:f32, w:f32, count:u32) → `Result<(), Error>`
- `Gfx.tex_env` (params: target:u32, pname:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.tex_gen` (params: target:u32, options:GfxTexGenOptions, pname:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.tex_rect` (params: x1:f32, y1:f32, x2:f32, y2:f32, s1:f32, t1:f32, s2:f32, t2:f32) → `Result<(), Error>`
- `Gfx.text` (params: text:&str, x:f32, y:f32, size:f32, options:&str) → `Result<(), Error>`
- `Gfx.text_env` (params: target:u32, pname:u32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.texture_info` (params: name:&str) → `Result<(i32, i32, i32, u32, u32, u32), Error>`
- `Gfx.translate` (params: x:f32, y:f32, z:f32) → `Result<(), Error>`
- `Gfx.unbind_buffer_range_vbo` (params: vbo_id:u32, binding_index:u32, element_offset:i32, element_count:i32, target:u32, bind:bool) → `Result<i32, Error>`
- `Gfx.uniform` (params: location:i32, values:[f32; 4], count:u32) → `Result<(), Error>`
- `Gfx.uniform_array_float` (params: location:i32, values:&[f32]) → `Result<(), Error>`
- `Gfx.uniform_array_int` (params: location:i32, values:&[i32]) → `Result<(), Error>`
- `Gfx.uniform_int` (params: location:i32, values:[i32; 4], count:u32) → `Result<(), Error>`
- `Gfx.uniform_matrix` (params: location:i32, values:&[f32], transpose:bool) → `Result<(), Error>`
- `Gfx.uniform_subroutine` (params: shader_type:u32, index:u32) → `Result<(), Error>`
- `Gfx.unit` (params: unit_id:i32, options:GfxUnitDrawOptions) → `Result<(), Error>`
- `Gfx.unit_mult_matrix` (params: value:i32) → `Result<(), Error>`
- `Gfx.unit_piece` (params: object_id:i32, piece_id:i32) → `Result<(), Error>`
- `Gfx.unit_piece_matrix` (params: object_id:i32, piece_id:i32) → `Result<(), Error>`
- `Gfx.unit_piece_mult_matrix` (params: object_id:i32, piece_id:i32) → `Result<(), Error>`
- `Gfx.unit_raw` (params: unit_id:i32, options:GfxUnitDrawOptions) → `Result<(), Error>`
- `Gfx.unit_shape` (params: def_id:i32, team_id:i32, options:GfxObjectShapeOptions) → `Result<(), Error>`
- `Gfx.unit_shape_textures` (params: object_id:i32, push:bool) → `Result<(), Error>`
- `Gfx.unit_textures` (params: object_id:i32, push:bool) → `Result<(), Error>`
- `Gfx.unsafe_state` (params: state:u32, reverse:bool, callback:F) → `Result<(), Error>`
- `Gfx.upload_texture` (params: name:&str, target:u32, level:i32, xoff:i32, yoff:i32, zoff:i32, width:i32, height:i32, depth:i32, format:u32, pixel_type:u32, data:&[u8]) → `Result<(), Error>`
- `Gfx.upload_vbo` (params: vbo_id:u32, data:&[f32], attribute_index:i32, element_offset:i32, data_start_index:i32, data_finish_index:i32) → `Result<u32, Error>`
- `Gfx.use_shader` (params: shader_id:u32) → `Result<bool, Error>`
- `Gfx.vertex` (params: x:f32, y:f32, z:f32, w:f32, count:u32) → `Result<(), Error>`
- `Gfx.viewport` (params: x:i32, y:i32, width:i32, height:i32) → `Result<(), Error>`

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
- `GroundDecals.get_ground_decal_textures` (params: options:GetGroundDecalTexturesOptions) → `Result<(Vec<String>, Vec<String>), Error>`
- `GroundDecals.get_ground_decal_tint` (params: decal_id:u32) → `Result<[f32`
- `GroundDecals.get_ground_decal_type` (params: decal_id:u32) → `Result<Option<String>, Error>`
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
- `Input.get_key_bindings` (params: key_set1:&str, key_set2:&str) → `Result<Vec<sys::KeyBindingEntry>, Error>`
- `Input.get_key_code` (params: key_sym:&str) → `Result<i32, Error>`
- `Input.get_key_from_scan_symbol` (params: scan_symbol:&str) → `Result<Option<String>, Error>`
- `Input.get_key_state` (params: key_code:i32) → `Result<bool, Error>`
- `Input.get_key_symbol` (params: key_code:i32) → `Result<(Option<String>, Option<String>), Error>`
- `Input.get_mod_key_state` (params: ) → `Result<(bool, bool, bool, bool), Error>`
- `Input.get_mouse_buttons_pressed` (params: buttons:&[i32]) → `Result<Vec<bool>, Error>`
- `Input.get_mouse_cursor` (params: ) → `Result<Option<String>, Error>`
- `Input.get_mouse_start_position` (params: button:i32) → `Result<(sys::Float2, sys::Float3, sys::Float3), Error>`
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

- `Los.get_closest_valid_position` (params: unit_def_id:i32, x:f32, z:f32, radius:f32) → `Result<sys::Float3, Error>`
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
- `Markers.marker_erase_position` (params: pos:sys::Float3, unused:f32, options:MarkerErasePositionOptions, player_id:i32) → `Result<bool, Error>`

## MathExtra (14 functions)

- `MathExtra.bit_and` (params: a:u32, b:u32) → `Result<u32, Error>`
- `MathExtra.bit_bits` (params: bits:&[u32]) → `Result<u32, Error>`
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

## Messages (21 functions)

- `Messages.echo` (params: message:&str, rest:&str) → `Result<bool, Error>`
- `Messages.get_console_buffer` (params: max_lines:u32) → `Result<Vec<sys::ConsoleEntry>, Error>`
- `Messages.get_console_entries` (params: max_lines:u32) → `Result<Vec<ConsoleEntry>, Error>`
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

## MoveCtrl (5 functions)

- `MoveCtrl.get_unit_estimated_path` (params: unit_id:i32) → `Result<(Vec<sys::PathWaypoint>, Vec<i32>), Error>`
- `MoveCtrl.get_unit_move_type_data` (params: unit_id:i32) → `Result<sys::MoveTypeData, Error>`
- `MoveCtrl.is_move_ctrl_enabled` (params: unit_id:i32) → `Result<bool, Error>`
- `MoveCtrl.move_ctrl` (params: unit_id:i32, enable:bool) → `Result<bool, Error>`
- `MoveCtrl.set_move_ctrl_gravity` (params: unit_id:i32, gravity_factor:f32) → `Result<bool, Error>`

## PathFinder (10 functions)

- `PathFinder.delete_path` (params: path_id:u32) → `Result<bool, Error>`
- `PathFinder.free_path_node_costs_array` (params: overlay_index:u32) → `Result<bool, Error>`
- `PathFinder.get_next_way_point` (params: path_id:u32, caller_pos:sys::Float3, min_dist:f32) → `Result<(sys::Float3, bool), Error>`
- `PathFinder.get_path_node_cost` (params: x:u32, z:u32) → `Result<f32, Error>`
- `PathFinder.get_path_node_costs` (params: overlay_index:u32) → `Result<Vec<f32>, Error>`
- `PathFinder.get_path_way_points` (params: path_id:u32) → `Result<(Vec<sys::Float3>, Vec<i32>), Error>`
- `PathFinder.init_path_node_costs_array` (params: overlay_index:u32, size_x:u32, size_z:u32) → `Result<bool, Error>`
- `PathFinder.request_path` (params: move_def_id:u32, move_def_name:Option<&str>, start_pos:sys::Float3, end_pos:sys::Float3, radius:f32) → `Result<u32, Error>`
- `PathFinder.set_path_node_cost` (params: overlay_index:u32, cost_index:u32, cost:f32) → `Result<bool, Error>`
- `PathFinder.set_path_node_costs` (params: overlay_index:u32) → `Result<bool, Error>`

## Platform (2 functions)

- `Platform.get_architecture` (params: ) → `Result<Option<String>, Error>`
- `Platform.is_headless` (params: ) → `Result<bool, Error>`

## Player (8 functions)

- `Player.get_local_ally_team_id` (params: ) → `Result<i32, Error>`
- `Player.get_local_player_id` (params: ) → `Result<i32, Error>`
- `Player.get_local_team_id` (params: ) → `Result<i32, Error>`
- `Player.get_player_roster` (params: sort_mode:i32, show_pathing_players:bool) → `Result<Vec<sys::RosterEntry>, Error>`
- `Player.get_player_roster_owned` (params: sort_mode:i32, show_pathing_players:bool) → `Result<Vec<RosterEntry>, Error>`
- `Player.get_player_statistics` (params: player_id:i32) → `Result<sys::PlayerStats, Error>`
- `Player.get_player_traffic` (params: player_id:i32, packet_id:i32) → `Result<Vec<sys::PlayerTraffic>, Error>`
- `Player.get_spectating_state` (params: ) → `Result<bool, Error>`

## Profiling (10 functions)

- `Profiling.diff_timers` (params: end_timer:u64, start_timer:u64, options:DiffTimersOptions) → `Result<f32, Error>`
- `Profiling.get_draw_seconds` (params: ) → `Result<f32, Error>`
- `Profiling.get_frame_timer` (params: last_frame_time:bool) → `Result<u64, Error>`
- `Profiling.get_lua_mem_usage` (params: ) → `Result<(f32, f32, f32, f32, f32, f32, f32, f32), Error>`
- `Profiling.get_profiler_record_names` (params: ) → `Result<Vec<String>, Error>`
- `Profiling.get_profiler_time_record` (params: name:&str, include_frame_data:bool) → `Result<(f32, f32, f32, f32, f32, Vec<f32>), Error>`
- `Profiling.get_synced_gcinfo` (params: collect:bool) → `Result<f32, Error>`
- `Profiling.get_timer` (params: ) → `Result<u64, Error>`
- `Profiling.get_timer_micros` (params: ) → `Result<u64, Error>`
- `Profiling.get_vid_mem_usage` (params: ) → `Result<(f32, f32), Error>`

## ProjectileControl (16 functions)

- `ProjectileControl.delete_projectile` (params: projectile_id:i32) → `Result<bool, Error>`
- `ProjectileControl.set_piece_projectile_params` (params: projectile_id:i32, expl_flags:i32, spin_angle:f32, spin_speed:f32, spin_vec:sys::Float3) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_always_visible` (params: projectile_id:i32, always_visible:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_ceg` (params: projectile_id:i32, ceg_name:&str) → `Result<i32, Error>`
- `ProjectileControl.set_projectile_collision` (params: projectile_id:i32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_damages` (params: projectile_id:i32, unused:i32, damage_key:&str, damage_value:f32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_gravity` (params: projectile_id:i32, gravity:f32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_ignore_tracking_error` (params: projectile_id:i32, ignore:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_is_intercepted` (params: projectile_id:i32, intercepted:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_move_control` (params: projectile_id:i32, enable:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_position` (params: projectile_id:i32, pos:sys::Float3) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_target` (params: projectile_id:i32, target:sys::ProjectileTargetRef) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_time_to_live` (params: projectile_id:i32, time_to_live:i32) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_use_air_los` (params: projectile_id:i32, use_air_los:bool) → `Result<bool, Error>`
- `ProjectileControl.set_projectile_velocity` (params: projectile_id:i32, velocity:sys::Float3) → `Result<bool, Error>`
- `ProjectileControl.spawn_projectile` (params: weapon_def_id:i32, projectile_params:sys::NativeProjectileParams) → `Result<i32, Error>`

## Projectiles (17 functions)

- `Projectiles.get_all_projectiles` (params: options:GetAllProjectilesOptions) → `Result<Vec<i32>, Error>`
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
- `Projectiles.get_projectile_type` (params: projectile_id:i32) → `Result<(bool, bool), Error>`
- `Projectiles.get_projectile_velocity` (params: projectile_id:i32) → `Result<sys::Float3, Error>`
- `Projectiles.get_projectiles_in_rectangle` (params: min_x:f32, min_z:f32, max_x:f32, max_z:f32, options:GetProjectilesInRectangleOptions) → `Result<Vec<i32>, Error>`
- `Projectiles.get_projectiles_in_sphere` (params: center:sys::Float3, radius:f32, options:GetProjectilesInSphereOptions) → `Result<Vec<i32>, Error>`

## RmlUi (222 functions)

- `RmlUi.add_translation_string` (params: key:&str, translation:&str) → `Result<bool, Error>`
- `RmlUi.bind` (params: name:&str, initial:T) → `Result<RmlDataVariable<'api, T>, Error>`
- `RmlUi.bind_choice_rows` (params: name:&str) → `Result<RmlDataChoiceRows<'api>, Error>`
- `RmlUi.bind_event` (params: name:&str, fields:&[RmlFieldType], callback:F) → `Result<RmlDataEvent<'api>, Error>`
- `RmlUi.bind_grid_rows` (params: name:&str) → `Result<RmlDataGridRows<'api>, Error>`
- `RmlUi.bind_icon_rows` (params: name:&str) → `Result<RmlDataIconRows<'api>, Error>`
- `RmlUi.bind_log_rows` (params: name:&str) → `Result<RmlDataLogRows<'api>, Error>`
- `RmlUi.bind_notification_rows` (params: name:&str) → `Result<RmlDataNotificationRows<'api>, Error>`
- `RmlUi.bind_option_rows` (params: name:&str) → `Result<RmlDataOptionRows<'api>, Error>`
- `RmlUi.bind_rows` (params: name:&str, fields:&[(&str, RmlFieldType)]) → `Result<RmlDataRows<'api>, Error>`
- `RmlUi.bind_status_rows` (params: name:&str) → `Result<RmlDataStatusRows<'api>, Error>`
- `RmlUi.bind_swatch_rows` (params: name:&str) → `Result<RmlDataSwatchRows<'api>, Error>`
- `RmlUi.bind_text_rows` (params: name:&str) → `Result<RmlDataTextRows<'api>, Error>`
- `RmlUi.clear_document_path_requests` (params: document_path:&str) → `Result<bool, Error>`
- `RmlUi.clear_translations` (params: ) → `Result<bool, Error>`
- `RmlUi.context_activate_theme` (params: context_handle:u64, name:&str, value:bool) → `Result<bool, Error>`
- `RmlUi.context_add_event_listener` (params: context_handle:u64, event:&str, in_capture_phase:bool, callback:F) → `Result<(u64, bool), Error>`
- `RmlUi.context_create_data_model` (params: context_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.context_create_document` (params: context_handle:u64, tag:&str) → `Result<(u64, bool), Error>`
- `RmlUi.context_enable_mouse_cursor` (params: context_handle:u64, value:bool) → `Result<bool, Error>`
- `RmlUi.context_get_density_independent_pixel_ratio` (params: context_handle:u64) → `Result<f32, Error>`
- `RmlUi.context_get_dimensions` (params: context_handle:u64) → `Result<(i32, i32), Error>`
- `RmlUi.context_get_document` (params: context_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.context_get_element_at_point` (params: context_handle:u64, x:f32, y:f32, ignore_element_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.context_get_focus_element` (params: context_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.context_get_hover_element` (params: context_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.context_get_name` (params: context_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.context_get_root_element` (params: context_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.context_is_mouse_interacting` (params: context_handle:u64) → `Result<bool, Error>`
- `RmlUi.context_is_theme_active` (params: context_handle:u64, name:&str) → `Result<bool, Error>`
- `RmlUi.context_load_document` (params: context_handle:u64, document_path:&str) → `Result<(u64, bool), Error>`
- `RmlUi.context_open_data_model` (params: context_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.context_process_key_down` (params: context_handle:u64, key:i32, key_modifier_state:i32) → `Result<bool, Error>`
- `RmlUi.context_process_key_up` (params: context_handle:u64, key:i32, key_modifier_state:i32) → `Result<bool, Error>`
- `RmlUi.context_process_mouse_button_down` (params: context_handle:u64, button:i32, key_modifier_state:i32) → `Result<bool, Error>`
- `RmlUi.context_process_mouse_button_up` (params: context_handle:u64, button:i32, key_modifier_state:i32) → `Result<bool, Error>`
- `RmlUi.context_process_mouse_leave` (params: context_handle:u64) → `Result<bool, Error>`
- `RmlUi.context_process_mouse_move` (params: context_handle:u64, x:f32, y:f32, key_modifier_state:i32) → `Result<bool, Error>`
- `RmlUi.context_process_mouse_wheel` (params: context_handle:u64, x:f32, y:f32, key_modifier_state:i32) → `Result<bool, Error>`
- `RmlUi.context_process_text_input` (params: context_handle:u64, text:&str) → `Result<bool, Error>`
- `RmlUi.context_pull_document_to_front` (params: context_handle:u64, document_handle:u64) → `Result<bool, Error>`
- `RmlUi.context_pull_to_front` (params: context_handle:u64) → `Result<bool, Error>`
- `RmlUi.context_push_document_to_back` (params: context_handle:u64, document_handle:u64) → `Result<bool, Error>`
- `RmlUi.context_remove_data_model` (params: context_handle:u64, name:&str) → `Result<bool, Error>`
- `RmlUi.context_render` (params: context_handle:u64) → `Result<bool, Error>`
- `RmlUi.context_set_density_independent_pixel_ratio` (params: context_handle:u64, value:f32) → `Result<bool, Error>`
- `RmlUi.context_set_dimensions` (params: context_handle:u64, x:i32, y:i32) → `Result<bool, Error>`
- `RmlUi.context_set_pointer_capture` (params: context_handle:u64, anchor_x:i32, anchor_y:i32, active:bool) → `Result<bool, Error>`
- `RmlUi.context_take_pointer_capture_delta` (params: context_handle:u64) → `Result<(i32, i32, i32), Error>`
- `RmlUi.context_unload_all_documents` (params: context_handle:u64) → `Result<bool, Error>`
- `RmlUi.context_unload_document` (params: context_handle:u64, document_handle:u64) → `Result<bool, Error>`
- `RmlUi.create_context` (params: name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.create_data_model` (params: context_handle:u64, name:&str) → `Result<RmlDataModel<'a>, Error>`
- `RmlUi.data_model_bind_bool` (params: data_model_handle:u64, name:&str, initial_value:bool) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_choice_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_color` (params: data_model_handle:u64, name:&str, red:u8, green:u8, blue:u8, alpha:u8) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_float` (params: data_model_handle:u64, name:&str, initial_value:f32) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_grid_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_icon_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_int` (params: data_model_handle:u64, name:&str, initial_value:i32) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_log_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_notification_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_option_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_percent` (params: data_model_handle:u64, name:&str, initial_value:f32) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_pixels` (params: data_model_handle:u64, name:&str, initial_value:f32) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_rows` (params: data_model_handle:u64, name:&str, fields:&sys::RmlDataFieldDef, field_count:u64) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_status_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_string` (params: data_model_handle:u64, name:&str, initial_value:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_swatch_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_bind_text_rows` (params: data_model_handle:u64, name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.data_model_get_bool` (params: variable_handle:u64) → `Result<(bool, bool), Error>`
- `RmlUi.data_model_get_color` (params: variable_handle:u64) → `Result<(u8, u8, u8, u8, bool), Error>`
- `RmlUi.data_model_get_float` (params: variable_handle:u64) → `Result<(f32, bool), Error>`
- `RmlUi.data_model_get_int` (params: variable_handle:u64) → `Result<(i32, bool), Error>`
- `RmlUi.data_model_get_percent` (params: variable_handle:u64) → `Result<(f32, bool), Error>`
- `RmlUi.data_model_get_pixels` (params: variable_handle:u64) → `Result<(f32, bool), Error>`
- `RmlUi.data_model_get_string` (params: variable_handle:u64) → `Result<(Option<String>, bool), Error>`
- `RmlUi.data_model_set_bool` (params: variable_handle:u64, value:bool) → `Result<bool, Error>`
- `RmlUi.data_model_set_choice_rows` (params: rows_handle:u64, rows:&sys::RmlDataChoiceRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_color` (params: variable_handle:u64, red:u8, green:u8, blue:u8, alpha:u8) → `Result<bool, Error>`
- `RmlUi.data_model_set_float` (params: variable_handle:u64, value:f32) → `Result<bool, Error>`
- `RmlUi.data_model_set_grid_rows` (params: rows_handle:u64, rows:&sys::RmlDataGridRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_icon_rows` (params: rows_handle:u64, rows:&sys::RmlDataIconRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_int` (params: variable_handle:u64, value:i32) → `Result<bool, Error>`
- `RmlUi.data_model_set_log_rows` (params: rows_handle:u64, rows:&sys::RmlDataLogRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_notification_rows` (params: rows_handle:u64, rows:&sys::RmlDataNotificationRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_option_rows` (params: rows_handle:u64, rows:&sys::RmlDataOptionRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_percent` (params: variable_handle:u64, value:f32) → `Result<bool, Error>`
- `RmlUi.data_model_set_pixels` (params: variable_handle:u64, value:f32) → `Result<bool, Error>`
- `RmlUi.data_model_set_rows` (params: rows_handle:u64, values:&sys::RmlDataValue, row_count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_status_rows` (params: rows_handle:u64, rows:&sys::RmlDataStatusRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_string` (params: variable_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.data_model_set_swatch_rows` (params: rows_handle:u64, rows:&sys::RmlDataSwatchRow, count:u64) → `Result<bool, Error>`
- `RmlUi.data_model_set_text_rows` (params: rows_handle:u64, rows:&sys::RmlDataTextRow, count:u64) → `Result<bool, Error>`
- `RmlUi.document_append_to_style_sheet` (params: document_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.document_close` (params: document_handle:u64) → `Result<bool, Error>`
- `RmlUi.document_create_element` (params: document_handle:u64, tag_name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.document_create_text_node` (params: document_handle:u64, value:&str) → `Result<(u64, bool), Error>`
- `RmlUi.document_get_context` (params: document_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.document_get_title` (params: document_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.document_get_url` (params: document_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.document_hide` (params: document_handle:u64) → `Result<bool, Error>`
- `RmlUi.document_is_modal` (params: document_handle:u64) → `Result<bool, Error>`
- `RmlUi.document_load_external_script` (params: document_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.document_load_inline_script` (params: document_handle:u64, content:&str, source_path:&str, source_line:i32) → `Result<bool, Error>`
- `RmlUi.document_pull_to_front` (params: document_handle:u64) → `Result<bool, Error>`
- `RmlUi.document_push_to_back` (params: document_handle:u64) → `Result<bool, Error>`
- `RmlUi.document_reload_style_sheet` (params: document_handle:u64) → `Result<bool, Error>`
- `RmlUi.document_set_title` (params: document_handle:u64, title:&str) → `Result<bool, Error>`
- `RmlUi.document_show` (params: document_handle:u64, options:RmlDocumentShowOptions) → `Result<bool, Error>`
- `RmlUi.document_update_document` (params: document_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_add_event_listener` (params: element_handle:u64, event:&str, in_capture_phase:bool, callback:F) → `Result<(u64, bool), Error>`
- `RmlUi.element_append_child` (params: element_handle:u64, element_ptr_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.element_are_pseudo_classes_set` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_blur` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_click` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_clone` (params: element_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.element_closest` (params: element_handle:u64, value:&str) → `Result<(u64, bool), Error>`
- `RmlUi.element_dispatch_event` (params: element_handle:u64, event:&str) → `Result<bool, Error>`
- `RmlUi.element_focus` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_form_control_input_get_selection` (params: element_handle:u64) → `Result<(i32, i32, Option<String>, bool), Error>`
- `RmlUi.element_form_control_input_select` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_form_control_input_set_selection` (params: element_handle:u64, start:i32, end:i32) → `Result<bool, Error>`
- `RmlUi.element_form_control_select_add` (params: element_handle:u64, element_ptr_handle:u64, before:i32) → `Result<bool, Error>`
- `RmlUi.element_form_control_select_remove` (params: element_handle:u64, index:i32) → `Result<bool, Error>`
- `RmlUi.element_form_control_select_remove_all` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_form_control_text_area_get_selection` (params: element_handle:u64) → `Result<(i32, i32, Option<String>, bool), Error>`
- `RmlUi.element_form_control_text_area_select` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_form_control_text_area_set_selection` (params: element_handle:u64, start:i32, end:i32) → `Result<bool, Error>`
- `RmlUi.element_form_submit` (params: element_handle:u64, name:&str, value:&str) → `Result<bool, Error>`
- `RmlUi.element_get_active_pseudo_classes` (params: element_handle:u64) → `Result<Vec<String>, Error>`
- `RmlUi.element_get_attribute` (params: element_handle:u64, name:&str) → `Result<(Option<String>, bool), Error>`
- `RmlUi.element_get_child` (params: element_handle:u64, index:i32) → `Result<(u64, bool), Error>`
- `RmlUi.element_get_class_name` (params: element_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.element_get_element_by_id` (params: element_handle:u64, value:&str) → `Result<(u64, bool), Error>`
- `RmlUi.element_get_elements_by_class_name` (params: element_handle:u64, value:&str) → `Result<Vec<u64>, Error>`
- `RmlUi.element_get_elements_by_class_name_count` (params: element_handle:u64, value:&str) → `Result<i32, Error>`
- `RmlUi.element_get_elements_by_tag_name` (params: element_handle:u64, value:&str) → `Result<Vec<u64>, Error>`
- `RmlUi.element_get_elements_by_tag_name_count` (params: element_handle:u64, value:&str) → `Result<i32, Error>`
- `RmlUi.element_get_id` (params: element_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.element_get_inner_rml` (params: element_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.element_get_rect` (params: element_handle:u64) → `Result<(f32, f32, f32, f32), Error>`
- `RmlUi.element_get_scroll_left` (params: element_handle:u64) → `Result<i32, Error>`
- `RmlUi.element_get_scroll_top` (params: element_handle:u64) → `Result<i32, Error>`
- `RmlUi.element_get_tag_name` (params: element_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.element_get_value` (params: element_handle:u64) → `Result<Option<String>, Error>`
- `RmlUi.element_has_attribute` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_has_child_nodes` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_insert_before` (params: element_handle:u64, element_ptr_handle:u64, adjacent_element_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.element_is_class_set` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_is_point_within_element` (params: element_handle:u64, x:f32, y:f32) → `Result<bool, Error>`
- `RmlUi.element_is_pseudo_class_set` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_is_visible` (params: element_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_matches` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_process_default_action` (params: element_handle:u64, event_handle:u64) → `Result<bool, Error>`
- `RmlUi.element_query_selector` (params: element_handle:u64, value:&str) → `Result<(u64, bool), Error>`
- `RmlUi.element_query_selector_all` (params: element_handle:u64, value:&str) → `Result<Vec<u64>, Error>`
- `RmlUi.element_query_selector_all_count` (params: element_handle:u64, value:&str) → `Result<i32, Error>`
- `RmlUi.element_remove_attribute` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_remove_child` (params: element_handle:u64, child_element_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.element_replace_child` (params: element_handle:u64, element_ptr_handle:u64, replaced_element_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.element_scroll_into_view` (params: element_handle:u64, align_with_top:bool) → `Result<bool, Error>`
- `RmlUi.element_set_attribute` (params: element_handle:u64, name:&str, value:&str) → `Result<bool, Error>`
- `RmlUi.element_set_class` (params: element_handle:u64, name:&str, value:bool) → `Result<bool, Error>`
- `RmlUi.element_set_class_name` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_set_id` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_set_inner_rml` (params: element_handle:u64, value:&str) → `Result<bool, Error>`
- `RmlUi.element_set_pseudo_class` (params: element_handle:u64, name:&str, value:bool) → `Result<bool, Error>`
- `RmlUi.element_set_scroll_left` (params: element_handle:u64, value:i32) → `Result<bool, Error>`
- `RmlUi.element_set_scroll_top` (params: element_handle:u64, value:i32) → `Result<bool, Error>`
- `RmlUi.element_tab_set_remove_tab` (params: element_handle:u64, index:i32) → `Result<bool, Error>`
- `RmlUi.element_tab_set_set_panel` (params: element_handle:u64, index:i32, rml:&str) → `Result<bool, Error>`
- `RmlUi.element_tab_set_set_tab` (params: element_handle:u64, index:i32, rml:&str) → `Result<bool, Error>`
- `RmlUi.event_get_current` (params: ) → `Result<(u64, u64, u64, bool), Error>`
- `RmlUi.event_get_current_element` (params: event_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.event_get_parameter_bool` (params: event_handle:u64, name:&str) → `Result<(bool, bool), Error>`
- `RmlUi.event_get_parameter_float` (params: event_handle:u64, name:&str) → `Result<(f32, bool), Error>`
- `RmlUi.event_get_parameter_int` (params: event_handle:u64, name:&str) → `Result<(i32, bool), Error>`
- `RmlUi.event_get_parameter_string` (params: event_handle:u64, name:&str) → `Result<(Option<String>, bool), Error>`
- `RmlUi.event_get_parameter_type` (params: event_handle:u64, name:&str) → `Result<(i32, bool), Error>`
- `RmlUi.event_get_phase` (params: event_handle:u64) → `Result<(i32, bool), Error>`
- `RmlUi.event_get_target_element` (params: event_handle:u64) → `Result<(u64, bool), Error>`
- `RmlUi.event_get_type` (params: event_handle:u64) → `Result<(Option<String>, bool), Error>`
- `RmlUi.event_is_immediate_propagating` (params: event_handle:u64) → `Result<(bool, bool), Error>`
- `RmlUi.event_is_interruptible` (params: event_handle:u64) → `Result<(bool, bool), Error>`
- `RmlUi.event_is_propagating` (params: event_handle:u64) → `Result<(bool, bool), Error>`
- `RmlUi.event_listener_on_attach` (params: event_listener_handle:u64, element_handle:u64) → `Result<bool, Error>`
- `RmlUi.event_listener_on_detach` (params: event_listener_handle:u64, element_handle:u64) → `Result<bool, Error>`
- `RmlUi.event_listener_process_event` (params: event_listener_handle:u64, event_handle:u64) → `Result<bool, Error>`
- `RmlUi.event_stop_immediate_propagation` (params: event_handle:u64) → `Result<bool, Error>`
- `RmlUi.event_stop_propagation` (params: event_handle:u64) → `Result<bool, Error>`
- `RmlUi.get` (params: ) → `Result<T, Error>`
- `RmlUi.get_context` (params: name:&str) → `Result<(u64, bool), Error>`
- `RmlUi.get_document_path_requests` (params: document_path:&str) → `Result<Vec<String>, Error>`
- `RmlUi.get_version` (params: ) → `Result<Option<String>, Error>`
- `RmlUi.is_ready` (params: ) → `Result<bool, Error>`
- `RmlUi.load_font_face` (params: file_path:&str, fallback:bool, weight:Option<i32>) → `Result<bool, Error>`
- `RmlUi.regiser_event_type` (params: event_type:&str, options:RmlRegisterEventTypeOptions) → `Result<i32, Error>`
- `RmlUi.register_event_type` (params: event_type:&str, options:RmlRegisterEventTypeOptions) → `Result<i32, Error>`
- `RmlUi.remove_context` (params: context_handle:u64) → `Result<bool, Error>`
- `RmlUi.remove_context_by_name` (params: name:&str) → `Result<bool, Error>`
- `RmlUi.remove_data_model` (params: context_handle:u64, name:&str) → `Result<(), Error>`
- `RmlUi.set` (params: value:T) → `Result<(), Error>`
- `RmlUi.set` (params: values:&[RmlValue]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlTextRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlLogRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlNotificationRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlIconRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlOptionRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlChoiceRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlStatusRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlSwatchRow]) → `Result<(), Error>`
- `RmlUi.set` (params: rows:&[RmlGridRow]) → `Result<(), Error>`
- `RmlUi.set_debug_context` (params: context_handle:u64) → `Result<bool, Error>`
- `RmlUi.set_debug_context_by_name` (params: name:&str) → `Result<bool, Error>`
- `RmlUi.set_mouse_cursor_alias` (params: rml_name:&str, recoil_name:&str) → `Result<bool, Error>`
- `RmlUi.sol_lua_data_model___set_dirty` (params: data_model_handle:u64, property:&str) → `Result<bool, Error>`
- `RmlUi.sol_lua_data_model_set_dirty` (params: data_model_handle:u64, property:&str) → `Result<bool, Error>`
- `RmlUi.take_pointer_capture_delta` (params: context_handle:u64) → `Result<RmlPointerCaptureDelta, Error>`
- `RmlUi.unbind` (params: ) → `Result<(), Error>`
- `RmlUi.vector2f_new` (params: x:f32, y:f32) → `Result<(f32, f32), Error>`
- `RmlUi.vector2i_new` (params: x:i32, y:i32) → `Result<(i32, i32), Error>`

## RulesParams (15 functions)

- `RulesParams.get_feature_rules_param` (params: feature_id:i32, param_name:&str) → `Result<(RulesParamValue, i32, bool), Error>`
- `RulesParams.get_feature_rules_params` (params: feature_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.get_game_rules_param` (params: param_name:&str) → `Result<(RulesParamValue, i32, bool), Error>`
- `RulesParams.get_game_rules_params` (params: ) → `Result<Vec<String>, Error>`
- `RulesParams.get_player_rules_param` (params: player_id:i32, param_name:&str) → `Result<(RulesParamValue, i32, bool), Error>`
- `RulesParams.get_player_rules_params` (params: player_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.get_team_rules_param` (params: team_id:i32, param_name:&str) → `Result<(RulesParamValue, i32, bool), Error>`
- `RulesParams.get_team_rules_params` (params: team_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.get_unit_rules_param` (params: unit_id:i32, param_name:&str) → `Result<(RulesParamValue, i32, bool), Error>`
- `RulesParams.get_unit_rules_params` (params: unit_id:i32) → `Result<Vec<String>, Error>`
- `RulesParams.set_feature_rules_param` (params: feature_id:i32, param_name:&str, value:RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_game_rules_param` (params: param_name:&str, value:RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_player_rules_param` (params: player_id:i32, param_name:&str, value:RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_team_rules_param` (params: team_id:i32, param_name:&str, value:RulesParamValue, los:i32) → `Result<bool, Error>`
- `RulesParams.set_unit_rules_param` (params: unit_id:i32, param_name:&str, value:RulesParamValue, los:i32) → `Result<bool, Error>`

## Selection (16 functions)

- `Selection.deselect_unit` (params: unit_id:i32) → `Result<bool, Error>`
- `Selection.deselect_unit_array` (params: unit_ids:&[i32]) → `Result<bool, Error>`
- `Selection.get_group_list` (params: ) → `Result<Vec<i32>, Error>`
- `Selection.get_group_units` (params: group_id:i32) → `Result<Vec<i32>, Error>`
- `Selection.get_group_units_count` (params: group_id:i32) → `Result<u32, Error>`
- `Selection.get_group_units_counts` (params: group_id:i32) → `Result<sys::SelectionCounts, Error>`
- `Selection.get_group_units_sorted` (params: group_id:i32) → `Result<Vec<sys::TeamUnitsByDef>, Error>`
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

- `SystemControl.call_as_team` (params: team_id:i32, callback:F) → `Result<bool, Error>`
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

## TeamControl (15 functions)

- `TeamControl.add_team_resource` (params: team_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`
- `TeamControl.add_team_resource_excess_stats` (params: team_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`
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

## Teams (21 functions)

- `Teams.are_players_allied` (params: player_id1:i32, player_id2:i32) → `Result<bool, Error>`
- `Teams.are_teams_allied` (params: team_id1:i32, team_id2:i32) → `Result<bool, Error>`
- `Teams.get_aiinfo` (params: team_id:i32) → `Result<(sys::AIInfo, bool), Error>`
- `Teams.get_ally_team_info` (params: ally_team_id:i32) → `Result<sys::AllyTeamInfo, Error>`
- `Teams.get_ally_team_list` (params: ) → `Result<Vec<i32>, Error>`
- `Teams.get_player_controlled_unit` (params: player_id:i32) → `Result<(i32, bool), Error>`
- `Teams.get_player_info` (params: player_id:i32, get_player_opts:bool) → `Result<sys::PlayerInfo, Error>`
- `Teams.get_player_info_owned` (params: player_id:i32, get_player_keys:bool) → `Result<PlayerInfo, Error>`
- `Teams.get_player_list` (params: team_id:i32, active:bool) → `Result<Vec<i32>, Error>`
- `Teams.get_player_list_in_ally_team` (params: ally_team_id:i32) → `Result<Vec<i32>, Error>`
- `Teams.get_player_list_in_team` (params: team_id:i32) → `Result<Vec<i32>, Error>`
- `Teams.get_team_ally_team_id` (params: team_id:i32) → `Result<i32, Error>`
- `Teams.get_team_info` (params: team_id:i32, get_team_keys:bool) → `Result<sys::TeamInfo, Error>`
- `Teams.get_team_info_owned` (params: team_id:i32, get_team_keys:bool) → `Result<TeamInfo, Error>`
- `Teams.get_team_list` (params: ally_team_id:i32) → `Result<Vec<i32>, Error>`
- `Teams.get_team_lua_ai` (params: team_id:i32) → `Result<Option<String>, Error>`
- `Teams.get_team_max_units` (params: team_id:i32) → `Result<i32, Error>`
- `Teams.get_team_resource_stats` (params: team_id:i32, resource:&str) → `Result<sys::TeamResources, Error>`
- `Teams.get_team_resources` (params: team_id:i32, resource:&str) → `Result<sys::TeamResources, Error>`
- `Teams.get_team_stats_history` (params: team_id:i32, start_index:i32, end_index:i32) → `Result<Vec<sys::TeamStatsHistoryPoint>, Error>`
- `Teams.get_team_unit_stats` (params: team_id:i32) → `Result<sys::TeamUnitStats, Error>`

## Terrain (13 functions)

- `Terrain.get_grass` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_ground_blocked` (params: x1:f32, z1:f32, x2:f32, z2:f32) → `Result<bool, Error>`
- `Terrain.get_ground_extremes` (params: ) → `Result<(f32, f32, f32, f32), Error>`
- `Terrain.get_ground_height` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_ground_info` (params: x:f32, z:f32) → `Result<(i32, Option<String>, f32, f32, f32, f32, f32, f32, bool), Error>`
- `Terrain.get_ground_normal` (params: x:f32, z:f32, smoothed:bool) → `Result<(sys::Float3, f32), Error>`
- `Terrain.get_ground_orig_height` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_height_map_size` (params: ) → `Result<(i32, i32), Error>`
- `Terrain.get_smooth_mesh_height` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_terrain_type_data` (params: terrain_type_index:i32) → `Result<(i32, Option<String>, f32, f32, f32, f32, f32, bool), Error>`
- `Terrain.get_water_level` (params: x:f32, z:f32) → `Result<f32, Error>`
- `Terrain.get_water_plane_level` (params: ) → `Result<f32, Error>`
- `Terrain.is_pos_in_map` (params: x:f32, z:f32) → `Result<(bool, bool), Error>`

## TerrainControl (25 functions)

- `TerrainControl.add_grass` (params: x:f32, z:f32, grass_value:u8) → `Result<bool, Error>`
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
- `TerrainControl.set_height_map_func` (params: callback:F) → `Result<bool, Error>`
- `TerrainControl.set_map_square_terrain_type` (params: x:i32, z:i32, terrain_type:i32) → `Result<bool, Error>`
- `TerrainControl.set_original_height_map` (params: x:f32, z:f32, height:f32, factor:f32) → `Result<bool, Error>`
- `TerrainControl.set_original_height_map_func` (params: callback:F) → `Result<bool, Error>`
- `TerrainControl.set_smooth_mesh` (params: x:f32, z:f32, height:f32, terraform:f32) → `Result<bool, Error>`
- `TerrainControl.set_smooth_mesh_func` (params: callback:F) → `Result<bool, Error>`
- `TerrainControl.set_terrain_type_data` (params: type_index:i32, tank_speed:f32, kbot_speed:f32, hover_speed:f32, ship_speed:f32, hardness:f32, receive_tracks:bool, name:&str) → `Result<bool, Error>`
- `TerrainControl.set_tidal` (params: tidal:f32) → `Result<bool, Error>`
- `TerrainControl.set_wind` (params: min_wind:f32, max_wind:f32) → `Result<bool, Error>`

## Tracing (7 functions)

- `Tracing.trace_ray` (params: ray:sys::Ray) → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_between_positions` (params: start:sys::Float3, end:sys::Float3, r#type:&str) → `Result<Vec<sys::TraceRayHit>, Error>`
- `Tracing.trace_ray_features` (params: ray:sys::Ray) → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_ground_between_positions` (params: start:sys::Float3, end:sys::Float3, options:TraceRayGroundBetweenPositionsOptions) → `Result<(bool, f32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_ground_in_direction` (params: start:sys::Float3, dir:sys::Float3, options:TraceRayGroundInDirectionOptions) → `Result<(bool, f32, sys::Float3, sys::Float3), Error>`
- `Tracing.trace_ray_in_direction` (params: pos:sys::Float3, dir:sys::Float3, options:TraceRayInDirectionOptions, r#type:&str) → `Result<Vec<sys::TraceRayHit>, Error>`
- `Tracing.trace_ray_units` (params: ray:sys::Ray) → `Result<(bool, i32, i32, sys::Float3, sys::Float3), Error>`

## UnitControl (88 functions)

- `UnitControl.add_object_decal` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitControl.add_unit_damage` (params: unit_id:i32, damage:f32, paralyze_time:f32, weapon_def_id:i32, attacker_id:i32, impulse:sys::Float3) → `Result<bool, Error>`
- `UnitControl.add_unit_experience` (params: unit_id:i32, experience:f32) → `Result<bool, Error>`
- `UnitControl.add_unit_impulse` (params: unit_id:i32, impulse:sys::Float3, decay_rate:f32) → `Result<bool, Error>`
- `UnitControl.add_unit_resource` (params: unit_id:i32, resource_type:&str, amount:f32) → `Result<bool, Error>`
- `UnitControl.add_unit_seismic_ping` (params: unit_id:i32, ping_size:f32) → `Result<bool, Error>`
- `UnitControl.bugger_off` (params: pos:sys::Float3, radius:f32, team_id:i32, options:BuggerOffOptions, exclude_unit_def_ids:&[i32]) → `Result<bool, Error>`
- `UnitControl.clear_unit_goal` (params: unit_id:i32, cancel_raw:bool) → `Result<bool, Error>`
- `UnitControl.create_unit` (params: unit_def:sys::DefRef, pos:sys::Float3, facing:i32, team_id:i32, options:CreateUnitOptions) → `Result<i32, Error>`
- `UnitControl.destroy_unit` (params: unit_id:i32, options:DestroyUnitOptions) → `Result<bool, Error>`
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
- `UnitControl.set_factory_bugger_off` (params: unit_id:i32, options:SetFactoryBuggerOffOptions) → `Result<bool, Error>`
- `UnitControl.set_unit_always_visible` (params: unit_id:i32, always_visible:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_armored` (params: unit_id:i32, armored_state:bool, armored_multiple:f32) → `Result<bool, Error>`
- `UnitControl.set_unit_blocking` (params: unit_id:i32, options:SetUnitBlockingOptions) → `Result<bool, Error>`
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
- `UnitControl.set_unit_leaves_ghost` (params: unit_id:i32, options:SetUnitLeavesGhostOptions) → `Result<bool, Error>`
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
- `UnitControl.set_unit_target` (params: unit_id:i32, target:sys::UnitTargetRef, options:SetUnitTargetOptions, weapon_num:i32) → `Result<bool, Error>`
- `UnitControl.set_unit_tooltip` (params: unit_id:i32, tooltip:&str) → `Result<bool, Error>`
- `UnitControl.set_unit_use_air_los` (params: unit_id:i32, use_air_los:bool) → `Result<bool, Error>`
- `UnitControl.set_unit_use_weapons` (params: unit_id:i32, options:SetUnitUseWeaponsOptions) → `Result<bool, Error>`
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

## UnitDefs (21 functions)

- `UnitDefs.get_unit_def_basic_info` (params: unit_def_id:i32) → `Result<Option<UnitDefBasicInfo>, Error>`
- `UnitDefs.get_unit_def_by_id` (params: unit_def_id:i32) → `Result<(bool, sys::UnitDefBasicInfo, sys::UnitDefCosts, sys::UnitDefPhysics, sys::UnitDefWeapons, sys::UnitDefBuildOptions, sys::UnitDefSensors, sys::UnitDefHealth, sys::UnitDefClassify), Error>`
- `UnitDefs.get_unit_def_classify` (params: unit_def_id:i32) → `Result<sys::UnitDefClassify, Error>`
- `UnitDefs.get_unit_def_costs` (params: unit_def_id:i32) → `Result<sys::UnitDefCosts, Error>`
- `UnitDefs.get_unit_def_count` (params: ) → `Result<u32, Error>`
- `UnitDefs.get_unit_def_custom_param` (params: unit_def_id:i32, key:&str) → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_custom_param_keys` (params: unit_def_id:i32) → `Result<Vec<String>, Error>`
- `UnitDefs.get_unit_def_health` (params: unit_def_id:i32) → `Result<f32, Error>`
- `UnitDefs.get_unit_def_human_name` (params: unit_def_id:i32) → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_idby_name` (params: unit_def_name:&str) → `Result<i32, Error>`
- `UnitDefs.get_unit_def_ids` (params: ) → `Result<Vec<i32>, Error>`
- `UnitDefs.get_unit_def_name` (params: unit_def_id:i32) → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_param_bool` (params: unit_def_id:i32, key:&str) → `Result<bool, Error>`
- `UnitDefs.get_unit_def_param_float` (params: unit_def_id:i32, key:&str) → `Result<f32, Error>`
- `UnitDefs.get_unit_def_param_int` (params: unit_def_id:i32, key:&str) → `Result<i32, Error>`
- `UnitDefs.get_unit_def_param_keys` (params: ) → `Result<Vec<sys::UnitDefParamKey>, Error>`
- `UnitDefs.get_unit_def_param_string` (params: unit_def_id:i32, key:&str) → `Result<Option<String>, Error>`
- `UnitDefs.get_unit_def_param_type` (params: key:&str) → `Result<i32, Error>`
- `UnitDefs.get_unit_def_parameter_keys` (params: ) → `Result<Vec<UnitDefParamKey>, Error>`
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
- `UnitRendering.get_visible_features` (params: ally_team_id:i32, radius:f32, options:GetVisibleFeaturesOptions) → `Result<Vec<i32>, Error>`
- `UnitRendering.get_visible_projectiles` (params: ally_team_id:i32, options:GetVisibleProjectilesOptions) → `Result<Vec<i32>, Error>`
- `UnitRendering.get_visible_units` (params: team_id:i32, radius:f32, include_icons:bool) → `Result<Vec<i32>, Error>`
- `UnitRendering.is_unit_icon` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.is_unit_in_view` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitRendering.is_unit_visible` (params: unit_id:i32, radius:f32, check_icon:bool) → `Result<bool, Error>`

## UnitsCommands (17 functions)

- `UnitsCommands.find_unit_cmd_desc` (params: unit_id:i32, cmd_id:i32) → `Result<(i32, bool), Error>`
- `UnitsCommands.get_command_params` (params: command:&sys::CommandFFI) → `Result<Vec<f32>, Error>`
- `UnitsCommands.get_command_queue` (params: unit_id:i32, max_commands:u32) → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_factory_bugger_off` (params: unit_id:i32) → `Result<(bool, f32, f32, i32, bool, bool), Error>`
- `UnitsCommands.get_factory_command_count` (params: unit_id:i32) → `Result<u32, Error>`
- `UnitsCommands.get_factory_commands` (params: unit_id:i32, max_commands:u32) → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_factory_counts` (params: unit_id:i32, count:i32, add_cmds:bool) → `Result<sys::FactoryQueueInfo, Error>`
- `UnitsCommands.get_full_build_queue` (params: unit_id:i32) → `Result<Vec<sys::BuildQueueEntry>, Error>`
- `UnitsCommands.get_real_build_queue` (params: unit_id:i32) → `Result<Vec<i32>, Error>`
- `UnitsCommands.get_unit_cmd_descs` (params: unit_id:i32) → `Result<Vec<sys::CommandDescription>, Error>`
- `UnitsCommands.get_unit_command_count` (params: unit_id:i32) → `Result<u32, Error>`
- `UnitsCommands.get_unit_command_descriptions` (params: unit_id:i32) → `Result<Vec<CommandDescription>, Error>`
- `UnitsCommands.get_unit_commands` (params: unit_id:i32, max_commands:u32) → `Result<Vec<sys::CommandFFI>, Error>`
- `UnitsCommands.get_unit_current_command` (params: unit_id:i32, cmd_index:i32) → `Result<(sys::CommandFFI, bool), Error>`
- `UnitsCommands.give_order` (params: cmd_id:i32, params:&[f32], options:u32, timeout:i32) → `Result<bool, Error>`
- `UnitsCommands.give_order_array_to_unit_map` (params: unit_ids:&[i32], commands:&[sys::CommandFFI]) → `Result<i32, Error>`
- `UnitsCommands.give_order_to_unit_map` (params: unit_ids:&[i32], cmd_id:i32, params:&[f32], options:u32, timeout:i32) → `Result<i32, Error>`

## UnitsInfo (57 functions)

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
- `UnitsInfo.get_unit_crashing` (params: unit_id:i32) → `Result<(bool, bool), Error>`
- `UnitsInfo.get_unit_current_build_power` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_def_id` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_direction` (params: unit_id:i32) → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_effective_build_range` (params: unit_id:i32, buildee_def_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_experience` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_flanking` (params: unit_id:i32) → `Result<sys::UnitFlanking, Error>`
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
- `UnitsInfo.get_unit_is_transporting` (params: unit_id:i32) → `Result<(Vec<i32>, bool), Error>`
- `UnitsInfo.get_unit_last_attacked_piece` (params: unit_id:i32) → `Result<sys::LastHitPiece, Error>`
- `UnitsInfo.get_unit_last_attacker` (params: unit_id:i32) → `Result<(sys::UnitLastAttacker, bool), Error>`
- `UnitsInfo.get_unit_los_state` (params: unit_id:i32, ally_team_id:i32, raw:bool) → `Result<sys::UnitLosState, Error>`
- `UnitsInfo.get_unit_mass` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_metal_extraction` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_move_def_id` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_nano_pieces` (params: unit_id:i32) → `Result<Vec<i32>, Error>`
- `UnitsInfo.get_unit_neutral` (params: unit_id:i32) → `Result<bool, Error>`
- `UnitsInfo.get_unit_piece_collision_volume_data` (params: unit_id:i32, piece_num:i32) → `Result<sys::CollisionVolumeData, Error>`
- `UnitsInfo.get_unit_pos_error_params` (params: unit_id:i32, ally_team_id:i32) → `Result<sys::UnitPosErrorParams, Error>`
- `UnitsInfo.get_unit_position` (params: unit_id:i32, options:GetUnitPositionOptions) → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_radius` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_resources` (params: unit_id:i32) → `Result<sys::UnitResources, Error>`
- `UnitsInfo.get_unit_rotation` (params: unit_id:i32) → `Result<sys::UnitRotation, Error>`
- `UnitsInfo.get_unit_seismic_signature` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_self_dtime` (params: unit_id:i32) → `Result<f32, Error>`
- `UnitsInfo.get_unit_sensor_radius` (params: unit_id:i32, r#type:&str) → `Result<sys::UnitSensorRadius, Error>`
- `UnitsInfo.get_unit_shield_state` (params: unit_id:i32, weapon_num:i32) → `Result<(sys::UnitShieldState, bool), Error>`
- `UnitsInfo.get_unit_states` (params: unit_id:i32, options:UnitStatesOptions) → `Result<sys::UnitStates, Error>`
- `UnitsInfo.get_unit_stockpile` (params: unit_id:i32) → `Result<(sys::UnitStockpile, bool), Error>`
- `UnitsInfo.get_unit_storage` (params: unit_id:i32) → `Result<sys::UnitStorage, Error>`
- `UnitsInfo.get_unit_team` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_tooltip` (params: unit_id:i32) → `Result<Option<String>, Error>`
- `UnitsInfo.get_unit_transporter` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsInfo.get_unit_vectors` (params: unit_id:i32) → `Result<sys::UnitVectors, Error>`
- `UnitsInfo.get_unit_velocity` (params: unit_id:i32) → `Result<sys::Float3, Error>`
- `UnitsInfo.get_unit_worker_task` (params: unit_id:i32) → `Result<sys::UnitWorkerTask, Error>`

## UnitsPieces (21 functions)

- `UnitsPieces.get_feature_piece_direction` (params: feature_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_feature_piece_info` (params: feature_id:i32, piece_num:i32) → `Result<(sys::PieceInfo, bool), Error>`
- `UnitsPieces.get_feature_piece_list` (params: feature_id:i32) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_feature_piece_map` (params: feature_id:i32) → `Result<Vec<sys::PieceMapEntry>, Error>`
- `UnitsPieces.get_feature_piece_matrix` (params: feature_id:i32, piece_num:i32) → `Result<sys::PieceMatrix, Error>`
- `UnitsPieces.get_feature_piece_pos_dir` (params: feature_id:i32, piece_num:i32) → `Result<sys::PiecePosDir, Error>`
- `UnitsPieces.get_feature_piece_position` (params: feature_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_feature_root_piece` (params: feature_id:i32) → `Result<i32, Error>`
- `UnitsPieces.get_model_piece_list` (params: model_name:&str) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_model_piece_map` (params: model_name:&str) → `Result<Vec<sys::PieceMapEntry>, Error>`
- `UnitsPieces.get_model_root_piece` (params: model_name:&str) → `Result<i32, Error>`
- `UnitsPieces.get_unit_piece_direction` (params: unit_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_unit_piece_info` (params: unit_id:i32, piece_num:i32) → `Result<(sys::PieceInfo, bool), Error>`
- `UnitsPieces.get_unit_piece_list` (params: unit_id:i32) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_unit_piece_map` (params: unit_id:i32) → `Result<Vec<sys::PieceMapEntry>, Error>`
- `UnitsPieces.get_unit_piece_matrix` (params: unit_id:i32, piece_num:i32) → `Result<sys::PieceMatrix, Error>`
- `UnitsPieces.get_unit_piece_pos_dir` (params: unit_id:i32, piece_num:i32) → `Result<sys::PiecePosDir, Error>`
- `UnitsPieces.get_unit_piece_position` (params: unit_id:i32, piece_num:i32) → `Result<sys::Float3, Error>`
- `UnitsPieces.get_unit_root_piece` (params: unit_id:i32) → `Result<i32, Error>`
- `UnitsPieces.get_unit_script_names` (params: unit_id:i32) → `Result<Vec<String>, Error>`
- `UnitsPieces.get_unit_script_piece` (params: unit_id:i32, script_num:i32) → `Result<i32, Error>`

## UnitsQuery (21 functions)

- `UnitsQuery.get_all_units` (params: ) → `Result<Vec<i32>, Error>`
- `UnitsQuery.get_closest_enemy_unit` (params: pos:sys::Float3, range:f32, ally_team_id:i32, options:GetClosestEnemyUnitOptions) → `Result<i32, Error>`
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
- `UnitsQuery.get_unit_nearest_enemy` (params: unit_id:i32, range:f32, options:GetUnitNearestEnemyOptions) → `Result<i32, Error>`
- `UnitsQuery.get_unit_separation` (params: unit_id1:i32, unit_id2:i32, options:GetUnitSeparationOptions) → `Result<f32, Error>`
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
- `UnitsWeapons.get_unit_weapon_have_free_line_of_fire` (params: unit_id:i32, weapon_num:i32, target_id:i32, source_pos:sys::Float3, target_pos:sys::Float3, options:GetUnitWeaponHaveFreeLineOfFireOptions) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_state` (params: unit_id:i32, weapon_num:i32, key:&str) → `Result<sys::UnitWeaponState, Error>`
- `UnitsWeapons.get_unit_weapon_target` (params: unit_id:i32, weapon_num:i32) → `Result<sys::UnitWeaponTarget, Error>`
- `UnitsWeapons.get_unit_weapon_test_range` (params: unit_id:i32, weapon_num:i32, target_pos:sys::Float3) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_test_target` (params: unit_id:i32, weapon_num:i32, target_id:i32, target_pos:sys::Float3, options:GetUnitWeaponTestTargetOptions) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_try_target` (params: unit_id:i32, weapon_num:i32, target_id:i32, target_pos:sys::Float3, options:GetUnitWeaponTryTargetOptions) → `Result<bool, Error>`
- `UnitsWeapons.get_unit_weapon_vectors` (params: unit_id:i32, weapon_num:i32) → `Result<sys::UnitWeaponVectors, Error>`

## UnsyncedCtrl (82 functions)

- `UnsyncedCtrl.assign_mouse_cursor` (params: command_name:&str, cursor_file_name:&str, overwrite:bool, hot_spot_top_left:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.deselect_unit_map` (params: unit_ids:&[i32]) → `Result<bool, Error>`
- `UnsyncedCtrl.draw_unit_commands` (params: unit_ids:&[i32], table_or_array:bool, queue_draw_depth:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.force_layout_update` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.force_tesselation_update` (params: normal:bool, shadow:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.get_water_texture` (params: tex_type:&str) → `Result<Option<String>, Error>`
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
- `UnsyncedCtrl.set_active_command` (params: cmd_index:i32, button:i32, options:SetActiveCommandOptions) → `Result<bool, Error>`
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
- `UnsyncedCtrl.set_map_shading_texture` (params: tex_type:&str, tex_name:&str, num:i32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_mini_map_rotation` (params: radians:f32) → `Result<(bool, i32), Error>`
- `UnsyncedCtrl.set_mouse_cursor` (params: cursor_name:&str, scale:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_nano_projectile_params` (params: r:f32, v:f32, a:f32, rand_r:f32, rand_v:f32, rand_a:f32) → `Result<bool, Error>`
- `UnsyncedCtrl.set_shock_front_factors` (params: options:SetShockFrontFactorsOptions) → `Result<bool, Error>`
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
- `UnsyncedCtrl.set_water_texture` (params: tex_type:&str, tex_name:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_window_geometry` (params: display_index:i32, window_pos_x:i32, window_pos_y:i32, window_size_x:i32, window_size_y:i32, options:SetWindowGeometryOptions) → `Result<bool, Error>`
- `UnsyncedCtrl.set_window_maximized` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.set_window_minimized` (params: ) → `Result<bool, Error>`
- `UnsyncedCtrl.set_wmcaption` (params: title:&str, title_short:&str) → `Result<bool, Error>`
- `UnsyncedCtrl.set_wmicon` (params: icon_file_name:&str, force_resolution:bool) → `Result<bool, Error>`
- `UnsyncedCtrl.warp_mouse` (params: x:i32, y:i32) → `Result<bool, Error>`

## UnsyncedRead (21 functions)

- `UnsyncedRead.get_active_cmd_desc` (params: cmd_index:i32) → `Result<(sys::ActiveCommandDescription, bool), Error>`
- `UnsyncedRead.get_active_cmd_descs` (params: ) → `Result<Vec<sys::ActiveCommandDescription>, Error>`
- `UnsyncedRead.get_box_selection_by_engine` (params: ) → `Result<bool, Error>`
- `UnsyncedRead.get_build_facing` (params: ) → `Result<i32, Error>`
- `UnsyncedRead.get_build_spacing` (params: ) → `Result<i32, Error>`
- `UnsyncedRead.get_clipboard` (params: ) → `Result<Option<String>, Error>`
- `UnsyncedRead.get_cmd_desc_index` (params: cmd_id:i32) → `Result<i32, Error>`
- `UnsyncedRead.get_custom_palette_color` (params: index:i32) → `Result<(f32, f32, f32, bool), Error>`
- `UnsyncedRead.get_draw_selection_info` (params: ) → `Result<bool, Error>`
- `UnsyncedRead.get_feature_palette_index` (params: feature_id:i32) → `Result<(i32, bool), Error>`
- `UnsyncedRead.get_game_seconds_interpolated` (params: ) → `Result<f32, Error>`
- `UnsyncedRead.get_last_message_positions` (params: ) → `Result<Vec<sys::Float3>, Error>`
- `UnsyncedRead.get_nano_projectile_params` (params: ) → `Result<(f32, f32, f32, f32, f32, f32), Error>`
- `UnsyncedRead.get_piece_projectile_name` (params: projectile_id:i32) → `Result<Option<String>, Error>`
- `UnsyncedRead.get_prev_frame_sync_checksum` (params: ) → `Result<Option<String>, Error>`
- `UnsyncedRead.get_team_damage_stats` (params: team_id:i32) → `Result<(f32, f32, bool), Error>`
- `UnsyncedRead.get_unit_palette_index` (params: unit_id:i32) → `Result<(i32, bool), Error>`
- `UnsyncedRead.is_unit_allied` (params: unit_id:i32) → `Result<bool, Error>`
- `UnsyncedRead.is_unit_selected` (params: unit_id:i32) → `Result<bool, Error>`
- `UnsyncedRead.solve_nurbscurve` (params: degree:i32, points:&[sys::Float4], knots:&[f32], segments:i32) → `Result<(Vec<sys::Float3>, bool), Error>`
- `UnsyncedRead.unit_rendering` (params: ) → `UnitRendering<'_>`

## Utils (7 functions)

- `Utils.closest_build_pos` (params: team_id:i32, unit_def_id:i32, pos:sys::Float3, search_radius:f32, min_dist:i32, facing:i32) → `Result<sys::Float3, Error>`
- `Utils.get_cegid` (params: ceg_name:&str) → `Result<i32, Error>`
- `Utils.get_feature_def_dimensions` (params: feature_def_id:i32) → `Result<sys::UnitDefDimensions, Error>`
- `Utils.get_unit_def_dimensions` (params: unit_def_id:i32) → `Result<sys::UnitDefDimensions, Error>`
- `Utils.pos2_build_pos` (params: unit_def_id:i32, pos:sys::Float3, facing:i32) → `Result<sys::Float3, Error>`
- `Utils.test_build_order` (params: unit_def_id:i32, pos:sys::Float3, facing:i32) → `Result<(i32, bool, i32), Error>`
- `Utils.test_move_order` (params: unit_def_id:i32, pos:sys::Float3, dir:sys::Float3, options:TestMoveOrderOptions) → `Result<bool, Error>`

## Vfs (55 functions)

- `Vfs.abort_download` (params: id:i32) → `Result<bool, Error>`
- `Vfs.calculate_hash` (params: data:&[u8], hash_type:i32) → `Result<Option<String>, Error>`
- `Vfs.compress_folder` (params: folder_path:&str, archive_type:&str, compressed_file_path:&str, include_folder:bool, mode:&str) → `Result<bool, Error>`
- `Vfs.create_dir` (params: path:&str) → `Result<bool, Error>`
- `Vfs.dir_list` (params: path:&str, pattern:&str, mode:&str, recursive:bool) → `Result<Vec<sys::DirEntry>, Error>`
- `Vfs.dir_list_names` (params: path:&str, pattern:&str, mode:&str, recursive:bool) → `Result<Vec<String>, Error>`
- `Vfs.download_archive` (params: filename:&str, category:&str) → `Result<(), Error>`
- `Vfs.extract_mod_archive_file` (params: path:&str) → `Result<bool, Error>`
- `Vfs.file_exists` (params: path:&str) → `Result<bool, Error>`
- `Vfs.get_all_archives` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.get_archive_checksum` (params: archive_name:&str) → `Result<(Option<String>, Option<String>), Error>`
- `Vfs.get_archive_containing_file` (params: path:&str, mode:&str) → `Result<Option<String>, Error>`
- `Vfs.get_archive_dependencies` (params: archive_name:&str) → `Result<Vec<String>, Error>`
- `Vfs.get_archive_info` (params: archive_name:&str) → `Result<Vec<sys::ArchiveInfoEntry>, Error>`
- `Vfs.get_archive_path` (params: archive_name:&str) → `Result<Option<String>, Error>`
- `Vfs.get_archive_replaces` (params: archive_name:&str) → `Result<Vec<String>, Error>`
- `Vfs.get_archives` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.get_available_ais` (params: game_archive_name:&str, map_archive_name:&str) → `Result<Vec<sys::AIInfoEntry>, Error>`
- `Vfs.get_file_absolute_path` (params: path:&str, mode:&str) → `Result<Option<String>, Error>`
- `Vfs.get_file_info` (params: path:&str) → `Result<(sys::FileInfo, bool), Error>`
- `Vfs.get_file_size` (params: path:&str) → `Result<u32, Error>`
- `Vfs.get_games` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.get_loaded_archives` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.get_map_square_texture` (params: tex_square_x:i32, tex_square_y:i32, lod_min:i32, texture_name:&str, lod_max:i32) → `Result<bool, Error>`
- `Vfs.get_map_square_texture_info` (params: ) → `Result<(i32, i32, i32), Error>`
- `Vfs.get_maps` (params: ) → `Result<Vec<String>, Error>`
- `Vfs.get_name_from_rapid_tag` (params: rapid_tag:&str) → `Result<Option<String>, Error>`
- `Vfs.has_archive` (params: archive_name:&str) → `Result<bool, Error>`
- `Vfs.is_directory` (params: path:&str) → `Result<bool, Error>`
- `Vfs.list_dir` (params: path:&str, pattern:&str, mode:&str, recursive:bool) → `Result<Vec<sys::DirEntry>, Error>`
- `Vfs.list_dir_names` (params: path:&str, pattern:&str, mode:&str, recursive:bool) → `Result<Vec<String>, Error>`
- `Vfs.list_entries` (params: path:&str, pattern:&str, mode:&str, recursive:bool) → `Result<Vec<DirectoryEntry>, Error>`
- `Vfs.load_file` (params: path:&str, mode:&str) → `Result<Vec<u8>, Error>`
- `Vfs.pack_f32` (params: values:&[f32]) → `Result<Vec<u8>, Error>`
- `Vfs.pack_s16` (params: values:&[i16]) → `Result<Vec<u8>, Error>`
- `Vfs.pack_s32` (params: values:&[i32]) → `Result<Vec<u8>, Error>`
- `Vfs.pack_s8` (params: values:&[i8]) → `Result<Vec<u8>, Error>`
- `Vfs.pack_u16` (params: values:&[u16]) → `Result<Vec<u8>, Error>`
- `Vfs.pack_u32` (params: values:&[u32]) → `Result<Vec<u8>, Error>`
- `Vfs.pack_u8` (params: values:&[u8]) → `Result<Vec<u8>, Error>`
- `Vfs.read_file` (params: path:&str) → `Result<Vec<u8>, Error>`
- `Vfs.read_file_as_string` (params: path:&str) → `Result<Option<String>, Error>`
- `Vfs.scan_all_dirs` (params: ) → `Result<(), Error>`
- `Vfs.set_map_square_texture` (params: tex_square_x:i32, tex_square_y:i32, texture_name:&str) → `Result<bool, Error>`
- `Vfs.sub_dirs` (params: path:&str, pattern:&str, mode:&str, recursive:bool) → `Result<Vec<String>, Error>`
- `Vfs.unpack_f32` (params: data:&[u8], byte_offset:u32, count:u32) → `Result<Vec<f32>, Error>`
- `Vfs.unpack_s16` (params: data:&[u8], byte_offset:u32, count:u32) → `Result<Vec<i16>, Error>`
- `Vfs.unpack_s32` (params: data:&[u8], byte_offset:u32, count:u32) → `Result<Vec<i32>, Error>`
- `Vfs.unpack_s8` (params: data:&[u8], byte_offset:u32, count:u32) → `Result<Vec<i8>, Error>`
- `Vfs.unpack_u16` (params: data:&[u8], byte_offset:u32, count:u32) → `Result<Vec<u16>, Error>`
- `Vfs.unpack_u32` (params: data:&[u8], byte_offset:u32, count:u32) → `Result<Vec<u32>, Error>`
- `Vfs.unpack_u8` (params: data:&[u8], byte_offset:u32, count:u32) → `Result<Vec<u8>, Error>`
- `Vfs.use_archive` (params: archive_name:&str, callback:F) → `Result<bool, Error>`
- `Vfs.zlib_compress` (params: data:&[u8]) → `Result<Vec<u8>, Error>`
- `Vfs.zlib_decompress` (params: data:&[u8]) → `Result<Vec<u8>, Error>`

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

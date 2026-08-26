

#[cfg(feature = "alloc")]
pub use crate::owned::gfx::{active_texture, add_atlas_texture, add_fallback_font, add_feature_defs_to_submission_vao, add_features_to_submission_vao, add_unit_defs_to_submission_vao, add_units_to_submission_vao, alpha_test, alpha_to_coverage, attach_index_buffer_vao, attach_instance_buffer_vao, attach_vertex_buffer_vao, begin_text, billboard, bind_buffer_range_vbo, bind_image_texture, bind_texture, blend_equation, blend_equation_separate, blend_func, blend_func_separate, blending, blit_fbo, call_list, change_texture_params, clear, clear_attachment_fbo, clear_fallback_fonts, clear_submission_vao, clear_vbo, clip_distance, clip_plane, color, color_mask, config_mini_map, copy_to_texture, copy_to_vbo, create_fbo, create_query, create_rbo, create_shader, create_texture, create_texture_atlas, cull_face, culling, define_vbo, delete_fbo, delete_font, delete_list, delete_query, delete_rbo, delete_shader, delete_texture, delete_texture_atlas, delete_texture_fbo, delete_vao, delete_vbo, depth_clamp, depth_mask, depth_test, dispatch_compute, download_vbo, draw_arrays_vao, draw_elements_vao, draw_ground_circle, draw_ground_quad, draw_list_at_unit, draw_mini_map, dump_definition_vbo, edge_flag, end_text, feature, feature_mult_matrix, feature_piece, feature_piece_matrix, feature_piece_mult_matrix, feature_raw, feature_shape, feature_shape_textures, feature_textures, finalize_texture_atlas, finish, flush, fog, fog_coord, font_begin, font_bind_texture, font_end, font_get_text_height, font_get_text_width, font_print, font_print_world, font_set_auto_outline_color, font_set_outline_color, font_set_text_color, font_submit_buffered, font_wrap_text, frustum, generate_mipmap, get_active_uniforms, get_atlas_texture, get_atmosphere, get_console_commands, get_engine_atlas_textures, get_engine_model_uniform_data_def, get_engine_model_uniform_data_size, get_engine_texture_names, get_engine_uniform_buffer_def, get_fixed_state, get_font_info, get_global_tex_coords, get_global_tex_names, get_idvbo, get_map_rendering, get_matrix_data, get_number, get_query, get_rbo_info, get_screen_view_trans, get_shader_log, get_shadow_map_params, get_string, get_subroutine_index, get_sun, get_text_height, get_text_width, get_uniform_location, get_vao, get_vbo, get_vbo_info, get_view_range, get_view_sizes, get_water_rendering, has_extension, instance_data_from_feature_defs_vbo, instance_data_from_features_vbo, instance_data_from_unit_defs_vbo, instance_data_from_units_vbo, is_valid_fbo, light, lighting, line_stipple, line_width, load_font, load_identity, load_matrix, logic_op, material, matrix_data_from_projectiles_vbo, matrix_mode, memory_barrier, models_vbo, multi_tex_coord, multi_tex_env, multi_tex_gen, mult_matrix, normal, object_label, ortho, point_parameter, point_size, point_sprite, polygon_mode, polygon_offset, pop_attrib, pop_debug_group, pop_matrix, push_attrib, push_debug_group, push_matrix, raw_bind_fbo, read_pixels, rect, remove_from_submission_vao, reset_matrices, reset_state, rotate, save_image, scale, scissor, secondary_color, set_fbo_attachment, set_fbo_draw_buffers, set_fbo_read_buffer, set_feature_buffer_uniforms, set_geometry_shader_parameter, set_tesselation_shader_parameter, set_unit_buffer_uniforms, shade_model, shape, slave_mini_map, stencil_func, stencil_func_separate, stencil_mask, stencil_mask_separate, stencil_op, stencil_op_separate, stencil_test, submit_vao, swap_buffers, tex_coord, tex_env, tex_gen, tex_rect, text, text_env, texture_info, translate, unbind_buffer_range_vbo, uniform, uniform_array_float, uniform_array_int, uniform_int, uniform_matrix, uniform_subroutine, unit, unit_mult_matrix, unit_piece, unit_piece_matrix, unit_piece_mult_matrix, unit_raw, unit_shape, unit_shape_textures, unit_textures, upload_texture, upload_vbo, use_shader, vertex, viewport};

#[path = "gfx_resources.rs"]
mod resources;
pub use resources::*;

use super::{ApiError, ErrorCode, Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:gfx")]
    unsafe extern "C" {
        #[link_name = "begin-end"]
        pub fn begin_end(primitive: i32, callback_id: i32, user_data: i32) -> i32;
        #[link_name = "active-fbo"]
        pub fn active_fbo(
            fbo_id: i32,
            target: i32,
            identities: i32,
            callback_id: i32,
            user_data: i32,
        ) -> i32;
        #[link_name = "active-shader"]
        pub fn active_shader(shader_id: i32, callback_id: i32, user_data: i32) -> i32;
        #[link_name = "create-list"]
        pub fn create_list(callback_id: i32, user_data: i32) -> i64;
        #[link_name = "draw-func-at-unit"]
        pub fn draw_func_at_unit(
            unit_id: i32,
            use_mid_pos: i32,
            callback_id: i32,
            user_data: i32,
        ) -> i32;
        #[link_name = "push-pop-matrix"]
        pub fn push_pop_matrix(callback_id: i32, user_data: i32) -> i32;
        #[link_name = "render-to-texture"]
        pub fn render_to_texture(
            name_ptr: i32,
            name_len: i32,
            callback_id: i32,
            user_data: i32,
        ) -> i32;
        #[link_name = "run-query"]
        pub fn run_query(query_id: i32, callback_id: i32, user_data: i32) -> i32;
        #[link_name = "unsafe-state"]
        pub fn unsafe_state(state: i32, reverse: i32, callback_id: i32, user_data: i32) -> i32;
    }
}

#[inline]
fn status(status: i32) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(ApiError::new(status))
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn string_parts(value: &str) -> Result<(i32, i32)> {
    let pointer = value.as_ptr() as usize;
    if pointer > u32::MAX as usize || value.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    Ok((pointer as u32 as i32, value.len() as u32 as i32))
}

#[inline]
pub fn begin_end(primitive: u32, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(unsafe {
            raw::begin_end(
                primitive as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (primitive, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn active_fbo(
    fbo_id: u32,
    target: u32,
    identities: bool,
    callback: SyncCallback,
) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(unsafe {
            raw::active_fbo(
                fbo_id as i32,
                target as i32,
                identities as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (fbo_id, target, identities, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn active_shader(shader_id: u32, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(unsafe {
            raw::active_shader(
                shader_id as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (shader_id, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn create_list(callback: SyncCallback) -> Result<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_i32(unsafe {
            raw::create_list(callback.id as i32, callback.user_data as i32)
        })
        .map(|value| value as u32)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = callback;
        Err(unreachable!())
    }
}

#[inline]
pub fn draw_func_at_unit(unit_id: i32, use_mid_pos: bool, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(unsafe {
            raw::draw_func_at_unit(
                unit_id,
                use_mid_pos as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, use_mid_pos, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn push_pop_matrix(callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(unsafe { raw::push_pop_matrix(callback.id as i32, callback.user_data as i32) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = callback;
        Err(unreachable!())
    }
}

#[inline]
pub fn render_to_texture(name: &str, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = string_parts(name)?;
        status(unsafe {
            raw::render_to_texture(
                pointer,
                length,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (name, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn run_query(query_id: u32, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(unsafe {
            raw::run_query(
                query_id as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (query_id, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn unsafe_state(state_id: u32, reverse: bool, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(unsafe {
            raw::unsafe_state(
                state_id as i32,
                reverse as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (state_id, reverse, callback);
        Err(unreachable!())
    }
}

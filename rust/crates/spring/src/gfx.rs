#[cfg(feature = "alloc")]
pub use crate::owned::gfx::*;

#[path = "gfx_resources.rs"]
mod resources;

pub use resources::{
    NATIVE_GFX_RESOURCE_NAME_MAX_BYTES, TextureCreateParams, create_texture_atlas_into,
    create_texture_into,
};

#[cfg(feature = "alloc")]
pub use resources::{create_texture, create_texture_atlas};

#[cfg(not(feature = "alloc"))]
pub use resources::{
    create_texture_atlas_into as create_texture_atlas, create_texture_into as create_texture,
};

use super::{ApiError, Result, SyncCallback};

/// Shader stage sources and optional geometry-stage configuration.
///
/// Every field defaults to "unset", so callers name only the stages they
/// actually supply:
///
/// ```ignore
/// let shader = spring::gfx::create_shader(spring::gfx::ShaderCreateParams {
///     fragment: FRAG_SRC,
///     ..Default::default()
/// })?;
/// ```
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ShaderCreateParams<'a> {
    /// Text prepended to every supplied stage, e.g. `#define` blocks.
    pub definitions: &'a str,
    pub vertex: &'a str,
    /// Tessellation control stage.
    pub tcs: &'a str,
    /// Tessellation evaluation stage.
    pub tes: &'a str,
    pub geometry: &'a str,
    pub fragment: &'a str,
    pub compute: &'a str,
    /// Geometry-stage input primitive; left to the engine default when `None`.
    pub geo_input_type: Option<u32>,
    /// Geometry-stage output primitive; left to the engine default when `None`.
    pub geo_output_type: Option<u32>,
    /// Maximum vertices the geometry stage emits; engine default when `None`.
    pub geo_output_verts: Option<i32>,
}

/// Compile and link a shader from the stages named in `params`.
///
/// This is the ergonomic form of the generated positional callout: the
/// unsupplied stages stay empty through [`Default`] instead of having to be
/// spelled out as `""` in the right order.
#[cfg(feature = "alloc")]
#[inline]
pub fn create_shader(
    params: ShaderCreateParams<'_>,
) -> Result<crate::owned::gfx::CreateShaderValue> {
    crate::owned::gfx::create_shader(
        params.definitions,
        params.vertex,
        params.tcs,
        params.tes,
        params.geometry,
        params.fragment,
        params.compute,
        crate::owned::gfx::GfxCreateShaderOptions {
            has_geo_input_type: params.geo_input_type.is_some(),
            geo_input_type: params.geo_input_type.unwrap_or(0),
            has_geo_output_type: params.geo_output_type.is_some(),
            geo_output_type: params.geo_output_type.unwrap_or(0),
            has_geo_output_verts: params.geo_output_verts.is_some(),
            geo_output_verts: params.geo_output_verts.unwrap_or(0),
        },
    )
}

#[cfg(all(test, feature = "alloc"))]
mod api_tests {
    use super::*;

    #[test]
    fn resource_creation_uses_typed_output_api() {
        let _: fn(i32, i32, i32, TextureCreateParams) -> Result<alloc::string::String> =
            create_texture;
        let _: fn(i32, i32, i32, TextureCreateParams, &mut [u8]) -> Result<&str> =
            create_texture_into;
        let _: fn(i32, i32, i32) -> Result<alloc::string::String> = create_texture_atlas;
        let _: fn(i32, i32, i32, &mut [u8]) -> Result<&str> = create_texture_atlas_into;
    }
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:gfx")]
    unsafe extern "C" {
        #[link_name = "begin-end"]
        pub safe fn begin_end(primitive: i32, callback_id: i32, user_data: i32) -> i32;
        #[link_name = "active-fbo"]
        pub safe fn active_fbo(
            fbo_id: i32,
            target: i32,
            identities: i32,
            callback_id: i32,
            user_data: i32,
        ) -> i32;
        #[link_name = "active-shader"]
        pub safe fn active_shader(shader_id: i32, callback_id: i32, user_data: i32) -> i32;
        #[link_name = "create-list"]
        pub safe fn create_list(callback_id: i32, user_data: i32) -> i64;
        #[link_name = "draw-func-at-unit"]
        pub safe fn draw_func_at_unit(
            unit_id: i32,
            use_mid_pos: i32,
            callback_id: i32,
            user_data: i32,
        ) -> i32;
        #[link_name = "push-pop-matrix"]
        pub safe fn push_pop_matrix(callback_id: i32, user_data: i32) -> i32;
        #[link_name = "render-to-texture"]
        pub safe fn render_to_texture(
            name_ptr: i32,
            name_len: i32,
            callback_id: i32,
            user_data: i32,
        ) -> i32;
        #[link_name = "run-query"]
        pub safe fn run_query(query_id: i32, callback_id: i32, user_data: i32) -> i32;
        #[link_name = "unsafe-state"]
        pub safe fn unsafe_state(state: i32, reverse: i32, callback_id: i32, user_data: i32)
        -> i32;
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
    super::wasm_slice_parts(value.as_bytes())
}

#[inline]
pub fn begin_end(primitive: u32, callback: impl crate::callback::SyncHandler) -> Result<()> {
    callback.run_sync(|cb| begin_end_callback(primitive, cb))
}

#[inline]
pub fn begin_end_callback(primitive: u32, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(raw::begin_end(
            primitive as i32,
            callback.id as i32,
            callback.user_data as i32,
        ))
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
    callback: impl crate::callback::SyncHandler,
) -> Result<()> {
    callback.run_sync(|cb| active_fbo_callback(fbo_id, target, identities, cb))
}

#[inline]
pub fn active_fbo_callback(
    fbo_id: u32,
    target: u32,
    identities: bool,
    callback: SyncCallback,
) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(raw::active_fbo(
            fbo_id as i32,
            target as i32,
            identities as i32,
            callback.id as i32,
            callback.user_data as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (fbo_id, target, identities, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn active_shader(shader_id: u32, callback: impl crate::callback::SyncHandler) -> Result<()> {
    callback.run_sync(|cb| active_shader_callback(shader_id, cb))
}

#[inline]
pub fn active_shader_callback(shader_id: u32, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(raw::active_shader(
            shader_id as i32,
            callback.id as i32,
            callback.user_data as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (shader_id, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn create_list(callback: impl crate::callback::SyncHandler<u32>) -> Result<u32> {
    callback.run_sync(create_list_callback)
}

#[inline]
pub fn create_list_callback(callback: SyncCallback) -> Result<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_i32(raw::create_list(
            callback.id as i32,
            callback.user_data as i32,
        ))
        .map(|value| value as u32)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = callback;
        Err(unreachable!())
    }
}

#[inline]
pub fn draw_func_at_unit(
    unit_id: i32,
    use_mid_pos: bool,
    callback: impl crate::callback::SyncHandler,
) -> Result<()> {
    callback.run_sync(|cb| draw_func_at_unit_callback(unit_id, use_mid_pos, cb))
}

#[inline]
pub fn draw_func_at_unit_callback(
    unit_id: i32,
    use_mid_pos: bool,
    callback: SyncCallback,
) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(raw::draw_func_at_unit(
            unit_id,
            use_mid_pos as i32,
            callback.id as i32,
            callback.user_data as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (unit_id, use_mid_pos, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn push_pop_matrix(callback: impl crate::callback::SyncHandler) -> Result<()> {
    callback.run_sync(push_pop_matrix_callback)
}

#[inline]
pub fn push_pop_matrix_callback(callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(raw::push_pop_matrix(
            callback.id as i32,
            callback.user_data as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = callback;
        Err(unreachable!())
    }
}

#[inline]
pub fn render_to_texture(name: &str, callback: impl crate::callback::SyncHandler) -> Result<()> {
    callback.run_sync(|cb| render_to_texture_callback(name, cb))
}

#[inline]
pub fn render_to_texture_callback(name: &str, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let (pointer, length) = string_parts(name)?;
        status(raw::render_to_texture(
            pointer,
            length,
            callback.id as i32,
            callback.user_data as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (name, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn run_query(query_id: u32, callback: impl crate::callback::SyncHandler) -> Result<()> {
    callback.run_sync(|cb| run_query_callback(query_id, cb))
}

#[inline]
pub fn run_query_callback(query_id: u32, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(raw::run_query(
            query_id as i32,
            callback.id as i32,
            callback.user_data as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (query_id, callback);
        Err(unreachable!())
    }
}

#[inline]
pub fn unsafe_state(
    state_id: u32,
    reverse: bool,
    callback: impl crate::callback::SyncHandler,
) -> Result<()> {
    callback.run_sync(|cb| unsafe_state_callback(state_id, reverse, cb))
}

#[inline]
pub fn unsafe_state_callback(state_id: u32, reverse: bool, callback: SyncCallback) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        status(raw::unsafe_state(
            state_id as i32,
            reverse as i32,
            callback.id as i32,
            callback.user_data as i32,
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (state_id, reverse, callback);
        Err(unreachable!())
    }
}

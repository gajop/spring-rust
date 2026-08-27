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
pub fn begin_end(primitive: u32, callback: SyncCallback) -> Result<()> {
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
pub fn active_shader(shader_id: u32, callback: SyncCallback) -> Result<()> {
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
pub fn create_list(callback: SyncCallback) -> Result<u32> {
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
pub fn draw_func_at_unit(unit_id: i32, use_mid_pos: bool, callback: SyncCallback) -> Result<()> {
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
pub fn push_pop_matrix(callback: SyncCallback) -> Result<()> {
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
pub fn render_to_texture(name: &str, callback: SyncCallback) -> Result<()> {
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
pub fn run_query(query_id: u32, callback: SyncCallback) -> Result<()> {
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
pub fn unsafe_state(state_id: u32, reverse: bool, callback: SyncCallback) -> Result<()> {
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

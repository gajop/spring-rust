#[path = "gfx_resources.rs"]
mod resources;
pub use resources::*;

use super::{ApiError, ErrorCode, Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:gfx")]
    extern "C" {
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
        return status(unsafe {
            raw::begin_end(
                primitive as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
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
        return status(unsafe {
            raw::active_fbo(
                fbo_id as i32,
                target as i32,
                identities as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
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
        return status(unsafe {
            raw::active_shader(
                shader_id as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
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
        return super::unpack_i32(unsafe {
            raw::create_list(callback.id as i32, callback.user_data as i32)
        })
        .map(|value| value as u32);
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
        return status(unsafe {
            raw::draw_func_at_unit(
                unit_id,
                use_mid_pos as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
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
        return status(unsafe {
            raw::push_pop_matrix(callback.id as i32, callback.user_data as i32)
        });
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
        return status(unsafe {
            raw::render_to_texture(
                pointer,
                length,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
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
        return status(unsafe {
            raw::run_query(
                query_id as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
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
        return status(unsafe {
            raw::unsafe_state(
                state_id as i32,
                reverse as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (state_id, reverse, callback);
        Err(unreachable!())
    }
}

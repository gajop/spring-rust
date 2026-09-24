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

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// Matrix mode for the camera's combined view and projection transform.
pub const MATRIX_VIEWPROJECTION: u32 = 0x10001;
/// Matrix mode for the inverse of the camera's combined view and projection transform.
pub const MATRIX_VIEWPROJECTION_INVERSE: u32 = 0x10000;
/// Named engine matrix modes accepted by `get_matrix_data`.
pub const MATRIX_VIEW: u32 = 0x10002;
pub const MATRIX_VIEW_INVERSE: u32 = 0x10003;
pub const MATRIX_PROJECTION: u32 = 0x10004;
pub const MATRIX_PROJECTION_INVERSE: u32 = 0x10005;
pub const MATRIX_BILLBOARD: u32 = 0x10006;
pub const MATRIX_SHADOW: u32 = 0x10007;

#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq)]
pub struct ValueQueryResult {
    pub values: [f32; 4],
    pub count: u32,
    pub bool_value: Option<bool>,
    pub string_value: Option<alloc::string::String>,
}

#[cfg(feature = "alloc")]
pub type AtmosphereValue = ValueQueryResult;

#[cfg(feature = "alloc")]
pub type SunValue = ValueQueryResult;

#[cfg(feature = "alloc")]
pub type WaterRenderingValue = ValueQueryResult;

#[cfg(feature = "alloc")]
pub type MapRenderingValue = ValueQueryResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderUniformInt<'a> {
    pub name: &'a str,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaderUniformFloat<'a> {
    pub name: &'a str,
    pub value: f32,
}

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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
    /// Integer uniforms to assign after the shader is linked. Bind samplers to
    /// texture units here, e.g. `ShaderUniformInt { name: "tex", value: 0 }`,
    /// instead of setting them after every `use_shader`.
    pub uniform_ints: &'a [ShaderUniformInt<'a>],
    /// Floating-point uniforms to assign after the shader is linked.
    pub uniform_floats: &'a [ShaderUniformFloat<'a>],
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
    let shader = crate::owned::gfx::create_shader(
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
    )?;
    // The engine reports a failed compile or link as shader 0; the log has
    // the compiler's messages.
    if shader.shader_id == 0 {
        return Err(crate::ApiError::new(crate::ErrorCode::OperationFailed as i32));
    }

    if params.uniform_ints.is_empty() && params.uniform_floats.is_empty() {
        return Ok(shader);
    }

    let configured = (|| {
        let _ = crate::owned::gfx::use_shader(shader.shader_id)?;
        for uniform in params.uniform_ints {
            let location = crate::owned::gfx::get_uniform_location(shader.shader_id, uniform.name)?;
            crate::owned::gfx::uniform_int(location, &[uniform.value], 1)?;
        }
        for uniform in params.uniform_floats {
            let location = crate::owned::gfx::get_uniform_location(shader.shader_id, uniform.name)?;
            crate::owned::gfx::uniform(location, &[uniform.value], 1)?;
        }
        Ok::<(), crate::ApiError>(())
    })();
    let unbound = crate::owned::gfx::use_shader(0);
    if let Err(error) = configured {
        let _ = unbound;
        return Err(error);
    }
    let _ = unbound?;

    Ok(shader)
}

/// A uniform's location in `shader`, or `None` when the shader has no active
/// uniform of that name (the GLSL compiler drops unused ones).
#[cfg(feature = "alloc")]
#[inline]
pub fn uniform_location(shader: u32, name: &str) -> Option<i32> {
    crate::owned::gfx::get_uniform_location(shader, name)
        .ok()
        .filter(|location| *location >= 0)
}

/// Return the camera's combined view and projection matrix.
#[cfg(feature = "alloc")]
#[inline]
pub fn get_view_projection_matrix() -> Result<[f32; 16]> {
    crate::owned::gfx::get_matrix_data(MATRIX_VIEWPROJECTION)
}

/// Return the inverse of the camera's combined view and projection matrix.
#[cfg(feature = "alloc")]
#[inline]
pub fn get_view_projection_matrix_inverse() -> Result<[f32; 16]> {
    crate::owned::gfx::get_matrix_data(MATRIX_VIEWPROJECTION_INVERSE)
}

/// Return one of the engine's named matrices used by Lua's `gl.GetMatrixData`.
#[cfg(feature = "alloc")]
#[inline]
pub fn get_named_matrix(mode: u32) -> Result<[f32; 16]> {
    crate::owned::gfx::get_matrix_data(mode)
}

/// Upload one of the engine's named matrices used by Lua's `gl.UniformMatrix`.
#[cfg(feature = "alloc")]
#[inline]
pub fn uniform_named_matrix(location: i32, mode: u32) -> Result<()> {
    let matrix = get_named_matrix(mode)?;
    crate::owned::gfx::uniform_matrix(location, &matrix, false)
}

/// Read one of the typed results from Lua's `gl.GetAtmosphere`, `gl.GetSun`,
/// `gl.GetWaterRendering`, or `gl.GetMapRendering` queries.
#[cfg(feature = "alloc")]
#[inline]
fn get_value_query(key: &str, mode: &str, call: fn(i32, i32) -> i32) -> Result<ValueQueryResult> {
    let mut key_wire = Vec::with_capacity(4 + key.len());
    key_wire.extend_from_slice(&(key.len() as u32).to_le_bytes());
    key_wire.extend_from_slice(key.as_bytes());
    let mut mode_wire = Vec::with_capacity(4 + mode.len());
    mode_wire.extend_from_slice(&(mode.len() as u32).to_le_bytes());
    mode_wire.extend_from_slice(mode.as_bytes());
    let (key_pointer, key_length) = super::wasm_slice_parts(&key_wire)?;
    let (mode_pointer, mode_length) = super::wasm_slice_parts(&mode_wire)?;
    let mut input = [
        key_pointer as u32,
        key_length as u32,
        mode_pointer as u32,
        mode_length as u32,
    ];
    let input_descriptor = super::wasm_output_ptr(&mut input)?;
    let mut string_output = Vec::new();
    let mut output = [0u32; 10];

    loop {
        let (string_pointer, string_capacity) = super::wasm_slice_parts(&string_output)?;
        output[0] = string_pointer as u32;
        output[1] = string_capacity as u32;
        output[2] = 0;
        let output_descriptor = super::wasm_output_ptr(&mut output)?;
        let status = call(input_descriptor, output_descriptor);
        let required = output[2] as usize;
        if status == super::ErrorCode::BufferOverflow as i32 {
            string_output.resize(required, 0);
            continue;
        }
        if status != 0 {
            return Err(ApiError::new(status));
        }
        let string_value = if required == 0 {
            None
        } else {
            Some(
                String::from_utf8(string_output)
                    .map_err(|_| ApiError::new(super::ErrorCode::Internal as i32))?,
            )
        };
        return Ok(ValueQueryResult {
            values: [
                f32::from_bits(output[3]),
                f32::from_bits(output[4]),
                f32::from_bits(output[5]),
                f32::from_bits(output[6]),
            ],
            count: output[7],
            bool_value: (output[8] != 0).then_some(output[9] != 0),
            string_value,
        });
    }
}

#[cfg(feature = "alloc")]
#[inline]
pub fn get_atmosphere(key: &str, mode: &str) -> Result<AtmosphereValue> {
    get_value_query(key, mode, crate::owned::gfx::get_atmosphere)
}

#[cfg(feature = "alloc")]
#[inline]
pub fn get_sun(key: &str, mode: &str) -> Result<SunValue> {
    get_value_query(key, mode, crate::owned::gfx::get_sun)
}

#[cfg(feature = "alloc")]
#[inline]
pub fn get_water_rendering(key: &str, mode: &str) -> Result<WaterRenderingValue> {
    get_value_query(key, mode, crate::owned::gfx::get_water_rendering)
}

#[cfg(feature = "alloc")]
#[inline]
pub fn get_map_rendering(key: &str, mode: &str) -> Result<MapRenderingValue> {
    get_value_query(key, mode, crate::owned::gfx::get_map_rendering)
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

/// Render into a named texture.
///
/// The engine resets the projection and modelview matrices to identity for
/// this pass. Set both matrices explicitly before drawing world-space data.
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

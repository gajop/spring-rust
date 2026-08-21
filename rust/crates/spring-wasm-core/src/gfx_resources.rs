//! Single-call Core transport for mutating Gfx resource creation.
//!
//! Native texture/atlas names are bounded to 17 bytes. Callers provide that
//! storage before the native mutation, so resource creation never needs a
//! probe/retry invocation merely to discover the returned string length.

use crate::{ApiError, ErrorCode, Result};

pub const NATIVE_GFX_RESOURCE_NAME_MAX_BYTES: usize = 17;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GfxTextureParams {
    pub target: u32,
    pub format: u32,
    pub border: i32,
    pub min_filter: u32,
    pub mag_filter: u32,
    pub wrap_s: u32,
    pub wrap_t: u32,
    pub wrap_r: u32,
    pub compare_func: u32,
    pub lod_bias: f32,
    pub aniso: f32,
    pub samples: u32,
    pub fbo: bool,
    pub fbo_depth: bool,
}

impl Default for GfxTextureParams {
    fn default() -> Self {
        Self {
            target: 0,
            format: 0,
            border: 0,
            min_filter: 0,
            mag_filter: 0,
            wrap_s: 0,
            wrap_t: 0,
            wrap_r: 0,
            compare_func: 0,
            lod_bias: 0.0,
            aniso: 0.0,
            samples: 0,
            fbo: false,
            fbo_depth: false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:gfx")]
    extern "C" {
        #[link_name = "create-texture"]
        pub fn create_texture(
            xsize: i32,
            ysize: i32,
            zsize: i32,
            params_ptr: i32,
            output_ptr: i32,
            output_capacity: i32,
        ) -> i64;

        #[link_name = "create-texture-atlas"]
        pub fn create_texture_atlas(
            xsize: i32,
            ysize: i32,
            alloc_type: i32,
            output_ptr: i32,
            output_capacity: i32,
        ) -> i64;
    }
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn put_u32(output: &mut [u8; 56], offset: usize, value: u32) {
    output[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn encode_texture_params(params: GfxTextureParams) -> [u8; 56] {
    let mut output = [0u8; 56];
    put_u32(&mut output, 0, params.target);
    put_u32(&mut output, 4, params.format);
    put_u32(&mut output, 8, params.border as u32);
    put_u32(&mut output, 12, params.min_filter);
    put_u32(&mut output, 16, params.mag_filter);
    put_u32(&mut output, 20, params.wrap_s);
    put_u32(&mut output, 24, params.wrap_t);
    put_u32(&mut output, 28, params.wrap_r);
    put_u32(&mut output, 32, params.compare_func);
    put_u32(&mut output, 36, params.lod_bias.to_bits());
    put_u32(&mut output, 40, params.aniso.to_bits());
    put_u32(&mut output, 44, params.samples);
    put_u32(&mut output, 48, params.fbo as u32);
    put_u32(&mut output, 52, params.fbo_depth as u32);
    output
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn checked_output(output: &mut [u8]) -> Result<(i32, i32)> {
    if output.len() < NATIVE_GFX_RESOURCE_NAME_MAX_BYTES || output.len() > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let pointer = output.as_mut_ptr() as usize;
    if pointer > u32::MAX as usize {
        return Err(ApiError::new(ErrorCode::OutOfBounds as i32));
    }
    Ok((pointer as u32 as i32, output.len() as u32 as i32))
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn finish_name<'a>(packed: i64, output: &'a [u8]) -> Result<&'a str> {
    let length = crate::unpack_i32(packed)?;
    if length < 0
        || length as usize > NATIVE_GFX_RESOURCE_NAME_MAX_BYTES
        || length as usize > output.len()
    {
        return Err(ApiError::new(ErrorCode::Internal as i32));
    }
    core::str::from_utf8(&output[..length as usize])
        .map_err(|_| ApiError::new(ErrorCode::Internal as i32))
}

#[inline]
pub fn create_texture<'a>(
    xsize: i32,
    ysize: i32,
    zsize: i32,
    params: GfxTextureParams,
    output: &'a mut [u8],
) -> Result<&'a str> {
    #[cfg(target_arch = "wasm32")]
    {
        let wire = encode_texture_params(params);
        let wire_pointer = wire.as_ptr() as usize;
        if wire_pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::OutOfBounds as i32));
        }
        let (output_pointer, output_capacity) = checked_output(output)?;
        let packed = unsafe {
            raw::create_texture(
                xsize,
                ysize,
                zsize,
                wire_pointer as u32 as i32,
                output_pointer,
                output_capacity,
            )
        };
        finish_name(packed, output)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (xsize, ysize, zsize, params, output);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

#[inline]
pub fn create_texture_atlas<'a>(
    xsize: i32,
    ysize: i32,
    alloc_type: i32,
    output: &'a mut [u8],
) -> Result<&'a str> {
    #[cfg(target_arch = "wasm32")]
    {
        let (output_pointer, output_capacity) = checked_output(output)?;
        let packed = unsafe {
            raw::create_texture_atlas(xsize, ysize, alloc_type, output_pointer, output_capacity)
        };
        finish_name(packed, output)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (xsize, ysize, alloc_type, output);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}

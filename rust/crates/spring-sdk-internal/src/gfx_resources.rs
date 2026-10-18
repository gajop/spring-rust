//! Single-call Core transport for mutating Gfx resource creation.
//!
//! Native texture/atlas names are bounded to 17 bytes. Callers provide that
//! storage before the native mutation, so resource creation never needs a
//! probe/retry invocation merely to discover the returned string length.

use crate::{ApiError, ErrorCode, Result};

#[cfg(feature = "alloc")]
use alloc::string::String;

pub const NATIVE_GFX_RESOURCE_NAME_MAX_BYTES: usize = 17;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureCreateParams {
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

impl Default for TextureCreateParams {
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

/// The engine-side texture parameter record has C layout and uses four-byte
/// boolean slots. Keeping this as a typed record lets the guest pass the
/// native layout directly; callers never need to construct an ABI byte blob.
#[cfg(target_arch = "wasm32")]
#[repr(C)]
#[derive(Clone, Copy)]
struct TextureCreateParamsWire {
    target: u32,
    format: u32,
    border: i32,
    min_filter: u32,
    mag_filter: u32,
    wrap_s: u32,
    wrap_t: u32,
    wrap_r: u32,
    compare_func: u32,
    lod_bias: f32,
    aniso: f32,
    samples: u32,
    fbo: u32,
    fbo_depth: u32,
}

#[cfg(target_arch = "wasm32")]
const _: () = assert!(core::mem::size_of::<TextureCreateParamsWire>() == 56);

#[cfg(target_arch = "wasm32")]
impl From<TextureCreateParams> for TextureCreateParamsWire {
    #[inline]
    fn from(params: TextureCreateParams) -> Self {
        Self {
            target: params.target,
            format: params.format,
            border: params.border,
            min_filter: params.min_filter,
            mag_filter: params.mag_filter,
            wrap_s: params.wrap_s,
            wrap_t: params.wrap_t,
            wrap_r: params.wrap_r,
            compare_func: params.compare_func,
            lod_bias: params.lod_bias,
            aniso: params.aniso,
            samples: params.samples,
            fbo: u32::from(params.fbo),
            fbo_depth: u32::from(params.fbo_depth),
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:gfx")]
    unsafe extern "C" {
        #[link_name = "create-texture"]
        pub safe fn create_texture(
            xsize: i32,
            ysize: i32,
            zsize: i32,
            params_ptr: i32,
            output_ptr: i32,
            output_capacity: i32,
        ) -> i64;

        #[link_name = "create-texture-atlas"]
        pub safe fn create_texture_atlas(
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
fn checked_output(output: &mut [u8]) -> Result<(i32, i32)> {
    if output.len() < NATIVE_GFX_RESOURCE_NAME_MAX_BYTES {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    crate::wasm_mut_slice_parts(output)
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn finish_name(packed: i64, output: &[u8]) -> Result<&str> {
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
pub fn create_texture_into(
    xsize: i32,
    ysize: i32,
    zsize: i32,
    params: TextureCreateParams,
    output: &mut [u8],
) -> Result<&str> {
    #[cfg(target_arch = "wasm32")]
    {
        let wire = TextureCreateParamsWire::from(params);
        let wire_pointer = crate::wasm_input_ptr(&wire)?;
        let (output_pointer, output_capacity) = checked_output(output)?;
        let packed = raw::create_texture(
            xsize,
            ysize,
            zsize,
            wire_pointer,
            output_pointer,
            output_capacity,
        );
        finish_name(packed, output)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (xsize, ysize, zsize, params, output);
        Err(unreachable!())
    }
}

#[inline]
pub fn create_texture_atlas_into(
    xsize: i32,
    ysize: i32,
    alloc_type: i32,
    output: &mut [u8],
) -> Result<&str> {
    #[cfg(target_arch = "wasm32")]
    {
        let (output_pointer, output_capacity) = checked_output(output)?;
        let packed =
            raw::create_texture_atlas(xsize, ysize, alloc_type, output_pointer, output_capacity);
        finish_name(packed, output)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (xsize, ysize, alloc_type, output);
        Err(unreachable!())
    }
}

/// Create a texture and return the native name as an owned string.
///
/// This is the convenient path for setup code. Render-loop code can use
/// [`create_texture_into`] with retained storage to avoid the allocation.
#[cfg(feature = "alloc")]
#[inline]
pub fn create_texture(
    xsize: i32,
    ysize: i32,
    zsize: i32,
    params: TextureCreateParams,
) -> Result<String> {
    let mut output = [0u8; NATIVE_GFX_RESOURCE_NAME_MAX_BYTES];
    create_texture_into(xsize, ysize, zsize, params, &mut output).map(String::from)
}

/// Create a texture atlas and return the native name as an owned string.
#[cfg(feature = "alloc")]
#[inline]
pub fn create_texture_atlas(xsize: i32, ysize: i32, alloc_type: i32) -> Result<String> {
    let mut output = [0u8; NATIVE_GFX_RESOURCE_NAME_MAX_BYTES];
    create_texture_atlas_into(xsize, ysize, alloc_type, &mut output).map(String::from)
}

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::slice;

use crate::{error::Error, raw::copy_c_string, sys};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleCommand {
    pub command: String,
    pub description: String,
    pub synced: bool,
    pub cheat: bool,
}

pub struct Gfx<'a> {
    api: &'a sys::GfxApi,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ShaderCreateParams<'a> {
    pub definitions: &'a str,
    pub vertex: &'a str,
    pub tcs: &'a str,
    pub tes: &'a str,
    pub geometry: &'a str,
    pub fragment: &'a str,
    pub compute: &'a str,
    pub geo_input_type: Option<u32>,
    pub geo_output_type: Option<u32>,
    pub geo_output_verts: Option<i32>,
    pub uniform_ints: &'a [ShaderUniformInt<'a>],
    pub uniform_floats: &'a [ShaderUniformFloat<'a>],
}

impl<'a> Gfx<'a> {
    pub(crate) fn new(api: &'a sys::GfxApi) -> Self {
        Self { api }
    }

    /// Compile, link, and initialize a shader's scalar uniforms.
    pub fn create_shader_configured(
        &self,
        params: ShaderCreateParams<'_>,
    ) -> Result<(u32, u32), Error> {
        let shader = self.create_shader(
            params.definitions,
            params.vertex,
            params.tcs,
            params.tes,
            params.geometry,
            params.fragment,
            params.compute,
            GfxCreateShaderOptions {
                has_geo_input_type: params.geo_input_type.is_some(),
                geo_input_type: params.geo_input_type.unwrap_or(0),
                has_geo_output_type: params.geo_output_type.is_some(),
                geo_output_type: params.geo_output_type.unwrap_or(0),
                has_geo_output_verts: params.geo_output_verts.is_some(),
                geo_output_verts: params.geo_output_verts.unwrap_or(0),
            },
        )?;

        if params.uniform_ints.is_empty() && params.uniform_floats.is_empty() {
            return Ok(shader);
        }

        let configured = (|| {
            let _ = self.use_shader(shader.0)?;
            for uniform in params.uniform_ints {
                let location = self.get_uniform_location(shader.0, uniform.name)?;
                self.uniform_int(location, [uniform.value, 0, 0, 0], 1)?;
            }
            for uniform in params.uniform_floats {
                let location = self.get_uniform_location(shader.0, uniform.name)?;
                self.uniform(location, [uniform.value, 0.0, 0.0, 0.0], 1)?;
            }
            Ok::<(), Error>(())
        })();
        let unbound = self.use_shader(0);
        if let Err(error) = configured {
            let _ = unbound;
            return Err(error);
        }
        let _ = unbound?;

        Ok(shader)
    }

    pub fn get_view_projection_matrix(&self) -> Result<[f32; 16], Error> {
        self.get_matrix_data(MATRIX_VIEWPROJECTION)
    }

    pub fn get_view_projection_matrix_inverse(&self) -> Result<[f32; 16], Error> {
        self.get_matrix_data(MATRIX_VIEWPROJECTION_INVERSE)
    }

    pub fn get_named_matrix(&self, mode: u32) -> Result<[f32; 16], Error> {
        self.get_matrix_data(mode)
    }

    pub fn uniform_named_matrix(&self, location: i32, mode: u32) -> Result<(), Error> {
        let matrix = self.get_named_matrix(mode)?;
        self.uniform_matrix(location, &matrix, false)
    }

    pub fn get_console_command_entries(&self) -> Result<Vec<ConsoleCommand>, Error> {
        self.get_console_commands().map(|commands| {
            commands
                .into_iter()
                .filter_map(ConsoleCommand::from_raw)
                .collect()
        })
    }
}

impl ConsoleCommand {
    fn from_raw(command: sys::GfxConsoleCommandEntry) -> Option<Self> {
        // SAFETY: command metadata is engine-owned and valid for this call.
        unsafe {
            Some(Self {
                command: copy_c_string(command.command)?,
                description: copy_c_string(command.description).unwrap_or_default(),
                synced: command.synced,
                cheat: command.cheat,
            })
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/gfx_generated.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    unsafe extern "C" fn mock_upload_texture(
        query: *const sys::GfxUploadTextureQuery,
        result: *mut sys::GfxEmptyResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!(
            unsafe { CStr::from_ptr(query.name) }.to_bytes(),
            b"!native7"
        );
        assert_eq!(query.target, 0);
        assert_eq!(query.level, 2);
        assert_eq!((query.xoff, query.yoff, query.zoff), (3, 4, 5));
        assert_eq!((query.width, query.height, query.depth), (6, 7, 8));
        assert_eq!(query.format, 0x1908);
        assert_eq!(query.pixelType, 0x1401);
        assert_eq!(query.dataSize, 4);
        assert_eq!(
            unsafe { std::slice::from_raw_parts(query.data, 4) },
            &[1, 2, 3, 4]
        );
        unsafe { (*result).error = std::ptr::null() };
    }

    #[test]
    fn upload_texture_passes_a_byte_slice_and_region() {
        let api = sys::GfxApi {
            UploadTexture: Some(mock_upload_texture),
            ..Default::default()
        };
        let gfx = Gfx::new(&api);

        gfx.upload_texture(
            "!native7",
            0,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            0x1908,
            0x1401,
            &[1, 2, 3, 4],
        )
        .expect("mock upload should succeed");
    }
}

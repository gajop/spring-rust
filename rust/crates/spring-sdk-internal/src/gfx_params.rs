/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

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

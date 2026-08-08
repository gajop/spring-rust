use super::*;
use serde_json::{Map, Value};
use spring_native::constants::*;
use std::ffi::CStr;

const PARITY_VERTEX_SHADER: &str = r#"#version 120
uniform float u_scalar;
uniform float u_floatArray[2];
uniform vec2 u_vector;
uniform int u_int;
uniform int u_intArray[2];
uniform mat4 u_matrix;
void main() {
    float offset = u_scalar + u_floatArray[0] + u_vector.x + float(u_int) +
        float(u_intArray[0]) + u_matrix[0][0];
    gl_Position = gl_Vertex + vec4(offset * 0.0001);
}
"#;

const PARITY_FRAGMENT_SHADER: &str = r#"#version 120
uniform float u_scalar;
uniform float u_floatArray[2];
uniform vec2 u_vector;
uniform int u_int;
uniform int u_intArray[2];
uniform mat4 u_matrix;
uniform vec4 u_color;
void main() {
    float offset = u_scalar + u_floatArray[1] + u_vector.y + float(u_int) +
        float(u_intArray[1]) + u_matrix[1][1];
    gl_FragColor = u_color + vec4(offset * 0.0001);
}
"#;

fn rounded(value: f32) -> Value {
    // Lua's fixture deliberately represents non-finite values, and the
    // engine's shadow parameters can use a very large finite sentinel for
    // the same unbounded value.  Normalize both forms before arithmetic can
    // overflow serde_json's number representation into `null`.
    if value == f32::INFINITY || value > 1.0e30 {
        return serde_json::json!("inf");
    }
    if value == f32::NEG_INFINITY || value < -1.0e30 {
        return serde_json::json!("-inf");
    }
    if value.is_nan() {
        return serde_json::json!("nan");
    }
    let value = (value * 100_000.0).round() / 100_000.0;
    let value = ((value as f64) * 100_000_000.0).round() / 100_000_000.0;
    if value.fract() == 0.0 {
        serde_json::json!(value as i64)
    } else {
        serde_json::json!(value)
    }
}

fn values(values: impl IntoIterator<Item = f32>) -> Vec<Value> {
    values.into_iter().map(rounded).collect()
}

fn record(result: &mut Map<String, Value>, name: &str, values: Vec<Value>) {
    result.insert(
        name.to_owned(),
        serde_json::json!({
            "n": values.len(),
            "values": values,
        }),
    );
}

fn record_void(result: &mut Map<String, Value>, name: &str) {
    record(result, name, Vec::new());
}

fn json_values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let Some(left) = left.as_f64() else {
                return false;
            };
            let Some(right) = right.as_f64() else {
                return false;
            };
            (left - right).abs() <= 1.0e-5
                || (left - right).abs() <= 1.0e-6 * left.abs().max(right.abs())
        }
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| json_values_equal(left, right))
        }
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left.iter().all(|(key, left)| {
                    right
                        .get(key)
                        .is_some_and(|right| json_values_equal(left, right))
                })
        }
        _ => left == right,
    }
}

fn compare_result(message: &Value, actual: Map<String, Value>, label: &str) -> Result<(), String> {
    let expected = message
        .get("result")
        .ok_or_else(|| format!("{label} payload is missing `result`"))?;
    let actual = Value::Object(actual);
    if !json_values_equal(expected, &actual) {
        return Err(format!(
            "{label} result mismatch: expected={expected}, actual={actual}"
        ));
    }
    Ok(())
}

fn atlas_entries_value(entries: &[spring_native::sys::GfxAtlasTextureEntry]) -> Value {
    let mut normalized = entries
        .iter()
        .map(|entry| {
            let name = if entry.name.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(entry.name).to_string_lossy().into_owned() }
            };
            (name, [entry.x1, entry.x2, entry.y1, entry.y2])
        })
        .collect::<Vec<_>>();
    normalized.sort_by(|left, right| left.0.cmp(&right.0));
    Value::Array(
        normalized
            .into_iter()
            .map(|(name, coords)| {
                serde_json::json!([
                    name,
                    rounded(coords[0]),
                    rounded(coords[1]),
                    rounded(coords[2]),
                    rounded(coords[3]),
                ])
            })
            .collect(),
    )
}

fn value_result(
    floats: [f32; 4],
    count: u32,
    bool_value: bool,
    has_bool: bool,
    string_value: Option<String>,
) -> Vec<Value> {
    let mut result = values(floats.into_iter().take(count as usize));
    if has_bool {
        result.push(serde_json::json!(bool_value));
    }
    if let Some(value) = string_value {
        result.push(serde_json::json!(value));
    }
    result
}

impl NativeApiParity {
    pub(crate) fn check_gl_state_queries(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        record(
            &mut actual,
            "gl.HasExtension",
            vec![serde_json::json!(gfx
                .has_extension("GL_NATIVE_API_PARITY_NOT_AN_EXTENSION")
                .map_err(|error| format!(
                    "HasExtension failed: {error:?}"
                ))?)],
        );

        let (viewport, viewport_count) = gfx
            .get_number(0x0BA2, 4)
            .map_err(|error| format!("GetNumber(VIEWPORT) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetNumber",
            values(viewport.into_iter().take(viewport_count as usize)),
        );

        let version = gfx
            .get_string(0x1F02)
            .map_err(|error| format!("GetString(VERSION) failed: {error:?}"))?
            .unwrap_or_else(|| "[NULL]".to_owned());
        record(
            &mut actual,
            "gl.GetString",
            vec![serde_json::json!(version)],
        );

        let (view_size_x, view_size_y) = gfx
            .get_view_sizes()
            .map_err(|error| format!("GetViewSizes failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetViewSizes",
            vec![
                serde_json::json!(view_size_x),
                serde_json::json!(view_size_y),
            ],
        );

        let (near_plane, far_plane, min_range, max_range) = gfx
            .get_view_range(5)
            .map_err(|error| format!("GetViewRange failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetViewRange",
            values([near_plane, far_plane, min_range, max_range]),
        );

        let shadow = gfx
            .get_shadow_map_params()
            .map_err(|error| format!("GetShadowMapParams failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetShadowMapParams",
            values([shadow.x, shadow.y, shadow.z, shadow.w]),
        );

        let atmosphere = gfx
            .get_atmosphere("", "")
            .map_err(|error| format!("GetAtmosphere() failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetAtmosphere",
            value_result(
                atmosphere.0,
                atmosphere.1,
                atmosphere.2,
                atmosphere.3,
                atmosphere.4,
            ),
        );

        let sun = gfx
            .get_sun("", "")
            .map_err(|error| format!("GetSun() failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetSun",
            value_result(sun.0, sun.1, sun.2, sun.3, sun.4),
        );

        let water = gfx
            .get_water_rendering("absorb", "")
            .map_err(|error| format!("GetWaterRendering(absorb) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetWaterRendering",
            value_result(water.0, water.1, water.2, water.3, water.4),
        );

        let map = gfx
            .get_map_rendering("voidWater", "")
            .map_err(|error| format!("GetMapRendering(voidWater) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetMapRendering",
            value_result(map.0, map.1, map.2, map.3, map.4),
        );

        let (screen_x, screen_y, screen_z) = gfx
            .get_screen_view_trans()
            .map_err(|error| format!("GetScreenViewTrans failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetScreenViewTrans",
            values([screen_x, screen_y, screen_z]),
        );

        compare_result(message, actual, "gl.state_queries")
    }

    pub(crate) fn check_gl_state_mutations(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }
        void!("gl.ResetState", gfx.reset_state());
        void!("gl.ResetMatrices", gfx.reset_matrices());
        void!("gl.MatrixMode", gfx.matrix_mode(GL_PROJECTION));
        void!("gl.LoadIdentity", gfx.load_identity());
        void!("gl.Translate", gfx.translate(1.0, 2.0, 3.0));
        void!("gl.Scale", gfx.scale(2.0, 3.0, 4.0));
        void!("gl.Rotate", gfx.rotate(15.0, 0.0, 1.0, 0.0));
        void!("gl.PushMatrix", gfx.push_matrix());
        void!("gl.PopMatrix", gfx.pop_matrix());
        void!("gl.Ortho", gfx.ortho(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0));
        void!("gl.Frustum", gfx.frustum(-0.1, 0.1, -0.1, 0.1, 0.1, 100.0));
        void!(
            "gl.LoadMatrix",
            gfx.load_matrix([
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ])
        );
        void!(
            "gl.MultMatrix",
            gfx.mult_matrix([
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 1.0,
            ])
        );
        let matrix = gfx
            .get_matrix_data(GL_PROJECTION)
            .map_err(|error| format!("GetMatrixData failed: {error:?}"))?;
        record(&mut actual, "gl.GetMatrixData", values(matrix));
        void!("gl.MatrixMode.restore", gfx.matrix_mode(GL_MODELVIEW));
        void!("gl.ResetMatrices.restore", gfx.reset_matrices());

        void!(
            "gl.DepthTest",
            gfx.depth_test(spring_native::GfxDepthTestOptions {
                enable: true,
                set_func: true,
                func: GL_LEQUAL,
            })
        );
        void!("gl.DepthMask", gfx.depth_mask(false));
        void!("gl.Culling", gfx.culling(true));
        void!("gl.Blending", gfx.blending(true));
        void!(
            "gl.BlendFunc",
            gfx.blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
        );
        void!("gl.BlendEquation", gfx.blend_equation(GL_FUNC_ADD));
        void!(
            "gl.ColorMask",
            gfx.color_mask(spring_native::GfxColorMaskOptions {
                red: true,
                green: false,
                blue: true,
                alpha: false,
            })
        );
        void!("gl.AlphaToCoverage", gfx.alpha_to_coverage(false));
        void!("gl.StencilTest", gfx.stencil_test(false));
        void!("gl.Scissor", gfx.scissor(0, 0, 64, 64));
        void!("gl.Viewport", gfx.viewport(0, 0, 64, 64));
        void!("gl.LineWidth", gfx.line_width(2.0));
        void!("gl.PointSize", gfx.point_size(3.0));
        void!("gl.Fog", gfx.fog(false));
        void!("gl.Lighting", gfx.lighting(false));
        void!("gl.ResetState.restore", gfx.reset_state());

        compare_result(message, actual, "gl.state_mutations")
    }

    pub(crate) fn check_gl_fixed_immediate(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }
        macro_rules! side_effect {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
            }};
        }

        // Keep this sequence in lockstep with runGlFixedImmediateSurfaceApiTest
        // in the Lua fixture.  The native descriptors use explicit options for
        // Lua's overloaded calls, but the resulting GL state must be identical.
        void!("gl.ResetState", gfx.reset_state());
        void!(
            "gl.DepthTest",
            gfx.depth_test(spring_native::GfxDepthTestOptions {
                enable: true,
                set_func: true,
                func: GL_GREATER,
            })
        );
        void!("gl.Culling", gfx.culling(true));
        side_effect!(
            "gl.BlendFunc",
            gfx.blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
        );
        void!("gl.Blending", gfx.blending(true));
        void!(
            "gl.BlendFuncSeparate",
            gfx.blend_func_separate(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_ONE, GL_ZERO)
        );
        void!(
            "gl.BlendEquationSeparate",
            gfx.blend_equation_separate(GL_FUNC_ADD, GL_FUNC_REVERSE_SUBTRACT)
        );
        void!(
            "gl.ColorMask",
            gfx.color_mask(spring_native::GfxColorMaskOptions {
                red: true,
                green: false,
                blue: true,
                alpha: false,
            })
        );
        void!("gl.AlphaTest", gfx.alpha_test(true, GL_LESS, 0.25));
        void!("gl.AlphaToCoverage", gfx.alpha_to_coverage(false));
        void!("gl.DepthClamp", gfx.depth_clamp(true));
        void!("gl.LogicOp", gfx.logic_op(true, GL_XOR));
        void!("gl.ShadeModel", gfx.shade_model(GL_FLAT));
        void!("gl.Scissor", gfx.scissor(4, 5, 64, 32));
        void!("gl.LineStipple", gfx.line_stipple(2, 0xAAAA));
        void!("gl.PointSprite", gfx.point_sprite(true));
        void!(
            "gl.PolygonMode",
            gfx.polygon_mode(GL_FRONT_AND_BACK, GL_FILL)
        );
        void!("gl.PolygonOffset", gfx.polygon_offset(1.25, 2.5));
        void!("gl.StencilTest", gfx.stencil_test(true));
        void!("gl.StencilFunc", gfx.stencil_func(GL_LEQUAL, 1, 0xFF));
        void!(
            "gl.StencilFuncSeparate",
            gfx.stencil_func_separate(GL_FRONT_AND_BACK, GL_GEQUAL, 2, 0x7F)
        );
        void!("gl.StencilMask", gfx.stencil_mask(0xF0));
        void!(
            "gl.StencilMaskSeparate",
            gfx.stencil_mask_separate(GL_FRONT_AND_BACK, 0x0F)
        );
        void!(
            "gl.StencilOp",
            gfx.stencil_op(0x1E00, GL_REPLACE, GL_INVERT)
        );
        void!(
            "gl.StencilOpSeparate",
            gfx.stencil_op_separate(GL_FRONT_AND_BACK, 0x1E00, GL_ZERO, GL_REPLACE)
        );
        void!("gl.ClipDistance", gfx.clip_distance(0, true));
        void!("gl.ClipPlane", gfx.clip_plane(1, [1.0, 0.0, 0.0, 0.0]));

        // Lua PointParameter expands one six-argument call into four GL
        // operations.  Exercise the same expansion through the native API.
        void!(
            "gl.PointParameter",
            gfx.point_parameter(0x8129, 0.0, [0.5, 0.25, 0.125, 0.0], 3,)
        );
        side_effect!(
            "gl.PointParameter.sizeMin",
            gfx.point_parameter(0x8126, 1.0, [0.0; 4], 1)
        );
        side_effect!(
            "gl.PointParameter.sizeMax",
            gfx.point_parameter(0x8127, 64.0, [0.0; 4], 1)
        );
        side_effect!(
            "gl.PointParameter.sizeFade",
            gfx.point_parameter(0x8128, 8.0, [0.0; 4], 1)
        );
        void!(
            "gl.Light",
            gfx.light(
                0,
                spring_native::GfxLightOptions::default(),
                GL_DIFFUSE,
                [1.0, 0.5, 0.25, 1.0],
                4,
            )
        );
        void!(
            "gl.Material",
            gfx.material(GL_AMBIENT, [0.1, 0.2, 0.3, 1.0], 4)
        );
        void!(
            "gl.TexEnv",
            gfx.tex_env(
                GL_TEXTURE_ENV,
                GL_TEXTURE_ENV_COLOR,
                [0.1, 0.2, 0.3, 0.4],
                4
            )
        );
        void!(
            "gl.MultiTexEnv",
            gfx.multi_tex_env(
                1,
                GL_TEXTURE_ENV,
                GL_TEXTURE_ENV_MODE,
                [GL_MODULATE as f32, 0.0, 0.0, 0.0],
                1
            )
        );
        void!(
            "gl.TexGen",
            gfx.tex_gen(
                GL_S,
                spring_native::GfxTexGenOptions::default(),
                GL_TEXTURE_GEN_MODE,
                [GL_OBJECT_LINEAR as f32, 0.0, 0.0, 0.0],
                1,
            )
        );
        void!(
            "gl.MultiTexGen",
            gfx.multi_tex_gen(
                1,
                GL_T,
                spring_native::GfxMultiTexGenOptions::default(),
                GL_TEXTURE_GEN_MODE,
                [GL_EYE_LINEAR as f32, 0.0, 0.0, 0.0],
                1,
            )
        );
        void!("gl.PushAttrib", gfx.push_attrib(GL_ENABLE_BIT));
        void!("gl.PopAttrib", gfx.pop_attrib());
        void!("gl.MemoryBarrier", gfx.memory_barrier(0));
        void!("gl.DispatchCompute", gfx.dispatch_compute(1, 1, 1, 0));
        void!("gl.ActiveTexture", gfx.active_texture(1));
        side_effect!(
            "gl.ActiveTexture.TexEnv",
            gfx.tex_env(
                GL_TEXTURE_ENV,
                GL_TEXTURE_ENV_MODE,
                [GL_MODULATE as f32, 0.0, 0.0, 0.0],
                1
            )
        );
        side_effect!("gl.ActiveTexture.restore", gfx.active_texture(0));
        void!(
            "gl.ObjectLabel",
            gfx.object_label(GL_TEXTURE, 0, "native-api-parity")
        );
        void!(
            "gl.PushDebugGroup",
            gfx.push_debug_group(1, "native-api-parity", false)
        );
        void!("gl.PopDebugGroup", gfx.pop_debug_group());

        let fixed = |name: &str| {
            gfx.get_fixed_state(name)
                .map_err(|error| format!("GetFixedState({name}) failed: {error:?}"))
        };
        let (bools, bool_count, ints, int_count, floats, float_count) = fixed("blending")?;
        record(
            &mut actual,
            "gl.GetFixedState.blending",
            std::iter::once(serde_json::json!(bools[0]))
                .chain(
                    ints.into_iter()
                        .take(int_count as usize)
                        .map(|v| serde_json::json!(v)),
                )
                .collect(),
        );
        if bool_count != 1 || float_count != 0 {
            return Err("GetFixedState(blending) returned unexpected slots".to_owned());
        }

        let (bools, bool_count, ints, int_count, floats, float_count) = fixed("depth")?;
        record(
            &mut actual,
            "gl.GetFixedState.depth",
            vec![
                serde_json::json!(bools[0]),
                serde_json::json!(bools[1]),
                serde_json::json!(ints[0]),
            ],
        );
        if bool_count != 2 || int_count != 1 || float_count != 0 {
            return Err("GetFixedState(depth) returned unexpected slots".to_owned());
        }

        let (bools, bool_count, ints, int_count, floats, float_count) = fixed("culling")?;
        record(
            &mut actual,
            "gl.GetFixedState.culling",
            vec![serde_json::json!(bools[0]), serde_json::json!(ints[0])],
        );
        if bool_count != 1 || int_count != 1 || float_count != 0 {
            return Err("GetFixedState(culling) returned unexpected slots".to_owned());
        }

        let (bools, bool_count, ints, int_count, floats, float_count) = fixed("colorMask")?;
        record(
            &mut actual,
            "gl.GetFixedState.colorMask",
            bools
                .into_iter()
                .take(bool_count as usize)
                .map(|v| serde_json::json!(v))
                .collect(),
        );
        if bool_count != 4 || int_count != 0 || float_count != 0 {
            return Err("GetFixedState(colorMask) returned unexpected slots".to_owned());
        }

        let (bools, bool_count, ints, int_count, floats, float_count) = fixed("alphaTest")?;
        record(
            &mut actual,
            "gl.GetFixedState.alphaTest",
            vec![
                serde_json::json!(bools[0]),
                serde_json::json!(ints[0]),
                rounded(floats[0]),
            ],
        );
        if bool_count != 1 || int_count != 1 || float_count != 1 {
            return Err("GetFixedState(alphaTest) returned unexpected slots".to_owned());
        }

        let (bools, bool_count, ints, int_count, floats, float_count) = fixed("lineWidth")?;
        record(
            &mut actual,
            "gl.GetFixedState.lineWidth",
            vec![rounded(floats[0])],
        );
        if bool_count != 0 || int_count != 0 || float_count != 1 {
            return Err("GetFixedState(lineWidth) returned unexpected slots".to_owned());
        }

        let (bools, bool_count, ints, int_count, floats, float_count) = fixed("pointSize")?;
        record(
            &mut actual,
            "gl.GetFixedState.pointSize",
            vec![serde_json::json!(bools[0]), rounded(floats[0])],
        );
        if bool_count != 1 || int_count != 0 || float_count != 1 {
            return Err("GetFixedState(pointSize) returned unexpected slots".to_owned());
        }

        let _ = (ints, floats); // keep the closure destructuring explicit above
        void!("gl.ResetState.restore", gfx.reset_state());
        compare_result(message, actual, "gl.fixed_immediate")
    }

    pub(crate) fn check_gl_immediate_primitives(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }

        void!(
            "gl.Clear",
            gfx.clear(GL_COLOR_BUFFER_BIT, [0.1, 0.2, 0.3, 0.4], 4)
        );
        void!(
            "gl.BeginEnd",
            gfx.begin_end(GL_TRIANGLES, || {
                let _ = gfx.color(0.2, 0.3, 0.4, 0.5);
                let _ = gfx.secondary_color(0.6, 0.7, 0.8);
                let _ = gfx.normal(0.0, 1.0, 0.0);
                let _ = gfx.tex_coord(0.1, 0.2, 0.3, 0.4, 4);
                let _ = gfx.multi_tex_coord(1, 0.2, 0.3, 0.4, 0.5, 4);
                let _ = gfx.fog_coord(0.6);
                let _ = gfx.edge_flag(false);
                let _ = gfx.vertex(0.0, 0.0, 0.0, 1.0, 4);
                let _ = gfx.vertex(1.0, 0.0, 0.0, 1.0, 4);
                let _ = gfx.vertex(0.0, 1.0, 0.0, 1.0, 4);
            })
        );

        let vertices = [
            spring_native::sys::GfxVertexData {
                vertex: [0.0, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                texCoord: [0.0, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
                hasVertex: true,
                hasNormal: true,
                hasTexCoord: true,
                hasColor: true,
            },
            spring_native::sys::GfxVertexData {
                vertex: [1.0, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                texCoord: [1.0, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                hasVertex: true,
                hasNormal: true,
                hasTexCoord: true,
                hasColor: true,
            },
            spring_native::sys::GfxVertexData {
                vertex: [0.0, 1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                texCoord: [0.0, 1.0],
                color: [0.0, 0.0, 1.0, 1.0],
                hasVertex: true,
                hasNormal: true,
                hasTexCoord: true,
                hasColor: true,
            },
        ];
        void!("gl.Shape", gfx.shape(GL_TRIANGLES, &vertices));
        void!("gl.Rect", gfx.rect(-1.0, -1.0, 1.0, 1.0));
        void!(
            "gl.TexRect",
            gfx.tex_rect(-1.0, -1.0, 1.0, 1.0, 0.1, 0.2, 0.9, 0.8)
        );
        void!("gl.Billboard", gfx.billboard());
        void!(
            "gl.PushPopMatrix",
            gfx.push_pop_matrix(|| {
                let _ = gfx.translate(1.0, 2.0, 3.0);
            })
        );
        void!("gl.UnsafeState", gfx.unsafe_state(GL_BLEND, true, || {}));
        void!("gl.Flush", gfx.flush());
        void!("gl.Finish", gfx.finish());

        gfx.color(0.11, 0.22, 0.33, 0.44)
            .map_err(|error| format!("current color setup failed: {error:?}"))?;
        gfx.normal(0.55, 0.66, 0.77)
            .map_err(|error| format!("current normal setup failed: {error:?}"))?;
        gfx.tex_coord(0.12, 0.23, 0.34, 0.45, 4)
            .map_err(|error| format!("current texcoord setup failed: {error:?}"))?;
        gfx.secondary_color(0.56, 0.67, 0.78)
            .map_err(|error| format!("current secondary color setup failed: {error:?}"))?;
        gfx.fog_coord(0.89)
            .map_err(|error| format!("current fog coordinate setup failed: {error:?}"))?;
        gfx.edge_flag(false)
            .map_err(|error| format!("current edge flag setup failed: {error:?}"))?;

        let read_number = |pname, count| {
            gfx.get_number(pname, count)
                .map_err(|error| format!("GetNumber({pname:#x}) failed: {error:?}"))
        };
        let (raw_values, count) = read_number(0x0B00, 4)?;
        record(
            &mut actual,
            "gl.GetNumber.currentColor",
            values(raw_values.into_iter().take(count as usize)),
        );
        let (raw_values, count) = read_number(0x0B02, 3)?;
        record(
            &mut actual,
            "gl.GetNumber.currentNormal",
            values(raw_values.into_iter().take(count as usize)),
        );
        let (raw_values, count) = read_number(0x0B03, 4)?;
        record(
            &mut actual,
            "gl.GetNumber.currentTexCoord",
            values(raw_values.into_iter().take(count as usize)),
        );
        let (raw_values, count) = read_number(0x8459, 4)?;
        record(
            &mut actual,
            "gl.GetNumber.currentSecondaryColor",
            values(raw_values.into_iter().take(count as usize)),
        );
        let (raw_values, count) = read_number(0x8453, 1)?;
        record(
            &mut actual,
            "gl.GetNumber.currentFogCoord",
            values(raw_values.into_iter().take(count as usize)),
        );
        let (raw_values, count) = read_number(0x0B43, 1)?;
        record(
            &mut actual,
            "gl.GetNumber.edgeFlag",
            values(raw_values.into_iter().take(count as usize)),
        );

        void!("gl.ResetState.restore", gfx.reset_state());
        compare_result(message, actual, "gl.immediate_primitives")
    }

    pub(crate) fn check_gl_shader_uniforms(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }
        macro_rules! side_effect {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
            }};
        }

        let (shader_id, raw_shader_id) = gfx
            .create_shader(
                "",
                PARITY_VERTEX_SHADER,
                "",
                "",
                "",
                PARITY_FRAGMENT_SHADER,
                "",
                spring_native::GfxCreateShaderOptions::default(),
            )
            .map_err(|error| format!("CreateShader failed: {error:?}"))?;
        if shader_id == 0 || raw_shader_id == 0 {
            return Err(format!(
                "CreateShader returned invalid handles: shader={shader_id}, program={raw_shader_id}"
            ));
        }
        record(
            &mut actual,
            "gl.CreateShader",
            vec![serde_json::json!(true), serde_json::json!(true)],
        );

        let shader_log = gfx
            .get_shader_log()
            .map_err(|error| format!("GetShaderLog failed: {error:?}"))?
            .unwrap_or_default();
        record(
            &mut actual,
            "gl.GetShaderLog",
            vec![serde_json::json!(shader_log)],
        );

        record(
            &mut actual,
            "gl.UseShader",
            vec![serde_json::json!(gfx
                .use_shader(shader_id)
                .map_err(|error| format!("UseShader failed: {error:?}"))?)],
        );
        let (current_program, current_program_count) = gfx
            .get_number(0x8B8D, 1)
            .map_err(|error| format!("GetNumber(CURRENT_PROGRAM) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetNumber.currentProgram",
            vec![serde_json::json!(
                current_program_count > 0 && current_program[0] > 0.0
            )],
        );

        let uniform_names = [
            "u_scalar",
            "u_floatArray",
            "u_vector",
            "u_int",
            "u_intArray",
            "u_matrix",
            "u_color",
        ];
        let mut locations = std::collections::BTreeMap::new();
        for name in uniform_names {
            let location = gfx
                .get_uniform_location(shader_id, name)
                .map_err(|error| format!("GetUniformLocation({name}) failed: {error:?}"))?;
            locations.insert(name, location);
            record(
                &mut actual,
                &format!("gl.GetUniformLocation.{name}"),
                vec![serde_json::json!(location)],
            );
        }

        let mut active_uniforms = gfx
            .get_active_uniforms(shader_id)
            .map_err(|error| format!("GetActiveUniforms failed: {error:?}"))?
            .into_iter()
            .map(|uniform| {
                let name = unsafe { CStr::from_ptr(uniform.name).to_string_lossy().into_owned() };
                let type_name =
                    unsafe { CStr::from_ptr(uniform.type_).to_string_lossy().into_owned() };
                serde_json::json!([
                    name,
                    type_name,
                    uniform.length,
                    uniform.size,
                    uniform.location
                ])
            })
            .collect::<Vec<_>>();
        active_uniforms.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
        record(
            &mut actual,
            "gl.GetActiveUniforms",
            vec![Value::Array(active_uniforms)],
        );

        void!(
            "gl.Uniform",
            gfx.uniform(locations["u_scalar"], [1.25, 0.0, 0.0, 0.0], 1)
        );
        side_effect!(
            "gl.Uniform.vector",
            gfx.uniform(locations["u_vector"], [2.0, 3.0, 0.0, 0.0], 2)
        );
        side_effect!(
            "gl.Uniform.color",
            gfx.uniform(locations["u_color"], [0.1, 0.2, 0.3, 0.4], 4)
        );
        void!(
            "gl.UniformInt",
            gfx.uniform_int(locations["u_int"], [7, 0, 0, 0], 1)
        );
        void!(
            "gl.UniformArray",
            gfx.uniform_array_float(locations["u_floatArray"], &[1.5, 2.5])
        );
        side_effect!(
            "gl.UniformArray.int",
            gfx.uniform_array_int(locations["u_intArray"], &[3, 4])
        );
        void!(
            "gl.UniformMatrix",
            gfx.uniform_matrix(
                locations["u_matrix"],
                &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,],
                false,
            )
        );
        void!(
            "gl.ActiveShader",
            gfx.active_shader(shader_id, || {
                let _ = gfx.uniform(locations["u_scalar"], [1.5, 0.0, 0.0, 0.0], 1);
            })
        );
        void!(
            "gl.SetGeometryShaderParameter",
            gfx.set_geometry_shader_parameter(shader_id, 0x8DDC, GL_TRIANGLES as i32)
        );
        void!(
            "gl.SetTesselationShaderParameter",
            gfx.set_tesselation_shader_parameter(0x8E72, 3, [0.0; 4], 0, false)
        );

        match gfx
            .get_engine_uniform_buffer_def(0)
            .map_err(|error| format!("GetEngineUniformBufferDef failed: {error:?}"))?
        {
            Some(value) => record(
                &mut actual,
                "gl.GetEngineUniformBufferDef",
                vec![serde_json::json!(value)],
            ),
            None => {}
        }
        let model_definition = gfx
            .get_engine_model_uniform_data_def()
            .map_err(|error| format!("GetEngineModelUniformDataDef failed: {error:?}"))?;
        if let Some(value) = model_definition {
            record(
                &mut actual,
                "gl.GetEngineModelUniformDataDef",
                vec![serde_json::json!(value)],
            );
            let (elements, bytes) = gfx
                .get_engine_model_uniform_data_size()
                .map_err(|error| format!("GetEngineModelUniformDataSize failed: {error:?}"))?;
            record(
                &mut actual,
                "gl.GetEngineModelUniformDataSize",
                vec![serde_json::json!(elements), serde_json::json!(bytes)],
            );
        }

        let restored = gfx
            .use_shader(0)
            .map_err(|error| format!("UseShader(0) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.UseShader.restore",
            vec![serde_json::json!(restored)],
        );
        let deleted = gfx
            .delete_shader(shader_id)
            .map_err(|error| format!("DeleteShader failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.DeleteShader",
            vec![serde_json::json!(deleted)],
        );
        let invalid = gfx
            .use_shader(shader_id)
            .map_err(|error| format!("UseShader(deleted) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.UseShader.invalid",
            vec![serde_json::json!(invalid)],
        );

        compare_result(message, actual, "gl.shader_uniforms")
    }

    pub(crate) fn check_gl_texture_resources(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }

        let texture_params = spring_native::sys::GfxTextureParams {
            target: GL_TEXTURE_2D,
            format: GL_RGBA8,
            minFilter: GL_LINEAR,
            magFilter: GL_LINEAR,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            wrapR: GL_CLAMP_TO_EDGE,
            ..spring_native::sys::GfxTextureParams::default()
        };
        let texture_name = gfx
            .create_texture(4, 4, 0, texture_params)
            .map_err(|error| format!("CreateTexture failed: {error:?}"))?
            .ok_or_else(|| "CreateTexture returned no texture name".to_owned())?;
        record(
            &mut actual,
            "gl.CreateTexture",
            vec![serde_json::json!(true)],
        );

        let (xsize, ysize, zsize, texture_id, target, _fbo) = gfx
            .texture_info(&texture_name)
            .map_err(|error| format!("TextureInfo failed: {error:?}"))?;
        actual.insert(
            "gl.TextureInfo".to_owned(),
            serde_json::json!({
                "n": 1,
                "values": [{
                    "xsize": xsize,
                    "ysize": ysize,
                    "zsize": zsize,
                    "target": target,
                }],
            }),
        );
        record(
            &mut actual,
            "gl.TextureInfo.idValid",
            vec![serde_json::json!(texture_id > 0)],
        );

        record(
            &mut actual,
            "gl.Texture",
            vec![serde_json::json!(gfx
                .bind_texture(&texture_name, -1, true)
                .map_err(|error| format!(
                    "Texture bind failed: {error:?}"
                ))?)],
        );
        let changed_params = spring_native::sys::GfxTextureParams {
            minFilter: GL_NEAREST,
            magFilter: GL_NEAREST,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            wrapR: GL_CLAMP_TO_EDGE,
            ..spring_native::sys::GfxTextureParams::default()
        };
        void!(
            "gl.ChangeTextureParams",
            gfx.change_texture_params(&texture_name, changed_params)
        );
        let (binding, binding_count) = gfx
            .get_number(0x8069, 1)
            .map_err(|error| format!("GetNumber(TEXTURE_BINDING_2D) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetNumber.textureBinding",
            vec![serde_json::json!(binding_count > 0 && binding[0] > 0.0)],
        );
        record(
            &mut actual,
            "gl.Texture.disable",
            vec![serde_json::json!(gfx
                .bind_texture(&texture_name, -1, false)
                .map_err(|error| format!(
                    "Texture disable failed: {error:?}"
                ))?)],
        );
        void!(
            "gl.CopyToTexture",
            gfx.copy_to_texture(&texture_name, 0, 0, 0, 0, 1, 1, GL_TEXTURE_2D, 0)
        );
        void!("gl.GenerateMipmap", gfx.generate_mipmap(&texture_name));
        void!(
            "gl.BindImageTexture",
            gfx.bind_image_texture(0, &texture_name, 0, 0, false, GL_READ_WRITE, GL_RGBA8)
        );

        void!(
            "gl.Clear",
            gfx.clear(GL_COLOR_BUFFER_BIT, [0.21, 0.31, 0.41, 0.51], 4)
        );
        let (pixels, _components) = gfx
            .read_pixels(0, 0, 1, 1, GL_RGBA)
            .map_err(|error| format!("ReadPixels failed: {error:?}"))?;
        record(&mut actual, "gl.ReadPixels", values(pixels));
        record(
            &mut actual,
            "gl.SaveImage",
            vec![serde_json::json!(gfx
                .save_image(
                    0,
                    0,
                    1,
                    1,
                    "native_api_parity_texture.png",
                    spring_native::GfxSaveImageOptions {
                        alpha: false,
                        yflip: false,
                        grayscale16bit: false,
                    },
                    0,
                )
                .map_err(|error| format!("SaveImage failed: {error:?}"))?)],
        );

        let fbo_params = spring_native::sys::GfxTextureParams {
            target: GL_TEXTURE_2D,
            format: GL_RGBA8,
            minFilter: GL_LINEAR,
            magFilter: GL_LINEAR,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            fbo: true,
            fboDepth: true,
            ..spring_native::sys::GfxTextureParams::default()
        };
        let fbo_texture_name = gfx
            .create_texture(4, 4, 0, fbo_params)
            .map_err(|error| format!("CreateTexture(fbo) failed: {error:?}"))?
            .ok_or_else(|| "CreateTexture(fbo) returned no texture name".to_owned())?;
        record(
            &mut actual,
            "gl.CreateTexture.fbo",
            vec![serde_json::json!(true)],
        );
        let mut callback_error = None;
        gfx.render_to_texture(&fbo_texture_name, || {
            if let Err(error) = gfx.clear(GL_COLOR_BUFFER_BIT, [0.21, 0.31, 0.41, 0.51], 4) {
                callback_error = Some(format!("{error:?}"));
            }
        })
        .map_err(|error| format!("RenderToTexture failed: {error:?}"))?;
        if let Some(error) = callback_error {
            return Err(format!("RenderToTexture callback failed: {error}"));
        }
        record_void(&mut actual, "gl.RenderToTexture");
        record(
            &mut actual,
            "gl.DeleteTextureFBO",
            vec![serde_json::json!(gfx
                .delete_texture_fbo(&fbo_texture_name)
                .map_err(|error| format!(
                    "DeleteTextureFBO failed: {error:?}"
                ))?)],
        );
        record(
            &mut actual,
            "gl.DeleteTexture.fbo",
            vec![serde_json::json!(gfx
                .delete_texture(&fbo_texture_name)
                .map_err(|error| format!(
                    "DeleteTexture(fbo) failed: {error:?}"
                ))?)],
        );
        record(
            &mut actual,
            "gl.DeleteTexture",
            vec![serde_json::json!(gfx
                .delete_texture(&texture_name)
                .map_err(|error| format!(
                    "DeleteTexture failed: {error:?}"
                ))?)],
        );

        compare_result(message, actual, "gl.texture_resources")
    }

    pub(crate) fn check_gl_lists_queries(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }

        void!("gl.ResetState", gfx.reset_state());
        let mut list_callback_error = None;
        let list_id = gfx
            .create_list(|| {
                if let Err(error) = gfx.begin_end(GL_TRIANGLES, || {
                    let _ = gfx.color(0.17, 0.27, 0.37, 0.47);
                    let _ = gfx.vertex(0.0, 0.0, 0.0, 1.0, 4);
                    let _ = gfx.vertex(1.0, 0.0, 0.0, 1.0, 4);
                    let _ = gfx.vertex(0.0, 1.0, 0.0, 1.0, 4);
                }) {
                    list_callback_error = Some(format!("{error:?}"));
                }
            })
            .map_err(|error| format!("CreateList failed: {error:?}"))?;
        if let Some(error) = list_callback_error {
            return Err(format!("CreateList callback failed: {error}"));
        }
        record(
            &mut actual,
            "gl.CreateList",
            vec![serde_json::json!(list_id > 0)],
        );
        void!("gl.CallList", gfx.call_list(list_id));
        void!("gl.DeleteList", gfx.delete_list(list_id));

        let query_id = gfx
            .create_query()
            .map_err(|error| format!("CreateQuery failed: {error:?}"))?;
        let mut query_callback_error = None;
        gfx.run_query(query_id, || {
            if let Err(error) = gfx.begin_end(GL_TRIANGLES, || {
                let _ = gfx.vertex(0.0, 0.0, 0.0, 1.0, 4);
                let _ = gfx.vertex(1.0, 0.0, 0.0, 1.0, 4);
                let _ = gfx.vertex(0.0, 1.0, 0.0, 1.0, 4);
            }) {
                query_callback_error = Some(format!("{error:?}"));
            }
        })
        .map_err(|error| format!("RunQuery failed: {error:?}"))?;
        if let Some(error) = query_callback_error {
            return Err(format!("RunQuery callback failed: {error}"));
        }
        record_void(&mut actual, "gl.RunQuery");
        record(
            &mut actual,
            "gl.GetQuery",
            vec![serde_json::json!(gfx
                .get_query(query_id)
                .map_err(|error| format!("GetQuery failed: {error:?}"))?)],
        );
        void!("gl.DeleteQuery", gfx.delete_query(query_id));

        compare_result(message, actual, "gl.lists_queries")
    }

    pub(crate) fn check_gl_atlas(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }

        void!("gl.ResetState", gfx.reset_state());
        let texture_params = spring_native::sys::GfxTextureParams {
            target: GL_TEXTURE_2D,
            format: GL_RGBA8,
            minFilter: GL_LINEAR,
            magFilter: GL_LINEAR,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            wrapR: GL_CLAMP_TO_EDGE,
            ..spring_native::sys::GfxTextureParams::default()
        };
        let texture_name = gfx
            .create_texture(4, 4, 0, texture_params)
            .map_err(|error| format!("CreateTexture failed: {error:?}"))?
            .ok_or_else(|| "CreateTexture returned no texture name".to_owned())?;
        let atlas_name = gfx
            .create_texture_atlas(256, 256, 0)
            .map_err(|error| format!("CreateTextureAtlas failed: {error:?}"))?
            .ok_or_else(|| "CreateTextureAtlas returned no atlas name".to_owned())?;
        record(
            &mut actual,
            "gl.CreateTexture",
            vec![serde_json::json!(true)],
        );
        record(
            &mut actual,
            "gl.CreateTextureAtlas",
            vec![serde_json::json!(true)],
        );
        void!(
            "gl.AddAtlasTexture",
            gfx.add_atlas_texture(&atlas_name, &texture_name)
        );
        record(
            &mut actual,
            "gl.FinalizeTextureAtlas",
            vec![serde_json::json!(gfx
                .finalize_texture_atlas(&atlas_name)
                .map_err(|error| format!(
                "FinalizeTextureAtlas failed: {error:?}"
            ))?)],
        );
        let (x1, x2, y1, y2, page) = gfx
            .get_atlas_texture(&atlas_name, &texture_name)
            .map_err(|error| format!("GetAtlasTexture failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetAtlasTexture",
            vec![
                rounded(x1),
                rounded(x2),
                rounded(y1),
                rounded(y2),
                serde_json::json!(page),
            ],
        );
        record(
            &mut actual,
            "gl.DeleteTextureAtlas",
            vec![serde_json::json!(gfx
                .delete_texture_atlas(&atlas_name)
                .map_err(|error| format!(
                    "DeleteTextureAtlas failed: {error:?}"
                ))?)],
        );
        record(
            &mut actual,
            "gl.DeleteTexture",
            vec![serde_json::json!(gfx
                .delete_texture(&texture_name)
                .map_err(|error| format!(
                    "DeleteTexture failed: {error:?}"
                ))?)],
        );

        let engine_atlas = gfx
            .get_engine_atlas_textures("$explosions")
            .map_err(|error| format!("GetEngineAtlasTextures failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetEngineAtlasTextures",
            vec![atlas_entries_value(&engine_atlas)],
        );

        let global_entries = gfx
            .get_global_tex_names()
            .map_err(|error| format!("GetGlobalTexNames failed: {error:?}"))?;
        let mut global_names = global_entries
            .iter()
            .filter_map(|entry| {
                (!entry.name.is_null())
                    .then(|| unsafe { CStr::from_ptr(entry.name).to_string_lossy().into_owned() })
            })
            .collect::<Vec<_>>();
        global_names.sort();
        record(
            &mut actual,
            "gl.GetGlobalTexNames",
            vec![serde_json::json!(global_names)],
        );
        if let Some(name) = global_names.first() {
            let (x1, x2, y1, y2, _page) = gfx
                .get_global_tex_coords(name)
                .map_err(|error| format!("GetGlobalTexCoords failed: {error:?}"))?;
            record(
                &mut actual,
                "gl.GetGlobalTexCoords",
                values([x1, x2, y1, y2]),
            );
        } else {
            record(&mut actual, "gl.GetGlobalTexCoords", Vec::new());
        }

        compare_result(message, actual, "gl.atlas")
    }

    pub(crate) fn check_gl_fbo(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }

        void!("gl.ResetState", gfx.reset_state());
        let texture_params = spring_native::sys::GfxTextureParams {
            target: GL_TEXTURE_2D,
            format: GL_RGBA8,
            minFilter: GL_LINEAR,
            magFilter: GL_LINEAR,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            wrapR: GL_CLAMP_TO_EDGE,
            ..spring_native::sys::GfxTextureParams::default()
        };
        let source_texture = gfx
            .create_texture(4, 4, 0, texture_params)
            .map_err(|error| format!("CreateTexture(source) failed: {error:?}"))?
            .ok_or_else(|| "CreateTexture(source) returned no name".to_owned())?;
        let destination_texture = gfx
            .create_texture(4, 4, 0, texture_params)
            .map_err(|error| format!("CreateTexture(destination) failed: {error:?}"))?
            .ok_or_else(|| "CreateTexture(destination) returned no name".to_owned())?;
        let depth_rbo = gfx
            .create_rbo(4, 4, GL_RENDERBUFFER, GL_DEPTH_COMPONENT24, 0)
            .map_err(|error| format!("CreateRBO failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.CreateTexture",
            vec![serde_json::json!(true)],
        );
        record(
            &mut actual,
            "gl.CreateRBO",
            vec![serde_json::json!(depth_rbo > 0)],
        );

        let draw_buffers = [GL_COLOR_ATTACHMENT0];
        let source_name = std::ffi::CString::new(source_texture.clone())
            .map_err(|error| format!("source texture name is invalid: {error}"))?;
        let source_attachments = [
            spring_native::sys::GfxFBOAttachment {
                attachment: GL_COLOR_ATTACHMENT0,
                textureName: source_name.as_ptr(),
                textureTarget: GL_TEXTURE_2D,
                mipLevel: 0,
                rboID: 0,
                useRBO: false,
            },
            spring_native::sys::GfxFBOAttachment {
                attachment: GL_DEPTH_ATTACHMENT,
                textureName: std::ptr::null(),
                textureTarget: 0,
                mipLevel: 0,
                rboID: depth_rbo,
                useRBO: true,
            },
        ];
        let destination_name = std::ffi::CString::new(destination_texture.clone())
            .map_err(|error| format!("destination texture name is invalid: {error}"))?;
        let destination_attachments = [spring_native::sys::GfxFBOAttachment {
            attachment: GL_COLOR_ATTACHMENT0,
            textureName: destination_name.as_ptr(),
            textureTarget: GL_TEXTURE_2D,
            mipLevel: 0,
            rboID: 0,
            useRBO: false,
        }];
        let (source_fbo, _) = gfx
            .create_fbo(GL_FRAMEBUFFER, &source_attachments, &draw_buffers, 0)
            .map_err(|error| format!("CreateFBO(source) failed: {error:?}"))?;
        let (destination_fbo, _) = gfx
            .create_fbo(GL_FRAMEBUFFER, &destination_attachments, &draw_buffers, 0)
            .map_err(|error| format!("CreateFBO(destination) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.CreateFBO",
            vec![serde_json::json!(source_fbo > 0 && destination_fbo > 0)],
        );

        let (source_valid, source_status) = gfx
            .is_valid_fbo(source_fbo, GL_FRAMEBUFFER)
            .map_err(|error| format!("IsValidFBO(source) failed: {error:?}"))?;
        let (destination_valid, destination_status) = gfx
            .is_valid_fbo(destination_fbo, GL_FRAMEBUFFER)
            .map_err(|error| format!("IsValidFBO(destination) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.IsValidFBO",
            vec![
                serde_json::json!(source_valid),
                serde_json::json!(source_status),
                serde_json::json!(destination_valid),
                serde_json::json!(destination_status),
            ],
        );

        let (previous_fbo, has_previous) =
            gfx.raw_bind_fbo(false, source_fbo, GL_FRAMEBUFFER, 0)
                .map_err(|error| format!("RawBindFBO(bind) failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.RawBindFBO",
            vec![serde_json::json!(previous_fbo)],
        );
        gfx.raw_bind_fbo(true, 0, GL_FRAMEBUFFER, 0)
            .map_err(|error| format!("RawBindFBO(restore) failed: {error:?}"))?;
        record_void(&mut actual, "gl.RawBindFBO.restore");
        let _ = has_previous;

        let mut clear_result = None;
        let mut callback_error = None;
        gfx.active_fbo(source_fbo, GL_FRAMEBUFFER, true, || {
            if let Err(error) = gfx.clear(GL_COLOR_BUFFER_BIT, [0.17, 0.27, 0.37, 0.47], 4) {
                callback_error = Some(format!("Clear failed: {error:?}"));
                return;
            }
            match gfx.clear_attachment_fbo(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                [0.21, 0.31, 0.41, 0.51],
                4,
            ) {
                Ok(value) => clear_result = Some(value),
                Err(error) => {
                    callback_error = Some(format!("ClearAttachmentFBO failed: {error:?}"))
                }
            }
        })
        .map_err(|error| format!("ActiveFBO failed: {error:?}"))?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        record_void(&mut actual, "gl.ActiveFBO");
        record(
            &mut actual,
            "gl.ClearAttachmentFBO",
            vec![serde_json::json!(clear_result.unwrap_or(false))],
        );

        void!(
            "gl.BlitFBO",
            gfx.blit_fbo(
                source_fbo,
                destination_fbo,
                0,
                0,
                4,
                4,
                0,
                0,
                4,
                4,
                GL_COLOR_BUFFER_BIT,
                GL_NEAREST,
            )
        );
        void!("gl.DeleteFBO", gfx.delete_fbo(source_fbo));
        gfx.delete_fbo(destination_fbo)
            .map_err(|error| format!("DeleteFBO(destination) failed: {error:?}"))?;
        void!("gl.DeleteRBO", gfx.delete_rbo(depth_rbo));
        gfx.delete_texture(&source_texture)
            .map_err(|error| format!("DeleteTexture(source) failed: {error:?}"))?;
        gfx.delete_texture(&destination_texture)
            .map_err(|error| format!("DeleteTexture(destination) failed: {error:?}"))?;
        record_void(&mut actual, "gl.DeleteTexture");

        compare_result(message, actual, "gl.fbo")
    }

    pub(crate) fn check_gl_fonts(&self, message: &Value) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let mut actual = Map::new();

        macro_rules! void {
            ($name:literal, $call:expr) => {{
                $call.map_err(|error| format!("{} failed: {error:?}", $name))?;
                record_void(&mut actual, $name);
            }};
        }

        void!("gl.ResetState", gfx.reset_state());
        void!("gl.Color", gfx.color(0.31, 0.41, 0.51, 0.61));
        void!("gl.BeginText", gfx.begin_text(false));
        void!("gl.Text", gfx.text("Native parity", 8.0, 8.0, 12.0, ""));
        void!("gl.EndText", gfx.end_text());
        record(
            &mut actual,
            "gl.GetTextWidth",
            vec![rounded(gfx.get_text_width("Native parity").map_err(
                |error| format!("GetTextWidth failed: {error:?}"),
            )?)],
        );
        let (height, descender, lines) = gfx
            .get_text_height("Native parity")
            .map_err(|error| format!("GetTextHeight failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.GetTextHeight",
            vec![
                rounded(height),
                rounded(descender),
                serde_json::json!(lines),
            ],
        );

        let font_id = gfx
            .load_font("fonts/FreeSansBold.otf", 12, 2, 15.0)
            .map_err(|error| format!("LoadFont failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.LoadFont",
            vec![serde_json::json!(font_id > 0)],
        );
        let (
            path,
            family,
            style,
            size,
            line_height,
            font_descender,
            outline_width,
            outline_weight,
            texture_width,
            texture_height,
        ) = gfx
            .get_font_info(font_id)
            .map_err(|error| format!("GetFontInfo failed: {error:?}"))?;
        record(
            &mut actual,
            "gl.LoadFont.info",
            vec![
                path.map_or(Value::Null, |value| serde_json::json!(value)),
                family.map_or(Value::Null, |value| serde_json::json!(value)),
                style.map_or(Value::Null, |value| serde_json::json!(value)),
                rounded(size),
                rounded(line_height),
                rounded(font_descender),
                rounded(outline_width),
                rounded(outline_weight),
                serde_json::json!(texture_width),
                serde_json::json!(texture_height),
            ],
        );
        record(
            &mut actual,
            "LuaFont.GetTextWidth",
            vec![rounded(
                gfx.font_get_text_width(font_id, "Native parity", 0.0, 0.0, 12.0, "")
                    .map_err(|error| format!("LuaFont.GetTextWidth failed: {error:?}"))?,
            )],
        );
        let (height, descender, lines) = gfx
            .font_get_text_height(font_id, "Native parity", 0.0, 0.0, 12.0, "")
            .map_err(|error| format!("LuaFont.GetTextHeight failed: {error:?}"))?;
        record(
            &mut actual,
            "LuaFont.GetTextHeight",
            vec![
                rounded(height),
                rounded(descender),
                serde_json::json!(lines),
            ],
        );
        let (wrapped, wrapped_lines) = gfx
            .font_wrap_text(font_id, "one two three four", 20.0, 100.0, 12.0)
            .map_err(|error| format!("LuaFont.WrapText failed: {error:?}"))?;
        record(
            &mut actual,
            "LuaFont.WrapText",
            vec![
                wrapped.map_or(Value::Null, |value| serde_json::json!(value)),
                serde_json::json!(wrapped_lines),
            ],
        );
        void!(
            "LuaFont.SetTextColor",
            gfx.font_set_text_color(font_id, 0.11, 0.22, 0.33, 0.44)
        );
        void!(
            "LuaFont.SetOutlineColor",
            gfx.font_set_outline_color(font_id, 0.55, 0.66, 0.77, 0.88)
        );
        void!(
            "LuaFont.SetAutoOutlineColor",
            gfx.font_set_auto_outline_color(font_id, true)
        );
        void!("LuaFont.Begin", gfx.font_begin(font_id, false));
        void!(
            "LuaFont.Print",
            gfx.font_print(font_id, "Native parity", 8.0, 8.0, 12.0, "")
        );
        void!(
            "LuaFont.PrintWorld",
            gfx.font_print_world(
                font_id,
                "Native parity",
                spring_native::sys::Float3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                12.0,
                "",
            )
        );
        void!("LuaFont.End", gfx.font_end(font_id));
        void!(
            "LuaFont.SubmitBuffered",
            gfx.font_submit_buffered(
                font_id,
                spring_native::GfxFontSubmitBufferedOptions {
                    no_billboarding: true,
                    user_defined_blending: false,
                },
            )
        );
        void!("LuaFont.BindTexture", gfx.font_bind_texture(font_id));
        void!("gl.DeleteFont", gfx.delete_font(font_id));
        record(
            &mut actual,
            "gl.AddFallbackFont",
            vec![serde_json::json!(gfx
                .add_fallback_font("fonts/FreeSansBold.otf")
                .map_err(|error| format!(
                    "AddFallbackFont failed: {error:?}"
                ))?)],
        );
        void!("gl.ClearFallbackFonts", gfx.clear_fallback_fonts());
        void!("gl.ResetState.restore", gfx.reset_state());

        compare_result(message, actual, "gl.fonts")
    }
}

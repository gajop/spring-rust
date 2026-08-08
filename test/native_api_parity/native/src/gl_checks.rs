use super::*;
use serde_json::{Map, Value};
use spring_native::constants::*;

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

fn compare_result(message: &Value, actual: Map<String, Value>, label: &str) -> Result<(), String> {
    let expected = message
        .get("result")
        .ok_or_else(|| format!("{label} payload is missing `result`"))?;
    let actual = Value::Object(actual);
    if expected != &actual {
        return Err(format!(
            "{label} result mismatch: expected={expected}, actual={actual}"
        ));
    }
    Ok(())
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
}

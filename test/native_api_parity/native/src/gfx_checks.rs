use super::*;
use spring_native::{constants::*, sys::GfxTextureParams};

const COMPUTE_SHADER: &str = r#"#version 430
layout(local_size_x = 2, local_size_y = 2) in;
layout(r32f, binding = 0) readonly uniform image2D source_image;
layout(r32f, binding = 1) writeonly uniform image2D destination_image;
void main() {
    ivec2 position = ivec2(gl_GlobalInvocationID.xy);
    float value = imageLoad(source_image, position).r;
    imageStore(destination_image, position, vec4(value * 2.0));
}
"#;

impl NativeApiParity {
    pub(crate) fn check_gfx_compute_upload(&self) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let params = GfxTextureParams {
            target: GL_TEXTURE_2D,
            format: GL_R32F,
            minFilter: GL_NEAREST,
            magFilter: GL_NEAREST,
            wrapS: GL_CLAMP_TO_EDGE,
            wrapT: GL_CLAMP_TO_EDGE,
            fbo: true,
            ..GfxTextureParams::default()
        };
        let source = gfx
            .create_texture(2, 2, 1, params)
            .map_err(|error| format!("{error:?}"))?
            .ok_or_else(|| "CreateTexture returned no source name".to_owned())?;
        let destination = gfx
            .create_texture(2, 2, 1, params)
            .map_err(|error| format!("{error:?}"))?
            .ok_or_else(|| "CreateTexture returned no destination name".to_owned())?;

        let result = self.run_gfx_compute_upload(&source, &destination);
        let _ = gfx.delete_texture(&source);
        let _ = gfx.delete_texture(&destination);
        result
    }

    fn run_gfx_compute_upload(&self, source: &str, destination: &str) -> Result<(), String> {
        let gfx = self.interface.gfx();
        let bytes = [1.0_f32, 2.0, 3.0, 4.0]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>();
        gfx.upload_texture(source, 0, 0, 0, 0, 0, 2, 2, 1, GL_RED, GL_FLOAT, &bytes)
            .map_err(|error| format!("{error:?}"))?;

        let (shader, _) = gfx
            .create_shader(
                "",
                "",
                "",
                "",
                "",
                "",
                COMPUTE_SHADER,
                false,
                0,
                false,
                0,
                false,
                0,
            )
            .map_err(|error| format!("{error:?}"))?;
        let result = self.dispatch_and_read(shader, source, destination);
        let _ = gfx.use_shader(0);
        let _ = gfx.bind_image_texture(0, "", 0, 0, false, GL_READ_ONLY, GL_R32F);
        let _ = gfx.bind_image_texture(1, "", 0, 0, false, GL_WRITE_ONLY, GL_R32F);
        let _ = gfx.delete_shader(shader);
        result
    }

    fn dispatch_and_read(
        &self,
        shader: u32,
        source: &str,
        destination: &str,
    ) -> Result<(), String> {
        let gfx = self.interface.gfx();
        if !gfx
            .use_shader(shader)
            .map_err(|error| format!("{error:?}"))?
        {
            let log = gfx.get_shader_log().map_err(|error| format!("{error:?}"))?;
            return Err(format!(
                "compute shader did not link: {}",
                log.unwrap_or_default()
            ));
        }
        gfx.bind_image_texture(0, source, 0, 0, false, GL_READ_ONLY, GL_R32F)
            .map_err(|error| format!("{error:?}"))?;
        gfx.bind_image_texture(1, destination, 0, 0, false, GL_WRITE_ONLY, GL_R32F)
            .map_err(|error| format!("{error:?}"))?;
        gfx.dispatch_compute(1, 1, 1, GL_SHADER_IMAGE_ACCESS_BARRIER_BIT)
            .map_err(|error| format!("{error:?}"))?;

        let mut readback = None;
        gfx.render_to_texture(destination, || {
            readback = Some(gfx.read_pixels(0, 0, 2, 2, GL_RED));
        })
        .map_err(|error| format!("{error:?}"))?;
        let (values, components) = readback
            .ok_or_else(|| "RenderToTexture callback was not called".to_owned())?
            .map_err(|error| format!("{error:?}"))?;
        if components != 1 || values != [2.0, 4.0, 6.0, 8.0] {
            return Err(format!(
                "unexpected readback: components={components}, values={values:?}"
            ));
        }
        Ok(())
    }
}

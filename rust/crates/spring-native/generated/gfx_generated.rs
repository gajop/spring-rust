#[derive(Debug, Clone, Copy, Default)]
pub struct GfxColorMaskOptions {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
}

impl From<GfxColorMaskOptions> for sys::GfxColorMaskOptions {
    fn from(options: GfxColorMaskOptions) -> Self {
        sys::GfxColorMaskOptions {
            red: options.red,
            green: options.green,
            blue: options.blue,
            alpha: options.alpha,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxCreateShaderOptions {
    pub has_geo_input_type: bool,
    pub geo_input_type: u32,
    pub has_geo_output_type: bool,
    pub geo_output_type: u32,
    pub has_geo_output_verts: bool,
    pub geo_output_verts: i32,
}

impl From<GfxCreateShaderOptions> for sys::GfxCreateShaderOptions {
    fn from(options: GfxCreateShaderOptions) -> Self {
        sys::GfxCreateShaderOptions {
            hasGeoInputType: options.has_geo_input_type,
            geoInputType: options.geo_input_type,
            hasGeoOutputType: options.has_geo_output_type,
            geoOutputType: options.geo_output_type,
            hasGeoOutputVerts: options.has_geo_output_verts,
            geoOutputVerts: options.geo_output_verts,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxDepthTestOptions {
    pub enable: bool,
    pub set_func: bool,
    pub func: u32,
}

impl From<GfxDepthTestOptions> for sys::GfxDepthTestOptions {
    fn from(options: GfxDepthTestOptions) -> Self {
        sys::GfxDepthTestOptions {
            enable: options.enable,
            setFunc: options.set_func,
            func: options.func,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxFeatureDrawOptions {
    pub apply_transform: bool,
    pub do_raw_draw: bool,
    pub no_lua_call: bool,
    pub has_lua_mat_lod: bool,
    pub lua_mat_lod: i32,
}

impl From<GfxFeatureDrawOptions> for sys::GfxFeatureDrawOptions {
    fn from(options: GfxFeatureDrawOptions) -> Self {
        sys::GfxFeatureDrawOptions {
            applyTransform: options.apply_transform,
            doRawDraw: options.do_raw_draw,
            noLuaCall: options.no_lua_call,
            hasLuaMatLOD: options.has_lua_mat_lod,
            luaMatLOD: options.lua_mat_lod,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxFontSubmitBufferedOptions {
    pub no_billboarding: bool,
    pub user_defined_blending: bool,
}

impl From<GfxFontSubmitBufferedOptions> for sys::GfxFontSubmitBufferedOptions {
    fn from(options: GfxFontSubmitBufferedOptions) -> Self {
        sys::GfxFontSubmitBufferedOptions {
            noBillboarding: options.no_billboarding,
            userDefinedBlending: options.user_defined_blending,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxLightOptions {
    pub set_state: bool,
    pub state: bool,
}

impl From<GfxLightOptions> for sys::GfxLightOptions {
    fn from(options: GfxLightOptions) -> Self {
        sys::GfxLightOptions {
            setState: options.set_state,
            state: options.state,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxMultiTexGenOptions {
    pub set_state: bool,
    pub state: bool,
}

impl From<GfxMultiTexGenOptions> for sys::GfxMultiTexGenOptions {
    fn from(options: GfxMultiTexGenOptions) -> Self {
        sys::GfxMultiTexGenOptions {
            setState: options.set_state,
            state: options.state,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxObjectShapeOptions {
    pub raw_state: bool,
    pub to_screen: bool,
    pub opaque: bool,
}

impl From<GfxObjectShapeOptions> for sys::GfxObjectShapeOptions {
    fn from(options: GfxObjectShapeOptions) -> Self {
        sys::GfxObjectShapeOptions {
            rawState: options.raw_state,
            toScreen: options.to_screen,
            opaque: options.opaque,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxSaveImageOptions {
    pub alpha: bool,
    pub yflip: bool,
    pub grayscale16bit: bool,
}

impl From<GfxSaveImageOptions> for sys::GfxSaveImageOptions {
    fn from(options: GfxSaveImageOptions) -> Self {
        sys::GfxSaveImageOptions {
            alpha: options.alpha,
            yflip: options.yflip,
            grayscale16bit: options.grayscale16bit,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxTexGenOptions {
    pub set_state: bool,
    pub state: bool,
}

impl From<GfxTexGenOptions> for sys::GfxTexGenOptions {
    fn from(options: GfxTexGenOptions) -> Self {
        sys::GfxTexGenOptions {
            setState: options.set_state,
            state: options.state,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GfxUnitDrawOptions {
    pub apply_transform: bool,
    pub do_raw_draw: bool,
    pub no_lua_call: bool,
    pub full_model: bool,
    pub has_lua_mat_lod: bool,
    pub lua_mat_lod: i32,
}

impl From<GfxUnitDrawOptions> for sys::GfxUnitDrawOptions {
    fn from(options: GfxUnitDrawOptions) -> Self {
        sys::GfxUnitDrawOptions {
            applyTransform: options.apply_transform,
            doRawDraw: options.do_raw_draw,
            noLuaCall: options.no_lua_call,
            fullModel: options.full_model,
            hasLuaMatLOD: options.has_lua_mat_lod,
            luaMatLOD: options.lua_mat_lod,
        }
    }
}

/// The complete result tuple returned by [`get_view_range`].
pub type GetViewRangeValue = (f32, f32, f32, f32);

/// The complete result tuple returned by [`get_atmosphere`].
pub type GetAtmosphereValue = ([f32; 4], u32, bool, bool, Option<String>);

/// The complete result tuple returned by [`get_sun`].
pub type GetSunValue = ([f32; 4], u32, bool, bool, Option<String>);

/// The complete result tuple returned by [`get_water_rendering`].
pub type GetWaterRenderingValue = ([f32; 4], u32, bool, bool, Option<String>);

/// The complete result tuple returned by [`get_map_rendering`].
pub type GetMapRenderingValue = ([f32; 4], u32, bool, bool, Option<String>);

/// The complete result tuple returned by [`texture_info`].
pub type TextureInfoValue = (i32, i32, i32, u32, u32, u32);

/// The complete result tuple returned by [`get_rboinfo`].
pub type GetRBOInfoValue = (bool, u32, u32, i32, i32, i32);

/// The complete result tuple returned by [`get_vboinfo`].
pub type GetVBOInfoValue = (u32, u32, u32, u32, u32, u32);

/// The complete result tuple returned by [`get_atlas_texture`].
pub type GetAtlasTextureValue = (f32, f32, f32, f32, i32);

/// The complete result tuple returned by [`get_global_tex_coords`].
pub type GetGlobalTexCoordsValue = (f32, f32, f32, f32, i32);

/// The complete result tuple returned by [`get_font_info`].
pub type GetFontInfoValue = (Option<String>, Option<String>, Option<String>, f32, f32, f32, f32, f32, i32, i32);

/// The complete result tuple returned by [`get_fixed_state`].
pub type GetFixedStateValue = ([bool; 8], u32, [i32; 16], u32, [f32; 16], u32);

impl<'a> Gfx<'a> {
    pub fn has_extension(&self, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::GfxStringQuery {
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.HasExtension.expect("HasExtension function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_number(&self, pname: u32, max_values: u32) -> Result<([f32; 16], u32), Error> {
        unsafe {
            let query = sys::GfxGetNumberQuery {
                pname,
                maxValues: max_values,
            };
            let mut result = MaybeUninit::<sys::GfxGetNumberResult>::zeroed();
            let func = self.api.GetNumber.expect("GetNumber function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.values,
                result.count,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_string(&self, pname: u32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GfxGetStringQuery {
                pname,
            };
            let mut result = MaybeUninit::<sys::GfxStringResult>::zeroed();
            let func = self.api.GetString.expect("GetString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_view_sizes(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxViewSizesResult>::zeroed();
            let func = self.api.GetViewSizes.expect("GetViewSizes function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.viewSizeX,
                result.viewSizeY,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_view_range(&self, camera_type: i32) -> Result<GetViewRangeValue, Error> {
        unsafe {
            let query = sys::GfxViewRangeQuery {
                cameraType: camera_type,
            };
            let mut result = MaybeUninit::<sys::GfxViewRangeResult>::zeroed();
            let func = self.api.GetViewRange.expect("GetViewRange function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.nearPlaneDist,
                result.farPlaneDist,
                result.minViewRange,
                result.maxViewRange,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_shadow_map_params(&self) -> Result<sys::Float4, Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxShadowMapParamsResult>::zeroed();
            let func = self.api.GetShadowMapParams.expect("GetShadowMapParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.params
            })
        }
    }

    pub fn get_atmosphere(&self, key: &str, mode: &str) -> Result<GetAtmosphereValue, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::GfxValueQuery {
                key: key_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxValueResult>::zeroed();
            let func = self.api.GetAtmosphere.expect("GetAtmosphere function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.values,
                result.count,
                result.boolValue,
                result.hasBool,
                {
                    if result.stringValue.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.stringValue).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_sun(&self, key: &str, mode: &str) -> Result<GetSunValue, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::GfxValueQuery {
                key: key_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxValueResult>::zeroed();
            let func = self.api.GetSun.expect("GetSun function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.values,
                result.count,
                result.boolValue,
                result.hasBool,
                {
                    if result.stringValue.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.stringValue).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_water_rendering(&self, key: &str, mode: &str) -> Result<GetWaterRenderingValue, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::GfxValueQuery {
                key: key_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxValueResult>::zeroed();
            let func = self.api.GetWaterRendering.expect("GetWaterRendering function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.values,
                result.count,
                result.boolValue,
                result.hasBool,
                {
                    if result.stringValue.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.stringValue).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_map_rendering(&self, key: &str, mode: &str) -> Result<GetMapRenderingValue, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::GfxValueQuery {
                key: key_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxValueResult>::zeroed();
            let func = self.api.GetMapRendering.expect("GetMapRendering function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.values,
                result.count,
                result.boolValue,
                result.hasBool,
                {
                    if result.stringValue.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.stringValue).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn reset_state(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ResetState.expect("ResetState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn clear(&self, bits: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxClearQuery {
                bits,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Clear.expect("Clear function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn flush(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Flush.expect("Flush function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn finish(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Finish.expect("Finish function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn swap_buffers(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SwapBuffers.expect("SwapBuffers function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn reset_matrices(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ResetMatrices.expect("ResetMatrices function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn depth_test(&self, options: GfxDepthTestOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxDepthTestQuery {
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DepthTest.expect("DepthTest function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn depth_mask(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DepthMask.expect("DepthMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn culling(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Culling.expect("Culling function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn cull_face(&self, face: sys::GfxCullFace) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxCullFaceQuery {
                face,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.CullFace.expect("CullFace function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn blending(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Blending.expect("Blending function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn blend_func(&self, src: u32, dst: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBlendFuncQuery {
                src,
                dst,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BlendFunc.expect("BlendFunc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBlendFuncSeparateQuery {
                srcRGB: src_rgb,
                dstRGB: dst_rgb,
                srcAlpha: src_alpha,
                dstAlpha: dst_alpha,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BlendFuncSeparate.expect("BlendFuncSeparate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn blend_equation(&self, mode: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBlendEquationQuery {
                mode,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BlendEquation.expect("BlendEquation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBlendEquationSeparateQuery {
                modeRGB: mode_rgb,
                modeAlpha: mode_alpha,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BlendEquationSeparate.expect("BlendEquationSeparate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn color_mask(&self, options: GfxColorMaskOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxColorMaskQuery {
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ColorMask.expect("ColorMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn alpha_test(&self, enable: bool, func: u32, r#ref: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxAlphaTestQuery {
                enable,
                func,
                ref_: r#ref,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.AlphaTest.expect("AlphaTest function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn alpha_to_coverage(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.AlphaToCoverage.expect("AlphaToCoverage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn stencil_test(&self, enable: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxStencilTestQuery {
                enable,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.StencilTest.expect("StencilTest function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn stencil_func(&self, func: u32, r#ref: i32, mask: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxStencilFuncQuery {
                func,
                ref_: r#ref,
                mask,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.StencilFunc.expect("StencilFunc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn stencil_func_separate(&self, face: u32, func: u32, r#ref: i32, mask: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxStencilFuncSeparateQuery {
                face,
                func,
                ref_: r#ref,
                mask,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.StencilFuncSeparate.expect("StencilFuncSeparate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn stencil_mask(&self, mask: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxStencilMaskQuery {
                mask,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.StencilMask.expect("StencilMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn stencil_mask_separate(&self, face: u32, mask: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxStencilMaskSeparateQuery {
                face,
                mask,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.StencilMaskSeparate.expect("StencilMaskSeparate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn stencil_op(&self, fail: u32, zfail: u32, zpass: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxStencilOpQuery {
                fail,
                zfail,
                zpass,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.StencilOp.expect("StencilOp function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn stencil_op_separate(&self, face: u32, fail: u32, zfail: u32, zpass: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxStencilOpSeparateQuery {
                face,
                fail,
                zfail,
                zpass,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.StencilOpSeparate.expect("StencilOpSeparate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn polygon_mode(&self, face: u32, mode: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxPolygonModeQuery {
                face,
                mode,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PolygonMode.expect("PolygonMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn polygon_offset(&self, factor: f32, units: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxPolygonOffsetQuery {
                factor,
                units,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PolygonOffset.expect("PolygonOffset function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn logic_op(&self, enable: bool, opcode: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxLogicOpQuery {
                enable,
                opcode,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.LogicOp.expect("LogicOp function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn shade_model(&self, mode: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxShadeModelQuery {
                mode,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ShadeModel.expect("ShadeModel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxScissorQuery {
                x,
                y,
                width,
                height,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Scissor.expect("Scissor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxViewportQuery {
                x,
                y,
                width,
                height,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Viewport.expect("Viewport function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn line_width(&self, value: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFloatQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.LineWidth.expect("LineWidth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn line_stipple(&self, factor: i32, pattern: u16) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxLineStippleQuery {
                factor,
                pattern,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.LineStipple.expect("LineStipple function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn point_size(&self, value: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFloatQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PointSize.expect("PointSize function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn point_sprite(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PointSprite.expect("PointSprite function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn point_parameter(&self, pname: u32, value: f32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxPointParameterQuery {
                pname,
                value,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PointParameter.expect("PointParameter function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn clip_plane(&self, plane: u32, equation: [f32; 4]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxClipPlaneQuery {
                plane,
                equation,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ClipPlane.expect("ClipPlane function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn clip_distance(&self, index: u32, enable: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxClipDistanceQuery {
                index,
                enable,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ClipDistance.expect("ClipDistance function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn push_attrib(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PushAttrib.expect("PushAttrib function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn pop_attrib(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PopAttrib.expect("PopAttrib function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn depth_clamp(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DepthClamp.expect("DepthClamp function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn fog(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Fog.expect("Fog function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn lighting(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Lighting.expect("Lighting function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn light(&self, light: i32, options: GfxLightOptions, pname: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxLightQuery {
                light,
                options: options.into(),
                pname,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Light.expect("Light function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn material(&self, pname: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMaterialQuery {
                pname,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Material.expect("Material function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn tex_env(&self, target: u32, pname: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTexEnvQuery {
                target,
                pname,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.TexEnv.expect("TexEnv function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn text_env(&self, target: u32, pname: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTexEnvQuery {
                target,
                pname,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.TextEnv.expect("TextEnv function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn multi_tex_env(&self, tex_num: i32, target: u32, pname: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMultiTexEnvQuery {
                texNum: tex_num,
                target,
                pname,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.MultiTexEnv.expect("MultiTexEnv function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn tex_gen(&self, target: u32, options: GfxTexGenOptions, pname: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTexGenQuery {
                target,
                options: options.into(),
                pname,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.TexGen.expect("TexGen function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn multi_tex_gen(&self, tex_num: i32, target: u32, options: GfxMultiTexGenOptions, pname: u32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMultiTexGenQuery {
                texNum: tex_num,
                target,
                options: options.into(),
                pname,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.MultiTexGen.expect("MultiTexGen function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn dispatch_compute(&self, num_group_x: u32, num_group_y: u32, num_group_z: u32, barriers: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxDispatchComputeQuery {
                numGroupX: num_group_x,
                numGroupY: num_group_y,
                numGroupZ: num_group_z,
                barriers,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DispatchCompute.expect("DispatchCompute function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn memory_barrier(&self, barriers: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMemoryBarrierQuery {
                barriers,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.MemoryBarrier.expect("MemoryBarrier function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn active_texture(&self, tex_num: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxActiveTextureQuery {
                texNum: tex_num,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ActiveTexture.expect("ActiveTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn object_label(&self, identifier: u32, object_id: u32, label: &str) -> Result<(), Error> {
        unsafe {
            let label_cstr = std::ffi::CString::new(label).map_err(|_| Error::invalid_argument("label"))?;
            let query = sys::GfxObjectLabelQuery {
                identifier,
                objectID: object_id,
                label: label_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ObjectLabel.expect("ObjectLabel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn push_debug_group(&self, id: u32, message: &str, source_is_third_party: bool) -> Result<(), Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::GfxPushDebugGroupQuery {
                id,
                message: message_cstr.as_ptr(),
                sourceIsThirdParty: source_is_third_party,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PushDebugGroup.expect("PushDebugGroup function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn pop_debug_group(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PopDebugGroup.expect("PopDebugGroup function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn create_shader(&self, definitions: &str, vertex: &str, tcs: &str, tes: &str, geometry: &str, fragment: &str, compute: &str, options: GfxCreateShaderOptions) -> Result<(u32, u32), Error> {
        unsafe {
            let definitions_cstr = std::ffi::CString::new(definitions).map_err(|_| Error::invalid_argument("definitions"))?;
            let vertex_cstr = std::ffi::CString::new(vertex).map_err(|_| Error::invalid_argument("vertex"))?;
            let tcs_cstr = std::ffi::CString::new(tcs).map_err(|_| Error::invalid_argument("tcs"))?;
            let tes_cstr = std::ffi::CString::new(tes).map_err(|_| Error::invalid_argument("tes"))?;
            let geometry_cstr = std::ffi::CString::new(geometry).map_err(|_| Error::invalid_argument("geometry"))?;
            let fragment_cstr = std::ffi::CString::new(fragment).map_err(|_| Error::invalid_argument("fragment"))?;
            let compute_cstr = std::ffi::CString::new(compute).map_err(|_| Error::invalid_argument("compute"))?;
            let query = sys::GfxCreateShaderQuery {
                definitions: definitions_cstr.as_ptr(),
                vertex: vertex_cstr.as_ptr(),
                tcs: tcs_cstr.as_ptr(),
                tes: tes_cstr.as_ptr(),
                geometry: geometry_cstr.as_ptr(),
                fragment: fragment_cstr.as_ptr(),
                compute: compute_cstr.as_ptr(),
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxCreateShaderResult>::zeroed();
            let func = self.api.CreateShader.expect("CreateShader function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.shaderID,
                result.glProgramID,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn delete_shader(&self, shader_id: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GfxShaderQuery {
                shaderID: shader_id,
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.DeleteShader.expect("DeleteShader function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn use_shader(&self, shader_id: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GfxShaderQuery {
                shaderID: shader_id,
            };
            let mut result = MaybeUninit::<sys::GfxUseShaderResult>::zeroed();
            let func = self.api.UseShader.expect("UseShader function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.linked
            })
        }
    }

    pub fn active_shader<F: FnMut()>(&self, shader_id: u32, mut callback: F) -> Result<(), Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxActiveShaderQuery {
                shaderID: shader_id,
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ActiveShader.expect("ActiveShader function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_shader_log(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxStringResult>::zeroed();
            let func = self.api.GetShaderLog.expect("GetShaderLog function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_uniform_location(&self, shader_id: u32, name: &str) -> Result<i32, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxUniformLocationQuery {
                shaderID: shader_id,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxUniformLocationResult>::zeroed();
            let func = self.api.GetUniformLocation.expect("GetUniformLocation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.location
            })
        }
    }

    pub fn get_active_uniforms(&self, shader_id: u32) -> Result<Vec<sys::GfxActiveUniformEntry>, Error> {
        unsafe {
            let query = sys::GfxShaderQuery {
                shaderID: shader_id,
            };
            let mut result = MaybeUninit::<sys::GfxActiveUniformsResult>::zeroed();
            let func = self.api.GetActiveUniforms.expect("GetActiveUniforms function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn uniform(&self, location: i32, values: [f32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUniformFloatQuery {
                location,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Uniform.expect("Uniform function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn uniform_int(&self, location: i32, values: [i32; 4], count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUniformIntQuery {
                location,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UniformInt.expect("UniformInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn uniform_array_float(&self, location: i32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUniformArrayFloatQuery {
                location,
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UniformArrayFloat.expect("UniformArrayFloat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn uniform_array_int(&self, location: i32, values: &[i32]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUniformArrayIntQuery {
                location,
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UniformArrayInt.expect("UniformArrayInt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn uniform_matrix(&self, location: i32, values: &[f32], transpose: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUniformMatrixQuery {
                location,
                values: values.as_ptr(),
                count: values.len() as u32,
                transpose,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UniformMatrix.expect("UniformMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_subroutine_index(&self, shader_id: u32, shader_type: u32, name: &str) -> Result<(i32, bool), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxSubroutineIndexQuery {
                shaderID: shader_id,
                shaderType: shader_type,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxSubroutineIndexResult>::zeroed();
            let func = self.api.GetSubroutineIndex.expect("GetSubroutineIndex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.index,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn uniform_subroutine(&self, shader_type: u32, index: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUniformSubroutineQuery {
                shaderType: shader_type,
                index,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UniformSubroutine.expect("UniformSubroutine function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn set_geometry_shader_parameter(&self, shader_id: u32, param: u32, value: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxGeometryShaderParameterQuery {
                shaderID: shader_id,
                param,
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SetGeometryShaderParameter.expect("SetGeometryShaderParameter function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn set_tesselation_shader_parameter(&self, param: u32, value: i32, values: [f32; 4], value_count: u32, use_float_array: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTesselationShaderParameterQuery {
                param,
                value,
                values,
                valueCount: value_count,
                useFloatArray: use_float_array,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SetTesselationShaderParameter.expect("SetTesselationShaderParameter function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_engine_uniform_buffer_def(&self, index: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GfxEngineUniformBufferDefQuery {
                index,
            };
            let mut result = MaybeUninit::<sys::GfxStringResult>::zeroed();
            let func = self.api.GetEngineUniformBufferDef.expect("GetEngineUniformBufferDef function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_engine_model_uniform_data_def(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxStringResult>::zeroed();
            let func = self.api.GetEngineModelUniformDataDef.expect("GetEngineModelUniformDataDef function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_engine_model_uniform_data_size(&self) -> Result<(u32, u32), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEngineModelUniformDataSizeResult>::zeroed();
            let func = self.api.GetEngineModelUniformDataSize.expect("GetEngineModelUniformDataSize function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.sizeInElements,
                result.sizeInBytesOnCPU,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn set_unit_buffer_uniforms(&self, object_id: i32, values: &[f32], offset: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxObjectBufferUniformsQuery {
                objectID: object_id,
                values: values.as_ptr(),
                count: values.len() as u32,
                offset,
            };
            let mut result = MaybeUninit::<sys::GfxObjectBufferUniformsResult>::zeroed();
            let func = self.api.SetUnitBufferUniforms.expect("SetUnitBufferUniforms function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn set_feature_buffer_uniforms(&self, object_id: i32, values: &[f32], offset: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxObjectBufferUniformsQuery {
                objectID: object_id,
                values: values.as_ptr(),
                count: values.len() as u32,
                offset,
            };
            let mut result = MaybeUninit::<sys::GfxObjectBufferUniformsResult>::zeroed();
            let func = self.api.SetFeatureBufferUniforms.expect("SetFeatureBufferUniforms function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn create_texture(&self, xsize: i32, ysize: i32, zsize: i32, params: sys::GfxTextureParams) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GfxCreateTextureQuery {
                xsize,
                ysize,
                zsize,
                params,
            };
            let mut result = MaybeUninit::<sys::GfxStringResult>::zeroed();
            let func = self.api.CreateTexture.expect("CreateTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn delete_texture(&self, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.DeleteTexture.expect("DeleteTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn delete_texture_fbo(&self, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.DeleteTextureFBO.expect("DeleteTextureFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn bind_texture(&self, name: &str, tex_num: i32, enable: bool) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureBindQuery {
                name: name_cstr.as_ptr(),
                texNum: tex_num,
                enable,
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.BindTexture.expect("BindTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn texture_info(&self, name: &str) -> Result<TextureInfoValue, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxTextureInfoResult>::zeroed();
            let func = self.api.TextureInfo.expect("TextureInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.xsize,
                result.ysize,
                result.zsize,
                result.id,
                result.target,
                result.fbo,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_engine_texture_names(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEngineTextureNamesResult>::zeroed();
            let func = self.api.GetEngineTextureNames.expect("GetEngineTextureNames function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.names.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.names, result.count as usize);
                        slice.iter().map(|&ptr| {
                            if ptr.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(ptr).to_string_lossy().into_owned()
                            }
                        }).collect()
                    }
                }
            })
        }
    }

    pub fn get_console_commands(&self) -> Result<Vec<sys::GfxConsoleCommandEntry>, Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxConsoleCommandsResult>::zeroed();
            let func = self.api.GetConsoleCommands.expect("GetConsoleCommands function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn change_texture_params(&self, name: &str, params: sys::GfxTextureParams) -> Result<(), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxChangeTextureParamsQuery {
                name: name_cstr.as_ptr(),
                params,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ChangeTextureParams.expect("ChangeTextureParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn copy_to_texture(&self, name: &str, xoff: i32, yoff: i32, x: i32, y: i32, width: i32, height: i32, target: u32, level: u32) -> Result<(), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxCopyToTextureQuery {
                name: name_cstr.as_ptr(),
                xoff,
                yoff,
                x,
                y,
                width,
                height,
                target,
                level,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.CopyToTexture.expect("CopyToTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn upload_texture(&self, name: &str, target: u32, level: i32, xoff: i32, yoff: i32, zoff: i32, width: i32, height: i32, depth: i32, format: u32, pixel_type: u32, data: &[u8]) -> Result<(), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxUploadTextureQuery {
                name: name_cstr.as_ptr(),
                target,
                level,
                xoff,
                yoff,
                zoff,
                width,
                height,
                depth,
                format,
                pixelType: pixel_type,
                data: data.as_ptr(),
                dataSize: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UploadTexture.expect("UploadTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn generate_mipmap(&self, name: &str) -> Result<(), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.GenerateMipmap.expect("GenerateMipmap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn bind_image_texture(&self, unit: u32, name: &str, level: i32, layer: i32, layered: bool, access: u32, format: u32) -> Result<(), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxBindImageTextureQuery {
                unit,
                name: name_cstr.as_ptr(),
                level,
                layer,
                layered,
                access,
                format,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BindImageTexture.expect("BindImageTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32, format: u32) -> Result<(Vec<f32>, u32), Error> {
        unsafe {
            let query = sys::GfxReadPixelsQuery {
                x,
                y,
                width,
                height,
                format,
            };
            let mut result = MaybeUninit::<sys::GfxReadPixelsResult>::zeroed();
            let func = self.api.ReadPixels.expect("ReadPixels function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                },
                result.components,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn create_rbo(&self, xsize: i32, ysize: i32, target: u32, format: u32, samples: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxRBOCreateQuery {
                xsize,
                ysize,
                target,
                format,
                samples,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.CreateRBO.expect("CreateRBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn delete_rbo(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DeleteRBO.expect("DeleteRBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_rboinfo(&self, rbo_id: u32) -> Result<GetRBOInfoValue, Error> {
        unsafe {
            let query = sys::GfxRBOInfoQuery {
                rboID: rbo_id,
            };
            let mut result = MaybeUninit::<sys::GfxRBOInfoResult>::zeroed();
            let func = self.api.GetRBOInfo.expect("GetRBOInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.valid,
                result.target,
                result.format,
                result.xsize,
                result.ysize,
                result.samples,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn create_fbo(&self, target: u32, attachments: &[sys::GfxFBOAttachment], draw_buffers: &[u32], read_buffer: u32) -> Result<(u32, u32), Error> {
        unsafe {
            let query = sys::GfxFBOCreateQuery {
                target,
                attachments: attachments.as_ptr(),
                attachmentCount: attachments.len() as u32,
                drawBuffers: draw_buffers.as_ptr(),
                drawBufferCount: draw_buffers.len() as u32,
                readBuffer: read_buffer,
            };
            let mut result = MaybeUninit::<sys::GfxFBOResult>::zeroed();
            let func = self.api.CreateFBO.expect("CreateFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.fboID,
                result.rawID,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn set_fboattachment(&self, fbo_id: u32, attachment: u32, texture_name: &str, texture_target: u32, mip_level: i32, rbo_id: u32, use_rbo: bool) -> Result<(), Error> {
        unsafe {
            let texture_name_cstr = std::ffi::CString::new(texture_name).map_err(|_| Error::invalid_argument("texture_name"))?;
            let query = sys::GfxFBOAttachmentQuery {
                fboID: fbo_id,
                attachment,
                textureName: texture_name_cstr.as_ptr(),
                textureTarget: texture_target,
                mipLevel: mip_level,
                rboID: rbo_id,
                useRBO: use_rbo,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SetFBOAttachment.expect("SetFBOAttachment function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn set_fbodraw_buffers(&self, fbo_id: u32, buffers: &[u32]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFBODrawBuffersQuery {
                fboID: fbo_id,
                buffers: buffers.as_ptr(),
                bufferCount: buffers.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SetFBODrawBuffers.expect("SetFBODrawBuffers function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn set_fboread_buffer(&self, fbo_id: u32, buffer: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFBOReadBufferQuery {
                fboID: fbo_id,
                buffer,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SetFBOReadBuffer.expect("SetFBOReadBuffer function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn delete_fbo(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DeleteFBO.expect("DeleteFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn is_valid_fbo(&self, fbo_id: u32, target: u32) -> Result<(bool, u32), Error> {
        unsafe {
            let query = sys::GfxFBOQuery {
                fboID: fbo_id,
                target,
            };
            let mut result = MaybeUninit::<sys::GfxFBOStatusResult>::zeroed();
            let func = self.api.IsValidFBO.expect("IsValidFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.valid,
                result.status,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn active_fbo<F: FnMut()>(&self, fbo_id: u32, target: u32, identities: bool, mut callback: F) -> Result<(), Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxActiveFBOQuery {
                fboID: fbo_id,
                target,
                identities,
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ActiveFBO.expect("ActiveFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn raw_bind_fbo(&self, bind_default: bool, fbo_id: u32, target: u32, raw_fbo_id: u32) -> Result<(u32, bool), Error> {
        unsafe {
            let query = sys::GfxRawBindFBOQuery {
                bindDefault: bind_default,
                fboID: fbo_id,
                target,
                rawFboID: raw_fbo_id,
            };
            let mut result = MaybeUninit::<sys::GfxRawBindFBOResult>::zeroed();
            let func = self.api.RawBindFBO.expect("RawBindFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.previouslyBoundRawFboID,
                result.hasPrevious,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn blit_fbo(&self, src_fboid: u32, dst_fboid: u32, x0_src: i32, y0_src: i32, x1_src: i32, y1_src: i32, x0_dst: i32, y0_dst: i32, x1_dst: i32, y1_dst: i32, mask: u32, filter: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBlitFBOQuery {
                srcFBOID: src_fboid,
                dstFBOID: dst_fboid,
                x0Src: x0_src,
                y0Src: y0_src,
                x1Src: x1_src,
                y1Src: y1_src,
                x0Dst: x0_dst,
                y0Dst: y0_dst,
                x1Dst: x1_dst,
                y1Dst: y1_dst,
                mask,
                filter,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BlitFBO.expect("BlitFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn clear_attachment_fbo(&self, target: u32, attachment: u32, values: [f32; 4], count: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GfxClearAttachmentFBOQuery {
                target,
                attachment,
                values,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.ClearAttachmentFBO.expect("ClearAttachmentFBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_vao(&self) -> Result<(u32, u32), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxVAOResult>::zeroed();
            let func = self.api.GetVAO.expect("GetVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.vaoID,
                result.rawID,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn delete_vao(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DeleteVAO.expect("DeleteVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn attach_vertex_buffer_vao(&self, vao_id: u32, vbo_id: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVAOBufferQuery {
                vaoID: vao_id,
                vboID: vbo_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.AttachVertexBufferVAO.expect("AttachVertexBufferVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn attach_instance_buffer_vao(&self, vao_id: u32, vbo_id: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVAOBufferQuery {
                vaoID: vao_id,
                vboID: vbo_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.AttachInstanceBufferVAO.expect("AttachInstanceBufferVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn attach_index_buffer_vao(&self, vao_id: u32, vbo_id: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVAOBufferQuery {
                vaoID: vao_id,
                vboID: vbo_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.AttachIndexBufferVAO.expect("AttachIndexBufferVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn draw_arrays_vao(&self, vao_id: u32, mode: u32, vertex_count: i32, vertex_first: i32, instance_count: i32, instance_first: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVAODrawArraysQuery {
                vaoID: vao_id,
                mode,
                vertexCount: vertex_count,
                vertexFirst: vertex_first,
                instanceCount: instance_count,
                instanceFirst: instance_first,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DrawArraysVAO.expect("DrawArraysVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn draw_elements_vao(&self, vao_id: u32, mode: u32, draw_count: i32, base_index: i32, instance_count: i32, base_vertex: i32, base_instance: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVAODrawElementsQuery {
                vaoID: vao_id,
                mode,
                drawCount: draw_count,
                baseIndex: base_index,
                instanceCount: instance_count,
                baseVertex: base_vertex,
                baseInstance: base_instance,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DrawElementsVAO.expect("DrawElementsVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn clear_submission_vao(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ClearSubmissionVAO.expect("ClearSubmissionVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn add_units_to_submission_vao(&self, vao_id: u32, ids: &[u32]) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVAOSubmissionQuery {
                vaoID: vao_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.AddUnitsToSubmissionVAO.expect("AddUnitsToSubmissionVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn add_features_to_submission_vao(&self, vao_id: u32, ids: &[u32]) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVAOSubmissionQuery {
                vaoID: vao_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.AddFeaturesToSubmissionVAO.expect("AddFeaturesToSubmissionVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn add_unit_defs_to_submission_vao(&self, vao_id: u32, ids: &[u32]) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVAOSubmissionQuery {
                vaoID: vao_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.AddUnitDefsToSubmissionVAO.expect("AddUnitDefsToSubmissionVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn add_feature_defs_to_submission_vao(&self, vao_id: u32, ids: &[u32]) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVAOSubmissionQuery {
                vaoID: vao_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.AddFeatureDefsToSubmissionVAO.expect("AddFeatureDefsToSubmissionVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn remove_from_submission_vao(&self, vao_id: u32, index: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVAORemoveSubmissionQuery {
                vaoID: vao_id,
                index,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.RemoveFromSubmissionVAO.expect("RemoveFromSubmissionVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn submit_vao(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SubmitVAO.expect("SubmitVAO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_vbo(&self, target: u32, freq_updated: bool) -> Result<(u32, u32, u32), Error> {
        unsafe {
            let query = sys::GfxVBOQuery {
                target,
                freqUpdated: freq_updated,
            };
            let mut result = MaybeUninit::<sys::GfxVBOResult>::zeroed();
            let func = self.api.GetVBO.expect("GetVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.vboID,
                result.rawID,
                result.target,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn delete_vbo(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DeleteVBO.expect("DeleteVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn define_vbo(&self, vbo_id: u32, elements_count: i32, element_array: bool, index_type: u32, use_default_attributes: bool, default_attribute_count: u32, attributes: &[sys::GfxVBOAttributeOptions]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVBODefineQuery {
                vboID: vbo_id,
                elementsCount: elements_count,
                elementArray: element_array,
                indexType: index_type,
                useDefaultAttributes: use_default_attributes,
                defaultAttributeCount: default_attribute_count,
                attributes: attributes.as_ptr(),
                attributeCount: attributes.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DefineVBO.expect("DefineVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_vboinfo(&self, vbo_id: u32) -> Result<GetVBOInfoValue, Error> {
        unsafe {
            let query = sys::GfxVBOInfoQuery {
                vboID: vbo_id,
            };
            let mut result = MaybeUninit::<sys::GfxVBOInfoResult>::zeroed();
            let func = self.api.GetVBOInfo.expect("GetVBOInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.elementsCount,
                result.bufferSizeInBytes,
                result.gpuBufferSizeInBytes,
                result.elemSizeInBytes,
                result.attributesCount,
                result.primitiveRestartIndex,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn upload_vbo(&self, vbo_id: u32, data: &[f32], attribute_index: i32, element_offset: i32, data_start_index: i32, data_finish_index: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVBOUploadQuery {
                vboID: vbo_id,
                data: data.as_ptr(),
                dataCount: data.len() as u32,
                attributeIndex: attribute_index,
                elementOffset: element_offset,
                dataStartIndex: data_start_index,
                dataFinishIndex: data_finish_index,
            };
            let mut result = MaybeUninit::<sys::GfxVBOUploadResult>::zeroed();
            let func = self.api.UploadVBO.expect("UploadVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.bytesWritten
            })
        }
    }

    pub fn download_vbo(&self, vbo_id: u32, attribute_index: i32, element_offset: i32, element_count: i32, force_gpuread: bool) -> Result<Vec<f32>, Error> {
        unsafe {
            let query = sys::GfxVBODownloadQuery {
                vboID: vbo_id,
                attributeIndex: attribute_index,
                elementOffset: element_offset,
                elementCount: element_count,
                forceGPURead: force_gpuread,
            };
            let mut result = MaybeUninit::<sys::GfxVBODownloadResult>::zeroed();
            let func = self.api.DownloadVBO.expect("DownloadVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn clear_vbo(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ClearVBO.expect("ClearVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn models_vbo(&self, value: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.ModelsVBO.expect("ModelsVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn instance_data_from_unit_defs_vbo(&self, vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVBOInstanceDataQuery {
                vboID: vbo_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
                attributeIndex: attribute_index,
                teamID: team_id,
                elementOffset: element_offset,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.InstanceDataFromUnitDefsVBO.expect("InstanceDataFromUnitDefsVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn instance_data_from_feature_defs_vbo(&self, vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVBOInstanceDataQuery {
                vboID: vbo_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
                attributeIndex: attribute_index,
                teamID: team_id,
                elementOffset: element_offset,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.InstanceDataFromFeatureDefsVBO.expect("InstanceDataFromFeatureDefsVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn instance_data_from_units_vbo(&self, vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVBOInstanceDataQuery {
                vboID: vbo_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
                attributeIndex: attribute_index,
                teamID: team_id,
                elementOffset: element_offset,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.InstanceDataFromUnitsVBO.expect("InstanceDataFromUnitsVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn instance_data_from_features_vbo(&self, vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVBOInstanceDataQuery {
                vboID: vbo_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
                attributeIndex: attribute_index,
                teamID: team_id,
                elementOffset: element_offset,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.InstanceDataFromFeaturesVBO.expect("InstanceDataFromFeaturesVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn matrix_data_from_projectiles_vbo(&self, vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxVBOInstanceDataQuery {
                vboID: vbo_id,
                ids: ids.as_ptr(),
                idCount: ids.len() as u32,
                attributeIndex: attribute_index,
                teamID: team_id,
                elementOffset: element_offset,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.MatrixDataFromProjectilesVBO.expect("MatrixDataFromProjectilesVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn bind_buffer_range_vbo(&self, vbo_id: u32, binding_index: u32, element_offset: i32, element_count: i32, target: u32, bind: bool) -> Result<i32, Error> {
        unsafe {
            let query = sys::GfxVBOBindRangeQuery {
                vboID: vbo_id,
                bindingIndex: binding_index,
                elementOffset: element_offset,
                elementCount: element_count,
                target,
                bind,
            };
            let mut result = MaybeUninit::<sys::GfxIntResult>::zeroed();
            let func = self.api.BindBufferRangeVBO.expect("BindBufferRangeVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn unbind_buffer_range_vbo(&self, vbo_id: u32, binding_index: u32, element_offset: i32, element_count: i32, target: u32, bind: bool) -> Result<i32, Error> {
        unsafe {
            let query = sys::GfxVBOBindRangeQuery {
                vboID: vbo_id,
                bindingIndex: binding_index,
                elementOffset: element_offset,
                elementCount: element_count,
                target,
                bind,
            };
            let mut result = MaybeUninit::<sys::GfxIntResult>::zeroed();
            let func = self.api.UnbindBufferRangeVBO.expect("UnbindBufferRangeVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn dump_definition_vbo(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DumpDefinitionVBO.expect("DumpDefinitionVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn copy_to_vbo(&self, source_vboid: u32, destination_vboid: u32, copy_size_in_bytes: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GfxVBOCopyQuery {
                sourceVBOID: source_vboid,
                destinationVBOID: destination_vboid,
                copySizeInBytes: copy_size_in_bytes,
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.CopyToVBO.expect("CopyToVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_idvbo(&self, value: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.GetIDVBO.expect("GetIDVBO function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn render_to_texture<F: FnMut()>(&self, name: &str, mut callback: F) -> Result<(), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxRenderToTextureQuery {
                name: name_cstr.as_ptr(),
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.RenderToTexture.expect("RenderToTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn create_texture_atlas(&self, xsize: i32, ysize: i32, alloc_type: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GfxCreateTextureAtlasQuery {
                xsize,
                ysize,
                allocType: alloc_type,
            };
            let mut result = MaybeUninit::<sys::GfxStringResult>::zeroed();
            let func = self.api.CreateTextureAtlas.expect("CreateTextureAtlas function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.value.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.value).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn finalize_texture_atlas(&self, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.FinalizeTextureAtlas.expect("FinalizeTextureAtlas function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn delete_texture_atlas(&self, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.DeleteTextureAtlas.expect("DeleteTextureAtlas function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn add_atlas_texture(&self, atlas_name: &str, texture_name: &str) -> Result<(), Error> {
        unsafe {
            let atlas_name_cstr = std::ffi::CString::new(atlas_name).map_err(|_| Error::invalid_argument("atlas_name"))?;
            let texture_name_cstr = std::ffi::CString::new(texture_name).map_err(|_| Error::invalid_argument("texture_name"))?;
            let query = sys::GfxAtlasTextureQuery {
                atlasName: atlas_name_cstr.as_ptr(),
                textureName: texture_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.AddAtlasTexture.expect("AddAtlasTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_atlas_texture(&self, atlas_name: &str, texture_name: &str) -> Result<GetAtlasTextureValue, Error> {
        unsafe {
            let atlas_name_cstr = std::ffi::CString::new(atlas_name).map_err(|_| Error::invalid_argument("atlas_name"))?;
            let texture_name_cstr = std::ffi::CString::new(texture_name).map_err(|_| Error::invalid_argument("texture_name"))?;
            let query = sys::GfxAtlasTextureQuery {
                atlasName: atlas_name_cstr.as_ptr(),
                textureName: texture_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxAtlasTextureResult>::zeroed();
            let func = self.api.GetAtlasTexture.expect("GetAtlasTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.x1,
                result.x2,
                result.y1,
                result.y2,
                result.pageNum,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_engine_atlas_textures(&self, name: &str) -> Result<Vec<sys::GfxAtlasTextureEntry>, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GfxTextureNameQuery {
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxAtlasTexturesResult>::zeroed();
            let func = self.api.GetEngineAtlasTextures.expect("GetEngineAtlasTextures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn save_image(&self, x: i32, y: i32, width: i32, height: i32, filename: &str, options: GfxSaveImageOptions, read_buffer: u32) -> Result<bool, Error> {
        unsafe {
            let filename_cstr = std::ffi::CString::new(filename).map_err(|_| Error::invalid_argument("filename"))?;
            let query = sys::GfxSaveImageQuery {
                x,
                y,
                width,
                height,
                filename: filename_cstr.as_ptr(),
                options: options.into(),
                readBuffer: read_buffer,
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.SaveImage.expect("SaveImage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn create_list<F: FnMut()>(&self, mut callback: F) -> Result<u32, Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxCallbackQuery {
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.CreateList.expect("CreateList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn call_list(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.CallList.expect("CallList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn delete_list(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DeleteList.expect("DeleteList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn create_query(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.CreateQuery.expect("CreateQuery function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn delete_query(&self, value: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DeleteQuery.expect("DeleteQuery function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn run_query<F: FnMut()>(&self, id: u32, mut callback: F) -> Result<(), Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxRunQueryQuery {
                id,
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.RunQuery.expect("RunQuery function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_query(&self, value: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GfxUIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxUIntResult>::zeroed();
            let func = self.api.GetQuery.expect("GetQuery function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_global_tex_names(&self) -> Result<Vec<sys::GfxAtlasTextureEntry>, Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxAtlasTexturesResult>::zeroed();
            let func = self.api.GetGlobalTexNames.expect("GetGlobalTexNames function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_global_tex_coords(&self, value: &str) -> Result<GetGlobalTexCoordsValue, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::GfxStringQuery {
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxAtlasTextureResult>::zeroed();
            let func = self.api.GetGlobalTexCoords.expect("GetGlobalTexCoords function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.x1,
                result.x2,
                result.y1,
                result.y2,
                result.pageNum,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn begin_text(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BeginText.expect("BeginText function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn text(&self, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<(), Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let options_cstr = std::ffi::CString::new(options).map_err(|_| Error::invalid_argument("options"))?;
            let query = sys::GfxTextQuery {
                text: text_cstr.as_ptr(),
                x,
                y,
                size,
                options: options_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Text.expect("Text function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn end_text(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.EndText.expect("EndText function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_text_width(&self, value: &str) -> Result<f32, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::GfxStringQuery {
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxFloatResult>::zeroed();
            let func = self.api.GetTextWidth.expect("GetTextWidth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn get_text_height(&self, value: &str) -> Result<(f32, f32, i32), Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::GfxStringQuery {
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxTextHeightResult>::zeroed();
            let func = self.api.GetTextHeight.expect("GetTextHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.height,
                result.descender,
                result.lines,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn add_fallback_font(&self, value: &str) -> Result<bool, Error> {
        unsafe {
            let value_cstr = std::ffi::CString::new(value).map_err(|_| Error::invalid_argument("value"))?;
            let query = sys::GfxStringQuery {
                value: value_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxBoolResult>::zeroed();
            let func = self.api.AddFallbackFont.expect("AddFallbackFont function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn clear_fallback_fonts(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ClearFallbackFonts.expect("ClearFallbackFonts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn load_font(&self, path: &str, size: i32, outline_width: i32, outline_weight: f32) -> Result<u32, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::GfxLoadFontQuery {
                path: path_cstr.as_ptr(),
                size,
                outlineWidth: outline_width,
                outlineWeight: outline_weight,
            };
            let mut result = MaybeUninit::<sys::GfxFontResult>::zeroed();
            let func = self.api.LoadFont.expect("LoadFont function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.fontID
            })
        }
    }

    pub fn delete_font(&self, font_id: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontQuery {
                fontID: font_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DeleteFont.expect("DeleteFont function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_font_info(&self, font_id: u32) -> Result<GetFontInfoValue, Error> {
        unsafe {
            let query = sys::GfxFontQuery {
                fontID: font_id,
            };
            let mut result = MaybeUninit::<sys::GfxFontInfoResult>::zeroed();
            let func = self.api.GetFontInfo.expect("GetFontInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.path.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.path).to_string_lossy().into_owned())
                    }
                },
                {
                    if result.family.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.family).to_string_lossy().into_owned())
                    }
                },
                {
                    if result.style.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.style).to_string_lossy().into_owned())
                    }
                },
                result.size,
                result.lineHeight,
                result.descender,
                result.outlineWidth,
                result.outlineWeight,
                result.textureWidth,
                result.textureHeight,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn font_begin(&self, font_id: u32, user_defined_blending: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontBeginQuery {
                fontID: font_id,
                userDefinedBlending: user_defined_blending,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontBegin.expect("FontBegin function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_end(&self, font_id: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontQuery {
                fontID: font_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontEnd.expect("FontEnd function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_print(&self, font_id: u32, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<(), Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let options_cstr = std::ffi::CString::new(options).map_err(|_| Error::invalid_argument("options"))?;
            let query = sys::GfxFontTextQuery {
                fontID: font_id,
                text: text_cstr.as_ptr(),
                x,
                y,
                size,
                options: options_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontPrint.expect("FontPrint function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_print_world(&self, font_id: u32, text: &str, pos: sys::Float3, size: f32, options: &str) -> Result<(), Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let options_cstr = std::ffi::CString::new(options).map_err(|_| Error::invalid_argument("options"))?;
            let query = sys::GfxFontWorldTextQuery {
                fontID: font_id,
                text: text_cstr.as_ptr(),
                pos,
                size,
                options: options_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontPrintWorld.expect("FontPrintWorld function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_submit_buffered(&self, font_id: u32, options: GfxFontSubmitBufferedOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontSubmitBufferedQuery {
                fontID: font_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontSubmitBuffered.expect("FontSubmitBuffered function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_wrap_text(&self, font_id: u32, text: &str, max_width: f32, max_height: f32, size: f32) -> Result<(Option<String>, i32), Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::GfxFontWrapTextQuery {
                fontID: font_id,
                text: text_cstr.as_ptr(),
                maxWidth: max_width,
                maxHeight: max_height,
                size,
            };
            let mut result = MaybeUninit::<sys::GfxFontWrapTextResult>::zeroed();
            let func = self.api.FontWrapText.expect("FontWrapText function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.text).to_string_lossy().into_owned())
                    }
                },
                result.lines,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn font_get_text_width(&self, font_id: u32, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<f32, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let options_cstr = std::ffi::CString::new(options).map_err(|_| Error::invalid_argument("options"))?;
            let query = sys::GfxFontTextQuery {
                fontID: font_id,
                text: text_cstr.as_ptr(),
                x,
                y,
                size,
                options: options_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxFloatResult>::zeroed();
            let func = self.api.FontGetTextWidth.expect("FontGetTextWidth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn font_get_text_height(&self, font_id: u32, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<(f32, f32, i32), Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let options_cstr = std::ffi::CString::new(options).map_err(|_| Error::invalid_argument("options"))?;
            let query = sys::GfxFontTextQuery {
                fontID: font_id,
                text: text_cstr.as_ptr(),
                x,
                y,
                size,
                options: options_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxTextHeightResult>::zeroed();
            let func = self.api.FontGetTextHeight.expect("FontGetTextHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.height,
                result.descender,
                result.lines,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn font_set_text_color(&self, font_id: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontColorQuery {
                fontID: font_id,
                r,
                g,
                b,
                a,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontSetTextColor.expect("FontSetTextColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_set_outline_color(&self, font_id: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontColorQuery {
                fontID: font_id,
                r,
                g,
                b,
                a,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontSetOutlineColor.expect("FontSetOutlineColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_set_auto_outline_color(&self, font_id: u32, enable: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontAutoOutlineColorQuery {
                fontID: font_id,
                enable,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontSetAutoOutlineColor.expect("FontSetAutoOutlineColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn font_bind_texture(&self, font_id: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFontQuery {
                fontID: font_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FontBindTexture.expect("FontBindTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn begin_end<F: FnMut()>(&self, primitive: u32, mut callback: F) -> Result<(), Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxBeginEndQuery {
                primitive,
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.BeginEnd.expect("BeginEnd function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn push_pop_matrix<F: FnMut()>(&self, mut callback: F) -> Result<(), Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxCallbackQuery {
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PushPopMatrix.expect("PushPopMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unsafe_state<F: FnMut()>(&self, state: u32, reverse: bool, mut callback: F) -> Result<(), Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxUnsafeStateQuery {
                state,
                reverse,
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnsafeState.expect("UnsafeState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn draw_ground_circle(&self, pos: sys::Float3, radius: f32, resolution: i32, ballistic: bool, slope: f32, gravity: f32, weapon_def_id: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxGroundCircleQuery {
                pos,
                radius,
                resolution,
                ballistic,
                slope,
                gravity,
                weaponDefID: weapon_def_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DrawGroundCircle.expect("DrawGroundCircle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn draw_ground_quad(&self, x0: f32, z0: f32, x1: f32, z1: f32, use_tex_coords: bool, tu0: f32, tv0: f32, tu1: f32, tv1: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxGroundQuadQuery {
                x0,
                z0,
                x1,
                z1,
                useTexCoords: use_tex_coords,
                tu0,
                tv0,
                tu1,
                tv1,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DrawGroundQuad.expect("DrawGroundQuad function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_fixed_state(&self, param: &str) -> Result<GetFixedStateValue, Error> {
        unsafe {
            let param_cstr = std::ffi::CString::new(param).map_err(|_| Error::invalid_argument("param"))?;
            let query = sys::GfxFixedStateQuery {
                param: param_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GfxFixedStateResult>::zeroed();
            let func = self.api.GetFixedState.expect("GetFixedState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.bools,
                result.boolCount,
                result.ints,
                result.intCount,
                result.floats,
                result.floatCount,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_screen_view_trans(&self) -> Result<(f32, f32, f32), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxTranslateResult>::zeroed();
            let func = self.api.GetScreenViewTrans.expect("GetScreenViewTrans function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.x,
                result.y,
                result.z,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn slave_mini_map(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SlaveMiniMap.expect("SlaveMiniMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn config_mini_map(&self, px: i32, py: i32, sx: i32, sy: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMiniMapConfigQuery {
                px,
                py,
                sx,
                sy,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.ConfigMiniMap.expect("ConfigMiniMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn draw_mini_map(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DrawMiniMap.expect("DrawMiniMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit(&self, unit_id: i32, options: GfxUnitDrawOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUnitDrawQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Unit.expect("Unit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_raw(&self, unit_id: i32, options: GfxUnitDrawOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxUnitDrawQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitRaw.expect("UnitRaw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_textures(&self, object_id: i32, push: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectTextureStateQuery {
                objectID: object_id,
                push,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitTextures.expect("UnitTextures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_shape(&self, def_id: i32, team_id: i32, options: GfxObjectShapeOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectShapeQuery {
                defID: def_id,
                teamID: team_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitShape.expect("UnitShape function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_shape_textures(&self, object_id: i32, push: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectTextureStateQuery {
                objectID: object_id,
                push,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitShapeTextures.expect("UnitShapeTextures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_mult_matrix(&self, value: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitMultMatrix.expect("UnitMultMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_piece(&self, object_id: i32, piece_id: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectPieceQuery {
                objectID: object_id,
                pieceID: piece_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitPiece.expect("UnitPiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_piece_matrix(&self, object_id: i32, piece_id: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectPieceQuery {
                objectID: object_id,
                pieceID: piece_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitPieceMatrix.expect("UnitPieceMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn unit_piece_mult_matrix(&self, object_id: i32, piece_id: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectPieceQuery {
                objectID: object_id,
                pieceID: piece_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.UnitPieceMultMatrix.expect("UnitPieceMultMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature(&self, feature_id: i32, options: GfxFeatureDrawOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFeatureDrawQuery {
                featureID: feature_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Feature.expect("Feature function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_raw(&self, feature_id: i32, options: GfxFeatureDrawOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFeatureDrawQuery {
                featureID: feature_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeatureRaw.expect("FeatureRaw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_textures(&self, object_id: i32, push: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectTextureStateQuery {
                objectID: object_id,
                push,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeatureTextures.expect("FeatureTextures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_shape(&self, def_id: i32, team_id: i32, options: GfxObjectShapeOptions) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectShapeQuery {
                defID: def_id,
                teamID: team_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeatureShape.expect("FeatureShape function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_shape_textures(&self, object_id: i32, push: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectTextureStateQuery {
                objectID: object_id,
                push,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeatureShapeTextures.expect("FeatureShapeTextures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_mult_matrix(&self, value: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxIntQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeatureMultMatrix.expect("FeatureMultMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_piece(&self, object_id: i32, piece_id: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectPieceQuery {
                objectID: object_id,
                pieceID: piece_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeaturePiece.expect("FeaturePiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_piece_matrix(&self, object_id: i32, piece_id: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectPieceQuery {
                objectID: object_id,
                pieceID: piece_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeaturePieceMatrix.expect("FeaturePieceMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn feature_piece_mult_matrix(&self, object_id: i32, piece_id: i32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxObjectPieceQuery {
                objectID: object_id,
                pieceID: piece_id,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FeaturePieceMultMatrix.expect("FeaturePieceMultMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn draw_list_at_unit(&self, unit_id: i32, list_id: u32, use_mid_pos: bool, scale: sys::Float3, degrees: f32, rot: sys::Float3) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxDrawListAtUnitQuery {
                unitID: unit_id,
                listID: list_id,
                useMidPos: use_mid_pos,
                scale,
                degrees,
                rot,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DrawListAtUnit.expect("DrawListAtUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn draw_func_at_unit<F: FnMut()>(&self, unit_id: i32, use_mid_pos: bool, mut callback: F) -> Result<(), Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::GfxDrawFuncAtUnitQuery {
                unitID: unit_id,
                useMidPos: use_mid_pos,
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.DrawFuncAtUnit.expect("DrawFuncAtUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn matrix_mode(&self, mode: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMatrixModeQuery {
                mode,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.MatrixMode.expect("MatrixMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn load_identity(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.LoadIdentity.expect("LoadIdentity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn load_matrix(&self, values: [f32; 16]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMatrixQuery {
                values,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.LoadMatrix.expect("LoadMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn mult_matrix(&self, values: [f32; 16]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMatrixQuery {
                values,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.MultMatrix.expect("MultMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn push_matrix(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PushMatrix.expect("PushMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn pop_matrix(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.PopMatrix.expect("PopMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTranslateQuery {
                x,
                y,
                z,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Translate.expect("Translate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxScaleQuery {
                x,
                y,
                z,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Scale.expect("Scale function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn rotate(&self, degrees: f32, x: f32, y: f32, z: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxRotateQuery {
                degrees,
                x,
                y,
                z,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Rotate.expect("Rotate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn ortho(&self, left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxOrthoQuery {
                left,
                right,
                bottom,
                top,
                nearVal: near_val,
                farVal: far_val,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Ortho.expect("Ortho function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn frustum(&self, left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFrustumQuery {
                left,
                right,
                bottom,
                top,
                nearVal: near_val,
                farVal: far_val,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Frustum.expect("Frustum function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn get_matrix_data(&self, mode: u32) -> Result<[f32; 16], Error> {
        unsafe {
            let query = sys::GfxGetMatrixDataQuery {
                mode,
            };
            let mut result = MaybeUninit::<sys::GfxGetMatrixDataResult>::zeroed();
            let func = self.api.GetMatrixData.expect("GetMatrixData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.values
            })
        }
    }

    pub fn vertex(&self, x: f32, y: f32, z: f32, w: f32, count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVertexQuery {
                x,
                y,
                z,
                w,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Vertex.expect("Vertex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn normal(&self, x: f32, y: f32, z: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTranslateQuery {
                x,
                y,
                z,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Normal.expect("Normal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn tex_coord(&self, x: f32, y: f32, z: f32, w: f32, count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxVertexQuery {
                x,
                y,
                z,
                w,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.TexCoord.expect("TexCoord function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn multi_tex_coord(&self, tex_num: i32, s: f32, t: f32, r: f32, q: f32, count: u32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxMultiTexCoordQuery {
                texNum: tex_num,
                s,
                t,
                r,
                q,
                count,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.MultiTexCoord.expect("MultiTexCoord function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn color(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxColorQuery {
                r,
                g,
                b,
                a,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Color.expect("Color function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn secondary_color(&self, x: f32, y: f32, z: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTranslateQuery {
                x,
                y,
                z,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.SecondaryColor.expect("SecondaryColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn fog_coord(&self, value: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxFloatQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.FogCoord.expect("FogCoord function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn edge_flag(&self, value: bool) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxBoolQuery {
                value,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.EdgeFlag.expect("EdgeFlag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn rect(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxRectQuery {
                x1,
                y1,
                x2,
                y2,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Rect.expect("Rect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn tex_rect(&self, x1: f32, y1: f32, x2: f32, y2: f32, s1: f32, t1: f32, s2: f32, t2: f32) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxTexRectQuery {
                x1,
                y1,
                x2,
                y2,
                s1,
                t1,
                s2,
                t2,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.TexRect.expect("TexRect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn shape(&self, primitive: u32, vertices: &[sys::GfxVertexData]) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxShapeQuery {
                primitive,
                vertices: vertices.as_ptr(),
                vertexCount: vertices.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Shape.expect("Shape function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn billboard(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::GfxEmptyQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GfxEmptyResult>::zeroed();
            let func = self.api.Billboard.expect("Billboard function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

}

    pub mod unsynced_ctrl {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnsyncedRandomApi {
            pub next_float: u32,
            pub next_int_up_to: u32,
            pub next_int: u32,
            pub set_seed: u32,
        }

        pub use super::types::{AssignMouseCursorQuery, AssignMouseCursorResult, AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, DeselectUnitMapQuery, DeselectUnitMapResult, DrawUnitCommandsQuery, DrawUnitCommandsResult, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, ForceLayoutUpdateQuery, ForceLayoutUpdateResult, ForceTesselationUpdateQuery, ForceTesselationUpdateResult, GetWaterTextureQuery, GetWaterTextureResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, Int2, Int3, Int32Array, Int32Result, LoadCmdColorsConfigQuery, LoadCmdColorsConfigResult, LoadCtrlPanelConfigQuery, LoadCtrlPanelConfigResult, LoadModelTexturesQuery, LoadModelTexturesResult, MapRenderingParams, NativeCommand, NativeExplosionParams, NativeProjectileParams, NextFloatQuery, NextFloatResult, NextIntQuery, NextIntResult, NextIntUpToQuery, NextIntUpToResult, NumberOrBool, PauseDollyCameraQuery, PauseDollyCameraResult, PreloadFeatureDefModelQuery, PreloadFeatureDefModelResult, PreloadUnitDefModelQuery, PreloadUnitDefModelResult, ProjectileTargetRef, ReplaceMouseCursorQuery, ReplaceMouseCursorResult, ResourcePack, ResumeDollyCameraQuery, ResumeDollyCameraResult, RgbColor, RunDollyCameraQuery, RunDollyCameraResult, SDLSetTextInputRectQuery, SDLSetTextInputRectResult, SDLStartTextInputQuery, SDLStartTextInputResult, SDLStopTextInputQuery, SDLStopTextInputResult, SelectUnitMapQuery, SelectUnitMapResult, SetActiveCommandOptions, SetActiveCommandQuery, SetActiveCommandResult, SetAtmosphereQuery, SetAtmosphereResult, SetAutoShowMetalQuery, SetAutoShowMetalResult, SetBoxSelectionByEngineQuery, SetBoxSelectionByEngineResult, SetBuildFacingQuery, SetBuildFacingResult, SetBuildSpacingQuery, SetBuildSpacingResult, SetCameraOffsetQuery, SetCameraOffsetResult, SetClipboardQuery, SetClipboardResult, SetCustomCommandDrawDataQuery, SetCustomCommandDrawDataResult, SetCustomPaletteColorQuery, SetCustomPaletteColorResult, SetDefaultInterfaceVisibleQuery, SetDefaultInterfaceVisibleResult, SetDollyCameraCurveQuery, SetDollyCameraCurveResult, SetDollyCameraLookCurveQuery, SetDollyCameraLookCurveResult, SetDollyCameraLookPositionQuery, SetDollyCameraLookPositionResult, SetDollyCameraLookUnitQuery, SetDollyCameraLookUnitResult, SetDollyCameraModeQuery, SetDollyCameraModeResult, SetDollyCameraPositionQuery, SetDollyCameraPositionResult, SetDollyCameraRelativeModeQuery, SetDollyCameraRelativeModeResult, SetDrawGroundDeferredQuery, SetDrawGroundDeferredResult, SetDrawGroundQuery, SetDrawGroundResult, SetDrawModelsDeferredQuery, SetDrawModelsDeferredResult, SetDrawSelectionInfoQuery, SetDrawSelectionInfoResult, SetDrawSkyQuery, SetDrawSkyResult, SetDrawWaterQuery, SetDrawWaterResult, SetEngineBuildSquareRenderingQuery, SetEngineBuildSquareRenderingResult, SetFeatureAlwaysUpdateMatrixQuery, SetFeatureAlwaysUpdateMatrixResult, SetFeatureEngineDrawMaskQuery, SetFeatureEngineDrawMaskResult, SetFeatureFadeQuery, SetFeatureFadeResult, SetFeatureLuaDrawQuery, SetFeatureLuaDrawResult, SetFeatureNoDrawQuery, SetFeatureNoDrawResult, SetFeaturePaletteIndexQuery, SetFeaturePaletteIndexResult, SetLastMessagePositionQuery, SetLastMessagePositionResult, SetLosViewColorsQuery, SetLosViewColorsResult, SetMapRenderingParamsQuery, SetMapRenderingParamsResult, SetMapShaderQuery, SetMapShaderResult, SetMapShadingTextureQuery, SetMapShadingTextureResult, SetMiniMapRotationQuery, SetMiniMapRotationResult, SetMouseCursorQuery, SetMouseCursorResult, SetNanoProjectileParamsQuery, SetNanoProjectileParamsResult, SetProjectileLuaDrawQuery, SetProjectileLuaDrawResult, SetSeedQuery, SetSeedResult, SetShockFrontFactorsOptions, SetShockFrontFactorsQuery, SetShockFrontFactorsResult, SetSkyBoxTextureQuery, SetSkyBoxTextureResult, SetSunDirectionQuery, SetSunDirectionResult, SetSunLightingQuery, SetSunLightingResult, SetUnitAlwaysUpdateMatrixQuery, SetUnitAlwaysUpdateMatrixResult, SetUnitDefIconQuery, SetUnitDefIconResult, SetUnitDefImageQuery, SetUnitDefImageResult, SetUnitEngineDrawMaskQuery, SetUnitEngineDrawMaskResult, SetUnitIconDrawQuery, SetUnitIconDrawResult, SetUnitIconQuery, SetUnitIconResult, SetUnitLeaveTracksQuery, SetUnitLeaveTracksResult, SetUnitLuaDrawQuery, SetUnitLuaDrawResult, SetUnitNoDrawQuery, SetUnitNoDrawResult, SetUnitNoGroupQuery, SetUnitNoGroupResult, SetUnitNoMinimapQuery, SetUnitNoMinimapResult, SetUnitNoSelectQuery, SetUnitNoSelectResult, SetUnitPaletteIndexQuery, SetUnitPaletteIndexResult, SetVideoCapturingModeQuery, SetVideoCapturingModeResult, SetVideoCapturingTimeOffsetQuery, SetVideoCapturingTimeOffsetResult, SetWMCaptionQuery, SetWMCaptionResult, SetWMIconQuery, SetWMIconResult, SetWaterParamsQuery, SetWaterParamsResult, SetWaterTextureQuery, SetWaterTextureResult, SetWindowGeometryOptions, SetWindowGeometryQuery, SetWindowGeometryResult, SetWindowMaximizedQuery, SetWindowMaximizedResult, SetWindowMinimizedQuery, SetWindowMinimizedResult, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WarpMouseQuery, WarpMouseResult, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetDrawGroundDeferredValue {
            pub success: bool,
            pub deferred: bool,
            pub forward: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetDrawModelsDeferredValue {
            pub success: bool,
            pub units_deferred: bool,
            pub features_deferred: bool,
            pub units_forward: bool,
            pub features_forward: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMiniMapRotationValue {
            pub success: bool,
            pub rotation: i32,
        }

        #[inline]
        pub fn assign_mouse_cursor(command_name: &str, cursor_file_name: &str, overwrite: bool, hot_spot_top_left: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(command_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(command_name)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(cursor_file_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(cursor_file_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::assign_mouse_cursor(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr(), overwrite, hot_spot_top_left)
        }

        #[inline]
        pub fn deselect_unit_map(unit_i_ds: &[i32]) -> Result<bool> {
            crate::generated::borrowed::unsynced_ctrl::deselect_unit_map(unit_i_ds)
        }

        #[inline]
        pub fn draw_unit_commands(unit_i_ds: &[i32], table_or_array: bool, queue_draw_depth: i32) -> Result<bool> {
            crate::generated::borrowed::unsynced_ctrl::draw_unit_commands(unit_i_ds, table_or_array, queue_draw_depth)
        }

        #[inline]
        pub fn force_layout_update(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::force_layout_update(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn force_tesselation_update(normal: bool, shadow: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::force_tesselation_update(normal, shadow)?;
            Ok(value)
        }

        #[inline]
        pub fn get_water_texture(tex_type: &str) -> Result<String> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + tex_type.len()); __b.extend_from_slice(&(tex_type.len() as u32).to_le_bytes()); __b.extend_from_slice(tex_type.as_bytes()); __b };
            // Last result size: repeated queries fit on the first call, so the
            // native getter does not run a second time just to size the buffer.
            static __SIZE_HINT: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
            let mut __output = alloc::vec![0u8; __SIZE_HINT.load(core::sync::atomic::Ordering::Relaxed)];
            loop {
                match crate::generated::dynamic_input::unsynced_ctrl::get_water_texture(&__blob0, &mut __output) {
                    Ok(required) => {
                        __SIZE_HINT.store(required, core::sync::atomic::Ordering::Relaxed);
                        __output.truncate(required);
                        return String::from_utf8(__output)
                            .map_err(|_| crate::ApiError::new(crate::ErrorCode::Internal as i32));
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn load_cmd_colors_config(filename: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(filename, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(filename)?),
            };
            crate::generated::borrowed::unsynced_ctrl::load_cmd_colors_config(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn load_ctrl_panel_config(filename: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(filename, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(filename)?),
            };
            crate::generated::borrowed::unsynced_ctrl::load_ctrl_panel_config(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn load_model_textures(model_name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(model_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(model_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::load_model_textures(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn pause_dolly_camera(percent: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::pause_dolly_camera(percent)?;
            Ok(value)
        }

        #[inline]
        pub fn preload_feature_def_model(def_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::preload_feature_def_model(def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn preload_unit_def_model(def_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::preload_unit_def_model(def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn replace_mouse_cursor(old_cursor_file_name: &str, new_cursor_file_name: &str, hot_spot_top_left: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(old_cursor_file_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(old_cursor_file_name)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(new_cursor_file_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(new_cursor_file_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::replace_mouse_cursor(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr(), hot_spot_top_left)
        }

        #[inline]
        pub fn resume_dolly_camera(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::resume_dolly_camera(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn run_dolly_camera(runtime_ms: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::run_dolly_camera(runtime_ms)?;
            Ok(value)
        }

        #[inline]
        pub fn sdl_set_text_input_rect(x: i32, y: i32, w: i32, h: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::sdl_set_text_input_rect(x, y, w, h)?;
            Ok(value)
        }

        #[inline]
        pub fn sdl_start_text_input(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::sdl_start_text_input(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn sdl_stop_text_input(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::sdl_stop_text_input(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn select_unit_map(unit_i_ds: &[i32], append: bool) -> Result<bool> {
            crate::generated::borrowed::unsynced_ctrl::select_unit_map(unit_i_ds, append)
        }

        #[inline]
        pub fn set_active_command(cmd_index: i32, button: i32, options: SetActiveCommandOptions) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_active_command(cmd_index, button, crate::generated::unsynced_ctrl::SetActiveCommandOptions { left_click: options.left_click, right_click: options.right_click, alt: options.alt, ctrl: options.ctrl, meta: options.meta, shift: options.shift })?;
            Ok(value)
        }

        #[inline]
        pub fn set_atmosphere(params: AtmosphereParams) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_atmosphere(crate::generated::unsynced_ctrl::AtmosphereParams { fog_color: match params.fog_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, sky_color: match params.sky_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, sun_color: match params.sun_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, cloud_color: match params.cloud_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, sky_axis_angle: match params.sky_axis_angle { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, fog_start: params.fog_start, fog_end: params.fog_end })?;
            Ok(value)
        }

        #[inline]
        pub fn set_auto_show_metal(enable: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_auto_show_metal(enable)?;
            Ok(value)
        }

        #[inline]
        pub fn set_box_selection_by_engine(state: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_box_selection_by_engine(state)?;
            Ok(value)
        }

        #[inline]
        pub fn set_build_facing(facing: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_build_facing(facing)?;
            Ok(value)
        }

        #[inline]
        pub fn set_build_spacing(spacing: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_build_spacing(spacing)?;
            Ok(value)
        }

        #[inline]
        pub fn set_camera_offset(pos_offset: Float3, tilt_offset: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_camera_offset(crate::generated::unsynced_ctrl::Float3 { x: pos_offset.x, y: pos_offset.y, z: pos_offset.z }, crate::generated::unsynced_ctrl::Float3 { x: tilt_offset.x, y: tilt_offset.y, z: tilt_offset.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_clipboard(text: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(text, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(text)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_clipboard(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn set_custom_command_draw_data(cmd_id: i32, cmd_reference: &DefRef, color: Float4, show_area: bool) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(cmd_reference.name.len() as u32).to_le_bytes()); __b.extend_from_slice(cmd_reference.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&cmd_reference.id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&color.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&color.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&color.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&color.w.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::unsynced_ctrl::set_custom_command_draw_data(cmd_id, show_area as i32, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_custom_palette_color(index: i32, r: f32, g: f32, b: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_custom_palette_color(index, r, g, b)?;
            Ok(value)
        }

        #[inline]
        pub fn set_default_interface_visible(parts: u32, visible: bool) -> Result<u32> {
            let value = crate::generated::unsynced_ctrl::set_default_interface_visible(parts, visible)?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_curve(degree: i32, control_points: &[Float4], knots: &[f32]) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(control_points.len() as u32).to_le_bytes()); for __item in control_points.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.w.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); }} __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(knots.len() as u32).to_le_bytes()); for __item in knots.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_bits().to_le_bytes());} __b };
            crate::generated::dynamic_input::unsynced_ctrl::set_dolly_camera_curve(degree, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_dolly_camera_look_curve(degree: i32, control_points: &[Float4], knots: &[f32]) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(control_points.len() as u32).to_le_bytes()); for __item in control_points.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.w.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); }} __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(knots.len() as u32).to_le_bytes()); for __item in knots.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_bits().to_le_bytes());} __b };
            crate::generated::dynamic_input::unsynced_ctrl::set_dolly_camera_look_curve(degree, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_dolly_camera_look_position(position: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_look_position(crate::generated::unsynced_ctrl::Float3 { x: position.x, y: position.y, z: position.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_look_unit(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_look_unit(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_mode(mode: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_mode(mode)?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_position(position: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_position(crate::generated::unsynced_ctrl::Float3 { x: position.x, y: position.y, z: position.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_dolly_camera_relative_mode(mode: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_dolly_camera_relative_mode(mode)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_ground(draw_ground: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_ground(draw_ground)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_ground_deferred(draw_deferred: bool, draw_forward: bool) -> Result<SetDrawGroundDeferredValue> {
            let value = crate::generated::unsynced_ctrl::set_draw_ground_deferred(draw_deferred, draw_forward)?;
            Ok(SetDrawGroundDeferredValue {
                success: value.0,
                deferred: value.1,
                forward: value.2
            })
        }

        #[inline]
        pub fn set_draw_models_deferred(draw_units_deferred: bool, draw_features_deferred: bool, draw_units_forward: bool, draw_features_forward: bool) -> Result<SetDrawModelsDeferredValue> {
            let value = crate::generated::unsynced_ctrl::set_draw_models_deferred(draw_units_deferred, draw_features_deferred, draw_units_forward, draw_features_forward)?;
            Ok(SetDrawModelsDeferredValue {
                success: value.0,
                units_deferred: value.1,
                features_deferred: value.2,
                units_forward: value.3,
                features_forward: value.4
            })
        }

        #[inline]
        pub fn set_draw_selection_info(draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_selection_info(draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_sky(draw_sky: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_sky(draw_sky)?;
            Ok(value)
        }

        #[inline]
        pub fn set_draw_water(draw_water: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_draw_water(draw_water)?;
            Ok(value)
        }

        #[inline]
        pub fn set_engine_build_square_rendering(enabled: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_engine_build_square_rendering(enabled)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_always_update_matrix(feature_id: i32, enable: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_always_update_matrix(feature_id, enable)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_engine_draw_mask(feature_id: i32, mask: u32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_engine_draw_mask(feature_id, mask)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_fade(feature_id: i32, allow: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_fade(feature_id, allow)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_lua_draw(feature_id: i32, lua_draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_lua_draw(feature_id, lua_draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_no_draw(feature_id: i32, no_draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_no_draw(feature_id, no_draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_feature_palette_index(feature_id: i32, custom_index: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_feature_palette_index(feature_id, custom_index)?;
            Ok(value)
        }

        #[inline]
        pub fn set_last_message_position(pos: Float3) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_last_message_position(crate::generated::unsynced_ctrl::Float3 { x: pos.x, y: pos.y, z: pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn set_los_view_colors(always: RgbColor, los: RgbColor, radar: RgbColor, jam: RgbColor, radar2: RgbColor) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_los_view_colors(crate::generated::unsynced_ctrl::RgbColor { r: always.r, g: always.g, b: always.b }, crate::generated::unsynced_ctrl::RgbColor { r: los.r, g: los.g, b: los.b }, crate::generated::unsynced_ctrl::RgbColor { r: radar.r, g: radar.g, b: radar.b }, crate::generated::unsynced_ctrl::RgbColor { r: jam.r, g: jam.g, b: jam.b }, crate::generated::unsynced_ctrl::RgbColor { r: radar2.r, g: radar2.g, b: radar2.b })?;
            Ok(value)
        }

        #[inline]
        pub fn set_map_rendering_params(params: MapRenderingParams) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_map_rendering_params(crate::generated::unsynced_ctrl::MapRenderingParams { splat_tex_scales: match params.splat_tex_scales { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, splat_tex_mults: match params.splat_tex_mults { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, void_water: params.void_water, void_ground: params.void_ground, splat_detail_normal_diffuse_alpha: params.splat_detail_normal_diffuse_alpha })?;
            Ok(value)
        }

        #[inline]
        pub fn set_map_shader(standard_shader_id: i32, deferred_shader_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_map_shader(standard_shader_id, deferred_shader_id)?;
            Ok(value)
        }

        #[inline]
        pub fn set_map_shading_texture(tex_type: &str, tex_name: &str, num: i32) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(tex_type, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(tex_type)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(tex_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(tex_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_map_shading_texture(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr(), num)
        }

        #[inline]
        pub fn set_mini_map_rotation(radians: f32) -> Result<SetMiniMapRotationValue> {
            let value = crate::generated::unsynced_ctrl::set_mini_map_rotation(radians)?;
            Ok(SetMiniMapRotationValue {
                success: value.0,
                rotation: value.1
            })
        }

        #[inline]
        pub fn set_mouse_cursor(cursor_name: &str, scale: f32) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(cursor_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(cursor_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_mouse_cursor(__core_string_0_buf.as_cstr(), scale)
        }

        #[inline]
        pub fn set_nano_projectile_params(r: f32, v: f32, a: f32, rand_r: f32, rand_v: f32, rand_a: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_nano_projectile_params(r, v, a, rand_r, rand_v, rand_a)?;
            Ok(value)
        }

        #[inline]
        pub fn set_projectile_lua_draw(projectile_id: i32, lua_draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_projectile_lua_draw(projectile_id, lua_draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_shock_front_factors(options: SetShockFrontFactorsOptions) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_shock_front_factors(crate::generated::unsynced_ctrl::SetShockFrontFactorsOptions { min_area: options.min_area, min_power: options.min_power, dist_adj: options.dist_adj })?;
            Ok(value)
        }

        #[inline]
        pub fn set_sky_box_texture(tex_name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(tex_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(tex_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_sky_box_texture(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn set_sun_direction(dir: Float3, intensity: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_sun_direction(crate::generated::unsynced_ctrl::Float3 { x: dir.x, y: dir.y, z: dir.z }, intensity)?;
            Ok(value)
        }

        #[inline]
        pub fn set_sun_lighting(params: SunLightingParams) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_sun_lighting(crate::generated::unsynced_ctrl::SunLightingParams { ground_ambient_color: match params.ground_ambient_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, ground_diffuse_color: match params.ground_diffuse_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, ground_specular_color: match params.ground_specular_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, model_ambient_color: match params.model_ambient_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, model_diffuse_color: match params.model_diffuse_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, model_specular_color: match params.model_specular_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 4 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 4]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, specular_exponent: params.specular_exponent, ground_shadow_density: params.ground_shadow_density, model_shadow_density: params.model_shadow_density })?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_always_update_matrix(unit_id: i32, always_update_matrix: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_always_update_matrix(unit_id, always_update_matrix)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_def_icon(unit_def_id: i32, icon_name: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(icon_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(icon_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_unit_def_icon(unit_def_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_unit_def_image(unit_def_id: i32, image: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(image, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(image)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_unit_def_image(unit_def_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_unit_engine_draw_mask(unit_id: i32, draw_mask: u32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_engine_draw_mask(unit_id, draw_mask)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_icon(unit_id: i32, icon_name: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(icon_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(icon_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_unit_icon(unit_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_unit_icon_draw(unit_id: i32, draw_icon: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_icon_draw(unit_id, draw_icon)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_leave_tracks(unit_id: i32, leave_tracks: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_leave_tracks(unit_id, leave_tracks)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_lua_draw(unit_id: i32, lua_draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_lua_draw(unit_id, lua_draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_draw(unit_id: i32, no_draw: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_draw(unit_id, no_draw)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_group(unit_id: i32, no_group: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_group(unit_id, no_group)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_minimap(unit_id: i32, no_minimap: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_minimap(unit_id, no_minimap)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_no_select(unit_id: i32, no_select: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_no_select(unit_id, no_select)?;
            Ok(value)
        }

        #[inline]
        pub fn set_unit_palette_index(unit_id: i32, custom_index: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_unit_palette_index(unit_id, custom_index)?;
            Ok(value)
        }

        #[inline]
        pub fn set_video_capturing_mode(allow_capture_mode: bool) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_video_capturing_mode(allow_capture_mode)?;
            Ok(value)
        }

        #[inline]
        pub fn set_video_capturing_time_offset(time_offset: f32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_video_capturing_time_offset(time_offset)?;
            Ok(value)
        }

        #[inline]
        pub fn set_wm_caption(title: &str, title_short: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(title, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(title)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(title_short, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(title_short)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_wm_caption(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_wm_icon(icon_file_name: &str, force_resolution: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(icon_file_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(icon_file_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_wm_icon(__core_string_0_buf.as_cstr(), force_resolution)
        }

        #[inline]
        pub fn set_water_params(params: WaterParams) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_water_params(crate::generated::unsynced_ctrl::WaterParams { absorb: match params.absorb { Some(value) => Some({ let __values = &value[..]; if __values.len() > 3 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 3]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, base_color: match params.base_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 3 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 3]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, min_color: match params.min_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 3 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 3]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, surface_color: match params.surface_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 3 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 3]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, diffuse_color: match params.diffuse_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 3 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 3]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, specular_color: match params.specular_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 3 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 3]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, plane_color: match params.plane_color { Some(value) => Some({ let __values = &value[..]; if __values.len() > 3 { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); } let mut __array = [0 as f32; 3]; __array[..__values.len()].copy_from_slice(__values); __array }), None => None }, repeat_x: params.repeat_x, repeat_y: params.repeat_y, surface_alpha: params.surface_alpha, ambient_factor: params.ambient_factor, diffuse_factor: params.diffuse_factor, specular_factor: params.specular_factor, specular_power: params.specular_power, fresnel_min: params.fresnel_min, fresnel_max: params.fresnel_max, fresnel_power: params.fresnel_power, reflection_distortion: params.reflection_distortion, blur_base: params.blur_base, blur_exponent: params.blur_exponent, perlin_start_freq: params.perlin_start_freq, perlin_lacunarity: params.perlin_lacunarity, perlin_amplitude: params.perlin_amplitude, wind_speed: params.wind_speed, wave_offset_factor: params.wave_offset_factor, wave_length: params.wave_length, wave_foam_distortion: params.wave_foam_distortion, wave_foam_intensity: params.wave_foam_intensity, caustics_resolution: params.caustics_resolution, caustics_strength: params.caustics_strength, num_tiles: params.num_tiles, shore_waves: params.shore_waves, force_rendering: params.force_rendering, has_water_plane: params.has_water_plane })?;
            Ok(value)
        }

        #[inline]
        pub fn set_water_texture(tex_type: &str, tex_name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(tex_type, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(tex_type)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(tex_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(tex_name)?),
            };
            crate::generated::borrowed::unsynced_ctrl::set_water_texture(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn set_window_geometry(display_index: i32, window_pos_x: i32, window_pos_y: i32, window_size_x: i32, window_size_y: i32, options: SetWindowGeometryOptions) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_window_geometry(display_index, window_pos_x, window_pos_y, window_size_x, window_size_y, crate::generated::unsynced_ctrl::SetWindowGeometryOptions { full_screen: options.full_screen, borderless: options.borderless })?;
            Ok(value)
        }

        #[inline]
        pub fn set_window_maximized(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_window_maximized(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn set_window_minimized(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::set_window_minimized(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn warp_mouse(x: i32, y: i32) -> Result<bool> {
            let value = crate::generated::unsynced_ctrl::warp_mouse(x, y)?;
            Ok(value)
        }

    }


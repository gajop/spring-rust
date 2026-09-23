    pub mod unsynced_random {
        use super::{Result};

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnsyncedCtrlApi {
            pub set_unit_no_draw: u32,
            pub set_unit_engine_draw_mask: u32,
            pub set_unit_always_update_matrix: u32,
            pub set_unit_no_minimap: u32,
            pub set_unit_no_group: u32,
            pub set_unit_no_select: u32,
            pub set_unit_leave_tracks: u32,
            pub set_mini_map_rotation: u32,
            pub set_clipboard: u32,
            pub set_mouse_cursor: u32,
            pub assign_mouse_cursor: u32,
            pub replace_mouse_cursor: u32,
            pub warp_mouse: u32,
            pub set_active_command: u32,
            pub sdl_start_text_input: u32,
            pub sdl_stop_text_input: u32,
            pub sdl_set_text_input_rect: u32,
            pub set_box_selection_by_engine: u32,
            pub set_build_facing: u32,
            pub set_build_spacing: u32,
            pub set_window_geometry: u32,
            pub set_window_minimized: u32,
            pub set_window_maximized: u32,
            pub set_wm_caption: u32,
            pub set_wm_icon: u32,
            pub set_video_capturing_mode: u32,
            pub run_dolly_camera: u32,
            pub pause_dolly_camera: u32,
            pub resume_dolly_camera: u32,
            pub set_dolly_camera_mode: u32,
            pub set_dolly_camera_position: u32,
            pub set_dolly_camera_curve: u32,
            pub set_dolly_camera_look_position: u32,
            pub set_dolly_camera_look_unit: u32,
            pub set_dolly_camera_look_curve: u32,
            pub set_dolly_camera_relative_mode: u32,
            pub set_video_capturing_time_offset: u32,
            pub set_camera_offset: u32,
            pub set_draw_ground: u32,
            pub set_draw_sky: u32,
            pub random: u32,
            pub set_default_interface_visible: u32,
            pub set_draw_water: u32,
            pub set_draw_ground_deferred: u32,
            pub set_draw_models_deferred: u32,
            pub set_atmosphere: u32,
            pub set_sun_direction: u32,
            pub set_sun_lighting: u32,
            pub set_water_params: u32,
            pub set_map_shader: u32,
            pub set_map_shading_texture: u32,
            pub set_sky_box_texture: u32,
            pub set_map_rendering_params: u32,
            pub set_los_view_colors: u32,
            pub set_draw_selection_info: u32,
            pub set_shock_front_factors: u32,
            pub set_custom_command_draw_data: u32,
            pub set_last_message_position: u32,
            pub load_cmd_colors_config: u32,
            pub load_ctrl_panel_config: u32,
            pub load_model_textures: u32,
            pub force_layout_update: u32,
            pub force_tesselation_update: u32,
            pub set_auto_show_metal: u32,
            pub set_unit_icon_draw: u32,
            pub set_unit_icon: u32,
            pub set_unit_def_icon: u32,
            pub set_unit_def_image: u32,
            pub set_custom_palette_color: u32,
            pub set_unit_palette_index: u32,
            pub set_feature_palette_index: u32,
            pub set_engine_build_square_rendering: u32,
            pub set_feature_no_draw: u32,
            pub set_feature_engine_draw_mask: u32,
            pub set_feature_always_update_matrix: u32,
            pub set_feature_fade: u32,
            pub set_nano_projectile_params: u32,
            pub preload_feature_def_model: u32,
            pub preload_unit_def_model: u32,
            pub select_unit_map: u32,
            pub deselect_unit_map: u32,
            pub draw_unit_commands: u32,
            pub set_water_texture: u32,
            pub get_water_texture: u32,
            pub set_unit_lua_draw: u32,
            pub set_feature_lua_draw: u32,
            pub set_projectile_lua_draw: u32,
        }

        pub use super::types::{AssignMouseCursorQuery, AssignMouseCursorResult, AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, DeselectUnitMapQuery, DeselectUnitMapResult, DrawUnitCommandsQuery, DrawUnitCommandsResult, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, ForceLayoutUpdateQuery, ForceLayoutUpdateResult, ForceTesselationUpdateQuery, ForceTesselationUpdateResult, GetWaterTextureQuery, GetWaterTextureResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, Int2, Int3, Int32Array, Int32Result, LoadCmdColorsConfigQuery, LoadCmdColorsConfigResult, LoadCtrlPanelConfigQuery, LoadCtrlPanelConfigResult, LoadModelTexturesQuery, LoadModelTexturesResult, MapRenderingParams, NativeCommand, NativeExplosionParams, NativeProjectileParams, NextFloatQuery, NextFloatResult, NextIntQuery, NextIntResult, NextIntUpToQuery, NextIntUpToResult, NumberOrBool, PauseDollyCameraQuery, PauseDollyCameraResult, PreloadFeatureDefModelQuery, PreloadFeatureDefModelResult, PreloadUnitDefModelQuery, PreloadUnitDefModelResult, ProjectileTargetRef, ReplaceMouseCursorQuery, ReplaceMouseCursorResult, ResourcePack, ResumeDollyCameraQuery, ResumeDollyCameraResult, RgbColor, RunDollyCameraQuery, RunDollyCameraResult, SDLSetTextInputRectQuery, SDLSetTextInputRectResult, SDLStartTextInputQuery, SDLStartTextInputResult, SDLStopTextInputQuery, SDLStopTextInputResult, SelectUnitMapQuery, SelectUnitMapResult, SetActiveCommandOptions, SetActiveCommandQuery, SetActiveCommandResult, SetAtmosphereQuery, SetAtmosphereResult, SetAutoShowMetalQuery, SetAutoShowMetalResult, SetBoxSelectionByEngineQuery, SetBoxSelectionByEngineResult, SetBuildFacingQuery, SetBuildFacingResult, SetBuildSpacingQuery, SetBuildSpacingResult, SetCameraOffsetQuery, SetCameraOffsetResult, SetClipboardQuery, SetClipboardResult, SetCustomCommandDrawDataQuery, SetCustomCommandDrawDataResult, SetCustomPaletteColorQuery, SetCustomPaletteColorResult, SetDefaultInterfaceVisibleQuery, SetDefaultInterfaceVisibleResult, SetDollyCameraCurveQuery, SetDollyCameraCurveResult, SetDollyCameraLookCurveQuery, SetDollyCameraLookCurveResult, SetDollyCameraLookPositionQuery, SetDollyCameraLookPositionResult, SetDollyCameraLookUnitQuery, SetDollyCameraLookUnitResult, SetDollyCameraModeQuery, SetDollyCameraModeResult, SetDollyCameraPositionQuery, SetDollyCameraPositionResult, SetDollyCameraRelativeModeQuery, SetDollyCameraRelativeModeResult, SetDrawGroundDeferredQuery, SetDrawGroundDeferredResult, SetDrawGroundQuery, SetDrawGroundResult, SetDrawModelsDeferredQuery, SetDrawModelsDeferredResult, SetDrawSelectionInfoQuery, SetDrawSelectionInfoResult, SetDrawSkyQuery, SetDrawSkyResult, SetDrawWaterQuery, SetDrawWaterResult, SetEngineBuildSquareRenderingQuery, SetEngineBuildSquareRenderingResult, SetFeatureAlwaysUpdateMatrixQuery, SetFeatureAlwaysUpdateMatrixResult, SetFeatureEngineDrawMaskQuery, SetFeatureEngineDrawMaskResult, SetFeatureFadeQuery, SetFeatureFadeResult, SetFeatureLuaDrawQuery, SetFeatureLuaDrawResult, SetFeatureNoDrawQuery, SetFeatureNoDrawResult, SetFeaturePaletteIndexQuery, SetFeaturePaletteIndexResult, SetLastMessagePositionQuery, SetLastMessagePositionResult, SetLosViewColorsQuery, SetLosViewColorsResult, SetMapRenderingParamsQuery, SetMapRenderingParamsResult, SetMapShaderQuery, SetMapShaderResult, SetMapShadingTextureQuery, SetMapShadingTextureResult, SetMiniMapRotationQuery, SetMiniMapRotationResult, SetMouseCursorQuery, SetMouseCursorResult, SetNanoProjectileParamsQuery, SetNanoProjectileParamsResult, SetProjectileLuaDrawQuery, SetProjectileLuaDrawResult, SetSeedQuery, SetSeedResult, SetShockFrontFactorsOptions, SetShockFrontFactorsQuery, SetShockFrontFactorsResult, SetSkyBoxTextureQuery, SetSkyBoxTextureResult, SetSunDirectionQuery, SetSunDirectionResult, SetSunLightingQuery, SetSunLightingResult, SetUnitAlwaysUpdateMatrixQuery, SetUnitAlwaysUpdateMatrixResult, SetUnitDefIconQuery, SetUnitDefIconResult, SetUnitDefImageQuery, SetUnitDefImageResult, SetUnitEngineDrawMaskQuery, SetUnitEngineDrawMaskResult, SetUnitIconDrawQuery, SetUnitIconDrawResult, SetUnitIconQuery, SetUnitIconResult, SetUnitLeaveTracksQuery, SetUnitLeaveTracksResult, SetUnitLuaDrawQuery, SetUnitLuaDrawResult, SetUnitNoDrawQuery, SetUnitNoDrawResult, SetUnitNoGroupQuery, SetUnitNoGroupResult, SetUnitNoMinimapQuery, SetUnitNoMinimapResult, SetUnitNoSelectQuery, SetUnitNoSelectResult, SetUnitPaletteIndexQuery, SetUnitPaletteIndexResult, SetVideoCapturingModeQuery, SetVideoCapturingModeResult, SetVideoCapturingTimeOffsetQuery, SetVideoCapturingTimeOffsetResult, SetWMCaptionQuery, SetWMCaptionResult, SetWMIconQuery, SetWMIconResult, SetWaterParamsQuery, SetWaterParamsResult, SetWaterTextureQuery, SetWaterTextureResult, SetWindowGeometryOptions, SetWindowGeometryQuery, SetWindowGeometryResult, SetWindowMaximizedQuery, SetWindowMaximizedResult, SetWindowMinimizedQuery, SetWindowMinimizedResult, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WarpMouseQuery, WarpMouseResult, WaterParams};

        #[inline]
        pub fn next_float(unused: u8) -> Result<f32> {
            let value = crate::generated::unsynced_random::next_float(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn next_int(lower: i32, upper: i32) -> Result<i32> {
            let value = crate::generated::unsynced_random::next_int(lower, upper)?;
            Ok(value)
        }

        #[inline]
        pub fn next_int_up_to(upper: i32) -> Result<i32> {
            let value = crate::generated::unsynced_random::next_int_up_to(upper)?;
            Ok(value)
        }

        #[inline]
        pub fn set_seed(seed: i32) -> Result<bool> {
            let value = crate::generated::unsynced_random::set_seed(seed)?;
            Ok(value)
        }

    }


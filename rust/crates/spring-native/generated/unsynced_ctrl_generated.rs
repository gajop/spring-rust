#[derive(Debug, Clone, Copy, Default)]
pub struct SetActiveCommandOptions {
    pub left_click: bool,
    pub right_click: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub shift: bool,
}

impl From<SetActiveCommandOptions> for sys::SetActiveCommandOptions {
    fn from(options: SetActiveCommandOptions) -> Self {
        sys::SetActiveCommandOptions {
            leftClick: options.left_click,
            rightClick: options.right_click,
            alt: options.alt,
            ctrl: options.ctrl,
            meta: options.meta,
            shift: options.shift,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SetShockFrontFactorsOptions {
    pub min_area: Option<f32>,
    pub min_power: Option<f32>,
    pub dist_adj: Option<f32>,
}

impl From<SetShockFrontFactorsOptions> for sys::SetShockFrontFactorsOptions {
    fn from(options: SetShockFrontFactorsOptions) -> Self {
        sys::SetShockFrontFactorsOptions {
            minArea: options.min_area.unwrap_or(0.0),
            hasMinArea: options.min_area.is_some(),
            minPower: options.min_power.unwrap_or(0.0),
            hasMinPower: options.min_power.is_some(),
            distAdj: options.dist_adj.unwrap_or(0.0),
            hasDistAdj: options.dist_adj.is_some(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SetWindowGeometryOptions {
    pub full_screen: bool,
    pub borderless: bool,
}

impl From<SetWindowGeometryOptions> for sys::SetWindowGeometryOptions {
    fn from(options: SetWindowGeometryOptions) -> Self {
        sys::SetWindowGeometryOptions {
            fullScreen: options.full_screen,
            borderless: options.borderless,
        }
    }
}

impl<'a> UnsyncedCtrl<'a> {
    pub fn set_unit_no_draw(&self, unit_id: i32, no_draw: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitNoDrawQuery {
                unitID: unit_id,
                noDraw: no_draw,
            };
            let mut result = MaybeUninit::<sys::SetUnitNoDrawResult>::zeroed();
            let func = self.api.SetUnitNoDraw.expect("SetUnitNoDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_engine_draw_mask(&self, unit_id: i32, draw_mask: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitEngineDrawMaskQuery {
                unitID: unit_id,
                drawMask: draw_mask,
            };
            let mut result = MaybeUninit::<sys::SetUnitEngineDrawMaskResult>::zeroed();
            let func = self.api.SetUnitEngineDrawMask.expect("SetUnitEngineDrawMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_always_update_matrix(&self, unit_id: i32, always_update_matrix: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitAlwaysUpdateMatrixQuery {
                unitID: unit_id,
                alwaysUpdateMatrix: always_update_matrix,
            };
            let mut result = MaybeUninit::<sys::SetUnitAlwaysUpdateMatrixResult>::zeroed();
            let func = self.api.SetUnitAlwaysUpdateMatrix.expect("SetUnitAlwaysUpdateMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_no_minimap(&self, unit_id: i32, no_minimap: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitNoMinimapQuery {
                unitID: unit_id,
                noMinimap: no_minimap,
            };
            let mut result = MaybeUninit::<sys::SetUnitNoMinimapResult>::zeroed();
            let func = self.api.SetUnitNoMinimap.expect("SetUnitNoMinimap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_no_group(&self, unit_id: i32, no_group: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitNoGroupQuery {
                unitID: unit_id,
                noGroup: no_group,
            };
            let mut result = MaybeUninit::<sys::SetUnitNoGroupResult>::zeroed();
            let func = self.api.SetUnitNoGroup.expect("SetUnitNoGroup function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_no_select(&self, unit_id: i32, no_select: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitNoSelectQuery {
                unitID: unit_id,
                noSelect: no_select,
            };
            let mut result = MaybeUninit::<sys::SetUnitNoSelectResult>::zeroed();
            let func = self.api.SetUnitNoSelect.expect("SetUnitNoSelect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_leave_tracks(&self, unit_id: i32, leave_tracks: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitLeaveTracksQuery {
                unitID: unit_id,
                leaveTracks: leave_tracks,
            };
            let mut result = MaybeUninit::<sys::SetUnitLeaveTracksResult>::zeroed();
            let func = self.api.SetUnitLeaveTracks.expect("SetUnitLeaveTracks function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_mini_map_rotation(&self, radians: f32) -> Result<(bool, i32), Error> {
        unsafe {
            let query = sys::SetMiniMapRotationQuery {
                radians: radians,
            };
            let mut result = MaybeUninit::<sys::SetMiniMapRotationResult>::zeroed();
            let func = self.api.SetMiniMapRotation.expect("SetMiniMapRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.success,
                result.rotation,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn set_clipboard(&self, text: &str) -> Result<bool, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::SetClipboardQuery {
                text: text_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetClipboardResult>::zeroed();
            let func = self.api.SetClipboard.expect("SetClipboard function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_mouse_cursor(&self, cursor_name: &str, scale: f32) -> Result<bool, Error> {
        unsafe {
            let cursor_name_cstr = std::ffi::CString::new(cursor_name).map_err(|_| Error::invalid_argument("cursor_name"))?;
            let query = sys::SetMouseCursorQuery {
                cursorName: cursor_name_cstr.as_ptr(),
                scale: scale,
            };
            let mut result = MaybeUninit::<sys::SetMouseCursorResult>::zeroed();
            let func = self.api.SetMouseCursor.expect("SetMouseCursor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn assign_mouse_cursor(&self, command_name: &str, cursor_file_name: &str, overwrite: bool, hot_spot_top_left: bool) -> Result<bool, Error> {
        unsafe {
            let command_name_cstr = std::ffi::CString::new(command_name).map_err(|_| Error::invalid_argument("command_name"))?;
            let cursor_file_name_cstr = std::ffi::CString::new(cursor_file_name).map_err(|_| Error::invalid_argument("cursor_file_name"))?;
            let query = sys::AssignMouseCursorQuery {
                commandName: command_name_cstr.as_ptr(),
                cursorFileName: cursor_file_name_cstr.as_ptr(),
                overwrite: overwrite,
                hotSpotTopLeft: hot_spot_top_left,
            };
            let mut result = MaybeUninit::<sys::AssignMouseCursorResult>::zeroed();
            let func = self.api.AssignMouseCursor.expect("AssignMouseCursor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn replace_mouse_cursor(&self, old_cursor_file_name: &str, new_cursor_file_name: &str, hot_spot_top_left: bool) -> Result<bool, Error> {
        unsafe {
            let old_cursor_file_name_cstr = std::ffi::CString::new(old_cursor_file_name).map_err(|_| Error::invalid_argument("old_cursor_file_name"))?;
            let new_cursor_file_name_cstr = std::ffi::CString::new(new_cursor_file_name).map_err(|_| Error::invalid_argument("new_cursor_file_name"))?;
            let query = sys::ReplaceMouseCursorQuery {
                oldCursorFileName: old_cursor_file_name_cstr.as_ptr(),
                newCursorFileName: new_cursor_file_name_cstr.as_ptr(),
                hotSpotTopLeft: hot_spot_top_left,
            };
            let mut result = MaybeUninit::<sys::ReplaceMouseCursorResult>::zeroed();
            let func = self.api.ReplaceMouseCursor.expect("ReplaceMouseCursor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn warp_mouse(&self, x: i32, y: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::WarpMouseQuery {
                x: x,
                y: y,
            };
            let mut result = MaybeUninit::<sys::WarpMouseResult>::zeroed();
            let func = self.api.WarpMouse.expect("WarpMouse function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_active_command(&self, cmd_index: i32, button: i32, options: SetActiveCommandOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetActiveCommandQuery {
                cmdIndex: cmd_index,
                button: button,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetActiveCommandResult>::zeroed();
            let func = self.api.SetActiveCommand.expect("SetActiveCommand function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn sdlstart_text_input(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::SDLStartTextInputQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::SDLStartTextInputResult>::zeroed();
            let func = self.api.SDLStartTextInput.expect("SDLStartTextInput function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn sdlstop_text_input(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::SDLStopTextInputQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::SDLStopTextInputResult>::zeroed();
            let func = self.api.SDLStopTextInput.expect("SDLStopTextInput function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn sdlset_text_input_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SDLSetTextInputRectQuery {
                x: x,
                y: y,
                w: w,
                h: h,
            };
            let mut result = MaybeUninit::<sys::SDLSetTextInputRectResult>::zeroed();
            let func = self.api.SDLSetTextInputRect.expect("SDLSetTextInputRect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_box_selection_by_engine(&self, state: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetBoxSelectionByEngineQuery {
                state: state,
            };
            let mut result = MaybeUninit::<sys::SetBoxSelectionByEngineResult>::zeroed();
            let func = self.api.SetBoxSelectionByEngine.expect("SetBoxSelectionByEngine function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_build_facing(&self, facing: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetBuildFacingQuery {
                facing: facing,
            };
            let mut result = MaybeUninit::<sys::SetBuildFacingResult>::zeroed();
            let func = self.api.SetBuildFacing.expect("SetBuildFacing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_build_spacing(&self, spacing: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetBuildSpacingQuery {
                spacing: spacing,
            };
            let mut result = MaybeUninit::<sys::SetBuildSpacingResult>::zeroed();
            let func = self.api.SetBuildSpacing.expect("SetBuildSpacing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_window_geometry(&self, display_index: i32, window_pos_x: i32, window_pos_y: i32, window_size_x: i32, window_size_y: i32, options: SetWindowGeometryOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetWindowGeometryQuery {
                displayIndex: display_index,
                windowPosX: window_pos_x,
                windowPosY: window_pos_y,
                windowSizeX: window_size_x,
                windowSizeY: window_size_y,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetWindowGeometryResult>::zeroed();
            let func = self.api.SetWindowGeometry.expect("SetWindowGeometry function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_window_minimized(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetWindowMinimizedQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::SetWindowMinimizedResult>::zeroed();
            let func = self.api.SetWindowMinimized.expect("SetWindowMinimized function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.minimized
            })
        }
    }

    pub fn set_window_maximized(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetWindowMaximizedQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::SetWindowMaximizedResult>::zeroed();
            let func = self.api.SetWindowMaximized.expect("SetWindowMaximized function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.maximized
            })
        }
    }

    pub fn set_wmcaption(&self, title: &str, title_short: &str) -> Result<bool, Error> {
        unsafe {
            let title_cstr = std::ffi::CString::new(title).map_err(|_| Error::invalid_argument("title"))?;
            let title_short_cstr = std::ffi::CString::new(title_short).map_err(|_| Error::invalid_argument("title_short"))?;
            let query = sys::SetWMCaptionQuery {
                title: title_cstr.as_ptr(),
                titleShort: title_short_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetWMCaptionResult>::zeroed();
            let func = self.api.SetWMCaption.expect("SetWMCaption function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_wmicon(&self, icon_file_name: &str, force_resolution: bool) -> Result<bool, Error> {
        unsafe {
            let icon_file_name_cstr = std::ffi::CString::new(icon_file_name).map_err(|_| Error::invalid_argument("icon_file_name"))?;
            let query = sys::SetWMIconQuery {
                iconFileName: icon_file_name_cstr.as_ptr(),
                forceResolution: force_resolution,
            };
            let mut result = MaybeUninit::<sys::SetWMIconResult>::zeroed();
            let func = self.api.SetWMIcon.expect("SetWMIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_video_capturing_mode(&self, allow_capture_mode: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetVideoCapturingModeQuery {
                allowCaptureMode: allow_capture_mode,
            };
            let mut result = MaybeUninit::<sys::SetVideoCapturingModeResult>::zeroed();
            let func = self.api.SetVideoCapturingMode.expect("SetVideoCapturingMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn run_dolly_camera(&self, runtime_ms: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RunDollyCameraQuery {
                runtimeMs: runtime_ms,
            };
            let mut result = MaybeUninit::<sys::RunDollyCameraResult>::zeroed();
            let func = self.api.RunDollyCamera.expect("RunDollyCamera function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn pause_dolly_camera(&self, percent: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::PauseDollyCameraQuery {
                percent: percent,
            };
            let mut result = MaybeUninit::<sys::PauseDollyCameraResult>::zeroed();
            let func = self.api.PauseDollyCamera.expect("PauseDollyCamera function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn resume_dolly_camera(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::ResumeDollyCameraQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::ResumeDollyCameraResult>::zeroed();
            let func = self.api.ResumeDollyCamera.expect("ResumeDollyCamera function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_dolly_camera_mode(&self, mode: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDollyCameraModeQuery {
                mode: mode,
            };
            let mut result = MaybeUninit::<sys::SetDollyCameraModeResult>::zeroed();
            let func = self.api.SetDollyCameraMode.expect("SetDollyCameraMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_dolly_camera_position(&self, position: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDollyCameraPositionQuery {
                position: position,
            };
            let mut result = MaybeUninit::<sys::SetDollyCameraPositionResult>::zeroed();
            let func = self.api.SetDollyCameraPosition.expect("SetDollyCameraPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_dolly_camera_curve(&self, degree: i32, control_points: &[sys::Float4], knots: &[f32]) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDollyCameraCurveQuery {
                degree: degree,
                controlPoints: control_points.as_ptr(),
                controlPointsCount: control_points.len() as u32,
                knots: knots.as_ptr(),
                knotsCount: knots.len() as u32,
            };
            let mut result = MaybeUninit::<sys::SetDollyCameraCurveResult>::zeroed();
            let func = self.api.SetDollyCameraCurve.expect("SetDollyCameraCurve function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_dolly_camera_look_position(&self, position: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDollyCameraLookPositionQuery {
                position: position,
            };
            let mut result = MaybeUninit::<sys::SetDollyCameraLookPositionResult>::zeroed();
            let func = self.api.SetDollyCameraLookPosition.expect("SetDollyCameraLookPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_dolly_camera_look_unit(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDollyCameraLookUnitQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::SetDollyCameraLookUnitResult>::zeroed();
            let func = self.api.SetDollyCameraLookUnit.expect("SetDollyCameraLookUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_dolly_camera_look_curve(&self, degree: i32, control_points: &[sys::Float4], knots: &[f32]) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDollyCameraLookCurveQuery {
                degree: degree,
                controlPoints: control_points.as_ptr(),
                controlPointsCount: control_points.len() as u32,
                knots: knots.as_ptr(),
                knotsCount: knots.len() as u32,
            };
            let mut result = MaybeUninit::<sys::SetDollyCameraLookCurveResult>::zeroed();
            let func = self.api.SetDollyCameraLookCurve.expect("SetDollyCameraLookCurve function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_dolly_camera_relative_mode(&self, mode: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDollyCameraRelativeModeQuery {
                mode: mode,
            };
            let mut result = MaybeUninit::<sys::SetDollyCameraRelativeModeResult>::zeroed();
            let func = self.api.SetDollyCameraRelativeMode.expect("SetDollyCameraRelativeMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_video_capturing_time_offset(&self, time_offset: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetVideoCapturingTimeOffsetQuery {
                timeOffset: time_offset,
            };
            let mut result = MaybeUninit::<sys::SetVideoCapturingTimeOffsetResult>::zeroed();
            let func = self.api.SetVideoCapturingTimeOffset.expect("SetVideoCapturingTimeOffset function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_camera_offset(&self, pos_offset: sys::Float3, tilt_offset: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetCameraOffsetQuery {
                posOffset: pos_offset,
                tiltOffset: tilt_offset,
            };
            let mut result = MaybeUninit::<sys::SetCameraOffsetResult>::zeroed();
            let func = self.api.SetCameraOffset.expect("SetCameraOffset function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_draw_ground(&self, draw_ground: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDrawGroundQuery {
                drawGround: draw_ground,
            };
            let mut result = MaybeUninit::<sys::SetDrawGroundResult>::zeroed();
            let func = self.api.SetDrawGround.expect("SetDrawGround function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_draw_sky(&self, draw_sky: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDrawSkyQuery {
                drawSky: draw_sky,
            };
            let mut result = MaybeUninit::<sys::SetDrawSkyResult>::zeroed();
            let func = self.api.SetDrawSky.expect("SetDrawSky function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_draw_water(&self, draw_water: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDrawWaterQuery {
                drawWater: draw_water,
            };
            let mut result = MaybeUninit::<sys::SetDrawWaterResult>::zeroed();
            let func = self.api.SetDrawWater.expect("SetDrawWater function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_draw_ground_deferred(&self, draw_deferred: bool, draw_forward: bool) -> Result<(bool, bool, bool), Error> {
        unsafe {
            let query = sys::SetDrawGroundDeferredQuery {
                drawDeferred: draw_deferred,
                drawForward: draw_forward,
            };
            let mut result = MaybeUninit::<sys::SetDrawGroundDeferredResult>::zeroed();
            let func = self.api.SetDrawGroundDeferred.expect("SetDrawGroundDeferred function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.success,
                result.deferred,
                result.forward,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn set_draw_models_deferred(&self, draw_units_deferred: bool, draw_features_deferred: bool, draw_units_forward: bool, draw_features_forward: bool) -> Result<(bool, bool, bool, bool, bool), Error> {
        unsafe {
            let query = sys::SetDrawModelsDeferredQuery {
                drawUnitsDeferred: draw_units_deferred,
                drawFeaturesDeferred: draw_features_deferred,
                drawUnitsForward: draw_units_forward,
                drawFeaturesForward: draw_features_forward,
            };
            let mut result = MaybeUninit::<sys::SetDrawModelsDeferredResult>::zeroed();
            let func = self.api.SetDrawModelsDeferred.expect("SetDrawModelsDeferred function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.success,
                result.unitsDeferred,
                result.featuresDeferred,
                result.unitsForward,
                result.featuresForward,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn set_atmosphere(&self, params: sys::AtmosphereParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetAtmosphereQuery {
                params: params,
            };
            let mut result = MaybeUninit::<sys::SetAtmosphereResult>::zeroed();
            let func = self.api.SetAtmosphere.expect("SetAtmosphere function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_sun_direction(&self, dir: sys::Float3, intensity: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetSunDirectionQuery {
                dir: dir,
                intensity: intensity,
            };
            let mut result = MaybeUninit::<sys::SetSunDirectionResult>::zeroed();
            let func = self.api.SetSunDirection.expect("SetSunDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_sun_lighting(&self, params: sys::SunLightingParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetSunLightingQuery {
                params: params,
            };
            let mut result = MaybeUninit::<sys::SetSunLightingResult>::zeroed();
            let func = self.api.SetSunLighting.expect("SetSunLighting function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_water_params(&self, params: sys::WaterParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetWaterParamsQuery {
                params: params,
            };
            let mut result = MaybeUninit::<sys::SetWaterParamsResult>::zeroed();
            let func = self.api.SetWaterParams.expect("SetWaterParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_map_shader(&self, standard_shader_id: i32, deferred_shader_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMapShaderQuery {
                standardShaderID: standard_shader_id,
                deferredShaderID: deferred_shader_id,
            };
            let mut result = MaybeUninit::<sys::SetMapShaderResult>::zeroed();
            let func = self.api.SetMapShader.expect("SetMapShader function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_map_shading_texture(&self, tex_type: &str, tex_name: &str, num: i32) -> Result<bool, Error> {
        unsafe {
            let tex_type_cstr = std::ffi::CString::new(tex_type).map_err(|_| Error::invalid_argument("tex_type"))?;
            let tex_name_cstr = std::ffi::CString::new(tex_name).map_err(|_| Error::invalid_argument("tex_name"))?;
            let query = sys::SetMapShadingTextureQuery {
                texType: tex_type_cstr.as_ptr(),
                texName: tex_name_cstr.as_ptr(),
                num: num,
            };
            let mut result = MaybeUninit::<sys::SetMapShadingTextureResult>::zeroed();
            let func = self.api.SetMapShadingTexture.expect("SetMapShadingTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_sky_box_texture(&self, tex_name: &str) -> Result<bool, Error> {
        unsafe {
            let tex_name_cstr = std::ffi::CString::new(tex_name).map_err(|_| Error::invalid_argument("tex_name"))?;
            let query = sys::SetSkyBoxTextureQuery {
                texName: tex_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetSkyBoxTextureResult>::zeroed();
            let func = self.api.SetSkyBoxTexture.expect("SetSkyBoxTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_map_rendering_params(&self, params: sys::MapRenderingParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMapRenderingParamsQuery {
                params: params,
            };
            let mut result = MaybeUninit::<sys::SetMapRenderingParamsResult>::zeroed();
            let func = self.api.SetMapRenderingParams.expect("SetMapRenderingParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_los_view_colors(&self, always: sys::RgbColor, los: sys::RgbColor, radar: sys::RgbColor, jam: sys::RgbColor, radar2: sys::RgbColor) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetLosViewColorsQuery {
                always: always,
                los: los,
                radar: radar,
                jam: jam,
                radar2: radar2,
            };
            let mut result = MaybeUninit::<sys::SetLosViewColorsResult>::zeroed();
            let func = self.api.SetLosViewColors.expect("SetLosViewColors function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_draw_selection_info(&self, draw: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetDrawSelectionInfoQuery {
                draw: draw,
            };
            let mut result = MaybeUninit::<sys::SetDrawSelectionInfoResult>::zeroed();
            let func = self.api.SetDrawSelectionInfo.expect("SetDrawSelectionInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_shock_front_factors(&self, options: SetShockFrontFactorsOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetShockFrontFactorsQuery {
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetShockFrontFactorsResult>::zeroed();
            let func = self.api.SetShockFrontFactors.expect("SetShockFrontFactors function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_custom_command_draw_data(&self, cmd_id: i32, cmd_reference: sys::DefRef, color: sys::Float4, show_area: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetCustomCommandDrawDataQuery {
                cmdID: cmd_id,
                cmdReference: cmd_reference,
                color: color,
                showArea: show_area,
            };
            let mut result = MaybeUninit::<sys::SetCustomCommandDrawDataResult>::zeroed();
            let func = self.api.SetCustomCommandDrawData.expect("SetCustomCommandDrawData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_last_message_position(&self, pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetLastMessagePositionQuery {
                pos: pos,
            };
            let mut result = MaybeUninit::<sys::SetLastMessagePositionResult>::zeroed();
            let func = self.api.SetLastMessagePosition.expect("SetLastMessagePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn load_cmd_colors_config(&self, filename: &str) -> Result<bool, Error> {
        unsafe {
            let filename_cstr = std::ffi::CString::new(filename).map_err(|_| Error::invalid_argument("filename"))?;
            let query = sys::LoadCmdColorsConfigQuery {
                filename: filename_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::LoadCmdColorsConfigResult>::zeroed();
            let func = self.api.LoadCmdColorsConfig.expect("LoadCmdColorsConfig function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn load_ctrl_panel_config(&self, filename: &str) -> Result<bool, Error> {
        unsafe {
            let filename_cstr = std::ffi::CString::new(filename).map_err(|_| Error::invalid_argument("filename"))?;
            let query = sys::LoadCtrlPanelConfigQuery {
                filename: filename_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::LoadCtrlPanelConfigResult>::zeroed();
            let func = self.api.LoadCtrlPanelConfig.expect("LoadCtrlPanelConfig function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn load_model_textures(&self, model_name: &str) -> Result<bool, Error> {
        unsafe {
            let model_name_cstr = std::ffi::CString::new(model_name).map_err(|_| Error::invalid_argument("model_name"))?;
            let query = sys::LoadModelTexturesQuery {
                modelName: model_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::LoadModelTexturesResult>::zeroed();
            let func = self.api.LoadModelTextures.expect("LoadModelTextures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn force_layout_update(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::ForceLayoutUpdateQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::ForceLayoutUpdateResult>::zeroed();
            let func = self.api.ForceLayoutUpdate.expect("ForceLayoutUpdate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn force_tesselation_update(&self, normal: bool, shadow: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::ForceTesselationUpdateQuery {
                normal: normal,
                shadow: shadow,
            };
            let mut result = MaybeUninit::<sys::ForceTesselationUpdateResult>::zeroed();
            let func = self.api.ForceTesselationUpdate.expect("ForceTesselationUpdate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_auto_show_metal(&self, enable: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetAutoShowMetalQuery {
                enable: enable,
            };
            let mut result = MaybeUninit::<sys::SetAutoShowMetalResult>::zeroed();
            let func = self.api.SetAutoShowMetal.expect("SetAutoShowMetal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_icon_draw(&self, unit_id: i32, draw_icon: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitIconDrawQuery {
                unitID: unit_id,
                drawIcon: draw_icon,
            };
            let mut result = MaybeUninit::<sys::SetUnitIconDrawResult>::zeroed();
            let func = self.api.SetUnitIconDraw.expect("SetUnitIconDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_icon(&self, unit_id: i32, icon_name: &str) -> Result<bool, Error> {
        unsafe {
            let icon_name_cstr = std::ffi::CString::new(icon_name).map_err(|_| Error::invalid_argument("icon_name"))?;
            let query = sys::SetUnitIconQuery {
                unitID: unit_id,
                iconName: icon_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetUnitIconResult>::zeroed();
            let func = self.api.SetUnitIcon.expect("SetUnitIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_def_icon(&self, unit_def_id: i32, icon_name: &str) -> Result<bool, Error> {
        unsafe {
            let icon_name_cstr = std::ffi::CString::new(icon_name).map_err(|_| Error::invalid_argument("icon_name"))?;
            let query = sys::SetUnitDefIconQuery {
                unitDefID: unit_def_id,
                iconName: icon_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetUnitDefIconResult>::zeroed();
            let func = self.api.SetUnitDefIcon.expect("SetUnitDefIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_def_image(&self, unit_def_id: i32, image: &str) -> Result<bool, Error> {
        unsafe {
            let image_cstr = std::ffi::CString::new(image).map_err(|_| Error::invalid_argument("image"))?;
            let query = sys::SetUnitDefImageQuery {
                unitDefID: unit_def_id,
                image: image_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetUnitDefImageResult>::zeroed();
            let func = self.api.SetUnitDefImage.expect("SetUnitDefImage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_custom_palette_color(&self, index: i32, r: f32, g: f32, b: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetCustomPaletteColorQuery {
                index: index,
                r: r,
                g: g,
                b: b,
            };
            let mut result = MaybeUninit::<sys::SetCustomPaletteColorResult>::zeroed();
            let func = self.api.SetCustomPaletteColor.expect("SetCustomPaletteColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_palette_index(&self, unit_id: i32, custom_index: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPaletteIndexQuery {
                unitID: unit_id,
                customIndex: custom_index,
            };
            let mut result = MaybeUninit::<sys::SetUnitPaletteIndexResult>::zeroed();
            let func = self.api.SetUnitPaletteIndex.expect("SetUnitPaletteIndex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_palette_index(&self, feature_id: i32, custom_index: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeaturePaletteIndexQuery {
                featureID: feature_id,
                customIndex: custom_index,
            };
            let mut result = MaybeUninit::<sys::SetFeaturePaletteIndexResult>::zeroed();
            let func = self.api.SetFeaturePaletteIndex.expect("SetFeaturePaletteIndex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_engine_build_square_rendering(&self, enabled: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetEngineBuildSquareRenderingQuery {
                enabled: enabled,
            };
            let mut result = MaybeUninit::<sys::SetEngineBuildSquareRenderingResult>::zeroed();
            let func = self.api.SetEngineBuildSquareRendering.expect("SetEngineBuildSquareRendering function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_no_draw(&self, feature_id: i32, no_draw: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureNoDrawQuery {
                featureID: feature_id,
                noDraw: no_draw,
            };
            let mut result = MaybeUninit::<sys::SetFeatureNoDrawResult>::zeroed();
            let func = self.api.SetFeatureNoDraw.expect("SetFeatureNoDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_engine_draw_mask(&self, feature_id: i32, mask: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureEngineDrawMaskQuery {
                featureID: feature_id,
                mask: mask,
            };
            let mut result = MaybeUninit::<sys::SetFeatureEngineDrawMaskResult>::zeroed();
            let func = self.api.SetFeatureEngineDrawMask.expect("SetFeatureEngineDrawMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_always_update_matrix(&self, feature_id: i32, enable: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureAlwaysUpdateMatrixQuery {
                featureID: feature_id,
                enable: enable,
            };
            let mut result = MaybeUninit::<sys::SetFeatureAlwaysUpdateMatrixResult>::zeroed();
            let func = self.api.SetFeatureAlwaysUpdateMatrix.expect("SetFeatureAlwaysUpdateMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_fade(&self, feature_id: i32, allow: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureFadeQuery {
                featureID: feature_id,
                allow: allow,
            };
            let mut result = MaybeUninit::<sys::SetFeatureFadeResult>::zeroed();
            let func = self.api.SetFeatureFade.expect("SetFeatureFade function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_nano_projectile_params(&self, r: f32, v: f32, a: f32, rand_r: f32, rand_v: f32, rand_a: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetNanoProjectileParamsQuery {
                r: r,
                v: v,
                a: a,
                randR: rand_r,
                randV: rand_v,
                randA: rand_a,
            };
            let mut result = MaybeUninit::<sys::SetNanoProjectileParamsResult>::zeroed();
            let func = self.api.SetNanoProjectileParams.expect("SetNanoProjectileParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn preload_feature_def_model(&self, def_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::PreloadFeatureDefModelQuery {
                defID: def_id,
            };
            let mut result = MaybeUninit::<sys::PreloadFeatureDefModelResult>::zeroed();
            let func = self.api.PreloadFeatureDefModel.expect("PreloadFeatureDefModel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn preload_unit_def_model(&self, def_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::PreloadUnitDefModelQuery {
                defID: def_id,
            };
            let mut result = MaybeUninit::<sys::PreloadUnitDefModelResult>::zeroed();
            let func = self.api.PreloadUnitDefModel.expect("PreloadUnitDefModel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn select_unit_map(&self, unit_ids: &[i32], append: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SelectUnitMapQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
                append: append,
            };
            let mut result = MaybeUninit::<sys::SelectUnitMapResult>::zeroed();
            let func = self.api.SelectUnitMap.expect("SelectUnitMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn deselect_unit_map(&self, unit_ids: &[i32]) -> Result<bool, Error> {
        unsafe {
            let query = sys::DeselectUnitMapQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::DeselectUnitMapResult>::zeroed();
            let func = self.api.DeselectUnitMap.expect("DeselectUnitMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn draw_unit_commands(&self, unit_ids: &[i32], table_or_array: bool, queue_draw_depth: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::DrawUnitCommandsQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
                tableOrArray: table_or_array,
                queueDrawDepth: queue_draw_depth,
            };
            let mut result = MaybeUninit::<sys::DrawUnitCommandsResult>::zeroed();
            let func = self.api.DrawUnitCommands.expect("DrawUnitCommands function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_water_texture(&self, tex_type: &str, tex_name: &str) -> Result<bool, Error> {
        unsafe {
            let tex_type_cstr = std::ffi::CString::new(tex_type).map_err(|_| Error::invalid_argument("tex_type"))?;
            let tex_name_cstr = std::ffi::CString::new(tex_name).map_err(|_| Error::invalid_argument("tex_name"))?;
            let query = sys::SetWaterTextureQuery {
                texType: tex_type_cstr.as_ptr(),
                texName: tex_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetWaterTextureResult>::zeroed();
            let func = self.api.SetWaterTexture.expect("SetWaterTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_water_texture(&self, tex_type: &str) -> Result<Option<String>, Error> {
        unsafe {
            let tex_type_cstr = std::ffi::CString::new(tex_type).map_err(|_| Error::invalid_argument("tex_type"))?;
            let query = sys::GetWaterTextureQuery {
                texType: tex_type_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetWaterTextureResult>::zeroed();
            let func = self.api.GetWaterTexture.expect("GetWaterTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.texName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.texName).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

}

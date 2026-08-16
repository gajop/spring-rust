impl<'a> Display<'a> {
    pub fn get_num_displays(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetNumDisplaysQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetNumDisplaysResult>::zeroed();
            let func = self.api.GetNumDisplays.expect("GetNumDisplays function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_view_geometry(&self) -> Result<sys::ViewGeometry, Error> {
        unsafe {
            let query = sys::GetViewGeometryQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetViewGeometryResult>::zeroed();
            let func = self.api.GetViewGeometry.expect("GetViewGeometry function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.geom
            })
        }
    }

    pub fn get_dual_view_geometry(&self) -> Result<sys::ViewGeometry, Error> {
        unsafe {
            let query = sys::GetDualViewGeometryQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetDualViewGeometryResult>::zeroed();
            let func = self.api.GetDualViewGeometry.expect("GetDualViewGeometry function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.geom
            })
        }
    }

    pub fn get_window_geometry(&self) -> Result<sys::ViewGeometry, Error> {
        unsafe {
            let query = sys::GetWindowGeometryQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetWindowGeometryResult>::zeroed();
            let func = self.api.GetWindowGeometry.expect("GetWindowGeometry function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.geom
            })
        }
    }

    pub fn get_screen_geometry(&self, screen_num: i32, query_usable: bool) -> Result<sys::ViewGeometry, Error> {
        unsafe {
            let query = sys::GetScreenGeometryQuery {
                screenNum: screen_num,
                queryUsable: query_usable,
            };
            let mut result = MaybeUninit::<sys::GetScreenGeometryResult>::zeroed();
            let func = self.api.GetScreenGeometry.expect("GetScreenGeometry function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.geom
            })
        }
    }

    pub fn get_mini_map_geometry(&self) -> Result<sys::MinimapGeometry, Error> {
        unsafe {
            let query = sys::GetMiniMapGeometryQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMiniMapGeometryResult>::zeroed();
            let func = self.api.GetMiniMapGeometry.expect("GetMiniMapGeometry function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.geom
            })
        }
    }

    pub fn get_mini_map_dual_screen(&self) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let query = sys::GetMiniMapDualScreenQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMiniMapDualScreenResult>::zeroed();
            let func = self.api.GetMiniMapDualScreen.expect("GetMiniMapDualScreen function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.position.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.position).to_string_lossy().into_owned())
                    }
                },
                result.dualScreen,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_mini_map_rotation(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetMiniMapRotationQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMiniMapRotationResult>::zeroed();
            let func = self.api.GetMiniMapRotation.expect("GetMiniMapRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.rotation
            })
        }
    }

    pub fn get_draw_frame(&self) -> Result<(u32, u32), Error> {
        unsafe {
            let query = sys::GetDrawFrameQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetDrawFrameResult>::zeroed();
            let func = self.api.GetDrawFrame.expect("GetDrawFrame function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.low16,
                result.high16,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_frame_time_offset(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFrameTimeOffsetQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetFrameTimeOffsetResult>::zeroed();
            let func = self.api.GetFrameTimeOffset.expect("GetFrameTimeOffset function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.offset
            })
        }
    }

    pub fn get_last_update_seconds(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetLastUpdateSecondsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLastUpdateSecondsResult>::zeroed();
            let func = self.api.GetLastUpdateSeconds.expect("GetLastUpdateSeconds function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.seconds
            })
        }
    }

    pub fn get_fps(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetFPSQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetFPSResult>::zeroed();
            let func = self.api.GetFPS.expect("GetFPS function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.fps
            })
        }
    }

    pub fn get_map_draw_mode(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetMapDrawModeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMapDrawModeResult>::zeroed();
            let func = self.api.GetMapDrawMode.expect("GetMapDrawMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.mode.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.mode).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_water_mode(&self) -> Result<(i32, Option<String>), Error> {
        unsafe {
            let query = sys::GetWaterModeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetWaterModeResult>::zeroed();
            let func = self.api.GetWaterMode.expect("GetWaterMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.mode,
                {
                    if result.name.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.name).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_los_view_colors(&self) -> Result<(sys::Float3, sys::Float3, sys::Float3, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::GetLosViewColorsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLosViewColorsResult>::zeroed();
            let func = self.api.GetLosViewColors.expect("GetLosViewColors function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.alwaysColor,
                result.losColor,
                result.radarColor,
                result.jamColor,
                result.radarColor2,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_game_speed(&self) -> Result<(f32, f32, bool), Error> {
        unsafe {
            let query = sys::GetGameSpeedQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameSpeedResult>::zeroed();
            let func = self.api.GetGameSpeed.expect("GetGameSpeed function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.wantedSpeed,
                result.speed,
                result.paused,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_team_color(&self, team_id: i32) -> Result<sys::TeamColor, Error> {
        unsafe {
            let query = sys::GetTeamColorQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamColorResult>::zeroed();
            let func = self.api.GetTeamColor.expect("GetTeamColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.color
            })
        }
    }

    pub fn get_team_orig_color(&self, team_id: i32) -> Result<sys::TeamColor, Error> {
        unsafe {
            let query = sys::GetTeamOrigColorQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamOrigColorResult>::zeroed();
            let func = self.api.GetTeamOrigColor.expect("GetTeamOrigColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.color
            })
        }
    }

    pub fn is_aabbin_view(&self, mins: sys::Float3, maxs: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsAABBInViewQuery {
                mins: mins,
                maxs: maxs,
            };
            let mut result = MaybeUninit::<sys::IsAABBInViewResult>::zeroed();
            let func = self.api.IsAABBInView.expect("IsAABBInView function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inView
            })
        }
    }

    pub fn is_sphere_in_view(&self, center: sys::Float3, radius: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsSphereInViewQuery {
                center: center,
                radius: radius,
            };
            let mut result = MaybeUninit::<sys::IsSphereInViewResult>::zeroed();
            let func = self.api.IsSphereInView.expect("IsSphereInView function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inView
            })
        }
    }

    pub fn is_guihidden(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsGUIHiddenQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsGUIHiddenResult>::zeroed();
            let func = self.api.IsGUIHidden.expect("IsGUIHidden function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.hidden
            })
        }
    }

    pub fn have_shadows(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::HaveShadowsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::HaveShadowsResult>::zeroed();
            let func = self.api.HaveShadows.expect("HaveShadows function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn have_adv_shading(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::HaveAdvShadingQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::HaveAdvShadingResult>::zeroed();
            let func = self.api.HaveAdvShading.expect("HaveAdvShading function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn set_team_color(&self, team_id: i32, color: sys::TeamColor) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetTeamColorQuery {
                teamID: team_id,
                color: color,
            };
            let mut result = MaybeUninit::<sys::SetTeamColorResult>::zeroed();
            let func = self.api.SetTeamColor.expect("SetTeamColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

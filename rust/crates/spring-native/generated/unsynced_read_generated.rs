impl<'a> UnsyncedRead<'a> {
    pub fn get_clipboard(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetClipboardQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetClipboardResult>::zeroed();
            let func = self.api.GetClipboard.expect("GetClipboard function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.text).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_prev_frame_sync_checksum(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetPrevFrameSyncChecksumQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetPrevFrameSyncChecksumResult>::zeroed();
            let func = self.api.GetPrevFrameSyncChecksum.expect("GetPrevFrameSyncChecksum function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.checksum.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.checksum).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_active_cmd_desc(&self, cmd_index: i32) -> Result<(sys::ActiveCommandDescription, bool), Error> {
        unsafe {
            let query = sys::GetActiveCmdDescQuery {
                cmdIndex: cmd_index,
            };
            let mut result = MaybeUninit::<sys::GetActiveCmdDescResult>::zeroed();
            let func = self.api.GetActiveCmdDesc.expect("GetActiveCmdDesc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.cmdDesc,
                result.hasCommand,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_active_cmd_descs(&self) -> Result<Vec<sys::ActiveCommandDescription>, Error> {
        unsafe {
            let query = sys::GetActiveCmdDescsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetActiveCmdDescsResult>::zeroed();
            let func = self.api.GetActiveCmdDescs.expect("GetActiveCmdDescs function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.cmdDescs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.cmdDescs as *const sys::ActiveCommandDescription, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_cmd_desc_index(&self, cmd_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetCmdDescIndexQuery {
                cmdID: cmd_id,
            };
            let mut result = MaybeUninit::<sys::GetCmdDescIndexResult>::zeroed();
            let func = self.api.GetCmdDescIndex.expect("GetCmdDescIndex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.index
            })
        }
    }

    pub fn get_box_selection_by_engine(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetBoxSelectionByEngineQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetBoxSelectionByEngineResult>::zeroed();
            let func = self.api.GetBoxSelectionByEngine.expect("GetBoxSelectionByEngine function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.enabled
            })
        }
    }

    pub fn get_build_facing(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetBuildFacingQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetBuildFacingResult>::zeroed();
            let func = self.api.GetBuildFacing.expect("GetBuildFacing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.facing
            })
        }
    }

    pub fn get_build_spacing(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetBuildSpacingQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetBuildSpacingResult>::zeroed();
            let func = self.api.GetBuildSpacing.expect("GetBuildSpacing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.spacing
            })
        }
    }

    pub fn get_draw_selection_info(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetDrawSelectionInfoQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetDrawSelectionInfoResult>::zeroed();
            let func = self.api.GetDrawSelectionInfo.expect("GetDrawSelectionInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.draw
            })
        }
    }

    pub fn get_nano_projectile_params(&self) -> Result<(f32, f32, f32, f32, f32, f32), Error> {
        unsafe {
            let query = sys::GetNanoProjectileParamsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetNanoProjectileParamsResult>::zeroed();
            let func = self.api.GetNanoProjectileParams.expect("GetNanoProjectileParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.r,
                result.v,
                result.a,
                result.randR,
                result.randV,
                result.randA,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_piece_projectile_name(&self, projectile_id: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetPieceProjectileNameQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetPieceProjectileNameResult>::zeroed();
            let func = self.api.GetPieceProjectileName.expect("GetPieceProjectileName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.name.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.name).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_team_damage_stats(&self, team_id: i32) -> Result<(f32, f32, bool), Error> {
        unsafe {
            let query = sys::GetTeamDamageStatsQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamDamageStatsResult>::zeroed();
            let func = self.api.GetTeamDamageStats.expect("GetTeamDamageStats function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.damageDealt,
                result.damageReceived,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_last_message_positions(&self) -> Result<Vec<sys::Float3>, Error> {
        unsafe {
            let query = sys::GetLastMessagePositionsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLastMessagePositionsResult>::zeroed();
            let func = self.api.GetLastMessagePositions.expect("GetLastMessagePositions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.positions.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.positions as *const sys::Float3, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn solve_nurbscurve(&self, degree: i32, points: &[sys::Float4], knots: &[f32], segments: i32) -> Result<(Vec<sys::Float3>, bool), Error> {
        unsafe {
            let query = sys::SolveNURBSCurveQuery {
                degree: degree,
                points: points.as_ptr(),
                pointCount: points.len() as u32,
                knots: knots.as_ptr(),
                knotCount: knots.len() as u32,
                segments: segments,
            };
            let mut result = MaybeUninit::<sys::SolveNURBSCurveResult>::zeroed();
            let func = self.api.SolveNURBSCurve.expect("SolveNURBSCurve function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    let slice = if result.count == 0 || result.points.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.points as *const sys::Float3, result.count as usize)
                    };
                    slice.to_vec()
                },
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn is_unit_selected(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitSelectedQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitSelectedResult>::zeroed();
            let func = self.api.IsUnitSelected.expect("IsUnitSelected function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.selected
            })
        }
    }

    pub fn is_unit_allied(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitAlliedQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitAlliedResult>::zeroed();
            let func = self.api.IsUnitAllied.expect("IsUnitAllied function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allied
            })
        }
    }

    pub fn get_custom_palette_color(&self, index: i32) -> Result<(f32, f32, f32, bool), Error> {
        unsafe {
            let query = sys::GetCustomPaletteColorQuery {
                index: index,
            };
            let mut result = MaybeUninit::<sys::GetCustomPaletteColorResult>::zeroed();
            let func = self.api.GetCustomPaletteColor.expect("GetCustomPaletteColor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.r,
                result.g,
                result.b,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_palette_index(&self, unit_id: i32) -> Result<(i32, bool), Error> {
        unsafe {
            let query = sys::GetUnitPaletteIndexQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitPaletteIndexResult>::zeroed();
            let func = self.api.GetUnitPaletteIndex.expect("GetUnitPaletteIndex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.customIndex,
                result.usingCustomColor,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_feature_palette_index(&self, feature_id: i32) -> Result<(i32, bool), Error> {
        unsafe {
            let query = sys::GetFeaturePaletteIndexQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePaletteIndexResult>::zeroed();
            let func = self.api.GetFeaturePaletteIndex.expect("GetFeaturePaletteIndex function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.customIndex,
                result.usingCustomColor,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_game_seconds_interpolated(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetGameSecondsInterpolatedQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameSecondsInterpolatedResult>::zeroed();
            let func = self.api.GetGameSecondsInterpolated.expect("GetGameSecondsInterpolated function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.seconds
            })
        }
    }

}

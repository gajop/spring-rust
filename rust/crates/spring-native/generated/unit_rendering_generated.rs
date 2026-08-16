#[derive(Debug, Clone, Copy, Default)]
pub struct GetVisibleFeaturesOptions {
    pub include_icons: bool,
    pub include_geos: bool,
}

impl From<GetVisibleFeaturesOptions> for sys::GetVisibleFeaturesOptions {
    fn from(options: GetVisibleFeaturesOptions) -> Self {
        sys::GetVisibleFeaturesOptions {
            includeIcons: options.include_icons,
            includeGeos: options.include_geos,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetVisibleProjectilesOptions {
    pub include_synced_projectiles: bool,
    pub include_weapon_projectiles: bool,
    pub include_piece_projectiles: bool,
}

impl From<GetVisibleProjectilesOptions> for sys::GetVisibleProjectilesOptions {
    fn from(options: GetVisibleProjectilesOptions) -> Self {
        sys::GetVisibleProjectilesOptions {
            includeSyncedProjectiles: options.include_synced_projectiles,
            includeWeaponProjectiles: options.include_weapon_projectiles,
            includePieceProjectiles: options.include_piece_projectiles,
        }
    }
}

impl<'a> UnitRendering<'a> {
    pub fn get_unit_no_draw(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitNoDrawQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitNoDrawResult>::zeroed();
            let func = self.api.GetUnitNoDraw.expect("GetUnitNoDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.noDraw
            })
        }
    }

    pub fn get_unit_lua_draw(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitLuaDrawQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitLuaDrawResult>::zeroed();
            let func = self.api.GetUnitLuaDraw.expect("GetUnitLuaDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.luaDraw
            })
        }
    }

    pub fn get_unit_engine_draw_mask(&self, unit_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetUnitEngineDrawMaskQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitEngineDrawMaskResult>::zeroed();
            let func = self.api.GetUnitEngineDrawMask.expect("GetUnitEngineDrawMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.engineDrawMask
            })
        }
    }

    pub fn get_unit_always_update_matrix(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitAlwaysUpdateMatrixQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitAlwaysUpdateMatrixResult>::zeroed();
            let func = self.api.GetUnitAlwaysUpdateMatrix.expect("GetUnitAlwaysUpdateMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.alwaysUpdateMatrix
            })
        }
    }

    pub fn get_unit_draw_flag(&self, unit_id: i32) -> Result<u8, Error> {
        unsafe {
            let query = sys::GetUnitDrawFlagQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDrawFlagResult>::zeroed();
            let func = self.api.GetUnitDrawFlag.expect("GetUnitDrawFlag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.drawFlag
            })
        }
    }

    pub fn get_unit_no_select(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitNoSelectQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitNoSelectResult>::zeroed();
            let func = self.api.GetUnitNoSelect.expect("GetUnitNoSelect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.noSelect
            })
        }
    }

    pub fn get_unit_no_minimap(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitNoMinimapQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitNoMinimapResult>::zeroed();
            let func = self.api.GetUnitNoMinimap.expect("GetUnitNoMinimap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.noMinimap
            })
        }
    }

    pub fn get_unit_no_group(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitNoGroupQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitNoGroupResult>::zeroed();
            let func = self.api.GetUnitNoGroup.expect("GetUnitNoGroup function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.noGroup
            })
        }
    }

    pub fn get_unit_view_position(&self, unit_id: i32, use_mid_pos: bool) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitViewPositionQuery {
                unitID: unit_id,
                useMidPos: use_mid_pos,
            };
            let mut result = MaybeUninit::<sys::GetUnitViewPositionResult>::zeroed();
            let func = self.api.GetUnitViewPosition.expect("GetUnitViewPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_unit_transform_matrix(&self, unit_id: i32) -> Result<[f32; 16], Error> {
        unsafe {
            let query = sys::GetUnitTransformMatrixQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitTransformMatrixResult>::zeroed();
            let func = self.api.GetUnitTransformMatrix.expect("GetUnitTransformMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.matrix
            })
        }
    }

    pub fn get_unit_selection_volume_data(&self, unit_id: i32) -> Result<(sys::Float3, sys::Float3, i32, bool, i32, bool), Error> {
        unsafe {
            let query = sys::GetUnitSelectionVolumeDataQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitSelectionVolumeDataResult>::zeroed();
            let func = self.api.GetUnitSelectionVolumeData.expect("GetUnitSelectionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.scales,
                result.offsets,
                result.volumeType,
                result.useContHitTest,
                result.primaryAxis,
                result.ignoreHits,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_icon_data(&self, unit_id: i32, full_data: bool) -> Result<(Option<String>, [f32; 4], f32, f32, bool), Error> {
        unsafe {
            let query = sys::GetUnitIconDataQuery {
                unitID: unit_id,
                fullData: full_data,
            };
            let mut result = MaybeUninit::<sys::GetUnitIconDataResult>::zeroed();
            let func = self.api.GetUnitIconData.expect("GetUnitIconData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.iconName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.iconName).to_string_lossy().into_owned())
                    }
                },
                result.atlasTexCoords,
                result.size,
                result.distance,
                result.radiusAdjust,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_icon(&self, unit_id: i32) -> Result<(Option<String>, [f32; 4], f32, f32, bool), Error> {
        unsafe {
            let query = sys::GetUnitIconQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIconResult>::zeroed();
            let func = self.api.GetUnitIcon.expect("GetUnitIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.iconName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.iconName).to_string_lossy().into_owned())
                    }
                },
                result.atlasTexCoords,
                result.size,
                result.distance,
                result.radiusAdjust,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_camera_rotation(&self) -> Result<(f32, f32, f32), Error> {
        unsafe {
            let query = sys::GetCameraRotationQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetCameraRotationResult>::zeroed();
            let func = self.api.GetCameraRotation.expect("GetCameraRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.rotX,
                result.rotY,
                result.rotZ,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_camera_vectors(&self) -> Result<(sys::Float3, sys::Float3, sys::Float3), Error> {
        unsafe {
            let query = sys::GetCameraVectorsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetCameraVectorsResult>::zeroed();
            let func = self.api.GetCameraVectors.expect("GetCameraVectors function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.forward,
                result.up,
                result.right,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_frustum_planes(&self) -> Result<[f32; 16], Error> {
        unsafe {
            let query = sys::GetFrustumPlanesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetFrustumPlanesResult>::zeroed();
            let func = self.api.GetFrustumPlanes.expect("GetFrustumPlanes function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.planes
            })
        }
    }

    pub fn get_visible_units(&self, team_id: i32, radius: f32, include_icons: bool) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetVisibleUnitsQuery {
                teamID: team_id,
                radius: radius,
                includeIcons: include_icons,
            };
            let mut result = MaybeUninit::<sys::GetVisibleUnitsResult>::zeroed();
            let func = self.api.GetVisibleUnits.expect("GetVisibleUnits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.unitIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.unitIDs as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_visible_features(&self, ally_team_id: i32, radius: f32, options: GetVisibleFeaturesOptions) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetVisibleFeaturesQuery {
                allyTeamID: ally_team_id,
                radius: radius,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetVisibleFeaturesResult>::zeroed();
            let func = self.api.GetVisibleFeatures.expect("GetVisibleFeatures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.featureIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.featureIDs as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_visible_projectiles(&self, ally_team_id: i32, options: GetVisibleProjectilesOptions) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetVisibleProjectilesQuery {
                allyTeamID: ally_team_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetVisibleProjectilesResult>::zeroed();
            let func = self.api.GetVisibleProjectiles.expect("GetVisibleProjectiles function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.projectileIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.projectileIDs as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_units_in_screen_rectangle(&self, left: f32, top: f32, right: f32, bottom: f32, allegiance: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitsInScreenRectangleQuery {
                left: left,
                top: top,
                right: right,
                bottom: bottom,
                allegiance: allegiance,
            };
            let mut result = MaybeUninit::<sys::GetUnitsInScreenRectangleResult>::zeroed();
            let func = self.api.GetUnitsInScreenRectangle.expect("GetUnitsInScreenRectangle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.unitIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.unitIDs as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_features_in_screen_rectangle(&self, left: f32, top: f32, right: f32, bottom: f32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetFeaturesInScreenRectangleQuery {
                left: left,
                top: top,
                right: right,
                bottom: bottom,
            };
            let mut result = MaybeUninit::<sys::GetFeaturesInScreenRectangleResult>::zeroed();
            let func = self.api.GetFeaturesInScreenRectangle.expect("GetFeaturesInScreenRectangle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.featureIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.featureIDs as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn is_unit_visible(&self, unit_id: i32, radius: f32, check_icon: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitVisibleQuery {
                unitID: unit_id,
                radius: radius,
                checkIcon: check_icon,
            };
            let mut result = MaybeUninit::<sys::IsUnitVisibleResult>::zeroed();
            let func = self.api.IsUnitVisible.expect("IsUnitVisible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.visible
            })
        }
    }

    pub fn is_unit_in_view(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitInViewQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitInViewResult>::zeroed();
            let func = self.api.IsUnitInView.expect("IsUnitInView function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inView
            })
        }
    }

    pub fn is_unit_icon(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUnitIconQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::IsUnitIconResult>::zeroed();
            let func = self.api.IsUnitIcon.expect("IsUnitIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isIcon
            })
        }
    }

}

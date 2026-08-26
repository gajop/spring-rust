#[derive(Debug, Clone, Copy, Default)]
pub struct GetClosestEnemyUnitOptions {
    pub use_los: bool,
    pub sphere_dist_test: bool,
    pub check_sight_dist: bool,
}

impl From<GetClosestEnemyUnitOptions> for sys::GetClosestEnemyUnitOptions {
    fn from(options: GetClosestEnemyUnitOptions) -> Self {
        sys::GetClosestEnemyUnitOptions {
            useLOS: options.use_los,
            sphereDistTest: options.sphere_dist_test,
            checkSightDist: options.check_sight_dist,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetUnitNearestEnemyOptions {
    pub use_los: bool,
    pub sphere_dist_test: bool,
    pub check_sight_dist: bool,
}

impl From<GetUnitNearestEnemyOptions> for sys::GetUnitNearestEnemyOptions {
    fn from(options: GetUnitNearestEnemyOptions) -> Self {
        sys::GetUnitNearestEnemyOptions {
            useLOS: options.use_los,
            sphereDistTest: options.sphere_dist_test,
            checkSightDist: options.check_sight_dist,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetUnitSeparationOptions {
    pub positional: bool,
    pub check_map: bool,
}

impl From<GetUnitSeparationOptions> for sys::GetUnitSeparationOptions {
    fn from(options: GetUnitSeparationOptions) -> Self {
        sys::GetUnitSeparationOptions {
            positional: options.positional,
            checkMap: options.check_map,
        }
    }
}

impl<'a> UnitsQuery<'a> {
    pub fn valid_unit_id(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::ValidUnitIDQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::ValidUnitIDResult>::zeroed();
            let func = self.api.ValidUnitID.expect("ValidUnitID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.valid
            })
        }
    }

    pub fn get_all_units(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetAllUnitsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetAllUnitsResult>::zeroed();
            let func = self.api.GetAllUnits.expect("GetAllUnits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_team_units(&self, team_id: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetTeamUnitsQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamUnitsResult>::zeroed();
            let func = self.api.GetTeamUnits.expect("GetTeamUnits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_team_units_sorted(&self, team_id: i32) -> Result<Vec<sys::TeamUnitsByDef>, Error> {
        unsafe {
            let query = sys::GetTeamUnitsSortedQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamUnitsSortedResult>::zeroed();
            let func = self.api.GetTeamUnitsSorted.expect("GetTeamUnitsSorted function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.groups.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.groups as *const sys::TeamUnitsByDef, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_team_units_counts(&self, team_id: i32) -> Result<Vec<sys::UnitDefCount>, Error> {
        unsafe {
            let query = sys::GetTeamUnitsCountsQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamUnitsCountsResult>::zeroed();
            let func = self.api.GetTeamUnitsCounts.expect("GetTeamUnitsCounts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.counts.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.counts as *const sys::UnitDefCount, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_team_units_by_defs(&self, team_id: i32, unit_def_ids: &[i32]) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetTeamUnitsByDefsQuery {
                teamID: team_id,
                unitDefIDs: unit_def_ids.as_ptr(),
                defCount: unit_def_ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GetTeamUnitsByDefsResult>::zeroed();
            let func = self.api.GetTeamUnitsByDefs.expect("GetTeamUnitsByDefs function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_team_unit_def_count(&self, team_id: i32, unit_def_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetTeamUnitDefCountQuery {
                teamID: team_id,
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamUnitDefCountResult>::zeroed();
            let func = self.api.GetTeamUnitDefCount.expect("GetTeamUnitDefCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_team_unit_count(&self, team_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetTeamUnitCountQuery {
                teamID: team_id,
            };
            let mut result = MaybeUninit::<sys::GetTeamUnitCountResult>::zeroed();
            let func = self.api.GetTeamUnitCount.expect("GetTeamUnitCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_units_in_rectangle(&self, xmin: f32, zmin: f32, xmax: f32, zmax: f32, allegiance: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitsInRectangleQuery {
                xmin,
                zmin,
                xmax,
                zmax,
                allegiance,
            };
            let mut result = MaybeUninit::<sys::GetUnitsInRectangleResult>::zeroed();
            let func = self.api.GetUnitsInRectangle.expect("GetUnitsInRectangle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    #[expect(clippy::too_many_arguments, reason = "NativeInterface preserves the corresponding Lua API arity")]
    pub fn get_units_in_box(&self, xmin: f32, ymin: f32, zmin: f32, xmax: f32, ymax: f32, zmax: f32, allegiance: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitsInBoxQuery {
                xmin,
                ymin,
                zmin,
                xmax,
                ymax,
                zmax,
                allegiance,
            };
            let mut result = MaybeUninit::<sys::GetUnitsInBoxResult>::zeroed();
            let func = self.api.GetUnitsInBox.expect("GetUnitsInBox function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_units_in_planes(&self, planes: sys::PlanesQuery, allegiance: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitsInPlanesQuery {
                planes,
                allegiance,
            };
            let mut result = MaybeUninit::<sys::GetUnitsInPlanesResult>::zeroed();
            let func = self.api.GetUnitsInPlanes.expect("GetUnitsInPlanes function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_units_in_sphere(&self, x: f32, y: f32, z: f32, radius: f32, allegiance: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitsInSphereQuery {
                x,
                y,
                z,
                radius,
                allegiance,
            };
            let mut result = MaybeUninit::<sys::GetUnitsInSphereResult>::zeroed();
            let func = self.api.GetUnitsInSphere.expect("GetUnitsInSphere function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_units_in_cylinder(&self, x: f32, z: f32, radius: f32, allegiance: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitsInCylinderQuery {
                x,
                z,
                radius,
                allegiance,
            };
            let mut result = MaybeUninit::<sys::GetUnitsInCylinderResult>::zeroed();
            let func = self.api.GetUnitsInCylinder.expect("GetUnitsInCylinder function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_array_centroid(&self, unit_ids: &[i32]) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitArrayCentroidQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GetUnitArrayCentroidResult>::zeroed();
            let func = self.api.GetUnitArrayCentroid.expect("GetUnitArrayCentroid function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.centroid
            })
        }
    }

    pub fn get_unit_map_centroid(&self, unit_ids: &[i32]) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitMapCentroidQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GetUnitMapCentroidResult>::zeroed();
            let func = self.api.GetUnitMapCentroid.expect("GetUnitMapCentroid function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.centroid
            })
        }
    }

    pub fn get_unit_nearest_ally(&self, unit_id: i32, range: f32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitNearestAllyQuery {
                unitID: unit_id,
                range,
            };
            let mut result = MaybeUninit::<sys::GetUnitNearestAllyResult>::zeroed();
            let func = self.api.GetUnitNearestAlly.expect("GetUnitNearestAlly function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitID
            })
        }
    }

    pub fn get_unit_nearest_enemy(&self, unit_id: i32, range: f32, options: GetUnitNearestEnemyOptions) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitNearestEnemyQuery {
                unitID: unit_id,
                range,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetUnitNearestEnemyResult>::zeroed();
            let func = self.api.GetUnitNearestEnemy.expect("GetUnitNearestEnemy function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitID
            })
        }
    }

    pub fn get_closest_enemy_unit(&self, pos: sys::Float3, range: f32, ally_team_id: i32, options: GetClosestEnemyUnitOptions) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetClosestEnemyUnitQuery {
                pos,
                range,
                allyTeamID: ally_team_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetClosestEnemyUnitResult>::zeroed();
            let func = self.api.GetClosestEnemyUnit.expect("GetClosestEnemyUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitID
            })
        }
    }

    pub fn get_unit_separation(&self, unit_id1: i32, unit_id2: i32, options: GetUnitSeparationOptions) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitSeparationQuery {
                unitID1: unit_id1,
                unitID2: unit_id2,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetUnitSeparationResult>::zeroed();
            let func = self.api.GetUnitSeparation.expect("GetUnitSeparation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.separation
            })
        }
    }

    pub fn get_render_units(&self, draw_mask: i32, send_mask: bool) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetRenderUnitsQuery {
                drawMask: draw_mask,
                sendMask: send_mask,
            };
            let mut result = MaybeUninit::<sys::GetRenderUnitsResult>::zeroed();
            let func = self.api.GetRenderUnits.expect("GetRenderUnits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_render_units_draw_flag_changed(&self, send_mask: bool) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetRenderUnitsDrawFlagChangedQuery {
                sendMask: send_mask,
            };
            let mut result = MaybeUninit::<sys::GetRenderUnitsDrawFlagChangedResult>::zeroed();
            let func = self.api.GetRenderUnitsDrawFlagChanged.expect("GetRenderUnitsDrawFlagChanged function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.units.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.units as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

}

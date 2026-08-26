#[derive(Debug, Clone, Copy, Default)]
pub struct TestMoveOrderOptions {
    pub test_terrain: bool,
    pub test_objects: bool,
    pub center_only: bool,
}

impl From<TestMoveOrderOptions> for sys::TestMoveOrderOptions {
    fn from(options: TestMoveOrderOptions) -> Self {
        sys::TestMoveOrderOptions {
            testTerrain: options.test_terrain,
            testObjects: options.test_objects,
            centerOnly: options.center_only,
        }
    }
}

impl<'a> Utils<'a> {
    pub fn get_cegid(&self, ceg_name: &str) -> Result<i32, Error> {
        unsafe {
            let ceg_name_cstr = std::ffi::CString::new(ceg_name).map_err(|_| Error::invalid_argument("ceg_name"))?;
            let query = sys::GetCEGIDQuery {
                cegName: ceg_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetCEGIDResult>::zeroed();
            let func = self.api.GetCEGID.expect("GetCEGID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.cegID
            })
        }
    }

    pub fn test_build_order(&self, unit_def_id: i32, pos: sys::Float3, facing: i32) -> Result<(i32, bool, i32), Error> {
        unsafe {
            let query = sys::TestBuildOrderQuery {
                unitDefID: unit_def_id,
                pos,
                facing,
            };
            let mut result = MaybeUninit::<sys::TestBuildOrderResult>::zeroed();
            let func = self.api.TestBuildOrder.expect("TestBuildOrder function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.status,
                result.canBuild,
                result.feature,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn pos2_build_pos(&self, unit_def_id: i32, pos: sys::Float3, facing: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::Pos2BuildPosQuery {
                unitDefID: unit_def_id,
                pos,
                facing,
            };
            let mut result = MaybeUninit::<sys::Pos2BuildPosResult>::zeroed();
            let func = self.api.Pos2BuildPos.expect("Pos2BuildPos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.buildPos
            })
        }
    }

    pub fn closest_build_pos(&self, team_id: i32, unit_def_id: i32, pos: sys::Float3, search_radius: f32, min_dist: i32, facing: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::ClosestBuildPosQuery {
                teamID: team_id,
                unitDefID: unit_def_id,
                pos,
                searchRadius: search_radius,
                minDist: min_dist,
                facing,
            };
            let mut result = MaybeUninit::<sys::ClosestBuildPosResult>::zeroed();
            let func = self.api.ClosestBuildPos.expect("ClosestBuildPos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.buildPos
            })
        }
    }

    pub fn test_move_order(&self, unit_def_id: i32, pos: sys::Float3, dir: sys::Float3, options: TestMoveOrderOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::TestMoveOrderQuery {
                unitDefID: unit_def_id,
                pos,
                dir,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::TestMoveOrderResult>::zeroed();
            let func = self.api.TestMoveOrder.expect("TestMoveOrder function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.canMove
            })
        }
    }

    pub fn get_unit_def_dimensions(&self, unit_def_id: i32) -> Result<sys::UnitDefDimensions, Error> {
        unsafe {
            let query = sys::GetUnitDefDimensionsQuery {
                unitDefID: unit_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefDimensionsResult>::zeroed();
            let func = self.api.GetUnitDefDimensions.expect("GetUnitDefDimensions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.dimensions
            })
        }
    }

    pub fn get_feature_def_dimensions(&self, feature_def_id: i32) -> Result<sys::UnitDefDimensions, Error> {
        unsafe {
            let query = sys::GetFeatureDefDimensionsQuery {
                featureDefID: feature_def_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefDimensionsResult>::zeroed();
            let func = self.api.GetFeatureDefDimensions.expect("GetFeatureDefDimensions function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.dimensions
            })
        }
    }

}

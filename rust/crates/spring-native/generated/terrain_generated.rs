/// The complete result tuple returned by [`get_ground_info`].
pub type GetGroundInfoValue = (i32, Option<String>, f32, f32, f32, f32, f32, f32, bool);

/// The complete result tuple returned by [`get_terrain_type_data`].
pub type GetTerrainTypeDataValue = (i32, Option<String>, f32, f32, f32, f32, f32, bool);

/// The complete result tuple returned by [`get_ground_extremes`].
pub type GetGroundExtremesValue = (f32, f32, f32, f32);

impl<'a> Terrain<'a> {
    pub fn is_pos_in_map(&self, x: f32, z: f32) -> Result<(bool, bool), Error> {
        unsafe {
            let query = sys::IsPosInMapQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::IsPosInMapResult>::zeroed();
            let func = self.api.IsPosInMap.expect("IsPosInMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.inMap,
                result.inPlayArea,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_height(&self, x: f32, z: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetGroundHeightQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetGroundHeightResult>::zeroed();
            let func = self.api.GetGroundHeight.expect("GetGroundHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.height
            })
        }
    }

    pub fn get_ground_orig_height(&self, x: f32, z: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetGroundOrigHeightQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetGroundOrigHeightResult>::zeroed();
            let func = self.api.GetGroundOrigHeight.expect("GetGroundOrigHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.height
            })
        }
    }

    pub fn get_smooth_mesh_height(&self, x: f32, z: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetSmoothMeshHeightQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetSmoothMeshHeightResult>::zeroed();
            let func = self.api.GetSmoothMeshHeight.expect("GetSmoothMeshHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.height
            })
        }
    }

    pub fn get_water_plane_level(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetWaterPlaneLevelQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetWaterPlaneLevelResult>::zeroed();
            let func = self.api.GetWaterPlaneLevel.expect("GetWaterPlaneLevel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.level
            })
        }
    }

    pub fn get_water_level(&self, x: f32, z: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetWaterLevelQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetWaterLevelResult>::zeroed();
            let func = self.api.GetWaterLevel.expect("GetWaterLevel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.level
            })
        }
    }

    pub fn get_ground_normal(&self, x: f32, z: f32, smoothed: bool) -> Result<(sys::Float3, f32), Error> {
        unsafe {
            let query = sys::GetGroundNormalQuery {
                x,
                z,
                smoothed,
            };
            let mut result = MaybeUninit::<sys::GetGroundNormalResult>::zeroed();
            let func = self.api.GetGroundNormal.expect("GetGroundNormal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.normal,
                result.slope,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_info(&self, x: f32, z: f32) -> Result<GetGroundInfoValue, Error> {
        unsafe {
            let query = sys::GetGroundInfoQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetGroundInfoResult>::zeroed();
            let func = self.api.GetGroundInfo.expect("GetGroundInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.terrainTypeIndex,
                {
                    if result.terrainTypeName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.terrainTypeName).to_string_lossy().into_owned())
                    }
                },
                result.metalExtraction,
                result.hardness,
                result.tankSpeed,
                result.kbotSpeed,
                result.hoverSpeed,
                result.shipSpeed,
                result.receiveTracks,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_terrain_type_data(&self, terrain_type_index: i32) -> Result<GetTerrainTypeDataValue, Error> {
        unsafe {
            let query = sys::GetTerrainTypeDataQuery {
                terrainTypeIndex: terrain_type_index,
            };
            let mut result = MaybeUninit::<sys::GetTerrainTypeDataResult>::zeroed();
            let func = self.api.GetTerrainTypeData.expect("GetTerrainTypeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.index,
                {
                    if result.name.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.name).to_string_lossy().into_owned())
                    }
                },
                result.hardness,
                result.tankSpeed,
                result.kbotSpeed,
                result.hoverSpeed,
                result.shipSpeed,
                result.receiveTracks,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_extremes(&self) -> Result<GetGroundExtremesValue, Error> {
        unsafe {
            let query = sys::GetGroundExtremesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGroundExtremesResult>::zeroed();
            let func = self.api.GetGroundExtremes.expect("GetGroundExtremes function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.initMinHeight,
                result.initMaxHeight,
                result.currMinHeight,
                result.currMaxHeight,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_height_map_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let query = sys::GetHeightMapSizeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetHeightMapSizeResult>::zeroed();
            let func = self.api.GetHeightMapSize.expect("GetHeightMapSize function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.pointsX,
                result.pointsZ,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_blocked(&self, x1: f32, z1: f32, x2: f32, z2: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetGroundBlockedQuery {
                x1,
                z1,
                x2,
                z2,
            };
            let mut result = MaybeUninit::<sys::GetGroundBlockedResult>::zeroed();
            let func = self.api.GetGroundBlocked.expect("GetGroundBlocked function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.blocked
            })
        }
    }

    pub fn get_grass(&self, x: f32, z: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetGrassQuery {
                x,
                z,
            };
            let mut result = MaybeUninit::<sys::GetGrassResult>::zeroed();
            let func = self.api.GetGrass.expect("GetGrass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.grassLevel
            })
        }
    }

}

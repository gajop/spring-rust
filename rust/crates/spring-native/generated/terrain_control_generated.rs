impl<'a> TerrainControl<'a> {
    pub fn add_height_map(&self, x: f32, z: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddHeightMapQuery {
                x: x,
                z: z,
                height: height,
            };
            let mut result = MaybeUninit::<sys::AddHeightMapResult>::zeroed();
            let func = self.api.AddHeightMap.expect("AddHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_height_map(&self, x: f32, z: f32, height: f32, terraform: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetHeightMapQuery {
                x: x,
                z: z,
                height: height,
                terraform: terraform,
            };
            let mut result = MaybeUninit::<sys::SetHeightMapResult>::zeroed();
            let func = self.api.SetHeightMap.expect("SetHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn revert_height_map(&self, x1: f32, z1: f32, x2: f32, z2: f32, orig_factor: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RevertHeightMapQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                origFactor: orig_factor,
            };
            let mut result = MaybeUninit::<sys::RevertHeightMapResult>::zeroed();
            let func = self.api.RevertHeightMap.expect("RevertHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_smooth_mesh(&self, x: f32, z: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddSmoothMeshQuery {
                x: x,
                z: z,
                height: height,
            };
            let mut result = MaybeUninit::<sys::AddSmoothMeshResult>::zeroed();
            let func = self.api.AddSmoothMesh.expect("AddSmoothMesh function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_smooth_mesh(&self, x: f32, z: f32, height: f32, terraform: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetSmoothMeshQuery {
                x: x,
                z: z,
                height: height,
                terraform: terraform,
            };
            let mut result = MaybeUninit::<sys::SetSmoothMeshResult>::zeroed();
            let func = self.api.SetSmoothMesh.expect("SetSmoothMesh function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn revert_smooth_mesh(&self, x1: f32, z1: f32, x2: f32, z2: f32, orig_factor: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RevertSmoothMeshQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                origFactor: orig_factor,
            };
            let mut result = MaybeUninit::<sys::RevertSmoothMeshResult>::zeroed();
            let func = self.api.RevertSmoothMesh.expect("RevertSmoothMesh function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_map_square_terrain_type(&self, x: i32, z: i32, terrain_type: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetMapSquareTerrainTypeQuery {
                x: x,
                z: z,
                terrainType: terrain_type,
            };
            let mut result = MaybeUninit::<sys::SetMapSquareTerrainTypeResult>::zeroed();
            let func = self.api.SetMapSquareTerrainType.expect("SetMapSquareTerrainType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_terrain_type_data(&self, type_index: i32, tank_speed: f32, kbot_speed: f32, hover_speed: f32, ship_speed: f32, hardness: f32, receive_tracks: bool, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::SetTerrainTypeDataQuery {
                typeIndex: type_index,
                tankSpeed: tank_speed,
                kbotSpeed: kbot_speed,
                hoverSpeed: hover_speed,
                shipSpeed: ship_speed,
                hardness: hardness,
                receiveTracks: receive_tracks,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetTerrainTypeDataResult>::zeroed();
            let func = self.api.SetTerrainTypeData.expect("SetTerrainTypeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_tidal(&self, tidal: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetTidalQuery {
                tidal: tidal,
            };
            let mut result = MaybeUninit::<sys::SetTidalResult>::zeroed();
            let func = self.api.SetTidal.expect("SetTidal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_wind(&self, min_wind: f32, max_wind: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetWindQuery {
                minWind: min_wind,
                maxWind: max_wind,
            };
            let mut result = MaybeUninit::<sys::SetWindResult>::zeroed();
            let func = self.api.SetWind.expect("SetWind function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_grass(&self, x: f32, z: f32, grass_value: u8) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddGrassQuery {
                x: x,
                z: z,
                grassValue: grass_value,
            };
            let mut result = MaybeUninit::<sys::AddGrassResult>::zeroed();
            let func = self.api.AddGrass.expect("AddGrass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn remove_grass(&self, x: f32, z: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RemoveGrassQuery {
                x: x,
                z: z,
            };
            let mut result = MaybeUninit::<sys::RemoveGrassResult>::zeroed();
            let func = self.api.RemoveGrass.expect("RemoveGrass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn adjust_height_map(&self, x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AdjustHeightMapQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                height: height,
            };
            let mut result = MaybeUninit::<sys::AdjustHeightMapResult>::zeroed();
            let func = self.api.AdjustHeightMap.expect("AdjustHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn level_height_map(&self, x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::LevelHeightMapQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                height: height,
            };
            let mut result = MaybeUninit::<sys::LevelHeightMapResult>::zeroed();
            let func = self.api.LevelHeightMap.expect("LevelHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_original_height_map(&self, x: f32, z: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddOriginalHeightMapQuery {
                x: x,
                z: z,
                height: height,
            };
            let mut result = MaybeUninit::<sys::AddOriginalHeightMapResult>::zeroed();
            let func = self.api.AddOriginalHeightMap.expect("AddOriginalHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_original_height_map(&self, x: f32, z: f32, height: f32, factor: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetOriginalHeightMapQuery {
                x: x,
                z: z,
                height: height,
                factor: factor,
            };
            let mut result = MaybeUninit::<sys::SetOriginalHeightMapResult>::zeroed();
            let func = self.api.SetOriginalHeightMap.expect("SetOriginalHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn revert_original_height_map(&self, x1: f32, z1: f32, x2: f32, z2: f32, orig_factor: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RevertOriginalHeightMapQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                origFactor: orig_factor,
            };
            let mut result = MaybeUninit::<sys::RevertOriginalHeightMapResult>::zeroed();
            let func = self.api.RevertOriginalHeightMap.expect("RevertOriginalHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn adjust_original_height_map(&self, x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AdjustOriginalHeightMapQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                height: height,
            };
            let mut result = MaybeUninit::<sys::AdjustOriginalHeightMapResult>::zeroed();
            let func = self.api.AdjustOriginalHeightMap.expect("AdjustOriginalHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn level_original_height_map(&self, x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::LevelOriginalHeightMapQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                height: height,
            };
            let mut result = MaybeUninit::<sys::LevelOriginalHeightMapResult>::zeroed();
            let func = self.api.LevelOriginalHeightMap.expect("LevelOriginalHeightMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn adjust_smooth_mesh(&self, x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AdjustSmoothMeshQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                height: height,
            };
            let mut result = MaybeUninit::<sys::AdjustSmoothMeshResult>::zeroed();
            let func = self.api.AdjustSmoothMesh.expect("AdjustSmoothMesh function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn level_smooth_mesh(&self, x1: f32, z1: f32, x2: f32, z2: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::LevelSmoothMeshQuery {
                x1: x1,
                z1: z1,
                x2: x2,
                z2: z2,
                height: height,
            };
            let mut result = MaybeUninit::<sys::LevelSmoothMeshResult>::zeroed();
            let func = self.api.LevelSmoothMesh.expect("LevelSmoothMesh function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn rebuild_smooth_mesh(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::RebuildSmoothMeshQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::RebuildSmoothMeshResult>::zeroed();
            let func = self.api.RebuildSmoothMesh.expect("RebuildSmoothMesh function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_height_map_func<F: FnMut()>(&self, mut callback: F) -> Result<bool, Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = &mut *(user_data as *mut F);
                f();
            }
            let query = sys::SetHeightMapFuncQuery {
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::SetHeightMapFuncResult>::zeroed();
            let func = self.api.SetHeightMapFunc.expect("SetHeightMapFunc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_original_height_map_func<F: FnMut()>(&self, mut callback: F) -> Result<bool, Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = &mut *(user_data as *mut F);
                f();
            }
            let query = sys::SetOriginalHeightMapFuncQuery {
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::SetOriginalHeightMapFuncResult>::zeroed();
            let func = self.api.SetOriginalHeightMapFunc.expect("SetOriginalHeightMapFunc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_smooth_mesh_func<F: FnMut()>(&self, mut callback: F) -> Result<bool, Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = &mut *(user_data as *mut F);
                f();
            }
            let query = sys::SetSmoothMeshFuncQuery {
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::SetSmoothMeshFuncResult>::zeroed();
            let func = self.api.SetSmoothMeshFunc.expect("SetSmoothMeshFunc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

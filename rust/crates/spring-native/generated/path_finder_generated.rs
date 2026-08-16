impl<'a> PathFinder<'a> {
    pub fn request_path(&self, move_def_id: u32, move_def_name: Option<&str>, start_pos: sys::Float3, end_pos: sys::Float3, radius: f32) -> Result<u32, Error> {
        unsafe {
            let move_def_name_cstr = move_def_name.as_ref().map(|value| std::ffi::CString::new(*value)).transpose().map_err(|_| Error::invalid_argument("move_def_name"))?;
            let query = sys::RequestPathQuery {
                moveDefID: move_def_id,
                moveDefName: move_def_name_cstr.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
                hasMoveDefName: move_def_name.is_some(),
                startPos: start_pos,
                endPos: end_pos,
                radius: radius,
            };
            let mut result = MaybeUninit::<sys::RequestPathResult>::zeroed();
            let func = self.api.RequestPath.expect("RequestPath function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.pathID
            })
        }
    }

    pub fn delete_path(&self, path_id: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::DeletePathQuery {
                pathID: path_id,
            };
            let mut result = MaybeUninit::<sys::DeletePathResult>::zeroed();
            let func = self.api.DeletePath.expect("DeletePath function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_path_way_points(&self, path_id: u32) -> Result<(Vec<sys::Float3>, Vec<i32>), Error> {
        unsafe {
            let query = sys::GetPathWayPointsQuery {
                pathID: path_id,
            };
            let mut result = MaybeUninit::<sys::GetPathWayPointsResult>::zeroed();
            let func = self.api.GetPathWayPoints.expect("GetPathWayPoints function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    let slice = if result.pointCount == 0 || result.points.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.points as *const sys::Float3, result.pointCount as usize)
                    };
                    slice.to_vec()
                },
                {
                    let slice = if result.startCount == 0 || result.starts.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.starts as *const i32, result.startCount as usize)
                    };
                    slice.to_vec()
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_next_way_point(&self, path_id: u32, caller_pos: sys::Float3, min_dist: f32) -> Result<(sys::Float3, bool), Error> {
        unsafe {
            let query = sys::GetNextWayPointQuery {
                pathID: path_id,
                callerPos: caller_pos,
                minDist: min_dist,
            };
            let mut result = MaybeUninit::<sys::GetNextWayPointResult>::zeroed();
            let func = self.api.GetNextWayPoint.expect("GetNextWayPoint function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.waypoint,
                result.hasWaypoint,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn init_path_node_costs_array(&self, overlay_index: u32, size_x: u32, size_z: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::InitPathNodeCostsArrayQuery {
                overlayIndex: overlay_index,
                sizeX: size_x,
                sizeZ: size_z,
            };
            let mut result = MaybeUninit::<sys::InitPathNodeCostsArrayResult>::zeroed();
            let func = self.api.InitPathNodeCostsArray.expect("InitPathNodeCostsArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn free_path_node_costs_array(&self, overlay_index: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::FreePathNodeCostsArrayQuery {
                overlayIndex: overlay_index,
            };
            let mut result = MaybeUninit::<sys::FreePathNodeCostsArrayResult>::zeroed();
            let func = self.api.FreePathNodeCostsArray.expect("FreePathNodeCostsArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_path_node_costs(&self, overlay_index: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetPathNodeCostsQuery {
                overlayIndex: overlay_index,
            };
            let mut result = MaybeUninit::<sys::SetPathNodeCostsResult>::zeroed();
            let func = self.api.SetPathNodeCosts.expect("SetPathNodeCosts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_path_node_costs(&self, overlay_index: u32) -> Result<Vec<f32>, Error> {
        unsafe {
            let query = sys::GetPathNodeCostsQuery {
                overlayIndex: overlay_index,
            };
            let mut result = MaybeUninit::<sys::GetPathNodeCostsResult>::zeroed();
            let func = self.api.GetPathNodeCosts.expect("GetPathNodeCosts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.costs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.costs as *const f32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn set_path_node_cost(&self, overlay_index: u32, cost_index: u32, cost: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetPathNodeCostQuery {
                overlayIndex: overlay_index,
                costIndex: cost_index,
                cost: cost,
            };
            let mut result = MaybeUninit::<sys::SetPathNodeCostResult>::zeroed();
            let func = self.api.SetPathNodeCost.expect("SetPathNodeCost function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_path_node_cost(&self, x: u32, z: u32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetPathNodeCostQuery {
                x: x,
                z: z,
            };
            let mut result = MaybeUninit::<sys::GetPathNodeCostResult>::zeroed();
            let func = self.api.GetPathNodeCost.expect("GetPathNodeCost function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.cost
            })
        }
    }

}

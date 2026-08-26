impl<'a> Selection<'a> {
    pub fn get_selected_units(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetSelectedUnitsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSelectedUnitsResult>::zeroed();
            let func = self.api.GetSelectedUnits.expect("GetSelectedUnits function pointer must be initialized");
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

    pub fn get_selected_units_sorted(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetSelectedUnitsSortedQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSelectedUnitsSortedResult>::zeroed();
            let func = self.api.GetSelectedUnitsSorted.expect("GetSelectedUnitsSorted function pointer must be initialized");
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

    pub fn get_selected_units_counts(&self) -> Result<sys::SelectionCounts, Error> {
        unsafe {
            let query = sys::GetSelectedUnitsCountsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSelectedUnitsCountsResult>::zeroed();
            let func = self.api.GetSelectedUnitsCounts.expect("GetSelectedUnitsCounts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.counts
            })
        }
    }

    pub fn get_selected_units_count(&self) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetSelectedUnitsCountQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSelectedUnitsCountResult>::zeroed();
            let func = self.api.GetSelectedUnitsCount.expect("GetSelectedUnitsCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn select_unit(&self, unit_id: i32, append: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SelectUnitQuery {
                unitID: unit_id,
                append,
            };
            let mut result = MaybeUninit::<sys::SelectUnitResult>::zeroed();
            let func = self.api.SelectUnit.expect("SelectUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn select_unit_array(&self, unit_ids: &[i32], append: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SelectUnitArrayQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
                append,
            };
            let mut result = MaybeUninit::<sys::SelectUnitArrayResult>::zeroed();
            let func = self.api.SelectUnitArray.expect("SelectUnitArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn deselect_unit(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::DeselectUnitQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::DeselectUnitResult>::zeroed();
            let func = self.api.DeselectUnit.expect("DeselectUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn deselect_unit_array(&self, unit_ids: &[i32]) -> Result<bool, Error> {
        unsafe {
            let query = sys::DeselectUnitArrayQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::DeselectUnitArrayResult>::zeroed();
            let func = self.api.DeselectUnitArray.expect("DeselectUnitArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_group_list(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetGroupListQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGroupListResult>::zeroed();
            let func = self.api.GetGroupList.expect("GetGroupList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.groups.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.groups as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_selected_group(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetSelectedGroupQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSelectedGroupResult>::zeroed();
            let func = self.api.GetSelectedGroup.expect("GetSelectedGroup function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.groupID
            })
        }
    }

    pub fn get_group_units(&self, group_id: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetGroupUnitsQuery {
                groupID: group_id,
            };
            let mut result = MaybeUninit::<sys::GetGroupUnitsResult>::zeroed();
            let func = self.api.GetGroupUnits.expect("GetGroupUnits function pointer must be initialized");
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

    pub fn get_group_units_sorted(&self, group_id: i32) -> Result<Vec<sys::TeamUnitsByDef>, Error> {
        unsafe {
            let query = sys::GetGroupUnitsSortedQuery {
                groupID: group_id,
            };
            let mut result = MaybeUninit::<sys::GetGroupUnitsSortedResult>::zeroed();
            let func = self.api.GetGroupUnitsSorted.expect("GetGroupUnitsSorted function pointer must be initialized");
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

    pub fn get_group_units_count(&self, group_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetGroupUnitsCountQuery {
                groupID: group_id,
            };
            let mut result = MaybeUninit::<sys::GetGroupUnitsCountResult>::zeroed();
            let func = self.api.GetGroupUnitsCount.expect("GetGroupUnitsCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_group_units_counts(&self, group_id: i32) -> Result<sys::SelectionCounts, Error> {
        unsafe {
            let query = sys::GetGroupUnitsCountsQuery {
                groupID: group_id,
            };
            let mut result = MaybeUninit::<sys::GetGroupUnitsCountsResult>::zeroed();
            let func = self.api.GetGroupUnitsCounts.expect("GetGroupUnitsCounts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.counts
            })
        }
    }

    pub fn get_unit_group(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitGroupQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitGroupResult>::zeroed();
            let func = self.api.GetUnitGroup.expect("GetUnitGroup function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.groupID
            })
        }
    }

    pub fn set_unit_group(&self, unit_id: i32, group_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitGroupQuery {
                unitID: unit_id,
                groupID: group_id,
            };
            let mut result = MaybeUninit::<sys::SetUnitGroupResult>::zeroed();
            let func = self.api.SetUnitGroup.expect("SetUnitGroup function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

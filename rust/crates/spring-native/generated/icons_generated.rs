impl<'a> Icons<'a> {
    #[expect(clippy::too_many_arguments, reason = "NativeInterface preserves the corresponding Lua API arity")]
    pub fn add_unit_icon(&self, icon_name: &str, tex_file: &str, size: f32, distance: f32, radius_adjust: bool, u0: f32, v0: f32, u1: f32, v1: f32) -> Result<bool, Error> {
        unsafe {
            let icon_name_cstr = std::ffi::CString::new(icon_name).map_err(|_| Error::invalid_argument("icon_name"))?;
            let tex_file_cstr = std::ffi::CString::new(tex_file).map_err(|_| Error::invalid_argument("tex_file"))?;
            let query = sys::AddUnitIconQuery {
                iconName: icon_name_cstr.as_ptr(),
                texFile: tex_file_cstr.as_ptr(),
                size,
                distance,
                radiusAdjust: radius_adjust,
                u0,
                v0,
                u1,
                v1,
            };
            let mut result = MaybeUninit::<sys::AddUnitIconResult>::zeroed();
            let func = self.api.AddUnitIcon.expect("AddUnitIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn free_unit_icon(&self, icon_name: &str) -> Result<bool, Error> {
        unsafe {
            let icon_name_cstr = std::ffi::CString::new(icon_name).map_err(|_| Error::invalid_argument("icon_name"))?;
            let query = sys::FreeUnitIconQuery {
                iconName: icon_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::FreeUnitIconResult>::zeroed();
            let func = self.api.FreeUnitIcon.expect("FreeUnitIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_icon_data(&self, icon_name: &str, full_data: bool) -> Result<sys::IconDataEntry, Error> {
        unsafe {
            let icon_name_cstr = std::ffi::CString::new(icon_name).map_err(|_| Error::invalid_argument("icon_name"))?;
            let query = sys::GetIconDataQuery {
                iconName: icon_name_cstr.as_ptr(),
                fullData: full_data,
            };
            let mut result = MaybeUninit::<sys::GetIconDataResult>::zeroed();
            let func = self.api.GetIconData.expect("GetIconData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.data
            })
        }
    }

    pub fn get_all_icon_data_array(&self, full_data: bool) -> Result<Vec<sys::IconDataEntry>, Error> {
        unsafe {
            let query = sys::GetAllIconDataArrayQuery {
                fullData: full_data,
            };
            let mut result = MaybeUninit::<sys::GetAllIconDataArrayResult>::zeroed();
            let func = self.api.GetAllIconDataArray.expect("GetAllIconDataArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::IconDataEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unit_icon_get_draw(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitIconGetDrawQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::UnitIconGetDrawResult>::zeroed();
            let func = self.api.UnitIconGetDraw.expect("UnitIconGetDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.drawIcon
            })
        }
    }

    pub fn unit_icon_set_draw(&self, unit_id: i32, draw_icon: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitIconSetDrawQuery {
                unitID: unit_id,
                drawIcon: draw_icon,
            };
            let mut result = MaybeUninit::<sys::UnitIconSetDrawResult>::zeroed();
            let func = self.api.UnitIconSetDraw.expect("UnitIconSetDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

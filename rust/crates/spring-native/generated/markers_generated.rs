#[derive(Debug, Clone, Copy, Default)]
pub struct MarkerErasePositionOptions {
    pub local_only: bool,
    pub always_erase: bool,
}

impl From<MarkerErasePositionOptions> for sys::MarkerErasePositionOptions {
    fn from(options: MarkerErasePositionOptions) -> Self {
        sys::MarkerErasePositionOptions {
            localOnly: options.local_only,
            alwaysErase: options.always_erase,
        }
    }
}

impl<'a> Markers<'a> {
    pub fn add_world_icon(&self, cmd_id: i32, pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddWorldIconQuery {
                cmdID: cmd_id,
                pos,
            };
            let mut result = MaybeUninit::<sys::AddWorldIconResult>::zeroed();
            let func = self.api.AddWorldIcon.expect("AddWorldIcon function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_world_text(&self, text: &str, pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::AddWorldTextQuery {
                text: text_cstr.as_ptr(),
                pos,
            };
            let mut result = MaybeUninit::<sys::AddWorldTextResult>::zeroed();
            let func = self.api.AddWorldText.expect("AddWorldText function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_world_unit(&self, unit_def_id: i32, pos: sys::Float3, team_id: i32, facing: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddWorldUnitQuery {
                unitDefID: unit_def_id,
                pos,
                teamID: team_id,
                facing,
            };
            let mut result = MaybeUninit::<sys::AddWorldUnitResult>::zeroed();
            let func = self.api.AddWorldUnit.expect("AddWorldUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn marker_add_point(&self, pos: sys::Float3, text: &str, local_only: bool, player_id: i32) -> Result<bool, Error> {
        unsafe {
            let text_cstr = std::ffi::CString::new(text).map_err(|_| Error::invalid_argument("text"))?;
            let query = sys::MarkerAddPointQuery {
                pos,
                text: text_cstr.as_ptr(),
                localOnly: local_only,
                playerID: player_id,
            };
            let mut result = MaybeUninit::<sys::MarkerAddPointResult>::zeroed();
            let func = self.api.MarkerAddPoint.expect("MarkerAddPoint function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn marker_add_line(&self, from: sys::Float3, to: sys::Float3, local_only: bool, player_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::MarkerAddLineQuery {
                from,
                to,
                localOnly: local_only,
                playerID: player_id,
            };
            let mut result = MaybeUninit::<sys::MarkerAddLineResult>::zeroed();
            let func = self.api.MarkerAddLine.expect("MarkerAddLine function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn marker_erase_position(&self, pos: sys::Float3, unused: f32, options: MarkerErasePositionOptions, player_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::MarkerErasePositionQuery {
                pos,
                unused,
                options: options.into(),
                playerID: player_id,
            };
            let mut result = MaybeUninit::<sys::MarkerErasePositionResult>::zeroed();
            let func = self.api.MarkerErasePosition.expect("MarkerErasePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

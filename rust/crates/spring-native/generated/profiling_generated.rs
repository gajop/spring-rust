#[derive(Debug, Clone, Copy, Default)]
pub struct DiffTimersOptions {
    pub return_ms: bool,
    pub from_micro_secs: bool,
}

impl From<DiffTimersOptions> for sys::DiffTimersOptions {
    fn from(options: DiffTimersOptions) -> Self {
        sys::DiffTimersOptions {
            returnMs: options.return_ms,
            fromMicroSecs: options.from_micro_secs,
        }
    }
}

impl<'a> Profiling<'a> {
    pub fn get_timer(&self) -> Result<u64, Error> {
        unsafe {
            let query = sys::GetTimerQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetTimerResult>::zeroed();
            let func = self.api.GetTimer.expect("GetTimer function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.timer
            })
        }
    }

    pub fn get_timer_micros(&self) -> Result<u64, Error> {
        unsafe {
            let query = sys::GetTimerMicrosQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetTimerMicrosResult>::zeroed();
            let func = self.api.GetTimerMicros.expect("GetTimerMicros function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.timer
            })
        }
    }

    pub fn diff_timers(&self, end_timer: u64, start_timer: u64, options: DiffTimersOptions) -> Result<f32, Error> {
        unsafe {
            let query = sys::DiffTimersQuery {
                endTimer: end_timer,
                startTimer: start_timer,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::DiffTimersResult>::zeroed();
            let func = self.api.DiffTimers.expect("DiffTimers function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.seconds
            })
        }
    }

    pub fn get_frame_timer(&self, last_frame_time: bool) -> Result<u64, Error> {
        unsafe {
            let query = sys::GetFrameTimerQuery {
                lastFrameTime: last_frame_time,
            };
            let mut result = MaybeUninit::<sys::GetFrameTimerResult>::zeroed();
            let func = self.api.GetFrameTimer.expect("GetFrameTimer function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.timer
            })
        }
    }

    pub fn get_draw_seconds(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetDrawSecondsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetDrawSecondsResult>::zeroed();
            let func = self.api.GetDrawSeconds.expect("GetDrawSeconds function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.seconds
            })
        }
    }

    pub fn get_profiler_time_record(&self, name: &str, include_frame_data: bool) -> Result<(f32, f32, f32, f32, f32, Vec<f32>), Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::GetProfilerTimeRecordQuery {
                name: name_cstr.as_ptr(),
                includeFrameData: include_frame_data,
            };
            let mut result = MaybeUninit::<sys::GetProfilerTimeRecordResult>::zeroed();
            let func = self.api.GetProfilerTimeRecord.expect("GetProfilerTimeRecord function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.totalMs,
                result.currentMs,
                result.maxDt,
                result.timePct,
                result.peakPct,
                {
                    let slice = if result.frameCount == 0 || result.frameData.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.frameData, result.frameCount as usize)
                    };
                    slice.to_vec()
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_profiler_record_names(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetProfilerRecordNamesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetProfilerRecordNamesResult>::zeroed();
            let func = self.api.GetProfilerRecordNames.expect("GetProfilerRecordNames function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.names.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.names, result.count as usize);
                        slice.iter().map(|&ptr| {
                            if ptr.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(ptr).to_string_lossy().into_owned()
                            }
                        }).collect()
                    }
                }
            })
        }
    }

    pub fn get_lua_mem_usage(&self) -> Result<(f32, f32, f32, f32, f32, f32, f32, f32), Error> {
        unsafe {
            let query = sys::GetLuaMemUsageQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLuaMemUsageResult>::zeroed();
            let func = self.api.GetLuaMemUsage.expect("GetLuaMemUsage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.handleAllocedKB,
                result.handleAllocsK,
                result.globalAllocedKB,
                result.globalAllocsK,
                result.unsyncedAllocedKB,
                result.unsyncedAllocsK,
                result.syncedAllocedKB,
                result.syncedAllocsK,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_vid_mem_usage(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let query = sys::GetVidMemUsageQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetVidMemUsageResult>::zeroed();
            let func = self.api.GetVidMemUsage.expect("GetVidMemUsage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.usedMB,
                result.availableMB,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_synced_gcinfo(&self, collect: bool) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetSyncedGCInfoQuery {
                collect: collect,
            };
            let mut result = MaybeUninit::<sys::GetSyncedGCInfoResult>::zeroed();
            let func = self.api.GetSyncedGCInfo.expect("GetSyncedGCInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.gcKB
            })
        }
    }

}

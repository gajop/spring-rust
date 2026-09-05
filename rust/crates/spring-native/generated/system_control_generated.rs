/// The complete result tuple returned by [`get_game_state`].
pub type GetGameStateValue = (bool, bool, bool, bool);

/// The complete result tuple returned by [`get_window_display_mode`].
pub type GetWindowDisplayModeValue = (i32, i32, i32, i32, Option<String>, bool);

impl<'a> SystemControl<'a> {
    pub fn call_as_team<F: FnMut()>(&self, team_id: i32, mut callback: F) -> Result<bool, Error> {
        unsafe {
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::CallAsTeamQuery {
                teamID: team_id,
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::CallAsTeamResult>::zeroed();
            let func = self.api.CallAsTeam.expect("CallAsTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn garbage_collect_ctrl(&self, iters_per_batch: i32, num_steps_per_iter: i32, min_steps_per_iter: i32, max_steps_per_iter: i32, min_loop_run_time: f32, max_loop_run_time: f32, base_run_time_mult: f32, base_mem_load_mult: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GarbageCollectCtrlQuery {
                itersPerBatch: iters_per_batch,
                numStepsPerIter: num_steps_per_iter,
                minStepsPerIter: min_steps_per_iter,
                maxStepsPerIter: max_steps_per_iter,
                minLoopRunTime: min_loop_run_time,
                maxLoopRunTime: max_loop_run_time,
                baseRunTimeMult: base_run_time_mult,
                baseMemLoadMult: base_mem_load_mult,
            };
            let mut result = MaybeUninit::<sys::GarbageCollectCtrlResult>::zeroed();
            let func = self.api.GarbageCollectCtrl.expect("GarbageCollectCtrl function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn clear_watch_dog_timer(&self, thread_name: &str, keep_stopped: bool) -> Result<bool, Error> {
        unsafe {
            let thread_name_cstr = std::ffi::CString::new(thread_name).map_err(|_| Error::invalid_argument("thread_name"))?;
            let query = sys::ClearWatchDogTimerQuery {
                threadName: thread_name_cstr.as_ptr(),
                keepStopped: keep_stopped,
            };
            let mut result = MaybeUninit::<sys::ClearWatchDogTimerResult>::zeroed();
            let func = self.api.ClearWatchDogTimer.expect("ClearWatchDogTimer function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn quit(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::QuitQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::QuitResult>::zeroed();
            let func = self.api.Quit.expect("Quit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn reload(&self, start_script: &str) -> Result<bool, Error> {
        unsafe {
            let start_script_cstr = std::ffi::CString::new(start_script).map_err(|_| Error::invalid_argument("start_script"))?;
            let query = sys::ReloadQuery {
                startScript: start_script_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::ReloadResult>::zeroed();
            let func = self.api.Reload.expect("Reload function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn restart(&self, cmd_args: &str, start_script: &str) -> Result<bool, Error> {
        unsafe {
            let cmd_args_cstr = std::ffi::CString::new(cmd_args).map_err(|_| Error::invalid_argument("cmd_args"))?;
            let start_script_cstr = std::ffi::CString::new(start_script).map_err(|_| Error::invalid_argument("start_script"))?;
            let query = sys::RestartQuery {
                cmdArgs: cmd_args_cstr.as_ptr(),
                startScript: start_script_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::RestartResult>::zeroed();
            let func = self.api.Restart.expect("Restart function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn start(&self, cmd_args: &str, start_script: &str) -> Result<bool, Error> {
        unsafe {
            let cmd_args_cstr = std::ffi::CString::new(cmd_args).map_err(|_| Error::invalid_argument("cmd_args"))?;
            let start_script_cstr = std::ffi::CString::new(start_script).map_err(|_| Error::invalid_argument("start_script"))?;
            let query = sys::StartQuery {
                cmdArgs: cmd_args_cstr.as_ptr(),
                startScript: start_script_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::StartResult>::zeroed();
            let func = self.api.Start.expect("Start function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn r#yield(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::YieldQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::YieldResult>::zeroed();
            let func = self.api.Yield.expect("Yield function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.keepYielding
            })
        }
    }

    pub fn request_start_position(&self, pos: sys::Float3, ready: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::RequestStartPositionQuery {
                pos,
                ready,
            };
            let mut result = MaybeUninit::<sys::RequestStartPositionResult>::zeroed();
            let func = self.api.RequestStartPosition.expect("RequestStartPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn ping(&self, tag: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::PingQuery {
                tag,
            };
            let mut result = MaybeUninit::<sys::PingResult>::zeroed();
            let func = self.api.Ping.expect("Ping function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_game_state(&self, max_latency: f32) -> Result<GetGameStateValue, Error> {
        unsafe {
            let query = sys::GetGameStateQuery {
                maxLatency: max_latency,
            };
            let mut result = MaybeUninit::<sys::GetGameStateResult>::zeroed();
            let func = self.api.GetGameState.expect("GetGameState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.doneLoading,
                result.isSavedGame,
                result.isClientPaused,
                result.isSimLagging,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_game_name(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetGameNameQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGameNameResult>::zeroed();
            let func = self.api.GetGameName.expect("GetGameName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.name.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.name).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_menu_name(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetMenuNameQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMenuNameResult>::zeroed();
            let func = self.api.GetMenuName.expect("GetMenuName function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.name.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.name).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_replay_length(&self) -> Result<(f32, bool), Error> {
        unsafe {
            let query = sys::GetReplayLengthQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetReplayLengthResult>::zeroed();
            let func = self.api.GetReplayLength.expect("GetReplayLength function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.seconds,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_replay_file_path(&self) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let query = sys::GetReplayFilePathQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetReplayFilePathResult>::zeroed();
            let func = self.api.GetReplayFilePath.expect("GetReplayFilePath function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.path.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.path).to_string_lossy().into_owned())
                    }
                },
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_replay_recording_file_path(&self) -> Result<(Option<String>, bool), Error> {
        unsafe {
            let query = sys::GetReplayRecordingFilePathQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetReplayRecordingFilePathResult>::zeroed();
            let func = self.api.GetReplayRecordingFilePath.expect("GetReplayRecordingFilePath function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.path.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.path).to_string_lossy().into_owned())
                    }
                },
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn is_replay(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsReplayQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsReplayResult>::zeroed();
            let func = self.api.IsReplay.expect("IsReplay function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isReplay
            })
        }
    }

    pub fn get_video_capturing_mode(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetVideoCapturingModeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetVideoCapturingModeResult>::zeroed();
            let func = self.api.GetVideoCapturingMode.expect("GetVideoCapturingMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allowRecord
            })
        }
    }

    pub fn get_window_display_mode(&self) -> Result<GetWindowDisplayModeValue, Error> {
        unsafe {
            let query = sys::GetWindowDisplayModeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetWindowDisplayModeResult>::zeroed();
            let func = self.api.GetWindowDisplayMode.expect("GetWindowDisplayMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.width,
                result.height,
                result.bpp,
                result.refresh,
                {
                    if result.formatName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.formatName).to_string_lossy().into_owned())
                    }
                },
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_gather_mode(&self) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetGatherModeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGatherModeResult>::zeroed();
            let func = self.api.GetGatherMode.expect("GetGatherMode function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.mode
            })
        }
    }

    pub fn set_share_level(&self, resource: &str, level: f32) -> Result<bool, Error> {
        unsafe {
            let resource_cstr = std::ffi::CString::new(resource).map_err(|_| Error::invalid_argument("resource"))?;
            let query = sys::SetShareLevelQuery {
                resource: resource_cstr.as_ptr(),
                level,
            };
            let mut result = MaybeUninit::<sys::SetShareLevelResult>::zeroed();
            let func = self.api.SetShareLevel.expect("SetShareLevel function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn share_resources(&self, team_id: i32, resource: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_cstr = std::ffi::CString::new(resource).map_err(|_| Error::invalid_argument("resource"))?;
            let query = sys::ShareResourcesQuery {
                teamID: team_id,
                resource: resource_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::ShareResourcesResult>::zeroed();
            let func = self.api.ShareResources.expect("ShareResources function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

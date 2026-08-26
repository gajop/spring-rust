impl<'a> Sound<'a> {
    pub fn play_sound_file(&self, sound_file: &str, volume: f32, pos: sys::Float3, velocity: sys::Float3, channel: i32) -> Result<bool, Error> {
        unsafe {
            let sound_file_cstr = std::ffi::CString::new(sound_file).map_err(|_| Error::invalid_argument("sound_file"))?;
            let query = sys::PlaySoundFileQuery {
                soundFile: sound_file_cstr.as_ptr(),
                volume,
                pos,
                velocity,
                channel,
            };
            let mut result = MaybeUninit::<sys::PlaySoundFileResult>::zeroed();
            let func = self.api.PlaySoundFile.expect("PlaySoundFile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn load_sound_def(&self, sound_name: &str) -> Result<bool, Error> {
        unsafe {
            let sound_name_cstr = std::ffi::CString::new(sound_name).map_err(|_| Error::invalid_argument("sound_name"))?;
            let query = sys::LoadSoundDefQuery {
                soundName: sound_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::LoadSoundDefResult>::zeroed();
            let func = self.api.LoadSoundDef.expect("LoadSoundDef function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn play_sound_stream(&self, ogg_file: &str, volume: f32, enqueue: bool) -> Result<bool, Error> {
        unsafe {
            let ogg_file_cstr = std::ffi::CString::new(ogg_file).map_err(|_| Error::invalid_argument("ogg_file"))?;
            let query = sys::PlaySoundStreamQuery {
                oggFile: ogg_file_cstr.as_ptr(),
                volume,
                enqueue,
            };
            let mut result = MaybeUninit::<sys::PlaySoundStreamResult>::zeroed();
            let func = self.api.PlaySoundStream.expect("PlaySoundStream function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn stop_sound_stream(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::StopSoundStreamQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::StopSoundStreamResult>::zeroed();
            let func = self.api.StopSoundStream.expect("StopSoundStream function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn pause_sound_stream(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::PauseSoundStreamQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::PauseSoundStreamResult>::zeroed();
            let func = self.api.PauseSoundStream.expect("PauseSoundStream function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_sound_stream_volume(&self, volume: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetSoundStreamVolumeQuery {
                volume,
            };
            let mut result = MaybeUninit::<sys::SetSoundStreamVolumeResult>::zeroed();
            let func = self.api.SetSoundStreamVolume.expect("SetSoundStreamVolume function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_sound_stream_time(&self) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetSoundStreamTimeQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSoundStreamTimeResult>::zeroed();
            let func = self.api.GetSoundStreamTime.expect("GetSoundStreamTime function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.time
            })
        }
    }

    pub fn get_sound_devices(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetSoundDevicesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSoundDevicesResult>::zeroed();
            let func = self.api.GetSoundDevices.expect("GetSoundDevices function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.devices.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.devices, result.count as usize);
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

    pub fn get_sound_effect_params(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetSoundEffectParamsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetSoundEffectParamsResult>::zeroed();
            let func = self.api.GetSoundEffectParams.expect("GetSoundEffectParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_sound_effect_params(&self, params: sys::SoundEffectParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetSoundEffectParamsQuery {
                params,
            };
            let mut result = MaybeUninit::<sys::SetSoundEffectParamsResult>::zeroed();
            let func = self.api.SetSoundEffectParams.expect("SetSoundEffectParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn preload_sound_item(&self, sound_name: &str) -> Result<bool, Error> {
        unsafe {
            let sound_name_cstr = std::ffi::CString::new(sound_name).map_err(|_| Error::invalid_argument("sound_name"))?;
            let query = sys::PreloadSoundItemQuery {
                soundName: sound_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::PreloadSoundItemResult>::zeroed();
            let func = self.api.PreloadSoundItem.expect("PreloadSoundItem function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

impl<'a> Vfs<'a> {
    pub fn file_exists(&self, path: &str) -> Result<bool, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::FileExistsQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::FileExistsResult>::zeroed();
            let func = self.api.FileExists.expect("FileExists function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.exists
            })
        }
    }

    pub fn get_file_info(&self, path: &str) -> Result<(sys::FileInfo, bool), Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::GetFileInfoQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetFileInfoResult>::zeroed();
            let func = self.api.GetFileInfo.expect("GetFileInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.info,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_file_size(&self, path: &str) -> Result<u32, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::GetFileSizeQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetFileSizeResult>::zeroed();
            let func = self.api.GetFileSize.expect("GetFileSize function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.size
            })
        }
    }

    pub fn load_file(&self, path: &str, mode: &str) -> Result<Vec<u8>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::LoadFileQuery {
                path: path_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::LoadFileResult>::zeroed();
            let func = self.api.LoadFile.expect("LoadFile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn list_dir(&self, path: &str, pattern: &str, mode: &str, recursive: bool) -> Result<Vec<sys::DirEntry>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let pattern_cstr = std::ffi::CString::new(pattern).map_err(|_| Error::invalid_argument("pattern"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::ListDirQuery {
                path: path_cstr.as_ptr(),
                pattern: pattern_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
                recursive,
            };
            let mut result = MaybeUninit::<sys::ListDirResult>::zeroed();
            let func = self.api.ListDir.expect("ListDir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::DirEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn dir_list(&self, path: &str, pattern: &str, mode: &str, recursive: bool) -> Result<Vec<sys::DirEntry>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let pattern_cstr = std::ffi::CString::new(pattern).map_err(|_| Error::invalid_argument("pattern"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::ListDirQuery {
                path: path_cstr.as_ptr(),
                pattern: pattern_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
                recursive,
            };
            let mut result = MaybeUninit::<sys::ListDirResult>::zeroed();
            let func = self.api.DirList.expect("DirList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::DirEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn sub_dirs(&self, path: &str, pattern: &str, mode: &str, recursive: bool) -> Result<Vec<String>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let pattern_cstr = std::ffi::CString::new(pattern).map_err(|_| Error::invalid_argument("pattern"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::SubDirsQuery {
                path: path_cstr.as_ptr(),
                pattern: pattern_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
                recursive,
            };
            let mut result = MaybeUninit::<sys::SubDirsResult>::zeroed();
            let func = self.api.SubDirs.expect("SubDirs function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.dirs.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.dirs, result.count as usize);
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

    pub fn get_file_absolute_path(&self, path: &str, mode: &str) -> Result<Option<String>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::GetFileAbsolutePathQuery {
                path: path_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetFileAbsolutePathResult>::zeroed();
            let func = self.api.GetFileAbsolutePath.expect("GetFileAbsolutePath function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.path.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.path).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_archive_containing_file(&self, path: &str, mode: &str) -> Result<Option<String>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::GetArchiveContainingFileQuery {
                path: path_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetArchiveContainingFileResult>::zeroed();
            let func = self.api.GetArchiveContainingFile.expect("GetArchiveContainingFile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.archiveName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.archiveName).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn is_directory(&self, path: &str) -> Result<bool, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::IsDirectoryQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::IsDirectoryResult>::zeroed();
            let func = self.api.IsDirectory.expect("IsDirectory function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isDirectory
            })
        }
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::ReadFileQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::ReadFileResult>::zeroed();
            let func = self.api.ReadFile.expect("ReadFile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn read_file_as_string(&self, path: &str) -> Result<Option<String>, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::ReadFileAsStringQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::ReadFileAsStringResult>::zeroed();
            let func = self.api.ReadFileAsString.expect("ReadFileAsString function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.content.is_null() {
                        None
                    } else {
                        let slice = if result.contentLength == 0 {
                            &[]
                        } else {
                            slice::from_raw_parts(result.content as *const u8, result.contentLength as usize)
                        };
                        Some(String::from_utf8_lossy(slice).into_owned())
                    }
                }
            })
        }
    }

    pub fn get_archives(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetArchivesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetArchivesResult>::zeroed();
            let func = self.api.GetArchives.expect("GetArchives function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.archives.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.archives, result.count as usize);
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

    pub fn get_maps(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetMapsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMapsResult>::zeroed();
            let func = self.api.GetMaps.expect("GetMaps function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.maps.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.maps, result.count as usize);
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

    pub fn get_games(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetGamesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetGamesResult>::zeroed();
            let func = self.api.GetGames.expect("GetGames function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.games.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.games, result.count as usize);
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

    pub fn get_all_archives(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetAllArchivesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetAllArchivesResult>::zeroed();
            let func = self.api.GetAllArchives.expect("GetAllArchives function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.archives.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.archives, result.count as usize);
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

    pub fn has_archive(&self, archive_name: &str) -> Result<bool, Error> {
        unsafe {
            let archive_name_cstr = std::ffi::CString::new(archive_name).map_err(|_| Error::invalid_argument("archive_name"))?;
            let query = sys::HasArchiveQuery {
                archiveName: archive_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::HasArchiveResult>::zeroed();
            let func = self.api.HasArchive.expect("HasArchive function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.hasArchive
            })
        }
    }

    pub fn get_loaded_archives(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetLoadedArchivesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetLoadedArchivesResult>::zeroed();
            let func = self.api.GetLoadedArchives.expect("GetLoadedArchives function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.archives.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.archives, result.count as usize);
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

    pub fn get_archive_path(&self, archive_name: &str) -> Result<Option<String>, Error> {
        unsafe {
            let archive_name_cstr = std::ffi::CString::new(archive_name).map_err(|_| Error::invalid_argument("archive_name"))?;
            let query = sys::GetArchivePathQuery {
                archiveName: archive_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetArchivePathResult>::zeroed();
            let func = self.api.GetArchivePath.expect("GetArchivePath function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.path.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.path).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_archive_info(&self, archive_name: &str) -> Result<Vec<sys::ArchiveInfoEntry>, Error> {
        unsafe {
            let archive_name_cstr = std::ffi::CString::new(archive_name).map_err(|_| Error::invalid_argument("archive_name"))?;
            let query = sys::GetArchiveInfoQuery {
                archiveName: archive_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetArchiveInfoResult>::zeroed();
            let func = self.api.GetArchiveInfo.expect("GetArchiveInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::ArchiveInfoEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_archive_dependencies(&self, archive_name: &str) -> Result<Vec<String>, Error> {
        unsafe {
            let archive_name_cstr = std::ffi::CString::new(archive_name).map_err(|_| Error::invalid_argument("archive_name"))?;
            let query = sys::GetArchiveDependenciesQuery {
                archiveName: archive_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetArchiveDependenciesResult>::zeroed();
            let func = self.api.GetArchiveDependencies.expect("GetArchiveDependencies function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.archives.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.archives, result.count as usize);
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

    pub fn get_archive_replaces(&self, archive_name: &str) -> Result<Vec<String>, Error> {
        unsafe {
            let archive_name_cstr = std::ffi::CString::new(archive_name).map_err(|_| Error::invalid_argument("archive_name"))?;
            let query = sys::GetArchiveReplacesQuery {
                archiveName: archive_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetArchiveReplacesResult>::zeroed();
            let func = self.api.GetArchiveReplaces.expect("GetArchiveReplaces function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.count == 0 || result.archives.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.archives, result.count as usize);
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

    pub fn get_archive_checksum(&self, archive_name: &str) -> Result<(Option<String>, Option<String>), Error> {
        unsafe {
            let archive_name_cstr = std::ffi::CString::new(archive_name).map_err(|_| Error::invalid_argument("archive_name"))?;
            let query = sys::GetArchiveChecksumQuery {
                archiveName: archive_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetArchiveChecksumResult>::zeroed();
            let func = self.api.GetArchiveChecksum.expect("GetArchiveChecksum function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.singleChecksum.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.singleChecksum).to_string_lossy().into_owned())
                    }
                },
                {
                    if result.completeChecksum.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.completeChecksum).to_string_lossy().into_owned())
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_name_from_rapid_tag(&self, rapid_tag: &str) -> Result<Option<String>, Error> {
        unsafe {
            let rapid_tag_cstr = std::ffi::CString::new(rapid_tag).map_err(|_| Error::invalid_argument("rapid_tag"))?;
            let query = sys::GetNameFromRapidTagQuery {
                rapidTag: rapid_tag_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetNameFromRapidTagResult>::zeroed();
            let func = self.api.GetNameFromRapidTag.expect("GetNameFromRapidTag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.archiveName.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.archiveName).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_available_ais(&self, game_archive_name: &str, map_archive_name: &str) -> Result<Vec<sys::AIInfoEntry>, Error> {
        unsafe {
            let game_archive_name_cstr = std::ffi::CString::new(game_archive_name).map_err(|_| Error::invalid_argument("game_archive_name"))?;
            let map_archive_name_cstr = std::ffi::CString::new(map_archive_name).map_err(|_| Error::invalid_argument("map_archive_name"))?;
            let query = sys::GetAvailableAIsQuery {
                gameArchiveName: game_archive_name_cstr.as_ptr(),
                mapArchiveName: map_archive_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetAvailableAIsResult>::zeroed();
            let func = self.api.GetAvailableAIs.expect("GetAvailableAIs function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.ais.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.ais as *const sys::AIInfoEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn use_archive<F: FnMut()>(&self, archive_name: &str, mut callback: F) -> Result<bool, Error> {
        unsafe {
            let archive_name_cstr = std::ffi::CString::new(archive_name).map_err(|_| Error::invalid_argument("archive_name"))?;
            unsafe extern "C" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {
                let f = unsafe { &mut *(user_data as *mut F) };
                f();
            }
            let query = sys::UseArchiveQuery {
                archiveName: archive_name_cstr.as_ptr(),
                callback: Some(trampoline::<F>),
                userData: &mut callback as *mut F as *mut std::ffi::c_void,
            };
            let mut result = MaybeUninit::<sys::UseArchiveResult>::zeroed();
            let func = self.api.UseArchive.expect("UseArchive function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn create_dir(&self, path: &str) -> Result<bool, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::CreateDirQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::CreateDirResult>::zeroed();
            let func = self.api.CreateDir.expect("CreateDir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn extract_mod_archive_file(&self, path: &str) -> Result<bool, Error> {
        unsafe {
            let path_cstr = std::ffi::CString::new(path).map_err(|_| Error::invalid_argument("path"))?;
            let query = sys::ExtractModArchiveFileQuery {
                path: path_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::ExtractModArchiveFileResult>::zeroed();
            let func = self.api.ExtractModArchiveFile.expect("ExtractModArchiveFile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn compress_folder(&self, folder_path: &str, archive_type: &str, compressed_file_path: &str, include_folder: bool, mode: &str) -> Result<bool, Error> {
        unsafe {
            let folder_path_cstr = std::ffi::CString::new(folder_path).map_err(|_| Error::invalid_argument("folder_path"))?;
            let archive_type_cstr = std::ffi::CString::new(archive_type).map_err(|_| Error::invalid_argument("archive_type"))?;
            let compressed_file_path_cstr = std::ffi::CString::new(compressed_file_path).map_err(|_| Error::invalid_argument("compressed_file_path"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::CompressFolderQuery {
                folderPath: folder_path_cstr.as_ptr(),
                archiveType: archive_type_cstr.as_ptr(),
                compressedFilePath: compressed_file_path_cstr.as_ptr(),
                includeFolder: include_folder,
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::CompressFolderResult>::zeroed();
            let func = self.api.CompressFolder.expect("CompressFolder function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn zlib_compress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::ZlibCompressQuery {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::ZlibCompressResult>::zeroed();
            let func = self.api.ZlibCompress.expect("ZlibCompress function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn zlib_decompress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::ZlibDecompressQuery {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
            };
            let mut result = MaybeUninit::<sys::ZlibDecompressResult>::zeroed();
            let func = self.api.ZlibDecompress.expect("ZlibDecompress function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn calculate_hash(&self, data: &[u8], hash_type: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::CalculateHashQuery {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                hashType: hash_type,
            };
            let mut result = MaybeUninit::<sys::CalculateHashResult>::zeroed();
            let func = self.api.CalculateHash.expect("CalculateHash function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.hash.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.hash).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn pack_u8(&self, values: &[u8]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::PackU8Query {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::PackU8Result>::zeroed();
            let func = self.api.PackU8.expect("PackU8 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn pack_u16(&self, values: &[u16]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::PackU16Query {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::PackU16Result>::zeroed();
            let func = self.api.PackU16.expect("PackU16 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn pack_u32(&self, values: &[u32]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::PackU32Query {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::PackU32Result>::zeroed();
            let func = self.api.PackU32.expect("PackU32 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn pack_s8(&self, values: &[i8]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::PackS8Query {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::PackS8Result>::zeroed();
            let func = self.api.PackS8.expect("PackS8 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn pack_s16(&self, values: &[i16]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::PackS16Query {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::PackS16Result>::zeroed();
            let func = self.api.PackS16.expect("PackS16 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn pack_s32(&self, values: &[i32]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::PackS32Query {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::PackS32Result>::zeroed();
            let func = self.api.PackS32.expect("PackS32 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn pack_f32(&self, values: &[f32]) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::PackF32Query {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::PackF32Result>::zeroed();
            let func = self.api.PackF32.expect("PackF32 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.size == 0 || result.data.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.data, result.size as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unpack_u8(&self, data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<u8>, Error> {
        unsafe {
            let query = sys::UnpackU8Query {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                byteOffset: byte_offset,
                count,
            };
            let mut result = MaybeUninit::<sys::UnpackU8Result>::zeroed();
            let func = self.api.UnpackU8.expect("UnpackU8 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unpack_u16(&self, data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<u16>, Error> {
        unsafe {
            let query = sys::UnpackU16Query {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                byteOffset: byte_offset,
                count,
            };
            let mut result = MaybeUninit::<sys::UnpackU16Result>::zeroed();
            let func = self.api.UnpackU16.expect("UnpackU16 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unpack_u32(&self, data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<u32>, Error> {
        unsafe {
            let query = sys::UnpackU32Query {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                byteOffset: byte_offset,
                count,
            };
            let mut result = MaybeUninit::<sys::UnpackU32Result>::zeroed();
            let func = self.api.UnpackU32.expect("UnpackU32 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unpack_s8(&self, data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<i8>, Error> {
        unsafe {
            let query = sys::UnpackS8Query {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                byteOffset: byte_offset,
                count,
            };
            let mut result = MaybeUninit::<sys::UnpackS8Result>::zeroed();
            let func = self.api.UnpackS8.expect("UnpackS8 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unpack_s16(&self, data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<i16>, Error> {
        unsafe {
            let query = sys::UnpackS16Query {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                byteOffset: byte_offset,
                count,
            };
            let mut result = MaybeUninit::<sys::UnpackS16Result>::zeroed();
            let func = self.api.UnpackS16.expect("UnpackS16 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unpack_s32(&self, data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::UnpackS32Query {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                byteOffset: byte_offset,
                count,
            };
            let mut result = MaybeUninit::<sys::UnpackS32Result>::zeroed();
            let func = self.api.UnpackS32.expect("UnpackS32 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn unpack_f32(&self, data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<f32>, Error> {
        unsafe {
            let query = sys::UnpackF32Query {
                data: data.as_ptr(),
                dataSize: data.len() as u32,
                byteOffset: byte_offset,
                count,
            };
            let mut result = MaybeUninit::<sys::UnpackF32Result>::zeroed();
            let func = self.api.UnpackF32.expect("UnpackF32 function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.values.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.values, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_map_square_texture(&self, tex_square_x: i32, tex_square_y: i32, lod_min: i32, texture_name: &str, lod_max: i32) -> Result<bool, Error> {
        unsafe {
            let texture_name_cstr = std::ffi::CString::new(texture_name).map_err(|_| Error::invalid_argument("texture_name"))?;
            let query = sys::GetMapSquareTextureQuery {
                texSquareX: tex_square_x,
                texSquareY: tex_square_y,
                lodMin: lod_min,
                textureName: texture_name_cstr.as_ptr(),
                lodMax: lod_max,
            };
            let mut result = MaybeUninit::<sys::GetMapSquareTextureResult>::zeroed();
            let func = self.api.GetMapSquareTexture.expect("GetMapSquareTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_map_square_texture(&self, tex_square_x: i32, tex_square_y: i32, texture_name: &str) -> Result<bool, Error> {
        unsafe {
            let texture_name_cstr = std::ffi::CString::new(texture_name).map_err(|_| Error::invalid_argument("texture_name"))?;
            let query = sys::SetMapSquareTextureQuery {
                texSquareX: tex_square_x,
                texSquareY: tex_square_y,
                textureName: texture_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetMapSquareTextureResult>::zeroed();
            let func = self.api.SetMapSquareTexture.expect("SetMapSquareTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_map_square_texture_info(&self) -> Result<(i32, i32, i32), Error> {
        unsafe {
            let query = sys::GetMapSquareTextureInfoQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetMapSquareTextureInfoResult>::zeroed();
            let func = self.api.GetMapSquareTextureInfo.expect("GetMapSquareTextureInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.squareSize,
                result.numSquaresX,
                result.numSquaresZ,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn scan_all_dirs(&self) -> Result<(), Error> {
        unsafe {
            let query = sys::ScanAllDirsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::ScanAllDirsResult>::zeroed();
            let func = self.api.ScanAllDirs.expect("ScanAllDirs function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn download_archive(&self, filename: &str, category: &str) -> Result<(), Error> {
        unsafe {
            let filename_cstr = std::ffi::CString::new(filename).map_err(|_| Error::invalid_argument("filename"))?;
            let category_cstr = std::ffi::CString::new(category).map_err(|_| Error::invalid_argument("category"))?;
            let query = sys::DownloadArchiveQuery {
                filename: filename_cstr.as_ptr(),
                category: category_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::DownloadArchiveResult>::zeroed();
            let func = self.api.DownloadArchive.expect("DownloadArchive function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, ())
        }
    }

    pub fn abort_download(&self, id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AbortDownloadQuery {
                id,
            };
            let mut result = MaybeUninit::<sys::AbortDownloadResult>::zeroed();
            let func = self.api.AbortDownload.expect("AbortDownload function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.removed
            })
        }
    }

}

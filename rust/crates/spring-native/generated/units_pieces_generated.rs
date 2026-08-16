impl<'a> UnitsPieces<'a> {
    pub fn get_model_root_piece(&self, model_name: &str) -> Result<i32, Error> {
        unsafe {
            let model_name_cstr = std::ffi::CString::new(model_name).map_err(|_| Error::invalid_argument("model_name"))?;
            let query = sys::GetModelRootPieceQuery {
                modelName: model_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetModelRootPieceResult>::zeroed();
            let func = self.api.GetModelRootPiece.expect("GetModelRootPiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.rootPiece
            })
        }
    }

    pub fn get_unit_root_piece(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitRootPieceQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitRootPieceResult>::zeroed();
            let func = self.api.GetUnitRootPiece.expect("GetUnitRootPiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.rootPiece
            })
        }
    }

    pub fn get_feature_root_piece(&self, feature_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetFeatureRootPieceQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureRootPieceResult>::zeroed();
            let func = self.api.GetFeatureRootPiece.expect("GetFeatureRootPiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.rootPiece
            })
        }
    }

    pub fn get_model_piece_list(&self, model_name: &str) -> Result<Vec<String>, Error> {
        unsafe {
            let model_name_cstr = std::ffi::CString::new(model_name).map_err(|_| Error::invalid_argument("model_name"))?;
            let query = sys::GetModelPieceListQuery {
                modelName: model_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetModelPieceListResult>::zeroed();
            let func = self.api.GetModelPieceList.expect("GetModelPieceList function pointer must be initialized");
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

    pub fn get_model_piece_map(&self, model_name: &str) -> Result<Vec<sys::PieceMapEntry>, Error> {
        unsafe {
            let model_name_cstr = std::ffi::CString::new(model_name).map_err(|_| Error::invalid_argument("model_name"))?;
            let query = sys::GetModelPieceMapQuery {
                modelName: model_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetModelPieceMapResult>::zeroed();
            let func = self.api.GetModelPieceMap.expect("GetModelPieceMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::PieceMapEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_piece_list(&self, unit_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetUnitPieceListQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitPieceListResult>::zeroed();
            let func = self.api.GetUnitPieceList.expect("GetUnitPieceList function pointer must be initialized");
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

    pub fn get_unit_piece_map(&self, unit_id: i32) -> Result<Vec<sys::PieceMapEntry>, Error> {
        unsafe {
            let query = sys::GetUnitPieceMapQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitPieceMapResult>::zeroed();
            let func = self.api.GetUnitPieceMap.expect("GetUnitPieceMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::PieceMapEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_feature_piece_list(&self, feature_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetFeaturePieceListQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePieceListResult>::zeroed();
            let func = self.api.GetFeaturePieceList.expect("GetFeaturePieceList function pointer must be initialized");
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

    pub fn get_feature_piece_map(&self, feature_id: i32) -> Result<Vec<sys::PieceMapEntry>, Error> {
        unsafe {
            let query = sys::GetFeaturePieceMapQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePieceMapResult>::zeroed();
            let func = self.api.GetFeaturePieceMap.expect("GetFeaturePieceMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::PieceMapEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_piece_info(&self, unit_id: i32, piece_num: i32) -> Result<(sys::PieceInfo, bool), Error> {
        unsafe {
            let query = sys::GetUnitPieceInfoQuery {
                unitID: unit_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitPieceInfoResult>::zeroed();
            let func = self.api.GetUnitPieceInfo.expect("GetUnitPieceInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.info,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_feature_piece_info(&self, feature_id: i32, piece_num: i32) -> Result<(sys::PieceInfo, bool), Error> {
        unsafe {
            let query = sys::GetFeaturePieceInfoQuery {
                featureID: feature_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePieceInfoResult>::zeroed();
            let func = self.api.GetFeaturePieceInfo.expect("GetFeaturePieceInfo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.info,
                result.exists,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_piece_position(&self, unit_id: i32, piece_num: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitPiecePositionQuery {
                unitID: unit_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitPiecePositionResult>::zeroed();
            let func = self.api.GetUnitPiecePosition.expect("GetUnitPiecePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_unit_piece_direction(&self, unit_id: i32, piece_num: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitPieceDirectionQuery {
                unitID: unit_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitPieceDirectionResult>::zeroed();
            let func = self.api.GetUnitPieceDirection.expect("GetUnitPieceDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.direction
            })
        }
    }

    pub fn get_unit_piece_pos_dir(&self, unit_id: i32, piece_num: i32) -> Result<sys::PiecePosDir, Error> {
        unsafe {
            let query = sys::GetUnitPiecePosDirQuery {
                unitID: unit_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitPiecePosDirResult>::zeroed();
            let func = self.api.GetUnitPiecePosDir.expect("GetUnitPiecePosDir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.posDir
            })
        }
    }

    pub fn get_feature_piece_position(&self, feature_id: i32, piece_num: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetFeaturePiecePositionQuery {
                featureID: feature_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePiecePositionResult>::zeroed();
            let func = self.api.GetFeaturePiecePosition.expect("GetFeaturePiecePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_feature_piece_direction(&self, feature_id: i32, piece_num: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetFeaturePieceDirectionQuery {
                featureID: feature_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePieceDirectionResult>::zeroed();
            let func = self.api.GetFeaturePieceDirection.expect("GetFeaturePieceDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.direction
            })
        }
    }

    pub fn get_feature_piece_pos_dir(&self, feature_id: i32, piece_num: i32) -> Result<sys::PiecePosDir, Error> {
        unsafe {
            let query = sys::GetFeaturePiecePosDirQuery {
                featureID: feature_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePiecePosDirResult>::zeroed();
            let func = self.api.GetFeaturePiecePosDir.expect("GetFeaturePiecePosDir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.posDir
            })
        }
    }

    pub fn get_unit_piece_matrix(&self, unit_id: i32, piece_num: i32) -> Result<sys::PieceMatrix, Error> {
        unsafe {
            let query = sys::GetUnitPieceMatrixQuery {
                unitID: unit_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitPieceMatrixResult>::zeroed();
            let func = self.api.GetUnitPieceMatrix.expect("GetUnitPieceMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.matrix
            })
        }
    }

    pub fn get_feature_piece_matrix(&self, feature_id: i32, piece_num: i32) -> Result<sys::PieceMatrix, Error> {
        unsafe {
            let query = sys::GetFeaturePieceMatrixQuery {
                featureID: feature_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePieceMatrixResult>::zeroed();
            let func = self.api.GetFeaturePieceMatrix.expect("GetFeaturePieceMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.matrix
            })
        }
    }

    pub fn get_unit_script_piece(&self, unit_id: i32, script_num: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitScriptPieceQuery {
                unitID: unit_id,
                scriptNum: script_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitScriptPieceResult>::zeroed();
            let func = self.api.GetUnitScriptPiece.expect("GetUnitScriptPiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.pieceNum
            })
        }
    }

    pub fn get_unit_script_names(&self, unit_id: i32) -> Result<Vec<String>, Error> {
        unsafe {
            let query = sys::GetUnitScriptNamesQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitScriptNamesResult>::zeroed();
            let func = self.api.GetUnitScriptNames.expect("GetUnitScriptNames function pointer must be initialized");
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

}

impl<'a> Features<'a> {
    pub fn valid_feature_id(&self, feature_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::ValidFeatureIDQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::ValidFeatureIDResult>::zeroed();
            let func = self.api.ValidFeatureID.expect("ValidFeatureID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.valid
            })
        }
    }

    pub fn get_all_features(&self) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetAllFeaturesQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetAllFeaturesResult>::zeroed();
            let func = self.api.GetAllFeatures.expect("GetAllFeatures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.features.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.features as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_features_in_rectangle(&self, min_x: f32, min_z: f32, max_x: f32, max_z: f32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetFeaturesInRectangleQuery {
                minX: min_x,
                minZ: min_z,
                maxX: max_x,
                maxZ: max_z,
            };
            let mut result = MaybeUninit::<sys::GetFeaturesInRectangleResult>::zeroed();
            let func = self.api.GetFeaturesInRectangle.expect("GetFeaturesInRectangle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.features.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.features as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_features_in_sphere(&self, center: sys::Float3, radius: f32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetFeaturesInSphereQuery {
                center: center,
                radius: radius,
            };
            let mut result = MaybeUninit::<sys::GetFeaturesInSphereResult>::zeroed();
            let func = self.api.GetFeaturesInSphere.expect("GetFeaturesInSphere function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.features.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.features as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_features_in_cylinder(&self, x: f32, z: f32, radius: f32, height: f32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetFeaturesInCylinderQuery {
                x: x,
                z: z,
                radius: radius,
                height: height,
            };
            let mut result = MaybeUninit::<sys::GetFeaturesInCylinderResult>::zeroed();
            let func = self.api.GetFeaturesInCylinder.expect("GetFeaturesInCylinder function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.features.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.features as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_feature_def_id(&self, feature_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetFeatureDefIDQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDefIDResult>::zeroed();
            let func = self.api.GetFeatureDefID.expect("GetFeatureDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.defID
            })
        }
    }

    pub fn get_feature_team(&self, feature_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetFeatureTeamQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureTeamResult>::zeroed();
            let func = self.api.GetFeatureTeam.expect("GetFeatureTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.teamID
            })
        }
    }

    pub fn get_feature_ally_team(&self, feature_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetFeatureAllyTeamQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureAllyTeamResult>::zeroed();
            let func = self.api.GetFeatureAllyTeam.expect("GetFeatureAllyTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allyTeamID
            })
        }
    }

    pub fn get_feature_health(&self, feature_id: i32) -> Result<sys::FeatureHealth, Error> {
        unsafe {
            let query = sys::GetFeatureHealthQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureHealthResult>::zeroed();
            let func = self.api.GetFeatureHealth.expect("GetFeatureHealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.health
            })
        }
    }

    pub fn get_feature_height(&self, feature_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureHeightQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureHeightResult>::zeroed();
            let func = self.api.GetFeatureHeight.expect("GetFeatureHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.height
            })
        }
    }

    pub fn get_feature_radius(&self, feature_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureRadiusQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureRadiusResult>::zeroed();
            let func = self.api.GetFeatureRadius.expect("GetFeatureRadius function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.radius
            })
        }
    }

    pub fn get_feature_mass(&self, feature_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureMassQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureMassResult>::zeroed();
            let func = self.api.GetFeatureMass.expect("GetFeatureMass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.mass
            })
        }
    }

    pub fn get_feature_position(&self, feature_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetFeaturePositionQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePositionResult>::zeroed();
            let func = self.api.GetFeaturePosition.expect("GetFeaturePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_feature_position_ext(&self, feature_id: i32) -> Result<sys::FeaturePositionExt, Error> {
        unsafe {
            let query = sys::GetFeaturePositionExtQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePositionExtResult>::zeroed();
            let func = self.api.GetFeaturePositionExt.expect("GetFeaturePositionExt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_feature_separation(&self, feature_id1: i32, feature_id2: i32, positional: bool) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureSeparationQuery {
                featureID1: feature_id1,
                featureID2: feature_id2,
                positional: positional,
            };
            let mut result = MaybeUninit::<sys::GetFeatureSeparationResult>::zeroed();
            let func = self.api.GetFeatureSeparation.expect("GetFeatureSeparation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.separation
            })
        }
    }

    pub fn get_feature_direction(&self, feature_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetFeatureDirectionQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDirectionResult>::zeroed();
            let func = self.api.GetFeatureDirection.expect("GetFeatureDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.direction
            })
        }
    }

    pub fn get_feature_velocity(&self, feature_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetFeatureVelocityQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureVelocityResult>::zeroed();
            let func = self.api.GetFeatureVelocity.expect("GetFeatureVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.velocity
            })
        }
    }

    pub fn get_feature_heading(&self, feature_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetFeatureHeadingQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureHeadingResult>::zeroed();
            let func = self.api.GetFeatureHeading.expect("GetFeatureHeading function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.heading
            })
        }
    }

    pub fn get_feature_rotation(&self, feature_id: i32) -> Result<sys::FeatureRotation, Error> {
        unsafe {
            let query = sys::GetFeatureRotationQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureRotationResult>::zeroed();
            let func = self.api.GetFeatureRotation.expect("GetFeatureRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.rotation
            })
        }
    }

    pub fn get_feature_resources(&self, feature_id: i32) -> Result<sys::FeatureResources, Error> {
        unsafe {
            let query = sys::GetFeatureResourcesQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureResourcesResult>::zeroed();
            let func = self.api.GetFeatureResources.expect("GetFeatureResources function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.resources
            })
        }
    }

    pub fn get_feature_blocking(&self, feature_id: i32) -> Result<sys::FeatureBlockingState, Error> {
        unsafe {
            let query = sys::GetFeatureBlockingQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureBlockingResult>::zeroed();
            let func = self.api.GetFeatureBlocking.expect("GetFeatureBlocking function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.blockingState
            })
        }
    }

    pub fn get_feature_no_select(&self, feature_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetFeatureNoSelectQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureNoSelectResult>::zeroed();
            let func = self.api.GetFeatureNoSelect.expect("GetFeatureNoSelect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.noSelect
            })
        }
    }

    pub fn get_feature_resurrect(&self, feature_id: i32) -> Result<(sys::FeatureResurrect, bool), Error> {
        unsafe {
            let query = sys::GetFeatureResurrectQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureResurrectResult>::zeroed();
            let func = self.api.GetFeatureResurrect.expect("GetFeatureResurrect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.resurrect,
                result.canResurrect,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_feature_last_attacked_piece(&self, feature_id: i32) -> Result<sys::FeatureLastHitPiece, Error> {
        unsafe {
            let query = sys::GetFeatureLastAttackedPieceQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureLastAttackedPieceResult>::zeroed();
            let func = self.api.GetFeatureLastAttackedPiece.expect("GetFeatureLastAttackedPiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.piece
            })
        }
    }

    pub fn get_feature_collision_volume_data(&self, feature_id: i32) -> Result<sys::CollisionVolumeData, Error> {
        unsafe {
            let query = sys::GetFeatureCollisionVolumeDataQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureCollisionVolumeDataResult>::zeroed();
            let func = self.api.GetFeatureCollisionVolumeData.expect("GetFeatureCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.volume
            })
        }
    }

    pub fn get_feature_piece_collision_volume_data(&self, feature_id: i32, piece_num: i32) -> Result<sys::CollisionVolumeData, Error> {
        unsafe {
            let query = sys::GetFeaturePieceCollisionVolumeDataQuery {
                featureID: feature_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetFeaturePieceCollisionVolumeDataResult>::zeroed();
            let func = self.api.GetFeaturePieceCollisionVolumeData.expect("GetFeaturePieceCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.volume
            })
        }
    }

    pub fn clear_features_previous_draw_flag(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::ClearFeaturesPreviousDrawFlagQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::ClearFeaturesPreviousDrawFlagResult>::zeroed();
            let func = self.api.ClearFeaturesPreviousDrawFlag.expect("ClearFeaturesPreviousDrawFlag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_feature_no_draw(&self, feature_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetFeatureNoDrawQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureNoDrawResult>::zeroed();
            let func = self.api.GetFeatureNoDraw.expect("GetFeatureNoDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.noDraw
            })
        }
    }

    pub fn get_feature_lua_draw(&self, feature_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetFeatureLuaDrawQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureLuaDrawResult>::zeroed();
            let func = self.api.GetFeatureLuaDraw.expect("GetFeatureLuaDraw function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.luaDraw
            })
        }
    }

    pub fn get_feature_engine_draw_mask(&self, feature_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetFeatureEngineDrawMaskQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureEngineDrawMaskResult>::zeroed();
            let func = self.api.GetFeatureEngineDrawMask.expect("GetFeatureEngineDrawMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.mask
            })
        }
    }

    pub fn get_feature_draw_flag(&self, feature_id: i32) -> Result<u8, Error> {
        unsafe {
            let query = sys::GetFeatureDrawFlagQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureDrawFlagResult>::zeroed();
            let func = self.api.GetFeatureDrawFlag.expect("GetFeatureDrawFlag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.flag
            })
        }
    }

    pub fn get_feature_always_update_matrix(&self, feature_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetFeatureAlwaysUpdateMatrixQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureAlwaysUpdateMatrixResult>::zeroed();
            let func = self.api.GetFeatureAlwaysUpdateMatrix.expect("GetFeatureAlwaysUpdateMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.update
            })
        }
    }

    pub fn get_feature_transform_matrix(&self, feature_id: i32) -> Result<sys::FeatureTransformMatrix, Error> {
        unsafe {
            let query = sys::GetFeatureTransformMatrixQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureTransformMatrixResult>::zeroed();
            let func = self.api.GetFeatureTransformMatrix.expect("GetFeatureTransformMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.matrix
            })
        }
    }

    pub fn get_feature_selection_volume_data(&self, feature_id: i32) -> Result<sys::FeatureSelectionVolumeData, Error> {
        unsafe {
            let query = sys::GetFeatureSelectionVolumeDataQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureSelectionVolumeDataResult>::zeroed();
            let func = self.api.GetFeatureSelectionVolumeData.expect("GetFeatureSelectionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.data
            })
        }
    }

    pub fn get_feature_fire_time(&self, feature_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureFireTimeQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureFireTimeResult>::zeroed();
            let func = self.api.GetFeatureFireTime.expect("GetFeatureFireTime function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.fireTime
            })
        }
    }

    pub fn get_feature_smoke_time(&self, feature_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetFeatureSmokeTimeQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::GetFeatureSmokeTimeResult>::zeroed();
            let func = self.api.GetFeatureSmokeTime.expect("GetFeatureSmokeTime function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.smokeTime
            })
        }
    }

    pub fn get_render_features(&self, draw_mask: i32, send_mask: bool) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetRenderFeaturesQuery {
                drawMask: draw_mask,
                sendMask: send_mask,
            };
            let mut result = MaybeUninit::<sys::GetRenderFeaturesResult>::zeroed();
            let func = self.api.GetRenderFeatures.expect("GetRenderFeatures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.features.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.features as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_render_features_draw_flag_changed(&self, send_mask: bool) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetRenderFeaturesDrawFlagChangedQuery {
                sendMask: send_mask,
            };
            let mut result = MaybeUninit::<sys::GetRenderFeaturesDrawFlagChangedResult>::zeroed();
            let func = self.api.GetRenderFeaturesDrawFlagChanged.expect("GetRenderFeaturesDrawFlagChanged function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.features.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.features as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

}

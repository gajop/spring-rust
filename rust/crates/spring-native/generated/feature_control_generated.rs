#[derive(Debug, Clone, Copy, Default)]
pub struct SetFeatureBlockingOptions {
    pub blocking: bool,
    pub solid_objects: bool,
    pub projectiles: bool,
    pub quad_map_rays: bool,
    pub crushable: bool,
    pub block_enemy_pushing: bool,
    pub block_height_changes: bool,
}

impl From<SetFeatureBlockingOptions> for sys::SetFeatureBlockingOptions {
    fn from(options: SetFeatureBlockingOptions) -> Self {
        sys::SetFeatureBlockingOptions {
            blocking: options.blocking,
            solidObjects: options.solid_objects,
            projectiles: options.projectiles,
            quadMapRays: options.quad_map_rays,
            crushable: options.crushable,
            blockEnemyPushing: options.block_enemy_pushing,
            blockHeightChanges: options.block_height_changes,
        }
    }
}

impl<'a> FeatureControl<'a> {
    pub fn create_feature(&self, feature_def: sys::DefRef, pos: sys::Float3, facing: i32, team_id: i32, feature_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::CreateFeatureQuery {
                featureDef: feature_def,
                pos,
                facing,
                teamID: team_id,
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::CreateFeatureResult>::zeroed();
            let func = self.api.CreateFeature.expect("CreateFeature function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.featureID
            })
        }
    }

    pub fn destroy_feature(&self, feature_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::DestroyFeatureQuery {
                featureID: feature_id,
            };
            let mut result = MaybeUninit::<sys::DestroyFeatureResult>::zeroed();
            let func = self.api.DestroyFeature.expect("DestroyFeature function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn transfer_feature(&self, feature_id: i32, new_team_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::TransferFeatureQuery {
                featureID: feature_id,
                newTeamID: new_team_id,
            };
            let mut result = MaybeUninit::<sys::TransferFeatureResult>::zeroed();
            let func = self.api.TransferFeature.expect("TransferFeature function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_health(&self, feature_id: i32, health: f32, check_destruction: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureHealthQuery {
                featureID: feature_id,
                health,
                checkDestruction: check_destruction,
            };
            let mut result = MaybeUninit::<sys::SetFeatureHealthResult>::zeroed();
            let func = self.api.SetFeatureHealth.expect("SetFeatureHealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_position(&self, feature_id: i32, pos: sys::Float3, snap_to_ground: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeaturePositionQuery {
                featureID: feature_id,
                pos,
                snapToGround: snap_to_ground,
            };
            let mut result = MaybeUninit::<sys::SetFeaturePositionResult>::zeroed();
            let func = self.api.SetFeaturePosition.expect("SetFeaturePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_direction(&self, feature_id: i32, front_dir: sys::Float3, right_dir: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureDirectionQuery {
                featureID: feature_id,
                frontDir: front_dir,
                rightDir: right_dir,
            };
            let mut result = MaybeUninit::<sys::SetFeatureDirectionResult>::zeroed();
            let func = self.api.SetFeatureDirection.expect("SetFeatureDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_velocity(&self, feature_id: i32, velocity: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureVelocityQuery {
                featureID: feature_id,
                velocity,
            };
            let mut result = MaybeUninit::<sys::SetFeatureVelocityResult>::zeroed();
            let func = self.api.SetFeatureVelocity.expect("SetFeatureVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_resources(&self, feature_id: i32, metal: f32, energy: f32, reclaim_time: f32, reclaim_left: f32, feature_def_metal: f32, feature_def_energy: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureResourcesQuery {
                featureID: feature_id,
                metal,
                energy,
                reclaimTime: reclaim_time,
                reclaimLeft: reclaim_left,
                featureDefMetal: feature_def_metal,
                featureDefEnergy: feature_def_energy,
            };
            let mut result = MaybeUninit::<sys::SetFeatureResourcesResult>::zeroed();
            let func = self.api.SetFeatureResources.expect("SetFeatureResources function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_feature_damage(&self, feature_id: i32, damage: f32, paralyze_time: f32, weapon_def_id: i32, attacker_id: i32, impulse: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddFeatureDamageQuery {
                featureID: feature_id,
                damage,
                paralyzeTime: paralyze_time,
                weaponDefID: weapon_def_id,
                attackerID: attacker_id,
                impulse,
            };
            let mut result = MaybeUninit::<sys::AddFeatureDamageResult>::zeroed();
            let func = self.api.AddFeatureDamage.expect("AddFeatureDamage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_blocking(&self, feature_id: i32, options: SetFeatureBlockingOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureBlockingQuery {
                featureID: feature_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetFeatureBlockingResult>::zeroed();
            let func = self.api.SetFeatureBlocking.expect("SetFeatureBlocking function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_mass(&self, feature_id: i32, mass: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureMassQuery {
                featureID: feature_id,
                mass,
            };
            let mut result = MaybeUninit::<sys::SetFeatureMassResult>::zeroed();
            let func = self.api.SetFeatureMass.expect("SetFeatureMass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_max_health(&self, feature_id: i32, max_health: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureMaxHealthQuery {
                featureID: feature_id,
                maxHealth: max_health,
            };
            let mut result = MaybeUninit::<sys::SetFeatureMaxHealthResult>::zeroed();
            let func = self.api.SetFeatureMaxHealth.expect("SetFeatureMaxHealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_reclaim(&self, feature_id: i32, reclaim_left: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureReclaimQuery {
                featureID: feature_id,
                reclaimLeft: reclaim_left,
            };
            let mut result = MaybeUninit::<sys::SetFeatureReclaimResult>::zeroed();
            let func = self.api.SetFeatureReclaim.expect("SetFeatureReclaim function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_resurrect(&self, feature_id: i32, unit_def: sys::DefRef, facing: i32, progress: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureResurrectQuery {
                featureID: feature_id,
                unitDef: unit_def,
                facing,
                progress,
            };
            let mut result = MaybeUninit::<sys::SetFeatureResurrectResult>::zeroed();
            let func = self.api.SetFeatureResurrect.expect("SetFeatureResurrect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_physics(&self, feature_id: i32, pos: sys::Float3, velocity: sys::Float3, rotation: sys::Float3, drag: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeaturePhysicsQuery {
                featureID: feature_id,
                pos,
                velocity,
                rotation,
                drag,
            };
            let mut result = MaybeUninit::<sys::SetFeaturePhysicsResult>::zeroed();
            let func = self.api.SetFeaturePhysics.expect("SetFeaturePhysics function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_move_ctrl(&self, feature_id: i32, enable: bool, velocity_or_mask: sys::Float3, acceleration_or_impulse_mask: sys::Float3, movement_mask: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureMoveCtrlQuery {
                featureID: feature_id,
                enable,
                velocityOrMask: velocity_or_mask,
                accelerationOrImpulseMask: acceleration_or_impulse_mask,
                movementMask: movement_mask,
            };
            let mut result = MaybeUninit::<sys::SetFeatureMoveCtrlResult>::zeroed();
            let func = self.api.SetFeatureMoveCtrl.expect("SetFeatureMoveCtrl function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_heading_and_up_dir(&self, feature_id: i32, heading: i32, up_dir: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureHeadingAndUpDirQuery {
                featureID: feature_id,
                heading,
                upDir: up_dir,
            };
            let mut result = MaybeUninit::<sys::SetFeatureHeadingAndUpDirResult>::zeroed();
            let func = self.api.SetFeatureHeadingAndUpDir.expect("SetFeatureHeadingAndUpDir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_rotation(&self, feature_id: i32, rotation: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureRotationQuery {
                featureID: feature_id,
                rotation,
            };
            let mut result = MaybeUninit::<sys::SetFeatureRotationResult>::zeroed();
            let func = self.api.SetFeatureRotation.expect("SetFeatureRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_always_visible(&self, feature_id: i32, always_visible: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureAlwaysVisibleQuery {
                featureID: feature_id,
                alwaysVisible: always_visible,
            };
            let mut result = MaybeUninit::<sys::SetFeatureAlwaysVisibleResult>::zeroed();
            let func = self.api.SetFeatureAlwaysVisible.expect("SetFeatureAlwaysVisible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_use_air_los(&self, feature_id: i32, use_air_los: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureUseAirLosQuery {
                featureID: feature_id,
                useAirLos: use_air_los,
            };
            let mut result = MaybeUninit::<sys::SetFeatureUseAirLosResult>::zeroed();
            let func = self.api.SetFeatureUseAirLos.expect("SetFeatureUseAirLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_no_select(&self, feature_id: i32, no_select: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureNoSelectQuery {
                featureID: feature_id,
                noSelect: no_select,
            };
            let mut result = MaybeUninit::<sys::SetFeatureNoSelectResult>::zeroed();
            let func = self.api.SetFeatureNoSelect.expect("SetFeatureNoSelect function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_mid_and_aim_pos(&self, feature_id: i32, mid_pos: sys::Float3, aim_pos: sys::Float3, set_relative: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureMidAndAimPosQuery {
                featureID: feature_id,
                midPos: mid_pos,
                aimPos: aim_pos,
                setRelative: set_relative,
            };
            let mut result = MaybeUninit::<sys::SetFeatureMidAndAimPosResult>::zeroed();
            let func = self.api.SetFeatureMidAndAimPos.expect("SetFeatureMidAndAimPos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_radius_and_height(&self, feature_id: i32, radius: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureRadiusAndHeightQuery {
                featureID: feature_id,
                radius,
                height,
            };
            let mut result = MaybeUninit::<sys::SetFeatureRadiusAndHeightResult>::zeroed();
            let func = self.api.SetFeatureRadiusAndHeight.expect("SetFeatureRadiusAndHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_collision_volume_data(&self, feature_id: i32, scales: sys::Float3, offsets: sys::Float3, volume_type: i32, test_type: i32, primary_axis: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureCollisionVolumeDataQuery {
                featureID: feature_id,
                scales,
                offsets,
                volumeType: volume_type,
                testType: test_type,
                primaryAxis: primary_axis,
            };
            let mut result = MaybeUninit::<sys::SetFeatureCollisionVolumeDataResult>::zeroed();
            let func = self.api.SetFeatureCollisionVolumeData.expect("SetFeatureCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_selection_volume_data(&self, feature_id: i32, scales: sys::Float3, offsets: sys::Float3, volume_type: i32, primary_axis: i32, use_cont_hit_test: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureSelectionVolumeDataQuery {
                featureID: feature_id,
                scales,
                offsets,
                volumeType: volume_type,
                primaryAxis: primary_axis,
                useContHitTest: use_cont_hit_test,
            };
            let mut result = MaybeUninit::<sys::SetFeatureSelectionVolumeDataResult>::zeroed();
            let func = self.api.SetFeatureSelectionVolumeData.expect("SetFeatureSelectionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_fire_time(&self, feature_id: i32, fire_time: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureFireTimeQuery {
                featureID: feature_id,
                fireTime: fire_time,
            };
            let mut result = MaybeUninit::<sys::SetFeatureFireTimeResult>::zeroed();
            let func = self.api.SetFeatureFireTime.expect("SetFeatureFireTime function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_smoke_time(&self, feature_id: i32, smoke_time: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeatureSmokeTimeQuery {
                featureID: feature_id,
                smokeTime: smoke_time,
            };
            let mut result = MaybeUninit::<sys::SetFeatureSmokeTimeResult>::zeroed();
            let func = self.api.SetFeatureSmokeTime.expect("SetFeatureSmokeTime function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn create_unit_wreck(&self, unit_id: i32, wreck_level: i32, do_smoke: bool) -> Result<i32, Error> {
        unsafe {
            let query = sys::CreateUnitWreckQuery {
                unitID: unit_id,
                wreckLevel: wreck_level,
                doSmoke: do_smoke,
            };
            let mut result = MaybeUninit::<sys::CreateUnitWreckResult>::zeroed();
            let func = self.api.CreateUnitWreck.expect("CreateUnitWreck function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.featureID
            })
        }
    }

    pub fn create_feature_wreck(&self, feature_id: i32, wreck_level: i32, do_smoke: bool) -> Result<i32, Error> {
        unsafe {
            let query = sys::CreateFeatureWreckQuery {
                featureID: feature_id,
                wreckLevel: wreck_level,
                doSmoke: do_smoke,
            };
            let mut result = MaybeUninit::<sys::CreateFeatureWreckResult>::zeroed();
            let func = self.api.CreateFeatureWreck.expect("CreateFeatureWreck function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.featureID
            })
        }
    }

    pub fn set_feature_piece_visible(&self, feature_id: i32, piece_index: i32, visible: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeaturePieceVisibleQuery {
                featureID: feature_id,
                pieceIndex: piece_index,
                visible,
            };
            let mut result = MaybeUninit::<sys::SetFeaturePieceVisibleResult>::zeroed();
            let func = self.api.SetFeaturePieceVisible.expect("SetFeaturePieceVisible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_feature_piece_matrix(&self, feature_id: i32, piece_index: i32, matrix: [f32; 16]) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeaturePieceMatrixQuery {
                featureID: feature_id,
                pieceIndex: piece_index,
                matrix,
            };
            let mut result = MaybeUninit::<sys::SetFeaturePieceMatrixResult>::zeroed();
            let func = self.api.SetFeaturePieceMatrix.expect("SetFeaturePieceMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.blockScriptAnims
            })
        }
    }

    pub fn set_feature_piece_collision_volume_data(&self, feature_id: i32, piece_index: i32, enable: bool, scales: sys::Float3, offsets: sys::Float3, volume_type: i32, primary_axis: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFeaturePieceCollisionVolumeDataQuery {
                featureID: feature_id,
                pieceIndex: piece_index,
                enable,
                scales,
                offsets,
                volumeType: volume_type,
                primaryAxis: primary_axis,
            };
            let mut result = MaybeUninit::<sys::SetFeaturePieceCollisionVolumeDataResult>::zeroed();
            let func = self.api.SetFeaturePieceCollisionVolumeData.expect("SetFeaturePieceCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

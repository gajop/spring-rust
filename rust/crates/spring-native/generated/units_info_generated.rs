#[derive(Debug, Clone, Copy, Default)]
pub struct GetUnitPositionOptions {
    pub mid_pos: bool,
    pub aim_pos: bool,
}

impl From<GetUnitPositionOptions> for sys::GetUnitPositionOptions {
    fn from(options: GetUnitPositionOptions) -> Self {
        sys::GetUnitPositionOptions {
            midPos: options.mid_pos,
            aimPos: options.aim_pos,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UnitStatesOptions {
    pub ret_table: bool,
    pub bin_state: bool,
    pub amt_state: bool,
}

impl From<UnitStatesOptions> for sys::UnitStatesOptions {
    fn from(options: UnitStatesOptions) -> Self {
        sys::UnitStatesOptions {
            retTable: options.ret_table,
            binState: options.bin_state,
            amtState: options.amt_state,
        }
    }
}

impl<'a> UnitsInfo<'a> {
    pub fn get_unit_tooltip(&self, unit_id: i32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetUnitTooltipQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitTooltipResult>::zeroed();
            let func = self.api.GetUnitTooltip.expect("GetUnitTooltip function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.tooltip.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.tooltip).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_unit_def_id(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitDefIDQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDefIDResult>::zeroed();
            let func = self.api.GetUnitDefID.expect("GetUnitDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitDefID
            })
        }
    }

    pub fn get_unit_move_def_id(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitMoveDefIDQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitMoveDefIDResult>::zeroed();
            let func = self.api.GetUnitMoveDefID.expect("GetUnitMoveDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.moveDefID
            })
        }
    }

    pub fn get_unit_team(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitTeamQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitTeamResult>::zeroed();
            let func = self.api.GetUnitTeam.expect("GetUnitTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.teamID
            })
        }
    }

    pub fn get_unit_ally_team(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitAllyTeamQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitAllyTeamResult>::zeroed();
            let func = self.api.GetUnitAllyTeam.expect("GetUnitAllyTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allyTeamID
            })
        }
    }

    pub fn get_unit_neutral(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitNeutralQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitNeutralResult>::zeroed();
            let func = self.api.GetUnitNeutral.expect("GetUnitNeutral function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.neutral
            })
        }
    }

    pub fn get_unit_health(&self, unit_id: i32) -> Result<sys::UnitHealth, Error> {
        unsafe {
            let query = sys::GetUnitHealthQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitHealthResult>::zeroed();
            let func = self.api.GetUnitHealth.expect("GetUnitHealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.health
            })
        }
    }

    pub fn get_unit_is_dead(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitIsDeadQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIsDeadResult>::zeroed();
            let func = self.api.GetUnitIsDead.expect("GetUnitIsDead function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isDead
            })
        }
    }

    pub fn get_unit_is_stunned(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitIsStunnedQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIsStunnedResult>::zeroed();
            let func = self.api.GetUnitIsStunned.expect("GetUnitIsStunned function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isStunned
            })
        }
    }

    pub fn get_unit_is_being_built(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitIsBeingBuiltQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIsBeingBuiltResult>::zeroed();
            let func = self.api.GetUnitIsBeingBuilt.expect("GetUnitIsBeingBuilt function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isBeingBuilt
            })
        }
    }

    pub fn get_unit_costs(&self, unit_id: i32) -> Result<sys::UnitCosts, Error> {
        unsafe {
            let query = sys::GetUnitCostsQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitCostsResult>::zeroed();
            let func = self.api.GetUnitCosts.expect("GetUnitCosts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.costs
            })
        }
    }

    pub fn get_unit_cost_table(&self, unit_id: i32) -> Result<sys::UnitCosts, Error> {
        unsafe {
            let query = sys::GetUnitCostTableQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitCostTableResult>::zeroed();
            let func = self.api.GetUnitCostTable.expect("GetUnitCostTable function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.costs
            })
        }
    }

    pub fn get_unit_resources(&self, unit_id: i32) -> Result<sys::UnitResources, Error> {
        unsafe {
            let query = sys::GetUnitResourcesQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitResourcesResult>::zeroed();
            let func = self.api.GetUnitResources.expect("GetUnitResources function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.resources
            })
        }
    }

    pub fn get_unit_storage(&self, unit_id: i32) -> Result<sys::UnitStorage, Error> {
        unsafe {
            let query = sys::GetUnitStorageQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitStorageResult>::zeroed();
            let func = self.api.GetUnitStorage.expect("GetUnitStorage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.storage
            })
        }
    }

    pub fn get_unit_metal_extraction(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitMetalExtractionQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitMetalExtractionResult>::zeroed();
            let func = self.api.GetUnitMetalExtraction.expect("GetUnitMetalExtraction function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.metalExtraction
            })
        }
    }

    pub fn get_unit_experience(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitExperienceQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitExperienceResult>::zeroed();
            let func = self.api.GetUnitExperience.expect("GetUnitExperience function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.experience
            })
        }
    }

    pub fn get_unit_states(&self, unit_id: i32, options: UnitStatesOptions) -> Result<sys::UnitStates, Error> {
        unsafe {
            let query = sys::GetUnitStatesQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetUnitStatesResult>::zeroed();
            let func = self.api.GetUnitStates.expect("GetUnitStates function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.states
            })
        }
    }

    pub fn get_unit_armored(&self, unit_id: i32) -> Result<sys::UnitArmoredState, Error> {
        unsafe {
            let query = sys::GetUnitArmoredQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitArmoredResult>::zeroed();
            let func = self.api.GetUnitArmored.expect("GetUnitArmored function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.armoredState
            })
        }
    }

    pub fn get_unit_is_active(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitIsActiveQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIsActiveResult>::zeroed();
            let func = self.api.GetUnitIsActive.expect("GetUnitIsActive function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isActive
            })
        }
    }

    pub fn get_unit_is_cloaked(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitIsCloakedQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIsCloakedResult>::zeroed();
            let func = self.api.GetUnitIsCloaked.expect("GetUnitIsCloaked function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isCloaked
            })
        }
    }

    pub fn get_unit_seismic_signature(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitSeismicSignatureQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitSeismicSignatureResult>::zeroed();
            let func = self.api.GetUnitSeismicSignature.expect("GetUnitSeismicSignature function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.seismicSignature
            })
        }
    }

    pub fn get_unit_sensor_radius(&self, unit_id: i32, r#type: &str) -> Result<sys::UnitSensorRadius, Error> {
        unsafe {
            let r#type_cstr = std::ffi::CString::new(r#type).map_err(|_| Error::invalid_argument("r#type"))?;
            let query = sys::GetUnitSensorRadiusQuery {
                unitID: unit_id,
                type_: r#type_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitSensorRadiusResult>::zeroed();
            let func = self.api.GetUnitSensorRadius.expect("GetUnitSensorRadius function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.radius
            })
        }
    }

    pub fn get_unit_pos_error_params(&self, unit_id: i32, ally_team_id: i32) -> Result<sys::UnitPosErrorParams, Error> {
        unsafe {
            let query = sys::GetUnitPosErrorParamsQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitPosErrorParamsResult>::zeroed();
            let func = self.api.GetUnitPosErrorParams.expect("GetUnitPosErrorParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.params
            })
        }
    }

    pub fn get_unit_height(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitHeightQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitHeightResult>::zeroed();
            let func = self.api.GetUnitHeight.expect("GetUnitHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.height
            })
        }
    }

    pub fn get_unit_radius(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitRadiusQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitRadiusResult>::zeroed();
            let func = self.api.GetUnitRadius.expect("GetUnitRadius function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.radius
            })
        }
    }

    pub fn get_unit_buildee_radius(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitBuildeeRadiusQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitBuildeeRadiusResult>::zeroed();
            let func = self.api.GetUnitBuildeeRadius.expect("GetUnitBuildeeRadius function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.radius
            })
        }
    }

    pub fn get_unit_mass(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitMassQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitMassResult>::zeroed();
            let func = self.api.GetUnitMass.expect("GetUnitMass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.mass
            })
        }
    }

    pub fn get_unit_position(&self, unit_id: i32, options: GetUnitPositionOptions) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitPositionQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetUnitPositionResult>::zeroed();
            let func = self.api.GetUnitPosition.expect("GetUnitPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_unit_base_position(&self, unit_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitBasePositionQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitBasePositionResult>::zeroed();
            let func = self.api.GetUnitBasePosition.expect("GetUnitBasePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_unit_vectors(&self, unit_id: i32) -> Result<sys::UnitVectors, Error> {
        unsafe {
            let query = sys::GetUnitVectorsQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitVectorsResult>::zeroed();
            let func = self.api.GetUnitVectors.expect("GetUnitVectors function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.vectors
            })
        }
    }

    pub fn get_unit_rotation(&self, unit_id: i32) -> Result<sys::UnitRotation, Error> {
        unsafe {
            let query = sys::GetUnitRotationQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitRotationResult>::zeroed();
            let func = self.api.GetUnitRotation.expect("GetUnitRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.rotation
            })
        }
    }

    pub fn get_unit_direction(&self, unit_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitDirectionQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitDirectionResult>::zeroed();
            let func = self.api.GetUnitDirection.expect("GetUnitDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.direction
            })
        }
    }

    pub fn get_unit_heading(&self, unit_id: i32, convert_to_radians: bool) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitHeadingQuery {
                unitID: unit_id,
                convertToRadians: convert_to_radians,
            };
            let mut result = MaybeUninit::<sys::GetUnitHeadingResult>::zeroed();
            let func = self.api.GetUnitHeading.expect("GetUnitHeading function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.heading
            })
        }
    }

    pub fn get_unit_velocity(&self, unit_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetUnitVelocityQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitVelocityResult>::zeroed();
            let func = self.api.GetUnitVelocity.expect("GetUnitVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.velocity
            })
        }
    }

    pub fn get_unit_build_facing(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitBuildFacingQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitBuildFacingResult>::zeroed();
            let func = self.api.GetUnitBuildFacing.expect("GetUnitBuildFacing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.facing
            })
        }
    }

    pub fn get_unit_is_building(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitIsBuildingQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIsBuildingResult>::zeroed();
            let func = self.api.GetUnitIsBuilding.expect("GetUnitIsBuilding function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.buildeeID
            })
        }
    }

    pub fn get_unit_worker_task(&self, unit_id: i32) -> Result<sys::UnitWorkerTask, Error> {
        unsafe {
            let query = sys::GetUnitWorkerTaskQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitWorkerTaskResult>::zeroed();
            let func = self.api.GetUnitWorkerTask.expect("GetUnitWorkerTask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.task
            })
        }
    }

    pub fn get_unit_effective_build_range(&self, unit_id: i32, buildee_def_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitEffectiveBuildRangeQuery {
                unitID: unit_id,
                buildeeDefID: buildee_def_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitEffectiveBuildRangeResult>::zeroed();
            let func = self.api.GetUnitEffectiveBuildRange.expect("GetUnitEffectiveBuildRange function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.range
            })
        }
    }

    pub fn get_unit_current_build_power(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitCurrentBuildPowerQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitCurrentBuildPowerResult>::zeroed();
            let func = self.api.GetUnitCurrentBuildPower.expect("GetUnitCurrentBuildPower function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.buildPower
            })
        }
    }

    pub fn get_unit_build_params(&self, unit_id: i32, param_name: &str) -> Result<(sys::NumberOrBool, bool), Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let query = sys::GetUnitBuildParamsQuery {
                unitID: unit_id,
                paramName: param_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetUnitBuildParamsResult>::zeroed();
            let func = self.api.GetUnitBuildParams.expect("GetUnitBuildParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.value,
                result.hasValue,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_in_build_stance(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitInBuildStanceQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitInBuildStanceResult>::zeroed();
            let func = self.api.GetUnitInBuildStance.expect("GetUnitInBuildStance function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.inBuildStance
            })
        }
    }

    pub fn get_unit_nano_pieces(&self, unit_id: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetUnitNanoPiecesQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitNanoPiecesResult>::zeroed();
            let func = self.api.GetUnitNanoPieces.expect("GetUnitNanoPieces function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.pieces.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.pieces as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_transporter(&self, unit_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetUnitTransporterQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitTransporterResult>::zeroed();
            let func = self.api.GetUnitTransporter.expect("GetUnitTransporter function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.transporterID
            })
        }
    }

    pub fn get_unit_is_transporting(&self, unit_id: i32) -> Result<(Vec<i32>, bool), Error> {
        unsafe {
            let query = sys::GetUnitIsTransportingQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitIsTransportingResult>::zeroed();
            let func = self.api.GetUnitIsTransporting.expect("GetUnitIsTransporting function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    let slice = if result.count == 0 || result.unitIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.unitIDs as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                },
                result.isTransporting,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_stockpile(&self, unit_id: i32) -> Result<(sys::UnitStockpile, bool), Error> {
        unsafe {
            let query = sys::GetUnitStockpileQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitStockpileResult>::zeroed();
            let func = self.api.GetUnitStockpile.expect("GetUnitStockpile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.stockpile,
                result.hasStockpile,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_self_dtime(&self, unit_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitSelfDTimeQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitSelfDTimeResult>::zeroed();
            let func = self.api.GetUnitSelfDTime.expect("GetUnitSelfDTime function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.selfDTime
            })
        }
    }

    pub fn get_unit_shield_state(&self, unit_id: i32, weapon_num: i32) -> Result<(sys::UnitShieldState, bool), Error> {
        unsafe {
            let query = sys::GetUnitShieldStateQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitShieldStateResult>::zeroed();
            let func = self.api.GetUnitShieldState.expect("GetUnitShieldState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.shield,
                result.hasShield,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_flanking(&self, unit_id: i32) -> Result<sys::UnitFlanking, Error> {
        unsafe {
            let query = sys::GetUnitFlankingQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitFlankingResult>::zeroed();
            let func = self.api.GetUnitFlanking.expect("GetUnitFlanking function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.flanking
            })
        }
    }

    pub fn get_unit_last_attacker(&self, unit_id: i32) -> Result<(sys::UnitLastAttacker, bool), Error> {
        unsafe {
            let query = sys::GetUnitLastAttackerQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitLastAttackerResult>::zeroed();
            let func = self.api.GetUnitLastAttacker.expect("GetUnitLastAttacker function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.attacker,
                result.hasAttacker,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_unit_last_attacked_piece(&self, unit_id: i32) -> Result<sys::LastHitPiece, Error> {
        unsafe {
            let query = sys::GetUnitLastAttackedPieceQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitLastAttackedPieceResult>::zeroed();
            let func = self.api.GetUnitLastAttackedPiece.expect("GetUnitLastAttackedPiece function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.piece
            })
        }
    }

    pub fn get_unit_los_state(&self, unit_id: i32, ally_team_id: i32, raw: bool) -> Result<sys::UnitLosState, Error> {
        unsafe {
            let query = sys::GetUnitLosStateQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
                raw,
            };
            let mut result = MaybeUninit::<sys::GetUnitLosStateResult>::zeroed();
            let func = self.api.GetUnitLosState.expect("GetUnitLosState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.losState
            })
        }
    }

    pub fn get_unit_collision_volume_data(&self, unit_id: i32) -> Result<sys::CollisionVolumeData, Error> {
        unsafe {
            let query = sys::GetUnitCollisionVolumeDataQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitCollisionVolumeDataResult>::zeroed();
            let func = self.api.GetUnitCollisionVolumeData.expect("GetUnitCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.volume
            })
        }
    }

    pub fn get_unit_piece_collision_volume_data(&self, unit_id: i32, piece_num: i32) -> Result<sys::CollisionVolumeData, Error> {
        unsafe {
            let query = sys::GetUnitPieceCollisionVolumeDataQuery {
                unitID: unit_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::GetUnitPieceCollisionVolumeDataResult>::zeroed();
            let func = self.api.GetUnitPieceCollisionVolumeData.expect("GetUnitPieceCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.volume
            })
        }
    }

    pub fn get_unit_blocking(&self, unit_id: i32) -> Result<sys::UnitBlockingState, Error> {
        unsafe {
            let query = sys::GetUnitBlockingQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitBlockingResult>::zeroed();
            let func = self.api.GetUnitBlocking.expect("GetUnitBlocking function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.blockingState
            })
        }
    }

    pub fn get_unit_harvest_storage(&self, unit_id: i32) -> Result<sys::UnitHarvestStorage, Error> {
        unsafe {
            let query = sys::GetUnitHarvestStorageQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitHarvestStorageResult>::zeroed();
            let func = self.api.GetUnitHarvestStorage.expect("GetUnitHarvestStorage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.storage
            })
        }
    }

    pub fn clear_units_previous_draw_flag(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::ClearUnitsPreviousDrawFlagQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::ClearUnitsPreviousDrawFlagResult>::zeroed();
            let func = self.api.ClearUnitsPreviousDrawFlag.expect("ClearUnitsPreviousDrawFlag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_unit_crashing(&self, unit_id: i32) -> Result<(bool, bool), Error> {
        unsafe {
            let query = sys::GetUnitCrashingQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitCrashingResult>::zeroed();
            let func = self.api.GetUnitCrashing.expect("GetUnitCrashing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.isAircraft,
                result.crashing,
            );
            Error::result_or(result.error, value)
        }
    }

}

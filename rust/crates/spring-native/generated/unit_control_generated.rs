#[derive(Debug, Clone, Copy, Default)]
pub struct BuggerOffOptions {
    pub spherical: bool,
    pub forced: bool,
    pub exclude_unit_id: i32,
}

impl From<BuggerOffOptions> for sys::BuggerOffOptions {
    fn from(options: BuggerOffOptions) -> Self {
        sys::BuggerOffOptions {
            spherical: options.spherical,
            forced: options.forced,
            excludeUnitID: options.exclude_unit_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CreateUnitOptions {
    pub build: bool,
    pub flatten_ground: bool,
    pub unit_id: i32,
    pub builder_id: i32,
}

impl From<CreateUnitOptions> for sys::CreateUnitOptions {
    fn from(options: CreateUnitOptions) -> Self {
        sys::CreateUnitOptions {
            build: options.build,
            flattenGround: options.flatten_ground,
            unitID: options.unit_id,
            builderID: options.builder_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DestroyUnitOptions {
    pub selfd: bool,
    pub reclaimed: bool,
    pub attacker_id: i32,
    pub recycle_id: bool,
}

impl From<DestroyUnitOptions> for sys::DestroyUnitOptions {
    fn from(options: DestroyUnitOptions) -> Self {
        sys::DestroyUnitOptions {
            selfd: options.selfd,
            reclaimed: options.reclaimed,
            attackerID: options.attacker_id,
            recycleID: options.recycle_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SetFactoryBuggerOffOptions {
    pub perform: bool,
    pub offset: f32,
    pub radius: f32,
    pub rel_heading: i32,
    pub spherical: bool,
    pub forced: bool,
}

impl From<SetFactoryBuggerOffOptions> for sys::SetFactoryBuggerOffOptions {
    fn from(options: SetFactoryBuggerOffOptions) -> Self {
        sys::SetFactoryBuggerOffOptions {
            perform: options.perform,
            offset: options.offset,
            radius: options.radius,
            relHeading: options.rel_heading,
            spherical: options.spherical,
            forced: options.forced,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SetUnitBlockingOptions {
    pub blocking: bool,
    pub solid_objects: bool,
    pub projectiles: bool,
    pub quad_map_rays: bool,
    pub crushable: bool,
    pub block_enemy_pushing: bool,
    pub block_height_changes: bool,
}

impl From<SetUnitBlockingOptions> for sys::SetUnitBlockingOptions {
    fn from(options: SetUnitBlockingOptions) -> Self {
        sys::SetUnitBlockingOptions {
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

#[derive(Debug, Clone, Copy, Default)]
pub struct SetUnitLeavesGhostOptions {
    pub leaves_ghost: bool,
    pub leave_dead_ghost: bool,
}

impl From<SetUnitLeavesGhostOptions> for sys::SetUnitLeavesGhostOptions {
    fn from(options: SetUnitLeavesGhostOptions) -> Self {
        sys::SetUnitLeavesGhostOptions {
            leavesGhost: options.leaves_ghost,
            leaveDeadGhost: options.leave_dead_ghost,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SetUnitTargetOptions {
    pub manual_fire: bool,
    pub user_target: bool,
}

impl From<SetUnitTargetOptions> for sys::SetUnitTargetOptions {
    fn from(options: SetUnitTargetOptions) -> Self {
        sys::SetUnitTargetOptions {
            manualFire: options.manual_fire,
            userTarget: options.user_target,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SetUnitUseWeaponsOptions {
    pub force_use_weapons: bool,
    pub allow_use_weapons: bool,
}

impl From<SetUnitUseWeaponsOptions> for sys::SetUnitUseWeaponsOptions {
    fn from(options: SetUnitUseWeaponsOptions) -> Self {
        sys::SetUnitUseWeaponsOptions {
            forceUseWeapons: options.force_use_weapons,
            allowUseWeapons: options.allow_use_weapons,
        }
    }
}

impl<'a> UnitControl<'a> {
    pub fn create_unit(&self, unit_def: sys::DefRef, pos: sys::Float3, facing: i32, team_id: i32, options: CreateUnitOptions) -> Result<i32, Error> {
        unsafe {
            let query = sys::CreateUnitQuery {
                unitDef: unit_def,
                pos,
                facing,
                teamID: team_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::CreateUnitResult>::zeroed();
            let func = self.api.CreateUnit.expect("CreateUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitID
            })
        }
    }

    pub fn destroy_unit(&self, unit_id: i32, options: DestroyUnitOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::DestroyUnitQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::DestroyUnitResult>::zeroed();
            let func = self.api.DestroyUnit.expect("DestroyUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn transfer_unit(&self, unit_id: i32, new_team_id: i32, given: bool, adjust_unit_limit: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::TransferUnitQuery {
                unitID: unit_id,
                newTeamID: new_team_id,
                given,
                adjustUnitLimit: adjust_unit_limit,
            };
            let mut result = MaybeUninit::<sys::TransferUnitResult>::zeroed();
            let func = self.api.TransferUnit.expect("TransferUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn give_order_to_unit(&self, unit_id: i32, cmd_id: i32, params: &[f32], options: u32, timeout: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GiveOrderToUnitQuery {
                unitID: unit_id,
                cmdID: cmd_id,
                params: params.as_ptr(),
                paramCount: params.len() as u32,
                options,
                timeout,
            };
            let mut result = MaybeUninit::<sys::GiveOrderToUnitResult>::zeroed();
            let func = self.api.GiveOrderToUnit.expect("GiveOrderToUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn give_order_to_unit_array(&self, unit_ids: &[i32], cmd_id: i32, params: &[f32], options: u32, timeout: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GiveOrderToUnitArrayQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
                cmdID: cmd_id,
                params: params.as_ptr(),
                paramCount: params.len() as u32,
                options,
                timeout,
            };
            let mut result = MaybeUninit::<sys::GiveOrderToUnitArrayResult>::zeroed();
            let func = self.api.GiveOrderToUnitArray.expect("GiveOrderToUnitArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn give_order_array_to_unit(&self, unit_id: i32, commands: &[sys::NativeCommand]) -> Result<bool, Error> {
        unsafe {
            let query = sys::GiveOrderArrayToUnitQuery {
                unitID: unit_id,
                commands: commands.as_ptr(),
                commandCount: commands.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GiveOrderArrayToUnitResult>::zeroed();
            let func = self.api.GiveOrderArrayToUnit.expect("GiveOrderArrayToUnit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn give_order_array_to_unit_array(&self, unit_ids: &[i32], commands: &[sys::NativeCommand], pairwise: bool) -> Result<i32, Error> {
        unsafe {
            let query = sys::GiveOrderArrayToUnitArrayQuery {
                unitIDs: unit_ids.as_ptr(),
                unitCount: unit_ids.len() as u32,
                commands: commands.as_ptr(),
                commandCount: commands.len() as u32,
                pairwise,
            };
            let mut result = MaybeUninit::<sys::GiveOrderArrayToUnitArrayResult>::zeroed();
            let func = self.api.GiveOrderArrayToUnitArray.expect("GiveOrderArrayToUnitArray function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitsOrdered
            })
        }
    }

    pub fn unit_finish_command(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitFinishCommandQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::UnitFinishCommandResult>::zeroed();
            let func = self.api.UnitFinishCommand.expect("UnitFinishCommand function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_health(&self, unit_id: i32, value: sys::UnitHealthValue) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitHealthQuery {
                unitID: unit_id,
                value,
            };
            let mut result = MaybeUninit::<sys::SetUnitHealthResult>::zeroed();
            let func = self.api.SetUnitHealth.expect("SetUnitHealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_max_health(&self, unit_id: i32, max_health: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitMaxHealthQuery {
                unitID: unit_id,
                maxHealth: max_health,
            };
            let mut result = MaybeUninit::<sys::SetUnitMaxHealthResult>::zeroed();
            let func = self.api.SetUnitMaxHealth.expect("SetUnitMaxHealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_experience(&self, unit_id: i32, experience: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitExperienceQuery {
                unitID: unit_id,
                experience,
            };
            let mut result = MaybeUninit::<sys::SetUnitExperienceResult>::zeroed();
            let func = self.api.SetUnitExperience.expect("SetUnitExperience function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_unit_experience(&self, unit_id: i32, experience: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddUnitExperienceQuery {
                unitID: unit_id,
                experience,
            };
            let mut result = MaybeUninit::<sys::AddUnitExperienceResult>::zeroed();
            let func = self.api.AddUnitExperience.expect("AddUnitExperience function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_neutral(&self, unit_id: i32, neutral: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitNeutralQuery {
                unitID: unit_id,
                neutral,
            };
            let mut result = MaybeUninit::<sys::SetUnitNeutralResult>::zeroed();
            let func = self.api.SetUnitNeutral.expect("SetUnitNeutral function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_resourcing(&self, unit_id: i32, r#type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let r#type_cstr = std::ffi::CString::new(r#type).map_err(|_| Error::invalid_argument("r#type"))?;
            let query = sys::SetUnitResourcingQuery {
                unitID: unit_id,
                type_: r#type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::SetUnitResourcingResult>::zeroed();
            let func = self.api.SetUnitResourcing.expect("SetUnitResourcing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_metal_extraction(&self, unit_id: i32, depth: f32, range: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitMetalExtractionQuery {
                unitID: unit_id,
                depth,
                range,
            };
            let mut result = MaybeUninit::<sys::SetUnitMetalExtractionResult>::zeroed();
            let func = self.api.SetUnitMetalExtraction.expect("SetUnitMetalExtraction function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_position(&self, unit_id: i32, pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPositionQuery {
                unitID: unit_id,
                pos,
            };
            let mut result = MaybeUninit::<sys::SetUnitPositionResult>::zeroed();
            let func = self.api.SetUnitPosition.expect("SetUnitPosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_velocity(&self, unit_id: i32, velocity: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitVelocityQuery {
                unitID: unit_id,
                velocity,
            };
            let mut result = MaybeUninit::<sys::SetUnitVelocityResult>::zeroed();
            let func = self.api.SetUnitVelocity.expect("SetUnitVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_rotation(&self, unit_id: i32, rotation: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitRotationQuery {
                unitID: unit_id,
                rotation,
            };
            let mut result = MaybeUninit::<sys::SetUnitRotationResult>::zeroed();
            let func = self.api.SetUnitRotation.expect("SetUnitRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_physics(&self, unit_id: i32, pos: sys::Float3, velocity: sys::Float3, rotation: sys::Float3, drag: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPhysicsQuery {
                unitID: unit_id,
                pos,
                velocity,
                rotation,
                drag,
            };
            let mut result = MaybeUninit::<sys::SetUnitPhysicsResult>::zeroed();
            let func = self.api.SetUnitPhysics.expect("SetUnitPhysics function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_unit_damage(&self, unit_id: i32, damage: f32, paralyze_time: f32, weapon_def_id: i32, attacker_id: i32, impulse: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddUnitDamageQuery {
                unitID: unit_id,
                damage,
                paralyzeTime: paralyze_time,
                weaponDefID: weapon_def_id,
                attackerID: attacker_id,
                impulse,
            };
            let mut result = MaybeUninit::<sys::AddUnitDamageResult>::zeroed();
            let func = self.api.AddUnitDamage.expect("AddUnitDamage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_unit_impulse(&self, unit_id: i32, impulse: sys::Float3, decay_rate: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddUnitImpulseQuery {
                unitID: unit_id,
                impulse,
                decayRate: decay_rate,
            };
            let mut result = MaybeUninit::<sys::AddUnitImpulseResult>::zeroed();
            let func = self.api.AddUnitImpulse.expect("AddUnitImpulse function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_cloak(&self, unit_id: i32, cloak: sys::NumberOrBool, cloak_arg: sys::NumberOrBool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitCloakQuery {
                unitID: unit_id,
                cloak,
                cloakArg: cloak_arg,
            };
            let mut result = MaybeUninit::<sys::SetUnitCloakResult>::zeroed();
            let func = self.api.SetUnitCloak.expect("SetUnitCloak function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_stealth(&self, unit_id: i32, stealth: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitStealthQuery {
                unitID: unit_id,
                stealth,
            };
            let mut result = MaybeUninit::<sys::SetUnitStealthResult>::zeroed();
            let func = self.api.SetUnitStealth.expect("SetUnitStealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_sonar_stealth(&self, unit_id: i32, sonar_stealth: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitSonarStealthQuery {
                unitID: unit_id,
                sonarStealth: sonar_stealth,
            };
            let mut result = MaybeUninit::<sys::SetUnitSonarStealthResult>::zeroed();
            let func = self.api.SetUnitSonarStealth.expect("SetUnitSonarStealth function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_seismic_signature(&self, unit_id: i32, seismic_signature: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitSeismicSignatureQuery {
                unitID: unit_id,
                seismicSignature: seismic_signature,
            };
            let mut result = MaybeUninit::<sys::SetUnitSeismicSignatureResult>::zeroed();
            let func = self.api.SetUnitSeismicSignature.expect("SetUnitSeismicSignature function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_armored(&self, unit_id: i32, armored_state: bool, armored_multiple: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitArmoredQuery {
                unitID: unit_id,
                armoredState: armored_state,
                armoredMultiple: armored_multiple,
            };
            let mut result = MaybeUninit::<sys::SetUnitArmoredResult>::zeroed();
            let func = self.api.SetUnitArmored.expect("SetUnitArmored function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_blocking(&self, unit_id: i32, options: SetUnitBlockingOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitBlockingQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetUnitBlockingResult>::zeroed();
            let func = self.api.SetUnitBlocking.expect("SetUnitBlocking function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_mass(&self, unit_id: i32, mass: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitMassQuery {
                unitID: unit_id,
                mass,
            };
            let mut result = MaybeUninit::<sys::SetUnitMassResult>::zeroed();
            let func = self.api.SetUnitMass.expect("SetUnitMass function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_leaves_ghost(&self, unit_id: i32, options: SetUnitLeavesGhostOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitLeavesGhostQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetUnitLeavesGhostResult>::zeroed();
            let func = self.api.SetUnitLeavesGhost.expect("SetUnitLeavesGhost function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_always_visible(&self, unit_id: i32, always_visible: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitAlwaysVisibleQuery {
                unitID: unit_id,
                alwaysVisible: always_visible,
            };
            let mut result = MaybeUninit::<sys::SetUnitAlwaysVisibleResult>::zeroed();
            let func = self.api.SetUnitAlwaysVisible.expect("SetUnitAlwaysVisible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_use_air_los(&self, unit_id: i32, use_air_los: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitUseAirLosQuery {
                unitID: unit_id,
                useAirLos: use_air_los,
            };
            let mut result = MaybeUninit::<sys::SetUnitUseAirLosResult>::zeroed();
            let func = self.api.SetUnitUseAirLos.expect("SetUnitUseAirLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_unit_leaves_ghost(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetUnitLeavesGhostQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitLeavesGhostResult>::zeroed();
            let func = self.api.GetUnitLeavesGhost.expect("GetUnitLeavesGhost function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.leavesGhost
            })
        }
    }

    pub fn get_unit_physical_state(&self, unit_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetUnitPhysicalStateQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitPhysicalStateResult>::zeroed();
            let func = self.api.GetUnitPhysicalState.expect("GetUnitPhysicalState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.physicalState
            })
        }
    }

    pub fn get_unit_feature_separation(&self, unit_id: i32, feature_id: i32, ignore_y: bool) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetUnitFeatureSeparationQuery {
                unitID: unit_id,
                featureID: feature_id,
                ignoreY: ignore_y,
            };
            let mut result = MaybeUninit::<sys::GetUnitFeatureSeparationResult>::zeroed();
            let func = self.api.GetUnitFeatureSeparation.expect("GetUnitFeatureSeparation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.distance
            })
        }
    }

    pub fn edit_unit_cmd_desc(&self, unit_id: i32, cmd_desc_index: u32, cmd_desc: &sys::NativeCommandDescription) -> Result<bool, Error> {
        unsafe {
            let query = sys::EditUnitCmdDescQuery {
                unitID: unit_id,
                cmdDescIndex: cmd_desc_index,
                cmdDesc: cmd_desc as *const _,
            };
            let mut result = MaybeUninit::<sys::EditUnitCmdDescResult>::zeroed();
            let func = self.api.EditUnitCmdDesc.expect("EditUnitCmdDesc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn insert_unit_cmd_desc(&self, unit_id: i32, cmd_desc_index: i32, cmd_desc: &sys::NativeCommandDescription) -> Result<bool, Error> {
        unsafe {
            let query = sys::InsertUnitCmdDescQuery {
                unitID: unit_id,
                cmdDescIndex: cmd_desc_index,
                cmdDesc: cmd_desc as *const _,
            };
            let mut result = MaybeUninit::<sys::InsertUnitCmdDescResult>::zeroed();
            let func = self.api.InsertUnitCmdDesc.expect("InsertUnitCmdDesc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn remove_unit_cmd_desc(&self, unit_id: i32, cmd_desc_index: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RemoveUnitCmdDescQuery {
                unitID: unit_id,
                cmdDescIndex: cmd_desc_index,
            };
            let mut result = MaybeUninit::<sys::RemoveUnitCmdDescResult>::zeroed();
            let func = self.api.RemoveUnitCmdDesc.expect("RemoveUnitCmdDesc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_costs(&self, unit_id: i32, costs: sys::UnitCostOverrides) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitCostsQuery {
                unitID: unit_id,
                costs,
            };
            let mut result = MaybeUninit::<sys::SetUnitCostsResult>::zeroed();
            let func = self.api.SetUnitCosts.expect("SetUnitCosts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    #[expect(clippy::too_many_arguments, reason = "NativeInterface preserves the corresponding Lua API arity")]
    pub fn set_unit_build_speed(&self, unit_id: i32, build_speed: f32, repair_speed: f32, reclaim_speed: f32, resurrect_speed: f32, capture_speed: f32, terraform_speed: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitBuildSpeedQuery {
                unitID: unit_id,
                buildSpeed: build_speed,
                repairSpeed: repair_speed,
                reclaimSpeed: reclaim_speed,
                resurrectSpeed: resurrect_speed,
                captureSpeed: capture_speed,
                terraformSpeed: terraform_speed,
            };
            let mut result = MaybeUninit::<sys::SetUnitBuildSpeedResult>::zeroed();
            let func = self.api.SetUnitBuildSpeed.expect("SetUnitBuildSpeed function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_collision_volume_data(&self, unit_id: i32, scales: sys::Float3, offsets: sys::Float3, volume_type: i32, test_type: i32, primary_axis: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitCollisionVolumeDataQuery {
                unitID: unit_id,
                scales,
                offsets,
                volumeType: volume_type,
                testType: test_type,
                primaryAxis: primary_axis,
            };
            let mut result = MaybeUninit::<sys::SetUnitCollisionVolumeDataResult>::zeroed();
            let func = self.api.SetUnitCollisionVolumeData.expect("SetUnitCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_selection_volume_data(&self, unit_id: i32, scales: sys::Float3, offsets: sys::Float3, volume_type: i32, test_type: i32, primary_axis: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitSelectionVolumeDataQuery {
                unitID: unit_id,
                scales,
                offsets,
                volumeType: volume_type,
                testType: test_type,
                primaryAxis: primary_axis,
            };
            let mut result = MaybeUninit::<sys::SetUnitSelectionVolumeDataResult>::zeroed();
            let func = self.api.SetUnitSelectionVolumeData.expect("SetUnitSelectionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    #[expect(clippy::too_many_arguments, reason = "NativeInterface preserves the corresponding Lua API arity")]
    pub fn set_unit_piece_collision_volume_data(&self, unit_id: i32, piece_index: i32, enable: bool, scales: sys::Float3, offsets: sys::Float3, volume_type: i32, primary_axis: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPieceCollisionVolumeDataQuery {
                unitID: unit_id,
                pieceIndex: piece_index,
                enable,
                scales,
                offsets,
                volumeType: volume_type,
                primaryAxis: primary_axis,
            };
            let mut result = MaybeUninit::<sys::SetUnitPieceCollisionVolumeDataResult>::zeroed();
            let func = self.api.SetUnitPieceCollisionVolumeData.expect("SetUnitPieceCollisionVolumeData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_target(&self, unit_id: i32, target: sys::UnitTargetRef, options: SetUnitTargetOptions, weapon_num: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitTargetQuery {
                unitID: unit_id,
                target,
                options: options.into(),
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::SetUnitTargetResult>::zeroed();
            let func = self.api.SetUnitTarget.expect("SetUnitTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_shield_state(&self, unit_id: i32, weapon_num: i32, enabled: bool, power: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitShieldStateQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                enabled,
                power,
            };
            let mut result = MaybeUninit::<sys::SetUnitShieldStateResult>::zeroed();
            let func = self.api.SetUnitShieldState.expect("SetUnitShieldState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_shield_recharge_delay(&self, unit_id: i32, weapon_num: i32, recharge_delay: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitShieldRechargeDelayQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                rechargeDelay: recharge_delay,
            };
            let mut result = MaybeUninit::<sys::SetUnitShieldRechargeDelayResult>::zeroed();
            let func = self.api.SetUnitShieldRechargeDelay.expect("SetUnitShieldRechargeDelay function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_flanking(&self, unit_id: i32, r#type: &str, args: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let r#type_cstr = std::ffi::CString::new(r#type).map_err(|_| Error::invalid_argument("r#type"))?;
            let query = sys::SetUnitFlankingQuery {
                unitID: unit_id,
                type_: r#type_cstr.as_ptr(),
                args,
            };
            let mut result = MaybeUninit::<sys::SetUnitFlankingResult>::zeroed();
            let func = self.api.SetUnitFlanking.expect("SetUnitFlanking function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_mid_and_aim_pos(&self, unit_id: i32, mid_pos: sys::Float3, aim_pos: sys::Float3, set_relative: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitMidAndAimPosQuery {
                unitID: unit_id,
                midPos: mid_pos,
                aimPos: aim_pos,
                setRelative: set_relative,
            };
            let mut result = MaybeUninit::<sys::SetUnitMidAndAimPosResult>::zeroed();
            let func = self.api.SetUnitMidAndAimPos.expect("SetUnitMidAndAimPos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_radius_and_height(&self, unit_id: i32, radius: f32, height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitRadiusAndHeightQuery {
                unitID: unit_id,
                radius,
                height,
            };
            let mut result = MaybeUninit::<sys::SetUnitRadiusAndHeightResult>::zeroed();
            let func = self.api.SetUnitRadiusAndHeight.expect("SetUnitRadiusAndHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_move_goal(&self, unit_id: i32, pos: sys::Float3, radius: f32, speed: f32, raw: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitMoveGoalQuery {
                unitID: unit_id,
                pos,
                radius,
                speed,
                raw,
            };
            let mut result = MaybeUninit::<sys::SetUnitMoveGoalResult>::zeroed();
            let func = self.api.SetUnitMoveGoal.expect("SetUnitMoveGoal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_land_goal(&self, unit_id: i32, pos: sys::Float3, radius_sq: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitLandGoalQuery {
                unitID: unit_id,
                pos,
                radiusSq: radius_sq,
            };
            let mut result = MaybeUninit::<sys::SetUnitLandGoalResult>::zeroed();
            let func = self.api.SetUnitLandGoal.expect("SetUnitLandGoal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn clear_unit_goal(&self, unit_id: i32, cancel_raw: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::ClearUnitGoalQuery {
                unitID: unit_id,
                cancelRaw: cancel_raw,
            };
            let mut result = MaybeUninit::<sys::ClearUnitGoalResult>::zeroed();
            let func = self.api.ClearUnitGoal.expect("ClearUnitGoal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_stockpile(&self, unit_id: i32, stockpile: i32, build_percent: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitStockpileQuery {
                unitID: unit_id,
                stockpile,
                buildPercent: build_percent,
            };
            let mut result = MaybeUninit::<sys::SetUnitStockpileResult>::zeroed();
            let func = self.api.SetUnitStockpile.expect("SetUnitStockpile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_direction(&self, unit_id: i32, front_dir: sys::Float3, right_dir: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitDirectionQuery {
                unitID: unit_id,
                frontDir: front_dir,
                rightDir: right_dir,
            };
            let mut result = MaybeUninit::<sys::SetUnitDirectionResult>::zeroed();
            let func = self.api.SetUnitDirection.expect("SetUnitDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn unit_attach(&self, transporter_id: i32, transportee_id: i32, piece_num: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitAttachQuery {
                transporterID: transporter_id,
                transporteeID: transportee_id,
                pieceNum: piece_num,
            };
            let mut result = MaybeUninit::<sys::UnitAttachResult>::zeroed();
            let func = self.api.UnitAttach.expect("UnitAttach function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn unit_detach(&self, transportee_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitDetachQuery {
                transporteeID: transportee_id,
            };
            let mut result = MaybeUninit::<sys::UnitDetachResult>::zeroed();
            let func = self.api.UnitDetach.expect("UnitDetach function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn unit_detach_from_air(&self, transportee_id: i32, pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitDetachFromAirQuery {
                transporteeID: transportee_id,
                pos,
            };
            let mut result = MaybeUninit::<sys::UnitDetachFromAirResult>::zeroed();
            let func = self.api.UnitDetachFromAir.expect("UnitDetachFromAir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_loading_transport(&self, unit_id: i32, transport_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitLoadingTransportQuery {
                unitID: unit_id,
                transportID: transport_id,
            };
            let mut result = MaybeUninit::<sys::SetUnitLoadingTransportResult>::zeroed();
            let func = self.api.SetUnitLoadingTransport.expect("SetUnitLoadingTransport function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_crashing(&self, unit_id: i32, want_crash: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitCrashingQuery {
                unitID: unit_id,
                wantCrash: want_crash,
            };
            let mut result = MaybeUninit::<sys::SetUnitCrashingResult>::zeroed();
            let func = self.api.SetUnitCrashing.expect("SetUnitCrashing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.stateChanged
            })
        }
    }

    pub fn set_unit_weapon_state(&self, unit_id: i32, weapon_num: i32, key: &str, value: f32) -> Result<bool, Error> {
        unsafe {
            let key_cstr = std::ffi::CString::new(key).map_err(|_| Error::invalid_argument("key"))?;
            let query = sys::SetUnitWeaponStateQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                key: key_cstr.as_ptr(),
                value,
            };
            let mut result = MaybeUninit::<sys::SetUnitWeaponStateResult>::zeroed();
            let func = self.api.SetUnitWeaponState.expect("SetUnitWeaponState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn unit_weapon_fire(&self, unit_id: i32, weapon_num: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitWeaponFireQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::UnitWeaponFireResult>::zeroed();
            let func = self.api.UnitWeaponFire.expect("UnitWeaponFire function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn unit_weapon_hold_fire(&self, unit_id: i32, weapon_num: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::UnitWeaponHoldFireQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
            };
            let mut result = MaybeUninit::<sys::UnitWeaponHoldFireResult>::zeroed();
            let func = self.api.UnitWeaponHoldFire.expect("UnitWeaponHoldFire function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_use_weapons(&self, unit_id: i32, options: SetUnitUseWeaponsOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitUseWeaponsQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetUnitUseWeaponsResult>::zeroed();
            let func = self.api.SetUnitUseWeapons.expect("SetUnitUseWeapons function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_max_range(&self, unit_id: i32, max_range: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitMaxRangeQuery {
                unitID: unit_id,
                maxRange: max_range,
            };
            let mut result = MaybeUninit::<sys::SetUnitMaxRangeResult>::zeroed();
            let func = self.api.SetUnitMaxRange.expect("SetUnitMaxRange function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_physical_state_bit(&self, unit_id: i32, state_bit: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPhysicalStateBitQuery {
                unitID: unit_id,
                stateBit: state_bit,
            };
            let mut result = MaybeUninit::<sys::SetUnitPhysicalStateBitResult>::zeroed();
            let func = self.api.SetUnitPhysicalStateBit.expect("SetUnitPhysicalStateBit function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_pos_error_params(&self, unit_id: i32, pos_error_vector: sys::Float3, pos_error_delta: sys::Float3, next_pos_error_update: i32, ally_team_id: i32, set_pos_error_bit: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPosErrorParamsQuery {
                unitID: unit_id,
                posErrorVector: pos_error_vector,
                posErrorDelta: pos_error_delta,
                nextPosErrorUpdate: next_pos_error_update,
                allyTeamID: ally_team_id,
                setPosErrorBit: set_pos_error_bit,
            };
            let mut result = MaybeUninit::<sys::SetUnitPosErrorParamsResult>::zeroed();
            let func = self.api.SetUnitPosErrorParams.expect("SetUnitPosErrorParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_weapon_damages(&self, unit_id: i32, weapon_num: i32, damage_key: &str, damage_value: f32) -> Result<bool, Error> {
        unsafe {
            let damage_key_cstr = std::ffi::CString::new(damage_key).map_err(|_| Error::invalid_argument("damage_key"))?;
            let query = sys::SetUnitWeaponDamagesQuery {
                unitID: unit_id,
                weaponNum: weapon_num,
                damageKey: damage_key_cstr.as_ptr(),
                damageValue: damage_value,
            };
            let mut result = MaybeUninit::<sys::SetUnitWeaponDamagesResult>::zeroed();
            let func = self.api.SetUnitWeaponDamages.expect("SetUnitWeaponDamages function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn force_unit_collision_update(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::ForceUnitCollisionUpdateQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::ForceUnitCollisionUpdateResult>::zeroed();
            let func = self.api.ForceUnitCollisionUpdate.expect("ForceUnitCollisionUpdate function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_heading(&self, unit_id: i32, heading: i32, use_smoothing: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitHeadingQuery {
                unitID: unit_id,
                heading,
                useSmoothing: use_smoothing,
            };
            let mut result = MaybeUninit::<sys::SetUnitHeadingResult>::zeroed();
            let func = self.api.SetUnitHeading.expect("SetUnitHeading function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_heading_and_up_dir(&self, unit_id: i32, heading: i32, up_dir: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitHeadingAndUpDirQuery {
                unitID: unit_id,
                heading,
                upDir: up_dir,
            };
            let mut result = MaybeUninit::<sys::SetUnitHeadingAndUpDirResult>::zeroed();
            let func = self.api.SetUnitHeadingAndUpDir.expect("SetUnitHeadingAndUpDir function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_object_decal(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddObjectDecalQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::AddObjectDecalResult>::zeroed();
            let func = self.api.AddObjectDecal.expect("AddObjectDecal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn remove_object_decal(&self, unit_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::RemoveObjectDecalQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::RemoveObjectDecalResult>::zeroed();
            let func = self.api.RemoveObjectDecal.expect("RemoveObjectDecal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_buildee_radius(&self, unit_id: i32, radius: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitBuildeeRadiusQuery {
                unitID: unit_id,
                radius,
            };
            let mut result = MaybeUninit::<sys::SetUnitBuildeeRadiusResult>::zeroed();
            let func = self.api.SetUnitBuildeeRadius.expect("SetUnitBuildeeRadius function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_sensor_radius(&self, unit_id: i32, sensor_type: &str, radius: i32) -> Result<i32, Error> {
        unsafe {
            let sensor_type_cstr = std::ffi::CString::new(sensor_type).map_err(|_| Error::invalid_argument("sensor_type"))?;
            let query = sys::SetUnitSensorRadiusQuery {
                unitID: unit_id,
                sensorType: sensor_type_cstr.as_ptr(),
                radius,
            };
            let mut result = MaybeUninit::<sys::SetUnitSensorRadiusResult>::zeroed();
            let func = self.api.SetUnitSensorRadius.expect("SetUnitSensorRadius function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.newRadius
            })
        }
    }

    pub fn set_unit_harvest_storage(&self, unit_id: i32, stored_metal: f32, max_stored_metal: f32, stored_energy: f32, max_stored_energy: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitHarvestStorageQuery {
                unitID: unit_id,
                storedMetal: stored_metal,
                maxStoredMetal: max_stored_metal,
                storedEnergy: stored_energy,
                maxStoredEnergy: max_stored_energy,
            };
            let mut result = MaybeUninit::<sys::SetUnitHarvestStorageResult>::zeroed();
            let func = self.api.SetUnitHarvestStorage.expect("SetUnitHarvestStorage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_build_params(&self, unit_id: i32, param_name: &str, value: sys::NumberOrBool) -> Result<bool, Error> {
        unsafe {
            let param_name_cstr = std::ffi::CString::new(param_name).map_err(|_| Error::invalid_argument("param_name"))?;
            let query = sys::SetUnitBuildParamsQuery {
                unitID: unit_id,
                paramName: param_name_cstr.as_ptr(),
                value,
            };
            let mut result = MaybeUninit::<sys::SetUnitBuildParamsResult>::zeroed();
            let func = self.api.SetUnitBuildParams.expect("SetUnitBuildParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_los_mask(&self, unit_id: i32, ally_team_id: i32, los_mask: u8) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitLosMaskQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
                losMask: los_mask,
            };
            let mut result = MaybeUninit::<sys::SetUnitLosMaskResult>::zeroed();
            let func = self.api.SetUnitLosMask.expect("SetUnitLosMask function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_los_state(&self, unit_id: i32, ally_team_id: i32, los_state: u8) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitLosStateQuery {
                unitID: unit_id,
                allyTeamID: ally_team_id,
                losState: los_state,
            };
            let mut result = MaybeUninit::<sys::SetUnitLosStateResult>::zeroed();
            let func = self.api.SetUnitLosState.expect("SetUnitLosState function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_storage(&self, unit_id: i32, resource: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_cstr = std::ffi::CString::new(resource).map_err(|_| Error::invalid_argument("resource"))?;
            let query = sys::SetUnitStorageQuery {
                unitID: unit_id,
                resource: resource_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::SetUnitStorageResult>::zeroed();
            let func = self.api.SetUnitStorage.expect("SetUnitStorage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_tooltip(&self, unit_id: i32, tooltip: &str) -> Result<bool, Error> {
        unsafe {
            let tooltip_cstr = std::ffi::CString::new(tooltip).map_err(|_| Error::invalid_argument("tooltip"))?;
            let query = sys::SetUnitTooltipQuery {
                unitID: unit_id,
                tooltip: tooltip_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetUnitTooltipResult>::zeroed();
            let func = self.api.SetUnitTooltip.expect("SetUnitTooltip function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_factory_bugger_off(&self, unit_id: i32, options: SetFactoryBuggerOffOptions) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetFactoryBuggerOffQuery {
                unitID: unit_id,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::SetFactoryBuggerOffResult>::zeroed();
            let func = self.api.SetFactoryBuggerOff.expect("SetFactoryBuggerOff function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.perform
            })
        }
    }

    pub fn bugger_off(&self, pos: sys::Float3, radius: f32, team_id: i32, options: BuggerOffOptions, exclude_unit_def_ids: &[i32]) -> Result<bool, Error> {
        unsafe {
            let query = sys::BuggerOffQuery {
                pos,
                radius,
                teamID: team_id,
                options: options.into(),
                excludeUnitDefIDs: exclude_unit_def_ids.as_ptr(),
                excludeUnitDefCount: exclude_unit_def_ids.len() as u32,
            };
            let mut result = MaybeUninit::<sys::BuggerOffResult>::zeroed();
            let func = self.api.BuggerOff.expect("BuggerOff function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_unit_seismic_ping(&self, unit_id: i32, ping_size: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::AddUnitSeismicPingQuery {
                unitID: unit_id,
                pingSize: ping_size,
            };
            let mut result = MaybeUninit::<sys::AddUnitSeismicPingResult>::zeroed();
            let func = self.api.AddUnitSeismicPing.expect("AddUnitSeismicPing function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn add_unit_resource(&self, unit_id: i32, resource_type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::AddUnitResourceQuery {
                unitID: unit_id,
                resourceType: resource_type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::AddUnitResourceResult>::zeroed();
            let func = self.api.AddUnitResource.expect("AddUnitResource function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn use_unit_resource(&self, unit_id: i32, resource_type: &str, amount: f32) -> Result<bool, Error> {
        unsafe {
            let resource_type_cstr = std::ffi::CString::new(resource_type).map_err(|_| Error::invalid_argument("resource_type"))?;
            let query = sys::UseUnitResourceQuery {
                unitID: unit_id,
                resourceType: resource_type_cstr.as_ptr(),
                amount,
            };
            let mut result = MaybeUninit::<sys::UseUnitResourceResult>::zeroed();
            let func = self.api.UseUnitResource.expect("UseUnitResource function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_piece_visible(&self, unit_id: i32, piece_index: i32, visible: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPieceVisibleQuery {
                unitID: unit_id,
                pieceIndex: piece_index,
                visible,
            };
            let mut result = MaybeUninit::<sys::SetUnitPieceVisibleResult>::zeroed();
            let func = self.api.SetUnitPieceVisible.expect("SetUnitPieceVisible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_piece_parent(&self, unit_id: i32, child_piece_index: i32, parent_piece_index: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPieceParentQuery {
                unitID: unit_id,
                childPieceIndex: child_piece_index,
                parentPieceIndex: parent_piece_index,
            };
            let mut result = MaybeUninit::<sys::SetUnitPieceParentResult>::zeroed();
            let func = self.api.SetUnitPieceParent.expect("SetUnitPieceParent function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_unit_piece_matrix(&self, unit_id: i32, piece_index: i32, matrix: [f32; 16]) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitPieceMatrixQuery {
                unitID: unit_id,
                pieceIndex: piece_index,
                matrix,
            };
            let mut result = MaybeUninit::<sys::SetUnitPieceMatrixResult>::zeroed();
            let func = self.api.SetUnitPieceMatrix.expect("SetUnitPieceMatrix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.blockScriptAnims
            })
        }
    }

    pub fn set_unit_nano_pieces(&self, unit_id: i32, piece_indices: &[i32]) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetUnitNanoPiecesQuery {
                unitID: unit_id,
                pieceIndices: piece_indices.as_ptr(),
                pieceCount: piece_indices.len() as u32,
            };
            let mut result = MaybeUninit::<sys::SetUnitNanoPiecesResult>::zeroed();
            let func = self.api.SetUnitNanoPieces.expect("SetUnitNanoPieces function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

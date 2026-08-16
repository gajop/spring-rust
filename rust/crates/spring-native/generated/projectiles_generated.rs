#[derive(Debug, Clone, Copy, Default)]
pub struct GetAllProjectilesOptions {
    pub exclude_weapon_projectiles: bool,
    pub exclude_piece_projectiles: bool,
}

impl From<GetAllProjectilesOptions> for sys::GetAllProjectilesOptions {
    fn from(options: GetAllProjectilesOptions) -> Self {
        sys::GetAllProjectilesOptions {
            excludeWeaponProjectiles: options.exclude_weapon_projectiles,
            excludePieceProjectiles: options.exclude_piece_projectiles,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetProjectilesInRectangleOptions {
    pub exclude_weapon_projectiles: bool,
    pub exclude_piece_projectiles: bool,
}

impl From<GetProjectilesInRectangleOptions> for sys::GetProjectilesInRectangleOptions {
    fn from(options: GetProjectilesInRectangleOptions) -> Self {
        sys::GetProjectilesInRectangleOptions {
            excludeWeaponProjectiles: options.exclude_weapon_projectiles,
            excludePieceProjectiles: options.exclude_piece_projectiles,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetProjectilesInSphereOptions {
    pub exclude_weapon_projectiles: bool,
    pub exclude_piece_projectiles: bool,
}

impl From<GetProjectilesInSphereOptions> for sys::GetProjectilesInSphereOptions {
    fn from(options: GetProjectilesInSphereOptions) -> Self {
        sys::GetProjectilesInSphereOptions {
            excludeWeaponProjectiles: options.exclude_weapon_projectiles,
            excludePieceProjectiles: options.exclude_piece_projectiles,
        }
    }
}

impl<'a> Projectiles<'a> {
    pub fn get_all_projectiles(&self, options: GetAllProjectilesOptions) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetAllProjectilesQuery {
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetAllProjectilesResult>::zeroed();
            let func = self.api.GetAllProjectiles.expect("GetAllProjectiles function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.projectiles.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.projectiles as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_projectiles_in_rectangle(&self, min_x: f32, min_z: f32, max_x: f32, max_z: f32, options: GetProjectilesInRectangleOptions) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetProjectilesInRectangleQuery {
                minX: min_x,
                minZ: min_z,
                maxX: max_x,
                maxZ: max_z,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetProjectilesInRectangleResult>::zeroed();
            let func = self.api.GetProjectilesInRectangle.expect("GetProjectilesInRectangle function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.projectiles.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.projectiles as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_projectiles_in_sphere(&self, center: sys::Float3, radius: f32, options: GetProjectilesInSphereOptions) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetProjectilesInSphereQuery {
                center: center,
                radius: radius,
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetProjectilesInSphereResult>::zeroed();
            let func = self.api.GetProjectilesInSphere.expect("GetProjectilesInSphere function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.projectiles.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.projectiles as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_projectile_position(&self, projectile_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetProjectilePositionQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectilePositionResult>::zeroed();
            let func = self.api.GetProjectilePosition.expect("GetProjectilePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.position
            })
        }
    }

    pub fn get_projectile_direction(&self, projectile_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetProjectileDirectionQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileDirectionResult>::zeroed();
            let func = self.api.GetProjectileDirection.expect("GetProjectileDirection function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.direction
            })
        }
    }

    pub fn get_projectile_velocity(&self, projectile_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetProjectileVelocityQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileVelocityResult>::zeroed();
            let func = self.api.GetProjectileVelocity.expect("GetProjectileVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.velocity
            })
        }
    }

    pub fn get_projectile_gravity(&self, projectile_id: i32) -> Result<sys::Float3, Error> {
        unsafe {
            let query = sys::GetProjectileGravityQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileGravityResult>::zeroed();
            let func = self.api.GetProjectileGravity.expect("GetProjectileGravity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.gravity
            })
        }
    }

    pub fn get_piece_projectile_params(&self, projectile_id: i32) -> Result<(sys::PieceProjectileParams, bool), Error> {
        unsafe {
            let query = sys::GetPieceProjectileParamsQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetPieceProjectileParamsResult>::zeroed();
            let func = self.api.GetPieceProjectileParams.expect("GetPieceProjectileParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.params,
                result.isPieceProjectile,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_projectile_target(&self, projectile_id: i32) -> Result<sys::ProjectileTarget, Error> {
        unsafe {
            let query = sys::GetProjectileTargetQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileTargetResult>::zeroed();
            let func = self.api.GetProjectileTarget.expect("GetProjectileTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.target
            })
        }
    }

    pub fn get_projectile_is_intercepted(&self, projectile_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GetProjectileIsInterceptedQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileIsInterceptedResult>::zeroed();
            let func = self.api.GetProjectileIsIntercepted.expect("GetProjectileIsIntercepted function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.isIntercepted
            })
        }
    }

    pub fn get_projectile_time_to_live(&self, projectile_id: i32) -> Result<f32, Error> {
        unsafe {
            let query = sys::GetProjectileTimeToLiveQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileTimeToLiveResult>::zeroed();
            let func = self.api.GetProjectileTimeToLive.expect("GetProjectileTimeToLive function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.ttl
            })
        }
    }

    pub fn get_projectile_owner_id(&self, projectile_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetProjectileOwnerIDQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileOwnerIDResult>::zeroed();
            let func = self.api.GetProjectileOwnerID.expect("GetProjectileOwnerID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.ownerID
            })
        }
    }

    pub fn get_projectile_team_id(&self, projectile_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetProjectileTeamIDQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileTeamIDResult>::zeroed();
            let func = self.api.GetProjectileTeamID.expect("GetProjectileTeamID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.teamID
            })
        }
    }

    pub fn get_projectile_ally_team_id(&self, projectile_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetProjectileAllyTeamIDQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileAllyTeamIDResult>::zeroed();
            let func = self.api.GetProjectileAllyTeamID.expect("GetProjectileAllyTeamID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.allyTeamID
            })
        }
    }

    pub fn get_projectile_type(&self, projectile_id: i32) -> Result<(bool, bool), Error> {
        unsafe {
            let query = sys::GetProjectileTypeQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileTypeResult>::zeroed();
            let func = self.api.GetProjectileType.expect("GetProjectileType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.weapon,
                result.piece,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_projectile_def_id(&self, projectile_id: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GetProjectileDefIDQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::GetProjectileDefIDResult>::zeroed();
            let func = self.api.GetProjectileDefID.expect("GetProjectileDefID function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.defID
            })
        }
    }

    pub fn get_projectile_damages(&self, projectile_id: i32, tag: &str) -> Result<sys::ProjectileDamages, Error> {
        unsafe {
            let tag_cstr = std::ffi::CString::new(tag).map_err(|_| Error::invalid_argument("tag"))?;
            let query = sys::GetProjectileDamagesQuery {
                projectileID: projectile_id,
                tag: tag_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::GetProjectileDamagesResult>::zeroed();
            let func = self.api.GetProjectileDamages.expect("GetProjectileDamages function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.damages
            })
        }
    }

}

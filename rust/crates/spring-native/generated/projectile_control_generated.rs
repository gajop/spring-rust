impl<'a> ProjectileControl<'a> {
    pub fn spawn_projectile(&self, weapon_def_id: i32, projectile_params: sys::NativeProjectileParams) -> Result<i32, Error> {
        unsafe {
            let query = sys::SpawnProjectileQuery {
                weaponDefID: weapon_def_id,
                projectileParams: projectile_params,
            };
            let mut result = MaybeUninit::<sys::SpawnProjectileResult>::zeroed();
            let func = self.api.SpawnProjectile.expect("SpawnProjectile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.projectileID
            })
        }
    }

    pub fn delete_projectile(&self, projectile_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::DeleteProjectileQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::DeleteProjectileResult>::zeroed();
            let func = self.api.DeleteProjectile.expect("DeleteProjectile function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_position(&self, projectile_id: i32, pos: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectilePositionQuery {
                projectileID: projectile_id,
                pos,
            };
            let mut result = MaybeUninit::<sys::SetProjectilePositionResult>::zeroed();
            let func = self.api.SetProjectilePosition.expect("SetProjectilePosition function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_velocity(&self, projectile_id: i32, velocity: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileVelocityQuery {
                projectileID: projectile_id,
                velocity,
            };
            let mut result = MaybeUninit::<sys::SetProjectileVelocityResult>::zeroed();
            let func = self.api.SetProjectileVelocity.expect("SetProjectileVelocity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_gravity(&self, projectile_id: i32, gravity: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileGravityQuery {
                projectileID: projectile_id,
                gravity,
            };
            let mut result = MaybeUninit::<sys::SetProjectileGravityResult>::zeroed();
            let func = self.api.SetProjectileGravity.expect("SetProjectileGravity function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_target(&self, projectile_id: i32, target: sys::ProjectileTargetRef) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileTargetQuery {
                projectileID: projectile_id,
                target,
            };
            let mut result = MaybeUninit::<sys::SetProjectileTargetResult>::zeroed();
            let func = self.api.SetProjectileTarget.expect("SetProjectileTarget function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_damages(&self, projectile_id: i32, unused: i32, damage_key: &str, damage_value: f32) -> Result<bool, Error> {
        unsafe {
            let damage_key_cstr = std::ffi::CString::new(damage_key).map_err(|_| Error::invalid_argument("damage_key"))?;
            let query = sys::SetProjectileDamagesQuery {
                projectileID: projectile_id,
                unused,
                damageKey: damage_key_cstr.as_ptr(),
                damageValue: damage_value,
            };
            let mut result = MaybeUninit::<sys::SetProjectileDamagesResult>::zeroed();
            let func = self.api.SetProjectileDamages.expect("SetProjectileDamages function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_time_to_live(&self, projectile_id: i32, time_to_live: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileTimeToLiveQuery {
                projectileID: projectile_id,
                timeToLive: time_to_live,
            };
            let mut result = MaybeUninit::<sys::SetProjectileTimeToLiveResult>::zeroed();
            let func = self.api.SetProjectileTimeToLive.expect("SetProjectileTimeToLive function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_is_intercepted(&self, projectile_id: i32, intercepted: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileIsInterceptedQuery {
                projectileID: projectile_id,
                intercepted,
            };
            let mut result = MaybeUninit::<sys::SetProjectileIsInterceptedResult>::zeroed();
            let func = self.api.SetProjectileIsIntercepted.expect("SetProjectileIsIntercepted function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_collision(&self, projectile_id: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileCollisionQuery {
                projectileID: projectile_id,
            };
            let mut result = MaybeUninit::<sys::SetProjectileCollisionResult>::zeroed();
            let func = self.api.SetProjectileCollision.expect("SetProjectileCollision function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_ceg(&self, projectile_id: i32, ceg_name: &str) -> Result<i32, Error> {
        unsafe {
            let ceg_name_cstr = std::ffi::CString::new(ceg_name).map_err(|_| Error::invalid_argument("ceg_name"))?;
            let query = sys::SetProjectileCEGQuery {
                projectileID: projectile_id,
                cegName: ceg_name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SetProjectileCEGResult>::zeroed();
            let func = self.api.SetProjectileCEG.expect("SetProjectileCEG function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.cegID
            })
        }
    }

    pub fn set_projectile_always_visible(&self, projectile_id: i32, always_visible: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileAlwaysVisibleQuery {
                projectileID: projectile_id,
                alwaysVisible: always_visible,
            };
            let mut result = MaybeUninit::<sys::SetProjectileAlwaysVisibleResult>::zeroed();
            let func = self.api.SetProjectileAlwaysVisible.expect("SetProjectileAlwaysVisible function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_use_air_los(&self, projectile_id: i32, use_air_los: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileUseAirLosQuery {
                projectileID: projectile_id,
                useAirLos: use_air_los,
            };
            let mut result = MaybeUninit::<sys::SetProjectileUseAirLosResult>::zeroed();
            let func = self.api.SetProjectileUseAirLos.expect("SetProjectileUseAirLos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_move_control(&self, projectile_id: i32, enable: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileMoveControlQuery {
                projectileID: projectile_id,
                enable,
            };
            let mut result = MaybeUninit::<sys::SetProjectileMoveControlResult>::zeroed();
            let func = self.api.SetProjectileMoveControl.expect("SetProjectileMoveControl function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_projectile_ignore_tracking_error(&self, projectile_id: i32, ignore: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetProjectileIgnoreTrackingErrorQuery {
                projectileID: projectile_id,
                ignore,
            };
            let mut result = MaybeUninit::<sys::SetProjectileIgnoreTrackingErrorResult>::zeroed();
            let func = self.api.SetProjectileIgnoreTrackingError.expect("SetProjectileIgnoreTrackingError function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_piece_projectile_params(&self, projectile_id: i32, expl_flags: i32, spin_angle: f32, spin_speed: f32, spin_vec: sys::Float3) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetPieceProjectileParamsQuery {
                projectileID: projectile_id,
                explFlags: expl_flags,
                spinAngle: spin_angle,
                spinSpeed: spin_speed,
                spinVec: spin_vec,
            };
            let mut result = MaybeUninit::<sys::SetPieceProjectileParamsResult>::zeroed();
            let func = self.api.SetPieceProjectileParams.expect("SetPieceProjectileParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

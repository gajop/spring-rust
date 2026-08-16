impl<'a> EffectsControl<'a> {
    pub fn spawn_explosion(&self, pos: sys::Float3, dir: sys::Float3, explosion_params: sys::NativeExplosionParams) -> Result<bool, Error> {
        unsafe {
            let query = sys::SpawnExplosionQuery {
                pos: pos,
                dir: dir,
                explosionParams: explosion_params,
            };
            let mut result = MaybeUninit::<sys::SpawnExplosionResult>::zeroed();
            let func = self.api.SpawnExplosion.expect("SpawnExplosion function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn spawn_ceg(&self, ceg: sys::DefRef, pos: sys::Float3, dir: sys::Float3, radius: f32, damage: f32, dmg_mod: f32) -> Result<(bool, i32), Error> {
        unsafe {
            let query = sys::SpawnCEGQuery {
                ceg: ceg,
                pos: pos,
                dir: dir,
                radius: radius,
                damage: damage,
                dmgMod: dmg_mod,
            };
            let mut result = MaybeUninit::<sys::SpawnCEGResult>::zeroed();
            let func = self.api.SpawnCEG.expect("SpawnCEG function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.success,
                result.cegID,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn spawn_sfx(&self, unit_id: i32, sfx_id: i32, pos: sys::Float3, dir: sys::Float3, radius: f32, damage: f32, absolute: bool) -> Result<bool, Error> {
        unsafe {
            let query = sys::SpawnSFXQuery {
                unitID: unit_id,
                sfxID: sfx_id,
                pos: pos,
                dir: dir,
                radius: radius,
                damage: damage,
                absolute: absolute,
            };
            let mut result = MaybeUninit::<sys::SpawnSFXResult>::zeroed();
            let func = self.api.SpawnSFX.expect("SpawnSFX function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

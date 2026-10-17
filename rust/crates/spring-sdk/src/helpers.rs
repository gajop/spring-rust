/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use crate::{
    ApiError, ErrorCode, Result,
    capabilities::{Game, Local, Synced},
};

impl<E: Local> crate::unsynced_random::UnsyncedRandom<'_, E> {
    // Preserve the former random::unsynced stream: seed once from the engine,
    // then draw locally so rendering cannot change this module's sequence.
    fn with_local_rng<R>(&self, call: impl FnOnce(&mut crate::pcg::Pcg32) -> R) -> Result<R> {
        self._engine.check()?;
        let mut rng = match self._engine.owner.unsynced_rng.get() {
            Some(rng) => rng,
            None => crate::pcg::Pcg32::seeded(self.next_int(i32::MIN, i32::MAX)?),
        };
        let result = call(&mut rng);
        self._engine.owner.unsynced_rng.set(Some(rng));
        self._engine.finish(Ok(result))
    }

    /// Draw from this module's own sequence, in `[0, 1)`.
    /// `next_float` draws directly from the engine's shared unsynced RNG.
    pub fn float(&self) -> Result<f32> {
        self.with_local_rng(crate::pcg::Pcg32::float)
    }

    /// Draw from this module's own sequence, in `[lower, upper]`.
    pub fn int(&self, lower: i32, upper: i32) -> Result<i32> {
        if lower > upper {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        self.with_local_rng(|rng| rng.int(lower, upper))
    }

    pub fn up_to(&self, upper: i32) -> Result<i32> {
        self.int(1, upper)
    }

    pub fn range(&self, lower: f32, upper: f32) -> Result<f32> {
        Ok(lower + (upper - lower) * self.float()?)
    }

    /// Restart this module's sequence without changing the engine's RNG.
    pub fn seed(&self, seed: i32) -> Result<()> {
        self._engine.check()?;
        self._engine
            .owner
            .unsynced_rng
            .set(Some(crate::pcg::Pcg32::seeded(seed)));
        Ok(())
    }
}

impl<E: Game> crate::game::Game<'_, E> {
    pub fn current_frame(&self) -> Result<i32> {
        let frame = self.get_game_frame()?;
        Ok(((frame.high16 << 16) | (frame.low16 & 0xffff)) as i32)
    }
    pub fn map_size_elmos(&self) -> Result<(i32, i32)> {
        let info = self.get_game_map_info()?;
        Ok((info.map_size_x, info.map_size_z))
    }
}

impl<E: Game> crate::terrain::Terrain<'_, E> {
    pub fn ground_heights(&self, positions: &[[f32; 2]]) -> Result<Vec<f32>> {
        self.get_ground_heights(positions.as_flattened())
    }
}

fn numeric_param(value: crate::rules_params::RulesParamValue) -> Result<f32> {
    if value.type_ == crate::rules_params::RulesParamType::RulesparamTypeFloat {
        Ok(value.float_value)
    } else {
        Err(ApiError::new(ErrorCode::InvalidArgument as i32))
    }
}
impl<E: Game> crate::rules_params::RulesParams<'_, E> {
    pub fn get_unit_rules_param_f32(&self, unit: i32, name: &str) -> Result<f32> {
        numeric_param(self.get_unit_rules_param(unit, name)?)
    }
    pub fn get_game_rules_param_f32(&self, name: &str) -> Result<f32> {
        numeric_param(self.get_game_rules_param(name)?)
    }
}
impl<E: Synced> crate::rules_params::RulesParams<'_, E> {
    pub fn set_unit_rules_param_f32(
        &self,
        unit: i32,
        name: &str,
        value: f32,
        los: i32,
    ) -> Result<bool> {
        self.set_unit_rules_param(
            unit,
            name,
            &crate::rules_params::RulesParamValue {
                type_: crate::rules_params::RulesParamType::RulesparamTypeFloat,
                bool_value: false,
                float_value: value,
                string_value: String::new(),
            },
            los,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hit {
    Unit { id: i32, position: crate::Float3 },
    Feature { id: i32, position: crate::Float3 },
    Ground(crate::Float3),
    Sky(crate::Float3),
}
impl<E: Local> crate::camera::Camera<'_, E> {
    pub fn trace_screen(
        &self,
        x: f32,
        y: f32,
        options: crate::camera::TraceScreenRayOptions,
    ) -> Result<Option<Hit>> {
        let value = self.trace_screen_ray(x, y, options)?;
        Ok(match value.hit_type {
            1 => Some(Hit::Unit {
                id: value.hit_id,
                position: value.hit_pos,
            }),
            2 => Some(Hit::Feature {
                id: value.hit_id,
                position: value.hit_pos,
            }),
            3 => Some(Hit::Ground(value.hit_pos)),
            4 => Some(Hit::Sky(value.hit_pos)),
            _ => None,
        })
    }
    pub fn ground_at(&self, x: f32, y: f32) -> Result<Option<crate::Float3>> {
        Ok(
            match self.trace_screen(
                x,
                y,
                crate::camera::TraceScreenRayOptions {
                    only_coords: true,
                    ..Default::default()
                },
            )? {
                Some(Hit::Ground(position)) => Some(position),
                _ => None,
            },
        )
    }
    pub fn unit_at(&self, x: f32, y: f32) -> Result<Option<i32>> {
        Ok(
            match self.trace_screen(x, y, crate::camera::TraceScreenRayOptions::default())? {
                Some(Hit::Unit { id, .. }) => Some(id),
                _ => None,
            },
        )
    }
}

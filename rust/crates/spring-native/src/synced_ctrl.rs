
use std::mem::MaybeUninit;

use crate::{error::Error, sys};

/// SyncedCtrl is a composite API that groups multiple control sub-APIs.
/// Access each sub-API through the corresponding method:
/// - `team()` - Team and ally team control
/// - `unit()` - Unit creation, destruction, and modification
/// - `feature()` - Feature creation and modification
/// - `terrain()` - Terrain height and type modification
/// - `projectile()` - Projectile spawning and control
pub struct SyncedCtrl<'a> {
    // Cache unwrapped sub-API pointers - guaranteed to be initialized by the engine
    team_api: &'a sys::TeamControlApi,
    unit_api: &'a sys::UnitControlApi,
    feature_api: &'a sys::FeatureControlApi,
    terrain_api: &'a sys::TerrainControlApi,
    projectile_api: &'a sys::ProjectileControlApi,
}

impl<'a> SyncedCtrl<'a> {
    pub(crate) fn new(api: &'a sys::SyncedCtrlApi) -> Self {
        // Unwrap all sub-API pointers at initialization - they are guaranteed to be set by the engine
        unsafe {
            Self {
                team_api: api.team.as_ref().expect("team API must be initialized"),
                unit_api: api.unit.as_ref().expect("unit API must be initialized"),
                feature_api: api.feature.as_ref().expect("feature API must be initialized"),
                terrain_api: api.terrain.as_ref().expect("terrain API must be initialized"),
                projectile_api: api.projectile.as_ref().expect("projectile API must be initialized"),
            }
        }
    }

    /// Team and ally team control (alliances, resources, start boxes)
    pub fn team(&self) -> TeamControl<'_> {
        TeamControl::new(self.team_api)
    }

    /// Unit creation, destruction, orders, and property modification
    pub fn unit(&self) -> UnitControl<'_> {
        UnitControl::new(self.unit_api)
    }

    /// Feature creation, destruction, and property modification
    pub fn feature(&self) -> FeatureControl<'_> {
        FeatureControl::new(self.feature_api)
    }

    /// Terrain height map and smooth mesh modification
    pub fn terrain(&self) -> TerrainControl<'_> {
        TerrainControl::new(self.terrain_api)
    }

    /// Projectile spawning and modification
    pub fn projectile(&self) -> ProjectileControl<'_> {
        ProjectileControl::new(self.projectile_api)
    }
}

// Sub-API wrappers
pub struct TeamControl<'a> {
    api: &'a sys::TeamControlApi,
}

impl<'a> TeamControl<'a> {
    pub(crate) fn new(api: &'a sys::TeamControlApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/team_control_generated.rs"));

pub struct UnitControl<'a> {
    api: &'a sys::UnitControlApi,
}

impl<'a> UnitControl<'a> {
    pub(crate) fn new(api: &'a sys::UnitControlApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/unit_control_generated.rs"));

pub struct FeatureControl<'a> {
    api: &'a sys::FeatureControlApi,
}

impl<'a> FeatureControl<'a> {
    pub(crate) fn new(api: &'a sys::FeatureControlApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/feature_control_generated.rs"));

pub struct TerrainControl<'a> {
    api: &'a sys::TerrainControlApi,
}

impl<'a> TerrainControl<'a> {
    pub(crate) fn new(api: &'a sys::TerrainControlApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/terrain_control_generated.rs"));

pub struct ProjectileControl<'a> {
    api: &'a sys::ProjectileControlApi,
}

impl<'a> ProjectileControl<'a> {
    pub(crate) fn new(api: &'a sys::ProjectileControlApi) -> Self {
        Self { api }
    }
}

include!(concat!(env!("OUT_DIR"), "/projectile_control_generated.rs"));

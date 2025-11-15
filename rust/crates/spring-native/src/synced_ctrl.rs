
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
    api: &'a sys::SyncedCtrlApi,
}

impl<'a> SyncedCtrl<'a> {
    pub(crate) fn new(api: &'a sys::SyncedCtrlApi) -> Self {
        Self { api }
    }

    /// Team and ally team control (alliances, resources, start boxes)
    pub fn team(&self) -> Option<TeamControl<'_>> {
        unsafe { self.api.team.as_ref().map(TeamControl::new) }
    }

    /// Unit creation, destruction, orders, and property modification
    pub fn unit(&self) -> Option<UnitControl<'_>> {
        unsafe { self.api.unit.as_ref().map(UnitControl::new) }
    }

    /// Feature creation, destruction, and property modification
    pub fn feature(&self) -> Option<FeatureControl<'_>> {
        unsafe { self.api.feature.as_ref().map(FeatureControl::new) }
    }

    /// Terrain height map and smooth mesh modification
    pub fn terrain(&self) -> Option<TerrainControl<'_>> {
        unsafe { self.api.terrain.as_ref().map(TerrainControl::new) }
    }

    /// Projectile spawning and modification
    pub fn projectile(&self) -> Option<ProjectileControl<'_>> {
        unsafe { self.api.projectile.as_ref().map(ProjectileControl::new) }
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

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
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

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
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

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
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

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
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

    #[inline(always)]
    fn get_fn<T>(option: Option<T>, name: &str) -> Result<T, Error> {
        option.ok_or_else(|| Error::unavailable(name))
    }
}

include!(concat!(env!("OUT_DIR"), "/projectile_control_generated.rs"));

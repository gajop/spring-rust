use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureDefInfo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub metal: f32,
    pub energy: f32,
    pub max_health: f32,
    pub reclaim_time: f32,
    pub mass: f32,
    pub destructible: bool,
    pub reclaimable: bool,
    pub blocking: bool,
    pub burnable: bool,
    pub floating: bool,
    pub geothermal: bool,
    pub model_name: String,
    pub resurrect_as: String,
}

pub struct FeatureDefs<'a> {
    api: &'a sys::FeatureDefsApi,
}

impl<'a> FeatureDefs<'a> {
    pub(crate) fn new(api: &'a sys::FeatureDefsApi) -> Self {
        Self { api }
    }

    pub fn get_feature_def_info(
        &self,
        feature_def_id: i32,
    ) -> Result<Option<FeatureDefInfo>, Error> {
        self.get_feature_def_by_id(feature_def_id)
            .map(|(info, exists)| exists.then(|| FeatureDefInfo::from_raw(info)))
    }
}

impl FeatureDefInfo {
    fn from_raw(info: sys::FeatureDefInfo) -> Self {
        // SAFETY: feature definition strings are engine-owned and valid for this call.
        unsafe {
            Self {
                id: info.id,
                name: copy_c_string(info.name).unwrap_or_default(),
                description: copy_c_string(info.description).unwrap_or_default(),
                tooltip: copy_c_string(info.tooltip).unwrap_or_default(),
                metal: info.metal,
                energy: info.energy,
                max_health: info.maxHealth,
                reclaim_time: info.reclaimTime,
                mass: info.mass,
                destructible: info.destructable,
                reclaimable: info.reclaimable,
                blocking: info.blocking,
                burnable: info.burnable,
                floating: info.floating,
                geothermal: info.geoThermal,
                model_name: copy_c_string(info.modelName).unwrap_or_default(),
                resurrect_as: copy_c_string(info.resurrectAs).unwrap_or_default(),
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/feature_defs_generated.rs"));

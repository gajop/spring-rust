/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! What is under a screen position, over `trace_screen_ray` (Lua's
//! `Spring.TraceScreenRay`).

use crate::Float3;
use crate::owned::camera::{TraceScreenRayOptions, trace_screen_ray};

/// What a screen ray hit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hit {
    Unit { id: i32, position: Float3 },
    Feature { id: i32, position: Float3 },
    Ground(Float3),
    Sky(Float3),
}

impl Hit {
    fn from_raw(hit_type: i32, id: i32, position: Float3) -> Option<Self> {
        match hit_type {
            1 => Some(Self::Unit { id, position }),
            2 => Some(Self::Feature { id, position }),
            3 => Some(Self::Ground(position)),
            4 => Some(Self::Sky(position)),
            _ => None,
        }
    }
}

/// What is under screen position `(x, y)`; `None` when nothing is (for
/// example outside the map without `include_sky`).
pub fn trace_screen(x: f32, y: f32, options: TraceScreenRayOptions) -> Option<Hit> {
    let value = trace_screen_ray(x, y, options).ok()?;
    Hit::from_raw(value.hit_type, value.hit_id, value.hit_pos)
}

/// The ground position under screen position `(x, y)`, looking through units
/// and features.
pub fn ground_at(x: f32, y: f32) -> Option<Float3> {
    let options = TraceScreenRayOptions {
        only_coords: true,
        ..Default::default()
    };
    match trace_screen(x, y, options)? {
        Hit::Ground(position) => Some(position),
        _ => None,
    }
}

/// The unit under screen position `(x, y)`.
pub fn unit_at(x: f32, y: f32) -> Option<i32> {
    match trace_screen(x, y, TraceScreenRayOptions::default())? {
        Hit::Unit { id, .. } => Some(id),
        _ => None,
    }
}

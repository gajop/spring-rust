    pub mod tracing {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TraceFlags {
            TraceFeatures,
            TraceGround,
            TraceNoEnemyUnits,
            TraceOnlyEnemy,
            TraceSky,
            TraceUnits,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Ray {
            pub origin: Float3,
            pub direction: Float3,
            pub length: f32,
            pub flags: u32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayBetweenPositionsQuery {
            pub start: Float3,
            pub end: Float3,
            pub type_: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayBetweenPositionsResult {
            pub hits: Vec<TraceRayHit>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayFeaturesQuery {
            pub ray: Ray,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayFeaturesResult {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundBetweenPositionsOptions {
            pub test_water: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundBetweenPositionsQuery {
            pub start: Float3,
            pub end: Float3,
            pub options: TraceRayGroundBetweenPositionsOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayGroundBetweenPositionsResult {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundInDirectionOptions {
            pub length: Option<f32>,
            pub test_water: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayGroundInDirectionQuery {
            pub start: Float3,
            pub dir: Float3,
            pub options: TraceRayGroundInDirectionOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayGroundInDirectionResult {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayHit {
            pub hit_length: f32,
            pub object_id: i32,
            pub object_type: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayInDirectionOptions {
            pub max_length: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayInDirectionQuery {
            pub pos: Float3,
            pub dir: Float3,
            pub options: TraceRayInDirectionOptions,
            pub type_: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayInDirectionResult {
            pub hits: Vec<TraceRayHit>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayQuery {
            pub ray: Ray,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayResult {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TraceRayUnitsQuery {
            pub ray: Ray,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayUnitsResult {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayValue {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayFeaturesValue {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayGroundBetweenPositionsValue {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayGroundInDirectionValue {
            pub hit: bool,
            pub hit_length: f32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceRayUnitsValue {
            pub hit: bool,
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
            pub hit_normal: Float3,
        }

        #[inline]
        pub fn trace_ray(ray: Ray) -> Result<TraceRayValue> {
            let value = crate::generated::tracing::trace_ray(crate::generated::tracing::Ray { origin: crate::generated::tracing::Float3 { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z }, direction: crate::generated::tracing::Float3 { x: ray.direction.x, y: ray.direction.y, z: ray.direction.z }, length: ray.length, flags: ray.flags, ally_team_id: ray.ally_team_id })?;
            Ok(TraceRayValue {
                hit: value.0,
                hit_type: value.1,
                hit_id: value.2,
                hit_pos: Float3 { x: value.3.x, y: value.3.y, z: value.3.z },
                hit_normal: Float3 { x: value.4.x, y: value.4.y, z: value.4.z }
            })
        }

        #[inline]
        pub fn trace_ray_between_positions(start: Float3, end: Float3, type_: &str) -> Result<Vec<TraceRayHit>> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&start.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&start.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&start.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&end.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&end.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&end.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob2 = { let mut __b = Vec::with_capacity(4 + type_.len()); __b.extend_from_slice(&(type_.len() as u32).to_le_bytes()); __b.extend_from_slice(type_.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::tracing::trace_ray_between_positions(&__blob0, &__blob1, &__blob2, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 12);
                        let mut __result = Vec::<TraceRayHit>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(TraceRayHit { hit_length: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, object_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, object_type: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? });
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 12, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn trace_ray_features(ray: Ray) -> Result<TraceRayFeaturesValue> {
            let value = crate::generated::tracing::trace_ray_features(crate::generated::tracing::Ray { origin: crate::generated::tracing::Float3 { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z }, direction: crate::generated::tracing::Float3 { x: ray.direction.x, y: ray.direction.y, z: ray.direction.z }, length: ray.length, flags: ray.flags, ally_team_id: ray.ally_team_id })?;
            Ok(TraceRayFeaturesValue {
                hit: value.0,
                hit_type: value.1,
                hit_id: value.2,
                hit_pos: Float3 { x: value.3.x, y: value.3.y, z: value.3.z },
                hit_normal: Float3 { x: value.4.x, y: value.4.y, z: value.4.z }
            })
        }

        #[inline]
        pub fn trace_ray_ground_between_positions(start: Float3, end: Float3, options: TraceRayGroundBetweenPositionsOptions) -> Result<TraceRayGroundBetweenPositionsValue> {
            let value = crate::generated::tracing::trace_ray_ground_between_positions(crate::generated::tracing::Float3 { x: start.x, y: start.y, z: start.z }, crate::generated::tracing::Float3 { x: end.x, y: end.y, z: end.z }, crate::generated::tracing::TraceRayGroundBetweenPositionsOptions { test_water: options.test_water })?;
            Ok(TraceRayGroundBetweenPositionsValue {
                hit: value.0,
                hit_length: value.1,
                hit_pos: Float3 { x: value.2.x, y: value.2.y, z: value.2.z },
                hit_normal: Float3 { x: value.3.x, y: value.3.y, z: value.3.z }
            })
        }

        #[inline]
        pub fn trace_ray_ground_in_direction(start: Float3, dir: Float3, options: TraceRayGroundInDirectionOptions) -> Result<TraceRayGroundInDirectionValue> {
            let value = crate::generated::tracing::trace_ray_ground_in_direction(crate::generated::tracing::Float3 { x: start.x, y: start.y, z: start.z }, crate::generated::tracing::Float3 { x: dir.x, y: dir.y, z: dir.z }, crate::generated::tracing::TraceRayGroundInDirectionOptions { length: options.length, test_water: options.test_water })?;
            Ok(TraceRayGroundInDirectionValue {
                hit: value.0,
                hit_length: value.1,
                hit_pos: Float3 { x: value.2.x, y: value.2.y, z: value.2.z },
                hit_normal: Float3 { x: value.3.x, y: value.3.y, z: value.3.z }
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_trace_ray_in_direction {
            #[link(wasm_import_module = "spring:tracing")]
            unsafe extern "C" {
                #[link_name = "trace-ray-in-direction"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:tracing.trace-ray-in-direction."]
        #[doc(hidden)]
        #[inline]
        pub fn trace_ray_in_direction(p0: i32, p1: i32) -> i32 {
            __core_owned_trace_ray_in_direction::call(p0, p1)
        }

        #[inline]
        pub fn trace_ray_units(ray: Ray) -> Result<TraceRayUnitsValue> {
            let value = crate::generated::tracing::trace_ray_units(crate::generated::tracing::Ray { origin: crate::generated::tracing::Float3 { x: ray.origin.x, y: ray.origin.y, z: ray.origin.z }, direction: crate::generated::tracing::Float3 { x: ray.direction.x, y: ray.direction.y, z: ray.direction.z }, length: ray.length, flags: ray.flags, ally_team_id: ray.ally_team_id })?;
            Ok(TraceRayUnitsValue {
                hit: value.0,
                hit_type: value.1,
                hit_id: value.2,
                hit_pos: Float3 { x: value.3.x, y: value.3.y, z: value.3.z },
                hit_normal: Float3 { x: value.4.x, y: value.4.y, z: value.4.z }
            })
        }

    }


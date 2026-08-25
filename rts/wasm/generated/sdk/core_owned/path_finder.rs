    pub mod path_finder {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DeletePathQuery {
            pub path_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DeletePathResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreePathNodeCostsArrayQuery {
            pub overlay_index: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreePathNodeCostsArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetNextWayPointQuery {
            pub path_id: u32,
            pub caller_pos: Float3,
            pub min_dist: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNextWayPointResult {
            pub waypoint: Option<Float3>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPathNodeCostQuery {
            pub x: u32,
            pub z: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPathNodeCostResult {
            pub cost: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPathNodeCostsQuery {
            pub overlay_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathNodeCostsResult {
            pub costs: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPathWayPointsQuery {
            pub path_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathWayPointsResult {
            pub points: Vec<Float3>,
            pub starts: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct InitPathNodeCostsArrayQuery {
            pub overlay_index: u32,
            pub size_x: u32,
            pub size_z: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct InitPathNodeCostsArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RequestPathQuery {
            pub move_def_id: u32,
            pub move_def_name: Option<String>,
            pub start_pos: Float3,
            pub end_pos: Float3,
            pub radius: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RequestPathResult {
            pub path_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPathNodeCostQuery {
            pub overlay_index: u32,
            pub cost_index: u32,
            pub cost: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPathNodeCostResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPathNodeCostsQuery {
            pub overlay_index: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPathNodeCostsResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_path_node_costs {
            #[link(wasm_import_module = "spring:path-finder")]
            extern "C" {
                #[link_name = "get-path-node-costs"]
                pub fn call(poverlay_index: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPathWayPointsValue {
            pub points: Vec<Float3>,
            pub starts: Vec<i32>,
        }

        #[inline]
        pub fn delete_path(path_id: u32) -> Result<bool> {
            let value = crate::generated::path_finder::delete_path(path_id)?;
            Ok(value)
        }

        #[inline]
        pub fn free_path_node_costs_array(overlay_index: u32) -> Result<bool> {
            let value = crate::generated::path_finder::free_path_node_costs_array(overlay_index)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_next_way_point {
            #[link(wasm_import_module = "spring:path-finder")]
            unsafe extern "C" {
                #[link_name = "get-next-way-point"]
                pub safe fn call(p0: i32, p1: f32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:path-finder.get-next-way-point."]
        #[doc(hidden)]
        #[inline]
        pub fn get_next_way_point(p0: i32, p1: f32, p2: i32, p3: i32) -> i32 {
            __core_owned_get_next_way_point::call(p0, p1, p2, p3)
        }

        #[inline]
        pub fn get_path_node_cost(x: u32, z: u32) -> Result<f32> {
            let value = crate::generated::path_finder::get_path_node_cost(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_path_node_costs(overlay_index: u32) -> Result<Vec<f32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<f32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_path_node_costs::call(overlay_index as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (overlay_index as i32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_path_way_points {
            #[link(wasm_import_module = "spring:path-finder")]
            unsafe extern "C" {
                #[link_name = "get-path-way-points"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:path-finder.get-path-way-points."]
        #[doc(hidden)]
        #[inline]
        pub fn get_path_way_points(p0: i32, p1: i32) -> i32 {
            __core_owned_get_path_way_points::call(p0, p1)
        }

        #[inline]
        pub fn init_path_node_costs_array(overlay_index: u32, size_x: u32, size_z: u32) -> Result<bool> {
            let value = crate::generated::path_finder::init_path_node_costs_array(overlay_index, size_x, size_z)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_request_path {
            #[link(wasm_import_module = "spring:path-finder")]
            unsafe extern "C" {
                #[link_name = "request-path"]
                pub safe fn call(p0: i32, p1: f32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:path-finder.request-path."]
        #[doc(hidden)]
        #[inline]
        pub fn request_path(p0: i32, p1: f32, p2: i32) -> i64 {
            __core_owned_request_path::call(p0, p1, p2)
        }

        #[inline]
        pub fn set_path_node_cost(overlay_index: u32, cost_index: u32, cost: f32) -> Result<bool> {
            let value = crate::generated::path_finder::set_path_node_cost(overlay_index, cost_index, cost)?;
            Ok(value)
        }

        #[inline]
        pub fn set_path_node_costs(overlay_index: u32) -> Result<bool> {
            let value = crate::generated::path_finder::set_path_node_costs(overlay_index)?;
            Ok(value)
        }

    }


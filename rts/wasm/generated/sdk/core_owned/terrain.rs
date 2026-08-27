    pub mod terrain {
        use super::{Result, String};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGrassQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGrassResult {
            pub grass_level: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundBlockedQuery {
            pub x1: f32,
            pub z1: f32,
            pub x2: f32,
            pub z2: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundBlockedResult {
            pub blocked: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundExtremesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundExtremesResult {
            pub init_min_height: f32,
            pub init_max_height: f32,
            pub curr_min_height: f32,
            pub curr_max_height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundHeightQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundInfoQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundInfoResult {
            pub terrain_type_index: i32,
            pub terrain_type_name: String,
            pub metal_extraction: f32,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundNormalQuery {
            pub x: f32,
            pub z: f32,
            pub smoothed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundNormalResult {
            pub normal: Float3,
            pub slope: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundOrigHeightQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundOrigHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetHeightMapSizeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetHeightMapSizeResult {
            pub points_x: i32,
            pub points_z: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSmoothMeshHeightQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSmoothMeshHeightResult {
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTerrainTypeDataQuery {
            pub terrain_type_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTerrainTypeDataResult {
            pub index: i32,
            pub name: String,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWaterLevelQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWaterLevelResult {
            pub level: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWaterPlaneLevelQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWaterPlaneLevelResult {
            pub level: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInMapQuery {
            pub x: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInMapResult {
            pub in_map: bool,
            pub in_play_area: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundExtremesValue {
            pub init_min_height: f32,
            pub init_max_height: f32,
            pub curr_min_height: f32,
            pub curr_max_height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundInfoValue {
            pub terrain_type_index: i32,
            pub terrain_type_name: String,
            pub metal_extraction: f32,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroundNormalValue {
            pub normal: Float3,
            pub slope: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetHeightMapSizeValue {
            pub points_x: i32,
            pub points_z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTerrainTypeDataValue {
            pub index: i32,
            pub name: String,
            pub hardness: f32,
            pub tank_speed: f32,
            pub kbot_speed: f32,
            pub hover_speed: f32,
            pub ship_speed: f32,
            pub receive_tracks: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInMapValue {
            pub in_map: bool,
            pub in_play_area: bool,
        }

        #[inline]
        pub fn get_grass(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_grass(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_ground_blocked(x1: f32, z1: f32, x2: f32, z2: f32) -> Result<bool> {
            let value = crate::generated::terrain::get_ground_blocked(x1, z1, x2, z2)?;
            Ok(value)
        }

        #[inline]
        pub fn get_ground_extremes(_unused: u8) -> Result<GetGroundExtremesValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = [0u8; 16];
                let status = __core_terrain_ground_extremes::call(crate::wasm_output_ptr(&mut output)?);
                if status != 0 { return Err(crate::ApiError::new(status)); }
                Ok(GetGroundExtremesValue {
                    init_min_height: f32::from_le_bytes(output[0..4].try_into().unwrap()),
                    init_max_height: f32::from_le_bytes(output[4..8].try_into().unwrap()),
                    curr_min_height: f32::from_le_bytes(output[8..12].try_into().unwrap()),
                    curr_max_height: f32::from_le_bytes(output[12..16].try_into().unwrap()),
                })
            }
            #[cfg(not(target_arch = "wasm32"))]
            { Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_ground_extremes {
            #[link(wasm_import_module = "spring:terrain")]
            unsafe extern "C" {
                #[link_name = "get-ground-extremes"]
                pub safe fn call(output: i32) -> i32;
            }
        }

        #[inline]
        pub fn get_ground_height(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_ground_height(x, z)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_ground_info {
            #[link(wasm_import_module = "spring:terrain")]
            unsafe extern "C" {
                #[link_name = "get-ground-info"]
                pub safe fn call(p0: f32, p1: f32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:terrain.get-ground-info."]
        #[doc(hidden)]
        #[inline]
        pub fn get_ground_info(p0: f32, p1: f32, p2: i32) -> i32 {
            __core_owned_get_ground_info::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_ground_normal(x: f32, z: f32, smoothed: bool) -> Result<GetGroundNormalValue> {
            let value = crate::generated::terrain::get_ground_normal(x, z, smoothed)?;
            Ok(GetGroundNormalValue {
                normal: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                slope: value.1
            })
        }

        #[inline]
        pub fn get_ground_orig_height(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_ground_orig_height(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_height_map_size(_unused: u8) -> Result<GetHeightMapSizeValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = [0u8; 8];
                let status = __core_terrain_height_map_size::call(crate::wasm_output_ptr(&mut output)?);
                if status != 0 { return Err(crate::ApiError::new(status)); }
                Ok(GetHeightMapSizeValue {
                    points_x: i32::from_le_bytes(output[0..4].try_into().unwrap()),
                    points_z: i32::from_le_bytes(output[4..8].try_into().unwrap()),
                })
            }
            #[cfg(not(target_arch = "wasm32"))]
            { Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_height_map_size {
            #[link(wasm_import_module = "spring:terrain")]
            unsafe extern "C" {
                #[link_name = "get-height-map-size"]
                pub safe fn call(output: i32) -> i32;
            }
        }

        #[inline]
        pub fn get_smooth_mesh_height(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_smooth_mesh_height(x, z)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_terrain_type_data {
            #[link(wasm_import_module = "spring:terrain")]
            unsafe extern "C" {
                #[link_name = "get-terrain-type-data"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:terrain.get-terrain-type-data."]
        #[doc(hidden)]
        #[inline]
        pub fn get_terrain_type_data(p0: i32, p1: i32) -> i32 {
            __core_owned_get_terrain_type_data::call(p0, p1)
        }

        #[inline]
        pub fn get_water_level(x: f32, z: f32) -> Result<f32> {
            let value = crate::generated::terrain::get_water_level(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_water_plane_level(_unused: u8) -> Result<f32> {
            #[cfg(target_arch = "wasm32")]
            {
                let packed = __core_terrain_water_plane_level::call() as u64;
                let status = (packed >> 32) as u32 as i32;
                if status != 0 { return Err(crate::ApiError::new(status)); }
                Ok(f32::from_bits(packed as u32))
            }
            #[cfg(not(target_arch = "wasm32"))]
            { Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_water_plane_level {
            #[link(wasm_import_module = "spring:terrain")]
            unsafe extern "C" {
                #[link_name = "get-water-plane-level"]
                pub safe fn call() -> i64;
            }
        }

        #[inline]
        pub fn is_pos_in_map(x: f32, z: f32) -> Result<IsPosInMapValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let packed = __core_terrain_is_pos_in_map::call(x, z) as u64;
                let status = (packed >> 32) as u32 as i32;
                if status != 0 { return Err(crate::ApiError::new(status)); }
                let flags = packed as u32;
                Ok(IsPosInMapValue { in_map: flags & 1 != 0, in_play_area: flags & 2 != 0 })
            }
            #[cfg(not(target_arch = "wasm32"))]
            { let _ = (x, z); Err(unreachable!()) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_terrain_is_pos_in_map {
            #[link(wasm_import_module = "spring:terrain")]
            unsafe extern "C" {
                #[link_name = "is-pos-in-map"]
                pub safe fn call(x: f32, z: f32) -> i64;
            }
        }

    }


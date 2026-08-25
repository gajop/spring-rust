    pub mod utils {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClosestBuildPosQuery {
            pub team_id: i32,
            pub unit_def_id: i32,
            pub pos: Float3,
            pub search_radius: f32,
            pub min_dist: i32,
            pub facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClosestBuildPosResult {
            pub build_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCEGIDQuery {
            pub ceg_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCEGIDResult {
            pub ceg_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefDimensionsQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefDimensionsResult {
            pub dimensions: UnitDefDimensions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefDimensionsQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefDimensionsResult {
            pub dimensions: UnitDefDimensions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Pos2BuildPosQuery {
            pub unit_def_id: i32,
            pub pos: Float3,
            pub facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Pos2BuildPosResult {
            pub build_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TestBuildOrderQuery {
            pub unit_def_id: i32,
            pub pos: Float3,
            pub facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TestBuildOrderResult {
            pub status: i32,
            pub can_build: bool,
            pub feature: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TestMoveOrderOptions {
            pub test_terrain: bool,
            pub test_objects: bool,
            pub center_only: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TestMoveOrderQuery {
            pub unit_def_id: i32,
            pub pos: Float3,
            pub dir: Float3,
            pub options: TestMoveOrderOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TestMoveOrderResult {
            pub can_move: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDefDimensions {
            pub height: f32,
            pub radius: f32,
            pub midx: f32,
            pub minx: f32,
            pub maxx: f32,
            pub midy: f32,
            pub miny: f32,
            pub maxy: f32,
            pub midz: f32,
            pub minz: f32,
            pub maxz: f32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TestBuildOrderValue {
            pub status: i32,
            pub can_build: bool,
            pub feature: i32,
        }

        #[inline]
        pub fn closest_build_pos(team_id: i32, unit_def_id: i32, pos: Float3, search_radius: f32, min_dist: i32, facing: i32) -> Result<Float3> {
            let value = crate::generated::utils::closest_build_pos(team_id, unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, search_radius, min_dist, facing)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_cegid(ceg_name: &str) -> Result<i32> {
            let mut ceg_name_bytes = ceg_name.as_bytes().to_vec();
            if ceg_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            ceg_name_bytes.push(0);
            let ceg_name_cstr = core::ffi::CStr::from_bytes_with_nul(&ceg_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::utils::get_cegid(ceg_name_cstr)
        }

        #[inline]
        pub fn get_feature_def_dimensions(feature_def_id: i32) -> Result<UnitDefDimensions> {
            let value = crate::generated::utils::get_feature_def_dimensions(feature_def_id)?;
            Ok(UnitDefDimensions { height: value.height, radius: value.radius, midx: value.midx, minx: value.minx, maxx: value.maxx, midy: value.midy, miny: value.miny, maxy: value.maxy, midz: value.midz, minz: value.minz, maxz: value.maxz })
        }

        #[inline]
        pub fn get_unit_def_dimensions(unit_def_id: i32) -> Result<UnitDefDimensions> {
            let value = crate::generated::utils::get_unit_def_dimensions(unit_def_id)?;
            Ok(UnitDefDimensions { height: value.height, radius: value.radius, midx: value.midx, minx: value.minx, maxx: value.maxx, midy: value.midy, miny: value.miny, maxy: value.maxy, midz: value.midz, minz: value.minz, maxz: value.maxz })
        }

        #[inline]
        pub fn pos2_build_pos(unit_def_id: i32, pos: Float3, facing: i32) -> Result<Float3> {
            let value = crate::generated::utils::pos2_build_pos(unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, facing)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn test_build_order(unit_def_id: i32, pos: Float3, facing: i32) -> Result<TestBuildOrderValue> {
            let value = crate::generated::utils::test_build_order(unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, facing)?;
            Ok(TestBuildOrderValue {
                status: value.0,
                can_build: value.1,
                feature: value.2
            })
        }

        #[inline]
        pub fn test_move_order(unit_def_id: i32, pos: Float3, dir: Float3, options: TestMoveOrderOptions) -> Result<bool> {
            let value = crate::generated::utils::test_move_order(unit_def_id, crate::generated::utils::Float3 { x: pos.x, y: pos.y, z: pos.z }, crate::generated::utils::Float3 { x: dir.x, y: dir.y, z: dir.z }, crate::generated::utils::TestMoveOrderOptions { test_terrain: options.test_terrain, test_objects: options.test_objects, center_only: options.center_only })?;
            Ok(value)
        }

    }


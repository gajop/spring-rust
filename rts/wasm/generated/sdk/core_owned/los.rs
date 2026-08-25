    pub mod los {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetClosestValidPositionQuery {
            pub unit_def_id: i32,
            pub x: f32,
            pub z: f32,
            pub radius: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetClosestValidPositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPositionLosStateQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPositionLosStateResult {
            pub state: PositionLosState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRadarErrorParamsQuery {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRadarErrorParamsResult {
            pub params: RadarErrorParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInAirLosQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInAirLosResult {
            pub in_air_los: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInLosQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInLosResult {
            pub in_los: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInRadarQuery {
            pub pos: Float3,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsPosInRadarResult {
            pub in_radar: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInAirLosQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInAirLosResult {
            pub in_air_los: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInJammerQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInJammerResult {
            pub in_jammer: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInLosQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInLosResult {
            pub in_los: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInRadarQuery {
            pub unit_id: i32,
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsUnitInRadarResult {
            pub in_radar: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PositionLosState {
            pub in_los_or_radar: bool,
            pub in_los: bool,
            pub in_radar: bool,
            pub in_jammer: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RadarErrorParams {
            pub radar_error_size: f32,
            pub base_radar_error_size: f32,
            pub base_radar_error_mult: f32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn get_closest_valid_position(unit_def_id: i32, x: f32, z: f32, radius: f32) -> Result<Float3> {
            let value = crate::generated::los::get_closest_valid_position(unit_def_id, x, z, radius)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_position_los_state(pos: Float3, ally_team_id: i32) -> Result<PositionLosState> {
            let value = crate::generated::los::get_position_los_state(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(PositionLosState { in_los_or_radar: value.in_los_or_radar, in_los: value.in_los, in_radar: value.in_radar, in_jammer: value.in_jammer })
        }

        #[inline]
        pub fn get_radar_error_params(ally_team_id: i32) -> Result<RadarErrorParams> {
            let value = crate::generated::los::get_radar_error_params(ally_team_id)?;
            Ok(RadarErrorParams { radar_error_size: value.radar_error_size, base_radar_error_size: value.base_radar_error_size, base_radar_error_mult: value.base_radar_error_mult })
        }

        #[inline]
        pub fn is_pos_in_air_los(pos: Float3, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_pos_in_air_los(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_pos_in_los(pos: Float3, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_pos_in_los(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_pos_in_radar(pos: Float3, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_pos_in_radar(crate::generated::los::Float3 { x: pos.x, y: pos.y, z: pos.z }, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_air_los(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_air_los(unit_id, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_jammer(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_jammer(unit_id, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_los(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_los(unit_id, ally_team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_radar(unit_id: i32, ally_team_id: i32) -> Result<bool> {
            let value = crate::generated::los::is_unit_in_radar(unit_id, ally_team_id)?;
            Ok(value)
        }

    }


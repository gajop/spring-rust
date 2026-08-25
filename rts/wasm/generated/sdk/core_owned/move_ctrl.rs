    pub mod move_ctrl {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitEstimatedPathQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitEstimatedPathResult {
            pub waypoints: Vec<PathWaypoint>,
            pub starts: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMoveTypeDataQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitMoveTypeDataResult {
            pub data: MoveTypeData,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsMoveCtrlEnabledQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsMoveCtrlEnabledResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MoveCtrlQuery {
            pub unit_id: i32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MoveCtrlResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MoveTypeData {
            pub name: String,
            pub max_speed: f32,
            pub max_wanted_speed: f32,
            pub goal_x: f32,
            pub goal_y: f32,
            pub goal_z: f32,
            pub turn_rate: f32,
            pub acc_rate: f32,
            pub dec_rate: f32,
            pub max_reverse_speed: f32,
            pub wanted_speed: f32,
            pub current_speed: f32,
            pub delta_speed: f32,
            pub max_bank: f32,
            pub max_pitch: f32,
            pub max_aileron: f32,
            pub max_elevator: f32,
            pub max_rudder: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PathWaypoint {
            pub pos: Float3,
            pub eta: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMoveCtrlGravityQuery {
            pub unit_id: i32,
            pub gravity_factor: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMoveCtrlGravityResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitEstimatedPathValue {
            pub waypoints: Vec<PathWaypoint>,
            pub starts: Vec<i32>,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_estimated_path {
            #[link(wasm_import_module = "spring:move-ctrl")]
            unsafe extern "C" {
                #[link_name = "get-unit-estimated-path"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:move-ctrl.get-unit-estimated-path."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_estimated_path(p0: i32, p1: i32) -> i32 {
            __core_owned_get_unit_estimated_path::call(p0, p1)
        }

        #[inline]
        pub fn get_unit_move_type_data(unit_id: i32) -> Result<MoveTypeData> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::move_ctrl::get_unit_move_type_data(unit_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = MoveTypeData { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_wanted_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, goal_x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, goal_y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, goal_z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, turn_rate: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, acc_rate: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, dec_rate: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_reverse_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, wanted_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, current_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, delta_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_bank: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_pitch: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_aileron: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_elevator: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_rudder: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn is_move_ctrl_enabled(unit_id: i32) -> Result<bool> {
            let value = crate::generated::move_ctrl::is_move_ctrl_enabled(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn move_ctrl(unit_id: i32, enable: bool) -> Result<bool> {
            let value = crate::generated::move_ctrl::move_ctrl(unit_id, enable)?;
            Ok(value)
        }

        #[inline]
        pub fn set_move_ctrl_gravity(unit_id: i32, gravity_factor: f32) -> Result<bool> {
            let value = crate::generated::move_ctrl::set_move_ctrl_gravity(unit_id, gravity_factor)?;
            Ok(value)
        }

    }


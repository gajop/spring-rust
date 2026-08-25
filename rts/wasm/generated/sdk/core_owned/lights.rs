    pub mod lights {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddLightTrackingTargetQuery {
            pub light_handle: u32,
            pub object_id: i32,
            pub track_unit: bool,
            pub enable_tracking: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddLightTrackingTargetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddMapLightQuery {
            pub params: LightParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddMapLightResult {
            pub light_handle: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddModelLightQuery {
            pub params: LightParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddModelLightResult {
            pub light_handle: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LightParams {
            pub position: Vec<f32>,
            pub direction: Vec<f32>,
            pub ambient_color: Vec<f32>,
            pub diffuse_color: Vec<f32>,
            pub specular_color: Vec<f32>,
            pub intensity_weight: Vec<f32>,
            pub attenuation: Vec<f32>,
            pub ambient_decay_rate: Vec<f32>,
            pub diffuse_decay_rate: Vec<f32>,
            pub specular_decay_rate: Vec<f32>,
            pub decay_function_type: Vec<f32>,
            pub radius: f32,
            pub fov: f32,
            pub ttl: u32,
            pub priority: u32,
            pub ignore_los: bool,
            pub local_space: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMapLightTrackingStateQuery {
            pub light_handle: u32,
            pub object_id: i32,
            pub enable_tracking: bool,
            pub track_unit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMapLightTrackingStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetModelLightTrackingStateQuery {
            pub light_handle: u32,
            pub object_id: i32,
            pub enable_tracking: bool,
            pub track_unit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetModelLightTrackingStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateMapLightQuery {
            pub light_handle: u32,
            pub params: LightParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UpdateMapLightResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UpdateModelLightQuery {
            pub light_handle: u32,
            pub params: LightParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UpdateModelLightResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn add_light_tracking_target(light_handle: u32, object_id: i32, track_unit: bool, enable_tracking: bool) -> Result<bool> {
            let value = crate::generated::lights::add_light_tracking_target(light_handle, object_id, track_unit, enable_tracking)?;
            Ok(value)
        }

        #[inline]
        pub fn add_map_light(params: &LightParams) -> Result<u32> {
            let value = crate::generated::lights::add_map_light(crate::generated::lights::LightParams { position: params.position.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

        #[inline]
        pub fn add_model_light(params: &LightParams) -> Result<u32> {
            let value = crate::generated::lights::add_model_light(crate::generated::lights::LightParams { position: params.position.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

        #[inline]
        pub fn set_map_light_tracking_state(light_handle: u32, object_id: i32, enable_tracking: bool, track_unit: bool) -> Result<bool> {
            let value = crate::generated::lights::set_map_light_tracking_state(light_handle, object_id, enable_tracking, track_unit)?;
            Ok(value)
        }

        #[inline]
        pub fn set_model_light_tracking_state(light_handle: u32, object_id: i32, enable_tracking: bool, track_unit: bool) -> Result<bool> {
            let value = crate::generated::lights::set_model_light_tracking_state(light_handle, object_id, enable_tracking, track_unit)?;
            Ok(value)
        }

        #[inline]
        pub fn update_map_light(light_handle: u32, params: &LightParams) -> Result<bool> {
            let value = crate::generated::lights::update_map_light(light_handle, crate::generated::lights::LightParams { position: params.position.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

        #[inline]
        pub fn update_model_light(light_handle: u32, params: &LightParams) -> Result<bool> {
            let value = crate::generated::lights::update_model_light(light_handle, crate::generated::lights::LightParams { position: params.position.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, direction: params.direction.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_color: params.ambient_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_color: params.diffuse_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_color: params.specular_color.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, intensity_weight: params.intensity_weight.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, attenuation: params.attenuation.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, ambient_decay_rate: params.ambient_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, diffuse_decay_rate: params.diffuse_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, specular_decay_rate: params.specular_decay_rate.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, decay_function_type: params.decay_function_type.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, radius: params.radius, fov: params.fov, ttl: params.ttl, priority: params.priority, ignore_los: params.ignore_los, local_space: params.local_space })?;
            Ok(value)
        }

    }


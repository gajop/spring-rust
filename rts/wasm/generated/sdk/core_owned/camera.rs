    pub mod camera {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct CameraState {
            pub name: String,
            pub pos: Float3,
            pub dir: Float3,
            pub up: Float3,
            pub right: Float3,
            pub fov: f32,
            pub rx: f32,
            pub ry: f32,
            pub rz: f32,
            pub dist: f32,
            pub height: f32,
            pub angle: f32,
            pub mode: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraDirectionQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraFOVQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraFOVResult {
            pub fov: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraNamesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraNamesResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraPositionQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraPositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraStateQuery {
            pub use_table: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCameraStateResult {
            pub state: CameraState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPixelDirQuery {
            pub screen_x: f32,
            pub screen_y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPixelDirResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraStateQuery {
            pub state: CameraState,
            pub transition_time: f32,
            pub transition_time_factor: f32,
            pub transition_time_exponent: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetCameraStateResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraTargetOptions {
            pub transition_time: Option<f32>,
            pub dir_x: Option<f32>,
            pub dir_y: Option<f32>,
            pub dir_z: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetCameraTargetQuery {
            pub target: Float3,
            pub options: SetCameraTargetOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetCameraTargetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceScreenRayOptions {
            pub only_coords: bool,
            pub use_minimap: bool,
            pub include_sky: bool,
            pub ignore_water: bool,
            pub height_offset: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceScreenRayQuery {
            pub screen_x: f32,
            pub screen_y: f32,
            pub options: TraceScreenRayOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceScreenRayResult {
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct WorldToScreenCoordsQuery {
            pub world_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct WorldToScreenCoordsResult {
            pub screen_pos: Float3,
            pub valid: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TraceScreenRayValue {
            pub hit_type: i32,
            pub hit_id: i32,
            pub hit_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct WorldToScreenCoordsValue {
            pub screen_pos: Float3,
            pub valid: bool,
        }

        #[inline]
        pub fn get_camera_direction(unused: u8) -> Result<Float3> {
            let value = crate::generated::camera::get_camera_direction(unused)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_camera_fov(unused: u8) -> Result<f32> {
            let value = crate::generated::camera::get_camera_fov(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_camera_names(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::camera::get_camera_names(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
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
        pub fn get_camera_position(unused: u8) -> Result<Float3> {
            let value = crate::generated::camera::get_camera_position(unused)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_camera_state(use_table: bool) -> Result<CameraState> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::camera::get_camera_state(use_table as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = CameraState { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, pos: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, dir: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, up: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, right: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, fov: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, rx: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, ry: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, rz: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, dist: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, height: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, angle: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mode: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
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
        pub fn get_pixel_dir(screen_x: f32, screen_y: f32) -> Result<Float3> {
            let value = crate::generated::camera::get_pixel_dir(screen_x, screen_y)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn set_camera_state(state: &CameraState, transition_time: f32, transition_time_factor: f32, transition_time_exponent: f32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(state.name.len() as u32).to_le_bytes()); __b.extend_from_slice(state.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.dir.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.dir.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.dir.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.up.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.up.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.up.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.right.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.right.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.right.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.fov.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.rx.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.ry.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.rz.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.dist.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.height.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.angle.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&state.mode.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::camera::set_camera_state(transition_time, transition_time_factor, transition_time_exponent, &__blob0)
        }

        #[inline]
        pub fn set_camera_target(target: Float3, options: SetCameraTargetOptions) -> Result<bool> {
            let value = crate::generated::camera::set_camera_target(crate::generated::camera::Float3 { x: target.x, y: target.y, z: target.z }, crate::generated::camera::SetCameraTargetOptions { transition_time: options.transition_time, dir_x: options.dir_x, dir_y: options.dir_y, dir_z: options.dir_z })?;
            Ok(value)
        }

        #[inline]
        pub fn trace_screen_ray(screen_x: f32, screen_y: f32, options: TraceScreenRayOptions) -> Result<TraceScreenRayValue> {
            let value = crate::generated::camera::trace_screen_ray(screen_x, screen_y, crate::generated::camera::TraceScreenRayOptions { only_coords: options.only_coords, use_minimap: options.use_minimap, include_sky: options.include_sky, ignore_water: options.ignore_water, height_offset: options.height_offset })?;
            Ok(TraceScreenRayValue {
                hit_type: value.0,
                hit_id: value.1,
                hit_pos: Float3 { x: value.2.x, y: value.2.y, z: value.2.z }
            })
        }

        #[inline]
        pub fn world_to_screen_coords(world_pos: Float3) -> Result<WorldToScreenCoordsValue> {
            let value = crate::generated::camera::world_to_screen_coords(crate::generated::camera::Float3 { x: world_pos.x, y: world_pos.y, z: world_pos.z })?;
            Ok(WorldToScreenCoordsValue {
                screen_pos: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                valid: value.1
            })
        }

    }


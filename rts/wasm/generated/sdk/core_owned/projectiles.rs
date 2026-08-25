    pub mod projectiles {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllProjectilesOptions {
            pub exclude_weapon_projectiles: bool,
            pub exclude_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllProjectilesQuery {
            pub options: GetAllProjectilesOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllProjectilesResult {
            pub projectiles: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPieceProjectileParamsQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileParamsResult {
            pub params: PieceProjectileParams,
            pub is_piece_projectile: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileAllyTeamIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileAllyTeamIDResult {
            pub ally_team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileDamagesQuery {
            pub projectile_id: i32,
            pub tag: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileDamagesResult {
            pub damages: ProjectileDamages,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileDefIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileDefIDResult {
            pub def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileDirectionQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileGravityQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileGravityResult {
            pub gravity: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileIsInterceptedQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileIsInterceptedResult {
            pub is_intercepted: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileOwnerIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileOwnerIDResult {
            pub owner_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectilePositionQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectilePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTargetQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTargetResult {
            pub target: ProjectileTarget,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTeamIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTeamIDResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTimeToLiveQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTimeToLiveResult {
            pub ttl: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTypeQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTypeResult {
            pub weapon: bool,
            pub piece: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileVelocityQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileVelocityResult {
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectilesInRectangleOptions {
            pub exclude_weapon_projectiles: bool,
            pub exclude_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectilesInRectangleQuery {
            pub min_x: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_z: f32,
            pub options: GetProjectilesInRectangleOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectilesInRectangleResult {
            pub projectiles: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectilesInSphereOptions {
            pub exclude_weapon_projectiles: bool,
            pub exclude_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectilesInSphereQuery {
            pub center: Float3,
            pub radius: f32,
            pub options: GetProjectilesInSphereOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectilesInSphereResult {
            pub projectiles: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PieceProjectileParams {
            pub pos: Float3,
            pub speed: Float3,
            pub gravity: Float3,
            pub spin_vec: Float3,
            pub expl_flags: i32,
            pub spin_angle: f32,
            pub spin_speed: f32,
            pub model_piece_num: i32,
            pub model_object_type: i32,
            pub model_name: String,
            pub team: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileDamages {
            pub damages: Vec<f32>,
            pub paralyze_damage_time: f32,
            pub impulse_factor: f32,
            pub impulse_boost: f32,
            pub crater_mult: f32,
            pub crater_boost: f32,
            pub default_damage: f32,
            pub dyn_damage_exp: f32,
            pub dyn_damage_min: f32,
            pub dyn_damage_range: f32,
            pub dyn_damage_inverted: bool,
            pub crater_area_of_effect: f32,
            pub damage_area_of_effect: f32,
            pub edge_effectiveness: f32,
            pub explosion_speed: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ProjectileTarget {
            pub target_type: i32,
            pub target_id: i32,
            pub target_pos: Float3,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileParamsValue {
            pub params: PieceProjectileParams,
            pub is_piece_projectile: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetProjectileTypeValue {
            pub weapon: bool,
            pub piece: bool,
        }

        #[inline]
        pub fn get_all_projectiles(options: GetAllProjectilesOptions) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut input = [0u8; 8];
                input[0..4].copy_from_slice(&u32::from(options.exclude_weapon_projectiles).to_le_bytes());
                input[4..8].copy_from_slice(&u32::from(options.exclude_piece_projectiles).to_le_bytes());
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { crate::generated::projectiles::raw::core_get_all_projectiles(input.as_ptr() as usize as u32 as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = options;
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_piece_projectile_params(projectile_id: i32) -> Result<GetPieceProjectileParamsValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::projectiles::get_piece_projectile_params(projectile_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetPieceProjectileParamsValue {
                            params: PieceProjectileParams { pos: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, speed: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, gravity: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, spin_vec: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, expl_flags: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, spin_angle: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, spin_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, model_piece_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, model_object_type: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, model_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, team: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            is_piece_projectile: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                        };
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
        pub fn get_projectile_ally_team_id(projectile_id: i32) -> Result<i32> {
            let value = crate::generated::projectiles::get_projectile_ally_team_id(projectile_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_projectile_damages {
            #[link(wasm_import_module = "spring:projectiles")]
            unsafe extern "C" {
                #[link_name = "get-projectile-damages"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:projectiles.get-projectile-damages."]
        #[doc(hidden)]
        #[inline]
        pub fn get_projectile_damages(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_projectile_damages::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_projectile_def_id(projectile_id: i32) -> Result<i32> {
            let value = crate::generated::projectiles::get_projectile_def_id(projectile_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_projectile_direction(projectile_id: i32) -> Result<Float3> {
            let value = crate::generated::projectiles::get_projectile_direction(projectile_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_projectile_gravity(projectile_id: i32) -> Result<Float3> {
            let value = crate::generated::projectiles::get_projectile_gravity(projectile_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_projectile_is_intercepted(projectile_id: i32) -> Result<bool> {
            let value = crate::generated::projectiles::get_projectile_is_intercepted(projectile_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_projectile_owner_id(projectile_id: i32) -> Result<i32> {
            let value = crate::generated::projectiles::get_projectile_owner_id(projectile_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_projectile_position(projectile_id: i32) -> Result<Float3> {
            let value = crate::generated::projectiles::get_projectile_position(projectile_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_projectile_target(projectile_id: i32) -> Result<ProjectileTarget> {
            let value = crate::generated::projectiles::get_projectile_target(projectile_id)?;
            Ok(ProjectileTarget { target_type: value.target_type, target_id: value.target_id, target_pos: Float3 { x: value.target_pos.x, y: value.target_pos.y, z: value.target_pos.z } })
        }

        #[inline]
        pub fn get_projectile_team_id(projectile_id: i32) -> Result<i32> {
            let value = crate::generated::projectiles::get_projectile_team_id(projectile_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_projectile_time_to_live(projectile_id: i32) -> Result<f32> {
            let value = crate::generated::projectiles::get_projectile_time_to_live(projectile_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_projectile_type(projectile_id: i32) -> Result<GetProjectileTypeValue> {
            let value = crate::generated::projectiles::get_projectile_type(projectile_id)?;
            Ok(GetProjectileTypeValue {
                weapon: value.0,
                piece: value.1
            })
        }

        #[inline]
        pub fn get_projectile_velocity(projectile_id: i32) -> Result<Float3> {
            let value = crate::generated::projectiles::get_projectile_velocity(projectile_id)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_projectiles_in_rectangle {
            #[link(wasm_import_module = "spring:projectiles")]
            unsafe extern "C" {
                #[link_name = "get-projectiles-in-rectangle"]
                pub safe fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:projectiles.get-projectiles-in-rectangle."]
        #[doc(hidden)]
        #[inline]
        pub fn get_projectiles_in_rectangle(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32 {
            __core_owned_get_projectiles_in_rectangle::call(p0, p1, p2, p3, p4, p5)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_projectiles_in_sphere {
            #[link(wasm_import_module = "spring:projectiles")]
            unsafe extern "C" {
                #[link_name = "get-projectiles-in-sphere"]
                pub safe fn call(p0: f32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:projectiles.get-projectiles-in-sphere."]
        #[doc(hidden)]
        #[inline]
        pub fn get_projectiles_in_sphere(p0: f32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_projectiles_in_sphere::call(p0, p1, p2)
        }

    }


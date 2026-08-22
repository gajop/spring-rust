    pub mod projectiles {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CommonErrorCode {
            ErrorAlreadyExists,
            ErrorBufferOverflow,
            ErrorInternal,
            ErrorInvalidArgument,
            ErrorInvalidId,
            ErrorInvalidState,
            ErrorNone,
            ErrorNotAvailable,
            ErrorNotFound,
            ErrorOperationFailed,
            ErrorOutOfBounds,
            ErrorPermissionDenied,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AtmosphereParams {
            pub fog_color: Option<Vec<f32>>,
            pub sky_color: Option<Vec<f32>>,
            pub sun_color: Option<Vec<f32>>,
            pub cloud_color: Option<Vec<f32>>,
            pub sky_axis_angle: Option<Vec<f32>>,
            pub fog_start: Option<f32>,
            pub fog_end: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CollisionVolumeData {
            pub scale_x: f32,
            pub scale_y: f32,
            pub scale_z: f32,
            pub offset_x: f32,
            pub offset_y: f32,
            pub offset_z: f32,
            pub volume_type: i32,
            pub test_type: i32,
            pub primary_axis: i32,
            pub disabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2 {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2Result {
            pub value: Float2,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Result {
            pub value: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4Result {
            pub value: Float4,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllProjectilesOptions {
            pub exclude_weapon_projectiles: bool,
            pub exclude_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllProjectilesQuery {
            pub options: GetAllProjectilesOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllProjectilesResult {
            pub projectiles: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileParamsQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileParamsResult {
            pub params: PieceProjectileParams,
            pub is_piece_projectile: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileAllyTeamIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileDefIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileDefIDResult {
            pub def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileDirectionQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileGravityQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileGravityResult {
            pub gravity: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileIsInterceptedQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileIsInterceptedResult {
            pub is_intercepted: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileOwnerIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileOwnerIDResult {
            pub owner_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectilePositionQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectilePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTargetQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTargetResult {
            pub target: ProjectileTarget,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTeamIDQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTeamIDResult {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTimeToLiveQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTimeToLiveResult {
            pub ttl: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTypeQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileTypeResult {
            pub weapon: bool,
            pub piece: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileVelocityQuery {
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectileVelocityResult {
            pub velocity: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectilesInRectangleOptions {
            pub exclude_weapon_projectiles: bool,
            pub exclude_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetProjectilesInSphereOptions {
            pub exclude_weapon_projectiles: bool,
            pub exclude_piece_projectiles: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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
        pub struct Int2 {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int3 {
            pub x: i32,
            pub y: i32,
            pub z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Result {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MapRenderingParams {
            pub splat_tex_scales: Option<Vec<f32>>,
            pub splat_tex_mults: Option<Vec<f32>>,
            pub void_water: Option<bool>,
            pub void_ground: Option<bool>,
            pub splat_detail_normal_diffuse_alpha: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeExplosionParams {
            pub damages: f32,
            pub weapon_def_id: i32,
            pub owner_id: i32,
            pub hit_unit_id: i32,
            pub hit_feature_id: i32,
            pub crater_area_of_effect: f32,
            pub damage_area_of_effect: f32,
            pub edge_effectiveness: f32,
            pub explosion_speed: f32,
            pub gfx_mod: f32,
            pub impact_only: bool,
            pub ignore_owner: bool,
            pub damage_ground: bool,
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeProjectileParams {
            pub pos: Float3,
            pub speed: Float3,
            pub spread: Float3,
            pub end: Float3,
            pub owner: i32,
            pub team: i32,
            pub weapon_num: i32,
            pub ttl: f32,
            pub gravity: f32,
            pub tracking: f32,
            pub max_range: f32,
            pub up_time: f32,
            pub start_alpha: f32,
            pub end_alpha: f32,
            pub model: String,
            pub ceg_tag: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NumberOrBool {
            pub number: f32,
            pub boolean: bool,
            pub use_boolean: bool,
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTarget {
            pub target_type: i32,
            pub target_id: i32,
            pub target_pos: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourcePack {
            pub metal: f32,
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RgbColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SunLightingParams {
            pub ground_ambient_color: Option<Vec<f32>>,
            pub ground_diffuse_color: Option<Vec<f32>>,
            pub ground_specular_color: Option<Vec<f32>>,
            pub model_ambient_color: Option<Vec<f32>>,
            pub model_diffuse_color: Option<Vec<f32>>,
            pub model_specular_color: Option<Vec<f32>>,
            pub specular_exponent: Option<f32>,
            pub ground_shadow_density: Option<f32>,
            pub model_shadow_density: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Result {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCostOverrides {
            pub build_time: f32,
            pub metal_cost: f32,
            pub energy_cost: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitHealthValue {
            pub health: f32,
            pub capture: f32,
            pub paralyze: f32,
            pub build: f32,
            pub use_amounts: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WaterParams {
            pub absorb: Option<Vec<f32>>,
            pub base_color: Option<Vec<f32>>,
            pub min_color: Option<Vec<f32>>,
            pub surface_color: Option<Vec<f32>>,
            pub diffuse_color: Option<Vec<f32>>,
            pub specular_color: Option<Vec<f32>>,
            pub plane_color: Option<Vec<f32>>,
            pub repeat_x: Option<f32>,
            pub repeat_y: Option<f32>,
            pub surface_alpha: Option<f32>,
            pub ambient_factor: Option<f32>,
            pub diffuse_factor: Option<f32>,
            pub specular_factor: Option<f32>,
            pub specular_power: Option<f32>,
            pub fresnel_min: Option<f32>,
            pub fresnel_max: Option<f32>,
            pub fresnel_power: Option<f32>,
            pub reflection_distortion: Option<f32>,
            pub blur_base: Option<f32>,
            pub blur_exponent: Option<f32>,
            pub perlin_start_freq: Option<f32>,
            pub perlin_lacunarity: Option<f32>,
            pub perlin_amplitude: Option<f32>,
            pub wind_speed: Option<f32>,
            pub wave_offset_factor: Option<f32>,
            pub wave_length: Option<f32>,
            pub wave_foam_distortion: Option<f32>,
            pub wave_foam_intensity: Option<f32>,
            pub caustics_resolution: Option<f32>,
            pub caustics_strength: Option<f32>,
            pub num_tiles: Option<f32>,
            pub shore_waves: Option<bool>,
            pub force_rendering: Option<bool>,
            pub has_water_plane: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPieceProjectileParamsValue {
            pub params: PieceProjectileParams,
            pub is_piece_projectile: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_piece_projectile_params {
            #[link(wasm_import_module = "spring:projectiles")]
            extern "C" {
                #[link_name = "get-piece-projectile-params"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:projectiles.get-piece-projectile-params."]
        #[inline]
        pub unsafe fn get_piece_projectile_params(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_piece_projectile_params::call(p0, p1) }
        }

        #[inline]
        pub fn get_projectile_ally_team_id(projectile_id: i32) -> Result<i32> {
            let value = crate::generated::projectiles::get_projectile_ally_team_id(projectile_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_projectile_damages {
            #[link(wasm_import_module = "spring:projectiles")]
            extern "C" {
                #[link_name = "get-projectile-damages"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:projectiles.get-projectile-damages."]
        #[inline]
        pub unsafe fn get_projectile_damages(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_projectile_damages::call(p0, p1, p2) }
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
            extern "C" {
                #[link_name = "get-projectiles-in-rectangle"]
                pub fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:projectiles.get-projectiles-in-rectangle."]
        #[inline]
        pub unsafe fn get_projectiles_in_rectangle(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32 {
            unsafe { __core_owned_get_projectiles_in_rectangle::call(p0, p1, p2, p3, p4, p5) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_projectiles_in_sphere {
            #[link(wasm_import_module = "spring:projectiles")]
            extern "C" {
                #[link_name = "get-projectiles-in-sphere"]
                pub fn call(p0: f32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:projectiles.get-projectiles-in-sphere."]
        #[inline]
        pub unsafe fn get_projectiles_in_sphere(p0: f32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_projectiles_in_sphere::call(p0, p1, p2) }
        }

    }


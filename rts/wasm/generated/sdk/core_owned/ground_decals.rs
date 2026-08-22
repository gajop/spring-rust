    pub mod ground_decals {
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
        pub struct CreateGroundDecalQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateGroundDecalResult {
            pub decal_id: u32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DestroyGroundDecalQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DestroyGroundDecalResult {
            pub success: bool,
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
        pub struct GetAllGroundDecalsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllGroundDecalsResult {
            pub decal_i_ds: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalAlphaQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalAlphaResult {
            pub alpha: f32,
            pub alpha_falloff: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalCreationFrameQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalCreationFrameResult {
            pub creation_frame_min: f32,
            pub creation_frame_max: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalGlowParamsQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalGlowParamsResult {
            pub glow: f32,
            pub glow_falloff: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalMiddlePosQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalMiddlePosResult {
            pub mid_pos: Vec<f32>,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalMiscQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalMiscResult {
            pub dot_elim_exp: f32,
            pub ref_height: f32,
            pub min_height: f32,
            pub max_height: f32,
            pub force_height_mode: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalNormalQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalNormalResult {
            pub normal: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalOwnerQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalOwnerResult {
            pub has_owner: bool,
            pub owner_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalQuadPosQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalQuadPosResult {
            pub positions: Vec<f32>,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalRotationQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalRotationResult {
            pub rotation: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalSizeAndHeightQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalSizeAndHeightResult {
            pub size_x: f32,
            pub size_z: f32,
            pub height: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTextureParamsQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTextureParamsResult {
            pub tex_wrap_distance: f32,
            pub tex_traveled_distance: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTextureQuery {
            pub decal_id: u32,
            pub main_tex: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTextureResult {
            pub texture: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTexturesOptions {
            pub main_tex: Option<bool>,
            pub include_filenames: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTexturesQuery {
            pub options: GetGroundDecalTexturesOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTexturesResult {
            pub textures: Vec<String>,
            pub filenames: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTintQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTintResult {
            pub tint: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTypeQuery {
            pub decal_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTypeResult {
            pub type_: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalUserDataQuery {
            pub decal_id: u32,
            pub quad_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalUserDataResult {
            pub values: Vec<f32>,
            pub success: bool,
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
        pub struct SetGroundDecalAlphaQuery {
            pub decal_id: u32,
            pub alpha: f32,
            pub alpha_falloff: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalAlphaResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalCreationFrameQuery {
            pub decal_id: u32,
            pub creation_frame_min: f32,
            pub creation_frame_max: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalCreationFrameResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalGlowParamsQuery {
            pub decal_id: u32,
            pub glow: f32,
            pub glow_falloff: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalGlowParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalMiscQuery {
            pub decal_id: u32,
            pub dot_elim_exp: f32,
            pub ref_height: f32,
            pub min_height: f32,
            pub max_height: f32,
            pub force_height_mode: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalMiscResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalNormalQuery {
            pub decal_id: u32,
            pub normal_x: f32,
            pub normal_y: f32,
            pub normal_z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalNormalResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalPosAndDimsQuery {
            pub decal_id: u32,
            pub mid_pos_x: f32,
            pub mid_pos_z: f32,
            pub size_x: f32,
            pub size_z: f32,
            pub proj_cube_height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalPosAndDimsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalQuadPosAndHeightQuery {
            pub decal_id: u32,
            pub pos_tlx: f32,
            pub pos_tly: f32,
            pub pos_trx: f32,
            pub pos_try: f32,
            pub pos_brx: f32,
            pub pos_bry: f32,
            pub pos_blx: f32,
            pub pos_bly: f32,
            pub proj_cube_height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalQuadPosAndHeightResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalRotationQuery {
            pub decal_id: u32,
            pub rotation: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalRotationResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalTextureParamsQuery {
            pub decal_id: u32,
            pub tex_wrap_distance: f32,
            pub tex_traveled_distance: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalTextureParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalTextureQuery {
            pub decal_id: u32,
            pub texture_name: String,
            pub main_tex: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalTintQuery {
            pub decal_id: u32,
            pub tint_r: f32,
            pub tint_g: f32,
            pub tint_b: f32,
            pub tint_a: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalTintResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalUserDataQuery {
            pub decal_id: u32,
            pub quad_index: u32,
            pub value_x: f32,
            pub value_y: f32,
            pub value_z: f32,
            pub value_w: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGroundDecalUserDataResult {
            pub success: bool,
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

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_all_ground_decals {
            #[link(wasm_import_module = "spring:ground-decals")]
            extern "C" {
                #[link_name = "get-all-ground-decals"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_ground_decal_texture {
            #[link(wasm_import_module = "spring:ground-decals")]
            extern "C" {
                #[link_name = "get-ground-decal-texture"]
                pub fn call(pdecal_id: i32, pmain_tex: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_ground_decal_type {
            #[link(wasm_import_module = "spring:ground-decals")]
            extern "C" {
                #[link_name = "get-ground-decal-type"]
                pub fn call(pdecal_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateGroundDecalValue {
            pub decal_id: u32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalAlphaValue {
            pub alpha: f32,
            pub alpha_falloff: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalCreationFrameValue {
            pub creation_frame_min: f32,
            pub creation_frame_max: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalGlowParamsValue {
            pub glow: f32,
            pub glow_falloff: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalMiddlePosValue {
            pub mid_pos: Vec<f32>,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalMiscValue {
            pub dot_elim_exp: f32,
            pub ref_height: f32,
            pub min_height: f32,
            pub max_height: f32,
            pub force_height_mode: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalOwnerValue {
            pub has_owner: bool,
            pub owner_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalQuadPosValue {
            pub positions: Vec<f32>,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalRotationValue {
            pub rotation: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalSizeAndHeightValue {
            pub size_x: f32,
            pub size_z: f32,
            pub height: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTextureParamsValue {
            pub tex_wrap_distance: f32,
            pub tex_traveled_distance: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalTexturesValue {
            pub textures: Vec<String>,
            pub filenames: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroundDecalUserDataValue {
            pub values: Vec<f32>,
            pub success: bool,
        }

        #[inline]
        pub fn create_ground_decal(unused: u8) -> Result<CreateGroundDecalValue> {
            let value = crate::generated::ground_decals::create_ground_decal(unused)?;
            Ok(CreateGroundDecalValue {
                decal_id: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn destroy_ground_decal(decal_id: u32) -> Result<bool> {
            let value = crate::generated::ground_decals::destroy_ground_decal(decal_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_all_ground_decals(unused: u8) -> Result<Vec<u32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_all_ground_decals::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_ground_decal_alpha(decal_id: u32) -> Result<GetGroundDecalAlphaValue> {
            let value = crate::generated::ground_decals::get_ground_decal_alpha(decal_id)?;
            Ok(GetGroundDecalAlphaValue {
                alpha: value.0,
                alpha_falloff: value.1
            })
        }

        #[inline]
        pub fn get_ground_decal_creation_frame(decal_id: u32) -> Result<GetGroundDecalCreationFrameValue> {
            let value = crate::generated::ground_decals::get_ground_decal_creation_frame(decal_id)?;
            Ok(GetGroundDecalCreationFrameValue {
                creation_frame_min: value.0,
                creation_frame_max: value.1
            })
        }

        #[inline]
        pub fn get_ground_decal_glow_params(decal_id: u32) -> Result<GetGroundDecalGlowParamsValue> {
            let value = crate::generated::ground_decals::get_ground_decal_glow_params(decal_id)?;
            Ok(GetGroundDecalGlowParamsValue {
                glow: value.0,
                glow_falloff: value.1
            })
        }

        #[inline]
        pub fn get_ground_decal_middle_pos(decal_id: u32) -> Result<GetGroundDecalMiddlePosValue> {
            let value = crate::generated::ground_decals::get_ground_decal_middle_pos(decal_id)?;
            Ok(GetGroundDecalMiddlePosValue {
                mid_pos: value.0.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?,
                success: value.1
            })
        }

        #[inline]
        pub fn get_ground_decal_misc(decal_id: u32) -> Result<GetGroundDecalMiscValue> {
            let value = crate::generated::ground_decals::get_ground_decal_misc(decal_id)?;
            Ok(GetGroundDecalMiscValue {
                dot_elim_exp: value.0,
                ref_height: value.1,
                min_height: value.2,
                max_height: value.3,
                force_height_mode: value.4
            })
        }

        #[inline]
        pub fn get_ground_decal_normal(decal_id: u32) -> Result<Vec<f32>> {
            let value = crate::generated::ground_decals::get_ground_decal_normal(decal_id)?;
            Ok(value.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?)
        }

        #[inline]
        pub fn get_ground_decal_owner(decal_id: u32) -> Result<GetGroundDecalOwnerValue> {
            let value = crate::generated::ground_decals::get_ground_decal_owner(decal_id)?;
            Ok(GetGroundDecalOwnerValue {
                has_owner: value.0,
                owner_id: value.1
            })
        }

        #[inline]
        pub fn get_ground_decal_quad_pos(decal_id: u32) -> Result<GetGroundDecalQuadPosValue> {
            let value = crate::generated::ground_decals::get_ground_decal_quad_pos(decal_id)?;
            Ok(GetGroundDecalQuadPosValue {
                positions: value.0.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?,
                success: value.1
            })
        }

        #[inline]
        pub fn get_ground_decal_rotation(decal_id: u32) -> Result<GetGroundDecalRotationValue> {
            let value = crate::generated::ground_decals::get_ground_decal_rotation(decal_id)?;
            Ok(GetGroundDecalRotationValue {
                rotation: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn get_ground_decal_size_and_height(decal_id: u32) -> Result<GetGroundDecalSizeAndHeightValue> {
            let value = crate::generated::ground_decals::get_ground_decal_size_and_height(decal_id)?;
            Ok(GetGroundDecalSizeAndHeightValue {
                size_x: value.0,
                size_z: value.1,
                height: value.2,
                success: value.3
            })
        }

        #[inline]
        pub fn get_ground_decal_texture(decal_id: u32, main_tex: bool) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_ground_decal_texture::call(decal_id as i32, u32::from(main_tex) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
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
                let _ = (decal_id as i32, u32::from(main_tex) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_ground_decal_texture_params(decal_id: u32) -> Result<GetGroundDecalTextureParamsValue> {
            let value = crate::generated::ground_decals::get_ground_decal_texture_params(decal_id)?;
            Ok(GetGroundDecalTextureParamsValue {
                tex_wrap_distance: value.0,
                tex_traveled_distance: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_ground_decal_textures {
            #[link(wasm_import_module = "spring:ground-decals")]
            extern "C" {
                #[link_name = "get-ground-decal-textures"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:ground-decals.get-ground-decal-textures."]
        #[inline]
        pub unsafe fn get_ground_decal_textures(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_ground_decal_textures::call(p0, p1) }
        }

        #[inline]
        pub fn get_ground_decal_tint(decal_id: u32) -> Result<Vec<f32>> {
            let value = crate::generated::ground_decals::get_ground_decal_tint(decal_id)?;
            Ok(value.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?)
        }

        #[inline]
        pub fn get_ground_decal_type(decal_id: u32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_ground_decal_type::call(decal_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
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
                let _ = (decal_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_ground_decal_user_data(decal_id: u32, quad_index: u32) -> Result<GetGroundDecalUserDataValue> {
            let value = crate::generated::ground_decals::get_ground_decal_user_data(decal_id, quad_index)?;
            Ok(GetGroundDecalUserDataValue {
                values: value.0.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?,
                success: value.1
            })
        }

        #[inline]
        pub fn set_ground_decal_alpha(decal_id: u32, alpha: f32, alpha_falloff: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_alpha(decal_id, alpha, alpha_falloff)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_creation_frame(decal_id: u32, creation_frame_min: f32, creation_frame_max: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_creation_frame(decal_id, creation_frame_min, creation_frame_max)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_glow_params(decal_id: u32, glow: f32, glow_falloff: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_glow_params(decal_id, glow, glow_falloff)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_misc(decal_id: u32, dot_elim_exp: f32, ref_height: f32, min_height: f32, max_height: f32, force_height_mode: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_misc(decal_id, dot_elim_exp, ref_height, min_height, max_height, force_height_mode)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_normal(decal_id: u32, normal_x: f32, normal_y: f32, normal_z: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_normal(decal_id, normal_x, normal_y, normal_z)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_pos_and_dims(decal_id: u32, mid_pos_x: f32, mid_pos_z: f32, size_x: f32, size_z: f32, proj_cube_height: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_pos_and_dims(decal_id, mid_pos_x, mid_pos_z, size_x, size_z, proj_cube_height)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_quad_pos_and_height(decal_id: u32, pos_tlx: f32, pos_tly: f32, pos_trx: f32, pos_try: f32, pos_brx: f32, pos_bry: f32, pos_blx: f32, pos_bly: f32, proj_cube_height: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_quad_pos_and_height(decal_id, pos_tlx, pos_tly, pos_trx, pos_try, pos_brx, pos_bry, pos_blx, pos_bly, proj_cube_height)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_rotation(decal_id: u32, rotation: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_rotation(decal_id, rotation)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_texture(decal_id: u32, texture_name: &str, main_tex: bool) -> Result<bool> {
            let mut texture_name_bytes = texture_name.as_bytes().to_vec();
            if texture_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            texture_name_bytes.push(0);
            let texture_name_cstr = core::ffi::CStr::from_bytes_with_nul(&texture_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::ground_decals::set_ground_decal_texture(decal_id, &texture_name_cstr, main_tex)
        }

        #[inline]
        pub fn set_ground_decal_texture_params(decal_id: u32, tex_wrap_distance: f32, tex_traveled_distance: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_texture_params(decal_id, tex_wrap_distance, tex_traveled_distance)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_tint(decal_id: u32, tint_r: f32, tint_g: f32, tint_b: f32, tint_a: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_tint(decal_id, tint_r, tint_g, tint_b, tint_a)?;
            Ok(value)
        }

        #[inline]
        pub fn set_ground_decal_user_data(decal_id: u32, quad_index: u32, value_x: f32, value_y: f32, value_z: f32, value_w: f32) -> Result<bool> {
            let value = crate::generated::ground_decals::set_ground_decal_user_data(decal_id, quad_index, value_x, value_y, value_z, value_w)?;
            Ok(value)
        }

    }


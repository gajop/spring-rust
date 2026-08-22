    pub mod math_extra {
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
        pub struct BitAndQuery {
            pub a: u32,
            pub b: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitAndResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitBitsQuery {
            pub bits: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitBitsResult {
            pub bits: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitInvQuery {
            pub a: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitInvResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitOrQuery {
            pub a: u32,
            pub b: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitOrResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitXorQuery {
            pub a: u32,
            pub b: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitXorResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClampQuery {
            pub value: f32,
            pub min: f32,
            pub max: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClampResult {
            pub clamped: f32,
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
        pub struct DiagQuery {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DiagResult {
            pub length: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ErfQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ErfResult {
            pub result: f32,
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
        pub struct HypotQuery {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HypotResult {
            pub value: f32,
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
        pub struct MixQuery {
            pub a: f32,
            pub b: f32,
            pub t: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MixResult {
            pub mixed: f32,
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
        pub struct NormalizeQuery {
            pub vec: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NormalizeResult {
            pub length: f32,
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
        pub struct RoundQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RoundResult {
            pub rounded: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SgnQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SgnResult {
            pub sign: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SmoothStepQuery {
            pub edge0: f32,
            pub edge1: f32,
            pub x: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SmoothStepResult {
            pub value: f32,
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
        pub struct NormalizeValue {
            pub length: f32,
            pub vec: Float3,
        }

        #[inline]
        pub fn bit_and(a: u32, b: u32) -> Result<u32> {
            let value = crate::generated::math_extra::bit_and(a, b)?;
            Ok(value)
        }

        #[inline]
        pub fn bit_bits(bits: &Vec<u32>) -> Result<u32> {
            crate::generated::borrowed::math_extra::bit_bits(bits.as_slice())
        }

        #[inline]
        pub fn bit_inv(a: u32) -> Result<u32> {
            let value = crate::generated::math_extra::bit_inv(a)?;
            Ok(value)
        }

        #[inline]
        pub fn bit_or(a: u32, b: u32) -> Result<u32> {
            let value = crate::generated::math_extra::bit_or(a, b)?;
            Ok(value)
        }

        #[inline]
        pub fn bit_xor(a: u32, b: u32) -> Result<u32> {
            let value = crate::generated::math_extra::bit_xor(a, b)?;
            Ok(value)
        }

        #[inline]
        pub fn clamp(value: f32, min: f32, max: f32) -> Result<f32> {
            let value = crate::generated::math_extra::clamp(value, min, max)?;
            Ok(value)
        }

        #[inline]
        pub fn diag(values: &Vec<f32>) -> Result<f32> {
            crate::generated::borrowed::math_extra::diag(values.as_slice())
        }

        #[inline]
        pub fn erf(value: f32) -> Result<f32> {
            let value = crate::generated::math_extra::erf(value)?;
            Ok(value)
        }

        #[inline]
        pub fn hypot(x: f32, y: f32) -> Result<f32> {
            let value = crate::generated::math_extra::hypot(x, y)?;
            Ok(value)
        }

        #[inline]
        pub fn mix(a: f32, b: f32, t: f32) -> Result<f32> {
            let value = crate::generated::math_extra::mix(a, b, t)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_math_extra_normalize {
            #[link(wasm_import_module = "spring:math-extra")]
            extern "C" {
                #[link_name = "normalize"]
                pub fn call(x: f32, y: f32, z: f32, output: i32) -> i32;
            }
        }

        #[inline]
        pub fn normalize(vec: Float3) -> Result<NormalizeValue> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut output = [0.0f32; 4];
                let pointer = output.as_mut_ptr() as usize;
                if pointer > u32::MAX as usize {
                    return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32));
                }
                let status = unsafe {
                    __core_math_extra_normalize::call(
                        vec.x,
                        vec.y,
                        vec.z,
                        pointer as u32 as i32,
                    )
                };
                if status != 0 {
                    return Err(crate::ApiError::new(status));
                }
                return Ok(NormalizeValue {
                    length: output[3],
                    vec: Float3 {
                        x: output[0],
                        y: output[1],
                        z: output[2],
                    },
                });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = vec;
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn round(value: f32) -> Result<f32> {
            let value = crate::generated::math_extra::round(value)?;
            Ok(value)
        }

        #[inline]
        pub fn sgn(value: f32) -> Result<f32> {
            let value = crate::generated::math_extra::sgn(value)?;
            Ok(value)
        }

        #[inline]
        pub fn smooth_step(edge0: f32, edge1: f32, x: f32) -> Result<f32> {
            let value = crate::generated::math_extra::smooth_step(edge0, edge1, x)?;
            Ok(value)
        }

    }


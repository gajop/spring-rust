    pub mod math_extra {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitAndQuery {
            pub a: u32,
            pub b: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitAndResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BitBitsQuery {
            pub bits: Vec<u32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitBitsResult {
            pub bits: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitInvQuery {
            pub a: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitInvResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitOrQuery {
            pub a: u32,
            pub b: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitOrResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitXorQuery {
            pub a: u32,
            pub b: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct BitXorResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClampQuery {
            pub value: f32,
            pub min: f32,
            pub max: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClampResult {
            pub clamped: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DiagQuery {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DiagResult {
            pub length: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ErfQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ErfResult {
            pub result: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HypotQuery {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HypotResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MixQuery {
            pub a: f32,
            pub b: f32,
            pub t: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MixResult {
            pub mixed: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct NormalizeQuery {
            pub vec: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct NormalizeResult {
            pub length: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RoundQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RoundResult {
            pub rounded: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SgnQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SgnResult {
            pub sign: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SmoothStepQuery {
            pub edge0: f32,
            pub edge1: f32,
            pub x: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SmoothStepResult {
            pub value: f32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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
        pub fn bit_bits(bits: &[u32]) -> Result<u32> {
            crate::generated::borrowed::math_extra::bit_bits(bits)
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
        pub fn diag(values: &[f32]) -> Result<f32> {
            crate::generated::borrowed::math_extra::diag(values)
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
                Ok(NormalizeValue {
                    length: output[3],
                    vec: Float3 {
                        x: output[0],
                        y: output[1],
                        z: output[2],
                    },
                })
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


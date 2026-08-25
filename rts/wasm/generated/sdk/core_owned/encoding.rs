    pub mod encoding {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct DecodeBase64Query {
            pub text: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DecodeBase64Result {
            pub decoded: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DecodeBase64UrlQuery {
            pub text: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DecodeBase64UrlResult {
            pub decoded: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EncodeBase64Query {
            pub text: Vec<u8>,
            pub strip_padding: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EncodeBase64Result {
            pub encoded: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EncodeBase64UrlQuery {
            pub text: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EncodeBase64UrlResult {
            pub encoded: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsValidBase64Query {
            pub text: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsValidBase64Result {
            pub valid: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsValidBase64UrlQuery {
            pub text: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsValidBase64UrlResult {
            pub valid: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn decode_base64(text: &str) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + text.len()); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); __b.extend_from_slice(text.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::encoding::decode_base64(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn decode_base64_url(text: &str) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + text.len()); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); __b.extend_from_slice(text.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::encoding::decode_base64_url(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_encode_base64 {
            #[link(wasm_import_module = "spring:encoding")]
            unsafe extern "C" {
                #[link_name = "encode-base64"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:encoding.encode-base64."]
        #[doc(hidden)]
        #[inline]
        pub fn encode_base64(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_encode_base64::call(p0, p1, p2)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_encode_base64_url {
            #[link(wasm_import_module = "spring:encoding")]
            unsafe extern "C" {
                #[link_name = "encode-base64-url"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:encoding.encode-base64-url."]
        #[doc(hidden)]
        #[inline]
        pub fn encode_base64_url(p0: i32, p1: i32) -> i32 {
            __core_owned_encode_base64_url::call(p0, p1)
        }

        #[inline]
        pub fn is_valid_base64(text: &str) -> Result<bool> {
            let mut text_bytes = text.as_bytes().to_vec();
            if text_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            text_bytes.push(0);
            let text_cstr = core::ffi::CStr::from_bytes_with_nul(&text_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::encoding::is_valid_base64(text_cstr)
        }

        #[inline]
        pub fn is_valid_base64_url(text: &str) -> Result<bool> {
            let mut text_bytes = text.as_bytes().to_vec();
            if text_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            text_bytes.push(0);
            let text_cstr = core::ffi::CStr::from_bytes_with_nul(&text_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::encoding::is_valid_base64_url(text_cstr)
        }

    }


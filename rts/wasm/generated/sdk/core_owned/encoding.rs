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

        #[inline]
        pub fn encode_base64(text: &[u8], strip_padding: bool) -> Result<String> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); for __item in text.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::encoding::encode_base64(strip_padding as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        return String::from_utf8(__output)
                            .map_err(|_| crate::ApiError::new(crate::ErrorCode::Internal as i32));
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn encode_base64_url(text: &[u8]) -> Result<String> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); for __item in text.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::encoding::encode_base64_url(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        return String::from_utf8(__output)
                            .map_err(|_| crate::ApiError::new(crate::ErrorCode::Internal as i32));
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn is_valid_base64(text: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(text, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(text)?),
            };
            crate::generated::borrowed::encoding::is_valid_base64(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn is_valid_base64_url(text: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(text, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(text)?),
            };
            crate::generated::borrowed::encoding::is_valid_base64_url(__core_string_0_buf.as_cstr())
        }

    }


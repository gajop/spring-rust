    pub mod config {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ConfigValueType {
            ConfigTypeBool,
            ConfigTypeFloat,
            ConfigTypeInt,
            ConfigTypeString,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ConfigParam {
            pub name: String,
            pub type_: ConfigValueType,
            pub description: String,
            pub default_value: String,
            pub minimum_value: String,
            pub maximum_value: String,
            pub read_only: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigFloatQuery {
            pub key: String,
            pub default_value: Option<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetConfigFloatResult {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigIntQuery {
            pub key: String,
            pub default_value: Option<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetConfigIntResult {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetConfigParamsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigParamsResult {
            pub params: Vec<ConfigParam>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigStringQuery {
            pub key: String,
            pub default_value: Option<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigStringResult {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLogSectionsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLogSectionsResult {
            pub sections: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigFloatQuery {
            pub key: String,
            pub value: f32,
            pub use_overlay: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetConfigFloatResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigIntQuery {
            pub key: String,
            pub value: i32,
            pub use_overlay: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetConfigIntResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetConfigStringQuery {
            pub key: String,
            pub value: String,
            pub use_overlay: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetConfigStringResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetLogSectionFilterLevelQuery {
            pub section: String,
            pub level: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetLogSectionFilterLevelResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetConfigFloatValue {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetConfigIntValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetConfigStringValue {
            pub value: String,
            pub exists: bool,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_config_float {
            #[link(wasm_import_module = "spring:config")]
            unsafe extern "C" {
                #[link_name = "get-config-float"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-config-float."]
        #[doc(hidden)]
        #[inline]
        pub fn get_config_float(p0: i32, p1: i32) -> i32 {
            __core_owned_get_config_float::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_config_int {
            #[link(wasm_import_module = "spring:config")]
            unsafe extern "C" {
                #[link_name = "get-config-int"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-config-int."]
        #[doc(hidden)]
        #[inline]
        pub fn get_config_int(p0: i32, p1: i32) -> i32 {
            __core_owned_get_config_int::call(p0, p1)
        }

        #[inline]
        pub fn get_config_params(unused: u8) -> Result<Vec<ConfigParam>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::config::get_config_params(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(ConfigParam { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, type_: { let __value = crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?; match __value {
                            3 => ConfigValueType::ConfigTypeBool,
                            1 => ConfigValueType::ConfigTypeFloat,
                            0 => ConfigValueType::ConfigTypeInt,
                            2 => ConfigValueType::ConfigTypeString,
                            _ => return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32)),
                        } }, description: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, default_value: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, minimum_value: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, maximum_value: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, read_only: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_config_string {
            #[link(wasm_import_module = "spring:config")]
            unsafe extern "C" {
                #[link_name = "get-config-string"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:config.get-config-string."]
        #[doc(hidden)]
        #[inline]
        pub fn get_config_string(p0: i32, p1: i32) -> i32 {
            __core_owned_get_config_string::call(p0, p1)
        }

        #[inline]
        pub fn get_log_sections(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::config::get_log_sections(unused as i32, &mut __output) {
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
        pub fn set_config_float(key: &str, value: f32, use_overlay: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(key, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            crate::generated::borrowed::config::set_config_float(__core_string_0_buf.as_cstr(), value, use_overlay)
        }

        #[inline]
        pub fn set_config_int(key: &str, value: i32, use_overlay: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(key, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            crate::generated::borrowed::config::set_config_int(__core_string_0_buf.as_cstr(), value, use_overlay)
        }

        #[inline]
        pub fn set_config_string(key: &str, value: &str, use_overlay: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(key, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::config::set_config_string(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr(), use_overlay)
        }

        #[inline]
        pub fn set_log_section_filter_level(section: &str, level: i32) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(section, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(section)?),
            };
            crate::generated::borrowed::config::set_log_section_filter_level(__core_string_0_buf.as_cstr(), level)
        }

    }


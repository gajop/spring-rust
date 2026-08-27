    pub mod feature_defs {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct FeatureDefInfo {
            pub id: i32,
            pub name: String,
            pub description: String,
            pub tooltip: String,
            pub metal: f32,
            pub energy: f32,
            pub max_health: f32,
            pub reclaim_time: f32,
            pub mass: f32,
            pub destructable: bool,
            pub reclaimable: bool,
            pub blocking: bool,
            pub burnable: bool,
            pub floating: bool,
            pub geo_thermal: bool,
            pub model_name: String,
            pub resurrect_as: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefByIDQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefByIDResult {
            pub info: FeatureDefInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefCustomParamKeysQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCustomParamKeysResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCustomParamQuery {
            pub feature_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefCustomParamResult {
            pub value: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefEnergyQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefEnergyResult {
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDByNameQuery {
            pub feature_def_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefIDByNameResult {
            pub id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefIDsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefIDsResult {
            pub ids: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefMetalQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefMetalResult {
            pub metal: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureDefNameQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidFeatureDefIDQuery {
            pub feature_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidFeatureDefIDResult {
            pub valid: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_feature_def_i_ds {
            #[link(wasm_import_module = "spring:feature-defs")]
            unsafe extern "C" {
                #[link_name = "get-feature-def-i-ds"]
                pub safe fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_feature_def_name {
            #[link(wasm_import_module = "spring:feature-defs")]
            unsafe extern "C" {
                #[link_name = "get-feature-def-name"]
                pub safe fn call(pfeature_def_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureDefByIDValue {
            pub info: FeatureDefInfo,
            pub exists: bool,
        }

        #[inline]
        pub fn get_feature_def_by_id(feature_def_id: i32) -> Result<GetFeatureDefByIDValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::feature_defs::get_feature_def_by_id(feature_def_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetFeatureDefByIDValue {
                            info: FeatureDefInfo { id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, description: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, tooltip: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, metal: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_health: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, reclaim_time: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, mass: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, destructable: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, reclaimable: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, blocking: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, burnable: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, floating: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, geo_thermal: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, model_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, resurrect_as: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
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
        pub fn get_feature_def_count(unused: u8) -> Result<u32> {
            let value = crate::generated::feature_defs::get_feature_def_count(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_def_custom_param {
            #[link(wasm_import_module = "spring:feature-defs")]
            unsafe extern "C" {
                #[link_name = "get-feature-def-custom-param"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:feature-defs.get-feature-def-custom-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_feature_def_custom_param(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_feature_def_custom_param::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_feature_def_custom_param_keys(feature_def_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::feature_defs::get_feature_def_custom_param_keys(feature_def_id, &mut __output) {
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
        pub fn get_feature_def_energy(feature_def_id: i32) -> Result<f32> {
            let value = crate::generated::feature_defs::get_feature_def_energy(feature_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_def_id_by_name(feature_def_name: &str) -> Result<i32> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(feature_def_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(feature_def_name)?),
            };
            crate::generated::borrowed::feature_defs::get_feature_def_id_by_name(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn get_feature_def_i_ds(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_feature_def_i_ds::call(unused as i32, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
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
        pub fn get_feature_def_metal(feature_def_id: i32) -> Result<f32> {
            let value = crate::generated::feature_defs::get_feature_def_metal(feature_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_def_name(feature_def_id: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_feature_def_name::call(feature_def_id, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (feature_def_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn valid_feature_def_id(feature_def_id: i32) -> Result<bool> {
            let value = crate::generated::feature_defs::valid_feature_def_id(feature_def_id)?;
            Ok(value)
        }

    }


    pub mod icons {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddUnitIconQuery {
            pub icon_name: String,
            pub tex_file: String,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
            pub u0: f32,
            pub v0: f32,
            pub u1: f32,
            pub v1: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddUnitIconResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeUnitIconQuery {
            pub icon_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeUnitIconResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllIconDataArrayQuery {
            pub full_data: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllIconDataArrayResult {
            pub entries: Vec<IconDataEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetIconDataQuery {
            pub icon_name: String,
            pub full_data: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetIconDataResult {
            pub data: IconDataEntry,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IconDataEntry {
            pub name: String,
            pub atlas_tex_coords: Vec<f32>,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitIconGetDrawQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitIconGetDrawResult {
            pub draw_icon: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitIconSetDrawQuery {
            pub unit_id: i32,
            pub draw_icon: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitIconSetDrawResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
        pub fn add_unit_icon(icon_name: &str, tex_file: &str, size: f32, distance: f32, radius_adjust: bool, u0: f32, v0: f32, u1: f32, v1: f32) -> Result<bool> {
            let mut icon_name_bytes = icon_name.as_bytes().to_vec();
            if icon_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            icon_name_bytes.push(0);
            let icon_name_cstr = core::ffi::CStr::from_bytes_with_nul(&icon_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut tex_file_bytes = tex_file.as_bytes().to_vec();
            if tex_file_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            tex_file_bytes.push(0);
            let tex_file_cstr = core::ffi::CStr::from_bytes_with_nul(&tex_file_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::icons::add_unit_icon(icon_name_cstr, tex_file_cstr, size, distance, radius_adjust, u0, v0, u1, v1)
        }

        #[inline]
        pub fn free_unit_icon(icon_name: &str) -> Result<bool> {
            let mut icon_name_bytes = icon_name.as_bytes().to_vec();
            if icon_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            icon_name_bytes.push(0);
            let icon_name_cstr = core::ffi::CStr::from_bytes_with_nul(&icon_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::icons::free_unit_icon(icon_name_cstr)
        }

        #[inline]
        pub fn get_all_icon_data_array(full_data: bool) -> Result<Vec<IconDataEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::icons::get_all_icon_data_array(full_data as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(IconDataEntry { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, atlas_tex_coords: { let mut __items = Vec::with_capacity(4); for _ in 0..4usize { __items.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, size: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, distance: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, radius_adjust: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        mod __core_owned_get_icon_data {
            #[link(wasm_import_module = "spring:icons")]
            unsafe extern "C" {
                #[link_name = "get-icon-data"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:icons.get-icon-data."]
        #[doc(hidden)]
        #[inline]
        pub fn get_icon_data(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_icon_data::call(p0, p1, p2)
        }

        #[inline]
        pub fn unit_icon_get_draw(unit_id: i32) -> Result<bool> {
            let value = crate::generated::icons::unit_icon_get_draw(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn unit_icon_set_draw(unit_id: i32, draw_icon: bool) -> Result<bool> {
            let value = crate::generated::icons::unit_icon_set_draw(unit_id, draw_icon)?;
            Ok(value)
        }

    }


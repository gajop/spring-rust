    pub mod weapon_defs {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefByIDQuery {
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefByIDResult {
            pub info: WeaponDefInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefCustomParamKeysQuery {
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefCustomParamKeysResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefCustomParamQuery {
            pub weapon_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefCustomParamResult {
            pub value: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefDamageQuery {
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefDamageResult {
            pub damage: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefIDQuery {
            pub weapon_def_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefIDResult {
            pub id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefIDsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefIDsResult {
            pub ids: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefNameQuery {
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefRangeQuery {
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWeaponDefRangeResult {
            pub range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidWeaponDefIDQuery {
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidWeaponDefIDResult {
            pub valid: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WeaponDefInfo {
            pub id: i32,
            pub name: String,
            pub type_: String,
            pub description: String,
            pub range: f32,
            pub reload_time: f32,
            pub damage: f32,
            pub area_of_effect: f32,
            pub projectile_speed: f32,
            pub paralyzer: bool,
            pub impact_only: bool,
            pub turret: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_weapon_def_i_ds {
            #[link(wasm_import_module = "spring:weapon-defs")]
            extern "C" {
                #[link_name = "get-weapon-def-i-ds"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_weapon_def_name {
            #[link(wasm_import_module = "spring:weapon-defs")]
            extern "C" {
                #[link_name = "get-weapon-def-name"]
                pub fn call(pweapon_def_id: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWeaponDefByIDValue {
            pub info: WeaponDefInfo,
            pub exists: bool,
        }

        #[inline]
        pub fn get_weapon_def_by_id(weapon_def_id: i32) -> Result<GetWeaponDefByIDValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::weapon_defs::get_weapon_def_by_id(weapon_def_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetWeaponDefByIDValue {
                            info: WeaponDefInfo { id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, type_: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, description: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, range: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, reload_time: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, damage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, area_of_effect: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, projectile_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, paralyzer: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, impact_only: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, turret: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
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
        pub fn get_weapon_def_count(unused: u8) -> Result<u32> {
            let value = crate::generated::weapon_defs::get_weapon_def_count(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_weapon_def_custom_param {
            #[link(wasm_import_module = "spring:weapon-defs")]
            unsafe extern "C" {
                #[link_name = "get-weapon-def-custom-param"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:weapon-defs.get-weapon-def-custom-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_weapon_def_custom_param(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_weapon_def_custom_param::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_weapon_def_custom_param_keys(weapon_def_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::weapon_defs::get_weapon_def_custom_param_keys(weapon_def_id, &mut __output) {
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
        pub fn get_weapon_def_damage(weapon_def_id: i32) -> Result<f32> {
            let value = crate::generated::weapon_defs::get_weapon_def_damage(weapon_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_weapon_def_id(weapon_def_name: &str) -> Result<i32> {
            let mut weapon_def_name_bytes = weapon_def_name.as_bytes().to_vec();
            if weapon_def_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            weapon_def_name_bytes.push(0);
            let weapon_def_name_cstr = core::ffi::CStr::from_bytes_with_nul(&weapon_def_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::weapon_defs::get_weapon_def_id(weapon_def_name_cstr)
        }

        #[inline]
        pub fn get_weapon_def_i_ds(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_weapon_def_i_ds::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn get_weapon_def_name(weapon_def_id: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_weapon_def_name::call(weapon_def_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (weapon_def_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_weapon_def_range(weapon_def_id: i32) -> Result<f32> {
            let value = crate::generated::weapon_defs::get_weapon_def_range(weapon_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn valid_weapon_def_id(weapon_def_id: i32) -> Result<bool> {
            let value = crate::generated::weapon_defs::valid_weapon_def_id(weapon_def_id)?;
            Ok(value)
        }

    }


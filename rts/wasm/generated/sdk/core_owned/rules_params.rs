    pub mod rules_params {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum RulesParamLOS {
            RulesparamlosAllied,
            RulesparamlosAlliedMask,
            RulesparamlosInlos,
            RulesparamlosInlosMask,
            RulesparamlosInradar,
            RulesparamlosInradarMask,
            RulesparamlosPrivate,
            RulesparamlosPrivateMask,
            RulesparamlosPublic,
            RulesparamlosPublicMask,
            RulesparamlosTyped,
            RulesparamlosTypedMask,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum RulesParamType {
            RulesparamTypeBool,
            RulesparamTypeFloat,
            RulesparamTypeString,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRulesParamQuery {
            pub feature_id: i32,
            pub param_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRulesParamResult {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureRulesParamsQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRulesParamsResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesParamQuery {
            pub param_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesParamResult {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameRulesParamsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesParamsResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerRulesParamQuery {
            pub player_id: i32,
            pub param_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerRulesParamResult {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPlayerRulesParamsQuery {
            pub player_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerRulesParamsResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamRulesParamQuery {
            pub team_id: i32,
            pub param_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamRulesParamResult {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamRulesParamsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamRulesParamsResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitRulesParamQuery {
            pub unit_id: i32,
            pub param_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitRulesParamResult {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitRulesParamsQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitRulesParamsResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RulesParamValue {
            pub type_: RulesParamType,
            pub bool_value: bool,
            pub float_value: f32,
            pub string_value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetFeatureRulesParamQuery {
            pub feature_id: i32,
            pub param_name: String,
            pub value: RulesParamValue,
            pub los: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetFeatureRulesParamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGameRulesParamQuery {
            pub param_name: String,
            pub value: RulesParamValue,
            pub los: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetGameRulesParamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetPlayerRulesParamQuery {
            pub player_id: i32,
            pub param_name: String,
            pub value: RulesParamValue,
            pub los: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetPlayerRulesParamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetTeamRulesParamQuery {
            pub team_id: i32,
            pub param_name: String,
            pub value: RulesParamValue,
            pub los: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTeamRulesParamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitRulesParamQuery {
            pub unit_id: i32,
            pub param_name: String,
            pub value: RulesParamValue,
            pub los: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitRulesParamResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeatureRulesParamValue {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameRulesParamValue {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPlayerRulesParamValue {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamRulesParamValue {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitRulesParamValue {
            pub value: RulesParamValue,
            pub los: i32,
            pub exists: bool,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_feature_rules_param {
            #[link(wasm_import_module = "spring:rules-params")]
            unsafe extern "C" {
                #[link_name = "get-feature-rules-param"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rules-params.get-feature-rules-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_feature_rules_param(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_feature_rules_param::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_feature_rules_params(feature_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::rules_params::get_feature_rules_params(feature_id, &mut __output) {
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_game_rules_param {
            #[link(wasm_import_module = "spring:rules-params")]
            unsafe extern "C" {
                #[link_name = "get-game-rules-param"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rules-params.get-game-rules-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_game_rules_param(p0: i32, p1: i32) -> i32 {
            __core_owned_get_game_rules_param::call(p0, p1)
        }

        #[inline]
        pub fn get_game_rules_params(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::rules_params::get_game_rules_params(unused as i32, &mut __output) {
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_player_rules_param {
            #[link(wasm_import_module = "spring:rules-params")]
            unsafe extern "C" {
                #[link_name = "get-player-rules-param"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rules-params.get-player-rules-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_player_rules_param(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_player_rules_param::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_player_rules_params(player_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::rules_params::get_player_rules_params(player_id, &mut __output) {
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_rules_param {
            #[link(wasm_import_module = "spring:rules-params")]
            unsafe extern "C" {
                #[link_name = "get-team-rules-param"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rules-params.get-team-rules-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_team_rules_param(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_team_rules_param::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_team_rules_params(team_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::rules_params::get_team_rules_params(team_id, &mut __output) {
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_rules_param {
            #[link(wasm_import_module = "spring:rules-params")]
            unsafe extern "C" {
                #[link_name = "get-unit-rules-param"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rules-params.get-unit-rules-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_rules_param(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_rules_param::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_rules_params(unit_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::rules_params::get_unit_rules_params(unit_id, &mut __output) {
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
        pub fn set_feature_rules_param(feature_id: i32, param_name: &str, value: &RulesParamValue, los: i32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + param_name.len()); __b.extend_from_slice(&(param_name.len() as u32).to_le_bytes()); __b.extend_from_slice(param_name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.type_ as i32).to_le_bytes()); __b.extend_from_slice(&(if value.bool_value { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&value.float_value.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.string_value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.string_value.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::rules_params::set_feature_rules_param(feature_id, los, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_game_rules_param(param_name: &str, value: &RulesParamValue, los: i32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + param_name.len()); __b.extend_from_slice(&(param_name.len() as u32).to_le_bytes()); __b.extend_from_slice(param_name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.type_ as i32).to_le_bytes()); __b.extend_from_slice(&(if value.bool_value { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&value.float_value.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.string_value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.string_value.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::rules_params::set_game_rules_param(los, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_player_rules_param(player_id: i32, param_name: &str, value: &RulesParamValue, los: i32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + param_name.len()); __b.extend_from_slice(&(param_name.len() as u32).to_le_bytes()); __b.extend_from_slice(param_name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.type_ as i32).to_le_bytes()); __b.extend_from_slice(&(if value.bool_value { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&value.float_value.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.string_value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.string_value.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::rules_params::set_player_rules_param(player_id, los, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_team_rules_param(team_id: i32, param_name: &str, value: &RulesParamValue, los: i32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + param_name.len()); __b.extend_from_slice(&(param_name.len() as u32).to_le_bytes()); __b.extend_from_slice(param_name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.type_ as i32).to_le_bytes()); __b.extend_from_slice(&(if value.bool_value { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&value.float_value.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.string_value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.string_value.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::rules_params::set_team_rules_param(team_id, los, &__blob0, &__blob1)
        }

        #[inline]
        pub fn set_unit_rules_param(unit_id: i32, param_name: &str, value: &RulesParamValue, los: i32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + param_name.len()); __b.extend_from_slice(&(param_name.len() as u32).to_le_bytes()); __b.extend_from_slice(param_name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.type_ as i32).to_le_bytes()); __b.extend_from_slice(&(if value.bool_value { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&value.float_value.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(value.string_value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.string_value.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::rules_params::set_unit_rules_param(unit_id, los, &__blob0, &__blob1)
        }

    }


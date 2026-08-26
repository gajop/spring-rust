    pub mod unit_defs {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum UnitDefParamType {
            UnitDefParamBool,
            UnitDefParamFloat,
            UnitDefParamInt,
            UnitDefParamMissing,
            UnitDefParamString,
            UnitDefParamTable,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefByIDQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefByIDResult {
            pub exists: bool,
            pub basic: UnitDefBasicInfo,
            pub costs: UnitDefCosts,
            pub physics: UnitDefPhysics,
            pub weapons: UnitDefWeapons,
            pub build_options: UnitDefBuildOptions,
            pub sensors: UnitDefSensors,
            pub health: UnitDefHealth,
            pub classify: UnitDefClassify,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefClassifyQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefClassifyResult {
            pub classify: UnitDefClassify,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefCostsQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefCostsResult {
            pub costs: UnitDefCosts,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefCustomParamKeysQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCustomParamKeysResult {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCustomParamQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefCustomParamResult {
            pub value: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefHealthQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefHealthResult {
            pub health: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefHumanNameQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefHumanNameResult {
            pub human_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefIDByNameQuery {
            pub unit_def_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefIDByNameResult {
            pub id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefIDsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefIDsResult {
            pub ids: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefNameQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamBoolQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefParamBoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamFloatQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefParamFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamIntQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefParamIntResult {
            pub value: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefParamKeysQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamKeysResult {
            pub keys: Vec<UnitDefParamKey>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamStringQuery {
            pub unit_def_id: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamStringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefParamTypeQuery {
            pub key: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefParamTypeResult {
            pub type_: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefSpeedQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitDefSpeedResult {
            pub speed: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefBasicInfo {
            pub id: i32,
            pub name: String,
            pub human_name: String,
            pub tooltip: String,
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefBuildOptions {
            pub buildable_unit_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDefClassify {
            pub is_transport: bool,
            pub is_immobile: bool,
            pub is_building: bool,
            pub is_builder: bool,
            pub is_mobile_builder: bool,
            pub is_static_builder: bool,
            pub is_factory: bool,
            pub is_extractor: bool,
            pub is_ground_unit: bool,
            pub is_air_unit: bool,
            pub is_strafing_air_unit: bool,
            pub is_hovering_air_unit: bool,
            pub is_fighter_air_unit: bool,
            pub is_bomber_air_unit: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDefCosts {
            pub metal_cost: f32,
            pub energy_cost: f32,
            pub build_time: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDefHealth {
            pub health: f32,
            pub auto_heal: f32,
            pub idle_auto_heal: f32,
            pub idle_time: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefParamKey {
            pub name: String,
            pub type_: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDefPhysics {
            pub mass: f32,
            pub height: f32,
            pub radius: f32,
            pub speed: f32,
            pub turn_rate: f32,
            pub acceleration: f32,
            pub brake_rate: f32,
            pub can_fly: bool,
            pub can_move: bool,
            pub can_hover: bool,
            pub float_on_water: bool,
            pub move_def_id: i32,
            pub can_submerge: bool,
            pub waterline: f32,
            pub min_water_depth: f32,
            pub max_water_depth: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitDefSensors {
            pub los_radius: f32,
            pub air_los_radius: f32,
            pub radar_radius: f32,
            pub sonar_radius: f32,
            pub seismic_radius: f32,
            pub radar_jammer_radius: f32,
            pub sonar_jammer_radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitDefWeapons {
            pub weapon_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidUnitDefIDQuery {
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ValidUnitDefIDResult {
            pub valid: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_unit_def_i_ds {
            #[link(wasm_import_module = "spring:unit-defs")]
            unsafe extern "C" {
                #[link_name = "get-unit-def-i-ds"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitDefByIDValue {
            pub exists: bool,
            pub basic: UnitDefBasicInfo,
            pub costs: UnitDefCosts,
            pub physics: UnitDefPhysics,
            pub weapons: UnitDefWeapons,
            pub build_options: UnitDefBuildOptions,
            pub sensors: UnitDefSensors,
            pub health: UnitDefHealth,
            pub classify: UnitDefClassify,
        }

        #[inline]
        pub fn get_unit_def_by_id(unit_def_id: i32) -> Result<GetUnitDefByIDValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::unit_defs::get_unit_def_by_id(unit_def_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetUnitDefByIDValue {
                            exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                            basic: UnitDefBasicInfo { id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, human_name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, tooltip: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, unit_def_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            costs: UnitDefCosts { metal_cost: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, energy_cost: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, build_time: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            physics: UnitDefPhysics { mass: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, height: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, turn_rate: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, acceleration: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, brake_rate: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, can_fly: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, can_move: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, can_hover: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, float_on_water: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, move_def_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, can_submerge: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, waterline: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, min_water_depth: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, max_water_depth: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            weapons: UnitDefWeapons { weapon_def_i_ds: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } },
                            build_options: UnitDefBuildOptions { buildable_unit_def_i_ds: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } },
                            sensors: UnitDefSensors { los_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, air_los_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, radar_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, sonar_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, seismic_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, radar_jammer_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, sonar_jammer_radius: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            health: UnitDefHealth { health: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, auto_heal: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, idle_auto_heal: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, idle_time: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
                            classify: UnitDefClassify { is_transport: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_immobile: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_building: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_builder: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_mobile_builder: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_static_builder: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_factory: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_extractor: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_ground_unit: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_air_unit: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_strafing_air_unit: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_hovering_air_unit: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_fighter_air_unit: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, is_bomber_air_unit: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? },
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
        pub fn get_unit_def_classify(unit_def_id: i32) -> Result<UnitDefClassify> {
            let value = crate::generated::unit_defs::get_unit_def_classify(unit_def_id)?;
            Ok(UnitDefClassify { is_transport: value.is_transport, is_immobile: value.is_immobile, is_building: value.is_building, is_builder: value.is_builder, is_mobile_builder: value.is_mobile_builder, is_static_builder: value.is_static_builder, is_factory: value.is_factory, is_extractor: value.is_extractor, is_ground_unit: value.is_ground_unit, is_air_unit: value.is_air_unit, is_strafing_air_unit: value.is_strafing_air_unit, is_hovering_air_unit: value.is_hovering_air_unit, is_fighter_air_unit: value.is_fighter_air_unit, is_bomber_air_unit: value.is_bomber_air_unit })
        }

        #[inline]
        pub fn get_unit_def_costs(unit_def_id: i32) -> Result<UnitDefCosts> {
            let value = crate::generated::unit_defs::get_unit_def_costs(unit_def_id)?;
            Ok(UnitDefCosts { metal_cost: value.metal_cost, energy_cost: value.energy_cost, build_time: value.build_time })
        }

        #[inline]
        pub fn get_unit_def_count(unused: u8) -> Result<u32> {
            let value = crate::generated::unit_defs::get_unit_def_count(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_custom_param {
            #[link(wasm_import_module = "spring:unit-defs")]
            unsafe extern "C" {
                #[link_name = "get-unit-def-custom-param"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-custom-param."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_def_custom_param(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_def_custom_param::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_def_custom_param_keys(unit_def_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::unit_defs::get_unit_def_custom_param_keys(unit_def_id, &mut __output) {
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
        pub fn get_unit_def_health(unit_def_id: i32) -> Result<f32> {
            let value = crate::generated::unit_defs::get_unit_def_health(unit_def_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_human_name {
            #[link(wasm_import_module = "spring:unit-defs")]
            unsafe extern "C" {
                #[link_name = "get-unit-def-human-name"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-human-name."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_def_human_name(p0: i32, p1: i32, p2: i32) -> i64 {
            __core_owned_get_unit_def_human_name::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_def_id_by_name(unit_def_name: &str) -> Result<i32> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(unit_def_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(unit_def_name)?),
            };
            crate::generated::borrowed::unit_defs::get_unit_def_id_by_name(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn get_unit_def_i_ds(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_unit_def_i_ds::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_def_name {
            #[link(wasm_import_module = "spring:unit-defs")]
            unsafe extern "C" {
                #[link_name = "get-unit-def-name"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-name."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_def_name(p0: i32, p1: i32, p2: i32) -> i64 {
            __core_owned_get_unit_def_name::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_def_param_bool(unit_def_id: i32, key: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(key, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            crate::generated::borrowed::unit_defs::get_unit_def_param_bool(unit_def_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn get_unit_def_param_float(unit_def_id: i32, key: &str) -> Result<f32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(key, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            crate::generated::borrowed::unit_defs::get_unit_def_param_float(unit_def_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn get_unit_def_param_int(unit_def_id: i32, key: &str) -> Result<i32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(key, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            crate::generated::borrowed::unit_defs::get_unit_def_param_int(unit_def_id, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn get_unit_def_param_keys(unused: u8) -> Result<Vec<UnitDefParamKey>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::unit_defs::get_unit_def_param_keys(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(UnitDefParamKey { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, type_: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        mod __core_owned_get_unit_def_param_string {
            #[link(wasm_import_module = "spring:unit-defs")]
            unsafe extern "C" {
                #[link_name = "get-unit-def-param-string"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-defs.get-unit-def-param-string."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_def_param_string(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_def_param_string::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_def_param_type(key: &str) -> Result<i32> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(key, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            crate::generated::borrowed::unit_defs::get_unit_def_param_type(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn get_unit_def_speed(unit_def_id: i32) -> Result<f32> {
            let value = crate::generated::unit_defs::get_unit_def_speed(unit_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn valid_unit_def_id(unit_def_id: i32) -> Result<bool> {
            let value = crate::generated::unit_defs::valid_unit_def_id(unit_def_id)?;
            Ok(value)
        }

    }


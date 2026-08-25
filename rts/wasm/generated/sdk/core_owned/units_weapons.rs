    pub mod units_weapons {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMaxRangeQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitMaxRangeResult {
            pub max_range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponCanFireQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponCanFireResult {
            pub can_fire: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponCountQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponDamagesQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponDamagesResult {
            pub damages: UnitWeaponDamages,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponHaveFreeLineOfFireOptions {
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponHaveFreeLineOfFireQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_id: i32,
            pub source_pos: Float3,
            pub target_pos: Float3,
            pub options: GetUnitWeaponHaveFreeLineOfFireOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponHaveFreeLineOfFireResult {
            pub has_free_line_of_fire: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponStateQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub key: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponStateResult {
            pub state: UnitWeaponState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTargetQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTargetResult {
            pub target: UnitWeaponTarget,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTestRangeQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTestRangeResult {
            pub in_range: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTestTargetOptions {
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTestTargetQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_id: i32,
            pub target_pos: Float3,
            pub options: GetUnitWeaponTestTargetOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTestTargetResult {
            pub can_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTryTargetOptions {
            pub user_target: bool,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTryTargetQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
            pub target_id: i32,
            pub target_pos: Float3,
            pub options: GetUnitWeaponTryTargetOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponTryTargetResult {
            pub can_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitWeaponVectorsQuery {
            pub unit_id: i32,
            pub weapon_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitWeaponVectorsResult {
            pub vectors: UnitWeaponVectors,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitWeaponDamages {
            pub damages: Vec<f32>,
            pub paralyze_damage_time: f32,
            pub impulse_factor: f32,
            pub impulse_boost: f32,
            pub crater_mult: f32,
            pub crater_boost: f32,
            pub default_damage: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWeaponState {
            pub reload_time: f32,
            pub reload_frame: f32,
            pub range: f32,
            pub projectile_speed: f32,
            pub accuracy: f32,
            pub spray_angle: f32,
            pub aim_from_height: f32,
            pub salvo_size: f32,
            pub salvo_delay: f32,
            pub salvo_error: f32,
            pub target_move_error: f32,
            pub turn_rate: f32,
            pub auto_target: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWeaponTarget {
            pub target_type: i32,
            pub target_id: i32,
            pub target_pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UnitWeaponVectors {
            pub weapon_muzzle_pos: Float3,
            pub weapon_aim_pos: Float3,
            pub weapon_dir: Float3,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn get_unit_max_range(unit_id: i32) -> Result<f32> {
            let value = crate::generated::units_weapons::get_unit_max_range(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_can_fire(unit_id: i32, weapon_num: i32) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_can_fire(unit_id, weapon_num)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_count(unit_id: i32) -> Result<u32> {
            let value = crate::generated::units_weapons::get_unit_weapon_count(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_damages(unit_id: i32, weapon_num: i32) -> Result<UnitWeaponDamages> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_weapons::get_unit_weapon_damages(unit_id, weapon_num, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = UnitWeaponDamages { damages: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, paralyze_damage_time: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, impulse_factor: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, impulse_boost: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, crater_mult: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, crater_boost: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, default_damage: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? };
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
        pub fn get_unit_weapon_have_free_line_of_fire(unit_id: i32, weapon_num: i32, target_id: i32, source_pos: Float3, target_pos: Float3, options: GetUnitWeaponHaveFreeLineOfFireOptions) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_have_free_line_of_fire(unit_id, weapon_num, target_id, crate::generated::units_weapons::Float3 { x: source_pos.x, y: source_pos.y, z: source_pos.z }, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z }, crate::generated::units_weapons::GetUnitWeaponHaveFreeLineOfFireOptions { is_ground_target: options.is_ground_target })?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_state(unit_id: i32, weapon_num: i32, key: &str) -> Result<UnitWeaponState> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + key.len()); __b.extend_from_slice(&(key.len() as u32).to_le_bytes()); __b.extend_from_slice(key.as_bytes()); __b };
            let mut __output = [0u8; 52];
            crate::generated::dynamic_input::units_weapons::get_unit_weapon_state(unit_id, weapon_num, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(UnitWeaponState { reload_time: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, reload_frame: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, range: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, projectile_speed: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, accuracy: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, spray_angle: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, aim_from_height: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, salvo_size: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, salvo_delay: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, salvo_error: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, target_move_error: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, turn_rate: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, auto_target: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? })
        }

        #[inline]
        pub fn get_unit_weapon_target(unit_id: i32, weapon_num: i32) -> Result<UnitWeaponTarget> {
            let value = crate::generated::units_weapons::get_unit_weapon_target(unit_id, weapon_num)?;
            Ok(UnitWeaponTarget { target_type: value.target_type, target_id: value.target_id, target_pos: Float3 { x: value.target_pos.x, y: value.target_pos.y, z: value.target_pos.z } })
        }

        #[inline]
        pub fn get_unit_weapon_test_range(unit_id: i32, weapon_num: i32, target_pos: Float3) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_test_range(unit_id, weapon_num, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_test_target(unit_id: i32, weapon_num: i32, target_id: i32, target_pos: Float3, options: GetUnitWeaponTestTargetOptions) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_test_target(unit_id, weapon_num, target_id, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z }, crate::generated::units_weapons::GetUnitWeaponTestTargetOptions { is_ground_target: options.is_ground_target })?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_try_target(unit_id: i32, weapon_num: i32, target_id: i32, target_pos: Float3, options: GetUnitWeaponTryTargetOptions) -> Result<bool> {
            let value = crate::generated::units_weapons::get_unit_weapon_try_target(unit_id, weapon_num, target_id, crate::generated::units_weapons::Float3 { x: target_pos.x, y: target_pos.y, z: target_pos.z }, crate::generated::units_weapons::GetUnitWeaponTryTargetOptions { user_target: options.user_target, is_ground_target: options.is_ground_target })?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_weapon_vectors(unit_id: i32, weapon_num: i32) -> Result<UnitWeaponVectors> {
            let value = crate::generated::units_weapons::get_unit_weapon_vectors(unit_id, weapon_num)?;
            Ok(UnitWeaponVectors { weapon_muzzle_pos: Float3 { x: value.weapon_muzzle_pos.x, y: value.weapon_muzzle_pos.y, z: value.weapon_muzzle_pos.z }, weapon_aim_pos: Float3 { x: value.weapon_aim_pos.x, y: value.weapon_aim_pos.y, z: value.weapon_aim_pos.z }, weapon_dir: Float3 { x: value.weapon_dir.x, y: value.weapon_dir.y, z: value.weapon_dir.z } })
        }

    }


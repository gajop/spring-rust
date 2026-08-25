    pub mod markers {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddWorldIconQuery {
            pub cmd_id: i32,
            pub pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddWorldIconResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AddWorldTextQuery {
            pub text: String,
            pub pos: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddWorldTextResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddWorldUnitQuery {
            pub unit_def_id: i32,
            pub pos: Float3,
            pub team_id: i32,
            pub facing: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AddWorldUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MarkerAddLineQuery {
            pub from: Float3,
            pub to: Float3,
            pub local_only: bool,
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MarkerAddLineResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MarkerAddPointQuery {
            pub pos: Float3,
            pub text: String,
            pub local_only: bool,
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MarkerAddPointResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MarkerErasePositionOptions {
            pub local_only: bool,
            pub always_erase: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MarkerErasePositionQuery {
            pub pos: Float3,
            pub unused: f32,
            pub options: MarkerErasePositionOptions,
            pub player_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MarkerErasePositionResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn add_world_icon(cmd_id: i32, pos: Float3) -> Result<bool> {
            let value = crate::generated::markers::add_world_icon(cmd_id, crate::generated::markers::Float3 { x: pos.x, y: pos.y, z: pos.z })?;
            Ok(value)
        }

        #[inline]
        pub fn add_world_text(text: &str, pos: Float3) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + text.len()); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); __b.extend_from_slice(text.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::markers::add_world_text(&__blob0, &__blob1)
        }

        #[inline]
        pub fn add_world_unit(unit_def_id: i32, pos: Float3, team_id: i32, facing: i32) -> Result<bool> {
            let value = crate::generated::markers::add_world_unit(unit_def_id, crate::generated::markers::Float3 { x: pos.x, y: pos.y, z: pos.z }, team_id, facing)?;
            Ok(value)
        }

        #[inline]
        pub fn marker_add_line(from: Float3, to: Float3, local_only: bool, player_id: i32) -> Result<bool> {
            let value = crate::generated::markers::marker_add_line(crate::generated::markers::Float3 { x: from.x, y: from.y, z: from.z }, crate::generated::markers::Float3 { x: to.x, y: to.y, z: to.z }, local_only, player_id)?;
            Ok(value)
        }

        #[inline]
        pub fn marker_add_point(pos: Float3, text: &str, local_only: bool, player_id: i32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob1 = { let mut __b = Vec::with_capacity(4 + text.len()); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); __b.extend_from_slice(text.as_bytes()); __b };
            crate::generated::dynamic_input::markers::marker_add_point(local_only as i32, player_id, &__blob0, &__blob1)
        }

        #[inline]
        pub fn marker_erase_position(pos: Float3, unused: f32, options: MarkerErasePositionOptions, player_id: i32) -> Result<bool> {
            let value = crate::generated::markers::marker_erase_position(crate::generated::markers::Float3 { x: pos.x, y: pos.y, z: pos.z }, unused, crate::generated::markers::MarkerErasePositionOptions { local_only: options.local_only, always_erase: options.always_erase }, player_id)?;
            Ok(value)
        }

    }


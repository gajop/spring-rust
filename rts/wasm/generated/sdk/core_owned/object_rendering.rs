    pub mod object_rendering {
        use super::{Result, String, Vec};

        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ObjectRenderingMaterialType {
            ObjectRenderingAlpha = 0,
            ObjectRenderingAlphaReflect = 2,
            ObjectRenderingOpaque = 1,
            ObjectRenderingOpaqueReflect = 3,
            ObjectRenderingShadow = 4,
        }

        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ObjectRenderingObjectType {
            ObjectRenderingFeature = 1,
            ObjectRenderingUnit = 0,
        }

        #[repr(i32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ObjectRenderingUniformType {
            ObjectRenderingUniformFloat = 0,
            ObjectRenderingUniformFloat2 = 1,
            ObjectRenderingUniformFloat3 = 2,
            ObjectRenderingUniformFloat4 = 3,
            ObjectRenderingUniformFloatMat3 = 4,
            ObjectRenderingUniformFloatMat4 = 5,
            ObjectRenderingUniformInt = 6,
            ObjectRenderingUniformInt2 = 7,
            ObjectRenderingUniformInt3 = 8,
            ObjectRenderingUniformInt4 = 9,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ClearObjectMaterialUniformQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub material_type: ObjectRenderingMaterialType,
            pub lod_level: u32,
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct GetObjectLODQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetObjectLODResult {
            pub lod_count: u32,
            pub current_lod: u32,
        }

        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct ObjectMaterialDescriptor {
            pub shader_id: i32,
            pub shader: String,
            pub order: i32,
            pub use_camera: bool,
            pub culling: u32,
            pub texture_units: Vec<u32>,
            pub texture_names: Vec<String>,
            pub pre_list: u32,
            pub post_list: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ObjectMaterialUniform {
            pub name: String,
            pub type_: ObjectRenderingUniformType,
            pub float_values: Vec<f32>,
            pub int_values: Vec<i32>,
            pub value_count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ObjectRenderingResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct SetObjectLODCountQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub lod_count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct SetObjectLODDistanceQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub lod_level: u32,
            pub distance: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct SetObjectLODLengthQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub lod_level: u32,
            pub length: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct SetObjectMaterialDisplayListsQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub lod_level: u32,
            pub material_type: ObjectRenderingMaterialType,
            pub pre_list: u32,
            pub post_list: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct SetObjectMaterialLastLODQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub material_type: ObjectRenderingMaterialType,
            pub lod_level: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetObjectMaterialQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub lod_level: u32,
            pub material_type: ObjectRenderingMaterialType,
            pub material: ObjectMaterialDescriptor,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetObjectMaterialUniformQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub material_type: ObjectRenderingMaterialType,
            pub lod_level: u32,
            pub uniform: ObjectMaterialUniform,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct SetObjectPieceListQuery {
            pub object_type: ObjectRenderingObjectType,
            pub object_id: i32,
            pub lod_level: u32,
            pub piece: u32,
            pub display_list: u32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, GiveOrderArrayToUnitArrayQuery, GiveOrderArrayToUnitArrayResult, GiveOrderArrayToUnitQuery, GiveOrderArrayToUnitResult, GiveOrderToUnitArrayQuery, GiveOrderToUnitArrayResult, GiveOrderToUnitQuery, GiveOrderToUnitResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeCommand, NativeExplosionParams, NativeProjectileParams, NextFloatQuery, NextFloatResult, NextIntQuery, NextIntResult, NextIntUpToQuery, NextIntUpToResult, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SetSeedQuery, SetSeedResult, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLODValue {
            pub lod_count: u32,
            pub current_lod: u32,
        }

        #[inline]
        pub fn clear_deferred_material_uniform(object_type: ObjectRenderingObjectType, object_id: i32, material_type: ObjectRenderingMaterialType, lod_level: u32, name: &str) -> Result<bool> {
            let mut __core_string_4_scratch = [0u8; 256];
            let __core_string_4_buf = match super::write_cstr(name, &mut __core_string_4_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::object_rendering::clear_deferred_material_uniform(object_type as i32, object_id, material_type as i32, lod_level, __core_string_4_buf.as_cstr())
        }

        #[inline]
        pub fn clear_forward_material_uniform(object_type: ObjectRenderingObjectType, object_id: i32, material_type: ObjectRenderingMaterialType, lod_level: u32, name: &str) -> Result<bool> {
            let mut __core_string_4_scratch = [0u8; 256];
            let __core_string_4_buf = match super::write_cstr(name, &mut __core_string_4_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::object_rendering::clear_forward_material_uniform(object_type as i32, object_id, material_type as i32, lod_level, __core_string_4_buf.as_cstr())
        }

        #[inline]
        pub fn get_lod(object_type: ObjectRenderingObjectType, object_id: i32) -> Result<GetLODValue> {
            let value = crate::generated::object_rendering::get_lod(match object_type { ObjectRenderingObjectType::ObjectRenderingFeature => 1i32, ObjectRenderingObjectType::ObjectRenderingUnit => 0i32 }, object_id)?;
            Ok(GetLODValue {
                lod_count: value.0,
                current_lod: value.1
            })
        }

        #[inline]
        pub fn set_deferred_material_uniform(object_type: ObjectRenderingObjectType, object_id: i32, material_type: ObjectRenderingMaterialType, lod_level: u32, uniform: &ObjectMaterialUniform) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(uniform.name.len() as u32).to_le_bytes()); __b.extend_from_slice(uniform.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(uniform.type_ as i32).to_le_bytes()); for __i281 in 0..32usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&uniform.float_values.get(__i281).copied().unwrap_or_default().to_bits().to_le_bytes()); } for __i473 in 0..32usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&uniform.int_values.get(__i473).copied().unwrap_or_default().to_le_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&uniform.value_count.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::object_rendering::set_deferred_material_uniform(object_type as i32, object_id, material_type as i32, lod_level as i32, &__blob0)
        }

        #[inline]
        pub fn set_forward_material_uniform(object_type: ObjectRenderingObjectType, object_id: i32, material_type: ObjectRenderingMaterialType, lod_level: u32, uniform: &ObjectMaterialUniform) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(uniform.name.len() as u32).to_le_bytes()); __b.extend_from_slice(uniform.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(uniform.type_ as i32).to_le_bytes()); for __i281 in 0..32usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&uniform.float_values.get(__i281).copied().unwrap_or_default().to_bits().to_le_bytes()); } for __i473 in 0..32usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&uniform.int_values.get(__i473).copied().unwrap_or_default().to_le_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&uniform.value_count.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::object_rendering::set_forward_material_uniform(object_type as i32, object_id, material_type as i32, lod_level as i32, &__blob0)
        }

        #[inline]
        pub fn set_lod_count(object_type: ObjectRenderingObjectType, object_id: i32, lod_count: u32) -> Result<bool> {
            let value = crate::generated::object_rendering::set_lod_count(match object_type { ObjectRenderingObjectType::ObjectRenderingFeature => 1i32, ObjectRenderingObjectType::ObjectRenderingUnit => 0i32 }, object_id, lod_count)?;
            Ok(value)
        }

        #[inline]
        pub fn set_lod_distance(object_type: ObjectRenderingObjectType, object_id: i32, lod_level: u32, distance: f32) -> Result<bool> {
            let value = crate::generated::object_rendering::set_lod_distance(match object_type { ObjectRenderingObjectType::ObjectRenderingFeature => 1i32, ObjectRenderingObjectType::ObjectRenderingUnit => 0i32 }, object_id, lod_level, distance)?;
            Ok(value)
        }

        #[inline]
        pub fn set_lod_length(object_type: ObjectRenderingObjectType, object_id: i32, lod_level: u32, length: f32) -> Result<bool> {
            let value = crate::generated::object_rendering::set_lod_length(match object_type { ObjectRenderingObjectType::ObjectRenderingFeature => 1i32, ObjectRenderingObjectType::ObjectRenderingUnit => 0i32 }, object_id, lod_level, length)?;
            Ok(value)
        }

        #[inline]
        pub fn set_material(object_type: ObjectRenderingObjectType, object_id: i32, lod_level: u32, material_type: ObjectRenderingMaterialType, material: &ObjectMaterialDescriptor) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&material.shader_id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(material.shader.len() as u32).to_le_bytes()); __b.extend_from_slice(material.shader.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&material.order.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(if material.use_camera { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&material.culling.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(material.texture_units.len() as u32).to_le_bytes()); for __item in material.texture_units.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(material.texture_names.len() as u32).to_le_bytes()); for __item in material.texture_names.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.len() as u32).to_le_bytes()); __b.extend_from_slice(__item.as_bytes()); } while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&material.pre_list.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&material.post_list.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::object_rendering::set_material(object_type as i32, object_id, lod_level as i32, material_type as i32, &__blob0)
        }

        #[inline]
        pub fn set_material_display_lists(object_type: ObjectRenderingObjectType, object_id: i32, lod_level: u32, material_type: ObjectRenderingMaterialType, pre_list: u32, post_list: u32) -> Result<bool> {
            let value = crate::generated::object_rendering::set_material_display_lists(match object_type { ObjectRenderingObjectType::ObjectRenderingFeature => 1i32, ObjectRenderingObjectType::ObjectRenderingUnit => 0i32 }, object_id, lod_level, match material_type { ObjectRenderingMaterialType::ObjectRenderingAlpha => 0i32, ObjectRenderingMaterialType::ObjectRenderingAlphaReflect => 2i32, ObjectRenderingMaterialType::ObjectRenderingOpaque => 1i32, ObjectRenderingMaterialType::ObjectRenderingOpaqueReflect => 3i32, ObjectRenderingMaterialType::ObjectRenderingShadow => 4i32 }, pre_list, post_list)?;
            Ok(value)
        }

        #[inline]
        pub fn set_material_last_lod(object_type: ObjectRenderingObjectType, object_id: i32, material_type: ObjectRenderingMaterialType, lod_level: u32) -> Result<bool> {
            let value = crate::generated::object_rendering::set_material_last_lod(match object_type { ObjectRenderingObjectType::ObjectRenderingFeature => 1i32, ObjectRenderingObjectType::ObjectRenderingUnit => 0i32 }, object_id, match material_type { ObjectRenderingMaterialType::ObjectRenderingAlpha => 0i32, ObjectRenderingMaterialType::ObjectRenderingAlphaReflect => 2i32, ObjectRenderingMaterialType::ObjectRenderingOpaque => 1i32, ObjectRenderingMaterialType::ObjectRenderingOpaqueReflect => 3i32, ObjectRenderingMaterialType::ObjectRenderingShadow => 4i32 }, lod_level)?;
            Ok(value)
        }

        #[inline]
        pub fn set_piece_list(object_type: ObjectRenderingObjectType, object_id: i32, lod_level: u32, piece: u32, display_list: u32) -> Result<bool> {
            let value = crate::generated::object_rendering::set_piece_list(match object_type { ObjectRenderingObjectType::ObjectRenderingFeature => 1i32, ObjectRenderingObjectType::ObjectRenderingUnit => 0i32 }, object_id, lod_level, piece, display_list)?;
            Ok(value)
        }

    }


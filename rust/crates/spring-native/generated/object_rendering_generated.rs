impl<'a> ObjectRendering<'a> {
    pub fn get_lod(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32) -> Result<(u32, u32), Error> {
        unsafe {
            let query = sys::GetObjectLODQuery {
                objectType: object_type,
                objectID: object_id,
            };
            let mut result = MaybeUninit::<sys::GetObjectLODResult>::zeroed();
            let func = self.api.GetLOD.expect("GetLOD function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.lodCount,
                result.currentLOD,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn set_lodcount(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, lod_count: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectLODCountQuery {
                objectType: object_type,
                objectID: object_id,
                lodCount: lod_count,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetLODCount.expect("SetLODCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_lodlength(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, lod_level: u32, length: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectLODLengthQuery {
                objectType: object_type,
                objectID: object_id,
                lodLevel: lod_level,
                length,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetLODLength.expect("SetLODLength function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_loddistance(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, lod_level: u32, distance: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectLODDistanceQuery {
                objectType: object_type,
                objectID: object_id,
                lodLevel: lod_level,
                distance,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetLODDistance.expect("SetLODDistance function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_piece_list(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, lod_level: u32, piece: u32, display_list: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectPieceListQuery {
                objectType: object_type,
                objectID: object_id,
                lodLevel: lod_level,
                piece,
                displayList: display_list,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetPieceList.expect("SetPieceList function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_material(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, lod_level: u32, material_type: sys::ObjectRenderingMaterialType, material: sys::ObjectMaterialDescriptor) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectMaterialQuery {
                objectType: object_type,
                objectID: object_id,
                lodLevel: lod_level,
                materialType: material_type,
                material,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetMaterial.expect("SetMaterial function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_material_last_lod(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, material_type: sys::ObjectRenderingMaterialType, lod_level: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectMaterialLastLODQuery {
                objectType: object_type,
                objectID: object_id,
                materialType: material_type,
                lodLevel: lod_level,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetMaterialLastLOD.expect("SetMaterialLastLOD function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_material_display_lists(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, lod_level: u32, material_type: sys::ObjectRenderingMaterialType, pre_list: u32, post_list: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectMaterialDisplayListsQuery {
                objectType: object_type,
                objectID: object_id,
                lodLevel: lod_level,
                materialType: material_type,
                preList: pre_list,
                postList: post_list,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetMaterialDisplayLists.expect("SetMaterialDisplayLists function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_forward_material_uniform(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, material_type: sys::ObjectRenderingMaterialType, lod_level: u32, uniform: sys::ObjectMaterialUniform) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectMaterialUniformQuery {
                objectType: object_type,
                objectID: object_id,
                materialType: material_type,
                lodLevel: lod_level,
                uniform,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetForwardMaterialUniform.expect("SetForwardMaterialUniform function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_deferred_material_uniform(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, material_type: sys::ObjectRenderingMaterialType, lod_level: u32, uniform: sys::ObjectMaterialUniform) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetObjectMaterialUniformQuery {
                objectType: object_type,
                objectID: object_id,
                materialType: material_type,
                lodLevel: lod_level,
                uniform,
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.SetDeferredMaterialUniform.expect("SetDeferredMaterialUniform function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn clear_forward_material_uniform(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, material_type: sys::ObjectRenderingMaterialType, lod_level: u32, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::ClearObjectMaterialUniformQuery {
                objectType: object_type,
                objectID: object_id,
                materialType: material_type,
                lodLevel: lod_level,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.ClearForwardMaterialUniform.expect("ClearForwardMaterialUniform function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn clear_deferred_material_uniform(&self, object_type: sys::ObjectRenderingObjectType, object_id: i32, material_type: sys::ObjectRenderingMaterialType, lod_level: u32, name: &str) -> Result<bool, Error> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::invalid_argument("name"))?;
            let query = sys::ClearObjectMaterialUniformQuery {
                objectType: object_type,
                objectID: object_id,
                materialType: material_type,
                lodLevel: lod_level,
                name: name_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::ObjectRenderingResult>::zeroed();
            let func = self.api.ClearDeferredMaterialUniform.expect("ClearDeferredMaterialUniform function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

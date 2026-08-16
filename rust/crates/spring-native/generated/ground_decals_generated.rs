#[derive(Debug, Clone, Copy, Default)]
pub struct GetGroundDecalTexturesOptions {
    pub main_tex: Option<bool>,
    pub include_filenames: bool,
}

impl From<GetGroundDecalTexturesOptions> for sys::GetGroundDecalTexturesOptions {
    fn from(options: GetGroundDecalTexturesOptions) -> Self {
        sys::GetGroundDecalTexturesOptions {
            mainTex: options.main_tex.unwrap_or(false),
            hasMainTex: options.main_tex.is_some(),
            includeFilenames: options.include_filenames,
        }
    }
}

impl<'a> GroundDecals<'a> {
    pub fn create_ground_decal(&self) -> Result<(u32, bool), Error> {
        unsafe {
            let query = sys::CreateGroundDecalQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::CreateGroundDecalResult>::zeroed();
            let func = self.api.CreateGroundDecal.expect("CreateGroundDecal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.decalID,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn destroy_ground_decal(&self, decal_id: u32) -> Result<bool, Error> {
        unsafe {
            let query = sys::DestroyGroundDecalQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::DestroyGroundDecalResult>::zeroed();
            let func = self.api.DestroyGroundDecal.expect("DestroyGroundDecal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_all_ground_decals(&self) -> Result<Vec<u32>, Error> {
        unsafe {
            let query = sys::GetAllGroundDecalsQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetAllGroundDecalsResult>::zeroed();
            let func = self.api.GetAllGroundDecals.expect("GetAllGroundDecals function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.decalIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.decalIDs, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_ground_decal_type(&self, decal_id: u32) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetGroundDecalTypeQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalTypeResult>::zeroed();
            let func = self.api.GetGroundDecalType.expect("GetGroundDecalType function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.type_.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.type_).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_ground_decal_owner(&self, decal_id: u32) -> Result<(bool, i32), Error> {
        unsafe {
            let query = sys::GetGroundDecalOwnerQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalOwnerResult>::zeroed();
            let func = self.api.GetGroundDecalOwner.expect("GetGroundDecalOwner function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.hasOwner,
                result.ownerID,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_textures(&self, options: GetGroundDecalTexturesOptions) -> Result<(Vec<String>, Vec<String>), Error> {
        unsafe {
            let query = sys::GetGroundDecalTexturesQuery {
                options: options.into(),
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalTexturesResult>::zeroed();
            let func = self.api.GetGroundDecalTextures.expect("GetGroundDecalTextures function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                {
                    if result.textureCount == 0 || result.textures.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.textures, result.textureCount as usize);
                        slice.iter().map(|&ptr| {
                            if ptr.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(ptr).to_string_lossy().into_owned()
                            }
                        }).collect()
                    }
                },
                {
                    if result.filenameCount == 0 || result.filenames.is_null() {
                        Vec::new()
                    } else {
                        let slice = slice::from_raw_parts(result.filenames, result.filenameCount as usize);
                        slice.iter().map(|&ptr| {
                            if ptr.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(ptr).to_string_lossy().into_owned()
                            }
                        }).collect()
                    }
                },
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_texture(&self, decal_id: u32, main_tex: bool) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetGroundDecalTextureQuery {
                decalID: decal_id,
                mainTex: main_tex,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalTextureResult>::zeroed();
            let func = self.api.GetGroundDecalTexture.expect("GetGroundDecalTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.texture.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.texture).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn get_ground_decal_texture_params(&self, decal_id: u32) -> Result<(f32, f32), Error> {
        unsafe {
            let query = sys::GetGroundDecalTextureParamsQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalTextureParamsResult>::zeroed();
            let func = self.api.GetGroundDecalTextureParams.expect("GetGroundDecalTextureParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.texWrapDistance,
                result.texTraveledDistance,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_alpha(&self, decal_id: u32) -> Result<(f32, f32), Error> {
        unsafe {
            let query = sys::GetGroundDecalAlphaQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalAlphaResult>::zeroed();
            let func = self.api.GetGroundDecalAlpha.expect("GetGroundDecalAlpha function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.alpha,
                result.alphaFalloff,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_tint(&self, decal_id: u32) -> Result<[f32; 4], Error> {
        unsafe {
            let query = sys::GetGroundDecalTintQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalTintResult>::zeroed();
            let func = self.api.GetGroundDecalTint.expect("GetGroundDecalTint function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.tint
            })
        }
    }

    pub fn get_ground_decal_normal(&self, decal_id: u32) -> Result<[f32; 3], Error> {
        unsafe {
            let query = sys::GetGroundDecalNormalQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalNormalResult>::zeroed();
            let func = self.api.GetGroundDecalNormal.expect("GetGroundDecalNormal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.normal
            })
        }
    }

    pub fn get_ground_decal_glow_params(&self, decal_id: u32) -> Result<(f32, f32), Error> {
        unsafe {
            let query = sys::GetGroundDecalGlowParamsQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalGlowParamsResult>::zeroed();
            let func = self.api.GetGroundDecalGlowParams.expect("GetGroundDecalGlowParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.glow,
                result.glowFalloff,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_misc(&self, decal_id: u32) -> Result<(f32, f32, f32, f32, f32), Error> {
        unsafe {
            let query = sys::GetGroundDecalMiscQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalMiscResult>::zeroed();
            let func = self.api.GetGroundDecalMisc.expect("GetGroundDecalMisc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.dotElimExp,
                result.refHeight,
                result.minHeight,
                result.maxHeight,
                result.forceHeightMode,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_creation_frame(&self, decal_id: u32) -> Result<(f32, f32), Error> {
        unsafe {
            let query = sys::GetGroundDecalCreationFrameQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalCreationFrameResult>::zeroed();
            let func = self.api.GetGroundDecalCreationFrame.expect("GetGroundDecalCreationFrame function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.creationFrameMin,
                result.creationFrameMax,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_user_data(&self, decal_id: u32, quad_index: u32) -> Result<([f32; 4], bool), Error> {
        unsafe {
            let query = sys::GetGroundDecalUserDataQuery {
                decalID: decal_id,
                quadIndex: quad_index,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalUserDataResult>::zeroed();
            let func = self.api.GetGroundDecalUserData.expect("GetGroundDecalUserData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.values,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_middle_pos(&self, decal_id: u32) -> Result<([f32; 2], bool), Error> {
        unsafe {
            let query = sys::GetGroundDecalMiddlePosQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalMiddlePosResult>::zeroed();
            let func = self.api.GetGroundDecalMiddlePos.expect("GetGroundDecalMiddlePos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.midPos,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_quad_pos(&self, decal_id: u32) -> Result<([f32; 8], bool), Error> {
        unsafe {
            let query = sys::GetGroundDecalQuadPosQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalQuadPosResult>::zeroed();
            let func = self.api.GetGroundDecalQuadPos.expect("GetGroundDecalQuadPos function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.positions,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_rotation(&self, decal_id: u32) -> Result<(f32, bool), Error> {
        unsafe {
            let query = sys::GetGroundDecalRotationQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalRotationResult>::zeroed();
            let func = self.api.GetGroundDecalRotation.expect("GetGroundDecalRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.rotation,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_ground_decal_size_and_height(&self, decal_id: u32) -> Result<(f32, f32, f32, bool), Error> {
        unsafe {
            let query = sys::GetGroundDecalSizeAndHeightQuery {
                decalID: decal_id,
            };
            let mut result = MaybeUninit::<sys::GetGroundDecalSizeAndHeightResult>::zeroed();
            let func = self.api.GetGroundDecalSizeAndHeight.expect("GetGroundDecalSizeAndHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.sizeX,
                result.sizeZ,
                result.height,
                result.success,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn set_ground_decal_pos_and_dims(&self, decal_id: u32, mid_pos_x: f32, mid_pos_z: f32, size_x: f32, size_z: f32, proj_cube_height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalPosAndDimsQuery {
                decalID: decal_id,
                midPosX: mid_pos_x,
                midPosZ: mid_pos_z,
                sizeX: size_x,
                sizeZ: size_z,
                projCubeHeight: proj_cube_height,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalPosAndDimsResult>::zeroed();
            let func = self.api.SetGroundDecalPosAndDims.expect("SetGroundDecalPosAndDims function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_quad_pos_and_height(&self, decal_id: u32, pos_tlx: f32, pos_tly: f32, pos_trx: f32, pos_try: f32, pos_brx: f32, pos_bry: f32, pos_blx: f32, pos_bly: f32, proj_cube_height: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalQuadPosAndHeightQuery {
                decalID: decal_id,
                posTLX: pos_tlx,
                posTLY: pos_tly,
                posTRX: pos_trx,
                posTRY: pos_try,
                posBRX: pos_brx,
                posBRY: pos_bry,
                posBLX: pos_blx,
                posBLY: pos_bly,
                projCubeHeight: proj_cube_height,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalQuadPosAndHeightResult>::zeroed();
            let func = self.api.SetGroundDecalQuadPosAndHeight.expect("SetGroundDecalQuadPosAndHeight function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_rotation(&self, decal_id: u32, rotation: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalRotationQuery {
                decalID: decal_id,
                rotation: rotation,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalRotationResult>::zeroed();
            let func = self.api.SetGroundDecalRotation.expect("SetGroundDecalRotation function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_texture(&self, decal_id: u32, texture_name: &str, main_tex: bool) -> Result<bool, Error> {
        unsafe {
            let texture_name_cstr = std::ffi::CString::new(texture_name).map_err(|_| Error::invalid_argument("texture_name"))?;
            let query = sys::SetGroundDecalTextureQuery {
                decalID: decal_id,
                textureName: texture_name_cstr.as_ptr(),
                mainTex: main_tex,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalTextureResult>::zeroed();
            let func = self.api.SetGroundDecalTexture.expect("SetGroundDecalTexture function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_texture_params(&self, decal_id: u32, tex_wrap_distance: f32, tex_traveled_distance: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalTextureParamsQuery {
                decalID: decal_id,
                texWrapDistance: tex_wrap_distance,
                texTraveledDistance: tex_traveled_distance,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalTextureParamsResult>::zeroed();
            let func = self.api.SetGroundDecalTextureParams.expect("SetGroundDecalTextureParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_alpha(&self, decal_id: u32, alpha: f32, alpha_falloff: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalAlphaQuery {
                decalID: decal_id,
                alpha: alpha,
                alphaFalloff: alpha_falloff,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalAlphaResult>::zeroed();
            let func = self.api.SetGroundDecalAlpha.expect("SetGroundDecalAlpha function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_tint(&self, decal_id: u32, tint_r: f32, tint_g: f32, tint_b: f32, tint_a: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalTintQuery {
                decalID: decal_id,
                tintR: tint_r,
                tintG: tint_g,
                tintB: tint_b,
                tintA: tint_a,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalTintResult>::zeroed();
            let func = self.api.SetGroundDecalTint.expect("SetGroundDecalTint function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_normal(&self, decal_id: u32, normal_x: f32, normal_y: f32, normal_z: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalNormalQuery {
                decalID: decal_id,
                normalX: normal_x,
                normalY: normal_y,
                normalZ: normal_z,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalNormalResult>::zeroed();
            let func = self.api.SetGroundDecalNormal.expect("SetGroundDecalNormal function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_glow_params(&self, decal_id: u32, glow: f32, glow_falloff: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalGlowParamsQuery {
                decalID: decal_id,
                glow: glow,
                glowFalloff: glow_falloff,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalGlowParamsResult>::zeroed();
            let func = self.api.SetGroundDecalGlowParams.expect("SetGroundDecalGlowParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_misc(&self, decal_id: u32, dot_elim_exp: f32, ref_height: f32, min_height: f32, max_height: f32, force_height_mode: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalMiscQuery {
                decalID: decal_id,
                dotElimExp: dot_elim_exp,
                refHeight: ref_height,
                minHeight: min_height,
                maxHeight: max_height,
                forceHeightMode: force_height_mode,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalMiscResult>::zeroed();
            let func = self.api.SetGroundDecalMisc.expect("SetGroundDecalMisc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_creation_frame(&self, decal_id: u32, creation_frame_min: f32, creation_frame_max: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalCreationFrameQuery {
                decalID: decal_id,
                creationFrameMin: creation_frame_min,
                creationFrameMax: creation_frame_max,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalCreationFrameResult>::zeroed();
            let func = self.api.SetGroundDecalCreationFrame.expect("SetGroundDecalCreationFrame function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn set_ground_decal_user_data(&self, decal_id: u32, quad_index: u32, value_x: f32, value_y: f32, value_z: f32, value_w: f32) -> Result<bool, Error> {
        unsafe {
            let query = sys::SetGroundDecalUserDataQuery {
                decalID: decal_id,
                quadIndex: quad_index,
                valueX: value_x,
                valueY: value_y,
                valueZ: value_z,
                valueW: value_w,
            };
            let mut result = MaybeUninit::<sys::SetGroundDecalUserDataResult>::zeroed();
            let func = self.api.SetGroundDecalUserData.expect("SetGroundDecalUserData function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

}

use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_ground_decal(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let name = base_test_name(label);
        match name {
            "ground_decal_create" => {
                let (decal_id, success) = self
                    .interface
                    .ground_decals()
                    .create_ground_decal()
                    .map_err(|err| format!("create_ground_decal() failed: {err:?}"))?;
                // The Lua call already created a decal in the native run.
                // Exercise the native create call, but remove its temporary
                // result so it does not contaminate later list comparisons.
                if success && decal_id > 0 {
                    self.interface
                        .ground_decals()
                        .destroy_ground_decal(decal_id)
                        .map_err(|err| format!("destroy_ground_decal() failed: {err:?}"))?;
                }
                self.same_bool_if_present(label, message, "created", success && decal_id > 0)
            }
            "ground_decal_all" | "ground_decal_destroy" => {
                let native = self
                    .interface
                    .ground_decals()
                    .get_all_ground_decals()
                    .map_err(|err| format!("get_all_ground_decals() failed: {err:?}"))?;
                let native = native.into_iter().map(|id| id as i32).collect::<Vec<_>>();
                self.same_i32_list_if_present(label, message, "decalIDs", &native)
            }
            "ground_decal_type" => {
                let native = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_type(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_type() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "decalType",
                    native.as_deref().unwrap_or(""),
                )
            }
            "ground_decal_owner" => {
                let (has_owner, owner_id) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_owner(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_owner() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasOwner", has_owner)?;
                self.same_i32_if_present(label, message, "ownerID", owner_id)
            }
            "ground_decal_textures_all" | "ground_decal_textures_main_with_files" => {
                let (textures, filenames) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_textures(spring_native::GetGroundDecalTexturesOptions {
                        main_tex: bool_field(message, "hasMainTex")?
                            .then(|| bool_field(message, "mainTex"))
                            .transpose()?,
                        include_filenames: bool_field(message, "includeFilenames")?,
                    })
                    .map_err(|err| format!("get_ground_decal_textures() failed: {err:?}"))?;
                self.same_string_set_if_present(label, message, "textures", &textures)?;
                self.same_string_set_if_present(label, message, "filenames", &filenames)
            }
            "ground_decal_texture" | "ground_decal_texture_after_set" => {
                let native = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_texture(
                        u32_field(message, "decalID")?,
                        bool_field(message, "mainTex")?,
                    )
                    .map_err(|err| format!("get_ground_decal_texture() failed: {err:?}"))?;
                self.same_string_if_present(
                    label,
                    message,
                    "texture",
                    native.as_deref().unwrap_or(""),
                )
            }
            "ground_decal_texture_params" | "ground_decal_texture_params_after_set" => {
                let (wrap, travelled) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_texture_params(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_texture_params() failed: {err:?}"))?;
                self.same_if_present(label, message, "wrap", wrap)?;
                self.same_if_present(label, message, "travelled", travelled)
            }
            "ground_decal_alpha" | "ground_decal_alpha_after_set" => {
                let (alpha, falloff) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_alpha(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_alpha() failed: {err:?}"))?;
                self.same_if_present(label, message, "alpha", alpha)?;
                self.same_if_present(label, message, "falloff", falloff)
            }
            "ground_decal_tint" | "ground_decal_tint_after_set" => {
                let native = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_tint(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_tint() failed: {err:?}"))?;
                self.same_if_present(label, message, "r", native[0])?;
                self.same_if_present(label, message, "g", native[1])?;
                self.same_if_present(label, message, "b", native[2])?;
                self.same_if_present(label, message, "a", native[3])
            }
            "ground_decal_normal" | "ground_decal_normal_after_set" => {
                let native = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_normal(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_normal() failed: {err:?}"))?;
                self.same_if_present(label, message, "x", native[0])?;
                self.same_if_present(label, message, "y", native[1])?;
                self.same_if_present(label, message, "z", native[2])
            }
            "ground_decal_glow" | "ground_decal_glow_after_set" => {
                let (glow, falloff) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_glow_params(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_glow_params() failed: {err:?}"))?;
                self.same_if_present(label, message, "glow", glow)?;
                self.same_if_present(label, message, "falloff", falloff)
            }
            "ground_decal_misc" | "ground_decal_misc_after_set" => {
                let (dot, reference, min, max, mode) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_misc(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_misc() failed: {err:?}"))?;
                self.same_if_present(label, message, "dot", dot)?;
                self.same_if_present(label, message, "reference", reference)?;
                self.same_if_present(label, message, "min", min)?;
                self.same_if_present(label, message, "max", max)?;
                self.same_if_present(label, message, "mode", mode)
            }
            "ground_decal_creation_frame" | "ground_decal_creation_frame_after_set" => {
                let (min, max) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_creation_frame(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_creation_frame() failed: {err:?}"))?;
                self.same_if_present(label, message, "min", min)?;
                self.same_if_present(label, message, "max", max)
            }
            "ground_decal_user_data" | "ground_decal_user_data_after_set" => {
                let (native, success) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_user_data(
                        u32_field(message, "decalID")?,
                        u32_field(message, "quadIndex")?,
                    )
                    .map_err(|err| format!("get_ground_decal_user_data() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", success)?;
                self.same_if_present(label, message, "x", native[0])?;
                self.same_if_present(label, message, "y", native[1])?;
                self.same_if_present(label, message, "z", native[2])?;
                self.same_if_present(label, message, "w", native[3])
            }
            "ground_decal_middle" => {
                let (native, success) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_middle_pos(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_middle_pos() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", success)?;
                self.same_if_present(label, message, "x", native[0])?;
                self.same_if_present(label, message, "z", native[1])
            }
            "ground_decal_quad" | "ground_decal_quad_after_set" => {
                let (native, _success) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_quad_pos(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_quad_pos() failed: {err:?}"))?;
                for (index, field) in ["tlX", "tlZ", "trX", "trZ", "brX", "brZ", "blX", "blZ"]
                    .into_iter()
                    .enumerate()
                {
                    self.same_if_present(label, message, field, native[index])?;
                }
                Ok(())
            }
            "ground_decal_rotation" | "ground_decal_rotation_after_set" => {
                let (rotation, success) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_rotation(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_rotation() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", success)?;
                self.same_if_present(label, message, "rotation", rotation)
            }
            "ground_decal_size" | "ground_decal_pos_after_set" => {
                let (size_x, size_z, height, success) = self
                    .interface
                    .ground_decals()
                    .get_ground_decal_size_and_height(u32_field(message, "decalID")?)
                    .map_err(|err| format!("get_ground_decal_size_and_height() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", success)?;
                self.same_if_present(label, message, "sizeX", size_x)?;
                self.same_if_present(label, message, "sizeZ", size_z)?;
                self.same_if_present(label, message, "height", height)
            }
            _ => Err(format!("unsupported ground decal check `{name}`")),
        }
    }

    pub(crate) fn set_ground_decal(&mut self, message: &Value) -> Result<(), String> {
        let name = base_test_name(test_name_field(message)?);
        let id = u32_field(message, "decalID")?;
        let decals = self.interface.ground_decals();
        match name {
            "ground_decal_destroy" => {
                decals
                    .destroy_ground_decal(id)
                    .map_err(|err| format!("destroy_ground_decal() failed: {err:?}"))?;
            }
            "ground_decal_pos_after_set" => {
                decals
                    .set_ground_decal_pos_and_dims(
                        id,
                        f32_field(message, "midX")?,
                        f32_field(message, "midZ")?,
                        f32_field(message, "sizeX")?,
                        f32_field(message, "sizeZ")?,
                        f32_field(message, "height")?,
                    )
                    .map_err(|err| format!("set_ground_decal_pos_and_dims() failed: {err:?}"))?;
            }
            "ground_decal_quad_after_set" => {
                decals
                    .set_ground_decal_quad_pos_and_height(
                        id,
                        f32_field(message, "tlX")?,
                        f32_field(message, "tlZ")?,
                        f32_field(message, "trX")?,
                        f32_field(message, "trZ")?,
                        f32_field(message, "brX")?,
                        f32_field(message, "brZ")?,
                        f32_field(message, "blX")?,
                        f32_field(message, "blZ")?,
                        f32_field(message, "height")?,
                    )
                    .map_err(|err| {
                        format!("set_ground_decal_quad_pos_and_height() failed: {err:?}")
                    })?;
            }
            "ground_decal_rotation_after_set" => decals
                .set_ground_decal_rotation(id, f32_field(message, "rotation")?)
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_rotation() failed: {err:?}"))?,
            "ground_decal_texture_after_set" => decals
                .set_ground_decal_texture(
                    id,
                    str_field(message, "textureName")?,
                    bool_field(message, "mainTex")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_texture() failed: {err:?}"))?,
            "ground_decal_texture_params_after_set" => decals
                .set_ground_decal_texture_params(
                    id,
                    f32_field(message, "wrap")?,
                    f32_field(message, "travelled")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_texture_params() failed: {err:?}"))?,
            "ground_decal_alpha_after_set" => decals
                .set_ground_decal_alpha(
                    id,
                    f32_field(message, "alpha")?,
                    f32_field(message, "falloff")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_alpha() failed: {err:?}"))?,
            "ground_decal_tint_after_set" => decals
                .set_ground_decal_tint(
                    id,
                    f32_field(message, "r")?,
                    f32_field(message, "g")?,
                    f32_field(message, "b")?,
                    f32_field(message, "a")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_tint() failed: {err:?}"))?,
            "ground_decal_normal_after_set" => decals
                .set_ground_decal_normal(
                    id,
                    f32_field(message, "x")?,
                    f32_field(message, "y")?,
                    f32_field(message, "z")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_normal() failed: {err:?}"))?,
            "ground_decal_glow_after_set" => decals
                .set_ground_decal_glow_params(
                    id,
                    f32_field(message, "glow")?,
                    f32_field(message, "falloff")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_glow_params() failed: {err:?}"))?,
            "ground_decal_misc_after_set" => decals
                .set_ground_decal_misc(
                    id,
                    f32_field(message, "dot")?,
                    f32_field(message, "reference")?,
                    f32_field(message, "min")?,
                    f32_field(message, "max")?,
                    f32_field(message, "mode")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_misc() failed: {err:?}"))?,
            "ground_decal_creation_frame_after_set" => decals
                .set_ground_decal_creation_frame(
                    id,
                    f32_field(message, "min")?,
                    f32_field(message, "max")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_creation_frame() failed: {err:?}"))?,
            "ground_decal_user_data_after_set" => decals
                .set_ground_decal_user_data(
                    id,
                    u32_field(message, "quadIndex")?,
                    f32_field(message, "x")?,
                    f32_field(message, "y")?,
                    f32_field(message, "z")?,
                    f32_field(message, "w")?,
                )
                .map(|_| ())
                .map_err(|err| format!("set_ground_decal_user_data() failed: {err:?}"))?,
            _ => return Err(format!("unsupported ground decal setter `{name}`")),
        };
        Ok(())
    }
}

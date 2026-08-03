use super::*;
use crate::support::*;

fn require_success(label: &str, success: bool) -> Result<(), String> {
    if success {
        Ok(())
    } else {
        Err(format!("{label}: native call returned false"))
    }
}

impl NativeApiParity {
    pub(crate) fn check_render_control(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "add_unit_icon" => {
                let icon_name = str_field(message, "iconName")?;
                let tex_file = str_field(message, "texFile")?;
                let native_icon_name =
                    format!("{icon_name}__native_add_{}", i32_field(message, "case")?);
                let added = self
                    .interface
                    .icons()
                    .add_unit_icon(
                        &native_icon_name,
                        tex_file,
                        f32_field(message, "size")?,
                        f32_field(message, "distance")?,
                        bool_field(message, "radiusAdjust")?,
                        f32_field(message, "u0")?,
                        f32_field(message, "v0")?,
                        f32_field(message, "u1")?,
                        f32_field(message, "v1")?,
                    )
                    .map_err(|err| {
                        format!("add_unit_icon({native_icon_name:?}) failed: {err:?}")
                    })?;
                let result = self.same_bool_if_present(label, message, "added", added);
                let _ = self.interface.icons().free_unit_icon(&native_icon_name);
                result
            }
            "free_unit_icon" => {
                let icon_name = str_field(message, "iconName")?;
                let tex_file = str_field(message, "texFile")?;
                let native_icon_name =
                    format!("{icon_name}__native_free_{}", i32_field(message, "case")?);
                require_success(
                    "add_unit_icon",
                    self.interface
                        .icons()
                        .add_unit_icon(
                            &native_icon_name,
                            tex_file,
                            f32_field(message, "size")?,
                            f32_field(message, "distance")?,
                            bool_field(message, "radiusAdjust")?,
                            f32_field(message, "u0")?,
                            f32_field(message, "v0")?,
                            f32_field(message, "u1")?,
                            f32_field(message, "v1")?,
                        )
                        .map_err(|err| {
                            format!("add_unit_icon({native_icon_name:?}) failed: {err:?}")
                        })?,
                )?;
                let freed = self
                    .interface
                    .icons()
                    .free_unit_icon(&native_icon_name)
                    .map_err(|err| {
                        format!("free_unit_icon({native_icon_name:?}) failed: {err:?}")
                    })?;
                self.same_bool_if_present(label, message, "freed", freed)
            }
            "set_unit_icon" => {
                let unit_id = i32_field(message, "unitID")?;
                let icon_name = str_field(message, "iconName")?;
                require_success(
                    "set_unit_icon",
                    self.interface
                        .unsynced_ctrl()
                        .set_unit_icon(unit_id, icon_name)
                        .map_err(|err| format!("set_unit_icon({unit_id}) failed: {err:?}"))?,
                )?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "add_world_icon" => {
                let success = self
                    .interface
                    .markers()
                    .add_world_icon(
                        i32_field(message, "cmdID")?,
                        vec3_from_fields(message, "x", "y", "z")?,
                    )
                    .map_err(|err| format!("add_world_icon() failed: {err:?}"))?;
                require_success("add_world_icon", success)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "add_world_text" => {
                let success = self
                    .interface
                    .markers()
                    .add_world_text(
                        str_field(message, "text")?,
                        vec3_from_fields(message, "x", "y", "z")?,
                    )
                    .map_err(|err| format!("add_world_text() failed: {err:?}"))?;
                require_success("add_world_text", success)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "add_world_unit" => {
                let success = self
                    .interface
                    .markers()
                    .add_world_unit(
                        i32_field(message, "unitDefID")?,
                        vec3_from_fields(message, "x", "y", "z")?,
                        i32_field(message, "teamID")?,
                        i32_field(message, "facing")?,
                    )
                    .map_err(|err| format!("add_world_unit() failed: {err:?}"))?;
                require_success("add_world_unit", success)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "marker_add_point" => {
                let success = self
                    .interface
                    .markers()
                    .marker_add_point(
                        vec3_from_fields(message, "x", "y", "z")?,
                        str_field(message, "text")?,
                        bool_field(message, "localOnly")?,
                        i32_field(message, "playerID")?,
                    )
                    .map_err(|err| format!("marker_add_point() failed: {err:?}"))?;
                require_success("marker_add_point", success)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "marker_add_line" => {
                let success = self
                    .interface
                    .markers()
                    .marker_add_line(
                        vec3_from_fields(message, "x1", "y1", "z1")?,
                        vec3_from_fields(message, "x2", "y2", "z2")?,
                        bool_field(message, "localOnly")?,
                        i32_field(message, "playerID")?,
                    )
                    .map_err(|err| format!("marker_add_line() failed: {err:?}"))?;
                require_success("marker_add_line", success)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            "marker_erase_position" => {
                let success = self
                    .interface
                    .markers()
                    .marker_erase_position(
                        vec3_from_fields(message, "x", "y", "z")?,
                        f32_field(message, "unused")?,
                        spring_native::MarkerErasePositionOptions {
                            local_only: bool_field(message, "localOnly")?,
                            always_erase: bool_field(message, "alwaysErase")?,
                        },
                        i32_field(message, "playerID")?,
                    )
                    .map_err(|err| format!("marker_erase_position() failed: {err:?}"))?;
                require_success("marker_erase_position", success)?;
                self.same_i32_if_present(label, message, "returnCount", 0)
            }
            name => Err(format!("unsupported render control check `{name}`")),
        }
    }

    pub(crate) fn set_render_control(&mut self, message: &Value) -> Result<(), String> {
        match base_test_name(test_name_field(message)?) {
            "set_unit_icon" => {
                let unit_id = i32_field(message, "unitID")?;
                let icon_name = str_field(message, "iconName")?;
                require_success(
                    "set_unit_icon",
                    self.interface
                        .unsynced_ctrl()
                        .set_unit_icon(unit_id, icon_name)
                        .map_err(|err| format!("set_unit_icon({unit_id}) failed: {err:?}"))?,
                )
            }
            name => Err(format!("unsupported render control setter `{name}`")),
        }
    }
}

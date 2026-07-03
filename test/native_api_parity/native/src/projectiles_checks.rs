use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_projectiles_list(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let test_name = base_test_name(label);
        let synced = bool_field(message, "synced")?;
        let weapon = bool_field(message, "weapon")?;
        let native = match test_name {
            "get_all_projectiles" => self
                .interface
                .projectiles()
                .get_all_projectiles(synced, weapon)
                .map_err(|err| {
                    format!("get_all_projectiles({synced}, {weapon}) failed: {err:?}")
                })?,
            "get_projectiles_in_rectangle" => self
                .interface
                .projectiles()
                .get_projectiles_in_rectangle(
                    f32_field(message, "minX")?,
                    f32_field(message, "minZ")?,
                    f32_field(message, "maxX")?,
                    f32_field(message, "maxZ")?,
                    synced,
                    weapon,
                )
                .map_err(|err| format!("get_projectiles_in_rectangle() failed: {err:?}"))?,
            "get_projectiles_in_sphere" => self
                .interface
                .projectiles()
                .get_projectiles_in_sphere(
                    vec3_from_fields(message, "x", "y", "z")?,
                    f32_field(message, "radius")?,
                    synced,
                    weapon,
                )
                .map_err(|err| format!("get_projectiles_in_sphere() failed: {err:?}"))?,
            _ => return Err(format!("unsupported projectiles list check `{label}`")),
        };
        self.same_i32_set_if_present(label, message, "projectileIDs", &native)
    }
}

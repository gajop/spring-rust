use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_ground_height(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let x = f32_field(message, "x")?;
        let z = f32_field(message, "z")?;
        let native = self
            .interface
            .terrain()
            .get_ground_height(x, z)
            .map_err(|err| format!("get_ground_height({x}, {z}) failed: {err:?}"))?;
        self.same_if_present(label, message, "height", native)
    }
    pub(crate) fn check_ground_orig_height(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let x = f32_field(message, "x")?;
        let z = f32_field(message, "z")?;
        let native = self
            .interface
            .terrain()
            .get_ground_orig_height(x, z)
            .map_err(|err| format!("get_ground_orig_height({x}, {z}) failed: {err:?}"))?;
        self.same_if_present(label, message, "height", native)
    }
    pub(crate) fn check_ground_normal(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let x = f32_field(message, "x")?;
        let z = f32_field(message, "z")?;
        let (normal, slope) = self
            .interface
            .terrain()
            .get_ground_normal(x, z, bool_field(message, "smoothed")?)
            .map_err(|err| format!("get_ground_normal({x}, {z}) failed: {err:?}"))?;
        self.same(&format!("{label}.normalX"), normal.x, f32_field(message, "normalX")?)?;
        self.same(&format!("{label}.normalY"), normal.y, f32_field(message, "normalY")?)?;
        self.same(&format!("{label}.normalZ"), normal.z, f32_field(message, "normalZ")?)?;
        self.same_if_present(label, message, "slope", slope)
    }
    pub(crate) fn check_terrain_f32(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let terrain = self.interface.terrain();
        let native = match test_name {
            "get_smooth_mesh_height" => terrain
                .get_smooth_mesh_height(f32_field(message, "x")?, f32_field(message, "z")?)
                .map_err(|err| format!("get_smooth_mesh_height() failed: {err:?}"))?,
            "get_water_level" => terrain
                .get_water_level(f32_field(message, "x")?, f32_field(message, "z")?)
                .map_err(|err| format!("get_water_level() failed: {err:?}"))?,
            "get_water_plane_level" => terrain
                .get_water_plane_level()
                .map_err(|err| format!("get_water_plane_level() failed: {err:?}"))?,
            "get_grass" => terrain
                .get_grass(f32_field(message, "x")?, f32_field(message, "z")?)
                .map_err(|err| format!("get_grass() failed: {err:?}"))?,
            _ => return Err(format!("unsupported terrain f32 check `{label}`")),
        };
        let field = match test_name {
            "get_smooth_mesh_height" => "height",
            "get_water_level" => "waterLevel",
            "get_water_plane_level" => "waterPlaneLevel",
            "get_grass" => "grassLevel",
            _ => unreachable!(),
        };
        self.same_if_present(label, message, field, native)
    }
    pub(crate) fn check_ground_extremes(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let (init_min, init_max, curr_min, curr_max) = self
            .interface
            .terrain()
            .get_ground_extremes()
            .map_err(|err| format!("get_ground_extremes() failed: {err:?}"))?;
        self.same_if_present(label, message, "initMinHeight", init_min)?;
        self.same_if_present(label, message, "initMaxHeight", init_max)?;
        self.same_if_present(label, message, "currMinHeight", curr_min)?;
        self.same_if_present(label, message, "currMaxHeight", curr_max)
    }
    pub(crate) fn check_ground_blocked(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let native = self
            .interface
            .terrain()
            .get_ground_blocked(
                f32_field(message, "x1")?,
                f32_field(message, "z1")?,
                f32_field(message, "x2")?,
                f32_field(message, "z2")?,
            )
            .map_err(|err| format!("get_ground_blocked() failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "blocked", native)
    }
    pub(crate) fn check_is_pos_in_map(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let (in_map, in_play_area) = self
            .interface
            .terrain()
            .is_pos_in_map(f32_field(message, "x")?, f32_field(message, "z")?)
            .map_err(|err| format!("is_pos_in_map() failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "inPlayArea", in_play_area)?;
        self.same_bool_if_present(label, message, "inMap", in_map)
    }
    pub(crate) fn check_terrain_info(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        match test_name {
            "get_ground_info" | "map_square_terrain_type" => {
                let (index, name, metal_extraction, hardness, tank_speed, kbot_speed, hover_speed, ship_speed, receive_tracks) = self
                    .interface
                    .terrain()
                    .get_ground_info(f32_field(message, "x")?, f32_field(message, "z")?)
                    .map_err(|err| format!("get_ground_info() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "terrainTypeIndex", index)?;
                self.same_string_if_present(label, message, "terrainTypeName", name.as_deref().unwrap_or(""))?;
                self.same_if_present(label, message, "metalExtraction", metal_extraction)?;
                self.same_if_present(label, message, "hardness", hardness)?;
                self.same_if_present(label, message, "tankSpeed", tank_speed)?;
                self.same_if_present(label, message, "kbotSpeed", kbot_speed)?;
                self.same_if_present(label, message, "hoverSpeed", hover_speed)?;
                self.same_if_present(label, message, "shipSpeed", ship_speed)?;
                self.same_bool_if_present(label, message, "receiveTracks", receive_tracks)
            }
            "get_terrain_type_data" | "terrain_type_data" => {
                let terrain_type_index = i32_field(message, "terrainTypeIndex")?;
                let (index, name, hardness, tank_speed, kbot_speed, hover_speed, ship_speed, receive_tracks) = self
                    .interface
                    .terrain()
                    .get_terrain_type_data(terrain_type_index)
                    .map_err(|err| format!("get_terrain_type_data({terrain_type_index}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "terrainTypeIndex", index)?;
                self.same_string_if_present(label, message, "terrainTypeName", name.as_deref().unwrap_or(""))?;
                self.same_if_present(label, message, "hardness", hardness)?;
                self.same_if_present(label, message, "tankSpeed", tank_speed)?;
                self.same_if_present(label, message, "kbotSpeed", kbot_speed)?;
                self.same_if_present(label, message, "hoverSpeed", hover_speed)?;
                self.same_if_present(label, message, "shipSpeed", ship_speed)?;
                self.same_bool_if_present(label, message, "receiveTracks", receive_tracks)
            }
            _ => Err(format!("unsupported terrain info check `{label}`")),
        }
    }
    pub(crate) fn set_ground_height(&mut self, message: &Value) -> Result<(), String> {
        let x = f32_field(message, "x")?;
        let z = f32_field(message, "z")?;
        let height = f32_field(message, "height")?;
        let terraform = f32_field(message, "terraform")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let terrain = synced_ctrl.terrain();
        let mut set_result = Ok(false);
        let edit_success = terrain
            .set_height_map_func(|| {
                set_result = terrain.set_height_map(x, z, height, terraform);
            })
            .map_err(|err| format!("set_height_map_func() failed: {err:?}"))?;
        if !edit_success {
            return Err("set_height_map_func() returned false".to_owned());
        }
        if !set_result.map_err(|err| format!("set_height_map({x}, {z}) failed: {err:?}"))? {
            return Err(format!("set_height_map({x}, {z}) returned false"));
        }
        Ok(())
    }
    pub(crate) fn set_wind(&mut self, message: &Value) -> Result<(), String> {
        let wind_strength = f32_field(message, "windStrength")?;
        self.interface
            .synced_ctrl()
            .terrain()
            .set_wind(wind_strength, wind_strength)
            .map_err(|err| format!("set_wind() failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_terrain_type_data(&mut self, message: &Value) -> Result<(), String> {
        let terrain_type_index = i32_field(message, "terrainTypeIndex")?;
        self.interface
            .synced_ctrl()
            .terrain()
            .set_terrain_type_data(
                terrain_type_index,
                f32_field(message, "tankSpeed")?,
                f32_field(message, "kbotSpeed")?,
                f32_field(message, "hoverSpeed")?,
                f32_field(message, "shipSpeed")?,
                f32_field(message, "hardness")?,
                bool_field(message, "receiveTracks")?,
                str_field(message, "terrainTypeName")?,
            )
            .map_err(|err| format!("set_terrain_type_data({terrain_type_index}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_map_square_terrain_type(&mut self, message: &Value) -> Result<(), String> {
        let x = f32_field(message, "x")? as i32;
        let z = f32_field(message, "z")? as i32;
        let terrain_type_index = i32_field(message, "terrainTypeIndex")?;
        self.interface
            .synced_ctrl()
            .terrain()
            .set_map_square_terrain_type(x, z, terrain_type_index)
            .map_err(|err| format!("set_map_square_terrain_type({x}, {z}, {terrain_type_index}) failed: {err:?}"))?;
        Ok(())
    }
}

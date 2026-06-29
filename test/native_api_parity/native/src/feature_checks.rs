use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_feature_health(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_health(feature_id)
            .map_err(|err| format!("get_feature_health({feature_id}) failed: {err:?}"))?;

        self.same_if_present(label, message, "health", native.health)?;
        self.same_if_present(label, message, "maxHealth", native.maxHealth)
    }
    pub(crate) fn check_feature_mass(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_mass(feature_id)
            .map_err(|err| format!("get_feature_mass({feature_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "mass", native)
    }
    pub(crate) fn check_feature_resources(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_resources(feature_id)
            .map_err(|err| format!("get_feature_resources({feature_id}) failed: {err:?}"))?;

        self.same_if_present(label, message, "metal", native.metal)?;
        self.same_if_present(label, message, "energy", native.energy)?;
        self.same_if_present(label, message, "reclaimTime", native.reclaimTime)?;
        self.same_if_present(label, message, "reclaimLeft", native.reclaimLeft)
    }
    pub(crate) fn check_feature_separation(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id1 = i32_field(message, "featureID1")?;
        let feature_id2 = i32_field(message, "featureID2")?;
        let positional = bool_field(message, "positional")?;
        let native = self
            .interface
            .features()
            .get_feature_separation(feature_id1, feature_id2, positional)
            .map_err(|err| format!("get_feature_separation({feature_id1}, {feature_id2}, {positional}) failed: {err:?}"))?;
        self.same_if_present(label, message, "separation", native)
    }
    pub(crate) fn check_features_spatial_list(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        let native = match test_name {
            "get_features_in_rectangle" => self.interface.features().get_features_in_rectangle(
                f32_field(message, "minX")?,
                f32_field(message, "minZ")?,
                f32_field(message, "maxX")?,
                f32_field(message, "maxZ")?,
            )
                .map_err(|err| format!("get_features_in_rectangle() failed: {err:?}"))?,
            "get_features_in_sphere" => self.interface.features().get_features_in_sphere(
                vec3_from_fields(message, "x", "y", "z")?,
                f32_field(message, "radius")?,
            )
                .map_err(|err| format!("get_features_in_sphere() failed: {err:?}"))?,
            "get_features_in_cylinder" => self.interface.features().get_features_in_cylinder(
                f32_field(message, "x")?,
                f32_field(message, "z")?,
                f32_field(message, "radius")?,
                f32_field(message, "height")?,
            )
                .map_err(|err| format!("get_features_in_cylinder() failed: {err:?}"))?,
            "get_render_features" => {
                let draw_mask = i32_field(message, "drawMask")?;
                let send_mask = bool_field(message, "sendMask")?;
                self.interface.features().get_render_features(draw_mask, send_mask)
                    .map_err(|err| format!("get_render_features({draw_mask}, {send_mask}) failed: {err:?}"))?
            }
            "get_render_features_draw_flag_changed" => {
                let send_mask = bool_field(message, "sendMask")?;
                self.interface.features().get_render_features_draw_flag_changed(send_mask)
                    .map_err(|err| format!("get_render_features_draw_flag_changed({send_mask}) failed: {err:?}"))?
            }
            _ => return Err(format!("unsupported feature spatial list check `{label}`")),
        };
        self.same_i32_set_if_present(label, message, "featureIDs", &native)
    }
    pub(crate) fn check_feature_reclaim(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_health(feature_id)
            .map_err(|err| format!("get_feature_health({feature_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "reclaimLeft", native.reclaimLeft)
    }
    pub(crate) fn check_feature_resurrect(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let (native, _) = self
            .interface
            .features()
            .get_feature_resurrect(feature_id)
            .map_err(|err| format!("get_feature_resurrect({feature_id}) failed: {err:?}"))?;

        self.same_i32_if_present(label, message, "facing", native.facingDir)?;
        if message.get("unitDef").is_some() {
            let expected = str_field(message, "unitDef")?;
            let actual = if native.resurrectAs.is_null() {
                ""
            } else {
                unsafe { CStr::from_ptr(native.resurrectAs) }
                    .to_str()
                    .map_err(|err| format!("invalid resurrectAs string: {err}"))?
            };
            if actual != expected {
                return Err(format!("{label}.unitDef: native={actual}, lua={expected}"));
            }
        }
        Ok(())
    }
    pub(crate) fn check_feature_position(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_position(feature_id)
            .map_err(|err| format!("get_feature_position({feature_id}) failed: {err:?}"))?;
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_feature_height(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_height(feature_id)
            .map_err(|err| format!("get_feature_height({feature_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "height", native)
    }
    pub(crate) fn check_feature_radius(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_radius(feature_id)
            .map_err(|err| format!("get_feature_radius({feature_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "radius", native)
    }
    pub(crate) fn check_feature_heading(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_heading(feature_id)
            .map_err(|err| format!("get_feature_heading({feature_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "heading", native)
    }
    pub(crate) fn check_feature_velocity(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_velocity(feature_id)
            .map_err(|err| format!("get_feature_velocity({feature_id}) failed: {err:?}"))?;
        self.same_vec3(label, native, message)
    }
    pub(crate) fn check_feature_direction(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let front = self
            .interface
            .features()
            .get_feature_direction(feature_id)
            .map_err(|err| format!("get_feature_direction({feature_id}) failed: {err:?}"))?;

        self.same(&format!("{label}.frontX"), front.x, f32_field(message, "frontX")?)?;
        self.same(&format!("{label}.frontY"), front.y, f32_field(message, "frontY")?)?;
        self.same(&format!("{label}.frontZ"), front.z, f32_field(message, "frontZ")?)
    }
    pub(crate) fn check_feature_rotation(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_rotation(feature_id)
            .map_err(|err| format!("get_feature_rotation({feature_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "pitch", native.pitch)?;
        self.same_if_present(label, message, "yaw", native.yaw)?;
        self.same_if_present(label, message, "roll", native.roll)
    }
    pub(crate) fn check_feature_no_select(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_no_select(feature_id)
            .map_err(|err| format!("get_feature_no_select({feature_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "noSelect", native)
    }
    pub(crate) fn check_feature_collision_volume_data(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_collision_volume_data(feature_id)
            .map_err(|err| format!("get_feature_collision_volume_data({feature_id}) failed: {err:?}"))?;
        self.same_collision_volume(label, message, native)
    }
    pub(crate) fn check_feature_time(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let test_name = base_test_name(label);
        let (field, native) = match test_name {
            "feature_fire_time" => ("fireTime", self.interface.features().get_feature_fire_time(feature_id)
                .map_err(|err| format!("get_feature_fire_time({feature_id}) failed: {err:?}"))?),
            "feature_smoke_time" => ("smokeTime", self.interface.features().get_feature_smoke_time(feature_id)
                .map_err(|err| format!("get_feature_smoke_time({feature_id}) failed: {err:?}"))?),
            _ => return Err(format!("unsupported feature time check `{label}`")),
        };
        self.same_if_present(label, message, field, native)
    }
    pub(crate) fn check_feature_blocking(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_blocking(feature_id)
            .map_err(|err| format!("get_feature_blocking({feature_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "isBlocking", native.isBlocking)?;
        self.same_bool_if_present(label, message, "isSolidObjectCollidable", native.isSolidObjectCollidable)?;
        self.same_bool_if_present(label, message, "isProjectileCollidable", native.isProjectileCollidable)?;
        self.same_bool_if_present(label, message, "isRaySegmentCollidable", native.isRaySegmentCollidable)?;
        self.same_bool_if_present(label, message, "crushable", native.crushable)?;
        self.same_bool_if_present(label, message, "blockEnemyPushing", native.blockEnemyPushing)?;
        self.same_bool_if_present(label, message, "blockHeightChanges", native.blockHeightChanges)
    }
    pub(crate) fn check_feature_render_flag(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        match base_test_name(label) {
            "get_feature_no_draw" => {
                let native = self
                    .interface
                    .features()
                    .get_feature_no_draw(feature_id)
                    .map_err(|err| format!("get_feature_no_draw({feature_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "noDraw", native)
            }
            "get_feature_lua_draw" => {
                let native = self
                    .interface
                    .features()
                    .get_feature_lua_draw(feature_id)
                    .map_err(|err| format!("get_feature_lua_draw({feature_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "luaDraw", native)
            }
            "get_feature_engine_draw_mask" => {
                let native = self
                    .interface
                    .features()
                    .get_feature_engine_draw_mask(feature_id)
                    .map_err(|err| format!("get_feature_engine_draw_mask({feature_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "mask", native as i32)
            }
            "get_feature_draw_flag" => {
                let native = self
                    .interface
                    .features()
                    .get_feature_draw_flag(feature_id)
                    .map_err(|err| format!("get_feature_draw_flag({feature_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "flag", native as i32)
            }
            "get_feature_always_update_matrix" => {
                let native = self
                    .interface
                    .features()
                    .get_feature_always_update_matrix(feature_id)
                    .map_err(|err| format!("get_feature_always_update_matrix({feature_id}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "update", native)
            }
            _ => Err(format!("unsupported feature render flag check `{label}`")),
        }
    }
    pub(crate) fn set_feature_collision_volume_data(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let scales = vec3_from_fields(message, "scaleX", "scaleY", "scaleZ")?;
        let offsets = vec3_from_fields(message, "offsetX", "offsetY", "offsetZ")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_collision_volume_data(
                feature_id,
                scales,
                offsets,
                i32_field(message, "volumeType")?,
                i32_field(message, "testType")?,
                i32_field(message, "primaryAxis")?,
            )
            .map_err(|err| format!("set_feature_collision_volume_data({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_radius_and_height(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_radius_and_height(feature_id, f32_field(message, "radius")?, f32_field(message, "height")?)
            .map_err(|err| format!("set_feature_radius_and_height({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_health(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_health(
                feature_id,
                f32_field(message, "health")?,
                bool_field(message, "checkDestruction")?,
            )
            .map_err(|err| format!("set_feature_health({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn add_feature_damage(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let synced_ctrl = self.interface.synced_ctrl();
        let feature = synced_ctrl.feature();
        feature.set_feature_health(feature_id, f32_field(message, "baseline")?, false)
            .map_err(|err| format!("set_feature_health({feature_id}) failed: {err:?}"))?;
        feature.add_feature_damage(
            feature_id,
            f32_field(message, "damage")?,
            0.0,
            -1,
            -1,
            spring_native::sys::Float3 { x: 0.0, y: 0.0, z: 0.0 },
        )
            .map_err(|err| format!("add_feature_damage({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_max_health(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let max_health = f32_field(message, "maxHealth")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_max_health(feature_id, max_health)
            .map_err(|err| format!("set_feature_max_health({feature_id}) failed: {err:?}"))?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_health(feature_id, max_health + 500.0, false)
            .map_err(|err| format!("set_feature_health({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_mass(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_mass(feature_id, f32_field(message, "mass")?)
            .map_err(|err| format!("set_feature_mass({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_resources(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_resources(
                feature_id,
                f32_field(message, "metal")?,
                f32_field(message, "energy")?,
                f32_field(message, "reclaimTime")?,
                f32_field(message, "reclaimLeft")?,
                f32_field(message, "featureDefMetal")?,
                f32_field(message, "featureDefEnergy")?,
            )
            .map_err(|err| format!("set_feature_resources({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_time(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let test_name = base_test_name(str_field(message, "name")?);
        match test_name {
            "feature_fire_time" => self.interface
                .synced_ctrl()
                .feature()
                .set_feature_fire_time(feature_id, f32_field(message, "fireTime")?)
                .map_err(|err| format!("set_feature_fire_time({feature_id}) failed: {err:?}"))?,
            "feature_smoke_time" => self.interface
                .synced_ctrl()
                .feature()
                .set_feature_smoke_time(feature_id, f32_field(message, "smokeTime")?)
                .map_err(|err| format!("set_feature_smoke_time({feature_id}) failed: {err:?}"))?,
            _ => return Err(format!("unsupported feature time setter `{test_name}`")),
        };
        Ok(())
    }
    pub(crate) fn set_feature_reclaim(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_reclaim(feature_id, f32_field(message, "reclaimLeft")?)
            .map_err(|err| format!("set_feature_reclaim({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_resurrect(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let unit_def = CString::new(str_field(message, "unitDef")?)
            .map_err(|_| "unitDef contains an interior NUL byte".to_string())?;
        let def_ref = spring_native::sys::DefRef {
            name: unit_def.as_ptr(),
            id: -1,
        };
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_resurrect(
                feature_id,
                def_ref,
                i32_field(message, "facing")?,
                f32_field(message, "progress")?,
            )
            .map_err(|err| format!("set_feature_resurrect({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_position(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_position(
                feature_id,
                vec3_from_fields(message, "x", "y", "z")?,
                bool_field(message, "snapToGround")?,
            )
            .map_err(|err| format!("set_feature_position({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_velocity(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_velocity(feature_id, vec3_from_fields(message, "x", "y", "z")?)
            .map_err(|err| format!("set_feature_velocity({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_direction(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_direction(
                feature_id,
                vec3_from_fields(message, "frontX", "frontY", "frontZ")?,
                vec3_from_fields(message, "rightX", "rightY", "rightZ")?,
            )
            .map_err(|err| format!("set_feature_direction({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_rotation(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let rotation = vec3_from_fields(message, "pitch", "yaw", "roll")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_rotation(feature_id, rotation)
            .map_err(|err| format!("set_feature_rotation({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
    pub(crate) fn set_feature_no_select(&mut self, message: &Value) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        self.interface
            .synced_ctrl()
            .feature()
            .set_feature_no_select(feature_id, bool_field(message, "noSelect")?)
            .map_err(|err| format!("set_feature_no_select({feature_id}) failed: {err:?}"))?;
        Ok(())
    }
}

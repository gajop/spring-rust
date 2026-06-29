use super::*;

impl NativeApiParity {
    pub(crate) fn same_vec3(&self, label: &str, native: spring_native::sys::Float3, message: &Value) -> Result<(), String> {
        self.same(&format!("{label}.x"), native.x, f32_field(message, "x")?)?;
        self.same(&format!("{label}.y"), native.y, f32_field(message, "y")?)?;
        self.same(&format!("{label}.z"), native.z, f32_field(message, "z")?)?;
        Ok(())
    }
    pub(crate) fn same_if_present(&self, label: &str, message: &Value, field: &str, native: f32) -> Result<(), String> {
        if message.get(field).is_some() {
            self.same(&format!("{label}.{field}"), native, f32_field(message, field)?)?;
        }
        Ok(())
    }
    pub(crate) fn same_bool_if_present(&self, label: &str, message: &Value, field: &str, native: bool) -> Result<(), String> {
        if message.get(field).is_some() {
            let lua = bool_field(message, field)?;
            if native != lua {
                return Err(format!("{label}.{field}: native={native}, lua={lua}"));
            }
        }
        Ok(())
    }
    pub(crate) fn same_i32_if_present(&self, label: &str, message: &Value, field: &str, native: i32) -> Result<(), String> {
        if message.get(field).is_some() {
            let lua = i32_field(message, field)?;
            if native != lua {
                return Err(format!("{label}.{field}: native={native}, lua={lua}"));
            }
        }
        Ok(())
    }
    pub(crate) fn same_string_if_present(&self, label: &str, message: &Value, field: &str, native: &str) -> Result<(), String> {
        if message.get(field).is_some() {
            let lua = str_field(message, field)?;
            if native != lua {
                return Err(format!("{label}.{field}: native={native}, lua={lua}"));
            }
        }
        Ok(())
    }
    pub(crate) fn same_i32_list_if_present(&self, label: &str, message: &Value, field: &str, native: &[i32]) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                value
                    .as_i64()
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| format!("{label}.{field}: expected integer array element"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if native != lua_values.as_slice() {
            return Err(format!("{label}.{field}: native={native:?}, lua={lua_values:?}"));
        }
        Ok(())
    }
    pub(crate) fn same_i32_set_if_present(&self, label: &str, message: &Value, field: &str, native: &[i32]) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let mut lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                value
                    .as_i64()
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| format!("{label}.{field}: expected integer array element"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut native_values = native.to_vec();
        lua_values.sort_unstable();
        native_values.sort_unstable();
        if native_values != lua_values {
            return Err(format!("{label}.{field}: native={native_values:?}, lua={lua_values:?}"));
        }
        Ok(())
    }
    pub(crate) fn same_string_set_if_present(&self, label: &str, message: &Value, field: &str, native: &[String]) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let mut lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| format!("{label}.{field}: expected string array element"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut native_values = native.to_vec();
        lua_values.sort();
        native_values.sort();
        if native_values != lua_values {
            return Err(format!("{label}.{field}: native={native_values:?}, lua={lua_values:?}"));
        }
        Ok(())
    }
    pub(crate) fn same_string_i32_pairs_if_present(&self, label: &str, message: &Value, field: &str, native: &[(String, i32)]) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let mut lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                let name = value
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| format!("{label}.{field}: expected name"))?;
                let piece_num = value
                    .get("pieceNum")
                    .and_then(Value::as_i64)
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| format!("{label}.{field}: expected pieceNum"))?;
                Ok((name, piece_num))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut native_values = native.to_vec();
        lua_values.sort();
        native_values.sort();
        if native_values != lua_values {
            return Err(format!("{label}.{field}: native={native_values:?}, lua={lua_values:?}"));
        }
        Ok(())
    }
    pub(crate) fn same_unit_def_counts_if_present(&self, label: &str, message: &Value, field: &str, native: &[spring_native::sys::UnitDefCount]) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let mut lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                let unit_def_id = value
                    .get("unitDefID")
                    .and_then(Value::as_i64)
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| format!("{label}.{field}: expected unitDefID"))?;
                let count = value
                    .get("count")
                    .and_then(Value::as_u64)
                    .and_then(|value| u32::try_from(value).ok())
                    .ok_or_else(|| format!("{label}.{field}: expected count"))?;
                Ok((unit_def_id, count))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut native_values = native
            .iter()
            .map(|count| (count.unitDefID, count.count))
            .collect::<Vec<_>>();
        lua_values.sort_unstable();
        native_values.sort_unstable();
        if native_values != lua_values {
            return Err(format!("{label}.{field}: native={native_values:?}, lua={lua_values:?}"));
        }
        Ok(())
    }
    pub(crate) fn same_team_units_by_def_if_present(&self, label: &str, message: &Value, field: &str, native: &[spring_native::sys::TeamUnitsByDef]) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let mut lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                let unit_def_id = value
                    .get("unitDefID")
                    .and_then(Value::as_i64)
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| format!("{label}.{field}: expected unitDefID"))?;
                let mut units = value
                    .get("unitIDs")
                    .and_then(Value::as_array)
                    .ok_or_else(|| format!("{label}.{field}: expected unitIDs"))?
                    .iter()
                    .map(|value| {
                        value
                            .as_i64()
                            .and_then(|value| i32::try_from(value).ok())
                            .ok_or_else(|| format!("{label}.{field}: expected unitID"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                units.sort_unstable();
                Ok((unit_def_id, units))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut native_values = native
            .iter()
            .map(|group| {
                let mut units = if group.count == 0 || group.units.is_null() {
                    Vec::new()
                } else {
                    unsafe { std::slice::from_raw_parts(group.units as *const i32, group.count as usize) }.to_vec()
                };
                units.sort_unstable();
                (group.unitDefID, units)
            })
            .collect::<Vec<_>>();
        lua_values.sort_unstable();
        native_values.sort_unstable();
        if native_values != lua_values {
            return Err(format!("{label}.{field}: native={native_values:?}, lua={lua_values:?}"));
        }
        Ok(())
    }
    pub(crate) fn same_start_positions_if_present(&self, label: &str, message: &Value, field: &str, native: &[spring_native::sys::StartPosition]) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let mut lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                let team_id = value
                    .get("teamID")
                    .and_then(Value::as_i64)
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| format!("{label}.{field}: expected teamID"))?;
                Ok((
                    team_id,
                    value.get("x").and_then(Value::as_f64).ok_or_else(|| format!("{label}.{field}: expected x"))? as f32,
                    value.get("y").and_then(Value::as_f64).ok_or_else(|| format!("{label}.{field}: expected y"))? as f32,
                    value.get("z").and_then(Value::as_f64).ok_or_else(|| format!("{label}.{field}: expected z"))? as f32,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut native_values = native
            .iter()
            .map(|pos| (pos.teamID, pos.pos.x, pos.pos.y, pos.pos.z))
            .collect::<Vec<_>>();
        lua_values.sort_by_key(|value| value.0);
        native_values.sort_by_key(|value| value.0);
        if native_values.len() != lua_values.len() {
            return Err(format!("{label}.{field}: native_len={}, lua_len={}", native_values.len(), lua_values.len()));
        }
        for (index, (native, lua)) in native_values.iter().zip(lua_values.iter()).enumerate() {
            if native.0 != lua.0 {
                return Err(format!("{label}.{field}[{index}].teamID: native={}, lua={}", native.0, lua.0));
            }
            self.same(&format!("{label}.{field}[{index}].x"), native.1, lua.1)?;
            self.same(&format!("{label}.{field}[{index}].y"), native.2, lua.2)?;
            self.same(&format!("{label}.{field}[{index}].z"), native.3, lua.3)?;
        }
        Ok(())
    }
    pub(crate) fn same_collision_volume(&self, label: &str, message: &Value, native: spring_native::sys::CollisionVolumeData) -> Result<(), String> {
        self.same_if_present(label, message, "scaleX", native.scaleX)?;
        self.same_if_present(label, message, "scaleY", native.scaleY)?;
        self.same_if_present(label, message, "scaleZ", native.scaleZ)?;
        self.same_if_present(label, message, "offsetX", native.offsetX)?;
        self.same_if_present(label, message, "offsetY", native.offsetY)?;
        self.same_if_present(label, message, "offsetZ", native.offsetZ)?;
        self.same_i32_if_present(label, message, "volumeType", native.volumeType)?;
        self.same_i32_if_present(label, message, "testType", native.testType)?;
        self.same_i32_if_present(label, message, "primaryAxis", native.primaryAxis)?;
        self.same_bool_if_present(label, message, "disabled", native.disabled)
    }
    pub(crate) fn same(&self, label: &str, native: f32, lua: f32) -> Result<(), String> {
        if (native - lua).abs() <= EPSILON {
            Ok(())
        } else {
            Err(format!("{label}: native={native}, lua={lua}"))
        }
    }
    pub(crate) fn record(&self, name: &str, status: &str, message: &str) {
        if let Some(parent) = self.output_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_path)
        {
            let row = serde_json::json!({
                "context": "native",
                "name": name,
                "status": status,
                "message": message,
            });
            let _ = writeln!(file, "{row}");
        }

        let _ = self.interface.messages().echo("[native-api-parity]", message);
    }
}

pub(crate) fn i32_field(message: &Value, field: &str) -> Result<i32, String> {
    message
        .get(field)
        .and_then(Value::as_i64)
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| format!("missing integer field `{field}`"))
}

pub(crate) fn u32_field(message: &Value, field: &str) -> Result<u32, String> {
    message
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| format!("missing unsigned integer field `{field}`"))
}

pub(crate) fn f32_field(message: &Value, field: &str) -> Result<f32, String> {
    message
        .get(field)
        .and_then(Value::as_f64)
        .map(|value| value as f32)
        .ok_or_else(|| format!("missing number field `{field}`"))
}

pub(crate) fn bool_field(message: &Value, field: &str) -> Result<bool, String> {
    message
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing boolean field `{field}`"))
}

pub(crate) fn str_field<'a>(message: &'a Value, field: &str) -> Result<&'a str, String> {
    message
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string field `{field}`"))
}

pub(crate) fn cstr_or_empty(value: *const std::os::raw::c_char) -> Result<String, String> {
    if value.is_null() {
        return Ok(String::new());
    }
    unsafe { CStr::from_ptr(value) }
        .to_str()
        .map(str::to_owned)
        .map_err(|err| format!("invalid native string: {err}"))
}

pub(crate) fn vec3_from_fields(message: &Value, x: &str, y: &str, z: &str) -> Result<spring_native::sys::Float3, String> {
    Ok(spring_native::sys::Float3 {
        x: f32_field(message, x)?,
        y: f32_field(message, y)?,
        z: f32_field(message, z)?,
    })
}

pub(crate) fn rules_param_float_value(value: f32) -> spring_native::sys::RulesParamValue {
    spring_native::sys::RulesParamValue {
        type_: spring_native::sys::RulesParamType_RULESPARAM_TYPE_FLOAT,
        __bindgen_anon_1: spring_native::sys::RulesParamValue__bindgen_ty_1 { floatValue: value },
    }
}

pub(crate) fn rules_param_float(value: spring_native::sys::RulesParamValue) -> Result<f32, String> {
    if value.type_ != spring_native::sys::RulesParamType_RULESPARAM_TYPE_FLOAT {
        return Err(format!("rules param has unexpected type {}", value.type_));
    }
    Ok(unsafe { value.__bindgen_anon_1.floatValue })
}

pub(crate) fn base_test_name(label: &str) -> &str {
    label
        .strip_prefix("native_")
        .or_else(|| label.strip_prefix("set_native_"))
        .unwrap_or(label)
}

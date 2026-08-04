use super::*;
use std::sync::Mutex;

fn rounded_float(value: f32) -> f64 {
    // Lua receives the engine's float values as doubles.  Rounding at the
    // trace boundary removes representation noise while retaining enough
    // precision to catch coordinate/sign/unit conversion mistakes.
    // The engine's parity Lua build uses float Lua numbers, so perform the
    // same quantization in f32 before formatting the resulting value with
    // Lua's nine-significant-digit convention.
    let rounded = (value * 100_000.0_f32 + 0.5_f32).floor()
        / 100_000.0_f32;
    if rounded == 0.0 {
        return 0.0;
    }
    let integer_digits = rounded.abs().log10().floor() as i32 + 1;
    let decimals = (9 - integer_digits).max(0) as usize;
    format!("{:.*}", decimals, rounded)
        .parse::<f64>()
        .unwrap_or(rounded as f64)
}

// Native callbacks can be re-entered while a lifecycle call is being handled.
// Serialize JSONL writes so a row cannot be interleaved with another callback.
static RECORD_LOCK: Mutex<()> = Mutex::new(());

impl NativeApiParity {
    pub(crate) fn record_callin(&self, name: &str, arity: usize) {
        if let Some(parent) = self.callin_trace_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let _record_guard = RECORD_LOCK.lock().expect("native callin recorder lock");
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.callin_trace_path)
        {
            let row = serde_json::json!({
                "context": "native_callin",
                "name": name,
                "arity": arity,
            });
            let _ = writeln!(file, "{row}");
        }
    }

    pub(crate) fn record_callin_args(&self, name: &str, args: Vec<Value>) {
        self.record_callin_args_result(name, args, Vec::new());
    }

    pub(crate) fn record_callin_args_result(
        &self,
        name: &str,
        args: Vec<Value>,
        results: Vec<Value>,
    ) {
        if let Some(parent) = self.callin_trace_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let _record_guard = RECORD_LOCK.lock().expect("native callin recorder lock");
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.callin_trace_path)
        {
            let row = serde_json::json!({
                "context": "native_callin",
                "name": name,
                "arity": args.len(),
                "args": args,
                "resultArity": results.len(),
                "results": results,
            });
            let _ = writeln!(file, "{row}");
        }
    }

    pub(crate) fn trace_nil(&self) -> Value {
        serde_json::json!({"type": "nil"})
    }

    pub(crate) fn trace_i32(&self, value: i32) -> Value {
        serde_json::json!(value)
    }

    pub(crate) fn trace_i64(&self, value: i64) -> Value {
        serde_json::json!(value)
    }

    pub(crate) fn trace_u32(&self, value: u32) -> Value {
        serde_json::json!(value)
    }

    pub(crate) fn trace_u8(&self, value: u8) -> Value {
        serde_json::json!(value)
    }

    pub(crate) fn trace_bool(&self, value: bool) -> Value {
        serde_json::json!(value)
    }

    pub(crate) fn trace_f32(&self, value: f32) -> Value {
        serde_json::json!(rounded_float(value))
    }

    pub(crate) fn trace_str(&self, value: &str) -> Value {
        serde_json::json!(value)
    }

    pub(crate) fn trace_table(&self, mut entries: Vec<(Value, Value)>) -> Value {
        entries.sort_by_key(|(key, value)| {
            format!(
                "{}:{}",
                serde_json::to_string(key).unwrap_or_default(),
                serde_json::to_string(value).unwrap_or_default()
            )
        });
        Value::Array(
            entries
                .into_iter()
                .map(|(key, value)| serde_json::json!({"key": key, "value": value}))
                .collect(),
        )
    }

    pub(crate) fn trace_float3(&self, value: &spring_native::sys::Float3) -> Vec<Value> {
        vec![
            self.trace_f32(value.x),
            self.trace_f32(value.y),
            self.trace_f32(value.z),
        ]
    }

    pub(crate) fn trace_optional_i32(&self, value: Option<i32>) -> Value {
        value.map_or_else(|| self.trace_nil(), |value| self.trace_i32(value))
    }

    pub(crate) fn trace_optional_f32(&self, value: Option<f32>) -> Value {
        value.map_or_else(|| self.trace_nil(), |value| self.trace_f32(value))
    }

    pub(crate) fn trace_optional_bool(&self, value: Option<bool>) -> Value {
        value.map_or_else(|| self.trace_nil(), |value| self.trace_bool(value))
    }

    pub(crate) fn trace_error(&self) -> Value {
        serde_json::json!({"type": "error"})
    }

    pub(crate) fn trace_optional_str(&self, value: Option<&str>) -> Value {
        value.map_or_else(|| self.trace_nil(), |value| self.trace_str(value))
    }

    pub(crate) fn trace_command(
        &self,
        command: &spring_native::sys::NativeCallinCommand,
    ) -> Vec<Value> {
        let params = if command.numParams == 0 || command.params.is_null() {
            Vec::new()
        } else {
            // The engine guarantees that params points to numParams floats for
            // every command callback.  Keep the null check above so a malformed
            // third-party engine cannot make the parity recorder dereference a
            // null pointer.
            unsafe { std::slice::from_raw_parts(command.params, command.numParams as usize) }
                .iter()
                .copied()
                .map(|value| self.trace_f32(value))
                .enumerate()
                .map(|(index, value)| (self.trace_i32(index as i32 + 1), value))
                .collect::<Vec<_>>()
        };
        // NativeCallinCommand.options is the compact public ABI bitfield;
        // Lua's CommandOptions.coded retains the engine's historical option
        // bits.  Reconstruct the Lua-facing numeric value for the semantic
        // parity trace instead of comparing the two unrelated layouts.
        let mut coded = 0u8;
        if command.options & spring_native::constants::CMD_OPT_META as u8 != 0 {
            coded |= 1 << 2;
        }
        if command.options & spring_native::constants::CMD_OPT_INTERNAL as u8 != 0 {
            coded |= 1 << 3;
        }
        if command.options & spring_native::constants::CMD_OPT_RIGHT as u8 != 0 {
            coded |= 1 << 4;
        }
        if command.options & spring_native::constants::CMD_OPT_SHIFT as u8 != 0 {
            coded |= 1 << 5;
        }
        if command.options & spring_native::constants::CMD_OPT_CTRL as u8 != 0 {
            coded |= 1 << 6;
        }
        if command.options & spring_native::constants::CMD_OPT_ALT as u8 != 0 {
            coded |= 1 << 7;
        }
        let options = self.trace_table(vec![
            (
                self.trace_str("coded"),
                self.trace_u8(coded),
            ),
            (
                self.trace_str("alt"),
                self.trace_bool(command.options & spring_native::constants::CMD_OPT_ALT as u8 != 0),
            ),
            (
                self.trace_str("ctrl"),
                self.trace_bool(command.options & spring_native::constants::CMD_OPT_CTRL as u8 != 0),
            ),
            (
                self.trace_str("shift"),
                self.trace_bool(command.options & spring_native::constants::CMD_OPT_SHIFT as u8 != 0),
            ),
            (
                self.trace_str("right"),
                self.trace_bool(command.options & spring_native::constants::CMD_OPT_RIGHT as u8 != 0),
            ),
            (
                self.trace_str("meta"),
                self.trace_bool(command.options & spring_native::constants::CMD_OPT_META as u8 != 0),
            ),
            (
                self.trace_str("internal"),
                self.trace_bool(command.options & spring_native::constants::CMD_OPT_INTERNAL as u8 != 0),
            ),
        ]);
        vec![
            self.trace_i32(command.id),
            self.trace_table(params),
            options,
            self.trace_u32(command.tag),
        ]
    }

    pub(crate) fn trace_command_without_tag(
        &self,
        command: &spring_native::sys::NativeCallinCommand,
    ) -> Vec<Value> {
        let mut values = self.trace_command(command);
        values.pop();
        values
    }

    pub(crate) fn trace_actions(&self, actions: &[spring_native::KeyAction<'_>]) -> Value {
        self.trace_table(
            actions
                .iter()
                .enumerate()
                .map(|(index, action)| {
                    (
                        self.trace_i32(index as i32 + 1),
                        self.trace_table(vec![
                            (self.trace_str("command"), self.trace_str(action.command)),
                            (self.trace_str("extra"), self.trace_str(action.extra)),
                            (self.trace_str("boundWith"), self.trace_str(action.bound_with)),
                        ]),
                    )
                })
                .collect(),
        )
    }

    pub(crate) fn trace_game_setup_states(
        &self,
        states: &[spring_native::GameSetupPlayerState<'_>],
    ) -> Value {
        self.trace_table(
            states
                .iter()
                .map(|state| (self.trace_i32(state.player_id), self.trace_str(state.state)))
                .collect(),
        )
    }

    pub(crate) fn trace_geometry(&self, geometry: &spring_native::ViewGeometry) -> Value {
        self.trace_table(vec![
            (self.trace_str("screenSizeX"), self.trace_i32(geometry.screen_size_x)),
            (self.trace_str("screenSizeY"), self.trace_i32(geometry.screen_size_y)),
            (self.trace_str("screenPosX"), self.trace_i32(geometry.screen_pos_x)),
            (self.trace_str("screenPosY"), self.trace_i32(geometry.screen_pos_y)),
            (self.trace_str("windowSizeX"), self.trace_i32(geometry.window_size_x)),
            (self.trace_str("windowSizeY"), self.trace_i32(geometry.window_size_y)),
            (self.trace_str("windowPosX"), self.trace_i32(geometry.window_pos_x)),
            (self.trace_str("windowPosY"), self.trace_i32(geometry.window_pos_y)),
            (self.trace_str("windowBorderTop"), self.trace_i32(geometry.window_border_top)),
            (self.trace_str("windowBorderLeft"), self.trace_i32(geometry.window_border_left)),
            (self.trace_str("windowBorderBottom"), self.trace_i32(geometry.window_border_bottom)),
            (self.trace_str("windowBorderRight"), self.trace_i32(geometry.window_border_right)),
            (self.trace_str("viewSizeX"), self.trace_i32(geometry.view_size_x)),
            (self.trace_str("viewSizeY"), self.trace_i32(geometry.view_size_y)),
            (self.trace_str("viewPosX"), self.trace_i32(geometry.view_pos_x)),
            (self.trace_str("viewPosY"), self.trace_i32(geometry.view_pos_y)),
        ])
    }

    pub(crate) fn trace_resource_excess(
        &self,
        entries: &[spring_native::sys::ResourceExcessEntry],
    ) -> Value {
        self.trace_table(
            entries
                .iter()
                .map(|entry| {
                    (
                        self.trace_i32(entry.teamID),
                        self.trace_table(vec![
                            (self.trace_i32(1), self.trace_f32(entry.resources[0])),
                            (self.trace_i32(2), self.trace_f32(entry.resources[1])),
                        ]),
                    )
                })
                .collect(),
        )
    }

    pub(crate) fn trace_game_id(&self, game_id: &[u8]) -> Value {
        self.trace_str(
            &game_id
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>(),
        )
    }

    pub(crate) fn trace_byte_table(&self, bytes: &[u8]) -> Value {
        self.trace_table(
            bytes
                .iter()
                .enumerate()
                .map(|(index, byte)| (self.trace_i32(index as i32 + 1), self.trace_u8(*byte)))
                .collect(),
        )
    }

    pub(crate) fn trace_opaque(&self) -> Value {
        self.trace_str("userdata")
    }

    pub(crate) fn record_callin_phase(&self, name: &str) {
        if let Some(parent) = self.callin_trace_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let _record_guard = RECORD_LOCK
            .lock()
            .expect("native callin phase recorder lock");
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.callin_trace_path)
        {
            let row = serde_json::json!({
                "context": "callin_phase",
                "name": name,
            });
            let _ = writeln!(file, "{row}");
        }
    }

    pub(crate) fn same_vec3(
        &self,
        label: &str,
        native: spring_native::sys::Float3,
        message: &Value,
    ) -> Result<(), String> {
        self.same(&format!("{label}.x"), native.x, f32_field(message, "x")?)?;
        self.same(&format!("{label}.y"), native.y, f32_field(message, "y")?)?;
        self.same(&format!("{label}.z"), native.z, f32_field(message, "z")?)?;
        Ok(())
    }
    pub(crate) fn same_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: f32,
    ) -> Result<(), String> {
        if message.get(field).is_some() {
            self.same(
                &format!("{label}.{field}"),
                native,
                f32_field(message, field)?,
            )?;
        }
        Ok(())
    }
    pub(crate) fn same_bool_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: bool,
    ) -> Result<(), String> {
        if message.get(field).is_some() {
            let lua = bool_field(message, field)?;
            if native != lua {
                return Err(format!("{label}.{field}: native={native}, lua={lua}"));
            }
        }
        Ok(())
    }
    pub(crate) fn same_i32_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: i32,
    ) -> Result<(), String> {
        if message.get(field).is_some() {
            let lua = i32_field(message, field)?;
            if native != lua {
                return Err(format!("{label}.{field}: native={native}, lua={lua}"));
            }
        }
        Ok(())
    }
    pub(crate) fn same_u32_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: u32,
    ) -> Result<(), String> {
        if message.get(field).is_some() {
            let lua = u32_field(message, field)?;
            if native != lua {
                return Err(format!("{label}.{field}: native={native}, lua={lua}"));
            }
        }
        Ok(())
    }
    pub(crate) fn same_string_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &str,
    ) -> Result<(), String> {
        if message.get(field).is_some() {
            let lua = str_field(message, field)?;
            if native != lua {
                return Err(format!("{label}.{field}: native={native}, lua={lua}"));
            }
        }
        Ok(())
    }
    pub(crate) fn same_i32_list_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[i32],
    ) -> Result<(), String> {
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
            return Err(format!(
                "{label}.{field}: native={native:?}, lua={lua_values:?}"
            ));
        }
        Ok(())
    }
    pub(crate) fn same_f32_list_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[f32],
    ) -> Result<(), String> {
        let Some(value) = message.get(field) else {
            return Ok(());
        };
        let lua_values = value
            .as_array()
            .ok_or_else(|| format!("{label}.{field}: expected array"))?
            .iter()
            .map(|value| {
                value
                    .as_f64()
                    .map(|value| value as f32)
                    .ok_or_else(|| format!("{label}.{field}: expected number array element"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if native.len() != lua_values.len() {
            return Err(format!(
                "{label}.{field}: native_len={}, lua_len={}",
                native.len(),
                lua_values.len()
            ));
        }
        for (index, (native, lua)) in native.iter().zip(lua_values.iter()).enumerate() {
            self.same(&format!("{label}.{field}[{index}]"), *native, *lua)?;
        }
        Ok(())
    }
    pub(crate) fn same_i32_set_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[i32],
    ) -> Result<(), String> {
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
            return Err(format!(
                "{label}.{field}: native={native_values:?}, lua={lua_values:?}"
            ));
        }
        Ok(())
    }
    pub(crate) fn same_string_set_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[String],
    ) -> Result<(), String> {
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
            return Err(format!(
                "{label}.{field}: native={native_values:?}, lua={lua_values:?}"
            ));
        }
        Ok(())
    }
    pub(crate) fn same_string_i32_pairs_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[(String, i32)],
    ) -> Result<(), String> {
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
            return Err(format!(
                "{label}.{field}: native={native_values:?}, lua={lua_values:?}"
            ));
        }
        Ok(())
    }
    pub(crate) fn same_unit_def_counts_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[spring_native::sys::UnitDefCount],
    ) -> Result<(), String> {
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
            return Err(format!(
                "{label}.{field}: native={native_values:?}, lua={lua_values:?}"
            ));
        }
        Ok(())
    }
    pub(crate) fn same_team_units_by_def_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[spring_native::sys::TeamUnitsByDef],
    ) -> Result<(), String> {
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
                    unsafe {
                        std::slice::from_raw_parts(group.units as *const i32, group.count as usize)
                    }
                    .to_vec()
                };
                units.sort_unstable();
                (group.unitDefID, units)
            })
            .collect::<Vec<_>>();
        lua_values.sort_unstable();
        native_values.sort_unstable();
        if native_values != lua_values {
            return Err(format!(
                "{label}.{field}: native={native_values:?}, lua={lua_values:?}"
            ));
        }
        Ok(())
    }
    pub(crate) fn same_start_positions_if_present(
        &self,
        label: &str,
        message: &Value,
        field: &str,
        native: &[spring_native::sys::StartPosition],
    ) -> Result<(), String> {
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
                    value
                        .get("x")
                        .and_then(Value::as_f64)
                        .ok_or_else(|| format!("{label}.{field}: expected x"))?
                        as f32,
                    value
                        .get("y")
                        .and_then(Value::as_f64)
                        .ok_or_else(|| format!("{label}.{field}: expected y"))?
                        as f32,
                    value
                        .get("z")
                        .and_then(Value::as_f64)
                        .ok_or_else(|| format!("{label}.{field}: expected z"))?
                        as f32,
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
            return Err(format!(
                "{label}.{field}: native_len={}, lua_len={}",
                native_values.len(),
                lua_values.len()
            ));
        }
        for (index, (native, lua)) in native_values.iter().zip(lua_values.iter()).enumerate() {
            if native.0 != lua.0 {
                return Err(format!(
                    "{label}.{field}[{index}].teamID: native={}, lua={}",
                    native.0, lua.0
                ));
            }
            self.same(&format!("{label}.{field}[{index}].x"), native.1, lua.1)?;
            self.same(&format!("{label}.{field}[{index}].y"), native.2, lua.2)?;
            self.same(&format!("{label}.{field}[{index}].z"), native.3, lua.3)?;
        }
        Ok(())
    }
    pub(crate) fn same_collision_volume(
        &self,
        label: &str,
        message: &Value,
        native: spring_native::sys::CollisionVolumeData,
    ) -> Result<(), String> {
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

        {
            let _record_guard = RECORD_LOCK.lock().expect("native parity recorder lock");
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
        }

        let _ = self
            .interface
            .messages()
            .echo("[native-api-parity]", message);
    }
}

pub(crate) fn i32_field(message: &Value, field: &str) -> Result<i32, String> {
    message
        .get(field)
        .and_then(Value::as_i64)
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| format!("missing integer field `{field}`"))
}

pub(crate) fn test_name_field(message: &Value) -> Result<&str, String> {
    message
        .get("testName")
        .or_else(|| message.get("name"))
        .and_then(Value::as_str)
        .ok_or_else(|| "missing string field `testName`".to_string())
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

pub(crate) fn vec3_from_fields(
    message: &Value,
    x: &str,
    y: &str,
    z: &str,
) -> Result<spring_native::sys::Float3, String> {
    Ok(spring_native::sys::Float3 {
        x: f32_field(message, x)?,
        y: f32_field(message, y)?,
        z: f32_field(message, z)?,
    })
}

pub(crate) fn rules_param_float_value(value: f32) -> spring_native::RulesParamValue {
    spring_native::RulesParamValue::Float(value)
}

pub(crate) fn rules_param_float(value: spring_native::RulesParamValue) -> Result<f32, String> {
    if let spring_native::RulesParamValue::Float(value) = value {
        return Ok(value);
    }
    Err(format!("rules param has unexpected type {value:?}"))
}

pub(crate) fn base_test_name(label: &str) -> &str {
    label
        .strip_prefix("native_")
        .or_else(|| label.strip_prefix("set_native_"))
        .unwrap_or(label)
}

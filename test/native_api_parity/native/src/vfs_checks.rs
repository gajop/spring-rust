use super::*;
use crate::support::*;

fn i64_list_field(message: &Value, field: &str) -> Result<Vec<i64>, String> {
    message
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing integer array field `{field}`"))?
        .iter()
        .map(|value| {
            value
                .as_i64()
                .ok_or_else(|| format!("{field} contains a non-integer value"))
        })
        .collect()
}

fn u64_list_field(message: &Value, field: &str) -> Result<Vec<u64>, String> {
    message
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing unsigned integer array field `{field}`"))?
        .iter()
        .map(|value| {
            value
                .as_u64()
                .ok_or_else(|| format!("{field} contains a non-unsigned integer value"))
        })
        .collect()
}

fn f64_list_field(message: &Value, field: &str) -> Result<Vec<f64>, String> {
    message
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing number array field `{field}`"))?
        .iter()
        .map(|value| {
            value
                .as_f64()
                .ok_or_else(|| format!("{field} contains a non-number value"))
        })
        .collect()
}

fn byte_hex(data: &[u8]) -> String {
    data.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn lua_byte_offset(message: &Value) -> Result<u32, String> {
    let pos = i32_field(message, "pos")?;
    if pos < 1 {
        return Err(format!("invalid Lua byte position {pos}"));
    }
    Ok((pos - 1) as u32)
}

fn unpack_count(message: &Value) -> Result<u32, String> {
    let count = i32_field(message, "count")?;
    if count < 0 {
        return Err(format!("invalid unpack count {count}"));
    }
    Ok(count as u32)
}

fn surface_result<'a>(message: &'a Value) -> Result<&'a Value, String> {
    message
        .get("result")
        .ok_or_else(|| "missing VFS archive surface result".to_string())
}

fn surface_field<'a>(result: &'a Value, field: &str) -> Result<&'a Value, String> {
    result
        .get(field)
        .ok_or_else(|| format!("missing VFS archive surface field `{field}`"))
}

fn compare_surface_values(label: &str, expected: &Value, actual: &Value) -> Result<(), String> {
    match (expected, actual) {
        (Value::Number(expected), Value::Number(actual)) => {
            let expected = expected
                .as_f64()
                .ok_or_else(|| format!("{label}: invalid expected number"))?;
            let actual = actual
                .as_f64()
                .ok_or_else(|| format!("{label}: invalid native number"))?;
            if (expected - actual).abs() > 0.0001 {
                return Err(format!("{label}: native={actual}, lua={expected}"));
            }
        }
        (Value::Array(expected), Value::Array(actual)) => {
            if expected.len() != actual.len() {
                return Err(format!(
                    "{label}: native array length={}, lua array length={}",
                    actual.len(),
                    expected.len()
                ));
            }
            for (index, (expected, actual)) in expected.iter().zip(actual).enumerate() {
                compare_surface_values(&format!("{label}[{index}]"), expected, actual)?;
            }
        }
        (Value::Object(expected), Value::Object(actual)) => {
            if expected.len() != actual.len() {
                return Err(format!(
                    "{label}: native object fields={}, lua object fields={}",
                    actual.len(),
                    expected.len()
                ));
            }
            for (key, expected) in expected {
                let actual = actual
                    .get(key)
                    .ok_or_else(|| format!("{label}: native object is missing `{key}`"))?;
                compare_surface_values(&format!("{label}.{key}"), expected, actual)?;
            }
        }
        (expected, actual) if expected == actual => {}
        (expected, actual) => {
            return Err(format!("{label}: native={actual}, lua={expected}"));
        }
    }
    Ok(())
}

fn optional_string_surface(value: Option<String>) -> Value {
    match value {
        Some(value) => serde_json::json!({ "present": true, "value": value }),
        None => serde_json::json!({ "present": false, "value": "" }),
    }
}

fn optional_path_surface(value: Option<String>) -> Value {
    let mut surface = serde_json::json!({
        "present": value.is_some(),
        "basename": "",
    });
    if let Some(object) = surface.as_object_mut() {
        let basename = value
            .as_deref()
            .and_then(|value| value.rsplit(['/', '\\']).next())
            .unwrap_or("");
        object.insert("basename".to_string(), Value::String(basename.to_string()));
    }
    surface
}

fn archive_info_surface(entries: &[spring_native::sys::ArchiveInfoEntry]) -> Result<Value, String> {
    let mut result = Vec::with_capacity(entries.len());
    for entry in entries {
        let key = cstr_or_empty(entry.key)?;
        let type_name = cstr_or_empty(entry.type_)?;
        let (value_type, value) = match type_name.as_str() {
            "string" => ("string", Value::String(cstr_or_empty(entry.stringValue)?)),
            "integer" => ("number", serde_json::json!(entry.intValue)),
            "float" => ("number", serde_json::json!(entry.floatValue)),
            "bool" => ("boolean", Value::Bool(entry.boolValue)),
            other => return Err(format!("unknown ArchiveInfoEntry type `{other}`")),
        };
        result.push(serde_json::json!({
            "key": key,
            "valueType": value_type,
            "value": value,
        }));
    }
    result.sort_by(|left, right| {
        left.get("key")
            .and_then(Value::as_str)
            .cmp(&right.get("key").and_then(Value::as_str))
    });
    Ok(Value::Array(result))
}

fn available_ais_surface(entries: &[spring_native::sys::AIInfoEntry]) -> Result<Value, String> {
    let mut result = Vec::with_capacity(entries.len());
    for entry in entries {
        result.push(serde_json::json!({
            "shortName": cstr_or_empty(entry.shortName)?,
            "version": cstr_or_empty(entry.version)?,
            "isLuaAI": entry.isLuaAI,
        }));
    }
    result.sort_by(|left, right| {
        let left_key = (
            left.get("shortName").and_then(Value::as_str).unwrap_or(""),
            left.get("version").and_then(Value::as_str).unwrap_or(""),
            left.get("isLuaAI")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        );
        let right_key = (
            right.get("shortName").and_then(Value::as_str).unwrap_or(""),
            right.get("version").and_then(Value::as_str).unwrap_or(""),
            right
                .get("isLuaAI")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        );
        left_key.cmp(&right_key)
    });
    Ok(Value::Array(result))
}

impl NativeApiParity {
    pub(crate) fn check_vfs_archive_surface(&mut self, message: &Value) -> Result<(), String> {
        let result = surface_result(message)?;
        let file_name = str_field(result, "fileName")?;
        let file_mode = str_field(result, "fileMode")?;
        let archive_name = str_field(result, "archiveName")?;
        let native_archive_name = str_field(result, "nativeArchiveName")?;
        let use_archive_name = str_field(surface_field(result, "useArchive")?, "archiveName")?;
        let raw_file = surface_field(result, "rawFile")?;
        let raw_file_name = str_field(raw_file, "name")?;
        let raw_file_mode = str_field(raw_file, "mode")?;
        let vfs = self.interface.vfs();

        let absolute_path = vfs
            .get_file_absolute_path(file_name, file_mode)
            .map_err(|err| format!("get_file_absolute_path() failed: {err:?}"))?;
        compare_surface_values(
            "fileAbsolutePath",
            surface_field(result, "fileAbsolutePath")?,
            &optional_path_surface(absolute_path),
        )?;

        let containing_file = vfs
            .get_archive_containing_file(file_name, file_mode)
            .map_err(|err| format!("get_archive_containing_file() failed: {err:?}"))?;
        compare_surface_values(
            "archiveContainingFile",
            surface_field(result, "archiveContainingFile")?,
            &optional_string_surface(containing_file),
        )?;

        let has_archive = vfs
            .has_archive(archive_name)
            .map_err(|err| format!("has_archive() failed: {err:?}"))?;
        compare_surface_values(
            "hasArchive",
            surface_field(result, "hasArchive")?,
            &serde_json::json!(has_archive),
        )?;

        let mut loaded_archives = vfs
            .get_loaded_archives()
            .map_err(|err| format!("get_loaded_archives() failed: {err:?}"))?;
        loaded_archives.sort();
        compare_surface_values(
            "loadedArchives",
            surface_field(result, "loadedArchives")?,
            &serde_json::json!(loaded_archives),
        )?;

        let mut all_archives = vfs
            .get_all_archives()
            .map_err(|err| format!("get_all_archives() failed: {err:?}"))?;
        all_archives.sort();
        compare_surface_values(
            "allArchives",
            surface_field(result, "allArchives")?,
            &serde_json::json!(all_archives),
        )?;

        let archive_path = vfs
            .get_archive_path(archive_name)
            .map_err(|err| format!("get_archive_path() failed: {err:?}"))?;
        compare_surface_values(
            "archivePath",
            surface_field(result, "archivePath")?,
            &optional_path_surface(archive_path),
        )?;

        let archive_info = vfs
            .get_archive_info(archive_name)
            .map_err(|err| format!("get_archive_info() failed: {err:?}"))?;
        compare_surface_values(
            "archiveInfo",
            surface_field(result, "archiveInfo")?,
            &archive_info_surface(&archive_info)?,
        )?;

        let mut dependencies = vfs
            .get_archive_dependencies(archive_name)
            .map_err(|err| format!("get_archive_dependencies() failed: {err:?}"))?;
        dependencies.sort();
        compare_surface_values(
            "archiveDependencies",
            surface_field(result, "archiveDependencies")?,
            &serde_json::json!(dependencies),
        )?;

        let mut replaces = vfs
            .get_archive_replaces(archive_name)
            .map_err(|err| format!("get_archive_replaces() failed: {err:?}"))?;
        replaces.sort();
        compare_surface_values(
            "archiveReplaces",
            surface_field(result, "archiveReplaces")?,
            &serde_json::json!(replaces),
        )?;

        let (single_checksum, complete_checksum) = vfs
            .get_archive_checksum(archive_name)
            .map_err(|err| format!("get_archive_checksum() failed: {err:?}"))?;
        compare_surface_values(
            "archiveChecksum",
            surface_field(result, "archiveChecksum")?,
            &serde_json::json!({
                "single": single_checksum.unwrap_or_default(),
                "complete": complete_checksum.unwrap_or_default(),
            }),
        )?;

        let rapid_tag = vfs
            .get_name_from_rapid_tag("native-api-parity-not-a-rapid-tag")
            .map_err(|err| format!("get_name_from_rapid_tag() failed: {err:?}"))?;
        compare_surface_values(
            "rapidTag",
            surface_field(result, "rapidTag")?,
            &optional_string_surface(rapid_tag),
        )?;

        let available_ais = vfs
            .get_available_ais("", "")
            .map_err(|err| format!("get_available_ais() failed: {err:?}"))?;
        compare_surface_values(
            "availableAIs",
            surface_field(result, "availableAIs")?,
            &available_ais_surface(&available_ais)?,
        )?;

        let raw_data = vfs
            .load_file(raw_file_name, raw_file_mode)
            .map_err(|err| format!("load_file() failed: {err:?}"))?;
        compare_surface_values(
            "rawFile.hex",
            surface_field(raw_file, "hex")?,
            &serde_json::json!(byte_hex(&raw_data)),
        )?;

        let mut raw_dir_list = vfs
            .dir_list(
                raw_file_name
                    .rsplit_once('/')
                    .map(|(path, _)| path)
                    .unwrap_or(""),
                "*.lua",
                raw_file_mode,
                false,
            )
            .map_err(|err| format!("dir_list() failed: {err:?}"))?
            .iter()
            .filter(|entry| !entry.isDirectory)
            .map(|entry| cstr_or_empty(entry.name))
            .collect::<Result<Vec<_>, _>>()?;
        raw_dir_list.sort();
        compare_surface_values(
            "rawFile.dirList",
            surface_field(raw_file, "dirList")?,
            &serde_json::json!(raw_dir_list),
        )?;

        let mut raw_sub_dirs = vfs
            .sub_dirs("LuaRules", "*", raw_file_mode, false)
            .map_err(|err| format!("sub_dirs() failed: {err:?}"))?;
        raw_sub_dirs.sort();
        compare_surface_values(
            "rawFile.subDirs",
            surface_field(raw_file, "subDirs")?,
            &serde_json::json!(raw_sub_dirs),
        )?;

        let compressed = vfs
            .compress_folder("LuaRules/Gadgets", "zip", native_archive_name, false, "r")
            .map_err(|err| format!("compress_folder() failed: {err:?}"))?;
        let compressed_exists = vfs
            .file_exists(native_archive_name)
            .map_err(|err| format!("file_exists(compressed archive) failed: {err:?}"))?;
        let compress_result = surface_field(result, "compress")?;
        compare_surface_values(
            "compress.ok",
            &serde_json::json!(true),
            &serde_json::json!(compressed),
        )?;
        compare_surface_values(
            "compress.exists",
            &compress_result
                .get("exists")
                .cloned()
                .ok_or_else(|| "missing compress.exists".to_string())?,
            &serde_json::json!(compressed_exists),
        )?;
        let mut callback_visible = false;
        let mut callback_file_exists = false;
        let use_archive_available = vfs
            .has_archive(use_archive_name)
            .map_err(|err| format!("has_archive(use archive) failed: {err:?}"))?;
        if !use_archive_available {
            return Err(format!(
                "use_archive archive is not present in the native archive scanner: {use_archive_name:?}"
            ));
        }
        let callback_interface = self.interface;
        let use_success = vfs
            .use_archive(use_archive_name, || {
                let callback_vfs = callback_interface.vfs();
                callback_visible = callback_vfs
                    .get_loaded_archives()
                    .map(|archives| archives.iter().any(|archive| archive == use_archive_name))
                    .unwrap_or(false);
                callback_file_exists = callback_vfs
                    .file_exists("anims/cursorattack_0.bmp")
                    .unwrap_or(false);
            })
            .map_err(|err| format!("use_archive() failed: {err:?}"))?;
        let mut post_loaded_archives = callback_interface
            .vfs()
            .get_loaded_archives()
            .map_err(|err| format!("get_loaded_archives(after UseArchive) failed: {err:?}"))?;
        post_loaded_archives.sort();
        let use_result = surface_field(result, "useArchive")?;
        compare_surface_values(
            "useArchive.ok",
            &use_result
                .get("ok")
                .cloned()
                .ok_or_else(|| "missing useArchive.ok".to_string())?,
            &serde_json::json!(use_success),
        )?;
        compare_surface_values(
            "useArchive.callbackVisible",
            &use_result
                .get("callbackVisible")
                .cloned()
                .ok_or_else(|| "missing useArchive.callbackVisible".to_string())?,
            &serde_json::json!(callback_visible),
        )?;
        compare_surface_values(
            "useArchive.callbackFileExists",
            &use_result
                .get("callbackFileExists")
                .cloned()
                .ok_or_else(|| "missing useArchive.callbackFileExists".to_string())?,
            &serde_json::json!(callback_file_exists),
        )?;
        compare_surface_values(
            "useArchive.postLoadedArchives",
            &use_result
                .get("postLoadedArchives")
                .cloned()
                .ok_or_else(|| "missing useArchive.postLoadedArchives".to_string())?,
            &serde_json::json!(post_loaded_archives),
        )?;

        Ok(())
    }

    pub(crate) fn check_vfs_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let test_name = base_test_name(label);
        match test_name {
            "vfs_pack_u8" => {
                let values = i64_list_field(message, "values")?
                    .into_iter()
                    .map(|value| {
                        u8::try_from(value).map_err(|_| format!("invalid u8 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let native = self
                    .interface
                    .vfs()
                    .pack_u8(&values)
                    .map_err(|err| format!("pack_u8() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_pack_u16" => {
                let values = i64_list_field(message, "values")?
                    .into_iter()
                    .map(|value| {
                        u16::try_from(value).map_err(|_| format!("invalid u16 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let native = self
                    .interface
                    .vfs()
                    .pack_u16(&values)
                    .map_err(|err| format!("pack_u16() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_pack_u32" => {
                let values = u64_list_field(message, "values")?
                    .into_iter()
                    .map(|value| {
                        u32::try_from(value).map_err(|_| format!("invalid u32 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let native = self
                    .interface
                    .vfs()
                    .pack_u32(&values)
                    .map_err(|err| format!("pack_u32() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_pack_s8" => {
                let values = i64_list_field(message, "values")?
                    .into_iter()
                    .map(|value| {
                        i8::try_from(value).map_err(|_| format!("invalid s8 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let native = self
                    .interface
                    .vfs()
                    .pack_s8(&values)
                    .map_err(|err| format!("pack_s8() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_pack_s16" => {
                let values = i64_list_field(message, "values")?
                    .into_iter()
                    .map(|value| {
                        i16::try_from(value).map_err(|_| format!("invalid s16 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let native = self
                    .interface
                    .vfs()
                    .pack_s16(&values)
                    .map_err(|err| format!("pack_s16() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_pack_s32" => {
                let values = i64_list_field(message, "values")?
                    .into_iter()
                    .map(|value| {
                        i32::try_from(value).map_err(|_| format!("invalid s32 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let native = self
                    .interface
                    .vfs()
                    .pack_s32(&values)
                    .map_err(|err| format!("pack_s32() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_pack_f32" => {
                let values = f64_list_field(message, "values")?
                    .into_iter()
                    .map(|value| value as f32)
                    .collect::<Vec<_>>();
                let native = self
                    .interface
                    .vfs()
                    .pack_f32(&values)
                    .map_err(|err| format!("pack_f32() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_unpack_u8" => {
                let values = i64_list_field(message, "source")?
                    .into_iter()
                    .map(|value| {
                        u8::try_from(value).map_err(|_| format!("invalid u8 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let packed = self
                    .interface
                    .vfs()
                    .pack_u8(&values)
                    .map_err(|err| format!("pack_u8() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .unpack_u8(&packed, lua_byte_offset(message)?, unpack_count(message)?)
                    .map_err(|err| format!("unpack_u8() failed: {err:?}"))?
                    .into_iter()
                    .map(i32::from)
                    .collect::<Vec<_>>();
                self.same_i32_list_if_present(label, message, "values", &native)
            }
            "vfs_unpack_u16" => {
                let values = i64_list_field(message, "source")?
                    .into_iter()
                    .map(|value| {
                        u16::try_from(value).map_err(|_| format!("invalid u16 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let packed = self
                    .interface
                    .vfs()
                    .pack_u16(&values)
                    .map_err(|err| format!("pack_u16() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .unpack_u16(&packed, lua_byte_offset(message)?, unpack_count(message)?)
                    .map_err(|err| format!("unpack_u16() failed: {err:?}"))?
                    .into_iter()
                    .map(i32::from)
                    .collect::<Vec<_>>();
                self.same_i32_list_if_present(label, message, "values", &native)
            }
            "vfs_unpack_u32" => {
                let values = u64_list_field(message, "source")?
                    .into_iter()
                    .map(|value| {
                        u32::try_from(value).map_err(|_| format!("invalid u32 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let packed = self
                    .interface
                    .vfs()
                    .pack_u32(&values)
                    .map_err(|err| format!("pack_u32() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .unpack_u32(&packed, lua_byte_offset(message)?, unpack_count(message)?)
                    .map_err(|err| format!("unpack_u32() failed: {err:?}"))?
                    .into_iter()
                    .map(|value| {
                        i32::try_from(value)
                            .map_err(|_| format!("u32 result {value} does not fit i32"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.same_i32_list_if_present(label, message, "values", &native)
            }
            "vfs_unpack_s8" => {
                let values = i64_list_field(message, "source")?
                    .into_iter()
                    .map(|value| {
                        i8::try_from(value).map_err(|_| format!("invalid s8 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let packed = self
                    .interface
                    .vfs()
                    .pack_s8(&values)
                    .map_err(|err| format!("pack_s8() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .unpack_s8(&packed, lua_byte_offset(message)?, unpack_count(message)?)
                    .map_err(|err| format!("unpack_s8() failed: {err:?}"))?
                    .into_iter()
                    .map(i32::from)
                    .collect::<Vec<_>>();
                self.same_i32_list_if_present(label, message, "values", &native)
            }
            "vfs_unpack_s16" => {
                let values = i64_list_field(message, "source")?
                    .into_iter()
                    .map(|value| {
                        i16::try_from(value).map_err(|_| format!("invalid s16 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let packed = self
                    .interface
                    .vfs()
                    .pack_s16(&values)
                    .map_err(|err| format!("pack_s16() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .unpack_s16(&packed, lua_byte_offset(message)?, unpack_count(message)?)
                    .map_err(|err| format!("unpack_s16() failed: {err:?}"))?
                    .into_iter()
                    .map(i32::from)
                    .collect::<Vec<_>>();
                self.same_i32_list_if_present(label, message, "values", &native)
            }
            "vfs_unpack_s32" => {
                let values = i64_list_field(message, "source")?
                    .into_iter()
                    .map(|value| {
                        i32::try_from(value).map_err(|_| format!("invalid s32 value {value}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let packed = self
                    .interface
                    .vfs()
                    .pack_s32(&values)
                    .map_err(|err| format!("pack_s32() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .unpack_s32(&packed, lua_byte_offset(message)?, unpack_count(message)?)
                    .map_err(|err| format!("unpack_s32() failed: {err:?}"))?;
                self.same_i32_list_if_present(label, message, "values", &native)
            }
            "vfs_unpack_f32" => {
                let values = f64_list_field(message, "source")?
                    .into_iter()
                    .map(|value| value as f32)
                    .collect::<Vec<_>>();
                let packed = self
                    .interface
                    .vfs()
                    .pack_f32(&values)
                    .map_err(|err| format!("pack_f32() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .unpack_f32(&packed, lua_byte_offset(message)?, unpack_count(message)?)
                    .map_err(|err| format!("unpack_f32() failed: {err:?}"))?;
                self.same_f32_list_if_present(label, message, "values", &native)
            }
            "vfs_zlib_compress" => {
                let input = str_field(message, "input")?;
                let native = self
                    .interface
                    .vfs()
                    .zlib_compress(input.as_bytes())
                    .map_err(|err| format!("zlib_compress() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_zlib_decompress" => {
                let input = str_field(message, "input")?;
                let compressed = self
                    .interface
                    .vfs()
                    .zlib_compress(input.as_bytes())
                    .map_err(|err| format!("zlib_compress() failed: {err:?}"))?;
                let native = self
                    .interface
                    .vfs()
                    .zlib_decompress(&compressed)
                    .map_err(|err| format!("zlib_decompress() failed: {err:?}"))?;
                self.same_string_if_present(label, message, "hex", &byte_hex(&native))
            }
            "vfs_calculate_hash_md5" | "vfs_calculate_hash_sha512" => {
                let input = str_field(message, "input")?;
                let hash_type = i32_field(message, "hashType")?;
                let native = self
                    .interface
                    .vfs()
                    .calculate_hash(input.as_bytes(), hash_type)
                    .map_err(|err| format!("calculate_hash() failed: {err:?}"))?
                    .ok_or_else(|| "calculate_hash() returned no hash".to_string())?;
                self.same_string_if_present(label, message, "hash", &native)
            }
            "vfs_download_archive_invalid_category" | "vfs_download_archive_missing_name" => {
                let filename = str_field(message, "filename")?;
                let category = str_field(message, "category")?;
                if self
                    .interface
                    .vfs()
                    .download_archive(filename, category)
                    .is_ok()
                {
                    return Err(format!(
                        "download_archive({filename:?}, {category:?}) unexpectedly succeeded"
                    ));
                }
                Ok(())
            }
            "vfs_abort_download_missing" => {
                let native = self
                    .interface
                    .vfs()
                    .abort_download(i32_field(message, "id")?)
                    .map_err(|err| format!("abort_download() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "removed", native)
            }
            "vfs_scan_all_dirs" => self
                .interface
                .vfs()
                .scan_all_dirs()
                .map_err(|err| format!("scan_all_dirs() failed: {err:?}")),
            "vfs_create_dir_invalid" => {
                let path = str_field(message, "path")?;
                if self.interface.vfs().create_dir(path).is_ok() {
                    return Err(format!("create_dir({path:?}) unexpectedly succeeded"));
                }
                Ok(())
            }
            "vfs_extract_archive_invalid" => {
                let path = str_field(message, "path")?;
                if self.interface.vfs().extract_mod_archive_file(path).is_ok() {
                    return Err(format!(
                        "extract_mod_archive_file({path:?}) unexpectedly succeeded"
                    ));
                }
                Ok(())
            }
            "vfs_get_map_square_texture_invalid" => {
                let native = self
                    .interface
                    .vfs()
                    .get_map_square_texture(
                        i32_field(message, "texSquareX")?,
                        i32_field(message, "texSquareY")?,
                        i32_field(message, "lodMin")?,
                        str_field(message, "textureName")?,
                        i32_field(message, "lodMax")?,
                    )
                    .map_err(|err| format!("get_map_square_texture() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native)
            }
            "vfs_set_map_square_texture_invalid" | "vfs_set_map_square_texture_default" => {
                let native = self
                    .interface
                    .vfs()
                    .set_map_square_texture(
                        i32_field(message, "texSquareX")?,
                        i32_field(message, "texSquareY")?,
                        str_field(message, "textureName")?,
                    )
                    .map_err(|err| format!("set_map_square_texture() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native)
            }
            _ => {
                let path = str_field(message, "path")?;
                match test_name {
                    "vfs_create_dir_existing" => {
                        let native = self
                            .interface
                            .vfs()
                            .create_dir(path)
                            .map_err(|err| format!("create_dir({path:?}) failed: {err:?}"))?;
                        self.same_bool_if_present(label, message, "created", native)
                    }
                    "vfs_file_exists" | "vfs_missing_file_exists" => {
                        let native = self
                            .interface
                            .vfs()
                            .file_exists(path)
                            .map_err(|err| format!("file_exists({path}) failed: {err:?}"))?;
                        self.same_bool_if_present(label, message, "exists", native)
                    }
                    "vfs_file_size" => {
                        let native = self
                            .interface
                            .vfs()
                            .get_file_size(path)
                            .map_err(|err| format!("get_file_size({path}) failed: {err:?}"))?;
                        self.same_i32_if_present(label, message, "size", native as i32)
                    }
                    "vfs_read_file" => {
                        let native = self
                            .interface
                            .vfs()
                            .read_file(path)
                            .map_err(|err| format!("read_file({path}) failed: {err:?}"))?;
                        self.same_i32_if_present(label, message, "size", native.len() as i32)
                    }
                    "vfs_read_file_as_string" => {
                        let native =
                            self.interface
                                .vfs()
                                .read_file_as_string(path)
                                .map_err(|err| {
                                    format!("read_file_as_string({path}) failed: {err:?}")
                                })?;
                        let size = native.as_deref().map(str::len).unwrap_or(0);
                        self.same_i32_if_present(label, message, "size", size as i32)
                    }
                    "vfs_list_dir_count" => {
                        let pattern = str_field(message, "pattern")?;
                        let native = self
                            .interface
                            .vfs()
                            .list_dir(path, pattern, "", false)
                            .map_err(|err| {
                                format!("list_dir({path}, {pattern}) failed: {err:?}")
                            })?;
                        self.same_i32_if_present(label, message, "count", native.len() as i32)
                    }
                    "vfs_get_all_archives_count" => {
                        let native = self
                            .interface
                            .vfs()
                            .get_archives()
                            .map_err(|err| format!("get_archives() failed: {err:?}"))?;
                        self.same_i32_if_present(label, message, "count", native.len() as i32)
                    }
                    "vfs_get_maps_count" => {
                        let native = self
                            .interface
                            .vfs()
                            .get_maps()
                            .map_err(|err| format!("get_maps() failed: {err:?}"))?;
                        self.same_i32_if_present(label, message, "count", native.len() as i32)
                    }
                    "vfs_get_games_count" => {
                        let native = self
                            .interface
                            .vfs()
                            .get_games()
                            .map_err(|err| format!("get_games() failed: {err:?}"))?;
                        self.same_i32_if_present(label, message, "count", native.len() as i32)
                    }
                    "vfs_is_directory" | "vfs_missing_is_directory" => {
                        let native = self
                            .interface
                            .vfs()
                            .is_directory(path)
                            .map_err(|err| format!("is_directory({path}) failed: {err:?}"))?;
                        self.same_bool_if_present(label, message, "isDirectory", native)
                    }
                    "vfs_file_info" => {
                        let (info, exists) = self
                            .interface
                            .vfs()
                            .get_file_info(path)
                            .map_err(|err| format!("get_file_info({path}) failed: {err:?}"))?;
                        self.same_bool_if_present(label, message, "exists", exists)?;
                        self.same_i32_if_present(label, message, "size", info.size as i32)
                    }
                    _ => Err(format!("unsupported vfs check `{label}`")),
                }
            }
        }
    }
}

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

impl NativeApiParity {
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

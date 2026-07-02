use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_vfs_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        let path = str_field(message, "path")?;
        match base_test_name(label) {
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
                let native = self
                    .interface
                    .vfs()
                    .read_file_as_string(path)
                    .map_err(|err| format!("read_file_as_string({path}) failed: {err:?}"))?;
                let size = native.as_deref().map(str::len).unwrap_or(0);
                self.same_i32_if_present(label, message, "size", size as i32)
            }
            "vfs_list_dir_count" => {
                let pattern = str_field(message, "pattern")?;
                let native = self
                    .interface
                    .vfs()
                    .list_dir(path, pattern, "", false)
                    .map_err(|err| format!("list_dir({path}, {pattern}) failed: {err:?}"))?;
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

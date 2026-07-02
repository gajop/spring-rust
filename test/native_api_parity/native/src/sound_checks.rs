use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_sound_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
            "get_sound_devices_count" => {
                let native = self
                    .interface
                    .sound()
                    .get_sound_devices()
                    .map_err(|err| format!("get_sound_devices() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_sound_stream_time" => {
                let native = self
                    .interface
                    .sound()
                    .get_sound_stream_time()
                    .map_err(|err| format!("get_sound_stream_time() failed: {err:?}"))?;
                self.same_if_present(label, message, "time", native)
            }
            "play_sound_stream_missing" => {
                let ogg_file = str_field(message, "oggFile")?;
                let volume = f32_field(message, "volume")?;
                let enqueue = bool_field(message, "enqueue")?;
                let native = self
                    .interface
                    .sound()
                    .play_sound_stream(ogg_file, volume, enqueue)
                    .map_err(|err| format!("play_sound_stream({ogg_file:?}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native)
            }
            "stop_sound_stream" => {
                self.interface
                    .sound()
                    .stop_sound_stream()
                    .map_err(|err| format!("stop_sound_stream() failed: {err:?}"))?;
                Ok(())
            }
            "pause_sound_stream" => {
                self.interface
                    .sound()
                    .pause_sound_stream()
                    .map_err(|err| format!("pause_sound_stream() failed: {err:?}"))?;
                Ok(())
            }
            "set_sound_stream_volume" => {
                let volume = f32_field(message, "volume")?;
                self.interface
                    .sound()
                    .set_sound_stream_volume(volume)
                    .map_err(|err| format!("set_sound_stream_volume({volume}) failed: {err:?}"))?;
                Ok(())
            }
            "preload_sound_item_missing" => {
                let sound_name = str_field(message, "soundName")?;
                let native = self
                    .interface
                    .sound()
                    .preload_sound_item(sound_name)
                    .map_err(|err| format!("preload_sound_item({sound_name:?}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native)
            }
            "load_sound_def_missing" => {
                let sound_file = str_field(message, "soundFile")?;
                let native = self
                    .interface
                    .sound()
                    .load_sound_def(sound_file)
                    .map_err(|err| format!("load_sound_def({sound_file:?}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native)
            }
            "play_sound_file_missing" => {
                let sound_file = str_field(message, "soundFile")?;
                let volume = f32_field(message, "volume")?;
                let channel = i32_field(message, "channel")?;
                let zero = spring_native::sys::Float3 { x: 0.0, y: 0.0, z: 0.0 };
                let native = self
                    .interface
                    .sound()
                    .play_sound_file(sound_file, volume, zero, zero, channel)
                    .map_err(|err| format!("play_sound_file({sound_file:?}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "success", native)
            }
            _ => Err(format!("unsupported sound check `{label}`")),
        }
    }
}

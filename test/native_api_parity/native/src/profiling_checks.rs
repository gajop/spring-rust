use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_profiling_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "get_profiler_record_names_count" => {
                let native = self
                    .interface
                    .profiling()
                    .get_profiler_record_names()
                    .map_err(|err| format!("get_profiler_record_names() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_lua_mem_usage" => {
                let (
                    _handle_alloced,
                    _handle_allocs,
                    global_alloced,
                    _global_allocs,
                    _unsynced_alloced,
                    _unsynced_allocs,
                    _synced_alloced,
                    _synced_allocs,
                ) = self
                    .interface
                    .profiling()
                    .get_lua_mem_usage()
                    .map_err(|err| format!("get_lua_mem_usage() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasGlobalAlloced", global_alloced > 0.0)
            }
            "diff_timers" => {
                let return_ms = bool_field(message, "returnMs")?;
                let from_micro_secs = bool_field(message, "fromMicroSecs")?;
                let native = self
                    .interface
                    .profiling()
                    .diff_timers(
                        1000,
                        1000,
                        spring_native::DiffTimersOptions {
                            return_ms,
                            from_micro_secs,
                        },
                    )
                    .map_err(|err| format!("diff_timers(1000, 1000, {return_ms}, {from_micro_secs}) failed: {err:?}"))?;
                self.same_if_present(label, message, "delta", native)
            }
            "get_timer" => {
                let native = self
                    .interface
                    .profiling()
                    .get_timer()
                    .map_err(|err| format!("get_timer() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasTimer", native > 0)
            }
            "get_timer_micros" => {
                let native = self
                    .interface
                    .profiling()
                    .get_timer_micros()
                    .map_err(|err| format!("get_timer_micros() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasTimer", native > 0)
            }
            "get_frame_timer" => {
                let last_frame_time = bool_field(message, "lastFrameTime")?;
                let native = self
                    .interface
                    .profiling()
                    .get_frame_timer(last_frame_time)
                    .map_err(|err| format!("get_frame_timer({last_frame_time}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasTimer", native > 0)
            }
            "get_draw_seconds" => {
                let native = self
                    .interface
                    .profiling()
                    .get_draw_seconds()
                    .map_err(|err| format!("get_draw_seconds() failed: {err:?}"))?;
                self.same_if_present(label, message, "seconds", native)
            }
            "get_profiler_time_record_empty" => {
                let name = str_field(message, "profilerName")?;
                let frame_data = bool_field(message, "frameData")?;
                let (total, current, max_dt, time_pct, peak_pct, _frames) = self
                    .interface
                    .profiling()
                    .get_profiler_time_record(name, frame_data)
                    .map_err(|err| {
                        format!("get_profiler_time_record({name}, {frame_data}) failed: {err:?}")
                    })?;
                self.same_if_present(label, message, "total", total)?;
                self.same_if_present(label, message, "current", current)?;
                self.same_if_present(label, message, "maxDt", max_dt)?;
                self.same_if_present(label, message, "timePct", time_pct)?;
                self.same_if_present(label, message, "peakPct", peak_pct)
            }
            "get_vid_mem_usage" => {
                let (used, available) = self
                    .interface
                    .profiling()
                    .get_vid_mem_usage()
                    .map_err(|err| format!("get_vid_mem_usage() failed: {err:?}"))?;
                self.same_if_present(label, message, "usedMB", used)?;
                self.same_if_present(label, message, "availableMB", available)
            }
            "get_synced_gc_info" | "get_synced_gc_info_fixed_available" => {
                let collect = bool_field(message, "collect")?;
                let native = self
                    .interface
                    .profiling()
                    .get_synced_gcinfo(collect)
                    .map_err(|err| format!("get_synced_gcinfo({collect}) failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "hasInfo", native > 0.0)
            }
            _ => Err(format!("unsupported profiling check `{label}`")),
        }
    }
}

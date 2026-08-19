#![no_std]

static mut RESULT_SINK: i32 = 0;

fn on_game_frame(_: i32) {
    // Unit 0 may or may not exist in a particular fixture. Either outcome is
    // useful here: both execute the full NativeInterface query path. Preserve
    // the returned value/error so the optimizer cannot discard the import.
    let value = match spring_wasm_core::get_unit_def_id(0) {
        Ok(unit_def_id) => unit_def_id,
        Err(error) => error.code,
    };
    // SAFETY: synced Core Wasm is single-threaded and this is the only writer.
    unsafe {
        core::ptr::write_volatile(&raw mut RESULT_SINK, value);
    }
}

spring_wasm_core::export_game_frame!(on_game_frame);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

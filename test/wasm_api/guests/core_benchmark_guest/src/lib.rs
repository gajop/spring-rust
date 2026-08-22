#![no_std]

// Keep one byte of guest memory live so the engine exercises the same memory
// discovery path used by real Core modules, without importing any host API.
static mut FRAME_SINK: i32 = 0;

#[export_name = "spring:callin/game-frame"]
pub extern "C" fn game_frame(frame: i32) {
    // SAFETY: WebAssembly execution is single-threaded for the synced profile.
    // Volatile prevents the empty callback from being folded away while adding
    // only one guest memory store to the measured entry.
    unsafe {
        core::ptr::write_volatile(&raw mut FRAME_SINK, frame);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

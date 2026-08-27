#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

// The roundtrip guest itself does not allocate, but the generated SDK contains
// alloc-capable modules. Keep this standalone test guest self-contained rather
// than pulling a runtime allocator into production guests.
struct TestBumpAllocator;

const HEAP_SIZE: usize = 64 * 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static NEXT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TestBumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let base = core::ptr::addr_of_mut!(HEAP).cast::<u8>() as usize;
        let mut current = NEXT.load(Ordering::Relaxed);
        loop {
            let aligned = (base + current + layout.align() - 1) & !(layout.align() - 1);
            let offset = aligned - base;
            let Some(end) = offset.checked_add(layout.size()) else {
                return core::ptr::null_mut();
            };
            if end > HEAP_SIZE {
                return core::ptr::null_mut();
            }
            match NEXT.compare_exchange_weak(current, end, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => return aligned as *mut u8,
                Err(next) => current = next,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: TestBumpAllocator = TestBumpAllocator;

static mut RESULT_SINK: i32 = 0;

fn on_game_frame(_: i32) {
    // Unit 0 may or may not exist in a particular fixture. Either outcome is
    // useful here: both execute the full NativeInterface query path. Preserve
    // the returned value/error so the optimizer cannot discard the import.
    let value = match spring::get_unit_def_id(spring::UnitId::from(0)) {
        Ok(unit_def_id) => i32::from(unit_def_id),
        Err(error) => error.code,
    };
    // SAFETY: synced Core Wasm is single-threaded and this is the only writer.
    unsafe {
        core::ptr::write_volatile(&raw mut RESULT_SINK, value);
    }
}

spring::export_game_frame!(on_game_frame);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

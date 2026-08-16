#![allow(clippy::all)]

mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "rules-synced",
    });
}

use bindings::exports::recoil::spring_api::allocator_fixture::Guest;
use bindings::recoil::spring_api::units_query;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicU32, Ordering};

struct ProbeAllocator;

const PROBE_DISARMED: u32 = 0x1357_2468;
const PROBE_REENTER: u32 = 0xA11C_E001;
const PROBE_TRAP: u32 = 0xA11C_E002;
const PROBE_BURN_FUEL: u32 = 0xA11C_E003;
static PROBE_NEXT_ALLOCATION: AtomicU32 = AtomicU32::new(PROBE_DISARMED);

unsafe impl GlobalAlloc for ProbeAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match PROBE_NEXT_ALLOCATION.swap(PROBE_DISARMED, Ordering::SeqCst) {
            PROBE_REENTER => {
                // This import is deliberately reached from the allocation used
                // by canonical lowering of a variable-size host result. The
                // host must reject it while the original Spring import is
                // guarded.
                let _ = units_query::get_bytes();
            }
            PROBE_TRAP => {
                // Exercise a guest allocator trap at the canonical boundary.
                #[cfg(target_arch = "wasm32")]
                core::arch::wasm32::unreachable();
                #[cfg(not(target_arch = "wasm32"))]
                panic!("allocator trap mode is only valid in the Wasm guest");
            }
            PROBE_BURN_FUEL => loop {
                // The loop is deliberately unbounded. Wasmtime fuel must stop
                // it while canonical lowering is still inside the guarded
                // import boundary.
                core::hint::spin_loop();
            },
            _ => {}
        }
        unsafe { std::alloc::System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { std::alloc::System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { std::alloc::System.realloc(pointer, layout, new_size) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { std::alloc::System.alloc_zeroed(layout) }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: ProbeAllocator = ProbeAllocator;

struct AllocatorGuest;

impl Guest for AllocatorGuest {
    fn probe() -> Vec<u8> {
        PROBE_NEXT_ALLOCATION.store(PROBE_REENTER, Ordering::SeqCst);
        units_query::get_bytes()
    }

    fn trap() -> Vec<u8> {
        PROBE_NEXT_ALLOCATION.store(PROBE_TRAP, Ordering::SeqCst);
        units_query::get_bytes()
    }

    fn burn_fuel() -> Vec<u8> {
        PROBE_NEXT_ALLOCATION.store(PROBE_BURN_FUEL, Ordering::SeqCst);
        units_query::get_bytes()
    }
}

bindings::export!(AllocatorGuest with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::PROBE_NEXT_ALLOCATION;
    use core::sync::atomic::Ordering;

    #[test]
    fn probe_starts_disabled() {
        assert_eq!(
            PROBE_NEXT_ALLOCATION.load(Ordering::SeqCst),
            super::PROBE_DISARMED
        );
    }
}

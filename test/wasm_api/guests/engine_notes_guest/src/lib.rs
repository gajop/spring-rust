// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
#![no_std]

#[link(wasm_import_module = "spring:messages")]
unsafe extern "C" {
    safe fn echo(pointer: i32, length: i32, kind: i32, priority: i32) -> i64;
}
#[link(wasm_import_module = "spring:desync")]
unsafe extern "C" {
    #[link_name = "get-timer"]
    safe fn legacy_timer() -> i64;
    #[link_name = "get-timer-micros"]
    safe fn legacy_micros() -> i64;
}

fn message(value: &str) {
    assert_eq!(echo(value.as_ptr() as usize as i32, value.len() as i32, 0, 0) >> 32, 0);
}

#[cfg(feature = "feature-pool-stress")]
fn create(index: u32) -> i32 {
    // DefRef wire layout: name length, UTF-8, alignment, signed definition ID.
    let definition = [14, 0, 0, 0, b'g', b'e', b'n', b'_', b'a', b's', b't', b'e',
                      b'r', b'o', b'i', b'd', b'_', b'1', 0, 0, 255, 255, 255, 255];
    let mut position = [0u8; 12];
    position[..4].copy_from_slice(&(10000.0 + (index % 128) as f32 * 64.0).to_le_bytes());
    position[4..8].copy_from_slice(&1000.0f32.to_le_bytes());
    position[8..].copy_from_slice(&(10000.0 + (index / 128) as f32 * 64.0).to_le_bytes());
    spring::generated::dynamic_input::feature_control::create_feature(
        0, -1, -1, &definition, &position).unwrap()
}

#[cfg(feature = "feature-pool-stress")]
static mut LAST_FEATURE: i32 = -1;

fn on_frame(frame: i32) {
    // Neither the current Result API nor old scalar timer imports may fault
    // the live rules module just because optional profiling is disabled.
    assert_eq!(spring::desync::get_timer().unwrap_err().code, 4);
    assert_eq!(spring::desync::get_timer_micros().unwrap_err().code, 4);
    assert_eq!(legacy_timer(), 0);
    assert_eq!(legacy_micros(), 0);
    assert_eq!(spring::desync::diff_timers(0, 0, false, true).unwrap(), 0.0);
    if frame == 0 {
        message("ENGINE_NOTES_TIMERS_DISABLED_PASS");
        #[cfg(feature = "feature-pool-stress")]
        {
            let mut created = 0;
            for index in 0..=32000 {
                let id = create(index);
                if id < 0 { break; }
                unsafe { LAST_FEATURE = id; }
                created += 1;
            }
            assert!(created > 0 && created <= 32000);
            assert_eq!(create(32001), -1);
            assert_eq!(create(32002), -1);
            message("ENGINE_NOTES_FEATURE_OVERFLOW_PASS");
            assert!(spring::generated::feature_control::destroy_feature(unsafe { LAST_FEATURE }).unwrap());
        }
    }
    #[cfg(feature = "feature-pool-stress")]
    if frame == 33 {
        // Feature IDs are deliberately recycled on the 32-frame maintenance tick.
        assert!(create(0) >= 0); // reclaiming one pool page makes allocation possible
        assert_eq!(create(1), -1);
        message("ENGINE_NOTES_FEATURE_REUSE_PASS");
    }
    if frame == 36 { message("ENGINE_NOTES_RULES_STILL_RUNNING_PASS"); }
}

spring::export_environment_mask!(1u32);
spring::export_game_frame!(on_frame);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

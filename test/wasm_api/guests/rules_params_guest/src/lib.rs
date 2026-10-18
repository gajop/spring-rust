// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
#![no_std]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

// The regression runs once; a small bump allocator keeps the guest self-contained.
struct TestBumpAllocator;

const HEAP_SIZE: usize = 1024 * 1024;
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

use spring::rules_params::*;
use spring::{ApiError, ErrorCode, Result};

fn check_scope(
    get: impl Fn(&str) -> Result<RulesParamValue>,
    set: impl Fn(&str, &RulesParamValue, i32) -> Result<bool>,
) {
    assert_eq!(
        get("missing").unwrap_err(),
        ApiError::new(ErrorCode::NotFound as i32)
    );
    for value in [
        RulesParamValue {
            type_: RulesParamType::RulesparamTypeBool,
            bool_value: true,
            float_value: 0.0,
            string_value: alloc::string::String::new(),
        },
        RulesParamValue {
            type_: RulesParamType::RulesparamTypeFloat,
            bool_value: false,
            float_value: -42.25,
            string_value: alloc::string::String::new(),
        },
        RulesParamValue {
            type_: RulesParamType::RulesparamTypeString,
            bool_value: false,
            float_value: 0.0,
            string_value: alloc::string::String::new(),
        },
        RulesParamValue {
            type_: RulesParamType::RulesparamTypeString,
            bool_value: false,
            float_value: 0.0,
            string_value: "long Unicode string: 船🚀".repeat(300),
        },
    ] {
        assert!(set("roundtrip", &value, los::PUBLIC).unwrap());
        assert_eq!(get("roundtrip").unwrap(), value);
        assert_eq!(get("roundtrip").unwrap(), value); // cached size and owned string lifetime
    }
    let hidden = RulesParamValue {
        type_: RulesParamType::RulesparamTypeFloat,
        bool_value: false,
        float_value: 7.0,
        string_value: alloc::string::String::new(),
    };
    assert!(set("hidden", &hidden, 0).unwrap());
    // This fixture is synced. UI visibility masks must not hide its own
    // private parameters from the simulation.
    assert_eq!(get("hidden").unwrap(), hidden);
}

fn on_frame(frame: i32) {
    if frame != 1 {
        return;
    }
    let unit = get_game_rules_param_f32("fixture_unit").unwrap() as i32;
    let feature = get_game_rules_param_f32("fixture_feature").unwrap() as i32;
    check_scope(get_game_rules_param, set_game_rules_param);
    check_scope(
        |name| get_team_rules_param(0, name),
        |name, value, los| set_team_rules_param(0, name, value, los),
    );
    check_scope(
        |name| get_player_rules_param(0, name),
        |name, value, los| set_player_rules_param(0, name, value, los),
    );
    check_scope(
        |name| get_feature_rules_param(feature, name),
        |name, value, los| set_feature_rules_param(feature, name, value, los),
    );
    check_scope(
        |name| get_unit_rules_param(unit, name),
        |name, value, los| set_unit_rules_param(unit, name, value, los),
    );
    assert_eq!(
        get_game_rules_param_f32("missing").unwrap_err().code,
        ErrorCode::NotFound as i32
    );
    assert_eq!(get_game_rules_param_f32("hidden").unwrap(), 7.0);
    assert_eq!(
        get_game_rules_param_f32("roundtrip").unwrap_err().code,
        ErrorCode::InvalidArgument as i32
    );
    let value = RulesParamValue {
        type_: RulesParamType::RulesparamTypeBool,
        bool_value: true,
        float_value: 0.0,
        string_value: alloc::string::String::new(),
    };
    assert!(set_game_rules_param("float_probe", &value, los::PUBLIC).unwrap());
    assert_eq!(
        get_game_rules_param_f32("float_probe").unwrap_err().code,
        ErrorCode::InvalidArgument as i32
    );
    let value = RulesParamValue {
        type_: RulesParamType::RulesparamTypeFloat,
        float_value: -42.25,
        ..value
    };
    assert!(set_game_rules_param("float_probe", &value, los::PUBLIC).unwrap());
    assert_eq!(get_game_rules_param_f32("float_probe").unwrap(), -42.25);
    assert_eq!(
        get_game_rules_param("float_probe").unwrap().float_value,
        -42.25
    );
    assert_eq!(get_unit_rules_param_f32(unit, "hidden").unwrap(), 7.0);
    assert_eq!(
        get_team_rules_param(-1, "roundtrip").unwrap_err().code,
        ErrorCode::InvalidArgument as i32
    );
    spring::messages::echo("RULES_PARAMS_TYPED_PASS", "").unwrap();
}

spring::export_environment_mask!(1u32);
spring::export_game_frame!(on_frame);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    let _ = spring::messages::echo(&alloc::format!("RULES_PARAMS_TYPED_FAIL: {info}"), "");
    core::arch::wasm32::unreachable()
}

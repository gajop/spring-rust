// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
#![no_std]

use spring::jobs::{JobError, JobId};

static mut INPUT: [u8; 16] = [0; 16];
static mut OUTPUT: [u8; 12] = [0; 12];
static mut CALLS: u32 = 0;
static mut FRAME: u32 = 0;
static mut LAST_ID: u32 = 0;
static mut DELIVERED: u32 = 0;
static mut DIGEST: u32 = 0;

fn calculate(mut seed: u32, iterations: u32) -> u32 {
    for _ in 0..iterations {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    }
    seed
}

#[unsafe(export_name = "SPRING_JOB_INPUT_BUFFER")]
pub extern "C" fn input_buffer() -> i32 {
    core::ptr::addr_of_mut!(INPUT).cast::<u8>() as usize as i32
}

#[unsafe(export_name = "SPRING_JOB_INPUT_CAPACITY")]
pub extern "C" fn input_capacity() -> i32 {
    16
}

#[unsafe(export_name = "work")]
pub extern "C" fn work(pointer: i32, len: i32) -> i64 {
    assert_eq!(pointer, input_buffer());
    assert_eq!(len, 8);
    // The engine copied into this fresh instance's private input buffer.
    // No guest threads, engine calls, or references to the live rules state.
    unsafe {
        let input = core::ptr::addr_of!(INPUT).read();
        let seed = u32::from_le_bytes(input[..4].try_into().unwrap());
        let iterations = u32::from_le_bytes(input[4..8].try_into().unwrap());
        CALLS += 1;
        let mut output = [0u8; 12];
        output[..4].copy_from_slice(&calculate(seed, iterations).to_le_bytes());
        output[4..8].copy_from_slice(&CALLS.to_le_bytes());
        output[8..12].copy_from_slice(&seed.to_le_bytes());
        core::ptr::addr_of_mut!(OUTPUT).write(output);
    }
    let pointer = core::ptr::addr_of!(OUTPUT).cast::<u8>() as usize as u32;
    ((12u64 << 32) | pointer as u64) as i64
}

fn on_result(id: JobId, result: core::result::Result<&[u8], JobError>) {
    let bytes = result.unwrap();
    assert_eq!(bytes.len(), 12);
    let value = u32::from_le_bytes(bytes[..4].try_into().unwrap());
    let calls = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    let seed = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    // All callbacks are serialized on this instance's simulation thread.
    unsafe {
        assert_eq!(id.0, LAST_ID + 1);
        let index = (id.0 - 1) % 2;
        assert_eq!(seed, FRAME * 2 + index);
        assert_eq!(calls, 1);
        assert_eq!(value, calculate(seed, (index + 1) * 20000));
        LAST_ID = id.0;
        DELIVERED += 1;
        DIGEST ^= value;
    }
}

fn on_frame(frame: i32) {
    let frame = frame as u32;
    unsafe {
        let delivered = DELIVERED;
        let last_id = LAST_ID;
        assert_eq!(delivered, if frame == 0 { 0 } else { 2 });
        assert_eq!(last_id, frame * 2);
        DELIVERED = 0;
        FRAME = frame;
    }
    for index in 0..2 {
        let seed = frame * 2 + index;
        let mut input = [0u8; 8];
        input[..4].copy_from_slice(&seed.to_le_bytes());
        input[4..].copy_from_slice(&((index + 1) * 20000).to_le_bytes());
        let id = if index == 0 {
            spring::jobs::submit("work", &input)
        } else {
            let module = if cfg!(feature = "external-worker") {
                "sync-jobs-worker"
            } else {
                "sync-jobs-test"
            };
            spring::jobs::submit_to(module, "work", &input)
        }
        .unwrap();
        assert_eq!(id.0, seed + 1);
        input.fill(255);
        core::hint::black_box(input);
    }
}

// RulesSynced bit, usable without enabling the SDK's allocation APIs.
spring::export_environment_mask!(1u32);
spring::export_game_frame!(on_frame);
spring::export_job_results!(on_result, 64);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
#![no_std]

use spring::jobs::{JobError, JobId};

#[cfg(not(feature = "retain-borrowed-result"))]
fn handler(_: JobId, _: core::result::Result<&[u8], JobError>) {}

#[cfg(feature = "retain-borrowed-result")]
fn handler(_: JobId, _: core::result::Result<&'static [u8], JobError>) {}

spring::export_job_results!(handler, 64);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
//! Jobs in fresh private instances, available in every Core environment.
//!
//! Synced results arrive before GameFrame in the next simulation frame, in global
//! submission order. Local/UI/menu results arrive before the next Update after
//! sealing; Intro uses loading-screen updates. Local IDs/limits are per owner.
//! Job exports receive copied bytes and must not call engine imports.

use crate::{ApiError, ErrorCode, Result, wasm_slice_parts};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct JobId(pub u32);

/// A worker execution failure, delivered deterministically rather than faulting
/// the rules instance. Host resource failures terminate simulation separately.
#[cfg(feature = "alloc")]
pub use crate::job_batch::JobError;
#[cfg(not(feature = "alloc"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobError(pub u32);

mod raw {
    #[link(wasm_import_module = "spring:jobs")]
    unsafe extern "C" {
        #[link_name = "submit"]
        pub safe fn submit(name: i32, name_bytes: i32, input: i32, input_bytes: i32) -> i64;
        #[link_name = "submit-to"]
        pub safe fn submit_to(
            module: i32,
            module_bytes: i32,
            name: i32,
            name_bytes: i32,
            input: i32,
            input_bytes: i32,
        ) -> i64;
        #[cfg(feature = "alloc")]
        #[link_name = "run-batch-to"]
        pub safe fn run_batch_to(
            module: i32,
            module_bytes: i32,
            name: i32,
            name_bytes: i32,
            input: i32,
            input_bytes: i32,
            output: i32,
            output_bytes: i32,
        ) -> i64;
    }
}

/// Synchronous ordered pure jobs, returning owned bytes directly. An empty
/// module name means this module. No result scratch export/callback is needed.
#[cfg(feature = "alloc")]
pub fn run_batch_to(
    module_name: &str,
    export_name: &str,
    jobs: &[crate::job_batch::BatchJob<'_>],
) -> Result<alloc::vec::Vec<crate::job_batch::OwnedJobResult>> {
    if module_name.len() > 256 || export_name.is_empty() || export_name.len() > 256 {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let (input, capacity) = crate::job_batch::encode(jobs)?;
    let mut output = alloc::vec![0u8; capacity];
    let (module, module_bytes) = wasm_slice_parts(module_name.as_bytes())?;
    let (name, name_bytes) = wasm_slice_parts(export_name.as_bytes())?;
    let (input, input_bytes) = wasm_slice_parts(&input)?;
    let (pointer, output_bytes) = crate::wasm_mut_slice_parts(&mut output)?;
    let packed = raw::run_batch_to(
        module,
        module_bytes,
        name,
        name_bytes,
        input,
        input_bytes,
        pointer,
        output_bytes,
    ) as u64;
    let status = (packed >> 32) as i32;
    if status != 0 {
        return Err(ApiError::new(status));
    }
    let size = packed as u32 as usize;
    if size > output.len() {
        return Err(ApiError::new(ErrorCode::InvalidState as i32));
    }
    output.truncate(size);
    crate::job_batch::decode(&output, jobs)
}

/// Submit a named `(input_ptr: i32, input_len: i32) -> i64` worker export.
/// The output packs a u32 pointer in the low word and byte length in the high
/// word. The engine copies inputs now and copies results back next frame.
///
/// Call `export_job_results!(handler, capacity)` once in the submitting module.
/// Export `SPRING_JOB_INPUT_BUFFER` and `SPRING_JOB_INPUT_CAPACITY` for workers.
#[inline]
pub fn submit(export_name: &str, input: &[u8]) -> Result<JobId> {
    if export_name.is_empty() || export_name.len() > 256 || input.len() > 1024 * 1024 {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let (name, name_bytes) = wasm_slice_parts(export_name.as_bytes())?;
    let (input, input_bytes) = wasm_slice_parts(input)?;
    let packed = raw::submit(name, name_bytes, input, input_bytes) as u64;
    let status = (packed >> 32) as i32;
    if status != 0 {
        return Err(ApiError::new(status));
    }
    Ok(JobId(packed as u32))
}

/// Run an export from another loaded compatible module, retaining results in the
/// submitting module. This lets rules use a small dedicated navigation worker
/// instead of instantiating the entire game's large linear memory per job.
/// Synced owners can target either synced world; local owners target their own
/// environment. Workers in every world are forbidden from calling host imports.
#[inline]
pub fn submit_to(module_name: &str, export_name: &str, input: &[u8]) -> Result<JobId> {
    if module_name.is_empty()
        || module_name.len() > 256
        || export_name.is_empty()
        || export_name.len() > 256
        || input.len() > 1024 * 1024
    {
        return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
    }
    let (module, module_bytes) = wasm_slice_parts(module_name.as_bytes())?;
    let (name, name_bytes) = wasm_slice_parts(export_name.as_bytes())?;
    let (input, input_bytes) = wasm_slice_parts(input)?;
    let packed = raw::submit_to(module, module_bytes, name, name_bytes, input, input_bytes) as u64;
    let status = (packed >> 32) as i32;
    if status != 0 {
        return Err(ApiError::new(status));
    }
    Ok(JobId(packed as u32))
}

/// Export the result scratch buffer and a callback taking
/// `(JobId, core::result::Result<&[u8], JobError>)`. The borrowed bytes are valid
/// only during the callback; copy anything needed later into game state.
#[macro_export]
macro_rules! export_job_results {
    ($handler:path, $capacity:expr) => {
        #[cfg(target_arch = "wasm32")]
        static mut __SPRING_JOB_RESULTS: [u8; $capacity] = [0; $capacity];

        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "SPRING_JOB_RESULT_BUFFER")]
        pub extern "C" fn __spring_job_result_buffer() -> i32 {
            core::ptr::addr_of_mut!(__SPRING_JOB_RESULTS) as *mut u8 as usize as i32
        }

        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "SPRING_JOB_RESULT_CAPACITY")]
        pub extern "C" fn __spring_job_result_capacity() -> i32 {
            const {
                assert!($capacity > 0 && $capacity <= 1024 * 1024);
            }
            $capacity as i32
        }

        #[cfg(target_arch = "wasm32")]
        #[unsafe(export_name = "spring:jobs/result")]
        pub extern "C" fn __spring_job_result(id: i32, status: i32, pointer: i32, len: i32) {
            // A handler requiring &'static bytes must not type-check: the
            // engine reuses scratch after this callback. Coerce to a function
            // accepting any borrowed lifetime before constructing the slice.
            let handler: for<'result> fn(
                $crate::jobs::JobId,
                core::result::Result<&'result [u8], $crate::jobs::JobError>,
            ) = $handler;
            if status != 0 {
                handler(
                    $crate::jobs::JobId(id as u32),
                    Err($crate::jobs::JobError(status as u32)),
                );
                return;
            }
            assert_eq!(pointer, __spring_job_result_buffer());
            assert!(len >= 0 && len as usize <= $capacity);
            // The engine has returned from all workers and written this private
            // buffer. Results never re-enter another result callback, and the
            // guest has no threads or shared memory.
            let bytes =
                unsafe { core::slice::from_raw_parts(pointer as u32 as *const u8, len as usize) };
            handler($crate::jobs::JobId(id as u32), Ok(bytes));
        }
    };
}

/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Pure byte-in/byte-out workers on native and Wasm. Declare workers with
//! `export_jobs!`. Use `spring.jobs().run_this_tick(...)` for ordered results
//! before returning, or `run_next_tick(...)` for delivery to `Module::job_result`
//! before the next tick. Workers must not read mutable globals or use engine
//! imports. Wasm runs each job in a fresh instance; native runs a plain function
//! on a bounded thread pool. Pass every input explicitly, including random seeds.
//!
//! Export GameFrame for synced modules and Update for other environments. Work
//! submitted in a tick is sealed after it, and results arrive before the next
//! tick, in submission order. Native loading currently supports Menu and All;
//! their jobs use Update. Shutdown waits for workers before releasing module code.

use crate::{ApiError, Environment, ErrorCode, Result, Spring};
pub use spring_sdk_internal::job_batch::{BatchJob, JobError, OwnedJobResult};
#[cfg(target_arch = "wasm32")]
pub use spring_sdk_internal::jobs::JobId;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct JobId(pub u32);
pub type JobResult<'a> = core::result::Result<&'a [u8], JobError>;

pub const MAX_BYTES: usize = 1024 * 1024;
#[cfg(not(target_arch = "wasm32"))]
const MAX_JOBS: usize = 256;
#[cfg(not(target_arch = "wasm32"))]
const MAX_BATCH_BYTES: usize = 16 * MAX_BYTES;

/// A declared pure worker. Use `export_jobs!` so its name is also exported in
/// Wasm. The function cannot capture a Spring handle or application state.
#[derive(Clone, Copy)]
pub struct Worker {
    name: &'static str,
    #[cfg(not(target_arch = "wasm32"))]
    run: fn(&[u8]) -> Vec<u8>,
    wasm_module: Option<&'static str>,
}
impl Worker {
    #[doc(hidden)]
    pub const fn new(name: &'static str, run: fn(&[u8]) -> Vec<u8>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let _ = run;
        Self {
            name,
            #[cfg(not(target_arch = "wasm32"))]
            run,
            wasm_module: None,
        }
    }

    /// Use an export from a small, separately loaded Wasm worker module instead
    /// of cloning the submitting module's memory. Native uses the worker's
    /// linked function; the module name describes only its Wasm deployment.
    pub const fn in_wasm_module(mut self, name: &'static str) -> Self {
        self.wasm_module = Some(name);
        self
    }
}

pub struct Jobs<'a, 'call, E: Environment>(&'a Spring<'call, E>);
impl<'call, E: Environment> Spring<'call, E> {
    pub fn jobs(&self) -> Jobs<'_, 'call, E> {
        Jobs(self)
    }
}
impl<E: Environment> Jobs<'_, '_, E> {
    /// Execute pure jobs now and join the whole batch before returning owned
    /// results in input order. No `Module::job_result` callback runs here; queued
    /// next-tick results keep their ordinary delivery boundary.
    ///
    /// Each input/output bound is at most 1 MiB. A tick admits 256 total jobs
    /// and 16 MiB total input across synchronous and deferred submissions, plus
    /// 16 MiB synchronous output reservations. Invalid/over-quota batches admit
    /// nothing. Guest traps/import attempts/oversized outputs occupy their own
    /// `Err(JobError)` slots. Host resource failure returns an API error/trap.
    pub fn run_this_tick(
        &self,
        worker: Worker,
        jobs: &[BatchJob<'_>],
    ) -> Result<Vec<OwnedJobResult>> {
        self.0.engine.check()?;
        if worker.name.is_empty()
            || worker.name.len() > 256
            || worker
                .wasm_module
                .is_some_and(|name| name.is_empty() || name.len() > 256)
        {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        spring_sdk_internal::job_batch::bounds(jobs)?;
        #[cfg(target_arch = "wasm32")]
        let result = spring_sdk_internal::jobs::run_batch_to(
            worker.wasm_module.unwrap_or(""),
            worker.name,
            jobs,
        );
        #[cfg(not(target_arch = "wasm32"))]
        let result = self
            .0
            .engine
            .owner
            .jobs
            .try_borrow_mut()
            .map_err(|_| ApiError::new(ErrorCode::InvalidState as i32))?
            .run_batch(worker, jobs);
        self.0.engine.finish(result)
    }

    /// Queue a pure worker; deliver its result to `Module::job_result` before
    /// the next tick, in submission order. For synced code a tick is a GameFrame;
    /// for unsynced code it is an Update.
    pub fn run_next_tick(&self, worker: Worker, input: &[u8]) -> Result<JobId> {
        self.0.engine.check()?;
        if worker.name.is_empty()
            || worker.name.len() > 256
            || input.len() > MAX_BYTES
            || worker
                .wasm_module
                .is_some_and(|name| name.is_empty() || name.len() > 256)
        {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        #[cfg(target_arch = "wasm32")]
        let result = match worker.wasm_module {
            Some(module) => spring_sdk_internal::jobs::submit_to(module, worker.name, input),
            None => spring_sdk_internal::jobs::submit(worker.name, input),
        };
        #[cfg(not(target_arch = "wasm32"))]
        let result = self.0.engine.owner.jobs.borrow_mut().submit(worker, input);
        self.0.engine.finish(result)
    }

    /// Compatibility alias for [`Self::run_next_tick`].
    #[deprecated(note = "use run_next_tick for explicit result delivery timing")]
    pub fn submit(&self, worker: Worker, input: &[u8]) -> Result<JobId> {
        self.run_next_tick(worker, input)
    }

    /// Compatibility alias for [`Self::run_this_tick`].
    #[deprecated(note = "use run_this_tick for explicit result delivery timing")]
    pub fn run_batch(&self, worker: Worker, jobs: &[BatchJob<'_>]) -> Result<Vec<OwnedJobResult>> {
        self.run_this_tick(worker, jobs)
    }
}

/// Declare worker tokens and the Wasm worker/result exports once per module.
/// Native calls the same pure functions on background threads. Implement
/// `Module::job_result` to consume results with a fresh Spring reference.
///
/// ```ignore
/// fn double(bytes: &[u8]) -> Vec<u8> { bytes.iter().map(|b| b * 2).collect() }
/// spring_sdk::export_jobs!(APP, { DOUBLE => ("double", double) });
/// // Inside a callin:
/// let id = spring.jobs().run_next_tick(DOUBLE, &[1, 2, 3])?;
/// ```
#[macro_export]
macro_rules! export_jobs {
    ($slot:ident, {$($token:ident => ($name:literal, $worker:path)),+ $(,)?}) => {
        $(pub const $token: $crate::jobs::Worker = $crate::jobs::Worker::new($name, $worker);)+
        #[cfg(target_arch="wasm32")]
        const _: () = {
            static mut INPUT: [u8; $crate::jobs::MAX_BYTES] = [0; $crate::jobs::MAX_BYTES];
            static mut OUTPUT: [u8; $crate::jobs::MAX_BYTES] = [0; $crate::jobs::MAX_BYTES];
            #[unsafe(export_name="SPRING_JOB_INPUT_BUFFER")]
            extern "C" fn input_buffer() -> i32 {
                core::ptr::addr_of_mut!(INPUT).cast::<u8>() as usize as i32
            }
            #[unsafe(export_name="SPRING_JOB_INPUT_CAPACITY")]
            extern "C" fn input_capacity() -> i32 { $crate::jobs::MAX_BYTES as i32 }
            #[unsafe(export_name="SPRING_JOB_RESULT_BUFFER")]
            extern "C" fn result_buffer() -> i32 {
                core::ptr::addr_of_mut!(OUTPUT).cast::<u8>() as usize as i32
            }
            #[unsafe(export_name="SPRING_JOB_RESULT_CAPACITY")]
            extern "C" fn result_capacity() -> i32 { $crate::jobs::MAX_BYTES as i32 }
            #[unsafe(export_name="spring:jobs/result")]
            extern "C" fn result(id: i32, status: i32, pointer: i32, len: i32) {
                assert_eq!(pointer, result_buffer());
                assert!(len >= 0 && len as usize <= $crate::jobs::MAX_BYTES);
                let bytes = unsafe { core::slice::from_raw_parts(pointer as u32 as *const u8, len as usize) };
                let value = if status == 0 { Ok(bytes) } else { Err($crate::jobs::JobError(status as u32)) };
                $crate::module::wasm_boundary($slot.with(|slot| slot.deliver_job_result($crate::jobs::JobId(id as u32), value)));
            }
            $(const _: () = {
                #[unsafe(export_name=$name)]
                extern "C" fn run(pointer: i32, len: i32) -> i64 {
                    assert_eq!(pointer, input_buffer());
                    assert!(len >= 0 && len as usize <= $crate::jobs::MAX_BYTES);
                    let input = unsafe { core::slice::from_raw_parts(pointer as u32 as *const u8, len as usize) };
                    let output = ($worker)(input);
                    if output.len() > $crate::jobs::MAX_BYTES {
                        // An out-of-bounds span reports InvalidOutput through
                        // the worker ABI, rather than faulting the owner.
                        return ((u32::MAX as u64) | ((u32::MAX as u64) << 32)) as i64;
                    }
                    let len = output.len();
                    unsafe { core::ptr::copy_nonoverlapping(output.as_ptr(), core::ptr::addr_of_mut!(OUTPUT).cast::<u8>(), len); }
                    ((result_buffer() as u32 as u64) | ((len as u64) << 32)) as i64
                }
            };)+
        };
    };
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native {
    use super::*;
    use std::sync::{Arc, Mutex, mpsc};

    type Outcome = core::result::Result<Vec<u8>, JobError>;
    type Delivery = (JobId, Outcome);
    struct Work {
        worker: Worker,
        input: Vec<u8>,
        result: mpsc::SyncSender<Outcome>,
        max_output_bytes: usize,
    }
    struct Pool {
        sender: Option<mpsc::Sender<Work>>,
        threads: Vec<std::thread::JoinHandle<()>>,
    }
    impl Pool {
        fn new() -> Result<Self> {
            let (sender, receiver) = mpsc::channel::<Work>();
            let receiver = Arc::new(Mutex::new(receiver));
            let count = std::thread::available_parallelism().map_or(1, |n| n.get().min(4));
            let mut pool = Self {
                sender: Some(sender),
                threads: Vec::new(),
            };
            for _ in 0..count {
                let receiver = receiver.clone();
                let thread = std::thread::Builder::new()
                    .name("spring-sdk-job".into())
                    .spawn(move || {
                        loop {
                            let work = receiver.lock().unwrap().recv();
                            let Ok(work) = work else {
                                break;
                            };
                            let result =
                                std::panic::catch_unwind(|| (work.worker.run)(&work.input))
                                    .map_err(|_| JobError(2))
                                    .and_then(|bytes| {
                                        if bytes.len() <= work.max_output_bytes {
                                            Ok(bytes)
                                        } else {
                                            Err(JobError(4))
                                        }
                                    });
                            let _ = work.result.send(result);
                        }
                    })
                    .map_err(|_| ApiError::new(ErrorCode::InvalidState as i32))?;
                pool.threads.push(thread);
            }
            Ok(pool)
        }
    }
    impl Drop for Pool {
        fn drop(&mut self) {
            self.sender.take();
            for thread in self.threads.drain(..) {
                let _ = thread.join();
            }
        }
    }

    pub(crate) struct State {
        pending: Vec<(JobId, Worker, Vec<u8>)>,
        active: Vec<(JobId, mpsc::Receiver<Outcome>)>,
        pool: Option<Pool>,
        bytes: usize,
        next_id: u32,
        in_tick: bool,
        batch_jobs: usize,
        batch_bytes: usize,
        batch_output: usize,
    }
    impl Default for State {
        fn default() -> Self {
            Self {
                pending: Vec::new(),
                active: Vec::new(),
                pool: None,
                bytes: 0,
                next_id: 1,
                in_tick: false,
                batch_jobs: 0,
                batch_bytes: 0,
                batch_output: 0,
            }
        }
    }
    pub(crate) fn is_phase<E: Environment>(phase: u8) -> bool {
        if E::MASK == crate::RulesSynced::MASK || E::MASK == crate::GaiaSynced::MASK {
            phase == 1
        } else {
            phase == 2
        }
    }
    impl State {
        pub(crate) fn advancing(&self) -> bool {
            self.in_tick
        }
        pub(crate) fn submit(&mut self, worker: Worker, input: &[u8]) -> Result<JobId> {
            if self.next_id == 0
                || self.pending.len() + self.batch_jobs >= MAX_JOBS
                || input.len() > MAX_BATCH_BYTES - self.bytes - self.batch_bytes
            {
                return Err(ApiError::new(ErrorCode::BufferOverflow as i32));
            }
            let id = JobId(self.next_id);
            self.next_id = self.next_id.wrapping_add(1);
            self.pending.push((id, worker, input.to_vec()));
            self.bytes += input.len();
            Ok(id)
        }
        pub(crate) fn run_batch(
            &mut self,
            worker: Worker,
            jobs: &[BatchJob<'_>],
        ) -> Result<Vec<OwnedJobResult>> {
            spring_sdk_internal::job_batch::bounds(jobs)?;
            let input_bytes: usize = jobs.iter().map(|job| job.input.len()).sum();
            let output_bytes: usize = jobs.iter().map(|job| job.max_output_bytes).sum();
            if jobs.len() > MAX_JOBS - self.pending.len() - self.batch_jobs
                || input_bytes > MAX_BATCH_BYTES - self.bytes - self.batch_bytes
                || output_bytes > MAX_BATCH_BYTES - self.batch_output
            {
                return Err(ApiError::new(ErrorCode::BufferOverflow as i32));
            }
            // Own every input/result channel and initialize the pool before
            // atomically reserving this complete batch. Never drain `active`.
            let (work, receivers): (Vec<_>, Vec<_>) = jobs
                .iter()
                .map(|job| {
                    let (result, receiver) = mpsc::sync_channel(1);
                    (
                        Work {
                            worker,
                            input: job.input.to_vec(),
                            result,
                            max_output_bytes: job.max_output_bytes,
                        },
                        receiver,
                    )
                })
                .unzip();
            if self.pool.is_none() {
                self.pool = Some(Pool::new()?);
            }
            self.batch_jobs += jobs.len();
            self.batch_bytes += input_bytes;
            self.batch_output += output_bytes;
            for job in work {
                self.pool
                    .as_ref()
                    .unwrap()
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(job)
                    .map_err(|_| ApiError::new(ErrorCode::InvalidState as i32))?;
            }
            receivers
                .into_iter()
                .map(|receiver| {
                    receiver
                        .recv()
                        .map_err(|_| ApiError::new(ErrorCode::InvalidState as i32))
                })
                .collect()
        }
        pub(crate) fn begin(&mut self) -> Result<Vec<Delivery>> {
            if self.in_tick {
                return Err(ApiError::new(ErrorCode::InvalidState as i32));
            }
            self.in_tick = true;
            self.active
                .drain(..)
                .map(|(id, receiver)| {
                    let outcome = receiver
                        .recv()
                        .map_err(|_| ApiError::new(ErrorCode::InvalidState as i32))?;
                    Ok((id, outcome))
                })
                .collect()
        }
        pub(crate) fn end(&mut self) -> Result<()> {
            if !self.in_tick || !self.active.is_empty() {
                return Err(ApiError::new(ErrorCode::InvalidState as i32));
            }
            if !self.pending.is_empty() && self.pool.is_none() {
                self.pool = Some(Pool::new()?);
            }
            for (id, worker, input) in self.pending.drain(..) {
                let (sender, receiver) = mpsc::sync_channel(1);
                self.pool
                    .as_ref()
                    .unwrap()
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(Work {
                        worker,
                        input,
                        result: sender,
                        max_output_bytes: MAX_BYTES,
                    })
                    .map_err(|_| ApiError::new(ErrorCode::InvalidState as i32))?;
                self.active.push((id, receiver));
            }
            self.bytes = 0;
            self.batch_jobs = 0;
            self.batch_bytes = 0;
            self.batch_output = 0;
            self.in_tick = false;
            Ok(())
        }
        pub(crate) fn shutdown(&mut self) {
            self.pending.clear();
            self.pool.take(); // joins every worker before module code may unload
            self.active.clear();
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::{
        ModuleContext, Ui,
        module::{Module, ModuleSlot},
    };
    use std::cell::RefCell;

    const DOUBLE: Worker = Worker::new("double", |bytes| bytes.iter().map(|b| b * 2).collect());

    #[test]
    fn synchronous_batches_are_ordered_owned_and_do_not_consume_deferred_results() {
        let mut queue = native::State::default();
        let deferred = queue.submit(DOUBLE, &[9]).unwrap();
        queue.begin().unwrap();
        queue.end().unwrap();
        let worker = Worker::new("skewed", |bytes| {
            if bytes == [0] {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            bytes.to_vec()
        });
        let inputs: Vec<_> = (0..24).map(|i| vec![i]).collect();
        let jobs: Vec<_> = inputs
            .iter()
            .map(|input| BatchJob {
                input,
                max_output_bytes: 1,
            })
            .collect();
        assert_eq!(
            queue.run_batch(worker, &jobs).unwrap(),
            inputs.iter().cloned().map(Ok).collect::<Vec<_>>()
        );
        assert_eq!(queue.begin().unwrap(), [(deferred, Ok(vec![18]))]);
        queue.end().unwrap();
        assert!(queue.begin().unwrap().is_empty());
        queue.end().unwrap();
    }

    #[test]
    fn synchronous_preflight_rejects_whole_batch_and_resets_quotas_at_tick_boundary() {
        let mut queue = native::State::default();
        let job = BatchJob {
            input: &[3],
            max_output_bytes: 1,
        };
        for _ in 0..MAX_JOBS - 1 {
            queue.submit(DOUBLE, &[]).unwrap();
        }
        assert_eq!(
            queue.run_batch(DOUBLE, &[job; 2]).unwrap_err().code,
            ErrorCode::BufferOverflow as i32
        );
        assert_eq!(queue.run_batch(DOUBLE, &[job]).unwrap(), [Ok(vec![6])]);
        assert!(queue.submit(DOUBLE, &[]).is_err());
        queue.begin().unwrap();
        queue.end().unwrap();
        let large = vec![0; MAX_BYTES];
        for _ in 0..15 {
            queue.submit(DOUBLE, &large).unwrap();
        }
        let huge = BatchJob {
            input: &large,
            max_output_bytes: 0,
        };
        assert_eq!(
            queue.run_batch(DOUBLE, &[huge; 2]).unwrap_err().code,
            ErrorCode::BufferOverflow as i32
        );
        assert_eq!(
            queue.run_batch(DOUBLE, &[huge]).unwrap(),
            [Err(JobError(4))]
        );
        assert!(queue.submit(DOUBLE, &[0]).is_err());
        queue.shutdown();
        let mut queue = native::State::default();
        queue
            .run_batch(
                DOUBLE,
                &[BatchJob {
                    input: &[],
                    max_output_bytes: MAX_BYTES,
                }; 16],
            )
            .unwrap();
        assert!(queue.run_batch(DOUBLE, &[job]).is_err());
        queue.begin().unwrap();
        queue.end().unwrap();
        assert_eq!(queue.run_batch(DOUBLE, &[job]).unwrap(), [Ok(vec![6])]);
    }

    #[test]
    fn synchronous_failures_keep_slots_and_live_module_callbacks_are_not_reentered() {
        let mut queue = native::State::default();
        let jobs = [
            BatchJob {
                input: &[0],
                max_output_bytes: 1,
            },
            BatchJob {
                input: &[1],
                max_output_bytes: 1,
            },
            BatchJob {
                input: &[2],
                max_output_bytes: 1,
            },
        ];
        let mixed = Worker::new("mixed", |bytes| match bytes[0] {
            0 => panic!("test worker trap"),
            1 => vec![1, 2],
            _ => vec![2],
        });
        assert_eq!(
            queue.run_batch(mixed, &jobs).unwrap(),
            [Err(JobError(2)), Err(JobError(4)), Ok(vec![2])]
        );
        struct App;
        impl Module<Ui> for App {
            fn initialize(spring: &mut Spring<'_, Ui>) -> Result<Self> {
                spring.jobs().run_next_tick(DOUBLE, &[5])?;
                Ok(Self)
            }
            fn job_result(&self, _: &mut Spring<'_, Ui>, _: JobId, _: JobResult<'_>) -> Result<()> {
                panic!("synchronous run must not deliver deferred callbacks");
            }
        }
        let interface = crate::__sys::NativeInterface::default();
        let slot = ModuleSlot::<Ui, App>::new();
        unsafe { slot.initialize_native(&interface) }.unwrap();
        slot.invoke_tick_for_environment::<127, 2, _>(|app, spring| {
            let one = [BatchJob {
                input: &[4],
                max_output_bytes: 1,
            }];
            assert_eq!(spring.jobs().run_this_tick(DOUBLE, &one)?, [Ok(vec![8])]);
            // The same active application and scope remain usable after join.
            slot.invoke_tick_for_environment::<127, 2, _>(|nested, spring| {
                assert!(core::ptr::eq(app, nested));
                assert_eq!(spring.jobs().run_this_tick(DOUBLE, &one)?, [Ok(vec![8])]);
                Ok(())
            })?;
            // Borrow conflicts produce an API error instead of panicking.
            let borrowed = spring.engine.owner.jobs.borrow_mut();
            assert_eq!(
                spring.jobs().run_this_tick(DOUBLE, &one).unwrap_err().code,
                ErrorCode::InvalidState as i32
            );
            drop(borrowed);
            Ok(())
        })
        .unwrap();
        slot.shutdown().unwrap();
    }

    #[test]
    fn results_are_ordered_and_new_batches_do_not_overlap_delivery() {
        let mut queue = native::State::default();
        let ids: Vec<_> = (0..64)
            .map(|i| queue.submit(DOUBLE, &[i]).unwrap())
            .collect();
        assert!(queue.begin().unwrap().is_empty());
        assert!(
            queue.begin().is_err(),
            "nested Update must not advance twice"
        );
        queue.end().unwrap();
        let results = queue.begin().unwrap();
        assert_eq!(results.len(), ids.len());
        for (i, (id, result)) in results.into_iter().enumerate() {
            assert_eq!(id, ids[i]);
            assert_eq!(result.unwrap(), [i as u8 * 2]);
        }
        queue.submit(DOUBLE, &[7]).unwrap();
        queue.end().unwrap();
        assert_eq!(queue.begin().unwrap()[0].1, Ok(vec![14]));
        queue.end().unwrap();
    }

    #[test]
    fn job_count_and_input_limits_reset_only_after_sealing() {
        let mut queue = native::State::default();
        for _ in 0..MAX_JOBS {
            queue.submit(DOUBLE, &[]).unwrap();
        }
        assert_eq!(
            queue.submit(DOUBLE, &[]).unwrap_err().code,
            ErrorCode::BufferOverflow as i32
        );
        queue.begin().unwrap();
        queue.end().unwrap();
        let input = vec![0; MAX_BYTES];
        for _ in 0..16 {
            queue.submit(DOUBLE, &input).unwrap();
        }
        assert_eq!(
            queue.submit(DOUBLE, &[0]).unwrap_err().code,
            ErrorCode::BufferOverflow as i32
        );
        queue.shutdown();
    }

    #[test]
    fn worker_failure_does_not_fault_other_jobs_or_the_pool() {
        let mut queue = native::State::default();
        queue
            .submit(Worker::new("panic", |_| panic!("job test panic")), &[])
            .unwrap();
        queue
            .submit(Worker::new("oversized", |_| vec![0; MAX_BYTES + 1]), &[])
            .unwrap();
        queue.submit(DOUBLE, &[5]).unwrap();
        queue.begin().unwrap();
        queue.end().unwrap();
        let results = queue.begin().unwrap();
        assert_eq!(
            results.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>(),
            [Err(JobError(2)), Err(JobError(4)), Ok(vec![10])]
        );
        queue.end().unwrap();
    }

    #[test]
    fn shutdown_joins_running_workers_before_returning() {
        use std::sync::atomic::{AtomicBool, Ordering};
        static FINISHED: AtomicBool = AtomicBool::new(false);
        let mut queue = native::State::default();
        queue
            .submit(
                Worker::new("finish", |_| {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    FINISHED.store(true, Ordering::SeqCst);
                    vec![]
                }),
                &[],
            )
            .unwrap();
        queue.begin().unwrap();
        queue.end().unwrap();
        queue.shutdown();
        assert!(FINISHED.load(Ordering::SeqCst));
    }

    #[test]
    fn callbacks_have_a_fresh_scope_and_can_submit_the_next_batch() {
        struct App {
            values: RefCell<Vec<Vec<u8>>>,
        }
        impl Module<Ui> for App {
            fn initialize(spring: &mut Spring<'_, Ui>) -> Result<Self> {
                spring.jobs().run_next_tick(DOUBLE, &[3])?;
                Ok(Self {
                    values: RefCell::new(Vec::new()),
                })
            }
            fn job_result(
                &self,
                spring: &mut Spring<'_, Ui>,
                _: JobId,
                result: core::result::Result<&[u8], JobError>,
            ) -> Result<()> {
                self.values.borrow_mut().push(result.unwrap().to_vec());
                if self.values.borrow().len() == 1 {
                    spring
                        .module_ref()
                        .with_spring(|spring| spring.jobs().run_next_tick(DOUBLE, &[4]))?;
                }
                Ok(())
            }
        }
        let interface = crate::__sys::NativeInterface::default();
        let slot = ModuleSlot::<Ui, App>::new();
        unsafe { slot.initialize_native(&interface) }.unwrap();
        // Ui ignores GameFrame. Its first Update seals initialization work.
        slot.invoke_tick_for_environment::<127, 1, _>(|app, _| {
            assert!(app.values.borrow().is_empty());
            Ok(())
        })
        .unwrap();
        slot.invoke_tick_for_environment::<127, 2, _>(|app, _| {
            assert!(app.values.borrow().is_empty());
            // Same-thread Update reentry runs immediately, without advancing
            // the job batch or sealing it before the outer Update returns.
            slot.invoke_tick_for_environment::<127, 2, _>(|nested, _| {
                assert!(core::ptr::eq(app, nested));
                assert!(nested.values.borrow().is_empty());
                Ok(())
            })?;
            Ok(())
        })
        .unwrap();
        slot.invoke_tick_for_environment::<127, 2, _>(|app, _| {
            assert_eq!(*app.values.borrow(), [vec![6]]);
            Ok(())
        })
        .unwrap();
        slot.invoke_tick_for_environment::<127, 2, _>(|app, _| {
            assert_eq!(*app.values.borrow(), [vec![6], vec![8]]);
            Ok(())
        })
        .unwrap();
        slot.shutdown().unwrap();
        // Public submission validates oversize input before touching a worker.
        let context = unsafe { ModuleContext::<Ui>::from_native(&interface) }.unwrap();
        assert_eq!(
            context
                .with_spring(|spring| spring.jobs().run_next_tick(DOUBLE, &vec![0; MAX_BYTES + 1]))
                .unwrap()
                .unwrap_err()
                .code,
            ErrorCode::InvalidArgument as i32
        );
    }
}

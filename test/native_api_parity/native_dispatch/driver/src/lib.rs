/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
//! Runs real DLL entry points against the live engine's interface. No mock API.
use libloading::Library;
use spring_native::sys as ffi;
use spring_native::{Error, NativeInterfaceRef, NativeModule};
use std::{env, ffi::CString, hint::black_box, io::Write, path::PathBuf, time::Instant};
type Entry<Q, R> =
    unsafe extern "C" fn(*const ffi::NativeInterface, *mut core::ffi::c_void, *const Q, *mut R);
struct Worker {
    label: String,
    library: Library,
    data: *mut core::ffi::c_void,
}
impl Worker {
    unsafe fn open(label: &str, path: &str, interface: *const ffi::NativeInterface) -> Self {
        let library = unsafe { Library::new(path) }.unwrap();
        let init = unsafe {
            library.get::<unsafe extern "C" fn(
                *const ffi::NativeInterface,
                *const ffi::InitializeNativeModuleQuery,
                *mut ffi::InitializeNativeModuleResult,
            )>(b"InitializeNativeModule")
        }
        .unwrap();
        let [major, minor, patch] = [
            spring_native::NATIVE_API_VERSION_MAJOR,
            spring_native::NATIVE_API_VERSION_MINOR,
            spring_native::NATIVE_API_VERSION_PATCH,
        ];
        let query = ffi::InitializeNativeModuleQuery {
            hostVersionMajor: major,
            hostVersionMinor: minor,
            hostVersionPatch: patch,
        };
        let mut result = ffi::InitializeNativeModuleResult::default();
        unsafe { init(interface, &query, &mut result) };
        assert!(result.error.is_null());
        assert!(!result.moduleData.is_null());
        Self {
            label: label.into(),
            library,
            data: result.moduleData,
        }
    }
    fn configure(&self, unit: i32, queries: u32) {
        unsafe {
            self.library
                .get::<unsafe extern "C" fn(i32, u32)>(b"ConfigureBenchmark")
                .unwrap()(unit, queries)
        }
    }
    fn configure_callout(&self, kind: u32, queries: u32) {
        unsafe {
            self.library
                .get::<unsafe extern "C" fn(u32, u32)>(b"ConfigureCallout")
                .unwrap()(kind, queries)
        }
    }
    fn checksum(&self) -> u64 {
        unsafe {
            self.library
                .get::<unsafe extern "C" fn() -> u64>(b"BenchmarkChecksum")
                .unwrap()()
        }
    }
    fn shutdown(&self, interface: *const ffi::NativeInterface) {
        let call = unsafe {
            self.library
                .get::<Entry<ffi::ShutdownQuery, ffi::ShutdownResult>>(b"Shutdown")
        }
        .unwrap();
        let mut r = ffi::ShutdownResult::default();
        unsafe { call(interface, self.data, &ffi::ShutdownQuery::default(), &mut r) };
        assert!(r.error.is_null());
    }
}
fn measure<Q, R: Default>(
    workers: &[Worker],
    interface: *const ffi::NativeInterface,
    event: &[u8],
    query: &Q,
    check: impl Fn(&R),
    out: &mut std::fs::File,
    case: (&str, usize),
) {
    let (name, operations) = case;
    let cold = env::var("SDK_DISPATCH_CACHE").as_deref() == Ok("cold");
    let iterations = if cold || operations > 1 {
        1
    } else {
        env::var("SDK_DISPATCH_ITERATIONS")
            .unwrap()
            .parse::<usize>()
            .unwrap()
    };
    let repeats = env::var("SDK_DISPATCH_REPEATS")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let calls = workers
        .iter()
        .map(|worker| unsafe { worker.library.get::<Entry<Q, R>>(event) }.unwrap())
        .collect::<Vec<_>>();
    let mut samples = vec![Vec::new(); workers.len()];
    let mut eviction = if cold {
        vec![0u8; 64 * 1024 * 1024]
    } else {
        Vec::new()
    };
    // Touch each cache line with a write: reading fresh calloc zero pages
    // alone need not fill caches with distinct physical pages on Linux.
    for index in (0..eviction.len()).step_by(64) {
        eviction[index] = (index >> 6) as u8;
    }
    for (index, worker) in workers.iter().enumerate() {
        for _ in 0..if operations > 1 { 1 } else { 1000 } {
            let mut r = R::default();
            unsafe { calls[index](interface, worker.data, query, &mut r) };
            check(&r);
        }
    }
    for repeat in 0..repeats {
        // Counterbalance library order within every sample, on one thread.
        for offset in 0..workers.len() {
            let index = (repeat + offset) % workers.len();
            let worker = &workers[index];
            let call = &calls[index];
            if cold {
                for index in (0..eviction.len()).step_by(64) {
                    black_box(unsafe { eviction.as_ptr().add(index).read_volatile() });
                }
            }
            let start = Instant::now();
            for _ in 0..iterations {
                let mut r = R::default();
                unsafe { call(black_box(interface), worker.data, black_box(query), &mut r) };
                black_box(&r);
                check(&r);
            }
            samples[index]
                .push(start.elapsed().as_nanos() as f64 / iterations as f64 / operations as f64);
        }
    }
    for (worker, samples) in workers.iter().zip(samples) {
        let mut sorted = samples.clone();
        sorted.sort_by(f64::total_cmp);
        writeln!(out,"{}",serde_json::json!({"backend":worker.label,"test":name,"iterations":iterations,"operationsPerEntry":operations,"samplesNs":samples,"medianNs":sorted[sorted.len()/2],"minNs":sorted[0],"maxNs":sorted[sorted.len()-1],"cache":if cold {"cold"}else{"warm"},"cpu":env::var("SDK_DISPATCH_CPU").ok(),"status":"pass"})).unwrap();
    }
}

struct Driver(NativeInterfaceRef, bool);
impl NativeModule for Driver {
    fn new(interface: NativeInterfaceRef) -> Self {
        Self(interface, false)
    }
    fn handle_lua_call(&mut self, message: &str) -> Result<(), Error> {
        if self.1 || !message.contains("benchmark") {
            return Ok(());
        }
        self.1 = true;
        #[cfg(target_os = "linux")]
        if let Ok(cpu) = env::var("SDK_DISPATCH_CPU") {
            let cpu = cpu.parse::<usize>().unwrap();
            let mut set = unsafe { core::mem::zeroed::<libc::cpu_set_t>() };
            unsafe {
                libc::CPU_SET(cpu, &mut set);
                assert_eq!(
                    libc::sched_setaffinity(0, core::mem::size_of_val(&set), &set),
                    0
                );
            }
        }
        let unit = self
            .0
            .units_query()
            .get_team_units(0)?
            .first()
            .copied()
            .expect("fixture unit");
        let def = self.0.units_info().get_unit_def_id(unit)?;
        let path = PathBuf::from(env::var_os("SPRING_NATIVE_PARITY_OUTPUT_DIR").unwrap())
            .join("dispatch_compare.jsonl");
        let mut out = std::fs::File::create(path).unwrap();
        let specification = env::var("SDK_DISPATCH_LIBRARIES").expect("label=path;...");
        let workers = specification
            .split(';')
            .map(|entry| {
                let (label, path) = entry.split_once('=').unwrap();
                unsafe { Worker::open(label, path, self.0.as_ptr()) }
            })
            .collect::<Vec<_>>();
        // Both libraries remain loaded in this
        // one engine process and use the identical live interface and fixture.
        macro_rules! measure {
            ($name:expr,$symbol:literal,$query:expr,$result:ty,$check:expr) => {{
                measure::<_, $result>(
                    &workers,
                    self.0.as_ptr(),
                    $symbol,
                    &$query,
                    $check,
                    &mut out,
                    ($name, 1),
                );
            }};
        }
        for queries in [0, 1, 16] {
            for worker in &workers {
                worker.configure(unit, queries);
            }
            measure!(
                &format!("gameframe_{queries}_queries"),
                b"GameFrame",
                ffi::GameFrameQuery { gameFrame: 321 },
                ffi::GameFrameResult,
                |r: &ffi::GameFrameResult| assert!(r.error.is_null())
            );
            for worker in &workers {
                assert_eq!(worker.checksum(), 321 + u64::from(queries) * def as u64);
            }
        }
        measure!(
            "update",
            b"Update",
            ffi::UpdateQuery {
                deltaSeconds: 0.016
            },
            ffi::UpdateResult,
            |r: &ffi::UpdateResult| assert!(r.error.is_null())
        );
        measure!(
            "unitcreated",
            b"UnitCreated",
            ffi::UnitCreatedQuery {
                unitID: unit,
                unitDefID: def,
                unitTeam: 0,
                builderID: -1
            },
            ffi::UnitCreatedResult,
            |r: &ffi::UnitCreatedResult| assert!(r.error.is_null())
        );
        measure!(
            "creation",
            b"AllowUnitCreation",
            ffi::AllowUnitCreationQuery {
                unitDefID: def,
                builderID: unit,
                builderTeam: 0,
                hasBuildInfo: true,
                buildPos: ffi::Float3 {
                    x: 100.0,
                    y: 96.0,
                    z: 100.0
                },
                buildFacing: 0
            },
            ffi::AllowUnitCreationResult,
            |r: &ffi::AllowUnitCreationResult| {
                assert!(r.error.is_null());
                assert!(r.allow);
                assert!(!r.dropOrder);
            }
        );
        measure!(
            "predamaged",
            b"UnitPreDamaged",
            ffi::UnitDamagedQuery {
                unitID: unit,
                unitDefID: def,
                unitTeam: 0,
                damage: 42.5,
                paralyzer: false,
                weaponDefID: 1,
                projectileID: 2,
                attackerID: unit,
                attackerDefID: def,
                attackerTeam: 0
            },
            ffi::DamageCallinResult,
            |r: &ffi::DamageCallinResult| {
                assert!(r.error.is_null());
                assert_eq!((r.newDamage, r.impulseMult), (42.5, 1.0));
            }
        );
        for count in [0, 4, 64, 1024] {
            let params = (0..count).map(|i| i as f32).collect::<Vec<_>>();
            measure!(
                &format!("command_{count}"),
                b"CommandNotify",
                ffi::CommandNotifyQuery {
                    command: ffi::NativeCallinCommand {
                        id: 10,
                        timeOut: -1,
                        pageIndex: 0,
                        numParams: count,
                        tag: 5,
                        options: 0,
                        params: params.as_ptr()
                    }
                },
                ffi::BoolCallinResult,
                |r: &ffi::BoolCallinResult| {
                    assert!(r.error.is_null());
                    assert!(!r.value);
                }
            );
        }
        for size in [0, 16, 1024] {
            let message = CString::new("x".repeat(size)).unwrap();
            let section = c"benchmark";
            measure!(
                &format!("console_{size}"),
                b"AddConsoleLine",
                ffi::AddConsoleLineQuery {
                    message: message.as_ptr(),
                    section: section.as_ptr(),
                    level: 3
                },
                ffi::BoolCallinResult,
                |r: &ffi::BoolCallinResult| {
                    assert!(r.error.is_null());
                    assert!(!r.value);
                }
            );
        }
        // The fixture has a queued move with three parameters. No simulation
        // advances inside this callin, so every worker reads the same list.
        let commands = self.0.units_commands().get_unit_commands(unit, 5)?;
        assert!(!commands.is_empty());
        assert!(commands[0].paramCount >= 3);
        // Batch queries inside one admitted callback; entry overhead amortizes
        // over exactly the same number of live host calls for each SDK.
        for (kind, name) in [
            (0, "scalar"),
            (1, "vec3"),
            (2, "string"),
            (3, "smalllist"),
            (4, "biglist"),
            (5, "spatial"),
            (6, "mutate"),
        ] {
            let count = if kind == 4 { 1000 } else { 10000 };
            for worker in &workers {
                worker.configure_callout(kind, count);
            }
            measure::<_, ffi::GameFrameResult>(
                &workers,
                self.0.as_ptr(),
                b"GameFrame",
                &ffi::GameFrameQuery { gameFrame: 321 },
                |r| assert!(r.error.is_null()),
                &mut out,
                (&format!("callout_{name}"), count as usize),
            );
            let expected = workers[0].checksum();
            assert!(workers.iter().all(|worker| worker.checksum() == expected));
            if kind != 3 {
                assert!(expected > 321, "callouts must return nonempty fixture data");
            }
            if kind == 6 {
                let (value, _, exists) =
                    self.0.rules_params().get_unit_rules_param(unit, "bench")?;
                assert!(exists);
                assert!(matches!(value, spring_native::RulesParamValue::Float(1.0)));
            }
        }
        for worker in workers {
            worker.shutdown(self.0.as_ptr());
        }
        self.0
            .messages()
            .send_lua_rules_msg("NATIVE_BENCH|{\"test\":\"complete\"}")?;
        Ok(())
    }
}
spring_native::export_module!(Driver);

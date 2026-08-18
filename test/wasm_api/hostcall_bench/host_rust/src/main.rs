// Standalone floor measurement for the Wasmtime Rust API host boundary.  It is
// the typed counterpart of host_c/bench_hostcall.cpp: same guests, same engine
// configuration, same directions, but reached through func_wrap and TypedFunc
// instead of the dynamically typed C API.

use std::time::Instant;

use anyhow::Result;
use wasmtime::component::{Component, ComponentType, Lift, Linker as ComponentLinker, Lower};
use wasmtime::{Config, Engine, Linker, Module, Store};

#[derive(Clone, Copy, ComponentType, Lift, Lower)]
#[component(record)]
struct SpringError {
    code: i32,
}

#[derive(Clone, Copy, ComponentType, Lift, Lower)]
#[component(record)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, ComponentType, Lift, Lower)]
#[component(record)]
struct PositionOptions {
    #[component(name = "mid-pos")]
    mid_pos: bool,
    #[component(name = "aim-pos")]
    aim_pos: bool,
}

#[derive(Clone, Copy, ComponentType, Lift, Lower)]
#[component(record)]
struct Sample {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

fn engine() -> Result<Engine> {
    // Mirrors rts/WasmInterface/WasmRuntime.cpp so both harnesses measure the
    // same Cranelift and trap configuration.
    let mut config = Config::new();
    config.consume_fuel(false);
    config.wasm_component_model(true);
    config.signals_based_traps(true);
    config.wasm_threads(false);
    config.wasm_relaxed_simd(false);
    config.relaxed_simd_deterministic(true);
    config.cranelift_nan_canonicalization(true);
    config.wasm_multi_value(true);
    config.wasm_bulk_memory(true);
    Ok(Engine::new(&config)?)
}

fn report(name: &str, mut samples: Vec<f64>, calls_per_sample: i32) {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let per = |value: f64| value / f64::from(calls_per_sample);
    println!(
        "{:<34} {:>9.2} ns  [{:.2} .. {:.2}]",
        name,
        per(samples[samples.len() / 2]),
        per(samples[0]),
        per(samples[samples.len() - 1])
    );
}

fn main() -> Result<()> {
    let mut arguments = std::env::args().skip(1);
    let root = arguments.next().unwrap_or_else(|| ".".to_string());
    let iterations: i32 = arguments
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(200_000);
    let repeats: usize = arguments
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(21);

    let engine = engine()?;
    println!(
        "wasmtime Rust API, {} guest iterations per sample, {} samples\n",
        iterations, repeats
    );

    // ------------------------------------------------------------- core wasm
    let module = Module::from_file(&engine, format!("{root}/core.wasm"))?;
    let mut core_linker = Linker::new(&engine);
    core_linker.func_wrap("spring", "add_i32", |_caller: wasmtime::Caller<'_, ()>, x: i32| x + 1)?;
    let mut core_store = Store::new(&engine, ());
    let core_instance = core_linker.instantiate(&mut core_store, &module)?;
    let core_callout =
        core_instance.get_typed_func::<(i32,), (i32,)>(&mut core_store, "run_callout")?;
    let core_spin = core_instance.get_typed_func::<(i32,), (i32,)>(&mut core_store, "run_spin")?;
    let core_noop = core_instance.get_typed_func::<(i32,), (i32,)>(&mut core_store, "noop")?;

    // ------------------------------------------------------------- component
    let component = Component::from_file(&engine, format!("{root}/bench_component.wasm"))?;
    let mut component_linker = ComponentLinker::<()>::new(&engine);
    {
        let mut host = component_linker.instance("recoil:hostcall-bench/host@1.0.0")?;
        host.func_wrap("add-i32", |_store, (x,): (i32,)| Ok((x + 1,)))?;
        host.func_wrap("add-i32-result", |_store, (x,): (i32,)| {
            Ok((Ok::<i32, SpringError>(x + 1),))
        })?;
        host.func_wrap("get-vec3", |_store, (x,): (i32,)| {
            let base = (x + 1) as f32;
            Ok((Ok::<Vec3, SpringError>(Vec3 { x: base, y: base, z: base }),))
        })?;
        host.func_wrap(
            "get-vec3-opts",
            |_store, (x, options): (i32, PositionOptions)| {
                let base = (x + 1) as f32;
                let bump = f32::from(u8::from(options.mid_pos || options.aim_pos));
                Ok((Ok::<Vec3, SpringError>(Vec3 {
                    x: base + bump,
                    y: base,
                    z: base,
                }),))
            },
        )?;
    }
    let mut component_store = Store::new(&engine, ());
    let component_instance =
        component_linker.instantiate(&mut component_store, &component)?;
    let component_callout = component_instance
        .get_typed_func::<(i32,), (i32,)>(&mut component_store, "run-callout")?;
    let component_callout_result = component_instance
        .get_typed_func::<(i32,), (i32,)>(&mut component_store, "run-callout-result")?;
    let component_callout_vec3 = component_instance
        .get_typed_func::<(i32,), (i32,)>(&mut component_store, "run-callout-vec3")?;
    let component_callout_vec3_opts = component_instance
        .get_typed_func::<(i32,), (i32,)>(&mut component_store, "run-callout-vec3-opts")?;
    let component_spin = component_instance
        .get_typed_func::<(i32,), (i32,)>(&mut component_store, "run-spin")?;
    let component_noop =
        component_instance.get_typed_func::<(i32,), (i32,)>(&mut component_store, "noop")?;
    let component_noop_record = component_instance
        .get_typed_func::<(Sample,), (i32,)>(&mut component_store, "noop-record")?;

    let drive_core = |func: &wasmtime::TypedFunc<(i32,), (i32,)>,
                          store: &mut Store<()>,
                          iterations: i32|
     -> Result<f64> {
        let start = Instant::now();
        let (returned,) = func.call(&mut *store, (iterations,))?;
        let elapsed = start.elapsed().as_nanos() as f64;
        anyhow::ensure!(returned == iterations, "core guest returned {returned}");
        Ok(elapsed)
    };
    let drive_component = |func: &wasmtime::component::TypedFunc<(i32,), (i32,)>,
                               store: &mut Store<()>,
                               iterations: i32|
     -> Result<f64> {
        let start = Instant::now();
        let (returned,) = func.call(&mut *store, (iterations,))?;
        let elapsed = start.elapsed().as_nanos() as f64;
        anyhow::ensure!(returned == iterations, "component guest returned {returned}");
        Ok(elapsed)
    };

    for _ in 0..3 {
        drive_core(&core_callout, &mut core_store, iterations)?;
        drive_core(&core_spin, &mut core_store, iterations)?;
        drive_component(&component_callout, &mut component_store, iterations)?;
        drive_component(&component_callout_result, &mut component_store, iterations)?;
        drive_component(&component_callout_vec3, &mut component_store, iterations)?;
        drive_component(&component_spin, &mut component_store, iterations)?;
    }

    let mut spin_core = Vec::new();
    let mut spin_component = Vec::new();
    let mut callout_core = Vec::new();
    let mut callout_component = Vec::new();
    let mut callout_component_result = Vec::new();
    let mut callout_component_vec3 = Vec::new();
    let mut callout_component_vec3_opts = Vec::new();
    for _ in 0..repeats {
        spin_core.push(drive_core(&core_spin, &mut core_store, iterations)?);
        spin_component.push(drive_component(&component_spin, &mut component_store, iterations)?);
        callout_core.push(drive_core(&core_callout, &mut core_store, iterations)?);
        callout_component
            .push(drive_component(&component_callout, &mut component_store, iterations)?);
        callout_component_result.push(drive_component(
            &component_callout_result,
            &mut component_store,
            iterations,
        )?);
        callout_component_vec3.push(drive_component(
            &component_callout_vec3,
            &mut component_store,
            iterations,
        )?);
        callout_component_vec3_opts.push(drive_component(
            &component_callout_vec3_opts,
            &mut component_store,
            iterations,
        )?);
    }

    println!("guest -> host (callout), per call");
    report("core, Rust typed func_wrap", callout_core, iterations);
    report("component, Rust typed func_wrap", callout_component, iterations);
    report("component, Rust typed result<>", callout_component_result, iterations);
    report("component, Rust typed vec3", callout_component_vec3, iterations);
    report(
        "component, Rust typed vec3+record arg",
        callout_component_vec3_opts,
        iterations,
    );
    report("guest loop only (core)", spin_core, iterations);
    report("guest loop only (component)", spin_component, iterations);

    let callin_iterations = iterations / 20;
    let mut callin_core = Vec::new();
    let mut callin_component = Vec::new();
    let mut callin_component_record = Vec::new();
    let sample = Sample { a: 7, b: 8, c: 9, d: 10 };
    for _ in 0..repeats {
        let start = Instant::now();
        for _ in 0..callin_iterations {
            core_noop.call(&mut core_store, (7,))?;
        }
        callin_core.push(start.elapsed().as_nanos() as f64);

        let start = Instant::now();
        for _ in 0..callin_iterations {
            component_noop.call(&mut component_store, (7,))?;
        }
        callin_component.push(start.elapsed().as_nanos() as f64);

        let start = Instant::now();
        for _ in 0..callin_iterations {
            component_noop_record.call(&mut component_store, (sample,))?;
        }
        callin_component_record.push(start.elapsed().as_nanos() as f64);
    }

    println!("\nhost -> guest (callin), per call");
    report("core, Rust TypedFunc", callin_core, callin_iterations);
    report("component, Rust TypedFunc", callin_component, callin_iterations);
    report("component, Rust TypedFunc record", callin_component_record, callin_iterations);

    Ok(())
}

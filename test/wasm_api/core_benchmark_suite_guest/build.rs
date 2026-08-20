use std::env;

fn main() {
    for name in [
        "SPRING_BENCHMARK_CASE",
        "SPRING_BENCHMARK_SCALE",
        "SPRING_BENCHMARK_ITERATIONS",
        "SPRING_BENCHMARK_REPEATS",
        "SPRING_BENCHMARK_CALLIN_VARIANT",
        "SPRING_BENCHMARK_CONTEXT",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }

    // Synced Core validation requires min == max. Put the linker contract in
    // the build script rather than relying on a nested .cargo/config.toml,
    // because the benchmark runner invokes Cargo from the repository root.
    // 16 MiB is intentionally roomy for Vec-heavy list/workload benchmarks
    // while remaining far below the engine's 64 MiB default Core memory cap.
    println!("cargo:rustc-link-arg=--initial-memory=16777216");
    println!("cargo:rustc-link-arg=--no-growable-memory");

    println!("cargo:rustc-check-cfg=cfg(benchmark_callin_unimplemented)");
    println!("cargo:rustc-check-cfg=cfg(benchmark_callin_consoleline)");
    println!("cargo:rustc-check-cfg=cfg(benchmark_callin_commandnotify)");
    println!("cargo:rustc-check-cfg=cfg(benchmark_context_unsynced)");
    println!("cargo:rustc-check-cfg=cfg(benchmark_context_ui)");

    match env::var("SPRING_BENCHMARK_CALLIN_VARIANT").as_deref() {
        Ok("unimplemented") => println!("cargo:rustc-cfg=benchmark_callin_unimplemented"),
        Ok("consoleline") => println!("cargo:rustc-cfg=benchmark_callin_consoleline"),
        Ok("commandnotify") => println!("cargo:rustc-cfg=benchmark_callin_commandnotify"),
        _ => {}
    }
    match env::var("SPRING_BENCHMARK_CONTEXT").as_deref() {
        Ok("unsynced_gadget") => println!("cargo:rustc-cfg=benchmark_context_unsynced"),
        Ok("ui") => println!("cargo:rustc-cfg=benchmark_context_ui"),
        _ => {}
    }
}

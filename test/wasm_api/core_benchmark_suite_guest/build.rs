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

    println!("cargo:rustc-check-cfg=cfg(benchmark_callin_unimplemented)");
    println!("cargo:rustc-check-cfg=cfg(benchmark_context_unsynced)");
    println!("cargo:rustc-check-cfg=cfg(benchmark_context_ui)");

    if env::var("SPRING_BENCHMARK_CALLIN_VARIANT").as_deref() == Ok("unimplemented") {
        println!("cargo:rustc-cfg=benchmark_callin_unimplemented");
    }
    match env::var("SPRING_BENCHMARK_CONTEXT").as_deref() {
        Ok("unsynced_gadget") => println!("cargo:rustc-cfg=benchmark_context_unsynced"),
        Ok("ui") => println!("cargo:rustc-cfg=benchmark_context_ui"),
        _ => {}
    }
}

use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let generated = manifest.join("../../../rts/wasm/generated/sdk/core_generated.rs");
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("core_generated.rs");

    println!("cargo:rerun-if-changed={}", generated.display());
    if generated.is_file() {
        fs::copy(&generated, &output).expect("copy generated Core Rust SDK");
    } else {
        fs::write(
            &output,
            "// Core SDK has not been generated yet; run spring-api-codegen.\n",
        )
        .expect("write empty generated Core Rust SDK fallback");
    }
}

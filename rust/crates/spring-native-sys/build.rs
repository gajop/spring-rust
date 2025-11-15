use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let mut project_root = manifest_dir.clone();
    // ../.. from crates/spring-native-sys -> spring-bar
    for _ in 0..3 {
        if !project_root.pop() {
            panic!("failed to locate project root");
        }
    }

    let header = project_root.join("rts/NativeInterface/NativeInterface.h");
    let include_dir = project_root.join("rts/NativeInterface/api");

    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", include_dir.display());

    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", project_root.join("rts").display()))
        .allowlist_type(".*Api")
        .allowlist_type("NativeInterface")
        .allowlist_type(".*Query")
        .allowlist_type(".*Result")
        .allowlist_type(".*Params")
        .allowlist_type(".*Error")
        .allowlist_type(".*Struct")
        .allowlist_function(".*")
        .allowlist_var(".*")
        .layout_tests(false)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

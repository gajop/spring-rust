use std::{env, fs, path::PathBuf};

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
    let constants_header = project_root.join("rts/NativeInterface/api/Constants.h");
    let include_dir = project_root.join("rts/NativeInterface/api");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindgen_header = out_path.join("spring_native_bindgen.hpp");
    fs::write(
        &bindgen_header,
        format!(
            "#define SPRING_NATIVE_BINDGEN 1\n#include \"{}\"\n#include \"{}\"\n",
            header.display(),
            constants_header.display()
        ),
    )
    .expect("write bindgen wrapper");

    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", constants_header.display());
    for entry in fs::read_dir(&include_dir).expect("read NativeInterface API directory") {
        let entry = entry.expect("read NativeInterface API directory entry");
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    let bindings = bindgen::Builder::default()
        .header(bindgen_header.to_string_lossy())
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", project_root.join("rts").display()))
        .allowlist_type(".*Api")
        .allowlist_type("NativeInterface")
        .allowlist_type("CommandID")
        .allowlist_type("CommandOption")
        .allowlist_type("FireState")
        .allowlist_type("MoveState")
        .allowlist_type("UnitCategory")
        .allowlist_type("DamageType")
        .allowlist_type("GameConstant")
        .allowlist_type("COBConstant")
        .allowlist_type("GLConstant")
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

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

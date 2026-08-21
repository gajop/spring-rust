use std::{env, fs, path::PathBuf};

fn append_generated(contents: &mut String, path: &PathBuf, description: &str) {
    println!("cargo:rerun-if-changed={}", path.display());
    if path.is_file() {
        contents.push_str(&fs::read_to_string(path).expect("read generated Core SDK fragment"));
    } else {
        contents.push_str(&format!(
            "// {description} has not been generated yet; run spring-api-codegen.\n"
        ));
    }
    contents.push('\n');
}

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let generated_dir = manifest.join("../../../rts/wasm/generated/sdk");
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("core_generated.rs");

    let mut contents = String::new();
    append_generated(
        &mut contents,
        &generated_dir.join("core_generated.rs"),
        "Core callout SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_borrowed.rs"),
        "borrowed Core callout SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_variable.rs"),
        "adapted variable-input Core callout SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_dynamic_input.rs"),
        "nested dynamic-input Core callout SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_dynamic_output.rs"),
        "dynamic-output Core callout SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_owned.rs"),
        "owned semantic Core callout SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_environments.rs"),
        "environment-filtered Core callout SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_callins.rs"),
        "Core numeric callin SDK",
    );
    append_generated(
        &mut contents,
        &generated_dir.join("core_callins_scratch.rs"),
        "Core shared-scratch callin SDK",
    );

    fs::write(&output, contents).expect("write combined generated Core Rust SDK");
}

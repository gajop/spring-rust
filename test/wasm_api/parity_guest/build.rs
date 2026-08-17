use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wit/parity.wit");
    println!("cargo:rustc-check-cfg=cfg(parity_has_synced_message)");
    println!("cargo:rustc-check-cfg=cfg(parity_is_synced)");

    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let wit_path = manifest_dir.join("wit/parity.wit");
    let wit = fs::read_to_string(wit_path).expect("read generated parity WIT");
    if wit.contains("recv-from-synced:") {
        println!("cargo:rustc-cfg=parity_has_synced_message");
    }
    if wit.contains("world rules-synced") || wit.contains("world gaia-synced") {
        println!("cargo:rustc-cfg=parity_is_synced");
    }
}

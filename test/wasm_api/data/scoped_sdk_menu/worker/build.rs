/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32") {
        println!("cargo:rustc-link-arg=--initial-memory=16777216");
        println!("cargo:rustc-link-arg=--no-growable-memory");
    }
}

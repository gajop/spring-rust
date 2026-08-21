use std::env;

fn main() {
    // Core validation requires fixed linear memory in synced contexts. Keep
    // this contract in the guest crate so every harness build gets it.
    println!("cargo:rustc-link-arg=--initial-memory=67108864");
    println!("cargo:rustc-link-arg=--no-growable-memory");
    println!("cargo:rustc-check-cfg=cfg(parity_has_synced_message)");

    if env::var_os("CARGO_FEATURE_CORE_RULES_SYNCED").is_some() {
        println!("cargo:rustc-cfg=parity_has_synced_message");
    }
}

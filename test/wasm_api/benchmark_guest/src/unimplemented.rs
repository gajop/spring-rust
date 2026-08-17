#![allow(clippy::all)]

pub(crate) mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "benchmark-rules-synced",
    });
}

struct BenchmarkUnimplementedGuest;

impl bindings::Guest for BenchmarkUnimplementedGuest {
    fn callback_1(_user_data: u32) {}
}

bindings::export!(BenchmarkUnimplementedGuest with_types_in bindings);

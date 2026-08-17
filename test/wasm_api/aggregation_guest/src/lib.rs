#![allow(clippy::all)]

mod bindings {
	wit_bindgen::generate!({
		path: "wit",
		world: "allow-command",
	});
}

use bindings::exports::recoil::spring_api::callins_rules_synced::{
	BoolCallinResult, Guest, SpringError, UnitCommandQuery,
};

struct AggregationGuest;

impl Guest for AggregationGuest {
	fn allow_command(_query: UnitCommandQuery) -> Result<BoolCallinResult, SpringError> {
		Ok(BoolCallinResult {
			value: cfg!(feature = "allow-true"),
		})
	}
}

bindings::export!(AggregationGuest with_types_in bindings);

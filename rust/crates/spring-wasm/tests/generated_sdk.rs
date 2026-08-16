/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#![allow(dead_code, non_camel_case_types, non_snake_case, clippy::all)]

// The library includes the checked-in generated façade.  This test exercises
// it from a downstream-crate position, catching accidental private exports.
use spring_wasm::{units_query, ApiError, CalloutBackend, Value};

struct MockBackend;

impl CalloutBackend for MockBackend {
    fn call(&mut self, function: &str, arguments: &[Value]) -> Result<Value, ApiError> {
        assert!(!function.is_empty());
        assert!(!arguments.iter().any(|value| matches!(value, Value::Unit)));
        match function {
            "units_query.GetTeamUnitCount" => Ok(Value::U32(7)),
            _ => Ok(Value::Unit),
        }
    }
}

#[test]
fn generated_sdk_is_constructible_and_forwards_semantic_arguments() {
    let mut backend = MockBackend;
    let mut api = units_query::new(&mut backend);
    let result = api.get_team_unit_count(1);
    assert!(result.is_ok());
}

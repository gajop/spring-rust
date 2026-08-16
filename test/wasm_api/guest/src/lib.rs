/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Representative owned module used by native/Wasm semantic parity tests.

use spring_wasm::{ApiError, CalloutBackend, Environment, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuerySummary {
    pub label: String,
    pub values: Vec<i32>,
    pub position: Float3,
    pub enabled: Option<bool>,
}

pub struct RepresentativeModule<B> {
    backend: B,
    pub environment: Environment,
}

impl<B: CalloutBackend> RepresentativeModule<B> {
    pub fn new(backend: B, environment: Environment) -> Self {
        Self {
            backend,
            environment,
        }
    }

    pub fn query_summary(&mut self, unit_id: i32) -> Result<QuerySummary, ApiError> {
        let value = self
            .backend
            .call("units.query-summary", &[Value::I32(unit_id)])?;
        let Value::Record(mut record) = value else {
            return Err(ApiError::new(1, spring_wasm::ErrorCategory::Internal));
        };
        let label = match record.remove("label") {
            Some(Value::String(value)) => value,
            _ => return Err(ApiError::new(2, spring_wasm::ErrorCategory::Internal)),
        };
        let values = match record.remove("values") {
            Some(Value::List(values)) => values
                .into_iter()
                .map(|value| match value {
                    Value::I32(value) => Ok(value),
                    _ => Err(ApiError::new(3, spring_wasm::ErrorCategory::Internal)),
                })
                .collect::<Result<Vec<_>, _>>()?,
            _ => return Err(ApiError::new(4, spring_wasm::ErrorCategory::Internal)),
        };
        let position = match record.remove("position") {
            Some(Value::Record(mut position)) => Float3 {
                x: as_f32(position.remove("x"))?,
                y: as_f32(position.remove("y"))?,
                z: as_f32(position.remove("z"))?,
            },
            _ => return Err(ApiError::new(5, spring_wasm::ErrorCategory::Internal)),
        };
        let enabled = match record.remove("enabled") {
            Some(Value::Bool(value)) => Some(value),
            Some(Value::Unit) | None => None,
            _ => return Err(ApiError::new(6, spring_wasm::ErrorCategory::Internal)),
        };
        Ok(QuerySummary {
            label,
            values,
            position,
            enabled,
        })
    }

    pub fn mutate(&mut self, value: i32) -> Result<(), ApiError> {
        self.backend
            .call("synced-control.set-value", &[Value::I32(value)])
            .map(|_| ())
    }
}

fn as_f32(value: Option<Value>) -> Result<f32, ApiError> {
    match value {
        Some(Value::F32(value)) => Ok(value),
        _ => Err(ApiError::new(7, spring_wasm::ErrorCategory::Internal)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    struct MockBackend;

    impl CalloutBackend for MockBackend {
        fn call(&mut self, function: &str, _arguments: &[Value]) -> Result<Value, ApiError> {
            match function {
                "units.query-summary" => {
                    let position = Value::Record(BTreeMap::from([
                        ("x".to_string(), Value::F32(1.0)),
                        ("y".to_string(), Value::F32(2.0)),
                        ("z".to_string(), Value::F32(3.0)),
                    ]));
                    Ok(Value::Record(BTreeMap::from([
                        ("label".to_string(), Value::String("unit".to_string())),
                        (
                            "values".to_string(),
                            Value::List(vec![Value::I32(1), Value::I32(2)]),
                        ),
                        ("position".to_string(), position),
                        ("enabled".to_string(), Value::Bool(true)),
                    ])))
                }
                "synced-control.set-value" => Ok(Value::Unit),
                _ => Err(ApiError::new(8, spring_wasm::ErrorCategory::Unavailable)),
            }
        }
    }

    #[test]
    fn exercises_owned_semantic_values() {
        let mut module = RepresentativeModule::new(MockBackend, Environment::RulesSynced);
        let summary = module.query_summary(42).unwrap();
        assert_eq!(summary.label, "unit");
        assert_eq!(summary.values, vec![1, 2]);
        assert_eq!(summary.position.z, 3.0);
        module.mutate(7).unwrap();
    }
}

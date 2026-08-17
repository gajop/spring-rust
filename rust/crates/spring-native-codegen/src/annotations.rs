//! Explicit semantic lowering exceptions.

use anyhow::{anyhow, Context, Result};
use std::{collections::BTreeSet, path::Path};

use crate::model::{ApiModel, LoweringStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Manual,
    Exclude,
    SyncedOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub kind: Kind,
    pub module: String,
    pub function: String,
    pub reason: String,
}

pub fn parse(path: &Path) -> Result<Vec<Annotation>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read annotations {}", path.display()))?;
    let mut result = Vec::new();
    for (line_number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty()
            || line.starts_with('#')
            || line.starts_with("/*")
            || line.starts_with('*')
        {
            continue;
        }
        let (kind, body) = line.split_once('(').ok_or_else(|| {
            anyhow!(
                "{}:{}: expected MANUAL(...), EXCLUDE(...) or SYNCED_ONLY(...)",
                path.display(),
                line_number + 1
            )
        })?;
        let fields = body
            .strip_suffix(')')
            .ok_or_else(|| anyhow!("{}:{}: missing ')'", path.display(), line_number + 1))?
            .splitn(3, ',')
            .map(str::trim)
            .collect::<Vec<_>>();
        if fields.len() != 3 || fields.iter().any(|field| field.is_empty()) {
            return Err(anyhow!(
                "{}:{}: annotations require module, function and reason",
                path.display(),
                line_number + 1
            ));
        }
        let kind = match kind {
            "MANUAL" => Kind::Manual,
            "EXCLUDE" => Kind::Exclude,
            "SYNCED_ONLY" => Kind::SyncedOnly,
            _ => {
                return Err(anyhow!(
                    "{}:{}: unknown annotation {kind}",
                    path.display(),
                    line_number + 1
                ))
            }
        };
        result.push(Annotation {
            kind,
            module: fields[0].to_string(),
            function: fields[1].to_string(),
            reason: fields[2].to_string(),
        });
    }
    Ok(result)
}

/// Apply explicit exceptions and return the excluded function identities.
///
/// Annotation files are part of the generator's source of truth.  A typo in
/// either the module or function name must therefore fail generation instead
/// of silently leaving a declaration with the wrong lowering status.
pub fn apply(model: &mut ApiModel, annotations: &[Annotation]) -> Result<Vec<(String, String)>> {
    let mut excluded = Vec::new();
    let mut seen = BTreeSet::new();
    for annotation in annotations {
        let identity = (annotation.module.as_str(), annotation.function.as_str());
        if !seen.insert(identity) {
            return Err(anyhow!(
                "duplicate Wasm annotation for {}::{}",
                annotation.module,
                annotation.function
            ));
        }
        let module = model
            .modules
            .iter_mut()
            .find(|module| module.name == annotation.module)
            .ok_or_else(|| {
                anyhow!(
                    "Wasm annotation references unknown module {}",
                    annotation.module
                )
            })?;
        let function = module
            .functions
            .iter_mut()
            .find(|function| function.name == annotation.function)
            .ok_or_else(|| {
                anyhow!(
                    "Wasm annotation references unknown function {}::{}",
                    annotation.module,
                    annotation.function
                )
            })?;
        match annotation.kind {
            Kind::Manual => {
                function.status = LoweringStatus::Manual;
                function.notes.push(annotation.reason.clone());
            }
            Kind::Exclude => {
                excluded.push((module.name.clone(), function.name.clone()));
            }
            Kind::SyncedOnly => {
                function
                    .environments
                    .retain(|environment| environment.is_synced());
                function.notes.push(annotation.reason.clone());
            }
        }
    }
    for (module, function) in &excluded {
        if let Some(api) = model.modules.iter_mut().find(|api| api.name == *module) {
            api.functions
                .retain(|candidate| candidate.name != *function);
        }
    }
    Ok(excluded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parses_reasons_with_commas() {
        let mut path = std::env::temp_dir();
        path.push(format!("recoil-annotations-{}.def", std::process::id()));
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "MANUAL(gfx, Submit, host callback, bounded re-entry)").unwrap();
        let annotations = parse(&path).unwrap();
        assert_eq!(annotations[0].reason, "host callback, bounded re-entry");
        let _ = std::fs::remove_file(path);
    }

    fn model_with_function() -> ApiModel {
        ApiModel {
            model_version: 1,
            native_api_version: None,
            modules: vec![crate::model::ApiModule {
                name: "synthetic".to_string(),
                interface_version: "1.0".to_string(),
                functions: vec![crate::model::FunctionModel {
                    name: "Call".to_string(),
                    query: "CallQuery".to_string(),
                    result: "CallResult".to_string(),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    environments: std::collections::BTreeSet::from([
                        crate::model::Environment::RulesSynced,
                    ]),
                    mutating: false,
                    visibility_sensitive: false,
                    status: LoweringStatus::Automatic,
                    notes: Vec::new(),
                }],
                records: Vec::new(),
                enums: Vec::new(),
            }],
            callins: Vec::new(),
        }
    }

    #[test]
    fn rejects_annotations_for_unknown_declarations() {
        let mut model = model_with_function();
        let annotations = vec![Annotation {
            kind: Kind::Manual,
            module: "missing".to_string(),
            function: "Call".to_string(),
            reason: "test".to_string(),
        }];
        let error = apply(&mut model, &annotations).unwrap_err();
        assert!(error.to_string().contains("unknown module"));

        let annotations = vec![Annotation {
            kind: Kind::Manual,
            module: "synthetic".to_string(),
            function: "Missing".to_string(),
            reason: "test".to_string(),
        }];
        let error = apply(&mut model, &annotations).unwrap_err();
        assert!(error.to_string().contains("unknown function"));
    }

    #[test]
    fn rejects_duplicate_annotations() {
        let mut model = model_with_function();
        let annotations = vec![
            Annotation {
                kind: Kind::Manual,
                module: "synthetic".to_string(),
                function: "Call".to_string(),
                reason: "first".to_string(),
            },
            Annotation {
                kind: Kind::Exclude,
                module: "synthetic".to_string(),
                function: "Call".to_string(),
                reason: "second".to_string(),
            },
        ];
        let error = apply(&mut model, &annotations).unwrap_err();
        assert!(error.to_string().contains("duplicate"));
    }
}

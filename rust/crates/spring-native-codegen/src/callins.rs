//! Parser for the canonical `NativeInterface/api/Callins.def` inventory.

use anyhow::{anyhow, Context, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::model::{ApiModel, CallinModel, Environment};

#[derive(Debug, Clone)]
enum Entry {
    Callin {
        name: String,
        query: String,
        result: String,
        environments: BTreeSet<Environment>,
        aggregation: String,
        flags: Vec<String>,
    },
    Alias {
        name: String,
        target: String,
    },
}

/// Parse and resolve the inventory. Aliases are retained on their canonical
/// callin so generated traits/exports can preserve the engine's legacy names.
pub fn parse(path: &Path) -> Result<Vec<CallinModel>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read callin inventory {}", path.display()))?;
    let mut entries = Vec::new();
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
                "{}:{}: expected CALLIN(...) or ALIAS(...)",
                path.display(),
                line_number + 1
            )
        })?;
        let body = body
            .strip_suffix(')')
            .ok_or_else(|| anyhow!("{}:{}: missing ')'", path.display(), line_number + 1))?;
        let fields = body.split(',').map(str::trim).collect::<Vec<_>>();
        match kind {
            "CALLIN" if fields.len() == 6 => {
                let mut environments = BTreeSet::new();
                for value in fields[3].split('|') {
                    environments.insert(Environment::parse(value).ok_or_else(|| {
                        anyhow!(
                            "{}:{}: unknown environment {value}",
                            path.display(),
                            line_number + 1
                        )
                    })?);
                }
                if environments.is_empty() {
                    return Err(anyhow!(
                        "{}:{}: empty environment set",
                        path.display(),
                        line_number + 1
                    ));
                }
                let flags = if fields[5] == "none" {
                    Vec::new()
                } else {
                    fields[5].split('|').map(ToString::to_string).collect()
                };
                entries.push(Entry::Callin {
                    name: fields[0].to_string(),
                    query: fields[1].to_string(),
                    result: fields[2].to_string(),
                    environments,
                    aggregation: fields[4].to_string(),
                    flags,
                });
            }
            "ALIAS" if fields.len() == 2 => entries.push(Entry::Alias {
                name: fields[0].to_string(),
                target: fields[1].to_string(),
            }),
            _ => {
                return Err(anyhow!(
                    "{}:{}: malformed {} entry ({} fields)",
                    path.display(),
                    line_number + 1,
                    kind,
                    fields.len()
                ));
            }
        }
    }

    let mut canonical = BTreeMap::new();
    let mut aliases = Vec::new();
    for entry in entries {
        match entry {
            Entry::Callin {
                name,
                query,
                result,
                environments,
                aggregation,
                flags,
            } => {
                if canonical
                    .insert(
                        name.clone(),
                        (query, result, environments, aggregation, flags),
                    )
                    .is_some()
                {
                    return Err(anyhow!("duplicate callin {name}"));
                }
            }
            Entry::Alias { name, target } => aliases.push((name, target)),
        }
    }

    let mut alias_map = BTreeMap::<String, String>::new();
    for (name, target) in aliases {
        if alias_map.insert(name.clone(), target).is_some() {
            return Err(anyhow!("duplicate callin alias {name}"));
        }
    }

    let mut resolved = Vec::new();
    for (name, (query, result, environments, aggregation, flags)) in canonical {
        let mut names = alias_map
            .keys()
            .filter_map(|alias| {
                let mut current = alias.as_str();
                let mut seen = BTreeSet::new();
                while let Some(target) = alias_map.get(current) {
                    if !seen.insert(current) {
                        return None;
                    }
                    current = target;
                }
                (current == name).then_some(alias.clone())
            })
            .collect::<Vec<_>>();
        names.sort();
        resolved.push(CallinModel {
            name,
            query,
            result,
            environments,
            aggregation,
            aliases: names,
            flags,
        });
    }

    for alias in alias_map.keys() {
        let mut current = alias.as_str();
        let mut seen = BTreeSet::new();
        while let Some(target) = alias_map.get(current) {
            if !seen.insert(current) {
                return Err(anyhow!("callin alias cycle involving {alias}"));
            }
            current = target;
        }
        if !resolved.iter().any(|callin| callin.name == current) {
            return Err(anyhow!(
                "callin alias {alias} references unknown canonical callin {current}"
            ));
        }
    }
    Ok(resolved)
}

/// Check that a callin inventory covers every native function-pointer name.
/// This is intentionally a separate check because aliases are the one place
/// where a source name may not have its own query/result declaration.
pub fn validate_names(path: &Path, native_header: &Path) -> Result<()> {
    let inventory = parse(path)?;
    let text = std::fs::read_to_string(native_header)
        .with_context(|| format!("failed to read {}", native_header.display()))?;
    let declared = text
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("using ")?;
            let name = rest.split_once("FuncPtr")?.0;
            Some(name.to_string())
        })
        .collect::<BTreeSet<_>>();
    let mut inventory_names = BTreeSet::new();
    for callin in &inventory {
        inventory_names.insert(callin.name.clone());
        inventory_names.extend(callin.aliases.iter().cloned());
    }
    let missing = declared
        .difference(&inventory_names)
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "callin inventory is missing: {}",
            missing.join(", ")
        ))
    }
}

/// The Lua engine declares Allow* callins on CSyncedLuaHandle. Keep the
/// generated environment mask aligned with that authoritative declaration:
/// an unsynced or UI Wasm module must never be able to influence a synced
/// permission decision.
pub fn validate_synced_environments(
    callins: &[CallinModel],
    lua_synced_header: &Path,
) -> Result<()> {
    let text = std::fs::read_to_string(lua_synced_header)
        .with_context(|| format!("failed to read {}", lua_synced_header.display()))?;
    let class_start = text.find("class CSyncedLuaHandle").ok_or_else(|| {
        anyhow!(
            "{} does not declare CSyncedLuaHandle",
            lua_synced_header.display()
        )
    })?;
    let class_end = text[class_start..].find("\n};").ok_or_else(|| {
        anyhow!(
            "{} has an unterminated CSyncedLuaHandle",
            lua_synced_header.display()
        )
    })?;
    let class_body = &text[class_start..class_start + class_end];

    let lua_synced_allow_names = class_body
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with("bool ")
                && !line.starts_with("int ")
                && !line.starts_with("std::pair")
            {
                return None;
            }
            let name = line.split('(').next()?.split_whitespace().last()?;
            name.starts_with("Allow").then_some(name.to_string())
        })
        .collect::<BTreeSet<_>>();

    for callin in callins {
        let mut names = std::iter::once(&callin.name).chain(callin.aliases.iter());
        if !names.any(|name| lua_synced_allow_names.contains(name)) {
            continue;
        }
        if callin.environments.iter().any(|environment| {
            !matches!(
                environment,
                Environment::RulesSynced | Environment::GaiaSynced
            )
        }) {
            return Err(anyhow!(
                "synced Lua callin {} is exposed to an unsynced or UI Wasm environment",
                callin.name
            ));
        }
    }
    Ok(())
}

/// Validate the semantic side of the inventory after all NativeInterface
/// headers have been parsed.  This catches a typo that happens to match a
/// native function-pointer name but does not have a query/result definition.
pub fn validate_model(callins: &[CallinModel], model: &ApiModel) -> Result<()> {
    let records = model
        .modules
        .iter()
        .flat_map(|module| module.records.iter())
        .map(|record| record.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut names = BTreeSet::new();
    for callin in callins {
        if !names.insert(callin.name.as_str()) {
            return Err(anyhow!("duplicate canonical callin {}", callin.name));
        }
        if !records.contains(callin.query.as_str()) || !records.contains(callin.result.as_str()) {
            return Err(anyhow!(
                "callin {} references missing query/result ({}/{})",
                callin.name,
                callin.query,
                callin.result
            ));
        }
        if callin.aggregation.is_empty() {
            return Err(anyhow!("callin {} has no aggregation rule", callin.name));
        }
        if !matches!(
            callin.aggregation.as_str(),
            "ignore" | "or-true" | "and-false" | "first" | "first-non-empty"
        ) {
            return Err(anyhow!(
                "callin {} has unknown aggregation rule {}",
                callin.name,
                callin.aggregation
            ));
        }
        if callin.aggregation == "and-false" && callin.result != "BoolCallinResult" {
            return Err(anyhow!(
                "callin {} uses and-false with non-boolean result {}",
                callin.name,
                callin.result
            ));
        }
        for alias in &callin.aliases {
            if !names.insert(alias.as_str()) {
                return Err(anyhow!("duplicate callin alias {}", alias));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn resolves_aliases_and_preserves_metadata() {
        let mut path = std::env::temp_dir();
        path.push(format!("recoil-callins-{}.def", std::process::id()));
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "CALLIN(GameFrame, GameFrameQuery, GameFrameResult, rules-synced|gaia-synced, ignore, none)").unwrap();
        writeln!(file, "ALIAS(Frame, GameFrame)").unwrap();
        let callins = parse(&path).unwrap();
        assert_eq!(callins.len(), 1);
        assert_eq!(callins[0].aliases, vec!["Frame"]);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn rejects_synced_lua_allow_callins_in_unsynced_worlds() {
        let mut path = std::env::temp_dir();
        path.push(format!("recoil-lua-synced-{}.h", std::process::id()));
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "class CSyncedLuaHandle {{").unwrap();
        writeln!(file, "public:").unwrap();
        writeln!(file, "\tbool AllowCommand(int); ").unwrap();
        writeln!(file, "}};").unwrap();

        let callins = vec![CallinModel {
            name: "AllowCommand".to_string(),
            query: "Query".to_string(),
            result: "BoolCallinResult".to_string(),
            environments: BTreeSet::from([Environment::RulesSynced, Environment::RulesUnsynced]),
            aggregation: "and-false".to_string(),
            aliases: Vec::new(),
            flags: Vec::new(),
        }];
        let error = validate_synced_environments(&callins, &path).unwrap_err();
        assert!(error.to_string().contains("AllowCommand"));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn accepts_synced_lua_allow_callins_in_synced_worlds() {
        let mut path = std::env::temp_dir();
        path.push(format!("recoil-lua-synced-ok-{}.h", std::process::id()));
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "class CSyncedLuaHandle {{").unwrap();
        writeln!(file, "public:").unwrap();
        writeln!(file, "\tbool AllowCommand(int); ").unwrap();
        writeln!(file, "}};").unwrap();

        let callins = vec![CallinModel {
            name: "AllowCommand".to_string(),
            query: "Query".to_string(),
            result: "BoolCallinResult".to_string(),
            environments: BTreeSet::from([Environment::RulesSynced, Environment::GaiaSynced]),
            aggregation: "and-false".to_string(),
            aliases: Vec::new(),
            flags: Vec::new(),
        }];
        validate_synced_environments(&callins, &path).unwrap();
        let _ = std::fs::remove_file(path);
    }
}

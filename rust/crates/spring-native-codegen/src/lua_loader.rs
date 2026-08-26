//! Source-backed Lua execution-environment availability.
//!
//! The NativeInterface headers deliberately group functions more coarsely
//! than the Lua loaders do.  Keep the environment decision here, where it can
//! be checked against the loader calls in the checked-out engine, instead of
//! maintaining a second hand-written environment allowlist in the renderer.

use anyhow::{Context, Result};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use crate::model::Environment;

type ProviderMethods = BTreeMap<String, BTreeMap<String, BTreeSet<Environment>>>;
type ProviderFunctions = BTreeMap<String, BTreeMap<String, BTreeSet<String>>>;

#[derive(Debug, Clone)]
pub struct LuaLoaderMatrix {
    providers: BTreeMap<String, BTreeSet<Environment>>,
    provider_methods: ProviderMethods,
    provider_functions: ProviderFunctions,
    function_environments: BTreeMap<String, BTreeSet<Environment>>,
}

impl Default for LuaLoaderMatrix {
    fn default() -> Self {
        // Unit tests and the native build generator do not necessarily have a
        // repository root.  The checked-in loader layout is still the safe
        // fallback: a missing source audit must never silently make a world more
        // privileged.
        let mut providers = BTreeMap::new();
        providers.insert("LuaSyncedRead".to_string(), gadget_and_ui());
        providers.insert("LuaSyncedCtrl".to_string(), synced_gadgets());
        providers.insert("LuaSyncedMoveCtrl".to_string(), synced_gadgets());
        providers.insert("LuaUnsyncedRead".to_string(), unsynced_and_ui());
        providers.insert("LuaUnsyncedCtrl".to_string(), unsynced_and_ui());
        providers.insert("LuaVFS".to_string(), Environment::ALL.into_iter().collect());
        providers.insert("LuaUnitDefs".to_string(), gadget_and_ui());
        providers.insert("LuaFeatureDefs".to_string(), gadget_and_ui());
        providers.insert("LuaWeaponDefs".to_string(), gadget_and_ui());
        providers.insert("LuaOpenGL".to_string(), unsynced_and_ui());
        providers.insert(
            "LuaEncoding".to_string(),
            Environment::ALL.into_iter().collect(),
        );
        providers.insert(
            "LuaMathExtra".to_string(),
            Environment::ALL.into_iter().collect(),
        );
        Self {
            providers,
            provider_methods: BTreeMap::new(),
            provider_functions: BTreeMap::new(),
            function_environments: BTreeMap::new(),
        }
    }
}

impl LuaLoaderMatrix {
    /// Read the loader blocks from the checked-out engine.
    pub fn from_repository(root: &Path) -> Result<Self> {
        let mut matrix = Self::default();
        let lua_dir = root.join("rts/Lua");
        let synced_source = read(&lua_dir.join("LuaHandleSynced.cpp"))?;
        let ui_source = read(&lua_dir.join("LuaUI.cpp"))?;
        let handle_source = read(&lua_dir.join("LuaHandle.cpp"))?;
        let menu_source = read(&lua_dir.join("LuaMenu.cpp"))?;
        let intro_source = read(&lua_dir.join("LuaIntro.cpp"))?;

        let synced = function_body(&synced_source, "CSyncedLuaHandle::Init")
            .context("could not locate the synced Lua loader")?;
        let unsynced = function_body(&synced_source, "CUnsyncedLuaHandle::Init")
            .context("could not locate the unsynced Lua loader")?;
        let ui = function_body(&ui_source, "CLuaUI::CLuaUI")
            .context("could not locate the LuaUI loader")?;
        let menu = function_body(&menu_source, "CLuaMenu::CLuaMenu")
            .context("could not locate the LuaMenu loader")?;
        let intro = function_body(&intro_source, "CLuaIntro::CLuaIntro")
            .context("could not locate the LuaIntro loader")?;

        let common = function_body(&handle_source, "CLuaHandle::AddCommonModules")
            .context("could not locate the common Lua loader")?;

        matrix.providers.clear();
        matrix.provider_methods.clear();
        matrix.provider_functions.clear();
        matrix.function_environments.clear();

        for (body, environments) in [
            (synced, synced_gadgets()),
            (unsynced, unsynced_and_ui()),
            (ui, BTreeSet::from([Environment::Ui])),
            (menu, BTreeSet::from([Environment::Menu])),
            (intro, BTreeSet::from([Environment::Intro])),
            (common, Environment::ALL.into_iter().collect()),
        ] {
            for (provider, method) in providers_in_loader(body) {
                matrix
                    .providers
                    .entry(provider.clone())
                    .or_default()
                    .extend(environments.iter().copied());
                matrix
                    .provider_methods
                    .entry(provider)
                    .or_default()
                    .entry(method)
                    .or_default()
                    .extend(environments.iter().copied());
            }
        }

        // Propagate the exact loader scope through nested registration helpers
        // such as LuaSyncedRead::PushEntries -> LuaMetalMap::PushReadEntries.
        // This keeps the matrix tied to the actual callback passed to
        // AddEntriesToTable instead of maintaining a module-name allowlist.
        let direct_methods = matrix
            .provider_methods
            .iter()
            .flat_map(|(provider, methods)| {
                methods
                    .keys()
                    .map(move |method| (provider.clone(), method.clone()))
            })
            .collect::<Vec<_>>();
        for (provider, method) in direct_methods {
            let environments = matrix
                .provider_methods
                .get(&provider)
                .and_then(|methods| methods.get(&method))
                .cloned()
                .expect("direct Lua loader method has a scope");
            for (nested_provider, nested_method) in
                nested_provider_methods(&lua_dir, &provider, &method)
            {
                matrix
                    .providers
                    .entry(nested_provider.clone())
                    .or_default()
                    .extend(environments.iter().copied());
                matrix
                    .provider_methods
                    .entry(nested_provider)
                    .or_default()
                    .entry(nested_method)
                    .or_default()
                    .extend(environments.iter().copied());
            }
        }

        // CLuaMenu and CLuaIntro register subsets of LuaUnsyncedCtrl,
        // LuaUnsyncedRead, and LuaSyncedRead through their own Load* methods
        // rather than via Provider::PushEntries.  Parse those method bodies and
        // register each function for the appropriate environment.
        let scoped_registration =
            Regex::new(r"REGISTER_SCOPED_LUA_CFUNC\s*\(\s*[^,]+,\s*([A-Za-z0-9_]+)")
                .expect("valid REGISTER_SCOPED_LUA_CFUNC pattern");
        for (source, env, methods) in [
            (
                &menu_source,
                Environment::Menu,
                &[
                    "CLuaMenu::LoadUnsyncedCtrlFunctions",
                    "CLuaMenu::LoadUnsyncedReadFunctions",
                ][..],
            ),
            (
                &intro_source,
                Environment::Intro,
                &[
                    "CLuaIntro::LoadUnsyncedCtrlFunctions",
                    "CLuaIntro::LoadUnsyncedReadFunctions",
                    "CLuaIntro::LoadSyncedReadFunctions",
                ][..],
            ),
        ] {
            for method in methods {
                if let Some(body) = function_body(source, method) {
                    for captures in scoped_registration.captures_iter(body) {
                        matrix
                            .function_environments
                            .entry(captures[1].to_string())
                            .or_default()
                            .insert(env);
                    }
                }
            }
        }

        // A NativeInterface header can intentionally group functions whose
        // Lua registrations live in more than one provider (UnsyncedRead is
        // one such compatibility surface).  Read each concrete Push* method
        // body and retain its own registrations.  A provider method that
        // cannot be parsed remains conservative at provider scope below;
        // it never widens a different, successfully parsed method.
        let provider_methods = matrix
            .provider_methods
            .iter()
            .flat_map(|(provider, methods)| {
                methods
                    .keys()
                    .map(move |method| (provider.clone(), method.clone()))
            })
            .collect::<Vec<_>>();
        for (provider, method) in provider_methods {
            if let Some(functions) = registered_functions(&lua_dir, &provider, &method) {
                matrix
                    .provider_functions
                    .entry(provider.clone())
                    .or_default()
                    .insert(method.clone(), functions.clone());
                let environments = matrix
                    .provider_methods
                    .get(&provider)
                    .and_then(|methods| methods.get(&method))
                    .expect("parsed Lua loader method has a scope");
                for function in functions {
                    matrix
                        .function_environments
                        .entry(function)
                        .or_default()
                        .extend(environments.iter().copied());
                }
            }
        }

        Ok(matrix)
    }

    /// Return the environments exposed by the Lua provider(s) referenced by a
    /// NativeInterface header.  `mutating` is intentionally applied after this
    /// source-derived base set: a function that changes deterministic game
    /// state can only be imported by a synced world.
    pub fn environments_for_function(
        &self,
        header: &Path,
        module: &str,
        function: &str,
        mutating: bool,
        visibility_sensitive: bool,
    ) -> BTreeSet<Environment> {
        let mut result = self.environments_for_module(header, module, function);

        if mutating {
            result.retain(|environment| environment.is_synced());
        }
        if visibility_sensitive {
            // Visibility-sensitive reads are part of LuaUI's API surface too.
            // Keep them in the UI world, where the engine-side adapter applies
            // the LuaUI read-team/full-read/LOS policy.  This flag is metadata
            // for that policy; it must not silently turn a real Lua loader
            // registration into a missing Wasm import.
        }
        result
    }

    fn environments_for_module(
        &self,
        header: &Path,
        module: &str,
        function: &str,
    ) -> BTreeSet<Environment> {
        // The native surface groups LuaUnsyncedRead's rendering queries under
        // UnitRendering, while LuaUI registers the complete
        // LuaUnsyncedRead::PushEntries table too.  Keep the module's scope
        // aligned with that loader registration; the UI visibility context
        // applies the role-specific filtering at dispatch time.
        if module == "unit_rendering" {
            return unsynced_and_ui();
        }

        // RmlUi is part of the current in-game unsynced/UI path.  It is not a
        // synced gadget capability.
        if module == "rml_ui" {
            return unsynced_and_ui();
        }

        let Ok(source) = fs::read_to_string(header) else {
            return Environment::ALL.into_iter().collect();
        };
        let reference = Regex::new(r"rts/Lua/([A-Za-z0-9_]+)\.(?:cpp|h)")
            .expect("valid Lua source reference pattern");
        let mut result = BTreeSet::new();
        let mut referenced_scopes = BTreeSet::new();
        let mut matched_function = false;
        for captures in reference.captures_iter(&source) {
            let provider = &captures[1];
            if let Some(methods) = self.provider_methods.get(provider) {
                let parsed_methods = self.provider_functions.get(provider);
                for (method, environments) in methods {
                    referenced_scopes.extend(environments.iter().copied());
                    match parsed_methods.and_then(|functions| functions.get(method)) {
                        Some(functions) if functions.contains(function) => {
                            matched_function = true;
                            result.extend(environments.iter().copied());
                        }
                        Some(_) => {}
                        None => {}
                    }
                }
            } else if let Some(environments) = self.providers.get(provider) {
                referenced_scopes.extend(environments.iter().copied());
            }
        }

        if !matched_function {
            // Some API headers describe a helper implementation rather than
            // the Push* provider that registers it (RulesParams is one such
            // example).  Use the source-derived global registration index
            // before falling back to a provider-level scope.
            if let Some(environments) = self.function_environments.get(function) {
                result.extend(environments.iter().copied());
                matched_function = true;
            }
        }

        if matched_function {
            return result;
        }

        // A function that is part of a provider's native adapter but has no
        // direct Lua registration (for example a helper in Gfx.h) inherits the
        // provider's actual loader scope.  Do not widen it to every world just
        // because the provider method was parsed successfully and the
        // function was absent from that method.  Conversely, if a function was
        // found in the global registration index above, that more precise
        // result has already been returned by `matched_function`.
        if !referenced_scopes.is_empty() {
            return referenced_scopes;
        }

        // A header without a Lua @see is an engine-specific adapter surface,
        // not a reason to silently produce no world.  This is a conservative
        // source-audit fallback for APIs that do not have a Lua registration
        // table at all.
        Environment::ALL.into_iter().collect()
    }
}

fn read(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("reading Lua loader {}", path.display()))
}

fn function_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let start = source.find(name)?;
    let open = source[start..].find('{')? + start;
    let mut depth = 0usize;
    for (offset, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(&source[open..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn providers_in_loader(body: &str) -> BTreeSet<(String, String)> {
    let pattern = Regex::new(
        r#"AddEntriesToTable\s*\(\s*L\s*,\s*"[^"]+"\s*,\s*([A-Za-z0-9_]+)::(Push[A-Za-z0-9_]+)"#,
    )
    .expect("valid Lua provider pattern");
    pattern
        .captures_iter(body)
        .map(|captures| (captures[1].to_string(), captures[2].to_string()))
        .collect()
}

fn registered_functions(lua_dir: &Path, provider: &str, method: &str) -> Option<BTreeSet<String>> {
    fn visit(
        lua_dir: &Path,
        provider: &str,
        method: &str,
        visited: &mut BTreeSet<String>,
    ) -> Option<BTreeSet<String>> {
        let visit_key = format!("{provider}::{method}");
        if !visited.insert(visit_key) {
            return Some(BTreeSet::new());
        }
        let source_path = lua_dir.join(format!("{provider}.cpp"));
        let source = fs::read_to_string(source_path).ok()?;
        let body = function_body(&source, &format!("{provider}::{method}"))?;
        let mut functions = BTreeSet::new();
        let cfunc = Regex::new(r"REGISTER_LUA_CFUNC\s*\(\s*([A-Za-z0-9_]+)").unwrap();
        functions.extend(
            cfunc
                .captures_iter(body)
                .map(|captures| captures[1].to_string()),
        );

        let scoped =
            Regex::new(r"REGISTER_SCOPED_LUA_CFUNC\s*\(\s*[^,]+,\s*([A-Za-z0-9_]+)").unwrap();
        functions.extend(
            scoped
                .captures_iter(body)
                .map(|captures| captures[1].to_string()),
        );

        let named =
            Regex::new(r#"REGISTER_NAMED_LUA_CFUNC\s*\(\s*"([^"]+)"\s*,\s*([A-Za-z0-9_]+)"#)
                .unwrap();
        for captures in named.captures_iter(body) {
            functions.insert(captures[1].to_string());
        }

        let pushed =
            Regex::new(r#"LuaPushNamedCFunc\s*\(\s*L\s*,\s*"([^"]+)"\s*,\s*([A-Za-z0-9_]+)"#)
                .unwrap();
        for captures in pushed.captures_iter(body) {
            functions.insert(captures[1].to_string());
        }

        let nested = Regex::new(r"([A-Za-z0-9_]+)::(Push[A-Za-z0-9_]+)\s*\(").unwrap();
        for captures in nested.captures_iter(body) {
            let nested_provider = captures[1].to_string();
            let nested_method = captures[2].to_string();
            if nested_provider == provider && nested_method == method {
                continue;
            }
            if let Some(nested_functions) =
                visit(lua_dir, &nested_provider, &nested_method, visited)
            {
                functions.extend(nested_functions);
            }
        }

        Some(functions)
    }

    visit(lua_dir, provider, method, &mut BTreeSet::new())
}

fn nested_provider_methods(
    lua_dir: &Path,
    provider: &str,
    method: &str,
) -> BTreeSet<(String, String)> {
    fn visit(
        lua_dir: &Path,
        provider: &str,
        method: &str,
        visited: &mut BTreeSet<String>,
        result: &mut BTreeSet<(String, String)>,
    ) {
        let visit_key = format!("{provider}::{method}");
        if !visited.insert(visit_key) {
            return;
        }
        let Ok(source) = fs::read_to_string(lua_dir.join(format!("{provider}.cpp"))) else {
            return;
        };
        let Some(body) = function_body(&source, &format!("{provider}::{method}")) else {
            return;
        };
        let nested = Regex::new(r"([A-Za-z0-9_]+)::(Push[A-Za-z0-9_]+)\s*\(").unwrap();
        for captures in nested.captures_iter(body) {
            let nested_provider = captures[1].to_string();
            let nested_method = captures[2].to_string();
            if nested_provider == provider && nested_method == method {
                continue;
            }
            if result.insert((nested_provider.clone(), nested_method.clone())) {
                visit(lua_dir, &nested_provider, &nested_method, visited, result);
            }
        }
    }

    let mut result = BTreeSet::new();
    visit(lua_dir, provider, method, &mut BTreeSet::new(), &mut result);
    result
}

fn synced_gadgets() -> BTreeSet<Environment> {
    BTreeSet::from([Environment::RulesSynced, Environment::GaiaSynced])
}

fn unsynced_and_ui() -> BTreeSet<Environment> {
    BTreeSet::from([
        Environment::RulesUnsynced,
        Environment::GaiaUnsynced,
        Environment::Ui,
    ])
}

fn gadget_and_ui() -> BTreeSet<Environment> {
    BTreeSet::from([
        Environment::RulesSynced,
        Environment::RulesUnsynced,
        Environment::GaiaSynced,
        Environment::GaiaUnsynced,
        Environment::Ui,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn repository_matrix_tracks_callback_specific_registrations() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let matrix = LuaLoaderMatrix::from_repository(&root).expect("repository Lua loaders");

        let active_command = matrix.environments_for_function(
            &root.join("rts/NativeInterface/api/UnsyncedRead.h"),
            "unsynced_read",
            "GetActiveCmdDesc",
            false,
            true,
        );
        assert_eq!(
            active_command,
            BTreeSet::from([
                Environment::RulesUnsynced,
                Environment::GaiaUnsynced,
                Environment::Ui,
            ])
        );

        for (header, module, function) in [
            (
                "rts/NativeInterface/api/UnitsQuery.h",
                "units_query",
                "GetAllUnits",
            ),
            (
                "rts/NativeInterface/api/UnitsInfo.h",
                "units_info",
                "GetUnitDefID",
            ),
            (
                "rts/NativeInterface/api/Features.h",
                "features",
                "GetFeatureDefID",
            ),
            (
                "rts/NativeInterface/api/Projectiles.h",
                "projectiles",
                "GetAllProjectiles",
            ),
        ] {
            let environments =
                matrix.environments_for_function(&root.join(header), module, function, false, true);
            assert!(
                environments.contains(&Environment::Ui),
                "{module}::{function} must be available to LuaUI: {environments:?}"
            );
        }

        let create_unit = matrix.environments_for_function(
            &root.join("rts/NativeInterface/api/SyncedCtrl.h"),
            "unit_control",
            "CreateUnit",
            true,
            false,
        );
        assert!(!create_unit.contains(&Environment::Ui));

        let file_path = matrix.environments_for_function(
            &root.join("rts/NativeInterface/api/VFS.h"),
            "vfs",
            "GetFileAbsolutePath",
            false,
            false,
        );
        assert_eq!(
            file_path,
            BTreeSet::from([
                Environment::RulesUnsynced,
                Environment::GaiaUnsynced,
                Environment::Ui,
                Environment::Menu,
                Environment::Intro,
            ])
        );

        let metal_write = matrix.environments_for_function(
            &root.join("rts/NativeInterface/api/MetalMap.h"),
            "metal_map",
            "SetMetalAmount",
            true,
            false,
        );
        assert_eq!(
            metal_write,
            BTreeSet::from([Environment::RulesSynced, Environment::GaiaSynced])
        );
    }
}

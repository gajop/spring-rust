//! Maintainer/CI entry point for normalized Spring API generation.

use anyhow::{anyhow, Context, Result};
use spring_native_codegen::{
    annotations, callins, extract_api_version,
    manifest::API_DEFINITIONS,
    model::{ApiModel, ValidationMode},
    render_callins, render_host, render_signatures, render_wasm_sdk, render_wit, CodeGenerator,
};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

mod model {
    pub use spring_native_codegen::model::*;
}
#[path = "../render_core_wasm.rs"]
mod render_core_wasm;
#[path = "../render_core_wasm_host.rs"]
mod render_core_wasm_host;
#[path = "../render_core_wasm_option_host.rs"]
mod render_core_wasm_option_host;
#[path = "../render_core_wasm_variable_host.rs"]
mod render_core_wasm_variable_host;
#[path = "../render_core_wasm_variable_io_host.rs"]
mod render_core_wasm_variable_io_host;
#[path = "../render_core_wasm_variable_output_host.rs"]
mod render_core_wasm_variable_output_host;
#[path = "../render_core_wasm_registry.rs"]
mod render_core_wasm_registry;

fn main() {
    if let Err(error) = run() {
        eprintln!("spring-api-codegen: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let arguments = Arguments::parse(env::args().skip(1))?;
    let root = arguments.root.canonicalize().with_context(|| {
        format!(
            "failed to resolve repository root {}",
            arguments.root.display()
        )
    })?;
    let api_dir = root.join("rts/NativeInterface/api");
    let include_dirs = vec![api_dir.clone(), root.join("rts")];
    let codegen = CodeGenerator::with_repository_root(&root)
        .context("initializing libclang and auditing Lua loaders")?;

    let mut modules = Vec::new();
    for definition in API_DEFINITIONS {
        if let Some(only) = &arguments.only {
            if only != definition.module {
                continue;
            }
        }
        let header = api_dir.join(definition.header);
        let module = codegen
            .semantic_module(
                &header,
                &include_dirs,
                definition.api_struct,
                definition.module,
            )
            .with_context(|| format!("semantic model for {}", definition.module))?;
        modules.push(module);
    }
    if modules.is_empty() {
        return Err(anyhow!("no API modules selected"));
    }

    let callin_path = api_dir.join("Callins.def");
    let callin_header = root.join("rts/NativeInterface/NativeInterfaceEventClient.h");
    callins::validate_names(&callin_path, &callin_header)?;
    let callin_model = callins::parse(&callin_path)?;
    callins::validate_synced_environments(&callin_model, &root.join("rts/Lua/LuaHandleSynced.h"))?;
    let version = extract_api_version(&api_dir.join("Common.h"))?;
    let mut model = ApiModel {
        model_version: 1,
        native_api_version: Some([version.0, version.1, version.2]),
        modules,
        callins: callin_model,
    };
    callins::validate_model(&model.callins, &model)?;

    let annotations_path = api_dir.join("WasmAnnotations.def");
    let annotations = annotations::parse(&annotations_path)?;
    annotations::apply(&mut model, &annotations)?;
    let validation = model.validate(ValidationMode::Report);
    if arguments.strict {
        model.validate(ValidationMode::Strict)?;
        let adapter_errors = render_host::native_adapter_coverage_errors(&model);
        if !adapter_errors.is_empty() {
            return Err(anyhow!(
                "native Wasm adapter coverage is incomplete:\n{}",
                adapter_errors.join("\n")
            ));
        }
    }

    fs::create_dir_all(&arguments.output)?;
    write(
        &arguments.output.join("model.json"),
        &model.to_pretty_json()?,
    )?;
    write(
        &arguments.output.join("WasmCalloutRegistry.h"),
        &render_host::render_registry(&model),
    )?;
    write(
        &arguments.output.join("WasmHostAdapter.h"),
        &render_host::render_native_adapter_header(),
    )?;
    write(
        &arguments.output.join("WasmHostAdapterSupport.h"),
        &render_host::render_native_adapter_support(&model),
    )?;
    remove_old_adapter_translation_units(&arguments.output)?;
    write(
        &arguments.output.join("WasmHostAdapter.cpp"),
        &render_host::render_native_adapter_common(&model),
    )?;
    for module in &model.modules {
        write(
            &arguments
                .output
                .join(render_host::adapter_module_filename(&module.name)),
            &render_host::render_native_adapter_module(&model, module),
        )?;
    }
    write(
        &arguments.output.join("WasmCallinRegistry.h"),
        &render_host::render_callin_registry(&model),
    )?;
    write(
        &arguments.output.join("callins.json"),
        &(serde_json::to_string_pretty(&model.callins)? + "\n"),
    )?;

    // Runtime-neutral Core ABI plan plus executable fixed, option and variable
    // callback subsets. The validator registry is derived only from callbacks
    // emitted below, never from the broader planning inventory.
    write(
        &arguments.output.join("core-abi.json"),
        &render_core_wasm::render_json(&model)?,
    )?;
    write(
        &arguments.output.join("WasmCoreAbiInventory.h"),
        &render_core_wasm::render_inventory_header(&model),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedRegistry.h"),
        &render_core_wasm_registry::render(&model),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedBindings.h"),
        &render_core_wasm_host::render_header(),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedBindings.cpp"),
        &render_core_wasm_host::render_cpp(&model),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedOptionBindings.h"),
        &render_core_wasm_option_host::render_header(),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedOptionBindings.cpp"),
        &render_core_wasm_option_host::render_cpp(&model),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedVariableBindings.h"),
        &render_core_wasm_variable_host::render_header(),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedVariableBindings.cpp"),
        &render_core_wasm_variable_host::render_cpp(&model),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedVariableOutputBindings.h"),
        &render_core_wasm_variable_output_host::render_header(),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedVariableOutputBindings.cpp"),
        &render_core_wasm_variable_output_host::render_cpp(&model),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedVariableIoBindings.h"),
        &render_core_wasm_variable_io_host::render_header(),
    )?;
    write(
        &arguments.output.join("WasmCoreGeneratedVariableIoBindings.cpp"),
        &render_core_wasm_variable_io_host::render_cpp(&model),
    )?;

    let wit_dir = arguments.output.join("wit");
    fs::create_dir_all(&wit_dir)?;
    remove_old_wit_files(&wit_dir)?;
    for module in &model.modules {
        for (name, variant) in render_wit::interface_variants(module) {
            write(
                &wit_dir.join(format!("{}.wit", name)),
                &render_wit::render_module(&variant),
            )?;
        }
    }
    for environment in spring_native_codegen::Environment::ALL {
        write(
            &wit_dir.join(format!("{}.wit", environment.as_str())),
            &render_wit::render_world(&model, environment),
        )?;
        write(
            &wit_dir.join(format!("callins-{}.wit", environment.as_str())),
            &render_callins::render_wit(&model, environment),
        )?;
    }

    render_wit::validate_world_graph(&model)?;
    let signature = render_signatures::round_trip(&model, &root, &arguments.output)?;
    write(
        &arguments.output.join("signatures.json"),
        &(serde_json::to_string_pretty(&signature)? + "\n"),
    )?;

    let sdk_dir = arguments.output.join("sdk");
    fs::create_dir_all(&sdk_dir)?;
    write(
        &sdk_dir.join("generated.rs"),
        &render_wasm_sdk::render(&model),
    )?;
    write(
        &sdk_dir.join("callins.rs"),
        &render_callins::render_rust(&model),
    )?;

    let summary = model.summary();
    let core_summary = render_core_wasm::plan(&model);
    let mut report = serde_json::to_string_pretty(&summary)?;
    report.push('\n');
    report.push_str(&format!(
        "\n# core-wasm planning\nautomatic={} manual={} unsupported={}\n",
        core_summary.automatic_count,
        core_summary.manual_count,
        core_summary.unsupported_count,
    ));
    if let Err(error) = validation {
        report.push_str("\n# report-mode validation findings\n");
        report.push_str(&error.to_string());
        report.push('\n');
    }
    write(&arguments.output.join("generation-report.json"), &report)?;
    eprintln!(
        "generated {} modules / {} functions ({} unsupported); Core plan: {} automatic / {} manual / {} unsupported",
        summary.modules,
        summary.functions,
        summary.unsupported,
        core_summary.automatic_count,
        core_summary.manual_count,
        core_summary.unsupported_count,
    );
    Ok(())
}

fn write(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("writing {}", path.display()))
}

fn remove_old_adapter_translation_units(output: &Path) -> Result<()> {
    for entry in fs::read_dir(output)? {
        let entry = entry?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let is_adapter = name == "WasmHostAdapter.cpp"
            || (name.starts_with("WasmHostAdapter_") && name.ends_with(".cpp"));
        if is_adapter && path.is_file() {
            fs::remove_file(&path).with_context(|| format!("removing stale {}", path.display()))?;
        }
    }
    Ok(())
}

fn remove_old_wit_files(output: &Path) -> Result<()> {
    for entry in fs::read_dir(output)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("wit") {
            fs::remove_file(&path).with_context(|| format!("removing stale {}", path.display()))?;
        }
    }
    Ok(())
}

struct Arguments {
    root: PathBuf,
    output: PathBuf,
    strict: bool,
    only: Option<String>,
}

impl Arguments {
    fn parse(values: impl Iterator<Item = String>) -> Result<Self> {
        let mut root = PathBuf::from(".");
        let mut output = PathBuf::from("rts/wasm/generated");
        let mut strict = false;
        let mut only = None;
        let values = values.collect::<Vec<_>>();
        let mut index = 0;
        while index < values.len() {
            match values[index].as_str() {
                "--root" => {
                    index += 1;
                    root = PathBuf::from(values.get(index).context("--root needs a path")?);
                }
                "--output" => {
                    index += 1;
                    output = PathBuf::from(values.get(index).context("--output needs a path")?);
                }
                "--only" => {
                    index += 1;
                    only = Some(values.get(index).context("--only needs a module")?.clone());
                }
                "--strict" => strict = true,
                "--help" | "-h" => {
                    println!("Usage: spring-api-codegen [--root PATH] [--output DIR] [--only MODULE] [--strict]");
                    std::process::exit(0);
                }
                value => return Err(anyhow!("unknown argument {value}")),
            }
            index += 1;
        }
        Ok(Self {
            root,
            output,
            strict,
            only,
        })
    }
}

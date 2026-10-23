/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Typed adapters for generated transports whose legacy facade exposes only
//! the wire signature. These leave the legacy facade unchanged.
use crate::model::{ApiModel, RecordModel, SemanticType};
use crate::render_core_wasm::{self, CoreType, InputStrategy};
use crate::render_core_wasm_owned_guest::{rust_ident, rust_param_type, rust_type};
use crate::render_core_wasm_variable_io_host as variable_io;
use crate::render_core_wasm_variable_output_host as variable;
use crate::render_core_wasm_wire as wire;
use anyhow::{Context, Result, bail};
use std::collections::BTreeMap;

pub fn render(model: &ApiModel, owned: &str) -> Result<String> {
    use quote::ToTokens;
    let syntax = syn::parse_file(owned)?;
    let syn::Item::Mod(owner) = &syntax.items[0] else {
        bail!("owned module")
    };
    let typed = owner
        .content
        .as_ref()
        .unwrap()
        .1
        .iter()
        .filter_map(|i| match i {
            syn::Item::Mod(m) => Some(m),
            _ => None,
        })
        .flat_map(|m| {
            m.content
                .as_ref()
                .unwrap()
                .1
                .iter()
                .filter_map(|i| match i {
                    syn::Item::Fn(f)
                        if f.sig
                            .output
                            .to_token_stream()
                            .to_string()
                            .contains("Result <") =>
                    {
                        Some((m.ident.to_string(), f.sig.ident.to_string()))
                    }
                    _ => None,
                })
        })
        .collect::<std::collections::BTreeSet<_>>();
    let records = model
        .modules
        .iter()
        .flat_map(|m| m.records.iter().map(|r| (r.name.clone(), r.clone())))
        .collect::<BTreeMap<_, _>>();
    let plans = render_core_wasm::plan(model)
        .functions
        .into_iter()
        .map(|p| ((p.module.clone(), p.function.clone()), p))
        .collect::<BTreeMap<_, _>>();
    let mut source = String::new();
    for module in &model.modules {
        let name = rust_ident(&module.name);
        let local_records = module.records.iter().map(|r| (r.name.clone(), r)).collect();
        let local_enums = module.enums.iter().map(|e| (e.name.clone(), e)).collect();
        let mut methods = String::new();
        for f in &module.functions {
            if name == "rml_ui"
                && matches!(
                    f.name.as_str(),
                    "ContextAddEventListener" | "ElementAddEventListener" | "DataModelBindEvent"
                )
            {
                let normal = f
                    .inputs
                    .iter()
                    .filter(|field| {
                        !matches!(
                            field.ty,
                            SemanticType::Callback { .. } | SemanticType::Pointer { .. }
                        )
                    })
                    .collect::<Vec<_>>();
                let params = normal
                    .iter()
                    .map(|field| {
                        format!(
                            "{}: {}",
                            rust_ident(&field.name),
                            rust_param_type(&field.ty, &local_records, &local_enums)
                        )
                    })
                    .chain(std::iter::once("callback:F".into()))
                    .collect::<Vec<_>>()
                    .join(",");
                let (schema, call, field) = match f.name.as_str() {
                    "ContextAddEventListener" => (
                        "None",
                        "spring_sdk_internal::context_add_event_listener(context_handle,event,in_capture_phase,crate::callbacks::wire(id))",
                        "event_listener_handle",
                    ),
                    "ElementAddEventListener" => (
                        "None",
                        "spring_sdk_internal::element_add_event_listener(element_handle,event,in_capture_phase,crate::callbacks::wire(id))",
                        "event_listener_handle",
                    ),
                    _ => (
                        "Some(field_types)",
                        "spring_sdk_internal::data_model_bind_event(data_model_handle,name,crate::callbacks::wire(id),field_types)",
                        "event_handle",
                    ),
                };
                let function = rust_ident(&f.name);
                methods.push_str(&format!("pub fn {function}<E:crate::Environment,F:for<'call>Fn(&mut crate::Spring<'call,E>,crate::callbacks::Event)->Result<()>+'static>(engine:&crate::Backend<'_>,{params})->Result<{}Value> {{ let id=crate::callbacks::register::<E,F>(engine,{schema},callback)?; let result={call}.map(|r|{}Value{{{field}:r.handle,success:r.success}}); if !result.as_ref().is_ok_and(|r|r.success) {{crate::callbacks::remove(id);}} result }}",f.name,f.name));
                continue;
            }
            if f.inputs
                .iter()
                .any(|f| matches!(f.ty, SemanticType::Callback { .. }))
                && name != "rml_ui"
            {
                let params = f
                    .inputs
                    .iter()
                    .filter(|field| {
                        !matches!(
                            field.ty,
                            SemanticType::Callback { .. } | SemanticType::Pointer { .. }
                        )
                    })
                    .map(|field| {
                        format!(
                            "{}: {}",
                            rust_ident(&field.name),
                            rust_param_type(&field.ty, &local_records, &local_enums)
                        )
                    })
                    .collect::<Vec<_>>();
                let args = f
                    .inputs
                    .iter()
                    .filter(|field| {
                        !matches!(
                            field.ty,
                            SemanticType::Callback { .. } | SemanticType::Pointer { .. }
                        )
                    })
                    .map(|field| rust_ident(&field.name))
                    .collect::<Vec<_>>();
                let params = params
                    .into_iter()
                    .chain(std::iter::once("mut callback:F".into()))
                    .collect::<Vec<_>>()
                    .join(",");
                let args = args
                    .into_iter()
                    .chain(std::iter::once(
                        "||engine.with_nested::<E,_>(|spring|callback(spring))".into(),
                    ))
                    .collect::<Vec<_>>()
                    .join(",");
                let result = match f.outputs.as_slice() {
                    [] => "()".into(),
                    [field] => rust_type(&field.ty, &local_records, &local_enums),
                    _ => bail!("multi-result synchronous callback"),
                };
                let function = rust_ident(&f.name);
                methods.push_str(&format!("pub fn {function}<E:crate::Environment,F:for<'call>FnMut(&mut crate::Spring<'call,E>)>(engine:&crate::Backend<'_>,{params})->Result<{result}> {{ spring_sdk_internal::{function}({args}) }}"));
                continue;
            }
            if typed.contains(&(name.clone(), rust_ident(&f.name))) {
                continue;
            }
            let p = &plans[&(module.name.clone(), f.name.clone())];
            let params = f
                .inputs
                .iter()
                .map(|field| {
                    format!(
                        "{}: {}",
                        rust_ident(&field.name),
                        rust_param_type(&field.ty, &local_records, &local_enums)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            let result = match f.outputs.as_slice() {
                [] => "()".into(),
                [field] => rust_type(&field.ty, &local_records, &local_enums),
                _ => format!("{}Value", f.name),
            };
            if let Some(body) = manual_body(&name, &f.name) {
                methods.push_str(&format!(
                    "pub fn {}({params})->Result<{result}> {{ {body} }}",
                    rust_ident(&f.name)
                ));
                continue;
            }
            if crate::render_core_wasm_owned_guest::routes_to_borrowed(
                p, &f.inputs, &f.outputs, &records,
            ) {
                let mut setup = String::new();
                let mut args = Vec::new();
                for (i, field) in f.inputs.iter().enumerate() {
                    let arg = rust_ident(&field.name);
                    match &field.ty {
                        SemanticType::String => {
                            setup.push_str(&format!("let string{i}=alloc::ffi::CString::new({arg}).map_err(|_|ApiError::new(ErrorCode::InvalidArgument as i32))?;"));
                            args.push(format!("string{i}.as_c_str()"));
                        }
                        SemanticType::Option { inner }
                            if matches!(inner.as_ref(), SemanticType::String) =>
                        {
                            setup.push_str(&format!("let string{i}={arg}.map(alloc::ffi::CString::new).transpose().map_err(|_|ApiError::new(ErrorCode::InvalidArgument as i32))?;"));
                            args.push(format!("string{i}.as_deref()"));
                        }
                        SemanticType::Scalar { .. } | SemanticType::Handle { .. } => args.push(arg),
                        SemanticType::Enum { .. } => args.push(format!("{arg} as _")),
                        SemanticType::Bytes | SemanticType::List { .. } => args.push(arg),
                        _ => {
                            let (bytes, alignment) = wire::fixed_wire_layout(&field.ty, &records)
                                .context("borrowed fixed input")?;
                            let encoded =
                                encode_fixed(&field.ty, &arg, &format!("input{i}"), &records)?;
                            setup.push_str(&format!("let mut input{i}=super::Writer::default(); {encoded} input{i}.align({alignment}); let input{i}:[u8;{bytes}]=input{i}.bytes.try_into().map_err(|_|ApiError::new(ErrorCode::InvalidArgument as i32))?;"));
                            args.push(format!("&input{i}"));
                        }
                    }
                }
                let call = format!(
                    "spring_sdk_internal::generated::borrowed::{name}::{}({})?",
                    rust_ident(&f.name),
                    args.join(",")
                );
                let body = match &p.result_strategy {
                    render_core_wasm::ResultStrategy::Status => format!("{call}; Ok(())"),
                    render_core_wasm::ResultStrategy::Packed32 => format!("Ok({call})"),
                    render_core_wasm::ResultStrategy::FixedOutputBuffer { .. } => {
                        let mut fields = Vec::new();
                        let mut decode = String::new();
                        for field in &f.outputs {
                            let n = rust_ident(&field.name);
                            let value = decode_fixed(&field.ty, "reader", &records, &module.enums)?;
                            decode.push_str(&format!("let value_{n}={value};"));
                            fields.push((n.clone(), format!("value_{n}")));
                        }
                        let value = match fields.as_slice() {
                            [] => "()".into(),
                            [(_, v)] => v.clone(),
                            _ => format!(
                                "{}Value{{{}}}",
                                f.name,
                                fields
                                    .iter()
                                    .map(|(n, v)| format!("{n}:{v}"))
                                    .collect::<Vec<_>>()
                                    .join(",")
                            ),
                        };
                        format!(
                            "let bytes={call}; let mut reader=super::Reader::new(&bytes); {decode} Ok({value})"
                        )
                    }
                    _ => bail!("borrowed result strategy"),
                };
                methods.push_str(&format!(
                    "pub fn {}({params})->Result<{result}> {{ {setup} {body} }}",
                    rust_ident(&f.name)
                ));
                continue;
            }
            let var_io = variable_io::eligible(p, &f.inputs, &f.outputs, &records);
            if f.mutating || !(variable::eligible(p, &f.inputs, &f.outputs, &records) || var_io) {
                continue;
            }
            let (descriptor_bytes, fixed_offset) = if var_io {
                let layout = variable_io::output_descriptor_layout(&f.outputs, &records)
                    .context("I/O output layout")?;
                (layout.descriptor_bytes, layout.fixed_offset)
            } else {
                let layout = variable::output_layout(&f.outputs, &records)
                    .context("eligible output layout")?;
                (layout.descriptor_bytes, layout.fixed_offset)
            };
            let raw_params = p
                .direct_params
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    format!(
                        "p{i}: {}",
                        match t {
                            CoreType::I32 => "i32",
                            CoreType::I64 => "i64",
                            CoreType::F32 => "f32",
                            CoreType::F64 => "f64",
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            let has_input = !matches!(p.input_strategy, InputStrategy::Direct);
            let mut input_setup = String::new();
            if has_input {
                input_setup.push_str("let mut input=super::Writer::default();");
                for (i, field) in f
                    .inputs
                    .iter()
                    .filter(|f| !wire::direct_type(&f.ty))
                    .enumerate()
                {
                    let arg = rust_ident(&field.name);
                    if wire::variable_type(&field.ty) {
                        input_setup.push_str(&format!("let mut blob{i}=super::Writer::default();"));
                        match &field.ty {
                            SemanticType::String=>input_setup.push_str(&format!("blob{i}.u32({arg}.len() as u32); blob{i}.bytes.extend_from_slice({arg}.as_bytes());")),
                            SemanticType::Bytes=>input_setup.push_str(&format!("blob{i}.u32({arg}.len() as u32); blob{i}.bytes.extend_from_slice({arg});")),
                            SemanticType::List{element}=>{
                                let encode=encode_fixed(element,"*item",&format!("blob{i}"),&records)?;
                                let (_,alignment)=wire::fixed_wire_layout(element,&records).context("input list element")?;
                                input_setup.push_str(&format!("blob{i}.u32({arg}.len() as u32); for item in {arg}.iter() {{ {encode} }} blob{i}.align({alignment});"));
                            }
                            _=>unreachable!(),
                        }
                        input_setup.push_str(&format!("input.u32(blob{i}.bytes.as_ptr() as u32); input.u32(blob{i}.bytes.len() as u32);"));
                    } else if matches!(&field.ty,SemanticType::Option{inner} if matches!(inner.as_ref(),SemanticType::String))
                    {
                        input_setup.push_str(&format!("let blob{i}={arg}.unwrap_or(\"\").as_bytes(); input.u32({arg}.is_some() as u32); input.u32(blob{i}.as_ptr() as u32); input.u32(blob{i}.len() as u32);"));
                    } else {
                        input_setup.push_str(&encode_fixed(&field.ty, &arg, "input", &records)?);
                    }
                }
                let alignment = if var_io {
                    wire::input_descriptor_layout(&f.inputs, &records)
                        .context("I/O input layout")?
                        .1
                } else {
                    wire::layout_fields(
                        &f.inputs
                            .iter()
                            .filter(|f| !wire::direct_type(&f.ty))
                            .cloned()
                            .collect::<Vec<_>>(),
                        &records,
                    )
                    .context("fixed input layout")?
                    .1
                };
                input_setup.push_str(&format!("input.align({alignment});"));
            }
            let call_args = f
                .inputs
                .iter()
                .filter(|field| wire::direct_type(&field.ty))
                .map(|field| format!("{} as _", rust_ident(&field.name)))
                .chain(has_input.then_some("input.bytes.as_ptr() as i32".into()))
                .chain(std::iter::once("descriptor.as_mut_ptr() as i32".into()))
                .collect::<Vec<_>>()
                .join(",");
            let mut setup = String::new();
            let mut controls = String::new();
            let mut resize = String::new();
            let mut truncate = String::new();
            let mut values = BTreeMap::new();
            let mut variable_index = 0;
            let mut fixed = String::new();
            for field in &f.outputs {
                let field_name = rust_ident(&field.name);
                if matches!(
                    field.ty,
                    SemanticType::String | SemanticType::Bytes | SemanticType::List { .. }
                ) {
                    let (element_bytes, alignment) = match &field.ty {
                        SemanticType::List { element } => {
                            wire::fixed_wire_layout(element, &records).context("list layout")?
                        }
                        _ => (1, 1),
                    };
                    let stem = format!("out{variable_index}");
                    let offset = variable_index * 12;
                    setup.push_str(&format!("let mut {stem}=Vec::<u8>::new();"));
                    let capacity = if element_bytes == 1 {
                        format!("{stem}.len()")
                    } else {
                        format!("{stem}.len()/{element_bytes}")
                    };
                    controls.push_str(&format!("descriptor[{offset}..{}].copy_from_slice(&({stem}.as_mut_ptr() as u32).to_le_bytes()); descriptor[{}..{}].copy_from_slice(&(({capacity}) as u32).to_le_bytes());",offset+4,offset+4,offset+8));
                    resize.push_str(&format!("let required=u32::from_le_bytes(descriptor[{}..{}].try_into().unwrap()) as usize; {stem}.resize(required.checked_mul({element_bytes}).ok_or(ApiError::new(ErrorCode::OutOfBounds as i32))?,0);",offset+8,offset+12));
                    truncate.push_str(&format!("let required=(u32::from_le_bytes(descriptor[{}..{}].try_into().unwrap()) as usize).checked_mul({element_bytes}).ok_or(ApiError::new(ErrorCode::OutOfBounds as i32))?; if required>{stem}.len() {{ return Err(ApiError::new(ErrorCode::InvalidState as i32)); }} {stem}.truncate(required);",offset+8,offset+12));
                    let decode = match &field.ty {
                        SemanticType::String => format!("super::decode_string({stem})"),
                        SemanticType::Bytes => stem.clone(),
                        SemanticType::List { element } => {
                            let item = decode_fixed(element, "reader", &records, &module.enums)?;
                            format!(
                                "{{ let required=u32::from_le_bytes(descriptor[{}..{}].try_into().unwrap()) as usize; let mut reader=super::Reader::new(&{stem}); let mut items=Vec::with_capacity(required); for _ in 0..required {{ items.push({item}); reader.align({alignment})?; }} items }}",
                                offset + 8,
                                offset + 12
                            )
                        }
                        _ => unreachable!(),
                    };
                    values.insert(field_name, decode);
                    variable_index += 1;
                } else {
                    let decoded = decode_fixed(&field.ty, "reader", &records, &module.enums)?;
                    fixed.push_str(&format!("let value_{field_name}={decoded};"));
                    values.insert(field_name.clone(), format!("value_{field_name}"));
                }
            }
            let value = match f.outputs.as_slice() {
                [] => "()".into(),
                [field] => values[&rust_ident(&field.name)].clone(),
                _ => format!(
                    "{}Value {{ {} }}",
                    f.name,
                    f.outputs
                        .iter()
                        .map(|field| format!(
                            "{}: {}",
                            rust_ident(&field.name),
                            values[&rust_ident(&field.name)]
                        ))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            };
            let function = rust_ident(&f.name);
            let reader = if fixed.is_empty() {
                String::new()
            } else {
                format!("let mut reader=super::Reader::new(&descriptor[{fixed_offset}..]);")
            };
            methods.push_str(&format!("pub fn {function}({params})->Result<{result}> {{ unsafe extern \"C\" {{ #[link_name=\"{}\"] fn raw({raw_params})->i32; }} {input_setup} {setup} let mut descriptor=[0u8;{descriptor_bytes}]; loop {{ {controls} let status=unsafe{{raw({call_args})}}; if status==0 {{ {truncate} {reader} {fixed} return Ok({value}); }} if status!=ErrorCode::BufferOverflow as i32 {{ return Err(ApiError::new(status)); }} {resize} }} }}",p.import_name));
            // Rust import-module attributes belong on the block, not the
            // declaration. Insert it after constructing the body above.
            let start = methods.rfind("unsafe extern \"C\"").unwrap();
            methods.insert_str(
                start,
                &format!("#[link(wasm_import_module=\"{}\")] ", p.import_module),
            );
        }
        if !methods.is_empty() {
            let imports = spring_native_codegen::render_sdk::type_imports(&methods, module)
                + &spring_native_codegen::render_sdk::alloc_imports(&methods);
            let core_imports = ["Result", "ApiError", "ErrorCode"]
                .into_iter()
                .filter(|name| {
                    methods
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .any(|word| word == *name)
                })
                .collect::<Vec<_>>()
                .join(",");
            source.push_str(&format!(
                "pub mod {name} {{ {imports} use crate::{{{core_imports}}}; {methods} }}"
            ));
        }
    }
    spring_native_codegen::render_sdk::formatted(&source)
}

fn manual_body(module: &str, function: &str) -> Option<&'static str> {
    Some(match (module, function) {
        ("units_query", "GetUnitsInRectangle") => {
            "spring_sdk_internal::get_units_in_rectangle(xmin,zmin,xmax,zmax,allegiance)"
        }
        ("units_query", "GetUnitsInBox") => {
            "spring_sdk_internal::get_units_in_box(crate::Float3{x:xmin,y:ymin,z:zmin},crate::Float3{x:xmax,y:ymax,z:zmax},allegiance)"
        }
        ("units_query", "GetUnitsInSphere") => {
            "spring_sdk_internal::get_units_in_sphere(crate::Float3{x,y,z},radius,allegiance)"
        }
        ("units_query", "GetUnitsInCylinder") => {
            "spring_sdk_internal::get_units_in_cylinder(x,z,radius,allegiance)"
        }
        ("units_query", "GetUnitArrayCentroid") => {
            "spring_sdk_internal::get_unit_array_centroid(unit_i_ds)"
        }
        ("units_query", "GetUnitMapCentroid") => {
            "spring_sdk_internal::get_unit_map_centroid(unit_i_ds)"
        }
        ("units_query", "GetTeamUnitsByDefs") => {
            "super::collect_i32(|output|spring_sdk_internal::get_team_units_by_defs_into(team_id,unit_def_i_ds,output))"
        }
        ("units_commands", "GiveOrder") => {
            "spring_sdk_internal::give_order(cmd_id,params,options,timeout)"
        }
        ("units_commands", "GiveOrderToUnitMap") => {
            "spring_sdk_internal::give_order_to_unit_map(unit_i_ds,cmd_id,params,options,timeout)"
        }
        ("unit_control", "GiveOrderToUnit") => {
            "spring_sdk_internal::give_order_to_unit(unit_id,cmd_id,params,options,timeout)"
        }
        ("units_commands", "GetUnitCommands") => {
            "spring_sdk_internal::get_unit_commands(unit_id,max_commands).map(|commands|commands.into_iter().map(|c|CommandFFI{cmd_id:c.cmd_id,options:c.options,tag:c.tag,ai_command_id:c.ai_command_id,time_out:c.timeout,params:c.params}).collect())"
        }
        ("unit_defs", "GetUnitDefName") => {
            "spring_sdk_internal::get_unit_def_name_bytes(unit_def_id).map(super::decode_string)"
        }
        ("unit_defs", "GetUnitDefHumanName") => {
            "spring_sdk_internal::get_unit_def_human_name_bytes(unit_def_id).map(super::decode_string)"
        }
        ("gfx", "CreateTextureAtlas") => {
            "spring_sdk_internal::create_texture_atlas(xsize,ysize,alloc_type)"
        }
        ("gfx", "CreateTexture") => {
            "spring_sdk_internal::create_texture(xsize,ysize,zsize,spring_sdk_internal::TextureCreateParams{target:params.target,format:params.format,border:params.border,min_filter:params.min_filter,mag_filter:params.mag_filter,wrap_s:params.wrap_s,wrap_t:params.wrap_t,wrap_r:params.wrap_r,compare_func:params.compare_func,lod_bias:params.lod_bias,aniso:params.aniso,samples:params.samples,fbo:params.fbo,fbo_depth:params.fbo_depth})"
        }
        ("game", "GetModOption") => {
            "spring_sdk_internal::get_mod_option(key).map(|v|GetModOptionValue{exists:v.is_some(),value:v.unwrap_or_default()})"
        }
        ("rml_ui", "DataModelUnbindEvent") => {
            "spring_sdk_internal::data_model_unbind_event(event_handle)"
        }
        ("rml_ui", "EventListenerOnAttach") => {
            "spring_sdk_internal::event_listener_on_attach(event_listener_handle,element_handle)"
        }
        ("rml_ui", "EventListenerOnDetach") => {
            "spring_sdk_internal::event_listener_on_detach(event_listener_handle,element_handle)"
        }
        ("rml_ui", "EventListenerProcessEvent") => {
            "spring_sdk_internal::event_listener_process_event(event_listener_handle,event_handle)"
        }
        ("messages", "SendMessageToTeam") => {
            "spring_sdk_internal::send_message_to_team(team_id,message)"
        }
        ("cob_script", "CallCOBScript") => {
            "let mut ret_values=alloc::vec![0i32;ret_args as usize]; let result=spring_sdk_internal::call_cob_script(unit_id,func.id,&func.name,ret_args,args,&mut ret_values)?; ret_values.truncate(result.ret_count); Ok(CallCOBScriptValue{ret_code:result.ret_code,ret_values})"
        }
        ("ground_decals", "GetGroundDecalTextures") => {
            "let mut input=super::Writer::default(); input.u32(options.main_tex.is_some() as u32); input.u32(options.main_tex.unwrap_or(false) as u32); input.u32(options.include_filenames as u32); let mut output=Vec::new(); loop { match spring_sdk_internal::generated::dynamic_output::ground_decals::get_ground_decal_textures(input.bytes.as_ptr() as i32,&mut output) { Ok(required)=>{ if required>output.len(){return Err(ApiError::new(ErrorCode::InvalidState as i32));} output.truncate(required); let mut reader=super::Reader::new(&output); let textures=reader.strings()?; let filenames=reader.strings()?; reader.finish(8)?; return Ok(GetGroundDecalTexturesValue{textures,filenames}); }, Err(error) if error.error.code==ErrorCode::BufferOverflow as i32=>output.resize(error.required,0),Err(error)=>return Err(error.error) } }"
        }
        _ => return None,
    })
}

fn decode_fixed(
    ty: &SemanticType,
    reader: &str,
    records: &BTreeMap<String, RecordModel>,
    enums: &[crate::model::EnumModel],
) -> Result<String> {
    let (size, alignment) = wire::fixed_wire_layout(ty, records).context("fixed output")?;
    let expr = match ty {
        SemanticType::Scalar { name } if name == "bool" => format!("{reader}.bool()?"),
        SemanticType::Scalar { name } => format!(
            "{}::from_le_bytes({reader}.take()?) as _",
            if name == "char" {
                "u32"
            } else if size == 4 && !["f32", "u32", "i32"].contains(&name.as_str()) {
                "i32"
            } else {
                name
            }
        ),
        SemanticType::Handle { .. } => format!("u64::from_le_bytes({reader}.take()?)"),
        SemanticType::Enum { name } => {
            use heck::ToUpperCamelCase;
            let e = enums
                .iter()
                .find(|e| &e.name == name)
                .context("enum result")?;
            let mut seen = std::collections::BTreeSet::new();
            let cases = e
                .variants
                .iter()
                .filter(|(_, v)| seen.insert(**v))
                .map(|(n, v)| format!("{v}=>{name}::{},", n.to_lowercase().to_upper_camel_case()))
                .collect::<String>();
            format!(
                "match i32::from_le_bytes({reader}.take()?) {{ {cases} _=>return Err(ApiError::new(ErrorCode::InvalidState as i32)) }}"
            )
        }
        SemanticType::Record { name } => format!(
            "{name} {{ {} }}",
            records[name]
                .fields
                .iter()
                .map(|f| Ok(format!(
                    "{}: {}",
                    rust_ident(&f.name),
                    decode_fixed(&f.ty, reader, records, enums)?
                )))
                .collect::<Result<Vec<_>>>()?
                .join(",")
        ),
        SemanticType::FixedArray { element, length } => format!(
            "{{ let mut items=Vec::with_capacity({length}); for _ in 0..{length} {{ items.push({}); }} items }}",
            decode_fixed(element, reader, records, enums)?
        ),
        SemanticType::Option { inner } => {
            let (payload_bytes, payload_alignment) =
                wire::fixed_wire_layout(inner, records).context("optional output")?;
            let payload = decode_fixed(inner, reader, records, enums)?;
            format!(
                "{{ let present={reader}.bool()?; {reader}.align({payload_alignment})?; let value=if present {{ Some({payload}) }} else {{ {reader}.skip({payload_bytes})?; None }}; {reader}.align({alignment})?; value }}"
            )
        }
        _ => bail!("fixed output {ty:?}"),
    };
    Ok(format!("{{ {reader}.align({alignment})?; {expr} }}"))
}

fn encode_fixed(
    ty: &SemanticType,
    value: &str,
    writer: &str,
    records: &BTreeMap<String, RecordModel>,
) -> Result<String> {
    let (_, alignment) = wire::fixed_wire_layout(ty, records).context("fixed input")?;
    let body = match ty {
        SemanticType::Scalar { name } if name == "bool" => {
            format!("{writer}.u32(({value}) as u32);")
        }
        SemanticType::Scalar { name } => {
            let method = match name.as_str() {
                "f32" => "f32",
                "f64" => "f64",
                "i64" => "i64",
                "u64" => "u64",
                "i8" | "i16" | "i32" => "i32",
                _ => "u32",
            };
            let encoded = if name == method {
                value.to_owned()
            } else {
                format!("({value}) as {method}")
            };
            format!("{writer}.bytes.extend_from_slice(&({encoded}).to_le_bytes());")
        }
        SemanticType::Enum { .. } => {
            format!("{writer}.bytes.extend_from_slice(&(({value}) as i32).to_le_bytes());")
        }
        SemanticType::Handle { .. } => {
            format!("{writer}.bytes.extend_from_slice(&({value}).to_le_bytes());")
        }
        SemanticType::Record { name } => records[name]
            .fields
            .iter()
            .map(|f| {
                encode_fixed(
                    &f.ty,
                    &format!(
                        "({}).{}",
                        value.strip_prefix('*').unwrap_or(value),
                        rust_ident(&f.name)
                    ),
                    writer,
                    records,
                )
            })
            .collect::<Result<Vec<_>>>()?
            .join(""),
        SemanticType::FixedArray { element, length } => {
            let inner = encode_fixed(element, "*item", writer, records)?;
            format!(
                "if ({value}).len()!={length} {{ return Err(ApiError::new(ErrorCode::InvalidArgument as i32)); }} for item in ({value}).iter() {{ {inner} }}"
            )
        }
        SemanticType::Option { inner } => {
            let (bytes, payload_alignment) =
                wire::fixed_wire_layout(inner, records).context("optional input")?;
            let inner = encode_fixed(inner, "*payload", writer, records)?;
            format!(
                "{writer}.u32(({value}).is_some() as u32); {writer}.align({payload_alignment}); if let Some(payload)=({value}).as_ref() {{ {inner} }} else {{ {writer}.zeroes({bytes}); }} {writer}.align({alignment});"
            )
        }
        _ => bail!("fixed input {ty:?}"),
    };
    Ok(format!("{writer}.align({alignment}); {body}"))
}

#[cfg(test)]
mod routing_tests {
    use super::*;
    use crate::model::{ApiModule, Environment, FieldModel, FunctionModel, LoweringStatus};
    use std::collections::BTreeSet;
    fn field(name: &str, ty: SemanticType) -> FieldModel {
        FieldModel {
            name: name.into(),
            ty,
            status: LoweringStatus::Automatic,
            metadata: Vec::new(),
        }
    }
    fn scalar(name: &str) -> SemanticType {
        SemanticType::Scalar { name: name.into() }
    }
    fn function(name: &str, inputs: Vec<FieldModel>, outputs: Vec<FieldModel>) -> FunctionModel {
        FunctionModel {
            name: name.into(),
            query: format!("{name}Query"),
            result: format!("{name}Result"),
            inputs,
            outputs,
            environments: BTreeSet::from([Environment::Ui]),
            mutating: false,
            visibility_sensitive: false,
            status: LoweringStatus::Automatic,
            notes: Vec::new(),
            import_name: None,
        }
    }
    #[test]
    fn scoped_adapters_fill_unclaimed_routes_without_replacing_legacy_string_wire() {
        let sensor = function(
            "GetUnitSensorRadius",
            vec![
                field("unitID", scalar("i32")),
                field("type", SemanticType::String),
            ],
            vec![field(
                "radius",
                SemanticType::Record {
                    name: "UnitSensorRadius".into(),
                },
            )],
        );
        let mut config = function(
            "GetConfigInt",
            vec![
                field("key", SemanticType::String),
                field(
                    "defaultValue",
                    SemanticType::Option {
                        inner: Box::new(scalar("i32")),
                    },
                ),
            ],
            vec![
                field("value", scalar("i32")),
                field("exists", scalar("bool")),
            ],
        );
        config.inputs[1]
            .metadata
            .push("presence-field:hasDefault".into());
        let record = RecordModel {
            name: "UnitSensorRadius".into(),
            fields: [
                "los",
                "airLos",
                "radar",
                "sonar",
                "seismic",
                "radarJammer",
                "sonarJammer",
            ]
            .into_iter()
            .map(|n| field(n, scalar("f32")))
            .collect(),
            status: LoweringStatus::Automatic,
        };
        let model = ApiModel {
            model_version: 1,
            native_api_version: None,
            callins: Vec::new(),
            modules: vec![
                ApiModule {
                    name: "units_info".into(),
                    interface_version: "1.0".into(),
                    functions: vec![sensor],
                    records: vec![record.clone()],
                    enums: Vec::new(),
                },
                ApiModule {
                    name: "config".into(),
                    interface_version: "1.0".into(),
                    functions: vec![config],
                    records: Vec::new(),
                    enums: Vec::new(),
                },
            ],
        };
        let records = BTreeMap::from([(record.name.clone(), record)]);
        let plans = render_core_wasm::plan(&model);
        for function in model.modules.iter().flat_map(|m| &m.functions) {
            let plan = plans
                .functions
                .iter()
                .find(|p| p.function == function.name)
                .unwrap();
            assert!(crate::render_core_wasm_borrowed_host::eligible(
                plan,
                &function.inputs,
                &function.outputs,
                &records
            ));
            let dynamic = crate::render_core_wasm_dynamic_input_host::eligible(
                plan,
                &function.inputs,
                &function.outputs,
                &records,
            );
            let borrowed = crate::render_core_wasm_owned_guest::routes_to_borrowed(
                plan,
                &function.inputs,
                &function.outputs,
                &records,
            );
            match function.name.as_str() {
                "GetUnitSensorRadius" => {
                    assert!(dynamic);
                    assert!(
                        !borrowed,
                        "existing length-prefixed strings must keep their wire format"
                    );
                }
                "GetConfigInt" => {
                    assert!(!dynamic);
                    assert!(borrowed, "new option adapter must have a host registration");
                }
                _ => unreachable!(),
            }
        }
    }
}

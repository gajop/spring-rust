use std::{env, fs, process};

fn main() {
	if let Err(error) = run() {
		eprintln!("componentize: {error}");
		process::exit(1);
	}
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
	let mut arguments = env::args_os().skip(1);
	let input = arguments.next().ok_or("missing core Wasm input path")?;
	let output = arguments
		.next()
		.ok_or("missing Component Model output path")?;
	if arguments.next().is_some() {
		return Err("expected exactly two paths: <core-wasm> <component-wasm>".into());
	}

	let module = fs::read(input)?;
	let component = wit_component::ComponentEncoder::default()
		.module(&module)?
		.validate(true)
		.encode()?;
	fs::write(output, component)?;
	Ok(())
}

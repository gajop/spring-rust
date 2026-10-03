#[derive(Debug)]
struct HeaderComments;
impl bindgen::callbacks::ParseCallbacks for HeaderComments {
    fn process_comment(&self,comment:&str)->Option<String> {
        Some(comment.replace('[',"\\[").replace(']',"\\]"))
    }
}
// Maintainer tool; published crates use checked-in bindings, never require libclang.
fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../crates/spring-skirmish-ai-sys");
    let include = root.join("include");
    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include \"ExternalAI/Interface/AISEvents.h\"\n#include \"ExternalAI/Interface/AISCommands.h\"\n")
        .clang_arg(format!("-I{}", include.display()))
        .clang_arg("-std=c99")
        .allowlist_type("SSkirmishAICallback|S.*Event|S.*Command|EventTopic|CommandTopic|UnitCommandOptions")
        .allowlist_var("EVENT_.*|COMMAND_.*|UNIT_COMMAND_.*|NUM_.*|MAX_RESPONSE_SIZE")
        .prepend_enum_name(false)
        .generate_comments(true)
        .parse_callbacks(Box::new(HeaderComments))
        .layout_tests(true)
        .derive_default(true)
        .generate().expect("Skirmish AI headers must parse")
        .to_string().replace("extern \"C\" fn", "extern \"system\" fn");
    let path=root.join("src/generated.rs");
    if std::env::args().any(|arg|arg=="--check") {
        assert_eq!(std::fs::read_to_string(path).unwrap(),bindings,"checked-in bindings differ; regenerate them");
    } else {std::fs::write(path,bindings).unwrap();}
}

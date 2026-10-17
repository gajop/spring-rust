/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
use spring_sdk::{Menu, Result, Spring, module::Module};
struct Probe;
fn check(spring: &mut Spring<'_, Menu>) -> Result<()> {
    if spring
        .config()
        .get_config_string("ScopedSdkProof", Some(""))?
        .value
        == "expected-init-failure"
    {
        assert!(
            !spring.rml_ui().get_context("scoped-sdk-proof")?.exists,
            "failed initializer leaked its Rml context"
        );
        // Creation also checks the raw registry, including pending removals
        // and ownership, rather than just the visible-context lookup.
        let context = spring.rml_ui().create_context("scoped-sdk-proof")?;
        assert!(context.success);
        assert!(spring.rml_ui().remove_context(context.context_handle)?);
        assert!(spring.config().set_config_string(
            "ScopedSdkProof",
            "init-failure-cleaned",
            false
        )?);
    }
    Ok(())
}
impl Module<Menu> for Probe {
    fn initialize(spring: &mut Spring<'_, Menu>) -> Result<Self> {
        check(spring)?;
        Ok(Self)
    }
    fn shutdown(&self, spring: &mut Spring<'_, Menu>) -> Result<()> {
        check(spring)
    }
}
spring_sdk::export_module!(PROBE, Menu, Probe);

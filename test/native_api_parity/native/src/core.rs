use super::*;

impl NativeApiParity {
    pub(crate) fn find(&self, name: &str) -> Option<&'static TestSpec> {
        TESTS.iter().find(|spec| spec.name == name)
    }
    pub(crate) fn check_complete(&self) -> Result<(), String> {
        if self.failures == 0 {
            self.record("complete", "pass", "Native API parity checks passed");
        } else {
            self.record(
                "complete",
                "fail",
                &format!(
                    "Native API parity checks finished with {} failure(s)",
                    self.failures
                ),
            );
        }
        Ok(())
    }
}

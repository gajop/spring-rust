/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#[path = "../../workers.rs"]
mod workers;
#[cfg(feature = "all")]
use spring_sdk::All as Env;
#[cfg(not(feature = "all"))]
use spring_sdk::RulesSynced as Env;
use spring_sdk::{Result, Spring, module::Module};

struct WorkerModule;
impl Module<Env> for WorkerModule {
    fn initialize(_: &mut Spring<'_, Env>) -> Result<Self> {
        Ok(Self)
    }
}
spring_sdk::export_module!(WORKER, Env, WorkerModule);
spring_sdk::export_jobs!(WORKER, {
    DOUBLE => ("double", workers::double),
    OVERSIZED => ("oversized", workers::oversized),
});

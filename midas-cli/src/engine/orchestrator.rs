use crate::errors::MidasError;
use crate::shutdown::ShutdownReceiver;
use midas_types::models::config::Config;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct SimpleOrchestrator {
    pub config: Config,
    pub runtime: Arc<Runtime>,
}

impl SimpleOrchestrator {
    pub fn new(config: Config, runtime: Arc<Runtime>) -> Self {
        Self { config, runtime }
    }

    pub fn run(&mut self, _shutdown: ShutdownReceiver) -> Result<(), MidasError> {
        Ok(())
    }
}

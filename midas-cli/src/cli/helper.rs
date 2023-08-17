use std::fs;
use std::sync::Arc;

use midas_types::models::config::Config;
use midas_types::serde_yaml;

use crate::engine::SimpleOrchestrator as Midas;
use crate::errors::CliError;
use crate::errors::CliError::CannotReadConfig;
use crate::errors::CliError::ConfigurationFilePathNotProvided;

pub fn init_midas(config_paths: Vec<String>) -> Result<Midas, CliError> {
    let runtime = tokio::runtime::Runtime::new().map_err(CliError::FailedToCreateTokioRuntime)?;
    let config = runtime.block_on(load_config(config_paths))?;

    Ok(Midas::new(config, Arc::new(runtime)))
}

async fn load_config(config_paths: Vec<String>) -> Result<Config, CliError> {
    let first_config_path = config_paths.get(0);
    match first_config_path {
        None => Err(ConfigurationFilePathNotProvided),
        Some(path) => {
            let config_str =
                fs::read_to_string(path.clone()).map_err(|e| CannotReadConfig(path.clone(), e))?;

            let config: Config = serde_yaml::from_str(&config_str)
                .map_err(|e: serde_yaml::Error| CliError::FailedToParseYaml(Box::new(e)))?;

            Ok(config)
        }
    }
}

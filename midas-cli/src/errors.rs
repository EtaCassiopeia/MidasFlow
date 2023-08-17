use midas_types::errors::internal::BoxedError;
use midas_types::thiserror;
use midas_types::thiserror::Error;

#[derive(Error, Debug)]
pub enum MidasError {
    #[error(transparent)]
    CliError(#[from] CliError),
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration file path not provided")]
    ConfigurationFilePathNotProvided,
    #[error("Can't find the configuration file(s) at: {0:?}")]
    FailedToFindConfigurationFiles(String),
    #[error("Failed to create tokio runtime: {0}")]
    FailedToCreateTokioRuntime(#[source] std::io::Error),
    #[error("Failed to parse dozer config: {0:?}")]
    FailedToParseYaml(#[source] BoxedError),
    #[error("Cannot read configuration: {0:?}")]
    CannotReadConfig(String, #[source] std::io::Error),
}

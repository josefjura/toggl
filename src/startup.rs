use thiserror::Error;

use crate::{
    cli::{init_args, Command},
    config::{init_config, ConfigError},
};

#[derive(Debug, Error)]
pub enum StartupError {
    #[error("unable to find default config path")]
    ConfigurationError(ConfigError),
}

#[derive(Debug)]
pub struct Config {
    pub api_key: Option<String>,
    pub command: Command,
}

impl Config {
    fn merge_values() -> Result<Self, StartupError> {
        let config = init_config().map_err(StartupError::ConfigurationError)?;
        let args = init_args();
        let command = args.command();
        Ok(Self {
            // If `args.api_key` is None, fallback to `config.api_key`
            api_key: args.api_key.or(config.map(|c| c.api_key).unwrap_or(None)),
            command,
        })
    }

    pub fn new() -> Result<Self, StartupError> {
        Self::merge_values()
    }
}

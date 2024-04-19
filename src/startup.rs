use clap::Parser;
use config::{Config, ConfigError, Environment, File, FileFormat};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StartupError {
    #[error("unable to find default config path")]
    ConfigPathError,
    #[error("unable to initialize config")]
    ConfigInitError(ConfigError),
    #[error("unable to deserialize config")]
    ConfigDeserializationError(ConfigError),
    #[error("missing API key")]
    MissingAPIKey,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TogglConfig {
    #[serde(default)]
    api_key: Option<String>,
}

#[derive(Debug, Parser)]

struct TogglArgs {
    /// Key for Toggl API
    #[arg(long, short = 'k')]
    pub api_key: Option<String>,
}

const QUALIFIER: &str = "org";
const ORGANIZATION: &str = "beardo";
const APPLICATION: &str = "toggl-tui";
const FILE_NAME: &str = "toml.init";
const ENV_PREFIX: &str = "TOGGL";

impl TogglConfig {
    fn init_config() -> Result<Self, StartupError> {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);

        let config_path = dirs
            .map(|d| d.config_dir().join(FILE_NAME))
            .map(|d| d.to_str().unwrap().to_string())
            .ok_or(StartupError::ConfigPathError)?;

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::new(&config_path, FileFormat::Toml).required(false))
            .add_source(Environment::with_prefix(ENV_PREFIX).separator("_"))
            .build()
            .map_err(StartupError::ConfigInitError)?;

        s.try_deserialize()
            .map_err(StartupError::ConfigDeserializationError)
    }
}
pub struct Startup {
    pub api_key: String,
}

impl Startup {
    fn init_args() -> TogglArgs {
        TogglArgs::parse()
    }

    fn merge_values() -> Result<Self, StartupError> {
        let config = TogglConfig::init_config()?;
        let args = Self::init_args();

        let api_key = args
            .api_key
            .or_else(|| config.api_key)
            .ok_or(StartupError::MissingAPIKey)?;

        Ok(Self { api_key })
    }

    pub fn new() -> Result<Self, StartupError> {
        Self::merge_values()
    }
}

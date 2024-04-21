use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("unable to find default config path")]
    PathNotFound,
    #[error("unable to parse config file")]
    FileNotParsed(std::io::Error),
    #[error("unable to serialize config file")]
    SerilizationError(toml::ser::Error),
    #[error("unable to create config directory")]
    CantCreateDirectory(std::io::Error),
    #[error("unable to create a config file")]
    FileNotCreated(std::io::Error),
    #[error("unable to write to a config file")]
    CantWriteToFile(std::io::Error),
}

pub const QUALIFIER: &str = "org";
pub const ORGANIZATION: &str = "beardo";
pub const APPLICATION: &str = "toggl-tui";
pub const FILE_NAME: &str = "toml.init";

#[derive(Deserialize, Serialize)]
pub struct ConfigFile {
    pub api_key: Option<String>,
}

pub fn get_directory_path() -> Result<std::path::PathBuf, ConfigError> {
    let dirs =
        ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).ok_or(ConfigError::PathNotFound)?;

    Ok(dirs.config_dir().to_path_buf())
}

fn ensure_path() -> Result<std::path::PathBuf, ConfigError> {
    let dir_path = get_directory_path()?;

    std::fs::create_dir_all(&dir_path).map_err(ConfigError::CantCreateDirectory)?;

    Ok(dir_path.join(FILE_NAME))
}

pub fn init_config() -> Result<Option<ConfigFile>, ConfigError> {
    let file_path = ensure_path()?;

    if !file_path.exists() {
        return Ok(None);
    }

    let config = std::fs::read_to_string(&file_path)
        .map(|f| toml::from_str::<ConfigFile>(&f).ok())
        .map_err(ConfigError::FileNotParsed)?;

    Ok(config)
}

pub fn save_api_key(api_key: &str) -> Result<(), ConfigError> {
    let file_path = ensure_path()?;

    if !file_path.exists() {
        std::fs::File::create(&file_path).map_err(ConfigError::FileNotCreated)?;
    }

    let config = std::fs::read_to_string(&file_path)
        .map(|f| toml::from_str::<ConfigFile>(&f).ok())
        .map_err(ConfigError::FileNotParsed)?;

    let config = match config {
        Some(mut c) => {
            c.api_key = Some(api_key.to_string());
            c
        }
        None => ConfigFile {
            api_key: Some(api_key.to_string()),
        },
    };

    let content = toml::to_string(&config).map_err(ConfigError::SerilizationError)?;

    std::fs::write(&file_path, content).map_err(ConfigError::CantWriteToFile)?;

    Ok(())
}

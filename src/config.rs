use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use tracing::{debug, warn};

use crate::format::OutputFormat;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    pub timeout_secs: u64,
    pub retries: u32,
    pub delay_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct OutputConfig {
    pub format: OutputFormat,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            retries: 3,
            delay_ms: 100,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Table,
        }
    }
}

fn user_config_path() -> Option<PathBuf> {
    user_config_path_from(
        std::env::var_os("XDG_CONFIG_HOME").as_deref(),
        std::env::var_os("HOME").as_deref(),
    )
}

pub fn user_config_path_from(
    xdg_config_home: Option<&OsStr>,
    home: Option<&OsStr>,
) -> Option<PathBuf> {
    if let Some(config_home) = xdg_config_home {
        return Some(
            PathBuf::from(config_home)
                .join("continente")
                .join("continente.toml"),
        );
    }

    home.map(PathBuf::from).map(|home| {
        home.join(".config")
            .join("continente")
            .join("continente.toml")
    })
}

pub fn try_load(path: &Path) -> anyhow::Result<Option<Config>> {
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            let config: Config = toml::from_str(&contents)
                .map_err(|e| anyhow::anyhow!("Failed to parse {}: {e}", path.display()))?;
            Ok(Some(config))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn load_config(explicit_path: Option<&Path>) -> anyhow::Result<Config> {
    if let Some(path) = explicit_path {
        return match try_load(path) {
            Ok(Some(config)) => Ok(config),
            Ok(None) => Err(anyhow::anyhow!("Config file not found: {}", path.display())),
            Err(e) => Err(e.context(format!("Failed to load config from {}", path.display()))),
        };
    }

    load_config_from_paths(Path::new("continente.toml"), user_config_path().as_deref())
}

pub fn load_config_from_paths(
    local_path: &Path,
    user_path: Option<&Path>,
) -> anyhow::Result<Config> {
    match try_load(local_path) {
        Ok(Some(config)) => return Ok(config),
        Ok(None) => {}
        Err(e) => warn!("Failed to parse {}: {e}", local_path.display()),
    }

    if let Some(user_path) = user_path {
        debug!("Checking user config path {}", user_path.display());
        match try_load(user_path) {
            Ok(Some(config)) => return Ok(config),
            Ok(None) => {}
            Err(e) => warn!("Failed to parse {}: {e}", user_path.display()),
        }
    }

    debug!("No config file found, using defaults");
    Ok(Config::default())
}

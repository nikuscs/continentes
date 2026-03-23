use std::path::{Path, PathBuf};

use serde::Deserialize;
use tracing::{debug, warn};

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
    pub format: String,
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
            format: String::from("table"),
        }
    }
}

impl Config {
    pub fn load(explicit_path: Option<&Path>) -> Self {
        if let Some(path) = explicit_path {
            return Self::load_from_file(path).unwrap_or_else(|e| {
                warn!("Failed to load config from {}: {e}", path.display());
                Self::default()
            });
        }

        let candidates = Self::config_candidates();
        for path in candidates {
            if path.exists() {
                debug!("Loading config from {}", path.display());
                match Self::load_from_file(&path) {
                    Ok(config) => return config,
                    Err(e) => {
                        warn!("Failed to parse {}: {e}", path.display());
                        return Self::default();
                    }
                }
            }
        }

        debug!("No config file found, using defaults");
        Self::default()
    }

    fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }

    fn config_candidates() -> Vec<PathBuf> {
        let mut candidates = vec![PathBuf::from("continente.toml")];

        if let Ok(home) = std::env::var("HOME") {
            candidates.push(
                PathBuf::from(home)
                    .join(".config")
                    .join("continente")
                    .join("continente.toml"),
            );
        }

        candidates
    }
}

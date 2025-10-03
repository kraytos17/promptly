use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;

/// Use thiserror to simplify error definitions
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct ModelSettings {
    pub auto_save: bool,
    pub compression: bool,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DefaultSettings {
    pub order: usize,
    pub max_words: usize,
    pub corpus: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub defaults: DefaultSettings,
    pub model: ModelSettings,
    pub logging: LoggingSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            defaults: DefaultSettings {
                order: 2,
                max_words: 100,
                corpus: "corpora/default.txt".to_string(),
            },
            model: ModelSettings {
                auto_save: true,
                compression: false,
                format: "json".to_string(),
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

impl Settings {
    /// Load settings from a specific file
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let settings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    /// Load default.json if no explicit config is given
    pub fn load_default() -> Result<Self, ConfigError> {
        let path = Path::new("config/default.json");
        if path.exists() {
            log::info!("Loading default config from {}", path.display());
            Self::load_from(path)
        } else {
            log::warn!("No config/default.json found, using built-in defaults");
            Ok(Self::default())
        }
    }

    /// Load settings from an optional path.
    /// If `Some(path)` is given, loads from that path.
    /// Otherwise, tries config/default.json, and finally falls back to built-in defaults.
    pub fn load_or_default<P: AsRef<Path>>(path: Option<P>) -> Result<Self, ConfigError> {
        if let Some(p) = path {
            let path_ref = p.as_ref();
            if path_ref.exists() {
                log::info!("Loading config from {}", path_ref.display());
                return Self::load_from(path_ref);
            }

            log::warn!(
                "Config file {} not found, falling back to default.json",
                path_ref.display()
            );
        }
        Self::load_default()
    }
}

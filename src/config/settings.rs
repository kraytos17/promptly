use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct ModelSettings {
    pub auto_save: bool,
    pub compression: bool,
    pub format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingSettings {
    pub level: String,
    pub file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultSettings {
    pub order: usize,
    pub max_words: usize,
    pub corpus: String,
}

#[derive(Debug, Deserialize, Clone)]
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
                format: "yaml".to_string(),
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let config_paths = vec![
            "config.yaml",
            "config/default.yaml",
            "~/.config/promptly/config.yaml",
        ];

        for path in config_paths {
            if Path::new(path).exists() {
                let content = fs::read_to_string(path)?;
                let settings = serde_yaml::from_str(&content)?;
                return Ok(settings);
            }
        }

        log::warn!("No config file found, using defaults");
        Ok(Self::default())
    }
}

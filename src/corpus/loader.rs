use std::{fs::File, io::Read, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CorpusError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid encoding")]
    InvalidEncoding,
}

pub fn load_text(path: &Path) -> Result<String, CorpusError> {
    if !path.exists() {
        return Err(CorpusError::FileNotFound(path.display().to_string()));
    }

    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn preprocess_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() || c == '\'' || c == '-' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

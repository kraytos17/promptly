use crate::markov::MarkovChain;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
struct SavedModel {
    pub order: usize,
    pub transitions: HashMap<Vec<String>, HashMap<String, usize>>,
}

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_yaml::Error),

    #[error("Invalid model format")]
    InvalidFormat,
}

pub fn save_model<P: AsRef<Path>>(chain: &MarkovChain, path: P) -> Result<(), ModelError> {
    let saved_model = SavedModel {
        order: chain.order,
        transitions: chain.transitions.clone(),
    };

    let serialized = serde_yaml::to_string(&saved_model)?;
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

pub fn load_model<P: AsRef<Path>>(path: P) -> Result<MarkovChain, ModelError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let saved_model: SavedModel = serde_yaml::from_str(&contents)?;

    Ok(MarkovChain {
        order: saved_model.order,
        transitions: saved_model.transitions,
    })
}

use crate::markov::{Interner, MarkovChain, chain::State};
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
    pub interner_words: Vec<String>,
    #[serde(with = "crate::markov::chain::vec_key_map")]
    pub states: HashMap<Vec<usize>, State>,
}

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Invalid model format")]
    InvalidFormat,
}

pub fn save_model<P: AsRef<Path>>(chain: &MarkovChain, path: P) -> Result<(), ModelError> {
    let saved_model = SavedModel {
        order: chain.order,
        interner_words: chain.interner.id_to_word.clone(),
        states: chain.states.clone(),
    };

    let serialized = serde_json::to_string(&saved_model)?;
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

pub fn load_model<P: AsRef<Path>>(path: P) -> Result<MarkovChain, ModelError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let saved_model: SavedModel = serde_json::from_str(&contents)?;

    let mut interner = Interner::new();
    for word in &saved_model.interner_words {
        interner.get_or_intern(word);
    }

    Ok(MarkovChain {
        order: saved_model.order,
        states: saved_model.states,
        interner,
    })
}

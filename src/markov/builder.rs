use crate::{
    corpus::{CorpusError, load_text, preprocess_text},
    markov::MarkovChain,
};
use std::path::Path;

pub fn train_from_corpus(corpus_path: &Path, order: usize) -> Result<MarkovChain, CorpusError> {
    log::info!("Loading corpus from: {}", corpus_path.display());
    let raw_text = load_text(corpus_path)?;

    log::info!("Preprocessing text (length: {} chars)", raw_text.len());
    let processed_text = preprocess_text(&raw_text);

    log::info!("Training Markov chain (order: {order})");
    let mut chain = MarkovChain::new(order);
    chain.train(&processed_text);

    log::info!("Training completed. States: {}", chain.states.len());
    Ok(chain)
}

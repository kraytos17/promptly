mod builder;
pub mod chain;
pub mod interner;

pub use builder::train_from_corpus;
pub use chain::{GenerationError, MarkovChain};
pub use interner::Interner;

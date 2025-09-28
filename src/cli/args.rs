use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "promptly")]
#[command(about = "Offline text generator using Markov chains", long_about = None)]
pub struct Args {
    /// The prompt to start generation from
    pub prompt: String,

    /// Maximum number of words to generate
    #[arg(short, long, default_value_t = 100)]
    pub max_words: usize,

    /// Order of the Markov chain (1-3 recommended)
    #[arg(short, long, default_value_t = 2)]
    pub order: usize,

    /// Training corpus file path
    #[arg(short, long, default_value = "corpora/default.txt")]
    pub corpus: PathBuf,

    /// Load a pre-trained model instead of training from corpus
    #[arg(long)]
    pub model: Option<PathBuf>,

    /// Save the trained model to file
    #[arg(long)]
    pub save_model: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

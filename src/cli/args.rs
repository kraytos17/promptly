use clap::{ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "promptly")]
#[command(about = "Offline text generator using Markov chains", long_about = None)]
#[command(group(
    ArgGroup::new("corpus_source")
        .required(false)
        .args(&["corpus", "corpus_files", "corpus_dir"])
))]
pub struct Args {
    /// The prompt to start generation from
    pub prompt: String,
    /// Maximum number of words to generate
    #[arg(short, long, default_value_t = 100)]
    pub max_words: usize,
    /// Order of the Markov chain (1-3 recommended)
    #[arg(short, long, default_value_t = 2)]
    pub order: usize,
    /// Single training corpus file path
    #[arg(short, long, default_value = "corpora/default.txt")]
    pub corpus: PathBuf,
    /// Multiple training corpus files
    #[arg(long)]
    pub corpus_files: Vec<PathBuf>,
    /// Directory containing training corpus files
    #[arg(long)]
    pub corpus_dir: Option<PathBuf>,
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

mod cli;
mod config;
mod corpus;
mod logger;
mod markov;
mod model;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use config::Settings;
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> Result<()> {
    let args = Args::parse();
    let settings = Settings::load_default()?;
    logger::init_logger(&settings.logging).expect("Can't init the logger");
    log::debug!("Configuration loaded: {settings:?}");

    let mut chain = if let Some(model_path) = &args.model {
        log::info!("Loading pre-trained model from: {}", model_path.display());
        model::load_model(model_path)?
    } else {
        log::info!("Training new model from corpus: {}", args.corpus.display());
        markov::train_from_corpus(&args.corpus, args.order)?
    };

    log::info!("Generating text with prompt: '{}'", args.prompt);
    let generated = chain.generate(&args.prompt, args.max_words)?;
    println!("{generated}");

    if let Some(save_path) = &args.save_model {
        log::info!("Saving model to: {}", save_path.display());
        model::save_model(&chain, save_path)?;
    } else if settings.model.auto_save && args.model.is_none() {
        let models_dir = Path::new("models");
        if !models_dir.exists() {
            fs::create_dir_all(models_dir)?;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let auto_path = models_dir.join(format!("model_order_{}_{}.json", args.order, timestamp));
        log::info!("Auto-saving model to: {}", auto_path.display());
        model::save_model(&chain, &auto_path)?;
    }

    log::info!("Generation completed successfully");
    Ok(())
}

mod cli;
mod config;
mod corpus;
mod markov;
mod model;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use config::Settings;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting Promptly");

    let args = Args::parse();
    let settings = Settings::load()?;
    log::debug!("Configuration loaded: {settings:?}");

    let chain = if let Some(model_path) = &args.model {
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
        let auto_path = format!("model_order_{}.yaml", args.order);
        log::info!("Auto-saving model to: {auto_path}");
        model::save_model(&chain, auto_path)?;
    }

    log::info!("Generation completed successfully");
    Ok(())
}

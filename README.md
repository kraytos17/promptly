# promptly
Offline text generator CLI using simple Markov chains.

- Core types and algorithms: [`markov::MarkovChain`](src/markov/chain.rs), [`markov::train_from_corpus`](src/markov/builder.rs)
- Model persistence (YAML): [`model::save_model`](src/model/mod.rs), [`model::load_model`](src/model/mod.rs)
- Corpus handling: [`corpus::load_text`](src/corpus/loader.rs), [`corpus::preprocess_text`](src/corpus/loader.rs)
- Entry point: [src/main.rs](src/main.rs)

## Features
- Train an $n$-gram Markov chain from a text corpus (order 1–3 recommended).
- Generate text from a prompt, or start from a random state if the prompt is too short.
- Save/load trained models as YAML.
- Optional auto-save of trained models via config when not loading an existing model.
- Logging via `env_logger` and `log`.

## Install
Requires Rust and Cargo.

```sh
cargo build --release
# or install into your Cargo bin
cargo install --path .
```

## Usage
```sh
promptly [OPTIONS] <PROMPT>
```

Options:
- `-m, --max-words <N>`: Maximum number of words to generate (default: 100)
- `-o, --order <N>`: Markov chain order, i.e., $n$-gram size (default: 2)
- `-c, --corpus <PATH>`: Training corpus file (default: corpora/default.txt)
- `--model <PATH>`: Load a pre-trained model instead of training
- `--save-model <PATH>`: Save the trained model to the given path
- `-v, --verbose`: Currently a no-op (use RUST_LOG for log level)

Examples:
```sh
# Generate using the default corpus with order 2
promptly "hello world"

# Train from a specific corpus and save the model
promptly -c corpora/default.txt -o 2 --max-words 80 --save-model model.yaml "the quick"

# Load an existing model and generate
promptly --model model.yaml --max-words 120 "knowledge is"

# Control logging via environment (recommended)
RUST_LOG=info promptly -c corpora/default.txt "start prompt"
```

See the bundled corpus: [corpora/default.txt](corpora/default.txt)

## Configuration
At startup the app looks for the first existing file in:
- `config.yaml` (repo root)
- `config/default.yaml` (repo config dir)
- `~/.config/promptly/config.yaml`

Format (YAML):
```yaml
defaults:
  order: 2
  max_words: 100
  corpus: corpora/default.txt

model:
  auto_save: true
  compression: false
  format: yaml

logging:
  level: info
  file:
```

Notes:
- Currently only `model.auto_save` affects behavior: if true and you did not pass `--model`, the app auto-saves to `model_order_<order>.yaml`.
- CLI flags control generation; the `defaults` and `logging` sections are not yet wired to override CLI or env logger.
- You can keep an example config at [config/default.yaml](config/default.yaml).

## Model format and persistence
Models are saved as YAML with:
- `order`: the chain order
- `transitions`: map from state (Vec<String>) to next-word counts

APIs:
- Save: [`model::save_model`](src/model/mod.rs)
- Load: [`model::load_model`](src/model/mod.rs)

Auto-save path (when enabled): `model_order_<order>.yaml`.

## Architecture overview
- Corpus I/O and preprocessing: [`corpus`](src/corpus/mod.rs)
- Markov training and generation: [`markov`](src/markov/mod.rs)
  - Chain type and generation: [`markov::MarkovChain`](src/markov/chain.rs)
  - Training helper: [`markov::train_from_corpus`](src/markov/builder.rs)
- Model I/O: [`model`](src/model/mod.rs)
- CLI args: [`cli::Args`](src/cli/args.rs)

Key generation steps:
- Prompt normalization and state selection
- Weighted random next-word selection based on observed counts
- Backoff to a random known state if the current state has no transitions

## Build and run
```sh
# Debug
cargo run -- "a starting prompt"

# Release
cargo run --release -- "a starting prompt"

# Binary after install
promptly "prompt here"
```

## Development notes
- Rust edition: see [Cargo.toml](Cargo.toml)
- Logging: use `RUST_LOG=debug|info|warn|error`
- Tests: none yet (scaffold via `dev-dependencies`)

## License
MIT — see [LICENSE](LICENSE).